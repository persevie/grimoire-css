use crate::buffer::add_message;
use crate::commands::init::init;
use crate::core::parser::parser_fs::ParserFs;
use crate::core::{GrimoireCssError, component::get_shorten_component, spell::Spell};
use std::fs;
use std::path::{Path, PathBuf};

/// CLI command: Shorten all spells in project files (FS mode only)
pub fn shorten(current_dir: &Path) -> Result<(), GrimoireCssError> {
    let config = init(current_dir, "shorten")?;

    let mut total_replaced = 0usize;
    let mut total_files_changed = 0usize;
    let mut total_bytes_saved = 0isize;

    for project in &config.projects {
        let input_paths = &project.input_paths;
        let parser = ParserFs::new(current_dir);

        let mut files_to_process = Vec::new();
        for input_path in input_paths {
            let abs_path = current_dir.join(input_path);
            collect_files_recursively(&abs_path, &mut files_to_process)?;
        }

        for file_path in files_to_process {
            let content = match fs::read_to_string(&file_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let orig_size = content.len();
            let raw_spells = parser.collect_raw_spells(&content)?;

            let mut new_content = content.clone();
            let mut replaced_any = false;
            let mut replaced_count = 0usize;

            for raw_spell in &raw_spells {
                if raw_spell.starts_with("g!") && raw_spell.ends_with(';') {
                    let inner = &raw_spell[2..raw_spell.len() - 1];
                    let parts: Vec<&str> = inner.split("--").collect();
                    let mut new_parts = Vec::with_capacity(parts.len());
                    let mut changed = false;
                    for part in parts {
                        if let Ok(Some(spell)) =
                            Spell::new(part, &config.shared_spells, &config.scrolls)
                        {
                            if let Some(short) = get_shorten_component(&spell.component) {
                                let short_part = part.replacen(&spell.component, short, 1);
                                if short_part != part {
                                    changed = true;
                                }
                                new_parts.push(short_part);
                            } else {
                                new_parts.push(part.to_string());
                            }
                        } else {
                            new_parts.push(part.to_string());
                        }
                    }
                    if changed {
                        let new_template = format!("g!{};", new_parts.join("--"));
                        if new_template != *raw_spell && new_content.contains(raw_spell) {
                            let count = new_content.matches(raw_spell).count();
                            new_content = new_content.replace(raw_spell, &new_template);
                            replaced_any = true;
                            replaced_count += count;
                        }
                    }
                } else if let Ok(Some(spell)) =
                    Spell::new(raw_spell, &config.shared_spells, &config.scrolls)
                    && let Some(short) = get_shorten_component(&spell.component)
                {
                    let short_spell = raw_spell.replacen(&spell.component, short, 1);
                    if raw_spell != &short_spell && new_content.contains(raw_spell) {
                        let count = new_content.matches(raw_spell).count();
                        new_content = new_content.replace(raw_spell, &short_spell);
                        replaced_any = true;
                        replaced_count += count;
                    }
                }
            }

            if replaced_any && new_content != content {
                fs::write(&file_path, &new_content)?;
                let new_size = new_content.len();
                let bytes_saved = orig_size as isize - new_size as isize;
                total_replaced += replaced_count;
                total_files_changed += 1;
                total_bytes_saved += bytes_saved;
            }
        }
    }

    if total_files_changed > 0 {
        let summary = format!(
            "{} spell{} shortened in {} file{}, {} saved.",
            total_replaced,
            if total_replaced == 1 { "" } else { "s" },
            total_files_changed,
            if total_files_changed == 1 { "" } else { "s" },
            format_size(total_bytes_saved)
        );
        add_message(summary);
    }

    fn format_size(bytes: isize) -> String {
        let abs_bytes = bytes.abs() as f64;
        let (value, unit) = if abs_bytes < 1024.0 {
            (bytes as f64, "B")
        } else if abs_bytes < 1024.0 * 1024.0 {
            (bytes as f64 / 1024.0, "KB")
        } else if abs_bytes < 1024.0 * 1024.0 * 1024.0 {
            (bytes as f64 / (1024.0 * 1024.0), "MB")
        } else {
            (bytes as f64 / (1024.0 * 1024.0 * 1024.0), "GB")
        };
        if unit == "B" {
            format!("{} {}", value as isize, unit)
        } else {
            format!("{value:.2} {unit}")
        }
    }
    Ok(())
}

fn collect_files_recursively(
    path: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), GrimoireCssError> {
    if path.is_file() {
        files.push(path.to_path_buf());
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            collect_files_recursively(&entry.path(), files)?;
        }
    }
    Ok(())
}
