<!-- file * -->
<!-- struct X11AppLaunchContext -->


# Implements

[`gdk::AppLaunchContextExt`](../gdk/trait.AppLaunchContextExt.html), [`gio::AppLaunchContextExt`](../gio/trait.AppLaunchContextExt.html)
<!-- struct X11DeviceManagerXI2 -->

<!-- enum X11DeviceType -->
<!-- struct X11DeviceXI2 -->


# Implements

[`gdk::DeviceExt`](../gdk/trait.DeviceExt.html)
<!-- struct X11Display -->


# Implements

[`gdk::DisplayExt`](../gdk/trait.DisplayExt.html)
<!-- impl X11Display::fn open -->
Tries to open a new display to the X server given by
`display_name`. If opening the display fails, `None` is
returned.
## `display_name`
name of the X display.
 See the XOpenDisplay() for details.

# Returns

The new display or
 `None` on error.
<!-- impl X11Display::fn set_program_class -->
Sets the program class.

The X11 backend uses the program class to set the class name part
of the `WM_CLASS` property on toplevel windows; see the ICCCM.
## `display`
a `gdk::Display`
## `program_class`
a string
<!-- impl X11Display::fn broadcast_startup_message -->
Sends a startup notification message of type `message_type` to
`self`.

This is a convenience function for use by code that implements the
freedesktop startup notification specification. Applications should
not normally need to call it directly. See the
[Startup Notification Protocol specification](http://standards.freedesktop.org/startup-notification-spec/startup-notification-latest.txt)
for definitions of the message types and keys that can be used.
## `message_type`
startup notification message type ("new", "change",
or "remove")
<!-- impl X11Display::fn error_trap_pop -->
Pops the error trap pushed by `X11Display::error_trap_push`.
Will XSync() if necessary and will always block until
the error is known to have occurred or not occurred,
so the error code can be returned.

If you don’t need to use the return value,
`X11Display::error_trap_pop_ignored` would be more efficient.

# Returns

X error code or 0 on success
<!-- impl X11Display::fn error_trap_pop_ignored -->
Pops the error trap pushed by `X11Display::error_trap_push`.
Does not block to see if an error occurred; merely records the
range of requests to ignore errors for, and ignores those errors
if they arrive asynchronously.
<!-- impl X11Display::fn error_trap_push -->
Begins a range of X requests on `self` for which X error events
will be ignored. Unignored errors (when no trap is pushed) will abort
the application. Use `X11Display::error_trap_pop` or
`X11Display::error_trap_pop_ignored`to lift a trap pushed
with this function.
<!-- impl X11Display::fn get_default_group -->
Returns the default group leader surface for all toplevel surfaces
on `self`. This surface is implicitly created by GDK.
See `X11Surface::set_group`.

# Returns

The default group leader surface
for `self`
<!-- impl X11Display::fn get_glx_version -->
Retrieves the version of the GLX implementation.
## `major`
return location for the GLX major version
## `minor`
return location for the GLX minor version

# Returns

`true` if GLX is available
<!-- impl X11Display::fn get_primary_monitor -->
Gets the primary monitor for the display.

The primary monitor is considered the monitor where the “main desktop”
lives. While normal application surfaces typically allow the window
manager to place the surfaces, specialized desktop applications
such as panels should place themselves on the primary monitor.

If no monitor is the designated primary monitor, any monitor
(usually the first) may be returned.

# Returns

the primary monitor, or any monitor if no
 primary monitor is configured by the user
<!-- impl X11Display::fn get_screen -->
Retrieves the `X11Screen` of the `self`.

# Returns

the `X11Screen`
<!-- impl X11Display::fn get_startup_notification_id -->
Gets the startup notification ID for a display.

# Returns

the startup notification ID for `self`
<!-- impl X11Display::fn get_user_time -->
Returns the timestamp of the last user interaction on
`self`. The timestamp is taken from events caused
by user interaction such as key presses or pointer
movements. See `X11Surface::set_user_time`.

# Returns

the timestamp of the last user interaction
<!-- impl X11Display::fn get_xcursor -->
Returns the X cursor belonging to a `gdk::Cursor`, potentially
creating the cursor.

Be aware that the returned cursor may not be unique to `cursor`.
It may for example be shared with its fallback cursor. On old
X servers that don't support the XCursor extension, all cursors
may even fall back to a few default cursors.
## `cursor`
a `gdk::Cursor`.

# Returns

an Xlib Cursor.
<!-- impl X11Display::fn get_xdisplay -->
Returns the X display of a `gdk::Display`.

# Returns

an X display
<!-- impl X11Display::fn get_xrootwindow -->
Returns the root X window used by `gdk::Display`.

# Returns

an X Window
<!-- impl X11Display::fn get_xscreen -->
Returns the X Screen used by `gdk::Display`.

# Returns

an X Screen
<!-- impl X11Display::fn grab -->
Call XGrabServer() on `self`.
To ungrab the display again, use `X11Display::ungrab`.

`X11Display::grab`/`X11Display::ungrab` calls can be nested.
<!-- impl X11Display::fn set_cursor_theme -->
Sets the cursor theme from which the images for cursor
should be taken.

If the windowing system supports it, existing cursors created
with `gdk::Cursor::new_from_name` are updated to reflect the theme
change. Custom cursors constructed with `gdk::Cursor::new_from_texture`
will have to be handled by the application (GTK applications can learn
about cursor theme changes by listening for change notification
for the corresponding ``GtkSetting``).
## `theme`
the name of the cursor theme to use, or `None` to unset
 a previously set value
## `size`
the cursor size to use, or 0 to keep the previous size
<!-- impl X11Display::fn set_startup_notification_id -->
Sets the startup notification ID for a display.

This is usually taken from the value of the DESKTOP_STARTUP_ID
environment variable, but in some cases (such as the application not
being launched using `exec`) it can come from other sources.

If the ID contains the string "_TIME" then the portion following that
string is taken to be the X11 timestamp of the event that triggered
the application to be launched and the GDK current event time is set
accordingly.

The startup ID is also what is used to signal that the startup is
complete (for example, when opening a window or when calling
`gdk::DisplayExt::notify_startup_complete`).
## `startup_id`
the startup notification ID (must be valid utf8)
<!-- impl X11Display::fn set_surface_scale -->
Forces a specific window scale for all windows on this display,
instead of using the default or user configured scale. This
is can be used to disable scaling support by setting `scale` to
1, or to programmatically set the window scale.

Once the scale is set by this call it will not change in response
to later user configuration changes.
## `scale`
The new scale value
<!-- impl X11Display::fn string_to_compound_text -->
Convert a string from the encoding of the current
locale into a form suitable for storing in a window property.
## `str`
a nul-terminated string
## `encoding`
location to store the encoding
 (to be used as the type for the property)
## `format`
location to store the format of the property
## `ctext`
location to store newly
 allocated data for the property
## `length`
the length of `ctext`, in bytes

# Returns

0 upon success, non-zero upon failure
<!-- impl X11Display::fn text_property_to_text_list -->
Convert a text string from the encoding as it is stored
in a property into an array of strings in the encoding of
the current locale. (The elements of the array represent the
nul-separated elements of the original text string.)
## `encoding`
a string representing the encoding. The most
 common values for this are "STRING", or "COMPOUND_TEXT".
 This is value used as the type for the property
## `format`
the format of the property
## `text`
The text data
## `length`
The number of items to transform
## `list`
location to store an array of strings in
 the encoding of the current locale. This array should be
 freed using `gdk_x11_free_text_list`.

# Returns

the number of strings stored in list, or 0,
 if the conversion failed
<!-- impl X11Display::fn ungrab -->
Ungrab `self` after it has been grabbed with
`X11Display::grab`.
<!-- impl X11Display::fn utf8_to_compound_text -->
Converts from UTF-8 to compound text.
## `str`
a UTF-8 string
## `encoding`
location to store resulting encoding
## `format`
location to store format of the result
## `ctext`
location to store the data of the result
## `length`
location to store the length of the data
 stored in `ctext`

# Returns

`true` if the conversion succeeded,
 otherwise `false`
<!-- impl X11Display::fn connect_xevent -->
The ::xevent signal is a low level signal that is emitted
whenever an XEvent has been received.

When handlers to this signal return `true`, no other handlers will be
invoked. In particular, the default handler for this function is
GDK's own event handling mechanism, so by returning `true` for an event
that GDK expects to translate, you may break GDK and/or GTK+ in
interesting ways. You have been warned.

If you want this signal handler to queue a `gdk::Event`, you can use
`gdk::DisplayExt::put_event`.

If you are interested in X GenericEvents, bear in mind that
XGetEventData() has been already called on the event, and
XFreeEventData() will be called afterwards.
## `xevent`
a pointer to the XEvent to process

# Returns

`true` to stop other handlers from being invoked for the event.
 `false` to propagate the event further.
<!-- struct X11Drag -->


# Implements

[`gdk::DragExt`](../gdk/trait.DragExt.html)
<!-- struct X11GLContext -->


# Implements

[`gdk::GLContextExt`](../gdk/trait.GLContextExt.html), [`gdk::DrawContextExt`](../gdk/trait.DrawContextExt.html)
<!-- struct X11Monitor -->


# Implements

[`gdk::MonitorExt`](../gdk/trait.MonitorExt.html)
<!-- impl X11Monitor::fn get_output -->
Returns the XID of the Output corresponding to `self`.

# Returns

the XID of `self`
<!-- impl X11Monitor::fn get_workarea -->
Retrieves the size and position of the “work area” on a monitor
within the display coordinate space. The returned geometry is in
”application pixels”, not in ”device pixels” (see
`gdk::MonitorExt::get_scale_factor`).
## `workarea`
a `gdk::Rectangle` to be filled with
 the monitor workarea
<!-- struct X11Screen -->

<!-- impl X11Screen::fn get_current_desktop -->
Returns the current workspace for `self` when running under a
window manager that supports multiple workspaces, as described
in the
[Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.

# Returns

the current workspace, or 0 if workspaces are not supported
<!-- impl X11Screen::fn get_monitor_output -->
Gets the XID of the specified output/monitor.
If the X server does not support version 1.2 of the RANDR
extension, 0 is returned.
## `monitor_num`
number of the monitor, between 0 and gdk_screen_get_n_monitors (screen)

# Returns

the XID of the monitor
<!-- impl X11Screen::fn get_number_of_desktops -->
Returns the number of workspaces for `self` when running under a
window manager that supports multiple workspaces, as described
in the
[Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.

# Returns

the number of workspaces, or 0 if workspaces are not supported
<!-- impl X11Screen::fn get_screen_number -->
Returns the index of a `X11Screen`.

# Returns

the position of `self` among the screens
 of its display
<!-- impl X11Screen::fn get_window_manager_name -->
Returns the name of the window manager for `self`.

# Returns

the name of the window manager screen `self`, or
"unknown" if the window manager is unknown. The string is owned by GDK
and should not be freed.
<!-- impl X11Screen::fn get_xscreen -->
Returns the screen of a `X11Screen`.

# Returns

an Xlib Screen*
<!-- impl X11Screen::fn supports_net_wm_hint -->
This function is specific to the X11 backend of GDK, and indicates
whether the window manager supports a certain hint from the
[Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.

When using this function, keep in mind that the window manager
can change over time; so you shouldn’t use this function in
a way that impacts persistent application state. A common bug
is that your application can start up before the window manager
does when the user logs in, and before the window manager starts
`X11Screen::supports_net_wm_hint` will return `false` for every property.
You can monitor the window_manager_changed signal on `X11Screen` to detect
a window manager change.
## `property_name`
name of the WM property

# Returns

`true` if the window manager supports `property`
<!-- struct X11Surface -->


# Implements

[`gdk::SurfaceExt`](../gdk/trait.SurfaceExt.html)
<!-- impl X11Surface::fn lookup_for_display -->
Looks up the `gdk::Surface` that wraps the given native window handle.
## `display`
the `gdk::Display` corresponding to the
 window handle
## `window`
an Xlib Window

# Returns

the `gdk::Surface` wrapper for the native
 window, or `None` if there is none.
<!-- impl X11Surface::fn get_desktop -->
Gets the number of the workspace `self` is on.

# Returns

the current workspace of `self`
<!-- impl X11Surface::fn get_group -->
Returns the group this surface belongs to.

# Returns

The group of this surface;
<!-- impl X11Surface::fn get_xid -->
Returns the X resource (surface) belonging to a `gdk::Surface`.

# Returns

the ID of `drawable`’s X resource.
<!-- impl X11Surface::fn move_to_current_desktop -->
Moves the surface to the correct workspace when running under a
window manager that supports multiple workspaces, as described
in the [Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.
Will not do anything if the surface is already on all workspaces.
<!-- impl X11Surface::fn move_to_desktop -->
Moves the surface to the given workspace when running unde a
window manager that supports multiple workspaces, as described
in the [Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.
## `desktop`
the number of the workspace to move the surface to
<!-- impl X11Surface::fn set_frame_sync_enabled -->
This function can be used to disable frame synchronization for a surface.
Normally frame synchronziation will be enabled or disabled based on whether
the system has a compositor that supports frame synchronization, but if
the surface is not directly managed by the window manager, then frame
synchronziation may need to be disabled. This is the case for a surface
embedded via the XEMBED protocol.
## `frame_sync_enabled`
whether frame-synchronization should be enabled
<!-- impl X11Surface::fn set_group -->
Sets the group leader of `self` to be `leader`.
See the ICCCM for details.
## `leader`
a `gdk::Surface`
<!-- impl X11Surface::fn set_skip_pager_hint -->
Sets a hint on `self` that pagers should not
display it. See the EWMH for details.
## `skips_pager`
`true` to skip pagers
<!-- impl X11Surface::fn set_skip_taskbar_hint -->
Sets a hint on `self` that taskbars should not
display it. See the EWMH for details.
## `skips_taskbar`
`true` to skip taskbars
<!-- impl X11Surface::fn set_theme_variant -->
GTK applications can request a dark theme variant. In order to
make other applications - namely window managers using GTK for
themeing - aware of this choice, GTK uses this function to
export the requested theme variant as _GTK_THEME_VARIANT property
on toplevel surfaces.

Note that this property is automatically updated by GTK, so this
function should only be used by applications which do not use GTK
to create toplevel surfaces.
## `variant`
the theme variant to export
<!-- impl X11Surface::fn set_urgency_hint -->
Sets a hint on `self` that it needs user attention.
See the ICCCM for details.
## `urgent`
`true` to indicate urgenct attention needed
<!-- impl X11Surface::fn set_user_time -->
The application can use this call to update the _NET_WM_USER_TIME
property on a toplevel surface. This property stores an Xserver
time which represents the time of the last user input event
received for this surface. This property may be used by the window
manager to alter the focus, stacking, and/or placement behavior of
surfaces when they are mapped depending on whether the new surface
was created by a user action or is a "pop-up" surface activated by a
timer or some other event.

Note that this property is automatically updated by GDK, so this
function should only be used by applications which handle input
events bypassing GDK.
## `timestamp`
An XServer timestamp to which the property should be set
<!-- impl X11Surface::fn set_utf8_property -->
This function modifies or removes an arbitrary X11 window
property of type UTF8_STRING. If the given `self` is
not a toplevel surface, it is ignored.
## `name`
Property name, will be interned as an X atom
## `value`
Property value, or `None` to delete
