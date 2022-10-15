use std::fs;

pub async fn publickey() -> String {
    match fs::read_to_string("./publickey.txt") {
        Ok(s) => s,
        Err(_) => "Unable to open file".to_string(),
    }
}
