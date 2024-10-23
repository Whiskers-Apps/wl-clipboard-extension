use std::process::exit;

use whiskers_launcher_core::features::{
    core::extensions::get_form_response, extensions::ExtensionRequest,
};

use crate::clipboard::{get_clipboard, write_clipboard, ImageClip, TextClip};

pub fn on_run_commands(request: ExtensionRequest) {
    let command = request.command.unwrap();
    let mut clipboard = get_clipboard();

    if command == "add-text" {
        let response = get_form_response();

        let name = response.get_result("name").unwrap().field_value;
        let text = response.get_result("text").unwrap().field_value;

        let clip = TextClip::new(name, text);
        clipboard.text_clips.push(clip);

        write_clipboard(&clipboard);
        exit(0)
    }

    if command == "add-image" {
        let response = get_form_response();

        let name = response.get_result("name").unwrap().field_value;
        let path = response.get_result("path").unwrap().field_value;

        let clip = ImageClip::new(name, path);
        clipboard.image_clips.push(clip);

        write_clipboard(&clipboard);
        exit(0)
    }

    if command == "edit-text-clip" {
        let response = get_form_response();

        let name = response.get_result("name").unwrap().field_value;
        let text = response.get_result("text").unwrap().field_value;
        let args = response.args;
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

    if command == "delete-text-clip" {
        let args = request.args;
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

    if command == "edit-image-clip" {
        let response = get_form_response();

        let name = response.get_result("name").unwrap().field_value;
        let path = response.get_result("path").unwrap().field_value;
        let args = response.args;
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

    if command == "delete-image-clip" {
        let args = request.args;
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
