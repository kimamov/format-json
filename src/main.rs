use serde_json::Value;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let json_source_path = &args[1];
    if args.len() < 3 {
        panic!("not enough arguments, need source as first argument and destination path as second")
    }
    if json_source_path.is_empty() {
        panic!("first argument has to be path to json file")
    }
    let json_destination_path = &args[2];
    if json_destination_path.is_empty() {
        panic!("second argument needs to be the path where to store the formatted json")
    }

    println!("reading json, {}", json_source_path);

    let file = fs::read_to_string(json_source_path).expect("failed to read file");
    let src: Value = serde_json::from_str(&file).expect("could not parse json");

    let dest = serde_json::to_string_pretty(&src).expect("could not serialize json");

    fs::write(json_destination_path, dest).expect("could not write json file");

    println!(
        "successfully formatted json and save it at path {}",
        json_destination_path
    );
}
