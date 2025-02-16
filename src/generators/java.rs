use crate::CrossCompiler;

use super::Generator;

struct Java<'a> {
    cross_compiler: &'a CrossCompiler
};

impl<'a> Generator<'a, (), ()> for Java<'a> {
    fn new(cross_compiler: &'a CrossCompiler, _options: ()) -> Self {
        Self {cross_compiler}
    }

    fn generate_type(&self, _options: (), composite_type_name: &str) -> String {
        let mut output = String::new();
        let composite_type = &self.cross_compiler.composite_types[composite_type_name];
        output
    }

    fn generate_types(&self, options: ()) -> String {
        todo!()
    }
}
