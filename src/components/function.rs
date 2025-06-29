use std::str::FromStr;

use crate::outputs::markdown::MarkDownImpl;

#[derive(Debug)]
pub struct Parameter {
    pub var_type: String,
    pub var_name: String,
    pub brief: String,
}

impl Parameter {
    pub fn new(var_type: &str, var_name: &str, brief: &str) -> Self {
        Self {
            var_type: var_type.to_string(),
            var_name: var_name.to_string(),
            brief: brief.to_string(),
        }
    }
}

impl MarkDownImpl for Parameter {
    fn render(&self) -> String {
        String::from("")
    }
}

// read all first then create var function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub brief: String,
    pub note: Option<String>,
}

impl Function {
    pub fn new(
        name: &str,
        params: Vec<Parameter>,
        return_type: Option<&str>,
        brief: &str,
        note: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_string(),
            params: params,
            return_type: return_type.map(|s| s.to_string()),
            brief: brief.to_string(),
            note: note.map(|v| v.to_string()),
        }
    }

    pub fn parse(string: &str) -> Result<Self, String> {
        let name = "";
        let params = vec![];
        let return_type = None;
        let brief = "";
        let note = None;

        Ok(Self {
            name: name.to_string(),
            params: params,
            return_type: return_type,
            brief: brief.to_string(),
            note: note,
        })
    }
}

impl MarkDownImpl for Function {
    fn render(&self) -> String {
        String::from_str("Function").unwrap()
    }
}
