extern crate stripper_lib;

pub enum Library {
    Cairo,
    Gdk,
    GdkPixbuf,
    Glib,
    Gtk,
    Pango,
}

fn gir_docs(lib: Library) -> &'static str {
    match lib {
        Library::Cairo => include_str!("../cairo.md"),
        Library::Gdk => include_str!("../gdk.md"),
        Library::GdkPixbuf => include_str!("../gdk-pixbuf.md"),
        Library::Glib => include_str!("../glib.md"),
        Library::Gtk => include_str!("../gtk.md"),
        Library::Pango => include_str!("../pango.md"),
    }
}

use std::io;
use std::path::Path;
use stripper_lib::{
    loop_over_files,
    parse_cmts,
    regenerate_comments,
    strip_comments,
};

/// Embeds the docs.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn embed<P: AsRef<Path>>(library: Library, path: P, ignores: &[&str]) {
    let mut infos = parse_cmts(gir_docs(library).lines(), true);
    loop_over_files(path.as_ref(), &mut |w, s| regenerate_comments(w, s, &mut infos, true, true),
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
