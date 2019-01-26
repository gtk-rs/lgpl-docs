<!-- file * -->
<!-- enum Colorspace -->
This enumeration defines the color spaces that are supported by
the gdk-pixbuf library. Currently only RGB is supported.
<!-- enum Colorspace::variant Rgb -->
Indicates a red/green/blue additive color space.
<!-- enum InterpType -->
This enumeration describes the different interpolation modes that
 can be used with the scaling functions. `InterpType::Nearest` is
 the fastest scaling method, but has horrible quality when
 scaling down. `InterpType::Bilinear` is the best choice if you
 aren't sure what to choose, it has a good speed/quality balance.

 `<note>`
    Cubic filtering is missing from the list; hyperbolic
    interpolation is just as fast and results in higher quality.
 `</note>`
<!-- enum InterpType::variant Nearest -->
Nearest neighbor sampling; this is the fastest
 and lowest quality mode. Quality is normally unacceptable when scaling
 down, but may be OK when scaling up.
<!-- enum InterpType::variant Tiles -->
This is an accurate simulation of the PostScript
 image operator without any interpolation enabled. Each pixel is
 rendered as a tiny parallelogram of solid color, the edges of which
 are implemented with antialiasing. It resembles nearest neighbor for
 enlargement, and bilinear for reduction.
<!-- enum InterpType::variant Bilinear -->
Best quality/speed balance; use this mode by
 default. Bilinear interpolation. For enlargement, it is
 equivalent to point-sampling the ideal bilinear-interpolated image.
 For reduction, it is equivalent to laying down small tiles and
 integrating over the coverage area.
<!-- enum InterpType::variant Hyper -->
This is the slowest and highest quality
 reconstruction function. It is derived from the hyperbolic filters in
 Wolberg's "Digital Image Warping", and is formally defined as the
 hyperbolic-filter sampling the ideal hyperbolic-filter interpolated
 image (the filter is designed to be idempotent for 1:1 pixel mapping).
<!-- struct Pixbuf -->
This is the main structure in the gdk-pixbuf library. It is
used to represent images. It contains information about the
image's pixel data, its color space, bits per sample, width and
height, and the rowstride (the number of bytes between the start of
one row and the start of the next).

# Implements

[`gio::IconExt`](../gio/trait.IconExt.html), [`gio::LoadableIconExt`](../gio/trait.LoadableIconExt.html)
<!-- impl Pixbuf::fn new -->
Creates a new `Pixbuf` structure and allocates a buffer for it. The
buffer has an optimal rowstride. Note that the buffer is not cleared;
you will have to fill it completely yourself.
## `colorspace`
Color space for image
## `has_alpha`
Whether the image should have transparency information
## `bits_per_sample`
Number of bits per color sample
## `width`
Width of image in pixels, must be > 0
## `height`
Height of image in pixels, must be > 0

# Returns

A newly-created `Pixbuf` with a reference count of 1, or
`None` if not enough memory could be allocated for the image buffer.
<!-- impl Pixbuf::fn new_from_bytes -->
Creates a new `Pixbuf` out of in-memory readonly image data.
Currently only RGB images with 8 bits per sample are supported.
This is the `glib::Bytes` variant of `Pixbuf::new_from_data`.

Feature: `v2_32`

## `data`
Image data in 8-bit/sample packed format inside a `glib::Bytes`
## `colorspace`
Colorspace for the image data
## `has_alpha`
Whether the data has an opacity channel
## `bits_per_sample`
Number of bits per sample
## `width`
Width of the image in pixels, must be > 0
## `height`
Height of the image in pixels, must be > 0
## `rowstride`
Distance in bytes between row starts

# Returns

A newly-created `Pixbuf` structure with a reference count of 1.
<!-- impl Pixbuf::fn new_from_data -->
Creates a new `Pixbuf` out of in-memory image data. Currently only RGB
images with 8 bits per sample are supported.

Since you are providing a pre-allocated pixel buffer, you must also
specify a way to free that data. This is done with a function of
type ``GdkPixbufDestroyNotify``. When a pixbuf created with is
finalized, your destroy notification function will be called, and
it is its responsibility to free the pixel array.

See also `Pixbuf::new_from_bytes`.
## `data`
Image data in 8-bit/sample packed format
## `colorspace`
Colorspace for the image data
## `has_alpha`
Whether the data has an opacity channel
## `bits_per_sample`
Number of bits per sample
## `width`
Width of the image in pixels, must be > 0
## `height`
Height of the image in pixels, must be > 0
## `rowstride`
Distance in bytes between row starts
## `destroy_fn`
Function used to free the data when the pixbuf's reference count
drops to zero, or `None` if the data should not be freed
## `destroy_fn_data`
Closure data to pass to the destroy notification function

# Returns

A newly-created `Pixbuf` structure with a reference count of 1.
<!-- impl Pixbuf::fn new_from_file -->
Creates a new pixbuf by loading an image from a file. The file format is
detected automatically. If `None` is returned, then `error` will be set.
Possible errors are in the `GDK_PIXBUF_ERROR` and `G_FILE_ERROR` domains.
## `filename`
Name of file to load, in the GLib file name encoding

# Returns

A newly-created pixbuf with a reference count of 1, or `None` if
any of several error conditions occurred: the file could not be opened,
there was no loader for the file's format, there was not enough memory to
allocate the image buffer, or the image file contained invalid data.
<!-- impl Pixbuf::fn new_from_file_at_scale -->
Creates a new pixbuf by loading an image from a file. The file format is
detected automatically. If `None` is returned, then `error` will be set.
Possible errors are in the `GDK_PIXBUF_ERROR` and `G_FILE_ERROR` domains.
The image will be scaled to fit in the requested size, optionally preserving
the image's aspect ratio.

When preserving the aspect ratio, a `width` of -1 will cause the image
to be scaled to the exact given height, and a `height` of -1 will cause
the image to be scaled to the exact given width. When not preserving
aspect ratio, a `width` or `height` of -1 means to not scale the image
at all in that dimension. Negative values for `width` and `height` are
allowed since 2.8.
## `filename`
Name of file to load, in the GLib file name encoding
## `width`
The width the image should have or -1 to not constrain the width
## `height`
The height the image should have or -1 to not constrain the height
## `preserve_aspect_ratio`
`true` to preserve the image's aspect ratio

# Returns

A newly-created pixbuf with a reference count of 1, or `None`
if any of several error conditions occurred: the file could not be opened,
there was no loader for the file's format, there was not enough memory to
allocate the image buffer, or the image file contained invalid data.
<!-- impl Pixbuf::fn new_from_file_at_size -->
Creates a new pixbuf by loading an image from a file.
The file format is detected automatically. If `None` is returned, then
`error` will be set. Possible errors are in the `GDK_PIXBUF_ERROR` and
`G_FILE_ERROR` domains.

The image will be scaled to fit in the requested size, preserving
the image's aspect ratio. Note that the returned pixbuf may be smaller
than `width` x `height`, if the aspect ratio requires it. To load
and image at the requested size, regardless of aspect ratio, use
`Pixbuf::new_from_file_at_scale`.
## `filename`
Name of file to load, in the GLib file name encoding
## `width`
The width the image should have or -1 to not constrain the width
## `height`
The height the image should have or -1 to not constrain the height

# Returns

A newly-created pixbuf with a reference count of 1, or
`None` if any of several error conditions occurred: the file could not
be opened, there was no loader for the file's format, there was not
enough memory to allocate the image buffer, or the image file contained
invalid data.
<!-- impl Pixbuf::fn new_from_inline -->
Create a `Pixbuf` from a flat representation that is suitable for
storing as inline data in a program. This is useful if you want to
ship a program with images, but don't want to depend on any
external files.

gdk-pixbuf ships with a program called [gdk-pixbuf-csource][gdk-pixbuf-csource],
which allows for conversion of ``GdkPixbufs`` into such a inline representation.
In almost all cases, you should pass the `--raw` option to
`gdk-pixbuf-csource`. A sample invocation would be:


