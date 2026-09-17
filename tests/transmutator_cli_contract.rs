use grimoire_css_lib::transmutator::Transmutation;
use std::{fs, process::Command};
use tempfile::tempdir;

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_grimoire_css"))
}

#[test]
fn transmute_mode_is_owned_by_the_central_command_handler() {
    let handler = fs::read_to_string("src/commands/handler.rs").unwrap();
    let library_entry = fs::read_to_string("src/lib.rs").unwrap();

    assert!(handler.contains("Some(\"transmute\")"));
    assert!(library_entry.contains("commands::process_machine_readable_mode"));
    assert!(!library_entry.contains("commands::transmute::run_transmute_cli"));
}

#[test]
fn inline_mode_prints_only_json_to_stdout_and_status_to_stderr() {
    let output = command()
        .args([
            "transmute",
            "--content",
            ".button { color: red; }",
            "--with-oneliner",
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Transmutation = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result.scrolls[0].name, "button");
    assert_eq!(result.scrolls[0].spells, ["color=red"]);
    assert_eq!(result.scrolls[0].oneliner.as_deref(), Some("color=red"));
    assert!(!output.stdout.starts_with(b"\n"));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Transmutation complete"));
}

#[test]
fn path_mode_uses_process_root_and_writes_only_to_explicit_output() {
    let root = tempdir().unwrap();
    fs::write(root.path().join("styles.css"), ".button { display: flex; }").unwrap();
    let output_path = root.path().join("result/transmuted.json");
    let output = command()
        .current_dir(root.path())
        .args([
            "transmute",
            "--paths",
            "*.css",
            "--output",
            output_path.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    let result: Transmutation = serde_json::from_slice(&fs::read(output_path).unwrap()).unwrap();
    assert_eq!(result.scrolls[0].spells, ["display=flex"]);
    assert!(!root.path().join("grimoire/transmuted.json").exists());
}

#[test]
fn cli_requires_exactly_one_input_mode() {
    let neither = command().arg("transmute").output().unwrap();
    assert!(!neither.status.success());
    assert!(
        String::from_utf8_lossy(&neither.stderr).contains("exactly one of --content or --paths")
    );

    let both = command()
        .args([
            "transmute",
            "--content",
            ".a { color: red; }",
            "--paths",
            "*.css",
        ])
        .output()
        .unwrap();
    assert!(!both.status.success());
    assert!(String::from_utf8_lossy(&both.stderr).contains("exactly one of --content or --paths"));
}

#[test]
fn content_value_may_start_with_a_hyphen() {
    let output = command()
        .args(["transmute", "--content", "--> .card { color: red; }"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Transmutation = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result.scrolls[0].name, "card");
    assert_eq!(result.scrolls[0].spells, ["color=red"]);
}

#[cfg(unix)]
#[test]
fn inline_mode_does_not_require_a_resolvable_process_cwd() {
    let root = tempdir().unwrap();
    let output = Command::new("sh")
        .current_dir(root.path())
        .env("GRIMOIRE_TEST_BIN", env!("CARGO_BIN_EXE_grimoire_css"))
        .args([
            "-c",
            r#"rmdir "$PWD" && exec "$GRIMOIRE_TEST_BIN" transmute --content '.detached { color: red; }'"#,
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Transmutation = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result.scrolls[0].name, "detached");
    assert_eq!(result.scrolls[0].spells, ["color=red"]);
}

#[test]
fn cascade_rejection_keeps_stdout_empty_and_preserves_output_file() {
    let root = tempdir().unwrap();
    let destination = root.path().join("scrolls.json");
    fs::write(&destination, "original").unwrap();
    for to_file in [false, true] {
        let mut cmd = command();
        cmd.args([
            "transmute",
            "--content",
            ".a{color:red}.b{color:blue}.a{color:green}",
        ]);
        if to_file {
            cmd.arg("--output").arg(&destination);
        }
        let result = cmd.output().unwrap();
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
        assert!(String::from_utf8_lossy(&result.stderr).contains("CSS cascade conflict"));
        assert_eq!(fs::read_to_string(&destination).unwrap(), "original");
    }
}

#[test]
fn file_url_rejection_keeps_cli_output_unchanged() {
    let root = tempdir().unwrap();
    let source = root.path().join("input.css");
    let destination = root.path().join("scrolls.json");
    fs::write(&source, ".card{background-image:url(image.png)}").unwrap();
    fs::write(&destination, "original").unwrap();
    for to_file in [false, true] {
        let mut cmd = command();
        cmd.current_dir(root.path())
            .args(["transmute", "--paths", "input.css"]);
        if to_file {
            cmd.arg("--output").arg(&destination);
        }
        let output = cmd.output().unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(
            error.contains("URL") && error.contains("input.css"),
            "{error}"
        );
        assert_eq!(fs::read_to_string(&destination).unwrap(), "original");
    }
}

#[test]
fn file_and_inline_migration_preserve_css_through_cli_build() {
    use lightningcss::{rules::CssRule, stylesheet::StyleSheet};
    let root = tempdir().unwrap();
    let source = r#".card::before { content: "mfs(invalid) mrs(invalid) _ x"; --value: g-invert(#ffffff); COLOR: red; font-family: O\'Reilly; animation: bounce-bottom 1s; }"#;
    fs::write(root.path().join("input.css"), source).unwrap();
    let inline = command()
        .current_dir(root.path())
        .args(["transmute", "--content", source])
        .output()
        .unwrap();
    let file = command()
        .current_dir(root.path())
        .args(["transmute", "--paths", "input.css"])
        .output()
        .unwrap();
    assert!(
        inline.status.success(),
        "{}",
        String::from_utf8_lossy(&inline.stderr)
    );
    assert!(
        file.status.success(),
        "{}",
        String::from_utf8_lossy(&file.stderr)
    );
    assert_eq!(inline.stdout, file.stdout);
    fs::create_dir_all(root.path().join("grimoire/config")).unwrap();
    fs::write(
        root.path()
            .join("grimoire/config/grimoire.functions.scrolls.json"),
        file.stdout,
    )
    .unwrap();
    fs::write(
        root.path().join("index.html"),
        r#"<div class="card"></div>"#,
    )
    .unwrap();
    fs::write(root.path().join("grimoire/config/grimoire.config.json"), serde_json::to_vec(&serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "projects": [{"projectName":"main", "inputPaths":["index.html"], "outputDirPath":"dist", "singleOutputFileName":"main.css"}]
    })).unwrap()).unwrap();
    let build = command()
        .current_dir(root.path())
        .arg("build")
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let output = fs::read_to_string(root.path().join("dist/main.css")).unwrap();
    let before = StyleSheet::parse(source, Default::default()).unwrap();
    let after = StyleSheet::parse(&output, Default::default()).unwrap();
    assert_eq!(after.rules.0.len(), 1);
    let (CssRule::Style(before), CssRule::Style(after)) = (&before.rules.0[0], &after.rules.0[0])
    else {
        panic!("missing rule: {output}")
    };
    assert_eq!(
        before.declarations.declarations,
        after.declarations.declarations
    );
}

#[test]
fn unsupported_css_rejection_preserves_cli_output_file() {
    let root = tempdir().unwrap();
    let output_path = root.path().join("scrolls.json");
    fs::write(&output_path, "existing output").unwrap();
    for (source, message) in [
        (".card{c:red}", "CSS property"),
        (".card{W:10px}", "CSS property"),
        (".card{g-anim:initial}", "CSS property"),
        (
            r".card{--foo\=bar:red;color:var(--foo\=bar)}",
            "custom property name",
        ),
        (".foo\u{00a0}bar{color:red}", "Scroll name"),
        (r".foo\a0 bar{color:red}", "Scroll name"),
    ] {
        fs::write(root.path().join("input.css"), source).unwrap();
        for input in [["--content", source], ["--paths", "input.css"]] {
            let result = command()
                .current_dir(root.path())
                .arg("transmute")
                .args(input)
                .args(["--output", output_path.to_str().unwrap()])
                .output()
                .unwrap();
            assert!(!result.status.success(), "{source}");
            assert!(result.stdout.is_empty());
            assert!(String::from_utf8_lossy(&result.stderr).contains(message));
            assert_eq!(fs::read_to_string(&output_path).unwrap(), "existing output");
        }
    }
}
