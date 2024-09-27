use std::{
    fs,
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Clipboard {
    pub text_clips: Vec<TextClip>,
    pub image_clips: Vec<ImageClip>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextClip {
    pub id: usize,
    pub name: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageClip {
    pub id: usize,
    pub name: String,
    pub path: PathBuf,
}

impl TextClip {
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        let name = name.into();
        let text = text.into();

        Self {
            id: get_new_text_clip_id(),
            name,
            text,
        }
    }

    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = name;
        self.to_owned()
    }

    pub fn set_text(&mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.text = text;
        self.to_owned()
    }
}

impl ImageClip {
    pub fn new(name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        let name = name.into();
        let path = path.into();

        Self {
            id: get_new_image_clip_id(),
            name,
            path,
        }
    }

    pub fn set_name(&mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = name;
        self.to_owned()
    }

    pub fn set_path(&mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        self.path = path;
        self.to_owned()
    }
}

pub fn get_clipboard_dir_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();
    path.push("wl-clipboard");
    path
}

pub fn get_clipboard_path() -> PathBuf {
    let mut path = get_clipboard_dir_path();
    path.push("clipboard.bin");
    path
}

/// Creates a directory and an empty clipboard if it doesn't exists
pub fn init_clipboard() {
    let path = get_clipboard_dir_path();

    if !path.exists() {
        fs::create_dir_all(&path).expect("Error creating directory");

        let clipboard = Clipboard {
            text_clips: Vec::new(),
            image_clips: Vec::new(),
        };

        write_clipboard(&clipboard);
    }
}

pub fn get_clipboard() -> Clipboard {
    let path = get_clipboard_path();
    let bytes = fs::read(&path).expect("Error reading clipbaord");
    let clipboard = bincode::deserialize(&bytes).expect("Error deserializing clipboard");
    clipboard
}

pub fn write_clipboard(clipboard: &Clipboard) {
    let path = get_clipboard_path();
    let bytes = bincode::serialize(clipboard).expect("Error serializing clipboard");
    fs::write(&path, &bytes).expect("Error writing clipboard");
}

pub fn get_new_text_clip_id() -> usize {
    let clips = get_clipboard().text_clips.clone();

    return if clips.is_empty() {
        0
    } else {
        let biggest_id = clips.iter().max_by_key(|clip| clip.id).unwrap().id;
        biggest_id + 1
    };
}

pub fn get_new_image_clip_id() -> usize {
    let clips = get_clipboard().image_clips.clone();

    return if clips.is_empty() {
        0
    } else {
        let biggest_id = clips.iter().max_by_key(|clip| clip.id).unwrap().id;
        biggest_id + 1
    };
}
