use gix_doc::components::function::Function;

const FUNCTION_DOC: &str = r"
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
";

#[test]
fn markdown_parse_function() {
    let md_function = Function::parse(FUNCTION_DOC).unwrap();

    assert_eq!(md_function.name, "add_two_number");
    assert_eq!(
        md_function.brief,
        r"This is test function brief that contains description of the function.
 This is can be multi line."
    );
    assert_eq!(md_function.params.len(), 2);
    assert_eq!(md_function.params[1].var_type, "int");
    assert_eq!(md_function.params[1].var_name, "y");
    assert_eq!(
        md_function.params[1].brief,
        r"This is second parameter of function and this is in multiline mode,
 because it's so long."
    );
    assert_eq!(md_function.return_type, Some("int".to_string()));
}
