mod args_reader;
mod file_handler;

extern crate core;

use std::{env, process};

use crate::args_reader::args_reader::read_args;
use crate::file_handler::read_txt_file::read_txt_file::{read_file};
use crate::file_handler::write_csv_file::write_csv_file::{get_template_values, write_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (file_path, sprint_id, squad) = read_args(&args);

    let content: String = read_file(file_path);
    let content_to_write = get_template_values(&content, sprint_id, squad);

    if let Err(err) = write_file(file_path, content_to_write.clone()){
        println!("error while writing file: {}", err);
        process::exit(1);
    }
}
