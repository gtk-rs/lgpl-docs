<!-- file * -->
<!-- struct AppLaunchContext -->
`AppLaunchContext` is an implementation of `gio::AppLaunchContext` that
handles launching an application in a graphical context. It provides
startup notification and allows to launch applications on a specific
screen or workspace.

## Launching an application


```C
GdkAppLaunchContext *context;

context = gdk_display_get_app_launch_context (display);

gdk_app_launch_context_set_display (display);
gdk_app_launch_context_set_timestamp (event->time);

if (!g_app_info_launch_default_for_uri ("http://www.gtk.org", context, &error))
  g_warning ("Launching failed: %s\n", error->message);

g_object_unref (context);
```
<!-- impl AppLaunchContext::fn set_desktop -->
Sets the workspace on which applications will be launched when
using this context when running under a window manager that
supports multiple workspaces, as described in the
[Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec).

When the workspace is not specified or `desktop` is set to -1,
it is up to the window manager to pick one, typically it will
be the current workspace.
## `desktop`
the number of a workspace, or -1
<!-- impl AppLaunchContext::fn set_icon -->
Sets the icon for applications that are launched with this
context.

Window Managers can use this information when displaying startup
notification.

See also `AppLaunchContext::set_icon_name`.
## `icon`
a `gio::Icon`, or `None`
<!-- impl AppLaunchContext::fn set_icon_name -->
Sets the icon for applications that are launched with this context.
The `icon_name` will be interpreted in the same way as the Icon field
in desktop files. See also `AppLaunchContext::set_icon`.

If both `icon` and `icon_name` are set, the `icon_name` takes priority.
If neither `icon` or `icon_name` is set, the icon is taken from either
the file that is passed to launched application or from the `gio::AppInfo`
for the launched application itself.
## `icon_name`
an icon name, or `None`
<!-- impl AppLaunchContext::fn set_timestamp -->
Sets the timestamp of `self`. The timestamp should ideally
be taken from the event that triggered the launch.

Window managers can use this information to avoid moving the
focus to the newly launched application when the user is busy
typing in another window. This is also known as 'focus stealing
prevention'.
## `timestamp`
a timestamp
<!-- enum AxisUse -->
An enumeration describing the way in which a device
axis (valuator) maps onto the predefined valuator
types that GTK understands.

Note that the X and Y axes are not really needed; pointer devices
report their location via the x/y members of events regardless. Whether
X and Y are present as axes depends on the GDK backend.
<!-- enum AxisUse::variant Ignore -->
the axis is ignored.
<!-- enum AxisUse::variant X -->
the axis is used as the x axis.
<!-- enum AxisUse::variant Y -->
the axis is used as the y axis.
<!-- enum AxisUse::variant Pressure -->
the axis is used for pressure information.
<!-- enum AxisUse::variant Xtilt -->
the axis is used for x tilt information.
<!-- enum AxisUse::variant Ytilt -->
the axis is used for y tilt information.
<!-- enum AxisUse::variant Wheel -->
the axis is used for wheel information.
<!-- enum AxisUse::variant Distance -->
the axis is used for pen/tablet distance information
<!-- enum AxisUse::variant Rotation -->
the axis is used for pen rotation information
<!-- enum AxisUse::variant Slider -->
the axis is used for pen slider information
<!-- enum AxisUse::variant Last -->
a constant equal to the numerically highest axis value.
<!-- enum ByteOrder -->
A set of values describing the possible byte-orders
for storing pixel values in memory.
<!-- enum ByteOrder::variant LsbFirst -->
The values are stored with the least-significant byte
 first. For instance, the 32-bit value 0xffeecc would be stored
 in memory as 0xcc, 0xee, 0xff, 0x00.
<!-- enum ByteOrder::variant MsbFirst -->
The values are stored with the most-significant byte
 first. For instance, the 32-bit value 0xffeecc would be stored
 in memory as 0x00, 0xff, 0xee, 0xcc.
<!-- struct CairoContext -->
`CairoContext` is an object representing the platform-specific
draw context.

``GdkCairoContexts`` are created for a `Display` using
`SurfaceExt::create_cairo_context`, and the context can then be used
to draw on that `Surface`.

# Implements

[`DrawContextExt`](trait.DrawContextExt.html), [`DrawContextExtManual`](prelude/trait.DrawContextExtManual.html)
<!-- impl CairoContext::fn cairo_create -->
Retrieves a Cairo context to be used to draw on the `Surface`
of `context`. A call to `DrawContextExt::begin_frame` with this
`context` must have been done or this function will return `None`.

The returned context is guaranteed to be valid until
`DrawContextExt::end_frame` is called.

# Returns

a Cairo context to be used
 to draw the contents of the `Surface`. `None` is returned
 when `contet` is not drawing.
<!-- struct Clipboard -->
The `Clipboard` object represents a clipboard of data shared
between different applications or between different parts of
the same application.

To get a `Clipboard` object, use `Display::get_clipboard` or
`Display::get_primary_clipboard`. You can find out about the data that
is currently available in a clipboard using `Clipboard::get_formats`.

To make text or image data available in a clipboard, use `Clipboard::set_text` or
`Clipboard::set_texture`. For other data, you can use `Clipboard::set_content`,
which takes a `ContentProvider` object.

To read textual or image data from a clipboard, use `Clipboard::read_text_async` or
`Clipboard::read_texture_async`. For other data, use `Clipboard::read_async`,
which provides a `gio::InputStream` object.
<!-- impl Clipboard::fn get_content -->
Returns the `ContentProvider` currently set on `self`. If the
`self` is empty or its contents are not owned by the current process,
`None` will be returned.

# Returns

The content of a clipboard or `None`
 if the clipboard does not maintain any content.
<!-- impl Clipboard::fn get_display -->
Gets the `Display` that the clipboard was created for.

# Returns

a `Display`
<!-- impl Clipboard::fn get_formats -->
Gets the formats that the clipboard can provide its current contents in.

# Returns

The formats of the clipboard
<!-- impl Clipboard::fn is_local -->
Returns if the clipboard is local. A clipboard is considered local if it was
last claimed by the running application.

Note that `Clipboard::get_content` may return `None` even on a local
clipboard. In this case the clipboard is empty.

# Returns

`true` if the clipboard is local
<!-- impl Clipboard::fn read_async -->
Asynchronously requests an input stream to read the `self`'s
contents from. When the operation is finished `callback` will be called.
You can then call `Clipboard::read_finish` to get the result of the
operation.

The clipboard will choose the most suitable mime type from the given list
to fulfill the request, preferring the ones listed first.
## `mime_types`
a `None`-terminated array of mime types to choose from
## `io_priority`
the [I/O priority][io-priority]
of the request.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Clipboard::fn read_finish -->
Finishes an asynchronous clipboard read started with `Clipboard::read_async`.
## `result`
a `gio::AsyncResult`
## `out_mime_type`
pointer to store
 the chosen mime type in or `None`

# Returns

a `gio::InputStream` or `None` on error.
<!-- impl Clipboard::fn read_text_async -->
Asynchronously request the `self` contents converted to a string.
When the operation is finished `callback` will be called. You can then
call `Clipboard::read_text_finish` to get the result.

This is a simple wrapper around `Clipboard::read_value_async`. Use
that function or `Clipboard::read_async` directly if you need more
control over the operation.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Clipboard::fn read_text_finish -->
Finishes an asynchronous clipboard read started with
`Clipboard::read_text_async`.
## `result`
a `gio::AsyncResult`

# Returns

a new string or `None` on error.
<!-- impl Clipboard::fn read_texture_async -->
Asynchronously request the `self` contents converted to a `gdk_pixbuf::Pixbuf`.
When the operation is finished `callback` will be called. You can then
call `Clipboard::read_texture_finish` to get the result.

This is a simple wrapper around `Clipboard::read_value_async`. Use
that function or `Clipboard::read_async` directly if you need more
control over the operation.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Clipboard::fn read_texture_finish -->
Finishes an asynchronous clipboard read started with
`Clipboard::read_texture_async`.
## `result`
a `gio::AsyncResult`

# Returns

a new `Texture` or `None` on error.
<!-- impl Clipboard::fn read_value_async -->
Asynchronously request the `self` contents converted to the given
`type_`. When the operation is finished `callback` will be called.
You can then call `Clipboard::read_value_finish` to get the resulting
`gobject::Value`.

For local clipboard contents that are available in the given `glib::Type`, the
value will be copied directly. Otherwise, GDK will try to use
`gdk_content_deserialize_async` to convert the clipboard's data.
## `type_`
a `glib::Type` to read
## `io_priority`
the [I/O priority][io-priority]
 of the request.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Clipboard::fn read_value_finish -->
Finishes an asynchronous clipboard read started with
`Clipboard::read_value_async`.
## `result`
a `gio::AsyncResult`

# Returns

a `gobject::Value` containing the result.
<!-- impl Clipboard::fn set -->
Sets the clipboard to contain the value collected from the given
varargs.
## `type_`
type of value to set
<!-- impl Clipboard::fn set_content -->
Sets a new content provider on `self`. The clipboard will claim the
`Display`'s resources and advertise these new contents to other
applications.

In the rare case of a failure, this function will return `false`. The
clipboard will then continue reporting its old contents and ignore
`provider`.

If the contents are read by either an external application or the
`self`'s read functions, `self` will select the best format to
transfer the contents and then request that format from `provider`.
## `provider`
the new contents of `self` or
 `None` to clear the clipboard

# Returns

`true` if setting the clipboard succeeded
<!-- impl Clipboard::fn set_text -->
Puts the given `text` into the clipboard.
## `text`
Text to put into the clipboard
<!-- impl Clipboard::fn set_texture -->
Puts the given `texture` into the clipboard.
## `texture`
a `Texture` to put into the clipboard
<!-- impl Clipboard::fn set_valist -->
Sets the clipboard to contain the value collected from the given
`args`.
## `type_`
type of value to set
## `args`
varargs containing the value of `type_`
<!-- impl Clipboard::fn set_value -->
Sets the `self` to contain the given `value`.
## `value`
a `gobject::Value` to set
<!-- impl Clipboard::fn store_async -->
Asynchronously instructs the `self` to store its contents remotely to
preserve them for later usage. If the clipboard is not local, this function
does nothing but report success.

This function is called automatically when `gtk_main` or ``GtkApplication``
exit, so you likely don't need to call it.
## `io_priority`
the [I/O priority][io-priority]
 of the request.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Clipboard::fn store_finish -->
Finishes an asynchronous clipboard store started with `Clipboard::store_async`.
## `result`
a `gio::AsyncResult`

# Returns

`true` if storing was successful.
<!-- impl Clipboard::fn connect_changed -->
The ::changed signal is emitted when the clipboard changes ownership.
<!-- impl Clipboard::fn get_property_content -->
The `ContentProvider` or `None` if the clipboard is empty or contents are
provided otherwise.
<!-- impl Clipboard::fn get_property_display -->
The `Display` that the clipboard belongs to.
<!-- impl Clipboard::fn set_property_display -->
The `Display` that the clipboard belongs to.
<!-- impl Clipboard::fn get_property_formats -->
The possible formats that the clipboard can provide its data in.
<!-- impl Clipboard::fn get_property_local -->
`true` if the contents of the clipboard are owned by this process.
<!-- struct ContentDeserializer -->
A `ContentDeserializer` is used to deserialize content received via
inter-application data transfers.
<!-- impl ContentDeserializer::fn get_cancellable -->
Gets the cancellable that was passed to `gdk_content_deserialize_async`.

# Returns

the cancellable for the current operation
<!-- impl ContentDeserializer::fn get_gtype -->
Gets the GType to create an instance of.

# Returns

the GType for the current operation
<!-- impl ContentDeserializer::fn get_input_stream -->
Gets the input stream that was passed to `gdk_content_deserialize_async`.

# Returns

the input stream for the current operation
<!-- impl ContentDeserializer::fn get_mime_type -->
Gets the mime type to deserialize from.

# Returns

the mime type for the current operation
<!-- impl ContentDeserializer::fn get_priority -->
Gets the io priority that was passed to `gdk_content_deserialize_async`.

# Returns

the io priority for the current operation
<!-- impl ContentDeserializer::fn get_task_data -->
Gets the data that was associated with `self` via `ContentDeserializer::set_task_data`.

# Returns

the task data for `self`
<!-- impl ContentDeserializer::fn get_user_data -->
Gets the user data that was passed when the deserializer was registered.

# Returns

the user data for this deserializer
<!-- impl ContentDeserializer::fn get_value -->
Gets the `gobject::Value` to store the deserialized object in.

# Returns

the `gobject::Value` for the current operation
<!-- impl ContentDeserializer::fn return_error -->
Indicate that the deserialization has ended with an error.
This function consumes `error`.
## `error`
a `glib::Error`
<!-- impl ContentDeserializer::fn return_success -->
Indicate that the deserialization has been successfully completed.
<!-- impl ContentDeserializer::fn set_task_data -->
Associate data with the current deserialization operation.
## `data`
data to associate with this operation
## `notify`
destroy notify for `data`
<!-- struct ContentFormats -->
This section describes the `ContentFormats` structure that is used to
advertise and negotiate the format of content passed between different
widgets, windows or applications using for example the clipboard or
drag'n'drop.

GDK supports content in 2 forms: `glib::Type` and mime type.
Using `GTypes` is meant only for in-process content transfers. Mime types
are meant to be used for data passing both in-process and out-of-process.
The details of how data is passed is described in the documentation of
the actual implementations.

A `ContentFormats` describes a set of possible formats content can be
exchanged in. It is assumed that this set is ordered. `GTypes` are more
important than mime types. Order between different `Gtypes` or mime types
is the order they were added in, most important first. Functions that
care about order, such as `ContentFormats::union` will describe in
their documentation how they interpret that order, though in general the
order of the first argument is considered the primary order of the result,
followed by the order of further arguments.

For debugging purposes, the function `ContentFormats::to_string` exists.
It will print a comma-seperated formats of formats from most important to least
important.

`ContentFormats` is an immutable struct. After creation, you cannot change
the types it represents. Instead, new `ContentFormats` have to be created.
The `ContentFormatsBuilder` structure is meant to help in this endeavor.
<!-- impl ContentFormats::fn new -->
Creates a new `ContentFormats` from an array of mime types.

The mime types must be valid and different from each other or the
behavior of the return value is undefined. If you cannot guarantee
this, use `ContentFormatsBuilder` instead.
## `mime_types`
Pointer to an
 array of mime types
## `n_mime_types`
number of entries in `mime_types`.

# Returns

the new `ContentFormats`.
<!-- impl ContentFormats::fn new_for_gtype -->
Creates a new `ContentFormats` for a given `glib::Type`.
## `type_`
a $GType

# Returns

a new `ContentFormats`
<!-- impl ContentFormats::fn contain_gtype -->
Checks if a given `glib::Type` is part of the given `self`.
## `type_`
the `glib::Type` to search for

# Returns

`true` if the `glib::Type` was found
<!-- impl ContentFormats::fn contain_mime_type -->
Checks if a given mime type is part of the given `self`.
## `mime_type`
the mime type to search for

# Returns

`true` if the mime_type was found
<!-- impl ContentFormats::fn get_gtypes -->
Gets the `GTypes` included in `self`. Note that `self` may not
contain any `GTypes`, in particular when they are empty. In that
case `None` will be returned.
## `n_gtypes`
optional pointer to take the
 number of `GTypes` contained in the return value

# Returns

`G_TYPE_INVALID`-terminated array of
 types included in `self` or `None` if none.
<!-- impl ContentFormats::fn get_mime_types -->
Gets the mime types included in `self`. Note that `self` may not
contain any mime types, in particular when they are empty. In that
case `None` will be returned.
## `n_mime_types`
optional pointer to take the
 number of mime types contained in the return value

# Returns

`None`-terminated array of
 interned strings of mime types included in `self` or `None`
 if none.
<!-- impl ContentFormats::fn match -->
Checks if `self` and `second` have any matching formats.
## `second`
the `ContentFormats` to intersect with

# Returns

`true` if a matching format was found.
<!-- impl ContentFormats::fn match_gtype -->
Finds the first `glib::Type` from `self` that is also contained
in `second`. If no matching `glib::Type` is found, `G_TYPE_INVALID`
is returned.
## `second`
the `ContentFormats` to intersect with

# Returns

The first common `glib::Type` or `G_TYPE_INVALID` if none.
<!-- impl ContentFormats::fn match_mime_type -->
Finds the first mime type from `self` that is also contained
in `second`. If no matching mime type is found, `None` is
returned.
## `second`
the `ContentFormats` to intersect with

# Returns

The first common mime type or `None` if none.
<!-- impl ContentFormats::fn print -->
Prints the given `self` into a string for human consumption.
This is meant for debugging and logging.

The form of the representation may change at any time and is
not guranteed to stay identical.
## `string`
a `glib::String` to print into
<!-- impl ContentFormats::fn ref -->
Increases the reference count of a `ContentFormats` by one.

# Returns

the passed in `ContentFormats`.
<!-- impl ContentFormats::fn to_string -->
Prints the given `self` into a human-readable string.
This is a small wrapper around `ContentFormats::print` to help
when debugging.

# Returns

a new string
<!-- impl ContentFormats::fn union -->
Append all missing types from `second` to `self`, in the order
they had in `second`.
## `second`
the `ContentFormats` to merge from

# Returns

a new `ContentFormats`
<!-- impl ContentFormats::fn union_deserialize_gtypes -->
Add GTypes for mime types in `self` for which deserializers are
registered.

# Returns

a new `ContentFormats`
<!-- impl ContentFormats::fn union_deserialize_mime_types -->
Add mime types for GTypes in `self` for which deserializers are
registered.

# Returns

a new `ContentFormats`
<!-- impl ContentFormats::fn union_serialize_gtypes -->
Add GTypes for the mime types in `self` for which serializers are
registered.

# Returns

a new `ContentFormats`
<!-- impl ContentFormats::fn union_serialize_mime_types -->
Add mime types for GTypes in `self` for which serializers are
registered.

# Returns

a new `ContentFormats`
<!-- impl ContentFormats::fn unref -->
Decreases the reference count of a `ContentFormats` by one.
If the resulting reference count is zero, frees the formats.
<!-- struct ContentFormatsBuilder -->
A `ContentFormatsBuilder` struct is an opaque struct. It is meant to
not be kept around and only be used to create new `ContentFormats`
objects.
<!-- impl ContentFormatsBuilder::fn new -->
Create a new `ContentFormatsBuilder` object. The resulting builder
would create an empty `ContentFormats`. Use addition functions to add
types to it.

# Returns

a new `ContentFormatsBuilder`
<!-- impl ContentFormatsBuilder::fn add_formats -->
Appends all formats from `formats` to `self`, skipping those that
already exist.
## `formats`
the formats to add
<!-- impl ContentFormatsBuilder::fn add_gtype -->
Appends `gtype` to `self` if it has not already been added.
## `type_`
a `glib::Type`
<!-- impl ContentFormatsBuilder::fn add_mime_type -->
Appends `mime_type` to `self` if it has not already been added.
## `mime_type`
a mime type
<!-- impl ContentFormatsBuilder::fn free_to_formats -->
Creates a new `ContentFormats` from the current state of the
given `self`, and frees the `self` instance.

# Returns

the newly created `ContentFormats`
 with all the formats added to `self`
<!-- impl ContentFormatsBuilder::fn ref -->
Acquires a reference on the given `self`.

This function is intended primarily for bindings. `ContentFormatsBuilder` objects
should not be kept around.

# Returns

the given `ContentFormatsBuilder` with
 its reference count increased
<!-- impl ContentFormatsBuilder::fn to_formats -->
Creates a new `ContentFormats` from the given `self`.

The given `ContentFormatsBuilder` is reset once this function returns;
you cannot call this function multiple times on the same `self` instance.

This function is intended primarily for bindings. C code should use
`ContentFormatsBuilder::free_to_formats`.

# Returns

the newly created `ContentFormats`
 with all the formats added to `self`
<!-- impl ContentFormatsBuilder::fn unref -->
Releases a reference on the given `self`.
<!-- struct ContentProvider -->
A `ContentProvider` is used to provide content for the clipboard in
a number of formats.

To create a `ContentProvider`, use `ContentProvider::new_for_value` or
`ContentProvider::new_for_bytes`.

GDK knows how to handle common text and image formats out-of-the-box. See
`ContentSerializer` and `ContentDeserializer` if you want to add support
for application-specific data formats.

# Implements

[`ContentProviderExt`](trait.ContentProviderExt.html)
<!-- trait ContentProviderExt -->
Trait containing all `ContentProvider` methods.

# Implementors

[`ContentProvider`](struct.ContentProvider.html)
<!-- impl ContentProvider::fn new_for_bytes -->
Create a content provider that provides the given `bytes` as data for
the given `mime_type`.
## `mime_type`
the mime type
## `bytes`
a `glib::Bytes` with the data for `mime_type`

# Returns

a new `ContentProvider`
<!-- impl ContentProvider::fn new_for_value -->
Create a content provider that provides the given `value`.
## `value`
a `gobject::Value`

# Returns

a new `ContentProvider`
<!-- trait ContentProviderExt::fn content_changed -->
Emits the `ContentProvider::contents-changed` signal.
<!-- trait ContentProviderExt::fn get_value -->
Gets the convtents of `self` stored in `value`.

The `value` will have been initialized to the `glib::Type` the value should be
provided in. This given `glib::Type` does not need to be listed in the formats
returned by `ContentProviderExt::ref_formats`. However, if the given
`glib::Type` is not supported, this operation can fail and
`gio::IOErrorEnum::NotSupported` will be reported.
## `value`
the `gobject::Value` to fill

# Returns

`true` if the value was set successfully. Otherwise
 `error` will be set to describe the failure.
<!-- trait ContentProviderExt::fn ref_formats -->
Gets the formats that the provider can provide its current contents in.

# Returns

The formats of the provider
<!-- trait ContentProviderExt::fn ref_storable_formats -->
Gets the formats that the provider suggests other applications to store
the data in.
An example of such an application would be a clipboard manager.

This can be assumed to be a subset of `ContentProviderExt::ref_formats`.

# Returns

The storable formats of the provider
<!-- trait ContentProviderExt::fn write_mime_type_async -->
Asynchronously writes the contents of `self` to `stream` in the given
`mime_type`. When the operation is finished `callback` will be called. You
can then call `ContentProviderExt::write_mime_type_finish` to get the
result of the operation.

The given mime type does not need to be listed in the formats returned by
`ContentProviderExt::ref_formats`. However, if the given `glib::Type` is not
supported, `gio::IOErrorEnum::NotSupported` will be reported.

The given `stream` will not be closed.
## `mime_type`
the mime type to provide the data in
## `stream`
the `gio::OutputStream` to write to
## `io_priority`
the [I/O priority][io-priority]
of the request.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait ContentProviderExt::fn write_mime_type_finish -->
Finishes an asynchronous write operation started with
`ContentProviderExt::write_mime_type_async`.
## `result`
a `gio::AsyncResult`

# Returns

`true` if the operation was completed successfully. Otherwise
 `error` will be set to describe the failure.
<!-- trait ContentProviderExt::fn get_property_formats -->
The possible formats that the provider can provide its data in.
<!-- trait ContentProviderExt::fn get_property_storable_formats -->
The subset of formats that clipboard managers should store this provider's data in.
<!-- struct ContentSerializer -->
A `ContentSerializer` is used to serialize content for inter-application
data transfers.
<!-- impl ContentSerializer::fn get_cancellable -->
Gets the cancellable that was passed to `gdk_content_serialize_async`.

