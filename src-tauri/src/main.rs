// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashSet, sync::Mutex};

use handlers::western_blot::*;
use models::app_state::AppState;
use persistance::local_files::{read_document_names, read_samples};

mod handlers;
mod models;
mod persistance;

fn main() {
    let document_names = read_document_names().unwrap_or(HashSet::new());
    let samples = read_samples().unwrap_or(Vec::new());

    tauri::Builder::default()
        .manage(AppState {
            samples: Mutex::new(samples),
            document_names: Mutex::new(document_names),
        })
        .invoke_handler(tauri::generate_handler![
            add_sample_data,
            get_samples_by_title,
            calculate,
            get_documents_names,
            add_document,
            get_sample_data,
            update_sample_data,
            delete_document,
            save_csv,
            delete_sample,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
