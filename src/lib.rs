use std::default;

use indexmap::IndexMap;
use pest::Parser;
use pest_derive::Parser;

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

impl<'a> CrossCompiler<'a> {
    fn parse(input: &'a str) -> Self {
        let pest_composite_declarations = PestParser::parse(Rule::file, input).unwrap();
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
                    let type_name = tokens.next().unwrap().as_str();
                    let mut struct_fields = IndexMap::new();
                    for pest_struct_field in tokens {
                        match pest_struct_field.as_rule() {
                            Rule::structField => {
                                let mut tokens = pest_struct_field.into_inner();
                                let field_name = tokens.next().unwrap().as_str();
                                let type_name = tokens.next().unwrap().as_str();
                                if struct_fields.insert(field_name, type_name).is_some() {
                                    panic!("expected field name to be unique");
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    if output
                        .composite_types
                        .insert(
                            type_name,
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
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = " struct Person { birthYear: u32, name: string, }";
        let c = CrossCompiler::parse(input);
        dbg!(c);
    }
}
