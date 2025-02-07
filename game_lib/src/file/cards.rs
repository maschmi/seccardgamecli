use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json;

use crate::cards::types::card_model::{Card, CardTrait};
use crate::file::general::{count_cards_in_directory, ensure_directory_exists};

pub fn write_card_to_file(card: &Card, base_path: Option<&str>) -> std::io::Result<()> {
    let card_directory = get_card_directory(card);
    let mut path = if let Some(base) = base_path {
        PathBuf::from(base)
    } else {
        PathBuf::new()
    };

    path.push(card_directory);

    match ensure_directory_exists(path.to_str().unwrap().trim()) {
        Ok(_) => (),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Could not create directory {}", card_directory),
            ))
        }
    }
    let current_cards_count = match count_cards_in_directory(path.to_str().unwrap().trim()) {
        Ok(card_number) => card_number,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not count files in directory",
            ))
        }
    };

    let file_name = generate_filename(card.title().value(), current_cards_count);

    path.push(file_name);
    write_data_to_file(card, &path)?;

    println!("Wrote to file {}", path.display());
    Ok(())
}

fn generate_filename(title: &str, current_cards_count: u32) -> String {
    let new_card_number = current_cards_count + 1;
    let padded_prefix = format!("{:0>4}", new_card_number);
    let sanitized_title = sanitize_filename(title);
    let shortened_title: String = sanitized_title.chars().take(60).collect();
    format!("{}-{}.json", padded_prefix, shortened_title)
}

pub fn write_data_to_file(card: &Card, path: &Path) -> std::io::Result<()> {
    match serde_json::to_string_pretty(&card) {
        Ok(serialized_card) => {
            let mut file = File::create(path)?;
            file.write_all(serialized_card.as_bytes())
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to serialize data",
        )),
    }
}

pub fn get_card_directory(card: &Card) -> &'static str {
    match card {
        Card::Event(_) => "events",
        Card::Attack(_) => "attacks",
        Card::Oopsie(_) => "oopsies",
        Card::Lucky(_) => "lucky",
        Card::Evaluation(_) => panic!("Evaluation cards should not be written to file"),
    }
}

fn replace_invalid_character(c: char) -> char {
    match c.is_alphanumeric() || c == '.' || c == '-' {
        true => c,
        false => '_',
    }
}

fn sanitize_filename(filename: &str) -> String {
    filename.chars().map(replace_invalid_character).collect()
}
