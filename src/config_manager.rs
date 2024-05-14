use crate::deserializer;
use crate::serializer;

use super::SettingsArgs;

pub fn serialize(user: String, api_key: String, unlist: bool, output: String) {
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

pub fn check_user_and_api(args: &SettingsArgs, config: deserializer::Config) {
    serialize(
        if args.user.is_some() {
            args.user.clone().unwrap()
        } else {
            config.user.clone()
        },
        if args.apikey.is_some() {
            args.apikey.clone().unwrap()
        } else {
            config.api_key.clone()
        },
        if args.unlist.is_some() {
            args.unlist.unwrap()
        } else {
            config.unlist
        },
        "".to_string()
            + if args.user.is_some() { "User" } else { "" }
            + if args.apikey.is_some() {
                if args.user.is_some() {
                    ", api"
                } else {
                    "Api"
                }
            } else {
                ""
            }
            + if args.user.is_some() || args.apikey.is_some() {
                " and unlist"
            } else {
                "Unlist"
            },
    );
}