```text
 gdk-pixbuf-csource --raw --name=myimage_inline myimage.png
```

For the typical case where the inline pixbuf is read-only static data,
you don't need to copy the pixel data unless you intend to write to
it, so you can pass `false` for `copy_pixels`. (If you pass `--rle` to
`gdk-pixbuf-csource`, a copy will be made even if `copy_pixels` is `false`,
so using this option is generally a bad idea.)

If you create a pixbuf from const inline data compiled into your
program, it's probably safe to ignore errors and disable length checks,
since things will always succeed:

```text
pixbuf = gdk_pixbuf_new_from_inline (-1, myimage_inline, FALSE, NULL);
```

For non-const inline data, you could get out of memory. For untrusted
inline data located at runtime, you could have corrupt inline data in
addition.

# Deprecated since 2.32

Use `gio::Resource` instead.
## `data_length`
Length in bytes of the `data` argument or -1 to
 disable length checks
## `data`
Byte data containing a
 serialized `Pixdata` structure
## `copy_pixels`
Whether to copy the pixel data, or use direct pointers
 `data` for the resulting pixbuf

# Returns

A newly-created `Pixbuf` structure with a reference,
 count of 1, or `None` if an error occurred.
<!-- impl Pixbuf::fn new_from_resource -->
Creates a new pixbuf by loading an image from an resource.

The file format is detected automatically. If `None` is returned, then
`error` will be set.
## `resource_path`
the path of the resource file

# Returns

A newly-created pixbuf, or `None` if any of several error
conditions occurred: the file could not be opened, the image format is
not supported, there was not enough memory to allocate the image buffer,
the stream contained invalid data, or the operation was cancelled.
<!-- impl Pixbuf::fn new_from_resource_at_scale -->
Creates a new pixbuf by loading an image from an resource.

The file format is detected automatically. If `None` is returned, then
`error` will be set.

The image will be scaled to fit in the requested size, optionally
preserving the image's aspect ratio. When preserving the aspect ratio,
a `width` of -1 will cause the image to be scaled to the exact given
height, and a `height` of -1 will cause the image to be scaled to the
exact given width. When not preserving aspect ratio, a `width` or
`height` of -1 means to not scale the image at all in that dimension.

The stream is not closed.
## `resource_path`
the path of the resource file
## `width`
The width the image should have or -1 to not constrain the width
## `height`
The height the image should have or -1 to not constrain the height
## `preserve_aspect_ratio`
`true` to preserve the image's aspect ratio

# Returns

A newly-created pixbuf, or `None` if any of several error
conditions occurred: the file could not be opened, the image format is
not supported, there was not enough memory to allocate the image buffer,
the stream contained invalid data, or the operation was cancelled.
<!-- impl Pixbuf::fn new_from_stream -->
Creates a new pixbuf by loading an image from an input stream.

The file format is detected automatically. If `None` is returned, then
`error` will be set. The `cancellable` can be used to abort the operation
from another thread. If the operation was cancelled, the error
`gio::IOErrorEnum::Cancelled` will be returned. Other possible errors are in
the `GDK_PIXBUF_ERROR` and `G_IO_ERROR` domains.

The stream is not closed.
## `stream`
a `gio::InputStream` to load the pixbuf from
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore

# Returns

A newly-created pixbuf, or `None` if any of several error
conditions occurred: the file could not be opened, the image format is
not supported, there was not enough memory to allocate the image buffer,
the stream contained invalid data, or the operation was cancelled.
<!-- impl Pixbuf::fn new_from_stream_at_scale -->
Creates a new pixbuf by loading an image from an input stream.

The file format is detected automatically. If `None` is returned, then
`error` will be set. The `cancellable` can be used to abort the operation
from another thread. If the operation was cancelled, the error
`gio::IOErrorEnum::Cancelled` will be returned. Other possible errors are in
the `GDK_PIXBUF_ERROR` and `G_IO_ERROR` domains.

The image will be scaled to fit in the requested size, optionally
preserving the image's aspect ratio.

When preserving the aspect ratio, a `width` of -1 will cause the image to be
scaled to the exact given height, and a `height` of -1 will cause the image
to be scaled to the exact given width. If both `width` and `height` are
given, this function will behave as if the smaller of the two values
is passed as -1.

When not preserving aspect ratio, a `width` or `height` of -1 means to not
scale the image at all in that dimension.

The stream is not closed.
## `stream`
a `gio::InputStream` to load the pixbuf from
## `width`
The width the image should have or -1 to not constrain the width
## `height`
The height the image should have or -1 to not constrain the height
## `preserve_aspect_ratio`
`true` to preserve the image's aspect ratio
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore

# Returns

A newly-created pixbuf, or `None` if any of several error
conditions occurred: the file could not be opened, the image format is
not supported, there was not enough memory to allocate the image buffer,
the stream contained invalid data, or the operation was cancelled.
<!-- impl Pixbuf::fn new_from_stream_finish -->
Finishes an asynchronous pixbuf creation operation started with
`Pixbuf::new_from_stream_async`.
## `async_result`
a `gio::AsyncResult`

# Returns

a `Pixbuf` or `None` on error. Free the returned
object with `gobject::Object::unref`.
<!-- impl Pixbuf::fn new_from_xpm_data -->
Creates a new pixbuf by parsing XPM data in memory. This data is commonly
the result of including an XPM file into a program's C source.
## `data`
Pointer to inline XPM data.

# Returns

A newly-created pixbuf with a reference count of 1.
<!-- impl Pixbuf::fn calculate_rowstride -->
Calculates the rowstride that an image created with those values would
have. This is useful for front-ends and backends that want to sanity
check image values without needing to create them.

Feature: `v2_36_8`

## `colorspace`
Color space for image
## `has_alpha`
Whether the image should have transparency information
## `bits_per_sample`
Number of bits per color sample
## `width`
Width of image in pixels, must be > 0
## `height`
Height of image in pixels, must be > 0

# Returns

the rowstride for the given values, or -1 in case of error.
<!-- impl Pixbuf::fn from_pixdata -->
Converts a `Pixdata` to a `Pixbuf`. If `copy_pixels` is `true` or
if the pixel data is run-length-encoded, the pixel data is copied into
newly-allocated memory; otherwise it is reused.

# Deprecated since 2.32

Use `gio::Resource` instead.
## `pixdata`
a `Pixdata` to convert into a `Pixbuf`.
## `copy_pixels`
whether to copy raw pixel data; run-length encoded
 pixel data is always copied.

# Returns

a new `Pixbuf`.
<!-- impl Pixbuf::fn get_file_info -->
Parses an image file far enough to determine its format and size.
## `filename`
The name of the file to identify.
## `width`
Return location for the width of the
 image, or `None`
## `height`
Return location for the height of the
 image, or `None`

# Returns

A `PixbufFormat` describing
 the image format of the file or `None` if the image format wasn't
 recognized. The return value is owned by `Pixbuf` and should
 not be freed.
<!-- impl Pixbuf::fn get_file_info_async -->
Asynchronously parses an image file far enough to determine its
format and size.

For more details see `Pixbuf::get_file_info`, which is the synchronous
version of this function.

When the operation is finished, `callback` will be called in the
main thread. You can then call `Pixbuf::get_file_info_finish` to
get the result of the operation.

Feature: `v2_32`

## `filename`
The name of the file to identify
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the file info is available
## `user_data`
the data to pass to the callback function
<!-- impl Pixbuf::fn get_file_info_finish -->
Finishes an asynchronous pixbuf parsing operation started with
`Pixbuf::get_file_info_async`.

Feature: `v2_32`

## `async_result`
a `gio::AsyncResult`
## `width`
Return location for the width of the image, or `None`
## `height`
Return location for the height of the image, or `None`

# Returns

A `PixbufFormat` describing the image
 format of the file or `None` if the image format wasn't
 recognized. The return value is owned by `Pixbuf` and should
 not be freed.
<!-- impl Pixbuf::fn get_formats -->
Obtains the available information about the image formats supported
by `Pixbuf`.

# Returns