# Returns

the cancellable for the current operation
<!-- impl ContentSerializer::fn get_gtype -->
Gets the GType to of the object to serialize.

# Returns

the GType for the current operation
<!-- impl ContentSerializer::fn get_mime_type -->
Gets the mime type to serialize to.

# Returns

the mime type for the current operation
<!-- impl ContentSerializer::fn get_output_stream -->
Gets the output stream that was passed to `gdk_content_serialize_async`.

# Returns

the output stream for the current operation
<!-- impl ContentSerializer::fn get_priority -->
Gets the io priority that was passed to `gdk_content_serialize_async`.

# Returns

the io priority for the current operation
<!-- impl ContentSerializer::fn get_task_data -->
Gets the data that was associated with `self` via `ContentSerializer::set_task_data`.

# Returns

the task data for `self`
<!-- impl ContentSerializer::fn get_user_data -->
Gets the user data that was passed when the serializer was registered.

# Returns

the user data for this serializer
<!-- impl ContentSerializer::fn get_value -->
Gets the `gobject::Value` to read the object to serialize from.

# Returns

the `gobject::Value` for the current operation
<!-- impl ContentSerializer::fn return_error -->
Indicate that the serialization has ended with an error.
This function consumes `error`.
## `error`
a `glib::Error`
<!-- impl ContentSerializer::fn return_success -->
Indicate that the serialization has been successfully completed.
<!-- impl ContentSerializer::fn set_task_data -->
Associate data with the current serialization operation.
## `data`
data to associate with this operation
## `notify`
destroy notify for `data`
<!-- enum CrossingMode -->
Specifies the crossing mode for enter and leave events.
<!-- enum CrossingMode::variant Normal -->
crossing because of pointer motion.
<!-- enum CrossingMode::variant Grab -->
crossing because a grab is activated.
<!-- enum CrossingMode::variant Ungrab -->
crossing because a grab is deactivated.
<!-- enum CrossingMode::variant GtkGrab -->
crossing because a GTK+ grab is activated.
<!-- enum CrossingMode::variant GtkUngrab -->
crossing because a GTK+ grab is deactivated.
<!-- enum CrossingMode::variant StateChanged -->
crossing because a GTK+ widget changed
 state (e.g. sensitivity).
<!-- enum CrossingMode::variant TouchBegin -->
crossing because a touch sequence has begun,
 this event is synthetic as the pointer might have not left the surface.
<!-- enum CrossingMode::variant TouchEnd -->
crossing because a touch sequence has ended,
 this event is synthetic as the pointer might have not left the surface.
<!-- enum CrossingMode::variant DeviceSwitch -->
crossing because of a device switch (i.e.
 a mouse taking control of the pointer after a touch device), this event
 is synthetic as the pointer didn’t leave the surface.
<!-- struct Cursor -->
A `Cursor` represents a cursor. Its contents are private.

Cursors are immutable objects, so they can not change after
they have been constructed.
<!-- impl Cursor::fn new_from_name -->
Creates a new cursor by looking up `name` in the current cursor
theme.

A recommended set of cursor names that will work across different
platforms can be found in the CSS specification:
- "none"
- ![](default_cursor.png) "default"
- ![](help_cursor.png) "help"
- ![](pointer_cursor.png) "pointer"
- ![](context_menu_cursor.png) "context-menu"
- ![](progress_cursor.png) "progress"
- ![](wait_cursor.png) "wait"
- ![](cell_cursor.png) "cell"
- ![](crosshair_cursor.png) "crosshair"
- ![](text_cursor.png) "text"
- ![](vertical_text_cursor.png) "vertical-text"
- ![](alias_cursor.png) "alias"
- ![](copy_cursor.png) "copy"
- ![](no_drop_cursor.png) "no-drop"
- ![](move_cursor.png) "move"
- ![](not_allowed_cursor.png) "not-allowed"
- ![](grab_cursor.png) "grab"
- ![](grabbing_cursor.png) "grabbing"
- ![](all_scroll_cursor.png) "all-scroll"
- ![](col_resize_cursor.png) "col-resize"
- ![](row_resize_cursor.png) "row-resize"
- ![](n_resize_cursor.png) "n-resize"
- ![](e_resize_cursor.png) "e-resize"
- ![](s_resize_cursor.png) "s-resize"
- ![](w_resize_cursor.png) "w-resize"
- ![](ne_resize_cursor.png) "ne-resize"
- ![](nw_resize_cursor.png) "nw-resize"
- ![](sw_resize_cursor.png) "sw-resize"
- ![](se_resize_cursor.png) "se-resize"
- ![](ew_resize_cursor.png) "ew-resize"
- ![](ns_resize_cursor.png) "ns-resize"
- ![](nesw_resize_cursor.png) "nesw-resize"
- ![](nwse_resize_cursor.png) "nwse-resize"
- ![](zoom_in_cursor.png) "zoom-in"
- ![](zoom_out_cursor.png) "zoom-out"
## `name`
the name of the cursor
## `fallback`
`None` or the `Cursor` to fall back to when
 this one cannot be supported

# Returns

a new `Cursor`, or `None` if there is no
 cursor with the given name
<!-- impl Cursor::fn new_from_texture -->
Creates a new cursor from a `Texture`.
## `texture`
the texture providing the pixel data
## `hotspot_x`
the horizontal offset of the “hotspot” of the cursor
## `hotspot_y`
the vertical offset of the “hotspot” of the cursor
## `fallback`
`None` or the `Cursor` to fall back to when
 this one cannot be supported

# Returns

a new `Cursor`.
<!-- impl Cursor::fn get_fallback -->
Returns the fallback for this `self`. The fallback will be used if this
cursor is not available on a given `Display`.

For named cursors, this can happen when using nonstandard names or when
using an incomplete cursor theme.
For textured cursors, this can happen when the texture is too large or
when the `Display` it is used on does not support textured cursors.

# Returns

the fallback of the cursor or `None` to use
 the default cursor as fallback.
<!-- impl Cursor::fn get_hotspot_x -->
Returns the horizontal offset of the hotspot. The hotspot indicates the
pixel that will be directly above the cursor.

# Returns

the horizontal offset of the hotspot or 0 for named cursors
<!-- impl Cursor::fn get_hotspot_y -->
Returns the vertical offset of the hotspot. The hotspot indicates the
pixel that will be directly above the cursor.

# Returns

the vertical offset of the hotspot or 0 for named cursors
<!-- impl Cursor::fn get_name -->
Returns the name of the cursor. If the cursor is not a named cursor, `None`
will be returned and the `Cursor`::texture property will be set.

# Returns

the name of the cursor or `None` if it is not
 a named cursor
<!-- impl Cursor::fn get_texture -->
Returns the texture for the cursor. If the cursor is a named cursor, `None`
will be returned and the `Cursor`::name property will be set.

# Returns

the texture for cursor or `None` if it is a
 named cursor
<!-- struct Device -->
The `Device` object represents a single input device, such
as a keyboard, a mouse, a touchpad, etc.

See the `Seat` documentation for more information
about the various kinds of master and slave devices, and their
relationships.
<!-- impl Device::fn free_history -->
Frees an array of `TimeCoord` that was returned by `Device::get_history`.
## `events`
an array of `TimeCoord`.
## `n_events`
the length of the array.
<!-- impl Device::fn get_associated_device -->
Returns the associated device to `self`, if `self` is of type
`DeviceType::Master`, it will return the paired pointer or
keyboard.

If `self` is of type `DeviceType::Slave`, it will return
the master device to which `self` is attached to.

If `self` is of type `DeviceType::Floating`, `None` will be
returned, as there is no associated device.

# Returns

The associated device, or
 `None`
<!-- impl Device::fn get_axes -->
Returns the axes currently available on the device.
<!-- impl Device::fn get_axis -->
Interprets an array of double as axis values for a given device,
and locates the value in the array for a given axis use.
## `axes`
pointer to an array of axes
## `use_`
the use to look for
## `value`
location to store the found value.

# Returns

`true` if the given axis use was found, otherwise `false`
<!-- impl Device::fn get_axis_use -->
Returns the axis use for `index_`.
## `index_`
the index of the axis.

# Returns

a `AxisUse` specifying how the axis is used.
<!-- impl Device::fn get_axis_value -->
Interprets an array of double as axis values for a given device,
and locates the value in the array for a given axis label, as returned
by `Device::list_axes`
## `axes`
pointer to an array of axes
## `axis_label`
name of the label
## `value`
location to store the found value.

# Returns

`true` if the given axis use was found, otherwise `false`.
<!-- impl Device::fn get_device_type -->
Returns the device type for `self`.

# Returns

the `DeviceType` for `self`.
<!-- impl Device::fn get_display -->
Returns the `Display` to which `self` pertains.

# Returns

a `Display`. This memory is owned
 by GTK+, and must not be freed or unreffed.
<!-- impl Device::fn get_has_cursor -->
Determines whether the pointer follows device motion.
This is not meaningful for keyboard devices, which don't have a pointer.

# Returns

`true` if the pointer follows device motion
<!-- impl Device::fn get_history -->
Obtains the motion history for a pointer device; given a starting and
ending timestamp, return all events in the motion history for
the device in the given range of time. Some windowing systems
do not support motion history, in which case, `false` will
be returned. (This is not distinguishable from the case where
motion history is supported and no events were found.)

Note that there is also `gdk_surface_set_event_compression` to get
more motion events delivered directly, independent of the windowing
system.
## `surface`
the surface with respect to which which the event coordinates will be reported
## `start`
starting timestamp for range of events to return
## `stop`
ending timestamp for the range of events to return
## `events`

 location to store a newly-allocated array of `TimeCoord`, or
 `None`
## `n_events`
location to store the length of
 `events`, or `None`

# Returns

`true` if the windowing system supports motion history and
 at least one event was found.
<!-- impl Device::fn get_key -->
If `index_` has a valid keyval, this function will return `true`
and fill in `keyval` and `modifiers` with the keyval settings.
## `index_`
the index of the macro button to get.
## `keyval`
return value for the keyval.
## `modifiers`
return value for modifiers.

# Returns

`true` if keyval is set for `index`.
<!-- impl Device::fn get_last_event_surface -->
Gets information about which surface the given pointer device is in, based on events
that have been received so far from the display server. If another application
has a pointer grab, or this application has a grab with owner_events = `false`,
`None` may be returned even if the pointer is physically over one of this
application's surfaces.

# Returns

the last surface the device
<!-- impl Device::fn get_mode -->
Determines the mode of the device.

# Returns

a `InputSource`
<!-- impl Device::fn get_n_axes -->
Returns the number of axes the device currently has.

# Returns

the number of axes.
<!-- impl Device::fn get_n_keys -->
Returns the number of keys the device currently has.

# Returns

the number of keys.
<!-- impl Device::fn get_name -->
Determines the name of the device.

# Returns

a name
<!-- impl Device::fn get_position -->
Gets the current location of `self` in double precision. As a slave device's
coordinates are those of its master pointer, this function
may not be called on devices of type `DeviceType::Slave`,
unless there is an ongoing grab on them. See `gdk_device_grab`.
## `x`
location to store root window X coordinate of `self`, or `None`.
## `y`
location to store root window Y coordinate of `self`, or `None`.
<!-- impl Device::fn get_product_id -->
Returns the product ID of this device, or `None` if this information couldn't
be obtained. This ID is retrieved from the device, and is thus constant for
it. See `Device::get_vendor_id` for more information.

# Returns

the product ID, or `None`
<!-- impl Device::fn get_seat -->
Returns the `Seat` the device belongs to.

# Returns

A `Seat`. This memory is owned by GTK+ and
 must not be freed.
<!-- impl Device::fn get_source -->
Determines the type of the device.

# Returns

a `InputSource`
<!-- impl Device::fn get_state -->
Gets the current state of a pointer device relative to `surface`. As a slave
device’s coordinates are those of its master pointer, this
function may not be called on devices of type `DeviceType::Slave`,
unless there is an ongoing grab on them. See `gdk_device_grab`.
## `surface`
a `Surface`.
## `axes`
an array of doubles to store the values of
the axes of `self` in, or `None`.
## `mask`
location to store the modifiers, or `None`.
<!-- impl Device::fn get_surface_at_position -->
Obtains the surface underneath `self`, returning the location of the device in `win_x` and `win_y` in
double precision. Returns `None` if the surface tree under `self` is not known to GDK (for example,
belongs to another application).

As a slave device coordinates are those of its master pointer, This
function may not be called on devices of type `DeviceType::Slave`,
unless there is an ongoing grab on them, see `gdk_device_grab`.
## `win_x`
return location for the X coordinate of the device location,
 relative to the surface origin, or `None`.
## `win_y`
return location for the Y coordinate of the device location,
 relative to the surface origin, or `None`.

# Returns

the `Surface` under the
 device position, or `None`.
<!-- impl Device::fn get_vendor_id -->
Returns the vendor ID of this device, or `None` if this information couldn't
be obtained. This ID is retrieved from the device, and is thus constant for
it.

This function, together with `Device::get_product_id`, can be used to eg.
compose `gio::Settings` paths to store settings for this device.


```C
 static GSettings *
 get_device_settings (GdkDevice *device)
 {
   const gchar *vendor, *product;
   GSettings *settings;
   GdkDevice *device;
   gchar *path;

   vendor = gdk_device_get_vendor_id (device);
   product = gdk_device_get_product_id (device);

   path = g_strdup_printf ("/org/example/app/devices/%s:%s/", vendor, product);
   settings = g_settings_new_with_path (DEVICE_SCHEMA, path);
   g_free (path);

   return settings;
 }
```

# Returns

the vendor ID, or `None`
<!-- impl Device::fn list_axes -->
Returns a `glib::List` of ``GdkAtoms``, containing the labels for
the axes that `self` currently has.

# Returns


 A `glib::List` of strings, free with `glib::List::free`.
<!-- impl Device::fn list_slave_devices -->
If the device if of type `DeviceType::Master`, it will return
the list of slave devices attached to it, otherwise it will return
`None`

# Returns


 the list of slave devices, or `None`. The list must be
 freed with `glib::List::free`, the contents of the list are
 owned by GTK+ and should not be freed.
<!-- impl Device::fn set_axis_use -->
Specifies how an axis of a device is used.
## `index_`
the index of the axis
## `use_`
specifies how the axis is used
<!-- impl Device::fn set_key -->
Specifies the X key event to generate when a macro button of a device
is pressed.
## `index_`
the index of the macro button to set
## `keyval`
the keyval to generate
## `modifiers`
the modifiers to set
<!-- impl Device::fn set_mode -->
Sets a the mode of an input device. The mode controls if the
device is active and whether the device’s range is mapped to the
entire screen or to a single surface.

Note: This is only meaningful for floating devices, master devices (and
slaves connected to these) drive the pointer cursor, which is not limited
by the input mode.
## `mode`
the input mode.

# Returns

`true` if the mode was successfully changed.
<!-- impl Device::fn connect_changed -->
The ::changed signal is emitted either when the `Device`
has changed the number of either axes or keys. For example
In X this will normally happen when the slave device routing
events through the master device changes (for example, user
switches from the USB mouse to a tablet), in that case the
master device will change to reflect the new slave device
axes and keys.
<!-- impl Device::fn connect_tool_changed -->
The ::tool-changed signal is emitted on pen/eraser
``GdkDevices`` whenever tools enter or leave proximity.
## `tool`
The new current tool
<!-- impl Device::fn get_property_associated_device -->
Associated pointer or keyboard with this device, if any. Devices of type `DeviceType::Master`
always come in keyboard/pointer pairs. Other device types will have a `None` associated device.
<!-- impl Device::fn get_property_axes -->
The axes currently available for this device.
<!-- impl Device::fn get_property_display -->
The `Display` the `Device` pertains to.
<!-- impl Device::fn set_property_display -->
The `Display` the `Device` pertains to.
<!-- impl Device::fn get_property_has_cursor -->
Whether the device is represented by a cursor on the screen. Devices of type
`DeviceType::Master` will have `true` here.
<!-- impl Device::fn set_property_has_cursor -->
Whether the device is represented by a cursor on the screen. Devices of type
`DeviceType::Master` will have `true` here.
<!-- impl Device::fn get_property_input_source -->
Source type for the device.
<!-- impl Device::fn set_property_input_source -->
Source type for the device.
<!-- impl Device::fn get_property_n_axes -->
Number of axes in the device.
<!-- impl Device::fn get_property_name -->
The device name.
<!-- impl Device::fn set_property_name -->
The device name.
<!-- impl Device::fn get_property_num_touches -->
The maximal number of concurrent touches on a touch device.
Will be 0 if the device is not a touch device or if the number
of touches is unknown.
<!-- impl Device::fn set_property_num_touches -->
The maximal number of concurrent touches on a touch device.
Will be 0 if the device is not a touch device or if the number
of touches is unknown.
<!-- impl Device::fn get_property_product_id -->
Product ID of this device, see `Device::get_product_id`.
<!-- impl Device::fn set_property_product_id -->
Product ID of this device, see `Device::get_product_id`.
<!-- impl Device::fn get_property_seat -->
`Seat` of this device.
<!-- impl Device::fn set_property_seat -->
`Seat` of this device.
<!-- impl Device::fn get_property_type -->
Device role in the device manager.
<!-- impl Device::fn set_property_type -->
Device role in the device manager.
<!-- impl Device::fn get_property_vendor_id -->
Vendor ID of this device, see `Device::get_vendor_id`.
<!-- impl Device::fn set_property_vendor_id -->
Vendor ID of this device, see `Device::get_vendor_id`.
<!-- struct DevicePad -->
`DevicePad` is an interface implemented by devices of type
`InputSource::TabletPad`, it allows querying the features provided
by the pad device.

Tablet pads may contain one or more groups, each containing a subset
of the buttons/rings/strips available. `DevicePad::get_n_groups`
can be used to obtain the number of groups, `DevicePad::get_n_features`
and `DevicePad::get_feature_group` can be combined to find out the
number of buttons/rings/strips the device has, and how are they grouped.

Each of those groups have different modes, which may be used to map
each individual pad feature to multiple actions. Only one mode is
effective (current) for each given group, different groups may have
different current modes. The number of available modes in a group can
be found out through `DevicePad::get_group_n_modes`, and the current
mode for a given group will be notified through the `EventPadGroupMode`
event.

# Implements

[`DevicePadExt`](trait.DevicePadExt.html), [`DeviceExt`](trait.DeviceExt.html)
<!-- trait DevicePadExt -->
Trait containing all `DevicePad` methods.

# Implementors

[`DevicePad`](struct.DevicePad.html)
<!-- trait DevicePadExt::fn get_feature_group -->
Returns the group the given `feature` and `idx` belong to,
or -1 if feature/index do not exist in `self`.
## `feature`
the feature type to get the group from
## `feature_idx`
the index of the feature to get the group from

# Returns

The group number of the queried pad feature.
<!-- trait DevicePadExt::fn get_group_n_modes -->
Returns the number of modes that `group` may have.
## `group_idx`
group to get the number of available modes from

# Returns

The number of modes available in `group`.
<!-- trait DevicePadExt::fn get_n_features -->
Returns the number of features a tablet pad has.
## `feature`
a pad feature

# Returns

The amount of elements of type `feature` that this pad has.
<!-- trait DevicePadExt::fn get_n_groups -->
Returns the number of groups this pad device has. Pads have
at least one group. A pad group is a subcollection of
buttons/strip/rings that is affected collectively by a same
current mode.

# Returns

The number of button/ring/strip groups in the pad.
<!-- enum DevicePadFeature -->
A pad feature.
<!-- enum DevicePadFeature::variant Button -->
a button
<!-- enum DevicePadFeature::variant Ring -->
a ring-shaped interactive area
<!-- enum DevicePadFeature::variant Strip -->
a straight interactive area
<!-- struct DeviceTool -->

<!-- impl DeviceTool::fn get_hardware_id -->
Gets the hardware ID of this tool, or 0 if it's not known. When
non-zero, the identificator is unique for the given tool model,
meaning that two identical tools will share the same `hardware_id`,
but will have different serial numbers (see `DeviceTool::get_serial`).

This is a more concrete (and device specific) method to identify
a `DeviceTool` than `DeviceTool::get_tool_type`, as a tablet
may support multiple devices with the same `DeviceToolType`,
but having different hardware identificators.

# Returns

The hardware identificator of this tool.
<!-- impl DeviceTool::fn get_serial -->
Gets the serial of this tool, this value can be used to identify a
physical tool (eg. a tablet pen) across program executions.

# Returns

The serial ID for this tool
<!-- impl DeviceTool::fn get_tool_type -->
Gets the `DeviceToolType` of the tool.

# Returns

The physical type for this tool. This can be used to figure out what
sort of pen is being used, such as an airbrush or a pencil.
<!-- enum DeviceToolType -->
Indicates the specific type of tool being used being a tablet. Such as an
airbrush, pencil, etc.
<!-- enum DeviceToolType::variant Unknown -->
Tool is of an unknown type.
<!-- enum DeviceToolType::variant Pen -->
Tool is a standard tablet stylus.
<!-- enum DeviceToolType::variant Eraser -->
Tool is standard tablet eraser.
<!-- enum DeviceToolType::variant Brush -->
Tool is a brush stylus.
<!-- enum DeviceToolType::variant Pencil -->
Tool is a pencil stylus.
<!-- enum DeviceToolType::variant Airbrush -->
Tool is an airbrush stylus.
<!-- enum DeviceToolType::variant Mouse -->
Tool is a mouse.
<!-- enum DeviceToolType::variant Lens -->
Tool is a lens cursor.
<!-- enum DeviceType -->
Indicates the device type.
<!-- enum DeviceType::variant Master -->
Device is a master (or virtual) device. There will
 be an associated focus indicator on the screen.
<!-- enum DeviceType::variant Slave -->
Device is a slave (or physical) device.
<!-- enum DeviceType::variant Floating -->
Device is a physical device, currently not attached to
 any seat.
<!-- struct Display -->
`Display` objects are the GDK representation of a workstation.

Their purpose are two-fold:
- To manage and provide information about input devices (pointers, keyboards, etc)
- To manage and provide information about output devices (monitors, projectors, etc)

Most of the input device handling has been factored out into separate `Seat`
objects. Every display has a one or more seats, which can be accessed with
`Display::get_default_seat` and `Display::list_seats`.

Output devices are represented by `Monitor` objects, which can be accessed
with `Display::get_monitor` and similar APIs.
<!-- impl Display::fn get_default -->
Gets the default `Display`. This is a convenience
function for:
`gdk_display_manager_get_default_display (gdk_display_manager_get ())`.

# Returns

a `Display`, or `None` if
 there is no default display.
<!-- impl Display::fn open -->
Opens a display.
## `display_name`
the name of the display to open

# Returns

a `Display`, or `None` if the
 display could not be opened
<!-- impl Display::fn beep -->
Emits a short beep on `self`
<!-- impl Display::fn close -->
Closes the connection to the windowing system for the given display,
and cleans up associated resources.
<!-- impl Display::fn device_is_grabbed -->
Returns `true` if there is an ongoing grab on `device` for `self`.
## `device`
a `Device`

# Returns

`true` if there is a grab in effect for `device`.
<!-- impl Display::fn flush -->
Flushes any requests queued for the windowing system; this happens automatically
when the main loop blocks waiting for new events, but if your application
is drawing without returning control to the main loop, you may need
to call this function explicitly. A common case where this function
needs to be called is when an application is executing drawing commands
from a thread other than the thread where the main loop is running.

