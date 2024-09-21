# 🦀 `rust-devicons`

`devicons` is a very simple and efficient Rust library inspired by [vim-devicons](https://github.com/ryanoasis/vim-devicons), providing functionality to retrieve filetype glyphs (icons) for a wide range of common file formats.


<img width="450" alt="Screenshot 2024-09-09 at 14 20 58" src="https://github.com/user-attachments/assets/f7fefd9e-df15-4536-ae5a-6e835ba4ce39">

## Features

- 🦞 **Icon Retrieval**: Get file or directory icons based on file name/extension.
- 🦞 **Icon Color**: Retrieve the color associated with the icon depending on the specified theme.
- 🦞 **Filetype Support**: Supports a wide range of filetypes and filename conventions (dockerfile, makefile, etc.).
- 🦞 **Customizable Themes**: Supports both light and dark themes.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
devicons = "0.1.0"
```

_**NOTE**: you'll need to use a [NerdFont](https://www.nerdfonts.com/) to properly display the icons._

## Usage

Here’s a simple example of how to use `devicons` to retrieve a file icon with the dark theme:

```rust
use std::path::Path;
use your_library::{File, Theme, icon_for_file};

fn main() {
    let path = Path::new("example.txt");
    let file = File::new(path);

    let icon = icon_for_file(&file, Some(Theme::Dark));

    println!("File: {}", file.name);
    println!("Icon: {}", icon.icon);
    println!("Color: {}", icon.color);
}
```

### Running the Examples

You can find more usage examples in the `examples` directory. To run them, use:

```bash
cargo run --example <example_name>
```

## License

This project is licensed under the [UNLICENSE license](LICENSE).

