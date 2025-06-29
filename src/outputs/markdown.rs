pub trait MarkDownImpl {
    fn render(&self) -> String;
}

pub trait MarkDownFileImpl {
    fn save_to_file(&self, path: &str) -> Result<(), String>;
}
