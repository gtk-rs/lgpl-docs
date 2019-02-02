<!-- file * -->
<!-- enum Alignment -->
A `Alignment` describes how to align the lines of a `Layout` within the
available space. If the `Layout` is set to justify
using `LayoutExt::set_justify`, this only has effect for partial lines.
<!-- enum Alignment::variant Left -->
Put all available space on the right
<!-- enum Alignment::variant Center -->
Center the line within the available space
<!-- enum Alignment::variant Right -->
Put all available space on the left
<!-- struct Analysis -->
The `Analysis` structure stores information about
the properties of a segment of text.
<!-- struct AttrClass -->
The `AttrClass` structure stores the type and operations for
a particular type of attribute. The functions in this structure should
not be called directly. Instead, one should use the wrapper functions
provided for `Attribute`.
<!-- struct AttrIterator -->
The `AttrIterator` structure is used to represent an
iterator through a `AttrList`. A new iterator is created
with `AttrList::get_iterator`. Once the iterator
is created, it can be advanced through the style changes
in the text using `AttrIterator::next`. At each
style change, the range of the current style segment and the
attributes currently in effect can be queried.
<!-- impl AttrIterator::fn copy -->
Copy a `AttrIterator`

# Returns

the newly allocated
 `AttrIterator`, which should be freed with
 `AttrIterator::destroy`.
<!-- impl AttrIterator::fn destroy -->
Destroy a `AttrIterator` and free all associated memory.
<!-- impl AttrIterator::fn get -->
Find the current attribute of a particular type at the iterator
location. When multiple attributes of the same type overlap,
the attribute whose range starts closest to the current location
is used.
## `type_`
the type of attribute to find.

# Returns

the current attribute of the given type,
 or `None` if no attribute of that type applies to the
 current location.
<!-- impl AttrIterator::fn get_attrs -->
Gets a list of all attributes at the current position of the
iterator.

# Returns

a list of
 all attributes for the current range.
 To free this value, call `Attribute::destroy` on
 each value and `glib::SList::free` on the list.
<!-- impl AttrIterator::fn get_font -->
Get the font and other attributes at the current iterator position.
## `desc`
a `FontDescription` to fill in with the current values.
 The family name in this structure will be set using
 `FontDescription::set_family_static` using values from
 an attribute in the `AttrList` associated with the iterator,
 so if you plan to keep it around, you must call:
 `<literal>`pango_font_description_set_family (desc, pango_font_description_get_family (desc))`</literal>`.
## `language`
if non-`None`, location to store language tag for item, or `None`
 if none is found.
## `extra_attrs`
if non-`None`,
 location in which to store a list of non-font
 attributes at the the current position; only the highest priority
 value of each attribute will be added to this list. In order
 to free this value, you must call `Attribute::destroy` on
 each member.
<!-- impl AttrIterator::fn next -->
Advance the iterator until the next change of style.

# Returns

`false` if the iterator is at the end of the list, otherwise `true`
<!-- impl AttrIterator::fn range -->
Get the range of the current segment. Note that the
stored return values are signed, not unsigned like
the values in `Attribute`. To deal with this API
oversight, stored return values that wouldn't fit into
a signed integer are clamped to `G_MAXINT`.
## `start`
location to store the start of the range
## `end`
location to store the end of the range
<!-- struct AttrList -->
The `AttrList` structure represents a list of attributes
that apply to a section of text. The attributes are, in general,
allowed to overlap in an arbitrary fashion, however, if the
attributes are manipulated only through `AttrList::change`,
the overlap between properties will meet stricter criteria.

Since the `AttrList` structure is stored as a linear list,
it is not suitable for storing attributes for large amounts
of text. In general, you should not use a single `AttrList`
for more than one paragraph of text.
<!-- impl AttrList::fn new -->
Create a new empty attribute list with a reference count of one.

# Returns

the newly allocated `AttrList`,
 which should be freed with `AttrList::unref`.
<!-- impl AttrList::fn change -->
Insert the given attribute into the `AttrList`. It will
replace any attributes of the same type on that segment
and be merged with any adjoining attributes that are identical.

This function is slower than `AttrList::insert` for
creating a attribute list in order (potentially much slower
for large lists). However, `AttrList::insert` is not
suitable for continually changing a set of attributes
since it never removes or combines existing attributes.
## `attr`
the attribute to insert. Ownership of this
 value is assumed by the list.
<!-- impl AttrList::fn copy -->
Copy `self` and return an identical new list.

# Returns

the newly allocated `AttrList`, with a
 reference count of one, which should
 be freed with `AttrList::unref`.
 Returns `None` if `self` was `None`.
<!-- impl AttrList::fn filter -->
Given a `AttrList` and callback function, removes any elements
of `self` for which `func` returns `true` and inserts them into
a new list.
## `func`
callback function; returns `true`
 if an attribute should be filtered out.
## `data`
Data to be passed to `func`

# Returns

the new `AttrList` or
 `None` if no attributes of the given types were found.
<!-- impl AttrList::fn get_iterator -->
Create a iterator initialized to the beginning of the list.
`self` must not be modified until this iterator is freed.

# Returns

the newly allocated `AttrIterator`, which should
 be freed with `AttrIterator::destroy`.
<!-- impl AttrList::fn insert -->
Insert the given attribute into the `AttrList`. It will
be inserted after all other attributes with a matching
`start_index`.
## `attr`
the attribute to insert. Ownership of this
 value is assumed by the list.
<!-- impl AttrList::fn insert_before -->
Insert the given attribute into the `AttrList`. It will
be inserted before all other attributes with a matching
`start_index`.
## `attr`
the attribute to insert. Ownership of this
 value is assumed by the list.
<!-- impl AttrList::fn ref -->
Increase the reference count of the given attribute list by one.

# Returns

The attribute list passed in
<!-- impl AttrList::fn splice -->
This function opens up a hole in `self`, fills it in with attributes from
the left, and then merges `other` on top of the hole.

This operation is equivalent to stretching every attribute
that applies at position `pos` in `self` by an amount `len`,
and then calling `AttrList::change` with a copy
of each attribute in `other` in sequence (offset in position by `pos`).

This operation proves useful for, for instance, inserting
a pre-edit string in the middle of an edit buffer.
## `other`
another `AttrList`
## `pos`
the position in `self` at which to insert `other`
## `len`
the length of the spliced segment. (Note that this
 must be specified since the attributes in `other`
 may only be present at some subsection of this range)
<!-- impl AttrList::fn unref -->
Decrease the reference count of the given attribute list by one.
If the result is zero, free the attribute list and the attributes
it contains.
<!-- enum AttrType -->
The `AttrType`
distinguishes between different types of attributes. Along with the
predefined values, it is possible to allocate additional values
for custom attributes using `AttrType::register`. The predefined
values are given below. The type of structure used to store the
attribute is listed in parentheses after the description.
<!-- enum AttrType::variant Invalid -->
does not happen
<!-- enum AttrType::variant Language -->
language (`AttrLanguage`)
<!-- enum AttrType::variant Family -->
font family name list (`AttrString`)
<!-- enum AttrType::variant Style -->
font slant style (`AttrInt`)
<!-- enum AttrType::variant Weight -->
font weight (`AttrInt`)
<!-- enum AttrType::variant Variant -->
font variant (normal or small caps) (`AttrInt`)
<!-- enum AttrType::variant Stretch -->
font stretch (`AttrInt`)
<!-- enum AttrType::variant Size -->
font size in points scaled by `PANGO_SCALE` (`AttrInt`)
<!-- enum AttrType::variant FontDesc -->
font description (`AttrFontDesc`)
<!-- enum AttrType::variant Foreground -->
foreground color (`AttrColor`)
<!-- enum AttrType::variant Background -->
background color (`AttrColor`)
<!-- enum AttrType::variant Underline -->
whether the text has an underline (`AttrInt`)
<!-- enum AttrType::variant Strikethrough -->
whether the text is struck-through (`AttrInt`)
<!-- enum AttrType::variant Rise -->
baseline displacement (`AttrInt`)
<!-- enum AttrType::variant Shape -->
shape (`AttrShape`)
<!-- enum AttrType::variant Scale -->
font size scale factor (`AttrFloat`)
<!-- enum AttrType::variant Fallback -->
whether fallback is enabled (`AttrInt`)
<!-- enum AttrType::variant LetterSpacing -->
letter spacing (`AttrInt`)
<!-- enum AttrType::variant UnderlineColor -->
underline color (`AttrColor`)
<!-- enum AttrType::variant StrikethroughColor -->
strikethrough color (`AttrColor`)
<!-- enum AttrType::variant AbsoluteSize -->
font size in pixels scaled by `PANGO_SCALE` (`AttrInt`)
<!-- enum AttrType::variant Gravity -->
base text gravity (`AttrInt`)
<!-- enum AttrType::variant GravityHint -->
gravity hint (`AttrInt`)
<!-- enum AttrType::variant FontFeatures -->
OpenType font features (`AttrString`). Since 1.38
<!-- enum AttrType::variant ForegroundAlpha -->
foreground alpha (`AttrInt`). Since 1.38
<!-- enum AttrType::variant BackgroundAlpha -->
background alpha (`AttrInt`). Since 1.38
<!-- struct Attribute -->
The `Attribute` structure represents the common portions of all
attributes. Particular types of attributes include this structure
as their initial portion. The common portion of the attribute holds
the range to which the value in the type-specific part of the attribute
applies and should be initialized using `Attribute::init`.
By default an attribute will have an all-inclusive range of [0,`G_MAXUINT`].
<!-- impl Attribute::fn copy -->
Make a copy of an attribute.

# Returns

the newly allocated `Attribute`,
 which should be freed with `Attribute::destroy`.
<!-- impl Attribute::fn destroy -->
Destroy a `Attribute` and free all associated memory.
<!-- impl Attribute::fn equal -->
Compare two attributes for equality. This compares only the
actual value of the two attributes and not the ranges that the
attributes apply to.
## `attr2`
another `Attribute`

# Returns

`true` if the two attributes have the same value.
<!-- impl Attribute::fn init -->
Initializes `self`'s klass to `klass`,
it's start_index to `PANGO_ATTR_INDEX_FROM_TEXT_BEGINNING`
and end_index to `PANGO_ATTR_INDEX_TO_TEXT_END`
such that the attribute applies
to the entire text by default.
## `klass`
a `AttrClass`
<!-- enum BidiType -->
The `BidiType` type represents the bidirectional character
type of a Unicode character as specified by the
<ulink url="http://www.unicode.org/reports/tr9/">Unicode bidirectional algorithm`</ulink>`.
<!-- enum BidiType::variant L -->
Left-to-Right
<!-- enum BidiType::variant Lre -->
Left-to-Right Embedding
<!-- enum BidiType::variant Lro -->
Left-to-Right Override
<!-- enum BidiType::variant R -->
Right-to-Left
<!-- enum BidiType::variant Al -->
Right-to-Left Arabic
<!-- enum BidiType::variant Rle -->
Right-to-Left Embedding
<!-- enum BidiType::variant Rlo -->
Right-to-Left Override
<!-- enum BidiType::variant Pdf -->
Pop Directional Format
<!-- enum BidiType::variant En -->
European Number
<!-- enum BidiType::variant Es -->
European Number Separator
<!-- enum BidiType::variant Et -->
European Number Terminator
<!-- enum BidiType::variant An -->
Arabic Number
<!-- enum BidiType::variant Cs -->
Common Number Separator
<!-- enum BidiType::variant Nsm -->
Nonspacing Mark
<!-- enum BidiType::variant Bn -->
Boundary Neutral
<!-- enum BidiType::variant B -->
Paragraph Separator
<!-- enum BidiType::variant S -->
Segment Separator
<!-- enum BidiType::variant Ws -->
Whitespace
<!-- enum BidiType::variant On -->
Other Neutrals
<!-- struct Color -->
The `Color` structure is used to
represent a color in an uncalibrated RGB color-space.
<!-- impl Color::fn copy -->
Creates a copy of `self`, which should be freed with
`Color::free`. Primarily used by language bindings,
not that useful otherwise (since colors can just be copied
by assignment in C).

# Returns

the newly allocated `Color`, which
 should be freed with `Color::free`, or `None` if
 `self` was `None`.
<!-- impl Color::fn free -->
Frees a color allocated by `Color::copy`.
<!-- impl Color::fn parse -->
Fill in the fields of a color from a string specification. The
string can either one of a large set of standard names. (Taken
from the CSS <ulink url="http://dev.w3.org/csswg/css-color/`named`-colors">specification`</ulink>`), or it can be a hexadecimal
value in the
form '&num;rgb' '&num;rrggbb' '&num;rrrgggbbb' or '&num;rrrrggggbbbb' where
'r', 'g' and 'b' are hex digits of the red, green, and blue
components of the color, respectively. (White in the four
forms is '&num;fff' '&num;ffffff' '&num;fffffffff' and '&num;ffffffffffff')
## `spec`
a string specifying the new color

# Returns

`true` if parsing of the specifier succeeded,
 otherwise false.
<!-- impl Color::fn to_string -->
Returns a textual specification of `self` in the hexadecimal form
`<literal>`&num;rrrrggggbbbb`</literal>`, where `<literal>`r`</literal>`,
`<literal>`g`</literal>` and `<literal>`b`</literal>` are hex digits representing
the red, green, and blue components respectively.

# Returns

a newly-allocated text string that must be freed with `g_free`.
<!-- struct Context -->
The `Context` structure stores global information
used to control the itemization process.

# Implements

[`ContextExt`](trait.ContextExt.html)
<!-- trait ContextExt -->
Trait containing all `Context` methods.

# Implementors

[`Context`](struct.Context.html)
<!-- impl Context::fn new -->
Creates a new `Context` initialized to default values.

This function is not particularly useful as it should always
be followed by a `ContextExt::set_font_map` call, and the
function `FontMapExt::create_context` does these two steps
together and hence users are recommended to use that.

If you are using Pango as part of a higher-level system,
that system may have it's own way of create a `Context`.
For instance, the GTK+ toolkit has, among others,
`gdk_pango_context_get_for_screen`, and
`gtk_widget_get_pango_context`. Use those instead.

# Returns

the newly allocated `Context`, which should
 be freed with `gobject::Object::unref`.
<!-- trait ContextExt::fn changed -->
Forces a change in the context, which will cause any `Layout`
using this context to re-layout.

This function is only useful when implementing a new backend
for Pango, something applications won't do. Backends should
call this function if they have attached extra data to the context
and such data is changed.

Feature: `v1_32_4`

<!-- trait ContextExt::fn get_base_dir -->
Retrieves the base direction for the context. See
`ContextExt::set_base_dir`.

# Returns

the base direction for the context.
<!-- trait ContextExt::fn get_base_gravity -->
Retrieves the base gravity for the context. See
`ContextExt::set_base_gravity`.

# Returns

the base gravity for the context.
<!-- trait ContextExt::fn get_font_description -->
Retrieve the default font description for the context.

# Returns

a pointer to the context's default font
 description. This value must not be modified or freed.
<!-- trait ContextExt::fn get_font_map -->
Gets the `FontMap` used to look up fonts for this context.

# Returns

the font map for the `Context`.
 This value is owned by Pango and should not be unreferenced.
<!-- trait ContextExt::fn get_gravity -->
Retrieves the gravity for the context. This is similar to
`ContextExt::get_base_gravity`, except for when the base gravity
is `Gravity::Auto` for which `Gravity::get_for_matrix` is used
to return the gravity from the current context matrix.

# Returns

the resolved gravity for the context.
<!-- trait ContextExt::fn get_gravity_hint -->
Retrieves the gravity hint for the context. See
`ContextExt::set_gravity_hint` for details.

# Returns

the gravity hint for the context.
<!-- trait ContextExt::fn get_language -->
Retrieves the global language tag for the context.

# Returns

the global language tag.
<!-- trait ContextExt::fn get_matrix -->
Gets the transformation matrix that will be applied when
rendering with this context. See `ContextExt::set_matrix`.

# Returns

the matrix, or `None` if no matrix has
 been set (which is the same as the identity matrix). The returned
 matrix is owned by Pango and must not be modified or freed.
