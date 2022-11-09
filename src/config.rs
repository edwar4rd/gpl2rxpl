use std::path::Path;

pub struct Config {
    pub filename: Option<String>,
    pub output_path: Option<String>,
    pub help: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut filename = None;
        let mut output_path = None;
        let mut output_isnext = false;
        for i in &args[1..] {
            if output_isnext {
                output_path = Some(if i.ends_with(".palette") {
                    i.to_string()
                } else {
                    format!("{}.palette", i)
                });
                output_isnext = false;
            } else if i == "--help" {
                return Result::Ok(Config {
                    filename: None,
                    output_path: None,
                    help: true,
                });
            } else if i == "--output" {
                output_isnext = true;
            } else {
                if !Path::new(i).exists() {
                    return Result::Err(format!("Given filepath {} do not exist", i));
                }
                filename = Some(i.to_string());
            }
        }

        Result::Ok(Config {
            filename,
            output_path,
            help: false,
        })
    }
}