This is most useful for X11. On windowing systems where requests are
handled synchronously, this function will do nothing.
<!-- impl Display::fn get_app_launch_context -->
Returns a `AppLaunchContext` suitable for launching
applications on the given display.

# Returns

a new `AppLaunchContext` for `self`.
 Free with `gobject::Object::unref` when done
<!-- impl Display::fn get_clipboard -->
Gets the clipboard used for copy/paste operations.

# Returns

the display's clipboard
<!-- impl Display::fn get_default_group -->
Returns the default group leader surface for all toplevel surfaces
on `self`. This surface is implicitly created by GDK.
See `gdk_surface_set_group`.

# Returns

The default group leader surface
for `self`
<!-- impl Display::fn get_default_seat -->
Returns the default `Seat` for this display.

# Returns

the default seat.
<!-- impl Display::fn get_event -->
Gets the next `Event` to be processed for `self`, fetching events from the
windowing system if necessary.

# Returns

the next `Event` to be processed,
 or `None` if no events are pending
<!-- impl Display::fn get_keymap -->
Returns the `Keymap` attached to `self`.

# Returns

the `Keymap` attached to `self`.
<!-- impl Display::fn get_monitor -->
Gets a monitor associated with this display.
## `monitor_num`
number of the monitor

# Returns

the `Monitor`, or `None` if
 `monitor_num` is not a valid monitor number
<!-- impl Display::fn get_monitor_at_point -->
Gets the monitor in which the point (`x`, `y`) is located,
or a nearby monitor if the point is not in any monitor.
## `x`
the x coordinate of the point
## `y`
the y coordinate of the point

# Returns

the monitor containing the point
<!-- impl Display::fn get_monitor_at_surface -->
Gets the monitor in which the largest area of `surface`
resides, or a monitor close to `surface` if it is outside
of all monitors.
## `surface`
a `Surface`

# Returns

the monitor with the largest overlap with `surface`
<!-- impl Display::fn get_n_monitors -->
Gets the number of monitors that belong to `self`.

The returned number is valid until the next emission of the
`Display::monitor-added` or `Display::monitor-removed` signal.

# Returns

the number of monitors
<!-- impl Display::fn get_name -->
Gets the name of the display.

# Returns

a string representing the display name. This string is owned
by GDK and should not be modified or freed.
<!-- impl Display::fn get_primary_clipboard -->
Gets the clipboard used for the primary selection. On backends where the
primary clipboard is not supported natively, GDK emulates this clipboard
locally.

# Returns

the primary clipboard
<!-- impl Display::fn get_primary_monitor -->
Gets the primary monitor for the display.

The primary monitor is considered the monitor where the “main desktop”
lives. While normal application surfaces typically allow the window
manager to place the surfaces, specialized desktop applications
such as panels should place themselves on the primary monitor.

If no monitor is the designated primary monitor, any monitor
(usually the first) may be returned. To make sure there is a dedicated
primary monitor, use `Monitor::is_primary` on the returned monitor.

# Returns

the primary monitor, or any monitor if no
 primary monitor is configured by the user
<!-- impl Display::fn get_setting -->
Retrieves a desktop-wide setting such as double-click time
for the `self`.
## `name`
the name of the setting
## `value`
location to store the value of the setting

# Returns

`true` if the setting existed and a value was stored
 in `value`, `false` otherwise
<!-- impl Display::fn get_startup_notification_id -->
Gets the startup notification ID for a Wayland display, or `None`
if no ID has been defined.

# Returns

the startup notification ID for `self`, or `None`
<!-- impl Display::fn has_pending -->
Returns whether the display has events that are waiting
to be processed.

# Returns

`true` if there are events ready to be processed.
<!-- impl Display::fn is_closed -->
Finds out if the display has been closed.

# Returns

`true` if the display is closed.
<!-- impl Display::fn is_composited -->
Returns whether surfaces can reasonably be expected to have
their alpha channel drawn correctly on the screen. Check
`Display::is_rgba` for wether the display supports an
alpha channel.

On X11 this function returns whether a compositing manager is
compositing on `self`.

On modern displays, this value is always `true`.

# Returns

Whether surfaces with RGBA visuals can reasonably be
expected to have their alpha channels drawn correctly on the screen.
<!-- impl Display::fn is_rgba -->
Returns wether surfaces on this `self` are created with an
alpha channel.

Even if a `true` is returned, it is possible that the
surface’s alpha channel won’t be honored when displaying the
surface on the screen: in particular, for X an appropriate
windowing manager and compositing manager must be running to
provide appropriate display. Use `Display::is_composited`
to check if that is the case.

For setting an overall opacity for a top-level surface, see
`SurfaceExt::set_opacity`.

On modern displays, this value is always `true`.

# Returns

`true` if surfaces are created with an alpha channel or
 `false` if the display does not support this functionality.
<!-- impl Display::fn list_seats -->
Returns the list of seats known to `self`.

# Returns

the
 list of seats known to the `Display`
<!-- impl Display::fn notify_startup_complete -->
Indicates to the GUI environment that the application has
finished loading, using a given identifier.

GTK+ will call this function automatically for ``GtkWindow``
with custom startup-notification identifier unless
`gtk_window_set_auto_startup_notification` is called to
disable that feature.
## `startup_id`
a startup-notification identifier, for which
 notification process should be completed
<!-- impl Display::fn peek_event -->
Gets a copy of the first `Event` in the `self`’s event queue, without
removing the event from the queue. (Note that this function will
not get more events from the windowing system. It only checks the events
that have already been moved to the GDK event queue.)

# Returns

the first `Event` on the
 event queue
<!-- impl Display::fn put_event -->
Appends a copy of the given event onto the front of the event
queue for `self`.
## `event`
a `Event`.
<!-- impl Display::fn supports_input_shapes -->
Returns `true` if `gdk_surface_input_shape_combine_mask` can
be used to modify the input shape of surfaces on `self`.

# Returns

`true` if surfaces with modified input shape are supported
<!-- impl Display::fn supports_shapes -->
Returns `true` if `gdk_surface_shape_combine_mask` can
be used to create shaped windows on `self`.

# Returns

`true` if shaped windows are supported
<!-- impl Display::fn sync -->
Flushes any requests queued for the windowing system and waits until all
requests have been handled. This is often used for making sure that the
display is synchronized with the current state of the program. Calling
`Display::sync` before `gdk_error_trap_pop` makes sure that any errors
generated from earlier requests are handled before the error trap is
removed.

This is most useful for X11. On windowing systems where requests are
handled synchronously, this function will do nothing.
<!-- impl Display::fn connect_closed -->
The ::closed signal is emitted when the connection to the windowing
system for `display` is closed.
## `is_error`
`true` if the display was closed due to an error
<!-- impl Display::fn connect_monitor_added -->
The ::monitor-added signal is emitted whenever a monitor is
added.
## `monitor`
the monitor that was just added
<!-- impl Display::fn connect_monitor_removed -->
The ::monitor-removed signal is emitted whenever a monitor is
removed.
## `monitor`
the monitor that was just removed
<!-- impl Display::fn connect_opened -->
The ::opened signal is emitted when the connection to the windowing
system for `display` is opened.
<!-- impl Display::fn connect_seat_added -->
The ::seat-added signal is emitted whenever a new seat is made
known to the windowing system.
## `seat`
the seat that was just added
<!-- impl Display::fn connect_seat_removed -->
The ::seat-removed signal is emitted whenever a seat is removed
by the windowing system.
## `seat`
the seat that was just removed
<!-- impl Display::fn connect_setting_changed -->
The ::setting-changed signal is emitted whenever a setting
changes its value.
## `setting`
the name of the setting that changed
<!-- impl Display::fn get_property_composited -->
`true` if the display properly composits the alpha channel.
See `Display::is_composited` for details.
<!-- impl Display::fn get_property_rgba -->
`true` if the display supports an alpha channel. See `Display::is_rgba`
for details.
<!-- struct DisplayManager -->
The purpose of the `DisplayManager` singleton object is to offer
notification when displays appear or disappear or the default display
changes.

You can use `DisplayManager::get` to obtain the `DisplayManager`
singleton, but that should be rarely necessary. Typically, initializing
GTK opens a display that you can work with without ever accessing the
`DisplayManager`.

The GDK library can be built with support for multiple backends.
The `DisplayManager` object determines which backend is used
at runtime.

When writing backend-specific code that is supposed to work with
multiple GDK backends, you have to consider both compile time and
runtime. At compile time, use the `GDK_WINDOWING_X11`, `GDK_WINDOWING_WIN32`
macros, etc. to find out which backends are present in the GDK library
you are building your application against. At runtime, use type-check
macros like GDK_IS_X11_DISPLAY() to find out which backend is in use:

## Backend-specific code ## {`backend`-specific}


```C
#ifdef GDK_WINDOWING_X11
  if (GDK_IS_X11_DISPLAY (display))
    {
      // make X11-specific calls here
    }
  else
#endif
#ifdef GDK_WINDOWING_QUARTZ
  if (GDK_IS_QUARTZ_DISPLAY (display))
    {
      // make Quartz-specific calls here
    }
  else
#endif
  g_error ("Unsupported GDK backend");
```
<!-- impl DisplayManager::fn get -->
Gets the singleton `DisplayManager` object.

When called for the first time, this function consults the
`GDK_BACKEND` environment variable to find out which
of the supported GDK backends to use (in case GDK has been compiled
with multiple backends). Applications can use `gdk_set_allowed_backends`
to limit what backends can be used.

# Returns

The global `DisplayManager` singleton;
 `gdk_parse_args`, `gdk_init`, or `gdk_init_check` must have
 been called first.
<!-- impl DisplayManager::fn get_default_display -->
Gets the default `Display`.

# Returns

a `Display`, or `None` if
 there is no default display.
<!-- impl DisplayManager::fn list_displays -->
List all currently open displays.

# Returns

a newly
 allocated `glib::SList` of `Display` objects. Free with `glib::SList::free`
 when you are done with it.
<!-- impl DisplayManager::fn open_display -->
Opens a display.
## `name`
the name of the display to open

# Returns

a `Display`, or `None` if the
 display could not be opened
<!-- impl DisplayManager::fn set_default_display -->
Sets `display` as the default display.
## `display`
a `Display`
<!-- impl DisplayManager::fn connect_display_opened -->
The ::display-opened signal is emitted when a display is opened.
## `display`
the opened display
<!-- struct Drag -->
The `Drag` struct contains only private fields and
should not be accessed directly.
<!-- impl Drag::fn begin -->
Starts a drag and creates a new drag context for it.

This function is called by the drag source.
## `surface`
the source surface for this drag
## `device`
the device that controls this drag
## `content`
the offered content
## `actions`
the actions supported by this drag
## `dx`
the x offset to `device`'s position where the drag nominally started
## `dy`
the y offset to `device`'s position where the drag nominally started

# Returns

a newly created `Drag` or
 `None` on error.
<!-- impl Drag::fn drop_done -->
Inform GDK if the drop ended successfully. Passing `false`
for `success` may trigger a drag cancellation animation.

This function is called by the drag source, and should
be the last call before dropping the reference to the
`self`.

The `Drag` will only take the first `Drag::drop_done`
call as effective, if this function is called multiple times,
all subsequent calls will be ignored.
## `success`
whether the drag was ultimatively successful
<!-- impl Drag::fn get_actions -->
Determines the bitmask of possible actions proposed by the source.

# Returns

the `DragAction` flags
<!-- impl Drag::fn get_device -->
Returns the `Device` associated to the `Drag` object.

# Returns

The `Device` associated to `self`.
<!-- impl Drag::fn get_display -->
Gets the `Display` that the drag object was created for.

# Returns

a `Display`
<!-- impl Drag::fn get_drag_surface -->
Returns the surface on which the drag icon should be rendered
during the drag operation. Note that the surface may not be
available until the drag operation has begun. GDK will move
the surface in accordance with the ongoing drag operation.
The surface is owned by `self` and will be destroyed when
the drag operation is over.

# Returns

the drag surface, or `None`
<!-- impl Drag::fn get_formats -->
Retrieves the formats supported by this `Drag` object.

# Returns

a `ContentFormats`
<!-- impl Drag::fn get_selected_action -->
Determines the action chosen by the drag destination.

# Returns

a `DragAction` value
<!-- impl Drag::fn set_hotspot -->
Sets the position of the drag surface that will be kept
under the cursor hotspot. Initially, the hotspot is at the
top left corner of the drag surface.
## `hot_x`
x coordinate of the drag surface hotspot
## `hot_y`
y coordinate of the drag surface hotspot
<!-- impl Drag::fn connect_cancel -->
The drag operation was cancelled.
## `reason`
The reason the drag was cancelled
<!-- impl Drag::fn connect_dnd_finished -->
The drag operation was finished, the destination
finished reading all data. The drag object can now
free all miscellaneous data.
<!-- impl Drag::fn connect_drop_performed -->
The drag operation was performed on an accepting client.
<!-- impl Drag::fn get_property_content -->
The `ContentProvider`.
<!-- impl Drag::fn set_property_content -->
The `ContentProvider`.
<!-- impl Drag::fn get_property_device -->
The `Device` that is performing the drag.
<!-- impl Drag::fn set_property_device -->
The `Device` that is performing the drag.
<!-- impl Drag::fn get_property_display -->
The `Display` that the drag belongs to.
<!-- impl Drag::fn get_property_formats -->
The possible formats that the drag can provide its data in.
<!-- impl Drag::fn set_property_formats -->
The possible formats that the drag can provide its data in.
<!-- enum DragCancelReason -->
Used in `Drag` to the reason of a cancelled DND operation.
<!-- enum DragCancelReason::variant NoTarget -->
There is no suitable drop target.
<!-- enum DragCancelReason::variant UserCancelled -->
Drag cancelled by the user
<!-- enum DragCancelReason::variant Error -->
Unspecified error.
<!-- struct DrawContext -->
`DrawContext` is the base object used by contexts implementing different
rendering methods, such as `GLContext` or `VulkanContext`. It provides
shared functionality between those contexts.

You will always interact with one of those s.ubclasses.

A `DrawContext` is always associated with a single toplevel surface.

# Implements

[`DrawContextExt`](trait.DrawContextExt.html), [`DrawContextExtManual`](prelude/trait.DrawContextExtManual.html)
<!-- trait DrawContextExt -->
Trait containing all `DrawContext` methods.

# Implementors

[`CairoContext`](struct.CairoContext.html), [`DrawContext`](struct.DrawContext.html), [`GLContext`](struct.GLContext.html), [`VulkanContext`](struct.VulkanContext.html)
<!-- trait DrawContextExt::fn begin_frame -->
Indicates that you are beginning the process of redrawing `region`
on the `self`'s surface.

Calling this function begins a drawing operation using `self` on the
surface that `self` was created from. The actual requirements and
guarantees for the drawing operation vary for different implementations
of drawing, so a `CairoContext` and a `GLContext` need to be treated
differently.

A call to this function is a requirement for drawing and must be followed
by a call to `DrawContextExt::end_frame`, which will complete the
drawing operation and ensure the contents become visible on screen.

Note that the `region` passed to this function is the minimum region that
needs to be drawn and depending on implementation, windowing system and
hardware in use, it might be necessary to draw a larger region. Drawing
implementation must use `DrawContext::get_frame_region` to query the
region that must be drawn.

When using GTK+, the widget system automatically places calls to
`DrawContextExt::begin_frame` and `DrawContextExt::end_frame` via the
use of `GskRenderers`, so application code does not need to call these
functions explicitly.
## `region`
minimum region that should be drawn
<!-- trait DrawContextExt::fn end_frame -->
Ends a drawing operation started with `DrawContextExt::begin_frame`
and makes the drawing available on screen. See that function for more
details about drawing.

When using a `GLContext`, this function may call `glFlush()`
implicitly before returning; it is not recommended to call `glFlush()`
explicitly before calling this function.
<!-- trait DrawContextExt::fn get_display -->
Retrieves the `Display` the `self` is created for

# Returns

a `Display` or `None`
<!-- trait DrawContextExt::fn get_frame_region -->
Retrieves the region that is currently in the process of being repainted.

After a call to `DrawContextExt::begin_frame` this function will return
a union of the region passed to that function and the area of the surface
that the `self` determined needs to be repainted.

If `self` is not inbetween calls to `DrawContextExt::begin_frame` and
`DrawContextExt::end_frame`, `None` will be returned.

# Returns

a Cairo region or `None` if not drawing
 a frame.
<!-- trait DrawContextExt::fn get_surface -->
Retrieves the `Surface` used by the `self`.

# Returns

a `Surface` or `None`
<!-- trait DrawContextExt::fn is_in_frame -->
Returns `true` if `self` is in the process of drawing to its surface
after a call to `DrawContextExt::begin_frame` and not yet having called
`DrawContextExt::end_frame`.
In this situation, drawing commands may be effecting the contents of a
`self`'s surface.

# Returns

`true` if the context is between `begin_frame` and `end_frame` calls.
<!-- trait DrawContextExt::fn get_property_display -->
The `Display` used to create the `DrawContext`.
<!-- trait DrawContextExt::fn get_property_surface -->
The `Surface` the gl context is bound to.
<!-- trait DrawContextExt::fn set_property_surface -->
The `Surface` the gl context is bound to.
<!-- struct Drop -->
The `Drop` struct contains only private fields and
should not be accessed directly.
<!-- impl Drop::fn finish -->
Ends the drag operation after a drop.

The `action` must be a single action selected from the actions
available via `Drop::get_actions`.
## `action`
the action performed by the destination or 0 if the drop
 failed
<!-- impl Drop::fn get_actions -->
Returns the possible actions for this `Drop`. If this value
contains multiple actions - ie `DragAction::is_unique`
returns `false` for the result - `gdk_drag_finish` must choose
the action to use when accepting the drop.

This value may change over the lifetime of the `Drop` both
as a response to source side actions as well as to calls to
`Drop::status` or `gdk_drag_finish`. The source side will
not change this value anymore once a drop has started.

# Returns

The possible ``GdkDragActions``
<!-- impl Drop::fn get_device -->
Returns the `Device` performing the drop.

# Returns

The `Device` performing the drop.
<!-- impl Drop::fn get_display -->
Gets the `Display` that `self` was created for.

# Returns

a `Display`
<!-- impl Drop::fn get_drag -->
If this is an in-app drag-and-drop operation, returns the `Drag`
that corresponds to this drop.

If it is not, `None` is returned.

# Returns

the corresponding `Drag`
<!-- impl Drop::fn get_formats -->
Returns the `ContentFormats` that the drop offers the data
to be read in.

# Returns

The possible `ContentFormats`
<!-- impl Drop::fn get_surface -->
Returns the `Surface` performing the drop.

# Returns

The `Surface` performing the drop.
<!-- impl Drop::fn read_async -->
Asynchronously read the dropped data from a `Drop`
in a format that complies with one of the mime types.
## `mime_types`

 pointer to an array of mime types
