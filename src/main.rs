use clap::Parser;

use std::collections::HashMap;

#[tokio::main]
async fn create_unlisted(
    user: &str,
    api_key: &str,
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
    user: &str,
    api_key: &str,
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

/// Simple program to upload or update a pastebin to paste.lol
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File a file to pastebin
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
    #[arg(short, long)]
    user: Option<String>,

    /// Set for your paste.lol api key
    #[arg(short, long)]
    apikey: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.title.unwrap());
    // if args.listed {
    //     create_listed(
    //         user,
    //         api_key,
    //         args.title.to_string(),
    //         args.content.to_string(),
    //     )
    //     .unwrap();
    // } else {
    //     create_unlisted(
    //         user,
    //         api_key,
    //         args.title.to_string(),
    //         args.content.to_string(),
    //     )
    //     .unwrap();
    // }
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
        create_listed(user, api_key, title.to_string(), content.to_string()).unwrap();
    }
}
