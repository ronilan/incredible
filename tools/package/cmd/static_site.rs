//! Zero-JS static site builder.
//!
//! Renders the same markdown sources the WASM app embeds (see
//! `src/ui/loader.rs`) into plain HTML pages styled by `web/style.css`.
//! No JavaScript: no theme picker, no WASM, default (Basic Dark) palette.
//!
//! Markdown files are the source of truth. `cargo run --bin package -- static`
//! regenerates `docs/` without running wasm-pack; the full `wasm` target
//! calls this too so `docs/` always contains both experiences.

use std::fs;
use std::io;
use std::path::Path;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::cargo;

/// (markdown source, html output), both repo-relative.
const PAGES: &[(&str, &str)] = &[
    ("index.md", "index.html"),
    ("blog/index.md", "blog/index.html"),
    (
        "blog/listen-to-the-redditors.md",
        "blog/listen-to-the-redditors.html",
    ),
    ("blog/hello-world.md", "blog/hello-world.html"),
    (
        "blog/tui-games-in-80x24.md",
        "blog/tui-games-in-80x24.html",
    ),
    ("markdowns/index.md", "markdowns/index.html"),
    ("markdowns/AI_POLICY.md", "markdowns/AI_POLICY.html"),
    ("markdowns/CONTRIBUTING.md", "markdowns/CONTRIBUTING.html"),
    ("markdowns/DEVELOPMENT.md", "markdowns/DEVELOPMENT.html"),
    (
        "markdowns/DEVELOPMENT_PREREQUISITES.md",
        "markdowns/DEVELOPMENT_PREREQUISITES.html",
    ),
    ("README.md", "README.html"),
];

/// Full static build: WASM shell refresh + markdown pages + images.
pub fn build() -> io::Result<()> {
    super::web::write_wasm_shell()?;
    build_static_pages()
}

/// Render every markdown page into `docs/` and copy referenced images.
pub fn build_static_pages() -> io::Result<()> {
    // Shared web assets (also copied by the full wasm build; idempotent).
    let docs = super::web::docs_dir();
    for asset in ["style.css", "favicon.svg"] {
        let from = Path::new("web").join(asset);
        if from.exists() {
            fs::copy(&from, docs.join(asset))?;
        }
    }
    let fonts_from = Path::new("web").join("fonts");
    if fonts_from.exists() {
        super::web::copy_dir_all(&fonts_from, &docs.join("fonts"))?;
    }

    for (src, out) in PAGES {
        let md = fs::read_to_string(src).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("static_site: cannot read {}: {}", src, e),
            )
        })?;
        let html = render_page(src, out, &md);
        write_file(&super::web::docs_dir().join(out), &html)?;
    }

    let images_from = Path::new("blog/images");
    if images_from.exists() {
        let images_to = super::web::docs_dir().join("blog/images");
        fs::create_dir_all(&images_to)?;
        for entry in fs::read_dir(images_from)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') {
                continue;
            }
            if entry.file_type()?.is_file() {
                fs::copy(entry.path(), images_to.join(name))?;
            }
        }
    }
    println!("Static site complete.");
    Ok(())
}

fn write_file(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

/// Repo-relative parent dir of a source path: "blog/x.md" -> "blog", "x.md" -> "".
fn parent_dir(src: &str) -> &str {
    match src.rfind('/') {
        Some(i) => &src[..i],
        None => "",
    }
}

/// "../" prefix needed from an output page to reach the site root.
fn root_prefix(out: &str) -> String {
    "../".repeat(parent_dir(out).is_empty().then_some(0).unwrap_or(
        parent_dir(out).split('/').count(),
    ))
}

/// Join a repo-relative dir and a link path, resolving "." segments.
fn join_link(dir: &str, link: &str) -> String {
    let link = link.strip_prefix("./").unwrap_or(link);
    let mut parts: Vec<&str> = Vec::new();
    if !dir.is_empty() {
        parts.extend(dir.split('/'));
    }
    for seg in link.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    parts.join("/")
}

/// Repo-relative path of `target` as seen from the directory of `from_out`.
fn rel_href(from_out: &str, target: &str) -> String {
    let from_dir = parent_dir(from_out);
    let from_parts: Vec<&str> = if from_dir.is_empty() {
        vec![]
    } else {
        from_dir.split('/').collect()
    };
    let to_parts: Vec<&str> = target.split('/').collect();
    let mut common = 0;
    while common < from_parts.len()
        && common + 1 < to_parts.len()
        && from_parts[common] == to_parts[common]
    {
        common += 1;
    }
    // common must not consume the target filename itself
    let mut href = String::new();
    for _ in common..from_parts.len() {
        href.push_str("../");
    }
    href.push_str(&to_parts[common..].join("/"));
    href
}

/// Map a repo-relative `.md` path to its generated `.html` output path.
/// `wasm.md` is the WASM app's home and has no static page of its own;
/// it aliases to the static home.
fn md_to_html(md_path: &str) -> Option<String> {
    if md_path == "wasm.md" {
        return Some("index.html".to_string());
    }
    for (src, out) in PAGES {
        if *src == md_path {
            return Some(out.to_string());
        }
    }
    None
}

fn is_external(url: &str) -> bool {
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("mailto:")
        || url.starts_with('#')
}

fn esc_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('"', "&quot;")
}

