use gix_doc::{
    GixDoc,
    components::function::Function,
    outputs::markdown::{MarkDownFileImpl, MarkDownImpl},
};

fn main() {
    println!("Hello, world!");

    let mut gix_doc = GixDoc::new("test");

    let fun = Function::new("function_1", vec![], None, None, "", None);
    let fun1 = Function::new("function_2", vec![], None, Some("int"), "", None);
    let fun2 = Function::new("function_3", vec![], None, None, "", None);
    gix_doc.add_function(fun);
    gix_doc.add_function(fun1);
    gix_doc.add_function(fun2);

    gix_doc.render();
    if let Err(x) = gix_doc.save_to_file("") {
        eprint!("Error: {}", x);
    }
}
