<!-- file * -->
<!-- struct X11AppLaunchContext -->


# Implements

[`gdk::AppLaunchContextExt`](../gdk/trait.AppLaunchContextExt.html), [`gio::AppLaunchContextExt`](../gio/trait.AppLaunchContextExt.html)
<!-- struct X11Cursor -->


# Implements

[`gdk::CursorExt`](../gdk/trait.CursorExt.html)
<!-- impl X11Cursor::fn get_xcursor -->
Returns the X cursor belonging to a `gdk::Cursor`.

# Returns

an Xlib Cursor.
<!-- impl X11Cursor::fn get_xdisplay -->
Returns the display of a `gdk::Cursor`.

# Returns

an Xlib Display*.
<!-- struct X11DeviceCore -->


# Implements

[`gdk::DeviceExt`](../gdk/trait.DeviceExt.html)
<!-- struct X11DeviceManagerCore -->


# Implements

[`gdk::DeviceManagerExt`](../gdk/trait.DeviceManagerExt.html)
<!-- struct X11DeviceManagerXI2 -->


# Implements

[`X11DeviceManagerCoreExt`](trait.X11DeviceManagerCoreExt.html), [`gdk::DeviceManagerExt`](../gdk/trait.DeviceManagerExt.html)
<!-- struct X11DeviceXI2 -->


# Implements

[`gdk::DeviceExt`](../gdk/trait.DeviceExt.html)
<!-- struct X11Display -->


# Implements

[`gdk::DisplayExt`](../gdk/trait.DisplayExt.html)
<!-- impl X11Display::fn get_glx_version -->
Retrieves the version of the GLX implementation.

Feature: `v3_16`

## `display`
a `gdk::Display`
## `major`
return location for the GLX major version
## `minor`
return location for the GLX minor version

# Returns

`true` if GLX is available
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

See `gdk_error_trap_pop` for the all-displays-at-once
equivalent.

# Returns

X error code or 0 on success
<!-- impl X11Display::fn error_trap_pop_ignored -->
Pops the error trap pushed by `X11Display::error_trap_push`.
Does not block to see if an error occurred; merely records the
range of requests to ignore errors for, and ignores those errors
if they arrive asynchronously.

See `gdk_error_trap_pop_ignored` for the all-displays-at-once
equivalent.
<!-- impl X11Display::fn error_trap_push -->
Begins a range of X requests on `self` for which X error events
will be ignored. Unignored errors (when no trap is pushed) will abort
the application. Use `X11Display::error_trap_pop` or
`X11Display::error_trap_pop_ignored`to lift a trap pushed
with this function.

See also `gdk_error_trap_push` to push a trap on all displays.
<!-- impl X11Display::fn get_startup_notification_id -->
Gets the startup notification ID for a display.

# Returns

the startup notification ID for `self`
<!-- impl X11Display::fn get_user_time -->
Returns the timestamp of the last user interaction on
`self`. The timestamp is taken from events caused
by user interaction such as key presses or pointer
movements. See `X11Window::set_user_time`.

# Returns

the timestamp of the last user interaction
<!-- impl X11Display::fn get_xdisplay -->
Returns the X display of a `gdk::Display`.

# Returns

an X display
<!-- impl X11Display::fn grab -->
Call XGrabServer() on `self`.
To ungrab the display again, use `X11Display::ungrab`.

`X11Display::grab`/`X11Display::ungrab` calls can be nested.
<!-- impl X11Display::fn set_cursor_theme -->
Sets the cursor theme from which the images for cursor
should be taken.

If the windowing system supports it, existing cursors created
with `gdk::Cursor::new`, `gdk::Cursor::new_for_display` and
`gdk::Cursor::new_from_name` are updated to reflect the theme
change. Custom cursors constructed with
`gdk::Cursor::new_from_pixbuf` will have to be handled
by the application (GTK+ applications can learn about
cursor theme changes by listening for change notification
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
`gdk_notify_startup_complete`).
## `startup_id`
the startup notification ID (must be valid utf8)
<!-- impl X11Display::fn set_window_scale -->
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
location to store the encoding atom
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
an atom representing the encoding. The most
 common values for this are STRING, or COMPOUND_TEXT.
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
 freed using `gdk_free_text_list`.

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
<!-- struct X11DisplayManager -->


# Implements

[`gdk::DisplayManagerExt`](../gdk/trait.DisplayManagerExt.html)
<!-- struct X11DragContext -->


# Implements

[`gdk::DragContextExt`](../gdk/trait.DragContextExt.html)
<!-- struct X11GLContext -->


Feature: `v3_16`

# Implements

[`gdk::GLContextExt`](../gdk/trait.GLContextExt.html)
<!-- struct X11Keymap -->


# Implements

[`gdk::KeymapExt`](../gdk/trait.KeymapExt.html)
<!-- impl X11Keymap::fn get_group_for_state -->
Extracts the group from the state field sent in an X Key event.
This is only needed for code processing raw X events, since `gdk::EventKey`
directly includes an is_modifier field.
## `state`
raw state returned from X

# Returns

the index of the active keyboard group for the event
<!-- impl X11Keymap::fn key_is_modifier -->
Determines whether a particular key code represents a key that
is a modifier. That is, it’s a key that normally just affects
the keyboard state and the behavior of other keys rather than
producing a direct effect itself. This is only needed for code
processing raw X events, since `gdk::EventKey` directly includes
an is_modifier field.
## `keycode`
the hardware keycode from a key event

