extern crate stripper_lib;

pub const CAIRO_DOCS: &'static str = include_str!("../cairo/comments.cmts");
pub const GDK_DOCS: &'static str = include_str!("../gdk/comments.cmts");
pub const GLIB_DOCS: &'static str = include_str!("../glib/comments.cmts");
pub const GTK_DOCS: &'static str = include_str!("../gtk/comments.cmts");
pub const PANGO_DOCS: &'static str = include_str!("../pango/comments.cmts");

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
    let mut infos = parse_cmts(docs.lines());
    loop_over_files(path.as_ref(), &mut infos, &regenerate_comments, &ignores, false);
}

/// Remove any doc comments.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn purge<P: AsRef<Path>>(path: P, ignores: &[&str]) {
    loop_over_files(path.as_ref(), &mut io::sink(), &|w, s, d| strip_comments(w, s, d, true),
        &ignores, false);
}