<!-- trait ContextExt::fn get_metrics -->
Get overall metric information for a particular font
description. Since the metrics may be substantially different for
different scripts, a language tag can be provided to indicate that
the metrics should be retrieved that correspond to the script(s)
used by that language.

The `FontDescription` is interpreted in the same way as
by `pango_itemize`, and the family name may be a comma separated
list of figures. If characters from multiple of these families
would be used to render the string, then the returned fonts would
be a composite of the metrics for the fonts loaded for the
individual families.
## `desc`
a `FontDescription` structure. `None` means that the
 font description from the context will be used.
## `language`
language tag used to determine which script to get
 the metrics for. `None` means that the language tag from the context
 will be used. If no language tag is set on the context, metrics
 for the default language (as determined by `Language::get_default`)
 will be returned.

# Returns

a `FontMetrics` object. The caller must call `FontMetrics::unref`
 when finished using the object.
<!-- trait ContextExt::fn get_serial -->
Returns the current serial number of `self`. The serial number is
initialized to an small number larger than zero when a new context
is created and is increased whenever the context is changed using any
of the setter functions, or the `FontMap` it uses to find fonts has
changed. The serial may wrap, but will never have the value 0. Since it
can wrap, never compare it with "less than", always use "not equals".

This can be used to automatically detect changes to a `Context`, and
is only useful when implementing objects that need update when their
`Context` changes, like `Layout`.

Feature: `v1_32_4`


# Returns

The current serial number of `self`.
<!-- trait ContextExt::fn list_families -->
List all families for a context.
## `families`
location to store a pointer to
 an array of `FontFamily` *. This array should be freed
 with `g_free`.
## `n_families`
location to store the number of elements in `descs`
<!-- trait ContextExt::fn load_font -->
Loads the font in one of the fontmaps in the context
that is the closest match for `desc`.
## `desc`
a `FontDescription` describing the font to load

# Returns

the newly allocated `Font`
 that was loaded, or `None` if no font matched.
<!-- trait ContextExt::fn load_fontset -->
Load a set of fonts in the context that can be used to render
a font matching `desc`.
## `desc`
a `FontDescription` describing the fonts to load
## `language`
a `Language` the fonts will be used for

# Returns

the newly allocated
 `Fontset` loaded, or `None` if no font matched.
<!-- trait ContextExt::fn set_base_dir -->
Sets the base direction for the context.

The base direction is used in applying the Unicode bidirectional
algorithm; if the `direction` is `Direction::Ltr` or
`Direction::Rtl`, then the value will be used as the paragraph
direction in the Unicode bidirectional algorithm. A value of
`Direction::WeakLtr` or `Direction::WeakRtl` is used only
for paragraphs that do not contain any strong characters themselves.
## `direction`
the new base direction
<!-- trait ContextExt::fn set_base_gravity -->
Sets the base gravity for the context.

The base gravity is used in laying vertical text out.
## `gravity`
the new base gravity
<!-- trait ContextExt::fn set_font_description -->
Set the default font description for the context
## `desc`
the new pango font description
<!-- trait ContextExt::fn set_font_map -->
Sets the font map to be searched when fonts are looked-up in this context.
This is only for internal use by Pango backends, a `Context` obtained
via one of the recommended methods should already have a suitable font map.
## `font_map`
the `FontMap` to set.
<!-- trait ContextExt::fn set_gravity_hint -->
Sets the gravity hint for the context.

The gravity hint is used in laying vertical text out, and is only relevant
if gravity of the context as returned by `ContextExt::get_gravity`
is set `Gravity::East` or `Gravity::West`.
## `hint`
the new gravity hint
<!-- trait ContextExt::fn set_language -->
Sets the global language tag for the context. The default language
for the locale of the running process can be found using
`Language::get_default`.
## `language`
the new language tag.
<!-- trait ContextExt::fn set_matrix -->
Sets the transformation matrix that will be applied when rendering
with this context. Note that reported metrics are in the user space
coordinates before the application of the matrix, not device-space
coordinates after the application of the matrix. So, they don't scale
with the matrix, though they may change slightly for different
matrices, depending on how the text is fit to the pixel grid.
## `matrix`
a `Matrix`, or `None` to unset any existing
matrix. (No matrix set is the same as setting the identity matrix.)
<!-- struct Coverage -->
The `Coverage` structure represents a map from Unicode characters
to `CoverageLevel`. It is an opaque structure with no public fields.
<!-- impl Coverage::fn copy -->
Copy an existing `Coverage`. (This function may now be unnecessary
since we refcount the structure. File a bug if you use it.)

# Returns

the newly allocated `Coverage`,
 with a reference count of one, which should be freed
 with `Coverage::unref`.
<!-- impl Coverage::fn get -->
Determine whether a particular index is covered by `self`
## `index_`
the index to check

# Returns

the coverage level of `self` for character `index_`.
<!-- impl Coverage::fn max -->
Set the coverage for each index in `self` to be the max (better)
value of the current coverage for the index and the coverage for
the corresponding index in `other`.
## `other`
another `Coverage`
<!-- impl Coverage::fn ref -->
Increase the reference count on the `Coverage` by one

# Returns

`self`
<!-- impl Coverage::fn set -->
Modify a particular index within `self`
## `index_`
the index to modify
## `level`
the new level for `index_`
<!-- impl Coverage::fn to_bytes -->
Convert a `Coverage` structure into a flat binary format
## `bytes`

 location to store result (must be freed with `g_free`)
## `n_bytes`
location to store size of result
<!-- impl Coverage::fn unref -->
Decrease the reference count on the `Coverage` by one.
If the result is zero, free the coverage and all associated memory.
<!-- impl Coverage::fn from_bytes -->
Convert data generated from `Coverage::to_bytes` back
to a `Coverage`
## `bytes`
binary data
 representing a `Coverage`
## `n_bytes`
the size of `bytes` in bytes

# Returns

a newly allocated
 `Coverage`, or `None` if the data was invalid.
<!-- impl Coverage::fn new -->
Create a new `Coverage`

# Returns

the newly allocated `Coverage`,
 initialized to `CoverageLevel::None`
 with a reference count of one, which
 should be freed with `Coverage::unref`.
<!-- enum CoverageLevel -->
Used to indicate how well a font can represent a particular Unicode
character point for a particular script.
<!-- enum CoverageLevel::variant None -->
The character is not representable with the font.
<!-- enum CoverageLevel::variant Fallback -->
The character is represented in a way that may be
comprehensible but is not the correct graphical form.
For instance, a Hangul character represented as a
a sequence of Jamos, or a Latin transliteration of a Cyrillic word.
<!-- enum CoverageLevel::variant Approximate -->
The character is represented as basically the correct
graphical form, but with a stylistic variant inappropriate for
the current script.
<!-- enum CoverageLevel::variant Exact -->
The character is represented as the correct graphical form.
<!-- enum Direction -->
The `Direction` type represents a direction in the
Unicode bidirectional algorithm; not every value in this
enumeration makes sense for every usage of `Direction`;
for example, the return value of `pango_unichar_direction`
and `pango_find_base_dir` cannot be `Direction::WeakLtr`
or `Direction::WeakRtl`, since every character is either
neutral or has a strong direction; on the other hand
`Direction::Neutral` doesn't make sense to pass
to `pango_itemize_with_base_dir`.

The `Direction::TtbLtr`, `Direction::TtbRtl`
values come from an earlier interpretation of this
enumeration as the writing direction of a block of
text and are no longer used; See `Gravity` for how
vertical text is handled in Pango.
<!-- enum Direction::variant Ltr -->
A strong left-to-right direction
<!-- enum Direction::variant Rtl -->
A strong right-to-left direction
<!-- enum Direction::variant TtbLtr -->
Deprecated value; treated the
 same as `Direction::Rtl`.
<!-- enum Direction::variant TtbRtl -->
Deprecated value; treated the
 same as `Direction::Ltr`
<!-- enum Direction::variant WeakLtr -->
A weak left-to-right direction
<!-- enum Direction::variant WeakRtl -->
A weak right-to-left direction
<!-- enum Direction::variant Neutral -->
No direction specified
<!-- enum EllipsizeMode -->
The `EllipsizeMode` type describes what sort of (if any)
ellipsization should be applied to a line of text. In
the ellipsization process characters are removed from the
text in order to make it fit to a given width and replaced
with an ellipsis.
<!-- enum EllipsizeMode::variant None -->
No ellipsization
<!-- enum EllipsizeMode::variant Start -->
Omit characters at the start of the text
<!-- enum EllipsizeMode::variant Middle -->
Omit characters in the middle of the text
<!-- enum EllipsizeMode::variant End -->
Omit characters at the end of the text
<!-- struct EngineLang -->
`[Deprecated since 1.38]` The `EngineLang` class is implemented by engines that
customize the rendering-system independent part of the
Pango pipeline for a particular script or language. For
instance, a custom `EngineLang` could be provided for
Thai to implement the dictionary-based word boundary
lookups needed for that language.
<!-- struct EngineShape -->
`[Deprecated since 1.38]` The `EngineShape` class is implemented by engines that
customize the rendering-system dependent part of the
Pango pipeline for a particular script or language.
A `EngineShape` implementation is then specific to both
a particular rendering system or group of rendering systems
and to a particular script. For instance, there is one
`EngineShape` implementation to handle shaping Arabic
for Fontconfig-based backends.
<!-- struct Font -->
The `Font` structure is used to represent
a font in a rendering-system-independent matter.
To create an implementation of a `Font`,
the rendering-system specific code should allocate
a larger structure that contains a nested
`Font`, fill in the `<structfield>`klass`</structfield>` member of
the nested `Font` with a pointer to
a appropriate `FontClass`, then call
`pango_font_init` on the structure.

The `Font` structure contains one member
which the implementation fills in.

# Implements

[`FontExt`](trait.FontExt.html)
<!-- trait FontExt -->
Trait containing all `Font` methods.

# Implementors

[`Font`](struct.Font.html)
<!-- impl Font::fn descriptions_free -->
Frees an array of font descriptions.
## `descs`
a pointer
to an array of `FontDescription`, may be `None`
## `n_descs`
number of font descriptions in `descs`
<!-- trait FontExt::fn describe -->
Returns a description of the font, with font size set in points.
Use `FontExt::describe_with_absolute_size` if you want the font
size in device units.

# Returns

a newly-allocated `FontDescription` object.
<!-- trait FontExt::fn describe_with_absolute_size -->
Returns a description of the font, with absolute font size set
(in device units). Use `FontExt::describe` if you want the font
size in points.

# Returns

a newly-allocated `FontDescription` object.
<!-- trait FontExt::fn find_shaper -->
Finds the best matching shaper for a font for a particular
language tag and character point.
## `language`
the language tag
## `ch`
a Unicode character.

# Returns

the best matching shaper.
<!-- trait FontExt::fn get_coverage -->
Computes the coverage map for a given font and language tag.
## `language`
the language tag

# Returns

a newly-allocated `Coverage`
 object.
<!-- trait FontExt::fn get_font_map -->
Gets the font map for which the font was created.

Note that the font maintains a `<firstterm>`weak`</firstterm>` reference
to the font map, so if all references to font map are dropped, the font
map will be finalized even if there are fonts created with the font
map that are still alive. In that case this function will return `None`.
It is the responsibility of the user to ensure that the font map is kept
alive. In most uses this is not an issue as a `Context` holds
a reference to the font map.

# Returns

the `FontMap` for the
 font, or `None` if `self` is `None`.
<!-- trait FontExt::fn get_glyph_extents -->
Gets the logical and ink extents of a glyph within a font. The
coordinate system for each rectangle has its origin at the
base line and horizontal origin of the character with increasing
coordinates extending to the right and down. The macros PANGO_ASCENT(),
PANGO_DESCENT(), PANGO_LBEARING(), and PANGO_RBEARING() can be used to convert
from the extents rectangle to more traditional font metrics. The units
of the rectangles are in 1/PANGO_SCALE of a device unit.

If `self` is `None`, this function gracefully sets some sane values in the
output variables and returns.
## `glyph`
the glyph index
## `ink_rect`
rectangle used to store the extents of the glyph
 as drawn or `None` to indicate that the result is not needed.
## `logical_rect`
rectangle used to store the logical extents of
 the glyph or `None` to indicate that the result is not needed.
<!-- trait FontExt::fn get_metrics -->
Gets overall metric information for a font. Since the metrics may be
substantially different for different scripts, a language tag can
be provided to indicate that the metrics should be retrieved that
correspond to the script(s) used by that language.

If `self` is `None`, this function gracefully sets some sane values in the
output variables and returns.
## `language`
language tag used to determine which script to get the metrics
 for, or `None` to indicate to get the metrics for the entire font.

# Returns

a `FontMetrics` object. The caller must call `FontMetrics::unref`
 when finished using the object.
<!-- struct FontDescription -->
The `FontDescription` structure represents the description
of an ideal font. These structures are used both to list
what fonts are available on the system and also for specifying
the characteristics of a font to load.
<!-- impl FontDescription::fn new -->
Creates a new font description structure with all fields unset.

# Returns

the newly allocated `FontDescription`, which
 should be freed using `FontDescription::free`.
<!-- impl FontDescription::fn better_match -->
Determines if the style attributes of `new_match` are a closer match
for `self` than those of `old_match` are, or if `old_match` is `None`,
determines if `new_match` is a match at all.
Approximate matching is done for
weight and style; other style attributes must match exactly.
Style attributes are all attributes other than family and size-related
attributes. Approximate matching for style considers PANGO_STYLE_OBLIQUE
and PANGO_STYLE_ITALIC as matches, but not as good a match as when the
styles are equal.

Note that `old_match` must match `self`.
## `old_match`
a `FontDescription`, or `None`
## `new_match`
a `FontDescription`

# Returns

`true` if `new_match` is a better match
<!-- impl FontDescription::fn copy -->
Make a copy of a `FontDescription`.

# Returns

the newly allocated
 `FontDescription`, which should be freed with
 `FontDescription::free`, or `None` if `self` was
 `None`.
<!-- impl FontDescription::fn copy_static -->
Like `FontDescription::copy`, but only a shallow copy is made
of the family name and other allocated fields. The result can only
be used until `self` is modified or freed. This is meant to be used
when the copy is only needed temporarily.

# Returns

the newly allocated
 `FontDescription`, which should be freed with
 `FontDescription::free`, or `None` if `self` was
 `None`.
<!-- impl FontDescription::fn equal -->
Compares two font descriptions for equality. Two font descriptions
are considered equal if the fonts they describe are provably identical.
This means that their masks do not have to match, as long as other fields
are all the same. (Two font descriptions may result in identical fonts
being loaded, but still compare `false`.)
## `desc2`
another `FontDescription`

# Returns

`true` if the two font descriptions are identical,
     `false` otherwise.
<!-- impl FontDescription::fn free -->
Frees a font description.
<!-- impl FontDescription::fn get_family -->
Gets the family name field of a font description. See
`FontDescription::set_family`.

# Returns

the family name field for the font
 description, or `None` if not previously set. This
 has the same life-time as the font description itself
 and should not be freed.
<!-- impl FontDescription::fn get_gravity -->
Gets the gravity field of a font description. See
`FontDescription::set_gravity`.

# Returns

the gravity field for the font description. Use
 `FontDescription::get_set_fields` to find out if
 the field was explicitly set or not.
<!-- impl FontDescription::fn get_set_fields -->
Determines which fields in a font description have been set.

# Returns

a bitmask with bits set corresponding to the
 fields in `self` that have been set.
<!-- impl FontDescription::fn get_size -->
Gets the size field of a font description.
See `FontDescription::set_size`.

# Returns

the size field for the font description in points or device units.
 You must call `FontDescription::get_size_is_absolute`
 to find out which is the case. Returns 0 if the size field has not
 previously been set or it has been set to 0 explicitly.
 Use `FontDescription::get_set_fields` to
 find out if the field was explicitly set or not.
<!-- impl FontDescription::fn get_size_is_absolute -->
Determines whether the size of the font is in points (not absolute) or device units (absolute).
See `FontDescription::set_size` and `FontDescription::set_absolute_size`.

# Returns

