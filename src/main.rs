use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

mod deserializer;
mod file_reader;
mod petition_manager;
mod petitions;
mod serializer;
mod write_markdown;

/// Create or update a pastebin on the pastebin service.
#[derive(Parser, Debug, Clone)]
struct AddArgs {
    /// Upload a file or update an existing file on the pastebin.\
    #[arg(value_enum)]
    file: Option<String>,

    /// Title of the new pastebin or the title of the pastebin to update.
    #[structopt(short, long)]
    title: Option<String>,

    /// Content of the new pastebin or the content of the pastebin to update.
    #[structopt(short, long)]
    content: Option<String>,
}

/// || rm - Remove a pastebin on the pastebin service.
#[derive(Parser, Debug, Clone)]
struct RemoveArgs {
    title: String,
}

///  || dl - Download a pastebin.
#[derive(Parser, Debug, Clone)]
struct DownloadArgs {
    title: String,
}

/// || ls - List all pastebins.
#[derive(Parser, Debug, Clone)]
struct ListArgs {}

/// || cat - View the pastebin.
#[derive(Parser, Debug, Clone)]
struct ViewArgs {
    title: String,
}

/// || find - Search by title for pastebins.
#[derive(Parser, Debug, Clone)]
struct SearchArgs {
    title: String,
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
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Valid subcommands are `add` and `remove`",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Valid subcommands are `add` and `remove`",
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
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Valid subcommands are `add` and `remove`",
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
        )
    }
}

/// Paste.lol on the command line.
#[derive(Parser, Debug, Clone)]
struct Cli {
    /// Set your username for the pastebin service.
    #[structopt(long)]
    setuser: Option<String>,

    /// Set your API key for the pastebin service.
    #[structopt(long)]
    setapikey: Option<String>,

    /// If true unlisted by default. (Default: false)
    #[structopt(long)]
    setunlist: Option<bool>,

    #[command(subcommand)]
    subcommand: CliSub,
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

fn check_user_and_api(args: Cli, config: deserializer::Config) {
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
            } else if args.setuser.is_some() || args.setapikey.is_some() {
                " and unlist"
            } else {
                "Unlist"
            },
    );
}

fn main() {
    let args = Cli::parse();
    let result = deserializer::deserialized();
    match result {
        Ok(config) => {
            if args.setuser.is_some() || args.setapikey.is_some() || args.setunlist.is_some() {
                check_user_and_api(args.clone(), config.clone());
            }
            match &args.subcommand {
                CliSub::Add(args) => {
                    petition_manager::add(args, config);
                }
                CliSub::Search(args) => {
                    petition_manager::search(args, config);
                }
                CliSub::Download(args) => {
                    petition_manager::download(args, config);
                }
                CliSub::View(args) => {
                    petition_manager::view(args, config);
                }
                CliSub::Remove(args) => {
                    petition_manager::remove(args, config);
                    println!("'myapp add' was used, name is: {args:?}")
                }
                CliSub::List(_) => {
                    petition_manager::list(config);
                }
            }
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
                    check_user_and_api(args.clone(), config);
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
