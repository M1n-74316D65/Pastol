# [Paste.lol](https://paste.lol/) Unofficial Command Line Interface (CLI)

Pastol enables users to interact with Paste.lol service directly from the command line. Pastol allows you to share text and files quickly and conveniently from the CL.

## Usage

```sh
pastol [OPTIONS] <COMMAND>
```

### Commands:

```sh
add       Create or update a pastebin on the pastebin service
remove    || rm - Remove a pastebin on the pastebin service
download  || dl - Download a pastebin
list      || ls - List all pastebins
view      || cat - View the pastebin
search    || find - Search by title for pastebins
help      Print this message or the help of the given subcommand(s)
```

### Options:

```sh
--setuser <SETUSER>      Set your username for the pastebin service
--setapikey <SETAPIKEY>  Set your API key for the pastebin service
--setunlist <SETUNLIST>  If true unlisted by default.
-h, --help               Print help

```

## Install

1. Install rust and cargo.

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. Install pastol.

   - With [binstall](https://github.com/cargo-bins/cargo-binstall) (better)

     1. Install [binstall](https://github.com/cargo-bins/cargo-binstall)

        #### Linux and macOS

        ```
        curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
        ```

        #### Windows

        ```
        Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content
        ```

     2. Install pastol

        ```sh
        cargo binstall pastol
        ```

   - Without [binstall](https://github.com/cargo-bins/cargo-binstall)

     1. If on **Linux** or **maybe macOS**(feedback needed):

        - macOS (Homebrew)

          ```sh
          brew install openssl@3
          ```

        - macOS (MacPorts)

          ```sh
          sudo port install openssl
          ```

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

     2. Install the crate.

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

    Your API key is stored locally in the config file.
    Linux example path:

    ```sh
    .config/pastol/config.toml
    ```

- OPTIONAL: Set all the new or updated pastebin to unlisted/hidden.
  ```sh
  pastol --setunlist true
  ```

## Examples

- Upload a file.

  ```sh
  pastol example.txt
  ```

- Download a pastebin as a file.

  ```sh
  pastol download example-title-as-apears-on-the-url
  ```

- Upload a file with custom title.

  ```sh
  pastol add example.txt -t "Example Title"
  ```

- Upload a file with custom content.

  ```sh
  pastol add example.txt -c "This is the content of the example file."
  ```

- Upload a custom.

  ```sh
  pastol add -t title-example -c "pastebin content example"
  ```

- Remove a pastebin.

  ```sh
  pastol remove hello-world
  ```

- List all listed pastebins.

  ```sh
  pastol list
  ```

- View the pastebin.

  ```sh
  pastol view example
  ```

- Search by title for pastebins.

  ```sh
  pastol search exa
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

3. If on **Linux** install or **maybe macos**():

   - macOS (Homebrew)

     ```sh
     brew install openssl@3
     ```

   - macOS (MacPorts)

     ```sh
     sudo port install openssl
     ```

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

##### [LOL](https://reply.cards/hskmnxkfpv)
