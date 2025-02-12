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
        let pest_parsed = PestParser::parse(Rule::file, input).unwrap();
        let mut output = Self {
            composite_types: Default::default(),
        };
        output
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
