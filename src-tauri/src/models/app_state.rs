use std::{collections::HashSet, sync::Mutex};

use serde::Serialize;

use super::sample::Sample;

#[derive(Serialize)]
pub struct AppState {
    pub samples: Mutex<Vec<Sample>>,
    pub document_names: Mutex<HashSet<String>>,
}
