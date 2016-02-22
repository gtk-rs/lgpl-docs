<!-- file cursor.rs -->
<!-- type Type::struct Cursor -->
Standard and pixmap cursors.
<!-- impl Cursor::fn new -->
Creates a new cursor from the set of builtin cursors for the default display. See
Cursor::new_for_display().

To make the cursor invisible, use Type::BlankCursor.

Warning : Cursor::new() has been deprecated since version 3.16 and should not be used
in newly-written code.

Use Cursor::new_for_display() instead.
<!-- impl Cursor::fn new_from_pixbuf -->
Creates a new cursor from a pixbuf.

Not all GDK backends support RGBA cursors. If they are not supported, a monochrome
approximation will be displayed. The functions Display::supports_cursor_alpha() and
Display::supports_cursor_color() can be used to determine whether RGBA cursors are
supported; Display::get_default_cursor_size() and Display::get_maximal_cursor_size()
give information about cursor sizes.

If `x` or `y` are -1, the pixbuf must have options named “x_hot” and “y_hot”, resp.,
containing integer values between 0 and the width resp. height of the pixbuf. (Since:
3.0)

On the X backend, support for RGBA cursors requires a sufficently new version of the
X Render extension.
<!-- impl Cursor::fn new_from_name -->
Creates a new cursor by looking up `name` in the current cursor theme.
<!-- impl Cursor::fn new_for_display -->
Creates a new cursor from the set of builtin cursors.

You can find a list of useful ones here:
https://developer.gnome.org/gdk3/stable/gdk3-Cursors.html#gdk-cursor-new-for-display
<!-- impl Cursor::fn get_display -->
Returns the display on which the Cursor is defined.
<!-- impl Cursor::fn get_image -->
Returns a Pixbuf with the image used to display the cursor.

Note that depending on the capabilities of the windowing system and on the cursor,
GDK may not be able to obtain the image data. In this case, None is returned.
<!-- impl Cursor::fn get_cursor_type -->
Returns the cursor type for this cursor.
<!-- file lib.rs -->
<!-- file_comment -->
Bindings and wrappers for __GDK__
<!-- file device_manager.rs -->
<!-- use ffi;::struct DeviceManager -->
Functions for handling input devices.

In addition to a single pointer and keyboard for user interface input, GDK contains support
for a variety of input devices, including graphics tablets, touchscreens and multiple
pointers/keyboards interacting simultaneously with the user interface. Such input devices
often have additional features, such as sub-pixel positioning information and additional
device-dependent information.

In order to query the device hierarchy and be aware of changes in the device hierarchy
(such as virtual devices being created or removed, or physical devices being plugged or
unplugged), GDK provides DeviceManager.

By default, and if the platform supports it, GDK is aware of multiple keyboard/pointer pairs
and multitouch devices. This behavior can be changed by calling
DeviceManager::disable_multidevice() before Display::open(). There should rarely be a need to
do that though, since GDK defaults to a compatibility mode in which it will emit just one
enter/leave event pair for all devices on a window. To enable per-device enter/leave events
and other multi-pointer interaction features, Window::set_support_multidevice() must be called
on gdk::Windows (or gtk::Widget::set_support_multidevice() on widgets) window. See the
Window::set_support_multidevice() documentation for more information.

On X11, multi-device support is implemented through XInput 2. Unless
DeviceManager::disable_multidevice() is called, the XInput 2 GdkDeviceManager implementation
will be used as the input source. Otherwise either the core or XInput 1 implementations will
be used.

For simple applications that don’t have any special interest in input devices, the so-called
"client pointer" provides a reasonable approximation to a simple setup with a single pointer
and keyboard. The device that has been set as the client pointer can be accessed via
DeviceManager::get_client_pointer().

Conceptually, in multidevice mode there are 2 device types. Virtual devices (or master devices)
are represented by the pointer cursors and keyboard foci that are seen on the screen. Physical
devices (or slave devices) represent the hardware that is controlling the virtual devices, and
thus have no visible cursor on the screen.

Virtual devices are always paired, so there is a keyboard device for every pointer device.
Associations between devices may be inspected through Device::get_associated_device().

There may be several virtual devices, and several physical devices could be controlling each of
these virtual devices. Physical devices may also be “floating”, which means they are not attached
to any virtual device.

#Master and slave devices

