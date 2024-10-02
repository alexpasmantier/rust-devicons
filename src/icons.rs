use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

pub mod dark;
pub mod light;

pub struct File<'a> {
    path: &'a Path,
    pub name: &'a str,
    ext: Option<String>,
}

impl File<'_> {
    pub fn new<'a, P>(path: &'a P) -> File<'a>
    where
        P: AsRef<Path> + 'a + ?Sized,
    {
        let path: &Path = path.as_ref();
        let name = path.file_name().unwrap().to_str().unwrap();
        let ext = Self::ext(path);

        File { path, name, ext }
    }

    fn points_to_directory(&self) -> bool {
        self.path.display().to_string().ends_with('/')
    }

    fn ext(path: &Path) -> Option<String> {
        if let Some(ext) = path.extension() {
            return Some(ext.to_string_lossy().to_string());
        }
        let name = path.file_name().map(|f| f.to_string_lossy().to_string())?;

        name.rfind('.').map(|p| name[p + 1..].to_ascii_lowercase())
    }
}

impl<'a, P> From<&'a P> for File<'a>
where
    P: AsRef<Path> + 'a + ?Sized,
{
    fn from(path: &'a P) -> File<'a> {
        File::new(path)
    }
}

pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct FileIcon {
    pub icon: char,
    pub color: &'static str,
}

impl<'a, P> From<P> for FileIcon
where
    P: AsRef<Path> + 'a,
{
    fn from(path: P) -> FileIcon {
        let path = path.as_ref();
        icon_for_file(path, None)
    }
}

impl Display for FileIcon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

impl Default for FileIcon {
    fn default() -> Self {
        DEFAULT_FILE_ICON
    }
}

const DEFAULT_FILE_ICON: FileIcon = FileIcon {
    icon: '\u{f016}',
    color: "#7e8e91",
};

const DEFAULT_DIR_ICON: FileIcon = FileIcon {
    icon: '\u{f115}',
    color: "#7e8e91",
};

pub fn icon_for_file<'a, F>(file: F, theme: Option<Theme>) -> FileIcon
where
    F: Into<File<'a>>,
{
    let file = file.into();
    match theme {
        Some(Theme::Light) => light_icon_for_file(&file),
        Some(Theme::Dark) => dark_icon_for_file(&file),
        None => dark_icon_for_file(&file),
    }
}

fn dark_icon_for_file(file: &File<'_>) -> FileIcon {
    if let Some(icon) = dark::ICONS_MAP.get(file.name) {
        return icon.clone();
    } else if let Some(extension) = &file.ext {
        if let Some(icon) = dark::ICONS_MAP.get(extension.as_str()) {
            return icon.clone();
        } else if let Some(icon) = dark::ICONS_MAP.get(extension.to_lowercase().as_str()) {
            return icon.clone();
        } else {
            DEFAULT_FILE_ICON
        }
    } else if file.points_to_directory() {
        DEFAULT_DIR_ICON
    } else {
        DEFAULT_FILE_ICON
    }
}

fn light_icon_for_file(file: &File<'_>) -> FileIcon {
    if let Some(icon) = light::ICONS_MAP.get(file.name) {
        return icon.clone();
    } else if let Some(extension) = &file.ext {
        if let Some(icon) = light::ICONS_MAP.get(extension.as_str()) {
            return icon.clone();
        } else if let Some(icon) = light::ICONS_MAP.get(extension.to_lowercase().as_str()) {
            return icon.clone();
        } else {
            DEFAULT_FILE_ICON
        }
    } else if file.points_to_directory() {
        DEFAULT_DIR_ICON
    } else {
        DEFAULT_FILE_ICON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_file_new() {
        let path = Path::new("example.txt");
        let file = File::new(path);

        assert_eq!(file.name, "example.txt");
        assert_eq!(file.ext, Some("txt".to_string()));
    }

    #[test]
    fn test_file_new_directory() {
        let path = Path::new("some_directory/");
        let file = File::new(path);

        assert_eq!(file.name, "some_directory");
        assert_eq!(file.ext, None);
    }

    #[test]
    fn test_points_to_directory() {
        let dir_path = Path::new("some_directory/");
        let file = File::new(dir_path);

        assert!(file.points_to_directory());

        let file_path = Path::new("file.txt");
        let file = File::new(file_path);

        assert!(!file.points_to_directory());
    }

    #[test]
    fn test_file_extension() {
        let path = Path::new("file.txt");
        let ext = File::ext(path);
        assert_eq!(ext, Some("txt".to_string()));

        let no_ext_path = Path::new("file");
        let ext = File::ext(no_ext_path);
        assert_eq!(ext, None);
    }

    #[test]
    fn test_icon_for_file_with_light_theme() {
        let path = Path::new("file.txt");

        let icon = icon_for_file(path, Some(Theme::Light));
        assert_eq!(icon.icon, '󰈙');
        assert_eq!(icon.color, "#447028");
    }

    #[test]
    fn test_icon_for_file_with_dark_theme() {
        let path = Path::new("file.txt");
        let file = File::new(path);

        let icon = icon_for_file(file, Some(Theme::Dark));
        assert_eq!(icon.icon, '󰈙');
        assert_eq!(icon.color, "#89e051");
    }

    #[test]
    fn test_default_icon_for_directory() {
        let path = Path::new("some_directory/");
        let file = File::new(path);

        let icon = icon_for_file(file, Some(Theme::Dark));
        assert_eq!(icon.icon, '\u{f115}'); // Default directory icon
        assert_eq!(icon.color, "#7e8e91");
    }

    #[test]
    fn test_icon_for_file_with_no_theme() {
        let path = Path::new("file.txt");
        let file = File::new(path);

        let icon = icon_for_file(file, None); // Should default to Dark theme
        assert_eq!(icon.icon, '󰈙');
        assert_eq!(icon.color, "#89e051");
    }
}