A list of
``GdkPixbufFormats`` describing the supported image formats. The list should
be freed when it is no longer needed, but the structures themselves are
owned by `Pixbuf` and should not be freed.
<!-- impl Pixbuf::fn new_from_stream_async -->
Creates a new pixbuf by asynchronously loading an image from an input stream.

For more details see `Pixbuf::new_from_stream`, which is the synchronous
version of this function.

When the operation is finished, `callback` will be called in the main thread.
You can then call `Pixbuf::new_from_stream_finish` to get the result of the operation.
## `stream`
a `gio::InputStream` from which to load the pixbuf
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the pixbuf is loaded
## `user_data`
the data to pass to the callback function
<!-- impl Pixbuf::fn new_from_stream_at_scale_async -->
Creates a new pixbuf by asynchronously loading an image from an input stream.

For more details see `Pixbuf::new_from_stream_at_scale`, which is the synchronous
version of this function.

When the operation is finished, `callback` will be called in the main thread.
You can then call `Pixbuf::new_from_stream_finish` to get the result of the operation.
## `stream`
a `gio::InputStream` from which to load the pixbuf
## `width`
the width the image should have or -1 to not constrain the width
## `height`
the height the image should have or -1 to not constrain the height
## `preserve_aspect_ratio`
`true` to preserve the image's aspect ratio
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the pixbuf is loaded
## `user_data`
the data to pass to the callback function
<!-- impl Pixbuf::fn save_to_stream_finish -->
Finishes an asynchronous pixbuf save operation started with
`Pixbuf::save_to_stream_async`.
## `async_result`
a `gio::AsyncResult`

# Returns

`true` if the pixbuf was saved successfully, `false` if an error was set.
<!-- impl Pixbuf::fn add_alpha -->
Takes an existing pixbuf and adds an alpha channel to it.
If the existing pixbuf already had an alpha channel, the channel
values are copied from the original; otherwise, the alpha channel
is initialized to 255 (full opacity).

If `substitute_color` is `true`, then the color specified by (`r`, `g`, `b`) will be
assigned zero opacity. That is, if you pass (255, 255, 255) for the
substitute color, all white pixels will become fully transparent.
## `substitute_color`
Whether to set a color to zero opacity. If this
is `false`, then the (`r`, `g`, `b`) arguments will be ignored.
## `r`
Red value to substitute.
## `g`
Green value to substitute.
## `b`
Blue value to substitute.

# Returns

A newly-created pixbuf with a reference count of 1.
<!-- impl Pixbuf::fn apply_embedded_orientation -->
Takes an existing pixbuf and checks for the presence of an
associated "orientation" option, which may be provided by the
jpeg loader (which reads the exif orientation tag) or the
tiff loader (which reads the tiff orientation tag, and
compensates it for the partial transforms performed by
libtiff). If an orientation option/tag is present, the
appropriate transform will be performed so that the pixbuf
is oriented correctly.

# Returns

A newly-created pixbuf, `None` if
not enough memory could be allocated for it, or a reference to the
input pixbuf (with an increased reference count).
<!-- impl Pixbuf::fn composite -->
Creates a transformation of the source image `self` by scaling by
`scale_x` and `scale_y` then translating by `offset_x` and `offset_y`.
This gives an image in the coordinates of the destination pixbuf.
The rectangle (`dest_x`, `dest_y`, `dest_width`, `dest_height`)
is then alpha blended onto the corresponding rectangle of the
original destination image.

When the destination rectangle contains parts not in the source
image, the data at the edges of the source image is replicated
to infinity.

![](composite.png)
## `dest`
the `Pixbuf` into which to render the results
## `dest_x`
the left coordinate for region to render
## `dest_y`
the top coordinate for region to render
## `dest_width`
the width of the region to render
## `dest_height`
the height of the region to render
## `offset_x`
the offset in the X direction (currently rounded to an integer)
## `offset_y`
the offset in the Y direction (currently rounded to an integer)
## `scale_x`
the scale factor in the X direction
## `scale_y`
the scale factor in the Y direction
## `interp_type`
the interpolation type for the transformation.
## `overall_alpha`
overall alpha for source image (0..255)
<!-- impl Pixbuf::fn composite_color -->
Creates a transformation of the source image `self` by scaling by
`scale_x` and `scale_y` then translating by `offset_x` and `offset_y`,
then alpha blends the rectangle (`dest_x` ,`dest_y`, `dest_width`,
`dest_height`) of the resulting image with a checkboard of the
colors `color1` and `color2` and renders it onto the destination
image.

If the source image has no alpha channel, and `overall_alpha` is 255, a fast
path is used which omits the alpha blending and just performs the scaling.

See `Pixbuf::composite_color_simple` for a simpler variant of this
function suitable for many tasks.
## `dest`
the `Pixbuf` into which to render the results
## `dest_x`
the left coordinate for region to render
## `dest_y`
the top coordinate for region to render
## `dest_width`
the width of the region to render
## `dest_height`
the height of the region to render
## `offset_x`
the offset in the X direction (currently rounded to an integer)
## `offset_y`
the offset in the Y direction (currently rounded to an integer)
## `scale_x`
the scale factor in the X direction
## `scale_y`
the scale factor in the Y direction
## `interp_type`
the interpolation type for the transformation.
## `overall_alpha`
overall alpha for source image (0..255)
## `check_x`
the X offset for the checkboard (origin of checkboard is at -`check_x`, -`check_y`)
## `check_y`
the Y offset for the checkboard
## `check_size`
the size of checks in the checkboard (must be a power of two)
## `color1`
the color of check at upper left
## `color2`
the color of the other check
<!-- impl Pixbuf::fn composite_color_simple -->
Creates a new `Pixbuf` by scaling `self` to `dest_width` x
`dest_height` and alpha blending the result with a checkboard of colors
`color1` and `color2`.
## `dest_width`
the width of destination image
## `dest_height`
the height of destination image
## `interp_type`
the interpolation type for the transformation.
## `overall_alpha`
overall alpha for source image (0..255)
## `check_size`
the size of checks in the checkboard (must be a power of two)
## `color1`
the color of check at upper left
## `color2`
the color of the other check

# Returns

the new `Pixbuf`, or `None` if not enough memory could be
allocated for it.
<!-- impl Pixbuf::fn copy -->
Creates a new `Pixbuf` with a copy of the information in the specified
`self`. Note that this does not copy the options set on the original `Pixbuf`,
use `Pixbuf::copy_options` for this.

# Returns

A newly-created pixbuf with a reference count of 1, or `None` if
not enough memory could be allocated.
<!-- impl Pixbuf::fn copy_area -->
Copies a rectangular area from `self` to `dest_pixbuf`. Conversion of
pixbuf formats is done automatically.

If the source rectangle overlaps the destination rectangle on the
same pixbuf, it will be overwritten during the copy operation.
Therefore, you can not use this function to scroll a pixbuf.
## `src_x`
Source X coordinate within `self`.
## `src_y`
Source Y coordinate within `self`.
## `width`
Width of the area to copy.
## `height`
Height of the area to copy.
## `dest_pixbuf`
Destination pixbuf.
## `dest_x`
X coordinate within `dest_pixbuf`.
## `dest_y`
Y coordinate within `dest_pixbuf`.
<!-- impl Pixbuf::fn copy_options -->
Copy the key/value pair options attached to a `Pixbuf` to another.
This is useful to keep original metadata after having manipulated
a file. However be careful to remove metadata which you've already
applied, such as the "orientation" option after rotating the image.

Feature: `v2_36`

## `dest_pixbuf`
the `Pixbuf` to copy options to

# Returns

`true` on success.
<!-- impl Pixbuf::fn fill -->
Clears a pixbuf to the given RGBA value, converting the RGBA value into
the pixbuf's pixel format. The alpha will be ignored if the pixbuf
doesn't have an alpha channel.
## `pixel`
RGBA pixel to clear to
 (0xffffffff is opaque white, 0x00000000 transparent black)
<!-- impl Pixbuf::fn flip -->
Flips a pixbuf horizontally or vertically and returns the
result in a new pixbuf.
## `horizontal`
`true` to flip horizontally, `false` to flip vertically

