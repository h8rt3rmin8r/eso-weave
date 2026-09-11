//! Build support for executable resources and bundled documentation.

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MDBOOK_VERSION: &str = "mdbook v0.5.4";
const LINKCHECK_VERSION: &str = "mdbook-linkcheck2 0.13.0";

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    for path in [
        "build.rs",
        "assets/dev-docs",
        "docs/book.toml",
        "docs/src",
        "docs/theme",
        "assets/icon.ico",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    let release = env::var("PROFILE").as_deref() == Ok("release");
    let source = if release {
        build_release_docs(&root)
    } else {
        root.join("assets/dev-docs")
    };
    write_manifest(&source, release);
    #[cfg(windows)]
    embed_windows_icon();
}

fn build_release_docs(root: &Path) -> PathBuf {
    verify_tool(root, "mdbook", MDBOOK_VERSION);
    verify_tool(root, "mdbook-linkcheck2", LINKCHECK_VERSION);
    let status = command("mdbook")
        .args(["build", "docs"])
        .current_dir(root)
        .status()
        .unwrap_or_else(|err| panic!("failed to start mdbook for bundled documentation: {err}"));
    assert!(
        status.success(),
        "mdbook failed while building bundled documentation"
    );
    root.join("target/docs-site/html")
}

fn verify_tool(root: &Path, executable: &str, expected: &str) {
    let output = command(executable)
        .arg("--version")
        .current_dir(root)
        .output()
        .unwrap_or_else(|err| {
            panic!("required documentation tool {executable} is unavailable: {err}")
        });
    assert!(output.status.success(), "{executable} --version failed");
    let actual = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    assert_eq!(actual, expected, "unsupported documentation tool version");
}

#[cfg(windows)]
fn command(executable: &str) -> Command {
    let mut command = Command::new(executable);
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
    command
}

#[cfg(not(windows))]
fn command(executable: &str) -> Command {
    Command::new(executable)
}

fn write_manifest(source: &Path, release: bool) {
    assert!(
        source.join("index.html").is_file(),
        "documentation site has no index.html"
    );
    let mut files = Vec::new();
    collect_files(source, source, &mut files);
    files.sort_by(|left, right| left.0.cmp(&right.0));
    assert!(!files.is_empty(), "documentation site contains no files");
    for pair in files.windows(2) {
        assert_ne!(pair[0].0, pair[1].0, "duplicate documentation path");
    }
    if release {
        validate_release_manifest(&files);
    }
    let mut generated = String::from("pub static EMBEDDED_ASSETS: &[EmbeddedAsset] = &[\n");
    for (path, absolute) in files {
        let absolute = absolute.to_string_lossy();
        generated.push_str(&format!(
            "    EmbeddedAsset {{ path: {path:?}, content_type: {:?}, bytes: include_bytes!({absolute:?}) }},\n",
            content_type(&path)
        ));
    }
    generated.push_str("];\n");
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("build output directory"));
    fs::write(output.join("embedded_docs.rs"), generated).expect("write documentation manifest");
}

fn validate_release_manifest(files: &[(String, PathBuf)]) {
    let contains = |expected: &str| files.iter().any(|(path, _)| path == expected);
    let matches = |prefix: &str, suffix: &str| {
        files
            .iter()
            .any(|(path, _)| path.starts_with(prefix) && path.ends_with(suffix))
    };
    for required in [
        "index.html",
        "404.html",
        "print.html",
        "toc.html",
        "assets/diagrams/architecture-ownership.svg",
        "assets/diagrams/action-authorization.svg",
        "assets/diagrams/safety-recovery.svg",
        "assets/diagrams/pixel-bus-validation.svg",
    ] {
        assert!(
            contains(required),
            "release documentation is missing {required}"
        );
    }
    for (label, prefix, suffix) in [
        ("search index", "searchindex-", ".js"),
        ("search runtime", "searcher-", ".js"),
        ("brand stylesheet", "theme/eso-weave-", ".css"),
        ("brand behavior", "theme/eso-weave-", ".js"),
    ] {
        assert!(
            matches(prefix, suffix),
            "release documentation is missing {label}"
        );
    }
}

fn collect_files(root: &Path, directory: &Path, files: &mut Vec<(String, PathBuf)>) {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .unwrap_or_else(|err| {
            panic!(
                "read documentation directory {}: {err}",
                directory.display()
            )
        })
        .map(|entry| entry.expect("read documentation entry"))
        .collect();
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let file_type = entry.file_type().expect("read documentation file type");
        if file_type.is_dir() {
            collect_files(root, &path, files);
        } else if file_type.is_file() {
            let relative = path
                .strip_prefix(root)
                .expect("documentation path under root");
            files.push((normalize_path(relative), path));
        } else {
            panic!(
                "documentation site contains unsupported entry {}",
                path.display()
            );
        }
    }
}

fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|part| {
            let part = part
                .as_os_str()
                .to_str()
                .expect("documentation paths must be UTF-8");
            assert!(
                !part.is_empty() && part != "." && part != "..",
                "invalid documentation path"
            );
            assert!(
                !part.contains(['/', '\\', '%']),
                "ambiguous documentation path"
            );
            part
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn content_type(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("")
    {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "ttf" => "font/ttf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "wasm" => "application/wasm",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(windows)]
fn embed_windows_icon() {
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon("assets/icon.ico");
    if let Err(err) = resource.compile() {
        println!("cargo:warning=failed to embed executable icon: {err}");
    }
}
