use whiskers_launcher_rs::api::extensions::get_extension_dir;

use crate::ID;

pub fn get_icon(name: impl Into<String>) -> String {
    let name = name.into();
    let mut path = get_extension_dir(ID).unwrap();

    path.push("src/icons");
    path.push(format!("{}.svg", name));
    path.into_os_string().into_string().unwrap()
}