# Returns

the new `Pixbuf`, or `None`
if not enough memory could be allocated for it.
<!-- impl Pixbuf::fn get_bits_per_sample -->
Queries the number of bits per color sample in a pixbuf.

# Returns

Number of bits per color sample.
<!-- impl Pixbuf::fn get_byte_length -->
Returns the length of the pixel data, in bytes.

# Returns

The length of the pixel data.
<!-- impl Pixbuf::fn get_colorspace -->
Queries the color space of a pixbuf.

# Returns

Color space.
<!-- impl Pixbuf::fn get_has_alpha -->
Queries whether a pixbuf has an alpha channel (opacity information).

# Returns

`true` if it has an alpha channel, `false` otherwise.
<!-- impl Pixbuf::fn get_height -->
Queries the height of a pixbuf.

# Returns

Height in pixels.
<!-- impl Pixbuf::fn get_n_channels -->
Queries the number of channels of a pixbuf.

# Returns

Number of channels.
<!-- impl Pixbuf::fn get_option -->
Looks up `key` in the list of options that may have been attached to the
`self` when it was loaded, or that may have been attached by another
function using `Pixbuf::set_option`.

For instance, the ANI loader provides "Title" and "Artist" options.
The ICO, XBM, and XPM loaders provide "x_hot" and "y_hot" hot-spot
options for cursor definitions. The PNG loader provides the tEXt ancillary
chunk key/value pairs as options. Since 2.12, the TIFF and JPEG loaders
return an "orientation" option string that corresponds to the embedded
TIFF/Exif orientation tag (if present). Since 2.32, the TIFF loader sets
the "multipage" option string to "yes" when a multi-page TIFF is loaded.
Since 2.32 the JPEG and PNG loaders set "x-dpi" and "y-dpi" if the file
contains image density information in dots per inch.
Since 2.36.6, the JPEG loader sets the "comment" option with the comment
EXIF tag.
## `key`
a nul-terminated string.

# Returns

the value associated with `key`. This is a nul-terminated
string that should not be freed or `None` if `key` was not found.
<!-- impl Pixbuf::fn get_options -->
Returns a `glib::HashTable` with a list of all the options that may have been
attached to the `self` when it was loaded, or that may have been
attached by another function using `Pixbuf::set_option`.

See `Pixbuf::get_option` for more details.

Feature: `v2_32`


# Returns

a `glib::HashTable` of key/values
<!-- impl Pixbuf::fn get_pixels -->
Queries a pointer to the pixel data of a pixbuf.

# Returns

A pointer to the pixbuf's pixel data.
Please see the section on [image data](image-data) for information
about how the pixel data is stored in memory.

This function will cause an implicit copy of the pixbuf data if the
pixbuf was created from read-only data.
<!-- impl Pixbuf::fn get_pixels_with_length -->
Queries a pointer to the pixel data of a pixbuf.
## `length`
The length of the binary data.

# Returns

A pointer to the pixbuf's
pixel data. Please see the section on [image data](image-data)
for information about how the pixel data is stored in memory.

This function will cause an implicit copy of the pixbuf data if the
pixbuf was created from read-only data.
<!-- impl Pixbuf::fn get_rowstride -->
Queries the rowstride of a pixbuf, which is the number of bytes between
the start of a row and the start of the next row.

# Returns

Distance between row starts.
<!-- impl Pixbuf::fn get_width -->
Queries the width of a pixbuf.

# Returns

Width in pixels.
<!-- impl Pixbuf::fn new_subpixbuf -->
Creates a new pixbuf which represents a sub-region of `self`.
The new pixbuf shares its pixels with the original pixbuf, so
writing to one affects both. The new pixbuf holds a reference to
`self`, so `self` will not be finalized until the new
pixbuf is finalized.

Note that if `self` is read-only, this function will force it
to be mutable.
## `src_x`
X coord in `self`
## `src_y`
Y coord in `self`
## `width`
width of region in `self`
## `height`
height of region in `self`

# Returns

a new pixbuf
<!-- impl Pixbuf::fn read_pixel_bytes -->

Feature: `v2_32`


# Returns

A new reference to a read-only copy of
the pixel data. Note that for mutable pixbufs, this function will
incur a one-time copy of the pixel data for conversion into the
returned `glib::Bytes`.
<!-- impl Pixbuf::fn read_pixels -->
Returns a read-only pointer to the raw pixel data; must not be
modified. This function allows skipping the implicit copy that
must be made if `Pixbuf::get_pixels` is called on a read-only
pixbuf.

Feature: `v2_32`

<!-- impl Pixbuf::fn ref -->
Adds a reference to a pixbuf.

# Deprecated since 2.0

Use `gobject::Object::ref`.

# Returns

The same as the `self` argument.
<!-- impl Pixbuf::fn remove_option -->
Remove the key/value pair option attached to a `Pixbuf`.

Feature: `v2_36`

## `key`
a nul-terminated string representing the key to remove.

# Returns

`true` if an option was removed, `false` if not.
<!-- impl Pixbuf::fn rotate_simple -->
Rotates a pixbuf by a multiple of 90 degrees, and returns the
result in a new pixbuf.

If `angle` is 0, a copy of `self` is returned, avoiding any rotation.
## `angle`
the angle to rotate by

# Returns

the new `Pixbuf`, or `None`
if not enough memory could be allocated for it.
<!-- impl Pixbuf::fn saturate_and_pixelate -->
Modifies saturation and optionally pixelates `self`, placing the result in
`dest`. `self` and `dest` may be the same pixbuf with no ill effects. If
`saturation` is 1.0 then saturation is not changed. If it's less than 1.0,
saturation is reduced (the image turns toward grayscale); if greater than
1.0, saturation is increased (the image gets more vivid colors). If `pixelate`
is `true`, then pixels are faded in a checkerboard pattern to create a
pixelated image. `self` and `dest` must have the same image format, size, and
rowstride.
## `dest`
place to write modified version of `self`
## `saturation`
saturation factor
## `pixelate`
whether to pixelate
<!-- impl Pixbuf::fn save -->
Saves pixbuf to a file in format `type_`. By default, "jpeg", "png", "ico"
and "bmp" are possible file formats to save in, but more formats may be
installed. The list of all writable formats can be determined in the
following way:


```text
void add_if_writable (GdkPixbufFormat *data, GSList **list)
{
  if (gdk_pixbuf_format_is_writable (data))
    *list = g_slist_prepend (*list, data);
}

GSList *formats = gdk_pixbuf_get_formats ();
GSList *writable_formats = NULL;
g_slist_foreach (formats, add_if_writable, &writable_formats);
g_slist_free (formats);
```

If `error` is set, `false` will be returned. Possible errors include
those in the `GDK_PIXBUF_ERROR` domain and those in the `G_FILE_ERROR` domain.

The variable argument list should be `None`-terminated; if not empty,
it should contain pairs of strings that modify the save
parameters. For example:

```text
gdk_pixbuf_save (pixbuf, handle, "jpeg", &error, "quality", "100", NULL);
```

Currently only few parameters exist. JPEG images can be saved with a
"quality" parameter; its value should be in the range [0,100]. JPEG
and PNG density can be set by setting the "x-dpi" and "y-dpi" parameters
to the appropriate values in dots per inch.

Text chunks can be attached to PNG images by specifying parameters of
the form "tEXt::key", where key is an ASCII string of length 1-79.
The values are UTF-8 encoded strings. The PNG compression level can
be specified using the "compression" parameter; it's value is in an
integer in the range of [0,9].

ICC color profiles can also be embedded into PNG, JPEG and TIFF images.
The "icc-profile" value should be the complete ICC profile encoded
into base64.


```text
gchar *contents;
gchar *contents_encode;
gsize length;
g_file_get_contents ("/home/hughsie/.color/icc/L225W.icm", &contents, &length, NULL);
contents_encode = g_base64_encode ((const guchar *) contents, length);
gdk_pixbuf_save (pixbuf, handle, "png", &error, "icc-profile", contents_encode, NULL);
```