# Returns

`true` if the hardware keycode is a modifier key
<!-- struct X11Monitor -->


Feature: `v3_22`

# Implements

[`gdk::MonitorExt`](../gdk/trait.MonitorExt.html)
<!-- struct X11Screen -->


# Implements

[`gdk::ScreenExt`](../gdk/trait.ScreenExt.html)
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
Returns the index of a `gdk::Screen`.

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
Returns the screen of a `gdk::Screen`.

# Returns

an Xlib Screen*
<!-- impl X11Screen::fn lookup_visual -->
Looks up the `gdk::Visual` for a particular screen and X Visual ID.
## `xvisualid`
an X Visual ID.

# Returns

the `gdk::Visual` (owned by the screen
 object), or `None` if the visual ID wasn’t found.
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
You can monitor the window_manager_changed signal on `gdk::Screen` to detect
a window manager change.
## `property`
a property atom.

# Returns

`true` if the window manager supports `property`
<!-- struct X11Visual -->


# Implements

[`gdk::VisualExt`](../gdk/trait.VisualExt.html)
<!-- impl X11Visual::fn get_xvisual -->
Returns the X visual belonging to a `gdk::Visual`.

# Returns

an Xlib Visual*.
<!-- struct X11Window -->


# Implements

[`gdk::WindowExt`](../gdk/trait.WindowExt.html)
<!-- impl X11Window::fn foreign_new_for_display -->
Wraps a native window in a `gdk::Window`. The function will try to
look up the window using `X11Window::lookup_for_display` first.
If it does not find it there, it will create a new window.

This may fail if the window has been destroyed. If the window
was already known to GDK, a new reference to the existing
`gdk::Window` is returned.
## `display`
the `gdk::Display` where the window handle comes from.
## `window`
an Xlib Window

# Returns

a `gdk::Window` wrapper for the native
 window, or `None` if the window has been destroyed. The wrapper
 will be newly created, if one doesn’t exist already.
<!-- impl X11Window::fn lookup_for_display -->
Looks up the `gdk::Window` that wraps the given native window handle.
## `display`
the `gdk::Display` corresponding to the
 window handle
## `window`
an Xlib Window

# Returns

the `gdk::Window` wrapper for the native
 window, or `None` if there is none.
<!-- impl X11Window::fn get_desktop -->
Gets the number of the workspace `self` is on.

# Returns

the current workspace of `self`
<!-- impl X11Window::fn get_xid -->
Returns the X resource (window) belonging to a `gdk::Window`.

# Returns

the ID of `drawable`’s X resource.
<!-- impl X11Window::fn move_to_current_desktop -->
Moves the window to the correct workspace when running under a
window manager that supports multiple workspaces, as described
in the [Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.
Will not do anything if the window is already on all workspaces.
<!-- impl X11Window::fn move_to_desktop -->
Moves the window to the given workspace when running unde a
window manager that supports multiple workspaces, as described
in the [Extended Window Manager Hints](http://www.freedesktop.org/Standards/wm-spec) specification.
## `desktop`
the number of the workspace to move the window to
<!-- impl X11Window::fn set_frame_sync_enabled -->
This function can be used to disable frame synchronization for a window.
Normally frame synchronziation will be enabled or disabled based on whether
the system has a compositor that supports frame synchronization, but if
the window is not directly managed by the window manager, then frame
synchronziation may need to be disabled. This is the case for a window
embedded via the XEMBED protocol.
## `frame_sync_enabled`
whether frame-synchronization should be enabled
<!-- impl X11Window::fn set_hide_titlebar_when_maximized -->
Set a hint for the window manager, requesting that the titlebar
should be hidden when the window is maximized.

Note that this property is automatically updated by GTK+, so this
function should only be used by applications which do not use GTK+
to create toplevel windows.
## `hide_titlebar_when_maximized`
whether to hide the titlebar when
 maximized
<!-- impl X11Window::fn set_theme_variant -->
GTK+ applications can request a dark theme variant. In order to
make other applications - namely window managers using GTK+ for
themeing - aware of this choice, GTK+ uses this function to
export the requested theme variant as _GTK_THEME_VARIANT property
on toplevel windows.

Note that this property is automatically updated by GTK+, so this
function should only be used by applications which do not use GTK+
to create toplevel windows.
## `variant`
the theme variant to export
<!-- impl X11Window::fn set_user_time -->
The application can use this call to update the _NET_WM_USER_TIME
property on a toplevel window. This property stores an Xserver
time which represents the time of the last user input event
received for this window. This property may be used by the window
manager to alter the focus, stacking, and/or placement behavior of
windows when they are mapped depending on whether the new window
was created by a user action or is a "pop-up" window activated by a
timer or some other event.

Note that this property is automatically updated by GDK, so this
function should only be used by applications which handle input
events bypassing GDK.
## `timestamp`
An XServer timestamp to which the property should be set
<!-- impl X11Window::fn set_utf8_property -->
This function modifies or removes an arbitrary X11 window
property of type UTF8_STRING. If the given `self` is
not a toplevel window, it is ignored.
## `name`
Property name, will be interned as an X atom
## `value`
Property value, or `None` to delete