## `io_priority`
the io priority for the read operation
## `cancellable`
optional `gio::Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when
 the request is satisfied
## `user_data`
the data to pass to `callback`
<!-- impl Drop::fn read_finish -->
Finishes an async drop read operation, see `Drop::read_async`.
## `result`
a `gio::AsyncResult`
## `out_mime_type`
return location for the used mime type

# Returns

the `gio::InputStream`, or `None`
<!-- impl Drop::fn read_text_async -->
Asynchronously request the drag operation's contents converted to a string.
When the operation is finished `callback` will be called. You can then
call `Drop::read_text_finish` to get the result.

This is a simple wrapper around `Drop::read_value_async`. Use
that function or `Drop::read_async` directly if you need more
control over the operation.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Drop::fn read_text_finish -->
Finishes an asynchronous read started with
`Drop::read_text_async`.
## `result`
a `gio::AsyncResult`

# Returns

a new string or `None` on error.
<!-- impl Drop::fn read_value_async -->
Asynchronously request the drag operation's contents converted to the given
`type_`. When the operation is finished `callback` will be called.
You can then call `Drop::read_value_finish` to get the resulting
`gobject::Value`.

For local drag'n'drop operations that are available in the given `glib::Type`, the
value will be copied directly. Otherwise, GDK will try to use
`gdk_content_deserialize_async` to convert the data.
## `type_`
a `glib::Type` to read
## `io_priority`
the [I/O priority][io-priority]
 of the request.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- impl Drop::fn read_value_finish -->
Finishes an async drop read started with
`Drop::read_value_async`.
## `result`
a `gio::AsyncResult`

# Returns

a `gobject::Value` containing the result.
<!-- impl Drop::fn status -->
Selects all actions that are potentially supported by the destination.

When calling this function, do not restrict the passed in actions to
the ones provided by `Drop::get_actions`. Those actions may
change in the future, even depending on the actions you provide here.

This function should be called by drag destinations in response to
`EventType::DragEnter` or `EventType::DragMotion` events. If the destination does
not yet know the exact actions it supports, it should set any possible
actions first and then later call this function again.
## `actions`
Supported actions of the destination, or 0 to indicate
 that a drop will not be accepted
<!-- impl Drop::fn get_property_actions -->
The possible actions for this drop
<!-- impl Drop::fn set_property_actions -->
The possible actions for this drop
<!-- impl Drop::fn get_property_device -->
The `Device` performing the drop
<!-- impl Drop::fn set_property_device -->
The `Device` performing the drop
<!-- impl Drop::fn get_property_display -->
The `Display` that the drop belongs to.
<!-- impl Drop::fn get_property_drag -->
The `Drag` that initiated this drop
<!-- impl Drop::fn set_property_drag -->
The `Drag` that initiated this drop
<!-- impl Drop::fn get_property_formats -->
The possible formats that the drop can provide its data in.
<!-- impl Drop::fn set_property_formats -->
The possible formats that the drop can provide its data in.
<!-- impl Drop::fn get_property_surface -->
The `Surface` the drop happens on
<!-- impl Drop::fn set_property_surface -->
The `Surface` the drop happens on
<!-- struct Event -->
The `Event` struct contains only private fields and
should not be accessed directly.
<!-- impl Event::fn new -->
Creates a new event of the given type. All fields are set to 0.
## `type_`
a `EventType`

# Returns

a newly-allocated `Event`. Free with `gobject::Object::unref`
<!-- impl Event::fn copy -->
Copies a `Event`, copying or incrementing the reference count of the
resources associated with it (e.g. `Surface`’s and strings).

# Returns

a copy of `self`. Free with `gobject::Object::unref`
<!-- impl Event::fn get_axes -->
Extracts all axis values from an event.
## `axes`
the array of values for all axes
## `n_axes`
the length of array

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_axis -->
Extract the axis value for a particular axis use from
an event structure.
## `axis_use`
the axis use to look for
## `value`
location to store the value found

# Returns

`true` if the specified axis was found, otherwise `false`
<!-- impl Event::fn get_button -->
Extract the button number from an event.
## `button`
location to store mouse button number

# Returns

`true` if the event delivered a button number
<!-- impl Event::fn get_click_count -->
Extracts the click count from an event.
## `click_count`
location to store click count

# Returns

`true` if the event delivered a click count
<!-- impl Event::fn get_coords -->
Extract the event surface relative x/y coordinates from an event.
## `x_win`
location to put event surface x coordinate
## `y_win`
location to put event surface y coordinate

# Returns

`true` if the event delivered event surface coordinates
<!-- impl Event::fn get_crossing_detail -->
Extracts the crossing detail from an event.
## `detail`
return location for the crossing detail

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_crossing_mode -->
Extracts the crossing mode from an event.
## `mode`
return location for the crossing mode

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_device -->
If the event contains a “device” field, this function will return
it, else it will return `None`.

# Returns

a `Device`, or `None`.
<!-- impl Event::fn get_device_tool -->
If the event was generated by a device that supports
different tools (eg. a tablet), this function will
return a `DeviceTool` representing the tool that
caused the event. Otherwise, `None` will be returned.

Note: the `DeviceTool`<!-- -->s will be constant during
the application lifetime, if settings must be stored
persistently across runs, see `DeviceTool::get_serial`

# Returns

The current device tool, or `None`
<!-- impl Event::fn get_display -->
Retrieves the `Display` associated to the `self`.

# Returns

a `Display`
<!-- impl Event::fn get_drop -->
Gets the `Drop` from a DND event.

# Returns

the drop
<!-- impl Event::fn get_event_sequence -->
If `self` if of type `EventType::TouchBegin`, `EventType::TouchUpdate`,
`EventType::TouchEnd` or `EventType::TouchCancel`, returns the `EventSequence`
to which the event belongs. Otherwise, return `None`.

# Returns

the event sequence that the event belongs to
<!-- impl Event::fn get_event_type -->
Retrieves the type of the event.

# Returns

a `EventType`
<!-- impl Event::fn get_focus_in -->
Extracts whether this is a focus-in or focus-out event.
## `focus_in`
return location for focus direction

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_grab_surface -->
Extracts the grab surface from a grab broken event.
## `surface`
Return location for the grab surface

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_key_group -->
Extracts the key group from an event.
## `group`
return location for the key group

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_key_is_modifier -->
Extracts whether the event is a key event for
a modifier key.
## `is_modifier`
return location for the value

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_keycode -->
Extracts the hardware keycode from an event.

Also see `Event::get_scancode`.
## `keycode`
location to store the keycode

# Returns

`true` if the event delivered a hardware keycode
<!-- impl Event::fn get_keyval -->
Extracts the keyval from an event.
## `keyval`
location to store the keyval

# Returns

`true` if the event delivered a key symbol
<!-- impl Event::fn get_motion_history -->
Retrieves the history of the `self` motion, as a list of time and
coordinates.

# Returns

a list
 of time and coordinates
<!-- impl Event::fn get_pad_axis_value -->
Extracts the information from a pad event.
## `index`
Return location for the axis index
## `value`
Return location for the axis value

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_pad_button -->
Extracts information about the pressed button from
a pad event.
## `button`
Return location for the button

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_pad_group_mode -->
Extracts group and mode information from a pad event.
## `group`
return location for the group
## `mode`
return location for the mode

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_pointer_emulated -->
Returns whether this event is an 'emulated' pointer event (typically
from a touch event), as opposed to a real one.

# Returns

`true` if this event is emulated
<!-- impl Event::fn get_root_coords -->
Extract the root window relative x/y coordinates from an event.
## `x_root`
location to put root window x coordinate
## `y_root`
location to put root window y coordinate

# Returns

`true` if the event delivered root window coordinates
<!-- impl Event::fn get_scancode -->
Gets the keyboard low-level scancode of a key event.

This is usually hardware_keycode. On Windows this is the high
word of WM_KEY{DOWN,UP} lParam which contains the scancode and
some extended flags.

# Returns

The associated keyboard scancode or 0
<!-- impl Event::fn get_scroll_deltas -->
Retrieves the scroll deltas from a `Event`
## `delta_x`
return location for X delta
## `delta_y`
return location for Y delta

# Returns

`true` if the event contains smooth scroll information
<!-- impl Event::fn get_scroll_direction -->
Extracts the scroll direction from an event.
## `direction`
location to store the scroll direction

# Returns

`true` if the event delivered a scroll direction
<!-- impl Event::fn get_seat -->
Returns the `Seat` this event was generated for.

# Returns

The `Seat` of this event
<!-- impl Event::fn get_source_device -->
This function returns the hardware (slave) `Device` that has
triggered the event, falling back to the virtual (master) device
(as in `Event::get_device`) if the event wasn’t caused by
interaction with a hardware device. This may happen for example
in synthesized crossing events after a `Surface` updates its
geometry or a grab is acquired/released.

If the event does not contain a device field, this function will
return `None`.

# Returns

a `Device`, or `None`.
<!-- impl Event::fn get_state -->
If the event contains a “state” field, puts that field in `state`.

Otherwise stores an empty state (0).
`self` may be `None`, in which case it’s treated
as if the event had no state field.
## `state`
return location for state

# Returns

`true` if there was a state field in the event
<!-- impl Event::fn get_surface -->
Extracts the `Surface` associated with an event.

# Returns

The `Surface` associated with the event
<!-- impl Event::fn get_time -->
Returns the time stamp from `self`, if there is one; otherwise
returns `GDK_CURRENT_TIME`. If `self` is `None`, returns `GDK_CURRENT_TIME`.

# Returns

time stamp field from `self`
<!-- impl Event::fn get_touch_emulating_pointer -->
Extracts whether a touch event is emulating a pointer event.
## `emulating`
Return location for information

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_touchpad_angle_delta -->
Extracts the angle from a touchpad event.
## `delta`
Return location for angle

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_touchpad_deltas -->
Extracts delta information from a touchpad event.
## `dx`
return location for x
## `dy`
return location for y

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_touchpad_gesture_n_fingers -->
Extracts the number of fingers from a touchpad event.
## `n_fingers`
return location for the number of fingers

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_touchpad_gesture_phase -->
Extracts the touchpad gesture phase from a touchpad event.
## `phase`
Return location for the gesture phase

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn get_touchpad_scale -->
Extracts the scale from a touchpad event.
## `scale`
Return location for scale

# Returns

`true` on success, otherwise `false`
<!-- impl Event::fn is_scroll_stop_event -->
Check whether a scroll event is a stop scroll event. Scroll sequences
with smooth scroll information may provide a stop scroll event once the
interaction with the device finishes, e.g. by lifting a finger. This
stop scroll event is the signal that a widget may trigger kinetic
scrolling based on the current velocity.

Stop scroll events always have a a delta of 0/0.

# Returns

`true` if the event is a scroll stop event
<!-- impl Event::fn is_sent -->
Returns whether the event was sent explicitly.

# Returns

`true` if the event was sent explicitly
<!-- impl Event::fn set_device -->
Sets the device for `self` to `device`. The event must
have been allocated by GTK+, for instance, by
`Event::copy`.
## `device`
a `Device`
<!-- impl Event::fn set_device_tool -->
Sets the device tool for this event, should be rarely used.
## `tool`
tool to set on the event, or `None`
<!-- impl Event::fn set_display -->
Sets the display that an event is associated with.
## `display`
a `Display`
<!-- impl Event::fn set_source_device -->
Sets the slave device for `self` to `device`.

The event must have been allocated by GTK+,
for instance by `Event::copy`.
## `device`
a `Device`
<!-- impl Event::fn triggers_context_menu -->
This function returns whether a `EventButton` should trigger a
context menu, according to platform conventions. The right mouse
button always triggers context menus. Additionally, if
`Keymap::get_modifier_mask` returns a non-0 mask for
`ModifierIntent::ContextMenu`, then the left mouse button will
also trigger a context menu if this modifier is pressed.

This function should always be used instead of simply checking for
event->button == `GDK_BUTTON_SECONDARY`.

# Returns

`true` if the event should trigger a context menu.
<!-- struct EventSequence -->
`EventSequence` is an opaque type representing a sequence
of related touch events.
<!-- enum EventType -->
Specifies the type of the event.

Do not confuse these events with the signals that GTK+ widgets emit.
Although many of these events result in corresponding signals being emitted,
the events are often transformed or filtered along the way.
<!-- enum EventType::variant Nothing -->
a special code to indicate a null event.
<!-- enum EventType::variant Delete -->
the window manager has requested that the toplevel surface be
 hidden or destroyed, usually when the user clicks on a special icon in the
 title bar.
<!-- enum EventType::variant Destroy -->
the surface has been destroyed.
<!-- enum EventType::variant MotionNotify -->
the pointer (usually a mouse) has moved.
<!-- enum EventType::variant ButtonPress -->
a mouse button has been pressed.
<!-- enum EventType::variant ButtonRelease -->
a mouse button has been released.
<!-- enum EventType::variant KeyPress -->
a key has been pressed.
<!-- enum EventType::variant KeyRelease -->
a key has been released.
<!-- enum EventType::variant EnterNotify -->
the pointer has entered the surface.
<!-- enum EventType::variant LeaveNotify -->
the pointer has left the surface.
<!-- enum EventType::variant FocusChange -->
the keyboard focus has entered or left the surface.
<!-- enum EventType::variant Configure -->
the size, position or stacking order of the surface has changed.
 Note that GTK+ discards these events for `SurfaceType::Child` surfaces.
<!-- enum EventType::variant ProximityIn -->
an input device has moved into contact with a sensing
 surface (e.g. a touchscreen or graphics tablet).
<!-- enum EventType::variant ProximityOut -->
an input device has moved out of contact with a sensing
 surface.
<!-- enum EventType::variant DragEnter -->
the mouse has entered the surface while a drag is in progress.
<!-- enum EventType::variant DragLeave -->
the mouse has left the surface while a drag is in progress.
<!-- enum EventType::variant DragMotion -->
the mouse has moved in the surface while a drag is in
 progress.
<!-- enum EventType::variant DropStart -->
a drop operation onto the surface has started.
<!-- enum EventType::variant Scroll -->
the scroll wheel was turned
<!-- enum EventType::variant GrabBroken -->
a pointer or keyboard grab was broken. This event type
 was added in 2.8.
<!-- enum EventType::variant TouchBegin -->
A new touch event sequence has just started. This event
 type was added in 3.4.
<!-- enum EventType::variant TouchUpdate -->
A touch event sequence has been updated. This event type
 was added in 3.4.
<!-- enum EventType::variant TouchEnd -->
A touch event sequence has finished. This event type
 was added in 3.4.
<!-- enum EventType::variant TouchCancel -->
A touch event sequence has been canceled. This event type
 was added in 3.4.
<!-- enum EventType::variant TouchpadSwipe -->
A touchpad swipe gesture event, the current state
 is determined by its phase field. This event type was added in 3.18.
<!-- enum EventType::variant TouchpadPinch -->
A touchpad pinch gesture event, the current state
 is determined by its phase field. This event type was added in 3.18.
<!-- enum EventType::variant PadButtonPress -->
A tablet pad button press event. This event type
 was added in 3.22.
<!-- enum EventType::variant PadButtonRelease -->
A tablet pad button release event. This event type
 was added in 3.22.
<!-- enum EventType::variant PadRing -->
A tablet pad axis event from a "ring". This event type was
 added in 3.22.
<!-- enum EventType::variant PadStrip -->
A tablet pad axis event from a "strip". This event type was
 added in 3.22.
<!-- enum EventType::variant PadGroupMode -->
A tablet pad group mode change. This event type was
 added in 3.22.
<!-- enum EventType::variant EventLast -->
marks the end of the `EventType` enumeration. Added in 2.18
<!-- struct FrameClock -->
A `FrameClock` tells the application when to update and repaint a
window. This may be synced to the vertical refresh rate of the
monitor, for example. Even when the frame clock uses a simple timer
rather than a hardware-based vertical sync, the frame clock helps
because it ensures everything paints at the same time (reducing the
total number of frames). The frame clock can also automatically
stop painting when it knows the frames will not be visible, or
scale back animation framerates.

`FrameClock` is designed to be compatible with an OpenGL-based
implementation or with mozRequestAnimationFrame in Firefox,
for example.

A frame clock is idle until someone requests a frame with
`FrameClock::request_phase`. At some later point that makes
sense for the synchronization being implemented, the clock will
process a frame and emit signals for each phase that has been
requested. (See the signals of the `FrameClock` class for
documentation of the phases. `FrameClockPhase::Update` and the
`FrameClock::update` signal are most interesting for application
writers, and are used to update the animations, using the frame time
given by `FrameClock::get_frame_time`.

The frame time is reported in microseconds and generally in the same
timescale as `g_get_monotonic_time`, however, it is not the same
as `g_get_monotonic_time`. The frame time does not advance during
the time a frame is being painted, and outside of a frame, an attempt
is made so that all calls to `FrameClock::get_frame_time` that
are called at a “similar” time get the same value. This means that
if different animations are timed by looking at the difference in
time between an initial value from `FrameClock::get_frame_time`
and the value inside the `FrameClock::update` signal of the clock,
they will stay exactly synchronized.
<!-- impl FrameClock::fn begin_updating -->
Starts updates for an animation. Until a matching call to
`FrameClock::end_updating` is made, the frame clock will continually
request a new frame with the `FrameClockPhase::Update` phase.
This function may be called multiple times and frames will be
requested until `FrameClock::end_updating` is called the same
number of times.
<!-- impl FrameClock::fn end_updating -->
Stops updates for an animation. See the documentation for
`FrameClock::begin_updating`.
<!-- impl FrameClock::fn get_current_timings -->
Gets the frame timings for the current frame.

# Returns

the `FrameTimings` for the
 frame currently being processed, or even no frame is being
 processed, for the previous frame. Before any frames have been
 processed, returns `None`.
<!-- impl FrameClock::fn get_frame_counter -->
A `FrameClock` maintains a 64-bit counter that increments for
each frame drawn.

# Returns

inside frame processing, the value of the frame counter
 for the current frame. Outside of frame processing, the frame
 counter for the last frame.
<!-- impl FrameClock::fn get_frame_time -->
Gets the time that should currently be used for animations. Inside
the processing of a frame, it’s the time used to compute the
animation position of everything in a frame. Outside of a frame, it's
the time of the conceptual “previous frame,” which may be either
the actual previous frame time, or if that’s too old, an updated
time.

# Returns

a timestamp in microseconds, in the timescale of
 of `g_get_monotonic_time`.
<!-- impl FrameClock::fn get_history_start -->
`FrameClock` internally keeps a history of `FrameTimings`
objects for recent frames that can be retrieved with
`FrameClock::get_timings`. The set of stored frames
is the set from the counter values given by
`FrameClock::get_history_start` and
`FrameClock::get_frame_counter`, inclusive.

# Returns

the frame counter value for the oldest frame
 that is available in the internal frame history of the
 `FrameClock`.
<!-- impl FrameClock::fn get_refresh_info -->
Using the frame history stored in the frame clock, finds the last
known presentation time and refresh interval, and assuming that
presentation times are separated by the refresh interval,
predicts a presentation time that is a multiple of the refresh
interval after the last presentation time, and later than `base_time`.
## `base_time`
base time for determining a presentaton time
## `refresh_interval_return`
a location to store the
determined refresh interval, or `None`. A default refresh interval of
1/60th of a second will be stored if no history is present.
## `presentation_time_return`
a location to store the next
 candidate presentation time after the given base time.
 0 will be will be stored if no history is present.
<!-- impl FrameClock::fn get_timings -->
Retrieves a `FrameTimings` object holding timing information
for the current frame or a recent frame. The `FrameTimings`
object may not yet be complete: see `FrameTimings::get_complete`.
## `frame_counter`
the frame counter value identifying the frame to
 be received.

# Returns

the `FrameTimings` object for
 the specified frame, or `None` if it is not available. See
 `FrameClock::get_history_start`.
<!-- impl FrameClock::fn request_phase -->
Asks the frame clock to run a particular phase. The signal
corresponding the requested phase will be emitted the next
time the frame clock processes. Multiple calls to
`FrameClock::request_phase` will be combined together
and only one frame processed. If you are displaying animated
content and want to continually request the
`FrameClockPhase::Update` phase for a period of time,
you should use `FrameClock::begin_updating` instead, since
this allows GTK to adjust system parameters to get maximally
smooth animations.
## `phase`
the phase that is requested
<!-- impl FrameClock::fn connect_after_paint -->
This signal ends processing of the frame. Applications
should generally not handle this signal.
<!-- impl FrameClock::fn connect_before_paint -->
This signal begins processing of the frame. Applications
should generally not handle this signal.
<!-- impl FrameClock::fn connect_flush_events -->
This signal is used to flush pending motion events that
are being batched up and compressed together. Applications
should not handle this signal.
<!-- impl FrameClock::fn connect_layout -->
This signal is emitted as the second step of toolkit and
application processing of the frame. Any work to update
sizes and positions of application elements should be
performed. GTK normally handles this internally.
<!-- impl FrameClock::fn connect_paint -->
This signal is emitted as the third step of toolkit and
application processing of the frame. The frame is
repainted. GDK normally handles this internally and
produces expose events, which are turned into GTK
``GtkWidget`::draw` signals.
<!-- impl FrameClock::fn connect_resume_events -->
This signal is emitted after processing of the frame is
finished, and is handled internally by GTK to resume normal
event processing. Applications should not handle this signal.
<!-- impl FrameClock::fn connect_update -->
This signal is emitted as the first step of toolkit and
application processing of the frame. Animations should
be updated using `FrameClock::get_frame_time`.
Applications can connect directly to this signal, or
use `gtk_widget_add_tick_callback` as a more convenient
interface.
<!-- struct FrameTimings -->
A `FrameTimings` object holds timing information for a single frame
of the application’s displays. To retrieve `FrameTimings` objects,
use `FrameClock::get_timings` or `FrameClock::get_current_timings`.
The information in `FrameTimings` is useful for precise synchronization
of video with the event or audio streams, and for measuring
quality metrics for the application’s display, such as latency and jitter.
<!-- impl FrameTimings::fn get_complete -->
The timing information in a `FrameTimings` is filled in
incrementally as the frame as drawn and passed off to the
window system for processing and display to the user. The
accessor functions for `FrameTimings` can return 0 to
indicate an unavailable value for two reasons: either because
the information is not yet available, or because it isn't
available at all. Once `FrameTimings::get_complete` returns
`true` for a frame, you can be certain that no further values
will become available and be stored in the `FrameTimings`.

# Returns

`true` if all information that will be available
 for the frame has been filled in.
<!-- impl FrameTimings::fn get_frame_counter -->
Gets the frame counter value of the `FrameClock` when this
this frame was drawn.

# Returns

the frame counter value for this frame
<!-- impl FrameTimings::fn get_frame_time -->
Returns the frame time for the frame. This is the time value
that is typically used to time animations for the frame. See
`FrameClock::get_frame_time`.

# Returns

the frame time for the frame, in the timescale
 of `g_get_monotonic_time`
<!-- impl FrameTimings::fn get_predicted_presentation_time -->
Gets the predicted time at which this frame will be displayed. Although
no predicted time may be available, if one is available, it will
be available while the frame is being generated, in contrast to
`FrameTimings::get_presentation_time`, which is only available
after the frame has been presented. In general, if you are simply
animating, you should use `FrameClock::get_frame_time` rather
than this function, but this function is useful for applications
that want exact control over latency. For example, a movie player
may want this information for Audio/Video synchronization.

# Returns

The predicted time at which the frame will be presented,
 in the timescale of `g_get_monotonic_time`, or 0 if no predicted
 presentation time is available.
<!-- impl FrameTimings::fn get_presentation_time -->
Reurns the presentation time. This is the time at which the frame
became visible to the user.

# Returns

the time the frame was displayed to the user, in the
 timescale of `g_get_monotonic_time`, or 0 if no presentation
 time is available. See `FrameTimings::get_complete`
<!-- impl FrameTimings::fn get_refresh_interval -->
Gets the natural interval between presentation times for
the display that this frame was displayed on. Frame presentation
usually happens during the “vertical blanking interval”.

# Returns

the refresh interval of the display, in microseconds,
 or 0 if the refresh interval is not available.
 See `FrameTimings::get_complete`.
<!-- impl FrameTimings::fn ref -->
Increases the reference count of `self`.

# Returns

`self`
<!-- impl FrameTimings::fn unref -->
Decreases the reference count of `self`. If `self`
is no longer referenced, it will be freed.
<!-- enum FullscreenMode -->
Indicates which monitor (in a multi-head setup) a surface should span over
when in fullscreen mode.
<!-- enum FullscreenMode::variant CurrentMonitor -->
Fullscreen on current monitor only.
<!-- enum FullscreenMode::variant AllMonitors -->
Span across all monitors when fullscreen.
<!-- struct GLContext -->
`GLContext` is an object representing the platform-specific
OpenGL draw context.

``GdkGLContexts`` are created for a `Surface` using
`SurfaceExt::create_gl_context`, and the context will match the
the characteristics of the surface.

A `GLContext` is not tied to any particular normal framebuffer.
For instance, it cannot draw to the `Surface` back buffer. The GDK
repaint system is in full control of the painting to that. Instead,
you can create render buffers or textures and use `gdk_cairo_draw_from_gl`
in the draw function of your widget to draw them. Then GDK will handle
the integration of your rendering with that of other widgets.

Support for `GLContext` is platform-specific, context creation
can fail, returning `None` context.

A `GLContext` has to be made "current" in order to start using
it, otherwise any OpenGL call will be ignored.

## Creating a new OpenGL context ##

In order to create a new `GLContext` instance you need a
`Surface`, which you typically get during the realize call
of a widget.

A `GLContext` is not realized until either `GLContext::make_current`,
or until it is realized using `GLContext::realize`. It is possible to
specify details of the GL context like the OpenGL version to be used, or
whether the GL context should have extra state validation enabled after
calling `SurfaceExt::create_gl_context` by calling `GLContext::realize`.
If the realization fails you have the option to change the settings of the
`GLContext` and try again.

## Using a `GLContext` ##

You will need to make the `GLContext` the current context
before issuing OpenGL calls; the system sends OpenGL commands to
whichever context is current. It is possible to have multiple
contexts, so you always need to ensure that the one which you
want to draw with is the current one before issuing commands:


```C
  gdk_gl_context_make_current (context);