TIFF images recognize: (1) a "bits-per-sample" option (integer) which
can be either 1 for saving bi-level CCITTFAX4 images, or 8 for saving
8-bits per sample; (2) a "compression" option (integer) which can be
1 for no compression, 2 for Huffman, 5 for LZW, 7 for JPEG and 8 for
DEFLATE (see the libtiff documentation and tiff.h for all supported
codec values); (3) an "icc-profile" option (zero-terminated string)
containing a base64 encoded ICC color profile.

ICO images can be saved in depth 16, 24, or 32, by using the "depth"
parameter. When the ICO saver is given "x_hot" and "y_hot" parameters,
it produces a CUR instead of an ICO.
## `filename`
name of file to save.
## `type_`
name of file format.
## `error`
return location for error, or `None`

# Returns

whether an error was set
<!-- impl Pixbuf::fn save_to_buffer -->
Saves pixbuf to a new buffer in format `type_`, which is currently "jpeg",
"png", "tiff", "ico" or "bmp". This is a convenience function that uses
`Pixbuf::save_to_callback` to do the real work. Note that the buffer
is not nul-terminated and may contain embedded nuls.
If `error` is set, `false` will be returned and `buffer` will be set to
`None`. Possible errors include those in the `GDK_PIXBUF_ERROR`
domain.

See `Pixbuf::save` for more details.
## `buffer`
location to receive a pointer
 to the new buffer.
## `buffer_size`
location to receive the size of the new buffer.
## `type_`
name of file format.
## `error`
return location for error, or `None`

# Returns

whether an error was set
<!-- impl Pixbuf::fn save_to_bufferv -->
Saves pixbuf to a new buffer in format `type_`, which is currently "jpeg",
"tiff", "png", "ico" or "bmp". See `Pixbuf::save_to_buffer`
for more details.
## `buffer`

 location to receive a pointer to the new buffer.
## `buffer_size`
location to receive the size of the new buffer.
## `type_`
name of file format.
## `option_keys`
name of options to set, `None`-terminated
## `option_values`
values for named options

# Returns

whether an error was set
<!-- impl Pixbuf::fn save_to_callback -->
Saves pixbuf in format `type_` by feeding the produced data to a
callback. Can be used when you want to store the image to something
other than a file, such as an in-memory buffer or a socket.
If `error` is set, `false` will be returned. Possible errors
include those in the `GDK_PIXBUF_ERROR` domain and whatever the save
function generates.

See `Pixbuf::save` for more details.
## `save_func`
a function that is called to save each block of data that
 the save routine generates.
## `user_data`
user data to pass to the save function.
## `type_`
name of file format.
## `error`
return location for error, or `None`

# Returns

whether an error was set
<!-- impl Pixbuf::fn save_to_callbackv -->
Saves pixbuf to a callback in format `type_`, which is currently "jpeg",
"png", "tiff", "ico" or "bmp". If `error` is set, `false` will be returned. See
gdk_pixbuf_save_to_callback () for more details.
## `save_func`
a function that is called to save each block of data that
 the save routine generates.
## `user_data`
user data to pass to the save function.
## `type_`
name of file format.
## `option_keys`
name of options to set, `None`-terminated
## `option_values`
values for named options

# Returns

whether an error was set
<!-- impl Pixbuf::fn save_to_stream -->
Saves `self` to an output stream.

Supported file formats are currently "jpeg", "tiff", "png", "ico" or
"bmp". See `Pixbuf::save_to_buffer` for more details.

The `cancellable` can be used to abort the operation from another
thread. If the operation was cancelled, the error `gio::IOErrorEnum::Cancelled`
will be returned. Other possible errors are in the `GDK_PIXBUF_ERROR`
and `G_IO_ERROR` domains.

The stream is not closed.
## `stream`
a `gio::OutputStream` to save the pixbuf to
## `type_`
name of file format
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `error`
return location for error, or `None`

# Returns

`true` if the pixbuf was saved successfully, `false` if an
 error was set.
<!-- impl Pixbuf::fn save_to_stream_async -->
Saves `self` to an output stream asynchronously.

For more details see `Pixbuf::save_to_stream`, which is the synchronous
version of this function.

When the operation is finished, `callback` will be called in the main thread.
You can then call `Pixbuf::save_to_stream_finish` to get the result of the operation.
## `stream`
a `gio::OutputStream` to which to save the pixbuf
## `type_`
name of file format
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the pixbuf is saved
## `user_data`
the data to pass to the callback function
<!-- impl Pixbuf::fn save_to_streamv -->
Saves `self` to an output stream.

Supported file formats are currently "jpeg", "tiff", "png", "ico" or
"bmp". See `Pixbuf::save_to_stream` for more details.

Feature: `v2_36`

## `stream`
a `gio::OutputStream` to save the pixbuf to
## `type_`
name of file format
## `option_keys`
name of options to set, `None`-terminated
## `option_values`
values for named options
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore

# Returns

`true` if the pixbuf was saved successfully, `false` if an
 error was set.
<!-- impl Pixbuf::fn save_to_streamv_async -->
Saves `self` to an output stream asynchronously.

For more details see `Pixbuf::save_to_streamv`, which is the synchronous
version of this function.

When the operation is finished, `callback` will be called in the main thread.
You can then call `Pixbuf::save_to_stream_finish` to get the result of the operation.

Feature: `v2_36`

## `stream`
a `gio::OutputStream` to which to save the pixbuf
## `type_`
name of file format
## `option_keys`
name of options to set, `None`-terminated
## `option_values`
values for named options
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the pixbuf is saved
## `user_data`
the data to pass to the callback function
<!-- impl Pixbuf::fn savev -->
Saves pixbuf to a file in `type_`, which is currently "jpeg", "png", "tiff", "ico" or "bmp".
If `error` is set, `false` will be returned.
See gdk_pixbuf_save () for more details.
## `filename`
name of file to save.
## `type_`
name of file format.
## `option_keys`
name of options to set, `None`-terminated
## `option_values`
values for named options

# Returns

whether an error was set
<!-- impl Pixbuf::fn scale -->
Creates a transformation of the source image `self` by scaling by
`scale_x` and `scale_y` then translating by `offset_x` and `offset_y`,
then renders the rectangle (`dest_x`, `dest_y`, `dest_width`,
`dest_height`) of the resulting image onto the destination image
replacing the previous contents.

Try to use `Pixbuf::scale_simple` first, this function is
the industrial-strength power tool you can fall back to if
`Pixbuf::scale_simple` isn't powerful enough.

If the source rectangle overlaps the destination rectangle on the
same pixbuf, it will be overwritten during the scaling which
results in rendering artifacts.
## `dest`
the `Pixbuf` into which to render the results
## `dest_x`
the left coordinate for region to render
## `dest_y`
the top coordinate for region to render
## `dest_width`
the width of the region to render
## `dest_height`
the height of the region to render
## `offset_x`
the offset in the X direction (currently rounded to an integer)
## `offset_y`
the offset in the Y direction (currently rounded to an integer)
## `scale_x`
the scale factor in the X direction
## `scale_y`
the scale factor in the Y direction
## `interp_type`
the interpolation type for the transformation.
<!-- impl Pixbuf::fn scale_simple -->
Create a new `Pixbuf` containing a copy of `self` scaled to
`dest_width` x `dest_height`. Leaves `self` unaffected. `interp_type`
should be `InterpType::Nearest` if you want maximum speed (but when
scaling down `InterpType::Nearest` is usually unusably ugly). The
default `interp_type` should be `InterpType::Bilinear` which offers
reasonable quality and speed.

You can scale a sub-portion of `self` by creating a sub-pixbuf
pointing into `self`; see `Pixbuf::new_subpixbuf`.

If `dest_width` and `dest_height` are equal to the `self` width and height, a
copy of `self` is returned, avoiding any scaling.

For more complicated scaling/alpha blending see `Pixbuf::scale`
and `Pixbuf::composite`.
## `dest_width`
the width of destination image
## `dest_height`
the height of destination image
## `interp_type`
the interpolation type for the transformation.

