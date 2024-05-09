use rand::Rng;
use regex::Regex;
use std::time::{Duration, UNIX_EPOCH};

use super::Args;
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

pub fn petition_manager(args: Args, config: deserializer::Config) {
    // Remove
    if args.remove.is_some() {
        let result = petitions::remove(
            config.user.clone(),
            config.api_key.clone(),
            args.remove.clone().unwrap(),
        );
        match result {
            Ok(result) => {
                println!("{}", result["response"]["message"].as_str().unwrap());
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }

    // Download
    } else if args.download.is_some() {
        let result = petitions::download(config.user.clone(), args.download.clone().unwrap());
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
                    println!("{}", result["response"]["message"].as_str().unwrap());
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

    // Info
    } else if args.info.is_some() {
        let result = petitions::show(
            config.user.clone(),
            config.api_key.clone(),
            args.info.clone().unwrap(),
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
                                        result["response"]["paste"]["modified_on"]
                                            .as_u64()
                                            .unwrap(),
                                    ),
                            )
                            .to_string()
                            .as_str()
                            + "\n\n"
                            + result["response"]["paste"]["content"].as_str().unwrap())
                        .as_str(),
                    );
                } else {
                    println!("{}", result["response"]["message"].as_str().unwrap());
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

    // List pastebins
    } else if args.list {
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
                    println!("{}", result["response"]["message"].as_str().unwrap());
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    } else if args.search.is_some() {
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
                            .contains(&args.search.clone().unwrap())
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
                    println!("{}", result["response"]["message"].as_str().unwrap());
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    } else {
        // Create
        let result = petitions::create(
            config.user,
            config.api_key,
            config.unlist,
            if args.title.is_some() {
                args.title.unwrap().to_string()
            } else if args.file.is_some() {
                file_reader::read_path(args.file.clone().unwrap())
            } else {
                generate_random_string(15)
            },
            if args.content.is_some() {
                args.content.unwrap().to_string()
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
                        "{}",
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
                    println!("{}", result["response"]["message"].as_str().unwrap());
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }
}
