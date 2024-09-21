use std::{fs, path::PathBuf, process::Command};

use freedesktop_desktop_entry::DesktopEntry;
use walkdir::WalkDir;

use crate::paths::{file_matches_icon, get_backup_dirs, get_icon_pack_path};

#[derive(Debug, Clone)]
pub struct IconFetcher {
    /// Name of the icon pack
    icon_pack: Option<String>,
    /// Path of the icon pack
    icon_pack_path: Option<PathBuf>,
    /// Directories where icons should be if they are not on the icon pack
    backup_dirs: Vec<PathBuf>,
    /// Returns the actual path in case of a symlink
    return_target_path: bool,
}

fn get_system_icon_pack() -> String {
    let icon_pack_command = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("icon-theme")
        .output()
        .expect("Error getting current icon theme.");

    let icon_pack = String::from_utf8(icon_pack_command.stdout)
        .unwrap_or("hicolor".to_string())
        .replace("'", "")
        .replace("\n", "");

    icon_pack
}

fn get_target_path(path: impl Into<PathBuf>) -> Option<PathBuf> {
    let path: PathBuf = path.into();

    if !path.is_symlink() {
        return Some(path);
    }

    return if let Ok(link) = path.read_link() {
        return if link.is_relative() {
            Some(path.join(link))
        } else {
            Some(link)
        };
    } else {
        None
    };
}

impl IconFetcher {
    /// Inits the IconFetcher with the system icon pack
    pub fn new() -> Self {
        let icon_pack = get_system_icon_pack();
        let icon_pack_path = get_icon_pack_path(&icon_pack);

        Self {
            icon_pack: Some(icon_pack),
            icon_pack_path: if let Some(icon_pack_path) = icon_pack_path {
                Some(icon_pack_path)
            } else {
                None
            },
            backup_dirs: get_backup_dirs(),
            return_target_path: false,
        }
    }

    /// Checks if the icon pack actually exists and sets the icon pack path accordingly
    pub fn set_icon_pack(&mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        let path = get_icon_pack_path(&name);

        self.icon_pack = Some(name);
        self.icon_pack_path = if let Some(p) = path { Some(p) } else { None };

        self.clone()
    }

    /// Returns the target path in case the icon path is a symlink
    pub fn set_return_target_path(&mut self, return_target_path: bool) -> Self {
        self.return_target_path = return_target_path;
        self.clone()
    }

    /// Returns the icon path given it's name.
    pub fn get_icon_path(self, icon_name: impl Into<String>) -> Option<PathBuf> {
        let icon_name: String = icon_name.into();

        if let Some(icon_pack_path) = self.icon_pack_path {
            for entry in WalkDir::new(&icon_pack_path).follow_links(true) {
                if let Ok(entry) = entry {
                    if file_matches_icon(entry.path(), &icon_name) {
                        return if self.return_target_path {
                            get_target_path(entry.into_path())
                        } else {
                            Some(entry.into_path())
                        };
                    }
                }
            }
        }

        for dir in self.backup_dirs {
            for entry in WalkDir::new(dir).follow_links(true) {
                if let Ok(entry) = entry {
                    if file_matches_icon(entry.path(), &icon_name) {
                        return if self.return_target_path {
                            get_target_path(entry.into_path())
                        } else {
                            Some(entry.into_path())
                        };
                    }
                }
            }
        }

        None
    }

    /// Gets the icon path given the desktop file path
    pub fn get_icon_path_from_desktop(self, path: impl Into<PathBuf>) -> Option<PathBuf> {
        let path = path.into();

        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                if let Some(icon) = entry.icon() {
                    let icon_path = PathBuf::from(&icon);

                    if icon_path.exists() && icon_path.is_file() {
                        return Some(icon_path);
                    } else {
                        return self.get_icon_path(icon);
                    }
                }
            }
        }

        None
    }
}
