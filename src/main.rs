use clap::Parser;

mod deserializer;
mod file_reader;
mod petition_manager;
mod petitions;
mod serializer;
mod write_markdown;

/// Paste.lol on the command line.
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

    /// Download the content of a pastebin .
    #[structopt(short, long)]
    download: Option<String>,

    /// Get detailed information about a pastebin.
    #[structopt(short, long)]
    info: Option<String>,

    /// Remove a pastebin from the pastebin service.
    #[structopt(short, long)]
    remove: Option<String>,

    /// List all pastebins.
    #[structopt(short, long, default_value = "false")]
    list: bool,

    /// Set your username for the pastebin service.
    #[structopt(long)]
    setuser: Option<String>,

    /// Set your API key for the pastebin service.
    #[structopt(long)]
    setapikey: Option<String>,

    /// If true unlisted by default. (Default: false)
    #[structopt(long)]
    setunlist: Option<bool>,
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
            "".to_string()
                + if args.setuser.is_some() {
                    "User"
                } else if args.setapikey.is_some() {
                    if args.setuser.is_some() {
                        ", api"
                    } else {
                        "Api"
                    }
                } else {
                    if args.setuser.is_some() || args.setapikey.is_some() {
                        " and unlist"
                    } else {
                        "Unlist"
                    }
                },
        );
    } else {
        petition_manager::petition_manager(args, config);
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
