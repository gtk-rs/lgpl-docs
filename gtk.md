<!-- file lib.rs -->
<!-- file_comment -->
Bindings and wrappers for __GTK__

To implement __GTK+__ inheritance in rust, we implemented gtk superclasses as traits
located in `rgtk::self::traits::*`. The various widgets implement these traits and
live in `rgtk::gtk::widgets::*` and are rexported into `rgtk::gtk::*`.

GTK Inheritance in rgtk
======================

You probably know but __Gtk+__ uses its own GObject system: inherited class and interface.

To respect this design I follow a special design on __rgtk__:

* Interface -> Implement them on a trait with only default methods.
* Class -> Implement the construct on the class impl and other methods on a traits.
* Sub-class -> Implement all the methods on the class.

Exemple for GtkOrientable, GtkBox, GtkButtonBox:

GtkOrientable is an interface with all the methods implemented as default method of the trait self::traits::Orientable.

GtkBox is a class with constructors implemented on the struct `gtk::Box`, and the other method as default methods of the trait `self::traits::Box`. So `gtk::Box` implements `self::traits::Orientable` and `self::traits::Box`.

GtkButtonBox is a sub-class of GtkBox, the struct `gtk::ButtonBox` implements all the methods of GtkButtonBox and the traits `self::traits::Orientable` and `self::traits::Box`.

Finally all the gtk widgets implement the trait self::traits::Widget.
<!-- file lock_button.rs -->
<!-- file_comment -->
A container for arranging buttons
<!-- file auto/separator.rs -->
<!-- file_comment -->
A separator widget
<!-- file auto/box_.rs -->
<!-- file_comment -->
A container box
<!-- file auto/stack_sidebar.rs -->
<!-- file_comment -->
A controller for StackSidebar
<!-- file auto/font_button.rs -->
<!-- file_comment -->
A button to launch a font chooser dialog
<!-- file auto/range.rs -->
<!-- use ffi; -->
A Range widget
<!-- file auto/menu_tool_button.rs -->
<!-- file_comment -->
A ToolItem containing a button with an additional dropdown menu
<!-- file auto/font_chooser_widget.rs -->
<!-- file_comment -->
GtkFontChooserWidget — A widget for selecting fonts
<!-- file auto/link_button.rs -->
<!-- file_comment -->
Create buttons bound to a URL
<!-- file auto/menu_item.rs -->
<!-- file_comment -->
The widget used for item in menus
<!-- file auto/cell_renderer_toggle.rs -->
<!-- file_comment -->
Renders a toggle button in a cell
<!-- file auto/tree_view_column.rs -->
<!-- file_comment -->
A widget that emits a signal when clicked on
<!-- file auto/info_bar.rs -->
<!-- file_comment -->
Report important messages to the user
<!-- file auto/toggle_tool_button.rs -->
<!-- file_comment -->
A ToolItem containing a toggle button
<!-- file auto/alignment.rs -->
<!-- file_comment -->
A widget which controls the alignment and size of its child
<!-- file auto/combo_box.rs -->
<!-- file_comment -->
GtkComboBox — A widget used to choose from a list of items
<!-- file auto/combo_box_text.rs -->
<!-- file_comment -->
GtkComboBox — A widget used to choose from a list of items
<!-- file auto/tool_palette.rs -->
<!-- file_comment -->
GtkToolPalette — A tool palette with categories
<!-- file auto/spin_button.rs -->
<!-- file_comment -->
Retrieve an integer or floating-point number from the user
<!-- file auto/entry_buffer.rs -->
<!-- file_comment -->
Text buffer for ::Entry
<!-- file auto/volume_button.rs -->
<!-- file_comment -->
A button which pops up a volume control
<!-- file auto/button_box.rs -->
<!-- file_comment -->
A container for arranging buttons
<!-- file auto/button.rs -->
<!-- file_comment -->
A widget that emits a signal when clicked on
<!-- file auto/label.rs -->
<!-- file_comment -->
A widget that displays a small to medium amount of text
<!-- file auto/scale.rs -->
<!-- file_comment -->
A slider widget for selecting a value from a range
<!-- file auto/text_tag.rs -->
<!-- file_comment -->
GtkTextTag — A tag that can be applied to text in a GtkTextBuffer
<!-- file auto/toggle_button.rs -->
<!-- file_comment -->
A button to launch a font chooser dialog
<!-- file auto/grid.rs -->
<!-- file_comment -->
Pack widgets in a rows and columns
<!-- file auto/separator_tool_item.rs -->
<!-- file_comment -->
The base class of widgets that can be added to ToolShe
<!-- file auto/overlay.rs -->
<!-- file_comment -->
A container which overlays widgets on top of each other
<!-- file auto/icon_view.rs -->
<!-- file auto/file_chooser_widget.rs -->
<!-- file_comment -->
GtkFileChooserWidget — A file chooser widget
<!-- file auto/gl_area.rs -->
<!-- file_comment -->
A widget for custom drawing with OpenGL
<!-- impl GLArea::fn new -->
Creates a new GLArea widget.
<!-- impl GLArea::fn get_context -->
Retrieves the gdk::GLContext used by area.
<!-- impl GLArea::fn make_current -->
Ensures that the gdk::GLContext used by area is associated with the gtk::GLArea.

