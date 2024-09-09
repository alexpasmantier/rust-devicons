use devicons::{icon_for_file, File, Theme};
use std::path::Path;

fn main() {
    // Create a new file instance for "example.txt"
    let path = Path::new("example.txt");
    let file = File::new(path);

    // Get the icon for this file with the dark theme
    let icon = icon_for_file(&file, Some(Theme::Dark));

    // Print its corresponding icon
    println!("Filename: {}", file.name);
    println!("Icon: {}", icon.icon);
    println!("Color: {}", icon.color);
}