# Returns

the new `Pixbuf`, or `None` if not enough memory could be
allocated for it.
<!-- impl Pixbuf::fn set_option -->
Attaches a key/value pair as an option to a `Pixbuf`. If `key` already
exists in the list of options attached to `self`, the new value is
ignored and `false` is returned.
## `key`
a nul-terminated string.
## `value`
a nul-terminated string.

# Returns

`true` on success.
<!-- impl Pixbuf::fn unref -->
Removes a reference from a pixbuf.

# Deprecated since 2.0

Use `gobject::Object::unref`.
<!-- trait PixbufExt::fn get_property_bits-per-sample -->
The number of bits per sample.
Currently only 8 bit per sample are supported.
<!-- trait PixbufExt::fn set_property_bits-per-sample -->
The number of bits per sample.
Currently only 8 bit per sample are supported.
<!-- trait PixbufExt::fn get_property_n-channels -->
The number of samples per pixel.
Currently, only 3 or 4 samples per pixel are supported.
<!-- trait PixbufExt::fn set_property_n-channels -->
The number of samples per pixel.
Currently, only 3 or 4 samples per pixel are supported.
<!-- trait PixbufExt::fn get_property_rowstride -->
The number of bytes between the start of a row and
the start of the next row. This number must (obviously)
be at least as large as the width of the pixbuf.
<!-- trait PixbufExt::fn set_property_rowstride -->
The number of bytes between the start of a row and
the start of the next row. This number must (obviously)
be at least as large as the width of the pixbuf.
<!-- enum PixbufAlphaMode -->
These values can be passed to
`gdk_pixbuf_xlib_render_to_drawable_alpha` to control how the alpha
channel of an image should be handled. This function can create a
bilevel clipping mask (black and white) and use it while painting
the image. In the future, when the X Window System gets an alpha
channel extension, it will be possible to do full alpha
compositing onto arbitrary drawables. For now both cases fall
back to a bilevel clipping mask.
<!-- enum PixbufAlphaMode::variant Bilevel -->
A bilevel clipping mask (black and white)
 will be created and used to draw the image. Pixels below 0.5 opacity
 will be considered fully transparent, and all others will be
 considered fully opaque.
<!-- enum PixbufAlphaMode::variant Full -->
For now falls back to `PixbufAlphaMode::Bilevel`.
 In the future it will do full alpha compositing.
<!-- struct PixbufAnimation -->
An opaque struct representing an animation.

# Implements

[`PixbufAnimationExt`](trait.PixbufAnimationExt.html)
<!-- trait PixbufAnimationExt -->
Trait containing all `PixbufAnimation` methods.

# Implementors

[`PixbufAnimation`](struct.PixbufAnimation.html), [`PixbufSimpleAnim`](struct.PixbufSimpleAnim.html)
<!-- impl PixbufAnimation::fn new_from_file -->
Creates a new animation by loading it from a file. The file format is
detected automatically. If the file's format does not support multi-frame
images, then an animation with a single frame will be created. Possible errors
are in the `GDK_PIXBUF_ERROR` and `G_FILE_ERROR` domains.
## `filename`
Name of file to load, in the GLib file name encoding

# Returns

A newly-created animation with a reference count of 1, or `None`
if any of several error conditions ocurred: the file could not be opened,
there was no loader for the file's format, there was not enough memory to
allocate the image buffer, or the image file contained invalid data.
<!-- impl PixbufAnimation::fn new_from_resource -->
Creates a new pixbuf animation by loading an image from an resource.

The file format is detected automatically. If `None` is returned, then
`error` will be set.

Feature: `v2_28`

## `resource_path`
the path of the resource file

# Returns

A newly-created animation, or `None` if any of several error
conditions occurred: the file could not be opened, the image format is
not supported, there was not enough memory to allocate the image buffer,
the stream contained invalid data, or the operation was cancelled.
<!-- impl PixbufAnimation::fn new_from_stream -->
Creates a new animation by loading it from an input stream.

The file format is detected automatically. If `None` is returned, then
`error` will be set. The `cancellable` can be used to abort the operation
from another thread. If the operation was cancelled, the error
`gio::IOErrorEnum::Cancelled` will be returned. Other possible errors are in
the `GDK_PIXBUF_ERROR` and `G_IO_ERROR` domains.

The stream is not closed.

Feature: `v2_28`

## `stream`
a `gio::InputStream` to load the pixbuf from
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore

# Returns

A newly-created pixbuf, or `None` if any of several error
conditions occurred: the file could not be opened, the image format is
not supported, there was not enough memory to allocate the image buffer,
the stream contained invalid data, or the operation was cancelled.
<!-- impl PixbufAnimation::fn new_from_stream_finish -->
Finishes an asynchronous pixbuf animation creation operation started with
`PixbufAnimation::new_from_stream_async`.

Feature: `v2_28`

## `async_result`
a `gio::AsyncResult`

# Returns

a `PixbufAnimation` or `None` on error. Free the returned
object with `gobject::Object::unref`.
<!-- impl PixbufAnimation::fn new_from_stream_async -->
Creates a new animation by asynchronously loading an image from an input stream.

For more details see `Pixbuf::new_from_stream`, which is the synchronous
version of this function.

When the operation is finished, `callback` will be called in the main thread.
You can then call `PixbufAnimation::new_from_stream_finish` to get the
result of the operation.

Feature: `v2_28`

## `stream`
a `gio::InputStream` from which to load the animation
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the pixbuf is loaded
## `user_data`
the data to pass to the callback function
<!-- trait PixbufAnimationExt::fn get_height -->
Queries the height of the bounding box of a pixbuf animation.

# Returns

Height of the bounding box of the animation.
<!-- trait PixbufAnimationExt::fn get_iter -->
Get an iterator for displaying an animation. The iterator provides
the frames that should be displayed at a given time. It should be
freed after use with `gobject::Object::unref`.

`start_time` would normally come from `g_get_current_time`, and marks
the beginning of animation playback. After creating an iterator, you
should immediately display the pixbuf returned by
`PixbufAnimationIterExt::get_pixbuf`. Then, you should install
a timeout (with `g_timeout_add`) or by some other mechanism ensure
that you'll update the image after
`PixbufAnimationIterExt::get_delay_time` milliseconds. Each time
the image is updated, you should reinstall the timeout with the new,
possibly-changed delay time.

As a shortcut, if `start_time` is `None`, the result of
`g_get_current_time` will be used automatically.

To update the image (i.e. possibly change the result of
`PixbufAnimationIterExt::get_pixbuf` to a new frame of the animation),
call `PixbufAnimationIterExt::advance`.

If you're using `PixbufLoader`, in addition to updating the image
after the delay time, you should also update it whenever you
receive the area_updated signal and
`PixbufAnimationIterExt::on_currently_loading_frame` returns
`true`. In this case, the frame currently being fed into the loader
has received new data, so needs to be refreshed. The delay time for
a frame may also be modified after an area_updated signal, for
example if the delay time for a frame is encoded in the data after
the frame itself. So your timeout should be reinstalled after any
area_updated signal.

A delay time of -1 is possible, indicating "infinite."
## `start_time`
time when the animation starts playing

# Returns

an iterator to move over the animation
<!-- trait PixbufAnimationExt::fn get_static_image -->
If an animation is really just a plain image (has only one frame),
this function returns that image. If the animation is an animation,
this function returns a reasonable thing to display as a static
unanimated image, which might be the first frame, or something more
sophisticated. If an animation hasn't loaded any frames yet, this
function will return `None`.

# Returns

unanimated image representing the animation
<!-- trait PixbufAnimationExt::fn get_width -->
Queries the width of the bounding box of a pixbuf animation.

# Returns

Width of the bounding box of the animation.
<!-- trait PixbufAnimationExt::fn is_static_image -->
If you load a file with `PixbufAnimation::new_from_file` and it
turns out to be a plain, unanimated image, then this function will
return `true`. Use `PixbufAnimationExt::get_static_image` to retrieve
the image.