```

You can now perform your drawing using OpenGL commands.

You can check which `GLContext` is the current one by using
`GLContext::get_current`; you can also unset any `GLContext`
that is currently set by calling `GLContext::clear_current`.

# Implements

[`DrawContextExt`](trait.DrawContextExt.html), [`DrawContextExtManual`](prelude/trait.DrawContextExtManual.html)
<!-- impl GLContext::fn clear_current -->
Clears the current `GLContext`.

Any OpenGL call after this function returns will be ignored
until `GLContext::make_current` is called.
<!-- impl GLContext::fn get_current -->
Retrieves the current `GLContext`.

# Returns

the current `GLContext`, or `None`
<!-- impl GLContext::fn get_debug_enabled -->
Retrieves the value set using `GLContext::set_debug_enabled`.

# Returns

`true` if debugging is enabled
<!-- impl GLContext::fn get_display -->
Retrieves the `Display` the `self` is created for

# Returns

a `Display` or `None`
<!-- impl GLContext::fn get_forward_compatible -->
Retrieves the value set using `GLContext::set_forward_compatible`.

# Returns

`true` if the context should be forward compatible
<!-- impl GLContext::fn get_required_version -->
Retrieves the major and minor version requested by calling
`GLContext::set_required_version`.
## `major`
return location for the major version to request
## `minor`
return location for the minor version to request
<!-- impl GLContext::fn get_shared_context -->
Retrieves the `GLContext` that this `self` share data with.

# Returns

a `GLContext` or `None`
<!-- impl GLContext::fn get_surface -->
Retrieves the `Surface` used by the `self`.

# Returns

a `Surface` or `None`
<!-- impl GLContext::fn get_use_es -->
Checks whether the `self` is using an OpenGL or OpenGL ES profile.

# Returns

`true` if the `GLContext` is using an OpenGL ES profile
<!-- impl GLContext::fn get_version -->
Retrieves the OpenGL version of the `self`.

The `self` must be realized prior to calling this function.
## `major`
return location for the major version
## `minor`
return location for the minor version
<!-- impl GLContext::fn is_legacy -->
Whether the `GLContext` is in legacy mode or not.

The `GLContext` must be realized before calling this function.

When realizing a GL context, GDK will try to use the OpenGL 3.2 core
profile; this profile removes all the OpenGL API that was deprecated
prior to the 3.2 version of the specification. If the realization is
successful, this function will return `false`.

If the underlying OpenGL implementation does not support core profiles,
GDK will fall back to a pre-3.2 compatibility profile, and this function
will return `true`.

You can use the value returned by this function to decide which kind
of OpenGL API to use, or whether to do extension discovery, or what
kind of shader programs to load.

# Returns

`true` if the GL context is in legacy mode
<!-- impl GLContext::fn make_current -->
Makes the `self` the current one.
<!-- impl GLContext::fn realize -->
Realizes the given `GLContext`.

It is safe to call this function on a realized `GLContext`.

# Returns

`true` if the context is realized
<!-- impl GLContext::fn set_debug_enabled -->
Sets whether the `GLContext` should perform extra validations and
run time checking. This is useful during development, but has
additional overhead.

The `GLContext` must not be realized or made current prior to
calling this function.
## `enabled`
whether to enable debugging in the context
<!-- impl GLContext::fn set_forward_compatible -->
Sets whether the `GLContext` should be forward compatible.

Forward compatibile contexts must not support OpenGL functionality that
has been marked as deprecated in the requested version; non-forward
compatible contexts, on the other hand, must support both deprecated and
non deprecated functionality.

The `GLContext` must not be realized or made current prior to calling
this function.
## `compatible`
whether the context should be forward compatible
<!-- impl GLContext::fn set_required_version -->
Sets the major and minor version of OpenGL to request.

Setting `major` and `minor` to zero will use the default values.

The `GLContext` must not be realized or made current prior to calling
this function.
## `major`
the major version to request
## `minor`
the minor version to request
<!-- impl GLContext::fn set_use_es -->
Requests that GDK create a OpenGL ES context instead of an OpenGL one,
if the platform and windowing system allows it.

The `self` must not have been realized.

By default, GDK will attempt to automatically detect whether the
underlying GL implementation is OpenGL or OpenGL ES once the `self`
is realized.

You should check the return value of `GLContext::get_use_es` after
calling `GLContext::realize` to decide whether to use the OpenGL or
OpenGL ES API, extensions, or shaders.
## `use_es`
whether the context should use OpenGL ES instead of OpenGL,
 or -1 to allow auto-detection
<!-- impl GLContext::fn get_property_shared_context -->
The `GLContext` that this context is sharing data with, or `None`
<!-- impl GLContext::fn set_property_shared_context -->
The `GLContext` that this context is sharing data with, or `None`
<!-- enum GLError -->
Error enumeration for `GLContext`.
<!-- enum GLError::variant NotAvailable -->
OpenGL support is not available
<!-- enum GLError::variant UnsupportedFormat -->
The requested visual format is not supported
<!-- enum GLError::variant UnsupportedProfile -->
The requested profile is not supported
<!-- enum GLError::variant CompilationFailed -->
The shader compilation failed
<!-- enum GLError::variant LinkFailed -->
The shader linking failed
<!-- struct GLTexture -->


# Implements

[`TextureExt`](trait.TextureExt.html), [`PaintableExt`](trait.PaintableExt.html)
<!-- impl GLTexture::fn new -->
Creates a new texture for an existing GL texture.

Note that the GL texture must not be modified until `destroy` is called,
which will happen when the `Texture` object is finalized, or due to
an explicit call of `GLTexture::release`.
## `context`
a `GLContext`
## `id`
the ID of a texture that was created with `context`
## `width`
the nominal width of the texture
## `height`
the nominal height of the texture
## `destroy`
a destroy notify that will be called when the GL resources
 are released
## `data`
data that gets passed to `destroy`

# Returns

A newly-created `Texture`
<!-- impl GLTexture::fn release -->
Releases the GL resources held by a `GLTexture` that
was created with `GLTexture::new`.

The texture contents are still available via the
`TextureExt::download` function, after this function
has been called.
<!-- struct Geometry -->
The `Geometry` struct gives the window manager information about
a surface’s geometry constraints. Normally you would set these on
the GTK+ level using `gtk_window_set_geometry_hints`. ``GtkWindow``
then sets the hints on the `Surface` it creates.

`SurfaceExt::set_geometry_hints` expects the hints to be fully valid already
and simply passes them to the window manager; in contrast,
`gtk_window_set_geometry_hints` performs some interpretation. For example,
``GtkWindow`` will apply the hints to the geometry widget instead of the
toplevel window, if you set a geometry widget. Also, the
`min_width`/`min_height`/`max_width`/`max_height` fields may be set to -1, and
``GtkWindow`` will substitute the size request of the surface or geometry widget.
If the minimum size hint is not provided, ``GtkWindow`` will use its requisition
as the minimum size. If the minimum size is provided and a geometry widget is
set, ``GtkWindow`` will take the minimum size as the minimum size of the
geometry widget rather than the entire surface. The base size is treated
similarly.

The canonical use-case for `gtk_window_set_geometry_hints` is to get a
terminal widget to resize properly. Here, the terminal text area should be
the geometry widget; ``GtkWindow`` will then automatically set the base size to
the size of other widgets in the terminal window, such as the menubar and
scrollbar. Then, the `width_inc` and `height_inc` fields should be set to the
size of one character in the terminal. Finally, the base size should be set
to the size of one character. The net effect is that the minimum size of the
terminal will have a 1x1 character terminal area, and only terminal sizes on
the “character grid” will be allowed.

Here’s an example of how the terminal example would be implemented, assuming
a terminal area widget called “terminal” and a toplevel window “toplevel”:


```C
    GdkGeometry hints;

    hints.base_width = terminal->char_width;
        hints.base_height = terminal->char_height;
        hints.min_width = terminal->char_width;
        hints.min_height = terminal->char_height;
        hints.width_inc = terminal->char_width;
        hints.height_inc = terminal->char_height;

 gtk_window_set_geometry_hints (GTK_WINDOW (toplevel),
                                GTK_WIDGET (terminal),
                                &hints,
                                GDK_HINT_RESIZE_INC |
                                GDK_HINT_MIN_SIZE |
                                GDK_HINT_BASE_SIZE);
```

The other useful fields are the `min_aspect` and `max_aspect` fields; these
contain a width/height ratio as a floating point number. If a geometry widget
is set, the aspect applies to the geometry widget rather than the entire
window. The most common use of these hints is probably to set `min_aspect` and
`max_aspect` to the same value, thus forcing the window to keep a constant
aspect ratio.
<!-- enum GrabOwnership -->
Defines how device grabs interact with other devices.
<!-- enum GrabOwnership::variant None -->
All other devices’ events are allowed.
<!-- enum GrabOwnership::variant Surface -->
Other devices’ events are blocked for the grab surface.
<!-- enum GrabOwnership::variant Application -->
Other devices’ events are blocked for the whole application.
<!-- enum GrabStatus -->
Returned by `gdk_device_grab` to indicate success or the reason for the
failure of the grab attempt.
<!-- enum GrabStatus::variant Success -->
the resource was successfully grabbed.
<!-- enum GrabStatus::variant AlreadyGrabbed -->
the resource is actively grabbed by another client.
<!-- enum GrabStatus::variant InvalidTime -->
the resource was grabbed more recently than the
 specified time.
<!-- enum GrabStatus::variant NotViewable -->
the grab surface or the `confine_to` surface are not
 viewable.
<!-- enum GrabStatus::variant Frozen -->
the resource is frozen by an active grab of another client.
<!-- enum GrabStatus::variant Failed -->
the grab failed for some other reason
<!-- enum Gravity -->
Defines the reference point of a surface and the meaning of coordinates
passed to `gtk_window_move`. See `gtk_window_move` and the "implementation
notes" section of the
[Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec)
specification for more details.
<!-- enum Gravity::variant NorthWest -->
the reference point is at the top left corner.
<!-- enum Gravity::variant North -->
the reference point is in the middle of the top edge.
<!-- enum Gravity::variant NorthEast -->
the reference point is at the top right corner.
<!-- enum Gravity::variant West -->
the reference point is at the middle of the left edge.
<!-- enum Gravity::variant Center -->
the reference point is at the center of the surface.
<!-- enum Gravity::variant East -->
the reference point is at the middle of the right edge.
<!-- enum Gravity::variant SouthWest -->
the reference point is at the lower left corner.
<!-- enum Gravity::variant South -->
the reference point is at the middle of the lower edge.
<!-- enum Gravity::variant SouthEast -->
the reference point is at the lower right corner.
<!-- enum Gravity::variant Static -->
the reference point is at the top left corner of the
 surface itself, ignoring window manager decorations.
<!-- enum InputMode -->
An enumeration that describes the mode of an input device.
<!-- enum InputMode::variant Disabled -->
the device is disabled and will not report any events.
<!-- enum InputMode::variant Screen -->
the device is enabled. The device’s coordinate space
 maps to the entire screen.
<!-- enum InputMode::variant Surface -->
the device is enabled. The device’s coordinate space
 is mapped to a single surface. The manner in which this surface
 is chosen is undefined, but it will typically be the same
 way in which the focus surface for key events is determined.
<!-- enum InputSource -->
An enumeration describing the type of an input device in general terms.
<!-- enum InputSource::variant Mouse -->
the device is a mouse. (This will be reported for the core
 pointer, even if it is something else, such as a trackball.)
<!-- enum InputSource::variant Pen -->
the device is a stylus of a graphics tablet or similar device.
<!-- enum InputSource::variant Eraser -->
the device is an eraser. Typically, this would be the other end
 of a stylus on a graphics tablet.
<!-- enum InputSource::variant Cursor -->
the device is a graphics tablet “puck” or similar device.
<!-- enum InputSource::variant Keyboard -->
the device is a keyboard.
<!-- enum InputSource::variant Touchscreen -->
the device is a direct-input touch device, such
 as a touchscreen or tablet. This device type has been added in 3.4.
<!-- enum InputSource::variant Touchpad -->
the device is an indirect touch device, such
 as a touchpad. This device type has been added in 3.4.
<!-- enum InputSource::variant Trackpoint -->
the device is a trackpoint. This device type has been
 added in 3.22
<!-- enum InputSource::variant TabletPad -->
the device is a "pad", a collection of buttons,
 rings and strips found in drawing tablets. This device type has been
 added in 3.22.
<!-- struct Keymap -->
A `Keymap` defines the translation from keyboard state
(including a hardware key, a modifier mask, and active keyboard group)
to a keyval. This translation has two phases. The first phase is
to determine the effective keyboard group and level for the keyboard
state; the second phase is to look up the keycode/group/level triplet
in the keymap and see what keyval it corresponds to.
<!-- impl Keymap::fn add_virtual_modifiers -->
Maps the non-virtual modifiers (i.e Mod2, Mod3, ...) which are set
in `state` to the virtual modifiers (i.e. Super, Hyper and Meta) and
set the corresponding bits in `state`.

GDK already does this before delivering key events, but for
compatibility reasons, it only sets the first virtual modifier
it finds, whereas this function sets all matching virtual modifiers.

This function is useful when matching key events against
accelerators.
## `state`
pointer to the modifier mask to change
<!-- impl Keymap::fn get_caps_lock_state -->
Returns whether the Caps Lock modifer is locked.

# Returns

`true` if Caps Lock is on
<!-- impl Keymap::fn get_direction -->
Returns the direction of effective layout of the keymap.
The direction of a layout is the direction of the majority of its
symbols. See `pango_unichar_direction`.

# Returns

`pango::Direction::Ltr` or `pango::Direction::Rtl`
 if it can determine the direction. `pango::Direction::Neutral`
 otherwise.
<!-- impl Keymap::fn get_display -->
Retrieves the `Display` associated to the `self`.

# Returns

a `Display`
<!-- impl Keymap::fn get_entries_for_keycode -->
Returns the keyvals bound to `hardware_keycode`.
The Nth `KeymapKey` in `keys` is bound to the Nth
keyval in `keyvals`. Free the returned arrays with `g_free`.
When a keycode is pressed by the user, the keyval from
this list of entries is selected by considering the effective
keyboard group and level. See `Keymap::translate_keyboard_state`.
## `hardware_keycode`
a keycode
## `keys`
return
 location for array of `KeymapKey`, or `None`
## `keyvals`
return
 location for array of keyvals, or `None`
## `n_entries`
length of `keys` and `keyvals`

# Returns

`true` if there were any entries
<!-- impl Keymap::fn get_entries_for_keyval -->
Obtains a list of keycode/group/level combinations that will
generate `keyval`. Groups and levels are two kinds of keyboard mode;
in general, the level determines whether the top or bottom symbol
on a key is used, and the group determines whether the left or
right symbol is used. On US keyboards, the shift key changes the
keyboard level, and there are no groups. A group switch key might
convert a keyboard between Hebrew to English modes, for example.
`EventKey` contains a `group` field that indicates the active
keyboard group. The level is computed from the modifier mask.
The returned array should be freed
with `g_free`.
## `keyval`
a keyval, such as `GDK_KEY_a`, `GDK_KEY_Up`, `GDK_KEY_Return`, etc.
## `keys`
return location
 for an array of `KeymapKey`
## `n_keys`
return location for number of elements in returned array

# Returns

`true` if keys were found and returned
<!-- impl Keymap::fn get_modifier_mask -->
Returns the modifier mask the `self`’s windowing system backend
uses for a particular purpose.

Note that this function always returns real hardware modifiers, not
virtual ones (e.g. it will return `ModifierType::Mod1Mask` rather than
`ModifierType::MetaMask` if the backend maps MOD1 to META), so there are use
cases where the return value of this function has to be transformed
by `Keymap::add_virtual_modifiers` in order to contain the
expected result.
## `intent`
the use case for the modifier mask

# Returns

the modifier mask used for `intent`.
<!-- impl Keymap::fn get_modifier_state -->
Returns the current modifier state.

# Returns

the current modifier state.
<!-- impl Keymap::fn get_num_lock_state -->
Returns whether the Num Lock modifer is locked.

# Returns

`true` if Num Lock is on
<!-- impl Keymap::fn get_scroll_lock_state -->
Returns whether the Scroll Lock modifer is locked.

# Returns

`true` if Scroll Lock is on
<!-- impl Keymap::fn have_bidi_layouts -->
Determines if keyboard layouts for both right-to-left and left-to-right
languages are in use.

# Returns

`true` if there are layouts in both directions, `false` otherwise
<!-- impl Keymap::fn lookup_key -->
Looks up the keyval mapped to a keycode/group/level triplet.
If no keyval is bound to `key`, returns 0. For normal user input,
you want to use `Keymap::translate_keyboard_state` instead of
this function, since the effective group/level may not be
the same as the current keyboard state.
## `key`
a `KeymapKey` with keycode, group, and level initialized

# Returns

a keyval, or 0 if none was mapped to the given `key`
<!-- impl Keymap::fn map_virtual_modifiers -->
Maps the virtual modifiers (i.e. Super, Hyper and Meta) which
are set in `state` to their non-virtual counterparts (i.e. Mod2,
Mod3,...) and set the corresponding bits in `state`.

This function is useful when matching key events against
accelerators.
## `state`
pointer to the modifier state to map

# Returns

`false` if two virtual modifiers were mapped to the
 same non-virtual modifier. Note that `false` is also returned
 if a virtual modifier is mapped to a non-virtual modifier that
 was already set in `state`.
<!-- impl Keymap::fn translate_keyboard_state -->
Translates the contents of a `EventKey` into a keyval, effective
group, and level. Modifiers that affected the translation and
are thus unavailable for application use are returned in
`consumed_modifiers`.
See [Groups][key-group-explanation] for an explanation of
groups and levels. The `effective_group` is the group that was
actually used for the translation; some keys such as Enter are not
affected by the active keyboard group. The `level` is derived from
`state`. For convenience, `EventKey` already contains the translated
keyval, so this function isn’t as useful as you might think.

`consumed_modifiers` gives modifiers that should be masked outfrom `state`
when comparing this key press to a hot key. For instance, on a US keyboard,
the `plus` symbol is shifted, so when comparing a key press to a
`<Control>plus` accelerator `<Shift>` should be masked out.


```C
// We want to ignore irrelevant modifiers like ScrollLock
#define ALL_ACCELS_MASK (GDK_CONTROL_MASK | GDK_SHIFT_MASK | GDK_MOD1_MASK)
gdk_keymap_translate_keyboard_state (keymap, event->hardware_keycode,
                                     event->state, event->group,
                                     &keyval, NULL, NULL, &consumed);
if (keyval == GDK_PLUS &&
    (event->state & ~consumed & ALL_ACCELS_MASK) == GDK_CONTROL_MASK)
  // Control was pressed
```

An older interpretation `consumed_modifiers` was that it contained
all modifiers that might affect the translation of the key;
this allowed accelerators to be stored with irrelevant consumed
modifiers, by doing:

```C
// XXX Don’t do this XXX
if (keyval == accel_keyval &&
    (event->state & ~consumed & ALL_ACCELS_MASK) == (accel_mods & ~consumed))
  // Accelerator was pressed
