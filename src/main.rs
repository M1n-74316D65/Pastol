use clap::Parser;

use std::collections::HashMap;

mod deserialize;
mod file_reader;
mod serializer;

/// Simple program to upload or update a pastebin to paste.lol
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Upload a file to pastebin
    #[arg(short, long)]
    file: Option<String>,

    /// Name of the new pastebin
    #[arg(short, long)]
    title: Option<String>,

    /// Content of the new pastebin
    #[arg(short, long)]
    content: Option<String>,

    /// Content of the new pastebin
    #[arg(short, long, default_value_t = true)]
    listed: bool,

    /// Set for your paste.lol username.
    #[arg(long)]
    setuser: Option<String>,

    /// Set for your paste.lol api key
    #[arg(long)]
    setapikey: Option<String>,
}

#[tokio::main]
async fn create_unlisted(
    user: String,
    api_key: String,
    title: String,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}{}{}", "https://api.omg.lol/address/", user, "/pastebin/");
    let mut map = HashMap::new();
    map.insert("title", title);
    map.insert("content", content);
    let client = reqwest::Client::new();
    client
        .post(url)
        .bearer_auth(api_key)
        .json(&map)
        .send()
        .await?;
    Ok(())
}

#[tokio::main]
async fn create_listed(
    user: String,
    api_key: String,
    title: String,
    content: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}{}{}", "https://api.omg.lol/address/", user, "/pastebin/");
    let mut map = HashMap::new();
    map.insert("title", title);
    map.insert("content", content);
    map.insert("listed", "".to_string());
    let client = reqwest::Client::new();
    client
        .post(url)
        .bearer_auth(api_key)
        .json(&map)
        .send()
        .await?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let result = deserialize::deserialized();
    match result {
        Ok(config) => {
            if args.setuser.is_some() {
                let result =
                    serializer::serialize(args.setuser.clone().unwrap(), config.api_key.clone());
                match result {
                    Ok(config) => {
                        println!("");
                        println!("Sucessfully set user: {}", config.user.clone());
                    }
                    Err(e) => {
                        println!("");
                        eprintln!("Error loading config: {}", e);
                    }
                }
            }
            if args.setapikey.is_some() {
                let result =
                    serializer::serialize(config.user.clone(), args.setapikey.clone().unwrap());
                match result {
                    Ok(_config) => {
                        println!("");
                        println!("Sucessfully set api.");
                    }
                    Err(e) => {
                        println!("");
                        eprintln!("Error loading config: {}", e);
                    }
                }
            }
            if args.listed {
                create_listed(
                    config.user,
                    config.api_key,
                    if args.title.is_some() {
                        args.title.unwrap().to_string()
                    } else if args.file.is_some() {
                        file_reader::read_path(args.file.clone().unwrap())
                    } else {
                        "no_title".to_string()
                    },
                    if args.content.is_some() {
                        args.content.unwrap().to_string()
                    } else if args.file.is_some() {
                        file_reader::read_file(args.file.clone().unwrap()).unwrap()
                    } else {
                        "no_content".to_string()
                    },
                )
                .unwrap();
            } else {
                create_unlisted(
                    config.user,
                    config.api_key,
                    if args.title.is_some() {
                        args.title.unwrap().to_string()
                    } else if args.file.is_some() {
                        file_reader::read_path(args.file.clone().unwrap())
                    } else {
                        "no_title".to_string()
                    },
                    if args.content.is_some() {
                        args.content.unwrap().to_string()
                    } else if args.file.is_some() {
                        file_reader::read_file(args.file.clone().unwrap()).unwrap()
                    } else {
                        "no_content".to_string()
                    },
                )
                .unwrap();
            }
        }

        // First run
        Err(_e) => {
            if args.setuser.is_some() && args.setapikey.is_some() {
                let result = serializer::serialize(
                    args.setuser.clone().unwrap(),
                    args.setapikey.clone().unwrap(),
                );
                match result {
                    Ok(config) => {
                        println!("");
                        println!("Sucessfully set user: {}", config.user.clone());
                        println!("");
                        println!("Sucessfully set api.");
                    }
                    Err(e) => {
                        println!("");
                        eprintln!("Error loading config: {}", e);
                    }
                }
            } else {
                let result = serializer::serialize("rust".to_string(), "rocks".to_string());
                match result {
                    Ok(_config) => {
                        println!("");
                        print!("Hello!");
                    }
                    Err(e) => {
                        println!("");
                        eprintln!("Error loading config: {}", e);
                    }
                }
                if args.setuser.is_some() {
                    let result =
                        serializer::serialize(args.setuser.clone().unwrap(), "rocks".to_string());
                    match result {
                        Ok(config) => {
                            println!("");
                            println!("Sucessfully set user: {}", config.user.clone());
                        }
                        Err(e) => {
                            println!("");
                            eprintln!("Error loading config: {}", e);
                        }
                    }
                }
                if args.setapikey.is_some() {
                    let result =
                        serializer::serialize("rust".to_string(), args.setapikey.clone().unwrap());
                    match result {
                        Ok(_config) => {
                            println!("");
                            println!("Sucessfully set api.");
                        }
                        Err(e) => {
                            println!("");
                            eprintln!("Error loading config: {}", e);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_listed() {
        let user = "";
        let api_key = "";
        let title = "new-paste";
        let content = "This is a new paste.";
        create_listed(
            user.to_string(),
            api_key.to_string(),
            title.to_string(),
            content.to_string(),
        )
        .unwrap();
    }
}