```text
carlos@sacarino:~$ xinput list
⎡ Virtual core pointer                          id=2    [master pointer  (3)]
⎜   ↳ Virtual core XTEST pointer                id=4    [slave  pointer  (2)]
⎜   ↳ Wacom ISDv4 E6 Pen stylus                 id=10   [slave  pointer  (2)]
⎜   ↳ Wacom ISDv4 E6 Finger touch               id=11   [slave  pointer  (2)]
⎜   ↳ SynPS/2 Synaptics TouchPad                id=13   [slave  pointer  (2)]
⎜   ↳ TPPS/2 IBM TrackPoint                     id=14   [slave  pointer  (2)]
⎜   ↳ Wacom ISDv4 E6 Pen eraser                 id=16   [slave  pointer  (2)]
⎣ Virtual core keyboard                         id=3    [master keyboard (2)]
    ↳ Virtual core XTEST keyboard               id=5    [slave  keyboard (3)]
    ↳ Power Button                              id=6    [slave  keyboard (3)]
    ↳ Video Bus                                 id=7    [slave  keyboard (3)]
    ↳ Sleep Button                              id=8    [slave  keyboard (3)]
    ↳ Integrated Camera                         id=9    [slave  keyboard (3)]
    ↳ AT Translated Set 2 keyboard              id=12   [slave  keyboard (3)]
    ↳ ThinkPad Extra Buttons                    id=15   [slave  keyboard (3)]
```

By default, GDK will automatically listen for events coming from all master devices, setting the
Device for all events coming from input devices. Events containing device information are
GDK_MOTION_NOTIFY, GDK_BUTTON_PRESS, GDK_2BUTTON_PRESS, GDK_3BUTTON_PRESS, GDK_BUTTON_RELEASE,
GDK_SCROLL, GDK_KEY_PRESS, GDK_KEY_RELEASE, GDK_ENTER_NOTIFY, GDK_LEAVE_NOTIFY, GDK_FOCUS_CHANGE,
GDK_PROXIMITY_IN, GDK_PROXIMITY_OUT, GDK_DRAG_ENTER, GDK_DRAG_LEAVE, GDK_DRAG_MOTION,
GDK_DRAG_STATUS, GDK_DROP_START, GDK_DROP_FINISHED and GDK_GRAB_BROKEN. When dealing with an event
on a master device, it is possible to get the source (slave) device that the event originated from
via Event::get_source_device().

On a standard session, all physical devices are connected by default to the "Virtual Core
Pointer/Keyboard" master devices, hence routing all events through these. This behavior is only
modified by device grabs, where the slave device is temporarily detached for as long as the grab is
held, and more permanently by user modifications to the device hierarchy.

On certain application specific setups, it may make sense to detach a physical device from its
master pointer, and mapping it to an specific window. This can be achieved by the combination of
Device::grab() and Device::set_mode().

In order to listen for events coming from devices other than a virtual device,
Window::set_device_events() must be called. Generally, this function can be used to modify the event
mask for any given device.

Input devices may also provide additional information besides X/Y. For example, graphics tablets may
also provide pressure and X/Y tilt information. This information is device-dependent, and may be
queried through Device::get_axis(). In multidevice mode, virtual devices will change axes in order to
always represent the physical device that is routing events through it. Whenever the physical device
changes, the “n-axes” property will be notified, and Device::list_axes() will return the new device
axes.

Devices may also have associated “keys” or macro buttons. Such keys can be globally set to map into
normal X keyboard events. The mapping is set using Device::set_key().
<!-- impl DeviceManager::fn disable_multidevice -->
Disables multidevice support in GDK. This call must happen prior to Display::open(),
gtk::init(), gtk::init_with_args() or gtk::init_check() in order to take effect.

Most common GTK+ applications won’t ever need to call this. Only applications that do
mixed GDK/Xlib calls could want to disable multidevice support if such Xlib code deals
with input devices in any way and doesn’t observe the presence of XInput 2.
<!-- impl DeviceManager::fn get_display -->
Gets the Display associated to `self`.
<!-- impl DeviceManager::fn get_client_pointer -->
Returns the client pointer, that is, the master pointer that acts as the core pointer
for this application. In X11, window managers may change this depending on the
interaction pattern under the presence of several pointers.

You should use this function seldomly, only in code that isn’t triggered by a Event and
there aren’t other means to get a meaningful Device to operate on.
<!-- file gl_context.rs -->
<!-- use ffi;::struct GLContext -->
An object representing the platform-specific OpenGL drawing context.

