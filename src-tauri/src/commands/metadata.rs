use crate::entities::paper;
use crate::AppState;
use pdf::{
    file::FileOptions,
    object::{FieldDictionary, FieldType, Resolve},
};
use sea_orm::ActiveValue;

#[tauri::command]
pub async fn import_metadata(
    state: tauri::State<'_, AppState>,
    file_path: String,
) -> Result<(), String> {
    let db = state
        .inner()
        .db
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("Database not connected")?;

    let mut paper_entity = paper::ActiveModel {
        ..Default::default()
    };
    let file = FileOptions::cached()
        .open(&file_path)
        .expect("failed to open pdf file");
    if let Some(ref info) = file.trailer.info_dict {
        info.iter()
            .filter(|(_, primitive)| primitive.to_string_lossy().is_ok())
            .for_each(|(key, value)| {
                eprintln!("{:>15}: {}", key, value.to_string_lossy().unwrap());
                match key.as_str() {
                    "Title" => {
                        paper_entity.title =
                            ActiveValue::set(clean_title(&value.to_string_lossy().unwrap()));
                    }
                    "Author" => {
                        paper_entity.authors =
                            ActiveValue::set(clean_authors(&value.to_string_lossy().unwrap()));
                    }
                    _ => {}
                }
            });
    }
    if let Some(ref forms) = file.get_root().forms {
        for field in forms.fields.iter() {
            print_field(field, &file);
        }
    }

    Ok(())
}

fn print_field(field: &FieldDictionary, resolve: &impl Resolve) {
    if field.typ == Some(FieldType::Signature) {
        println!("{:?}", field);
    }
    for &kid in field.kids.iter() {
        let child = resolve.get(kid).unwrap();
        print_field(&child, resolve);
    }
}

fn clean_title(title: &str) -> String {
    title
        .trim()
        .replace(".pdf", "")
        .replace(".PDF", "")
        .to_string()
}

fn clean_authors(authors: &str) -> String {
    authors
        .trim()
        .replace(";", ",")
        .replace(" and ", ",")
        .replace("And ", ",")
        .replace("AND ", ",")
        .to_string()
}
