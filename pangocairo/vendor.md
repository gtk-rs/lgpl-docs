<!-- file * -->
<!-- struct Font -->
`Font` is an interface exported by fonts for
use with Cairo. The actual type of the font will depend
on the particular font technology Cairo was compiled to use.

# Implements

[`FontExt`](trait.FontExt.html), [`pango::FontExt`](../pango/trait.FontExt.html)
<!-- trait FontExt -->
Trait containing all `Font` methods.

# Implementors

[`Font`](struct.Font.html)
<!-- trait FontExt::fn get_scaled_font -->
Gets the `cairo::ScaledFont` used by `self`.
The scaled font can be referenced and kept using
`cairo_scaled_font_reference`.

# Returns

the `cairo::ScaledFont` used by `self`,
 or `None` if `self` is `None`.
<!-- struct FontMap -->
`FontMap` is an interface exported by font maps for
use with Cairo. The actual type of the font map will depend
on the particular font technology Cairo was compiled to use.

# Implements

[`FontMapExt`](trait.FontMapExt.html), [`pango::FontMapExt`](../pango/trait.FontMapExt.html)
<!-- trait FontMapExt -->
Trait containing all `FontMap` methods.

# Implementors

[`FontMap`](struct.FontMap.html)
<!-- impl FontMap::fn get_default -->
Gets a default `FontMap` to use with Cairo.

Note that the type of the returned object will depend
on the particular font backend Cairo was compiled to use;
You generally should only use the `pango::FontMap` and
`FontMap` interfaces on the returned object.

The default Cairo fontmap can be changed by using
`FontMap::set_default`. This can be used to
change the Cairo font backend that the default fontmap
uses for example.

Note that since Pango 1.32.6, the default fontmap is per-thread.
Each thread gets its own default fontmap. In this way,
PangoCairo can be used safely from multiple threads.

# Returns

the default PangoCairo fontmap
 for the current thread. This object is owned by Pango and must not be freed.
<!-- impl FontMap::fn new -->
Creates a new `FontMap` object; a fontmap is used
to cache information about available fonts, and holds
certain global parameters such as the resolution.
In most cases, you can use `FontMap::get_default`
instead.

Note that the type of the returned object will depend
on the particular font backend Cairo was compiled to use;
You generally should only use the `pango::FontMap` and
`FontMap` interfaces on the returned object.

You can override the type of backend returned by using an
environment variable `PANGOCAIRO_BACKEND`. Supported types,
based on your build, are fc (fontconfig), win32, and coretext.
If requested type is not available, NULL is returned. Ie.
this is only useful for testing, when at least two backends
are compiled in.

# Returns

the newly allocated `pango::FontMap`,
 which should be freed with `gobject::Object::unref`.
<!-- impl FontMap::fn new_for_font_type -->
Creates a new `FontMap` object of the type suitable
to be used with cairo font backend of type `fonttype`.

In most cases one should simply use `FontMap::new`(),
or in fact in most of those cases, just use
`FontMap::get_default`().
## `fonttype`
desired `cairo::FontType`

# Returns

the newly allocated
 `pango::FontMap` of suitable type which should be freed
 with `gobject::Object::unref`, or `None` if the requested
 cairo font backend is not supported / compiled in.
<!-- trait FontMapExt::fn get_font_type -->
Gets the type of Cairo font backend that `self` uses.

# Returns

the `cairo::FontType` cairo font backend type
<!-- trait FontMapExt::fn get_resolution -->
Gets the resolution for the fontmap. See `FontMap::set_resolution`

# Returns

the resolution in "dots per inch"
<!-- trait FontMapExt::fn set_default -->
Sets a default `FontMap` to use with Cairo.

This can be used to change the Cairo font backend that the
default fontmap uses for example. The old default font map
is unreffed and the new font map referenced.

Note that since Pango 1.32.6, the default fontmap is per-thread.
This function only changes the default fontmap for
the current thread. Default fontmaps of exisiting threads
are not changed. Default fontmaps of any new threads will
still be created using `FontMap::new`.

A value of `None` for `self` will cause the current default
font map to be released and a new default font
map to be created on demand, using `FontMap::new`.
<!-- trait FontMapExt::fn set_resolution -->
Sets the resolution for the fontmap. This is a scale factor between
points specified in a `pango::FontDescription` and Cairo units. The
default value is 96, meaning that a 10 point font will be 13
units high. (10 * 96. / 72. = 13.3).
## `dpi`
the resolution in "dots per inch". (Physical inches aren't actually
 involved; the terminology is conventional.)
