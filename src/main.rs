use clap::Parser;
use rand::Rng;
use std::time::{Duration, UNIX_EPOCH};

mod deserializer;
mod file_reader;
mod petitions;
mod serializer;

/// Paste.lol on the command line (cant list unlisted, only target unlisted).
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Upload a file or update an existing file on the pastebin.
    #[structopt(short, long)]
    file: Option<String>,

    /// Title of the new pastebin or the title of the pastebin to update.
    #[structopt(short, long)]
    title: Option<String>,

    /// Content of the new pastebin or the content of the pastebin to update.
    #[structopt(short, long)]
    content: Option<String>,

    /// WIP Download the content of a pastebin .
    #[structopt(long)]
    download: Option<String>,

    /// WIP Get detailed information about a pastebin.
    #[structopt(short, long)]
    info: Option<String>,

    /// Remove a pastebin from the pastebin service.
    #[structopt(short, long)]
    remove: Option<String>,

    /// WIP List all the publicly listed pastebins.
    #[structopt(short, long, default_value = "false")]
    list: bool,

    /// Set your username for the pastebin service.
    #[structopt(long)]
    setuser: Option<String>,

    /// Set your API key for the pastebin service.
    #[structopt(long)]
    setapikey: Option<String>,

    /// Set to true if you want newly created pastebins to be unlisted by default. (Default: false)
    #[structopt(long)]
    setunlist: Option<bool>,
}

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

fn run(args: Args, config: deserializer::Config) {
    // println!("run");
    // Remove
    if args.remove.is_some() {
        let result = petitions::remove(
            config.user.clone(),
            config.api_key.clone(),
            args.remove.clone().unwrap(),
        )
        .unwrap();
        if result.status().is_success() {
            println!("Result: {:?}", result);
        }
    // TODO
    // Download
    } else if args.download.is_some() {
        let result =
            petitions::download(config.user.clone(), args.download.clone().unwrap()).unwrap();
        if result.status().is_success() {
            println!("Result: {:?}", result);
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
                println!();
                println!("--------------------------------------------------");
                println!(
                    "Title: {}",
                    result["response"]["paste"]["title"].as_str().unwrap()
                );
                println!(
                    "Modified on: {}",
                    chrono::DateTime::<chrono::Utc>::from(
                        UNIX_EPOCH
                            + Duration::from_secs(
                                result["response"]["paste"]["modified_on"].as_u64().unwrap()
                            )
                    )
                );
                if result["response"]["paste"]["listed"] == 1 {
                    println!("Listed");
                }
                println!("Content:");
                println!(
                    "{}",
                    result["response"]["paste"]["content"].as_str().unwrap()
                );
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

    // TODO: Improve the content printing
    // List
    } else if args.list {
        let result = petitions::list(config.user.clone());
        match result {
            Ok(result) => {
                for i in 0..result["response"]["pastebin"].as_array().unwrap().len() {
                    if i != 0 {
                        println!();
                        println!();
                        println!("--------------------------------------------------");
                        // println!("Number: {}", i);
                    }
                    println!(
                        "Title: {}",
                        result["response"]["pastebin"][i]["title"].as_str().unwrap()
                    );
                    println!(
                        "Modified on: {}",
                        chrono::DateTime::<chrono::Utc>::from(
                            UNIX_EPOCH
                                + Duration::from_secs(
                                    result["response"]["pastebin"][i]["modified_on"]
                                        .as_u64()
                                        .unwrap()
                                )
                        )
                    );
                    println!("Content: ");
                    println!();
                    println!(
                        "{}",
                        result["response"]["pastebin"][i]["content"]
                            .as_str()
                            .unwrap()
                    );
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }

        // Create a listed
    } else if !config.unlist {
        let response = petitions::create_listed(
            config.user,
            config.api_key,
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
        )
        .unwrap();
        if response.status().is_success() {
            println!("Sucessfully created pastebin");
        }
        // Wait for reply https://discourse.lol/t/feat-api-add-the-new-pastebin-url-in-the-response/960/1
        // println!("Result: {:?}", response);

        // Create a unlisted
    } else if config.unlist {
        let response = petitions::create_unlisted(
            config.user,
            config.api_key,
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
        )
        .unwrap();
        if response.status().is_success() {
            println!("Sucessfully created pastebin");
        }
        // Wait for reply https://discourse.lol/t/feat-api-add-the-new-pastebin-url-in-the-response/960/1
        // println!("Result: {:?}", response);
    }
}

fn serialize(user: String, api_key: String, unlist: bool, output: String) {
    let result = serializer::serialize(user, api_key, unlist);
    match result {
        Ok(_config) => {
            println!("{} sucessfully set.", output);
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
        }
    }
}

fn check_user_and_api(args: Args, config: deserializer::Config) {
    // println!("check_user_and_api");
    if args.setuser.is_some() || args.setapikey.is_some() || args.setunlist.is_some() {
        serialize(
            if args.setuser.is_some() {
                args.setuser.clone().unwrap()
            } else {
                config.user.clone()
            },
            if args.setapikey.is_some() {
                args.setapikey.clone().unwrap()
            } else {
                config.api_key.clone()
            },
            if args.setunlist.is_some() {
                args.setunlist.unwrap()
            } else {
                config.unlist
            },
            // TODO: Make all the possible options
            if args.setuser.is_some() && args.setapikey.is_some() {
                "User and api".to_string()
            } else if args.setapikey.is_some() {
                "Api".to_string()
            } else if args.setuser.is_some() {
                "User".to_string()
            } else {
                "Unlist".to_string()
            },
        );
        let result = deserializer::deserialized();
        match result {
            Ok(config) => {
                run(args, config);
            }

            // First run
            Err(_e) => {}
        }
    } else {
        run(args, config);
    }
}

fn main() {
    let args = Args::parse();
    let result = deserializer::deserialized();
    match result {
        Ok(config) => {
            check_user_and_api(args, config);
        }

        // First run
        Err(_e) => {
            serialize(
                "rust".to_string(),
                "rocks".to_string(),
                false,
                "First run!".to_string(),
            );
            let result = deserializer::deserialized();
            match result {
                Ok(config) => {
                    check_user_and_api(args, config);
                }

                Err(_e) => {}
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
        let title = "test";
        let content = "This is a test.";
        petitions::create_listed(
            user.to_string(),
            api_key.to_string(),
            title.to_string(),
            content.to_string(),
        )
        .unwrap();
    }
}