# Returns

`true` if the "animation" was really just an image
<!-- trait PixbufAnimationExt::fn ref -->
Adds a reference to an animation.

# Deprecated since 2.0

Use `gobject::Object::ref`.

# Returns

The same as the `self` argument.
<!-- trait PixbufAnimationExt::fn unref -->
Removes a reference from an animation.

# Deprecated since 2.0

Use `gobject::Object::unref`.
<!-- struct PixbufAnimationIter -->
An opaque struct representing an iterator which points to a
certain position in an animation.

# Implements

[`PixbufAnimationIterExt`](trait.PixbufAnimationIterExt.html)
<!-- trait PixbufAnimationIterExt -->
Trait containing all `PixbufAnimationIter` methods.

# Implementors

[`PixbufAnimationIter`](struct.PixbufAnimationIter.html)
<!-- trait PixbufAnimationIterExt::fn advance -->
Possibly advances an animation to a new frame. Chooses the frame based
on the start time passed to `PixbufAnimationExt::get_iter`.

`current_time` would normally come from `g_get_current_time`, and
must be greater than or equal to the time passed to
`PixbufAnimationExt::get_iter`, and must increase or remain
unchanged each time `PixbufAnimationIterExt::get_pixbuf` is
called. That is, you can't go backward in time; animations only
play forward.

As a shortcut, pass `None` for the current time and `g_get_current_time`
will be invoked on your behalf. So you only need to explicitly pass
`current_time` if you're doing something odd like playing the animation
at double speed.

If this function returns `false`, there's no need to update the animation
display, assuming the display had been rendered prior to advancing;
if `true`, you need to call `PixbufAnimationIterExt::get_pixbuf`
and update the display with the new pixbuf.
## `current_time`
current time

# Returns

`true` if the image may need updating
<!-- trait PixbufAnimationIterExt::fn get_delay_time -->
Gets the number of milliseconds the current pixbuf should be displayed,
or -1 if the current pixbuf should be displayed forever. `g_timeout_add`
conveniently takes a timeout in milliseconds, so you can use a timeout
to schedule the next update.

Note that some formats, like GIF, might clamp the timeout values in the
image file to avoid updates that are just too quick. The minimum timeout
for GIF images is currently 20 milliseconds.

# Returns

