use grimoire_css_lib::{
    GrimoireCssError,
    transmutator::{
        Transmutation, TransmuteOptions, TransmutedScroll, transmute_css, transmute_paths,
    },
};
use serde_json::json;
use std::fs;
use tempfile::tempdir;

#[test]
fn inline_api_is_typed_source_ordered_and_json_compatible() {
    let result = transmute_css(
        ".button { color: red; display: flex; color: red; } .card:hover { padding: 1rem; }",
        TransmuteOptions {
            with_oneliner: true,
        },
    )
    .unwrap();

    assert_eq!(
        result,
        Transmutation {
            scrolls: vec![
                TransmutedScroll {
                    name: "button".into(),
                    spells: vec!["display=flex".into(), "color=red".into()],
                    oneliner: Some("display=flex color=red".into()),
                },
                TransmutedScroll {
                    name: "card".into(),
                    spells: vec!["{:hover}padding=1rem".into()],
                    oneliner: Some("{:hover}padding=1rem".into()),
                },
            ],
        }
    );
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        json!({
            "scrolls": [
                {"name":"button","spells":["display=flex","color=red"],"oneliner":"display=flex color=red"},
                {"name":"card","spells":["{:hover}padding=1rem"],"oneliner":"{:hover}padding=1rem"}
            ]
        })
    );
}

#[test]
fn path_api_uses_explicit_root_and_stable_file_order() {
    let root = tempdir().unwrap();
    // Different properties keep this file-order fixture independent of the cascade.
    fs::write(root.path().join("b.css"), ".second { opacity: 0.5; }").unwrap();
    fs::write(root.path().join("a.css"), ".first { display: flex; }").unwrap();

    let first = transmute_paths(
        root.path(),
        &["*.css".to_string()],
        TransmuteOptions::default(),
    )
    .unwrap();
    let second = transmute_paths(
        root.path(),
        &["*.css".to_string()],
        TransmuteOptions::default(),
    )
    .unwrap();

    assert_eq!(first, second);
    assert_eq!(
        first
            .scrolls
            .iter()
            .map(|scroll| scroll.name.as_str())
            .collect::<Vec<_>>(),
        ["first", "second"]
    );
    assert!(first.scrolls.iter().all(|scroll| scroll.oneliner.is_none()));
}

#[test]
fn path_api_treats_glob_metacharacters_in_root_as_literal_path_characters() {
    let parent = tempdir().unwrap();
    let root = parent.path().join(if cfg!(windows) {
        "project[x]-literal"
    } else {
        "project[x]-with-?-literal"
    });
    fs::create_dir(&root).unwrap();
    fs::write(root.join("input.css"), ".rooted { display: grid; }").unwrap();

    let result =
        transmute_paths(&root, &["*.css".to_string()], TransmuteOptions::default()).unwrap();

    assert_eq!(result.scrolls[0].name, "rooted");
    assert_eq!(result.scrolls[0].spells, ["display=grid"]);
}

#[test]
fn empty_content_and_unmatched_paths_keep_domain_errors() {
    assert!(matches!(
        transmute_css("/* nothing */", TransmuteOptions::default()),
        Err(GrimoireCssError::InvalidInput(_))
    ));

    let root = tempdir().unwrap();
    assert!(matches!(
        transmute_paths(
            root.path(),
            &["missing/*.css".to_string()],
            TransmuteOptions::default()
        ),
        Err(GrimoireCssError::InvalidPath(_))
    ));
    assert!(matches!(
        transmute_paths(
            root.path(),
            &["../outside.css".to_string()],
            TransmuteOptions::default()
        ),
        Err(GrimoireCssError::InvalidPath(_))
    ));
}

#[test]
fn invalid_css_and_already_grimoire_spell_classes_produce_no_fake_scrolls() {
    assert!(matches!(
        transmute_css(".broken { color:", TransmuteOptions::default()),
        Err(GrimoireCssError::InvalidInput(_))
    ));
    assert!(matches!(
        transmute_css(
            r#".display\=flex { color: red; }"#,
            TransmuteOptions::default()
        ),
        Err(GrimoireCssError::InvalidInput(_))
    ));
}

#[test]
fn final_declaration_does_not_require_a_trailing_semicolon() {
    let result = transmute_css(
        ".card { color: red; display: flex }",
        TransmuteOptions::default(),
    )
    .unwrap();

    assert_eq!(result.scrolls[0].spells, ["color=red", "display=flex"]);
}