whether the size for the font description is in
 points or device units. Use `FontDescription::get_set_fields` to
 find out if the size field of the font description was explicitly set or not.
<!-- impl FontDescription::fn get_stretch -->
Gets the stretch field of a font description.
See `FontDescription::set_stretch`.

# Returns

the stretch field for the font description. Use
 `FontDescription::get_set_fields` to find out if
 the field was explicitly set or not.
<!-- impl FontDescription::fn get_style -->
Gets the style field of a `FontDescription`. See
`FontDescription::set_style`.

# Returns

the style field for the font description.
 Use `FontDescription::get_set_fields` to find out if
 the field was explicitly set or not.
<!-- impl FontDescription::fn get_variant -->
Gets the variant field of a `FontDescription`. See
`FontDescription::set_variant`.

# Returns

the variant field for the font description. Use
 `FontDescription::get_set_fields` to find out if
 the field was explicitly set or not.
<!-- impl FontDescription::fn get_weight -->
Gets the weight field of a font description. See
`FontDescription::set_weight`.

# Returns

the weight field for the font description. Use
 `FontDescription::get_set_fields` to find out if
 the field was explicitly set or not.
<!-- impl FontDescription::fn hash -->
Computes a hash of a `FontDescription` structure suitable
to be used, for example, as an argument to `glib::HashTable::new`.
The hash value is independent of `self`->mask.

# Returns

the hash value.
<!-- impl FontDescription::fn merge -->
Merges the fields that are set in `desc_to_merge` into the fields in
`self`. If `replace_existing` is `false`, only fields in `self` that
are not already set are affected. If `true`, then fields that are
already set will be replaced as well.

If `desc_to_merge` is `None`, this function performs nothing.
## `desc_to_merge`
the `FontDescription` to merge from, or `None`
## `replace_existing`
if `true`, replace fields in `self` with the
 corresponding values from `desc_to_merge`, even if they
 are already exist.
<!-- impl FontDescription::fn merge_static -->
Like `FontDescription::merge`, but only a shallow copy is made
of the family name and other allocated fields. `self` can only be
used until `desc_to_merge` is modified or freed. This is meant
to be used when the merged font description is only needed temporarily.
## `desc_to_merge`
the `FontDescription` to merge from
## `replace_existing`
if `true`, replace fields in `self` with the
 corresponding values from `desc_to_merge`, even if they
 are already exist.
<!-- impl FontDescription::fn set_absolute_size -->
Sets the size field of a font description, in device units. This is mutually
exclusive with `FontDescription::set_size` which sets the font size
in points.
## `size`
the new size, in Pango units. There are `PANGO_SCALE` Pango units in one
 device unit. For an output backend where a device unit is a pixel, a `size`
 value of 10 * PANGO_SCALE gives a 10 pixel font.
<!-- impl FontDescription::fn set_family -->
Sets the family name field of a font description. The family
name represents a family of related font styles, and will
resolve to a particular `FontFamily`. In some uses of
`FontDescription`, it is also possible to use a comma
separated list of family names for this field.
## `family`
a string representing the family name.
<!-- impl FontDescription::fn set_family_static -->
Like `FontDescription::set_family`, except that no
copy of `family` is made. The caller must make sure that the
string passed in stays around until `self` has been freed
or the name is set again. This function can be used if
`family` is a static string such as a C string literal, or
if `self` is only needed temporarily.
## `family`
a string representing the family name.
<!-- impl FontDescription::fn set_gravity -->
Sets the gravity field of a font description. The gravity field
specifies how the glyphs should be rotated. If `gravity` is
`Gravity::Auto`, this actually unsets the gravity mask on
the font description.

