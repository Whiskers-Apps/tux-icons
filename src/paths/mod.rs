use std::{env, path::PathBuf};

use walkdir::WalkDir;

pub fn get_backup_dirs() -> Vec<PathBuf> {
    let home_dir = dirs::home_dir().expect("Error getting home directory");

    let mut local_hicolor = home_dir.clone();
    local_hicolor.push(".local/share/icons/hicolor");

    let mut local_pixmap = home_dir.clone();
    local_pixmap.push(".local/share/icons/pixmaps");

    let usr_hicolor = PathBuf::from("/usr/share/icons/hicolor");
    let usr_pixmap = PathBuf::from("/usr/share/pixmaps");

    let flatpak_apps = PathBuf::from("/var/lib/flatpak/app/");
    let home_flatpak_apps = home_dir.join(".local/share/flatpak");

    let mut dirs = Vec::<PathBuf>::new();

    if local_hicolor.exists() {
        dirs.push(local_hicolor);
    }

    if local_pixmap.exists() {
        dirs.push(local_pixmap);
    }

    if usr_hicolor.exists() {
        dirs.push(usr_hicolor);
    }

    if usr_pixmap.exists() {
        dirs.push(usr_pixmap);
    }

    if flatpak_apps.exists(){
        dirs.push(flatpak_apps);
    }

    if home_flatpak_apps.exists(){
        dirs.push(home_flatpak_apps);
    }

    dirs
}

fn add_if_exists(dirs: Vec<PathBuf>, path: impl Into<String>) -> Vec<PathBuf> {
    let path = path.into();
    let dir = PathBuf::from(&path);

    if dir.exists() && dir.is_dir() && !dirs.contains(&dir) {
        let mut dirs = dirs.to_owned();
        dirs.push(dir);
        return dirs;
    } else {
        dirs
    }
}

pub fn file_matches_icon(path: impl Into<PathBuf>, icon: impl Into<String>) -> bool {
    let path: PathBuf = path.into();
    let icon = icon.into();

    if path.exists() && path.is_file() {
        let stem = path.file_stem().unwrap();
        let stem_str: String = stem.to_string_lossy().into();

        return stem_str == icon;
    }

    false
}

fn get_icon_packs_dirs() -> Vec<PathBuf> {
    let xdg_dirs_var = env::var("XDG_DATA_DIRS").unwrap_or("".to_string());
    let xdg_dirs_split: Vec<&str> = xdg_dirs_var.split(":").collect();
    let mut xdg_dirs = Vec::<PathBuf>::new();

    for dir in xdg_dirs_split {
        let mut icon_dir = PathBuf::from(dir);
        icon_dir.push("icons");

        if icon_dir.exists() && icon_dir.is_dir() {
            xdg_dirs.push(icon_dir);
        }
    }

    xdg_dirs = add_if_exists(xdg_dirs, "/usr/share/icons");

    let home_dir = dirs::home_dir().expect("Error creating home directory");
    let home_dir_string = home_dir.into_os_string().into_string().unwrap();

    xdg_dirs = add_if_exists(xdg_dirs, format!("{}/.icons", home_dir_string));
    xdg_dirs = add_if_exists(xdg_dirs, format!("{}/.local/share/icons", home_dir_string));
    xdg_dirs = add_if_exists(
        xdg_dirs,
        format!("{}/.local/share/pixmaps", home_dir_string),
    );

    xdg_dirs = add_if_exists(xdg_dirs, "/var/lib/flatpak/exports/share/icons");

    xdg_dirs
}

pub fn get_icon_pack_path(name: impl Into<String>) -> Option<PathBuf> {
    let name = name.into();

    for dir in get_icon_packs_dirs() {
        for entry in WalkDir::new(dir) {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    let file_name = entry.file_name().to_os_string().into_string().unwrap();

                    if &name == &file_name {
                        return Some(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    None
}
