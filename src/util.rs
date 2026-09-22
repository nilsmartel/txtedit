pub fn read_file() -> (Option<String>, String) {
    if let Some(filename) = std::env::args().nth(1) {
        let content = std::fs::read_to_string(&filename).expect("read file");
        return (Some(filename), content);
    }
    (None, String::new())
}

pub fn str_to_buffer(s: String) -> Vec<Vec<u16>> {
    s.split("\n").map(|s| str_to_vec16(s.to_string())).collect()
}

pub fn str_to_vec16(s: String) -> Vec<u16> {
    s.encode_utf16().collect()
}

pub fn vec16_to_str(v: &[u16]) -> String {
    String::from_utf16(&v).expect("transform back into string")
}
