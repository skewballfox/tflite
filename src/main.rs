mod tflite_generated;
use std::{fs, path::PathBuf};

use crate::tflite_generated::tflite::Model;

fn main() {
    let mut model_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    model_file_path.push("test_data/hey_mycroft.tflite");
    //get the model from file under projectroot/test_data
    let model_file = &*fs::read(model_file_path).unwrap();

    // /let model = Model::(&mut builder, model_file);
    let table = unsafe { flatbuffers::Table::new(&model_file, 28) };
    let model = unsafe { Model::init_from_table(table) };
    println!("Model version: {:?}", model);
}
