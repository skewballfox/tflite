mod tflite_generated;
use std::{fs, path::PathBuf};

use crate::tflite_generated::tflite::{Model, ENUM_VALUES_BUILTIN_OPERATOR};

fn main() {
    let mut model_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    model_file_path.push("test_data/hey_mycroft.tflite");
    //get the model from file under projectroot/test_data
    let model_file = &*fs::read(model_file_path).unwrap();

    // /let model = Model::(&mut builder, model_file);
    let table = unsafe { flatbuffers::Table::new(&model_file, 28) };
    let model = unsafe { Model::init_from_table(table) };
    //println!("Model Metadata: {:#?}", model.metadata());
    print_subgraphs(model);
    //print_ops(model);
    //println!("Model Buffers{:?}", model.buffers());
    //println!("Model : {:?}", model)
}

fn print_subgraphs(model: Model) {
    if let Some(subgraphs) = model.subgraphs() {
        for subgraph in subgraphs.iter() {
            println!("Subgraph: {:?}", subgraph.operators());
        }
        //println!("Subgraphs: {:#?}", subgraphs.iter().next().unwrap());
    }
}

fn print_ops(model: Model) {
    if let Some(operators) = model.operator_codes() {
        for operator in operators.iter() {
            println!(
                "Operator: {:#?}",
                ENUM_VALUES_BUILTIN_OPERATOR[operator.deprecated_builtin_code() as usize]
            );
        }
    }
}