#[test]
fn malformed_trailing_rule_cannot_return_a_partial_success() {
    for css in [
        ".valid { color: red; } .broken { display:",
        ".valid { color: red; } .broken { background: url(foo\"bar); }",
        ".valid { color: red; } .broken color: blue;",
        ".valid { color: red; } @media (min-width: 1px); .later { display: block; }",
        "@media (min-width: 1px) { .valid { color: red; } .broken color: blue; }",
        ".valid { color: red; } @supports (display: grid) { .broken { color red; } }",
        ".valid { color: red; } @layer utilities { .broken { color red; } }",
        ".valid { color: red; } @keyframes spin { from { opacity 0; } }",
    ] {
        assert!(matches!(
            transmute_css(css, TransmuteOptions::default()),
            Err(GrimoireCssError::InvalidInput(_))
        ));
    }
}

#[test]
fn malformed_declarations_cannot_return_a_partial_success() {
    for css in [
        ".valid { color red; display: block; }",
        ".valid { color: ; display: block; }",
        ".valid { color: red; broken; display: block; }",
        "@media (min-width: 1px) { .valid { color red; display: block; } }",
    ] {
        assert!(
            matches!(
                transmute_css(css, TransmuteOptions::default()),
                Err(GrimoireCssError::InvalidInput(_))
            ),
            "malformed declaration list must reject the complete input: {css}"
        );
    }
}

#[test]
fn malformed_selectors_cannot_produce_plausible_scrolls() {
    for css in [
        ".a,,.b { color: red; }",
        ".a > { color: red; }",
        ", .a { color: red; }",
        ".a[foo=] { color: red; }",
        "& .a { color: red; }",
        ".a & { color: red; }",
        ".a:is(&) { color: red; }",
    ] {
        assert!(
            matches!(
                transmute_css(css, TransmuteOptions::default()),
                Err(GrimoireCssError::InvalidInput(_))
            ),
            "malformed selector list must reject the complete input: {css}"
        );
    }
}

#[test]
fn malformed_media_queries_cannot_produce_plausible_spells() {
    for css in [
        "@media { .a { color: red; } }",
        "@media (min-width:) { .a { color: red; } }",
    ] {
        assert!(
            matches!(
                transmute_css(css, TransmuteOptions::default()),
                Err(GrimoireCssError::InvalidInput(_))
            ),
            "malformed media query must reject the complete input: {css}"
        );
    }
}

#[test]
fn migration_preserves_simple_media_and_child_class_selectors() {
    let media = transmute_css(
        "@media (min-width: 768px) { .card:hover { color: red; } }",
        TransmuteOptions::default(),
    )
    .unwrap();
    assert_eq!(
        media.scrolls[0].spells,
        ["(min-width:_768px)__{:hover}color=red"]
    );

    let uppercase_media = transmute_css(
        "@MEDIA print { .print-only { color: black; } }",
        TransmuteOptions::default(),
    )
    .unwrap();
    assert_eq!(uppercase_media.scrolls[0].spells, ["print__color=black"]);

    let media_list = transmute_css(
        "@media print, screen and (min-width: 768px) { .card { color: red; } }",
        TransmuteOptions::default(),
    )
    .unwrap();
    assert_eq!(
        media_list.scrolls[0].spells,
        ["print,_screen_and_(min-width:_768px)__color=red"]
    );

    let selector = transmute_css(
        r#".menu > .item::before { content: "x"; display: block; }"#,
        TransmuteOptions::default(),
    )
    .unwrap();
    assert_eq!(selector.scrolls[0].name, "menu");
    assert_eq!(
        selector.scrolls[0].spells,
        [
            r#"{_>_.item::before}content="x""#,
            "{_>_.item::before}display=block"
        ]
    );
}

