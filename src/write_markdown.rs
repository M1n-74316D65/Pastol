use std::fs::File;
use std::io::Write;

pub fn write_markdown(name: String, content: String) {
    let mut file =
        File::create("./".to_string() + name.as_str() + ".md").expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data to file");

    println!("{}.md created successfully!", name);
}
