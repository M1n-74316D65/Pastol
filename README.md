# [Paste.lol](https://paste.lol/) Unofficial Command Line Interface (CLI)

[Pastol](https://reply.cards/hskmnxkfpv) enables users to interact with Paste.lol service directly from the command line. Pastol allows you to share text and files quickly and conveniently from the CL.

## Usage

```sh
pastol [OPTIONS]
```

## Options

- -f, --file <FILE>: Upload a file or update an existing file on the pastebin.
- -t, --title <TITLE>: Title of the new pastebin or the title of the pastebin to update.
- -c, --content <CONTENT>: Content of the new pastebin or the content of the pastebin to update.
- -d, --download <DOWNLOAD>: WIP Download the content of a pastebin.
- -i, --info <INFO>: Get detailed information about a pastebin.
- -r, --remove <REMOVE>: Remove a pastebin from the pastebin service.
- -l, --list: List all the publicly listed pastebins.
- --setuser <SETUSER>: Set your username for the pastebin service.
- --setapikey <SETAPIKEY>: Set your API key for the pastebin service.
- --setunlist <SETUNLIST>: Set to true if you want newly created pastebins to be unlisted by default. (Default: false) [possible values: true, false].
- -h, --help: Print help.
- -V, --version: Print version.

## Install

1.  Install rust and cargo.

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2.  If on **Linux** install:

    - Arch Linux

      ```sh
      sudo pacman -S pkg-config openssl
      ```

    - Debian and Ubuntu

      ```sh
      sudo apt-get install pkg-config libssl-dev
      ```

    - Fedora

      ```sh
      sudo dnf install pkg-config perl-FindBin openssl-devel
      ```

    - Alpine Linux

      ```sh
      apk add pkgconfig openssl-dev
      ```

    - openSUSE

      ```sh
      sudo zypper in libopenssl-devel
      ```

3.  Install the crate.
    ```sh
    cargo install pastol
    ```

## Setup

1.  Set user and API key.
    ```sh
    pastol --setuser your_username --setapikey your_api_key
    ```
2.  Exampe as adam.
    ```sh
    pastel --setuser adam --setapikey a321dwageaawdwadw
    ```

- OPTIONAL: Set all the new or updated pastebin to unlisted/hidden.
  ```sh
  pastol --setunlist true
  ```

## Examples

- Upload a file.

  ```sh
  pastol -f example.txt
  ```

- Download a pastebin as a file.

  ```sh
  pastol -d example-title-as-apears-on-the-url
  ```

- Upload a file with custom title.

  ```sh
  pastol -f example.txt -t "Example Title"
  ```

- Upload a file with custom content.

  ```sh
  pastol -f example.txt -c "This is the content of the example file."
  ```

- Upload a custom.

  ```sh
  pastol -t title-example -c "pastebin content example"
  ```

- Remove a pastebin.

  ```sh
  pastol -r hello-world
  ```

- List all listed(non hiden) pastebins.

  ```sh
  pastol -l
  ```

- Get all the info of a pastebin.

  ```sh
  pastol -i example
  ```

## Build

1. Install cargo

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repo

   ```sh
   git clone https://github.com/M1n-74316D65/Pastol
   ```

3. If on **Linux** install:

   - Arch Linux

     ```sh
     sudo pacman -S pkg-config openssl
     ```

   - Debian and Ubuntu

     ```sh
     sudo apt-get install pkg-config libssl-dev
     ```

   - Fedora

     ```sh
     sudo dnf install pkg-config perl-FindBin openssl-devel
     ```

   - Alpine Linux

     ```sh
     apk add pkgconfig openssl-dev
     ```

   - openSUSE

     ```sh
     sudo zypper in libopenssl-devel
     ```

4. Build

   - Using [just](https://just.systems/). (use this pls)
     1. Install just if not installed
     2. check the justfile to see available commands.
   - Using cargo
     1. ```sh
        cargo build
        ```
     2. This is the path of the pastol executable:
        ```sh
        ./target/debug/pastol
        ```

For more info check the [justfile](https://github.com/M1n-74316D65/Pastol/blob/master/justfile) or use just.
