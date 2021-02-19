<!-- file * -->
<!-- struct WaylandDevice -->


# Implements

[`gdk::DeviceExt`](../gdk/trait.DeviceExt.html), [`WaylandDeviceManualExt`](prelude/trait.WaylandDeviceManualExt.html)
<!-- impl WaylandDevice::fn get_node_path -->
Returns the `/dev/input/event*` path of this device.

For ``GdkDevices`` that possibly coalesce multiple hardware
devices (eg. mouse, keyboard, touch,...), this function
will return `None`.

This is most notably implemented for devices of type
`gdk::InputSource::Pen`, `gdk::InputSource::TabletPad`.

# Returns

the `/dev/input/event*` path of this device
<!-- impl WaylandDevice::fn get_wl_keyboard -->
Returns the Wayland wl_keyboard of a `gdk::Device`.

# Returns

a Wayland wl_keyboard
<!-- impl WaylandDevice::fn get_wl_pointer -->
Returns the Wayland wl_pointer of a `gdk::Device`.

# Returns

a Wayland wl_pointer
<!-- impl WaylandDevice::fn get_wl_seat -->
Returns the Wayland wl_seat of a `gdk::Device`.

# Returns

a Wayland wl_seat
<!-- struct WaylandDisplay -->


# Implements

[`gdk::DisplayExt`](../gdk/trait.DisplayExt.html), [`WaylandDisplayManualExt`](prelude/trait.WaylandDisplayManualExt.html)
<!-- impl WaylandDisplay::fn get_startup_notification_id -->
Gets the startup notification ID for a Wayland display, or `None`
if no ID has been defined.

# Returns

the startup notification ID for `self`, or `None`
<!-- impl WaylandDisplay::fn get_wl_compositor -->
Returns the Wayland global singleton compositor of a `gdk::Display`.

# Returns

a Wayland wl_compositor
<!-- impl WaylandDisplay::fn get_wl_display -->
Returns the Wayland wl_display of a `gdk::Display`.

# Returns

a Wayland wl_display
<!-- impl WaylandDisplay::fn query_registry -->
Returns `true` if the the interface was found in the display
`wl_registry.global` handler.
## `global`
global interface to query in the registry

# Returns

`true` if the global is offered by the compositor
<!-- impl WaylandDisplay::fn set_cursor_theme -->
Sets the cursor theme for the given `self`.
## `name`
the new cursor theme
## `size`
the size to use for cursors
<!-- impl WaylandDisplay::fn set_startup_notification_id -->
Sets the startup notification ID for a display.

This is usually taken from the value of the DESKTOP_STARTUP_ID
environment variable, but in some cases (such as the application not
being launched using `exec`) it can come from other sources.

The startup ID is also what is used to signal that the startup is
complete (for example, when opening a window or when calling
`gdk::DisplayExt::notify_startup_complete`).
## `startup_id`
the startup notification ID (must be valid utf8)
<!-- struct WaylandGLContext -->


# Implements

[`gdk::GLContextExt`](../gdk/trait.GLContextExt.html), [`gdk::DrawContextExt`](../gdk/trait.DrawContextExt.html)
<!-- struct WaylandMonitor -->


# Implements

[`gdk::MonitorExt`](../gdk/trait.MonitorExt.html), [`WaylandMonitorManualExt`](prelude/trait.WaylandMonitorManualExt.html)
<!-- impl WaylandMonitor::fn get_wl_output -->
Returns the Wayland wl_output of a `gdk::Monitor`.

# Returns

a Wayland wl_output
<!-- struct WaylandPopup -->


# Implements

[`WaylandSurfaceExt`](trait.WaylandSurfaceExt.html), [`gdk::SurfaceExt`](../gdk/trait.SurfaceExt.html), [`gdk::PopupExt`](../gdk/trait.PopupExt.html), [`WaylandSurfaceManualExt`](prelude/trait.WaylandSurfaceManualExt.html)
<!-- struct WaylandSeat -->


# Implements

[`gdk::SeatExt`](../gdk/trait.SeatExt.html), [`WaylandSeatManualExt`](prelude/trait.WaylandSeatManualExt.html)
<!-- impl WaylandSeat::fn get_wl_seat -->
Returns the Wayland `wl_seat` of a `gdk::Seat`.

# Returns

a Wayland `wl_seat`
<!-- struct WaylandSurface -->


# Implements

[`gdk::SurfaceExt`](../gdk/trait.SurfaceExt.html), [`WaylandSurfaceManualExt`](prelude/trait.WaylandSurfaceManualExt.html)
<!-- impl WaylandSurface::fn get_wl_surface -->
Returns the Wayland surface of a `gdk::Surface`.

# Returns

a Wayland wl_surface
<!-- struct WaylandToplevel -->


# Implements

[`WaylandSurfaceExt`](trait.WaylandSurfaceExt.html), [`gdk::SurfaceExt`](../gdk/trait.SurfaceExt.html), [`gdk::ToplevelExt`](../gdk/trait.ToplevelExt.html), [`WaylandSurfaceManualExt`](prelude/trait.WaylandSurfaceManualExt.html)
<!-- impl WaylandToplevel::fn export_handle -->
Asynchronously obtains a handle for a surface that can be passed
to other processes. When the handle has been obtained, `callback`
will be called.

It is an error to call this function on a surface that is already
exported.

When the handle is no longer needed, `WaylandToplevel::unexport_handle`
should be called to clean up resources.

The main purpose for obtaining a handle is to mark a surface
from another surface as transient for this one, see
`WaylandToplevel::set_transient_for_exported`.

Note that this API depends on an unstable Wayland protocol,
and thus may require changes in the future.
## `callback`
callback to call with the handle
## `user_data`
user data for `callback`
## `destroy_func`
destroy notify for `user_data`

# Returns

`true` if the handle has been requested, `false` if
 an error occurred.
<!-- impl WaylandToplevel::fn set_application_id -->
Sets the application id on a `gdk::Toplevel`.
## `application_id`
the application id for the `self`
<!-- impl WaylandToplevel::fn set_transient_for_exported -->
Marks `self` as transient for the surface to which the given
`parent_handle_str` refers. Typically, the handle will originate
from a `WaylandToplevel::export_handle` call in another process.

Note that this API depends on an unstable Wayland protocol,
and thus may require changes in the future.
## `parent_handle_str`
an exported handle for a surface

# Returns

`true` if the surface has been marked as transient,
 `false` if an error occurred.
<!-- impl WaylandToplevel::fn unexport_handle -->
Destroys the handle that was obtained with
`WaylandToplevel::export_handle`.

It is an error to call this function on a surface that
does not have a handle.

Note that this API depends on an unstable Wayland protocol,
and thus may require changes in the future.
