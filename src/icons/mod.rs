use std::path::PathBuf;

use whiskers_launcher_core::features::extensions::get_extension_dir;

use crate::ID;

pub fn get_icon(name: impl Into<String>) -> PathBuf {
    let name = name.into();
    let mut path = get_extension_dir(ID).unwrap();

    path.push("src/icons");
    path.push(format!("{}.svg", name));
    path
}