fn esc_text(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            _ => o.push(c),
        }
    }
    o
}

/// Push a grapheme cluster as text: wide clusters (emoji, CJK, flags)
/// get the framework's fixed 2-cell `.wc` span, everything else escaped raw.
fn push_grapheme(out: &mut String, g: &str) {
    if UnicodeWidthStr::width(g) >= 2 {
        out.push_str(&format!("<span class=\"wc\">{}</span>", esc_text(g)));
    } else {
        out.push_str(&esc_text(g));
    }
}

/// Escaped code text with wide clusters wrapped in `.wc` (per-cell, like the
/// HTML backend's row spans).
fn render_code_inner(s: &str) -> String {
    let mut o = String::new();
    for g in s.graphemes(true) {
        push_grapheme(&mut o, g);
    }
    o
}

/// Basic Dark RGB for an ANSI index (must match `#probes .ansi-N`).
fn ansi_hex(n: u8) -> &'static str {
    match n {
        7 => "#a8a8a8",
        8 => "#585858",
        9 => "#ff8787",
        10 => "#87ff87",
        11 => "#ffd75f",
        12 => "#00afff",
        13 => "#d7afff",
        14 => "#00ffff",
        _ => "#eeeeee",
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Split a code line into (ansi_color, text) runs. Focused tokenizer for the
/// languages our docs use; mirrors the framework's SyntaxTheme mapping
/// (comment 8, string 10, keyword/storage 13, function 12, type 11,
/// numeric 11, variable 9, fg 7).
fn highlight_line(lang: &str, line: &str) -> Vec<(u8, String)> {
    let c: Vec<char> = line.chars().collect();
    let mut runs: Vec<(u8, String)> = Vec::new();
    let cur_color: u8 = 7;
    let mut cur = String::new();
    let mut i = 0;
    let flush = |runs: &mut Vec<(u8, String)>, cur: &mut String, color: u8| {
        if !cur.is_empty() {
            runs.push((color, std::mem::take(cur)));
        }
    };
    let word_at = |c: &[char], i: usize| -> String {
        let mut j = i;
        while j < c.len() && is_ident_char(c[j]) {
            j += 1;
        }
        c[i..j].iter().collect()
    };
    // In shell languages a hyphen continues the word (xcode-select is one
    // command, not a `select` keyword); in Rust it never does.
    let prev_is_ident = |c: &[char], i: usize| -> bool {
        if i == 0 {
            return false;
        }
        let p = c[i - 1];
        is_ident_char(p) || (lang != "rust" && p == '-')
    };

    while i < c.len() {
        // Line comments (Rust uses // only; # starts attributes there).
        let is_comment = match lang {
            "rust" => c[i] == '/' && i + 1 < c.len() && c[i + 1] == '/',
            _ => c[i] == '#',
        };
        if is_comment {
            flush(&mut runs, &mut cur, cur_color);
            runs.push((8, c[i..].iter().collect()));
            break;
        }
        // Strings with backslash escapes.
        if c[i] == '"' || (c[i] == '\'' && lang != "rust") || (c[i] == '\'' && lang == "rust" && is_rust_char_literal(&c, i)) {
            flush(&mut runs, &mut cur, cur_color);
            let quote = c[i];
            let mut s = String::from(quote);
            i += 1;
            while i < c.len() {
                s.push(c[i]);
                if c[i] == '\\' && i + 1 < c.len() {
                    i += 1;
                    s.push(c[i]);
                } else if c[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            runs.push((10, s));
            continue;
        }
        // PowerShell variables.
        if lang == "powershell" && c[i] == '$' && i + 1 < c.len() && (is_ident_char(c[i + 1]) || c[i + 1] == '{') {
            flush(&mut runs, &mut cur, cur_color);
            let mut s = String::from("$");
            i += 1;
            while i < c.len() && (is_ident_char(c[i]) || c[i] == ':' || (c[i] == '}' )) {
                if c[i] == '}' {
                    s.push('}');
                    i += 1;
                    break;
                }
                s.push(c[i]);
                i += 1;
            }
            runs.push((9, s));
            continue;
        }
        // Numbers (not part of identifiers).
        if c[i].is_ascii_digit() && !prev_is_ident(&c, i) {
            flush(&mut runs, &mut cur, cur_color);
            let mut s = String::new();
            while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == '_' || c[i] == '.' || c[i] == 'x' || c[i] == 'X') {
                // Stop a trailing '.' unless it's a decimal point (digit follows).
                if c[i] == '.' && (i + 1 >= c.len() || !c[i + 1].is_ascii_digit()) {
                    break;
                }
                s.push(c[i]);
                i += 1;
            }
            runs.push((11, s));
            continue;
        }
        // Words: keywords, constants, types, calls, macros.
        if (c[i].is_alphabetic() || c[i] == '_') && !prev_is_ident(&c, i) {
            let w = word_at(&c, i);
            let after: String = c[i + w.chars().count()..].iter().take(1).collect();
            let (color, take_macro) = classify_word(lang, &w, &after);
            flush(&mut runs, &mut cur, cur_color);
            let mut s = w.clone();
            let mut len = w.chars().count();
            if take_macro {
                s.push('!');
                len += 1;
            }
            runs.push((color, s));
            i += len;
            continue;
        }
        cur.push(c[i]);
        i += 1;
    }
    flush(&mut runs, &mut cur, cur_color);
    runs
}

/// Rust `'a'` char literal vs `'a` lifetime: literal only for `'x'` or an
/// escape `'\\x'`.
fn is_rust_char_literal(c: &[char], i: usize) -> bool {
    if i + 2 < c.len() && c[i + 2] == '\'' {
        return true;
    }
    i + 3 < c.len() && c[i + 1] == '\\' && c[i + 3] == '\''
}

/// Classify an identifier: (ansi_color, consume_trailing_bang).
fn classify_word(lang: &str, w: &str, after: &str) -> (u8, bool) {
    if after == "!" {
        return (12, true); // macro invocation
    }
    match lang {
        "rust" => {
            if w == "true" || w == "false" {
                return (11, false);
            }
            if RUST_KEYWORDS.contains(&w) {
                return (13, false);
            }
            if after == "(" {
                return (12, false); // call site (incl. methods)
            }
            if w.chars().next().map(|ch| ch.is_uppercase()).unwrap_or(false) {
                return (11, false); // CamelCase type
            }
            (7, false)
        }
        "bash" | "powershell" => {
            if w == "true" || w == "false" {
                return (11, false);
            }
            let kws: &[&str] = if lang == "bash" {
                &BASH_KEYWORDS
            } else {
                &POWERSHELL_KEYWORDS
            };
            if kws.contains(&w) {
                return (13, false);
            }
            (7, false)
        }
        _ => (7, false),
    }
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "do", "dyn", "else", "enum",
    "extern", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "type",
    "unsafe", "use", "where", "while",
];

const BASH_KEYWORDS: &[&str] = &[
    "case", "coproc", "do", "done", "elif", "else", "esac", "fi", "for", "function", "if",
    "in", "select", "then", "time", "until", "while",
];

const POWERSHELL_KEYWORDS: &[&str] = &[
    "begin", "break", "catch", "continue", "do", "else", "elseif", "end", "exit", "finally",
    "for", "foreach", "function", "if", "in", "param", "process", "return", "switch", "throw",
    "try", "until", "while",
];

/// Render a fenced code line: highlighted spans for known languages, plain
/// escaped text otherwise. Wide clusters keep their `.wc` cell inside spans.
fn render_code_line(lang: &str, line: &str) -> String {
    if !matches!(lang, "rust" | "bash" | "powershell") {
        return render_code_inner(line);
    }
    let mut o = String::new();
    for (color, text) in highlight_line(lang, line) {
        if color == 7 {
            o.push_str(&render_code_inner(&text));
        } else {
            o.push_str(&format!(
                "<span style=\"color:{}\">{}</span>",
                ansi_hex(color),
                render_code_inner(&text)
            ));
        }
    }
    o
}

fn strip_tags(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => o.push(c),
            _ => {}
        }
    }
    o
}

