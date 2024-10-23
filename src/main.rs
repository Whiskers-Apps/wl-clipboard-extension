//REQUIRED FOR WINDOWS EXTENSIONS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actions::on_run_commands;
use clipboard::init_clipboard;
use results::on_get_results;
use whiskers_launcher_core::features::extensions::ExtensionRequestType::{GetResults, RunCommand};
use whiskers_launcher_core::features::core::extensions::get_extension_request;

mod actions;
mod clipboard;
mod results;
mod icons;

pub const ID: &str = "whiskers-apps/clipboard";

fn main() {
    init_clipboard();

    let request = get_extension_request();

    match request.request_type {
        GetResults => {
            on_get_results(request);
        },
        RunCommand => {
            on_run_commands(request);
        },
    }
}


