use std::io;

pub fn check_extension(str: &str, extension: &str) -> bool {
    let file_ext: usize = str.len() - 4;
    if str[file_ext..] != *extension {
        false
    } else {
        true
    }
}

pub fn read_lines(file_name: &str) -> io::Result<Vec<String>> {
    Ok(
        std::fs::read_to_string(file_name)?
            .lines()
            .map(String::from)
            .collect()
    )
}