/// Find `]` matching `[` at `open`. Returns (inner text, index past `]`).
fn parse_bracket(c: &[char], open: usize) -> Option<(String, usize)> {
    let mut depth = 0;
    let mut j = open;
    while j < c.len() {
        if c[j] == '[' {
            depth += 1;
        } else if c[j] == ']' {
            depth -= 1;
            if depth == 0 {
                return Some((c[open + 1..j].iter().collect(), j + 1));
            }
        }
        j += 1;
    }
    None
}

/// Parse `(url)` at `open` (c[open] == '('). Returns (url, index past `)`).
fn parse_paren(c: &[char], open: usize) -> Option<(String, usize)> {
    let mut j = open + 1;
    while j < c.len() && c[j] != ')' {
        j += 1;
    }
    if j < c.len() {
        Some((c[open + 1..j].iter().collect(), j + 1))
    } else {
        None
    }
}

fn find_strong_end(c: &[char], from: usize) -> Option<usize> {
    let mut j = from;
    while j + 1 < c.len() {
        if c[j] == '*' && c[j + 1] == '*' {
            return Some(j);
        }
        j += 1;
    }
    None
}

fn find_em_end(c: &[char], from: usize) -> Option<usize> {
    let mut j = from;
    while j < c.len() {
        if c[j] == '*'
            && (j == 0 || c[j - 1] != '*')
            && (j + 1 >= c.len() || c[j + 1] != '*')
        {
            return Some(j);
        }
        j += 1;
    }
    None
}

