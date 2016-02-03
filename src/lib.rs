extern crate stripper_lib;

pub const CAIRO_DOCS: &'static str = include_str!("../cairo/comments.cmts");
pub const GDK_DOCS: &'static str = include_str!("../gdk/comments.cmts");
pub const GLIB_DOCS: &'static str = include_str!("../glib/comments.cmts");
pub const GTK_DOCS: &'static str = include_str!("../gtk/comments.cmts");
pub const PANGO_DOCS: &'static str = include_str!("../pango/comments.cmts");

use std::io;
use stripper_lib::{
    loop_over_files,
    parse_cmts,
    regenerate_comments,
    strip_comments,
};

pub fn embed(docs: &str, path: &str, ignores: &[&str]) {
    let mut infos = parse_cmts(docs.lines());
    loop_over_files(path, &mut infos, &regenerate_comments, &ignores, false);
}

pub fn purge(path: &str, ignores: &[&str]) {
    loop_over_files(path, &mut io::sink(), &strip_comments, &ignores, false);
}
