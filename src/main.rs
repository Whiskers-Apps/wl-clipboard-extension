//REQUIRED FOR WINDOWS EXTENSIONS
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use actions::run_actions;
use clipboard::init_clipboard;
use results::show_results;
use whiskers_launcher_rs::api::extensions::get_extension_request;

mod actions;
mod clipboard;
mod results;
mod icons;

pub const ID: &str = "whiskers-apps/clipboard";

fn main() {
    // cat gengar.jpg | wl-copy --type image/png
    
    init_clipboard();

    let request = get_extension_request();

    match request.action_context {
        whiskers_launcher_rs::api::extensions::ActionContext::ResultsRequest => {
            show_results(request.clone());
        }
        whiskers_launcher_rs::api::extensions::ActionContext::RunAction => {
            run_actions(request.clone());
        }
    }
}


