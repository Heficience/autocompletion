extern crate getopts;
extern crate env_logger;
extern crate libc;
extern crate reqwest;
#[macro_use]
extern crate log;

mod input;

use input::{is_key_event, is_key_press, is_key_release, is_shift, get_key_text, InputEvent, KEY_NAMES};

use std::process::{exit, Command};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{env, mem};
use std::io;
use std::fs;
use getopts::Options;
use serde::Deserialize;

use std::path::Path;
use csv;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Config {
    device_file: String,
    log_file: String
}

impl Config {
    fn new(device_file: String, log_file: String) -> Self {
        Config { device_file: device_file, log_file: log_file }
    }
}

#[derive(Debug,Deserialize)]
struct Record {
    ortho: String,
    freqlemfilms2: f64,
    freqlemlivres: f64,
    freqfilms2: f64,
    freqlivres: f64,
}


fn main() {
    root_check();

    env_logger::init();

    parse_args();
    println!("{}", VERSION);
    // if dataset is not downloaded, download it
    if dataset_downloaded() == false {
        println!("Downloading dataset...");
        download_file("https://github.com/Heficience/autocompletion/raw/master/Lexique383.csv", "./Lexique383.csv");
    }


    let dataset = load_dataset();
    println!("Dataset loaded");
    println!("dataset (first 5 words) : {:?}", dataset.get(0..5));

    // TODO: use the sizeof function (not available yet) instead of hard-coding 24.
    let buf: [u8; 24] = unsafe { mem::zeroed() };

    let mut shift_pressed = 0;
    loop {
  
        let event: InputEvent = unsafe { mem::transmute(buf) };
        if is_key_event(event.type_) {
            if is_key_press(event.value) {
                if is_shift(event.code) {
                    shift_pressed += 1;
                }

                let text = get_key_text(event.code, shift_pressed).to_string();

                // Ici on pourra commencer a predire le texte 
                // On pourra utiliser le texte pour faire des predictions
                
                let predictions = search_partial_dataset(&text, &dataset);
                println!("{:?}", predictions);
                println!("{:?}",KEY_NAMES[event.code as usize]);
               
            } else if is_key_release(event.value) {
                if is_shift(event.code) {
                    shift_pressed -= 1;
                }
            }
        }
    }
}



fn root_check() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    }
}

fn parse_args() {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");


    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!("{}", e));
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        exit(0);
    }


}

fn get_default_device() -> String {
    let mut filenames = get_keyboard_device_filenames();
    debug!("Detected devices: {:?}", filenames);

    if filenames.len() == 1 {
        filenames.swap_remove(0)
    } else {
        panic!("The following keyboard devices were detected: {:?}. Please select one using \
                the `-d` flag", filenames);
    }
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames
}

fn load_dataset() -> Vec<(String, String)> {
    let mut dataset = Vec::new();
    let mut reader = csv::Reader::from_path("./Lexique383.csv").unwrap();   
    
    for record in reader.deserialize() {
        let record: Record = record.unwrap();
        println!("{:?}", record);
        // create moyenne frequency with for 1-4 attributs (float)
        let moyenne_frequency = 0;

        dataset.push((record.ortho, moyenne_frequency.to_string()));
    }
    dataset
}

fn search_partial_dataset(partial_word: &str, dataset: &Vec<(String, String)>) -> Vec<String> {
    let mut result = Vec::new();

    for (word, _) in dataset {
        if word.starts_with(partial_word) {
            result.push(word.clone());
        }
    }

    // short by moyenne frequency (column 2)
    result.sort_by(|a, b| {
        let a_moyenne = a.split(",").nth(2).unwrap().parse::<f32>().unwrap();
        let b_moyenne = b.split(",").nth(2).unwrap().parse::<f32>().unwrap();
        a_moyenne.partial_cmp(&b_moyenne).unwrap()
    });

    result

}

fn dataset_downloaded() -> bool{
    // check if the folder Lexique383 exist
    if Path::new("Lexique383.csv").exists() {
        return true;
    }else{
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