This function is seldom useful to the user. Gravity should normally
be set on a `Context`.
## `gravity`
the gravity for the font description.
<!-- impl FontDescription::fn set_size -->
Sets the size field of a font description in fractional points. This is mutually
exclusive with `FontDescription::set_absolute_size`.
## `size`
the size of the font in points, scaled by PANGO_SCALE. (That is,
 a `size` value of 10 * PANGO_SCALE is a 10 point font. The conversion
 factor between points and device units depends on system configuration
 and the output device. For screen display, a logical DPI of 96 is
 common, in which case a 10 point font corresponds to a 10 * (96 / 72) = 13.3
 pixel font. Use `FontDescription::set_absolute_size` if you need
 a particular size in device units.
<!-- impl FontDescription::fn set_stretch -->
Sets the stretch field of a font description. The stretch field
specifies how narrow or wide the font should be.
## `stretch`
the stretch for the font description
<!-- impl FontDescription::fn set_style -->
Sets the style field of a `FontDescription`. The
`Style` enumeration describes whether the font is slanted and
the manner in which it is slanted; it can be either
`Style::Normal`, `Style::Italic`, or `Style::Oblique`.
Most fonts will either have a italic style or an oblique
style, but not both, and font matching in Pango will
match italic specifications with oblique fonts and vice-versa
if an exact match is not found.
## `style`
the style for the font description
<!-- impl FontDescription::fn set_variant -->
Sets the variant field of a font description. The `Variant`
can either be `Variant::Normal` or `Variant::SmallCaps`.
## `variant`
the variant type for the font description.
<!-- impl FontDescription::fn set_weight -->
Sets the weight field of a font description. The weight field
specifies how bold or light the font should be. In addition
to the values of the `Weight` enumeration, other intermediate
numeric values are possible.
## `weight`
the weight for the font description.
<!-- impl FontDescription::fn to_filename -->
Creates a filename representation of a font description. The
filename is identical to the result from calling
`FontDescription::to_string`, but with underscores instead of
characters that are untypical in filenames, and in lower case only.

# Returns

a new string that must be freed with `g_free`.
<!-- impl FontDescription::fn to_string -->
Creates a string representation of a font description. See
`FontDescription::from_string` for a description of the
format of the string representation. The family list in the
string description will only have a terminating comma if the
last word of the list is a valid style option.

# Returns

a new string that must be freed with `g_free`.
<!-- impl FontDescription::fn unset_fields -->
Unsets some of the fields in a `FontDescription`. The unset
fields will get back to their default values.
## `to_unset`
bitmask of fields in the `self` to unset.
<!-- impl FontDescription::fn from_string -->
Creates a new font description from a string representation in the
form "[FAMILY-LIST] [STYLE-OPTIONS] [SIZE]", where FAMILY-LIST is a
comma separated list of families optionally terminated by a comma,
STYLE_OPTIONS is a whitespace separated list of words where each word
describes one of style, variant, weight, stretch, or gravity, and SIZE
is a decimal number (size in points) or optionally followed by the
unit modifier "px" for absolute size. Any one of the options may
be absent. If FAMILY-LIST is absent, then the family_name field of
the resulting font description will be initialized to `None`. If
STYLE-OPTIONS is missing, then all style options will be set to the
default values. If SIZE is missing, the size in the resulting font
description will be set to 0.
## `str`
string representation of a font description.

# Returns

a new `FontDescription`.
<!-- struct FontFace -->
The `FontFace` structure is used to represent a group of fonts with
the same family, slant, weight, width, but varying sizes.

# Implements

[`FontFaceExt`](trait.FontFaceExt.html)
<!-- trait FontFaceExt -->
Trait containing all `FontFace` methods.

# Implementors

[`FontFace`](struct.FontFace.html)
<!-- trait FontFaceExt::fn describe -->
Returns the family, style, variant, weight and stretch of
a `FontFace`. The size field of the resulting font description
will be unset.

# Returns

a newly-created `FontDescription` structure
 holding the description of the face. Use `FontDescription::free`
 to free the result.
<!-- trait FontFaceExt::fn get_face_name -->
Gets a name representing the style of this face among the
different faces in the `FontFamily` for the face. This
name is unique among all faces in the family and is suitable
for displaying to users.

# Returns

the face name for the face. This string is
 owned by the face object and must not be modified or freed.
<!-- trait FontFaceExt::fn is_synthesized -->
Returns whether a `FontFace` is synthesized by the underlying
font rendering engine from another face, perhaps by shearing, emboldening,
or lightening it.

# Returns

whether `self` is synthesized.
<!-- trait FontFaceExt::fn list_sizes -->
List the available sizes for a font. This is only applicable to bitmap
fonts. For scalable fonts, stores `None` at the location pointed to by
`sizes` and 0 at the location pointed to by `n_sizes`. The sizes returned
are in Pango units and are sorted in ascending order.
## `sizes`

 location to store a pointer to an array of int. This array
 should be freed with `g_free`.
## `n_sizes`
location to store the number of elements in `sizes`
<!-- struct FontFamily -->
The `FontFamily` structure is used to represent a family of related
font faces. The faces in a family share a common design, but differ in
slant, weight, width and other aspects.

# Implements

[`FontFamilyExt`](trait.FontFamilyExt.html)
<!-- trait FontFamilyExt -->
Trait containing all `FontFamily` methods.

# Implementors

[`FontFamily`](struct.FontFamily.html)
<!-- trait FontFamilyExt::fn get_name -->
Gets the name of the family. The name is unique among all
fonts for the font backend and can be used in a `FontDescription`
to specify that a face from this family is desired.

# Returns

the name of the family. This string is owned
 by the family object and must not be modified or freed.
<!-- trait FontFamilyExt::fn is_monospace -->
A monospace font is a font designed for text display where the the
characters form a regular grid. For Western languages this would
mean that the advance width of all characters are the same, but
this categorization also includes Asian fonts which include
double-width characters: characters that occupy two grid cells.
`g_unichar_iswide` returns a result that indicates whether a
character is typically double-width in a monospace font.

The best way to find out the grid-cell size is to call
`FontMetrics::get_approximate_digit_width`, since the results
of `FontMetrics::get_approximate_char_width` may be affected
by double-width characters.

# Returns

`true` if the family is monospace.
<!-- trait FontFamilyExt::fn list_faces -->
Lists the different font faces that make up `self`. The faces
in a family share a common design, but differ in slant, weight,
width and other aspects.
## `faces`

 location to store an array of pointers to `FontFace` objects,
 or `None`. This array should be freed with `g_free` when it is no
 longer needed.
## `n_faces`
location to store number of elements in `faces`.
<!-- struct FontMap -->
The `FontMap` represents the set of fonts available for a
particular rendering system. This is a virtual object with
implementations being specific to particular rendering systems. To
create an implementation of a `FontMap`, the rendering-system
specific code should allocate a larger structure that contains a nested
`FontMap`, fill in the `<structfield>`klass`</structfield>` member of the nested `FontMap` with a
pointer to a appropriate `FontMapClass`, then call
`pango_font_map_init` on the structure.

The `FontMap` structure contains one member which the implementation
fills in.

# Implements

[`FontMapExt`](trait.FontMapExt.html)
<!-- trait FontMapExt -->
Trait containing all `FontMap` methods.

# Implementors

[`FontMap`](struct.FontMap.html)
<!-- trait FontMapExt::fn changed -->
Forces a change in the context, which will cause any `Context`
using this fontmap to change.

This function is only useful when implementing a new backend
for Pango, something applications won't do. Backends should
call this function if they have attached extra data to the context
and such data is changed.

Feature: `v1_34`

<!-- trait FontMapExt::fn create_context -->
Creates a `Context` connected to `self`. This is equivalent
to `Context::new` followed by `ContextExt::set_font_map`.

If you are using Pango as part of a higher-level system,
that system may have it's own way of create a `Context`.
For instance, the GTK+ toolkit has, among others,
`gdk_pango_context_get_for_screen`, and
`gtk_widget_get_pango_context`. Use those instead.

# Returns

the newly allocated `Context`,
 which should be freed with `gobject::Object::unref`.
<!-- trait FontMapExt::fn get_serial -->
Returns the current serial number of `self`. The serial number is
initialized to an small number larger than zero when a new fontmap
is created and is increased whenever the fontmap is changed. It may
wrap, but will never have the value 0. Since it can wrap, never compare
it with "less than", always use "not equals".

The fontmap can only be changed using backend-specific API, like changing
fontmap resolution.

This can be used to automatically detect changes to a `FontMap`, like
in `Context`.

Feature: `v1_32_4`


# Returns

The current serial number of `self`.
<!-- trait FontMapExt::fn get_shape_engine_type -->
Returns the render ID for shape engines for this fontmap.
See the `<structfield>`render_type`</structfield>` field of
`EngineInfo`.

# Deprecated since 1.38


# Returns

the ID string for shape engines for
 this fontmap. Owned by Pango, should not be modified
 or freed.
<!-- trait FontMapExt::fn list_families -->
List all families for a fontmap.
## `families`
location to store a pointer to an array of `FontFamily` *.
 This array should be freed with `g_free`.
## `n_families`
location to store the number of elements in `families`
<!-- trait FontMapExt::fn load_font -->
Load the font in the fontmap that is the closest match for `desc`.
## `context`
the `Context` the font will be used with
## `desc`
a `FontDescription` describing the font to load

# Returns

the newly allocated `Font`
 loaded, or `None` if no font matched.
<!-- trait FontMapExt::fn load_fontset -->
Load a set of fonts in the fontmap that can be used to render
a font matching `desc`.
## `context`
the `Context` the font will be used with
## `desc`
a `FontDescription` describing the font to load
## `language`
a `Language` the fonts will be used for

# Returns

the newly allocated
 `Fontset` loaded, or `None` if no font matched.
<!-- struct FontMetrics -->
A `FontMetrics` structure holds the overall metric information
for a font (possibly restricted to a script). The fields of this
structure are private to implementations of a font backend. See
the documentation of the corresponding getters for documentation
of their meaning.
<!-- impl FontMetrics::fn new -->
Creates a new `FontMetrics` structure. This is only for
internal use by Pango backends and there is no public way
to set the fields of the structure.

# Returns

a newly-created `FontMetrics` structure
 with a reference count of 1.
<!-- impl FontMetrics::fn get_approximate_char_width -->
Gets the approximate character width for a font metrics structure.
This is merely a representative value useful, for example, for
determining the initial size for a window. Actual characters in
text will be wider and narrower than this.

# Returns

the character width, in Pango units.
<!-- impl FontMetrics::fn get_approximate_digit_width -->
Gets the approximate digit width for a font metrics structure.
This is merely a representative value useful, for example, for
determining the initial size for a window. Actual digits in
text can be wider or narrower than this, though this value
is generally somewhat more accurate than the result of
`FontMetrics::get_approximate_char_width` for digits.

# Returns

the digit width, in Pango units.
<!-- impl FontMetrics::fn get_ascent -->
Gets the ascent from a font metrics structure. The ascent is
the distance from the baseline to the logical top of a line
of text. (The logical top may be above or below the top of the
actual drawn ink. It is necessary to lay out the text to figure
where the ink will be.)

# Returns

the ascent, in Pango units.
<!-- impl FontMetrics::fn get_descent -->
Gets the descent from a font metrics structure. The descent is
the distance from the baseline to the logical bottom of a line
of text. (The logical bottom may be above or below the bottom of the
actual drawn ink. It is necessary to lay out the text to figure
where the ink will be.)

# Returns

the descent, in Pango units.
<!-- impl FontMetrics::fn get_strikethrough_position -->
Gets the suggested position to draw the strikethrough.
The value returned is the distance `<emphasis>`above`</emphasis>` the
baseline of the top of the strikethrough.

# Returns

the suggested strikethrough position, in Pango units.
<!-- impl FontMetrics::fn get_strikethrough_thickness -->
Gets the suggested thickness to draw for the strikethrough.

# Returns

the suggested strikethrough thickness, in Pango units.
<!-- impl FontMetrics::fn get_underline_position -->
Gets the suggested position to draw the underline.
The value returned is the distance `<emphasis>`above`</emphasis>` the
baseline of the top of the underline. Since most fonts have
underline positions beneath the baseline, this value is typically
negative.

# Returns

the suggested underline position, in Pango units.
<!-- impl FontMetrics::fn get_underline_thickness -->
Gets the suggested thickness to draw for the underline.

# Returns

the suggested underline thickness, in Pango units.
<!-- impl FontMetrics::fn ref -->
Increase the reference count of a font metrics structure by one.

# Returns

`self`
<!-- impl FontMetrics::fn unref -->
Decrease the reference count of a font metrics structure by one. If
the result is zero, frees the structure and any associated
memory.
<!-- struct Fontset -->
A `Fontset` represents a set of `Font` to use
when rendering text. It is the result of resolving a
`FontDescription` against a particular `Context`.
It has operations for finding the component font for
a particular Unicode character, and for finding a composite
set of metrics for the entire fontset.

# Implements

[`FontsetExt`](trait.FontsetExt.html)
<!-- trait FontsetExt -->
Trait containing all `Fontset` methods.

# Implementors

[`FontsetSimple`](struct.FontsetSimple.html), [`Fontset`](struct.Fontset.html)
<!-- trait FontsetExt::fn foreach -->
Iterates through all the fonts in a fontset, calling `func` for
each one. If `func` returns `true`, that stops the iteration.
## `func`
Callback function
## `data`
data to pass to the callback function
<!-- trait FontsetExt::fn get_font -->
Returns the font in the fontset that contains the best glyph for the
Unicode character `wc`.
## `wc`
a Unicode character

# Returns

a `Font`. The caller must call
 g_object_unref when finished with the font.
<!-- trait FontsetExt::fn get_metrics -->
Get overall metric information for the fonts in the fontset.

# Returns

a `FontMetrics` object. The caller must call `FontMetrics::unref`
 when finished using the object.
<!-- struct FontsetSimple -->
`FontsetSimple` is a implementation of the abstract
`Fontset` base class in terms of an array of fonts,
which the creator provides when constructing the
`FontsetSimple`.

# Implements

[`FontsetSimpleExt`](trait.FontsetSimpleExt.html), [`FontsetExt`](trait.FontsetExt.html)
<!-- trait FontsetSimpleExt -->
Trait containing all `FontsetSimple` methods.

# Implementors

[`FontsetSimple`](struct.FontsetSimple.html)
<!-- impl FontsetSimple::fn new -->
Creates a new `FontsetSimple` for the given language.
## `language`
a `Language` tag

# Returns

the newly allocated `FontsetSimple`, which should
 be freed with `gobject::Object::unref`.
<!-- trait FontsetSimpleExt::fn append -->
Adds a font to the fontset.
## `font`
a `Font`.
<!-- trait FontsetSimpleExt::fn size -->
Returns the number of fonts in the fontset.

# Returns

the size of `self`.
<!-- struct GlyphItem -->
A `GlyphItem` is a pair of a `Item` and the glyphs
resulting from shaping the text corresponding to an item.
As an example of the usage of `GlyphItem`, the results
of shaping text with `Layout` is a list of `LayoutLine`,
each of which contains a list of `GlyphItem`.
<!-- impl GlyphItem::fn apply_attrs -->
Splits a shaped item (PangoGlyphItem) into multiple items based
on an attribute list. The idea is that if you have attributes
that don't affect shaping, such as color or underline, to avoid
affecting shaping, you filter them out (`AttrList::filter`),
apply the shaping process and then reapply them to the result using
this function.

All attributes that start or end inside a cluster are applied
to that cluster; for instance, if half of a cluster is underlined
and the other-half strikethrough, then the cluster will end
up with both underline and strikethrough attributes. In these
cases, it may happen that item->extra_attrs for some of the
result items can have multiple attributes of the same type.

This function takes ownership of `self`; it will be reused
as one of the elements in the list.
## `text`
text that `list` applies to
## `list`
a `AttrList`

# Returns

a
 list of glyph items resulting from splitting `self`. Free
 the elements using `GlyphItem::free`, the list using
 `glib::SList::free`.
<!-- impl GlyphItem::fn copy -->
Make a deep copy of an existing `GlyphItem` structure.

# Returns

the newly allocated `GlyphItem`, which should
 be freed with `GlyphItem::free`, or `None`
 if `self` was `None`.
<!-- impl GlyphItem::fn free -->
Frees a `GlyphItem` and resources to which it points.
<!-- impl GlyphItem::fn get_logical_widths -->
Given a `GlyphItem` and the corresponding
text, determine the screen width corresponding to each character. When
multiple characters compose a single cluster, the width of the entire
cluster is divided equally among the characters.

See also `GlyphString::get_logical_widths`.
## `text`
text that `self` corresponds to
 (glyph_item->item->offset is an offset from the
 start of `text`)
## `logical_widths`
an array whose length is the number of
 characters in glyph_item (equal to
 glyph_item->item->num_chars) to be filled in with
 the resulting character widths.
<!-- impl GlyphItem::fn letter_space -->
Adds spacing between the graphemes of `self` to
give the effect of typographic letter spacing.
## `text`
text that `self` corresponds to
 (glyph_item->item->offset is an offset from the
 start of `text`)
## `log_attrs`
logical attributes for the item
 (the first logical attribute refers to the position
 before the first character in the item)
## `letter_spacing`
amount of letter spacing to add
 in Pango units. May be negative, though too large
 negative values will give ugly results.
<!-- impl GlyphItem::fn split -->
Modifies `self` to cover only the text after `split_index`, and
returns a new item that covers the text before `split_index` that
used to be in `self`. You can think of `split_index` as the length of
the returned item. `split_index` may not be 0, and it may not be
greater than or equal to the length of `self` (that is, there must
be at least one byte assigned to each item, you can't create a
zero-length item).

This function is similar in function to `Item::split` (and uses
it internally.)
## `text`
text to which positions in `self` apply
## `split_index`
byte index of position to split item, relative to the start of the item

# Returns

the newly allocated item representing text before
 `split_index`, which should be freed
 with `GlyphItem::free`.
<!-- struct GlyphItemIter -->
A `GlyphItemIter` is an iterator over the clusters in a
`GlyphItem`. The `<firstterm>`forward direction`</firstterm>` of the
iterator is the logical direction of text. That is, with increasing
`start_index` and `start_char` values. If `glyph_item` is right-to-left
(that is, if `<literal>``glyph_item`->item->analysis.level`</literal>` is odd),
then `start_glyph` decreases as the iterator moves forward. Moreover,
in right-to-left cases, `start_glyph` is greater than `end_glyph`.

An iterator should be initialized using either of
`GlyphItemIter::init_start` and
`GlyphItemIter::init_end`, for forward and backward iteration
respectively, and walked over using any desired mixture of
`GlyphItemIter::next_cluster` and
`GlyphItemIter::prev_cluster`. A common idiom for doing a
forward iteration over the clusters is:
`<programlisting>`
PangoGlyphItemIter cluster_iter;
gboolean have_cluster;

for (have_cluster = pango_glyph_item_iter_init_start (&amp;cluster_iter,
 glyph_item, text);
 have_cluster;
 have_cluster = pango_glyph_item_iter_next_cluster (&amp;cluster_iter))
{
 ...
}
`</programlisting>`

Note that `text` is the start of the text for layout, which is then
indexed by `<literal>``glyph_item`->item->offset`</literal>` to get to the
text of `glyph_item`. The `start_index` and `end_index` values can directly
index into `text`. The `start_glyph`, `end_glyph`, `start_char`, and `end_char`
values however are zero-based for the `glyph_item`. For each cluster, the
item pointed at by the start variables is included in the cluster while
the one pointed at by end variables is not.

None of the members of a `GlyphItemIter` should be modified manually.
<!-- impl GlyphItemIter::fn copy -->
Make a shallow copy of an existing `GlyphItemIter` structure.

# Returns

the newly allocated `GlyphItemIter`, which should
 be freed with `GlyphItemIter::free`, or `None`
 if `self` was `None`.
<!-- impl GlyphItemIter::fn free -->
Frees a `GlyphItemIter` created by `GlyphItemIter::copy`.
<!-- impl GlyphItemIter::fn init_end -->
Initializes a `GlyphItemIter` structure to point to the
last cluster in a glyph item.
See `GlyphItemIter` for details of cluster orders.
## `glyph_item`
the glyph item to iterate over
## `text`
text corresponding to the glyph item

# Returns

`false` if there are no clusters in the glyph item
<!-- impl GlyphItemIter::fn init_start -->
Initializes a `GlyphItemIter` structure to point to the
first cluster in a glyph item.
See `GlyphItemIter` for details of cluster orders.
## `glyph_item`
the glyph item to iterate over
## `text`
text corresponding to the glyph item

# Returns

`false` if there are no clusters in the glyph item
<!-- impl GlyphItemIter::fn next_cluster -->
Advances the iterator to the next cluster in the glyph item.
See `GlyphItemIter` for details of cluster orders.

# Returns

`true` if the iterator was advanced, `false` if we were already on the
 last cluster.
<!-- impl GlyphItemIter::fn prev_cluster -->
Moves the iterator to the preceding cluster in the glyph item.
See `GlyphItemIter` for details of cluster orders.

# Returns

`true` if the iterator was moved, `false` if we were already on the
 first cluster.
<!-- struct GlyphString -->
The `GlyphString` structure is used to store strings
of glyphs with geometry and visual attribute information.
The storage for the glyph information is owned
by the structure which simplifies memory management.
<!-- impl GlyphString::fn new -->
Create a new `GlyphString`.

# Returns

the newly allocated `GlyphString`, which
 should be freed with `GlyphString::free`.
<!-- impl GlyphString::fn copy -->
Copy a glyph string and associated storage.

# Returns

the newly allocated `GlyphString`,
 which should be freed with `GlyphString::free`,
 or `None` if `self` was `None`.
<!-- impl GlyphString::fn extents -->
Compute the logical and ink extents of a glyph string. See the documentation
for `FontExt::get_glyph_extents` for details about the interpretation
of the rectangles.
## `font`
a `Font`
## `ink_rect`
rectangle used to store the extents of the glyph string
 as drawn or `None` to indicate that the result is not needed.
## `logical_rect`
rectangle used to store the logical extents of the
 glyph string or `None` to indicate that the result is not needed.
<!-- impl GlyphString::fn extents_range -->
Computes the extents of a sub-portion of a glyph string. The extents are
relative to the start of the glyph string range (the origin of their
coordinate system is at the start of the range, not at the start of the entire
glyph string).
## `start`
start index
## `end`
end index (the range is the set of bytes with
     indices such that start <= index < end)
## `font`
a `Font`
## `ink_rect`
rectangle used to
 store the extents of the glyph string range as drawn or
 `None` to indicate that the result is not needed.
## `logical_rect`
rectangle used to
 store the logical extents of the glyph string range or
 `None` to indicate that the result is not needed.
<!-- impl GlyphString::fn free -->
Free a glyph string and associated storage.
<!-- impl GlyphString::fn get_logical_widths -->
Given a `GlyphString` resulting from `pango_shape` and the corresponding
text, determine the screen width corresponding to each character. When
multiple characters compose a single cluster, the width of the entire
cluster is divided equally among the characters.

See also `GlyphItem::get_logical_widths`.
## `text`
the text corresponding to the glyphs
## `length`
the length of `text`, in bytes
## `embedding_level`
the embedding level of the string
## `logical_widths`
an array whose length is the number of
 characters in text (equal to g_utf8_strlen (text,
 length) unless text has NUL bytes) to be filled in
 with the resulting character widths.
<!-- impl GlyphString::fn get_width -->
Computes the logical width of the glyph string as can also be computed
using `GlyphString::extents`. However, since this only computes the
width, it's much faster. This is in fact only a convenience function that
computes the sum of geometry.width for each glyph in the `self`.

# Returns

the logical width of the glyph string.
<!-- impl GlyphString::fn index_to_x -->
Converts from character position to x position. (X position
is measured from the left edge of the run). Character positions
are computed by dividing up each cluster into equal portions.
## `text`
the text for the run
## `length`
the number of bytes (not characters) in `text`.
## `analysis`
the analysis information return from `pango_itemize`
## `index_`
the byte index within `text`
## `trailing`
whether we should compute the result for the beginning (`false`)
 or end (`true`) of the character.
## `x_pos`
location to store result
<!-- impl GlyphString::fn set_size -->
Resize a glyph string to the given length.
## `new_len`
the new length of the string.
<!-- impl GlyphString::fn x_to_index -->
Convert from x offset to character position. Character positions
are computed by dividing up each cluster into equal portions.
In scripts where positioning within a cluster is not allowed
(such as Thai), the returned value may not be a valid cursor
position; the caller must combine the result with the logical
attributes for the text to compute the valid cursor position.
## `text`
the text for the run
## `length`
the number of bytes (not characters) in text.
## `analysis`
the analysis information return from `pango_itemize`
## `x_pos`
the x offset (in Pango units)
## `index_`
location to store calculated byte index within `text`
## `trailing`
location to store a boolean indicating
 whether the user clicked on the leading or trailing
 edge of the character.
<!-- enum Gravity -->
The `Gravity` type represents the orientation of glyphs in a segment
of text. This is useful when rendering vertical text layouts. In
those situations, the layout is rotated using a non-identity PangoMatrix,
and then glyph orientation is controlled using `Gravity`.
Not every value in this enumeration makes sense for every usage of
`Gravity`; for example, `Gravity::Auto` only can be passed to
`ContextExt::set_base_gravity` and can only be returned by
`ContextExt::get_base_gravity`.

See also: `GravityHint`
<!-- enum Gravity::variant South -->
Glyphs stand upright (default)
<!-- enum Gravity::variant East -->
Glyphs are rotated 90 degrees clockwise
<!-- enum Gravity::variant North -->
Glyphs are upside-down
<!-- enum Gravity::variant West -->
Glyphs are rotated 90 degrees counter-clockwise
<!-- enum Gravity::variant Auto -->
Gravity is resolved from the context matrix
<!-- enum GravityHint -->
The `GravityHint` defines how horizontal scripts should behave in a
vertical context. That is, English excerpt in a vertical paragraph for
example.

See `Gravity`.
<!-- enum GravityHint::variant Natural -->
scripts will take their natural gravity based
on the base gravity and the script. This is the default.
<!-- enum GravityHint::variant Strong -->
always use the base gravity set, regardless of
the script.
<!-- enum GravityHint::variant Line -->
for scripts not in their natural direction (eg.
Latin in East gravity), choose per-script gravity such that every script
respects the line progression. This means, Latin and Arabic will take
opposite gravities and both flow top-to-bottom for example.
<!-- struct Item -->
The `Item` structure stores information about a segment of text.
<!-- impl Item::fn new -->
Creates a new `Item` structure initialized to default values.

# Returns

the newly allocated `Item`, which should
 be freed with `Item::free`.
<!-- impl Item::fn copy -->
Copy an existing `Item` structure.

# Returns

the newly allocated `Item`, which
 should be freed with `Item::free`, or `None` if
 `self` was `None`.
<!-- impl Item::fn free -->
Free a `Item` and all associated memory.
<!-- impl Item::fn split -->
Modifies `self` to cover only the text after `split_index`, and
returns a new item that covers the text before `split_index` that
used to be in `self`. You can think of `split_index` as the length of
the returned item. `split_index` may not be 0, and it may not be
greater than or equal to the length of `self` (that is, there must
be at least one byte assigned to each item, you can't create a
zero-length item). `split_offset` is the length of the first item in
chars, and must be provided because the text used to generate the
item isn't available, so `Item::split` can't count the char
length of the split items itself.
## `split_index`
byte index of position to split item, relative to the start of the item
## `split_offset`
number of chars between start of `self` and `split_index`

# Returns

new item representing text before `split_index`, which
 should be freed with `Item::free`.
<!-- struct Language -->
The `Language` structure is used to
represent a language.

`Language` pointers can be efficiently
copied and compared with each other.
<!-- impl Language::fn get_sample_string -->
Get a string that is representative of the characters needed to
render a particular language.

The sample text may be a pangram, but is not necessarily. It is chosen to
be demonstrative of normal text in the language, as well as exposing font
feature requirements unique to the language. It is suitable for use
as sample text in a font selection dialog.

If `self` is `None`, the default language as found by
`Language::get_default` is used.

If Pango does not have a sample string for `self`, the classic
"The quick brown fox..." is returned. This can be detected by
comparing the returned pointer value to that returned for (non-existent)
language code "xx". That is, compare to:
`<informalexample>``<programlisting>`
pango_language_get_sample_string (pango_language_from_string ("xx"))
`</programlisting>``</informalexample>`

# Returns

the sample string. This value is owned by Pango
 and should not be freed.
<!-- impl Language::fn get_scripts -->
Determines the scripts used to to write `self`.
If nothing is known about the language tag `self`,
or if `self` is `None`, then `None` is returned.
The list of scripts returned starts with the script that the
language uses most and continues to the one it uses least.

The value `num_script` points at will be set to the number
of scripts in the returned array (or zero if `None` is returned).

Most languages use only one script for writing, but there are
some that use two (Latin and Cyrillic for example), and a few
use three (Japanese for example). Applications should not make
any assumptions on the maximum number of scripts returned
though, except that it is positive if the return value is not
`None`, and it is a small number.

The `Language::includes_script` function uses this function
internally.
## `num_scripts`
location to return number of scripts,
 or `None`

# Returns

An array of
`Script` values, with the number of entries in the array stored
in `num_scripts`, or `None` if Pango does not have any information
about this particular language tag (also the case if `self` is
`None`). The returned array is owned by Pango and should not be
modified or freed.
<!-- impl Language::fn includes_script -->
Determines if `script` is one of the scripts used to
write `self`. The returned value is conservative;
if nothing is known about the language tag `self`,
`true` will be returned, since, as far as Pango knows,
`script` might be used to write `self`.

This routine is used in Pango's itemization process when
determining if a supplied language tag is relevant to
a particular section of text. It probably is not useful for
applications in most circumstances.

This function uses `Language::get_scripts` internally.
## `script`
a `Script`

# Returns

`true` if `script` is one of the scripts used
to write `self` or if nothing is known about `self`
(including the case that `self` is `None`),
`false` otherwise.
<!-- impl Language::fn matches -->
Checks if a language tag matches one of the elements in a list of
language ranges. A language tag is considered to match a range
in the list if the range is '*', the range is exactly the tag,
or the range is a prefix of the tag, and the character after it
in the tag is '-'.
## `range_list`
a list of language ranges, separated by ';', ':',
 ',', or space characters.
 Each element must either be '*', or a RFC 3066 language range
 canonicalized as by `Language::from_string`

# Returns

`true` if a match was found.
<!-- impl Language::fn to_string -->
Gets the RFC-3066 format string representing the given language tag.

# Returns

a string representing the language tag. This is owned by
 Pango and should not be freed.
<!-- impl Language::fn from_string -->
Take a RFC-3066 format language tag as a string and convert it to a
`Language` pointer that can be efficiently copied (copy the
pointer) and compared with other language tags (compare the
pointer.)

This function first canonicalizes the string by converting it to
lowercase, mapping '_' to '-', and stripping all characters other
than letters and '-'.

Use `Language::get_default` if you want to get the `Language` for
the current locale of the process.
## `language`
a string representing a language tag, or `None`

# Returns

an opaque pointer to a
 `Language` structure, or `None` if `language` was
 `None`. The returned pointer will be valid forever
 after, and should not be freed.
<!-- impl Language::fn get_default -->
Returns the `Language` for the current locale of the process.
Note that this can change over the life of an application.

On Unix systems, this is the return value is derived from
`<literal>`setlocale(LC_CTYPE, NULL)`</literal>`, and the user can
affect this through the environment variables LC_ALL, LC_CTYPE or
LANG (checked in that order). The locale string typically is in
the form lang_COUNTRY, where lang is an ISO-639 language code, and
COUNTRY is an ISO-3166 country code. For instance, sv_FI for
Swedish as written in Finland or pt_BR for Portuguese as written in
Brazil.

On Windows, the C library does not use any such environment
variables, and setting them won't affect the behavior of functions
like `ctime`. The user sets the locale through the Regional Options
in the Control Panel. The C library (in the `setlocale` function)
does not use country and language codes, but country and language
names spelled out in English.
However, this function does check the above environment
variables, and does return a Unix-style locale string based on
either said environment variables or the thread's current locale.

Your application should call `<literal>`setlocale(LC_ALL, "");`</literal>`
for the user settings to take effect. Gtk+ does this in its initialization
functions automatically (by calling `gtk_set_locale`).
See `<literal>`man setlocale`</literal>` for more details.

# Returns

the default language as a
 `Language`, must not be freed.
<!-- struct Layout -->
The `Layout` structure represents an entire paragraph
of text. It is initialized with a `Context`, UTF-8 string
and set of attributes for that string. Once that is done, the
set of formatted lines can be extracted from the object,
the layout can be rendered, and conversion between logical
character positions within the layout's text, and the physical
position of the resulting glyphs can be made.

There are also a number of parameters to adjust the formatting
of a `Layout`, which are illustrated in <xref linkend="parameters"/>.
It is possible, as well, to ignore the 2-D setup, and simply
treat the results of a `Layout` as a list of lines.

<figure id="parameters">
`<title>`Adjustable parameters for a PangoLayout`</title>`
<graphic fileref="layout.gif" format="GIF">`</graphic>`
`</figure>`

The `Layout` structure is opaque, and has no user-visible
fields.

# Implements

[`LayoutExt`](trait.LayoutExt.html)
<!-- trait LayoutExt -->
Trait containing all `Layout` methods.

# Implementors

[`Layout`](struct.Layout.html)
<!-- impl Layout::fn new -->
Create a new `Layout` object with attributes initialized to
default values for a particular `Context`.
## `context`
a `Context`

# Returns

the newly allocated `Layout`, with a reference
 count of one, which should be freed with
 `gobject::Object::unref`.
<!-- trait LayoutExt::fn context_changed -->
Forces recomputation of any state in the `Layout` that
might depend on the layout's context. This function should
be called if you make changes to the context subsequent
to creating the layout.
<!-- trait LayoutExt::fn copy -->
Does a deep copy-by-value of the `self` layout. The attribute list,
tab array, and text from the original layout are all copied by
value.

# Returns

the newly allocated `Layout`,
 with a reference count of one, which should be freed
 with `gobject::Object::unref`.
<!-- trait LayoutExt::fn get_alignment -->
Gets the alignment for the layout: how partial lines are
positioned within the horizontal space available.

# Returns

the alignment.
<!-- trait LayoutExt::fn get_attributes -->
Gets the attribute list for the layout, if any.

# Returns

a `AttrList`.
<!-- trait LayoutExt::fn get_auto_dir -->
Gets whether to calculate the bidirectional base direction
for the layout according to the contents of the layout.
See `LayoutExt::set_auto_dir`.

# Returns

`true` if the bidirectional base direction
 is computed from the layout's contents, `false` otherwise.
<!-- trait LayoutExt::fn get_baseline -->
Gets the Y position of baseline of the first line in `self`.

# Returns

baseline of first line, from top of `self`.
<!-- trait LayoutExt::fn get_character_count -->
Returns the number of Unicode characters in the
the text of `self`.

# Returns

the number of Unicode characters
 in the text of `self`
<!-- trait LayoutExt::fn get_context -->
Retrieves the `Context` used for this layout.

# Returns

the `Context` for the layout.
This does not have an additional refcount added, so if you want to
keep a copy of this around, you must reference it yourself.
<!-- trait LayoutExt::fn get_cursor_pos -->
Given an index within a layout, determines the positions that of the
strong and weak cursors if the insertion point is at that
index. The position of each cursor is stored as a zero-width
rectangle. The strong cursor location is the location where
characters of the directionality equal to the base direction of the
layout are inserted. The weak cursor location is the location
where characters of the directionality opposite to the base
direction of the layout are inserted.
## `index_`
the byte index of the cursor
## `strong_pos`
location to store the strong cursor position
 (may be `None`)
## `weak_pos`
location to store the weak cursor position (may be `None`)
<!-- trait LayoutExt::fn get_ellipsize -->
Gets the type of ellipsization being performed for `self`.
See `LayoutExt::set_ellipsize`

# Returns

the current ellipsization mode for `self`.

Use `LayoutExt::is_ellipsized` to query whether any paragraphs
were actually ellipsized.
<!-- trait LayoutExt::fn get_extents -->
Computes the logical and ink extents of `self`. Logical extents
are usually what you want for positioning things. Note that both extents
may have non-zero x and y. You may want to use those to offset where you
render the layout. Not doing that is a very typical bug that shows up as
right-to-left layouts not being correctly positioned in a layout with
a set width.

The extents are given in layout coordinates and in Pango units; layout
coordinates begin at the top left corner of the layout.
## `ink_rect`
rectangle used to store the extents of the
 layout as drawn or `None` to indicate that the result is
 not needed.
## `logical_rect`
rectangle used to store the logical
 extents of the layout or `None` to indicate that the
 result is not needed.
<!-- trait LayoutExt::fn get_font_description -->
Gets the font description for the layout, if any.

# Returns

a pointer to the layout's font
 description, or `None` if the font description from the layout's
 context is inherited. This value is owned by the layout and must
 not be modified or freed.
<!-- trait LayoutExt::fn get_height -->
Gets the height of layout used for ellipsization. See
`LayoutExt::set_height` for details.

# Returns

the height, in Pango units if positive, or
number of lines if negative.
<!-- trait LayoutExt::fn get_indent -->
Gets the paragraph indent width in Pango units. A negative value
indicates a hanging indentation.

# Returns

the indent in Pango units.
<!-- trait LayoutExt::fn get_iter -->
Returns an iterator to iterate over the visual extents of the layout.

# Returns

the new `LayoutIter` that should be freed using
 `LayoutIter::free`.
<!-- trait LayoutExt::fn get_justify -->
Gets whether each complete line should be stretched to fill the entire
width of the layout.

# Returns

the justify.
<!-- trait LayoutExt::fn get_line -->
Retrieves a particular line from a `Layout`.

Use the faster `LayoutExt::get_line_readonly` if you do not plan
to modify the contents of the line (glyphs, glyph widths, etc.).
## `line`
the index of a line, which must be between 0 and
 `<literal>`pango_layout_get_line_count(layout) - 1`</literal>`, inclusive.

# Returns

the requested
 `LayoutLine`, or `None` if the index is out of
 range. This layout line can be ref'ed and retained,
 but will become invalid if changes are made to the
 `Layout`.
<!-- trait LayoutExt::fn get_line_count -->
Retrieves the count of lines for the `self`.

# Returns

the line count.
<!-- trait LayoutExt::fn get_line_readonly -->
Retrieves a particular line from a `Layout`.

This is a faster alternative to `LayoutExt::get_line`,
but the user is not expected
to modify the contents of the line (glyphs, glyph widths, etc.).
## `line`
the index of a line, which must be between 0 and
 `<literal>`pango_layout_get_line_count(layout) - 1`</literal>`, inclusive.

# Returns

the requested
 `LayoutLine`, or `None` if the index is out of
 range. This layout line can be ref'ed and retained,
 but will become invalid if changes are made to the
 `Layout`. No changes should be made to the line.
<!-- trait LayoutExt::fn get_lines -->
Returns the lines of the `self` as a list.

Use the faster `LayoutExt::get_lines_readonly` if you do not plan
to modify the contents of the lines (glyphs, glyph widths, etc.).

# Returns

a `glib::SList` containing
the lines in the layout. This points to internal data of the `Layout`
and must be used with care. It will become invalid on any change to the layout's
text or properties.
<!-- trait LayoutExt::fn get_lines_readonly -->
Returns the lines of the `self` as a list.

This is a faster alternative to `LayoutExt::get_lines`,
but the user is not expected
to modify the contents of the lines (glyphs, glyph widths, etc.).

# Returns

a `glib::SList` containing
the lines in the layout. This points to internal data of the `Layout` and
must be used with care. It will become invalid on any change to the layout's
text or properties. No changes should be made to the lines.
<!-- trait LayoutExt::fn get_log_attrs -->
Retrieves an array of logical attributes for each character in
the `self`.
## `attrs`

 location to store a pointer to an array of logical attributes
 This value must be freed with `g_free`.
## `n_attrs`
location to store the number of the attributes in the
 array. (The stored value will be one more than the total number
 of characters in the layout, since there need to be attributes
 corresponding to both the position before the first character
 and the position after the last character.)
<!-- trait LayoutExt::fn get_log_attrs_readonly -->
Retrieves an array of logical attributes for each character in
the `self`.

This is a faster alternative to `LayoutExt::get_log_attrs`.
The returned array is part of `self` and must not be modified.
Modifying the layout will invalidate the returned array.

The number of attributes returned in `n_attrs` will be one more
than the total number of characters in the layout, since there
need to be attributes corresponding to both the position before
the first character and the position after the last character.
## `n_attrs`
location to store the number of the attributes in
 the array

# Returns

an array of logical attributes
<!-- trait LayoutExt::fn get_pixel_extents -->
Computes the logical and ink extents of `self` in device units.
This function just calls `LayoutExt::get_extents` followed by
two `pango_extents_to_pixels` calls, rounding `ink_rect` and `logical_rect`
such that the rounded rectangles fully contain the unrounded one (that is,
passes them as first argument to `pango_extents_to_pixels`).
## `ink_rect`
rectangle used to store the extents of the
 layout as drawn or `None` to indicate that the result is
 not needed.
## `logical_rect`
rectangle used to store the logical
 extents of the layout or `None` to indicate that the
 result is not needed.
<!-- trait LayoutExt::fn get_pixel_size -->
Determines the logical width and height of a `Layout`
in device units. (`LayoutExt::get_size` returns the width
and height scaled by `PANGO_SCALE`.) This
is simply a convenience function around
`LayoutExt::get_pixel_extents`.
## `width`
location to store the logical width, or `None`
## `height`
location to store the logical height, or `None`
<!-- trait LayoutExt::fn get_serial -->
Returns the current serial number of `self`. The serial number is
initialized to an small number larger than zero when a new layout
is created and is increased whenever the layout is changed using any
of the setter functions, or the `Context` it uses has changed.
The serial may wrap, but will never have the value 0. Since it
can wrap, never compare it with "less than", always use "not equals".

This can be used to automatically detect changes to a `Layout`, and
is useful for example to decide whether a layout needs redrawing.
To force the serial to be increased, use `LayoutExt::context_changed`.

Feature: `v1_32_4`


# Returns

The current serial number of `self`.
<!-- trait LayoutExt::fn get_single_paragraph_mode -->
Obtains the value set by `LayoutExt::set_single_paragraph_mode`.

# Returns

`true` if the layout does not break paragraphs at
paragraph separator characters, `false` otherwise.
<!-- trait LayoutExt::fn get_size -->
Determines the logical width and height of a `Layout`
in Pango units (device units scaled by `PANGO_SCALE`). This
is simply a convenience function around `LayoutExt::get_extents`.
## `width`
location to store the logical width, or `None`
## `height`
location to store the logical height, or `None`
<!-- trait LayoutExt::fn get_spacing -->
Gets the amount of spacing between the lines of the layout.

# Returns

the spacing in Pango units.
<!-- trait LayoutExt::fn get_tabs -->
Gets the current `TabArray` used by this layout. If no
`TabArray` has been set, then the default tabs are in use
and `None` is returned. Default tabs are every 8 spaces.
The return value should be freed with `TabArray::free`.

# Returns

a copy of the tabs for this layout, or
`None`.
<!-- trait LayoutExt::fn get_text -->
Gets the text in the layout. The returned text should not
be freed or modified.

# Returns

the text in the `self`.
<!-- trait LayoutExt::fn get_unknown_glyphs_count -->
Counts the number unknown glyphs in `self`. That is, zero if
glyphs for all characters in the layout text were found, or more
than zero otherwise.

This function can be used to determine if there are any fonts
available to render all characters in a certain string, or when
used in combination with `AttrType::Fallback`, to check if a
certain font supports all the characters in the string.

# Returns

The number of unknown glyphs in `self`.
<!-- trait LayoutExt::fn get_width -->
Gets the width to which the lines of the `Layout` should wrap.

# Returns

the width in Pango units, or -1 if no width set.
<!-- trait LayoutExt::fn get_wrap -->
Gets the wrap mode for the layout.

Use `LayoutExt::is_wrapped` to query whether any paragraphs
were actually wrapped.

# Returns

active wrap mode.
<!-- trait LayoutExt::fn index_to_line_x -->
Converts from byte `index_` within the `self` to line and X position.
(X position is measured from the left edge of the line)
## `index_`
the byte index of a grapheme within the layout.
## `trailing`
an integer indicating the edge of the grapheme to retrieve the
 position of. If > 0, the trailing edge of the grapheme, if 0,
 the leading of the grapheme.
## `line`
location to store resulting line index. (which will
 between 0 and pango_layout_get_line_count(layout) - 1), or `None`
## `x_pos`
location to store resulting position within line
 (`PANGO_SCALE` units per device unit), or `None`
<!-- trait LayoutExt::fn index_to_pos -->
Converts from an index within a `Layout` to the onscreen position
corresponding to the grapheme at that index, which is represented
as rectangle. Note that `<literal>`pos->x`</literal>` is always the leading
edge of the grapheme and `<literal>`pos->x + pos->width`</literal>` the trailing
edge of the grapheme. If the directionality of the grapheme is right-to-left,
then `<literal>`pos->width`</literal>` will be negative.
## `index_`
byte index within `self`
## `pos`
rectangle in which to store the position of the grapheme
<!-- trait LayoutExt::fn is_ellipsized -->
Queries whether the layout had to ellipsize any paragraphs.

This returns `true` if the ellipsization mode for `self`
is not `EllipsizeMode::None`, a positive width is set on `self`,
and there are paragraphs exceeding that width that have to be
ellipsized.

# Returns

`true` if any paragraphs had to be ellipsized, `false`
otherwise.
<!-- trait LayoutExt::fn is_wrapped -->
Queries whether the layout had to wrap any paragraphs.

This returns `true` if a positive width is set on `self`,
ellipsization mode of `self` is set to `EllipsizeMode::None`,
and there are paragraphs exceeding the layout width that have
to be wrapped.

# Returns

`true` if any paragraphs had to be wrapped, `false`
otherwise.
<!-- trait LayoutExt::fn move_cursor_visually -->
Computes a new cursor position from an old position and
a count of positions to move visually. If `direction` is positive,
then the new strong cursor position will be one position
to the right of the old cursor position. If `direction` is negative,
then the new strong cursor position will be one position
to the left of the old cursor position.

In the presence of bidirectional text, the correspondence
between logical and visual order will depend on the direction
of the current run, and there may be jumps when the cursor
is moved off of the end of a run.

Motion here is in cursor positions, not in characters, so a
single call to `LayoutExt::move_cursor_visually` may move the
cursor over multiple characters when multiple characters combine
to form a single grapheme.
## `strong`
whether the moving cursor is the strong cursor or the
 weak cursor. The strong cursor is the cursor corresponding
 to text insertion in the base direction for the layout.
## `old_index`
the byte index of the grapheme for the old index
## `old_trailing`
if 0, the cursor was at the leading edge of the
 grapheme indicated by `old_index`, if > 0, the cursor
 was at the trailing edge.
## `direction`
direction to move cursor. A negative
 value indicates motion to the left.
## `new_index`
location to store the new cursor byte index. A value of -1
 indicates that the cursor has been moved off the beginning
 of the layout. A value of `G_MAXINT` indicates that
 the cursor has been moved off the end of the layout.
## `new_trailing`
number of characters to move forward from the
 location returned for `new_index` to get the position
 where the cursor should be displayed. This allows
 distinguishing the position at the beginning of one
 line from the position at the end of the preceding
 line. `new_index` is always on the line where the
 cursor should be displayed.
<!-- trait LayoutExt::fn set_alignment -->
Sets the alignment for the layout: how partial lines are
positioned within the horizontal space available.
## `alignment`
the alignment
<!-- trait LayoutExt::fn set_attributes -->
Sets the text attributes for a layout object.
References `attrs`, so the caller can unref its reference.
## `attrs`
a `AttrList`, can be `None`
<!-- trait LayoutExt::fn set_auto_dir -->
Sets whether to calculate the bidirectional base direction
for the layout according to the contents of the layout;
when this flag is on (the default), then paragraphs in
 `self` that begin with strong right-to-left characters
(Arabic and Hebrew principally), will have right-to-left
layout, paragraphs with letters from other scripts will
have left-to-right layout. Paragraphs with only neutral
characters get their direction from the surrounding paragraphs.

When `false`, the choice between left-to-right and
right-to-left layout is done according to the base direction
of the layout's `Context`. (See `ContextExt::set_base_dir`).

When the auto-computed direction of a paragraph differs from the
base direction of the context, the interpretation of
`Alignment::Left` and `Alignment::Right` are swapped.
## `auto_dir`
if `true`, compute the bidirectional base direction
 from the layout's contents.
<!-- trait LayoutExt::fn set_ellipsize -->
Sets the type of ellipsization being performed for `self`.
Depending on the ellipsization mode `ellipsize` text is
removed from the start, middle, or end of text so they
fit within the width and height of layout set with
`LayoutExt::set_width` and `LayoutExt::set_height`.

If the layout contains characters such as newlines that
force it to be layed out in multiple paragraphs, then whether
each paragraph is ellipsized separately or the entire layout
is ellipsized as a whole depends on the set height of the layout.
See `LayoutExt::set_height` for details.
## `ellipsize`
the new ellipsization mode for `self`
<!-- trait LayoutExt::fn set_font_description -->
Sets the default font description for the layout. If no font
description is set on the layout, the font description from
the layout's context is used.
## `desc`
the new `FontDescription`, or `None` to unset the
 current font description
<!-- trait LayoutExt::fn set_height -->
Sets the height to which the `Layout` should be ellipsized at. There
are two different behaviors, based on whether `height` is positive or
negative.

If `height` is positive, it will be the maximum height of the layout. Only
lines would be shown that would fit, and if there is any text omitted,
an ellipsis added. At least one line is included in each paragraph regardless
of how small the height value is. A value of zero will render exactly one
line for the entire layout.

If `height` is negative, it will be the (negative of) maximum number of lines per
paragraph. That is, the total number of lines shown may well be more than
this value if the layout contains multiple paragraphs of text.
The default value of -1 means that first line of each paragraph is ellipsized.
This behvaior may be changed in the future to act per layout instead of per
paragraph. File a bug against pango at <ulink
url="http://bugzilla.gnome.org/">http://bugzilla.gnome.org/`</ulink>` if your
code relies on this behavior.

Height setting only has effect if a positive width is set on
`self` and ellipsization mode of `self` is not `EllipsizeMode::None`.
The behavior is undefined if a height other than -1 is set and
ellipsization mode is set to `EllipsizeMode::None`, and may change in the
future.
## `height`
the desired height of the layout in Pango units if positive,
 or desired number of lines if negative.
<!-- trait LayoutExt::fn set_indent -->
Sets the width in Pango units to indent each paragraph. A negative value
of `indent` will produce a hanging indentation. That is, the first line will
have the full width, and subsequent lines will be indented by the
absolute value of `indent`.

The indent setting is ignored if layout alignment is set to
`Alignment::Center`.
## `indent`
the amount by which to indent.
<!-- trait LayoutExt::fn set_justify -->
Sets whether each complete line should be stretched to
fill the entire width of the layout. This stretching is typically
done by adding whitespace, but for some scripts (such as Arabic),
the justification may be done in more complex ways, like extending
the characters.

Note that this setting is not implemented and so is ignored in Pango
older than 1.18.
## `justify`
whether the lines in the layout should be justified.
<!-- trait LayoutExt::fn set_markup -->
Same as `LayoutExt::set_markup_with_accel`, but
the markup text isn't scanned for accelerators.
## `markup`
marked-up text
## `length`
length of marked-up text in bytes, or -1 if `markup` is
 null-terminated
<!-- trait LayoutExt::fn set_markup_with_accel -->
Sets the layout text and attribute list from marked-up text (see
<link linkend="PangoMarkupFormat">markup format`</link>`). Replaces
the current text and attribute list.

If `accel_marker` is nonzero, the given character will mark the
character following it as an accelerator. For example, `accel_marker`
might be an ampersand or underscore. All characters marked
as an accelerator will receive a `Underline::Low` attribute,
and the first character so marked will be returned in `accel_char`.
Two `accel_marker` characters following each other produce a single
literal `accel_marker` character.
## `markup`
marked-up text
(see <link linkend="PangoMarkupFormat">markup format`</link>`)
## `length`
length of marked-up text in bytes, or -1 if `markup` is
 null-terminated
## `accel_marker`
marker for accelerators in the text
## `accel_char`
return location
 for first located accelerator, or `None`
<!-- trait LayoutExt::fn set_single_paragraph_mode -->
If `setting` is `true`, do not treat newlines and similar characters
as paragraph separators; instead, keep all text in a single paragraph,
and display a glyph for paragraph separator characters. Used when
you want to allow editing of newlines on a single text line.
## `setting`
new setting
<!-- trait LayoutExt::fn set_spacing -->
Sets the amount of spacing in Pango unit between the lines of the
layout.
## `spacing`
the amount of spacing
<!-- trait LayoutExt::fn set_tabs -->
Sets the tabs to use for `self`, overriding the default tabs
(by default, tabs are every 8 spaces). If `tabs` is `None`, the default
tabs are reinstated. `tabs` is copied into the layout; you must
free your copy of `tabs` yourself.
## `tabs`
a `TabArray`, or `None`
<!-- trait LayoutExt::fn set_text -->
Sets the text of the layout.

Note that if you have used
`LayoutExt::set_markup` or `LayoutExt::set_markup_with_accel` on
`self` before, you may want to call `LayoutExt::set_attributes` to clear
the attributes set on the layout from the markup as this function does not
clear attributes.
## `text`
a valid UTF-8 string
## `length`
maximum length of `text`, in bytes. -1 indicates that
 the string is nul-terminated and the length should be
 calculated. The text will also be truncated on
 encountering a nul-termination even when `length` is
 positive.
<!-- trait LayoutExt::fn set_width -->
Sets the width to which the lines of the `Layout` should wrap or
ellipsized. The default value is -1: no width set.
## `width`
the desired width in Pango units, or -1 to indicate that no
 wrapping or ellipsization should be performed.
<!-- trait LayoutExt::fn set_wrap -->
Sets the wrap mode; the wrap mode only has effect if a width
is set on the layout with `LayoutExt::set_width`.
To turn off wrapping, set the width to -1.
## `wrap`
the wrap mode
<!-- trait LayoutExt::fn xy_to_index -->
Converts from X and Y position within a layout to the byte
index to the character at that logical position. If the
Y position is not inside the layout, the closest position is chosen
(the position will be clamped inside the layout). If the
X position is not within the layout, then the start or the
end of the line is chosen as described for `LayoutLine::x_to_index`.
If either the X or Y positions were not inside the layout, then the
function returns `false`; on an exact hit, it returns `true`.
## `x`
the X offset (in Pango units)
 from the left edge of the layout.
## `y`
the Y offset (in Pango units)
 from the top edge of the layout
## `index_`
location to store calculated byte index
## `trailing`
location to store a integer indicating where
 in the grapheme the user clicked. It will either
 be zero, or the number of characters in the
 grapheme. 0 represents the leading edge of the grapheme.

# Returns

`true` if the coordinates were inside text, `false` otherwise.
<!-- struct LayoutIter -->
A `LayoutIter` structure can be used to
iterate over the visual extents of a `Layout`.

The `LayoutIter` structure is opaque, and
has no user-visible fields.
<!-- impl LayoutIter::fn at_last_line -->
Determines whether `self` is on the last line of the layout.

# Returns

`true` if `self` is on the last line.
<!-- impl LayoutIter::fn copy -->
Copies a `LayoutIter`.

# Returns

the newly allocated `LayoutIter`,
 which should be freed with `LayoutIter::free`,
 or `None` if `self` was `None`.
<!-- impl LayoutIter::fn free -->
Frees an iterator that's no longer in use.
<!-- impl LayoutIter::fn get_baseline -->
Gets the Y position of the current line's baseline, in layout
coordinates (origin at top left of the entire layout).

# Returns

baseline of current line.
<!-- impl LayoutIter::fn get_char_extents -->
Gets the extents of the current character, in layout coordinates
(origin is the top left of the entire layout). Only logical extents
can sensibly be obtained for characters; ink extents make sense only
down to the level of clusters.
## `logical_rect`
rectangle to fill with
 logical extents
<!-- impl LayoutIter::fn get_cluster_extents -->
Gets the extents of the current cluster, in layout coordinates
(origin is the top left of the entire layout).
## `ink_rect`
rectangle to fill with ink extents, or `None`
## `logical_rect`
rectangle to fill with logical extents, or `None`
<!-- impl LayoutIter::fn get_index -->
Gets the current byte index. Note that iterating forward by char
moves in visual order, not logical order, so indexes may not be
sequential. Also, the index may be equal to the length of the text
in the layout, if on the `None` run (see `LayoutIter::get_run`).

# Returns

current byte index.
<!-- impl LayoutIter::fn get_layout -->
Gets the layout associated with a `LayoutIter`.

# Returns

the layout associated with `self`.
<!-- impl LayoutIter::fn get_layout_extents -->
Obtains the extents of the `Layout` being iterated
over. `ink_rect` or `logical_rect` can be `None` if you
aren't interested in them.
## `ink_rect`
rectangle to fill with ink extents,
 or `None`
## `logical_rect`
rectangle to fill with logical
 extents, or `None`
<!-- impl LayoutIter::fn get_line -->
Gets the current line.

Use the faster `LayoutIter::get_line_readonly` if you do not plan
to modify the contents of the line (glyphs, glyph widths, etc.).

# Returns

the current line.
<!-- impl LayoutIter::fn get_line_extents -->
Obtains the extents of the current line. `ink_rect` or `logical_rect`
can be `None` if you aren't interested in them. Extents are in layout
coordinates (origin is the top-left corner of the entire
`Layout`). Thus the extents returned by this function will be
the same width/height but not at the same x/y as the extents
returned from `LayoutLine::get_extents`.
## `ink_rect`
rectangle to fill with ink extents, or `None`
## `logical_rect`
rectangle to fill with logical extents, or `None`
<!-- impl LayoutIter::fn get_line_readonly -->
Gets the current line for read-only access.

This is a faster alternative to `LayoutIter::get_line`,
but the user is not expected
to modify the contents of the line (glyphs, glyph widths, etc.).

# Returns

the current line, that should not be
modified.
<!-- impl LayoutIter::fn get_line_yrange -->
Divides the vertical space in the `Layout` being iterated over
between the lines in the layout, and returns the space belonging to
the current line. A line's range includes the line's logical
extents, plus half of the spacing above and below the line, if
`LayoutExt::set_spacing` has been called to set layout spacing.
The Y positions are in layout coordinates (origin at top left of the
entire layout).
## `y0_`
start of line, or `None`
## `y1_`
end of line, or `None`
<!-- impl LayoutIter::fn get_run -->
Gets the current run. When iterating by run, at the end of each
line, there's a position with a `None` run, so this function can return
`None`. The `None` run at the end of each line ensures that all lines have
at least one run, even lines consisting of only a newline.

Use the faster `LayoutIter::get_run_readonly` if you do not plan
to modify the contents of the run (glyphs, glyph widths, etc.).

# Returns

the current run.
<!-- impl LayoutIter::fn get_run_extents -->
Gets the extents of the current run in layout coordinates
(origin is the top left of the entire layout).
## `ink_rect`
rectangle to fill with ink extents, or `None`
## `logical_rect`
rectangle to fill with logical extents, or `None`
<!-- impl LayoutIter::fn get_run_readonly -->
Gets the current run. When iterating by run, at the end of each
line, there's a position with a `None` run, so this function can return
`None`. The `None` run at the end of each line ensures that all lines have
at least one run, even lines consisting of only a newline.

This is a faster alternative to `LayoutIter::get_run`,
but the user is not expected
to modify the contents of the run (glyphs, glyph widths, etc.).

# Returns

the current run, that
should not be modified.
<!-- impl LayoutIter::fn next_char -->
Moves `self` forward to the next character in visual order. If `self` was already at
the end of the layout, returns `false`.

# Returns

whether motion was possible.
<!-- impl LayoutIter::fn next_cluster -->
Moves `self` forward to the next cluster in visual order. If `self`
was already at the end of the layout, returns `false`.

# Returns

whether motion was possible.
<!-- impl LayoutIter::fn next_line -->
Moves `self` forward to the start of the next line. If `self` is
already on the last line, returns `false`.

# Returns

whether motion was possible.
<!-- impl LayoutIter::fn next_run -->
Moves `self` forward to the next run in visual order. If `self` was
already at the end of the layout, returns `false`.

# Returns

whether motion was possible.
<!-- struct LayoutLine -->
The `LayoutLine` structure represents one of the lines resulting
from laying out a paragraph via `Layout`. `LayoutLine`
structures are obtained by calling `LayoutExt::get_line` and
are only valid until the text, attributes, or settings of the
parent `Layout` are modified.

Routines for rendering PangoLayout objects are provided in
code specific to each rendering system.
<!-- impl LayoutLine::fn get_extents -->
Computes the logical and ink extents of a layout line. See
`FontExt::get_glyph_extents` for details about the interpretation
of the rectangles.
## `ink_rect`
rectangle used to store the extents of
 the glyph string as drawn, or `None`
## `logical_rect`
rectangle used to store the logical
 extents of the glyph string, or `None`
<!-- impl LayoutLine::fn get_pixel_extents -->
Computes the logical and ink extents of `self` in device units.
This function just calls `LayoutLine::get_extents` followed by
two `pango_extents_to_pixels` calls, rounding `ink_rect` and `logical_rect`
such that the rounded rectangles fully contain the unrounded one (that is,
passes them as first argument to `pango_extents_to_pixels`).
## `ink_rect`
rectangle used to store the extents of
 the glyph string as drawn, or `None`
## `logical_rect`
rectangle used to store the logical
 extents of the glyph string, or `None`
<!-- impl LayoutLine::fn get_x_ranges -->
Gets a list of visual ranges corresponding to a given logical range.
This list is not necessarily minimal - there may be consecutive
ranges which are adjacent. The ranges will be sorted from left to
right. The ranges are with respect to the left edge of the entire
layout, not with respect to the line.
## `start_index`
Start byte index of the logical range. If this value
 is less than the start index for the line, then
 the first range will extend all the way to the leading
 edge of the layout. Otherwise it will start at the
 leading edge of the first character.
## `end_index`
Ending byte index of the logical range. If this value
 is greater than the end index for the line, then
 the last range will extend all the way to the trailing
 edge of the layout. Otherwise, it will end at the
 trailing edge of the last character.
## `ranges`

 location to store a pointer to an array of ranges.
 The array will be of length `<literal>`2*n_ranges`</literal>`,
 with each range starting at `<literal>`(*ranges)[2*n]`</literal>`
 and of width `<literal>`(*ranges)[2*n + 1] - (*ranges)[2*n]`</literal>`.
 This array must be freed with `g_free`. The coordinates are relative
 to the layout and are in Pango units.
## `n_ranges`
The number of ranges stored in `ranges`.
<!-- impl LayoutLine::fn index_to_x -->
Converts an index within a line to a X position.
## `index_`
byte offset of a grapheme within the layout
## `trailing`
an integer indicating the edge of the grapheme to retrieve
 the position of. If > 0, the trailing edge of the grapheme,
 if 0, the leading of the grapheme.
## `x_pos`
location to store the x_offset (in Pango unit)
<!-- impl LayoutLine::fn ref -->
Increase the reference count of a `LayoutLine` by one.

# Returns

the line passed in.
<!-- impl LayoutLine::fn unref -->
Decrease the reference count of a `LayoutLine` by one.
If the result is zero, the line and all associated memory
will be freed.
<!-- impl LayoutLine::fn x_to_index -->
Converts from x offset to the byte index of the corresponding
character within the text of the layout. If `x_pos` is outside the line,
`index_` and `trailing` will point to the very first or very last position
in the line. This determination is based on the resolved direction
of the paragraph; for example, if the resolved direction is
right-to-left, then an X position to the right of the line (after it)
results in 0 being stored in `index_` and `trailing`. An X position to the
left of the line results in `index_` pointing to the (logical) last
grapheme in the line and `trailing` being set to the number of characters
in that grapheme. The reverse is true for a left-to-right line.
## `x_pos`
the X offset (in Pango units)
 from the left edge of the line.
## `index_`
location to store calculated byte index for
 the grapheme in which the user clicked.
## `trailing`
location to store an integer indicating where
 in the grapheme the user clicked. It will either
 be zero, or the number of characters in the
 grapheme. 0 represents the leading edge of the grapheme.

# Returns

`false` if `x_pos` was outside the line, `true` if inside
<!-- struct Matrix -->
A structure specifying a transformation between user-space
coordinates and device coordinates. The transformation
is given by

`<programlisting>`
x_device = x_user * matrix->xx + y_user * matrix->xy + matrix->x0;
y_device = x_user * matrix->yx + y_user * matrix->yy + matrix->y0;
`</programlisting>`
<!-- impl Matrix::fn concat -->
Changes the transformation represented by `self` to be the
transformation given by first applying transformation
given by `new_matrix` then applying the original transformation.
## `new_matrix`
a `Matrix`
<!-- impl Matrix::fn copy -->
Copies a `Matrix`.

# Returns

the newly allocated `Matrix`, which
 should be freed with `Matrix::free`, or `None` if
 `self` was `None`.
<!-- impl Matrix::fn free -->
Free a `Matrix` created with `Matrix::copy`.
<!-- impl Matrix::fn get_font_scale_factor -->
Returns the scale factor of a matrix on the height of the font.
That is, the scale factor in the direction perpendicular to the
vector that the X coordinate is mapped to. If the scale in the X
coordinate is needed as well, use `Matrix::get_font_scale_factors`.

# Returns

the scale factor of `self` on the height of the font,
or 1.0 if `self` is `None`.
<!-- impl Matrix::fn get_font_scale_factors -->
Calculates the scale factor of a matrix on the width and height of the font.
That is, `xscale` is the scale factor in the direction of the X coordinate,
and `yscale` is the scale factor in the direction perpendicular to the
vector that the X coordinate is mapped to.

Note that output numbers will always be non-negative.

Feature: `v1_38`

## `xscale`
output scale factor in the x direction, or `None`
## `yscale`
output scale factor perpendicular to the x direction, or `None`
<!-- impl Matrix::fn rotate -->
Changes the transformation represented by `self` to be the
transformation given by first rotating by `degrees` degrees
counter-clockwise then applying the original transformation.
## `degrees`
degrees to rotate counter-clockwise
<!-- impl Matrix::fn scale -->
Changes the transformation represented by `self` to be the
transformation given by first scaling by `sx` in the X direction
and `sy` in the Y direction then applying the original
transformation.
## `scale_x`
amount to scale by in X direction
## `scale_y`
amount to scale by in Y direction
<!-- impl Matrix::fn transform_distance -->
Transforms the distance vector (`dx`,`dy`) by `self`. This is
similar to `Matrix::transform_point` except that the translation
components of the transformation are ignored. The calculation of
the returned vector is as follows:

`<programlisting>`
dx2 = dx1 * xx + dy1 * xy;
dy2 = dx1 * yx + dy1 * yy;
`</programlisting>`

Affine transformations are position invariant, so the same vector
always transforms to the same vector. If (`x1`,`y1`) transforms
to (`x2`,`y2`) then (`x1`+`dx1`,`y1`+`dy1`) will transform to
(`x1`+`dx2`,`y1`+`dy2`) for all values of `x1` and `x2`.
## `dx`
in/out X component of a distance vector
## `dy`
in/out Y component of a distance vector
<!-- impl Matrix::fn transform_pixel_rectangle -->
First transforms the `rect` using `self`, then calculates the bounding box
of the transformed rectangle. The rectangle should be in device units
(pixels).

This function is useful for example when you want to draw a rotated
`Layout` to an image buffer, and want to know how large the image
should be and how much you should shift the layout when rendering.

For better accuracy, you should use `Matrix::transform_rectangle` on
original rectangle in Pango units and convert to pixels afterward
using `pango_extents_to_pixels`'s first argument.
## `rect`
in/out bounding box in device units, or `None`
<!-- impl Matrix::fn transform_point -->
Transforms the point (`x`, `y`) by `self`.
## `x`
in/out X position
## `y`
in/out Y position
<!-- impl Matrix::fn transform_rectangle -->
First transforms `rect` using `self`, then calculates the bounding box
of the transformed rectangle. The rectangle should be in Pango units.

This function is useful for example when you want to draw a rotated
`Layout` to an image buffer, and want to know how large the image
should be and how much you should shift the layout when rendering.

If you have a rectangle in device units (pixels), use
`Matrix::transform_pixel_rectangle`.

If you have the rectangle in Pango units and want to convert to
transformed pixel bounding box, it is more accurate to transform it first
(using this function) and pass the result to `pango_extents_to_pixels`,
first argument, for an inclusive rounded rectangle.
However, there are valid reasons that you may want to convert
to pixels first and then transform, for example when the transformed
coordinates may overflow in Pango units (large matrix translation for
example).
## `rect`
in/out bounding box in Pango units, or `None`
<!-- impl Matrix::fn translate -->
Changes the transformation represented by `self` to be the
transformation given by first translating by (`tx`, `ty`)
then applying the original transformation.
## `tx`
amount to translate in the X direction
## `ty`
amount to translate in the Y direction
<!-- struct Rectangle -->
The `Rectangle` structure represents a rectangle. It is frequently
used to represent the logical or ink extents of a single glyph or section
of text. (See, for instance, `FontExt::get_glyph_extents`)
<!-- enum RenderPart -->
`RenderPart` defines different items to render for such
purposes as setting colors.
<!-- enum RenderPart::variant Foreground -->
the text itself
<!-- enum RenderPart::variant Background -->
the area behind the text
<!-- enum RenderPart::variant Underline -->
underlines
<!-- enum RenderPart::variant Strikethrough -->
strikethrough lines
<!-- struct Renderer -->
`Renderer` is a base class for objects that are used to
render Pango objects such as `GlyphString` and
`Layout`.

# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- trait RendererExt -->
Trait containing all `Renderer` methods.

# Implementors

[`Renderer`](struct.Renderer.html)
<!-- trait RendererExt::fn activate -->
Does initial setup before rendering operations on `self`.
`RendererExt::deactivate` should be called when done drawing.
Calls such as `RendererExt::draw_layout` automatically
activate the layout before drawing on it. Calls to
`RendererExt::activate` and `RendererExt::deactivate` can
be nested and the renderer will only be initialized and
deinitialized once.
<!-- trait RendererExt::fn deactivate -->
Cleans up after rendering operations on `self`. See
docs for `RendererExt::activate`.
<!-- trait RendererExt::fn draw_error_underline -->
Draw a squiggly line that approximately covers the given rectangle
in the style of an underline used to indicate a spelling error.
(The width of the underline is rounded to an integer number
of up/down segments and the resulting rectangle is centered
in the original rectangle)

This should be called while `self` is already active. Use
`RendererExt::activate` to activate a renderer.
## `x`
X coordinate of underline, in Pango units in user coordinate system
## `y`
Y coordinate of underline, in Pango units in user coordinate system
## `width`
width of underline, in Pango units in user coordinate system
## `height`
height of underline, in Pango units in user coordinate system
<!-- trait RendererExt::fn draw_glyph -->
Draws a single glyph with coordinates in device space.
## `font`
a `Font`
## `glyph`
the glyph index of a single glyph
## `x`
X coordinate of left edge of baseline of glyph
## `y`
Y coordinate of left edge of baseline of glyph
<!-- trait RendererExt::fn draw_glyph_item -->
Draws the glyphs in `glyph_item` with the specified `Renderer`,
embedding the text associated with the glyphs in the output if the
output format supports it (PDF for example).

Note that `text` is the start of the text for layout, which is then
indexed by `<literal>``glyph_item`->item->offset`</literal>`.

If `text` is `None`, this simply calls `RendererExt::draw_glyphs`.

The default implementation of this method simply falls back to
`RendererExt::draw_glyphs`.
## `text`
the UTF-8 text that `glyph_item` refers to, or `None`
## `glyph_item`
a `GlyphItem`
## `x`
X position of left edge of baseline, in user space coordinates
 in Pango units.
## `y`
Y position of left edge of baseline, in user space coordinates
 in Pango units.
<!-- trait RendererExt::fn draw_glyphs -->
Draws the glyphs in `glyphs` with the specified `Renderer`.
## `font`
a `Font`
## `glyphs`
a `GlyphString`
## `x`
X position of left edge of baseline, in user space coordinates
 in Pango units.
## `y`
Y position of left edge of baseline, in user space coordinates
 in Pango units.
<!-- trait RendererExt::fn draw_layout -->
Draws `layout` with the specified `Renderer`.
## `layout`
a `Layout`
## `x`
X position of left edge of baseline, in user space coordinates
 in Pango units.
## `y`
Y position of left edge of baseline, in user space coordinates
 in Pango units.
<!-- trait RendererExt::fn draw_layout_line -->
Draws `line` with the specified `Renderer`.
## `line`
a `LayoutLine`
## `x`
X position of left edge of baseline, in user space coordinates
 in Pango units.
## `y`
Y position of left edge of baseline, in user space coordinates
 in Pango units.
<!-- trait RendererExt::fn draw_rectangle -->
Draws an axis-aligned rectangle in user space coordinates with the
specified `Renderer`.

This should be called while `self` is already active. Use
`RendererExt::activate` to activate a renderer.
## `part`
type of object this rectangle is part of
## `x`
X position at which to draw rectangle, in user space coordinates in Pango units
## `y`
Y position at which to draw rectangle, in user space coordinates in Pango units
## `width`
width of rectangle in Pango units in user space coordinates
## `height`
height of rectangle in Pango units in user space coordinates
<!-- trait RendererExt::fn draw_trapezoid -->
Draws a trapezoid with the parallel sides aligned with the X axis
using the given `Renderer`; coordinates are in device space.
## `part`
type of object this trapezoid is part of
## `y1_`
Y coordinate of top of trapezoid
## `x11`
X coordinate of left end of top of trapezoid
## `x21`
X coordinate of right end of top of trapezoid
## `y2`
Y coordinate of bottom of trapezoid
## `x12`
X coordinate of left end of bottom of trapezoid
## `x22`
X coordinate of right end of bottom of trapezoid
<!-- trait RendererExt::fn get_alpha -->
Gets the current alpha for the specified part.

Feature: `v1_38`

## `part`
the part to get the alpha for

# Returns

the alpha for the specified part,
 or 0 if it hasn't been set and should be
 inherited from the environment.
<!-- trait RendererExt::fn get_color -->
Gets the current rendering color for the specified part.
## `part`
the part to get the color for

# Returns

the color for the
 specified part, or `None` if it hasn't been set and should be
 inherited from the environment.
<!-- trait RendererExt::fn get_layout -->
Gets the layout currently being rendered using `self`.
Calling this function only makes sense from inside a subclass's
methods, like in its draw_shape<!---->() for example.

The returned layout should not be modified while still being
rendered.

# Returns

the layout, or `None` if
 no layout is being rendered using `self` at this time.
<!-- trait RendererExt::fn get_layout_line -->
Gets the layout line currently being rendered using `self`.
Calling this function only makes sense from inside a subclass's
methods, like in its draw_shape<!---->() for example.

The returned layout line should not be modified while still being
rendered.

# Returns

the layout line, or `None`
 if no layout line is being rendered using `self` at this time.
<!-- trait RendererExt::fn get_matrix -->
Gets the transformation matrix that will be applied when
rendering. See `RendererExt::set_matrix`.

# Returns

the matrix, or `None` if no matrix has
 been set (which is the same as the identity matrix). The returned
 matrix is owned by Pango and must not be modified or freed.
<!-- trait RendererExt::fn part_changed -->
Informs Pango that the way that the rendering is done
for `part` has changed in a way that would prevent multiple
pieces being joined together into one drawing call. For
instance, if a subclass of `Renderer` was to add a stipple
option for drawing underlines, it needs to call

`<informalexample>``<programlisting>`
pango_renderer_part_changed (render, PANGO_RENDER_PART_UNDERLINE);
`</programlisting>``</informalexample>`

When the stipple changes or underlines with different stipples
might be joined together. Pango automatically calls this for
changes to colors. (See `RendererExt::set_color`)
## `part`
the part for which rendering has changed.
<!-- trait RendererExt::fn set_alpha -->
Sets the alpha for part of the rendering.
Note that the alpha may only be used if a color is
specified for `part` as well.

Feature: `v1_38`

## `part`
the part to set the alpha for
## `alpha`
an alpha value between 1 and 65536, or 0 to unset the alpha
<!-- trait RendererExt::fn set_color -->
Sets the color for part of the rendering.
Also see `RendererExt::set_alpha`.
## `part`
the part to change the color of
## `color`
the new color or `None` to unset the current color
<!-- trait RendererExt::fn set_matrix -->
Sets the transformation matrix that will be applied when rendering.
## `matrix`
a `Matrix`, or `None` to unset any existing matrix.
 (No matrix set is the same as setting the identity matrix.)
<!-- enum Script -->
The `Script` enumeration identifies different writing
systems. The values correspond to the names as defined in the
Unicode standard.
Note that new types may be added in the future. Applications should be ready
to handle unknown values. This enumeration is interchangeable with
`glib::UnicodeScript`. See <ulink
url="http://www.unicode.org/reports/tr24/">Unicode Standard Annex
`24`: Script names`</ulink>`.
<!-- enum Script::variant InvalidCode -->
a value never returned from `Script::for_unichar`
<!-- enum Script::variant Common -->
a character used by multiple different scripts
<!-- enum Script::variant Inherited -->
a mark glyph that takes its script from the
base glyph to which it is attached
<!-- enum Script::variant Arabic -->
Arabic
<!-- enum Script::variant Armenian -->
Armenian
<!-- enum Script::variant Bengali -->
Bengali
<!-- enum Script::variant Bopomofo -->
Bopomofo
<!-- enum Script::variant Cherokee -->
Cherokee
<!-- enum Script::variant Coptic -->
Coptic
<!-- enum Script::variant Cyrillic -->
Cyrillic
<!-- enum Script::variant Deseret -->
Deseret
<!-- enum Script::variant Devanagari -->
Devanagari
<!-- enum Script::variant Ethiopic -->
Ethiopic
<!-- enum Script::variant Georgian -->
Georgian
<!-- enum Script::variant Gothic -->
Gothic
<!-- enum Script::variant Greek -->
Greek
<!-- enum Script::variant Gujarati -->
Gujarati
<!-- enum Script::variant Gurmukhi -->
Gurmukhi
<!-- enum Script::variant Han -->
Han
<!-- enum Script::variant Hangul -->
Hangul
<!-- enum Script::variant Hebrew -->
Hebrew
<!-- enum Script::variant Hiragana -->
Hiragana
<!-- enum Script::variant Kannada -->
Kannada
<!-- enum Script::variant Katakana -->
Katakana
<!-- enum Script::variant Khmer -->
Khmer
<!-- enum Script::variant Lao -->
Lao
<!-- enum Script::variant Latin -->
Latin
<!-- enum Script::variant Malayalam -->
Malayalam
<!-- enum Script::variant Mongolian -->
Mongolian
<!-- enum Script::variant Myanmar -->
Myanmar
<!-- enum Script::variant Ogham -->
Ogham
<!-- enum Script::variant OldItalic -->
Old Italic
<!-- enum Script::variant Oriya -->
Oriya
<!-- enum Script::variant Runic -->
Runic
<!-- enum Script::variant Sinhala -->
Sinhala
<!-- enum Script::variant Syriac -->
Syriac
<!-- enum Script::variant Tamil -->
Tamil
<!-- enum Script::variant Telugu -->
Telugu
<!-- enum Script::variant Thaana -->
Thaana
<!-- enum Script::variant Thai -->
Thai
<!-- enum Script::variant Tibetan -->
Tibetan
<!-- enum Script::variant CanadianAboriginal -->
Canadian Aboriginal
<!-- enum Script::variant Yi -->
Yi
<!-- enum Script::variant Tagalog -->
Tagalog
<!-- enum Script::variant Hanunoo -->
Hanunoo
<!-- enum Script::variant Buhid -->
Buhid
<!-- enum Script::variant Tagbanwa -->
Tagbanwa
<!-- enum Script::variant Braille -->
Braille
<!-- enum Script::variant Cypriot -->
Cypriot
<!-- enum Script::variant Limbu -->
Limbu
<!-- enum Script::variant Osmanya -->
Osmanya
<!-- enum Script::variant Shavian -->
Shavian
<!-- enum Script::variant LinearB -->
Linear B
<!-- enum Script::variant TaiLe -->
Tai Le
<!-- enum Script::variant Ugaritic -->
Ugaritic
<!-- enum Script::variant NewTaiLue -->
New Tai Lue. Since 1.10
<!-- enum Script::variant Buginese -->
Buginese. Since 1.10
<!-- enum Script::variant Glagolitic -->
Glagolitic. Since 1.10
<!-- enum Script::variant Tifinagh -->
Tifinagh. Since 1.10
<!-- enum Script::variant SylotiNagri -->
Syloti Nagri. Since 1.10
<!-- enum Script::variant OldPersian -->
Old Persian. Since 1.10
<!-- enum Script::variant Kharoshthi -->
Kharoshthi. Since 1.10
<!-- enum Script::variant Unknown -->
an unassigned code point. Since 1.14
<!-- enum Script::variant Balinese -->
Balinese. Since 1.14
<!-- enum Script::variant Cuneiform -->
Cuneiform. Since 1.14
<!-- enum Script::variant Phoenician -->
Phoenician. Since 1.14
<!-- enum Script::variant PhagsPa -->
Phags-pa. Since 1.14
<!-- enum Script::variant Nko -->
N'Ko. Since 1.14
<!-- enum Script::variant KayahLi -->
Kayah Li. Since 1.20.1
<!-- enum Script::variant Lepcha -->
Lepcha. Since 1.20.1
<!-- enum Script::variant Rejang -->
Rejang. Since 1.20.1
<!-- enum Script::variant Sundanese -->
Sundanese. Since 1.20.1
<!-- enum Script::variant Saurashtra -->
Saurashtra. Since 1.20.1
<!-- enum Script::variant Cham -->
Cham. Since 1.20.1
<!-- enum Script::variant OlChiki -->
Ol Chiki. Since 1.20.1
<!-- enum Script::variant Vai -->
Vai. Since 1.20.1
<!-- enum Script::variant Carian -->
Carian. Since 1.20.1
<!-- enum Script::variant Lycian -->
Lycian. Since 1.20.1
<!-- enum Script::variant Lydian -->
Lydian. Since 1.20.1
<!-- enum Script::variant Batak -->
Batak. Since 1.32
<!-- enum Script::variant Brahmi -->
Brahmi. Since 1.32
<!-- enum Script::variant Mandaic -->
Mandaic. Since 1.32
<!-- enum Script::variant Chakma -->
Chakma. Since: 1.32
<!-- enum Script::variant MeroiticCursive -->
Meroitic Cursive. Since: 1.32
<!-- enum Script::variant MeroiticHieroglyphs -->
Meroitic Hieroglyphs. Since: 1.32
<!-- enum Script::variant Miao -->
Miao. Since: 1.32
<!-- enum Script::variant Sharada -->
Sharada. Since: 1.32
<!-- enum Script::variant SoraSompeng -->
Sora Sompeng. Since: 1.32
<!-- enum Script::variant Takri -->
Takri. Since: 1.32
<!-- enum Script::variant BassaVah -->
Bassa. Since: 1.40
<!-- enum Script::variant CaucasianAlbanian -->
Caucasian Albanian. Since: 1.40
<!-- enum Script::variant Duployan -->
Duployan. Since: 1.40
<!-- enum Script::variant Elbasan -->
Elbasan. Since: 1.40
<!-- enum Script::variant Grantha -->
Grantha. Since: 1.40
<!-- enum Script::variant Khojki -->
Kjohki. Since: 1.40
<!-- enum Script::variant Khudawadi -->
Khudawadi, Sindhi. Since: 1.40
<!-- enum Script::variant LinearA -->
Linear A. Since: 1.40
<!-- enum Script::variant Mahajani -->
Mahajani. Since: 1.40
<!-- enum Script::variant Manichaean -->
Manichaean. Since: 1.40
<!-- enum Script::variant MendeKikakui -->
Mende Kikakui. Since: 1.40
<!-- enum Script::variant Modi -->
Modi. Since: 1.40
<!-- enum Script::variant Mro -->
Mro. Since: 1.40
<!-- enum Script::variant Nabataean -->
Nabataean. Since: 1.40
<!-- enum Script::variant OldNorthArabian -->
Old North Arabian. Since: 1.40
<!-- enum Script::variant OldPermic -->
Old Permic. Since: 1.40
<!-- enum Script::variant PahawhHmong -->
Pahawh Hmong. Since: 1.40
<!-- enum Script::variant Palmyrene -->
Palmyrene. Since: 1.40
<!-- enum Script::variant PauCinHau -->
Pau Cin Hau. Since: 1.40
<!-- enum Script::variant PsalterPahlavi -->
Psalter Pahlavi. Since: 1.40
<!-- enum Script::variant Siddham -->
Siddham. Since: 1.40
<!-- enum Script::variant Tirhuta -->
Tirhuta. Since: 1.40
<!-- enum Script::variant WarangCiti -->
Warang Citi. Since: 1.40
<!-- enum Script::variant Ahom -->
Ahom. Since: 1.40
<!-- enum Script::variant AnatolianHieroglyphs -->
Anatolian Hieroglyphs. Since: 1.40
<!-- enum Script::variant Hatran -->
Hatran. Since: 1.40
<!-- enum Script::variant Multani -->
Multani. Since: 1.40
<!-- enum Script::variant OldHungarian -->
Old Hungarian. Since: 1.40
<!-- enum Script::variant Signwriting -->
Signwriting. Since: 1.40
<!-- enum Stretch -->
An enumeration specifying the width of the font relative to other designs
within a family.
<!-- enum Stretch::variant UltraCondensed -->
ultra condensed width
<!-- enum Stretch::variant ExtraCondensed -->
extra condensed width
<!-- enum Stretch::variant Condensed -->
condensed width
<!-- enum Stretch::variant SemiCondensed -->
semi condensed width
<!-- enum Stretch::variant Normal -->
the normal width
<!-- enum Stretch::variant SemiExpanded -->
semi expanded width
<!-- enum Stretch::variant Expanded -->
expanded width
<!-- enum Stretch::variant ExtraExpanded -->
extra expanded width
<!-- enum Stretch::variant UltraExpanded -->
ultra expanded width
<!-- enum Style -->
An enumeration specifying the various slant styles possible for a font.
<!-- enum Style::variant Normal -->
the font is upright.
<!-- enum Style::variant Oblique -->
the font is slanted, but in a roman style.
<!-- enum Style::variant Italic -->
the font is slanted in an italic style.
<!-- enum TabAlign -->
A `TabAlign` specifies where a tab stop appears relative to the text.
<!-- enum TabAlign::variant Left -->
the tab stop appears to the left of the text.
<!-- struct TabArray -->
A `TabArray` struct contains an array
of tab stops. Each tab stop has an alignment and a position.
<!-- impl TabArray::fn new -->
Creates an array of `initial_size` tab stops. Tab stops are specified in
pixel units if `positions_in_pixels` is `true`, otherwise in Pango
units. All stops are initially at position 0.
## `initial_size`
Initial number of tab stops to allocate, can be 0
## `positions_in_pixels`
whether positions are in pixel units

# Returns

the newly allocated `TabArray`, which should
 be freed with `TabArray::free`.
<!-- impl TabArray::fn new_with_positions -->
This is a convenience function that creates a `TabArray`
and allows you to specify the alignment and position of each
tab stop. You `<emphasis>`must`</emphasis>` provide an alignment
and position for `size` tab stops.
## `size`
number of tab stops in the array
## `positions_in_pixels`
whether positions are in pixel units
## `first_alignment`
alignment of first tab stop
## `first_position`
position of first tab stop

# Returns

the newly allocated `TabArray`, which should
 be freed with `TabArray::free`.
<!-- impl TabArray::fn copy -->
Copies a `TabArray`

# Returns

the newly allocated `TabArray`, which should
 be freed with `TabArray::free`.
<!-- impl TabArray::fn free -->
Frees a tab array and associated resources.
<!-- impl TabArray::fn get_positions_in_pixels -->
Returns `true` if the tab positions are in pixels, `false` if they are
in Pango units.

# Returns

whether positions are in pixels.
<!-- impl TabArray::fn get_size -->
Gets the number of tab stops in `self`.

# Returns

the number of tab stops in the array.
<!-- impl TabArray::fn get_tab -->
Gets the alignment and position of a tab stop.
## `tab_index`
tab stop index
## `alignment`
location to store alignment, or `None`
## `location`
location to store tab position, or `None`
<!-- impl TabArray::fn get_tabs -->
If non-`None`, `alignments` and `locations` are filled with allocated
arrays of length `TabArray::get_size`. You must free the
returned array.
## `alignments`
location to store an array of tab
 stop alignments, or `None`
## `locations`
location to store an array
 of tab positions, or `None`
<!-- impl TabArray::fn resize -->
Resizes a tab array. You must subsequently initialize any tabs that
were added as a result of growing the array.
## `new_size`
new size of the array
<!-- impl TabArray::fn set_tab -->
Sets the alignment and location of a tab stop.
`alignment` must always be `TabAlign::Left` in the current
implementation.
## `tab_index`
the index of a tab stop
## `alignment`
tab alignment
## `location`
tab location in Pango units
<!-- enum Underline -->
The `Underline` enumeration is used to specify
whether text should be underlined, and if so, the type
of underlining.
<!-- enum Underline::variant None -->
no underline should be drawn
<!-- enum Underline::variant Single -->
a single underline should be drawn
<!-- enum Underline::variant Double -->
a double underline should be drawn
<!-- enum Underline::variant Low -->
a single underline should be drawn at a position
beneath the ink extents of the text being
underlined. This should be used only for underlining
single characters, such as for keyboard
accelerators. `Underline::Single` should
be used for extended portions of text.
<!-- enum Underline::variant Error -->
a wavy underline should be drawn below.
This underline is typically used to indicate
an error such as a possilble mispelling; in some
cases a contrasting color may automatically
be used. This type of underlining is available since Pango 1.4.
<!-- enum Variant -->
An enumeration specifying capitalization variant of the font.
<!-- enum Variant::variant Normal -->
A normal font.
<!-- enum Variant::variant SmallCaps -->
A font with the lower case characters
replaced by smaller variants of the capital characters.
<!-- enum Weight -->
An enumeration specifying the weight (boldness) of a font. This is a numerical
value ranging from 100 to 1000, but there are some predefined values:
<!-- enum Weight::variant Thin -->
the thin weight (= 100; Since: 1.24)
<!-- enum Weight::variant Ultralight -->
the ultralight weight (= 200)
<!-- enum Weight::variant Light -->
the light weight (= 300)
<!-- enum Weight::variant Semilight -->
the semilight weight (= 350; Since: 1.36.7)
<!-- enum Weight::variant Book -->
the book weight (= 380; Since: 1.24)
<!-- enum Weight::variant Normal -->
the default weight (= 400)
<!-- enum Weight::variant Medium -->
the normal weight (= 500; Since: 1.24)
<!-- enum Weight::variant Semibold -->
the semibold weight (= 600)
<!-- enum Weight::variant Bold -->
the bold weight (= 700)
<!-- enum Weight::variant Ultrabold -->
the ultrabold weight (= 800)
<!-- enum Weight::variant Heavy -->
the heavy weight (= 900)
<!-- enum Weight::variant Ultraheavy -->
the ultraheavy weight (= 1000; Since: 1.24)
<!-- enum WrapMode -->
A `WrapMode` describes how to wrap the lines of a `Layout` to the desired width.
<!-- enum WrapMode::variant Word -->
wrap lines at word boundaries.
<!-- enum WrapMode::variant Char -->
wrap lines at character boundaries.
<!-- enum WrapMode::variant WordChar -->
wrap lines at word boundaries, but fall back to character boundaries if there is not
enough space for a full word.
