use std::{collections::HashMap, path::Path};

#[derive(serde::Deserialize, Debug)]
struct Config {
    items: HashMap<String, Item>,
}

#[derive(serde::Deserialize, Debug)]
struct Item {
    paths: Vec<String>,

    #[serde(default)]
    ask: bool,
}

fn main() -> std::io::Result<()> {
    let xdg_paths = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?;
    let home_dir = std::env::var("HOME")
        .expect("Could not get $HOME environment variable. Are you on a unix system?");

    let cfg = std::fs::read_to_string(xdg_paths.get_config_file("config.toml"))?;
    let cfg: Config = toml::from_str(&cfg).unwrap();

    let mut files = Vec::new();
    for pattern in cfg.items.values().flat_map(|item| item.paths.iter()) {
        for path in glob::glob(&transform_glob(pattern, &home_dir)).unwrap() {
            if let Ok(path) = path {
                println!(" - {}", path.display());
                files.push(path);
            }
        }
    }

    if !inquire::Confirm::new("Remove files")
        .with_default(false)
        .prompt()
        .unwrap()
    {
        println!("Aborted");
        return Ok(());
    };

    for file in files {
        println!("RM {}", file.display());
        std::fs::remove_dir_all(&file).expect(&format!("Could not remove dir {}", file.display()));
    }

    println!("Done.");

    Ok(())
}

/// Replaces the `~` with the user home
fn transform_glob(path: &str, user_home: &str) -> String {
    if path.starts_with("~/") {
        return user_home.to_owned() + &path[1..];
    }
    return path.to_owned();
}
