# Thingy

Lil CLI helper tool to generate mdx for the [Figura Wiki](https://wiki.figuramc.org/).
Sorry some stuff is messy! Feel free to contribute :D

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install).

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Clone the repo

    ```sh
    git clone https://github.com/joeyleesi/thingy.git
    ```

3. Navigate into the projects directory and build

    ```sh
    cd thingy
    cargo build --release
    ```

4. All set! You can find the binary at `build/release/thingy`

### Usage

    ```
    Usage: thingy --input <FILE> <COMMAND>

    Commands:
      generate  Generate documentation for the given pages
      pages     List the possible pages to generate
      types     List the possible types used in the documentation (useful for creating link map)
      help      Print this message or the help of the given subcommand(s)

    Options:
      -i, --input <FILE>  Path to the exported_docs.json file
      -h, --help          Print help
      -V, --version       Print version
    ```

### Examples

- List all the possible pages that can be generated

    ```sh
    ./thingy -i path/to/your/exported_docs.json pages
    ```

- Generate a page

    ```sh
    ./thingy -i path/to/your/exported_docs.json generate -p Action
    ```
