use std::fmt::Write;

use crate::CrossCompiler;

use super::Generator;

pub(crate) struct Java<'a> {
    pub(crate) cross_compiler: &'a CrossCompiler<'a>,
}

impl<'a> Generator<'a, (), ()> for Java<'a> {
    fn new(cross_compiler: &'a CrossCompiler, _options: ()) -> Self {
        Self { cross_compiler }
    }

    fn generate_type(&self, _options: (), composite_type_name: &str) -> String {
        let mut output = String::new();
        let composite_type = &self.cross_compiler.composite_types[composite_type_name];
        write!(&mut output, "interface {composite_type_name} {{\n").unwrap();
        write!(&mut output, "}}\n").unwrap();
        output
    }

    fn generate_types(&self, options: ()) -> String {
        let mut output = String::new();
        for &name in self.cross_compiler.composite_types.keys() {
            output.push_str(&self.generate_type(options, name));
        }

        output
    }
}
