use crate::{
    GrimoireCssError,
    transmutator::{TransmuteOptions, transmute_css, transmute_paths},
};
use std::{fs, path::PathBuf, time::Instant};

pub fn run_transmute_cli(args: Vec<String>) -> Result<(), GrimoireCssError> {
    let mut content = None;
    let mut paths = None;
    let mut output = None;
    let mut with_oneliner = false;
    let mut index = 2;

    while index < args.len() {
        match args[index].as_str() {
            "--content" | "-c" => content = Some(take_value(&args, &mut index, "--content")?),
            "--paths" | "-p" => paths = Some(take_value(&args, &mut index, "--paths")?),
            "--output" | "-o" => output = Some(take_value(&args, &mut index, "--output")?),
            "--with-oneliner" | "-l" => with_oneliner = true,
            unknown => {
                return Err(GrimoireCssError::InvalidInput(format!(
                    "Unknown transmute option: {unknown}"
                )));
            }
        }
        index += 1;
    }

    if content.is_some() == paths.is_some() {
        return Err(GrimoireCssError::InvalidInput(
            "transmute requires exactly one of --content or --paths".into(),
        ));
    }

    let options = TransmuteOptions { with_oneliner };
    let started = Instant::now();
    let result = if let Some(css) = content {
        transmute_css(&css, options)?
    } else {
        let root = std::env::current_dir().map_err(GrimoireCssError::Io)?;
        let patterns = paths
            .unwrap_or_default()
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();
        transmute_paths(&root, &patterns, options)?
    };
    let json = serde_json::to_string_pretty(&result).map_err(GrimoireCssError::Serde)?;

    if let Some(path) = output {
        let path = PathBuf::from(path);
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(GrimoireCssError::Io)?;
        }
        fs::write(&path, json).map_err(GrimoireCssError::Io)?;
    } else {
        println!("{json}");
    }

    eprintln!("Transmutation complete in {:.2?}", started.elapsed());
    Ok(())
}

fn take_value(args: &[String], index: &mut usize, flag: &str) -> Result<String, GrimoireCssError> {
    *index += 1;
    args.get(*index)
        .filter(|value| !value.is_empty() && !is_transmute_flag(value))
        .cloned()
        .ok_or_else(|| GrimoireCssError::InvalidInput(format!("{flag} requires a value")))
}

fn is_transmute_flag(value: &str) -> bool {
    matches!(
        value,
        "--content" | "-c" | "--paths" | "-p" | "--output" | "-o" | "--with-oneliner" | "-l"
    )
}
