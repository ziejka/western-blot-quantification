use std::{collections::HashSet, fs, path::PathBuf};

use serde::Deserialize;

use crate::{
    models::{app_state::AppState, sample::Sample},
    persistance::{
        csv::Csv,
        local_files::{save_document_names, save_samples_to_file},
    },
};

#[derive(Deserialize, Debug)]
pub struct SampleData {
    pub membrane_title: String,
    pub name: String,
    pub values: String,
    pub is_reference: bool,
}

#[tauri::command]
pub fn get_samples_by_title(state: tauri::State<AppState>, title: &str) -> Vec<Sample> {
    let samples = state.samples.lock().expect("lock poisoned");
    samples
        .iter()
        .filter(|sample| sample.document_title == title)
        .cloned()
        .collect()
}

#[tauri::command]
pub fn get_samples_names(state: tauri::State<AppState>) -> HashSet<String> {
    let samples = state.samples.lock().expect("lock poisoned");
    samples.iter().map(|s| s.name.clone()).collect()
}

#[tauri::command]
pub fn add_sample_data(
    state: tauri::State<AppState>,
    sample_data: SampleData,
) -> Result<(), String> {
    let mut sample: Sample = sample_data.try_into()?;
    sample.calculate_blank();

    let mut samples = state.samples.lock().expect("lock poisoned");
    samples.push(sample);
    save_samples_to_file(&samples)?;
    Ok(())
}

#[tauri::command]
pub fn delete_sample(state: tauri::State<AppState>, title: &str, name: &str) -> Result<(), String> {
    let mut samples = state.samples.lock().expect("lock poisoned");
    if let Some(pos) = samples
        .iter()
        .position(|s| s.document_title == title && s.name == name)
    {
        samples.remove(pos);
    }
    Ok(())
}

#[tauri::command]
pub fn update_sample_data(
    state: tauri::State<AppState>,
    sample_data: SampleData,
) -> Result<(), String> {
    let mut sample: Sample = sample_data.try_into()?;
    sample.calculate_blank();

    let mut samples = state.samples.lock().map_err(|e| e.to_string())?;
    let index = samples
        .iter_mut()
        .position(|s| s.name == sample.name && s.document_title == sample.document_title)
        .ok_or("Sample for update not found")?;

    samples[index] = sample;
    save_samples_to_file(&samples)?;
    Ok(())
}

#[tauri::command]
pub fn get_sample_data(
    state: tauri::State<AppState>,
    title: String,
    name: String,
) -> Result<Sample, String> {
    state
        .samples
        .lock()
        .expect("lock poisoned")
        .iter()
        .find(|s| s.name == name && s.document_title == title)
        .ok_or("Sample not found".into())
        .cloned()
}

#[tauri::command]
pub fn calculate(state: tauri::State<AppState>, title: String) -> Result<(), String> {
    let mut samples_lock = state.samples.lock().expect("lock poisoned");
    let mut document_samples = samples_lock
        .iter_mut()
        .filter(|s| s.document_title == title)
        .collect::<Vec<_>>();

    let reference_index = document_samples
        .iter()
        .position(|s| s.is_reference)
        .ok_or("no reference found")?;

    let reference_sample = document_samples
        .get_mut(reference_index)
        .ok_or("Reference sample not found")?
        .clone();

    for (i, sample) in document_samples.iter_mut().enumerate() {
        if i == reference_index {
            continue;
        };
        sample.calculate_norm_by(&reference_sample);
        sample.calculate_normalized();
    }

    Ok(())
}

#[tauri::command]
pub fn add_document(state: tauri::State<AppState>, name: String) -> Result<(), String> {
    if name.is_empty() {
        return Err("Document needs a title".to_string());
    }
    let mut document_names = state.document_names.lock().expect("lock poisoned");
    document_names.insert(name);

    save_document_names(&document_names)
}

#[tauri::command]
pub fn delete_document(state: tauri::State<AppState>, title: &str) {
    let mut document_names = state.document_names.lock().expect("lock poisoned");
    document_names.remove(title);

    state
        .samples
        .lock()
        .expect("lock poisoned")
        .retain(|s| s.document_title != title);

    save_document_names(&document_names).unwrap_or_default();
}

#[tauri::command]
pub fn save_csv(state: tauri::State<AppState>, title: &str, path_str: &str) -> Result<(), String> {
    if title.is_empty() || path_str.is_empty() {
        return Err("Document needs a title".to_string());
    }

    let mut path_buf = PathBuf::from(path_str);
    match path_buf.extension() {
        Some(extension) if extension != "csv" => {
            path_buf.set_extension("csv");
        }
        None => {
            path_buf.set_extension("csv");
        }
        _ => {}
    };

    let samples = state.samples.lock().expect("lock poisoned");
    let document_samples = samples
        .iter()
        .filter(|sample| sample.document_title == title)
        .collect::<Vec<_>>();

    if document_samples.is_empty() {
        return Err("No samples for document found".to_string());
    }

    let csv: Csv = document_samples
        .try_into()
        .map_err(|_| "Failed to convert samples to CSV")?;

    fs::write(path_buf, csv.0).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_documents_names(state: tauri::State<AppState>) -> HashSet<String> {
    state.document_names.lock().expect("lock poisoned").clone()
}
