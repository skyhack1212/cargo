use cargotest::support::registry::Package;
use cargotest::support::{basic_bin_manifest, basic_lib_manifest, execs, main_file, project};
use hamcrest::assert_that;

#[test]
fn cargo_metadata_simple() {
    let p = project("foo")
        .file("src/foo.rs", "")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .build();

    assert_that(
        p.cargo("metadata"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": [
                    "wycats@example.com"
                ],
                "categories": [],
                "name": "foo",
                "version": "0.5.0",
                "id": "foo[..]",
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "readme": null,
                "repository": null,
                "targets": [
                    {
                        "kind": [
                            "bin"
                        ],
                        "crate_types": [
                            "bin"
                        ],
                        "name": "foo",
                        "src_path": "[..][/]foo[/]src[/]foo.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": ["foo 0.5.0 (path+file:[..]foo)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [],
                    "features": [],
                    "id": "foo 0.5.0 (path+file:[..]foo)"
                }
            ],
            "root": "foo 0.5.0 (path+file:[..]foo)"
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn cargo_metadata_warns_on_implicit_version() {
    let p = project("foo")
        .file("src/foo.rs", "")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .build();

    assert_that(p.cargo("metadata"),
                execs().with_stderr("[WARNING] please specify `--format-version` flag explicitly to avoid compatibility problems"));

    assert_that(
        p.cargo("metadata").arg("--format-version").arg("1"),
        execs().with_stderr(""),
    );
}

#[test]
fn library_with_several_crate_types() {
    let p = project("foo")
        .file("src/lib.rs", "")
        .file(
            "Cargo.toml",
            r#"
[package]
name = "foo"
version = "0.5.0"

[lib]
crate-type = ["lib", "staticlib"]
            "#,
        )
        .build();

    assert_that(
        p.cargo("metadata"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": [],
                "categories": [],
                "name": "foo",
                "readme": null,
                "repository": null,
                "version": "0.5.0",
                "id": "foo[..]",
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [
                            "lib",
                            "staticlib"
                        ],
                        "crate_types": [
                            "lib",
                            "staticlib"
                        ],
                        "name": "foo",
                        "src_path": "[..][/]foo[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": ["foo 0.5.0 (path+file:[..]foo)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [],
                    "features": [],
                    "id": "foo 0.5.0 (path+file:[..]foo)"
                }
            ],
            "root": "foo 0.5.0 (path+file:[..]foo)"
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn library_with_features() {
    let p = project("foo")
        .file("src/lib.rs", "")
        .file(
            "Cargo.toml",
            r#"
[package]
name = "foo"
version = "0.5.0"

[features]
default = ["default_feat"]
default_feat = []
optional_feat = []
            "#,
        )
        .build();

    assert_that(
        p.cargo("metadata"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": [],
                "categories": [],
                "name": "foo",
                "readme": null,
                "repository": null,
                "version": "0.5.0",
                "id": "foo[..]",
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [
                            "lib"
                        ],
                        "crate_types": [
                            "lib"
                        ],
                        "name": "foo",
                        "src_path": "[..][/]foo[/]src[/]lib.rs"
                    }
                ],
                "features": {
                  "default": [
                      "default_feat"
                  ],
                  "default_feat": [],
                  "optional_feat": []
                },
                "manifest_path": "[..]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": ["foo 0.5.0 (path+file:[..]foo)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [],
                    "features": [
                      "default",
                      "default_feat"
                    ],
                    "id": "foo 0.5.0 (path+file:[..]foo)"
                }
            ],
            "root": "foo 0.5.0 (path+file:[..]foo)"
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn cargo_metadata_with_deps_and_version() {
    let p = project("foo")
        .file("src/foo.rs", "")
        .file(
            "Cargo.toml",
            r#"
            [project]
            name = "foo"
            version = "0.5.0"
            authors = []
            license = "MIT"
            description = "foo"

            [[bin]]
            name = "foo"

            [dependencies]
            bar = "*"
        "#,
        )
        .build();
    Package::new("baz", "0.0.1").publish();
    Package::new("bar", "0.0.1").dep("baz", "0.0.1").publish();

    assert_that(
        p.cargo("metadata")
            .arg("-q")
            .arg("--format-version")
            .arg("1"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": [],
                "categories": [],
                "dependencies": [],
                "description": null,
                "features": {},
                "id": "baz 0.0.1 (registry+[..])",
                "keywords": [],
                "manifest_path": "[..]Cargo.toml",
                "name": "baz",
                "readme": null,
                "repository": null,
                "source": "registry+[..]",
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [
                            "lib"
                        ],
                        "crate_types": [
                            "lib"
                        ],
                        "name": "baz",
                        "src_path": "[..]lib.rs"
                    }
                ],
                "version": "0.0.1",
                "metadata": null
            },
            {
                "authors": [],
                "categories": [],
                "dependencies": [
                    {
                        "features": [],
                        "kind": null,
                        "name": "baz",
                        "optional": false,
                        "req": "^0.0.1",
                        "source": "registry+[..]",
                        "target": null,
                        "uses_default_features": true,
                        "rename": null
                    }
                ],
                "features": {},
                "id": "bar 0.0.1 (registry+[..])",
                "keywords": [],
                "manifest_path": "[..]Cargo.toml",
                "name": "bar",
                "readme": null,
                "repository": null,
                "source": "registry+[..]",
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [
                            "lib"
                        ],
                        "crate_types": [
                            "lib"
                        ],
                        "name": "bar",
                        "src_path": "[..]lib.rs"
                    }
                ],
                "version": "0.0.1",
                "metadata": null
            },
            {
                "authors": [],
                "categories": [],
                "dependencies": [
                    {
                        "features": [],
                        "kind": null,
                        "name": "bar",
                        "optional": false,
                        "req": "*",
                        "source": "registry+[..]",
                        "target": null,
                        "uses_default_features": true,
                        "rename": null
                    }
                ],
                "features": {},
                "id": "foo 0.5.0 (path+file:[..]foo)",
                "keywords": [],
                "manifest_path": "[..]Cargo.toml",
                "name": "foo",
                "readme": null,
                "repository": null,
                "source": null,
                "license": "MIT",
                "license_file": null,
                "description": "foo",
                "targets": [
                    {
                        "kind": [
                            "bin"
                        ],
                        "crate_types": [
                            "bin"
                        ],
                        "name": "foo",
                        "src_path": "[..]foo.rs"
                    }
                ],
                "version": "0.5.0",
                "metadata": null
            }
        ],
        "workspace_members": ["foo 0.5.0 (path+file:[..]foo)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [
                        "bar 0.0.1 (registry+[..])"
                    ],
                    "features": [],
                    "id": "foo 0.5.0 (path+file:[..]foo)"
                },
                {
                    "dependencies": [
                        "baz 0.0.1 (registry+[..])"
                    ],
                    "features": [],
                    "id": "bar 0.0.1 (registry+[..])"
                },
                {
                    "dependencies": [],
                    "features": [],
                    "id": "baz 0.0.1 (registry+[..])"
                }
            ],
            "root": "foo 0.5.0 (path+file:[..]foo)"
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn example() {
    let p = project("foo")
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "")
        .file(
            "Cargo.toml",
            r#"
[package]
name = "foo"
version = "0.1.0"

[[example]]
name = "ex"
            "#,
        )
        .build();

    assert_that(
        p.cargo("metadata"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": [],
                "categories": [],
                "name": "foo",
                "readme": null,
                "repository": null,
                "version": "0.1.0",
                "id": "foo[..]",
                "keywords": [],
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": [ "lib" ],
                        "name": "foo",
                        "src_path": "[..][/]foo[/]src[/]lib.rs"
                    },
                    {
                        "kind": [ "example" ],
                        "crate_types": [ "bin" ],
                        "name": "ex",
                        "src_path": "[..][/]foo[/]examples[/]ex.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": [
            "foo 0.1.0 (path+file:[..]foo)"
        ],
        "resolve": {
            "root": "foo 0.1.0 (path+file://[..]foo)",
            "nodes": [
                {
                    "id": "foo 0.1.0 (path+file:[..]foo)",
                    "features": [],
                    "dependencies": []
                }
            ]
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn example_lib() {
    let p = project("foo")
        .file("src/lib.rs", "")
        .file("examples/ex.rs", "")
        .file(
            "Cargo.toml",
            r#"
[package]
name = "foo"
version = "0.1.0"

[[example]]
name = "ex"
crate-type = ["rlib", "dylib"]
            "#,
        )
        .build();

    assert_that(
        p.cargo("metadata"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": [],
                "categories": [],
                "name": "foo",
                "readme": null,
                "repository": null,
                "version": "0.1.0",
                "id": "foo[..]",
                "keywords": [],
                "license": null,
                "license_file": null,
                "description": null,
                "source": null,
                "dependencies": [],
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": [ "lib" ],
                        "name": "foo",
                        "src_path": "[..][/]foo[/]src[/]lib.rs"
                    },
                    {
                        "kind": [ "example" ],
                        "crate_types": [ "rlib", "dylib" ],
                        "name": "ex",
                        "src_path": "[..][/]foo[/]examples[/]ex.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": [
            "foo 0.1.0 (path+file:[..]foo)"
        ],
        "resolve": {
            "root": "foo 0.1.0 (path+file://[..]foo)",
            "nodes": [
                {
                    "id": "foo 0.1.0 (path+file:[..]foo)",
                    "features": [],
                    "dependencies": []
                }
            ]
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn workspace_metadata() {
    let p = project("foo")
        .file(
            "Cargo.toml",
            r#"
            [workspace]
            members = ["bar", "baz"]
        "#,
        )
        .file("bar/Cargo.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .file("baz/Cargo.toml", &basic_lib_manifest("baz"))
        .file("baz/src/lib.rs", "")
        .build();

    assert_that(
        p.cargo("metadata"),
        execs().with_status(0).with_json(
            r#"
    {
        "packages": [
            {
                "authors": [
                    "wycats@example.com"
                ],
                "categories": [],
                "name": "bar",
                "version": "0.5.0",
                "id": "bar[..]",
                "readme": null,
                "repository": null,
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": [ "lib" ],
                        "name": "bar",
                        "src_path": "[..]bar[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]bar[/]Cargo.toml",
                "metadata": null
            },
            {
                "authors": [
                    "wycats@example.com"
                ],
                "categories": [],
                "name": "baz",
                "readme": null,
                "repository": null,
                "version": "0.5.0",
                "id": "baz[..]",
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": [ "lib" ],
                        "name": "baz",
                        "src_path": "[..]baz[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]baz[/]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": ["baz 0.5.0 (path+file:[..]baz)", "bar 0.5.0 (path+file:[..]bar)"],
        "resolve": {
            "nodes": [
                {
                    "dependencies": [],
                    "features": [],
                    "id": "baz 0.5.0 (path+file:[..]baz)"
                },
                {
                    "dependencies": [],
                    "features": [],
                    "id": "bar 0.5.0 (path+file:[..]bar)"
                }
            ],
            "root": null
        },
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    )
}

#[test]
fn workspace_metadata_no_deps() {
    let p = project("foo")
        .file(
            "Cargo.toml",
            r#"
            [workspace]
            members = ["bar", "baz"]
        "#,
        )
        .file("bar/Cargo.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .file("baz/Cargo.toml", &basic_lib_manifest("baz"))
        .file("baz/src/lib.rs", "")
        .build();

    assert_that(
        p.cargo("metadata").arg("--no-deps"),
        execs().with_status(0).with_json(
            r#"
    {
        "packages": [
            {
                "authors": [
                    "wycats@example.com"
                ],
                "categories": [],
                "name": "bar",
                "readme": null,
                "repository": null,
                "version": "0.5.0",
                "id": "bar[..]",
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": [ "lib" ],
                        "name": "bar",
                        "src_path": "[..]bar[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]bar[/]Cargo.toml",
                "metadata": null
            },
            {
                "authors": [
                    "wycats@example.com"
                ],
                "categories": [],
                "name": "baz",
                "readme": null,
                "repository": null,
                "version": "0.5.0",
                "id": "baz[..]",
                "keywords": [],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": ["lib"],
                        "name": "baz",
                        "src_path": "[..]baz[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]baz[/]Cargo.toml",
                "metadata": null
            }
        ],
        "workspace_members": ["baz 0.5.0 (path+file:[..]baz)", "bar 0.5.0 (path+file:[..]bar)"],
        "resolve": null,
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    )
}

#[test]
fn cargo_metadata_with_invalid_manifest() {
    let p = project("foo").file("Cargo.toml", "").build();

    assert_that(
        p.cargo("metadata").arg("--format-version").arg("1"),
        execs().with_status(101).with_stderr(
            "\
[ERROR] failed to parse manifest at `[..]`

Caused by:
  virtual manifests must be configured with [workspace]",
        ),
    )
}

const MANIFEST_OUTPUT: &str = r#"
{
    "packages": [{
        "authors": [
            "wycats@example.com"
        ],
        "categories": [],
        "name":"foo",
        "version":"0.5.0",
        "id":"foo[..]0.5.0[..](path+file://[..]/foo)",
        "source":null,
        "dependencies":[],
        "keywords": [],
        "license": null,
        "license_file": null,
        "description": null,
        "targets":[{
            "kind":["bin"],
            "crate_types":["bin"],
            "name":"foo",
            "src_path":"[..][/]foo[/]src[/]foo.rs"
        }],
        "features":{},
        "manifest_path":"[..]Cargo.toml",
        "metadata": null,
        "readme": null,
        "repository": null
    }],
    "workspace_members": [ "foo 0.5.0 (path+file:[..]foo)" ],
    "resolve": null,
    "target_directory": "[..]foo[/]target",
    "version": 1,
    "workspace_root": "[..][/]foo"
}"#;

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_relative() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    assert_that(
        p.cargo("metadata")
            .arg("--no-deps")
            .arg("--manifest-path")
            .arg("foo/Cargo.toml")
            .cwd(p.root().parent().unwrap()),
        execs().with_status(0).with_json(MANIFEST_OUTPUT),
    );
}

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_absolute() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    assert_that(
        p.cargo("metadata")
            .arg("--no-deps")
            .arg("--manifest-path")
            .arg(p.root().join("Cargo.toml"))
            .cwd(p.root().parent().unwrap()),
        execs().with_status(0).with_json(MANIFEST_OUTPUT),
    );
}

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_parent_relative() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    assert_that(
        p.cargo("metadata")
            .arg("--no-deps")
            .arg("--manifest-path")
            .arg("foo")
            .cwd(p.root().parent().unwrap()),
        execs().with_status(101).with_stderr(
            "[ERROR] the manifest-path must be \
             a path to a Cargo.toml file",
        ),
    );
}

#[test]
fn cargo_metadata_no_deps_path_to_cargo_toml_parent_absolute() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    assert_that(
        p.cargo("metadata")
            .arg("--no-deps")
            .arg("--manifest-path")
            .arg(p.root())
            .cwd(p.root().parent().unwrap()),
        execs().with_status(101).with_stderr(
            "[ERROR] the manifest-path must be \
             a path to a Cargo.toml file",
        ),
    );
}

#[test]
fn cargo_metadata_no_deps_cwd() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    assert_that(
        p.cargo("metadata").arg("--no-deps").cwd(p.root()),
        execs().with_status(0).with_json(MANIFEST_OUTPUT),
    );
}

#[test]
fn cargo_metadata_bad_version() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]))
        .build();

    assert_that(
        p.cargo("metadata")
            .arg("--no-deps")
            .arg("--format-version")
            .arg("2")
            .cwd(p.root()),
        execs().with_status(1).with_stderr_contains(
            "\
error: '2' isn't a valid value for '--format-version <VERSION>'
<tab>[possible values: 1]
",
        ),
    );
}

#[test]
fn multiple_features() {
    let p = project("foo")
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []

            [features]
            a = []
            b = []
        "#,
        )
        .file("src/lib.rs", "")
        .build();

    assert_that(
        p.cargo("metadata").arg("--features").arg("a b"),
        execs().with_status(0),
    );
}

#[test]
fn package_metadata() {
    let p = project("foo")
        .file(
            "Cargo.toml",
            r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = ["wycats@example.com"]
            categories = ["database"]
            keywords = ["database"]
            readme = "README.md"
            repository = "https://github.com/rust-lang/cargo"

            [package.metadata.bar]
            baz = "quux"
        "#,
        )
        .file("src/lib.rs", "")
        .build();

    assert_that(
        p.cargo("metadata").arg("--no-deps"),
        execs().with_json(
            r#"
    {
        "packages": [
            {
                "authors": ["wycats@example.com"],
                "categories": ["database"],
                "name": "foo",
                "readme": "README.md",
                "repository": "https://github.com/rust-lang/cargo",
                "version": "0.1.0",
                "id": "foo[..]",
                "keywords": ["database"],
                "source": null,
                "dependencies": [],
                "license": null,
                "license_file": null,
                "description": null,
                "targets": [
                    {
                        "kind": [ "lib" ],
                        "crate_types": [ "lib" ],
                        "name": "foo",
                        "src_path": "[..]foo[/]src[/]lib.rs"
                    }
                ],
                "features": {},
                "manifest_path": "[..]foo[/]Cargo.toml",
                "metadata": {
                    "bar": {
                        "baz": "quux"
                    }
                }
            }
        ],
        "workspace_members": ["foo[..]"],
        "resolve": null,
        "target_directory": "[..]foo[/]target",
        "version": 1,
        "workspace_root": "[..][/]foo"
    }"#,
        ),
    );
}

#[test]
fn cargo_metadata_path_to_cargo_toml_project() {
    let p = project("foo")
        .file(
            "Cargo.toml",
            r#"
            [workspace]
            members = ["bar"]
        "#,
        )
        .file("bar/Cargo.toml", &basic_lib_manifest("bar"))
        .file("bar/src/lib.rs", "")
        .build();

    assert_that(
    p.cargo("package")
        .arg("--manifest-path")
        .arg(p.root().join("bar/Cargo.toml"))
        .cwd(p.root().parent().unwrap()),
        execs().with_status(0)
        );

    assert_that(
        p.cargo("metadata")
            .arg("--manifest-path")
            .arg(p.root().join("target/package/bar-0.5.0/Cargo.toml")),
        execs().with_status(0).with_json(
        r#"
        {
            "packages": [
            {
                "authors": [
                    "wycats@example.com"
                ],
                "categories": [],
                "dependencies": [],
                "description": null,
                "features": {},
                "id": "bar 0.5.0 ([..])",
                "keywords": [],
                "license": null,
                "license_file": null,
                "manifest_path": "[..]Cargo.toml",
                "metadata": null,
                "name": "bar",
                "readme": null,
                "repository": null,
                "source": null,
                "targets": [
                {
                    "crate_types": [
                        "lib"
                    ],
                    "kind": [
                        "lib"
                    ],
                    "name": "bar",
                    "src_path": "[..]src[/]lib.rs"
                }
                ],
                "version": "0.5.0"
            }
            ],
            "resolve": {
                "nodes": [
                {
                    "dependencies": [],
                    "features": [],
                    "id": "bar 0.5.0 ([..])"
                }
                ],
                "root": "bar 0.5.0 (path+file:[..])"
            },
            "target_directory": "[..]",
            "version": 1,
            "workspace_members": [
                "bar 0.5.0 (path+file:[..])"
            ],
            "workspace_root": "[..]"
        }
"#
),
    );
}