struct Doc<'a> {
    doc_dir: &'a str,
    out_path: &'a str,
}

fn resolve_local(url_path: &str, doc: &Doc) -> Option<String> {
    // Split off any #fragment (none of our sources use one, but be safe).
    let (path, frag) = match url_path.find('#') {
        Some(i) => (&url_path[..i], &url_path[i..]),
        None => (url_path, ""),
    };
    // Mirror the framework: `.md` links are app-root store keys used verbatim
    // (MarkdownRelativeLink clicks load_path(raw url)), while images and other
    // files resolve document-relative with a root fallback (resolve_image_url).
    if path.ends_with(".md") {
        let found = join_link("", path);
        let target = md_to_html(&found).unwrap_or(found);
        return Some(format!("{}{}", rel_href(doc.out_path, &target), frag));
    }
    // Prefer document-relative, fall back to repo-root-relative, then to
    // web/ (source of built pages like wasm.html, mirrored into docs/).
    let mut tried: Option<String> = None;
    for candidate in [
        join_link(doc.doc_dir, path),
        join_link("", path),
        join_link("web", path),
    ] {
        if Path::new(&candidate).exists() {
            tried = Some(candidate);
            break;
        }
    }
    let found = tried?;
    let found = found.strip_prefix("web/").unwrap_or(&found).to_string();
    Some(format!("{}{}", rel_href(doc.out_path, &found), frag))
}

fn render_link(inner: &str, url: &str, doc: &Doc) -> String {
    let label = render_inline(inner, doc);
    if is_external(url) {
        format!(
            "<a class=\"ext\" target=\"_blank\" rel=\"noopener\" href=\"{}\">{}</a>",
            esc_attr(url),
            label
        )
    } else {
        match resolve_local(url, doc) {
            Some(href) => format!(
                "<a class=\"rel\" href=\"{}\">{}</a>",
                esc_attr(&href),
                label
            ),
            None => {
                eprintln!("static_site: warning: unresolved link '{}'", url);
                format!(
                    "<a class=\"rel\" href=\"{}\">{}</a>",
                    esc_attr(url),
                    label
                )
            }
        }
    }
}

fn render_image(inner: &str, url: &str, doc: &Doc) -> String {
    let alt = esc_attr(&strip_tags(&render_inline(inner, doc)));
    let src = if is_external(url) {
        url.to_string()
    } else {
        resolve_local(url, doc).unwrap_or_else(|| {
            eprintln!("static_site: warning: unresolved image '{}'", url);
            url.to_string()
        })
    };
    format!("<img alt=\"{}\" src=\"{}\" />", alt, esc_attr(&src))
}

