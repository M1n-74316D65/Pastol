use rand::Rng;
use regex::Regex;
use std::time::{Duration, UNIX_EPOCH};

use super::AddArgs;
use super::DownloadArgs;
use super::RemoveArgs;
use super::SearchArgs;
use super::ViewArgs;
use crate::deserializer;
use crate::file_reader;
use crate::petitions;
use crate::write_markdown;

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    random_string
}

pub fn remove(args: &RemoveArgs, config: deserializer::Config) {
    let result = petitions::remove(
        config.user.clone(),
        config.api_key.clone(),
        args.title.clone(),
    );
    match result {
        Ok(result) => {
            println!("\n{}", result["response"]["message"].as_str().unwrap());
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

pub fn download(args: &DownloadArgs, config: deserializer::Config) {
    let result = petitions::download(config.user.clone(), args.title.clone());
    match result {
        Ok(result) => {
            if result["request"]["status_code"].as_i64().unwrap() == 200 {
                write_markdown::write_markdown(
                    result["response"]["paste"]["title"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    result["response"]["paste"]["content"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                );
            } else {
                println!("\n{}", result["response"]["message"].as_str().unwrap());
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

pub fn view(args: &ViewArgs, config: deserializer::Config) {
    let result = petitions::show(
        config.user.clone(),
        config.api_key.clone(),
        args.title.clone(),
    );
    match result {
        Ok(result) => {
            if result["request"]["status_code"].as_i64().unwrap() == 200 {
                termimad::print_text(
                    ("\n----".to_string()
                        + "\n\n# "
                        + result["response"]["paste"]["title"].as_str().unwrap()
                        + "\n## "
                        + if result["response"]["paste"]["listed"].as_i64().is_none() {
                            "Unlisted"
                        } else {
                            "Listed"
                        }
                        + "\n## "
                        + chrono::DateTime::<chrono::Utc>::from(
                            UNIX_EPOCH
                                + Duration::from_secs(
                                    result["response"]["paste"]["modified_on"].as_u64().unwrap(),
                                ),
                        )
                        .to_string()
                        .as_str()
                        + "\n\n"
                        + result["response"]["paste"]["content"].as_str().unwrap())
                    .as_str(),
                );
            } else {
                println!("\n{}", result["response"]["message"].as_str().unwrap());
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

pub fn list(config: deserializer::Config) {
    let result = petitions::list(config.user.clone(), config.api_key.clone());
    match result {
        Ok(result) => {
            if result["request"]["status_code"].as_i64().unwrap() == 200 {
                for i in (0..result["response"]["pastebin"].as_array().unwrap().len()).rev() {
                    termimad::print_text(
                        ("\n----".to_string()
                            + "\n\n**Title: **"
                            + result["response"]["pastebin"][i]["title"].as_str().unwrap()
                            + "\n**Status: **"
                            + if result["response"]["pastebin"][i]["listed"]
                                .as_i64()
                                .is_none()
                            {
                                "Unlisted"
                            } else {
                                "Listed"
                            }
                            + "\n**Modified on: **"
                            + chrono::DateTime::<chrono::Utc>::from(
                                UNIX_EPOCH
                                    + Duration::from_secs(
                                        result["response"]["pastebin"][i]["modified_on"]
                                            .as_u64()
                                            .unwrap(),
                                    ),
                            )
                            .to_string()
                            .as_str())
                        .as_str(),
                    );
                }
            } else {
                // I don't know how it can fail but if it does
                println!("\n{}", result["response"]["message"].as_str().unwrap());
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

pub fn search(args: &SearchArgs, config: deserializer::Config) {
    let result = petitions::list(config.user.clone(), config.api_key.clone());
    match result {
        Ok(result) => {
            if result["request"]["status_code"].as_i64().unwrap() == 200 {
                let mut exists = false;
                for i in (0..result["response"]["pastebin"].as_array().unwrap().len()).rev() {
                    // Check if the script contains the word "hola"
                    if result["response"]["pastebin"][i]["title"]
                        .as_str()
                        .unwrap()
                        .contains(&args.title.clone())
                    {
                        termimad::print_text(
                            ("\n----".to_string()
                                + "\n\n# "
                                + result["response"]["pastebin"][i]["title"].as_str().unwrap()
                                + "\n## "
                                + if result["response"]["pastebin"][i]["listed"]
                                    .as_i64()
                                    .is_none()
                                {
                                    "Unlisted"
                                } else {
                                    "Listed"
                                }
                                + "\n## "
                                + chrono::DateTime::<chrono::Utc>::from(
                                    UNIX_EPOCH
                                        + Duration::from_secs(
                                            result["response"]["pastebin"][i]["modified_on"]
                                                .as_u64()
                                                .unwrap(),
                                        ),
                                )
                                .to_string()
                                .as_str()
                                + "\n\n"
                                + result["response"]["pastebin"][i]["content"]
                                    .as_str()
                                    .unwrap())
                            .as_str(),
                        );
                        exists = true;
                    }
                }
                if !exists {
                    termimad::print_text("\n **Not found**");
                }
            } else {
                // I don't know how it can fail but if it does
                println!("\n{}", result["response"]["message"].as_str().unwrap());
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

pub fn add(args: &AddArgs, config: deserializer::Config) {
    // Create
    let result = petitions::create(
        config.user,
        config.api_key,
        config.unlist,
        if args.title.is_some() {
            args.title.as_ref().unwrap().to_string()
        } else if args.file.is_some() {
            file_reader::read_path(args.file.clone().unwrap())
        } else {
            generate_random_string(15)
        },
        if args.content.is_some() {
            args.content.as_ref().unwrap().to_string()
        } else if args.file.is_some() {
            file_reader::read_file(args.file.clone().unwrap()).unwrap()
        } else {
            "".to_string()
        },
    );
    match result {
        Ok(result) => {
            if result["request"]["status_code"].as_i64().unwrap() == 200 {
                println!(
                    "\n{}",
                    // Remove all html tags
                    Regex::new(r#"<a[^>]*\bhref="([^"]*)"[^>]*>.*?</a>"#)
                        .unwrap()
                        .replace_all(
                            result["response"]["message"].as_str().unwrap(),
                            |caps: &regex::Captures<'_>| -> String {
                                if let Some(m) = caps.get(1) {
                                    m.as_str().to_string()
                                } else {
                                    String::new()
                                }
                            },
                        )
                );
            } else {
                println!("\n{}", result["response"]["message"].as_str().unwrap());
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
