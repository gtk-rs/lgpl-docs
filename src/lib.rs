extern crate stripper_lib;

use std::io;
use std::path::Path;
use stripper_lib::{loop_over_files, parse_cmts, regenerate_comments, strip_comments};

#[derive(Clone, Copy, Debug)]
pub enum Library {
    Atk,
    Cairo,
    Gdk,
    Gdk4,
    Gdk4X11,
    Gdk4Wayland,
    GdkPixbuf,
    Gio,
    Glib,
    Graphene,
    Gsk4,
    Gtk,
    Gtk4,
    Pango,
    PangoCairo,
    SourceView,
}

fn docs(lib: Library) -> Option<&'static str> {
    match lib {
        Library::Glib => Some(include_str!("../glib/docs.md")),
        Library::Gio => Some(include_str!("../gio/docs.md")),
        Library::Cairo => Some(include_str!("../cairo/docs.md")),
        Library::Gdk => Some(include_str!("../gdk/docs.md")),
        Library::Gdk4 => Some(include_str!("../gdk4/docs.md")),
        Library::Gdk4X11 => Some(include_str!("../gdk4-x11/docs.md")),
        Library::Gdk4Wayland => Some(include_str!("../gdk4-wayland/docs.md")),
        Library::GdkPixbuf => Some(include_str!("../gdk-pixbuf/docs.md")),
        Library::Graphene => Some(include_str!("../graphene/docs.md")),
        Library::Gsk4 => Some(include_str!("../gsk4/docs.md")),
        Library::Gtk => Some(include_str!("../gtk/docs.md")),
        Library::Gtk4 => Some(include_str!("../gtk4/docs.md")),
        _ => None,
    }
}

fn vendor_docs(lib: Library) -> Option<&'static str> {
    match lib {
        Library::Atk => Some(include_str!("../atk/vendor.md")),
        Library::Glib => Some(include_str!("../glib/vendor.md")),
        Library::Gio => Some(include_str!("../gio/vendor.md")),
        Library::Pango => Some(include_str!("../pango/vendor.md")),
        Library::Gdk => Some(include_str!("../gdk/vendor.md")),
        Library::Gdk4 => Some(include_str!("../gdk4/vendor.md")),
        Library::Gdk4X11 => Some(include_str!("../gdk4-wayland/vendor.md")),
        Library::Gdk4Wayland => Some(include_str!("../gdk4-wayland/vendor.md")),
        Library::GdkPixbuf => Some(include_str!("../gdk-pixbuf/vendor.md")),
        Library::Graphene => Some(include_str!("../graphene/vendor.md")),
        Library::Gsk4 => Some(include_str!("../gsk4/vendor.md")),
        Library::Gtk => Some(include_str!("../gtk/vendor.md")),
        Library::Gtk4 => Some(include_str!("../gtk4/vendor.md")),
        Library::SourceView => Some(include_str!("../sourceview/vendor.md")),
        Library::PangoCairo => Some(include_str!("../pangocairo/vendor.md")),
        _ => None,
    }
}

/// Embeds the docs.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn embed<P: AsRef<Path>>(library: Library, path: P, ignores: &[&str]) {
    if let Some(docs) = docs(library) {
        do_embed(docs, path.as_ref(), ignores);
    }
    if let Some(docs) = vendor_docs(library) {
        do_embed(docs, path.as_ref(), ignores);
    }
}

fn do_embed(docs: &str, path: &Path, ignores: &[&str]) {
    let mut infos = parse_cmts(docs.lines(), true);
    loop_over_files(
        path,
        &mut |w, s| regenerate_comments(w, s, &mut infos, true, true),
        &ignores,
        false,
    );
}

/// Remove any doc comments.
///
/// `path` is the root directory to process.
///
/// `ignores` is the list of files to skip (relative to `path`).
pub fn purge<P: AsRef<Path>>(path: P, ignores: &[&str]) {
    loop_over_files(
        path.as_ref(),
        &mut |w, s| strip_comments(w, s, &mut io::sink(), true),
        &ignores,
        false,
    );
}
