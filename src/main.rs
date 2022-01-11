use std::env;
use std::process::Command;

use csv;
use device_query::{DeviceQuery, DeviceState};
use enigo::{Enigo, KeyboardControllable};
use std::path::Path;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const KEYBOARD: &'static str = "azerty"; // can be azerty or qwerty
const ADD_NEW_WORDS: bool = false; // false by default for security reasons (this add all word to the dictionary locally)
fn main() {
    logo();

    // if dataset is not downloaded, download it
    if dataset_downloaded() == false {
        println!("Downloading dataset...");
        download_file(
            "https://github.com/Heficience/autocompletion/raw/master/dataset/Lexique383.csv",
            "./dataset.csv",
        );
    }

    let mut dataset = load_dataset();
    println!("Dataset loaded");
    println!("dataset count {}", dataset.len());
    // spawn auto_save_database
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    let mut word = String::new();
    let mut client_enigo = Enigo::new();

    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys && !keys.is_empty() {
            //println!("{:?}", keys);

            // if space is pressed, clear the word [Space]
            if keys[0].to_string() == "Space" || keys[0].to_string() == "Enter" {
                // add the word to the dataset or update it
                if word.len() > 3 {
                    dataset = add_to_dataset(&word, &dataset);
                }
                word.clear();
            }
            // if backspace is pressed, remove the last char [Backspace]
            else if keys[0].to_string() == "Backspace" {
                if !word.is_empty() {
                    word.pop();
                }
            }
            // if a letter is pressed, add it to the word
            let k = format_to_good_keyboard(&keys[0].to_string().to_lowercase());
            if "abcdefghijklmnopqrqstuvwxyz".contains(&k) {
                word.push(k.chars().nth(0).unwrap());
                //println!("{}\n", word);
            }
            // if LControl + space is pressed, search the word in the dataset
            if keys.len() > 1 && word.len() > 2 {
                if keys[0].to_string() == "LControl" && keys[1].to_string() == "Space" {
                    let result = search_partial_dataset(word.as_str(), &dataset, &1);
                    if result.len() > 0 {
                        println!(
                            "Results : {}",
                            result[0].0.split(",").collect::<Vec<&str>>()[0]
                        );
                        // remove firsts caracters already typed
                        let mut word_to_type = result[0].0.to_string();
                        for _ in 0..word.len() {
                            word_to_type.remove(0);
                        }
                        println!("{}", word_to_type);
                        // wait for key release
                        while !device_state.get_keys().is_empty() {}
                        client_enigo.key_sequence(&word_to_type);

                        save_dataset(&dataset);
                    }
                }
            }
        }
        prev_keys = keys;
    }
}

fn format_to_good_keyboard(key: &str) -> String {
    let mut k = key.to_string();
    if KEYBOARD == "azerty" {
        k = match key {
            "q" => "a".to_string(),
            "a" => "q".to_string(),
            "w" => "z".to_string(),
            "z" => "w".to_string(),
            "m" => ",".to_string(),
            "semicolon" => "m".to_string(),
            ":" => "m".to_string(),
            ";" => "m".to_string(),
            "," => "m".to_string(),
            k => k.to_string(),
        }
    } // else, already in qwerty
    k
}



fn add_to_dataset(word: &str, dataset: &Vec<(String, f64)>) -> Vec<(String, f64)> {
    let mut new_dataset = dataset.clone();

    let mut found = false;
    for i in 0..new_dataset.len() {
        if new_dataset[i].0 == word {
            new_dataset[i].1 += 0.1;
            found = true;
            break;
        }
    }
    if found == false && ADD_NEW_WORDS {
        new_dataset.push((word.to_string(), 1.0));
    }
    new_dataset
}
fn save_dataset(dataset: &Vec<(String, f64)>) {
    println!("Saving dataset...");
    let mut file = csv::Writer::from_path("./dataset.csv").unwrap();
    for i in 0..dataset.len() {
        file.write_record([&dataset[i].0, &dataset[i].1.to_string()])
            .unwrap();
    }
    println!("Dataset saved");
}

fn load_dataset() -> Vec<(String, f64)> {
    let mut dataset = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_path("./dataset.csv")
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
    // check if the folder dataset exist
    if Path::new("dataset.csv").exists() {
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

fn logo() {
    println!("-----------------------------------------------------");
    println!(" - Bienvenue sur le programme d'auto-completion de mot");
    println!(" - Ce programme est en version alpha, il est encore en développement");
    println!(" - La version actuelle est {}", VERSION);
    println!(" - Ce programme est développé et maintenu par : @andronedev avec l'equipe Heficience (heficience.com)");
    println!(" - Github : https://github.com/Heficience/autocompletion");
    println!("-----------------------------------------------------");

}
