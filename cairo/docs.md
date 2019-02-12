<!-- file enums.rs -->
<!-- enum Status -->
Status is used to indicate errors that can occur when using Cairo. In some cases it is
returned directly by functions, but when using `Context`, the last error, if any, is
stored in the context and can be retrieved with `Context::status()`.

New entries may be added in future versions. Use `Context::status_to_string()` to get a
human-readable representation of an error message.
<!-- enum Status::variant Success -->
no error has occurred (Since 1.0)
<!-- enum Status::variant NoMemory, -->
out of memory (Since 1.0)
<!-- enum Status::variant InvalidRestore, -->
`Context::restore()` called without matching `Context::save()` (Since 1.0)
<!-- enum Status::variant InvalidPopGroup, -->
no saved group to pop, i.e. `Context::pop_group()` without matching
`Context::push_group()` (Since 1.0)
<!-- enum Status::variant NoCurrentPoint, -->
no current point defined (Since 1.0)
<!-- enum Status::variant InvalidMatrix, -->
invalid matrix (not invertible) (Since 1.0)
<!-- enum Status::variant InvalidStatus, -->
invalid value for an input `Status` (Since 1.0)
<!-- enum Status::variant NullPointer, -->
NULL pointer (Since 1.0)
<!-- enum Status::variant InvalidString, -->
input string not valid UTF-8 (Since 1.0)
<!-- enum Status::variant InvalidPathData, -->
input path data not valid (Since 1.0)
<!-- enum Status::variant ReadError, -->
error while reading from input stream (Since 1.0)
<!-- enum Status::variant WriteError, -->
error while writing to output stream (Since 1.0)
<!-- enum Status::variant SurfaceFinished, -->
target surface has been finished (Since 1.0)
<!-- enum Status::variant SurfaceTypeMismatch, -->
the surface type is not appropriate for the operation (Since 1.0)
<!-- enum Status::variant PatternTypeMismatch, -->
the pattern type is not appropriate for the operation (Since 1.0)
<!-- enum Status::variant InvalidContent, -->
invalid value for an input `Content` (Since 1.0)
<!-- enum Status::variant InvalidFormat, -->
invalid value for an input `Format` (Since 1.0)
<!-- enum Status::variant InvalidVisual, -->
invalid value for an input Visual* (Since 1.0)
<!-- enum Status::variant FileNotFound, -->
file not found (Since 1.0)
<!-- enum Status::variant InvalidDash, -->
invalid value for a dash setting (Since 1.0)
<!-- enum Status::variant InvalidDscComment, -->
invalid value for a DSC comment (Since 1.2)
<!-- enum Status::variant InvalidIndex, -->
invalid index passed to getter (Since 1.4)
<!-- enum Status::variant ClipNotRepresentable, -->
clip region not representable in desired format (Since 1.4)
<!-- enum Status::variant TempFileError, -->
error creating or writing to a temporary file (Since 1.6)
<!-- enum Status::variant InvalidStride, -->
invalid value for stride (Since 1.6)
<!-- enum Status::variant FontTypeMismatch, -->
the font type is not appropriate for the operation (Since 1.8)
<!-- enum Status::variant UserFontImmutable, -->
the user-font is immutable (Since 1.8)
<!-- enum Status::variant UserFontError, -->
error occurred in a user-font callback function (Since 1.8)
<!-- enum Status::variant NegativeCount, -->
negative number used where it is not allowed (Since 1.8)
<!-- enum Status::variant InvalidClusters, -->
input clusters do not represent the accompanying text and glyph array (Since 1.8)
<!-- enum Status::variant InvalidSlant, -->
invalid value for an input `FontSlant` (Since 1.8)
<!-- enum Status::variant InvalidWeight, -->
invalid value for an input `FontWeight` (Since 1.8)
<!-- enum Status::variant InvalidSize, -->
invalid value (typically too big) for the size of the input (surface, pattern,
etc.) (Since 1.10)
<!-- enum Status::variant UserFontNotImplemented, -->
user-font method not implemented (Since 1.10)
<!-- enum Status::variant DeviceTypeMismatch, -->
the device type is not appropriate for the operation (Since 1.10)
<!-- enum Status::variant DeviceError, -->
an operation to the device caused an unspecified error (Since 1.10)
<!-- enum Status::variant InvalidMeshConstruction, -->
a mesh pattern construction operation was used outside of a
`Context::mesh_pattern_begin_patch()`/`Context::mesh_pattern_end_patch()`
pair (Since 1.12)
<!-- enum Status::variant DeviceFinished, -->
target device has been finished (Since 1.12)
<!-- enum Status::variant LastStatus -->
this is a special value indicating the number of status values defined in this
enumeration. When using this value, note that the version of cairo at run-time
may have additional status values defined than the value of this symbol at
compile-time. (Since 1.10)
<!-- enum Antialias -->
Specifies the type of antialiasing to do when rendering text or shapes.

As it is not necessarily clear from the above what advantages a particular antialias method
provides, since 1.12, there is also a set of hints:
`Fast`: Allow the backend to degrade raster quality for speed.
`Good`: A balance between speed and quality.
`Best`: A high-fidelity, but potentially slow, raster mode.

These make no guarantee on how the backend will perform its rasterisation (if it even
rasterises!), nor that they have any differing effect other than to enable some form of
antialiasing. In the case of glyph rendering, `Fast` and `Good` will be mapped to `Gray`, with
`Best` being equivalent to `Subpixel`.

The interpretation of `Default` is left entirely up to the backend, typically this will be
similar to `Good`.
<!-- enum Antialias::variant Default, -->
Use the default antialiasing for the subsystem and target device, since 1.0
<!-- enum Antialias::variant None, -->
Use a bilevel alpha mask, since 1.0
<!-- enum Antialias::variant Gray, -->
Perform single-color antialiasing (using shades of gray for black text on a white
background, for example), since 1.0
<!-- enum Antialias::variant Subpixel, -->
Perform antialiasing by taking advantage of the order of subpixel elements on devices
such as LCD panels, since 1.0
<!-- enum Antialias::variant Fast, -->
Hint that the backend should perform some antialiasing but prefer speed over quality,
since 1.12
<!-- enum Antialias::variant Good, -->
The backend should balance quality against performance, since 1.12
<!-- enum Antialias::variant Best -->
Hint that the backend should render at the highest quality, sacrificing speed if
necessary, since 1.12
<!-- enum FillRule -->
`FillRule` is used to select how paths are filled. For both fill rules, whether or not
a point is included in the fill is determined by taking a ray from that point to infinity
and looking at intersections with the path. The ray can be in any direction, as long as
it doesn't pass through the end point of a segment or have a tricky intersection such as
intersecting tangent to the path. (Note that filling is not actually implemented in this
way. This is just a description of the rule that is applied.)

The default fill rule is `Winding`.

New entries may be added in future versions.
<!-- enum FillRule::variant Winding, -->
If the path crosses the ray from left-to-right, counts +1. If the path crosses the ray
from right to left, counts -1. (Left and right are determined from the perspective of
looking along the ray from the starting point.) If the total count is non-zero, the point
will be filled. (Since 1.0)
<!-- enum FillRule::variant EvenOdd -->
Counts the total number of intersections, without regard to the orientation of the contour.
If the total number of intersections is odd, the point will be filled. (Since 1.0)
<!-- enum LineCap -->
Specifies how to render the endpoints of the path when stroking.

The default line cap style is `Butt`.
<!-- enum LineCap::variant Butt, -->
start(stop) the line exactly at the start(end) point (Since 1.0)
<!-- enum LineCap::variant Round, -->
use a round ending, the center of the circle is the end point (Since 1.0)
<!-- enum LineCap::variant Square -->
use squared ending, the center of the square is the end point (Since 1.0)
<!-- enum LineJoin -->
Specifies how to render the junction of two lines when stroking.

The default line join style is `Miter`.
<!-- enum LineJoin::variant Miter, -->
use a sharp (angled) corner, see `Context::set_miter_limit()` (Since 1.0)
<!-- enum LineJoin::variant Round, -->
use a rounded join, the center of the circle is the joint point (Since 1.0)
<!-- enum LineJoin::variant nBevel -->
use a cut-off join, the join is cut off at half the line width from the
joint point (Since 1.0)
<!-- enum Operator -->
`Operator` is used to set the compositing operator for all cairo drawing operations.

The default operator is `Over`.

The operators marked as unbounded modify their destination even outside of the mask layer
(that is, their effect is not bound by the mask layer). However, their effect can still be
limited by way of clipping.