fn render_inline(input: &str, doc: &Doc) -> String {
    let c: Vec<char> = input.chars().collect();
    // Byte offset of each char, for slicing grapheme clusters out of `input`.
    let mut starts: Vec<usize> = Vec::with_capacity(c.len() + 1);
    for (off, _) in input.char_indices() {
        starts.push(off);
    }
    starts.push(input.len());
    let mut out = String::new();
    let mut i = 0;
    while i < c.len() {
        match c[i] {
            '~' => {
                // Strikethrough ~~text~~ (framework Strike kind).
                if i + 1 < c.len() && c[i + 1] == '~' {
                    let mut j = i + 2;
                    let mut found = None;
                    while j + 1 < c.len() {
                        if c[j] == '~' && c[j + 1] == '~' {
                            found = Some(j);
                            break;
                        }
                        j += 1;
                    }
                    match found {
                        Some(end) => {
                            let inner: String = c[i + 2..end].iter().collect();
                            out.push_str("<s>");
                            out.push_str(&render_inline(&inner, doc));
                            out.push_str("</s>");
                            i = end + 2;
                        }
                        None => {
                            out.push_str("~~");
                            i += 2;
                        }
                    }
                } else {
                    out.push('~');
                    i += 1;
                }
            }
            '`' => match c[i + 1..].iter().position(|&x| x == '`') {
                Some(end) => {
                    let inner: String = c[i + 1..i + 1 + end].iter().collect();
                    out.push_str("<code>");
                    out.push_str(&render_code_inner(&inner));
                    out.push_str("</code>");
                    i += end + 2;
                }
                None => {
                    out.push_str("&#96;");
                    i += 1;
                }
            },
            '*' => {
                if i + 1 < c.len() && c[i + 1] == '*' {
                    match find_strong_end(&c, i + 2) {
                        Some(end) => {
                            let inner: String = c[i + 2..end].iter().collect();
                            out.push_str("<span class=\"b\">");
                            out.push_str(&render_inline(&inner, doc));
                            out.push_str("</span>");
                            i = end + 2;
                        }
                        None => {
                            out.push_str("**");
                            i += 2;
                        }
                    }
                } else {
                    match find_em_end(&c, i + 1) {
                        Some(end) => {
                            let inner: String = c[i + 1..end].iter().collect();
                            out.push_str("<em>");
                            out.push_str(&render_inline(&inner, doc));
                            out.push_str("</em>");
                            i = end + 1;
                        }
                        None => {
                            out.push('*');
                            i += 1;
                        }
                    }
                }
            }
            '!' if i + 1 < c.len() && c[i + 1] == '[' => {
                match parse_bracket(&c, i + 1) {
                    Some((inner, next))
                        if next < c.len() && c[next] == '(' =>
                    {
                        match parse_paren(&c, next) {
                            Some((url, after)) => {
                                out.push_str(&render_image(&inner, &url, doc));
                                i = after;
                            }
                            None => {
                                out.push('!');
                                i += 1;
                            }
                        }
                    }
                    _ => {
                        out.push('!');
                        i += 1;
                    }
                }
            }
            '[' => match parse_bracket(&c, i) {
                Some((inner, next)) if next < c.len() && c[next] == '(' => {
                    match parse_paren(&c, next) {
                        Some((url, after)) => {
                            out.push_str(&render_link(&inner, &url, doc));
                            i = after;
                        }
                        None => {
                            out.push('[');
                            i += 1;
                        }
                    }
                }
                _ => {
                    out.push('[');
                    i += 1;
                }
            },
            '&' => {
                out.push_str("&amp;");
                i += 1;
            }
            '<' => {
                out.push_str("&lt;");
                i += 1;
            }
            '>' => {
                out.push_str("&gt;");
                i += 1;
            }
            _ => {
                // Whole grapheme cluster: keeps emoji + modifiers / regional
                // indicator pairs together; wide ones get the 2ch `.wc` cell
                // so following text lands exactly where the terminal puts it.
                let g = input[starts[i]..].graphemes(true).next().unwrap_or("");
                if g == "&" {
                    out.push_str("&amp;");
                } else if g == "<" {
                    out.push_str("&lt;");
                } else if g == ">" {
                    out.push_str("&gt;");
                } else {
                    push_grapheme(&mut out, g);
                }
                i += g.chars().count().max(1);
            }
        }
    }
    out
}

