use crate::CrossCompiler;

mod java;

trait Generator<'a, InitOptions, GenerateOptions> {
    fn new(cross_compiler: &'a CrossCompiler, options: InitOptions) -> Self;

    fn generate_type(&self, options: GenerateOptions, composite_type_name: &str) -> String;
    fn generate_types(&self, options: GenerateOptions) -> String;
}
