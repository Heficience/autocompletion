extern crate reqwest;
use std::env;
use std::process::Command;

use device_query::{DeviceQuery, DeviceState};

use csv;
use enigo::{Enigo, KeyboardControllable};
use libc;
use std::path::Path;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    root_check();
    println!("{}", VERSION);
    // if dataset is not downloaded, download it
    if dataset_downloaded() == false {
        println!("Downloading dataset...");
        download_file(
            "https://github.com/Heficience/autocompletion/raw/master/Lexique383.csv",
            "./Lexique383.csv",
        );
    }

    let dataset = load_dataset();
    println!("Dataset loaded");
    println!("dataset (first 5 words) : {:?}", dataset.get(0..5));
    println!(
        "dataset search for 'salu' {:?}",
        search_partial_dataset("salu", &dataset, &10)
    );

    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    let mut word = String::new();
    let mut client_enigo = Enigo::new();

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys && !keys.is_empty() {
            // if space is pressed, clear the word [Space]
            if keys[0].to_string() == "Space" {
                word.clear();
            }
            // if backspace is pressed, remove the last char [Backspace]
            else if keys[0].to_string() == "Backspace" {
                if !word.is_empty() {
                    word.pop();
                }
            }
            // if a letter is pressed, add it to the word
            if "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(keys[0].to_string().as_str()) {
                word.push(keys[0].to_string().to_lowercase().chars().nth(0).unwrap());
                println!("{}", word);
            }
            // if LControl + space is pressed, search the word in the dataset
            if keys.len() > 1 && word.len() > 3 {
                println!("{:?}", keys);
                if keys[0].to_string() == "LControl" && keys[1].to_string() == "Space" {
                    let result = search_partial_dataset(word.as_str(), &dataset, &1);
                    println!("{:?}", result);
                    if result.len() > 0 {
                        println!("{}", result[0].0);
                        // remove firsts caracters already typed
                        let mut word_to_type = result[0].0.to_string();
                        for _ in 0..word.len() {
                            word_to_type.remove(0);
                        }
                        client_enigo.key_sequence(&word_to_type);
                    }
                }
            }
        }
        prev_keys = keys;
    }
}

fn root_check() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    }
}

fn load_dataset() -> Vec<(String, f64)> {
    let mut dataset = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./Lexique383.csv")
        .unwrap();
    for record in reader.records() {
        let record = record.unwrap();
        // println!("{:?}", record);
        let word = record[0].to_string();
        let freq = record[1].parse::<f64>().unwrap();
        dataset.push((word, freq));
    }
    dataset
}

fn search_partial_dataset(
    partial_word: &str,
    dataset: &Vec<(String, f64)>,
    limit: &i32,
) -> Vec<(String, f64)> {
    let mut result = Vec::new();

    for (word, freq) in dataset {
        if word.starts_with(partial_word) {
            result.push((word.clone(), freq.clone()));
        }
    }

    // shot frequence f64
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // remove duplicates
    result.dedup_by(|a, b| a.0 == b.0);
    // limit the result
    result.truncate(*limit as usize);

    result
}

fn dataset_downloaded() -> bool {
    // check if the folder Lexique383 exist
    if Path::new("Lexique383.csv").exists() {
        return true;
    } else {
        return false;
    }
}

fn download_file(url: &str, path: &str) {
    let wget_cmd = format!("wget -O {} {}", path, url);
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(wget_cmd);
    let output = cmd.output().expect("failed to execute process");
    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