/// Block-char "Incredible" logo, composed from the same crate data the WASM
/// app uses (BlockCharsShadow Big, BlockCharsPlainSmall), with the
/// GradientLabel transform baked in (stops ansi4/5/6/4, horizontal, rest
/// frame offset 0 — no animation in the static build).
const LOGO_BIG: &str = include_str!("logo_big.txt");
const LOGO_SMALL: &str = include_str!("logo_small.txt");

/// Basic Dark ANSI stops for the logo gradient (must match the
/// `#probes .ansi-4/5/6` colors in web/style.css).
const LOGO_STOPS: [[u8; 3]; 4] = [
    [0x00, 0x5F, 0xAF],
    [0xAF, 0x5F, 0xAF],
    [0x00, 0xAF, 0xAF],
    [0x00, 0x5F, 0xAF],
];

/// Mirror `lerp_stops_t`/`position_t` (offset 0, true color): color of
/// column `x` in a `width`-column row as `#rrggbb`.
fn logo_gradient_hex(x: usize, width: usize) -> String {
    let eps = f32::EPSILON;
    let mut t = if width > 1 {
        x as f32 / (width - 1) as f32
    } else {
        0.0
    };
    t = t.min(1.0 - eps);
    let t = if (t - 1.0).abs() < eps {
        1.0
    } else {
        t.rem_euclid(1.0)
    };
    let scaled = t * (LOGO_STOPS.len() - 1) as f32;
    let lo = scaled.floor() as usize;
    let hi = (lo + 1).min(LOGO_STOPS.len() - 1);
    let frac = scaled - lo as f32;
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * frac).round() as u8;
    let [r0, g0, b0] = LOGO_STOPS[lo];
    let [r1, g1, b1] = LOGO_STOPS[hi];
    format!(
        "#{:02x}{:02x}{:02x}",
        lerp(r0, r1),
        lerp(g0, g1),
        lerp(b0, b1)
    )
}

fn logo_pre(art: &str, class: &str) -> String {
    let rows: Vec<Vec<char>> = art
        .trim_end()
        .lines()
        .map(|l| l.trim_end().chars().collect())
        .collect();
    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut out = format!(
        "<pre class=\"{}\" role=\"img\" aria-label=\"Incredible\">",
        class
    );
    for (y, row) in rows.iter().enumerate() {
        if y > 0 {
            out.push('\n');
        }
        for x in 0..width {
            match row.get(x) {
                Some(' ') | None => out.push(' '),
                Some(c) => {
                    out.push_str(&format!(
                        "<span style=\"color:{}\">{}</span>",
                        logo_gradient_hex(x, width),
                        esc_text(&c.to_string())
                    ));
                }
            }
        }
    }
    out.push_str("</pre>");
    out
}

fn logo_html() -> String {
    format!(
        "{}\n{}",
        logo_pre(LOGO_BIG, "logo-big"),
        logo_pre(LOGO_SMALL, "logo-small")
    )
}

/// Horizontal rule: one `·` per cell, exactly like
/// `HorizontalLineKind::Dotted` at the markdown width.
fn rule_html() -> String {
    format!("<div class=\"rule\">{}</div>", "·".repeat(80))
}

fn is_list_marker(trimmed: &str) -> Option<(bool, usize)> {
    // Returns (ordered, length of marker incl. trailing space).
    if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        return Some((false, 2));
    }
    let mut j = 0;
    let bytes = trimmed.as_bytes();
    while j < bytes.len() && bytes[j].is_ascii_digit() {
        j += 1;
    }
    if j > 0 && j + 1 < bytes.len() && bytes[j] == b'.' && bytes[j + 1] == b' ' {
        return Some((true, j + 2));
    }
    None
}

fn raw_html_line(trimmed: &str) -> Option<String> {
    if trimmed == "&nbsp;" {
        return Some("<div class=\"spacer\" aria-hidden=\"true\">&nbsp;</div>".to_string());
    }
    if trimmed.starts_with('<') && trimmed.ends_with('>') {
        if trimmed.starts_with("<embed") {
            if trimmed.contains("src=\"logo\"") {
                return Some(logo_html());
            }
            if trimmed.contains("src=\"moose_clicker\"") {
                return Some("<p class=\"moose\"><span class=\"wc\">🫎</span>: 0</p>".to_string());
            }
            return Some(String::new());
        }
        return Some(trimmed.to_string());
    }
    None
}