This function is automatically called before emitting the "render" signal, and doesn't
normally need to be called by application code.
<!-- impl GLArea::fn queue_render -->
Marks the currently rendered data (if any) as invalid, and queues a redraw of the widget,
ensuring that the "render" signal is emitted during the draw.

This is only needed when the GLArea::set_auto_render() has been called with a false value.
The default behaviour is to emit "render" on each draw.
<!-- impl GLArea::fn attach_buffers -->
Ensures that the area framebuffer object is made the current draw and read target, and that
all the required buffers for the area are created and bound to the frambuffer.

This function is automatically called before emitting the "render" signal, and doesn't
normally need to be called by application code.
<!-- impl GLArea::fn get_error -->
Gets the current error set on the area.
<!-- impl GLArea::fn set_has_alpha -->
If has_alpha is true the buffer allocated by the widget will have an alpha channel component,
and when rendering to the window the result will be composited over whatever is below the widget.

If has_alpha is false there will be no alpha channel, and the buffer will fully replace
anything below the widget.
<!-- impl GLArea::fn get_has_alpha -->
Returns whether the area has an alpha component.
<!-- impl GLArea::fn set_has_depth_buffer -->
If has_depth_buffer is true the widget will allocate and enable a depth buffer for the target
framebuffer. Otherwise there will be none.
<!-- impl GLArea::fn get_has_depth_buffer -->
Returns whether the area has a depth buffer.
<!-- impl GLArea::fn set_has_stencil_buffer -->
If has_stencil_buffer is true the widget will allocate and enable a stencil buffer for the
target framebuffer. Otherwise there will be none.
<!-- impl GLArea::fn get_has_stencil_buffer -->
Returns whether the area has a stencil buffer.
<!-- impl GLArea::fn set_auto_render -->
If auto_render is true the “render” signal will be emitted every time the widget draws. This
is the default and is useful if drawing the widget is faster.

If auto_render is false the data from previous rendering is kept around and will be used for
drawing the widget the next time, unless the window is resized. In order to force a rendering
GLArea::queue_render() must be called. This mode is useful when the scene changes seldomly,
but takes a long time to redraw.
<!-- impl GLArea::fn get_auto_render -->
Returns whether the area is in auto render mode or not.
<!-- impl GLArea::fn get_required_version -->
Retrieves the required version of OpenGL set using GLArea::set_required_version().
<!-- impl GLArea::fn set_required_version -->
Sets the required version of OpenGL to be used when creating the context for the widget.