To keep things simple, the operator descriptions here document the behavior for when both
source and destination are either fully transparent or fully opaque. The actual implementation
works for translucent layers too. For a more detailed explanation of the effects of each
operator, including the mathematical definitions, see http:cairographics.org/operators/.
<!-- enum Operator::variant Clear, -->
clear destination layer (bounded) (Since 1.0)
<!-- enum Operator::variant Source, -->
replace destination layer (bounded) (Since 1.0)
<!-- enum Operator::variant Over, -->
draw source layer on top of destination layer (bounded) (Since 1.0)
<!-- enum Operator::variant In, -->
draw source where there was destination content (unbounded) (Since 1.0)
<!-- enum Operator::variant Out, -->
draw source where there was no destination content (unbounded) (Since 1.0)
<!-- enum Operator::variant Atop, -->
draw source on top of destination content and only there (Since 1.0)
<!-- enum Operator::variant Dest, -->
ignore the source (Since 1.0)
<!-- enum Operator::variant DestOver, -->
draw destination on top of source (Since 1.0)
<!-- enum Operator::variant DestIn, -->
leave destination only where there was source content (unbounded) (Since 1.0)
<!-- enum Operator::variant DestOut, -->
leave destination only where there was no source content (Since 1.0)
<!-- enum Operator::variant DestAtop, -->
leave destination on top of source content and only there (unbounded) (Since 1.0)
<!-- enum Operator::variant Xor, -->
source and destination are shown where there is only one of them (Since 1.0)
<!-- enum Operator::variant Add, -->
source and destination layers are accumulated (Since 1.0)
<!-- enum Operator::variant Saturate, -->
like over, but assuming source and dest are disjoint geometries (Since 1.0)
<!-- enum Operator::variant Multiply, -->
source and destination layers are multiplied. This causes the result to be at
least as dark as the darker inputs. (Since 1.10)
<!-- enum Operator::variant Screen, -->
source and destination are complemented and multiplied. This causes the result
to be at least as light as the lighter inputs. (Since 1.10)
<!-- enum Operator::variant Overlay, -->
multiplies or screens, depending on the lightness of the destination color.
(Since 1.10)
<!-- enum Operator::variant Darken, -->
replaces the destination with the source if it is darker, otherwise keeps the
source. (Since 1.10)
<!-- enum Operator::variant Lighten, -->
replaces the destination with the source if it is lighter, otherwise keeps the
source. (Since 1.10)
<!-- enum Operator::variant ColorDodge, -->
brightens the destination color to reflect the source color. (Since 1.10)
<!-- enum Operator::variant ColorBurn, -->
darkens the destination color to reflect the source color. (Since 1.10)
<!-- enum Operator::variant HardLight, -->
Multiplies or screens, dependent on source color. (Since 1.10)
<!-- enum Operator::variant SoftLight, -->
Darkens or lightens, dependent on source color. (Since 1.10)
<!-- enum Operator::variant Difference, -->
Takes the difference of the source and destination color. (Since 1.10)
<!-- enum Operator::variant Exclusion, -->
Produces an effect similar to difference, but with lower contrast. (Since 1.10)
<!-- enum Operator::variant HslHue, -->
Creates a color with the hue of the source and the saturation and luminosity of
the target. (Since 1.10)
<!-- enum Operator::variant HslSaturation, -->
Creates a color with the saturation of the source and the hue and luminosity of
the target. Painting with this mode onto a gray area produces no change. (Since 1.10)
<!-- enum Operator::variant HslColor, -->
Creates a color with the hue and saturation of the source and the luminosity of the
target. This preserves the gray levels of the target and is useful for coloring
monochrome images or tinting color images. (Since 1.10)
<!-- enum Operator::variant HslLuminosity -->
Creates a color with the luminosity of the source and the hue and saturation of the
target. This produces an inverse effect to `HslColor`. (Since 1.10)
<!-- enum PathDataType -->
`PathData` is used to describe the type of one portion of a path when represented as a `Path`.
See `PathData` for details.
<!-- enum PathDataType::variant MoveTo, -->
A move-to operation, since 1.0
<!-- enum PathDataType::variant LineTo, -->
A line-to operation, since 1.0
<!-- enum PathDataType::variant CurveTo, -->
A curve-to operation, since 1.0
<!-- enum PathDataType::variant ClosePath -->
A close-path operation, since 1.0
<!-- enum Content -->
`Content` is used to describe the content that a surface will contain, whether color
information, alpha information (translucence vs. opacity), or both.

Note: The large values here are designed to keep `Content` values distinct from `Format`
values so that the implementation can detect the error if users confuse the two types.
<!-- enum Content::variant Color -->
The surface will hold color content only. (Since 1.0)
<!-- enum Content::variant Alpha -->
The surface will hold alpha content only. (Since 1.0)
<!-- enum Content::variant ColorAlpha -->
The surface will hold color and alpha content. (Since 1.0)
<!-- enum Extend -->
`Extend` is used to describe how pattern color/alpha will be determined for areas
"outside" the pattern's natural area, (for example, outside the surface bounds or
outside the gradient geometry).

Mesh patterns are not affected by the extend mode.

The default extend mode is `None` for surface patterns and `Pad` for gradient
patterns.

New entries may be added in future versions.
<!-- enum Extend::variant None, -->
pixels outside of the source pattern are fully transparent (Since 1.0)
<!-- enum Extend::variant Repeat, -->
the pattern is tiled by repeating (Since 1.0)
<!-- enum Extend::variant Reflect, -->
the pattern is tiled by reflecting at the edges (Since 1.0; but only implemented
for surface patterns since 1.6)
<!-- enum Extend::variant Pad -->
pixels outside of the pattern copy the closest pixel from the source (Since 1.2;
but only implemented for surface patterns since 1.6)
<!-- enum Filter -->
`Filter` is used to indicate what filtering should be applied when reading pixel values
from patterns. See `Pattern::set_filter()` for indicating the desired filter to be used
with a particular pattern.
<!-- enum Filter::variant Fast, -->
A high-performance filter, with quality similar to `Nearest` (Since 1.0)
<!-- enum Filter::variant Good, -->
A reasonable-performance filter, with quality similar to `Bilinear` (Since 1.0)
<!-- enum Filter::variant Best, -->
The highest-quality available, performance may not be suitable for interactive
use. (Since 1.0)
<!-- enum Filter::variant Nearest, -->
Nearest-neighbor filtering (Since 1.0)
<!-- enum Filter::variant Bilinear, -->
Linear interpolation in two dimensions (Since 1.0)
<!-- enum Filter::variant Gaussian -->
This filter value is currently unimplemented, and should not be used in current
code. (Since 1.0)
<!-- enum PatternType -->
`PatternType` is used to describe the type of a given pattern.

The type of a pattern is determined by the function used to create it. The
`Pattern::create_rgb()` and `Pattern::create_rgba()` functions create `Solid` patterns.
The remaining cairo_pattern_create functions map to pattern types in obvious ways.

The pattern type can be queried with `Pattern::get_type()`.

Most `Pattern` functions can be called with a pattern of any type, (though trying to
change the extend or filter for a solid pattern will have no effect). A notable exception
is `Pattern::add_color_stop_rgb()` and `Pattern::add_color_stop_rgba()` which must only be
called with gradient patterns (either `Linear` or `Radial`). Otherwise the pattern will be
shutdown and put into an error state.

New entries may be added in future versions.
<!-- enum PatternType::variant Solid, -->
The pattern is a solid (uniform) color. It may be opaque or translucent, since 1.2.
<!-- enum PatternType::variant Surface, -->
The pattern is a based on a surface (an image), since 1.2.
<!-- enum PatternType::variant LinearGradient, -->
The pattern is a linear gradient, since 1.2.
<!-- enum PatternType::variant RadialGradient, -->
The pattern is a radial gradient, since 1.2.
<!-- enum PatternType::variant Mesh, -->
The pattern is a mesh, since 1.12.
<!-- enum PatternType::variant RasterSource -->
The pattern is a user pattern providing raster data, since 1.12.
<!-- enum FontSlant -->
Specifies variants of a font face based on their slant.
<!-- enum FontSlant::variant Normal, -->
Upright font style, since 1.0
<!-- enum FontSlant::variant Italic, -->
Italic font style, since 1.0
<!-- enum FontSlant::variant Oblique -->
Oblique font style, since 1.0
<!-- enum FontWeight -->
Specifies variants of a font face based on their weight.
<!-- enum FontWeight::variant Normal, -->
Normal font weight, since 1.0
<!-- enum FontWeight::variant Bold -->
Bold font weight, since 1.0
<!-- enum TextClusterFlags -->
Specifies properties of a text cluster mapping.
<!-- enum TextClusterFlags::variant Backward -->
The clusters in the cluster array map to glyphs in the glyph array from end
to start. (Since 1.8)
<!-- enum FontType -->
`FontType` is used to describe the type of a given font face or scaled font. The
font types are also known as "font backends" within cairo.

The type of a font face is determined by the function used to create it, which will
generally be of the form `Context::type_font_face_create()`. The font face type can
be queried with `Context::font_face_get_type()`.

The various cairo_font_face_t functions can be used with a font face of any type.

The type of a scaled font is determined by the type of the font face passed to
`Context::scaled_font_create()`. The scaled font type can be queried with
`Context::scaled_font_get_type()`.

The various `ScaledFont` functions can be used with scaled fonts of any type, but some
font backends also provide type-specific functions that must only be called with a
scaled font of the appropriate type. These functions have names that begin with
`Context::type_scaled_font()` such as `Context::ft_scaled_font_lock_face()`.

The behavior of calling a type-specific function with a scaled font of the wrong type
is undefined.

New entries may be added in future versions.
<!-- enum FontType::variant FontTypeToy, -->
The font was created using cairo's toy font api (Feature: `v1_2`)
<!-- enum FontType::variant FontTypeFt, -->
The font is of type FreeType (Feature: `v1_2`)
<!-- enum FontType::variant FontTypeWin32, -->
The font is of type Win32 (Feature: `v1_2`)
<!-- enum FontType::variant FontTypeQuartz, -->
The font is of type Quartz (Feature: `v1_6`, in 1.2 and 1.4 it was named
CAIRO_FONT_TYPE_ATSUI)
<!-- enum FontType::variant FontTypeUser -->
The font was create using cairo's user font api (Feature: `v1_8`)
<!-- enum SubpixelOrder -->
The subpixel order specifies the order of color elements within each pixel on the display
device when rendering with an antialiasing mode of `Antialias::Subpixel`.
<!-- enum SubpixelOrder::variant Default, -->
Use the default subpixel order for for the target device, since 1.0
<!-- enum SubpixelOrder::variant Rgb, -->
Subpixel elements are arranged horizontally with red at the left, since 1.0
<!-- enum SubpixelOrder::variant Bgr, -->
Subpixel elements are arranged horizontally with blue at the left, since 1.0
<!-- enum SubpixelOrder::variant Vrgb, -->
Subpixel elements are arranged vertically with red at the top, since 1.0
<!-- enum SubpixelOrder::variant Vbgr -->
Subpixel elements are arranged vertically with blue at the top, since 1.0
<!-- enum HintStyle -->
Specifies the type of hinting to do on font outlines. Hinting is the process of fitting
outlines to the pixel grid in order to improve the appearance of the result. Since hinting
outlines involves distorting them, it also reduces the faithfulness to the original outline
shapes. Not all of the outline hinting styles are supported by all font backends.

New entries may be added in future versions.
<!-- enum HintStyle::variant Default, -->
Use the default hint style for font backend and target device, since 1.0
<!-- enum HintStyle::variant None, -->
Do not hint outlines, since 1.0
<!-- enum HintStyle::variant Slight, -->
Hint outlines slightly to improve contrast while retaining good fidelity to the
original shapes, since 1.0
<!-- enum HintStyle::variant Medium, -->
Hint outlines with medium strength giving a compromise between fidelity to the
original shapes and contrast, since 1.0
<!-- enum HintStyle::variant Full -->
Hint outlines to maximize contrast, since 1.0
<!-- enum HintMetrics -->
Specifies whether to hint font metrics; hinting font metrics means quantizing them so
that they are integer values in device space. Doing this improves the consistency of
letter and line spacing, however it also means that text will be laid out differently
at different zoom factors
<!-- enum HintMetrics::variant Default, -->
Hint metrics in the default manner for the font backend and target device, since 1.0
<!-- enum HintMetrics::variant Off, -->
Do not hint font metrics, since 1.0
<!-- enum HintMetrics::variant On -->
Hint font metrics, since 1.0
<!-- file context.rs -->
<!-- file_comment -->
The cairo drawing context
<!-- impl Context::fn new -->
Creates a new Context with all graphics state parameters set to default values
and with target as a target surface. The target surface should be constructed
with a backend-specific function such as cairo_image_surface_create() (or any other
Context::backend_surface_create() variant).