#[test]
fn file_mode_uses_css_tokens_for_comments_and_matches_inline_strings() {
    let root = tempdir().unwrap();
    let css = r#"
        @media /* ignored */ (min-width: 768px) {
            .card {
                color: red /* ignored */;
                /* ignored */ display: flex;
                padding: 1rem /* ignored */
            }
        }
        .label::before { content: "/* keep me */"; }
    "#;
    fs::write(root.path().join("styles.css"), css).unwrap();
    let from_paths = transmute_paths(
        root.path(),
        &["styles.css".to_string()],
        TransmuteOptions::default(),
    )
    .unwrap();
    let inline = transmute_css(css, TransmuteOptions::default()).unwrap();
    assert_eq!(from_paths, inline);
    assert_eq!(
        from_paths,
        Transmutation {
            scrolls: vec![
                TransmutedScroll {
                    name: "card".into(),
                    spells: vec![
                        "(min-width:_768px)__color=red".into(),
                        "(min-width:_768px)__display=flex".into(),
                        "(min-width:_768px)__padding=1rem".into(),
                    ],
                    oneliner: None,
                },
                TransmutedScroll {
                    name: "label".into(),
                    spells: vec![r#"{::before}content="/*_keep_me_*/""#.into()],
                    oneliner: None,
                },
            ],
        }
    );
}

#[test]
fn migration_preserves_compound_and_descendant_selectors() {
    for (css, name, expected) in [
        (".card.active { color: red; }", "card", "{.active}color=red"),
        (".card .child { color: red; }", "card", "{_.child}color=red"),
        (
            ".menu > .item::before { display: block; }",
            "menu",
            "{_>_.item::before}display=block",
        ),
    ] {
        let result = transmute_css(css, TransmuteOptions::default()).unwrap();
        assert_eq!(result.scrolls.len(), 1, "{css}");
        assert_eq!(result.scrolls[0].name, name);
        assert_eq!(result.scrolls[0].spells, [expected]);
    }
}

#[test]
fn migration_preserves_media_for_every_selector_and_nested_conditions() {
    let result = transmute_css(
        "@media print { .card, .other { color: red; } }",
        TransmuteOptions::default(),
    )
    .unwrap();
    for scroll in result.scrolls {
        assert_eq!(scroll.spells, ["print__color=red"]);
    }
    let result = transmute_css(
        "@media print { @media (min-width: 768px) { .card { color: red; } } }",
        TransmuteOptions::default(),
    )
    .unwrap();
    let spell = &result.scrolls[0].spells[0];
    assert!(spell.contains("print"), "{spell}");
    assert!(spell.contains("768px"), "{spell}");
}

#[test]
fn migration_rejects_unsupported_rules_instead_of_returning_partial_css() {
    for rule in [
        "@layer utilities { .hidden { display: none; } }",
        "@supports (display: grid) { .grid { display: grid; } }",
        "@keyframes spin { from { opacity: 0; } to { opacity: 1; } }",
        "@import 'theme.css';",
        "h1 { color: red; }",
    ] {
        let result = transmute_css(
            &format!(".card {{ color: red; }} {rule}"),
            TransmuteOptions::default(),
        );
        assert!(result.is_err(), "must not discard {rule}: {result:?}");
    }
}

#[test]
fn migration_preserves_token_boundaries_around_comments() {
    let result =
        transmute_css(".card { margin: 1px/**/2px; }", TransmuteOptions::default()).unwrap();
    assert_eq!(result.scrolls[0].spells, ["margin=1px/**/2px"]);
}

#[test]
fn comments_between_selector_parts_do_not_create_descendant_combinators() {
    let result = transmute_css(
        ".card/**/.active { color: red; }",
        TransmuteOptions::default(),
    )
    .unwrap();
    assert_eq!(result.scrolls[0].spells, ["{.active}color=red"]);
}

#[test]
fn inline_migration_rejects_names_the_engine_cannot_invoke() {
    for css in [
        r#".a\:b {color:red}"#,
        r#".\31 23 {color:red}"#,
        ".a__b {color:red}",
        ".foo\u{00a0}bar {color:red}",
        r".foo\a0 bar {color:red}",
        ".foo\u{2003}bar {color:red}",
        ".\u{00a0}foo {color:red}",
        ".foo\u{00a0} {color:red}",
    ] {
        let error = transmute_css(css, TransmuteOptions::default()).unwrap_err();
        assert!(error.to_string().contains("Scroll name"), "{error}");
    }
}

#[test]
fn migration_rejects_component_names_shadowed_by_its_own_scrolls() {
    for css in [
        ".color {color:red}",
        ".color {display:block} .card {color:red}",
    ] {
        let error = transmute_css(css, TransmuteOptions::default()).unwrap_err();
        assert!(
            error.to_string().contains("component/Scroll conflict"),
            "{error}"
        );
    }
}

#[test]
fn empty_custom_properties_are_valid_but_empty_normal_properties_are_not() {
    for css in [
        ".card {--empty:}",
        ".card {--empty: ;}",
        ".card {--empty:/**/;}",
    ] {
        let result = transmute_css(css, TransmuteOptions::default()).unwrap();
        assert_eq!(result.scrolls[0].spells, ["--empty="]);
    }
    assert!(transmute_css(".card {color:}", TransmuteOptions::default()).is_err());
    assert!(transmute_css(".card {--:}", TransmuteOptions::default()).is_err());
}

#[test]
fn migration_rejects_cascade_dependencies_between_scrolls() {
    for css in [
        ".a{color:red}.b{color:blue}.a{color:green}",
        ".a{color:red}.b{color:blue}",
        ".a{margin:1px}.b{margin-left:2px}",
        ".a{margin-inline-start:1px}.b{margin-left:2px}",
        ".a{all:initial}.b{color:blue}",
        ".a{--tone:red}.b{--tone:blue}",
        ".a{COLOR:red}.b{color:blue}",
        ".a{word-wrap:normal}.b{overflow-wrap:anywhere}",
        ".a{border:none}.b{border-image-source:url(x.png)}",
        ".a{font:12px serif}.b{font-kerning:none}",
        ".a{color:red!important}.b{color:blue!important}",
        "@media print {.a{color:red}} .b{color:blue}",
        ".a .child{color:red}.b .child{color:blue}",
        ".a:hover{color:red}.b:focus{color:blue}",
    ] {
        let error = transmute_css(css, TransmuteOptions::default())
            .unwrap_err()
            .to_string();
        assert!(error.contains("CSS cascade conflict"), "{css}: {error}");
        assert!(error.contains(".a") && error.contains(".b"), "{error}");
        assert!(error.contains("shared.styles"), "{error}");
    }
}

#[test]
fn migration_accepts_order_independent_scrolls_and_keeps_local_order() {
    for css in [
        ".a{color:red}.b{display:flex}",
        ".a{color:red}.b:hover{color:blue}",
        ".a{color:red!important}.b{color:blue}",
        ".a{--Tone:red}.b{--tone:blue}",
        ".a{--tone:red}.b{color:var(--tone)}",
        ".a,.b{margin:1px;color:red}",
        ".a{color:red}.b{color:red}",
        ".a{color:red}.a{color:green}",
        ".a{margin:1px}.a{margin-left:2px}.a{margin:1px}",
    ] {
        assert!(
            transmute_css(css, TransmuteOptions::default()).is_ok(),
            "{css}"
        );
    }
}

#[test]
fn path_migration_checks_cascade_across_files() {
    let root = tempdir().unwrap();
    fs::write(root.path().join("a.css"), ".a{color:red}").unwrap();
    fs::write(root.path().join("b.css"), ".b{color:blue}").unwrap();
    let error =
        transmute_paths(root.path(), &["*.css".into()], TransmuteOptions::default()).unwrap_err();
    assert!(
        error.to_string().contains("CSS cascade conflict"),
        "{error}"
    );
}

#[test]
fn file_migration_rejects_urls_that_depend_on_the_source_directory() {
    let root = tempdir().unwrap();
    for value in [
        "url(./image.png)",
        "url('../image.png')",
        "url(\"image.png\")",
        "url(?version=1)",
        "url('')",
        "URL(image.png)",
        r"u\72l(image.png)",
        "image-set(\"image.png\" 1x)",
        "-webkit-image-set(url(image.png) 1x)",
        "image(\"image.png\", red)",
        "src(\"image.png\")",
        "src(var(--path))",
        "image-set(var(--images))",
    ] {
        fs::write(
            root.path().join("source.css"),
            format!(".card{{background-image:{value}}}"),
        )
        .unwrap();
        let error = transmute_paths(root.path(), &["source.css".into()], Default::default())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("source.css") && error.contains("URL"),
            "{value}: {error}"
        );
        assert!(
            error.contains("root-relative") || error.contains("absolute"),
            "{error}"
        );
    }
}

