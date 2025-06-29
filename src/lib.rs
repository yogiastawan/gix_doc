use crate::{
    components::function::Function,
    outputs::markdown::{MarkDownFileImpl, MarkDownImpl},
};

pub mod components;
pub mod outputs;

pub struct GixDoc {
    name_file: String,
    functions: Vec<Function>,
}

impl GixDoc {
    pub fn new(name_file: &str) -> Self {
        Self {
            name_file: name_file.to_string(),
            functions: vec![],
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn start_parse(&mut self, src: &str) -> Result<(), String> {
        Ok(())
    }
}

impl MarkDownImpl for GixDoc {
    fn render(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.name_file.as_str());
        output.push_str("\n\n");
        for function in &self.functions {
            output.push_str(&function.render());
        }
        output
    }
}

impl MarkDownFileImpl for GixDoc {
    fn save_to_file(&self, path: &str) -> Result<(), String> {
        let output = self.render();
        //write file

        Ok(())
    }
}
