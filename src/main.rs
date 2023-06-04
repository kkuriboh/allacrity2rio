use std::{
    fs::{create_dir, read_to_string, File},
    io::{BufWriter, Write},
};

use types::AlacrittyConf;

mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dir = std::fs::read_dir("alacritty-theme/themes")?;
    let _ = create_dir("themes");

    while let Some(Ok(entry)) = dir.next() {
        let entry_path = entry.path();
        let conf: AlacrittyConf = serde_yaml::from_str(&read_to_string(&entry_path)?)?;

        let mut f = BufWriter::new(File::create(format!(
            "themes/{}.toml",
            entry_path.file_stem().unwrap().to_str().unwrap()
        ))?);
        f.write_all(b"[colors]\n")?;

        for (key, value) in conf.colors {
            let key = match key.as_str() {
                "bright" => "light-".to_string(),
                "primary" | "normal" | "cursor" => "".to_string(),
                _ => key + "-",
            };

            match value {
                types::ColorsOrIndexed::Colors(colors) => colors.into_iter().for_each(|color| {
                    f.write_fmt(format_args!(
                        "{key}{} = \"{}\"\n",
                        color.0.replace("_", "-").replace("bright", "light"),
                        color.1.replace("0x", "#")
                    ))
                    .unwrap()
                }),
                // idk what any those other options are
                _ => {}
            }
        }
        f.flush()?;
    }

    Ok(())
}
