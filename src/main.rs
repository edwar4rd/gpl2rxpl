use gpl2rxpl::Config;
use gpl2rxpl::GIMPPalette;
use gpl2rxpl::RxPalette;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => {
            print_args_help();
            eprintln!("\nError happened parsing args:\n\t{err}");
            return;
        }
    };

    if config.help {
        print_args_help();
        return;
    }

    let filename = match config.filename {
        Some(filename) => filename,
        None => {
            print_args_help();
            eprintln!("\nOne <file> operand is required for operation.");
            return;
        }
    };

    let palette = match GIMPPalette::from_str(&match fs::read_to_string(&filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed reading file: \n\t{}", err.to_string());
            return;
        }
    }) {
        Ok(palette) => palette,
        Err(err) => {
            eprintln!("Error happened parsing file: \n\t{}", err);
            return;
        }
    };

    let title = palette.get_name().clone();

    match fs::write(
        match config.output_path {
            Some(output_path) => output_path,
            None => format!(
                "{}{}",
                filename.trim_end_matches(".gpl").to_string(),
                ".palette"
            ),
        },
        RxPalette::from_vec_with_title(palette.to_vec(), title).to_string(),
    ) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed writing to file: \n\t{}", err.to_string());
            return;
        }
    }
}

fn print_args_help() {
    println!("gpl2rxpl <file [--silent] [--output path]| --help> ");
    println!("    file: one .gpl file path as input");
    println!("    --output: specify a file path for output");
    println!("    --help: print this message");
    println!("")
}