This function must be called before the area has been realized.
<!-- file auto/event_box.rs -->
<!-- file_comment -->
GtkEventBox — A widget used to catch events for widgets which do not have their own window
<!-- file auto/app_chooser_widget.rs -->
<!-- file_comment -->
Application chooser widget that can be embedded in other widgets
<!-- file auto/level_bar.rs -->
<!-- file_comment -->
A bar that can used as a level indicator
<!-- file auto/text_mark.rs -->
<!-- file_comment -->
GtkTextMark — A position in the buffer preserved across buffer modifications
<!-- file auto/note_book.rs -->
<!-- file_comment -->
A tabbed notebook container
<!-- file auto/viewport.rs -->
<!-- file_comment -->
An adapter which makes widgets scrollable
<!-- file auto/recent_chooser_widget.rs -->
<!-- file_comment -->
GtkRecentChooserWidget — Displays recently used files
<!-- file auto/popover.rs -->
<!-- file_comment -->
GtkPopover — Context dependent bubbles
<!-- file auto/calendar.rs -->
<!-- file_comment -->
Displays a calendar and allows the user to select a date
<!-- file auto/tree_iter.rs -->
<!-- use ffi;::struct TreeIter -->
Text buffer iterator
<!-- file auto/text_buffer.rs -->
<!-- file auto/tool_item.rs -->
<!-- file_comment -->
The base class of widgets that can be added to ToolShe
<!-- file auto/gtype.rs -->
<!-- file_comment -->
A numerical value which represents the unique identifier of a registered type.
<!-- file auto/revealer.rs -->
<!-- file_comment -->
Hide and show with animation
<!-- file auto/flow_box.rs -->
<!-- file_comment -->
A container that allows reflowing its children
<!-- file auto/check_button.rs -->
<!-- file_comment -->
Create widgets with a discrete toggle button
<!-- file auto/separator_menu_item.rs -->
<!-- file_comment -->
The widget used for item in menus
<!-- file auto/action_bar.rs -->
<!-- file_comment -->
Hide and show with animation
<!-- file auto/widget.rs -->
<!-- file_comment -->
A widget
<!-- file auto/status_bar.rs -->
<!-- file_comment -->
An adapter which makes widgets scrollable
<!-- file auto/color_button.rs -->
<!-- file_comment -->
A button to launch a color selection dialog
<!-- file auto/menu_button.rs -->
<!-- file_comment -->
A widget that shows a menu when clicked on
<!-- file auto/image.rs -->
<!-- file_comment -->
A widget displaying an image
<!-- file auto/search_entry.rs -->
<!-- file_comment -->
An entry which shows a search icon
<!-- file auto/fixed.rs -->
<!-- file_comment -->
A container which allows you to position widgets at fixed coordinates
<!-- file auto/tool_bar.rs -->
<!-- file_comment -->
Create bars of buttons and other widgets
<!-- file auto/popover_menu.rs -->
<!-- file_comment -->
GtkPopoverMenu — Popovers to use as menus
<!-- file auto/adjustment.rs -->
<!-- file_comment -->
Adjustment — A representation of an adjustable bounded value
<!-- file auto/places_sidebar.rs -->
<!-- file_comment -->
GtkPlacesSidebar — Sidebar that displays frequently-used places in the file system
<!-- file auto/arrow.rs -->
<!-- file_comment -->
Displays an arrow
<!-- file auto/aspect_frame.rs -->
<!-- file_comment -->
A frame that constrains its child to a particular aspect ratio
<!-- file auto/stack.rs -->
<!-- file_comment -->
A stacking container
<!-- file auto/entry_completion.rs -->
<!-- file_comment -->
GtkEntryCompletion — Completion functionality for GtkEntry
<!-- file auto/scrollbar.rs -->
<!-- file_comment -->
A Scrollbar
<!-- file auto/text_iter.rs -->
<!-- file auto/header_bar.rs -->
<!-- file_comment -->
A Box::new(with) a centered child
<!-- file auto/expander.rs -->
<!-- file_comment -->
A container which can hide its child
<!-- file auto/text_view.rs -->
<!-- file_comment -->
GtkTextView — Widget that displays a GtkTextBuffer
<!-- file auto/color_chooser_widget.rs -->
<!-- file_comment -->
GtkColorChooserWidget — A widget for choosing colors
<!-- file auto/drawing_area.rs -->
<!-- file auto/tool_button.rs -->
<!-- file_comment -->
A ToolItem subclass that displays buttons
<!-- file auto/scale_button.rs -->
<!-- file_comment -->
A button which pops up a scale
<!-- file auto/progress_bar.rs -->
<!-- file_comment -->
A widget which indicates progress visually
<!-- file auto/window.rs -->
<!-- file_comment -->
Toplevel which can contain other widgets
<!-- file auto/list_box.rs -->
<!-- file_comment -->
A container that allows reflowing its children
<!-- file auto/scrolled_window.rs -->
<!-- file auto/tree_selection.rs -->
<!-- file_comment -->
GtkTreeSelection — The selection object for GtkTreeView
<!-- file auto/paned.rs -->
<!-- file_comment -->
A widget with two adjustable panes
<!-- file auto/switch.rs -->
<!-- file_comment -->
A "light switch" style toggle
<!-- file auto/text_tag_table.rs -->
<!-- file auto/entry.rs -->
<!-- file_comment -->
A single line text entry field
<!-- file auto/size_group.rs -->
<!-- file_comment -->
GtkSizeGroup — Grouping widgets so they request the same size
<!-- file auto/cell_renderer_text.rs -->
<!-- file_comment -->
Renders text in a cell
<!-- file auto/text_attributes.rs -->
<!-- file_comment -->
GtkTextTag — A tag that can be applied to text in a GtkTextBuffer
<!-- file auto/layout.rs -->
<!-- file_comment -->
Infinite scrollable area containing child widgets and/or custom drawing
<!-- file auto/socket.rs -->
<!-- file_comment -->
GtkSocket — Container for widgets from other processes
<!-- file auto/tree_view.rs -->
<!-- file_comment -->
A widget for displaying both trees and lists
<!-- file auto/tool_item_group.rs -->
<!-- file_comment -->
GtkToolItemGroup — A sub container used in a tool palette
<!-- file auto/stack_switcher.rs -->
<!-- file_comment -->
A controller for GtkStack
<!-- file auto/check_menu_item.rs -->
<!-- file_comment -->
The widget used for item in menus
<!-- file auto/spinner.rs -->
<!-- file_comment -->
Show a spinner animation
<!-- file auto/search_bar.rs -->
<!-- file_comment -->
A container box
<!-- file auto/radio_button.rs -->
<!-- file_comment -->
A choice from multiple check buttons
<!-- file auto/frame.rs -->
<!-- file_comment -->
A bin with a decorative frame and optional label
<!-- file auto/menu_item.rs -->
<!-- file_comment -->
The widget used for item in menus
<!-- file auto/cell_editable.rs -->
<!-- file_comment -->
Interface for widgets which can are used for editing cells
<!-- file auto/scrollable.rs -->
<!-- file_comment -->
An interface for scrollable widgets
<!-- file auto/style_provider.rs -->
<!-- file auto/actionable.rs -->
<!-- file_comment -->
GtkActionable — An interface for widgets that can be associated with actions
<!-- file auto/dialog.rs -->
<!-- file auto/menu_shell.rs -->
<!-- file_comment -->
A base class for menu objects
<!-- file auto/check_menu_item.rs -->
<!-- file_comment -->
The widget used for item in menus
