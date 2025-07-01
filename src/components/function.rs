use std::{collections::HashMap, str::FromStr};

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
        let output = format!("- *{} {}*: {}", self.var_type, self.var_name, self.brief);
        output
    }
}

enum FunctionState {
    Unknown,
    Brief(String),
    Param(String, String),
    ReturnDesc(String),
    Note(String),
    FunctionDeclare(String),
}

// read all first then create var function
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_desc: Option<String>,
    pub return_type: Option<String>,
    pub brief: String,
    pub note: Option<String>,
    func_declare: String,
}

impl Function {
    pub fn parse(string: &str) -> Result<Self, String> {
        let mut current_state = FunctionState::Unknown;
        let lines = string.lines();

        if string.is_empty() {
            return Err("String to parse is empty".to_string());
        }

        let mut params: HashMap<String, String> = HashMap::new();
        let mut return_desc: Option<String> = None;
        let mut brief = String::new();
        let mut note: Option<String> = None;
        let mut func_declare = String::new();

        for line in lines {
            // println!("||{}", line);

            let line = line.trim_start();

            if line.trim_end().len() <= "*".len() {
                continue;
            }

            if "/**" == &line[..] {
                continue;
            }

            if &line[..] == "*/" {
                continue;
            }

            // println!("{}", &line[0..8]);
            if "*" != &line[..1] {
                let fun = function_saniter(line)?;
                current_state = FunctionState::FunctionDeclare(fun);
            } else if "* @" != &line[..3] {
                multi_line_handle(line, &mut current_state);
            } else if "* @note" == &line[..7] {
                note = Some(String::new());
                let note = parse_note(line)?;
                current_state = FunctionState::Note(note);
            } else if "* @param" == &line[..8] {
                let (name_param, desc) = parse_param(line)?;
                params.insert(name_param.clone(), String::new());
                current_state = FunctionState::Param(name_param, desc);
            } else if "* @brief" == &line[..8] {
                let brief = parse_brief(line)?;
                current_state = FunctionState::Brief(brief);
            } else if "* @return" == &line[..9] {
                let ret = parse_return(line)?;
                return_desc = Some(String::new());
                current_state = FunctionState::ReturnDesc(ret);
            } else {
                current_state = FunctionState::Unknown;
            }

            match current_state {
                FunctionState::Brief(ref s) => {
                    // println!("push brief: {}", &s);
                    brief.push_str(&s);
                }
                FunctionState::Param(ref name, ref desc) => {
                    let param = params
                        .get_mut(name.as_str())
                        .ok_or(format!("Parameter with name {} not found in hasmap", &name))?;
                    // println!("push param brief: {}", &desc);
                    param.push_str(&desc);
                }
                FunctionState::ReturnDesc(ref r) => {
                    return_desc.as_mut().map(|s| s.push_str(&r));
                }
                FunctionState::Note(ref n) => {
                    note.as_mut().map(|s| s.push_str(n));
                }
                FunctionState::FunctionDeclare(ref s) => {
                    func_declare.push_str(&s);
                }
                _ => {}
            }
        }

        // println!(">>{}", &brief);
        // for p in params.values() {
        //     println!(">>{:?}", p);
        // }
        // println!(">>{:?}", return_desc);
        // println!(">>{:?}", note);
        // println!(">>{}", func_declare);
        //
        let (fun, name, return_type, parameters) = parse_function_decl(&func_declare, &params)?;

        // println!("Func: {}", fun);
        Ok(Self {
            name,
            params: parameters,
            return_desc: return_desc,
            return_type: return_type,
            brief: brief,
            note: note,
            func_declare: fun,
        })
    }
}

impl MarkDownImpl for Function {
    fn render(&self) -> String {
        let name = &self.name;
        let brief = &self.brief;
        let params = if self.params.is_empty() {
            "No parameter (void)".to_string()
        } else {
            self.params
                .iter()
                .map(|p| p.render())
                .collect::<Vec<String>>()
                .join("\n")
        };
        let return_desc = match &self.return_desc {
            Some(x) => x.to_string(),
            None => "No return value".to_string(),
        };
        let note = match &self.note {
            Some(x) => format!("> {}", x),
            None => "".to_string(),
        };
        let fun = &self.func_declare;

        let out_render = format!(
            r"## {name}
```c
{fun}
```
{brief}
### Parameters:
{params}
### Return:
{return_desc}
{note}"
        );

        out_render
    }
}

