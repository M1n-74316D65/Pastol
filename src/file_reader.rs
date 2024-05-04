use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_file(filename: String) -> io::Result<String> {
    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut content = String::new();

    // Read the file line by line
    for line in reader.lines() {
        content.push_str(&line?);
    }

    Ok(content)
}

pub fn read_path(file_path: String) -> String {
    // Convert the file path to a Path object
    let path = Path::new(file_path.as_str());

    // Get the file name
    if let Some(file_name) = path.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            file_name_str.to_string()
        } else {
            file_path
        }
    } else {
        file_path
    }
}
