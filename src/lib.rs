use std::collections::HashSet;

use indexmap::IndexMap;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use thiserror::Error;
#[derive(Parser)]
#[grammar = "propTypes.pest"]
pub struct PestParser;

#[derive(Debug, PartialEq, Eq)]
enum CompositeType<'a> {
    Alias { references: &'a str },
    Struct { fields: IndexMap<&'a str, &'a str> },
}

fn type_name_is_primitive(name: &str) -> bool {
    name.chars()
        .next()
        .map(|c| c.is_lowercase())
        .unwrap_or(false)
}

impl<'a> CompositeType<'a> {
    fn direct_references(&self) -> Vec<&'a str> {
        match self {
            CompositeType::Alias { references } => vec![*references],
            CompositeType::Struct { fields } => fields.values().copied().collect(),
        }
    }

    fn parse(
        composite_type_name: &'a str,
        pest_composite_declaration: Pair<'a, Rule>,
    ) -> Result<Self, CrossCompilerParseError> {
        match pest_composite_declaration.as_rule() {
            Rule::EOI
            | Rule::primitive
            | Rule::compositeTypeName
            | Rule::typeName
            | Rule::structFieldName
            | Rule::structField
            | Rule::compositeDeclaration
            | Rule::file
            | Rule::WHITESPACE => unreachable!(),
            Rule::structDeclaration => {
                let tokens = pest_composite_declaration.into_inner();
                let mut struct_fields = IndexMap::new();
                for pest_struct_field in tokens {
                    match pest_struct_field.as_rule() {
                        Rule::structField => {
                            let mut tokens = pest_struct_field.into_inner();
                            let field_name = tokens.next().unwrap().as_str();
                            let type_name = tokens.next().unwrap().as_str();
                            if struct_fields.insert(field_name, type_name).is_some() {
                                return Err(CrossCompilerParseError::NonUniqueStructField {
                                    field_name: field_name.to_string(),
                                    struct_name: composite_type_name.to_string(),
                                });
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Ok(CompositeType::Struct {
                    fields: struct_fields,
                })
            }
            Rule::aliasDeclaration => {
                let mut tokens = pest_composite_declaration.into_inner();
                let references = tokens.next().unwrap().as_str();

                Ok(CompositeType::Alias { references })
            }
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CrossCompilerParseError {
    #[error("input invalid to Pest")]
    PestParse(#[from] Box<pest::error::Error<Rule>>),
    #[error("field {field_name:?} name is duplicated in struct {struct_name:?}")]
    NonUniqueStructField {
        field_name: String,
        struct_name: String,
    },
    #[error("type name {type_name:?} is duplicated")]
    NonUniqueCompositeTypeName { type_name: String },
    #[error(
        "on {composite_type_name:?} type name {reference_name:?} is referenced but not defined"
    )]
    ReferenceNotDefined {
        composite_type_name: String,
        reference_name: String,
    },
    #[error("type name {type_names:?} does not include any base primitives")]
    TypesHaveNoBase { type_names: Vec<String> },
}

#[derive(Debug, PartialEq, Eq)]
pub struct CrossCompiler<'a> {
    composite_types: IndexMap<&'a str, CompositeType<'a>>,
}

impl<'a> CrossCompiler<'a> {
    fn parse(input: &'a str) -> Result<Self, CrossCompilerParseError> {
        let pest_composite_declarations = PestParser::parse(Rule::file, input).map_err(Box::new)?;
        let mut output = Self {
            composite_types: Default::default(),
        };
        for (pest_composite_type_name, pest_composite_declaration) in Iterator::zip(
            pest_composite_declarations.clone(),
            pest_composite_declarations.skip(1),
        )
        .step_by(2)
        {
            let composite_type_name = match pest_composite_type_name.as_rule() {
                Rule::EOI
                | Rule::primitive
                | Rule::typeName
                | Rule::structFieldName
                | Rule::structField
                | Rule::compositeDeclaration
                | Rule::file
                | Rule::structDeclaration
                | Rule::aliasDeclaration
                | Rule::WHITESPACE => unreachable!(),
                Rule::compositeTypeName => pest_composite_type_name.as_str(),
            };
            let composite_type =
                CompositeType::parse(composite_type_name, pest_composite_declaration)?;
            if output
                .composite_types
                .insert(composite_type_name, composite_type)
                .is_some()
            {
                return Err(CrossCompilerParseError::NonUniqueCompositeTypeName {
                    type_name: composite_type_name.to_string(),
                });
            }
        }
        Ok(output)
    }

    /**
    Need to make sure that all references resolve to types.

    Recursive types are allowed but there must be branches that resolve to
    primitive types (the recursive base case).
    */
    fn validate_references(&'a self) -> Result<(), CrossCompilerParseError> {
        for (&name, composite_type) in &self.composite_types {
            for reference in composite_type.direct_references() {
                if !type_name_is_primitive(reference)
                    && !self.composite_types.contains_key(reference)
                {
                    return Err(CrossCompilerParseError::ReferenceNotDefined {
                        composite_type_name: name.to_string(),
                        reference_name: reference.to_string(),
                    });
                }
            }
        }

        let mut references_primitive: HashSet<&'a str> = HashSet::new();
        while references_primitive.len() < self.composite_types.len() {
            let mut changed = false;
            for (&name, composite_type) in &self.composite_types {
                if references_primitive.contains(name) {
                    continue;
                }
                for reference in composite_type.direct_references() {
                    if type_name_is_primitive(reference) || references_primitive.contains(reference)
                    {
                        references_primitive.insert(name);
                        changed = true;
                    }
                }
            }

            if !changed {
                return Err(CrossCompilerParseError::TypesHaveNoBase {
                    type_names: self
                        .composite_types
                        .keys()
                        .filter(|&&k| !references_primitive.contains(k))
                        .map(|k| k.to_string())
                        .collect(),
                });
            }
        }

        Ok(())
    }

    pub fn parse_and_validate(input: &'a str) -> Result<Self, CrossCompilerParseError> {
        let parsed = Self::parse(input)?;
        parsed.validate_references()?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_errors_with_bad_syntax() {
        let input = "struct Person { birthYear: u32, name: string, };";
        let c = CrossCompiler::parse_and_validate(input);
        assert!(matches!(c, Err(CrossCompilerParseError::PestParse(_))));
    }

    #[test]
    fn it_errors_with_duplicated_field() {
        let input = "type Person = struct  { birthYear: u32, birthYear: string, };";
        let c = CrossCompiler::parse_and_validate(input);
        assert_eq!(
            c,
            Err(CrossCompilerParseError::NonUniqueStructField {
                field_name: "birthYear".to_string(),
                struct_name: "Person".to_string()
            })
        );
    }

    #[test]
    fn it_errors_with_duplicated_composite_type_name() {
        let input = "type Person = struct { birthYear: u32, } ; type Person =  alias u32;";
        let c = CrossCompiler::parse_and_validate(input);
        assert_eq!(
            c,
            Err(CrossCompilerParseError::NonUniqueCompositeTypeName {
                type_name: "Person".to_string()
            })
        );
    }

    #[test]
    fn it_errors_with_undefined_reference() {
        let input = "type Person = struct { birthYear: Year, } ;";
        let c = CrossCompiler::parse_and_validate(input);
        assert_eq!(
            c,
            Err(CrossCompilerParseError::ReferenceNotDefined {
                composite_type_name: "Person".to_string(),
                reference_name: "Year".to_string()
            })
        );
    }

    #[test]
    fn it_errors_with_references_with_no_indirect_primitives() {
        let input = "type Person = struct { birthYear: Year, } ; type Year = alias Year;";
        let c = CrossCompiler::parse_and_validate(input);
        assert_eq!(
            c,
            Err(CrossCompilerParseError::TypesHaveNoBase {
                type_names: vec!["Person".to_string(), "Year".to_string()]
            })
        );
    }

    #[test]
    fn it_parses_structs_and_aliases() {
        let input =
            "type Year = alias u32; type Person = struct { birthYear: Year, name: string, };";
        let c = CrossCompiler::parse_and_validate(input);
        assert_eq!(
            c,
            Ok(CrossCompiler {
                composite_types: [
                    ("Year", CompositeType::Alias { references: "u32" }),
                    (
                        "Person",
                        CompositeType::Struct {
                            fields: [("birthYear", "Year"), ("name", "string")]
                                .into_iter()
                                .collect()
                        }
                    )
                ]
                .into_iter()
                .collect()
            })
        )
    }
}