```

However, this did not work if multi-modifier combinations were
used in the keymap, since, for instance, `<Control>` would be
masked out even if only `<Control><Alt>` was used in the keymap.
To support this usage as well as well as possible, all single
modifier combinations that could affect the key for any combination
of modifiers will be returned in `consumed_modifiers`; multi-modifier
combinations are returned only when actually found in `state`. When
you store accelerators, you should always store them with consumed
modifiers removed. Store `<Control>plus`, not `<Control><Shift>plus`,
## `hardware_keycode`
a keycode
## `state`
a modifier state
## `group`
active keyboard group
## `keyval`
return location for keyval, or `None`
## `effective_group`
return location for effective
 group, or `None`
## `level`
return location for level, or `None`
## `consumed_modifiers`
return location for modifiers
 that were used to determine the group or level, or `None`

# Returns

`true` if there was a keyval bound to the keycode/state/group
<!-- impl Keymap::fn connect_direction_changed -->
The ::direction-changed signal gets emitted when the direction
of the keymap changes. See `Keymap::get_direction`.
<!-- impl Keymap::fn connect_keys_changed -->
The ::keys-changed signal is emitted when the mapping represented by
`keymap` changes.
<!-- impl Keymap::fn connect_state_changed -->
The ::state-changed signal is emitted when the state of the
keyboard changes, e.g when Caps Lock is turned on or off.
See `Keymap::get_caps_lock_state`.
<!-- struct KeymapKey -->
A `KeymapKey` is a hardware key that can be mapped to a keyval.
<!-- enum MemoryFormat -->
`MemoryFormat` describes a format that bytes can have in memory.

It describes formats by listing the contents of the memory passed to it.
So GDK_MEMORY_A8R8G8B8 will be 1 byte (8 bits) of alpha, followed by a
byte each of red, green and blue. It is not endian-dependent, so
CAIRO_FORMAT_ARGB32 is represented by different ``GdkMemoryFormats`` on
architectures with different endiannesses.

Its naming is modelled after VkFormat (see
https://www.khronos.org/registry/vulkan/specs/1.0/html/vkspec.html`VkFormat`
for details).
<!-- enum MemoryFormat::variant B8g8r8a8Premultiplied -->
4 bytes; for blue, green, red, alpha.
 The color values are premultiplied with the alpha value.
<!-- enum MemoryFormat::variant A8r8g8b8Premultiplied -->
4 bytes; for alpha, red, green, blue.
 The color values are premultiplied with the alpha value.
<!-- enum MemoryFormat::variant B8g8r8a8 -->
4 bytes; for blue, green, red, alpha.
<!-- enum MemoryFormat::variant A8r8g8b8 -->
4 bytes; for alpha, red, green, blue.
<!-- enum MemoryFormat::variant R8g8b8a8 -->
4 bytes; for red, green, blue, alpha.
<!-- enum MemoryFormat::variant A8b8g8r8 -->
4 bytes; for alpha, blue, green, red.
<!-- enum MemoryFormat::variant R8g8b8 -->
3 bytes; for red, green, blue. The data is opaque.
<!-- enum MemoryFormat::variant B8g8r8 -->
3 bytes; for blue, green, red. The data is opaque.
<!-- enum MemoryFormat::variant NFormats -->
The number of formats. This value will change as
 more formats get added, so do not rely on its concrete integer.
<!-- struct MemoryTexture -->


# Implements

[`TextureExt`](trait.TextureExt.html), [`PaintableExt`](trait.PaintableExt.html)
<!-- impl MemoryTexture::fn new -->
Creates a new texture for a blob of image data.
The `glib::Bytes` must contain `stride` x `height` pixels
in the given format.
## `width`
the width of the texture
## `height`
the height of the texture
## `format`
the format of the data
## `bytes`
the `glib::Bytes` containing the pixel data
## `stride`
rowstride for the data

# Returns

A newly-created `Texture`
<!-- enum ModifierIntent -->
This enum is used with `Keymap::get_modifier_mask`
in order to determine what modifiers the
currently used windowing system backend uses for particular
purposes. For example, on X11/Windows, the Control key is used for
invoking menu shortcuts (accelerators), whereas on Apple computers
it’s the Command key (which correspond to `ModifierType::ControlMask` and
`ModifierType::Mod2Mask`, respectively).
<!-- enum ModifierIntent::variant PrimaryAccelerator -->
the primary modifier used to invoke
 menu accelerators.
<!-- enum ModifierIntent::variant ContextMenu -->
the modifier used to invoke context menus.
 Note that mouse button 3 always triggers context menus. When this modifier
 is not 0, it additionally triggers context menus when used with mouse button 1.
<!-- enum ModifierIntent::variant ExtendSelection -->
the modifier used to extend selections
 using `modifier`-click or `modifier`-cursor-key
<!-- enum ModifierIntent::variant ModifySelection -->
the modifier used to modify selections,
 which in most cases means toggling the clicked item into or out of the selection.
<!-- enum ModifierIntent::variant NoTextInput -->
when any of these modifiers is pressed, the
 key event cannot produce a symbol directly. This is meant to be used for
 input methods, and for use cases like typeahead search.
<!-- enum ModifierIntent::variant ShiftGroup -->
the modifier that switches between keyboard
 groups (AltGr on X11/Windows and Option/Alt on OS X).
<!-- enum ModifierIntent::variant DefaultModMask -->
The set of modifier masks accepted
as modifiers in accelerators. Needed because Command is mapped to MOD2 on
OSX, which is widely used, but on X11 MOD2 is NumLock and using that for a
mod key is problematic at best.
Ref: https://bugzilla.gnome.org/show_bug.cgi?id=736125.
<!-- struct Monitor -->
`Monitor` objects represent the individual outputs that are
associated with a `Display`. `Display` has APIs to enumerate
monitors with `Display::get_n_monitors` and `Display::get_monitor`, and
to find particular monitors with `Display::get_primary_monitor` or
`Display::get_monitor_at_surface`.
<!-- impl Monitor::fn get_display -->
Gets the display that this monitor belongs to.

# Returns

the display
<!-- impl Monitor::fn get_geometry -->
Retrieves the size and position of an individual monitor within the
display coordinate space. The returned geometry is in ”application pixels”,
not in ”device pixels” (see `Monitor::get_scale_factor`).
## `geometry`
a `Rectangle` to be filled with the monitor geometry
<!-- impl Monitor::fn get_height_mm -->
Gets the height in millimeters of the monitor.

# Returns

the physical height of the monitor
<!-- impl Monitor::fn get_manufacturer -->
Gets the name of the monitor's manufacturer, if available.

# Returns

the name of the manufacturer, or `None`
<!-- impl Monitor::fn get_model -->
Gets the a string identifying the monitor model, if available.

# Returns

the monitor model, or `None`
<!-- impl Monitor::fn get_refresh_rate -->
Gets the refresh rate of the monitor, if available.

The value is in milli-Hertz, so a refresh rate of 60Hz
is returned as 60000.

# Returns

the refresh rate in milli-Hertz, or 0
<!-- impl Monitor::fn get_scale_factor -->
Gets the internal scale factor that maps from monitor coordinates
to the actual device pixels. On traditional systems this is 1, but
on very high density outputs this can be a higher value (often 2).

This can be used if you want to create pixel based data for a
particular monitor, but most of the time you’re drawing to a surface
where it is better to use `SurfaceExt::get_scale_factor` instead.

# Returns

the scale factor
<!-- impl Monitor::fn get_subpixel_layout -->
Gets information about the layout of red, green and blue
primaries for each pixel in this monitor, if available.

# Returns

the subpixel layout
<!-- impl Monitor::fn get_width_mm -->
Gets the width in millimeters of the monitor.

# Returns

the physical width of the monitor
<!-- impl Monitor::fn get_workarea -->
Retrieves the size and position of the “work area” on a monitor
within the display coordinate space. The returned geometry is in
”application pixels”, not in ”device pixels” (see
`Monitor::get_scale_factor`).

The work area should be considered when positioning menus and
similar popups, to avoid placing them below panels, docks or other
desktop components.

Note that not all backends may have a concept of workarea. This
function will return the monitor geometry if a workarea is not
available, or does not apply.
## `workarea`
a `Rectangle` to be filled with
 the monitor workarea
<!-- impl Monitor::fn is_primary -->
Gets whether this monitor should be considered primary
(see `Display::get_primary_monitor`).

# Returns

`true` if `self` is primary
<!-- impl Monitor::fn is_valid -->
Returns `true` if the `self` object corresponds to a
physical monitor. The `self` becomes invalid when the
physical monitor is unplugged or removed.

# Returns

`true` if the object corresponds to a physical monitor
<!-- impl Monitor::fn connect_invalidate -->
The ::invalidate signal gets emitted when the output represented
by `monitor` gets disconnected.
<!-- enum NotifyType -->
Specifies the kind of crossing for enter and leave events.

See the X11 protocol specification of LeaveNotify for
full details of crossing event generation.
<!-- enum NotifyType::variant Ancestor -->
the surface is entered from an ancestor or
 left towards an ancestor.
<!-- enum NotifyType::variant Virtual -->
the pointer moves between an ancestor and an
 inferior of the surface.
<!-- enum NotifyType::variant Inferior -->
the surface is entered from an inferior or
 left towards an inferior.
<!-- enum NotifyType::variant Nonlinear -->
the surface is entered from or left towards
 a surface which is neither an ancestor nor an inferior.
<!-- enum NotifyType::variant NonlinearVirtual -->
the pointer moves between two surfaces
 which are not ancestors of each other and the surface is part of
 the ancestor chain between one of these surfaces and their least
 common ancestor.
<!-- enum NotifyType::variant Unknown -->
an unknown type of enter/leave event occurred.
<!-- struct Paintable -->
`Paintable` is a simple interface used by GDK and GDK to represent
objects that can be painted anywhere at any size without requiring any
sort of layout. The interface is inspired by similar concepts elsewhere,
such as [ClutterContent](https://developer.gnome.org/clutter/stable/ClutterContent.html),
[HTML/CSS Paint Sources](https://www.w3.org/TR/css-images-4/`paint`-source),
or [SVG Paint Servers](https://www.w3.org/TR/SVG2/pservers.html).

A `Paintable` can be snapshot at any time and size using
`Paintable::snapshot`. How the paintable interprets that size and if it
scales or centers itself into the given rectangle is implementation defined,
though if you are implementing a `Paintable` and don't know what to do, it
is suggested that you scale your paintable ignoring any potential aspect ratio.

The contents that a `Paintable` produces may depend on the `Snapshot` passed
to it. For example, paintables may decide to use more detailed images on higher
resolution screens or when OpenGL is available. A `Paintable` will however
always produce the same output for the same snapshot.

A `Paintable` may change its contents, meaning that it will now produce a
different output with the same snpashot. Once that happens, it will call
`Paintable::invalidate_contents` which will emit the
`Paintable::invalidate-contents` signal.
If a paintable is known to never change its contents, it will set the
`GDK_PAINTABLE_STATIC_CONTENT` flag. If a consumer cannot deal with changing
contents, it may call `gdk_paintable_get_static_image` which will return a
static paintable and use that.

A paintable can report an intrinsic (or preferred) size or aspect ratio it
wishes to be rendered at, though it doesn't have to. Consumers of the interface
can use this information to layout thepaintable appropriately.
Just like the contents, the size of a paintable can change. A paintable will
indicate this by calling `Paintable::invalidate_size` which will emit the
`Paintable::invalidate-size` signal.
And just like for contents, if a paintable is known to never change its size,
it will set the `PaintableFlags::Size` flag.

Besides API for applications, there are some functions that are only
useful for implementing subclasses and should not be used by applications:
`Paintable::invalidate_contents`,
`Paintable::invalidate_size`,
`Paintable::new_empty`.

# Implements

[`PaintableExt`](trait.PaintableExt.html)
<!-- trait PaintableExt -->
Trait containing all `Paintable` methods.

# Implementors

[`GLTexture`](struct.GLTexture.html), [`MemoryTexture`](struct.MemoryTexture.html), [`Paintable`](struct.Paintable.html), [`Texture`](struct.Texture.html)
<!-- impl Paintable::fn new_empty -->
Returns a paintable that has the given intrinsic size and draws nothing.
This is often useful for implementing the `GdkPaintableClass`:`get_current_image`
virtual function when the paintable is in an incomplete state (like a
``GtkMediaStream`` before receiving the first frame).
## `intrinsic_width`
The intrinsic width to report. Can be 0 for no width.
## `intrinsic_height`
The intrinsic height to report. Can be 0 for no height.

# Returns

a `Paintable`
<!-- trait PaintableExt::fn compute_concrete_size -->
Applies the sizing algorithm outlined in
https://drafts.csswg.org/css-images-3/`default`-sizing
to the given `self`. See that link for more details.

It is not necessary to call this function when both `specified_width`
and `specified_height` are known, but it is useful to call this
function in `GtkWidget`:measure implementations to compute the
other dimension when only one dimension is given.
## `specified_width`
the width `self` could be drawn into or
 0.0 if unknown
## `specified_height`
the height `self` could be drawn into or
 0.0 if unknown
## `default_width`
the width `self` would be drawn into if
 no other constraints were given
## `default_height`
the height `self` would be drawn into if
 no other constraints were given
## `concrete_width`
will be set to the concrete width
 computed.
## `concrete_height`
will be set to the concrete height
 computed.
<!-- trait PaintableExt::fn get_current_image -->
Gets an immutable paintable for the current contents displayed by `self`.

This is useful when you want to retain the current state of an animation, for
example to take a screenshot of a running animation.

If the `self` is already immutable, it will return itself.

# Returns

An immutable paintable for the current
 contents of `self`.
<!-- trait PaintableExt::fn get_flags -->
Get flags for the paintable. This is oftentimes useful for optimizations.

See `PaintableFlags` for the flags and what they mean.

# Returns

The `PaintableFlags` for this paintable.
<!-- trait PaintableExt::fn get_intrinsic_aspect_ratio -->
Gets the preferred aspect ratio the `self` would like to be displayed at.
The aspect ration is the width divided by the height, so a value of 0.5 means
that the `self` prefers to be displayed twice as high as it is wide.
Consumers of this interface can use this to preserve aspect ratio when displaying
this paintable.

This is a purely informational value and does not in any way limit the values
that may be passed to `Paintable::snapshot`.

Usually when a `self` returns non-0 values from
`Paintable::get_intrinsic_width` and `Paintable::get_intrinsic_height`
the aspect ratio should conform to those values, though that is not required.

If the `self` does not have a preferred aspect ratio, it returns 0.0.
Negative values are never returned.

# Returns

the intrinsic aspect ratio of `self` or 0.0 if none.
<!-- trait PaintableExt::fn get_intrinsic_height -->
Gets the preferred height the `self` would like to be displayed at.
Consumers of this interface can use this to reserve enough space to draw
the paintable.

This is a purely informational value and does not in any way limit the values
that may be passed to `Paintable::snapshot`.

If the `self` does not have a preferred height, it returns 0. Negative
values are never returned.

# Returns

the intrinsic height of `self` or 0 if none.
<!-- trait PaintableExt::fn get_intrinsic_width -->
Gets the preferred width the `self` would like to be displayed at.
Consumers of this interface can use this to reserve enough space to draw
the paintable.

This is a purely informational value and does not in any way limit the values
that may be passed to `Paintable::snapshot`.

If the `self` does not have a preferred width, it returns 0. Negative
values are never returned.

# Returns

the intrinsic width of `self` or 0 if none.
<!-- trait PaintableExt::fn invalidate_contents -->
Called by implementations of `Paintable` to invalidate their contents.
Unless the contents are invalidated, implementations must guarantee that
multiple calls to `Paintable`::snapshot produce the same output.

This function will emit the `Paintable`::invalidate-contents signal.

If a `self` reports the `PaintableFlags::Contents` flag,
it must not call this function.
<!-- trait PaintableExt::fn invalidate_size -->
Called by implementations of `Paintable` to invalidate their size.
As long as the size is not invalidated, `self` must return the same values
for its width, height and intrinsic height.

This function will emit the `Paintable`::invalidate-size signal.

If a `self` reports the `PaintableFlags::Size` flag,
it must not call this function.
<!-- trait PaintableExt::fn snapshot -->
Snapshots the given paintable with the given `width` and `height` at the
current (0,0) offset of the `snapshot`. If `width` and `height` are not larger
than zero, this function will do nothing.
## `snapshot`
a `Snapshot` to snapshot to
## `width`
width to snapshot in
## `height`
height to snapshot in
<!-- trait PaintableExt::fn connect_invalidate_contents -->
Emitted when the contents of the `paintable` change.

Examples for such an event would be videos changing to the next frame or
the icon theme for an icon changing.
<!-- trait PaintableExt::fn connect_invalidate_size -->
Emitted when the intrinsic size of the `paintable` changes. This means the values
reported by at least one of `Paintable::get_intrinsic_width`,
`Paintable::get_intrinsic_height` or `Paintable::get_intrinsic_aspect_ratio`
has changed.

Examples for such an event would be a paintable displaying the contents of a toplevel
surface being resized.
<!-- struct RGBA -->
A `RGBA` is used to represent a (possibly translucent)
color, in a way that is compatible with cairo’s notion of color.
<!-- impl RGBA::fn copy -->
Makes a copy of a `RGBA`.

The result must be freed through `RGBA::free`.

# Returns

A newly allocated `RGBA`, with the same contents as `self`
<!-- impl RGBA::fn equal -->
Compares two RGBA colors.
## `p2`
another `RGBA` pointer

# Returns

`true` if the two colors compare equal
<!-- impl RGBA::fn free -->
Frees a `RGBA` created with `RGBA::copy`
<!-- impl RGBA::fn hash -->
A hash function suitable for using for a hash
table that stores ``GdkRGBAs``.

# Returns

The hash value for `self`
<!-- impl RGBA::fn is_clear -->
Checks if an `self` value is transparent. That is, drawing with the value
would not produce any change.

# Returns

`true` if the `self` is clear
<!-- impl RGBA::fn is_opaque -->
Checks if an `self` value is opaque. That is, drawing with the value
will not retain any results from previous contents.

# Returns

`true` if the `self` is opaque
<!-- impl RGBA::fn parse -->
Parses a textual representation of a color, filling in
the `red`, `green`, `blue` and `alpha` fields of the `self` `RGBA`.

The string can be either one of:
- A standard name (Taken from the X11 rgb.txt file).
- A hexadecimal value in the form “\#rgb”, “\#rrggbb”,
 “\#rrrgggbbb” or ”\#rrrrggggbbbb”
- A RGB color in the form “rgb(r,g,b)” (In this case the color will
 have full opacity)
- A RGBA color in the form “rgba(r,g,b,a)”

Where “r”, “g”, “b” and “a” are respectively the red, green, blue and
alpha color values. In the last two cases, “r”, “g”, and “b” are either integers
in the range 0 to 255 or percentage values in the range 0% to 100%, and
a is a floating point value in the range 0 to 1.
## `spec`
the string specifying the color

# Returns

`true` if the parsing succeeded
<!-- impl RGBA::fn to_string -->
Returns a textual specification of `self` in the form
`rgb(r,g,b)` or
`rgba(r g,b,a)`,
where “r”, “g”, “b” and “a” represent the red, green,
blue and alpha values respectively. “r”, “g”, and “b” are
represented as integers in the range 0 to 255, and “a”
is represented as a floating point value in the range 0 to 1.

These string forms are string forms that are supported by
the CSS3 colors module, and can be parsed by `RGBA::parse`.

Note that this string representation may lose some
precision, since “r”, “g” and “b” are represented as 8-bit
integers. If this is a concern, you should use a
different representation.

# Returns

A newly allocated text string
<!-- struct Rectangle -->
Defines the position and size of a rectangle. It is identical to
`cairo::RectangleInt`.
<!-- impl Rectangle::fn contains_point -->
Returns `true` if `self` contains the point described by `x` and `y`.
## `x`
X coordinate
## `y`
Y coordinate

# Returns

`true` if `self` contains the point
<!-- impl Rectangle::fn equal -->
Checks if the two given rectangles are equal.
## `rect2`
a `Rectangle`

# Returns

`true` if the rectangles are equal.
<!-- impl Rectangle::fn intersect -->
Calculates the intersection of two rectangles. It is allowed for
`dest` to be the same as either `self` or `src2`. If the rectangles
do not intersect, `dest`’s width and height is set to 0 and its x
and y values are undefined. If you are only interested in whether
the rectangles intersect, but not in the intersecting area itself,
pass `None` for `dest`.
## `src2`
a `Rectangle`
## `dest`
return location for the
intersection of `self` and `src2`, or `None`

# Returns

`true` if the rectangles intersect.
<!-- impl Rectangle::fn union -->
Calculates the union of two rectangles.
The union of rectangles `self` and `src2` is the smallest rectangle which
includes both `self` and `src2` within it.
It is allowed for `dest` to be the same as either `self` or `src2`.

Note that this function does not ignore 'empty' rectangles (ie. with
zero width or height).
## `src2`
a `Rectangle`
## `dest`
return location for the union of `self` and `src2`
<!-- enum ScrollDirection -->
Specifies the direction for scroll events.
<!-- enum ScrollDirection::variant Up -->
the surface is scrolled up.
<!-- enum ScrollDirection::variant Down -->
the surface is scrolled down.
<!-- enum ScrollDirection::variant Left -->
the surface is scrolled to the left.
<!-- enum ScrollDirection::variant Right -->
the surface is scrolled to the right.
<!-- enum ScrollDirection::variant Smooth -->
the scrolling is determined by the delta values
 in scroll events. See `Event::get_scroll_deltas`
<!-- struct Seat -->
The `Seat` object represents a collection of input devices
that belong to a user.
<!-- impl Seat::fn get_capabilities -->
Returns the capabilities this `Seat` currently has.

# Returns

the seat capabilities
<!-- impl Seat::fn get_display -->
Returns the `Display` this seat belongs to.

# Returns

a `Display`. This object is owned by GTK
 and must not be freed.
<!-- impl Seat::fn get_keyboard -->
Returns the master device that routes keyboard events.

# Returns

a master `Device` with keyboard
 capabilities. This object is owned by GTK and must not be freed.
<!-- impl Seat::fn get_master_pointers -->
Returns all master pointers with the given capabilities driven by this `self`.
On most backends this function will return a list with a single element (meaning
that all input devices drive the same onscreen cursor).

In other backends where there can possibly be multiple foci (eg. wayland),
this function will return all master ``GdkDevices`` that represent these.
## `capabilities`
Queried capabilities

# Returns

A list
of master pointing devices
<!-- impl Seat::fn get_pointer -->
Returns the master device that routes pointer events.

# Returns

a master `Device` with pointer
 capabilities. This object is owned by GTK and must not be freed.
<!-- impl Seat::fn get_slaves -->
Returns the slave devices that match the given capabilities.
## `capabilities`
capabilities to get devices for

# Returns

A list of ``GdkDevices``.
 The list must be freed with `glib::List::free`, the elements are owned
 by GDK and must not be freed.
<!-- impl Seat::fn grab -->
Grabs the seat so that all events corresponding to the given `capabilities`
are passed to this application until the seat is ungrabbed with `Seat::ungrab`,
or the surface becomes hidden. This overrides any previous grab on the
seat by this client.

As a rule of thumb, if a grab is desired over `SeatCapabilities::Pointer`,
all other "pointing" capabilities (eg. `SeatCapabilities::Touch`) should
be grabbed too, so the user is able to interact with all of those while
the grab holds, you should thus use `SeatCapabilities::AllPointing` most
commonly.

Grabs are used for operations which need complete control over the
events corresponding to the given capabilities. For example in GTK this
is used for Drag and Drop operations, popup menus and such.

Note that if the event mask of a `Surface` has selected both button press
and button release events, or touch begin and touch end, then a press event
will cause an automatic grab until the button is released, equivalent to a
grab on the surface with `owner_events` set to `true`. This is done because most
applications expect to receive paired press and release events.

If you set up anything at the time you take the grab that needs to be
cleaned up when the grab ends, you should handle the `EventGrabBroken`
events that are emitted when the grab ends unvoluntarily.
## `surface`
the `Surface` which will own the grab
## `capabilities`
capabilities that will be grabbed
## `owner_events`
if `false` then all device events are reported with respect to
 `surface` and are only reported if selected by `event_mask`. If
 `true` then pointer events for this application are reported
 as normal, but pointer events outside this application are
 reported with respect to `surface` and only if selected by
 `event_mask`. In either mode, unreported events are discarded.
## `cursor`
the cursor to display while the grab is active. If
 this is `None` then the normal cursors are used for
 `surface` and its descendants, and the cursor for `surface` is used
 elsewhere.
## `event`
the event that is triggering the grab, or `None` if none
 is available.
## `prepare_func`
function to
 prepare the surface to be grabbed, it can be `None` if `surface` is
 visible before this call.
## `prepare_func_data`
user data to pass to `prepare_func`

# Returns

`GrabStatus::Success` if the grab was successful.
<!-- impl Seat::fn ungrab -->
Releases a grab added through `Seat::grab`.
<!-- impl Seat::fn connect_device_added -->
The ::device-added signal is emitted when a new input
device is related to this seat.
## `device`
the newly added `Device`.
<!-- impl Seat::fn connect_device_removed -->
The ::device-removed signal is emitted when an
input device is removed (e.g. unplugged).
## `device`
the just removed `Device`.
<!-- impl Seat::fn connect_tool_added -->
The ::tool-added signal is emitted whenever a new tool
is made known to the seat. The tool may later be assigned
to a device (i.e. on proximity with a tablet). The device
will emit the `Device::tool-changed` signal accordingly.

A same tool may be used by several devices.
## `tool`
the new `DeviceTool` known to the seat
<!-- impl Seat::fn connect_tool_removed -->
This signal is emitted whenever a tool is no longer known
to this `seat`.
## `tool`
the just removed `DeviceTool`
<!-- impl Seat::fn get_property_display -->
`Display` of this seat.
<!-- impl Seat::fn set_property_display -->
`Display` of this seat.
<!-- struct Snapshot -->

<!-- enum SubpixelLayout -->
This enumeration describes how the red, green and blue components
of physical pixels on an output device are laid out.
<!-- enum SubpixelLayout::variant Unknown -->
The layout is not known
<!-- enum SubpixelLayout::variant None -->
Not organized in this way
<!-- enum SubpixelLayout::variant HorizontalRgb -->
The layout is horizontal, the order is RGB
<!-- enum SubpixelLayout::variant HorizontalBgr -->
The layout is horizontal, the order is BGR
<!-- enum SubpixelLayout::variant VerticalRgb -->
The layout is vertical, the order is RGB
<!-- enum SubpixelLayout::variant VerticalBgr -->
The layout is vertical, the order is BGR
<!-- struct Surface -->
A `Surface` is a (usually) rectangular region on the screen.
It’s a low-level object, used to implement high-level objects such as
``GtkWidget`` and ``GtkWindow`` on the GTK level. A ``GtkWindow`` is a toplevel
surface, the thing a user might think of as a “window” with a titlebar
and so on; a ``GtkWindow`` may contain many sub-`GdkSurfaces`.

# Implements

[`SurfaceExt`](trait.SurfaceExt.html), [`SurfaceExtManual`](prelude/trait.SurfaceExtManual.html)
<!-- trait SurfaceExt -->
Trait containing all `Surface` methods.

# Implementors

[`Surface`](struct.Surface.html)
<!-- impl Surface::fn new_child -->
Creates a new client-side child surface.
## `parent`
the parent surface
## `position`
placement of the surface inside `parent`

# Returns

the new `Surface`
<!-- impl Surface::fn new_popup -->
Creates a new toplevel popup surface. The surface will bypass surface
management.
## `display`
the display to create the surface on
## `position`
position of the surface on screen

# Returns

the new `Surface`
<!-- impl Surface::fn new_temp -->
Creates a new toplevel temporary surface. The surface will be
situated off-screen and not handle output.

You most likely do not want to use this function.
## `display`
the display to create the surface on

# Returns

the new `Surface`
<!-- impl Surface::fn new_toplevel -->
Creates a new toplevel surface. The surface will be managed by the surface
manager.
## `display`
the display to create the surface on
## `width`
width of new surface
## `height`
height of new surface

# Returns

the new `Surface`
<!-- impl Surface::fn constrain_size -->
Constrains a desired width and height according to a
set of geometry hints (such as minimum and maximum size).
## `geometry`
a `Geometry` structure
## `flags`
a mask indicating what portions of `geometry` are set
## `width`
desired width of surface
## `height`
desired height of the surface
## `new_width`
location to store resulting width
## `new_height`
location to store resulting height
<!-- trait SurfaceExt::fn beep -->
Emits a short beep associated to `self` in the appropriate
display, if supported. Otherwise, emits a short beep on
the display just as `Display::beep`.
<!-- trait SurfaceExt::fn begin_move_drag -->
Begins a surface move operation (for a toplevel surface).

This function assumes that the drag is controlled by the
client pointer device, use `SurfaceExt::begin_move_drag_for_device`
to begin a drag with a different device.
## `button`
the button being used to drag, or 0 for a keyboard-initiated drag
## `x`
surface X coordinate of mouse click that began the drag
## `y`
surface Y coordinate of mouse click that began the drag
## `timestamp`
timestamp of mouse click that began the drag
<!-- trait SurfaceExt::fn begin_move_drag_for_device -->
Begins a surface move operation (for a toplevel surface).
## `device`
the device used for the operation
## `button`
the button being used to drag, or 0 for a keyboard-initiated drag
## `x`
surface X coordinate of mouse click that began the drag
## `y`
surface Y coordinate of mouse click that began the drag
## `timestamp`
timestamp of mouse click that began the drag
<!-- trait SurfaceExt::fn begin_resize_drag -->
Begins a surface resize operation (for a toplevel surface).

This function assumes that the drag is controlled by the
client pointer device, use `SurfaceExt::begin_resize_drag_for_device`
to begin a drag with a different device.
## `edge`
the edge or corner from which the drag is started
## `button`
the button being used to drag, or 0 for a keyboard-initiated drag
## `x`
surface X coordinate of mouse click that began the drag
## `y`
surface Y coordinate of mouse click that began the drag
## `timestamp`
timestamp of mouse click that began the drag (use `Event::get_time`)
<!-- trait SurfaceExt::fn begin_resize_drag_for_device -->
Begins a surface resize operation (for a toplevel surface).
You might use this function to implement a “window resize grip,”
## `edge`
the edge or corner from which the drag is started
## `device`
the device used for the operation
## `button`
the button being used to drag, or 0 for a keyboard-initiated drag
## `x`
surface X coordinate of mouse click that began the drag
## `y`
surface Y coordinate of mouse click that began the drag
## `timestamp`
timestamp of mouse click that began the drag (use `Event::get_time`)
<!-- trait SurfaceExt::fn coords_from_parent -->
Transforms surface coordinates from a parent surface to a child
surface.

Calling this function is equivalent to subtracting the return
values of `SurfaceExt::get_position` from the parent coordinates.

See also: `SurfaceExt::coords_to_parent`
## `parent_x`
X coordinate in parent’s coordinate system
## `parent_y`
Y coordinate in parent’s coordinate system
## `x`
return location for X coordinate in child’s coordinate system
## `y`
return location for Y coordinate in child’s coordinate system
<!-- trait SurfaceExt::fn coords_to_parent -->
Transforms surface coordinates from a child surface to its parent
surface. Calling this function is equivalent to adding the return
values of `SurfaceExt::get_position` to the child coordinates.

See also: `SurfaceExt::coords_from_parent`
## `x`
X coordinate in child’s coordinate system
## `y`
Y coordinate in child’s coordinate system
## `parent_x`
return location for X coordinate
in parent’s coordinate system, or `None`
## `parent_y`
return location for Y coordinate
in parent’s coordinate system, or `None`
<!-- trait SurfaceExt::fn create_cairo_context -->
Creates a new `CairoContext` for rendering on `self`.

# Returns

the newly created `CairoContext`
<!-- trait SurfaceExt::fn create_gl_context -->
Creates a new `GLContext` matching the
framebuffer format to the visual of the `Surface`. The context
is disconnected from any particular surface or surface.

If the creation of the `GLContext` failed, `error` will be set.

Before using the returned `GLContext`, you will need to
call `GLContext::make_current` or `GLContext::realize`.

# Returns

the newly created `GLContext`, or
`None` on error
<!-- trait SurfaceExt::fn create_similar_surface -->
Create a new surface that is as compatible as possible with the
given `self`. For example the new surface will have the same
fallback resolution and font options as `self`. Generally, the new
surface will also use the same backend as `self`, unless that is
not possible for some reason. The type of the returned surface may
be examined with `cairo_surface_get_type`.

Initially the surface contents are all 0 (transparent if contents
have transparency, black otherwise.)
## `content`
the content for the new surface
## `width`
width of the new surface
## `height`
height of the new surface

# Returns

a pointer to the newly allocated surface. The caller
owns the surface and should call `cairo_surface_destroy` when done
with it.

This function always returns a valid pointer, but it will return a
pointer to a “nil” surface if `other` is already in an error state
or any other error occurs.
<!-- trait SurfaceExt::fn create_vulkan_context -->
Creates a new `VulkanContext` for rendering on `self`.

If the creation of the `VulkanContext` failed, `error` will be set.

# Returns

the newly created `VulkanContext`, or
`None` on error
<!-- trait SurfaceExt::fn deiconify -->
Attempt to deiconify (unminimize) `self`. On X11 the window manager may
choose to ignore the request to deiconify. When using GTK,
use `gtk_window_deiconify` instead of the `Surface` variant. Or better yet,
you probably want to use `gtk_window_present_with_time`, which raises the surface, focuses it,
unminimizes it, and puts it on the current desktop.
<!-- trait SurfaceExt::fn destroy -->
Destroys the window system resources associated with `self` and decrements `self`'s
reference count. The window system resources for all children of `self` are also
destroyed, but the children’s reference counts are not decremented.

Note that a surface will not be destroyed automatically when its reference count
reaches zero. You must call this function yourself before that happens.
<!-- trait SurfaceExt::fn focus -->
Sets keyboard focus to `self`. In most cases, `gtk_window_present_with_time`
should be used on a ``GtkWindow``, rather than calling this function.
## `timestamp`
timestamp of the event triggering the surface focus
<!-- trait SurfaceExt::fn freeze_updates -->
Temporarily freezes a surface such that it won’t receive expose
events. The surface will begin receiving expose events again when
`SurfaceExt::thaw_updates` is called. If `SurfaceExt::freeze_updates`
has been called more than once, `SurfaceExt::thaw_updates` must be called
an equal number of times to begin processing exposes.
<!-- trait SurfaceExt::fn fullscreen -->
Moves the surface into fullscreen mode. This means the
surface covers the entire screen and is above any panels
or task bars.

If the surface was already fullscreen, then this function does nothing.

On X11, asks the window manager to put `self` in a fullscreen
state, if the window manager supports this operation. Not all
window managers support this, and some deliberately ignore it or
don’t have a concept of “fullscreen”; so you can’t rely on the
fullscreenification actually happening. But it will happen with
most standard window managers, and GDK makes a best effort to get
it to happen.
<!-- trait SurfaceExt::fn fullscreen_on_monitor -->
Moves the surface into fullscreen mode on the given monitor. This means
the surface covers the entire screen and is above any panels or task bars.

If the surface was already fullscreen, then this function does nothing.
## `monitor`
Which monitor to display fullscreen on.
<!-- trait SurfaceExt::fn get_accept_focus -->
Determines whether or not the desktop environment shuld be hinted that
the surface does not want to receive input focus.

# Returns

whether or not the surface should receive input focus.
<!-- trait SurfaceExt::fn get_children -->
Gets the list of children of `self` known to GDK.
This function only returns children created via GDK,
so for example it’s useless when used with the root window;
it only returns surfaces an application created itself.

The returned list must be freed, but the elements in the
list need not be.

# Returns


 list of child surfaces inside `self`
<!-- trait SurfaceExt::fn get_cursor -->
Retrieves a `Cursor` pointer for the cursor currently set on the
specified `Surface`, or `None`. If the return value is `None` then
there is no custom cursor set on the specified surface, and it is
using the cursor for its parent surface.

# Returns

a `Cursor`, or `None`. The
 returned object is owned by the `Surface` and should not be
 unreferenced directly. Use `SurfaceExt::set_cursor` to unset the
 cursor of the surface
<!-- trait SurfaceExt::fn get_decorations -->
Returns the decorations set on the `Surface` with
`SurfaceExt::set_decorations`.
## `decorations`
The surface decorations will be written here

# Returns

`true` if the surface has decorations set, `false` otherwise.
<!-- trait SurfaceExt::fn get_device_cursor -->
Retrieves a `Cursor` pointer for the `device` currently set on the
specified `Surface`, or `None`. If the return value is `None` then
there is no custom cursor set on the specified surface, and it is
using the cursor for its parent surface.
## `device`
a master, pointer `Device`.

# Returns

a `Cursor`, or `None`. The
 returned object is owned by the `Surface` and should not be
 unreferenced directly. Use `SurfaceExt::set_cursor` to unset the
 cursor of the surface
<!-- trait SurfaceExt::fn get_device_position -->
Obtains the current device position in doubles and modifier state.
The position is given in coordinates relative to the upper left
corner of `self`.
## `device`
pointer `Device` to query to.
## `x`
return location for the X coordinate of `device`, or `None`.
## `y`
return location for the Y coordinate of `device`, or `None`.
## `mask`
return location for the modifier mask, or `None`.

# Returns

The surface underneath `device`
(as with `Device::get_surface_at_position`), or `None` if the
surface is not known to GDK.
<!-- trait SurfaceExt::fn get_display -->
Gets the `Display` associated with a `Surface`.

# Returns

the `Display` associated with `self`
<!-- trait SurfaceExt::fn get_focus_on_map -->
Determines whether or not the desktop environment should be hinted that the
surface does not want to receive input focus when it is mapped.

# Returns

whether or not the surface wants to receive input focus when
it is mapped.
<!-- trait SurfaceExt::fn get_frame_clock -->
Gets the frame clock for the surface. The frame clock for a surface
never changes unless the surface is reparented to a new toplevel
surface.

# Returns

the frame clock
<!-- trait SurfaceExt::fn get_frame_extents -->
Obtains the bounding box of the surface, including window manager
titlebar/borders if any. The frame position is given in root window
coordinates. To get the position of the surface itself (rather than
the frame) in root window coordinates, use `SurfaceExt::get_origin`.
## `rect`
rectangle to fill with bounding box of the surface frame
<!-- trait SurfaceExt::fn get_fullscreen_mode -->
Obtains the `FullscreenMode` of the `self`.

# Returns

The `FullscreenMode` applied to the surface when fullscreen.
<!-- trait SurfaceExt::fn get_geometry -->
Any of the return location arguments to this function may be `None`,
if you aren’t interested in getting the value of that field.

The X and Y coordinates returned are relative to the parent surface
of `self`, which for toplevels usually means relative to the
surface decorations (titlebar, etc.) rather than relative to the
root window (screen-size background window).

On the X11 platform, the geometry is obtained from the X server,
so reflects the latest position of `self`; this may be out-of-sync
with the position of `self` delivered in the most-recently-processed
`EventConfigure`. `SurfaceExt::get_position` in contrast gets the
position from the most recent configure event.

Note: If `self` is not a toplevel, it is much better
to call `SurfaceExt::get_position`, `SurfaceExt::get_width` and
`SurfaceExt::get_height` instead, because it avoids the roundtrip to
the X server and because these functions support the full 32-bit
coordinate space, whereas `SurfaceExt::get_geometry` is restricted to
the 16-bit coordinates of X11.
## `x`
return location for X coordinate of surface (relative to its parent)
## `y`
return location for Y coordinate of surface (relative to its parent)
## `width`
return location for width of surface
## `height`
return location for height of surface
<!-- trait SurfaceExt::fn get_height -->
Returns the height of the given `self`.

On the X11 platform the returned size is the size reported in the
most-recently-processed configure event, rather than the current
size on the X server.

# Returns

The height of `self`
<!-- trait SurfaceExt::fn get_modal_hint -->
Determines whether or not the window manager is hinted that `self`
has modal behaviour.

# Returns

whether or not the surface has the modal hint set.
<!-- trait SurfaceExt::fn get_origin -->
Obtains the position of a surface in root window coordinates.
(Compare with `SurfaceExt::get_position` and
`SurfaceExt::get_geometry` which return the position of a surface
relative to its parent surface.)
## `x`
return location for X coordinate
## `y`
return location for Y coordinate

# Returns

not meaningful, ignore
<!-- trait SurfaceExt::fn get_parent -->
Obtains the parent of `self`, as known to GDK. Does not query the
X server; thus this returns the parent as passed to `gdk_surface_new`,
not the actual parent. This should never matter unless you’re using
Xlib calls mixed with GDK calls on the X11 platform. It may also
matter for toplevel windows, because the window manager may choose
to reparent them.

# Returns

parent of `self`
<!-- trait SurfaceExt::fn get_pass_through -->
Returns whether input to the surface is passed through to the surface
below.

See `SurfaceExt::set_pass_through` for details
<!-- trait SurfaceExt::fn get_position -->
Obtains the position of the surface as reported in the
most-recently-processed `EventConfigure`. Contrast with
`SurfaceExt::get_geometry` which queries the X server for the
current surface position, regardless of which events have been
received or processed.

The position coordinates are relative to the surface’s parent surface.
## `x`
X coordinate of surface
## `y`
Y coordinate of surface
<!-- trait SurfaceExt::fn get_root_coords -->
Obtains the position of a surface position in root
window coordinates. This is similar to
`SurfaceExt::get_origin` but allows you to pass
in any position in the surface, not just the origin.
## `x`
X coordinate in surface
## `y`
Y coordinate in surface
## `root_x`
return location for X coordinate
## `root_y`
return location for Y coordinate
<!-- trait SurfaceExt::fn get_root_origin -->
Obtains the top-left corner of the window manager frame in root
surface coordinates.
## `x`
return location for X position of surface frame
## `y`
return location for Y position of surface frame
<!-- trait SurfaceExt::fn get_scale_factor -->
Returns the internal scale factor that maps from surface coordiantes
to the actual device pixels. On traditional systems this is 1, but
on very high density outputs this can be a higher value (often 2).

A higher value means that drawing is automatically scaled up to
a higher resolution, so any code doing drawing will automatically look
nicer. However, if you are supplying pixel-based data the scale
value can be used to determine whether to use a pixel resource
with higher resolution data.

The scale of a surface may change during runtime, if this happens
a configure event will be sent to the toplevel surface.

# Returns

the scale factor
<!-- trait SurfaceExt::fn get_state -->
Gets the bitwise OR of the currently active surface state flags,
from the `SurfaceState` enumeration.

# Returns

surface state bitfield
<!-- trait SurfaceExt::fn get_support_multidevice -->
Returns `true` if the surface is aware of the existence of multiple
devices.

# Returns

`true` if the surface handles multidevice features.
<!-- trait SurfaceExt::fn get_surface_type -->
Gets the type of the surface. See `SurfaceType`.

# Returns

type of surface
<!-- trait SurfaceExt::fn get_toplevel -->
Gets the toplevel surface that’s an ancestor of `self`.

Any surface type but `SurfaceType::Child` is considered a
toplevel surface, as is a `SurfaceType::Child` surface that
has a root surface as parent.

# Returns

the toplevel surface containing `self`
<!-- trait SurfaceExt::fn get_type_hint -->
This function returns the type hint set for a surface.

# Returns

The type hint set for `self`
<!-- trait SurfaceExt::fn get_width -->
Returns the width of the given `self`.

On the X11 platform the returned size is the size reported in the
most-recently-processed configure event, rather than the current
size on the X server.

# Returns

The width of `self`
<!-- trait SurfaceExt::fn has_native -->
Checks whether the surface has a native surface or not.

# Returns

`true` if the `self` has a native surface, `false` otherwise.
<!-- trait SurfaceExt::fn hide -->
For toplevel surfaces, withdraws them, so they will no longer be
known to the window manager; for all surfaces, unmaps them, so
they won’t be displayed. Normally done automatically as
part of `gtk_widget_hide`.
<!-- trait SurfaceExt::fn iconify -->
Asks to iconify (minimize) `self`. The window manager may choose
to ignore the request, but normally will honor it. Using
`gtk_window_iconify` is preferred, if you have a ``GtkWindow`` widget.

This function only makes sense when `self` is a toplevel surface.
<!-- trait SurfaceExt::fn input_shape_combine_region -->
Like `gdk_surface_shape_combine_region`, but the shape applies
only to event handling. Mouse events which happen while
the pointer position corresponds to an unset bit in the
mask will be passed on the surface below `self`.

An input shape is typically used with RGBA surfaces.
The alpha channel of the surface defines which pixels are
invisible and allows for nicely antialiased borders,
and the input shape controls where the surface is
“clickable”.

On the X11 platform, this requires version 1.1 of the
shape extension.

On the Win32 platform, this functionality is not present and the
function does nothing.
## `shape_region`
region of surface to be non-transparent
## `offset_x`
X position of `shape_region` in `self` coordinates
## `offset_y`
Y position of `shape_region` in `self` coordinates
<!-- trait SurfaceExt::fn is_destroyed -->
Check to see if a surface is destroyed..

# Returns

`true` if the surface is destroyed
<!-- trait SurfaceExt::fn is_input_only -->
Determines whether or not the surface is an input only surface.

# Returns

`true` if `self` is input only
<!-- trait SurfaceExt::fn is_viewable -->
Check if the surface and all ancestors of the surface are
mapped. (This is not necessarily "viewable" in the X sense, since
we only check as far as we have GDK surface parents, not to the root
surface.)

# Returns

`true` if the surface is viewable
<!-- trait SurfaceExt::fn is_visible -->
Checks whether the surface has been mapped (with `SurfaceExt::show` or
`SurfaceExt::show_unraised`).

# Returns

`true` if the surface is mapped
<!-- trait SurfaceExt::fn lower -->
Lowers `self` to the bottom of the Z-order (stacking order), so that
other surfaces with the same parent surface appear above `self`.
This is true whether or not the other surfaces are visible.

If `self` is a toplevel, the window manager may choose to deny the
request to move the surface in the Z-order, `SurfaceExt::lower` only
requests the restack, does not guarantee it.

Note that `SurfaceExt::show` raises the surface again, so don’t call this
function before `SurfaceExt::show`. (Try `SurfaceExt::show_unraised`.)
<!-- trait SurfaceExt::fn maximize -->
Maximizes the surface. If the surface was already maximized, then
this function does nothing.

On X11, asks the window manager to maximize `self`, if the window
manager supports this operation. Not all window managers support
this, and some deliberately ignore it or don’t have a concept of
“maximized”; so you can’t rely on the maximization actually
happening. But it will happen with most standard window managers,
and GDK makes a best effort to get it to happen.

On Windows, reliably maximizes the surface.
<!-- trait SurfaceExt::fn merge_child_input_shapes -->
Merges the input shape masks for any child surfaces into the
input shape mask for `self`. i.e. the union of all input masks
for `self` and its children will become the new input mask
for `self`. See `SurfaceExt::input_shape_combine_region`.

This function is distinct from `SurfaceExt::set_child_input_shapes`
because it includes `self`’s input shape mask in the set of
shapes to be merged.
<!-- trait SurfaceExt::fn move -->
Repositions a surface relative to its parent surface.
For toplevel surfaces, window managers may ignore or modify the move;
you should probably use `gtk_window_move` on a ``GtkWindow`` widget
anyway, instead of using GDK functions. For child surfaces,
the move will reliably succeed.

If you’re also planning to resize the surface, use `SurfaceExt::move_resize`
to both move and resize simultaneously, for a nicer visual effect.
## `x`
X coordinate relative to surface’s parent
## `y`
Y coordinate relative to surface’s parent
<!-- trait SurfaceExt::fn move_resize -->
Equivalent to calling `SurfaceExt::move` and `SurfaceExt::resize`,
except that both operations are performed at once, avoiding strange
visual effects. (i.e. the user may be able to see the surface first
move, then resize, if you don’t use `SurfaceExt::move_resize`.)
## `x`
new X position relative to surface’s parent
## `y`
new Y position relative to surface’s parent
## `width`
new width
## `height`
new height
<!-- trait SurfaceExt::fn move_to_rect -->
Moves `self` to `rect`, aligning their anchor points.

`rect` is relative to the top-left corner of the surface that `self` is
transient for. `rect_anchor` and `surface_anchor` determine anchor points on
`rect` and `self` to pin together. `rect`'s anchor point can optionally be
offset by `rect_anchor_dx` and `rect_anchor_dy`, which is equivalent to
offsetting the position of `self`.

`anchor_hints` determines how `self` will be moved if the anchor points cause
it to move off-screen. For example, `AnchorHints::FlipX` will replace
`Gravity::NorthWest` with `Gravity::NorthEast` and vice versa if
`self` extends beyond the left or right edges of the monitor.

Connect to the `Surface::moved-to-rect` signal to find out how it was
actually positioned.
## `rect`
the destination `Rectangle` to align `self` with
## `rect_anchor`
the point on `rect` to align with `self`'s anchor point
## `surface_anchor`
the point on `self` to align with `rect`'s anchor point
## `anchor_hints`
positioning hints to use when limited on space
## `rect_anchor_dx`
horizontal offset to shift `self`, i.e. `rect`'s anchor
 point
## `rect_anchor_dy`
vertical offset to shift `self`, i.e. `rect`'s anchor point
<!-- trait SurfaceExt::fn peek_children -->
Like `SurfaceExt::get_children`, but does not copy the list of
children, so the list does not need to be freed.

# Returns


 a reference to the list of child surfaces in `self`
<!-- trait SurfaceExt::fn queue_expose -->
Forces an expose event for `self` to be scheduled.

If the invalid area of `self` is empty, an expose event will
still be emitted. Its invalid region will be empty.

This function is useful for implementations that track invalid
regions on their own.
<!-- trait SurfaceExt::fn raise -->
Raises `self` to the top of the Z-order (stacking order), so that
other surfaces with the same parent surface appear below `self`.
This is true whether or not the surfaces are visible.

If `self` is a toplevel, the window manager may choose to deny the
request to move the surface in the Z-order, `SurfaceExt::raise` only
requests the restack, does not guarantee it.
<!-- trait SurfaceExt::fn register_dnd -->
Registers a surface as a potential drop destination.
<!-- trait SurfaceExt::fn resize -->
Resizes `self`; for toplevel surfaces, asks the window manager to resize
the surface. The window manager may not allow the resize. When using GTK,
use `gtk_window_resize` instead of this low-level GDK function.

Surfaces may not be resized below 1x1.

If you’re also planning to move the surface, use `SurfaceExt::move_resize`
to both move and resize simultaneously, for a nicer visual effect.
## `width`
new width of the surface
## `height`
new height of the surface
<!-- trait SurfaceExt::fn restack -->
Changes the position of `self` in the Z-order (stacking order), so that
it is above `sibling` (if `above` is `true`) or below `sibling` (if `above` is
`false`).

If `sibling` is `None`, then this either raises (if `above` is `true`) or
lowers the surface.

If `self` is a toplevel, the window manager may choose to deny the
request to move the surface in the Z-order, `SurfaceExt::restack` only
requests the restack, does not guarantee it.
## `sibling`
a `Surface` that is a sibling of `self`, or `None`
## `above`
a boolean
<!-- trait SurfaceExt::fn set_accept_focus -->
Setting `accept_focus` to `false` hints the desktop environment that the
surface doesn’t want to receive input focus.

On X, it is the responsibility of the window manager to interpret this
hint. ICCCM-compliant window manager usually respect it.
## `accept_focus`
`true` if the surface should receive input focus
<!-- trait SurfaceExt::fn set_child_input_shapes -->
Sets the input shape mask of `self` to the union of input shape masks
for all children of `self`, ignoring the input shape mask of `self`
itself. Contrast with `SurfaceExt::merge_child_input_shapes` which includes
the input shape mask of `self` in the masks to be merged.
<!-- trait SurfaceExt::fn set_cursor -->
Sets the default mouse pointer for a `Surface`.

Note that `cursor` must be for the same display as `self`.

Use `Cursor::new_from_name` or `Cursor::new_from_texture` to
create the cursor. To make the cursor invisible, use `GDK_BLANK_CURSOR`.
Passing `None` for the `cursor` argument to `SurfaceExt::set_cursor` means
that `self` will use the cursor of its parent surface. Most surfaces
should use this default.
## `cursor`
a cursor
<!-- trait SurfaceExt::fn set_decorations -->
“Decorations” are the features the window manager adds to a toplevel `Surface`.
This function sets the traditional Motif window manager hints that tell the
window manager which decorations you would like your surface to have.
Usually you should use `gtk_window_set_decorated` on a ``GtkWindow`` instead of
using the GDK function directly.

The `decorations` argument is the logical OR of the fields in
the `WMDecoration` enumeration. If `WMDecoration::All` is included in the
mask, the other bits indicate which decorations should be turned off.
If `WMDecoration::All` is not included, then the other bits indicate
which decorations should be turned on.

Most window managers honor a decorations hint of 0 to disable all decorations,
but very few honor all possible combinations of bits.
## `decorations`
decoration hint mask
<!-- trait SurfaceExt::fn set_device_cursor -->
Sets a specific `Cursor` for a given device when it gets inside `self`.
Use `gdk_cursor_new_fromm_name` or `Cursor::new_from_texture` to create
the cursor. To make the cursor invisible, use `GDK_BLANK_CURSOR`. Passing
`None` for the `cursor` argument to `SurfaceExt::set_cursor` means that
`self` will use the cursor of its parent surface. Most surfaces should
use this default.
## `device`
a master, pointer `Device`
## `cursor`
a `Cursor`
<!-- trait SurfaceExt::fn set_focus_on_map -->
Setting `focus_on_map` to `false` hints the desktop environment that the
surface doesn’t want to receive input focus when it is mapped.
focus_on_map should be turned off for surfaces that aren’t triggered
interactively (such as popups from network activity).

On X, it is the responsibility of the window manager to interpret
this hint. Window managers following the freedesktop.org window
manager extension specification should respect it.
## `focus_on_map`
`true` if the surface should receive input focus when mapped
<!-- trait SurfaceExt::fn set_fullscreen_mode -->
Specifies whether the `self` should span over all monitors (in a multi-head
setup) or only the current monitor when in fullscreen mode.

The `mode` argument is from the `FullscreenMode` enumeration.
If `FullscreenMode::AllMonitors` is specified, the fullscreen `self` will
span over all monitors of the display.

On X11, searches through the list of monitors display the ones
which delimit the 4 edges of the entire display and will ask the window
manager to span the `self` over these monitors.

If the XINERAMA extension is not available or not usable, this function
has no effect.

Not all window managers support this, so you can’t rely on the fullscreen
surface to span over the multiple monitors when `FullscreenMode::AllMonitors`
is specified.
## `mode`
fullscreen mode
<!-- trait SurfaceExt::fn set_functions -->
Sets hints about the window management functions to make available
via buttons on the window frame.

On the X backend, this function sets the traditional Motif window
manager hint for this purpose. However, few window managers do
anything reliable or interesting with this hint. Many ignore it
entirely.

The `functions` argument is the logical OR of values from the
`WMFunction` enumeration. If the bitmask includes `WMFunction::All`,
then the other bits indicate which functions to disable; if
it doesn’t include `WMFunction::All`, it indicates which functions to
enable.
## `functions`
bitmask of operations to allow on `self`
<!-- trait SurfaceExt::fn set_geometry_hints -->
Sets the geometry hints for `self`. Hints flagged in `geom_mask`
are set, hints not flagged in `geom_mask` are unset.
To unset all hints, use a `geom_mask` of 0 and a `geometry` of `None`.

This function provides hints to the surfaceing system about
acceptable sizes for a toplevel surface. The purpose of
this is to constrain user resizing, but the windowing system
will typically (but is not required to) also constrain the
current size of the surface to the provided values and
constrain programatic resizing via `SurfaceExt::resize` or
`SurfaceExt::move_resize`.

Note that on X11, this effect has no effect on surfaces
of type `SurfaceType::Temp` since these surfaces are not resizable
by the user.

Since you can’t count on the windowing system doing the
constraints for programmatic resizes, you should generally
call `Surface::constrain_size` yourself to determine
appropriate sizes.
## `geometry`
geometry hints
## `geom_mask`
bitmask indicating fields of `geometry` to pay attention to
<!-- trait SurfaceExt::fn set_icon_list -->
Sets a list of icons for the surface. One of these will be used
to represent the surface when it has been iconified. The icon is
usually shown in an icon box or some sort of task bar. Which icon
size is shown depends on the window manager. The window manager
can scale the icon but setting several size icons can give better
image quality since the window manager may only need to scale the
icon by a small amount or not at all.

Note that some platforms don't support surface icons.
## `surfaces`

 A list of image surfaces, of different sizes.
<!-- trait SurfaceExt::fn set_icon_name -->
Surfaces may have a name used while minimized, distinct from the
name they display in their titlebar. Most of the time this is a bad
idea from a user interface standpoint. But you can set such a name
with this function, if you like.

After calling this with a non-`None` `name`, calls to `SurfaceExt::set_title`
will not update the icon title.

Using `None` for `name` unsets the icon title; further calls to
`SurfaceExt::set_title` will again update the icon title as well.

Note that some platforms don't support surface icons.
## `name`
name of surface while iconified (minimized)
<!-- trait SurfaceExt::fn set_keep_above -->
Set if `self` must be kept above other surfaces. If the
surface was already above, then this function does nothing.

On X11, asks the window manager to keep `self` above, if the window
manager supports this operation. Not all window managers support
this, and some deliberately ignore it or don’t have a concept of
“keep above”; so you can’t rely on the surface being kept above.
But it will happen with most standard window managers,
and GDK makes a best effort to get it to happen.
## `setting`
whether to keep `self` above other surfaces
<!-- trait SurfaceExt::fn set_keep_below -->
Set if `self` must be kept below other surfaces. If the
surface was already below, then this function does nothing.

On X11, asks the window manager to keep `self` below, if the window
manager supports this operation. Not all window managers support
this, and some deliberately ignore it or don’t have a concept of
“keep below”; so you can’t rely on the surface being kept below.
But it will happen with most standard window managers,
and GDK makes a best effort to get it to happen.
## `setting`
whether to keep `self` below other surfaces
<!-- trait SurfaceExt::fn set_modal_hint -->
The application can use this hint to tell the window manager
that a certain surface has modal behaviour. The window manager
can use this information to handle modal surfaces in a special
way.

You should only use this on surfaces for which you have
previously called `SurfaceExt::set_transient_for`
## `modal`
`true` if the surface is modal, `false` otherwise.
<!-- trait SurfaceExt::fn set_opacity -->
Set `self` to render as partially transparent,
with opacity 0 being fully transparent and 1 fully opaque. (Values
of the opacity parameter are clamped to the [0,1] range.)

For toplevel surfaces this depends on support from the windowing system
that may not always be there. For instance, On X11, this works only on
X screens with a compositing manager running. On Wayland, there is no
per-surface opacity value that the compositor would apply. Instead, use
`gdk_surface_set_opaque_region (surface, NULL)` to tell the compositor
that the entire surface is (potentially) non-opaque, and draw your content
with alpha, or use `gtk_widget_set_opacity` to set an overall opacity
for your widgets.

Support for non-toplevel surfaces was added in 3.8.
## `opacity`
opacity
<!-- trait SurfaceExt::fn set_opaque_region -->
For optimisation purposes, compositing window managers may
like to not draw obscured regions of surfaces, or turn off blending
during for these regions. With RGB windows with no transparency,
this is just the shape of the window, but with ARGB32 windows, the
compositor does not know what regions of the window are transparent
or not.

This function only works for toplevel surfaces.

GTK will update this property automatically if
the `self` background is opaque, as we know where the opaque regions
are. If your surface background is not opaque, please update this
property in your ``GtkWidget`::style-updated` handler.
## `region`
a region, or `None`
<!-- trait SurfaceExt::fn set_pass_through -->
Sets whether input to the surface is passed through to the surface
below.

The default value of this is `false`, which means that pointer
events that happen inside the surface are send first to the surface,
but if the event is not selected by the event mask then the event
is sent to the parent surface, and so on up the hierarchy.

If `pass_through` is `true` then such pointer events happen as if the
surface wasn't there at all, and thus will be sent first to any
surfaces below `self`. This is useful if the surface is used in a
transparent fashion. In the terminology of the web this would be called
"pointer-events: none".

Note that a surface with `pass_through` `true` can still have a subsurface
without pass through, so you can get events on a subset of a surface. And in
that cases you would get the in-between related events such as the pointer
enter/leave events on its way to the destination surface.
## `pass_through`
a boolean
<!-- trait SurfaceExt::fn set_shadow_width -->
Newer GTK windows using client-side decorations use extra geometry
around their frames for effects like shadows and invisible borders.
Window managers that want to maximize windows or snap to edges need
to know where the extents of the actual frame lie, so that users
don’t feel like windows are snapping against random invisible edges.

Note that this property is automatically updated by GTK, so this
function should only be used by applications which do not use GTK
to create toplevel surfaces.
## `left`
The left extent
## `right`
The right extent
## `top`
The top extent
## `bottom`
The bottom extent
<!-- trait SurfaceExt::fn set_startup_id -->
When using GTK, typically you should use `gtk_window_set_startup_id`
instead of this low-level function.
## `startup_id`
a string with startup-notification identifier
<!-- trait SurfaceExt::fn set_support_multidevice -->
This function will enable multidevice features in `self`.

Multidevice aware surfaces will need to handle properly multiple,
per device enter/leave events, device grabs and grab ownerships.
## `support_multidevice`
`true` to enable multidevice support in `self`.
<!-- trait SurfaceExt::fn set_title -->
Sets the title of a toplevel surface, to be displayed in the titlebar.
If you haven’t explicitly set the icon name for the surface
(using `SurfaceExt::set_icon_name`), the icon name will be set to
`title` as well. `title` must be in UTF-8 encoding (as with all
user-readable strings in GDK and GTK). `title` may not be `None`.
## `title`
title of `self`
<!-- trait SurfaceExt::fn set_transient_for -->
Indicates to the window manager that `self` is a transient dialog
associated with the application surface `parent`. This allows the
window manager to do things like center `self` on `parent` and
keep `self` above `parent`.

See `gtk_window_set_transient_for` if you’re using ``GtkWindow`` or
``GtkDialog``.
## `parent`
another toplevel `Surface`
<!-- trait SurfaceExt::fn set_type_hint -->
The application can use this call to provide a hint to the surface
manager about the functionality of a surface. The window manager
can use this information when determining the decoration and behaviour
of the surface.

The hint must be set before the surface is mapped.
## `hint`
A hint of the function this surface will have
<!-- trait SurfaceExt::fn show -->
Like `SurfaceExt::show_unraised`, but also raises the surface to the
top of the surface stack (moves the surface to the front of the
Z-order).

This function maps a surface so it’s visible onscreen. Its opposite
is `SurfaceExt::hide`.

When implementing a ``GtkWidget``, you should call this function on the widget's
`Surface` as part of the “map” method.
<!-- trait SurfaceExt::fn show_unraised -->
Shows a `Surface` onscreen, but does not modify its stacking
order. In contrast, `SurfaceExt::show` will raise the surface
to the top of the surface stack.

On the X11 platform, in Xlib terms, this function calls
XMapWindow() (it also updates some internal GDK state, which means
that you can’t really use XMapWindow() directly on a GDK surface).
<!-- trait SurfaceExt::fn show_window_menu -->
Asks the windowing system to show the window menu. The window menu
is the menu shown when right-clicking the titlebar on traditional
windows managed by the window manager. This is useful for windows
using client-side decorations, activating it with a right-click
on the window decorations.
## `event`
a `Event` to show the menu for

# Returns

`true` if the window menu was shown and `false` otherwise.
<!-- trait SurfaceExt::fn stick -->
“Pins” a surface such that it’s on all workspaces and does not scroll
with viewports, for window managers that have scrollable viewports.
(When using ``GtkWindow``, `gtk_window_stick` may be more useful.)

On the X11 platform, this function depends on window manager
support, so may have no effect with many window managers. However,
GDK will do the best it can to convince the window manager to stick
the surface. For window managers that don’t support this operation,
there’s nothing you can do to force it to happen.
<!-- trait SurfaceExt::fn thaw_updates -->
Thaws a surface frozen with `SurfaceExt::freeze_updates`.
<!-- trait SurfaceExt::fn unfullscreen -->
Moves the surface out of fullscreen mode. If the surface was not
fullscreen, does nothing.

On X11, asks the window manager to move `self` out of the fullscreen
state, if the window manager supports this operation. Not all
window managers support this, and some deliberately ignore it or
don’t have a concept of “fullscreen”; so you can’t rely on the
unfullscreenification actually happening. But it will happen with
most standard window managers, and GDK makes a best effort to get
it to happen.
<!-- trait SurfaceExt::fn unmaximize -->
Unmaximizes the surface. If the surface wasn’t maximized, then this
function does nothing.

On X11, asks the window manager to unmaximize `self`, if the
window manager supports this operation. Not all window managers
support this, and some deliberately ignore it or don’t have a
concept of “maximized”; so you can’t rely on the unmaximization
actually happening. But it will happen with most standard window
managers, and GDK makes a best effort to get it to happen.

On Windows, reliably unmaximizes the surface.
<!-- trait SurfaceExt::fn unstick -->
Reverse operation for `SurfaceExt::stick`; see `SurfaceExt::stick`,
and `gtk_window_unstick`.
<!-- trait SurfaceExt::fn connect_event -->
Emitted when GDK receives an input event for `surface`.
## `event`
an input event

# Returns

`true` to indicate that the event has been handled
<!-- trait SurfaceExt::fn connect_moved_to_rect -->
Emitted when the position of `surface` is finalized after being moved to a
destination rectangle.

`surface` might be flipped over the destination rectangle in order to keep
it on-screen, in which case `flipped_x` and `flipped_y` will be set to `true`
accordingly.

`flipped_rect` is the ideal position of `surface` after any possible
flipping, but before any possible sliding. `final_rect` is `flipped_rect`,
but possibly translated in the case that flipping is still ineffective in
keeping `surface` on-screen.
## `flipped_rect`
the position of `surface` after any possible
 flipping or `None` if the backend can't obtain it
## `final_rect`
the final position of `surface` or `None` if the
 backend can't obtain it
## `flipped_x`
`true` if the anchors were flipped horizontally
## `flipped_y`
`true` if the anchors were flipped vertically
<!-- trait SurfaceExt::fn connect_render -->
Emitted when part of the surface needs to be redrawn.
## `region`
the region that needs to be redrawn

# Returns

`true` to indicate that the signal has been handled
<!-- trait SurfaceExt::fn connect_size_changed -->
Emitted when the size of `surface` is changed.
## `width`
the new width
## `height`
the new height
<!-- trait SurfaceExt::fn get_property_cursor -->
The mouse pointer for a `Surface`. See `SurfaceExt::set_cursor` and
`SurfaceExt::get_cursor` for details.
<!-- trait SurfaceExt::fn set_property_cursor -->
The mouse pointer for a `Surface`. See `SurfaceExt::set_cursor` and
`SurfaceExt::get_cursor` for details.
<!-- trait SurfaceExt::fn get_property_display -->
The `Display` connection of the surface. See `SurfaceExt::get_display`
for details.
<!-- trait SurfaceExt::fn set_property_display -->
The `Display` connection of the surface. See `SurfaceExt::get_display`
for details.
<!-- enum SurfaceEdge -->
Determines a surface edge or corner.
<!-- enum SurfaceEdge::variant NorthWest -->
the top left corner.
<!-- enum SurfaceEdge::variant North -->
the top edge.
<!-- enum SurfaceEdge::variant NorthEast -->
the top right corner.
<!-- enum SurfaceEdge::variant West -->
the left edge.
<!-- enum SurfaceEdge::variant East -->
the right edge.
<!-- enum SurfaceEdge::variant SouthWest -->
the lower left corner.
<!-- enum SurfaceEdge::variant South -->
the lower edge.
<!-- enum SurfaceEdge::variant SouthEast -->
the lower right corner.
<!-- enum SurfaceType -->
Describes the kind of surface.
<!-- enum SurfaceType::variant Toplevel -->
toplevel window (used to implement ``GtkWindow``)
<!-- enum SurfaceType::variant Child -->
child surface (used to implement e.g. ``GtkEntry``)
<!-- enum SurfaceType::variant Temp -->
override redirect temporary surface (used to implement ``GtkMenu``)
<!-- enum SurfaceTypeHint -->
These are hints for the window manager that indicate what type of function
the window has. The window manager can use this when determining decoration
and behaviour of the window. The hint must be set before mapping the window.

See the [Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec)
specification for more details about window types.
<!-- enum SurfaceTypeHint::variant Normal -->
Normal toplevel window.
<!-- enum SurfaceTypeHint::variant Dialog -->
Dialog window.
<!-- enum SurfaceTypeHint::variant Menu -->
Window used to implement a menu; GTK uses
 this hint only for torn-off menus, see ``GtkTearoffMenuItem``.
<!-- enum SurfaceTypeHint::variant Toolbar -->
Window used to implement toolbars.
<!-- enum SurfaceTypeHint::variant Splashscreen -->
Window used to display a splash
 screen during application startup.
<!-- enum SurfaceTypeHint::variant Utility -->
Utility windows which are not detached
 toolbars or dialogs.
<!-- enum SurfaceTypeHint::variant Dock -->
Used for creating dock or panel windows.
<!-- enum SurfaceTypeHint::variant Desktop -->
Used for creating the desktop background
 window.
<!-- enum SurfaceTypeHint::variant DropdownMenu -->
A menu that belongs to a menubar.
<!-- enum SurfaceTypeHint::variant PopupMenu -->
A menu that does not belong to a menubar,
 e.g. a context menu.
<!-- enum SurfaceTypeHint::variant Tooltip -->
A tooltip.
<!-- enum SurfaceTypeHint::variant Notification -->
A notification - typically a “bubble”
 that belongs to a status icon.
<!-- enum SurfaceTypeHint::variant Combo -->
A popup from a combo box.
<!-- enum SurfaceTypeHint::variant Dnd -->
A window that is used to implement a DND cursor.
<!-- struct Texture -->
A `Texture` represents image data that can be displayed on screen.

There are various ways to create `Texture` objects from a `gdk_pixbuf::Pixbuf`
or a cairo surface, or other pixel data.

An important aspect of `GdkTextures` is that they are immutable - once
the image data has been wrapped in a `Texture`, it may be uploaded
to the GPU or used in other ways that make it impractical to allow
modification.

# Implements

[`TextureExt`](trait.TextureExt.html), [`PaintableExt`](trait.PaintableExt.html)
<!-- trait TextureExt -->
Trait containing all `Texture` methods.

# Implementors

[`GLTexture`](struct.GLTexture.html), [`MemoryTexture`](struct.MemoryTexture.html), [`Texture`](struct.Texture.html)
<!-- impl Texture::fn new_for_pixbuf -->
Creates a new texture object representing the `gdk_pixbuf::Pixbuf`.
## `pixbuf`
a `gdk_pixbuf::Pixbuf`

# Returns

a new `Texture`
<!-- impl Texture::fn new_from_file -->
Creates a new texture by loading an image from a file. The file format is
detected automatically. If `None` is returned, then `error` will be set.
## `file`
`gio::File` to load

# Returns

A newly-created `Texture` or `None` if an error occured.
<!-- impl Texture::fn new_from_resource -->
Creates a new texture by loading an image from a resource.
The file format is detected automatically.

It is a fatal error if `resource_path` does not specify a valid
image resource and the program will abort if that happens.
If you are unsure about the validity of a resource, use
`Texture::new_from_file` to load it.
## `resource_path`
the path of the resource file

# Returns

A newly-created texture
<!-- trait TextureExt::fn download -->
Downloads the `self` into local memory. This may be
an expensive operation, as the actual texture data may
reside on a GPU or on a remote display server.

The data format of the downloaded data is equivalent to
`cairo::Format::Argb32`, so every downloaded pixel requires
4 bytes of memory.

Downloading a texture into a Cairo image surface:

```C
surface = cairo_image_surface_create (CAIRO_FORMAT_ARGB32,
                                      gdk_texture_get_width (texture),
                                      gdk_texture_get_height (texture));