struct Blocks<'a> {
    doc: Doc<'a>,
    html: Vec<String>,
    para: Vec<String>,
    quote: Vec<String>,
    list_ordered: Option<bool>,
    list_items: Vec<String>,
    list_current: Option<String>,
    pending_code: Vec<String>,
    code_lang: Option<String>,
    code_lines: Vec<String>,
    code_in_list: bool,
}

impl<'a> Blocks<'a> {
    fn flush_para(&mut self) {
        if !self.para.is_empty() {
            // One source line = one forced row (framework keeps `\n` in the
            // paragraph source and splits rows on it), NOT a softbreak space.
            let mut rows = Vec::new();
            for line in &self.para {
                rows.push(render_inline(line, &self.doc));
            }
            self.html.push(format!("<p>{}</p>", rows.join("<br />\n")));
            self.para.clear();
        }
    }

    fn flush_quote(&mut self) {
        if !self.quote.is_empty() {
            let mut rows = Vec::new();
            for line in &self.quote {
                rows.push(render_inline(line, &self.doc));
            }
            self.html.push(format!(
                "<blockquote><p>{}</p></blockquote>",
                rows.join("<br />\n")
            ));
            self.quote.clear();
        }
    }

    fn flush_list_item(&mut self) {
        if let Some(cur) = self.list_current.take() {
            // Forced row per source line; splice fenced code blocks back in
            // raw (they were stashed behind \0 sentinels so `<pre>` survives).
            let mut rows = Vec::new();
            for line in cur.split('\n') {
                let mut rendered = String::new();
                for (k, seg) in line.split('\0').enumerate() {
                    if k % 2 == 0 {
                        rendered.push_str(&render_inline(seg.trim(), &self.doc));
                    } else if let Ok(idx) = seg.parse::<usize>() {
                        if let Some(block) = self.pending_code.get(idx) {
                            rendered.push(' ');
                            rendered.push_str(block);
                        }
                    }
                }
                rows.push(rendered.trim().to_string());
            }
            self.list_items
                .push(format!("<li>{}</li>", rows.join("<br />\n")));
        }
    }

    fn close_list(&mut self) {
        self.flush_list_item();
        if let Some(ordered) = self.list_ordered.take() {
            let tag = if ordered { "ol" } else { "ul" };
            let mut s = format!("<{}>", tag);
            for item in self.list_items.drain(..) {
                s.push_str(&item);
            }
            s.push_str(&format!("</{}>", tag));
            self.html.push(s);
        }
    }

    fn flush_code(&mut self) {
        if let Some(lang) = self.code_lang.take() {
            let lang = lang.trim().to_lowercase();
            let body = self
                .code_lines
                .iter()
                .map(|l| render_code_line(&lang, l))
                .collect::<Vec<_>>()
                .join("\n");
            self.code_lines.clear();
            let block = if lang.is_empty() {
                format!("<pre><code>{}</code></pre>", body)
            } else {
                format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>",
                    esc_attr(&lang),
                    body
                )
            };
            if self.code_in_list {
                let idx = self.pending_code.len();
                self.pending_code.push(block);
                match self.list_current.as_mut() {
                    Some(cur) => {
                        cur.push('\0');
                        cur.push_str(&idx.to_string());
                        cur.push('\0');
                    }
                    None => self.html.push(self.pending_code[idx].clone()),
                }
            } else {
                self.html.push(block);
            }
            self.code_in_list = false;
        }
    }

    fn push_list_item(&mut self, ordered: bool, text: &str) {
        if self.list_ordered != Some(ordered) {
            self.close_list();
            self.list_ordered = Some(ordered);
        } else {
            self.flush_list_item();
        }
        self.list_current = Some(text.to_string());
    }
}

