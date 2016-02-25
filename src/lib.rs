extern crate stripper_lib;

pub const CAIRO_DOCS: &'static str = include_str!("../cairo.md");
pub const GDK_DOCS: &'static str = include_str!("../gdk.md");
pub const GDK_PIXBUF_DOCS: &'static str = include_str!("../gdk-pixbuf.md");
pub const GLIB_DOCS: &'static str = include_str!("../glib.md");
pub const GTK_DOCS: &'static str = include_str!("../gtk.md");
pub const PANGO_DOCS: &'static str = include_str!("../pango.md");

use std::io;
use std::path::Path;
use stripper_lib::{
    loop_over_files,
    parse_cmts,
    regenerate_comments,
    strip_comments,
};

/// Embeds the docs from `docs`.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn embed<P: AsRef<Path>>(docs: &str, path: P, ignores: &[&str]) {
    let mut infos = parse_cmts(docs.lines(), true);
    loop_over_files(path.as_ref(), &mut |w, s| regenerate_comments(w, s, &mut infos, true),
        &ignores, false);
}

/// Remove any doc comments.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn purge<P: AsRef<Path>>(path: P, ignores: &[&str]) {
    loop_over_files(path.as_ref(), &mut |w, s| strip_comments(w, s, &mut io::sink(), true),
        &ignores, false);
}
