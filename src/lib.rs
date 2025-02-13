use indexmap::IndexMap;
use pest::Parser;
use pest_derive::Parser;

use thiserror::Error;
#[derive(Parser)]
#[grammar = "propTypes.pest"]
pub struct PestParser;

#[derive(Debug)]
enum CompositeType<'a> {
    Alias { references: &'a str },
    Struct { fields: IndexMap<&'a str, &'a str> },
}

#[derive(Debug)]
struct CrossCompiler<'a> {
    composite_types: IndexMap<&'a str, CompositeType<'a>>,
}

#[derive(Error, Debug, PartialEq, Eq)]
enum CrossCompilerParseError {
    #[error("input invalid to Pest")]
    PestParse(#[from] Box<pest::error::Error<Rule>>),
    #[error("field {field_name:?} name is duplicated in struct {struct_name:?}")]
    NonUniqueStructField {
        field_name: String,
        struct_name: String,
    },
}

impl<'a> CrossCompiler<'a> {
    fn parse(input: &'a str) -> Result<Self, CrossCompilerParseError> {
        let pest_composite_declarations = PestParser::parse(Rule::file, input).map_err(Box::new)?;
        let mut output = Self {
            composite_types: Default::default(),
        };
        for pest_composite_declaration in pest_composite_declarations {
            match pest_composite_declaration.as_rule() {
                Rule::EOI => {}
                Rule::primitive
                | Rule::compositeTypeName
                | Rule::typeName
                | Rule::structFieldName
                | Rule::structField
                | Rule::aliasDeclaration
                | Rule::compositeDeclaration
                | Rule::file
                | Rule::WHITESPACE => unreachable!(),
                Rule::structDeclaration => {
                    let mut tokens = pest_composite_declaration.into_inner();
                    let composite_type_name = tokens.next().unwrap().as_str();
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
                    if output
                        .composite_types
                        .insert(
                            composite_type_name,
                            CompositeType::Struct {
                                fields: struct_fields,
                            },
                        )
                        .is_some()
                    {
                        panic!("expected type name to be unique");
                    };
                }
            }
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_errors_with_bad_syntax() {
        let input = "struct Person [ birthYear: u32, name: string, ]";
        let c = CrossCompiler::parse(input);
        assert!(matches!(c, Err(CrossCompilerParseError::PestParse(_))));
    }

    #[test]
    fn it_errors_with_duplicated_field() {
        let input = "struct Person { birthYear: u32, birthYear: string, }";
        let c = CrossCompiler::parse(input);
        assert_eq!(
            c.err(),
            Some(CrossCompilerParseError::NonUniqueStructField {
                field_name: "birthYear".to_string(),
                struct_name: "Person".to_string()
            })
        );
    }

    #[test]
    fn it_works() {
        let input = " struct Person { birthYear: u32, name: string, }";
        let c = CrossCompiler::parse(input);
        dbg!(c);
    }
}
