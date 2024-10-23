use std::path::PathBuf;

use crate::{clipboard::get_clipboard, icons::get_icon, ID};
use sniffer_rs::sniffer::Sniffer;
use whiskers_launcher_core::{
    features::extensions::{send_search_results, ExtensionRequest},
    results::{
        CopyImageAction, CopyTextAction, FormField, FormFilePickerField, FormInputField,
        OpenFormAction, ResultAction, RunExtensionAction, SearchResult, SearchResults,
    },
    utils::get_search_query,
};

pub fn on_get_results(request: ExtensionRequest) {
    let search_query = get_search_query(request.search_text.unwrap());
    let search_text = search_query.search_text;
    let keyword = search_query.keyword;
    let mut edit = false;
    let mut delete = false;
    let mut results = Vec::<SearchResult>::new();

    if let Some(keyword) = keyword {
        edit = keyword == "edit" || keyword == "e";
        delete = keyword == "delete" || keyword == "d";
    }

    if search_text.is_empty() {
        results.push(
            SearchResult::new(
                "Add text to clipboard",
                ResultAction::new_open_form_action(
                    OpenFormAction::new(
                        ID,
                        "add-text",
                        vec![
                            FormField::new_input_field(
                                "name",
                                FormInputField::new(
                                    "Name",
                                    "The name of the item to add to clipboard",
                                )
                                .set_not_empty_validation(),
                            ),
                            FormField::new_input_field(
                                "text",
                                FormInputField::new("Text", "The text to add to clipboard")
                                    .set_not_empty_validation(),
                            ),
                        ],
                    )
                    .set_title("Add text to clipboard")
                    .set_action_text("Add"),
                ),
            )
            .set_icon(get_icon("plus"))
            .set_accent_icon_tint(),
        );

        results.push(
            SearchResult::new(
                "Add image to clipboard",
                ResultAction::new_open_form_action(
                    OpenFormAction::new(
                        ID,
                        "add-image",
                        vec![
                            FormField::new_input_field(
                                "name",
                                FormInputField::new(
                                    "Name",
                                    "The name of the item to add to clipboard",
                                )
                                .set_not_empty_validation(),
                            ),
                            FormField::new_file_picker_field(
                                "path",
                                FormFilePickerField::new("Path", "the image file path")
                                    .set_image_file_types()
                                    .set_not_empty_validation(),
                            ),
                        ],
                    )
                    .set_title("Add image to clipboard")
                    .set_action_text("Add"),
                ),
            )
            .set_icon(get_icon("plus"))
            .set_accent_icon_tint(),
        );

        send_search_results(SearchResults::new_list_results(results.clone()));
    }

    let clipboard = get_clipboard();
    let sniffer = Sniffer::new();

    for clip in clipboard.text_clips {
        if sniffer.clone().matches(&clip.name, &search_text) {
            let result = if edit {
                SearchResult::new(
                    format!("Edit {}", &clip.name),
                    ResultAction::new_open_form_action(
                        OpenFormAction::new(
                            ID,
                            "edit-text-clip",
                            vec![
                                FormField::new_input_field(
                                    "name",
                                    FormInputField::new(
                                        "Name",
                                        "The name of the item to add to clipboard",
                                    )
                                    .set_text(&clip.name)
                                    .set_not_empty_validation(),
                                ),
                                FormField::new_input_field(
                                    "text",
                                    FormInputField::new("Text", "The text to add to clipboard")
                                        .set_text(&clip.text)
                                        .set_not_empty_validation(),
                                ),
                            ],
                        )
                        .set_title("Edit Text Clip")
                        .set_action_text("Save")
                        .add_arg(clip.id.to_string()),
                    ),
                )
                .set_icon(get_icon("pencil"))
                .set_accent_icon_tint()
            } else if delete {
                SearchResult::new(
                    format!("Delete {}", &clip.name),
                    ResultAction::new_run_extension_action(
                        RunExtensionAction::new(ID, "delete-text-clip")
                            .add_arg(clip.id.to_string()),
                    )
                    .set_dangerous(true),
                )
                .set_description(&clip.name)
                .set_icon(get_icon("trash"))
                .set_accent_icon_tint()
            } else {
                SearchResult::new(
                    &clip.name,
                    ResultAction::new_copy_text_action(CopyTextAction::new(&clip.text)),
                )
                .set_description(&clip.text)
                .set_icon(get_icon("copy"))
                .set_accent_icon_tint()
            };

            results.push(result);
        }
    }

    for clip in clipboard.image_clips {
        if sniffer.clone().matches(&clip.name, &search_text) {
            let result = if edit {
                SearchResult::new(
                    format!("Edit {}", &clip.name),
                    ResultAction::new_open_form_action(
                        OpenFormAction::new(
                            ID,
                            "edit-image-clip",
                            vec![
                                FormField::new_input_field(
                                    "name",
                                    FormInputField::new(
                                        "Name",
                                        "The name of the item to add to clipboard",
                                    )
                                    .set_text(&clip.name)
                                    .set_not_empty_validation(),
                                ),
                                FormField::new_file_picker_field(
                                    "path",
                                    FormFilePickerField::new("Path", "The image path")
                                        .set_file_path(PathBuf::from(&clip.path))
                                        .set_image_file_types()
                                        .set_not_empty_validation(),
                                ),
                            ],
                        )
                        .add_arg(clip.id.to_string()),
                    ),
                )
                .set_description("Image")
                .set_icon(get_icon("pencil"))
                .set_accent_icon_tint()
            } else if delete {
                SearchResult::new(
                    format!("Delete {}", &clip.name),
                    ResultAction::new_run_extension_action(
                        RunExtensionAction::new(ID, "delete-image-clip")
                            .add_arg(clip.id.to_string()),
                    )
                    .set_dangerous(true),
                )
                .set_description("Image")
                .set_icon(get_icon("trash"))
                .set_accent_icon_tint()
            } else {
                SearchResult::new(
                    &clip.name,
                    ResultAction::new_copy_image_action(CopyImageAction::new(&clip.path)),
                )
                .set_description("Image")
                .set_icon(PathBuf::from(&clip.path))
            };

            results.push(result);
        }
    }

    send_search_results(SearchResults::new_list_results(results));
}
