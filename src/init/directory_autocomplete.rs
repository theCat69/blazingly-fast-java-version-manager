use std::{error::Error, path::Path};

use inquire::Autocomplete;
use walkdir::{DirEntry, WalkDir};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Clone, Debug)]
pub struct InitAutocomplete {
    cur_input: String,
    mem: Vec<String>,
    cur_autocomplete_pos: u32,
    mem_autocomplete: Vec<String>,
}

impl Default for InitAutocomplete {
    fn default() -> Self {
        InitAutocomplete {
            cur_input: "".to_string(),
            mem: Vec::new(),
            cur_autocomplete_pos: 0,
            mem_autocomplete: Vec::new(),
        }
    }
}

impl Autocomplete for InitAutocomplete {
    fn get_suggestions(
        &mut self,
        input: &str,
    ) -> std::result::Result<Vec<String>, inquire::CustomUserError> {
        // should convert to CustomUserError here
        if input == "" {
            self.mem = initial_display().unwrap();
        }
        if exist(Path::new(input)) {
            self.mem = walk_dirs(input).unwrap();
            self.cur_autocomplete_pos = 0;
        }
        self.cur_input = input.to_string();
        Ok(self.mem.to_vec())
    }

    fn get_completion(
        &mut self,
        input: &str,
        _highlighted_suggestion: Option<String>,
    ) -> std::result::Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        let mem = self.mem.to_vec();
        self.mem_autocomplete = mem.into_iter().filter(|d| d.starts_with(input)).collect();
        match self.mem_autocomplete.len() {
            1 => Ok(Some(self.mem_autocomplete.get(0).unwrap().to_string())),
            _ => Ok(None),
        }
    }
}

fn walk_dirs(input: &str) -> std::result::Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let mut vec_to_keep: Vec<String> = Vec::new();
    for entry in WalkDir::new(input.trim())
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden_or_file(e))
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_str().unwrap();
        if !(path == input) {
            vec_to_keep.push(path.to_string());
        }
    }
    Ok(vec_to_keep)
}

fn exist(input_path: &Path) -> bool {
    match input_path.try_exists().ok() {
        Some(result) => result,
        None => false,
    }
}

#[cfg(target_family = "windows")]
fn initial_display() -> Result<Vec<String>> {
    let mut initial_display: Vec<String> = Vec::new();
    unsafe {
        let drive_mask = windows::Win32::Storage::FileSystem::GetLogicalDrives();
        if drive_mask != 0 {
            for index in 0..26 {
                if (drive_mask & (1 << index)) != 0 {
                    let drive_letter = (b'A' + index as u8) as char;
                    initial_display.push(drive_letter.to_string() + ":\\");
                }
            }
        }
    }
    Ok(initial_display)
}

fn is_hidden_or_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    entry
        .file_name()
        .to_str()
        .map(|s| path.is_file() || s.starts_with(".") || path.to_str().unwrap_or("$").contains("$"))
        .unwrap_or(true)
}