GLContexts are created for a GdkWindow using gdk_window_create_gl_context(), and the context
will match the GdkVisual of the window.

A GLContext is not tied to any particular normal framebuffer. For instance, it cannot draw to
the GdkWindow back buffer. The GDK repaint system is in full control of the painting to that.
Instead, you can create render buffers or textures and use gdk_cairo_draw_from_gl() in the draw
function of your widget to draw them. Then GDK will handle the integration of your rendering
with that of other widgets.

Support for GLContext is platform-specific, context creation can fail, returning None context.

A GLContext has to be made "current" in order to start using it, otherwise any OpenGL call will
be ignored.

##Creating a new OpenGL context

In order to create a new GLContext instance you need a GdkWindow, which you typically get during
the realize call of a widget.

A GLContext is not realized until either gdk_gl_context_make_current(), or until it is realized
using gdk_gl_context_realize(). It is possible to specify details of the GL context like the
OpenGL version to be used, or whether the GL context should have extra state validation enabled
after calling gdk_window_create_gl_context() by calling gdk_gl_context_realize(). If the
realization fails you have the option to change the settings of the GLContext and try again.

##Using a GLContext

You will need to make the GLContext the current context before issuing OpenGL calls; the system
sends OpenGL commands to whichever context is current. It is possible to have multiple contexts,
so you always need to ensure that the one which you want to draw with is the current one before
issuing commands:

```Rust,ignore
GLContext::make_current(context);
```

You can now perform your drawing using OpenGL commands.

You can check which GLContext is the current one by using gdk_gl_context_get_current(); you can
also unset any GLContext that is currently set by calling gdk_gl_context_clear_current().
<!-- impl GLContext::fn get_display -->
Retrieves the Display the context is created for
<!-- impl GLContext::fn get_window -->
Retrieves the GdkWindow used by self.
<!-- impl GLContext::fn get_shared_context -->
Retrieves the GLContext that this context share data with.
<!-- impl GLContext::fn get_version -->
Retrieves the OpenGL version of the context.

The context must be realized prior to calling this function.
<!-- impl GLContext::fn set_required_version -->
Sets the major and minor version of OpenGL to request.

Setting major and minor to zero will use the default values.

The GLContext must not be realized or made current prior to calling this function.
<!-- impl GLContext::fn set_debug_enabled -->
Sets whether the GLContext should perform extra validations and run time checking.
This is useful during development, but has additional overhead.

The GLContext must not be realized or made current prior to calling this function.
<!-- impl GLContext::fn get_debug_enabled -->
Retrieves the value set using set_debug_enabled().
<!-- impl GLContext::fn set_forward_compatible -->
Sets whether the GLContext should be forward compatible.

Forward compatibile contexts must not support OpenGL functionality that has been
marked as deprecated in the requested version; non-forward compatible contexts, on
the other hand, must support both deprecated and non deprecated functionality.

The GLContext must not be realized or made current prior to calling this function.
<!-- impl GLContext::fn get_forward_compatible -->
Retrieves the value set using set_forward_compatible().
<!-- impl GLContext::fn realize -->
Realizes the given GLContext.

It is safe to call this function on a realized GLContext.
<!-- impl GLContext::fn make_current -->
Makes self the current context.
<!-- impl GLContext::fn get_current -->
Retrieves the current GLContext.
<!-- impl GLContext::fn clear_current -->
Clears the current GLContext.

Any OpenGL call after this function returns will be ignored until GLContext::make_current()
is called.
<!-- file display_manager.rs -->
<!-- use ffi;::struct DisplayManager -->
Maintains a list of all open `Display`s.
<!-- file pixbuf/animation.rs -->
<!-- file_comment -->
Animations — Animated images.
<!-- file pixbuf/mod.rs -->
<!-- use std -->
The GdkPixbuf structure contains information that describes an image in memory.
<!-- use self::struct Pixbuf -->
Represents an image.

This is the main structure in the `gdk-pixbuf` library. It is used to represent images. It contains information about the image's pixel
data, its color space, bits per sample, width and height, and the rowstride (the number of bytes between the start of one row and the
start of the next).
<!-- impl Pixbuf::fn new_from_vec -->
Creates a `Pixbuf` using a `Vec` as image data.

