pub fn read_file() -> (Option<String>, String) {
    if let Some(filename) = std::env::args().nth(1) {
        let content = std::fs::read_to_string(&filename).expect("read file");
        return (Some(filename), content);
    }
    (None, String::new())
}