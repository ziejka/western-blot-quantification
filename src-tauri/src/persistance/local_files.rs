use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use crate::models::sample::Sample;

fn get_dir(file_name: &str) -> Result<PathBuf, &str> {
    let app_support_dir =
        dirs::config_dir().ok_or("Could not find Application Support directory")?;
    let app_dir = app_support_dir.join("WesternBlotCalculatorApp");
    fs::create_dir_all(&app_dir).map_err(|_| "Can't create file")?;

    Ok(app_dir.join(file_name))
}

pub fn save_samples_to_file(samples: &Vec<Sample>) -> Result<(), String> {
    let path = get_dir("samples.msgpack")?;
    let data = rmp_serde::to_vec(samples).map_err(|e| e.to_string())?;
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn save_document_names(document_names: &HashSet<String>) -> Result<(), String> {
    let path = get_dir("document_names.msgpack")?;
    let data = rmp_serde::to_vec(document_names).map_err(|e| e.to_string())?;
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn read_document_names() -> Result<HashSet<String>, String> {
    let path = get_dir("document_names.msgpack")?;
    let file = File::open(path).map_err(|e| e.to_string())?;
    let document_names: HashSet<String> = rmp_serde::from_read(file).map_err(|e| e.to_string())?;
    Ok(document_names)
}

pub fn read_samples() -> Result<Vec<Sample>, String> {
    let path = get_dir("samples.msgpack")?;
    let file = File::open(path).map_err(|e| e.to_string())?;
    let samples: Vec<Sample> = rmp_serde::from_read(file).map_err(|e| e.to_string())?;
    Ok(samples)
}