fn render_blocks(src: &str, doc: Doc) -> String {
    let mut b = Blocks {
        doc,
        html: Vec::new(),
        para: Vec::new(),
        quote: Vec::new(),
        list_ordered: None,
        list_items: Vec::new(),
        list_current: None,
        pending_code: Vec::new(),
        code_lang: None,
        code_lines: Vec::new(),
        code_in_list: false,
    };

    for raw_line in src.lines() {
        let trimmed = raw_line.trim();
        let indented =
            raw_line.starts_with(' ') || raw_line.starts_with('\t');

        // Inside a fenced code block.
        if b.code_lang.is_some() {
            if trimmed.starts_with("```") {
                b.flush_code();
            } else {
                b.code_lines.push(raw_line.to_string());
            }
            continue;
        }

        // Fence open (also indented inside list items).
        if trimmed.starts_with("```") {
            b.code_in_list = b.list_current.is_some();
            b.code_lang = Some(trimmed[3..].trim().to_string());
            continue;
        }

        if trimmed.is_empty() {
            b.flush_para();
            b.flush_quote();
            continue;
        }

        // Headings.
        if trimmed.starts_with("# ") || trimmed.starts_with("## ") || trimmed.starts_with("### ") {
            b.flush_para();
            b.flush_quote();
            b.close_list();
            let level = if trimmed.starts_with("### ") {
                3
            } else if trimmed.starts_with("## ") {
                2
            } else {
                1
            };
            let text = trimmed[level + 1..].trim();
            b.html.push(format!(
                "<h{}>{}</h{}>",
                level,
                render_inline(text, &b.doc),
                level
            ));
            continue;
        }

        // Horizontal rule.
        if trimmed.len() >= 3 && trimmed.chars().all(|ch| ch == '-') {
            b.flush_para();
            b.flush_quote();
            b.close_list();
            b.html.push(rule_html());
            continue;
        }

        // Raw HTML / embeds.
        if let Some(raw) = raw_html_line(trimmed) {
            b.flush_para();
            b.flush_quote();
            b.close_list();
            if !raw.is_empty() {
                b.html.push(raw);
            }
            continue;
        }

        // Blockquote.
        if let Some(rest) = trimmed.strip_prefix('>') {
            b.flush_para();
            b.close_list();
            b.quote.push(rest.strip_prefix(' ').unwrap_or(rest).to_string());
            continue;
        }

        // List item (markers start the line, not indented).
        if !indented {
            if let Some((ordered, marker_len)) = is_list_marker(trimmed) {
                b.flush_para();
                b.flush_quote();
                b.push_list_item(ordered, trimmed[marker_len..].trim());
                continue;
            }
        }

        // Indented continuation inside a list item: new forced row.
        if indented && b.list_current.is_some() {
            match b.list_current.as_mut() {
                Some(cur) => {
                    cur.push('\n');
                    cur.push_str(trimmed);
                }
                None => {}
            }
            continue;
        }

        // Plain text.
        b.close_list();
        b.flush_quote();
        b.para.push(trimmed.to_string());
    }

    b.flush_code();
    b.flush_para();
    b.flush_quote();
    b.close_list();
    b.html.join("\n")
}

fn page_title(md: &str, fallback: &str) -> String {
    // First `# ` heading, else first `## ` heading, else the output path.
    for prefix in ["# ", "## "] {
        for line in md.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix(prefix) {
                // Skip deeper levels (e.g. "## " also matches "# " logic above
                // is avoided by checking the char after the prefix).
                if rest.starts_with('#') {
                    continue;
                }
                let doc = Doc {
                    doc_dir: "",
                    out_path: fallback,
                };
                let title = strip_tags(&render_inline(rest.trim(), &doc));
                if !title.is_empty() {
                    return title;
                }
            }
        }
    }
    fallback.to_string()
}

fn render_page(src: &str, out: &str, md: &str) -> String {
    let doc = Doc {
        doc_dir: parent_dir(src),
        out_path: out,
    };
    let body = render_blocks(md, Doc {
        doc_dir: doc.doc_dir,
        out_path: doc.out_path,
    });
    let title = esc_text(&page_title(md, out));
    let prefix = root_prefix(out);
    let version = cargo::package_version();

    format!(
        "<!DOCTYPE html>\n\
        <!-- GENERATED from {src} by the package tool (static_site). Do not edit; edit the markdown instead. -->\n\
        <html lang=\"en\">\n\
        <head>\n\
        <meta charset=\"UTF-8\" />\n\
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n\
        <title>{title}</title>\n\
        <meta name=\"version\" content=\"{version}\" />\n\
        <link rel=\"icon\" type=\"image/svg+xml\" href=\"{prefix}favicon.svg\" />\n\
        <link rel=\"stylesheet\" href=\"{prefix}style.css\" />\n\
        </head>\n\
        <body class=\"static\">\n\
        <div id=\"terminal\" class=\"static\">\n\
        <main class=\"static-doc\">\n\
        {body}\n\
        </main>\n\
        </div>\n\
        </body>\n\
        </html>\n"
    )
}
