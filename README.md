# lgpl-docs

LGPL-licensed docs for Gtk-rs crates.

The plan is to generate docs from [GIR](https://github.com/gtk-rs/gir-files)
first then fix and expand them manually.

We're currently in the first phase: no manual corrections while we keep
improving the [generator](https://github.com/gtk-rs/gir) and regenerating
these.

The above does not apply to `cairo.md`: cairo doesn't have GIR definitions,
there's nothing to generate.