This function references target , so you can immediately call Surface::drop() on
it if you don't need to maintain a separate reference to it.
<!-- impl Context::fn status -->
Checks whether an error has previously occurred for this context.
<!-- impl Context::fn save -->
Makes a copy of the current state of self and saves it on an internal stack of
saved states for self. When Context::restore() is called, self will be restored to
the saved state. Multiple calls to Context::save() and Context::restore() can be nested;
each call to Context::restore() restores the state from the matching paired
Context::save().

It isn't necessary to clear all saved states before a Context is freed. If the
reference count of a Context drops to zero in response to a call to Context::drop(), any
saved states will be freed along with the Context.
<!-- impl Context::fn restore -->
Restores self to the state saved by a preceding call to Context::save() and removes
that state from the stack of saved states.
<!-- impl Context::fn push_group -->
Temporarily redirects drawing to an intermediate surface known as a group. The
redirection lasts until the group is completed by a call to Context::pop_group()
or Context::pop_group_to_source(). These calls provide the result of any drawing
to the group as a pattern, (either as an explicit object, or set as the source
pattern).

This group functionality can be convenient for performing intermediate compositing.
One common use of a group is to render objects as opaque within the group, (so that
they occlude each other), and then blend the result with translucence onto the
destination.

Groups can be nested arbitrarily deep by making balanced calls to Context::push_group()
/ Context::pop_group(). Each call pushes  /pops the new target group onto/from a stack.

The Context::push_group() function calls Context::save() so that any changes to the
graphics state will not be visible outside the group, (the pop_group functions call
Context::restore()).

By default the intermediate group will have a content type of Content::ColorAlpha.
Other content types can be chosen for the group by using
Context::push_group_with_content() instead.

As an example, here is how one might fill and stroke a path with translucence, but
without any portion of the fill being visible under the stroke:

```ignore
cr.push_group();
cr.set_source (fill_pattern);
cr.fill_preserve();
cr.set_source();
cr.stroke();
cr.pop_group_to_source();
cr.paint_with_alpha(alpha);
```
<!-- impl Context::fn pop_group -->
Terminates the redirection begun by a call to Context::push_group() or
Context::push_group_with_content() and returns a new pattern containing the results
of all drawing operations performed to the group.

The Context::pop_group() function calls Context::restore(), (balancing a call to
Context::save() by the push_group function), so that any changes to the graphics
state will not be visible outside the group.
<!-- impl Context::fn pop_group_to_source -->
Terminates the redirection begun by a call to Context::push_group() or
Context::push_group_with_content() and installs the resulting pattern as the
source pattern in the given cairo context.

The behavior of this function is equivalent to the sequence of operations:

```ignore
let mut group = context.pop_group();
context.set_source(group);
```

but is more convenient as their is no need for a variable to store the short-lived
pointer to the pattern.

The Context::pop_group() function calls Context::restore(), (balancing a call to
Context::save() by the push_group function), so that any changes to the graphics state
will not be visible outside the group.
<!-- impl Context::fn set_source_rgb -->
Sets the source pattern within self to an opaque color. This opaque color will then be
used for any subsequent drawing operation until a new source pattern is set.

The color components are floating point numbers in the range 0 to 1.

If the values passed in are outside that range, they will be clamped.

The default source pattern is opaque black, (that is, it is equivalent to
Context::set_source_rgb(0.0, 0.0, 0.0)).
<!-- impl Context::fn set_source_rgba -->
Sets the source pattern within self to a translucent color. This color will then be used
for any subsequent drawing operation until a new source pattern is set.

The color and alpha components are floating point numbers in the range 0 to 1. If the
values passed in are outside that range, they will be clamped.

The default source pattern is opaque black, (that is, it is equivalent to
Context::set_source_rgba(0.0, 0.0, 0.0, 1.0)).
<!-- impl Context::fn set_source -->
Sets the source pattern within self to source. This pattern will then be used for any
subsequent drawing operation until a new source pattern is set.

Note: The pattern's transformation matrix will be locked to the user space in effect
at the time of Context::set_source(). This means that further modifications of the
current transformation matrix will not affect the source pattern. See
Pattern::set_matrix().

The default source pattern is a solid pattern that is opaque black, (that is, it is
equivalent to Context::set_source_rgb(0.0, 0.0, 0.0)).
<!-- impl Context::fn get_source -->
Gets the current source pattern for self.
<!-- impl Context::fn set_antialias -->
Set the antialiasing mode of the rasterizer used for drawing shapes. This value
is a hint, and a particular backend may or may not support a particular value.
At the current time, no backend supports CAIRO_ANTIALIAS_SUBPIXEL when drawing
shapes.

Note that this option does not affect text rendering, instead see
FontOptions::set_antialias().
<!-- impl Context::fn get_antialias -->
Gets the current shape antialiasing mode, as set by Context::set_antialias().
<!-- impl Context::fn set_dash -->
Sets the dash pattern to be used by cairo_stroke(). A dash pattern is specified by dashes,
an array of positive values. Each value provides the length of alternate "on" and "off"
portions of the stroke. The offset specifies an offset into the pattern at which the
stroke begins.

Each "on" segment will have caps applied as if the segment were a separate sub-path. In
particular, it is valid to use an "on" length of 0.0 with Line::CapRound or Line::CapSquare
in order to distributed dots or squares along a path.

Note: The length values are in user-space units as evaluated at the time of stroking.
This is not necessarily the same as the user space at the time of Context::set_dash().

If num_dashes is 0 dashing is disabled.

If num_dashes is 1 a symmetric pattern is assumed with alternating on and off portions
of the size specified by the single value in dashes .

If any value in dashes is negative, or if all values are 0, then self will be put into
an error state with a status of Status::InvalidDash.
<!-- impl Context::fn get_dash_count -->
This function returns the length of the dash array in self (0 if dashing is not
currently in effect).
<!-- impl Context::fn get_dash -->
Gets the current dash array. If not NULL, dashes should be big enough to hold at
least the number of values returned by Context::get_dash_count().
<!-- impl Context::fn set_fill_rule -->
Set the current fill rule within the cairo context. The fill rule is used to determine
which regions are inside or outside a complex (potentially self-intersecting) path.
The current fill rule affects both Context::fill() and Context::clip(). See FillRule
enum for details on the semantics of each available fill rule.

The default fill rule is FillRule::Winding.
<!-- impl Context::fn get_fill_rule -->
Gets the current fill rule, as set by Context::set_fill_rule().
<!-- impl Context::fn set_line_cap -->
Sets the current line cap style within the cairo context. See LineCap enum
for details about how the available line cap styles are drawn.

As with the other stroke parameters, the current line cap style is examined
by Context::stroke(), Context::stroke_extents(), and Context::::stroke_to_path(),
but does not have any effect during path construction.

The default line cap style is LineCap::Butt.
<!-- impl Context::fn get_line_cap -->
Gets the current line cap style, as set by Context::set_line_cap().
<!-- impl Context::fn set_line_join -->
Sets the current line join style within the cairo context. See LineJoin enum
for details about how the available line join styles are drawn.

As with the other stroke parameters, the current line join style is examined by
Context::stroke(), cairo_stroke_extents(), and Context::stroke_to_path(), but
does not have any effect during path construction.

The default line join style is LineJoin::Miter.
<!-- impl Context::fn get_line_join -->
Gets the current line join style, as set by Context::set_line_join().
<!-- impl Context::fn set_line_width -->
Sets the current line width within the cairo context. The line width value specifies
the diameter of a pen that is circular in user space, (though device-space pen may be
an ellipse in general due to scaling/shear/rotation of the CTM).

Note: When the description above refers to user space and CTM it refers to the user
space and CTM in effect at the time of the stroking operation, not the user space and
CTM in effect at the time of the call to Context::set_line_width(). The simplest usage
makes both of these spaces identical. That is, if there is no change to the CTM between
a call to Context::set_line_width() and the stroking operation, then one can just pass
user-space values to Context::set_line_width() and ignore this note.

As with the other stroke parameters, the current line width is examined by
Context::stroke(), Context::stroke_extents(), and Context::stroke_to_path(), but does
not have any effect during path construction.

The default line width value is 2.0.
<!-- impl Context::fn get_line_width -->
This function returns the current line width value exactly as set by
Context::set_line_width(). Note that the value is unchanged even if the CTM has changed
between the calls to Context::set_line_width() and Context::get_line_width().
<!-- impl Context::fn set_miter_limit -->
Sets the current miter limit within the cairo context.

If the current line join style is set to LineJoin::Miter (see Context::set_line_join()),
the miter limit is used to determine whether the lines should be joined with a bevel
instead of a miter. Cairo divides the length of the miter by the line width. If the result
is greater than the miter limit, the style is converted to a bevel.

As with the other stroke parameters, the current line miter limit is examined by
Context::stroke(), Context::stroke_extents(), and Context::stroke_to_path(), but does
not have any effect during path construction.

The default miter limit value is 10.0, which will convert joins with interior angles less
than 11 degrees to bevels instead of miters. For reference, a miter limit of 2.0 makes the
miter cutoff at 60 degrees, and a miter limit of 1.414 makes the cutoff at 90 degrees.

A miter limit for a desired angle can be computed as: miter limit = 1/sin(angle/2)
<!-- impl Context::fn get_miter_limit -->
Gets the current miter limit, as set by Contextset_miter_limit().
<!-- impl Context::fn set_tolerance -->
Sets the tolerance used when converting paths into trapezoids. Curved segments of
the path will be subdivided until the maximum deviation between the original path
and the polygonal approximation is less than tolerance . The default value is 0.1.
A larger value will give better performance, a smaller value, better appearance.
(Reducing the value from the default value of 0.1 is unlikely to improve appearance
significantly.) The accuracy of paths within Cairo is limited by the precision of
its internal arithmetic, and the prescribed tolerance is restricted to the smallest
representable internal value.
<!-- impl Context::fn get_tolerance -->
Gets the current tolerance value, as set by Context::set_tolerance().
<!-- impl Context::fn clip -->
Establishes a new clip region by intersecting the current clip region with the current
path as it would be filled by Context::fill() and according to the current fill rule
(see Context::set_fill_rule()).

After Context::clip(), the current path will be cleared from the cairo context.

The current clip region affects all drawing operations by effectively masking out any
changes to the surface that are outside the current clip region.

