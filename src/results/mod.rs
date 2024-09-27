use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_rs::{
    action::{
        Action, CopyAction, DialogAction, ExtensionAction, Field, FileFilter, FilePickerField,
        InputField,
    },
    api::extensions::{send_response, ExtensionRequest},
    result::{TextResult, TitleAndDescriptionResult, WLResult},
    utils::get_search,
};

use crate::{clipboard::get_clipboard, icons::get_icon, ID};

pub fn show_results(request: ExtensionRequest) {
    let search = get_search(request.search_text.unwrap());
    let search_text = search.search_text;
    let keyword = search.keyword;
    let mut edit = false;
    let mut delete = false;
    let mut results = Vec::<WLResult>::new();

    if let Some(keyword) = keyword {
        edit = keyword == "edit" || keyword == "e";
        delete = keyword == "delete" || keyword == "d";
    }

    if search_text.is_empty() {
        results.push(WLResult::new_text(
            TextResult::new(
                "Add text to clipboard",
                Action::new_dialog(DialogAction::new(
                    "whiskers-apps/clipboard",
                    "add-text",
                    "Add text to clipboard",
                    "Add",
                    vec![
                        Field::new_input(
                            "name",
                            InputField::new("", "Name", "The name of the item to add to clipboard"),
                        ),
                        Field::new_input(
                            "text",
                            InputField::new("", "Text", "The text to add to clipboard"),
                        ),
                    ],
                )),
            )
            .icon(get_icon("plus"))
            .tint("accent"),
        ));

        results.push(WLResult::new_text(
            TextResult::new(
                "Add image to clipboard",
                Action::new_dialog(DialogAction::new(
                    "whiskers-apps/clipboard",
                    "add-image",
                    "Add image to clipboard",
                    "Add",
                    vec![
                        Field::new_input(
                            "name",
                            InputField::new("", "Name", "The name of the item to add to clipboard"),
                        ),
                        Field::new_file_picker(
                            "path",
                            FilePickerField::new("Path", "the image file path").filters(vec![
                                FileFilter::new(
                                    "Supported Image Types",
                                    vec![
                                        "png".to_string(),
                                        "jpg".to_string(),
                                        "jpeg".to_string(),
                                        "webp".to_string(),
                                    ],
                                ),
                            ]),
                        ),
                    ],
                )),
            )
            .icon(get_icon("plus"))
            .tint("accent"),
        ));

        send_response(results.clone());
    }

    let clipboard = get_clipboard();
    let sniffer = Sniffer::new();

    for clip in clipboard.text_clips {
        if sniffer.clone().matches(&clip.name, &search_text) {
            let result = if edit {
                WLResult::new_title_and_description(
                    TitleAndDescriptionResult::new(
                        format!("Edit {}", &clip.name),
                        &clip.text,
                        Action::new_dialog(
                            DialogAction::new(
                                ID,
                                "edit-text-clip",
                                "Edit Text Clip",
                                "Save",
                                vec![
                                    Field::new_input(
                                        "name",
                                        InputField::new(
                                            &clip.name,
                                            "Name",
                                            "The name of the item to add to clipboard",
                                        ),
                                    ),
                                    Field::new_input(
                                        "text",
                                        InputField::new(
                                            &clip.text,
                                            "Text",
                                            "The text to add to clipboard",
                                        ),
                                    ),
                                ],
                            )
                            .args(vec![clip.id.to_string()]),
                        ),
                    )
                    .icon(get_icon("pencil"))
                    .tint("accent"),
                )
            } else if delete {
                WLResult::new_title_and_description(
                    TitleAndDescriptionResult::new(
                        format!("Delete {}", &clip.name),
                        &clip.text,
                        Action::new_extension(
                            ExtensionAction::new(ID, "delete-text-clip")
                                .args(vec![clip.id.to_string()]),
                        ),
                    )
                    .icon(get_icon("trash"))
                    .tint("accent"),
                )
            } else {
                WLResult::new_title_and_description(
                    TitleAndDescriptionResult::new(
                        &clip.name,
                        &clip.text,
                        Action::new_copy(CopyAction::new(&clip.text)),
                    )
                    .icon(get_icon("copy"))
                    .tint("accent"),
                )
            };

            results.push(result);
        }
    }

    for clip in clipboard.image_clips {
        if sniffer.clone().matches(&clip.name, &search_text) {
            let result = if edit {
                WLResult::new_title_and_description(
                    TitleAndDescriptionResult::new(
                        format!("Edit {}", &clip.name),
                        "Image",
                        Action::new_dialog(
                            DialogAction::new(
                                ID,
                                "edit-image-clip",
                                "Edit Image Clip",
                                "Save",
                                vec![
                                    Field::new_input(
                                        "name",
                                        InputField::new(
                                            &clip.name,
                                            "Name",
                                            "The name of the item to add to clipboard",
                                        ),
                                    ),
                                    Field::new_file_picker(
                                        "path",
                                        FilePickerField::new("Path", "the image file path")
                                            .filters(vec![FileFilter::new(
                                                "Supported Image Types",
                                                vec![
                                                    "png".to_string(),
                                                    "jpg".to_string(),
                                                    "jpeg".to_string(),
                                                    "webp".to_string(),
                                                ],
                                            )])
                                            .default_path(
                                                &clip.path.into_os_string().into_string().unwrap(),
                                            ),
                                    ),
                                ],
                            )
                            .args(vec![clip.id.to_string()]),
                        ),
                    )
                    .icon(get_icon("pencil"))
                    .tint("accent"),
                )
            } else if delete {
                WLResult::new_title_and_description(
                    TitleAndDescriptionResult::new(
                        format!("Delete {}", &clip.name),
                        "Image",
                        Action::new_extension(
                            ExtensionAction::new(ID, "delete-image-clip")
                                .args(vec![clip.id.to_string()]),
                        ),
                    )
                    .icon(get_icon("trash"))
                    .tint("accent"),
                )
            } else {
                WLResult::new_title_and_description(
                    TitleAndDescriptionResult::new(
                        &clip.name,
                        "Image",
                        Action::new_extension(
                            ExtensionAction::new(ID, "copy-image").args(vec![clip.id.to_string()]),
                        ),
                    )
                    .icon(clip.path.into_os_string().into_string().unwrap()),
                )
            };

            results.push(result);
        }
    }

    send_response(results);
}
