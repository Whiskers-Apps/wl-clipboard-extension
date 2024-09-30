use std::{
    env,
    process::{exit, Command},
};

use whiskers_launcher_rs::api::extensions::{
    get_dialog_response, DialogResponse, ExtensionRequest,
};

use crate::clipboard::{get_clipboard, write_clipboard, ImageClip, TextClip};

pub fn run_actions(request: ExtensionRequest) {
    let action = request.extension_action.unwrap();
    let mut clipboard = get_clipboard();

    if action == "add-text" {
        let response = get_dialog_response();

        if !has_valid_results(vec!["name", "text"], response.clone()) {
            send_error_message();
        }

        let name = response.clone().get_result_value("name").unwrap();
        let text = response.clone().get_result_value("text").unwrap();

        let clip = TextClip::new(name, text);
        clipboard.text_clips.push(clip);

        write_clipboard(&clipboard);
        exit(0)
    }

    if action == "add-image" {
        let response = get_dialog_response();

        if !has_valid_results(vec!["name", "path"], response.clone()) {
            send_error_message();
        }

        let name = response.clone().get_result_value("name").unwrap();
        let path = response.clone().get_result_value("path").unwrap();

        let clip = ImageClip::new(name, path);
        clipboard.image_clips.push(clip);

        write_clipboard(&clipboard);
        exit(0)
    }

    if action == "copy-image" {
        let args = request.args.clone().unwrap();
        let id: usize = args.get(0).unwrap().parse().unwrap();
        let clip: ImageClip = clipboard
            .image_clips
            .iter()
            .find(|c| c.id == id)
            .unwrap()
            .to_owned();

        let clip_path_str = clip.path.into_os_string().into_string().unwrap();

        if is_wayland() {
            if cfg!(target_os = "linux") {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("cat {} | wl-copy --type image/png", &clip_path_str))
                    .spawn()
                    .expect("Error copying image");
            }
        }

        if cfg!(target_os = "windows") {
            let script = r#"
Add-Type -AssemblyName System.Windows.Forms
[System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile("%path%"))"#
                .replace("%path%", &clip_path_str);

            powershell_script::run(&script).expect("Error executing copy command");
        }

        exit(0)
    }

    if action == "edit-text-clip" {
        let response = get_dialog_response();

        if !has_valid_results(vec!["name", "text"], response.clone()) {
            send_error_message();
        }

        let name = response.clone().get_result_value("name").unwrap();
        let text = response.clone().get_result_value("text").unwrap();
        let args = response.args.unwrap();
        let id: usize = args.get(0).unwrap().parse().unwrap();
        let mut new_text_clips = Vec::<TextClip>::new();

        for clip in clipboard.clone().text_clips {
            new_text_clips.push(if clip.id == id {
                clip.to_owned().set_name(&name).set_text(&text)
            } else {
                clip
            });
        }

        let mut clipboard_mod = clipboard.clone();
        clipboard_mod.text_clips = new_text_clips;

        write_clipboard(&clipboard_mod);

        exit(0)
    }

    if action == "delete-text-clip" {
        let args = request.args.clone().unwrap();
        let id: usize = args.get(0).unwrap().parse().unwrap();
        let mut clipboard_mod = clipboard.clone();
        clipboard_mod.text_clips = clipboard
            .text_clips
            .iter()
            .map(|clip| clip.to_owned())
            .filter(|clip| clip.id != id)
            .collect();

        write_clipboard(&clipboard_mod);

        exit(0)
    }

    if action == "edit-image-clip" {
        let response = get_dialog_response();

        if !has_valid_results(vec!["name", "path"], response.clone()) {
            send_error_message();
        }

        let name = response.clone().get_result_value("name").unwrap();
        let path = response.clone().get_result_value("path").unwrap();
        let args = response.args.unwrap();
        let id: usize = args.get(0).unwrap().parse().unwrap();
        let mut image_clips_mod = Vec::<ImageClip>::new();

        for clip in clipboard.clone().image_clips {
            image_clips_mod.push(if clip.id == id {
                clip.to_owned().set_name(&name).set_path(&path)
            } else {
                clip
            });
        }

        let mut clipboard_mod = clipboard.clone();
        clipboard_mod.image_clips = image_clips_mod;

        write_clipboard(&clipboard_mod);

        exit(0)
    }

    if action == "delete-image-clip" {
        let args = request.args.clone().unwrap();
        let id: usize = args.get(0).unwrap().parse().unwrap();
        let mut clipboard_mod = clipboard.clone();
        clipboard_mod.image_clips = clipboard
            .image_clips
            .iter()
            .map(|clip| clip.to_owned())
            .filter(|clip| clip.id != id)
            .collect();

        write_clipboard(&clipboard_mod);

        exit(0)
    }
}

pub fn is_wayland() -> bool {
    if env::consts::OS != "linux" {
        return false;
    }

    match env::var("XDG_SESSION_TYPE") {
        Ok(session) => &session.to_lowercase() == "wayland",
        Err(_) => false,
    }
}

fn has_valid_results(fields: Vec<&str>, response: DialogResponse) -> bool {
    for field in fields {
        let result = response.clone().get_result_value(field);

        if result.is_none() {
            return false;
        }

        if result.unwrap().trim().is_empty() {
            return false;
        }
    }

    return true;
}

fn send_error_message() {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "notify-send \"Form Error\" \"⚠️ Invalid fields. Action was canceled\""
        ))
        .spawn()
        .expect("Error sending notification");

    exit(1);
}
