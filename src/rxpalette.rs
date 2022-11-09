use crate::Color3u8;

pub struct RxPalette {
    title: String,
    colors: Vec<(Color3u8, Option<String>)>,
}

impl RxPalette {
    pub fn from_vec_with_title(
        colors: Vec<((u8, u8, u8), Option<String>)>,
        title: String,
    ) -> RxPalette {
        assert!(title.is_ascii());
        RxPalette { title, colors }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        let title_length = self.title.len();
        let title_wrap = std::iter::repeat("-")
            .take(title_length + 6)
            .collect::<String>();
        output.push_str(&title_wrap);
        output.push_str("\n-- ");
        output.push_str(&self.title);
        output.push_str(" --\n");
        output.push_str(&title_wrap);
        output.push_str("\n\np/clear\n");
        for c in &self.colors {
            output.push_str(&format!("#{:02x}{:02x}{:02x}", c.0 .0, c.0 .1, c.0 .2));
            match &c.1 {
                Some(color_name) => {
                    output.push_str(" -- ");
                    output.push_str(&color_name);
                }
                None => {}
            };
            output.push('\n');
        }

        output
    }
}
