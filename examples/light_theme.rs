use devicons::{icon_for_file, File, Theme};
use std::path::Path;

fn main() {
    // Create a new file instance for a directory "my_folder/"
    let path = Path::new("my_folder/");
    let file = File::new(path);

    // Get the icon for this directory with the light theme
    let icon = icon_for_file(&file, Some(Theme::Light));

    // Print its corresponding icon
    println!("Filename: {}", file.name);
    println!("Icon: {}", icon.icon);
    println!("Color: {}", icon.color);
}