delay time in milliseconds (thousandths of a second)
<!-- trait PixbufAnimationIterExt::fn get_pixbuf -->
Gets the current pixbuf which should be displayed; the pixbuf might not
be the same size as the animation itself
(`PixbufAnimationExt::get_width`, `PixbufAnimationExt::get_height`).
This pixbuf should be displayed for
`PixbufAnimationIterExt::get_delay_time` milliseconds. The caller
of this function does not own a reference to the returned pixbuf;
the returned pixbuf will become invalid when the iterator advances
to the next frame, which may happen anytime you call
`PixbufAnimationIterExt::advance`. Copy the pixbuf to keep it
(don't just add a reference), as it may get recycled as you advance
the iterator.

# Returns

the pixbuf to be displayed
<!-- trait PixbufAnimationIterExt::fn on_currently_loading_frame -->
Used to determine how to respond to the area_updated signal on
`PixbufLoader` when loading an animation. area_updated is emitted
for an area of the frame currently streaming in to the loader. So if
you're on the currently loading frame, you need to redraw the screen for
the updated area.

# Returns

`true` if the frame we're on is partially loaded, or the last frame
<!-- enum PixbufError -->
An error code in the `GDK_PIXBUF_ERROR` domain. Many gdk-pixbuf
operations can cause errors in this domain, or in the `G_FILE_ERROR`
domain.
<!-- enum PixbufError::variant CorruptImage -->
An image file was broken somehow.
<!-- enum PixbufError::variant InsufficientMemory -->
Not enough memory.
<!-- enum PixbufError::variant BadOption -->
A bad option was passed to a pixbuf save module.
<!-- enum PixbufError::variant UnknownType -->
Unknown image type.
<!-- enum PixbufError::variant UnsupportedOperation -->
Don't know how to perform the
 given operation on the type of image at hand.
<!-- enum PixbufError::variant Failed -->
Generic failure code, something went wrong.
<!-- enum PixbufError::variant IncompleteAnimation -->
Only part of the animation was loaded.
<!-- struct PixbufFormat -->
<!-- impl PixbufFormat::fn copy -->
Creates a copy of `self`

# Returns

the newly allocated copy of a `PixbufFormat`. Use
 `PixbufFormat::free` to free the resources when done
<!-- impl PixbufFormat::fn free -->
Frees the resources allocated when copying a `PixbufFormat`
using `PixbufFormat::copy`
<!-- impl PixbufFormat::fn get_description -->
Returns a description of the format.

# Returns

a description of the format.
<!-- impl PixbufFormat::fn get_extensions -->
Returns the filename extensions typically used for files in the
given format.

# Returns

a `None`-terminated array of filename extensions which must be
freed with `g_strfreev` when it is no longer needed.
<!-- impl PixbufFormat::fn get_license -->
Returns information about the license of the image loader for the format. The
returned string should be a shorthand for a wellknown license, e.g. "LGPL",
"GPL", "QPL", "GPL/QPL", or "other" to indicate some other license. This
string should be freed with `g_free` when it's no longer needed.

# Returns

a string describing the license of `self`.
<!-- impl PixbufFormat::fn get_mime_types -->
Returns the mime types supported by the format.

# Returns

a `None`-terminated array of mime types which must be freed with
`g_strfreev` when it is no longer needed.
<!-- impl PixbufFormat::fn get_name -->
Returns the name of the format.

# Returns

the name of the format.
<!-- impl PixbufFormat::fn is_disabled -->
Returns whether this image format is disabled. See
`PixbufFormat::set_disabled`.

# Returns

whether this image format is disabled.
<!-- impl PixbufFormat::fn is_save_option_supported -->
Returns `true` if the save option specified by `option_key` is supported when
saving a pixbuf using the module implementing `self`.
See `Pixbuf::save` for more information about option keys.

Feature: `v2_36`

## `option_key`
the name of an option

# Returns

`true` if the specified option is supported
<!-- impl PixbufFormat::fn is_scalable -->
Returns whether this image format is scalable. If a file is in a
scalable format, it is preferable to load it at the desired size,
rather than loading it at the default size and scaling the
resulting pixbuf to the desired size.

# Returns

whether this image format is scalable.
<!-- impl PixbufFormat::fn is_writable -->
Returns whether pixbufs can be saved in the given format.

# Returns

whether pixbufs can be saved in the given format.
<!-- impl PixbufFormat::fn set_disabled -->
Disables or enables an image format. If a format is disabled,
gdk-pixbuf won't use the image loader for this format to load
images. Applications can use this to avoid using image loaders
with an inappropriate license, see `PixbufFormat::get_license`.
## `disabled`
`true` to disable the format `self`
<!-- struct PixbufLoader -->
The `PixbufLoader` struct contains only private
fields.

# Implements

[`PixbufLoaderExt`](trait.PixbufLoaderExt.html)
<!-- trait PixbufLoaderExt -->
Trait containing all `PixbufLoader` methods.

# Implementors

[`PixbufLoader`](struct.PixbufLoader.html)
<!-- impl PixbufLoader::fn new -->
Creates a new pixbuf loader object.

# Returns

A newly-created pixbuf loader.
<!-- impl PixbufLoader::fn new_with_mime_type -->
Creates a new pixbuf loader object that always attempts to parse
image data as if it were an image of mime type `mime_type`, instead of
identifying the type automatically. Useful if you want an error if
the image isn't the expected mime type, for loading image formats
that can't be reliably identified by looking at the data, or if
the user manually forces a specific mime type.

The list of supported mime types depends on what image loaders
are installed, but typically "image/png", "image/jpeg", "image/gif",
"image/tiff" and "image/x-xpixmap" are among the supported mime types.
To obtain the full list of supported mime types, call
`PixbufFormat::get_mime_types` on each of the `PixbufFormat`
structs returned by `Pixbuf::get_formats`.
## `mime_type`
the mime type to be loaded

# Returns

A newly-created pixbuf loader.
<!-- impl PixbufLoader::fn new_with_type -->
Creates a new pixbuf loader object that always attempts to parse
image data as if it were an image of type `image_type`, instead of
identifying the type automatically. Useful if you want an error if
the image isn't the expected type, for loading image formats
that can't be reliably identified by looking at the data, or if
the user manually forces a specific type.

The list of supported image formats depends on what image loaders
are installed, but typically "png", "jpeg", "gif", "tiff" and
"xpm" are among the supported formats. To obtain the full list of
supported image formats, call `PixbufFormat::get_name` on each
of the `PixbufFormat` structs returned by `Pixbuf::get_formats`.
## `image_type`
name of the image format to be loaded with the image

# Returns

A newly-created pixbuf loader.
<!-- trait PixbufLoaderExt::fn close -->
Informs a pixbuf loader that no further writes with
`PixbufLoaderExt::write` will occur, so that it can free its
internal loading structures. Also, tries to parse any data that
hasn't yet been parsed; if the remaining data is partial or
corrupt, an error will be returned. If `false` is returned, `error`
will be set to an error from the `GDK_PIXBUF_ERROR` or `G_FILE_ERROR`
domains. If you're just cancelling a load rather than expecting it
to be finished, passing `None` for `error` to ignore it is
reasonable.

Remember that this does not unref the loader, so if you plan not to
use it anymore, please `gobject::Object::unref` it.

# Returns

`true` if all image data written so far was successfully
 passed out via the update_area signal
<!-- trait PixbufLoaderExt::fn get_animation -->
Queries the `PixbufAnimation` that a pixbuf loader is currently creating.
In general it only makes sense to call this function after the "area-prepared"
signal has been emitted by the loader. If the loader doesn't have enough
bytes yet (hasn't emitted the "area-prepared" signal) this function will
return `None`.

# Returns

The `PixbufAnimation` that the loader is loading, or `None` if
not enough data has been read to determine the information.
<!-- trait PixbufLoaderExt::fn get_format -->
Obtains the available information about the format of the
currently loading image file.

# Returns

A `PixbufFormat` or
`None`. The return value is owned by `Pixbuf` and should not be
freed.
<!-- trait PixbufLoaderExt::fn get_pixbuf -->
Queries the `Pixbuf` that a pixbuf loader is currently creating.
In general it only makes sense to call this function after the
"area-prepared" signal has been emitted by the loader; this means
that enough data has been read to know the size of the image that
will be allocated. If the loader has not received enough data via
`PixbufLoaderExt::write`, then this function returns `None`. The
returned pixbuf will be the same in all future calls to the loader,
so simply calling `gobject::Object::ref` should be sufficient to continue
using it. Additionally, if the loader is an animation, it will
return the "static image" of the animation
(see `PixbufAnimationExt::get_static_image`).

# Returns

The `Pixbuf` that the loader is creating, or `None` if not
enough data has been read to determine how to create the image buffer.
<!-- trait PixbufLoaderExt::fn set_size -->
Causes the image to be scaled while it is loaded. The desired
image size can be determined relative to the original size of
the image by calling `PixbufLoaderExt::set_size` from a
signal handler for the ::size-prepared signal.

Attempts to set the desired image size are ignored after the
emission of the ::size-prepared signal.
## `width`
The desired width of the image being loaded.
## `height`
The desired height of the image being loaded.
<!-- trait PixbufLoaderExt::fn write -->
This will cause a pixbuf loader to parse the next `count` bytes of
an image. It will return `true` if the data was loaded successfully,
and `false` if an error occurred. In the latter case, the loader
will be closed, and will not accept further writes. If `false` is
returned, `error` will be set to an error from the `GDK_PIXBUF_ERROR`
or `G_FILE_ERROR` domains.
## `buf`
Pointer to image data.
## `count`
Length of the `buf` buffer in bytes.

# Returns

`true` if the write was successful, or `false` if the loader
cannot parse the buffer.
<!-- trait PixbufLoaderExt::fn write_bytes -->
This will cause a pixbuf loader to parse a buffer inside a `glib::Bytes`
for an image. It will return `true` if the data was loaded successfully,
and `false` if an error occurred. In the latter case, the loader
will be closed, and will not accept further writes. If `false` is
returned, `error` will be set to an error from the `GDK_PIXBUF_ERROR`
or `G_FILE_ERROR` domains.

See also: `PixbufLoaderExt::write`

Feature: `v2_30`

## `buffer`
The image data as a `glib::Bytes`

# Returns

`true` if the write was successful, or `false` if the loader
cannot parse the buffer.
<!-- trait PixbufLoaderExt::fn connect_area_prepared -->
This signal is emitted when the pixbuf loader has allocated the
pixbuf in the desired size. After this signal is emitted,
applications can call `PixbufLoaderExt::get_pixbuf` to fetch
the partially-loaded pixbuf.
<!-- trait PixbufLoaderExt::fn connect_area_updated -->
This signal is emitted when a significant area of the image being
loaded has been updated. Normally it means that a complete
scanline has been read in, but it could be a different area as
well. Applications can use this signal to know when to repaint
areas of an image that is being loaded.
## `x`
X offset of upper-left corner of the updated area.
## `y`
Y offset of upper-left corner of the updated area.
## `width`
Width of updated area.
## `height`
Height of updated area.
<!-- trait PixbufLoaderExt::fn connect_closed -->
This signal is emitted when `PixbufLoaderExt::close` is called.
It can be used by different parts of an application to receive
notification when an image loader is closed by the code that
drives it.
<!-- trait PixbufLoaderExt::fn connect_size_prepared -->
This signal is emitted when the pixbuf loader has been fed the
initial amount of data that is required to figure out the size
of the image that it will create. Applications can call
`PixbufLoaderExt::set_size` in response to this signal to set
the desired size to which the image should be scaled.
## `width`
the original width of the image
## `height`
the original height of the image
<!-- enum PixbufRotation -->
The possible rotations which can be passed to `Pixbuf::rotate_simple`.
To make them easier to use, their numerical values are the actual degrees.
<!-- enum PixbufRotation::variant None -->
No rotation.
<!-- enum PixbufRotation::variant Counterclockwise -->
Rotate by 90 degrees.
<!-- enum PixbufRotation::variant Upsidedown -->
Rotate by 180 degrees.
<!-- enum PixbufRotation::variant Clockwise -->
Rotate by 270 degrees.
<!-- struct PixbufSimpleAnim -->
An opaque struct representing a simple animation.

# Implements

[`PixbufSimpleAnimExt`](trait.PixbufSimpleAnimExt.html), [`PixbufAnimationExt`](trait.PixbufAnimationExt.html)
<!-- trait PixbufSimpleAnimExt -->
Trait containing all `PixbufSimpleAnim` methods.

# Implementors

[`PixbufSimpleAnim`](struct.PixbufSimpleAnim.html)
<!-- impl PixbufSimpleAnim::fn new -->
Creates a new, empty animation.
## `width`
the width of the animation
## `height`
the height of the animation
## `rate`
the speed of the animation, in frames per second

# Returns

a newly allocated `PixbufSimpleAnim`
<!-- trait PixbufSimpleAnimExt::fn add_frame -->
Adds a new frame to `self`. The `pixbuf` must
have the dimensions specified when the animation
was constructed.
## `pixbuf`
the pixbuf to add
<!-- trait PixbufSimpleAnimExt::fn get_loop -->
Gets whether `self` should loop indefinitely when it reaches the end.

# Returns

`true` if the animation loops forever, `false` otherwise
<!-- trait PixbufSimpleAnimExt::fn set_loop -->
Sets whether `self` should loop indefinitely when it reaches the end.
## `loop_`
whether to loop the animation
<!-- trait PixbufSimpleAnimExt::fn get_property_loop -->
Whether the animation should loop when it reaches the end.
<!-- trait PixbufSimpleAnimExt::fn set_property_loop -->
Whether the animation should loop when it reaches the end.
