//! Shell helpers for the static-site tool: `docs/` layout, recursive copy,
//! and the WASM shell writer (`web/wasm.html` -> `docs/wasm.html` with
//! package metadata injected).

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::cargo::{self, HtmlMetadata};

pub fn docs_dir() -> PathBuf {
    PathBuf::from("docs")
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn inject_html(html: &str, meta: &HtmlMetadata) -> String {
    let mut out = html.to_string();

    if !meta.title.is_empty() {
        if let Some(start) = out.find("<title>") {
            if let Some(rel) = out[start..].find("</title>") {
                let end = start + rel + "</title>".len();
                out.replace_range(start..end, &format!("<title>{}</title>", meta.title));
            }
        }
    }

    if !meta.min_width.is_empty() {
        out = replace_attr(&out, "data-min-width", &meta.min_width);
    }
    if !meta.min_height.is_empty() {
        out = replace_attr(&out, "data-min-height", &meta.min_height);
    }

    let mut extra = String::new();
    if !meta.description.is_empty() {
        extra.push_str(&format!(
            "<meta name=\"description\" content=\"{}\">\n",
            meta.description
        ));
    }
    if !meta.keywords.is_empty() {
        extra.push_str(&format!(
            "<meta name=\"keywords\" content=\"{}\">\n",
            meta.keywords
        ));
    }
    extra.push_str(&format!(
        "<meta name=\"version\" content=\"{}\">\n",
        meta.version
    ));

    if let Some(pos) = out.find("</head>") {
        out.insert_str(pos, &extra);
    }

    out
}

fn replace_attr(html: &str, attr: &str, value: &str) -> String {
    let needle = format!("{}=\"", attr);
    let out = html.to_string();
    let chars: Vec<char> = out.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let mut i = 0;
    let mut result = String::new();
    let mut replaced = false;
    while i < chars.len() && !replaced {
        if chars[i..].starts_with(&needle_chars[..]) {
            let start = i;
            let mut j = i + needle_chars.len();
            while j < chars.len() && chars[j] != '"' {
                j += 1;
            }
            result.push_str(&chars[..start].iter().collect::<String>());
            result.push_str(&needle);
            result.push_str(value);
            result.push('"');
            result.push_str(&chars[j + 1..].iter().collect::<String>());
            replaced = true;
            break;
        }
        i += 1;
    }
    if replaced { result } else { out }
}

/// Write the WASM shell (`web/wasm.html` -> `docs/wasm.html`) with
/// package metadata injected.
pub fn write_wasm_shell() -> io::Result<()> {
    let meta = cargo::html_metadata();
    let index_from = match fs::read_to_string(Path::new("web").join("wasm.html")) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let injected = inject_html(&index_from, &meta);
    write_file(&docs_dir().join("wasm.html"), &injected)?;
    Ok(())
}