Only `bits_per_sample == 8` supported.
<!-- impl Pixbuf::fn scale_simple -->
Create a new GdkPixbuf containing a copy of the current pixbuf scaled
to `dest_width` x `dest_height`. The calling pixbuf remains unaffected.
`interp_type` should be `Nearest` if you want maximum speed (but when
scaling down `Nearest` is usually unusably ugly). The default
`interp_type` should be `Bilinear` which offers reasonable quality and
speed.

You can scale a sub-portion of `src` by creating a sub-pixbuf pointing
into src ; see `new_subpixbuf()`.

For more complicated scaling/compositing see `scale()` and
`composite()`.
<!-- impl Pixbuf::fn scale -->
Creates a transformation of the calling pixbuf by scaling by
`scale_x` and `scale_y` then translating by `offset_x` and `offset_y`,
then renders the rectangle (`dest_x`, `dest_y`, `dest_width`,
`dest_height`) of the resulting pixbuf onto the destination pixbuf,
replacing the previous contents.

Try to use `scale_simple()` first; this function is the
industrial-strength power tool you can fall back to if `scale_simple()`
isn't powerful enough.

If the source rectangle overlaps the destination rectangle on the same
pixbuf, it will be overwritten during the scaling which results in
rendering artifacts.
<!-- impl Pixbuf::fn composite -->
Creates a transformation of the calling pixbuf by scaling by `scale_x`
and `scale_y` then translating by `offset_x` and `offset_y`. This gives
an image in the coordinates of the destination pixbuf. The rectangle
(`dest_x`, `dest_y`, `dest_width`, `dest_height`) is then composited
onto the corresponding rectangle of the original destination image.

