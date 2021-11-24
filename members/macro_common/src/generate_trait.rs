use proc_macro2::{TokenStream};

pub trait Generate {
    fn generate(&mut self, input: TokenStream) -> Result<TokenStream, String>;
    
    fn auto_append(&self) -> bool;
    
    fn generate_auto(&mut self, input: TokenStream) -> Result<TokenStream, String> {
        //self.generate(input)
        #[cfg(feature = "verbose")]
        println!(" --- \\/ Generating \\/ --- ");
        let generated = self.generate(input.clone())?;
        let mut output;
        if self.auto_append() {
            output = input;
            output.extend(generated);
        } else {
            output = generated;
        }
        #[cfg(feature = "verbose")]
        println!(" --- \\/ Generated \\/ --- \n{}\n --- /\\ Generated /\\ (auto:{})---", output, self.auto_append());
        Ok(output)
    }
}
