use grimoire_css_lib::{Spell, build};
use serde_json::json;
use std::{collections::HashSet, fs, process::Command};
use tempfile::tempdir;

#[test]
fn css_comments_do_not_change_scroll_value_validation_or_builds() {
    for (spell, expected_css) in [
        ("color=red/*don't-change*/", "color:red"),
        ("color=red/*\"*/", "color:red"),
        ("width=calc(10px/*)*/_+_2px)", "width:12px"),
        ("content='/*'", "content:\"/*\""),
        (
            "background-image=url(https://example.test/icons/*)",
            "icons/*",
        ),
    ] {
        for cli in [false, true] {
            let css = build_scroll(spell, cli);
            assert!(css.contains(expected_css), "{spell}: {css}");
        }
    }
}

#[test]
fn comments_cannot_hide_unclosed_strings_or_parentheses() {
    for (spell, message) in [
        ("content='/*comment*/", "unclosed quoted string"),
        ("content=\"/*comment*/", "unclosed quoted string"),
        ("width=calc(10px/*)*/", "unclosed '('"),
        ("width=10px/*(*/)", "unexpected ')'"),
    ] {
        let error = Spell::new(spell, &HashSet::new(), &None, (0, spell.len()), None)
            .unwrap_err()
            .to_string();
        assert!(error.contains(message), "{spell}: {error}");
    }
}

fn build_scroll(spell: &str, cli: bool) -> String {
    let root = tempdir().unwrap();
    let config_path = root.path().join("grimoire/config/grimoire.config.json");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    fs::write(root.path().join("index.html"), "<div class=\"card\"></div>").unwrap();
    fs::write(&config_path, serde_json::to_vec(&json!({
    "version": env!("CARGO_PKG_VERSION"),
    "projects": [{"projectName":"main", "inputPaths":["index.html"], "outputDirPath":"dist", "singleOutputFileName":"main.css"}],
    "scrolls": [{"name":"card", "spells":[spell]}]
})).unwrap()).unwrap();
    if cli {
        let output = Command::new(env!("CARGO_BIN_EXE_grimoire_css"))
            .current_dir(root.path())
            .arg("build")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{spell}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        build(root.path()).unwrap_or_else(|error| panic!("{spell}: {error}"));
    }
    fs::read_to_string(root.path().join("dist/main.css")).unwrap()
}

#[test]
fn comments_in_focus_and_media_prefixes_preserve_generated_rules() {
    for (spell, expected) in [
        ("{/*don't*/_p}color=red", ".card p{color:red}"),
        ("{/*(*/_p}color=red", ".card p{color:red}"),
        ("{/*\"__={[(*/_p}color=red", ".card p{color:red}"),
        (
            "(min-width:600px)/*don't*/__color=red",
            "@media (width>=600px){.card{color:red}}",
        ),
        (
            "(min-width:600px)/*\"__={[(*/__{_p}color=red",
            "@media (width>=600px){.card p{color:red}}",
        ),
    ] {
        for cli in [false, true] {
            assert_eq!(build_scroll(spell, cli), expected, "{spell}, cli={cli}");
        }
    }
}

#[test]
fn comment_markers_in_quoted_prefix_data_remain_literal() {
    let spell = r#"{[data-note="/*'__={[("]}color=red"#;
    let parsed = Spell::new(spell, &HashSet::new(), &None, (0, spell.len()), None)
        .unwrap()
        .unwrap();
    assert_eq!(parsed.focus(), r#"[data-note="/*'__={[("]"#);
    assert_eq!(parsed.component(), "color");
    assert_eq!(parsed.component_target(), "red");
    for cli in [false, true] {
        let css = build_scroll(spell, cli);
        // Authored Spell underscores retain their normal space-decoding semantics.
        assert!(css.contains("/*'  ={[("), "{css}");
        assert!(css.ends_with("{color:red}"), "{css}");
    }
}