fn multi_line_handle(src: &str, state: &mut FunctionState) {
    // * New line here
    let mut out = String::from(" ");
    out.push_str(&src[2..].to_string().trim_end());
    match state {
        FunctionState::Brief(_x) => *state = FunctionState::Brief(out),
        FunctionState::Param(n, _d) => *state = FunctionState::Param(n.to_string(), out),
        _ => *state = FunctionState::Unknown,
    }
}

fn parse_brief(string: &str) -> Result<String, String> {
    let string = string.trim_end();
    if string.len() <= "* @brief".len() {
        return Err("No brief description found".to_string());
    }

    let output = &string[9..];
    Ok(output.to_string())
}

fn parse_param(src: &str) -> Result<(String, String), String> {
    let src = src.trim_end().split(' ').collect::<Vec<&str>>();
    // * @param name desc
    if src.len() < 2 {
        return Err("No param name found".to_string());
    } else if src.len() < 3 {
        return Err("No param description found".to_string());
    }

    let name = src[2].to_string();
    let desc = &src[3..].join(" ");

    Ok((name, desc.to_string()))
}

fn parse_return(src: &str) -> Result<String, String> {
    let src = src.trim_end();
    if src.len() <= "* @return".len() {
        return Err("".to_string());
    }
    let ret = &src[10..];
    Ok(ret.to_string())
}

fn parse_note(src: &str) -> Result<String, String> {
    let src = src.trim_end();

    if src.len() <= "* @note".len() {
        return Err("No note description found".to_string());
    }
    let note = &src[8..];
    Ok(note.to_string())
}

fn function_saniter(src: &str) -> Result<String, String> {
    let src = src.trim_end();

    // println!("rc: {}", src);

    if src.len() < 1 {
        return Err("Declaration not found".to_string());
    }
    let src = if src.contains("//") {
        let index = src
            .find("//")
            .ok_or("Cannot find inline comment".to_string())?;

        src[..index].to_string()
    } else {
        src.to_string()
    };
    // println!("src:{}", &src);
    let mut func = String::new();
    let src_com = src.split("/*").collect::<Vec<&str>>();
    if src_com.len() <= 1 {
        return Ok(src.to_string());
    }

    // println!("==>COM::{:?}", &src_com);

    for f in src_com {
        let s = f.split("*/").collect::<Vec<&str>>();
        // println!("==>{:?}", &s);
        let f = if s.len() <= 1 { f } else { s[1] };
        func.push_str(f.trim_end().trim_start());
        func.push_str(" ");
    }

    let func = func.trim_end();

    Ok(func.to_string())
}

fn parse_function_decl(
    src: &str,
    params: &HashMap<String, String>,
) -> Result<(String, String, Option<String>, Vec<Parameter>), String> {
    // int function_name(int var_name, char var_name);
    let src = src.trim_end().trim_start();
    if src.len() < 1 {
        return Err("No function declaration found".to_string());
    }

    let l_index = src
        .find('(')
        .ok_or("No left parenthesis found".to_string())?;
    let r_index = src
        .find(')')
        .ok_or("No right parenthesis found".to_string())?;

    //get return type and name func
    let part1 = &src[..l_index].trim_end();
    let part1 = part1.split(' ').collect::<Vec<&str>>();

    if part1.len() <= 1 {
        return Err("No return type or function name found".to_string());
    }

    let function_name = part1.last().ok_or("Cannot get function name".to_string())?;
    let return_type = Some(part1[..part1.len() - 1].join(" "));

    //get parameters
    let part2 = &src[l_index + 1..r_index].trim_end().trim_start();
    let part2 = part2.split(',').collect::<Vec<&str>>();

    let mut parameters: Vec<Parameter> = Vec::with_capacity(part2.len());

    let mut param_variable: Vec<String> = Vec::with_capacity(part2.len());

    for p in part2 {
        let prm = p.split(' ').collect::<Vec<&str>>();
        let param_type = &prm[..prm.len() - 1].join(" ");
        let param_name = prm.last().ok_or("Cannot get parameter name".to_string())?;
        let param_desc = params
            .get(&param_name.to_string())
            .ok_or(format!("Parameter name {} is not documented", &param_name))?;
        let param_desc = param_desc;

        param_variable.push(format!("{} {}", param_type, param_name));

        parameters.push(Parameter::new(param_type, param_name, &param_desc));
    }

    let (return_type, ret) = match return_type.as_deref() {
        Some("void") | None => (None, "void".to_string()),
        Some(x) => (Some(x.to_string()), x.to_string()),
    };

    let fun = format!(
        "{} {}({});",
        &ret,
        &function_name,
        param_variable.join(", ")
    );

    Ok((fun, function_name.to_string(), return_type, parameters))
}
