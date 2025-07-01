use std::fmt::Write;

use crate::outputs::markdown::MarkDownImpl;

pub struct FieldStruct {
    pub field_type: String,
    pub field_name: String,
    pub field_desc: Option<String>,
}

impl FieldStruct {
    pub fn new(field_type: &str, field_name: &str, field_desc: Option<&str>) -> Self {
        Self {
            field_type: field_type.to_string(),
            field_name: field_name.to_string(),
            field_desc: field_desc.map(|s| s.to_string()),
        }
    }
}

impl MarkDownImpl for FieldStruct {
    fn render(&self) -> String {
        let desc = self
            .field_desc
            .as_ref()
            .map_or(String::new(), |x| format!(": {}", x));
        format!("- *{} {}*{}", self.field_type, self.field_name, desc)
    }
}

pub struct CStruct {
    pub src: String,
    pub name: String,
    pub desc: String,
    pub fields: Vec<FieldStruct>,
    pub note: Option<String>,
}

impl CStruct {
    pub fn parse(src: &str) -> Result<Self, String> {
        Ok(Self {
            src: String::new(),
            name: String::new(),
            desc: String::new(),
            fields: vec![],
            note: None,
        })
    }
}

impl MarkDownImpl for CStruct {
    fn render(&self) -> String {
        let name = &self.name;
        let src = &self.src;
        let desc = &self.desc;
        let fields = if self.fields.is_empty() {
            "None".to_string()
        } else {
            let mut res = String::new();
            for (i, f) in self.fields.iter().enumerate() {
                if i > 0 {
                    res.push('\n');
                }
                let _ = write!(res, "{}", f.render());
            }
            res
        };

        let note = self
            .note
            .as_ref()
            .map_or(String::new(), |n| format!("\n> {}", n));

        format!(
            r"## {name}
```c
{src}
```
{desc}
### Fields:
{fields}{note}"
        )
    }
}
