All the option that retrive something from the API are WIP.

# Paste.lol Unofficial Command Line Interface (CLI)

Pastol enables users to interact with Paste.lol service directly from the command line. Pastol allows you to share text and files quickly and conveniently from the CL.

## Usage

```sh
pastol [OPTIONS]
```

## Options

- -f, --file <FILE>: Upload a file or update an existing file on the pastebin.
- -t, --title <TITLE>: Title of the new pastebin or the title of the pastebin to update.
- -c, --content <CONTENT>: Content of the new pastebin or the content of the pastebin to update.
- -d, --download <DOWNLOAD>: WIP Download the content of a pastebin.
- -i, --info <INFO>: WIP Get detailed information about a pastebin.
- -r, --remove <REMOVE>: Remove a pastebin from the pastebin service.
- -l, --list: WIP List all the publicly listed pastebins.
- --setuser <SETUSER>: Set your username for the pastebin service.
- --setapikey <SETAPIKEY>: Set your API key for the pastebin service.
- --setunlist <SETUNLIST>: Set to true if you want newly created pastebins to be unlisted by default. (Default: false) [possible values: true, false].
- -h, --help: Print help.
- -V, --version: Print version.

## Setup

```sh
# Set user and API key
pastol --setuser your_username --setapikey your_api_key

# Exampe 2
pastel --setuser adam --setapikey a321dwageaawdwadw
```

## Example

```sh
# Upload a file
pastol -f example.txt

# Upload a file with custom title
pastol -f example.txt -t "Example Title"

# Upload a file with custom content
pastol -f example.txt -c "This is the content of the example file."

# Upload a custom
pastol -t title-example -c "pastebin content example"

# Remove a pastebin
pastol -r hello-world
```
