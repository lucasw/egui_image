/*
 * Lucas Walter
 * February 2021
 *
 * Load a csv file and display the contents as plots.
 * Watch the file for changes and reload it and redraw the plots, so another process
 * can be updating it or replacing it.
 *
 * Add sliders to control scale factors on each column plotted.
 *
 * Initially draw raw pixels to an image, later look into egui plotting capabilities.
 */

// use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
// use std::process;

pub fn load_csv(csv_file: File) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    // println!("loading '{}'", csv_file);

    let mut data: Vec<Vec<f64>> = Vec::new();

    let mut reader = csv::Reader::from_reader(csv_file);
    for (i, result) in reader.records().enumerate() {
        // TODO(lucasw) Use a fixed size array inside the outer Vec
        let record = result?;
        if i == 0 {
            data.resize(record.len(), Vec::new());
            println!("processing {} columns", data.len());
        }
        if data.len() != record.len() {
            println!("unexpected record len {} {}", data.len(), record.len());
            continue;
        }
        // println!("{:?}", record);
        for (j, val_str) in record.iter().enumerate() {
            // print!(" '{}' -> ", val_str);
            let val = val_str.trim_start_matches(' ').parse::<f64>().unwrap();
            // print!(" {},", val);
            data[j].push(val);
        }
        // println!("");
    }
    Ok(data)
}

pub fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    // println!("args {:?}", args);
    // load a csv file
    let filename;
    if args.len() > 1 {
        filename = args[1].clone();
    } else {
        filename = "data.csv".to_string();
    }
    println!("file '{}'", filename);
    filename
}

fn demo_load_csv() {
    let filename = get_filename();

    let path = Path::new(&filename);
    let csv_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(csv_file) => csv_file,
    };

    let columns = load_csv(csv_file).unwrap();

    for (i, column) in columns.iter().enumerate() {
        println!("{} {:?}", i, column);
    }
}