Calling Context::clip() can only make the clip region smaller, never larger. But the
current clip is part of the graphics state, so a temporary restriction of the clip
region can be achieved by calling Context::clip() within a Context::save() /
Context::restore() pair. The only other means of increasing the size of the clip
region is Context::reset_clip().
<!-- impl Context::fn clip_preserve -->
Establishes a new clip region by intersecting the current clip region with the current
path as it would be filled by Context::fill() and according to the current fill rule
(see Context::set_fill_rule()).

Unlike Context::clip(), Context::clip_preserve() preserves the path within the cairo
context.

The current clip region affects all drawing operations by effectively masking out any
changes to the surface that are outside the current clip region.

Calling Context::clip_preserve() can only make the clip region smaller, never larger.
But the current clip is part of the graphics state, so a temporary restriction of the
clip region can be achieved by calling Context::clip_preserve() within a
Context::save()/cairo_restore() pair. The only other means of increasing the size of
the clip region is Context::reset_clip().
<!-- impl Context::fn clip_extents -->
Computes a bounding box in user coordinates covering the area inside the current clip.
<!-- impl Context::fn in_clip -->
Tests whether the given point is inside the area that would be visible through the current
clip, i.e. the area that would be filled by a Context::paint() operation.

See Context::clip(), and Context::clip_preserve().
<!-- impl Context::fn reset_clip -->
Reset the current clip region to its original, unrestricted state. That is, set the clip
region to an infinitely large shape containing the target surface. Equivalently, if
infinity is too hard to grasp, one can imagine the clip region being reset to the exact
bounds of the target surface.

Note that code meant to be reusable should not call Context::reset_clip() as it will
cause results unexpected by higher-level code which calls Context::clip(). Consider
using Context::save() and Context::restore() around Context::clip() as a more robust
means of temporarily restricting the clip region.
<!-- impl Context::fn copy_clip_rectangle_list -->
Gets the current clip region as a list of rectangles in user coordinates.

The status in the list may be Status::ClipNotRepresentable to indicate that
the clip region cannot be represented as a list of user-space rectangles.
The status may have other values to indicate other errors.
<!-- impl Context::fn fill -->
A drawing operator that fills the current path according to the current fill
rule, (each sub-path is implicitly closed before being filled). After
Context::fill(), the current path will be cleared from the cairo context. See
Context::set_fill_rule() and Context::fill_preserve().
<!-- impl Context::fn fill_preserve -->
A drawing operator that fills the current path according to the current fill rule,
(each sub-path is implicitly closed before being filled). Unlike Context::fill(),
Context::fill_preserve() preserves the path within the cairo context.

See Context::set_fill_rule() and Context::fill().
<!-- impl Context::fn fill_extents -->
Computes a bounding box in user coordinates covering the area that would be affected,
(the "inked" area), by a Context::fill() operation given the current path and fill
parameters. If the current path is empty, returns an empty rectangle ((0,0), (0,0)).
Surface dimensions and clipping are not taken into account.

Contrast with Context::path_extents(), which is similar, but returns non-zero extents
for some paths with no inked area, (such as a simple line segment).

Note that Context::fill_extents() must necessarily do more work to compute the precise
inked areas in light of the fill rule, so Context::path_extents() may be more desirable
for sake of performance if the non-inked path extents are desired.

See Context::fill(), Context::set_fill_rule() and Context::fill_preserve().
<!-- impl Context::fn in_fill -->
Tests whether the given point is inside the area that would be affected by a
Context::fill() operation given the current path and filling parameters. Surface
dimensions and clipping are not taken into account.

See Context::fill(), Context::set_fill_rule() and Context::fill_preserve().
<!-- impl Context::fn mask -->
A drawing operator that paints the current source using the alpha channel of
pattern as a mask. (Opaque areas of pattern are painted with the source, transparent
areas are not painted.)
<!-- impl Context::fn paint -->
A drawing operator that paints the current source everywhere within the current clip region.
<!-- impl Context::fn paint_with_alpha -->
A drawing operator that paints the current source everywhere within the current clip region
using a mask of constant alpha value alpha . The effect is similar to Context::paint(), but
the drawing is faded out using the alpha value.
<!-- impl Context::fn stroke -->
A drawing operator that strokes the current path according to the current line width, line
join, line cap, and dash settings. After Context::stroke(), the current path will be cleared
from the cairo context. See Context::set_line_width(), Context::set_line_join(),
Context::set_line_cap(), Context::set_dash(), and Context::stroke_preserve().

Note: Degenerate segments and sub-paths are treated specially and provide a useful result.
These can result in two different situations:

1. Zero-length "on" segments set in Context::set_dash(). If the cap style is LineCap::Round
or LineCap::Square then these segments will be drawn as circular dots or squares respectively.
In the case of LineCap::Square, the orientation of the squares is determined by the direction
of the underlying path.

2. A sub-path created by Context::move_to() followed by either a Context::close_path() or one
or more calls to Context::line_to() to the same coordinate as the Context::move_to(). If the
cap style is LineCap::Round then these sub-paths will be drawn as circular dots. Note that in
the case of LineCap::Square a degenerate sub-path will not be drawn at all, (since the correct
orientation is indeterminate).

In no case will a cap style of LineCap::Butt cause anything to be drawn in the case of either
degenerate segments or sub-paths.
<!-- impl Context::fn stroke_preserve -->
A drawing operator that strokes the current path according to the current line width, line
join, line cap, and dash settings. Unlike Context::stroke(), Context::stroke_preserve()
preserves the path within the cairo context.

See Context::set_line_width(), Context::set_line_join(), Context::set_line_cap(),
Context::set_dash(), and Context::stroke_preserve().
<!-- impl Context::fn stroke_extents -->
Computes a bounding box in user coordinates covering the area that would be affected,
(the "inked" area), by a Context::stroke() operation given the current path and stroke
parameters. If the current path is empty, returns an empty rectangle ((0,0), (0,0)).
Surface dimensions and clipping are not taken into account.

Note that if the line width is set to exactly zero, then Context::stroke_extents() will
return an empty rectangle. Contrast with cairo_path_extents() which can be used to compute
the non-empty bounds as the line width approaches zero.

Note that cairo_stroke_extents() must necessarily do more work to compute the precise inked
areas in light of the stroke parameters, so cairo_path_extents() may be more desirable for
sake of performance if non-inked path extents are desired.

See Context::stroke(), Context::set_line_width(), Context::set_line_join(),
Context::set_line_cap(), Context::set_dash(), and Context::stroke_preserve().
<!-- impl Context::fn in_stroke -->
Tests whether the given point is inside the area that would be affected by a 
Context::stroke() operation given the current path and stroking parameters. Surface
dimensions and clipping are not taken into account.

See Context::stroke(), Context::set_line_width(), Context::set_line_join(),
Context::set_line_cap(), Context::set_dash(), and Context::stroke_preserve().
<!-- impl Context::fn copy_page -->
Emits the current page for backends that support multiple pages, but doesn't clear
it, so, the contents of the current page will be retained for the next page too. Use
Context::show_page() if you want to get an empty page after the emission.

This is a convenience function that simply calls Surface::copy_page() on self's
target.
<!-- impl Context::fn show_page -->
Emits and clears the current page for backends that support multiple pages. Use
Context::copy_page() if you don't want to clear the page.

This is a convenience function that simply calls Surface::show_page() on self's
target.
<!-- impl Context::fn get_reference_count -->
Returns the current reference count of self.
<!-- impl Context::fn translate -->
Modifies the current transformation matrix (CTM) by translating the user-space
origin by (tx , ty ). This offset is interpreted as a user-space coordinate
according to the CTM in place before the new call to cairo_translate(). In other
words, the translation of the user-space origin takes place after any existing
transformation.
<!-- impl Context::fn scale -->
Modifies the current transformation matrix (CTM) by scaling the X and Y user-space
axes by sx and sy respectively. The scaling of the axes takes place after any
existing transformation of user space.
<!-- impl Context::fn rotate -->
Modifies the current transformation matrix (CTM) by rotating the user-space axes by
angle radians. The rotation of the axes takes places after any existing transformation
of user space. The rotation direction for positive angles is from the positive X axis
toward the positive Y axis.
<!-- impl Context::fn identity_matrix -->
Resets the current transformation matrix (CTM) by setting it equal to the identity
matrix. That is, the user-space and device-space axes will be aligned and one user-space
unit will transform to one device-space unit.
<!-- impl Context::fn user_to_device -->
Transform a coordinate from user space to device space by multiplying the given point
by the current transformation matrix (CTM).
<!-- impl Context::fn user_to_device_distance -->
Transform a distance vector from user space to device space. This function is similar
to Context::user_to_device() except that the translation components of the CTM will
be ignored when transforming (dx ,dy ).
<!-- impl Context::fn device_to_user -->
Transform a coordinate from device space to user space by multiplying the given point
by the inverse of the current transformation matrix (CTM).
<!-- impl Context::fn device_to_user_distance -->
Transform a distance vector from device space to user space. This function is similar
to Context::device_to_user() except that the translation components of the inverse CTM
will be ignored when transforming (dx ,dy ).
<!-- impl Context::fn select_font_face -->
Note: The Context::select_font_face() function call is part of what the cairo designers
call the "toy" text API. It is convenient for short demos and simple programs, but it
is not expected to be adequate for serious text-using applications.

Selects a family and style of font from a simplified description as a family name, slant
and weight. Cairo provides no operation to list available family names on the system (this
is a "toy", remember), but the standard CSS2 generic family names, ("serif", "sans-serif",
"cursive", "fantasy", "monospace"), are likely to work as expected.

If family starts with the string "cairo :", or if no native font backends are compiled in,
cairo will use an internal font family. The internal font family recognizes many modifiers
in the family string, most notably, it recognizes the string "monospace". That is, the
family name "cairo :monospace" will use the monospace version of the internal font family.

For "real" font selection, see the font-backend-specific font_face_create functions for the
font backend you are using. (For example, if you are using the freetype-based cairo-ft font
backend, see Font::create_for_ft_face() or Font::create_for_pattern().) The resulting font
face could then be used with Context::scaled_font_create() and Context::set_scaled_font().

Similarly, when using the "real" font support, you can call directly into the underlying
font system, (such as fontconfig or freetype), for operations such as listing available
fonts, etc.

It is expected that most applications will need to use a more comprehensive font handling
and text layout library, (for example, pango), in conjunction with cairo.

If text is drawn without a call to Context::select_font_face(), (nor Context::set_font_face()
nor Context::set_scaled_font()), the default family is platform-specific, but is essentially
"sans-serif". Default slant is FontSlant::Normal, and default weight is FontWeight::Normal.