#[test]
fn file_migration_accepts_urls_independent_of_the_source_directory() {
    let root = tempdir().unwrap();
    for value in [
        "url(/images/icon.png)",
        "url(//cdn.example/icon.png)",
        "url(https://example.test/icon.png)",
        "url(data:image/png;base64,AA==)",
        "url(#icon)",
        "image-set(\"/images/icon.png\" 1x type(\"image/png\"))",
        "src(\"/images/icon.png\")",
    ] {
        let css = format!(".card{{background-image:{value}}}");
        fs::write(root.path().join("source.css"), &css).unwrap();
        let result = transmute_paths(root.path(), &["source.css".into()], Default::default());
        assert_eq!(
            result.unwrap(),
            transmute_css(&css, Default::default()).unwrap(),
            "{value}"
        );
    }
    let css = r#".card::before{content:"url(image.png)"}"#;
    fs::write(root.path().join("source.css"), css).unwrap();
    assert!(transmute_paths(root.path(), &["source.css".into()], Default::default()).is_ok());
}

#[test]
fn standard_property_case_is_normalized_and_custom_property_case_is_preserved() {
    let result = transmute_css(
        ".card { COLOR: red; DiSpLaY: flex; --Primary: red; --primary: blue; }",
        TransmuteOptions::default(),
    )
    .unwrap();
    assert_eq!(
        result.scrolls[0].spells,
        [
            "color=red",
            "display=flex",
            "--Primary=red",
            "--primary=blue"
        ]
    );
}

#[test]
fn migration_rejects_custom_property_names_that_change_spell_components() {
    for source in [
        r".card{--foo\=bar:red;color:var(--foo\=bar)}",
        r".card{--foo\:bar:red}",
        r".card{--foo\ bar:red}",
        ".card{--foo__bar:red}",
        ".card{--цвет:red}",
    ] {
        let error = transmute_css(source, Default::default())
            .unwrap_err()
            .to_string();
        assert!(error.contains("custom property name"), "{source}: {error}");
        assert!(error.contains("shared.styles"), "{error}");
    }
}