gdk_texture_download (texture,
                      cairo_image_surface_get_data (surface),
                      cairo_image_surface_get_stride (surface));
cairo_surface_mark_dirty (surface);
```
## `data`
pointer to enough memory to be filled with the
 downloaded data of `self`
## `stride`
rowstride in bytes
<!-- trait TextureExt::fn get_height -->
Returns the height of the `self`.

# Returns

the height of the `Texture`
<!-- trait TextureExt::fn get_width -->
Returns the width of `self`.

# Returns

the width of the `Texture`
<!-- trait TextureExt::fn save_to_png -->
Store the given `self` to the `filename` as a PNG file.

This is a utility function intended for debugging and testing.
If you want more control over formats, proper error handling or
want to store to a `gio::File` or other location, you might want to
look into using the gdk-pixbuf library.
## `filename`
the filename to store to

# Returns

`true` if saving succeeded, `false` on failure.
<!-- trait TextureExt::fn get_property_height -->
The height of the texture.
<!-- trait TextureExt::fn set_property_height -->
The height of the texture.
<!-- trait TextureExt::fn get_property_width -->
The width of the texture.
<!-- trait TextureExt::fn set_property_width -->
The width of the texture.
<!-- struct TimeCoord -->
A `TimeCoord` stores a single event in a motion history.
<!-- enum TouchpadGesturePhase -->
Specifies the current state of a touchpad gesture. All gestures are
guaranteed to begin with an event with phase `TouchpadGesturePhase::Begin`,
followed by 0 or several events with phase `TouchpadGesturePhase::Update`.

A finished gesture may have 2 possible outcomes, an event with phase
`TouchpadGesturePhase::End` will be emitted when the gesture is
considered successful, this should be used as the hint to perform any
permanent changes.

Cancelled gestures may be so for a variety of reasons, due to hardware
or the compositor, or due to the gesture recognition layers hinting the
gesture did not finish resolutely (eg. a 3rd finger being added during
a pinch gesture). In these cases, the last event will report the phase
`TouchpadGesturePhase::Cancel`, this should be used as a hint
to undo any visible/permanent changes that were done throughout the
progress of the gesture.
<!-- enum TouchpadGesturePhase::variant Begin -->
The gesture has begun.
<!-- enum TouchpadGesturePhase::variant Update -->
The gesture has been updated.
<!-- enum TouchpadGesturePhase::variant End -->
The gesture was finished, changes
 should be permanently applied.
<!-- enum TouchpadGesturePhase::variant Cancel -->
The gesture was cancelled, all
 changes should be undone.
<!-- struct VulkanContext -->
`VulkanContext` is an object representing the platform-specific
Vulkan draw context.

``GdkVulkanContexts`` are created for a `Surface` using
`SurfaceExt::create_vulkan_context`, and the context will match the
the characteristics of the surface.

Support for `VulkanContext` is platform-specific, context creation
can fail, returning `None` context.

# Implements

[`DrawContextExt`](trait.DrawContextExt.html), [`DrawContextExtManual`](prelude/trait.DrawContextExtManual.html)
<!-- impl VulkanContext::fn connect_images_updated -->
This signal is emitted when the images managed by this context have
changed. Usually this means that the swapchain had to be recreated,
for example in response to a change of the surface size.
<!-- enum VulkanError -->
Error enumeration for `VulkanContext`.
<!-- enum VulkanError::variant Unsupported -->
Vulkan is not supported on this backend or has not been
 compiled in.
<!-- enum VulkanError::variant NotAvailable -->
Vulkan support is not available on this Surface
