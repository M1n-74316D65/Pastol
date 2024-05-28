use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};
use std::env;

mod config_manager;
mod deserializer;
mod file_reader;
mod petition_manager;
mod petitions;
mod serializer;
mod write_markdown;

/// Create or update a pastebin on paste.lol.
#[derive(Parser, Debug, Clone)]
struct AddArgs {
    /// Upload a file or update an existing file.
    #[arg(value_enum)]
    file: Option<String>,

    /// Title of the new pastebin or the title of the pastebin to update.
    #[structopt(short, long)]
    title: Option<String>,

    /// Content of the new pastebin or the content of the pastebin to update.
    #[structopt(short, long)]
    content: Option<String>,
}

/// || rm - Remove a pastebin on paste.lol.
#[derive(Parser, Debug, Clone)]
struct RemoveArgs {
    title: String,
}

///  || dl - Download by title a pastebin.
#[derive(Parser, Debug, Clone)]
struct DownloadArgs {
    title: String,
}

/// || ls - List all pastebins.
#[derive(Parser, Debug, Clone)]
struct ListArgs {}

/// || cat - View by title the pastebin.
#[derive(Parser, Debug, Clone)]
struct ViewArgs {
    title: String,
}

/// || find - Search by title for pastebins.
#[derive(Parser, Debug, Clone)]
struct SearchArgs {
    title: String,
}

/// || config - Change the settings.
#[derive(Parser, Debug, Clone)]
struct SettingsArgs {
    ///  Set your username for paste.lol.
    #[structopt(long)]
    user: Option<String>,

    ///  Set your API key for paste.lol.
    #[structopt(long)]
    apikey: Option<String>,

    ///  Set to true if you want newly created pastebins to be unlisted by default.
    #[structopt(long)]
    unlist: Option<bool>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum CliSub {
    Add(AddArgs),
    Remove(RemoveArgs),
    Download(DownloadArgs),
    List(ListArgs),
    View(ViewArgs),
    Search(SearchArgs),
    Settings(SettingsArgs),
}

impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("add", args)) => Ok(Self::Add(AddArgs::from_arg_matches(args)?)),
            Some(("remove", args)) => Ok(Self::Remove(RemoveArgs::from_arg_matches(args)?)),
            Some(("download", args)) => Ok(Self::Download(DownloadArgs::from_arg_matches(args)?)),
            Some(("list", args)) => Ok(Self::List(ListArgs::from_arg_matches(args)?)),
            Some(("view", args)) => Ok(Self::View(ViewArgs::from_arg_matches(args)?)),
            Some(("search", args)) => Ok(Self::Search(SearchArgs::from_arg_matches(args)?)),
            Some(("settings", args)) => Ok(Self::Settings(SettingsArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Invalid subcommands",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Invalid subcommands",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("add", args)) => *self = Self::Add(AddArgs::from_arg_matches(args)?),
            Some(("remove", args)) => *self = Self::Remove(RemoveArgs::from_arg_matches(args)?),
            Some(("download", args)) => {
                *self = Self::Download(DownloadArgs::from_arg_matches(args)?)
            }
            Some(("list", args)) => *self = Self::List(ListArgs::from_arg_matches(args)?),
            Some(("view", args)) => *self = Self::View(ViewArgs::from_arg_matches(args)?),
            Some(("search", args)) => *self = Self::Search(SearchArgs::from_arg_matches(args)?),
            Some(("settings", args)) => {
                *self = Self::Settings(SettingsArgs::from_arg_matches(args)?)
            }
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Invalid subcommands",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for CliSub {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(RemoveArgs::augment_args(Command::new("remove").alias("rm")))
            .subcommand(DownloadArgs::augment_args(
                Command::new("download").alias("dl"),
            ))
            .subcommand(ListArgs::augment_args(Command::new("list").alias("ls")))
            .subcommand(ViewArgs::augment_args(Command::new("view").alias("cat")))
            .subcommand(SearchArgs::augment_args(
                Command::new("search").alias("find"),
            ))
            .subcommand(SettingsArgs::augment_args(
                Command::new("settings").alias("config"),
            ))
            .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(RemoveArgs::augment_args(Command::new("rm").alias("remove")))
            .subcommand(DownloadArgs::augment_args(
                Command::new("download").alias("dl"),
            ))
            .subcommand(ListArgs::augment_args(Command::new("list").alias("ls")))
            .subcommand(ViewArgs::augment_args(Command::new("view").alias("cat")))
            .subcommand(SearchArgs::augment_args(
                Command::new("search").alias("find"),
            ))
            .subcommand(SettingsArgs::augment_args(
                Command::new("settings").alias("config"),
            ))
            .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(
            name,
            "add"
                | "remove"
                | "rm"
                | "download"
                | "dl"
                | "list"
                | "ls"
                | "view"
                | "cat"
                | "search"
                | "find"
                | "settings"
                | "config"
        )
    }
}

/// Paste.lol on the command line.
#[derive(Parser, Debug, Clone)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<CliSub>,
}

fn main() {
    let args = Cli::parse();
    let result = deserializer::deserialized();
    match result {
        Ok(mut config) => match &args.subcommand {
            Some(CliSub::Add(args)) => {
                if let Some(val) = env::var("api_omg_lol").ok() {
                    config.api_key = val;
                }
                petition_manager::add(args, config);
            }
            Some(CliSub::Search(args)) => {
                if let Some(val) = env::var("api_omg_lol").ok() {
                    config.api_key = val;
                }
                petition_manager::search(args, config);
            }
            Some(CliSub::Download(args)) => {
                if let Some(val) = env::var("api_omg_lol").ok() {
                    config.api_key = val;
                }
                petition_manager::download(args, config);
            }
            Some(CliSub::View(args)) => {
                if let Some(val) = env::var("api_omg_lol").ok() {
                    config.api_key = val;
                }
                petition_manager::view(args, config);
            }
            Some(CliSub::Remove(args)) => {
                if let Some(val) = env::var("api_omg_lol").ok() {
                    config.api_key = val;
                }
                petition_manager::remove(args, config);
            }
            Some(CliSub::List(_)) => {
                if let Some(val) = env::var("api_omg_lol").ok() {
                    config.api_key = val;
                }
                petition_manager::list(config);
            }
            Some(CliSub::Settings(args)) => {
                config_manager::check_user_and_api(args, config);
            }
            None => println!("No command provided"),
        },

        // First run
        Err(_e) => {
            config_manager::serialize(
                "rust".to_string(),
                "rocks".to_string(),
                false,
                "First run!".to_string(),
            );
            let result = deserializer::deserialized();
            match result {
                Ok(config) => match &args.subcommand {
                    Some(CliSub::Settings(args)) => {
                        config_manager::check_user_and_api(args, config);
                    }
                    None => println!("No command provided"),
                    _ => println!("No command provided"),
                },

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
        let unlist = false;
        let title = "test";
        let content = "This is a test.";
        petitions::create(
            user.to_string(),
            api_key.to_string(),
            unlist,
            title.to_string(),
            content.to_string(),
        )
        .unwrap();
    }
}