![Diagram of `composite` process](https://developer.gnome.org/gdk-pixbuf/unstable/composite.png)

When the destination rectangle contains parts not in the source image,
the data at the edges of the source image is replicated to infinity.
<!-- impl Pixbuf::fn flip -->
Flips a pixbuf horizontally or vertically and returns the result in a
new pixbuf, or `Err` if not enough memory could be allocated for it.
<!-- impl Pixbuf::fn put_pixel -->
a convenient function
It won't work for pixbufs with images that are other than 8 bits per sample or channel, but it will work for most of the
pixbufs that GTK+ uses.
<!-- file pixbuf/loader.rs -->
<!-- use super::struct PixbufLoader -->
Application-driven progressive image loading.
<!-- impl PixbufLoader::fn new -->
Creates a new pixbuf loader object.

# Failures
Returns `None` if the pixbuf loader cannot be created.
<!-- impl PixbufLoader::fn new_with_type -->
Creates a new pixbuf loader object that always attempts to parse image
data as if it were an image of type `image_type`, instead of
identifying the type automatically. Useful if you want an error if the
image isn't the expected type, for loading image formats that can't be
reliably identified by looking at the data, or if the user manually
forces a specific type.

The list of supported image formats depends on what image loaders are
installed, but typically "png", "jpeg", "gif", "tiff" and "xpm" are
among the supported formats.

# Failures
Returns an `Error` if the pixbuf loader cannot be created. Query the
error for more detailed information.

# Panics
Fails if the pixbuf loader cannot be retrieved. 
<!-- impl PixbufLoader::fn new_with_mime_type -->
Creates a new pixbuf loader object that always attempts to parse image
data as if it were an image of mime type `mime_type`, instead of
identifying the type automatically. Useful if you want an error if the
image isn't the expected mime type, for loading image formats that can't
be reliably identified by looking at the data, or if the user manually
forces a specific mime type.

The list of supported mime types depends on what image loaders are
installed, but typically "image/png", "image/jpeg", "image/gif",
"image/tiff" and "image/x-xpixmap" are among the supported mime types.

# Failures
Returns an `Error` if the pixbuf loader cannot be created. Query the
error for more detailed information.

# Panics
Fails if the pixbuf loader cannot be retrieved. 
<!-- impl PixbufLoader::fn get_format -->
Obtains the available information about the format of the currently
loading image file.  Returns `None` if not enough data has been written
to determine the format.
<!-- impl PixbufLoader::fn loader_write -->
This will cause a pixbuf loader to parse the  bytes of an image stored
in `buf`. It will return `Ok` if the data was loaded successfully, and
`Err` if an error occurred. In the latter case, the loader will be
closed, and will not accept further writes.

# Failures
Returns an `Error` if `buf` cannot be written to the loader.  Query the
`Error` for more detailed information.
<!-- impl PixbufLoader::fn set_size -->
Causes the image to be scaled while it is loaded. The desired image
size can be determined relative to the original size of the image by
calling `set_size()` from a signal handler for the `size-prepared`
signal.

Attempts to set the desired image size are ignored after the emission
of the `size-prepared` signal.
<!-- impl PixbufLoader::fn get_pixbuf -->
Queries the `Pixbuf` that a pixbuf loader is currently creating. In
general it only makes sense to call this function after the
`area-prepared` signal has been emitted by the loader; this means that
enough data has been read to know the size of the image that will be
allocated. If the loader has not received enough data via
`loader_write()`, then this function returns `None`. If the loader is
an animation, it will return the "static image" of the animation.
<!-- impl PixbufLoader::fn close -->
Informs a pixbuf loader that no further writes with `loader_write()`
will occur, so that it can free its internal loading structures. Also,
tries to parse any data that hasn't yet been parsed.

# Failures
If the remaining data is partial or corrupt, an error will be returned.
Query the `Error` for more detailed information.
<!-- file pixbuf/format.rs -->
<!-- file_comment -->
A GdkPixbufFormat contains information about the image format accepted by a module. Only modules should
access the fields directly, applications should use the gdk_pixbuf_format_* functions.
<!-- file rgba.rs -->
<!-- file_comment -->
RGBA Colors — RGBA colors
<!-- file keys.rs -->
<!-- file_comment -->
Keyboard Handling Functions
<!-- file frame_clock.rs -->
<!-- type Phase::struct FrameClock -->
Frame clock syncs painting to a window or display.
<!-- file drag_context.rs -->
<!-- file rectangle.rs -->
<!-- file_comment -->
Rectangles — Simple graphical data type
<!-- file screen.rs -->
<!-- use ffi;::struct Screen -->
Object representing a physical screen.
<!-- file device.rs -->
<!-- use ffi;::struct Device -->
Object representing an input device.
<!-- impl Device::fn get_name -->
Determines the name of the device.
<!-- impl Device::fn get_vendor_id -->
Returns the vendor ID of this device, or NULL if this information couldn't be obtained. This
ID is retrieved from the device, and is thus constant for it.

This function, together with gdk_device_get_product_id(), can be used to eg. compose GSettings
paths to store settings for this device.

```ignore
fn get_device_settings(device: &Device) -> GSettings {
    GSettings *settings;
    GdkDevice *device;
    gchar *path;

    let vendor = device.get_vendor_id().unwrap();
    let product = device.get_product_id().unwrap();

    let path = format!("/org/example/app/devices/{}:{}/", vendor, product);
    g_settings_new_with_path(DEVICE_SCHEMA, path);
}
```
<!-- impl Device::fn get_product_id -->
Returns the product ID of this device, or None if this information couldn't be obtained. This
ID is retrieved from the device, and is thus constant for it. See Device::get_vendor_id() for
more information.
<!-- impl Device::fn get_source -->
Determines the type of the device.
<!-- impl Device::fn set_mode -->
Sets a the mode of an input device. The mode controls if the device is active and whether the
device’s range is mapped to the entire screen or to a single window.

Note: This is only meaningful for floating devices, master devices (and slaves connected to
these) drive the pointer cursor, which is not limited by the input mode.
<!-- impl Device::fn get_mode -->
Determines the mode of the device.
<!-- impl Device::fn set_key -->
Specifies the X key event to generate when a macro button of a device is pressed.
<!-- impl Device::fn get_key -->
If `index_` has a valid keyval, this function will return true and fill in `keyval` and
`modifiers` with the keyval settings.
<!-- impl Device::fn set_axis_use -->
Specifies how an axis of a device is used.
<!-- impl Device::fn get_axis_use -->
Returns the axis use for `index_`.
<!-- impl Device::fn get_associated_device -->
Returns the associated `self` to `self` , if `self` is of type DeviceType::Master, it
will return the paired pointer or keyboard.

If `self` is of type DeviceType::Slave, it will return the master device to which `self`
is attached to.

If `self` is of type DeviceType::Floating, None will be returned, as there is no
associated device.
<!-- impl Device::fn get_device_type -->
Returns the device type for `self`.
<!-- impl Device::fn get_display -->
Returns the Display to which `self` pertains.
<!-- impl Device::fn get_has_cursor -->
Determines whether the pointer follows device motion. This is not meaningful for
keyboard devices, which don't have a pointer.
<!-- impl Device::fn get_n_axes -->
Returns the number of axes the device currently has.
<!-- impl Device::fn get_n_keys -->
Returns the number of keys the device currently has.
<!-- impl Device::fn warp -->
Warps `self` in display to the point `x`, `y` on the screen `screen`, unless the device
is confined to a window by a grab, in which case it will be moved as far as allowed by
the grab. Warping the pointer creates events as if the user had moved the mouse
instantaneously to the destination.

Note that the pointer should normally be under the control of the user. This function
was added to cover some rare use cases like keyboard navigation support for the color
picker in the gtk::ColorSelectionDialog.
<!-- impl Device::fn grab -->
Grabs the device so that all events coming from this device are passed to this
application until the device is ungrabbed with Device::ungrab(), or the window becomes
unviewable. This overrides any previous grab on the device by this client.

Note that `self` and `window` need to be on the same display.

Device grabs are used for operations which need complete control over the given device
events (either pointer or keyboard). For example in GTK+ this is used for Drag and Drop
operations, popup menus and such.

Note that if the event mask of an X window has selected both button press and button
release events, then a button press event will cause an automatic pointer grab until
the button is released. X does this automatically since most applications expect to
receive button press and release events in pairs. It is equivalent to a pointer grab
on the window with `owner_events` set to true.

If you set up anything at the time you take the grab that needs to be cleaned up when
the grab ends, you should handle the GdkEventGrabBroken events that are emitted when
the grab ends unvoluntarily.
<!-- impl Device::fn ungrab -->
Release any grab on `self`.
<!-- impl Device::fn get_axis -->
Interprets an array of double as axis values for a given device, and locates the value
in the array for a given axis use.
<!-- file app_launch_context.rs -->
<!-- use ffi;::struct AppLaunchContext -->
Application launching — startup notification for applications.
<!-- impl AppLaunchContext::fn new -->
Creates a new AppLaunchContext.
<!-- impl AppLaunchContext::fn set_display -->
Sets the display on which applications will be launched when using this context. See also
`AppLaunchContext::set_screen()`.
<!-- impl AppLaunchContext::fn set_screen -->
Sets the screen on which applications will be launched when using this context. See also
`AppLaunchContext::set_display()`.

If both screen and display are set, the screen takes priority. If neither screen or display
are set, the default screen and display are used.
<!-- impl AppLaunchContext::fn set_desktop -->
Sets the workspace on which applications will be launched when using this context when running
under a window manager that supports multiple workspaces, as described in the
Extended Window Manager Hints.

When the workspace is not specified or desktop is set to -1, it is up to the window manager to
pick one, typically it will be the current workspace.
<!-- impl AppLaunchContext::fn set_timestamp -->
Sets the timestamp of self. The timestamp should ideally be taken from the event that triggered
the launch.

Window managers can use this information to avoid moving the focus to the newly launched application
when the user is busy typing in another window. This is also known as 'focus stealing prevention'.
<!-- impl AppLaunchContext::fn set_icon_name -->
Sets the icon for applications that are launched with this context. The icon_name will be
interpreted in the same way as the Icon field in desktop files. See also `AppLaunchContext::set_icon()`.

If both icon and icon_name are set, the icon_name takes priority. If neither icon or icon_name is set,
the icon is taken from either the file that is passed to launched application or from the GAppInfo for
the launched application itself.
<!-- file frame_timings.rs -->
<!-- use ffi;::struct FrameTimings -->
Object holding timing information for a single frame.
<!-- file visual.rs -->
<!-- use ffi;::struct Visual -->
Low-level display hardware information.
<!-- file window.rs -->
<!-- struct Attributes::variant title -->
title of the window (for toplevel windows)
<!-- struct Attributes::variant event_mask -->
event mask (see [gdk_window_set_events()](https://developer.gnome.org/gdk3/3.14/gdk3-Windows.html#gdk-window-set-events))
<!-- struct Attributes::variant x -->
X coordinate relative to parent window
<!-- struct Attributes::variant y -->
Y coordinate relative to parent window
<!-- struct Attributes::variant width -->
width of window
<!-- struct Attributes::variant height -->
height of window
<!-- struct Attributes::variant wclass -->
GDK_INPUT_OUTPUT (normal window) or GDK_INPUT_ONLY (invisible window that receives events)
<!-- struct Attributes::variant visual -->
GdkVisual for window
<!-- struct Attributes::variant window_type -->
type of window
<!-- struct Attributes::variant cursor -->
cursor for the window
<!-- struct Attributes::variant override_redirect -->
TRUE to bypass the window manager
<!-- struct Attributes::variant type_hint -->
a hint of the function of the window
<!-- file display.rs -->
<!-- use window::struct Display -->
Controls a set of `Screen`s and their associated input devices.

Display objects purpose are two fold:
* To manage and provide information about input devices (pointers and keyboards)
* To manage and provide information about the available Screens

Display objects are the GDK representation of an X Display, which can be described as
a workstation consisting of a keyboard, a pointing device (such as a mouse) and one or
more screens. It is used to open and keep track of various GdkScreen objects currently
instantiated by the application. It is also used to access the keyboard(s) and mouse
pointer(s) of the display.

Most of the input device handling has been factored out into the separate `DeviceManager`
object. Every display has a device manager, which you can obtain using
`Display::get_device_manager()`.
<!-- impl Display::fn open -->
Opens a display.
<!-- impl Display::fn get_default -->
Gets the default `Display`. This is a convenience function for:
`DisplayManager::get_default_display(DisplayManager::get())`.
<!-- impl Display::fn get_name -->
Gets the name of the display.
<!-- impl Display::fn get_screen -->
Returns a screen object for one of the screens of the display.
<!-- impl Display::fn get_default_screen -->
Get the default `Screen` for `self`.
<!-- impl Display::fn get_device_manager -->
Returns the `DeviceManager` associated to `self`.
<!-- impl Display::fn device_is_grabbed -->
Returns true if there is an ongoing grab on `device` for `self`.
<!-- impl Display::fn beep -->
Emits a short beep on `self`.
<!-- impl Display::fn sync -->
Flushes any requests queued for the windowing system and waits until all requests
have been handled. This is often used for making sure that the display is
synchronized with the current state of the program. Calling `Display::sync()`
before `gdk::error_trap_pop()` makes sure that any errors generated from earlier
requests are handled before the error trap is removed.

This is most useful for X11. On windowing systems where requests are handled
synchronously, this function will do nothing.
<!-- impl Display::fn flush -->
Flushes any requests queued for the windowing system; this happens automatically
when the main loop blocks waiting for new events, but if your application is drawing
without returning control to the main loop, you may need to call this function
explicitly. A common case where this function needs to be called is when an
application is executing drawing commands from a thread other than the thread where
the main loop is running.

This is most useful for X11. On windowing systems where requests are handled
synchronously, this function will do nothing.
<!-- impl Display::fn close -->
Closes the connection to the windowing system for the given display, and cleans
up associated resources.
<!-- impl Display::fn is_closed -->
Finds out if the display has been closed.
<!-- impl Display::fn has_pending -->
Returns whether the display has events that are waiting to be processed.
<!-- impl Display::fn set_double_click_time -->
Sets the double click time (two clicks within this time interval count as a
double click and result in a GDK_2BUTTON_PRESS event). Applications should not
set this, it is a global user-configured setting.
<!-- impl Display::fn set_double_click_distance -->
Sets the double click distance (two clicks within this distance count as a double
click and result in a GDK_2BUTTON_PRESS event). See also
`Display::set_double_click_time()`. Applications should not set this, it is a
global user-configured setting.
<!-- impl Display::fn supports_cursor_color -->
Returns true if multicolored cursors are supported on `self`. Otherwise, cursors
have only a forground and a background color.
<!-- impl Display::fn supports_cursor_alpha -->
Returns true if cursors can use an 8bit alpha channel on `self`. Otherwise, cursors
are restricted to bilevel alpha (i.e. a mask).
<!-- impl Display::fn get_default_cursor_size -->
Returns the default size to use for cursors on `self`.
<!-- impl Display::fn get_maximal_cursor_size -->
Gets the maximal size to use for cursors on `self`.
<!-- impl Display::fn get_default_group -->
Returns the default group leader window for all toplevel windows on `self`. This
window is implicitly created by GDK. See `Window::set_group()`.
<!-- impl Display::fn supports_selection_notification -->
Returns whether `EventOwnerChange` events will be sent when the owner of a
selection changes.
<!-- impl Display::fn request_selection_notification -->
Request `EventOwnerChange` events for ownership changes of the selection named
by the given atom.
<!-- impl Display::fn supports_clipboard_persistence -->
Returns whether the speicifed display supports clipboard persistance; i.e. if
it’s possible to store the clipboard data after an application has quit. On X11
this checks if a clipboard daemon is running.
<!-- impl Display::fn supports_shapes -->
Returns true if `Window::shape_combine_mask()` can be used to create shaped windows
on `self`.
<!-- impl Display::fn supports_input_shapes -->
Returns true if `Window::input_shape_combine_mask()` can be used to modify the input
shape of windows on `self`.
<!-- impl Display::fn supports_composite -->
Returns true if `Window::set_composited()` can be used to redirect drawing on the
window using compositing.

Currently this only works on X11 with XComposite and XDamage extensions available.

Deprecated since GDK 3.16
<!-- impl Display::fn get_app_launch_context -->
Returns a `AppLaunchContext` suitable for launching applications on the given
display.
<!-- impl Display::fn notify_startup_complete -->
Indicates to the GUI environment that the application has finished loading, using
a given identifier.

GTK+ will call this function automatically for GtkWindow with custom startup-notification
identifier unless `gtk::Window::set_auto_startup_notification()` is called to disable
that feature.
<!-- file enums.rs -->
<!-- mod modifier_type::const ShiftMask -->
the Shift key.,
<!-- mod modifier_type::const LockMask -->
a Lock key (depending on the modifier mapping of the X server this may either be CapsLock or ShiftLock).
<!-- mod modifier_type::const ControlMask -->
the Control key.
<!-- mod modifier_type::const Mod1Mask -->
the fourth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier, but normally it is the Alt key).
<!-- mod modifier_type::const Mod2Mask -->
the fifth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
<!-- mod modifier_type::const Mod3Mask -->
the sixth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
<!-- mod modifier_type::const Mod4Mask -->
the seventh modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
<!-- mod modifier_type::const Mod5Mask -->
the eighth modifier key (it depends on the modifier mapping of the X server which key is interpreted as this modifier).
<!-- mod modifier_type::const Button1Mask -->
the first mouse button.
<!-- mod modifier_type::const Button2Mask -->
the second mouse button.
<!-- mod modifier_type::const Button3Mask -->
the third mouse button.
<!-- mod modifier_type::const Button4Mask -->
the fourth mouse button.
<!-- mod modifier_type::const Button5Mask -->
the fifth mouse button.
<!-- mod modifier_type::const SuperMask -->
the Super modifier.
<!-- mod modifier_type::const HyperMask -->
the Hyper modifier.
<!-- mod modifier_type::const MetaMask -->
the Meta modifier.
<!-- mod modifier_type::const ReleaseMask -->
not used in GDK itself. GTK+ uses it to differentiate between (keyval, modifiers) pairs from key press and release events.
<!-- mod modifier_type::const ModifierMask -->
a mask covering all modifier types.
<!-- file cairo_interaction.rs -->
<!-- file_comment -->
Functions to support using cairo
<!-- trait ContextExt::fn create_from_window -->
Creates a Cairo context for drawing to `window`.

Note that calling `reset_clip()` on the resulting `Context` will
produce undefined results, so avoid it at all costs.
<!-- trait ContextExt::fn get_clip_rectangle -->
This is a convenience function around `clip_extents()`. It rounds
the clip extents to integer coordinates and returns a `RectangleInt`,
or `None` if no clip area exists.
<!-- trait ContextExt::fn set_source_rgba -->
Sets the specified `GdkRGBA` as the source color of `cr`.
<!-- trait ContextExt::fn set_source_pixbuf -->
Sets the given pixbuf as the source pattern for `cr`. The pattern has
an extend mode of `ExtendNone` and is aligned so that the origin of
`pixbuf` is (`x`, `y`).
<!-- trait ContextExt::fn set_source_window -->
Sets the given window as the source pattern for `cr`. The pattern has
an extend mode of `ExtendNone` and is aligned so that the origin of
window is (`x` , `y`). The window contains all its subwindows when
rendering.

Note that the contents of `window` are undefined outside of the visible
part of `window`, so use this function with care.
<!-- trait ContextExt::fn rectangle -->
Adds the given rectangle to the current path of `cr`.
<!-- file atom.rs -->
<!-- impl Atom::fn intern -->
Finds or creates an atom corresponding to a given string.
<!-- impl Atom::fn name -->
Determines the string corresponding to an atom.
