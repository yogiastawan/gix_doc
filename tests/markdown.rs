use gix_doc::{components::function::Function, outputs::markdown::MarkDownImpl};

const CODE_SRC: &str = r"
// another comment
/*
 * multi line comment
 */

/**
 * @brief This is test function brief that contains description of the function.
 * This is can be multi line.
 *
 * @param x This is first parameter of function.
 * @param y This is second parameter of function and this is in multiline mode,
 * because it's so long.
 * 
 * @return int This is return type of the function.
 *
 * @note This is note of the function. It can be multiline because usualy this
 * contains long text.
 */
int add_two_number(int x,int y);

struct MyStruct{
  int i;
  uint8_t age;
};";

const FUNCTION_SRC: &str = r"
/**
 * @brief This is test function brief that contains description of the function.
 * This is can be multi line.
 *
 * @param x This is first parameter of function.
 * @param y This is second parameter of function and this is in multiline mode,
 * because it's so long.
 * n
 * @param my_struct My Custom struct
 *
 * @return This is return description of the function.
 *
 * @note This is note of the function. It can be multiline because usualy this
 * contains long text.
 */
/*jggkkg*/ int /*hhuhfikgi*/ add_two_number/*commounn*/(int /*comm*/ x, //comment1
                   int /*commenn*/ y /*comment2*/,
                   struct MyStruct my_struct);";

const FUNCTION_SRC2: &str = r"
/**
 * @brief This is test function brief that contains description of the function.
 * This is can be multi line.
 *
 * @note This is note of the function. It can be multiline because usualy this
 * contains long text.
 */
void say_hello(void/*void param*/);";

#[test]
fn markdown_parse_function() {
    let md_function = Function::parse(FUNCTION_SRC).unwrap();

    println!("{}", &md_function.render());

    assert_eq!(md_function.name, "add_two_number");
    assert_eq!(
        md_function.brief,
        "This is test function brief that contains description of the function. This is can be multi line."
    );
    assert_eq!(md_function.params.len(), 3);
    assert_eq!(md_function.params[1].var_type, "int".to_string());
    assert_eq!(md_function.params[1].var_name, "y");
    assert_eq!(
        md_function.params[1].brief,
        "This is second parameter of function and this is in multiline mode, because it's so long. n"
    );
    assert_eq!(md_function.return_type, Some("int".to_string()));

    let md_fun2 = Function::parse(FUNCTION_SRC2).unwrap();

    println!("{}", md_fun2.render());
}