This function is equivalent to a call to cairo_toy_font_face_create() followed by
Context::set_font_face().
<!-- impl Context::fn set_font_size -->
Sets the current font matrix to a scale by a factor of size , replacing any font matrix
previously set with Context::set_font_size() or Context::set_font_matrix(). This results
in a font size of size user space units. (More precisely, this matrix will result in the
font's em-square being a size by size square in user space.)

If text is drawn without a call to Context::set_font_size(), (nor
Context::set_font_matrix() nor Context::set_scaled_font()), the default font size is 10.0.
<!-- impl Context::fn get_font_matrix -->
Stores the current font matrix into matrix . See Context::set_font_matrix().
<!-- impl Context::fn set_font_options -->
Sets a set of custom font rendering options for the Context. Rendering options are
derived by merging these options with the options derived from underlying surface;
if the value in options has a default value (like Antialias::Default), then the value
from the surface is used.
<!-- impl Context::fn get_font_options -->
Retrieves font rendering options set via Context::set_font_options. Note that the returned
options do not include any options derived from the underlying surface; they are literally
the options passed to Context::set_font_options().
<!-- impl Context::fn set_font_face -->
Replaces the current FontFace object in the Context with font_face. The replaced
font face in the cairo_t will be destroyed if there are no other references to it.
<!-- impl Context::fn get_font_face -->
Gets the current font face for a Context object.
<!-- impl Context::fn set_scaled_font -->
Replaces the current font face, font matrix, and font options in the Context with
those of the ScaledFont object. Except for some translation, the current CTM of the
Context should be the same as that of the ScaledFont object, which can be accessed
using Context::scaled_font_get_ctm().
<!-- impl Context::fn get_scaled_font -->
Gets the current scaled font for a Context.
<!-- impl Context::fn show_text -->
A drawing operator that generates the shape from a string of UTF-8 characters,
rendered according to the current FontFace, FontSize (FontMatrix), and
font_options.

This function first computes a set of glyphs for the string of text. The first
glyph is placed so that its origin is at the current point. The origin of each
subsequent glyph is offset from that of the previous glyph by the advance values
of the previous glyph.

After this call the current point is moved to the origin of where the next glyph
would be placed in this same progression. That is, the current point will be at
the origin of the final glyph offset by its advance values. This allows for easy
display of a single logical string with multiple calls to Context::show_text().

Note: The Context::show_text() function call is part of what the cairo designers
call the "toy" text API. It is convenient for short demos and simple programs,
but it is not expected to be adequate for serious text-using applications. See
Context::show_glyphs() for the "real" text display API in cairo.
<!-- impl Context::fn show_text_glyphs -->
A drawing operator that generates the shape from an array of glyphs, rendered
according to the current font face, font size (font matrix), and font options.
<!-- impl Context::fn font_extents -->
Gets the font extents for the currently selected font.
<!-- impl Context::fn text_extents -->
Gets the extents for a string of text. The extents describe a user-space rectangle
that encloses the "inked" portion of the text, (as it would be drawn by
Context::show_text()). Additionally, the x_advance and y_advance values indicate
the amount by which the current point would be advanced by Context::show_text().

Note that whitespace characters do not directly contribute to the size of the rectangle
(extents.width and extents.height). They do contribute indirectly by changing the
position of non-whitespace characters. In particular, trailing whitespace characters
are likely to not affect the size of the rectangle, though they will affect the
x_advance and y_advance values.
<!-- impl Context::fn glyph_extents -->
Gets the extents for an array of glyphs. The extents describe a user-space rectangle
that encloses the "inked" portion of the glyphs, (as they would be drawn by
Context::show_glyphs()). Additionally, the x_advance and y_advance values indicate
the amount by which the current point would be advanced by Context::show_glyphs().

Note that whitespace glyphs do not contribute to the size of the rectangle
(extents.width and extents.height).
<!-- impl Context::fn copy_path -->
Creates a copy of the current path and returns it to the user as a Path. See
PathData for hints on how to iterate over the returned data structure.

This function will always return a valid pointer, but the result will have no
data (data==NULL and num_data==0), if either of the following conditions hold:

1. If there is insufficient memory to copy the path. In this case path.status
will be set to Status::NoMemory.
2. If self is already in an error state. In this case path.status will contain
the same status that would be returned by cairo_status().
<!-- impl Context::fn copy_path_flat -->
Gets a flattened copy of the current path and returns it to the user as a Path.
See PathData for hints on how to iterate over the returned data structure.

This function is like cairo_copy_path() except that any curves in the path will
be approximated with piecewise-linear approximations, (accurate to within the
current tolerance value). That is, the result is guaranteed to not have any
elements of type Path::CurveTo which will instead be replaced by a series of
Path::Line elements.

This function will always return a valid pointer, but the result will have no
data (data==NULL and num_data==0), if either of the following conditions hold:

1. If there is insufficient memory to copy the path. In this case path->status
will be set to Status::NoMemory.
2. If self is already in an error state. In this case path->status will contain
the same status that would be returned by Context::status().
<!-- impl Context::fn append_path -->
Append the path onto the current path. The path may be either the return value
from one of Context::copy_path() or Context::copy_path_flat() or it may be
constructed manually. See Path for details on how the path data structure should
be initialized, and note that path.status must be initialized to Status::Success.
<!-- impl Context::fn has_current_point -->
Returns whether a current point is defined on the current path. See
Context::get_current_point() for details on the current point.
<!-- impl Context::fn get_current_point -->
Gets the current point of the current path, which is conceptually the final
point reached by the path so far.

The current point is returned in the user-space coordinate system. If there is
no defined current point or if cr is in an error status, x and y will both be set
to 0.0. It is possible to check this in advance with cairo_has_current_point().

Most path construction functions alter the current point. See the following for
details on how they affect the current point: Context::new_path(),
Context::new_sub_path(), Context::append_path(), Context::close_path(),
Context::move_to(), Context::line_to(), Context::curve_to(), cairo_rel_move_to(),
Context::rel_line_to(), Context::rel_curve_to(), Context::arc(),
Context::arc_negative(), Context::rectangle(), Context::text_path(),
Context::glyph_path(), Context::stroke_to_path().

Some functions use and alter the current point but do not otherwise change current path: Context::show_text().

Some functions unset the current path and as a result, current point: Context::fill(), Context::stroke().
<!-- impl Context::fn new_path -->
Clears the current path. After this call there will be no path and no current point.
<!-- impl Context::fn new_sub_path -->
Begin a new sub-path. Note that the existing path is not affected. After this call
there will be no current point.

In many cases, this call is not needed since new sub-paths are frequently started
with Context::move_to().

A call to Context::new_sub_path() is particularly useful when beginning a new
sub-path with one of the Context::arc() calls. This makes things easier as it is
no longer necessary to manually compute the arc's initial coordinates for a call to
Context::move_to().
<!-- impl Context::fn close_path -->
Adds a line segment to the path from the current point to the beginning of
the current sub-path, (the most recent point passed to cairo_move_to()), and
closes this sub-path. After this call the current point will be at the joined
endpoint of the sub-path.

The behavior of Context::close_path() is distinct from simply calling
Context::line_to() with the equivalent coordinate in the case of stroking. When
a closed sub-path is stroked, there are no caps on the ends of the sub-path.
Instead, there is a line join connecting the final and initial segments of the
sub-path.

If there is no current point before the call to Context::close_path(), this
function will have no effect.

Note: As of cairo version 1.2.4 any call to Context::close_path() will place
an explicit MOVE_TO element into the path immediately after the CLOSE_PATH
element, (which can be seen in Context::copy_path() for example). This can
simplify path processing in some cases as it may not be necessary to save the
"last move_to point" during processing as the MOVE_TO immediately after the
CLOSE_PATH will provide that point.
<!-- impl Context::fn arc -->
Adds a circular arc of the given radius to the current path. The arc is centered
at (xc , yc ), begins at angle1 and proceeds in the direction of increasing
angles to end at angle2 . If angle2 is less than angle1 it will be progressively
increased by 2*M_PI until it is greater than angle1 .

If there is a current point, an initial line segment will be added to the path
to connect the current point to the beginning of the arc. If this initial line is
undesired, it can be avoided by calling Context::new_sub_path() before calling
Context::arc().

Angles are measured in radians. An angle of 0.0 is in the direction of the
positive X axis (in user space). An angle of M_PI/2.0 radians (90 degrees) is
in the direction of the positive Y axis (in user space). Angles increase in the
direction from the positive X axis toward the positive Y axis. So with the default
transformation matrix, angles increase in a clockwise direction.

(To convert from degrees to radians, use degrees * (M_PI / 180.).)

This function gives the arc in the direction of increasing angles; see
Context::arc_negative() to get the arc in the direction of decreasing angles.

The arc is circular in user space. To achieve an elliptical arc, you can scale
the current transformation matrix by different amounts in the X and Y directions.
For example, to draw an ellipse in the box given by x , y , width , height :

```ignore
cr.save();
cr.translate(x + width / 2., y + height / 2.);
cr.scale(width / 2., height / 2.);
cr.arc(0., 0., 1., 0., 2 * M_PI);
cr.restore();
```
<!-- impl Context::fn arc_negative -->
Adds a circular arc of the given radius to the current path. The arc is centered
at (xc , yc ), begins at angle1 and proceeds in the direction of decreasing
angles to end at angle2 . If angle2 is greater than angle1 it will be
progressively decreased by 2*M_PI until it is less than angle1.

See Context::arc() for more details. This function differs only in the direction
of the arc between the two angles.
<!-- impl Context::fn curve_to -->
Adds a cubic Bézier spline to the path from the current point to position
(x3 , y3 ) in user-space coordinates, using (x1 , y1 ) and (x2 , y2 ) as the
control points. After this call the current point will be (x3 , y3 ).

If there is no current point before the call to Context::curve_to() this function
will behave as if preceded by a call to Context::move_to(cr , x1 , y1 ).
<!-- impl Context::fn line_to -->
Adds a line to the path from the current point to position (x , y ) in user-space
coordinates. After this call the current point will be (x , y ).

If there is no current point before the call to cairo_line_to() this function
will behave as cairo_move_to(cr , x , y ).
<!-- impl Context::fn move_to -->
Begin a new sub-path. After this call the current point will be (x , y ).
<!-- impl Context::fn rectangle -->
Adds a closed sub-path rectangle of the given size to the current path at
position (x , y ) in user-space coordinates.

This function is logically equivalent to:

```ignore
cr.move_to(, x, y);
cr.rel_line_to(width, 0);
cr.rel_line_to(0, height);
cr.rel_line_to(-width, 0);
cr.close_path();
```
<!-- impl Context::fn text_path -->
Adds closed paths for the glyphs to the current path. The generated path if 
filled, achieves an effect similar to that of Context::show_glyphs().
<!-- impl Context::fn rel_curve_to -->
Relative-coordinate version of Context::curve_to(). All offsets are relative to
the current point. Adds a cubic Bézier spline to the path from the current point
to a point offset from the current point by (dx3 , dy3 ), using points offset by
(dx1 , dy1 ) and (dx2 , dy2 ) as the control points. After this call the current
point will be offset by (dx3 , dy3 ).

Given a current point of (x, y),
Context::rel_curve_to(dx1 , dy1 , dx2 , dy2 , dx3 , dy3 ) is logically
equivalent to Context::curve_to(x+dx1 , y+dy1 , x+dx2 , y+dy2 , x+dx3 , y+dy3 ).

It is an error to call this function with no current point. Doing so will cause
self to shutdown with a status of Status::NoCurrentPoint.
<!-- impl Context::fn rel_line_to -->
Relative-coordinate version of Context::line_to(). Adds a line to the path from
the current point to a point that is offset from the current point by (dx , dy )
in user space. After this call the current point will be offset by (dx , dy ).

Given a current point of (x, y), Context::rel_line_to(dx , dy ) is logically
equivalent to Context::line_to( x + dx , y + dy ).

It is an error to call this function with no current point. Doing so will cause
self to shutdown with a status of Status::NoCurrentPoint.
<!-- impl Context::fn rel_move_to -->
Begin a new sub-path. After this call the current point will offset by (x , y ).

Given a current point of (x, y), Context::rel_move_to(dx , dy ) is logically
equivalent to Context::move_to(x + dx , y + dy ).

It is an error to call this function with no current point. Doing so will
cause self to shutdown with a status of Status::NoCurrenPoint.
<!-- file fonts.rs -->
<!-- file_comment -->
FontOptions: How a font should be rendered.

FontFace: Base class for font faces.

ScaledFont: Font face at particular size and options.
<!-- impl FontOptions -->
The font options specify how fonts should be rendered. Most of the time the font options
implied by a surface are just right and do not need any changes, but for pixel-based targets
tweaking font options may result in superior output on a particular display.
<!-- impl FontOptions::fn new -->
Allocates a new font options object with all options initialized to default values.
<!-- impl FontOptions::fn ensure_status -->
Checks whether an error has previously occurred for this font options object.
<!-- impl FontOptions::fn merge -->
Merges non-default options from other into self, replacing existing values.
This operation can be thought of as somewhat similar to compositing other onto
self with the operation of Operator::Over.
<!-- impl FontOptions::fn hash -->
Compute a hash for the font options object; this value will be useful when
storing an object containing a FontOptions in a hash table.
<!-- impl FontOptions::fn set_antialias -->
Sets the antialiasing mode for the font options object. This specifies the type
of antialiasing to do when rendering text.
<!-- impl FontOptions::fn get_antialias -->
Gets the antialiasing mode for the font options object.
<!-- impl FontOptions::fn set_subpixel_order -->
Sets the subpixel order for the font options object. The subpixel order specifies
the order of color elements within each pixel on the display device when rendering
with an antialiasing mode of Antialias::Subpixel. See the documentation for
SubpixelOrder for full details.
<!-- impl FontOptions::fn get_subpixel_order -->
Gets the subpixel order for the font options object. See the documentation for
SubpixelOrder for full details.
<!-- impl FontOptions::fn set_hint_style -->
Sets the hint style for font outlines for the font options object. This controls
whether to fit font outlines to the pixel grid, and if so, whether to optimize
for fidelity or contrast. See the documentation for HintStyle for full
details.
<!-- impl FontOptions::fn get_hint_style -->
Gets the hint style for font outlines for the font options object. See the
documentation for HintStyle for full details.
<!-- impl FontOptions::fn set_hint_metrics -->
Sets the metrics hinting mode for the font options object. This controls
whether metrics are quantized to integer values in device units. See the
documentation for HintMetrics for full details.
<!-- impl FontOptions::fn get_hint_metrics -->
Gets the metrics hinting mode for the font options object. See the documentation
for HintMetrics for full details.
<!-- impl PartialEq for FontOptions::fn eq -->
Compares two font options objects for equality.
<!-- struct FontFace -->
FontFace represents a particular font at a particular weight, slant, and other
characteristic but no size, transformation, or size.

Font faces are created using font-backend-specific constructors, typically of the
form Context::backend_font_face_create(), or implicitly using the toy text API by
way of Context::select_font_face(). The resulting face can be accessed using
Context::get_font_face().
<!-- impl FontFace::fn toy_create -->
Creates a font face from a triplet of family, slant, and weight. These font faces
are used in implementation of the the cairo "toy" font API.

If family is the zero-length string "", the platform-specific default family is assumed.
The default family then can be queried using FontFace::toy_get_family().

The Context::select_font_face() function uses this to create font faces. See that
function for limitations and other details of toy font faces.
<!-- impl FontFace::fn toy_get_family -->
Gets the familly name of a toy font.
<!-- impl FontFace::fn toy_get_slant -->
Gets the slant a toy font.
<!-- impl FontFace::fn toy_get_weight -->
Gets the weight a toy font.
<!-- impl FontFace::fn ensure_status -->
Checks whether an error has previously occurred for this font face.
<!-- impl FontFace::fn get_type -->
This function returns the type of the backend used to create a font face. See
FontType for available types.
<!-- impl FontFace::fn get_reference_count -->
Returns the current reference count of self.
<!-- impl FontFace::fn reference -->
Increases the reference count on self by one. This prevents self from being
destroyed until a matching call to FontFace drop trait is made.

The number of references to a FontFace can be get using
FontFace::get_reference_count().
<!-- struct ScaledFont -->
ScaledFont represents a realization of a font face at a particular size and
transformation and a certain set of font options.
<!-- impl ScaledFont::fn new -->
Creates a ScaledFont object from a font face and matrices that describe the
size of the font and the environment in which it will be used.
<!-- impl ScaledFont::fn ensure_status -->
Checks whether an error has previously occurred for this ScaledFont.
<!-- impl ScaledFont::fn get_type -->
This function returns the type of the backend used to create a scaled font.
See FontType for available types. However, this function never returns
FontType::Toy.
<!-- impl ScaledFont::fn get_reference_count -->
Returns the current reference count of self.
<!-- impl ScaledFont::fn reference -->
Increases the reference count on self by one. This prevents self from being
destroyed until a matching call to ScaledFont drop trait is made.

The number of references to a cairo_scaled_font_t can be get using
ScaledFont::get_reference_count().
<!-- file lib.rs -->
<!-- file_comment -->


<!-- file matrices.rs -->
<!-- file_comment -->
Generic matrix operations
<!-- trait MatrixTrait::fn null -->
Creates a new Matrix filled with zeroes
<!-- trait MatrixTrait::fn new -->
Creates a new matrix and fills it with given values
<!-- trait MatrixTrait::fn multiply -->
Multiplies the affine transformations in a and b together and stores the result in
the returned Matrix. The effect of the resulting transformation is to first apply
the transformation in left to the coordinates and then apply the transformation in
right to the coordinates.

It is allowable for the returned Matrix to be identical to either a or  .
<!-- trait MatrixTrait::fn identity -->
Returns a new matrix after modifying it to be an identity transformation.
<!-- trait MatrixTrait::fn init -->
Sets self to be the affine transformation given by xx , yx , xy , yy , x0 , y0. The
transformation is given by:

```ignore
x_new = xx * x + xy * y + x0;
y_new = yx * x + yy * y + y0;
```
<!-- trait MatrixTrait::fn translate -->
Applies a translation by tx, ty to the transformation in self. The effect of the new
transformation is to first translate the coordinates by tx and ty, then apply the
original transformation to the coordinates.
<!-- trait MatrixTrait::fn scale -->
Applies scaling by sx, sy to the transformation in self. The effect of the new
transformation is to first scale the coordinates by sx and sy, then apply the original
transformation to the coordinates.
<!-- trait MatrixTrait::fn rotate -->
Applies rotation by radians to the transformation in self. The effect of the new
transformation is to first rotate the coordinates by radians , then apply the original
transformation to the coordinates.
<!-- trait MatrixTrait::fn invert -->
Inverts a matrix in-place.

# Panics
Panics if the matrix is not invertible: if the matrix
collapses points together (it is degenerate), then it has no
inverse and this function will panic.

Normally a non-invertible matrix indicates a bug; all matrices
that originate from your code should always be valid,
invertible matrices.

If you construct a matrix from untrusted data and need to validate
it, you can use [`try_invert()`].

[`try_invert()`]: #tymethod.try_invert

# Example
```ignore
use cairo::{Matrix, MatrixTrait};

let mut matrix = Matrix::identity();
matrix.invert();
assert!(matrix == Matrix::identity());
```

<!-- trait MatrixTrait::fn try_invert -->
Tries to invert a matrix, and returns the inverted result or an error
if the matrix is not invertible.

A matrix is not invertible if it collapses points together (it is
degenerate).

Normally, matrices that originate from your code should always
be valid, invertible matrices.  You should use this function
only to validate matrices that come from untrusted data.

# Errors
Non-invertible matrices yield an `Err(Status::InvalidMatrix)` error.

# Example

```ignore
use cairo::{Matrix, MatrixTrait};

let matrix = Matrix::identity();
assert!(matrix.try_invert().unwrap() == Matrix::identity());

let all_zeros_matrix = Matrix::null();
assert!(all_zeros_matrix.try_invert().is_err());
```
<!-- trait MatrixTrait::fn transform_distance -->
Transforms the distance vector (dx, dy) by self. This is similar to
Matrix::transform_point() except that the translation components of the transformation
are ignored. The calculation of the returned vector is as follows:

```ignore
dx2 = dx1 * a + dy1 * c;
dy2 = dx1 * b + dy1 * d;
```

Affine transformations are position invariant, so the same vector always transforms to
the same vector. If (x1 ,y1 ) transforms to (x2 ,y2 ) then (x1 +dx1 ,y1 +dy1 ) will
transform to (x1 + dx2, y1 + dy2) for all values of x1 and x2 .
<!-- trait MatrixTrait::fn transform_point -->
Transforms the point (x , y) by self.
<!-- file paths.rs -->
<!-- file_comment -->
Creating paths and manipulating path data
<!-- struct Path -->
Paths are the most basic drawing tools and are primarily used to implicitly generate simple masks.
<!-- file patterns.rs -->
<!-- file_comment -->
Sources for drawing
<!-- trait Pattern::fn status -->
Checks whether an error has previously occurred for this pattern.
<!-- trait Pattern::fn get_reference_count -->
Returns the current reference count of self.
<!-- trait Pattern::fn set_extend -->
Sets the mode to be used for drawing outside the area of a pattern. See cairo_extend_t for
details on the semantics of each extend strategy.

The default extend mode is Extend::None for surface patterns and Extend::Pad for gradient
patterns.
<!-- trait Pattern::fn get_extend -->
Gets the current extend mode for a pattern. See Extend enum for details on the semantics
of each extend strategy.
<!-- trait Pattern::fn set_filter -->
Sets the filter to be used for resizing when using this pattern. See Filter enum for
details on each filter.

Note that you might want to control filtering even when you do not have an explicit
Pattern object, (for example when using cairo_set_source_surface()). In these
cases, it is convenient to use cairo_get_source() to get access to the pattern that cairo
creates implicitly. For example:

```ignore
Context::set_source_surface(image, x, y);
p.set_filter(Filter::nearest);
```
<!-- trait Pattern::fn get_filter -->
Gets the current filter for a pattern. See Filter enum for details on each filter.
<!-- trait Pattern::fn set_matrix -->
Sets the pattern's transformation matrix, which is a transformation
from user space to pattern space.

When a pattern is first created it always has the identity matrix for
its transformation matrix, which means that pattern space is initially
identical to user space.

Note that the direction of this transformation matrix is from user
space to pattern space.  This means that if you imagine the flow from
a pattern to user space (and on to device space), then coordinates
inthat flow will be transformed by the inverse of the pattern matrix.

For example, if you want to make a pattern appear twice as large as it
does by default, you should use this code:

```ignore
use cairo::{Matrix, MatrixTrait};

let matrix = Matrix::identity().scale (0.5, 0.5);
my_pattern.set_matrix (matrix);
```
<!-- trait Pattern::fn get_matrix -->
Gets the current transformation matrix for a pattern.  See the
documentation for [`set_matrix()`] for the details on this matrix.

[`set_matrix()`]: #tymethod.set_matrix
<!-- impl SolidPattern::fn from_rgb -->
Creates a new SolidPattern corresponding to an opaque color. The color components
are floating point numbers in the range 0 to 1.

Note : If the values passed in are outside
that range, they will be clamped.
<!-- impl SolidPattern::fn from_rgba -->
Creates a new SolidPattern corresponding to a translucent color. The color components
are floating point numbers in the range 0 to

Note : If the values passed in are outside that range, they will be clamped.
<!-- impl SolidPattern::fn get_rgba -->
Gets the solid color for a solid color pattern.
<!-- trait Gradient::fn add_color_stop_rgb -->
Adds an opaque color stop to a gradient pattern. The offset specifies the
location along the gradient's control vector. For example, a linear gradient's
control vector is from (x0,y0) to (x1,y1) while a radial gradient's control
vector is from any point on the start circle to the corresponding point on the
end circle.

The color is specified in the same way as in Context::set_source_rgba().

If two (or more) stops are specified with identical offset values, they will be
sorted according to the order in which the stops are added, (stops added earlier
will compare less than stops added later). This can be useful for reliably making
sharp color transitions instead of the typical blend.

Note: If the pattern is not a gradient pattern, (eg. a linear or radial pattern),
then the pattern will be put into an error status with a status of
StatusPattern::TypeMismatch.
<!-- trait Gradient::fn add_color_stop_rgba -->
Adds a translucent color stop to a gradient pattern. The offset specifies the
location along the gradient's control vector. For example, a linear gradient's
control vector is from (x0,y0) to (x1,y1) while a radial gradient's control vector
is from any point on the start circle to the corresponding point on the end circle.

The color is specified in the same way as in Context::set_source_rgba().

If two (or more) stops are specified with identical offset values, they will be
sorted according to the order in which the stops are added, (stops added earlier will
compare less than stops added later). This can be useful for reliably making sharp
color transitions instead of the typical blend.

Note: If the pattern is not a gradient pattern, (eg. a linear or radial pattern), then
the pattern will be put into an error status with a status of StatusPattern::TypeMismatch.
<!-- trait Gradient::fn get_color_stop_count -->
Gets the number of color stops specified in the given gradient pattern.
<!-- trait Gradient::fn get_color_stop_rgba -->
Gets the color and offset information at the given index for a gradient pattern.
Values of index range from 0 to n-1 where n is the number returned by
Pattern::get_color_stop_count().
<!-- impl LinearGradient::fn new -->
Create a new linear gradient Pattern object along the line defined by
(x0, y0) and (x1, y1). Before using the gradient pattern, a number of color
stops should be defined using Pattern::add_color_stop_rgb() or
Pattern::add_color_stop_rgba().

Note: The coordinates here are in pattern space. For a new pattern, pattern
space is identical to user space, but the relationship between the spaces can
be changed with Pattern::set_matrix().
<!-- impl LinearGradient::fn get_linear_points -->
Gets the gradient endpoints for a linear gradient.
<!-- impl RadialGradient::fn new -->
Creates a new radial gradient Pattern between the two circles
defined by (cx0, cy0, radius0) and (cx1, cy1, radius1). Before
using the gradient pattern, a number of color stops should be
defined using Pattern::add_color_stop_rgb() or Pattern::add_color_stop_rgba().

Note: The coordinates here are in pattern space. For a new pattern, pattern
space is identical to user space, but the relationship between the spaces can
be changed with Pattern::set_matrix().
<!-- impl RadialGradient::fn get_radial_circles -->
Gets the gradient endpoint circles for a radial gradient, each specified as a center
coordinate and a radius.
<!-- impl Mesh::fn new -->
Create a new mesh pattern.

Mesh patterns are tensor-product patch meshes (type 7 shadings in PDF). Mesh
patterns may also be used to create other types of shadings that are special
cases of tensor-product patch meshes such as Coons patch meshes (type 6 shading
in PDF) and Gouraud-shaded triangle meshes (type 4 and 5 shadings in PDF).

Mesh patterns consist of one or more tensor-product patches, which should be
defined before using the mesh pattern. Using a mesh pattern with a partially
defined patch as source or mask will put the context in an error status with
a status of Status::InvalidMeshConstruction.

A tensor-product patch is defined by 4 Bézier curves (side 0, 1, 2, 3) and by
4 additional control points (P0, P1, P2, P3) that provide further control over
the patch and complete the definition of the tensor-product patch. The corner
C0 is the first point of the patch.

Degenerate sides are permitted so straight lines may be used. A zero length
line on one side may be used to create 3 sided patches.

```text
      C1     Side 1       C2
       +---------------+
       |               |
       |  P1       P2  |
       |               |
Side 0 |               | Side 2
       |               |
       |               |
       |  P0       P3  |
       |               |
       +---------------+
    C0     Side 3        C3
```

Each patch is constructed by first calling Mesh::begin_patch(),
then Mesh::move_to() to specify the first point in the patch (C0).
Then the sides are specified with calls to Mesh::curve_to() and
cairo_mesh_pattern_line_to().

The four additional control points (P0, P1, P2, P3) in a patch can
be specified with Mesh::set_control_point().

At each corner of the patch (C0, C1, C2, C3) a color may be specified with
Mesh::set_corner_color_rgb() or Mesh::set_corner_color_rgba(). Any corner
whose color is not explicitly specified defaults to transparent black.

A Coons patch is a special case of the tensor-product patch where the control
points are implicitly defined by the sides of the patch. The default value for
any control point not specified is the implicit value for a Coons patch, i.e.
if no control points are specified the patch is a Coons patch.

A triangle is a special case of the tensor-product patch where the control points
are implicitly defined by the sides of the patch, all the sides are lines and one
of them has length 0, i.e. if the patch is specified using just 3 lines, it is a
triangle. If the corners connected by the 0-length side have the same color, the
patch is a Gouraud-shaded triangle.

Patches may be oriented differently to the above diagram. For example the first
point could be at the top left. The diagram only shows the relationship between
the sides, corners and control points. Regardless of where the first point is
located, when specifying colors, corner 0 will always be the first point, corner
1 the point between side 0 and side 1 etc.

Calling Mesh::end_patch() completes the current patch. If less than 4 sides have
been defined, the first missing side is defined as a line from the current point
to the first point of the patch (C0) and the other sides are degenerate lines from
C0 to C0. The corners between the added sides will all be coincident with C0 of
the patch and their color will be set to be the same as the color of C0.

Additional patches may be added with additional calls to
Mesh::begin_patch()/Mesh::end_patch().

```ignore
let mut pattern = Mesh::new();
 Add a Coons patch 
pattern.begin_patch();
pattern.move_to(0, 0);
pattern.curve_to(30, -30,  60,  30, 100, 0);
pattern.curve_to(60,  30, 130,  60, 100, 100);
pattern.curve_to(60,  70,  30, 130,   0, 100);
pattern.curve_to(30,  70, -30,  30,   0, 0);
pattern.set_corner_color_rgb(0, 1, 0, 0);
pattern.set_corner_color_rgb(1, 0, 1, 0);
pattern.set_corner_color_rgb(2, 0, 0, 1);
pattern.set_corner_color_rgb(3, 1, 1, 0);
pattern.end_patch();

 Add a Gouraud-shaded triangle 
pattern.begin_patch()
pattern.move_to(100, 100);
pattern.line_to(130, 130);
pattern.line_to(130,  70);
pattern.set_corner_color_rgb(0, 1, 0, 0);
pattern.set_corner_color_rgb(1, 0, 1, 0);
pattern.set_corner_color_rgb(2, 0, 0, 1);
pattern.end_patch();
```

When two patches overlap, the last one that has been added is drawn over the first
one.

When a patch folds over itself, points are sorted depending on their parameter
coordinates inside the patch. The v coordinate ranges from 0 to 1 when moving from
side 3 to side 1; the u coordinate ranges from 0 to 1 when going from side 0 to side

Points with higher v coordinate hide points with lower v coordinate. When two points
have the same v coordinate, the one with higher u coordinate is above. This means
that points nearer to side 1 are above points nearer to side 3; when this is not
sufficient to decide which point is above (for example when both points belong to
side 1 or side 3) points nearer to side 2 are above points nearer to side 0.

For a complete definition of tensor-product patches, see the PDF specification (ISO32000),
which describes the parametrization in detail.

Note: The coordinates are always in pattern space. For a new pattern, pattern space is
identical to user space, but the relationship between the spaces can be changed with
Pattern::set_matrix().
<!-- impl Mesh::fn begin_patch -->
Begin a patch in a mesh pattern.

After calling this function, the patch shape should be defined with Mesh::move_to(),
Mesh::line_to() and Mesh::curve_to().

After defining the patch, Mesh::end_patch() must be called before using pattern as
a source or mask.

Note: If pattern is not a mesh pattern then pattern will be put into an error status
with a status of Status::PatternTypeMismatch. If pattern already has a current patch,
it will be put into an error status with a status of Status::InvalidMeshConstruction.
<!-- impl Mesh::fn end_patch -->
Indicates the end of the current patch in a mesh pattern.

If the current patch has less than 4 sides, it is closed with a straight line from the
current point to the first point of the patch as if Mesh::line_to() was used.

Note: If pattern is not a mesh pattern then pattern will be put into an error status
with a status of Status::PatternTypeMismatch. If pattern has no current patch or the
current patch has no current point, pattern will be put into an error status with a
status of Status::InvalidMeshConstruction.
<!-- impl Mesh::fn move_to -->
Define the first point of the current patch in a mesh pattern.

After this call the current point will be (x , y ).

Note: If pattern is not a mesh pattern then pattern will be put into an error status with
a status of Status::PatternTypeMismatch. If pattern has no current patch or the current
patch already has at least one side, pattern will be put into an error status with a status
of Status::InvalidMeshConstruction.
<!-- impl Mesh::fn line_to -->
Adds a line to the current patch from the current point to position (x , y ) in
pattern-space coordinates.

If there is no current point before the call to cairo_mesh_pattern_line_to() this function
will behave as Mesh::move_to(pattern , x , y ).

After this call the current point will be (x , y ).

Note: If pattern is not a mesh pattern then pattern will be put into an error status with
a status of Status::PatternTypeMismatch. If pattern has no current patch or the current
patch already has 4 sides, pattern will be put into an error status with a status of
Status::InvalidMeshConstruction.
<!-- impl Mesh::fn curve_to -->
Adds a cubic Bézier spline to the current patch from the current point to position
(x3 , y3 ) in pattern-space coordinates, using (x1 , y1 ) and (x2 , y2 ) as the control
points.

If the current patch has no current point before the call to Mesh::curve_to(), this
function will behave as if preceded by a call to Mesh::move_to(pattern , x1 , y1 ).

After this call the current point will be (x3 , y3 ).

Note: If pattern is not a mesh pattern then pattern will be put into an error status with
a status of Status::PatternTypeMismatch. If pattern has no current patch or the current
patch already has 4 sides, pattern will be put into an error status with a status of
Status::InvalidMeshConstruction.
<!-- impl Mesh::fn set_control_point -->
Set an internal control point of the current patch.

Valid values for point_num are from 0 to 3 and identify the control points as explained in
Mesh::new().

Note: If pattern is not a mesh pattern then pattern will be put into an error status with a
status of Status::PatternTypeMismatch. If point_num is not valid, pattern will be put into
an error status with a status of Status::InvalidIndex. If pattern has no current patch,
pattern will be put into an error status with a status of Status::InvalidMeshConstruction.
<!-- impl Mesh::fn get_control_point -->
Gets the control point point_num of patch patch_num for a mesh pattern.

patch_num can range from 0 to n-1 where n is the number returned by Mesh::get_patch_count().

Valid values for point_num are from 0 to 3 and identify the control points as explained
in Mesh::new().
<!-- impl Mesh::fn set_corner_color_rgb -->
Sets the color of a corner of the current patch in a mesh pattern.

The color is specified in the same way as in Context::set_source_rgb().

Valid values for corner_num are from 0 to 3 and identify the corners as explained in
Mesh::new().

Note: If pattern is not a mesh pattern then pattern will be put into an error status
with a status of Status::PatternTypeMismatch. If corner_num is not valid, pattern will
be put into an error status with a status of Status::InvalidIndex. If pattern has no
current patch, pattern will be put into an error status with a status of
Status::InvalidMeshConstruction.
<!-- impl Mesh::fn set_corner_color_rgba -->
Sets the color of a corner of the current patch in a mesh pattern.

The color is specified in the same way as in Context::set_source_rgba().

Valid values for corner_num are from 0 to 3 and identify the corners as explained
in Mesh::new().

Note: If pattern is not a mesh pattern then pattern will be put into an error status with a
status of Status::PatternTypeMismatch. If corner_num is not valid, pattern will be put into
an error status with a status of Status::InvalidIndex. If pattern has no current patch,
pattern will be put into an error status with a status of Status::InvalidMeshConstruction.
<!-- impl Mesh::fn get_corner_color_rgba -->
Gets the color information in corner corner_num of patch patch_num for a mesh pattern.

patch_num can range from 0 to n-1 where n is the number returned by Mesh::get_patch_count().

Valid values for corner_num are from 0 to 3 and identify the corners as explained in
Mesh::new().
<!-- impl Mesh::fn get_patch_count -->
Gets the number of patches specified in the given mesh pattern.

The number only includes patches which have been finished by calling Mesh::end_patch().
For example it will be 0 during the definition of the first patch.
<!-- impl Mesh::fn get_path -->
Gets path defining the patch patch_num for a mesh pattern.

patch_num can range from 0 to n-1 where n is the number returned by Mesh::get_patch_count().
<!-- file pdf.rs -->
<!-- struct File -->
A PDF surface that writes to a file
<!-- impl File::fn new -->
Create a new PDF file surface.

```
use cairo::Context;
use cairo::pdf;
use cairo::prelude::*;

let surface = pdf::File::new(100.0, 100.0, "test.pdf");
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- impl File::fn restrict -->
Specify what PDF version to generate.
<!-- struct Writer -->
A PDF surface that writes to a generic `io::Write` type (owning variant)

The `Writer` takes ownership of the write object.
Once you're done using the surface, you can obtain the write object
back using the `finish()` method.

If you would like the surface to reference the write object
instead, use `RefWriter`.
<!-- fn new -->
Create a new writer surface.

```
use std::fs::File;

use cairo::Context;
use cairo::pdf;
use cairo::prelude::*;

let surface = pdf::Writer::new(100.0, 100.0, File::create("test.pdf").unwrap());
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- fn restrict -->
Specify what PDF version to generate.
<!-- struct Stream -->
Streaming PDF surface.
<!-- fn new -->
Create a new streaming surface.

```
use std::fs::File;
use std::io::Write;

use cairo::Context;
use cairo::pdf;
use cairo::prelude::*;

let mut file = File::create("test.pdf").unwrap();
let surface = pdf::Writer::new(100.0, 100.0, file);
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- fn restrict -->
Specify what PDF version to generate.
<!-- struct Writer -->
A PDF surface that writes to a generic `io::Write` type (owning variant)

The `Writer` takes ownership of the write object.
Once you're done using the surface, you can obtain the write object
back using the `finish()` method.

If you would like the surface to reference the write object
instead, use `RefWriter`.
<!-- struct RefWriter -->
A PDF surface that writes to a generic `io::Write` type (referencing variant)

The `RefWriter` references the write object,
which is why a lifetime parameter is required.

If you would like the surface to own the write object
instead, use `Writer`.
<!-- file ps.rs -->
<!-- struct File -->
A PostScript surface that writes to a file
<!-- impl File::fn new -->
Create a new PostScript file surface.

```
use cairo::Context;
use cairo::ps;
use cairo::prelude::*;

let surface = ps::File::new(100.0, 100.0, "test.ps");
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- impl File::fn restrict -->
Specify what PostScript level to generate.
<!-- struct Writer -->
A PostScript surface that writes to a generic `io::Write` type (owning variant)

The `Writer` takes ownership of the write object.
Once you're done using the surface, you can obtain the write object
back using the `finish()` method.

If you would like the surface to reference the write object
instead, use `RefWriter`.
<!-- fn new -->
Create a new writer surface.

```
use std::fs::File;

use cairo::Context;
use cairo::ps;
use cairo::prelude::*;

let surface = ps::Writer::new(100.0, 100.0, File::create("test.ps").unwrap());
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- fn restrict -->
Specify what PostScript level to generate.
<!-- struct Stream -->
Streaming PDF surface.
<!-- fn new -->
Create a new streaming surface.

```
use std::fs::File;
use std::io::Write;

use cairo::Context;
use cairo::ps;
use cairo::prelude::*;

let mut file = File::create("test.ps").unwrap();
let surface = ps::Writer::new(100.0, 100.0, file);
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- fn restrict -->
Specify what PostScript level to generate.
<!-- struct RefWriter -->
A PostScript surface that writes to a generic `io::Write` type (referencing variant)

The `RefWriter` references the write object,
which is why a lifetime parameter is required.

If you would like the surface to own the write object
instead, use `Writer`.
<!-- file support.rs -->
<!-- struct Stream -->
Handles dark magic to maintain a stream closure based surface.

The closure is boxed twice so it can be passed around as a `*mut c_void`,
and it's then converted back into an usable trait object by removing the lifetime.

This seems to be okay because the closure is alive as long as the surface is.
<!-- struct Writer -->
Uses the `Stream` abstraction to implement streaming on a `T: io::Write`,
nothing fancy going on here.
<!-- struct Buffer -->
Uses the `Stream` abstraction to implement streaming to a buffer bound to
the surface.

The `Vec<u8>` is actually kept around as a `*mut Vec<u8>` since the closure
will be alive as long as the vector.
<!-- file svg.rs -->
<!-- struct File -->
An SVG surface that writes to a file
<!-- impl File::fn new -->
Create a new SVG file surface.

```
use cairo::Context;
use cairo::svg;
use cairo::prelude::*;

let surface = svg::File::new(100.0, 100.0, "test.svg");
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- impl File::fn restrict -->
Specify what SVG version to generate.
<!-- struct Writer -->
An SVG surface that writes to a generic `io::Write` type (owning variant)

The `Writer` takes ownership of the write object.
Once you're done using the surface, you can obtain the write object
back using the `finish()` method.

If you would like the surface to reference the write object
instead, use `RefWriter`.
<!-- fn new -->
Create a new writer surface.

```
use std::fs::File;

use cairo::Context;
use cairo::svg;
use cairo::prelude::*;

let surface = svg::Writer::new(100.0, 100.0, File::create("test.svg").unwrap());
let context = Context::new(&surface);

// Draw things on the context.

surface.finish();
```
<!-- fn restrict -->
Specify what SVG version to generate.
<!-- struct RefWriter -->
An SVG surface that writes to a generic `io::Write` type (referencing variant)

The `RefWriter` references the write object,
which is why a lifetime parameter is required.

If you would like the surface to own the write object
instead, use `Writer`.
