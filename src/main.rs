extern crate getopts;
extern crate env_logger;
extern crate libc;

#[macro_use]
extern crate log;

mod input;

use input::{is_key_event, is_key_press, is_key_release, is_shift, get_key_text, InputEvent, KEY_NAMES};

use std::process::{exit, Command};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{env, mem};

use getopts::Options;

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

fn main() {
    root_check();

    env_logger::init();

    parse_args();


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

                let text = get_key_text(event.code, shift_pressed).as_bytes();

                // Ici on pourra commencer a predire le texte 
                // On pourra utiliser le texte pour faire des predictions
                
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
