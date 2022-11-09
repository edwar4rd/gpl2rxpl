use std::path::Path;

pub struct Config {
    pub filename: Option<String>,
    pub help: bool,
    // pub silent: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        let mut filename = None;
        // let mut silent = false;
        for i in &args[1..] {
            if i == "--help" {
                return Result::Ok(Config {
                    filename: None,
                    help: true,
                    // silent: false,
                });
            // } else if i == "--silent" {
                // silent = true;
            } else {
                if !Path::new(i).exists() {
                    return Result::Err(format!("Given filepath {} do not exist", i));
                }
                filename = Some(i.to_string());
            }
        }

        Result::Ok(Config {
            filename: filename,
            help: false,
            // silent,
        })
    }
}
