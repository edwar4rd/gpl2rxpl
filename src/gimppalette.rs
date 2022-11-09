use crate::Color3u8;

pub struct GIMPPalette {
    name: String,
    columns: u32,
    colors: Vec<(Color3u8, Option<String>)>,
}

impl GIMPPalette {
    pub fn from_str(src: &str) -> Result<GIMPPalette, String> {
        assert!(src.starts_with("GIMP Palette\n"));
        let mut name = "Untitled";
        let mut columns = 1;
        let mut colors = Vec::new();

        for &line in &src.lines().collect::<Vec<&str>>()[1..] {
            if line != "GIMP Palette" && line != "" && !line.starts_with('#') {
                if line.starts_with("Name: ") {
                    name = line.strip_prefix("Name: ").unwrap();
                } else if line.starts_with("Columns: ") {
                    columns = line
                        .strip_prefix("Columns: ")
                        .unwrap()
                        .parse::<u32>()
                        .map_err(|err| {
                            format!("Error parsing u32 for column:\n\t\t{}", err.to_string())
                        })?;
                } else {
                    let components = line
                        .split(char::is_whitespace)
                        .filter(|&x| x != "")
                        .map(|x| x.trim())
                        .collect::<Vec<&str>>();
                    if components.len() >=3 {
                        let color = (
                            components[0].parse::<u8>().map_err(|err| {
                                format!("Error parsing u8 for color[0]:\n\t\t{}", err.to_string())
                            })?,
                            components[1].parse::<u8>().map_err(|err| {
                                format!("Error parsing u8 for color[1]:\n\t\t{}", err.to_string())
                            })?,
                            components[2].parse::<u8>().map_err(|err| {
                                format!("Error parsing u8 for color[2]:\n\t\t{}", err.to_string())
                            })?,
                        );

                        if components.len() >= 4 && components[3] != "Untitled" {
                            colors.push((color, Some(components[3..].join(" "))));
                        } else {
                            colors.push((color, None));
                        }
                    } else {
                        return Err(
                            "Line is having too many space separated components".to_string()
                        );
                    }
                }
            }
        }

        Ok(GIMPPalette {
            name: name.to_string(),
            columns,
            colors,
        })
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_column(&self) -> u32 {
        return self.columns;
    }

    pub fn to_vec(self) -> Vec<(Color3u8, Option<String>)> {
        self.colors
    }
}
