use std::fs::File;
use std::io::prelude::*;

pub fn read_integer_file(file_name: &str) -> Result<Vec<i32>, std::io::Error> {
    let file_contents = read_file(file_name)?;
    let result = file_contents
        .split("\n")
        .filter_map(|item| item.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    Ok(result)
}

pub fn read_file_lines(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let file_contents = read_file(file_name)?;
    let result = file_contents
        .split("\n")
        .map(|str| String::from(str))
        .collect::<Vec<String>>();
    Ok(result)
}

pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(format!("inputs/{0}", file_name))?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(file_content)
}

#[cfg(test)]
mod tests {
    use super::read_integer_file;
    #[test]
    fn can_read_integer_file() {
        let result = read_integer_file("day1.txt");
        assert!(
            result.unwrap().len() > 0,
            "List of files contained no integers"
        );
    }
}
