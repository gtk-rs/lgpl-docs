<!-- file * -->
<!-- struct Action -->
`Action` should be implemented by instances of `Object` classes
with which the user can interact directly, i.e. buttons,
checkboxes, scrollbars, e.g. components which are not "passive"
providers of UI information.

Exceptions: when the user interaction is already covered by another
appropriate interface such as `EditableText` (insert/delete text,
etc.) or `Value` (set value) then these actions should not be
exposed by `Action` as well.

Though most UI interactions on components should be invocable via
keyboard as well as mouse, there will generally be a close mapping
between "mouse actions" that are possible on a component and the
AtkActions. Where mouse and keyboard actions are redundant in
effect, `Action` should expose only one action rather than
exposing redundant actions if possible. By convention we have been
using "mouse centric" terminology for `Action` names.

# Implements

[`AtkActionExt`](trait.AtkActionExt.html)
<!-- trait AtkActionExt -->
Trait containing all `Action` methods.

# Implementors

[`Action`](struct.Action.html), [`Hyperlink`](struct.Hyperlink.html), [`NoOpObject`](struct.NoOpObject.html)
<!-- trait AtkActionExt::fn do_action -->
Perform the specified action on the object.
## `i`
the action index corresponding to the action to be performed

# Returns

`true` if success, `false` otherwise
<!-- trait AtkActionExt::fn get_description -->
Returns a description of the specified action of the object.
## `i`
the action index corresponding to the action to be performed

# Returns

a description string, or `None` if `self` does
not implement this interface.
<!-- trait AtkActionExt::fn get_keybinding -->
Gets the keybinding which can be used to activate this action, if one
exists. The string returned should contain localized, human-readable,
key sequences as they would appear when displayed on screen. It must
be in the format "mnemonic;sequence;shortcut".

- The mnemonic key activates the object if it is presently enabled onscreen.
 This typically corresponds to the underlined letter within the widget.
 Example: "n" in a traditional "New..." menu item or the "a" in "Apply" for
 a button.
- The sequence is the full list of keys which invoke the action even if the
 relevant element is not currently shown on screen. For instance, for a menu
 item the sequence is the keybindings used to open the parent menus before
 invoking. The sequence string is colon-delimited. Example: "Alt+F:N" in a
 traditional "New..." menu item.
- The shortcut, if it exists, will invoke the same action without showing
 the component or its enclosing menus or dialogs. Example: "Ctrl+N" in a
 traditional "New..." menu item.

Example: For a traditional "New..." menu item, the expected return value
would be: "N;Alt+F:N;Ctrl+N" for the English locale and "N;Alt+D:N;Strg+N"
for the German locale. If, hypothetically, this menu item lacked a mnemonic,
it would be represented by ";;Ctrl+N" and ";;Strg+N" respectively.
## `i`
the action index corresponding to the action to be performed

# Returns

the keybinding which can be used to activate
this action, or `None` if there is no keybinding for this action.
<!-- trait AtkActionExt::fn get_localized_name -->
Returns the localized name of the specified action of the object.
## `i`
the action index corresponding to the action to be performed

# Returns

a name string, or `None` if `self` does not
implement this interface.
<!-- trait AtkActionExt::fn get_n_actions -->
Gets the number of accessible actions available on the object.
If there are more than one, the first one is considered the
"default" action of the object.

# Returns

a the number of actions, or 0 if `self` does not
implement this interface.
<!-- trait AtkActionExt::fn get_name -->
Returns a non-localized string naming the specified action of the
object. This name is generally not descriptive of the end result
of the action, but instead names the 'interaction type' which the
object supports. By convention, the above strings should be used to
represent the actions which correspond to the common point-and-click
interaction techniques of the same name: i.e.
"click", "press", "release", "drag", "drop", "popup", etc.
The "popup" action should be used to pop up a context menu for the
object, if one exists.

For technical reasons, some toolkits cannot guarantee that the
reported action is actually 'bound' to a nontrivial user event;
i.e. the result of some actions via `Action::do_action` may be
NIL.
## `i`
the action index corresponding to the action to be performed

# Returns

a name string, or `None` if `self` does not
implement this interface.
<!-- trait AtkActionExt::fn set_description -->
Sets a description of the specified action of the object.
## `i`
the action index corresponding to the action to be performed
## `desc`
the description to be assigned to this action

# Returns

a gboolean representing if the description was successfully set;
<!-- struct Attribute -->
AtkAttribute is a string name/value pair representing a generic
attribute. This can be used to expose additional information from
an accessible object as a whole (see `AtkObjectExt::get_attributes`)
or an document (see `Document::get_attributes`). In the case of
text attributes (see `Text::get_default_attributes`),
`TextAttribute` enum defines all the possible text attribute
names. You can use `TextAttribute::get_name` to get the string
name from the enum value. See also `TextAttribute::for_name`
and `TextAttribute::get_value` for more information.

A string name/value pair representing a generic attribute.
<!-- impl Attribute::fn set_free -->
Frees the memory used by an `AttributeSet`, including all its
`AtkAttributes`.
## `attrib_set`
The `AttributeSet` to free
<!-- struct Component -->
`Component` should be implemented by most if not all UI elements
with an actual on-screen presence, i.e. components which can be
said to have a screen-coordinate bounding box. Virtually all
widgets will need to have `Component` implementations provided
for their corresponding `Object` class. In short, only UI
elements which are *not* GUI elements will omit this ATK interface.

A possible exception might be textual information with a
transparent background, in which case text glyph bounding box
information is provided by `Text`.

# Implements

[`ComponentExt`](trait.ComponentExt.html)
<!-- trait ComponentExt -->
Trait containing all `Component` methods.

# Implementors

[`Component`](struct.Component.html), [`NoOpObject`](struct.NoOpObject.html), [`Plug`](struct.Plug.html), [`Socket`](struct.Socket.html)
<!-- trait ComponentExt::fn add_focus_handler -->
Add the specified handler to the set of functions to be called
when this object receives focus events (in or out). If the handler is
already added it is not added again

# Deprecated since 2.9.4

If you need to track when an object gains or
lose the focus, use the `Object::state-change` "focused" notification instead.
## `handler`
The `AtkFocusHandler` to be attached to `self`

# Returns

a handler id which can be used in `Component::remove_focus_handler`
or zero if the handler was already added.
<!-- trait ComponentExt::fn contains -->
Checks whether the specified point is within the extent of the `self`.

Toolkit implementor note: ATK provides a default implementation for
this virtual method. In general there are little reason to
re-implement it.
## `x`
x coordinate
## `y`
y coordinate
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the components top level window

# Returns

`true` or `false` indicating whether the specified point is within
the extent of the `self` or not
<!-- trait ComponentExt::fn get_alpha -->
Returns the alpha value (i.e. the opacity) for this
`self`, on a scale from 0 (fully transparent) to 1.0
(fully opaque).

# Returns

An alpha value from 0 to 1.0, inclusive.
<!-- trait ComponentExt::fn get_extents -->
Gets the rectangle which gives the extent of the `self`.
## `x`
address of `gint` to put x coordinate
## `y`
address of `gint` to put y coordinate
## `width`
address of `gint` to put width
## `height`
address of `gint` to put height
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the components top level window
<!-- trait ComponentExt::fn get_layer -->
Gets the layer of the component.

# Returns

an `Layer` which is the layer of the component
<!-- trait ComponentExt::fn get_mdi_zorder -->
Gets the zorder of the component. The value G_MININT will be returned
if the layer of the component is not ATK_LAYER_MDI or ATK_LAYER_WINDOW.

# Returns

a gint which is the zorder of the component, i.e. the depth at
which the component is shown in relation to other components in the same
container.
<!-- trait ComponentExt::fn get_position -->
Gets the position of `self` in the form of
a point specifying `self`'s top-left corner.

# Deprecated

Since 2.12. Use `Component::get_extents` instead.
## `x`
address of `gint` to put x coordinate position
## `y`
address of `gint` to put y coordinate position
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the components top level window
<!-- trait ComponentExt::fn get_size -->
Gets the size of the `self` in terms of width and height.

# Deprecated

Since 2.12. Use `Component::get_extents` instead.
## `width`
address of `gint` to put width of `self`
## `height`
address of `gint` to put height of `self`
<!-- trait ComponentExt::fn grab_focus -->
Grabs focus for this `self`.

# Returns

`true` if successful, `false` otherwise.
<!-- trait ComponentExt::fn ref_accessible_at_point -->
Gets a reference to the accessible child, if one exists, at the
coordinate point specified by `x` and `y`.
## `x`
x coordinate
## `y`
y coordinate
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the components top level window

# Returns

a reference to the accessible
child, if one exists
<!-- trait ComponentExt::fn remove_focus_handler -->
Remove the handler specified by `handler_id` from the list of
functions to be executed when this object receives focus events
(in or out).

# Deprecated since 2.9.4

If you need to track when an object gains or
lose the focus, use the `Object::state-change` "focused" notification instead.
## `handler_id`
the handler id of the focus handler to be removed
from `self`
<!-- trait ComponentExt::fn scroll_to -->
Makes `self` visible on the screen by scrolling all necessary parents.

Contrary to atk_component_set_position, this does not actually move
`self` in its parent, this only makes the parents scroll so that the
object shows up on the screen, given its current position within the parents.

Feature: `v2_30`

## `type_`
specify where the object should be made visible.

# Returns

whether scrolling was successful.
<!-- trait ComponentExt::fn scroll_to_point -->
Makes an object visible on the screen at a given position by scrolling all
necessary parents.

Feature: `v2_30`

## `coords`
specify whether coordinates are relative to the screen or to the
parent object.
## `x`
x-position where to scroll to
## `y`
y-position where to scroll to

# Returns

whether scrolling was successful.
<!-- trait ComponentExt::fn set_extents -->
Sets the extents of `self`.
## `x`
x coordinate
## `y`
y coordinate
## `width`
width to set for `self`
## `height`
height to set for `self`
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the components top level window

# Returns

`true` or `false` whether the extents were set or not
<!-- trait ComponentExt::fn set_position -->
Sets the position of `self`.

Contrary to atk_component_scroll_to, this does not trigger any scrolling,
this just moves `self` in its parent.
## `x`
x coordinate
## `y`
y coordinate
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the component's top level window

# Returns

`true` or `false` whether or not the position was set or not
<!-- trait ComponentExt::fn set_size -->
Set the size of the `self` in terms of width and height.
## `width`
width to set for `self`
## `height`
height to set for `self`

# Returns

`true` or `false` whether the size was set or not
<!-- trait ComponentExt::fn connect_bounds_changed -->
The 'bounds-changed" signal is emitted when the bposition or
size of the component changes.
## `arg1`
The AtkRectangle giving the new position and size.
<!-- enum CoordType -->
Specifies how xy coordinates are to be interpreted. Used by functions such
as `Component::get_position` and `Text::get_character_extents`
<!-- enum CoordType::variant Screen -->
specifies xy coordinates relative to the screen
<!-- enum CoordType::variant Window -->
specifies xy coordinates relative to the widget's
top-level window
<!-- enum CoordType::variant Parent -->
specifies xy coordinates relative to the widget's
immediate parent. Since: 2.30
<!-- struct Document -->
The AtkDocument interface should be supported by any object whose
content is a representation or view of a document. The AtkDocument
interface should appear on the toplevel container for the document
content; however AtkDocument instances may be nested (i.e. an
AtkDocument may be a descendant of another AtkDocument) in those
cases where one document contains "embedded content" which can
reasonably be considered a document in its own right.

# Implements

[`DocumentExt`](trait.DocumentExt.html)
<!-- trait DocumentExt -->
Trait containing all `Document` methods.

# Implementors

[`Document`](struct.Document.html), [`NoOpObject`](struct.NoOpObject.html)
<!-- trait DocumentExt::fn get_attribute_value -->
## `attribute_name`
a character string representing the name of the attribute
 whose value is being queried.

# Returns

a string value associated with the named
 attribute for this document, or NULL if a value for
 `attribute_name` has not been specified for this document.
<!-- trait DocumentExt::fn get_attributes -->
Gets an AtkAttributeSet which describes document-wide
 attributes as name-value pairs.

# Returns

An AtkAttributeSet containing the explicitly
 set name-value-pair attributes associated with this document
 as a whole.
<!-- trait DocumentExt::fn get_current_page_number -->

# Returns

current page number inside `self`. -1 if not
implemented, not know by the implementor or irrelevant.
<!-- trait DocumentExt::fn get_document -->
Gets a `gpointer` that points to an instance of the DOM. It is
up to the caller to check atk_document_get_type to determine
how to cast this pointer.

# Deprecated

Since 2.12. `self` is already a representation of
the document. Use it directly, or one of its children, as an
instance of the DOM.

# Returns

a `gpointer` that points to an instance of the DOM.
<!-- trait DocumentExt::fn get_document_type -->
Gets a string indicating the document type.

# Deprecated

Since 2.12. Please use `Document::get_attributes` to
ask for the document type if it applies.

# Returns

a string indicating the document type
<!-- trait DocumentExt::fn get_locale -->
Gets a UTF-8 string indicating the POSIX-style LC_MESSAGES locale
 of the content of this document instance. Individual
 text substrings or images within this document may have
 a different locale, see atk_text_get_attributes and
 atk_image_get_image_locale.

# Deprecated since 2.7.90

Please use `AtkObjectExt::get_object_locale` instead.

# Returns

a UTF-8 string indicating the POSIX-style LC_MESSAGES
 locale of the document content as a whole, or NULL if
 the document content does not specify a locale.
<!-- trait DocumentExt::fn get_page_count -->

# Returns

total page count of `self`. -1 if not implemented, not
know by the implementor or irrelevant.
<!-- trait DocumentExt::fn set_attribute_value -->
## `attribute_name`
a character string representing the name of the attribute
 whose value is being set.
## `attribute_value`
a string value to be associated with `attribute_name`.

# Returns

TRUE if `value` is successfully associated with `attribute_name`
 for this document, FALSE otherwise (e.g. if the document does not
 allow the attribute to be modified).
<!-- trait DocumentExt::fn connect_load_complete -->
The 'load-complete' signal is emitted when a pending load of
a static document has completed. This signal is to be
expected by ATK clients if and when AtkDocument implementors
expose ATK_STATE_BUSY. If the state of an AtkObject which
implements AtkDocument does not include ATK_STATE_BUSY, it
should be safe for clients to assume that the AtkDocument's
static contents are fully loaded into the container.
(Dynamic document contents should be exposed via other
signals.)
<!-- trait DocumentExt::fn connect_load_stopped -->
The 'load-stopped' signal is emitted when a pending load of
document contents is cancelled, paused, or otherwise
interrupted by the user or application logic. It should not
however be emitted while waiting for a resource (for instance
while blocking on a file or network read) unless a
user-significant timeout has occurred.
<!-- trait DocumentExt::fn connect_page_changed -->
The 'page-changed' signal is emitted when the current page of
a document changes, e.g. pressing page up/down in a document
viewer.
## `page_number`
the new page number. If this value is unknown
or not applicable, -1 should be provided.
<!-- trait DocumentExt::fn connect_reload -->
The 'reload' signal is emitted when the contents of a
document is refreshed from its source. Once 'reload' has
been emitted, a matching 'load-complete' or 'load-stopped'
signal should follow, which clients may await before
interrogating ATK for the latest document content.
<!-- struct EditableText -->
`EditableText` should be implemented by UI components which
contain text which the user can edit, via the `Object`
corresponding to that component (see `Object`).

`EditableText` is a subclass of `Text`, and as such, an object
which implements `EditableText` is by definition an `Text`
implementor as well.

See also: `Text`

# Implements

[`EditableTextExt`](trait.EditableTextExt.html)
<!-- trait EditableTextExt -->
Trait containing all `EditableText` methods.

# Implementors

[`EditableText`](struct.EditableText.html), [`NoOpObject`](struct.NoOpObject.html)
<!-- trait EditableTextExt::fn copy_text -->
Copy text from `start_pos` up to, but not including `end_pos`
to the clipboard.
## `start_pos`
start position
## `end_pos`
end position
<!-- trait EditableTextExt::fn cut_text -->
Copy text from `start_pos` up to, but not including `end_pos`
to the clipboard and then delete from the widget.
## `start_pos`
start position
## `end_pos`
end position
<!-- trait EditableTextExt::fn delete_text -->
Delete text `start_pos` up to, but not including `end_pos`.
## `start_pos`
start position
## `end_pos`
end position
<!-- trait EditableTextExt::fn insert_text -->
Insert text at a given position.
## `string`
the text to insert
## `length`
the length of text to insert, in bytes
## `position`
The caller initializes this to
the position at which to insert the text. After the call it
points at the position after the newly inserted text.
<!-- trait EditableTextExt::fn paste_text -->
Paste text from clipboard to specified `position`.
## `position`
position to paste
<!-- trait EditableTextExt::fn set_run_attributes -->
Sets the attributes for a specified range. See the ATK_ATTRIBUTE
macros (such as `ATK_ATTRIBUTE_LEFT_MARGIN`) for examples of attributes
that can be set. Note that other attributes that do not have corresponding
ATK_ATTRIBUTE macros may also be set for certain text widgets.
## `attrib_set`
an `AttributeSet`
## `start_offset`
start of range in which to set attributes
## `end_offset`
end of range in which to set attributes

# Returns

`true` if attributes successfully set for the specified
range, otherwise `false`
<!-- trait EditableTextExt::fn set_text_contents -->
Set text contents of `self`.
## `string`
string to set for text contents of `self`
<!-- struct GObjectAccessible -->
This object class is derived from AtkObject. It can be used as a
basis for implementing accessible objects for GObjects which are
not derived from `GtkWidget`. One example of its use is in providing
an accessible object for GnomeCanvasItem in the GAIL library.

# Implements

[`GObjectAccessibleExt`](trait.GObjectAccessibleExt.html), [`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait GObjectAccessibleExt -->
Trait containing all `GObjectAccessible` methods.

# Implementors

[`GObjectAccessible`](struct.GObjectAccessible.html)
<!-- impl GObjectAccessible::fn for_object -->
Gets the accessible object for the specified `obj`.
## `obj`
a `gobject::Object`

# Returns

a `Object` which is the accessible object for
the `obj`
<!-- trait GObjectAccessibleExt::fn get_object -->
Gets the GObject for which `self` is the accessible object.

# Returns

a `gobject::Object` which is the object for which `self` is
the accessible object
<!-- struct Hyperlink -->
An ATK object which encapsulates a link or set of links (for
instance in the case of client-side image maps) in a hypertext
document. It may implement the AtkAction interface. AtkHyperlink
may also be used to refer to inline embedded content, since it
allows specification of a start and end offset within the host
AtkHypertext object.

# Implements

[`HyperlinkExt`](trait.HyperlinkExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`AtkActionExt`](trait.AtkActionExt.html)
<!-- trait HyperlinkExt -->
Trait containing all `Hyperlink` methods.

# Implementors

[`Hyperlink`](struct.Hyperlink.html)
<!-- trait HyperlinkExt::fn get_end_index -->
Gets the index with the hypertext document at which this link ends.

# Returns

the index with the hypertext document at which this link ends
<!-- trait HyperlinkExt::fn get_n_anchors -->
Gets the number of anchors associated with this hyperlink.

# Returns

the number of anchors associated with this hyperlink
<!-- trait HyperlinkExt::fn get_object -->
Returns the item associated with this hyperlinks nth anchor.
For instance, the returned `Object` will implement `Text`
if `self` is a text hyperlink, `Image` if `self` is an image
hyperlink etc.

Multiple anchors are primarily used by client-side image maps.
## `i`
a (zero-index) integer specifying the desired anchor

# Returns

an `Object` associated with this hyperlinks
i-th anchor
<!-- trait HyperlinkExt::fn get_start_index -->
Gets the index with the hypertext document at which this link begins.

# Returns

the index with the hypertext document at which this link begins
<!-- trait HyperlinkExt::fn get_uri -->
Get a the URI associated with the anchor specified
by `i` of `self`.

Multiple anchors are primarily used by client-side image maps.
## `i`
a (zero-index) integer specifying the desired anchor

# Returns

a string specifying the URI
<!-- trait HyperlinkExt::fn is_inline -->
Indicates whether the link currently displays some or all of its
 content inline. Ordinary HTML links will usually return
 `false`, but an inline &lt;src&gt; HTML element will return
 `true`.

# Returns

whether or not this link displays its content inline.
<!-- trait HyperlinkExt::fn is_selected_link -->
Determines whether this AtkHyperlink is selected

# Deprecated since 1.8

Please use ATK_STATE_FOCUSABLE for all links,
and ATK_STATE_FOCUSED for focused links.

# Returns

True if the AtkHyperlink is selected, False otherwise
<!-- trait HyperlinkExt::fn is_valid -->
Since the document that a link is associated with may have changed
this method returns `true` if the link is still valid (with
respect to the document it references) and `false` otherwise.

# Returns

whether or not this link is still valid
<!-- trait HyperlinkExt::fn connect_link_activated -->
The signal link-activated is emitted when a link is activated.
<!-- trait HyperlinkExt::fn get_property_selected-link -->
Selected link

# Deprecated since 1.8

Please use ATK_STATE_FOCUSABLE for all links, and
ATK_STATE_FOCUSED for focused links.
<!-- struct HyperlinkImpl -->
AtkHyperlinkImpl allows AtkObjects to refer to their associated
AtkHyperlink instance, if one exists. AtkHyperlinkImpl differs
from AtkHyperlink in that AtkHyperlinkImpl is an interface, whereas
AtkHyperlink is a object type. The AtkHyperlinkImpl interface
allows a client to query an AtkObject for the availability of an
associated AtkHyperlink instance, and obtain that instance. It is
thus particularly useful in cases where embedded content or inline
content within a text object is present, since the embedding text
object implements AtkHypertext and the inline/embedded objects are
exposed as children which implement AtkHyperlinkImpl, in addition
to their being obtainable via AtkHypertext:getLink followed by
AtkHyperlink:getObject.

The AtkHyperlinkImpl interface should be supported by objects
exposed within the hierarchy as children of an AtkHypertext
container which correspond to "links" or embedded content within
the text. HTML anchors are not, for instance, normally exposed
this way, but embedded images and components which appear inline in
the content of a text object are. The AtkHyperlinkIface interface
allows a means of determining which children are hyperlinks in this
sense of the word, and for obtaining their corresponding
AtkHyperlink object, from which the embedding range, URI, etc. can
be obtained.

To some extent this interface exists because, for historical
reasons, AtkHyperlink was defined as an object type, not an
interface. Thus, in order to interact with AtkObjects via
AtkHyperlink semantics, a new interface was required.

# Implements

[`HyperlinkImplExt`](trait.HyperlinkImplExt.html)
<!-- trait HyperlinkImplExt -->
Trait containing all `HyperlinkImpl` methods.

# Implementors

[`HyperlinkImpl`](struct.HyperlinkImpl.html)
<!-- trait HyperlinkImplExt::fn get_hyperlink -->
Gets the hyperlink associated with this object.

# Returns

an AtkHyperlink object which points to this
implementing AtkObject.
<!-- struct Hypertext -->
An interface used for objects which implement linking between
multiple resource or content locations, or multiple 'markers'
within a single document. A Hypertext instance is associated with
one or more Hyperlinks, which are associated with particular
offsets within the Hypertext's included content. While this
interface is derived from Text, there is no requirement that
Hypertext instances have textual content; they may implement Image
as well, and Hyperlinks need not have non-zero text offsets.

# Implements

[`HypertextExt`](trait.HypertextExt.html)
<!-- trait HypertextExt -->
Trait containing all `Hypertext` methods.

# Implementors

[`Hypertext`](struct.Hypertext.html), [`NoOpObject`](struct.NoOpObject.html)
<!-- trait HypertextExt::fn get_link -->
Gets the link in this hypertext document at index
`link_index`
## `link_index`
an integer specifying the desired link

# Returns

the link in this hypertext document at
index `link_index`
<!-- trait HypertextExt::fn get_link_index -->
Gets the index into the array of hyperlinks that is associated with
the character specified by `char_index`.
## `char_index`
a character index

# Returns

an index into the array of hyperlinks in `self`,
or -1 if there is no hyperlink associated with this character.
<!-- trait HypertextExt::fn get_n_links -->
Gets the number of links within this hypertext document.

# Returns

the number of links within this hypertext document
<!-- trait HypertextExt::fn connect_link_selected -->
The "link-selected" signal is emitted by an AtkHyperText
object when one of the hyperlinks associated with the object
is selected.
## `arg1`
the index of the hyperlink which is selected
<!-- struct Image -->
`Image` should be implemented by `Object` subtypes on behalf of
components which display image/pixmap information onscreen, and
which provide information (other than just widget borders, etc.)
via that image content. For instance, icons, buttons with icons,
toolbar elements, and image viewing panes typically should
implement `Image`.

`Image` primarily provides two types of information: coordinate
information (useful for screen review mode of screenreaders, and
for use by onscreen magnifiers), and descriptive information. The
descriptive information is provided for alternative, text-only
presentation of the most significant information present in the
image.

# Implements

[`AtkImageExt`](trait.AtkImageExt.html)
<!-- trait AtkImageExt -->
Trait containing all `Image` methods.

# Implementors

[`Image`](struct.Image.html), [`NoOpObject`](struct.NoOpObject.html)
<!-- trait AtkImageExt::fn get_image_description -->
Get a textual description of this image.

# Returns

a string representing the image description
<!-- trait AtkImageExt::fn get_image_locale -->

# Returns

a string corresponding to the POSIX
LC_MESSAGES locale used by the image description, or `None` if the
image does not specify a locale.
<!-- trait AtkImageExt::fn get_image_position -->
Gets the position of the image in the form of a point specifying the
images top-left corner.
## `x`
address of `gint` to put x coordinate position; otherwise, -1 if value cannot be obtained.
## `y`
address of `gint` to put y coordinate position; otherwise, -1 if value cannot be obtained.
## `coord_type`
specifies whether the coordinates are relative to the screen
or to the components top level window
<!-- trait AtkImageExt::fn get_image_size -->
Get the width and height in pixels for the specified image.
The values of `width` and `height` are returned as -1 if the
values cannot be obtained (for instance, if the object is not onscreen).
## `width`
filled with the image width, or -1 if the value cannot be obtained.
## `height`
filled with the image height, or -1 if the value cannot be obtained.
<!-- trait AtkImageExt::fn set_image_description -->
Sets the textual description for this image.
## `description`
a string description to set for `self`

# Returns

boolean TRUE, or FALSE if operation could
not be completed.
<!-- enum Layer -->
Describes the layer of a component

These enumerated "layer values" are used when determining which UI
rendering layer a component is drawn into, which can help in making
determinations of when components occlude one another.
<!-- enum Layer::variant Invalid -->
The object does not have a layer
<!-- enum Layer::variant Background -->
This layer is reserved for the desktop background
<!-- enum Layer::variant Canvas -->
This layer is used for Canvas components
<!-- enum Layer::variant Widget -->
This layer is normally used for components
<!-- enum Layer::variant Mdi -->
This layer is used for layered components
<!-- enum Layer::variant Popup -->
This layer is used for popup components, such as menus
<!-- enum Layer::variant Overlay -->
This layer is reserved for future use.
<!-- enum Layer::variant Window -->
This layer is used for toplevel windows.
<!-- struct Misc -->
A set of utility functions for thread locking. This interface and
all his related methods are deprecated since 2.12.

# Implements

[`AtkMiscExt`](trait.AtkMiscExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AtkMiscExt -->
Trait containing all `Misc` methods.

# Implementors

[`Misc`](struct.Misc.html)
<!-- impl Misc::fn get_instance -->
Obtain the singleton instance of AtkMisc for this application.

# Deprecated

Since 2.12.

# Returns

The singleton instance of AtkMisc for this application.
<!-- trait AtkMiscExt::fn threads_enter -->
Take the thread mutex for the GUI toolkit,
if one exists.
(This method is implemented by the toolkit ATK implementation layer;
 for instance, for GTK+, GAIL implements this via GDK_THREADS_ENTER).

# Deprecated

Since 2.12.
<!-- trait AtkMiscExt::fn threads_leave -->
Release the thread mutex for the GUI toolkit,
if one exists. This method, and atk_misc_threads_enter,
are needed in some situations by threaded application code which
services ATK requests, since fulfilling ATK requests often
requires calling into the GUI toolkit. If a long-running or
potentially blocking call takes place inside such a block, it should
be bracketed by atk_misc_threads_leave/atk_misc_threads_enter calls.
(This method is implemented by the toolkit ATK implementation layer;
 for instance, for GTK+, GAIL implements this via GDK_THREADS_LEAVE).

# Deprecated

Since 2.12.
<!-- struct NoOpObject -->
An AtkNoOpObject is an AtkObject which purports to implement all
ATK interfaces. It is the type of AtkObject which is created if an
accessible object is requested for an object type for which no
factory type is specified.

# Implements

[`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`AtkActionExt`](trait.AtkActionExt.html), [`ComponentExt`](trait.ComponentExt.html), [`DocumentExt`](trait.DocumentExt.html), [`EditableTextExt`](trait.EditableTextExt.html), [`HypertextExt`](trait.HypertextExt.html), [`AtkImageExt`](trait.AtkImageExt.html), [`SelectionExt`](trait.SelectionExt.html), [`TableExt`](trait.TableExt.html), [`TableCellExt`](trait.TableCellExt.html), [`TextExt`](trait.TextExt.html), [`ValueExt`](trait.ValueExt.html), [`AtkWindowExt`](trait.AtkWindowExt.html)
<!-- impl NoOpObject::fn new -->
Provides a default (non-functioning stub) `Object`.
Application maintainers should not use this method.
## `obj`
a `gobject::Object`

# Returns

a default (non-functioning stub) `Object`
<!-- struct NoOpObjectFactory -->
The AtkObjectFactory which creates an AtkNoOpObject. An instance of
this is created by an AtkRegistry if no factory type has not been
specified to create an accessible object of a particular type.

# Implements

[`ObjectFactoryExt`](trait.ObjectFactoryExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl NoOpObjectFactory::fn new -->
Creates an instance of an `ObjectFactory` which generates primitive
(non-functioning) `AtkObjects`.

# Returns

an instance of an `ObjectFactory`
<!-- struct Object -->
This class is the primary class for accessibility support via the
Accessibility ToolKit (ATK). Objects which are instances of
`Object` (or instances of AtkObject-derived types) are queried
for properties which relate basic (and generic) properties of a UI
component such as name and description. Instances of `Object`
may also be queried as to whether they implement other ATK
interfaces (e.g. `Action`, `Component`, etc.), as appropriate
to the role which a given UI component plays in a user interface.

All UI components in an application which provide useful
information or services to the user must provide corresponding
`Object` instances on request (in GTK+, for instance, usually on
a call to `gtk_widget_get_accessible` ()), either via ATK support
built into the toolkit for the widget class or ancestor class, or
in the case of custom widgets, if the inherited `Object`
implementation is insufficient, via instances of a new `Object`
subclass.

See also: `ObjectFactory`, `Registry`. (GTK+ users see also
``GtkAccessible``).

# Implements

[`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AtkObjectExt -->
Trait containing all `Object` methods.

# Implementors

[`GObjectAccessible`](struct.GObjectAccessible.html), [`NoOpObject`](struct.NoOpObject.html), [`Object`](struct.Object.html), [`Plug`](struct.Plug.html), [`Socket`](struct.Socket.html), [`TableCell`](struct.TableCell.html), [`Window`](struct.Window.html)
<!-- trait AtkObjectExt::fn add_relationship -->
Adds a relationship of the specified type with the specified target.
## `relationship`
The `RelationType` of the relation
## `target`
The `Object` which is to be the target of the relation.

# Returns

TRUE if the relationship is added.
<!-- trait AtkObjectExt::fn connect_property_change_handler -->

# Deprecated

Since 2.12. Connect directly to property-change or
notify signals.
## `handler`
a function to be called when a property changes its value

# Returns

a `guint` which is the handler id used in
`AtkObjectExt::remove_property_change_handler`
<!-- trait AtkObjectExt::fn get_attributes -->
Get a list of properties applied to this object as a whole, as an `AttributeSet` consisting of
name-value pairs. As such these attributes may be considered weakly-typed properties or annotations,
as distinct from strongly-typed object data available via other get/set methods.
Not all objects have explicit "name-value pair" `AttributeSet` properties.

# Returns

an `AttributeSet` consisting of all
explicit properties/annotations applied to the object, or an empty
set if the object has no name-value pair attributes assigned to
it. This `atkattributeset` should be freed by a call to
`Attribute::set_free`.
<!-- trait AtkObjectExt::fn get_description -->
Gets the accessible description of the accessible.

# Returns

a character string representing the accessible description
of the accessible.
<!-- trait AtkObjectExt::fn get_index_in_parent -->
Gets the 0-based index of this accessible in its parent; returns -1 if the
accessible does not have an accessible parent.

# Returns

an integer which is the index of the accessible in its parent
<!-- trait AtkObjectExt::fn get_layer -->
Gets the layer of the accessible.

# Deprecated

Use atk_component_get_layer instead.

# Returns

an `Layer` which is the layer of the accessible
<!-- trait AtkObjectExt::fn get_mdi_zorder -->
Gets the zorder of the accessible. The value G_MININT will be returned
if the layer of the accessible is not ATK_LAYER_MDI.

# Deprecated

Use atk_component_get_mdi_zorder instead.

# Returns

a gint which is the zorder of the accessible, i.e. the depth at
which the component is shown in relation to other components in the same
container.
<!-- trait AtkObjectExt::fn get_n_accessible_children -->
Gets the number of accessible children of the accessible.

# Returns

an integer representing the number of accessible children
of the accessible.
<!-- trait AtkObjectExt::fn get_name -->
Gets the accessible name of the accessible.

# Returns

a character string representing the accessible name of the object.
<!-- trait AtkObjectExt::fn get_object_locale -->
Gets a UTF-8 string indicating the POSIX-style LC_MESSAGES locale
of `self`.

# Returns

a UTF-8 string indicating the POSIX-style LC_MESSAGES
 locale of `self`.
<!-- trait AtkObjectExt::fn get_parent -->
Gets the accessible parent of the accessible. By default this is
the one assigned with `AtkObjectExt::set_parent`, but it is assumed
that ATK implementors have ways to get the parent of the object
without the need of assigning it manually with
`AtkObjectExt::set_parent`, and will return it with this method.

If you are only interested on the parent assigned with
`AtkObjectExt::set_parent`, use `AtkObjectExt::peek_parent`.

# Returns

an `Object` representing the accessible
parent of the accessible
<!-- trait AtkObjectExt::fn get_role -->
Gets the role of the accessible.

# Returns

an `Role` which is the role of the accessible
<!-- trait AtkObjectExt::fn initialize -->
This function is called when implementing subclasses of `Object`.
It does initialization required for the new object. It is intended
that this function should called only in the ...`_new` functions used
to create an instance of a subclass of `Object`
## `data`
a `gpointer` which identifies the object for which the AtkObject was created.
<!-- trait AtkObjectExt::fn notify_state_change -->
Emits a state-change signal for the specified state.

Note that as a general rule when the state of an existing object changes,
emitting a notification is expected.
## `state`
an `State` whose state is changed
## `value`
a gboolean which indicates whether the state is being set on or off
<!-- trait AtkObjectExt::fn peek_parent -->
Gets the accessible parent of the accessible, if it has been
manually assigned with atk_object_set_parent. Otherwise, this
function returns `None`.

This method is intended as an utility for ATK implementors, and not
to be exposed to accessible tools. See `AtkObjectExt::get_parent` for
further reference.

# Returns

an `Object` representing the accessible
parent of the accessible if assigned
<!-- trait AtkObjectExt::fn ref_accessible_child -->
Gets a reference to the specified accessible child of the object.
The accessible children are 0-based so the first accessible child is
at index 0, the second at index 1 and so on.
## `i`
a gint representing the position of the child, starting from 0

# Returns

an `Object` representing the specified
accessible child of the accessible.
<!-- trait AtkObjectExt::fn ref_relation_set -->
Gets the `RelationSet` associated with the object.

# Returns

an `RelationSet` representing the relation set
of the object.
<!-- trait AtkObjectExt::fn ref_state_set -->
Gets a reference to the state set of the accessible; the caller must
unreference it when it is no longer needed.

# Returns

a reference to an `StateSet` which is the state
set of the accessible
<!-- trait AtkObjectExt::fn remove_property_change_handler -->

# Deprecated

Since 2.12.

Removes a property change handler.
## `handler_id`
a guint which identifies the handler to be removed.
<!-- trait AtkObjectExt::fn remove_relationship -->
Removes a relationship of the specified type with the specified target.
## `relationship`
The `RelationType` of the relation
## `target`
The `Object` which is the target of the relation to be removed.

# Returns

TRUE if the relationship is removed.
<!-- trait AtkObjectExt::fn set_description -->
Sets the accessible description of the accessible. You can't set
the description to NULL. This is reserved for the initial value. In
this aspect NULL is similar to ATK_ROLE_UNKNOWN. If you want to set
the name to a empty value you can use "".
## `description`
a character string to be set as the accessible description
<!-- trait AtkObjectExt::fn set_name -->
Sets the accessible name of the accessible. You can't set the name
to NULL. This is reserved for the initial value. In this aspect
NULL is similar to ATK_ROLE_UNKNOWN. If you want to set the name to
a empty value you can use "".
## `name`
a character string to be set as the accessible name
<!-- trait AtkObjectExt::fn set_parent -->
Sets the accessible parent of the accessible. `parent` can be NULL.
## `parent`
an `Object` to be set as the accessible parent
<!-- trait AtkObjectExt::fn set_role -->
Sets the role of the accessible.
## `role`
an `Role` to be set as the role
<!-- trait AtkObjectExt::fn connect_active_descendant_changed -->
The "active-descendant-changed" signal is emitted by an object
which has the state ATK_STATE_MANAGES_DESCENDANTS when the focus
object in the object changes. For instance, a table will emit the
signal when the cell in the table which has focus changes.
## `arg1`
the newly focused object.
<!-- trait AtkObjectExt::fn connect_children_changed -->
The signal "children-changed" is emitted when a child is added or
removed form an object. It supports two details: "add" and
"remove"
## `arg1`
The index of the added or removed child. The value can be
-1. This is used if the value is not known by the implementor
when the child is added/removed or irrelevant.
## `arg2`
A gpointer to the child AtkObject which was added or
removed. If the child was removed, it is possible that it is not
available for the implementor. In that case this pointer can be
NULL.
<!-- trait AtkObjectExt::fn connect_focus_event -->
The signal "focus-event" is emitted when an object gained or lost
focus.

# Deprecated since 2.9.4

Use the `Object::state-change` signal instead.
## `arg1`
a boolean value which indicates whether the object gained
or lost focus.
<!-- trait AtkObjectExt::fn connect_property_change -->
The signal "property-change" is emitted when an object's property
value changes. `arg1` contains an `PropertyValues` with the name
and the new value of the property whose value has changed. Note
that, as with GObject notify, getting this signal does not
guarantee that the value of the property has actually changed; it
may also be emitted when the setter of the property is called to
reinstate the previous value.

Toolkit implementor note: ATK implementors should use
`gobject::ObjectExt::notify` to emit property-changed
notifications. `Object::property-changed` is needed by the
implementation of `atk_add_global_event_listener` because GObject
notify doesn't support emission hooks.
## `arg1`
an `PropertyValues` containing the new
value of the property which changed.
<!-- trait AtkObjectExt::fn connect_state_change -->
The "state-change" signal is emitted when an object's state
changes. The detail value identifies the state type which has
changed.
## `arg1`
The name of the state which has changed
## `arg2`
A boolean which indicates whether the state has been set or unset.
<!-- trait AtkObjectExt::fn connect_visible_data_changed -->
The "visible-data-changed" signal is emitted when the visual
appearance of the object changed.
<!-- trait AtkObjectExt::fn get_property_accessible-table-caption -->
Table caption.

# Deprecated

Since 1.3. Use table-caption-object instead.
<!-- trait AtkObjectExt::fn set_property_accessible-table-caption -->
Table caption.

# Deprecated

Since 1.3. Use table-caption-object instead.
<!-- trait AtkObjectExt::fn get_property_accessible-table-column-description -->
Accessible table column description.

# Deprecated

Since 2.12. Use `Table::get_column_description`
and `Table::set_column_description` instead.
<!-- trait AtkObjectExt::fn set_property_accessible-table-column-description -->
Accessible table column description.

# Deprecated

Since 2.12. Use `Table::get_column_description`
and `Table::set_column_description` instead.
<!-- trait AtkObjectExt::fn get_property_accessible-table-column-header -->
Accessible table column header.

# Deprecated

Since 2.12. Use `Table::get_column_header` and
`Table::set_column_header` instead.
<!-- trait AtkObjectExt::fn set_property_accessible-table-column-header -->
Accessible table column header.

# Deprecated

Since 2.12. Use `Table::get_column_header` and
`Table::set_column_header` instead.
<!-- trait AtkObjectExt::fn get_property_accessible-table-row-description -->
Accessible table row description.

# Deprecated

Since 2.12. Use `Table::get_row_description` and
`Table::set_row_description` instead.
<!-- trait AtkObjectExt::fn set_property_accessible-table-row-description -->
Accessible table row description.

# Deprecated

Since 2.12. Use `Table::get_row_description` and
`Table::set_row_description` instead.
<!-- trait AtkObjectExt::fn get_property_accessible-table-row-header -->
Accessible table row header.

# Deprecated

Since 2.12. Use `Table::get_row_header` and
`Table::set_row_header` instead.
<!-- trait AtkObjectExt::fn set_property_accessible-table-row-header -->
Accessible table row header.

# Deprecated

Since 2.12. Use `Table::get_row_header` and
`Table::set_row_header` instead.
<!-- trait AtkObjectExt::fn get_property_accessible-value -->
Numeric value of this object, in case being and AtkValue.

# Deprecated

Since 2.12. Use `Value::get_value_and_text` to get
the value, and value-changed signal to be notified on their value
changes.
<!-- trait AtkObjectExt::fn set_property_accessible-value -->
Numeric value of this object, in case being and AtkValue.

# Deprecated

Since 2.12. Use `Value::get_value_and_text` to get
the value, and value-changed signal to be notified on their value
changes.
<!-- struct ObjectFactory -->
This class is the base object class for a factory used to create an
accessible object for a specific GType. The function
`RegistryExt::set_factory_type` is normally called to store in the
registry the factory type to be used to create an accessible of a
particular GType.

# Implements

[`ObjectFactoryExt`](trait.ObjectFactoryExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait ObjectFactoryExt -->
Trait containing all `ObjectFactory` methods.

# Implementors

[`NoOpObjectFactory`](struct.NoOpObjectFactory.html), [`ObjectFactory`](struct.ObjectFactory.html)
<!-- trait ObjectFactoryExt::fn create_accessible -->
Provides an `Object` that implements an accessibility interface
on behalf of `obj`
## `obj`
a `gobject::Object`

# Returns

an `Object` that implements an accessibility
interface on behalf of `obj`
<!-- trait ObjectFactoryExt::fn get_accessible_type -->
Gets the GType of the accessible which is created by the factory.

# Returns

the type of the accessible which is created by the `self`.
The value G_TYPE_INVALID is returned if no type if found.
<!-- trait ObjectFactoryExt::fn invalidate -->
Inform `self` that it is no longer being used to create
accessibles. When called, `self` may need to inform
`AtkObjects` which it has created that they need to be re-instantiated.
Note: primarily used for runtime replacement of `AtkObjectFactorys`
in object registries.
<!-- struct Plug -->
See `Socket`

# Implements

[`AtkPlugExt`](trait.AtkPlugExt.html), [`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ComponentExt`](trait.ComponentExt.html)
<!-- trait AtkPlugExt -->
Trait containing all `Plug` methods.

# Implementors

[`Plug`](struct.Plug.html)
<!-- trait AtkPlugExt::fn get_id -->
Gets the unique ID of an `Plug` object, which can be used to
embed inside of an `Socket` using `AtkSocketExt::embed`.

Internally, this calls a class function that should be registered
by the IPC layer (usually at-spi2-atk). The implementor of an
`Plug` object should call this function (after atk-bridge is
loaded) and pass the value to the process implementing the
`Socket`, so it could embed the plug.

# Returns

the unique ID for the plug
<!-- struct Range -->
`Range` are used on `Value`, in order to represent the full
range of a given component (for example an slider or a range
control), or to define each individual subrange this full range is
splitted if available. See `Value` documentation for further
details.
<!-- impl Range::fn new -->
Creates a new `Range`.
## `lower_limit`
inferior limit for this range
## `upper_limit`
superior limit for this range
## `description`
human readable description of this range.

# Returns

a new `Range`
<!-- impl Range::fn copy -->
Returns a new `Range` that is a exact copy of `self`

# Returns

a new `Range` copy of `self`
<!-- impl Range::fn free -->
Free `self`
<!-- impl Range::fn get_description -->
Returns the human readable description of `self`

# Returns

the human-readable description of `self`
<!-- impl Range::fn get_lower_limit -->
Returns the lower limit of `self`

# Returns

the lower limit of `self`
<!-- impl Range::fn get_upper_limit -->
Returns the upper limit of `self`

# Returns

the upper limit of `self`
<!-- struct Rectangle -->
A data structure for holding a rectangle. Those coordinates are
relative to the component top-level parent.
<!-- struct Registry -->
The AtkRegistry is normally used to create appropriate ATK "peers"
for user interface components. Application developers usually need
only interact with the AtkRegistry by associating appropriate ATK
implementation classes with GObject classes via the
atk_registry_set_factory_type call, passing the appropriate GType
for application custom widget classes.

# Implements

[`RegistryExt`](trait.RegistryExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait RegistryExt -->
Trait containing all `Registry` methods.

# Implementors

[`Registry`](struct.Registry.html)
<!-- trait RegistryExt::fn get_factory -->
Gets an `ObjectFactory` appropriate for creating `AtkObjects`
appropriate for `type_`.
## `type_`
a `glib::Type` with which to look up the associated `ObjectFactory`

# Returns

an `ObjectFactory` appropriate for creating
`AtkObjects` appropriate for `type_`.
<!-- trait RegistryExt::fn get_factory_type -->
Provides a `glib::Type` indicating the `ObjectFactory` subclass
associated with `type_`.
## `type_`
a `glib::Type` with which to look up the associated `ObjectFactory`
subclass

# Returns

a `glib::Type` associated with type `type_`
<!-- trait RegistryExt::fn set_factory_type -->
Associate an `ObjectFactory` subclass with a `glib::Type`. Note:
The associated `factory_type` will thereafter be responsible for
the creation of new `Object` implementations for instances
appropriate for `type_`.
## `type_`
an `Object` type
## `factory_type`
an `ObjectFactory` type to associate with `type_`. Must
implement AtkObject appropriate for `type_`.
<!-- struct Relation -->
An AtkRelation describes a relation between an object and one or
more other objects. The actual relations that an object has with
other objects are defined as an AtkRelationSet, which is a set of
AtkRelations.

# Implements

[`RelationExt`](trait.RelationExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait RelationExt -->
Trait containing all `Relation` methods.

# Implementors

[`Relation`](struct.Relation.html)
<!-- impl Relation::fn new -->
Create a new relation for the specified key and the specified list
of targets. See also `AtkObjectExt::add_relationship`.
## `targets`
an array of pointers to
 `AtkObjects`
## `n_targets`
number of `AtkObjects` pointed to by `targets`
## `relationship`
an `RelationType` with which to create the new
 `Relation`

# Returns

a pointer to a new `Relation`
<!-- trait RelationExt::fn add_target -->
Adds the specified AtkObject to the target for the relation, if it is
not already present. See also `AtkObjectExt::add_relationship`.
## `target`
an `Object`
<!-- trait RelationExt::fn get_relation_type -->
Gets the type of `self`

# Returns

the type of `self`
<!-- trait RelationExt::fn get_target -->
Gets the target list of `self`

# Returns

the target list of `self`
<!-- trait RelationExt::fn remove_target -->
Remove the specified AtkObject from the target for the relation.
## `target`
an `Object`

# Returns

TRUE if the removal is successful.
<!-- struct RelationSet -->
The AtkRelationSet held by an object establishes its relationships
with objects beyond the normal "parent/child" hierarchical
relationships that all user interface objects have.
AtkRelationSets establish whether objects are labelled or
controlled by other components, share group membership with other
components (for instance within a radio-button group), or share
content which "flows" between them, among other types of possible
relationships.

# Implements

[`RelationSetExt`](trait.RelationSetExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait RelationSetExt -->
Trait containing all `RelationSet` methods.

# Implementors

[`RelationSet`](struct.RelationSet.html)
<!-- impl RelationSet::fn new -->
Creates a new empty relation set.

# Returns

a new `RelationSet`
<!-- trait RelationSetExt::fn add -->
Add a new relation to the current relation set if it is not already
present.
This function ref's the AtkRelation so the caller of this function
should unref it to ensure that it will be destroyed when the AtkRelationSet
is destroyed.
## `relation`
an `Relation`
<!-- trait RelationSetExt::fn add_relation_by_type -->
Add a new relation of the specified type with the specified target to
the current relation set if the relation set does not contain a relation
of that type. If it is does contain a relation of that typea the target
is added to the relation.
## `relationship`
an `RelationType`
## `target`
an `Object`
<!-- trait RelationSetExt::fn contains -->
Determines whether the relation set contains a relation that matches the
specified type.
## `relationship`
an `RelationType`

# Returns

`true` if `relationship` is the relationship type of a relation
in `self`, `false` otherwise
<!-- trait RelationSetExt::fn contains_target -->
Determines whether the relation set contains a relation that
matches the specified pair formed by type `relationship` and object
`target`.
## `relationship`
an `RelationType`
## `target`
an `Object`

# Returns

`true` if `self` contains a relation with the relationship
type `relationship` with an object `target`, `false` otherwise
<!-- trait RelationSetExt::fn get_n_relations -->
Determines the number of relations in a relation set.

# Returns

an integer representing the number of relations in the set.
<!-- trait RelationSetExt::fn get_relation -->
Determines the relation at the specified position in the relation set.
## `i`
a gint representing a position in the set, starting from 0.

# Returns

a `Relation`, which is the relation at
position i in the set.
<!-- trait RelationSetExt::fn get_relation_by_type -->
Finds a relation that matches the specified type.
## `relationship`
an `RelationType`

# Returns

an `Relation`, which is a relation matching the
specified type.
<!-- trait RelationSetExt::fn remove -->
Removes a relation from the relation set.
This function unref's the `Relation` so it will be deleted unless there
is another reference to it.
## `relation`
an `Relation`
<!-- enum RelationType -->
Describes the type of the relation
<!-- enum RelationType::variant Null -->
Not used, represens "no relationship" or an error condition.
<!-- enum RelationType::variant ControlledBy -->
Indicates an object controlled by one or more target objects.
<!-- enum RelationType::variant ControllerFor -->
Indicates an object is an controller for one or more target objects.
<!-- enum RelationType::variant LabelFor -->
Indicates an object is a label for one or more target objects.
<!-- enum RelationType::variant LabelledBy -->
Indicates an object is labelled by one or more target objects.
<!-- enum RelationType::variant MemberOf -->
Indicates an object is a member of a group of one or more target objects.
<!-- enum RelationType::variant NodeChildOf -->
Indicates an object is a cell in a treetable which is displayed because a cell in the same column is expanded and identifies that cell.
<!-- enum RelationType::variant FlowsTo -->
Indicates that the object has content that flows logically to another
 AtkObject in a sequential way, (for instance text-flow).
<!-- enum RelationType::variant FlowsFrom -->
Indicates that the object has content that flows logically from
 another AtkObject in a sequential way, (for instance text-flow).
<!-- enum RelationType::variant SubwindowOf -->
Indicates a subwindow attached to a component but otherwise has no connection in the UI heirarchy to that component.
<!-- enum RelationType::variant Embeds -->
Indicates that the object visually embeds
 another object's content, i.e. this object's content flows around
 another's content.
<!-- enum RelationType::variant EmbeddedBy -->
Reciprocal of `RelationType::Embeds`, indicates that
 this object's content is visualy embedded in another object.
<!-- enum RelationType::variant PopupFor -->
Indicates that an object is a popup for another object.
<!-- enum RelationType::variant ParentWindowOf -->
Indicates that an object is a parent window of another object.
<!-- enum RelationType::variant DescribedBy -->
Reciprocal of `RelationType::DescriptionFor`. Indicates that one
or more target objects provide descriptive information about this object. This relation
type is most appropriate for information that is not essential as its presentation may
be user-configurable and/or limited to an on-demand mechanism such as an assistive
technology command. For brief, essential information such as can be found in a widget's
on-screen label, use `RelationType::LabelledBy`. For an on-screen error message, use
`RelationType::ErrorMessage`. For lengthy extended descriptive information contained in
an on-screen object, consider using `RelationType::Details` as assistive technologies may
provide a means for the user to navigate to objects containing detailed descriptions so
that their content can be more closely reviewed.
<!-- enum RelationType::variant DescriptionFor -->
Reciprocal of `RelationType::DescribedBy`. Indicates that this
object provides descriptive information about the target object(s). See also
`RelationType::DetailsFor` and `RelationType::ErrorFor`.
<!-- enum RelationType::variant NodeParentOf -->
Indicates an object is a cell in a treetable and is expanded to display other cells in the same column.
<!-- enum RelationType::variant Details -->
Reciprocal of `RelationType::DetailsFor`. Indicates that this object
has a detailed or extended description, the contents of which can be found in the target
object(s). This relation type is most appropriate for information that is sufficiently
lengthy as to make navigation to the container of that information desirable. For less
verbose information suitable for announcement only, see `RelationType::DescribedBy`. If
the detailed information describes an error condition, `RelationType::ErrorFor` should be
used instead. `Since`: ATK-2.26.
<!-- enum RelationType::variant DetailsFor -->
Reciprocal of `RelationType::Details`. Indicates that this object
provides a detailed or extended description about the target object(s). See also
`RelationType::DescriptionFor` and `RelationType::ErrorFor`. `Since`: ATK-2.26.
<!-- enum RelationType::variant ErrorMessage -->
Reciprocal of `RelationType::ErrorFor`. Indicates that this object
has one or more errors, the nature of which is described in the contents of the target
object(s). Objects that have this relation type should also contain `StateType::InvalidEntry`
in their `StateSet`. `Since`: ATK-2.26.
<!-- enum RelationType::variant ErrorFor -->
Reciprocal of `RelationType::ErrorMessage`. Indicates that this object
contains an error message describing an invalid condition in the target object(s). `Since`:
ATK_2.26.
<!-- enum RelationType::variant LastDefined -->
Not used, this value indicates the end of the enumeration.
<!-- enum Role -->
Describes the role of an object

These are the built-in enumerated roles that UI components can have in
ATK. Other roles may be added at runtime, so an AtkRole >=
ATK_ROLE_LAST_DEFINED is not necessarily an error.
<!-- enum Role::variant Invalid -->
Invalid role
<!-- enum Role::variant AcceleratorLabel -->
A label which represents an accelerator
<!-- enum Role::variant Alert -->
An object which is an alert to the user. Assistive Technologies typically respond to ATK_ROLE_ALERT by reading the entire onscreen contents of containers advertising this role. Should be used for warning dialogs, etc.
<!-- enum Role::variant Animation -->
An object which is an animated image
<!-- enum Role::variant Arrow -->
An arrow in one of the four cardinal directions
<!-- enum Role::variant Calendar -->
An object that displays a calendar and allows the user to select a date
<!-- enum Role::variant Canvas -->
An object that can be drawn into and is used to trap events
<!-- enum Role::variant CheckBox -->
A choice that can be checked or unchecked and provides a separate indicator for the current state
<!-- enum Role::variant CheckMenuItem -->
A menu item with a check box
<!-- enum Role::variant ColorChooser -->
A specialized dialog that lets the user choose a color
<!-- enum Role::variant ColumnHeader -->
The header for a column of data
<!-- enum Role::variant ComboBox -->
A collapsible list of choices the user can select from
<!-- enum Role::variant DateEditor -->
An object whose purpose is to allow a user to edit a date
<!-- enum Role::variant DesktopIcon -->
An inconifed internal frame within a DESKTOP_PANE
<!-- enum Role::variant DesktopFrame -->
A pane that supports internal frames and iconified versions of those internal frames
<!-- enum Role::variant Dial -->
An object whose purpose is to allow a user to set a value
<!-- enum Role::variant Dialog -->
A top level window with title bar and a border
<!-- enum Role::variant DirectoryPane -->
A pane that allows the user to navigate through and select the contents of a directory
<!-- enum Role::variant DrawingArea -->
An object used for drawing custom user interface elements
<!-- enum Role::variant FileChooser -->
A specialized dialog that lets the user choose a file
<!-- enum Role::variant Filler -->
A object that fills up space in a user interface
<!-- enum Role::variant FontChooser -->
A specialized dialog that lets the user choose a font
<!-- enum Role::variant Frame -->
A top level window with a title bar, border, menubar, etc.
<!-- enum Role::variant GlassPane -->
A pane that is guaranteed to be painted on top of all panes beneath it
<!-- enum Role::variant HtmlContainer -->
A document container for HTML, whose children represent the document content
<!-- enum Role::variant Icon -->
A small fixed size picture, typically used to decorate components
<!-- enum Role::variant Image -->
An object whose primary purpose is to display an image
<!-- enum Role::variant InternalFrame -->
A frame-like object that is clipped by a desktop pane
<!-- enum Role::variant Label -->
An object used to present an icon or short string in an interface
<!-- enum Role::variant LayeredPane -->
A specialized pane that allows its children to be drawn in layers, providing a form of stacking order
<!-- enum Role::variant List -->
An object that presents a list of objects to the user and allows the user to select one or more of them
<!-- enum Role::variant ListItem -->
An object that represents an element of a list
<!-- enum Role::variant Menu -->
An object usually found inside a menu bar that contains a list of actions the user can choose from
<!-- enum Role::variant MenuBar -->
An object usually drawn at the top of the primary dialog box of an application that contains a list of menus the user can choose from
<!-- enum Role::variant MenuItem -->
An object usually contained in a menu that presents an action the user can choose
<!-- enum Role::variant OptionPane -->
A specialized pane whose primary use is inside a DIALOG
<!-- enum Role::variant PageTab -->
An object that is a child of a page tab list
<!-- enum Role::variant PageTabList -->
An object that presents a series of panels (or page tabs), one at a time, through some mechanism provided by the object
<!-- enum Role::variant Panel -->
A generic container that is often used to group objects
<!-- enum Role::variant PasswordText -->
A text object uses for passwords, or other places where the text content is not shown visibly to the user
<!-- enum Role::variant PopupMenu -->
A temporary window that is usually used to offer the user a list of choices, and then hides when the user selects one of those choices
<!-- enum Role::variant ProgressBar -->
An object used to indicate how much of a task has been completed
<!-- enum Role::variant PushButton -->
An object the user can manipulate to tell the application to do something
<!-- enum Role::variant RadioButton -->
A specialized check box that will cause other radio buttons in the same group to become unchecked when this one is checked
<!-- enum Role::variant RadioMenuItem -->
A check menu item which belongs to a group. At each instant exactly one of the radio menu items from a group is selected
<!-- enum Role::variant RootPane -->
A specialized pane that has a glass pane and a layered pane as its children
<!-- enum Role::variant RowHeader -->
The header for a row of data
<!-- enum Role::variant ScrollBar -->
An object usually used to allow a user to incrementally view a large amount of data.
<!-- enum Role::variant ScrollPane -->
An object that allows a user to incrementally view a large amount of information
<!-- enum Role::variant Separator -->
An object usually contained in a menu to provide a visible and logical separation of the contents in a menu
<!-- enum Role::variant Slider -->
An object that allows the user to select from a bounded range
<!-- enum Role::variant SplitPane -->
A specialized panel that presents two other panels at the same time
<!-- enum Role::variant SpinButton -->
An object used to get an integer or floating point number from the user
<!-- enum Role::variant Statusbar -->
An object which reports messages of minor importance to the user
<!-- enum Role::variant Table -->
An object used to represent information in terms of rows and columns
<!-- enum Role::variant TableCell -->
A cell in a table
<!-- enum Role::variant TableColumnHeader -->
The header for a column of a table
<!-- enum Role::variant TableRowHeader -->
The header for a row of a table
<!-- enum Role::variant TearOffMenuItem -->
A menu item used to tear off and reattach its menu
<!-- enum Role::variant Terminal -->
An object that represents an accessible terminal. `Since`: ATK-0.6
<!-- enum Role::variant Text -->
An interactive widget that supports multiple lines of text and
optionally accepts user input, but whose purpose is not to solicit user input.
Thus ATK_ROLE_TEXT is appropriate for the text view in a plain text editor
but inappropriate for an input field in a dialog box or web form. For widgets
whose purpose is to solicit input from the user, see ATK_ROLE_ENTRY and
ATK_ROLE_PASSWORD_TEXT. For generic objects which display a brief amount of
textual information, see ATK_ROLE_STATIC.
<!-- enum Role::variant ToggleButton -->
A specialized push button that can be checked or unchecked, but does not provide a separate indicator for the current state
<!-- enum Role::variant ToolBar -->
A bar or palette usually composed of push buttons or toggle buttons
<!-- enum Role::variant ToolTip -->
An object that provides information about another object
<!-- enum Role::variant Tree -->
An object used to represent hierarchical information to the user
<!-- enum Role::variant TreeTable -->
An object capable of expanding and collapsing rows as well as showing multiple columns of data. `Since`: ATK-0.7
<!-- enum Role::variant Unknown -->
The object contains some Accessible information, but its role is not known
<!-- enum Role::variant Viewport -->
An object usually used in a scroll pane
<!-- enum Role::variant Window -->
A top level window with no title or border.
<!-- enum Role::variant Header -->
An object that serves as a document header. `Since`: ATK-1.1.1
<!-- enum Role::variant Footer -->
An object that serves as a document footer. `Since`: ATK-1.1.1
<!-- enum Role::variant Paragraph -->
An object which is contains a paragraph of text content. `Since`: ATK-1.1.1
<!-- enum Role::variant Ruler -->
An object which describes margins and tab stops, etc. for text objects which it controls (should have CONTROLLER_FOR relation to such). `Since`: ATK-1.1.1
<!-- enum Role::variant Application -->
The object is an application object, which may contain `Role::Frame` objects or other types of accessibles. The root accessible of any application's ATK hierarchy should have ATK_ROLE_APPLICATION. `Since`: ATK-1.1.4
<!-- enum Role::variant Autocomplete -->
The object is a dialog or list containing items for insertion into an entry widget, for instance a list of words for completion of a text entry. `Since`: ATK-1.3
<!-- enum Role::variant EditBar -->
The object is an editable text object in a toolbar. `Since`: ATK-1.5
<!-- enum Role::variant Embedded -->
The object is an embedded container within a document or panel. This role is a grouping "hint" indicating that the contained objects share a context. `Since`: ATK-1.7.2
<!-- enum Role::variant Entry -->
The object is a component whose textual content may be entered or modified by the user, provided `StateType::Editable` is present. `Since`: ATK-1.11
<!-- enum Role::variant Chart -->
The object is a graphical depiction of quantitative data. It may contain multiple subelements whose attributes and/or description may be queried to obtain both the quantitative data and information about how the data is being presented. The LABELLED_BY relation is particularly important in interpreting objects of this type, as is the accessible-description property. `Since`: ATK-1.11
<!-- enum Role::variant Caption -->
The object contains descriptive information, usually textual, about another user interface element such as a table, chart, or image. `Since`: ATK-1.11
<!-- enum Role::variant DocumentFrame -->
The object is a visual frame or container which contains a view of document content. Document frames may occur within another Document instance, in which case the second document may be said to be embedded in the containing instance. HTML frames are often ROLE_DOCUMENT_FRAME. Either this object, or a singleton descendant, should implement the Document interface. `Since`: ATK-1.11
<!-- enum Role::variant Heading -->
The object serves as a heading for content which follows it in a document. The 'heading level' of the heading, if availabe, may be obtained by querying the object's attributes.
<!-- enum Role::variant Page -->
The object is a containing instance which encapsulates a page of information. `Role::Page` is used in documents and content which support a paginated navigation model. `Since`: ATK-1.11
<!-- enum Role::variant Section -->
The object is a containing instance of document content which constitutes a particular 'logical' section of the document. The type of content within a section, and the nature of the section division itself, may be obtained by querying the object's attributes. Sections may be nested. `Since`: ATK-1.11
<!-- enum Role::variant RedundantObject -->
The object is redundant with another object in the hierarchy, and is exposed for purely technical reasons. Objects of this role should normally be ignored by clients. `Since`: ATK-1.11
<!-- enum Role::variant Form -->
The object is a container for form controls, for instance as part of a
web form or user-input form within a document. This role is primarily a tag/convenience for
clients when navigating complex documents, it is not expected that ordinary GUI containers will
always have ATK_ROLE_FORM. `Since`: ATK-1.12.0
<!-- enum Role::variant Link -->
The object is a hypertext anchor, i.e. a "link" in a
hypertext document. Such objects are distinct from 'inline'
content which may also use the Hypertext/Hyperlink interfaces
to indicate the range/location within a text object where
an inline or embedded object lies. `Since`: ATK-1.12.1
<!-- enum Role::variant InputMethodWindow -->
The object is a window or similar viewport
which is used to allow composition or input of a 'complex character',
in other words it is an "input method window." `Since`: ATK-1.12.1
<!-- enum Role::variant TableRow -->
A row in a table. `Since`: ATK-2.1.0
<!-- enum Role::variant TreeItem -->
An object that represents an element of a tree. `Since`: ATK-2.1.0
<!-- enum Role::variant DocumentSpreadsheet -->
A document frame which contains a spreadsheet. `Since`: ATK-2.1.0
<!-- enum Role::variant DocumentPresentation -->
A document frame which contains a presentation or slide content. `Since`: ATK-2.1.0
<!-- enum Role::variant DocumentText -->
A document frame which contains textual content, such as found in a word processing application. `Since`: ATK-2.1.0
<!-- enum Role::variant DocumentWeb -->
A document frame which contains HTML or other markup suitable for display in a web browser. `Since`: ATK-2.1.0
<!-- enum Role::variant DocumentEmail -->
A document frame which contains email content to be displayed or composed either in plain text or HTML. `Since`: ATK-2.1.0
<!-- enum Role::variant Comment -->
An object found within a document and designed to present a comment, note, or other annotation. In some cases, this object might not be visible until activated. `Since`: ATK-2.1.0
<!-- enum Role::variant ListBox -->
A non-collapsible list of choices the user can select from. `Since`: ATK-2.1.0
<!-- enum Role::variant Grouping -->
A group of related widgets. This group typically has a label. `Since`: ATK-2.1.0
<!-- enum Role::variant ImageMap -->
An image map object. Usually a graphic with multiple hotspots, where each hotspot can be activated resulting in the loading of another document or section of a document. `Since`: ATK-2.1.0
<!-- enum Role::variant Notification -->
A transitory object designed to present a message to the user, typically at the desktop level rather than inside a particular application. `Since`: ATK-2.1.0
<!-- enum Role::variant InfoBar -->
An object designed to present a message to the user within an existing window. `Since`: ATK-2.1.0
<!-- enum Role::variant LevelBar -->
A bar that serves as a level indicator to, for instance, show the strength of a password or the state of a battery. `Since`: ATK-2.7.3
<!-- enum Role::variant TitleBar -->
A bar that serves as the title of a window or a
dialog. `Since`: ATK-2.12
<!-- enum Role::variant BlockQuote -->
An object which contains a text section
that is quoted from another source. `Since`: ATK-2.12
<!-- enum Role::variant Audio -->
An object which represents an audio element. `Since`: ATK-2.12
<!-- enum Role::variant Video -->
An object which represents a video element. `Since`: ATK-2.12
<!-- enum Role::variant Definition -->
A definition of a term or concept. `Since`: ATK-2.12
<!-- enum Role::variant Article -->
A section of a page that consists of a
composition that forms an independent part of a document, page, or
site. Examples: A blog entry, a news story, a forum post. `Since`:
ATK-2.12
<!-- enum Role::variant Landmark -->
A region of a web page intended as a
navigational landmark. This is designed to allow Assistive
Technologies to provide quick navigation among key regions within a
document. `Since`: ATK-2.12
<!-- enum Role::variant Log -->
A text widget or container holding log content, such
as chat history and error logs. In this role there is a
relationship between the arrival of new items in the log and the
reading order. The log contains a meaningful sequence and new
information is added only to the end of the log, not at arbitrary
points. `Since`: ATK-2.12
<!-- enum Role::variant Marquee -->
A container where non-essential information
changes frequently. Common usages of marquee include stock tickers
and ad banners. The primary difference between a marquee and a log
is that logs usually have a meaningful order or sequence of
important content changes. `Since`: ATK-2.12
<!-- enum Role::variant Math -->
A text widget or container that holds a mathematical
expression. `Since`: ATK-2.12
<!-- enum Role::variant Rating -->
A widget whose purpose is to display a rating,
such as the number of stars associated with a song in a media
player. Objects of this role should also implement
AtkValue. `Since`: ATK-2.12
<!-- enum Role::variant Timer -->
An object containing a numerical counter which
indicates an amount of elapsed time from a start point, or the time
remaining until an end point. `Since`: ATK-2.12
<!-- enum Role::variant DescriptionList -->
An object that represents a list of
term-value groups. A term-value group represents a individual
description and consist of one or more names
(ATK_ROLE_DESCRIPTION_TERM) followed by one or more values
(ATK_ROLE_DESCRIPTION_VALUE). For each list, there should not be
more than one group with the same term name. `Since`: ATK-2.12
<!-- enum Role::variant DescriptionTerm -->
An object that represents a term or phrase
with a corresponding definition. `Since`: ATK-2.12
<!-- enum Role::variant DescriptionValue -->
An object that represents the
description, definition or value of a term. `Since`: ATK-2.12
<!-- enum Role::variant Static -->
A generic non-container object whose purpose is to display a
brief amount of information to the user and whose role is known by the
implementor but lacks semantic value for the user. Examples in which
ATK_ROLE_STATIC is appropriate include the message displayed in a message box
and an image used as an alternative means to display text. ATK_ROLE_STATIC
should not be applied to widgets which are traditionally interactive, objects
which display a significant amount of content, or any object which has an
accessible relation pointing to another object. Implementors should expose the
displayed information through the accessible name of the object. If doing so seems
inappropriate, it may indicate that a different role should be used. For
labels which describe another widget, see ATK_ROLE_LABEL. For text views, see
ATK_ROLE_TEXT. For generic containers, see ATK_ROLE_PANEL. For objects whose
role is not known by the implementor, see ATK_ROLE_UNKNOWN. `Since`: ATK-2.16.
<!-- enum Role::variant MathFraction -->
An object that represents a mathematical fraction.
<!-- enum Role::variant MathRoot -->
An object that represents a mathematical expression
displayed with a radical. `Since`: ATK-2.16.
<!-- enum Role::variant Subscript -->
An object that contains text that is displayed as a
subscript. `Since`: ATK-2.16.
<!-- enum Role::variant Superscript -->
An object that contains text that is displayed as a
superscript. `Since`: ATK-2.16.
<!-- enum Role::variant Footnote -->
An object that contains the text of a footnote. `Since`: ATK-2.26.
<!-- enum Role::variant LastDefined -->
not a valid role, used for finding end of the enumeration
<!-- enum ScrollType -->
Specifies where an object should be placed on the screen when using scroll_to.
<!-- enum ScrollType::variant TopLeft -->
Scroll the object vertically and horizontally to the top
left corner of the window.
<!-- enum ScrollType::variant BottomRight -->
Scroll the object vertically and horizontally to the
bottom right corner of the window.
<!-- enum ScrollType::variant TopEdge -->
Scroll the object vertically to the top edge of the
 window.
<!-- enum ScrollType::variant BottomEdge -->
Scroll the object vertically to the bottom edge of
the window.
<!-- enum ScrollType::variant LeftEdge -->
Scroll the object vertically and horizontally to the
left edge of the window.
<!-- enum ScrollType::variant RightEdge -->
Scroll the object vertically and horizontally to the
right edge of the window.
<!-- enum ScrollType::variant Anywhere -->
Scroll the object vertically and horizontally so that
as much as possible of the object becomes visible. The exact placement is
determined by the application.

Feature: `v2_30`

<!-- struct Selection -->
`Selection` should be implemented by UI components with children
which are exposed by `atk_object_ref_child` and
`atk_object_get_n_children`, if the use of the parent UI component
ordinarily involves selection of one or more of the objects
corresponding to those `Object` children - for example,
selectable lists.

Note that other types of "selection" (for instance text selection)
are accomplished a other ATK interfaces - `Selection` is limited
to the selection/deselection of children.

# Implements

[`SelectionExt`](trait.SelectionExt.html)
<!-- trait SelectionExt -->
Trait containing all `Selection` methods.

# Implementors

[`NoOpObject`](struct.NoOpObject.html), [`Selection`](struct.Selection.html)
<!-- trait SelectionExt::fn add_selection -->
Adds the specified accessible child of the object to the
object's selection.
## `i`
a `gint` specifying the child index.

# Returns

TRUE if success, FALSE otherwise.
<!-- trait SelectionExt::fn clear_selection -->
Clears the selection in the object so that no children in the object
are selected.

# Returns

TRUE if success, FALSE otherwise.
<!-- trait SelectionExt::fn get_selection_count -->
Gets the number of accessible children currently selected.
Note: callers should not rely on `None` or on a zero value for
indication of whether AtkSelectionIface is implemented, they should
use type checking/interface checking macros or the
`atk_get_accessible_value` convenience method.

# Returns

a gint representing the number of items selected, or 0
if `self` does not implement this interface.
<!-- trait SelectionExt::fn is_child_selected -->
Determines if the current child of this object is selected
Note: callers should not rely on `None` or on a zero value for
indication of whether AtkSelectionIface is implemented, they should
use type checking/interface checking macros or the
`atk_get_accessible_value` convenience method.
## `i`
a `gint` specifying the child index.

# Returns

a gboolean representing the specified child is selected, or 0
if `self` does not implement this interface.
<!-- trait SelectionExt::fn ref_selection -->
Gets a reference to the accessible object representing the specified
selected child of the object.
Note: callers should not rely on `None` or on a zero value for
indication of whether AtkSelectionIface is implemented, they should
use type checking/interface checking macros or the
`atk_get_accessible_value` convenience method.
## `i`
a `gint` specifying the index in the selection set. (e.g. the
ith selection as opposed to the ith child).

# Returns

an `Object` representing the
selected accessible, or `None` if `self` does not implement this
interface.
<!-- trait SelectionExt::fn remove_selection -->
Removes the specified child of the object from the object's selection.
## `i`
a `gint` specifying the index in the selection set. (e.g. the
ith selection as opposed to the ith child).

# Returns

TRUE if success, FALSE otherwise.
<!-- trait SelectionExt::fn select_all_selection -->
Causes every child of the object to be selected if the object
supports multiple selections.

# Returns

TRUE if success, FALSE otherwise.
<!-- trait SelectionExt::fn connect_selection_changed -->
The "selection-changed" signal is emitted by an object which
implements AtkSelection interface when the selection changes.
<!-- struct Socket -->
Together with `Plug`, `Socket` provides the ability to embed
accessibles from one process into another in a fashion that is
transparent to assistive technologies. `Socket` works as the
container of `Plug`, embedding it using the method
`AtkSocketExt::embed`. Any accessible contained in the `Plug` will
appear to the assistive technologies as being inside the
application that created the `Socket`.

The communication between a `Socket` and a `Plug` is done by
the IPC layer of the accessibility framework, normally implemented
by the D-Bus based implementation of AT-SPI (at-spi2). If that is
the case, at-spi-atk2 is the responsible to implement the abstract
methods `AtkPlugExt::get_id` and `AtkSocketExt::embed`, so an ATK
implementor shouldn't reimplement them. The process that contains
the `Plug` is responsible to send the ID returned by
`atk_plug_id` to the process that contains the `Socket`, so it
could call the method `AtkSocketExt::embed` in order to embed it.

For the same reasons, an implementor doesn't need to implement
`AtkObjectExt::get_n_accessible_children` and
`AtkObjectExt::ref_accessible_child`. All the logic related to those
functions will be implemented by the IPC layer.

# Implements

[`AtkSocketExt`](trait.AtkSocketExt.html), [`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ComponentExt`](trait.ComponentExt.html)
<!-- trait AtkSocketExt -->
Trait containing all `Socket` methods.

# Implementors

[`Socket`](struct.Socket.html)
<!-- trait AtkSocketExt::fn embed -->
Embeds the children of an `Plug` as the children of the
`Socket`. The plug may be in the same process or in a different
process.

The class item used by this function should be filled in by the IPC
layer (usually at-spi2-atk). The implementor of the AtkSocket
should call this function and pass the id for the plug as returned
by `AtkPlugExt::get_id`. It is the responsibility of the application
to pass the plug id on to the process implementing the `Socket`
as needed.
## `plug_id`
the ID of an `Plug`
<!-- trait AtkSocketExt::fn is_occupied -->
Determines whether or not the socket has an embedded plug.

# Returns

TRUE if a plug is embedded in the socket
<!-- struct StateSet -->
An AtkStateSet is a read-only representation of the full set of `AtkStates`
that apply to an object at a given time. This set is not meant to be
modified, but rather created when `AtkObjectExt::ref_state_set`() is called.

# Implements

[`StateSetExt`](trait.StateSetExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait StateSetExt -->
Trait containing all `StateSet` methods.

# Implementors

[`StateSet`](struct.StateSet.html)
<!-- impl StateSet::fn new -->
Creates a new empty state set.

# Returns

a new `StateSet`
<!-- trait StateSetExt::fn add_state -->
Adds the state of the specified type to the state set if it is not already
present.

Note that because an `StateSet` is a read-only object, this method should
be used to add a state to a newly-created set which will then be returned by
`AtkObjectExt::ref_state_set`. It should not be used to modify the existing state
of an object. See also `AtkObjectExt::notify_state_change`.
## `type_`
an `StateType`

# Returns

`true` if the state for `type_` is not already in `self`.
<!-- trait StateSetExt::fn add_states -->
Adds the states of the specified types to the state set.

Note that because an `StateSet` is a read-only object, this method should
be used to add states to a newly-created set which will then be returned by
`AtkObjectExt::ref_state_set`. It should not be used to modify the existing state
of an object. See also `AtkObjectExt::notify_state_change`.
## `types`
an array of `StateType`
## `n_types`
The number of elements in the array
<!-- trait StateSetExt::fn and_sets -->
Constructs the intersection of the two sets, returning `None` if the
intersection is empty.
## `compare_set`
another `StateSet`

# Returns

a new `StateSet` which is the intersection of
the two sets.
<!-- trait StateSetExt::fn clear_states -->
Removes all states from the state set.
<!-- trait StateSetExt::fn contains_state -->
Checks whether the state for the specified type is in the specified set.
## `type_`
an `StateType`

# Returns

`true` if `type_` is the state type is in `self`.
<!-- trait StateSetExt::fn contains_states -->
Checks whether the states for all the specified types are in the
specified set.
## `types`
an array of `StateType`
## `n_types`
The number of elements in the array

# Returns

`true` if all the states for `type_` are in `self`.
<!-- trait StateSetExt::fn is_empty -->
Checks whether the state set is empty, i.e. has no states set.

# Returns

`true` if `self` has no states set, otherwise `false`
<!-- trait StateSetExt::fn or_sets -->
Constructs the union of the two sets.
## `compare_set`
another `StateSet`

# Returns

a new `StateSet` which is
the union of the two sets, returning `None` is empty.
<!-- trait StateSetExt::fn remove_state -->
Removes the state for the specified type from the state set.

Note that because an `StateSet` is a read-only object, this method should
be used to remove a state to a newly-created set which will then be returned
by `AtkObjectExt::ref_state_set`. It should not be used to modify the existing
state of an object. See also `AtkObjectExt::notify_state_change`.
## `type_`
an `AtkType`

# Returns

`true` if `type_` was the state type is in `self`.
<!-- trait StateSetExt::fn xor_sets -->
Constructs the exclusive-or of the two sets, returning `None` is empty.
The set returned by this operation contains the states in exactly
one of the two sets.
## `compare_set`
another `StateSet`

# Returns

a new `StateSet` which contains the states
which are in exactly one of the two sets.
<!-- enum StateType -->
The possible types of states of an object
<!-- enum StateType::variant Invalid -->
Indicates an invalid state - probably an error condition.
<!-- enum StateType::variant Active -->
Indicates a window is currently the active window, or an object is the active subelement within a container or table. ATK_STATE_ACTIVE should not be used for objects which have ATK_STATE_FOCUSABLE or ATK_STATE_SELECTABLE: Those objects should use ATK_STATE_FOCUSED and ATK_STATE_SELECTED respectively. ATK_STATE_ACTIVE is a means to indicate that an object which is not focusable and not selectable is the currently-active item within its parent container.
<!-- enum StateType::variant Armed -->
Indicates that the object is 'armed', i.e. will be activated by if a pointer button-release event occurs within its bounds. Buttons often enter this state when a pointer click occurs within their bounds, as a precursor to activation. ATK_STATE_ARMED has been deprecated since ATK-2.16 and should not be used in newly-written code.
<!-- enum StateType::variant Busy -->
Indicates the current object is busy, i.e. onscreen representation is in the process of changing, or the object is temporarily unavailable for interaction due to activity already in progress. This state may be used by implementors of Document to indicate that content loading is underway. It also may indicate other 'pending' conditions; clients may wish to interrogate this object when the ATK_STATE_BUSY flag is removed.
<!-- enum StateType::variant Checked -->
Indicates this object is currently checked, for instance a checkbox is 'non-empty'.
<!-- enum StateType::variant Defunct -->
Indicates that this object no longer has a valid backing widget (for instance, if its peer object has been destroyed)
<!-- enum StateType::variant Editable -->
Indicates that this object can contain text, and that the
user can change the textual contents of this object by editing those contents
directly. For an object which is expected to be editable due to its type, but
which cannot be edited due to the application or platform preventing the user
from doing so, that object's `StateSet` should lack ATK_STATE_EDITABLE and
should contain ATK_STATE_READ_ONLY.
<!-- enum StateType::variant Enabled -->
Indicates that this object is enabled, i.e. that it currently reflects some application state. Objects that are "greyed out" may lack this state, and may lack the STATE_SENSITIVE if direct user interaction cannot cause them to acquire STATE_ENABLED. See also: ATK_STATE_SENSITIVE
<!-- enum StateType::variant Expandable -->
Indicates this object allows progressive disclosure of its children
<!-- enum StateType::variant Expanded -->
Indicates this object its expanded - see ATK_STATE_EXPANDABLE above
<!-- enum StateType::variant Focusable -->
Indicates this object can accept keyboard focus, which means all events resulting from typing on the keyboard will normally be passed to it when it has focus
<!-- enum StateType::variant Focused -->
Indicates this object currently has the keyboard focus
<!-- enum StateType::variant Horizontal -->
Indicates the orientation of this object is horizontal; used, for instance, by objects of ATK_ROLE_SCROLL_BAR. For objects where vertical/horizontal orientation is especially meaningful.
<!-- enum StateType::variant Iconified -->
Indicates this object is minimized and is represented only by an icon
<!-- enum StateType::variant Modal -->
Indicates something must be done with this object before the user can interact with an object in a different window
<!-- enum StateType::variant MultiLine -->
Indicates this (text) object can contain multiple lines of text
<!-- enum StateType::variant Multiselectable -->
Indicates this object allows more than one of its children to be selected at the same time, or in the case of text objects, that the object supports non-contiguous text selections.
<!-- enum StateType::variant Opaque -->
Indicates this object paints every pixel within its rectangular region.
<!-- enum StateType::variant Pressed -->
Indicates this object is currently pressed.
<!-- enum StateType::variant Resizable -->
Indicates the size of this object is not fixed
<!-- enum StateType::variant Selectable -->
Indicates this object is the child of an object that allows its children to be selected and that this child is one of those children that can be selected
<!-- enum StateType::variant Selected -->
Indicates this object is the child of an object that allows its children to be selected and that this child is one of those children that has been selected
<!-- enum StateType::variant Sensitive -->
Indicates this object is sensitive, e.g. to user interaction.
STATE_SENSITIVE usually accompanies STATE_ENABLED for user-actionable controls,
but may be found in the absence of STATE_ENABLED if the current visible state of the
control is "disconnected" from the application state. In such cases, direct user interaction
can often result in the object gaining STATE_SENSITIVE, for instance if a user makes
an explicit selection using an object whose current state is ambiguous or undefined.
`see` STATE_ENABLED, STATE_INDETERMINATE.
<!-- enum StateType::variant Showing -->
Indicates this object, the object's parent, the object's parent's parent, and so on,
are all 'shown' to the end-user, i.e. subject to "exposure" if blocking or obscuring objects do not interpose
between this object and the top of the window stack.
<!-- enum StateType::variant SingleLine -->
Indicates this (text) object can contain only a single line of text
<!-- enum StateType::variant Stale -->
Indicates that the information returned for this object may no longer be
synchronized with the application state. This is implied if the object has STATE_TRANSIENT,
and can also occur towards the end of the object peer's lifecycle. It can also be used to indicate that
the index associated with this object has changed since the user accessed the object (in lieu of
"index-in-parent-changed" events).
<!-- enum StateType::variant Transient -->
Indicates this object is transient, i.e. a snapshot which may not emit events when its
state changes. Data from objects with ATK_STATE_TRANSIENT should not be cached, since there may be no
notification given when the cached data becomes obsolete.
<!-- enum StateType::variant Vertical -->
Indicates the orientation of this object is vertical
<!-- enum StateType::variant Visible -->
Indicates this object is visible, e.g. has been explicitly marked for exposure to the user.
<!-- enum StateType::variant ManagesDescendants -->
Indicates that "active-descendant-changed" event
is sent when children become 'active' (i.e. are selected or navigated to onscreen).
Used to prevent need to enumerate all children in very large containers, like tables.
The presence of STATE_MANAGES_DESCENDANTS is an indication to the client.
that the children should not, and need not, be enumerated by the client.
Objects implementing this state are expected to provide relevant state
notifications to listening clients, for instance notifications of visibility
changes and activation of their contained child objects, without the client
having previously requested references to those children.
<!-- enum StateType::variant Indeterminate -->
Indicates that the value, or some other quantifiable
property, of this AtkObject cannot be fully determined. In the case of a large
data set in which the total number of items in that set is unknown (e.g. 1 of
999+), implementors should expose the currently-known set size (999) along
with this state. In the case of a check box, this state should be used to
indicate that the check box is a tri-state check box which is currently
neither checked nor unchecked.
<!-- enum StateType::variant Truncated -->
Indicates that an object is truncated, e.g. a text value in a speradsheet cell.
<!-- enum StateType::variant Required -->
Indicates that explicit user interaction with an object is required by the user interface, e.g. a required field in a "web-form" interface.
<!-- enum StateType::variant InvalidEntry -->
Indicates that the object has encountered an error condition due to failure of input validation. For instance, a form control may acquire this state in response to invalid or malformed user input.
<!-- enum StateType::variant SupportsAutocompletion -->
Indicates that the object in question implements some form of ¨typeahead¨ or
pre-selection behavior whereby entering the first character of one or more sub-elements
causes those elements to scroll into view or become selected. Subsequent character input
may narrow the selection further as long as one or more sub-elements match the string.
This state is normally only useful and encountered on objects that implement Selection.
In some cases the typeahead behavior may result in full or partial ¨completion¨ of
the data in the input field, in which case these input events may trigger text-changed
events from the AtkText interface. This state supplants `Role::Autocomplete`.
<!-- enum StateType::variant SelectableText -->
Indicates that the object in question supports text selection. It should only be exposed on objects which implement the Text interface, in order to distinguish this state from `StateType::Selectable`, which infers that the object in question is a selectable child of an object which implements Selection. While similar, text selection and subelement selection are distinct operations.
<!-- enum StateType::variant Default -->
Indicates that the object is the "default" active component, i.e. the object which is activated by an end-user press of the "Enter" or "Return" key. Typically a "close" or "submit" button.
<!-- enum StateType::variant Animated -->
Indicates that the object changes its appearance dynamically as an inherent part of its presentation. This state may come and go if an object is only temporarily animated on the way to a 'final' onscreen presentation.
`note` some applications, notably content viewers, may not be able to detect
all kinds of animated content. Therefore the absence of this state should not
be taken as definitive evidence that the object's visual representation is
static; this state is advisory.
<!-- enum StateType::variant Visited -->
Indicates that the object (typically a hyperlink) has already been 'activated', and/or its backing data has already been downloaded, rendered, or otherwise "visited".
<!-- enum StateType::variant Checkable -->
Indicates this object has the potential to be
 checked, such as a checkbox or toggle-able table cell. `Since`:
 ATK-2.12
<!-- enum StateType::variant HasPopup -->
Indicates that the object has a popup context
menu or sub-level menu which may or may not be showing. This means
that activation renders conditional content. Note that ordinary
tooltips are not considered popups in this context. `Since`: ATK-2.12
<!-- enum StateType::variant HasTooltip -->
Indicates this object has a tooltip. `Since`: ATK-2.16
<!-- enum StateType::variant ReadOnly -->
Indicates that a widget which is ENABLED and SENSITIVE
has a value which can be read, but not modified, by the user. Note that this
state should only be applied to widget types whose value is normally directly
user modifiable, such as check boxes, radio buttons, spin buttons, text input
fields, and combo boxes, as a means to convey that the expected interaction
with that widget is not possible. When the expected interaction with a
widget does not include modification by the user, as is the case with
labels and containers, ATK_STATE_READ_ONLY should not be applied. See also
ATK_STATE_EDITABLE. `Since`: ATK-2-16
<!-- enum StateType::variant LastDefined -->
Not a valid state, used for finding end of enumeration
<!-- struct StreamableContent -->
An interface whereby an object allows its backing content to be
streamed to clients. Typical implementors would be images or
icons, HTML content, or multimedia display/rendering widgets.

Negotiation of content type is allowed. Clients may examine the
backing data and transform, convert, or parse the content in order
to present it in an alternate form to end-users.

The AtkStreamableContent interface is particularly useful for
saving, printing, or post-processing entire documents, or for
persisting alternate views of a document. If document content
itself is being serialized, stored, or converted, then use of the
AtkStreamableContent interface can help address performance
issues. Unlike most ATK interfaces, this interface is not strongly
tied to the current user-agent view of the a particular document,
but may in some cases give access to the underlying model data.

# Implements

[`StreamableContentExt`](trait.StreamableContentExt.html)
<!-- trait StreamableContentExt -->
Trait containing all `StreamableContent` methods.

# Implementors

[`StreamableContent`](struct.StreamableContent.html)
<!-- trait StreamableContentExt::fn get_mime_type -->
Gets the character string of the specified mime type. The first mime
type is at position 0, the second at position 1, and so on.
## `i`
a gint representing the position of the mime type starting from 0

# Returns

a gchar* representing the specified mime type; the caller
should not free the character string.
<!-- trait StreamableContentExt::fn get_n_mime_types -->
Gets the number of mime types supported by this object.

# Returns

a gint which is the number of mime types supported by the object.
<!-- trait StreamableContentExt::fn get_stream -->
Gets the content in the specified mime type.
## `mime_type`
a gchar* representing the mime type

# Returns

A `glib::IOChannel` which contains the content in the
specified mime type.
<!-- trait StreamableContentExt::fn get_uri -->
Get a string representing a URI in IETF standard format
(see http://www.ietf.org/rfc/rfc2396.txt) from which the object's content
may be streamed in the specified mime-type, if one is available.
If mime_type is NULL, the URI for the default (and possibly only) mime-type is
returned.

Note that it is possible for get_uri to return NULL but for
get_stream to work nonetheless, since not all GIOChannels connect to URIs.
## `mime_type`
a gchar* representing the mime type, or NULL to request a URI
for the default mime type.

# Returns

Returns a string representing a URI, or `None`
if no corresponding URI can be constructed.
<!-- struct Table -->
`Table` should be implemented by components which present
elements ordered via rows and columns. It may also be used to
present tree-structured information if the nodes of the trees can
be said to contain multiple "columns". Individual elements of an
`Table` are typically referred to as "cells". Those cells should
implement the interface `TableCell`, but `Atk` doesn't require
them to be direct children of the current `Table`. They can be
grand-children, grand-grand-children etc. `Table` provides the
API needed to get a individual cell based on the row and column
numbers.

Children of `Table` are frequently "lightweight" objects, that
is, they may not have backing widgets in the host UI toolkit. They
are therefore often transient.

Since tables are often very complex, `Table` includes provision
for offering simplified summary information, as well as row and
column headers and captions. Headers and captions are `AtkObjects`
which may implement other interfaces (`Text`, `Image`, etc.) as
appropriate. `Table` summaries may themselves be (simplified)
`AtkTables`, etc.

Note for implementors: in the past, `Table` required that all the
cells should be direct children of `Table`, and provided some
index based methods to request the cells. The practice showed that
that forcing made `Table` implementation complex, and hard to
expose other kind of children, like rows or captions. Right now,
index-based methods are deprecated.

# Implements

[`TableExt`](trait.TableExt.html)
<!-- trait TableExt -->
Trait containing all `Table` methods.

# Implementors

[`NoOpObject`](struct.NoOpObject.html), [`Table`](struct.Table.html)
<!-- trait TableExt::fn add_column_selection -->
Adds the specified `column` to the selection.
## `column`
a `gint` representing a column in `self`

# Returns

a gboolean representing if the column was successfully added to
the selection, or 0 if value does not implement this interface.
<!-- trait TableExt::fn add_row_selection -->
Adds the specified `row` to the selection.
## `row`
a `gint` representing a row in `self`

# Returns

a gboolean representing if row was successfully added to selection,
or 0 if value does not implement this interface.
<!-- trait TableExt::fn get_caption -->
Gets the caption for the `self`.

# Returns

a AtkObject* representing the
table caption, or `None` if value does not implement this interface.
<!-- trait TableExt::fn get_column_at_index -->
Gets a `gint` representing the column at the specified `index_`.

# Deprecated

Since 2.12.
## `index_`
a `gint` representing an index in `self`

# Returns

a gint representing the column at the specified index,
or -1 if the table does not implement this method.
<!-- trait TableExt::fn get_column_description -->
Gets the description text of the specified `column` in the table
## `column`
a `gint` representing a column in `self`

# Returns

a gchar* representing the column description, or `None`
if value does not implement this interface.
<!-- trait TableExt::fn get_column_extent_at -->
Gets the number of columns occupied by the accessible object
at the specified `row` and `column` in the `self`.
## `row`
a `gint` representing a row in `self`
## `column`
a `gint` representing a column in `self`

# Returns

a gint representing the column extent at specified position, or 0
if value does not implement this interface.
<!-- trait TableExt::fn get_column_header -->
Gets the column header of a specified column in an accessible table.
## `column`
a `gint` representing a column in the table

# Returns

a AtkObject* representing the
specified column header, or `None` if value does not implement this
interface.
<!-- trait TableExt::fn get_index_at -->
Gets a `gint` representing the index at the specified `row` and
`column`.

# Deprecated

Since 2.12. Use `Table::ref_at` in order to get the
accessible that represents the cell at (`row`, `column`)
## `row`
a `gint` representing a row in `self`
## `column`
a `gint` representing a column in `self`

# Returns

a `gint` representing the index at specified position.
The value -1 is returned if the object at row,column is not a child
of table or table does not implement this interface.
<!-- trait TableExt::fn get_n_columns -->
Gets the number of columns in the table.

# Returns

a gint representing the number of columns, or 0
if value does not implement this interface.
<!-- trait TableExt::fn get_n_rows -->
Gets the number of rows in the table.

# Returns

a gint representing the number of rows, or 0
if value does not implement this interface.
<!-- trait TableExt::fn get_row_at_index -->
Gets a `gint` representing the row at the specified `index_`.

# Deprecated

since 2.12.
## `index_`
a `gint` representing an index in `self`

# Returns

a gint representing the row at the specified index,
or -1 if the table does not implement this method.
<!-- trait TableExt::fn get_row_description -->
Gets the description text of the specified row in the table
## `row`
a `gint` representing a row in `self`

# Returns

a gchar* representing the row description, or
`None` if value does not implement this interface.
<!-- trait TableExt::fn get_row_extent_at -->
Gets the number of rows occupied by the accessible object
at a specified `row` and `column` in the `self`.
## `row`
a `gint` representing a row in `self`
## `column`
a `gint` representing a column in `self`

# Returns

a gint representing the row extent at specified position, or 0
if value does not implement this interface.
<!-- trait TableExt::fn get_row_header -->
Gets the row header of a specified row in an accessible table.
## `row`
a `gint` representing a row in the table

# Returns

a AtkObject* representing the
specified row header, or `None` if value does not implement this
interface.
<!-- trait TableExt::fn get_selected_columns -->
Gets the selected columns of the table by initializing **selected with
the selected column numbers. This array should be freed by the caller.
## `selected`
a `gint`** that is to contain the selected columns numbers

# Returns

a gint representing the number of selected columns,
or `0` if value does not implement this interface.
<!-- trait TableExt::fn get_selected_rows -->
Gets the selected rows of the table by initializing **selected with
the selected row numbers. This array should be freed by the caller.
## `selected`
a `gint`** that is to contain the selected row numbers

# Returns

a gint representing the number of selected rows,
or zero if value does not implement this interface.
<!-- trait TableExt::fn get_summary -->
Gets the summary description of the table.

# Returns

a AtkObject* representing a summary description
of the table, or zero if value does not implement this interface.
<!-- trait TableExt::fn is_column_selected -->
Gets a boolean value indicating whether the specified `column`
is selected
## `column`
a `gint` representing a column in `self`

# Returns

a gboolean representing if the column is selected, or 0
if value does not implement this interface.
<!-- trait TableExt::fn is_row_selected -->
Gets a boolean value indicating whether the specified `row`
is selected
## `row`
a `gint` representing a row in `self`

# Returns

a gboolean representing if the row is selected, or 0
if value does not implement this interface.
<!-- trait TableExt::fn is_selected -->
Gets a boolean value indicating whether the accessible object
at the specified `row` and `column` is selected
## `row`
a `gint` representing a row in `self`
## `column`
a `gint` representing a column in `self`

# Returns

a gboolean representing if the cell is selected, or 0
if value does not implement this interface.
<!-- trait TableExt::fn ref_at -->
Get a reference to the table cell at `row`, `column`. This cell
should implement the interface `TableCell`
## `row`
a `gint` representing a row in `self`
## `column`
a `gint` representing a column in `self`

# Returns

an `Object` representing the referred
to accessible
<!-- trait TableExt::fn remove_column_selection -->
Adds the specified `column` to the selection.
## `column`
a `gint` representing a column in `self`

# Returns

a gboolean representing if the column was successfully removed from
the selection, or 0 if value does not implement this interface.
<!-- trait TableExt::fn remove_row_selection -->
Removes the specified `row` from the selection.
## `row`
a `gint` representing a row in `self`

# Returns

a gboolean representing if the row was successfully removed from
the selection, or 0 if value does not implement this interface.
<!-- trait TableExt::fn set_caption -->
Sets the caption for the table.
## `caption`
a `Object` representing the caption to set for `self`
<!-- trait TableExt::fn set_column_description -->
Sets the description text for the specified `column` of the `self`.
## `column`
a `gint` representing a column in `self`
## `description`
a `gchar` representing the description text
to set for the specified `column` of the `self`
<!-- trait TableExt::fn set_column_header -->
Sets the specified column header to `header`.
## `column`
a `gint` representing a column in `self`
## `header`
an `Table`
<!-- trait TableExt::fn set_row_description -->
Sets the description text for the specified `row` of `self`.
## `row`
a `gint` representing a row in `self`
## `description`
a `gchar` representing the description text
to set for the specified `row` of `self`
<!-- trait TableExt::fn set_row_header -->
Sets the specified row header to `header`.
## `row`
a `gint` representing a row in `self`
## `header`
an `Table`
<!-- trait TableExt::fn set_summary -->
Sets the summary description of the table.
## `accessible`
an `Object` representing the summary description
to set for `self`
<!-- trait TableExt::fn connect_column_deleted -->
The "column-deleted" signal is emitted by an object which
implements the AtkTable interface when a column is deleted.
## `arg1`
The index of the first column deleted.
## `arg2`
The number of columns deleted.
<!-- trait TableExt::fn connect_column_inserted -->
The "column-inserted" signal is emitted by an object which
implements the AtkTable interface when a column is inserted.
## `arg1`
The index of the column inserted.
## `arg2`
The number of colums inserted.
<!-- trait TableExt::fn connect_column_reordered -->
The "column-reordered" signal is emitted by an object which
implements the AtkTable interface when the columns are
reordered.
<!-- trait TableExt::fn connect_model_changed -->
The "model-changed" signal is emitted by an object which
implements the AtkTable interface when the model displayed by
the table changes.
<!-- trait TableExt::fn connect_row_deleted -->
The "row-deleted" signal is emitted by an object which
implements the AtkTable interface when a row is deleted.
## `arg1`
The index of the first row deleted.
## `arg2`
The number of rows deleted.
<!-- trait TableExt::fn connect_row_inserted -->
The "row-inserted" signal is emitted by an object which
implements the AtkTable interface when a row is inserted.
## `arg1`
The index of the first row inserted.
## `arg2`
The number of rows inserted.
<!-- trait TableExt::fn connect_row_reordered -->
The "row-reordered" signal is emitted by an object which
implements the AtkTable interface when the rows are
reordered.
<!-- struct TableCell -->
Being `Table` a component which present elements ordered via rows
and columns, an `TableCell` is the interface which each of those
elements, so "cells" should implement.

See also `Table`.

# Implements

[`TableCellExt`](trait.TableCellExt.html), [`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TableCellExt -->
Trait containing all `TableCell` methods.

# Implementors

[`NoOpObject`](struct.NoOpObject.html), [`TableCell`](struct.TableCell.html)
<!-- trait TableCellExt::fn get_column_header_cells -->
Returns the column headers as an array of cell accessibles.

# Returns

a GPtrArray of AtkObjects
representing the column header cells.
<!-- trait TableCellExt::fn get_column_span -->
Returns the number of columns occupied by this cell accessible.

# Returns

a gint representing the number of columns occupied by this cell,
or 0 if the cell does not implement this method.
<!-- trait TableCellExt::fn get_position -->
Retrieves the tabular position of this cell.
## `row`
the row of the given cell.
## `column`
the column of the given cell.

# Returns

TRUE if successful; FALSE otherwise.
<!-- trait TableCellExt::fn get_row_column_span -->
Gets the row and column indexes and span of this cell accessible.

Note: If the object does not implement this function, then, by default, atk
will implement this function by calling get_row_span and get_column_span
on the object.
## `row`
the row index of the given cell.
## `column`
the column index of the given cell.
## `row_span`
the number of rows occupied by this cell.
## `column_span`
the number of columns occupied by this cell.

# Returns

TRUE if successful; FALSE otherwise.
<!-- trait TableCellExt::fn get_row_header_cells -->
Returns the row headers as an array of cell accessibles.

# Returns

a GPtrArray of AtkObjects
representing the row header cells.
<!-- trait TableCellExt::fn get_row_span -->
Returns the number of rows occupied by this cell accessible.

# Returns

a gint representing the number of rows occupied by this cell,
or 0 if the cell does not implement this method.
<!-- trait TableCellExt::fn get_table -->
Returns a reference to the accessible of the containing table.

# Returns

the atk object for the containing table.
<!-- struct Text -->
`Text` should be implemented by `AtkObjects` on behalf of widgets
that have text content which is either attributed or otherwise
non-trivial. `AtkObjects` whose text content is simple,
unattributed, and very brief may expose that content via
`AtkObjectExt::get_name` instead; however if the text is editable,
multi-line, typically longer than three or four words, attributed,
selectable, or if the object already uses the 'name' ATK property
for other information, the `Text` interface should be used to
expose the text content. In the case of editable text content,
`EditableText` (a subtype of the `Text` interface) should be
implemented instead.

 `Text` provides not only traversal facilities and change
notification for text content, but also caret tracking and glyph
bounding box calculations. Note that the text strings are exposed
as UTF-8, and are therefore potentially multi-byte, and
caret-to-byte offset mapping makes no assumptions about the
character length; also bounding box glyph-to-offset mapping may be
complex for languages which use ligatures.

# Implements

[`TextExt`](trait.TextExt.html)
<!-- trait TextExt -->
Trait containing all `Text` methods.

# Implementors

[`NoOpObject`](struct.NoOpObject.html), [`Text`](struct.Text.html)
<!-- impl Text::fn free_ranges -->
Frees the memory associated with an array of AtkTextRange. It is assumed
that the array was returned by the function atk_text_get_bounded_ranges
and is NULL terminated.
## `ranges`
A pointer to an array of `TextRange` which is
 to be freed.
<!-- trait TextExt::fn add_selection -->
Adds a selection bounded by the specified offsets.
## `start_offset`
the starting character offset of the selected region
## `end_offset`
the offset of the first character after the selected region.

# Returns

`true` if success, `false` otherwise
<!-- trait TextExt::fn get_bounded_ranges -->
Get the ranges of text in the specified bounding box.
## `rect`
An AtkTextRectangle giving the dimensions of the bounding box.
## `coord_type`
Specify whether coordinates are relative to the screen or widget window.
## `x_clip_type`
Specify the horizontal clip type.
## `y_clip_type`
Specify the vertical clip type.

# Returns

Array of AtkTextRange. The last
 element of the array returned by this function will be NULL.
<!-- trait TextExt::fn get_caret_offset -->
Gets the offset of the position of the caret (cursor).

# Returns

the character offset of position of the caret (cursor).
<!-- trait TextExt::fn get_character_at_offset -->
Gets the specified text.
## `offset`
a character offset within `self`

# Returns

the character at `offset`.
<!-- trait TextExt::fn get_character_count -->
Gets the character count.

# Returns

the number of characters.
<!-- trait TextExt::fn get_character_extents -->
Get the bounding box containing the glyph representing the character at
 a particular text offset.
## `offset`
The offset of the text character for which bounding information is required.
## `x`
Pointer for the x coordinate of the bounding box
## `y`
Pointer for the y coordinate of the bounding box
## `width`
Pointer for the width of the bounding box
## `height`
Pointer for the height of the bounding box
## `coords`
specify whether coordinates are relative to the screen or widget window
<!-- trait TextExt::fn get_default_attributes -->
Creates an `AttributeSet` which consists of the default values of
attributes for the text. See the enum AtkTextAttribute for types of text
attributes that can be returned. Note that other attributes may also be
returned.

# Returns

an `AttributeSet` which contains the default
values of attributes. at `offset`. this `atkattributeset` should be freed by
a call to `Attribute::set_free`.
<!-- trait TextExt::fn get_n_selections -->
Gets the number of selected regions.

# Returns

The number of selected regions, or -1 if a failure
 occurred.
<!-- trait TextExt::fn get_offset_at_point -->
Gets the offset of the character located at coordinates `x` and `y`. `x` and `y`
are interpreted as being relative to the screen or this widget's window
depending on `coords`.
## `x`
screen x-position of character
## `y`
screen y-position of character
## `coords`
specify whether coordinates are relative to the screen or
widget window

# Returns

the offset to the character which is located at
the specified `x` and `y` coordinates.
<!-- trait TextExt::fn get_range_extents -->
Get the bounding box for text within the specified range.
## `start_offset`
The offset of the first text character for which boundary
 information is required.
## `end_offset`
The offset of the text character after the last character
 for which boundary information is required.
## `coord_type`
Specify whether coordinates are relative to the screen or widget window.
## `rect`
A pointer to a AtkTextRectangle which is filled in by this function.
<!-- trait TextExt::fn get_run_attributes -->
Creates an `AttributeSet` which consists of the attributes explicitly
set at the position `offset` in the text. `start_offset` and `end_offset` are
set to the start and end of the range around `offset` where the attributes are
invariant. Note that `end_offset` is the offset of the first character
after the range. See the enum AtkTextAttribute for types of text
attributes that can be returned. Note that other attributes may also be
returned.
## `offset`
the character offset at which to get the attributes, -1 means the offset of
the character to be inserted at the caret location.
## `start_offset`
the address to put the start offset of the range
## `end_offset`
the address to put the end offset of the range

# Returns

an `AttributeSet` which contains the attributes
explicitly set at `offset`. This `AttributeSet` should be freed by a call
to `Attribute::set_free`.
<!-- trait TextExt::fn get_selection -->
Gets the text from the specified selection.
## `selection_num`
The selection number. The selected regions are
assigned numbers that correspond to how far the region is from the
start of the text. The selected region closest to the beginning
of the text region is assigned the number 0, etc. Note that adding,
moving or deleting a selected region can change the numbering.
## `start_offset`
passes back the starting character offset of the selected region
## `end_offset`
passes back the ending character offset (offset immediately past)
of the selected region

# Returns

a newly allocated string containing the selected text. Use `g_free`
 to free the returned string.
<!-- trait TextExt::fn get_string_at_offset -->
Gets a portion of the text exposed through an `Text` according to a given `offset`
and a specific `granularity`, along with the start and end offsets defining the
boundaries of such a portion of text.

If `granularity` is ATK_TEXT_GRANULARITY_CHAR the character at the
offset is returned.

If `granularity` is ATK_TEXT_GRANULARITY_WORD the returned string
is from the word start at or before the offset to the word start after
the offset.

The returned string will contain the word at the offset if the offset
is inside a word and will contain the word before the offset if the
offset is not inside a word.

If `granularity` is ATK_TEXT_GRANULARITY_SENTENCE the returned string
is from the sentence start at or before the offset to the sentence
start after the offset.

The returned string will contain the sentence at the offset if the offset
is inside a sentence and will contain the sentence before the offset
if the offset is not inside a sentence.

If `granularity` is ATK_TEXT_GRANULARITY_LINE the returned string
is from the line start at or before the offset to the line
start after the offset.

If `granularity` is ATK_TEXT_GRANULARITY_PARAGRAPH the returned string
is from the start of the paragraph at or before the offset to the start
of the following paragraph after the offset.
## `offset`
position
## `granularity`
An `TextGranularity`
## `start_offset`
the starting character offset of the returned string, or -1
 if an error has occurred (e.g. invalid offset, not implemented)
## `end_offset`
the offset of the first character after the returned string,
 or -1 if an error has occurred (e.g. invalid offset, not implemented)

# Returns

a newly allocated string containing the text
 at the `offset` bounded by the specified `granularity`. Use
 `g_free` to free the returned string. Returns `None` if the
 offset is invalid or no implementation is available.
<!-- trait TextExt::fn get_text -->
Gets the specified text.
## `start_offset`
a starting character offset within `self`
## `end_offset`
an ending character offset within `self`, or -1 for the end of the string.

# Returns

a newly allocated string containing the text from `start_offset` up
 to, but not including `end_offset`. Use `g_free` to free the returned string.
<!-- trait TextExt::fn get_text_after_offset -->
Gets the specified text.

# Deprecated since 2.9.3

Please use `Text::get_string_at_offset` instead.
## `offset`
position
## `boundary_type`
An `TextBoundary`
## `start_offset`
the starting character offset of the returned string
## `end_offset`
the offset of the first character after the
 returned substring

# Returns

a newly allocated string containing the text after `offset` bounded
 by the specified `boundary_type`. Use `g_free` to free the returned string.
<!-- trait TextExt::fn get_text_at_offset -->
Gets the specified text.

If the boundary_type if ATK_TEXT_BOUNDARY_CHAR the character at the
offset is returned.

If the boundary_type is ATK_TEXT_BOUNDARY_WORD_START the returned string
is from the word start at or before the offset to the word start after
the offset.

The returned string will contain the word at the offset if the offset
is inside a word and will contain the word before the offset if the
offset is not inside a word.

If the boundary type is ATK_TEXT_BOUNDARY_SENTENCE_START the returned
string is from the sentence start at or before the offset to the sentence
start after the offset.

The returned string will contain the sentence at the offset if the offset
is inside a sentence and will contain the sentence before the offset
if the offset is not inside a sentence.

If the boundary type is ATK_TEXT_BOUNDARY_LINE_START the returned
string is from the line start at or before the offset to the line
start after the offset.

# Deprecated

This method is deprecated since ATK version
2.9.4. Please use `Text::get_string_at_offset` instead.
## `offset`
position
## `boundary_type`
An `TextBoundary`
## `start_offset`
the starting character offset of the returned string
## `end_offset`
the offset of the first character after the
 returned substring

# Returns

a newly allocated string containing the text at `offset` bounded by
 the specified `boundary_type`. Use `g_free` to free the returned string.
<!-- trait TextExt::fn get_text_before_offset -->
Gets the specified text.

# Deprecated since 2.9.3

Please use `Text::get_string_at_offset` instead.
## `offset`
position
## `boundary_type`
An `TextBoundary`
## `start_offset`
the starting character offset of the returned string
## `end_offset`
the offset of the first character after the
 returned substring

# Returns

a newly allocated string containing the text before `offset` bounded
 by the specified `boundary_type`. Use `g_free` to free the returned string.
<!-- trait TextExt::fn remove_selection -->
Removes the specified selection.
## `selection_num`
The selection number. The selected regions are
assigned numbers that correspond to how far the region is from the
start of the text. The selected region closest to the beginning
of the text region is assigned the number 0, etc. Note that adding,
moving or deleting a selected region can change the numbering.

# Returns

`true` if success, `false` otherwise
<!-- trait TextExt::fn set_caret_offset -->
Sets the caret (cursor) position to the specified `offset`.
## `offset`
the character offset of the new caret position

# Returns

`true` if success, `false` otherwise.
<!-- trait TextExt::fn set_selection -->
Changes the start and end offset of the specified selection.
## `selection_num`
The selection number. The selected regions are
assigned numbers that correspond to how far the region is from the
start of the text. The selected region closest to the beginning
of the text region is assigned the number 0, etc. Note that adding,
moving or deleting a selected region can change the numbering.
## `start_offset`
the new starting character offset of the selection
## `end_offset`
the new end position of (e.g. offset immediately past)
the selection

# Returns

`true` if success, `false` otherwise
<!-- trait TextExt::fn connect_text_attributes_changed -->
The "text-attributes-changed" signal is emitted when the text
attributes of the text of an object which implements AtkText
changes.
<!-- trait TextExt::fn connect_text_caret_moved -->
The "text-caret-moved" signal is emitted when the caret
position of the text of an object which implements AtkText
changes.
## `arg1`
The new position of the text caret.
<!-- trait TextExt::fn connect_text_changed -->
The "text-changed" signal is emitted when the text of the
object which implements the AtkText interface changes, This
signal will have a detail which is either "insert" or
"delete" which identifies whether the text change was an
insertion or a deletion.

# Deprecated since 2.9.4

Use `Object::text-insert` or
`Object::text-remove` instead.
## `arg1`
The position (character offset) of the insertion or deletion.
## `arg2`
The length (in characters) of text inserted or deleted.
<!-- trait TextExt::fn connect_text_insert -->
The "text-insert" signal is emitted when a new text is
inserted. If the signal was not triggered by the user
(e.g. typing or pasting text), the "system" detail should be
included.
## `arg1`
The position (character offset) of the insertion.
## `arg2`
The length (in characters) of text inserted.
## `arg3`
The new text inserted
<!-- trait TextExt::fn connect_text_remove -->
The "text-remove" signal is emitted when a new text is
removed. If the signal was not triggered by the user
(e.g. typing or pasting text), the "system" detail should be
included.
## `arg1`
The position (character offset) of the removal.
## `arg2`
The length (in characters) of text removed.
## `arg3`
The old text removed
<!-- trait TextExt::fn connect_text_selection_changed -->
The "text-selection-changed" signal is emitted when the
selected text of an object which implements AtkText changes.
<!-- enum TextAttribute -->
Describes the text attributes supported
<!-- enum TextAttribute::variant Invalid -->
Invalid attribute, like bad spelling or grammar.
<!-- enum TextAttribute::variant LeftMargin -->
The pixel width of the left margin
<!-- enum TextAttribute::variant RightMargin -->
The pixel width of the right margin
<!-- enum TextAttribute::variant Indent -->
The number of pixels that the text is indented
<!-- enum TextAttribute::variant Invisible -->
Either "true" or "false" indicating whether text is visible or not
<!-- enum TextAttribute::variant Editable -->
Either "true" or "false" indicating whether text is editable or not
<!-- enum TextAttribute::variant PixelsAboveLines -->
Pixels of blank space to leave above each newline-terminated line.
<!-- enum TextAttribute::variant PixelsBelowLines -->
Pixels of blank space to leave below each newline-terminated line.
<!-- enum TextAttribute::variant PixelsInsideWrap -->
Pixels of blank space to leave between wrapped lines inside the same newline-terminated line (paragraph).
<!-- enum TextAttribute::variant BgFullHeight -->
"true" or "false" whether to make the background color for each character the height of the highest font used on the current line, or the height of the font used for the current character.
<!-- enum TextAttribute::variant Rise -->
Number of pixels that the characters are risen above the baseline
<!-- enum TextAttribute::variant Underline -->
"none", "single", "double", "low", or "error"
<!-- enum TextAttribute::variant Strikethrough -->
"true" or "false" whether the text is strikethrough
<!-- enum TextAttribute::variant Size -->
The size of the characters in points. eg: 10
<!-- enum TextAttribute::variant Scale -->
The scale of the characters. The value is a string representation of a double
<!-- enum TextAttribute::variant Weight -->
The weight of the characters.
<!-- enum TextAttribute::variant Language -->
The language used
<!-- enum TextAttribute::variant FamilyName -->
The font family name
<!-- enum TextAttribute::variant BgColor -->
The background color. The value is an RGB value of the format "`u`,`u`,`u`"
<!-- enum TextAttribute::variant FgColor -->
The foreground color. The value is an RGB value of the format "`u`,`u`,`u`"
<!-- enum TextAttribute::variant BgStipple -->
"true" if a ``GdkBitmap`` is set for stippling the background color.
<!-- enum TextAttribute::variant FgStipple -->
"true" if a ``GdkBitmap`` is set for stippling the foreground color.
<!-- enum TextAttribute::variant WrapMode -->
The wrap mode of the text, if any. Values are "none", "char", "word", or "word_char".
<!-- enum TextAttribute::variant Direction -->
The direction of the text, if set. Values are "none", "ltr" or "rtl"
<!-- enum TextAttribute::variant Justification -->
The justification of the text, if set. Values are "left", "right", "center" or "fill"
<!-- enum TextAttribute::variant Stretch -->
The stretch of the text, if set. Values are "ultra_condensed", "extra_condensed", "condensed", "semi_condensed", "normal", "semi_expanded", "expanded", "extra_expanded" or "ultra_expanded"
<!-- enum TextAttribute::variant Variant -->
The capitalization variant of the text, if set. Values are "normal" or "small_caps"
<!-- enum TextAttribute::variant Style -->
The slant style of the text, if set. Values are "normal", "oblique" or "italic"
<!-- enum TextAttribute::variant LastDefined -->
not a valid text attribute, used for finding end of enumeration
<!-- enum TextBoundary -->
Text boundary types used for specifying boundaries for regions of text.
This enumeration is deprecated since 2.9.4 and should not be used. Use
AtkTextGranularity with `Text::get_string_at_offset` instead.
<!-- enum TextBoundary::variant Char -->
Boundary is the boundary between characters
(including non-printing characters)
<!-- enum TextBoundary::variant WordStart -->
Boundary is the start (i.e. first character) of a word.
<!-- enum TextBoundary::variant WordEnd -->
Boundary is the end (i.e. last
character) of a word.
<!-- enum TextBoundary::variant SentenceStart -->
Boundary is the first character in a sentence.
<!-- enum TextBoundary::variant SentenceEnd -->
Boundary is the last (terminal)
character in a sentence; in languages which use "sentence stop"
punctuation such as English, the boundary is thus the '.', '?', or
similar terminal punctuation character.
<!-- enum TextBoundary::variant LineStart -->
Boundary is the initial character of the content or a
character immediately following a newline, linefeed, or return character.
<!-- enum TextBoundary::variant LineEnd -->
Boundary is the linefeed, or return
character.
<!-- enum TextClipType -->
Describes the type of clipping required.
<!-- enum TextClipType::variant None -->
No clipping to be done
<!-- enum TextClipType::variant Min -->
Text clipped by min coordinate is omitted
<!-- enum TextClipType::variant Max -->
Text clipped by max coordinate is omitted
<!-- enum TextClipType::variant Both -->
Only text fully within mix/max bound is retained
<!-- enum TextGranularity -->
Text granularity types used for specifying the granularity of the region of
text we are interested in.
<!-- enum TextGranularity::variant Char -->
Granularity is defined by the boundaries between characters
(including non-printing characters)
<!-- enum TextGranularity::variant Word -->
Granularity is defined by the boundaries of a word,
starting at the beginning of the current word and finishing at the beginning of
the following one, if present.
<!-- enum TextGranularity::variant Sentence -->
Granularity is defined by the boundaries of a sentence,
starting at the beginning of the current sentence and finishing at the beginning of
the following one, if present.
<!-- enum TextGranularity::variant Line -->
Granularity is defined by the boundaries of a line,
starting at the beginning of the current line and finishing at the beginning of
the following one, if present.
<!-- enum TextGranularity::variant Paragraph -->
Granularity is defined by the boundaries of a paragraph,
starting at the beginning of the current paragraph and finishing at the beginning of
the following one, if present.
<!-- struct TextRange -->
A structure used to describe a text range.
<!-- struct TextRectangle -->
A structure used to store a rectangle used by AtkText.
<!-- struct Util -->
A set of ATK utility functions which are used to support event
registration of various types, and obtaining the 'root' accessible
of a process and information about the current ATK implementation
and toolkit version.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- struct Value -->
`Value` should be implemented for components which either display
a value from a bounded range, or which allow the user to specify a
value from a bounded range, or both. For instance, most sliders and
range controls, as well as dials, should have `Object`
representations which implement `Value` on the component's
behalf. `AtKValues` may be read-only, in which case attempts to
alter the value return would fail.

<refsect1 id="current-value-text">
`<title>`On the subject of current value text`</title>`
`<para>`
In addition to providing the current value, implementors can
optionally provide an end-user-consumable textual description
associated with this value. This description should be included
when the numeric value fails to convey the full, on-screen
representation seen by users.
`</para>`

`<example>`
`<title>`Password strength`</title>`
A password strength meter whose value changes as the user types
their new password. Red is used for values less than 4.0, yellow
for values between 4.0 and 7.0, and green for values greater than
7.0. In this instance, value text should be provided by the
implementor. Appropriate value text would be "weak", "acceptable,"
and "strong" respectively.
`</example>`

A level bar whose value changes to reflect the battery charge. The
color remains the same regardless of the charge and there is no
on-screen text reflecting the fullness of the battery. In this
case, because the position within the bar is the only indication
the user has of the current charge, value text should not be
provided by the implementor.

<refsect2 id="implementor-notes">
`<title>`Implementor Notes`</title>`
`<para>`
Implementors should bear in mind that assistive technologies will
likely prefer the value text provided over the numeric value when
presenting a widget's value. As a result, strings not intended for
end users should not be exposed in the value text, and strings
which are exposed should be localized. In the case of widgets which
display value text on screen, for instance through a separate label
in close proximity to the value-displaying widget, it is still
expected that implementors will expose the value text using the
above API.
`</para>`

`<para>`
`Value` should NOT be implemented for widgets whose displayed
value is not reflective of a meaningful amount. For instance, a
progress pulse indicator whose value alternates between 0.0 and 1.0
to indicate that some process is still taking place should not
implement `Value` because the current value does not reflect
progress towards completion.
`</para>`
`</refsect2>`
`</refsect1>`

<refsect1 id="ranges">
`<title>`On the subject of ranges`</title>`
`<para>`
In addition to providing the minimum and maximum values,
implementors can optionally provide details about subranges
associated with the widget. These details should be provided by the
implementor when both of the following are communicated visually to
the end user:
`</para>`
`<itemizedlist>`
 `<listitem>`The existence of distinct ranges such as "weak",
 "acceptable", and "strong" indicated by color, bar tick marks,
 and/or on-screen text.`</listitem>`
 `<listitem>`Where the current value stands within a given subrange,
 for instance illustrating progression from very "weak" towards
 nearly "acceptable" through changes in shade and/or position on
 the bar within the "weak" subrange.`</listitem>`
`</itemizedlist>`
`<para>`
If both of the above do not apply to the widget, it should be
sufficient to expose the numeric value, along with the value text
if appropriate, to make the widget accessible.
`</para>`

<refsect2 id="ranges-implementor-notes">
`<title>`Implementor Notes`</title>`
`<para>`
If providing subrange details is deemed necessary, all possible
values of the widget are expected to fall within one of the
subranges defined by the implementor.
`</para>`
`</refsect2>`
`</refsect1>`

<refsect1 id="localization">
`<title>`On the subject of localization of end-user-consumable text
values`</title>`
`<para>`
Because value text and subrange descriptors are human-consumable,
implementors are expected to provide localized strings which can be
directly presented to end users via their assistive technology. In
order to simplify this for implementors, implementors can use
`ValueType::get_localized_name` with the following
already-localized constants for commonly-needed values can be used:
`</para>`

`<itemizedlist>`
 `<listitem>`ATK_VALUE_VERY_WEAK`</listitem>`
 `<listitem>`ATK_VALUE_WEAK`</listitem>`
 `<listitem>`ATK_VALUE_ACCEPTABLE`</listitem>`
 `<listitem>`ATK_VALUE_STRONG`</listitem>`
 `<listitem>`ATK_VALUE_VERY_STRONG`</listitem>`
 `<listitem>`ATK_VALUE_VERY_LOW`</listitem>`
 `<listitem>`ATK_VALUE_LOW`</listitem>`
 `<listitem>`ATK_VALUE_MEDIUM`</listitem>`
 `<listitem>`ATK_VALUE_HIGH`</listitem>`
 `<listitem>`ATK_VALUE_VERY_HIGH`</listitem>`
 `<listitem>`ATK_VALUE_VERY_BAD`</listitem>`
 `<listitem>`ATK_VALUE_BAD`</listitem>`
 `<listitem>`ATK_VALUE_GOOD`</listitem>`
 `<listitem>`ATK_VALUE_VERY_GOOD`</listitem>`
 `<listitem>`ATK_VALUE_BEST`</listitem>`
 `<listitem>`ATK_VALUE_SUBSUBOPTIMAL`</listitem>`
 `<listitem>`ATK_VALUE_SUBOPTIMAL`</listitem>`
 `<listitem>`ATK_VALUE_OPTIMAL`</listitem>`
`</itemizedlist>`
`<para>`
Proposals for additional constants, along with their use cases,
should be submitted to the GNOME Accessibility Team.
`</para>`
`</refsect1>`

<refsect1 id="changes">
`<title>`On the subject of changes`</title>`
`<para>`
Note that if there is a textual description associated with the new
numeric value, that description should be included regardless of
whether or not it has also changed.
`</para>`
`</refsect1>`

# Implements

[`ValueExt`](trait.ValueExt.html)
<!-- trait ValueExt -->
Trait containing all `Value` methods.

# Implementors

[`NoOpObject`](struct.NoOpObject.html), [`Value`](struct.Value.html)
<!-- trait ValueExt::fn get_current_value -->
Gets the value of this object.

# Deprecated

Since 2.12. Use `Value::get_value_and_text`
instead.
## `value`
a `gobject::Value` representing the current accessible value
<!-- trait ValueExt::fn get_increment -->
Gets the minimum increment by which the value of this object may be
changed. If zero, the minimum increment is undefined, which may
mean that it is limited only by the floating point precision of the
platform.

# Returns

the minimum increment by which the value of this
object may be changed. zero if undefined.
<!-- trait ValueExt::fn get_maximum_value -->
Gets the maximum value of this object.

# Deprecated

Since 2.12. Use `Value::get_range` instead.
## `value`
a `gobject::Value` representing the maximum accessible value
<!-- trait ValueExt::fn get_minimum_increment -->
Gets the minimum increment by which the value of this object may be changed. If zero,
the minimum increment is undefined, which may mean that it is limited only by the
floating point precision of the platform.

# Deprecated

Since 2.12. Use `Value::get_increment` instead.
## `value`
a `gobject::Value` representing the minimum increment by which the accessible value may be changed
<!-- trait ValueExt::fn get_minimum_value -->
Gets the minimum value of this object.

# Deprecated

Since 2.12. Use `Value::get_range` instead.
## `value`
a `gobject::Value` representing the minimum accessible value
<!-- trait ValueExt::fn get_range -->
Gets the range of this object.

# Returns

a newly allocated `Range`
that represents the minimum, maximum and descriptor (if available)
of `self`. NULL if that range is not defined.
<!-- trait ValueExt::fn get_sub_ranges -->
Gets the list of subranges defined for this object. See `Value`
introduction for examples of subranges and when to expose them.

# Returns

an `glib::SList` of
`Range` which each of the subranges defined for this object. Free
the returns list with `glib::SList::free`.
<!-- trait ValueExt::fn get_value_and_text -->
Gets the current value and the human readable text alternative of
`self`. `text` is a newly created string, that must be freed by the
caller. Can be NULL if no descriptor is available.
## `value`
address of `gdouble` to put the current value of `self`
## `text`
address of `gchar` to put the human
readable text alternative for `value`
<!-- trait ValueExt::fn set_current_value -->
Sets the value of this object.

# Deprecated

Since 2.12. Use `Value::set_value` instead.
## `value`
a `gobject::Value` which is the desired new accessible value.

# Returns

`true` if new value is successfully set, `false` otherwise.
<!-- trait ValueExt::fn set_value -->
Sets the value of this object.

This method is intended to provide a way to change the value of the
object. In any case, it is possible that the value can't be
modified (ie: a read-only component). If the value changes due this
call, it is possible that the text could change, and will trigger
an `Value::value-changed` signal emission.

Note for implementors: the deprecated `Value::set_current_value`
method returned TRUE or FALSE depending if the value was assigned
or not. In the practice several implementors were not able to
decide it, and returned TRUE in any case. For that reason it is not
required anymore to return if the value was properly assigned or
not.
## `new_value`
a double which is the desired new accessible value.
<!-- trait ValueExt::fn connect_value_changed -->
The 'value-changed' signal is emitted when the current value
that represent the object changes. `value` is the numerical
representation of this new value. `text` is the human
readable text alternative of `value`, and can be NULL if it is
not available. Note that if there is a textual description
associated with the new numeric value, that description
should be included regardless of whether or not it has also
changed.

Example: a password meter whose value changes as the user
types their new password. Appropiate value text would be
"weak", "acceptable" and "strong".
## `value`
the new value in a numerical form.
## `text`
human readable text alternative (also called
description) of this object. NULL if not available.
<!-- enum ValueType -->
Default types for a given value. Those are defined in order to
easily get localized strings to describe a given value or a given
subrange, using `ValueType::get_localized_name`.
<!-- struct Window -->
`Window` should be implemented by the UI elements that represent
a top-level window, such as the main window of an application or
dialog.

# Implements

[`AtkWindowExt`](trait.AtkWindowExt.html), [`AtkObjectExt`](trait.AtkObjectExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AtkWindowExt -->
Trait containing all `Window` methods.

# Implementors

[`NoOpObject`](struct.NoOpObject.html), [`Window`](struct.Window.html)
<!-- trait AtkWindowExt::fn connect_activate -->
The signal `Window::activate` is emitted when a window
becomes the active window of the application or session.
<!-- trait AtkWindowExt::fn connect_create -->
The signal `Window::create` is emitted when a new window
is created.
<!-- trait AtkWindowExt::fn connect_deactivate -->
The signal `Window::deactivate` is emitted when a window is
no longer the active window of the application or session.
<!-- trait AtkWindowExt::fn connect_destroy -->
The signal `Window::destroy` is emitted when a window is
destroyed.
<!-- trait AtkWindowExt::fn connect_maximize -->
The signal `Window::maximize` is emitted when a window
is maximized.
<!-- trait AtkWindowExt::fn connect_minimize -->
The signal `Window::minimize` is emitted when a window
is minimized.
<!-- trait AtkWindowExt::fn connect_move -->
The signal `Window::move` is emitted when a window
is moved.
<!-- trait AtkWindowExt::fn connect_resize -->
The signal `Window::resize` is emitted when a window
is resized.
<!-- trait AtkWindowExt::fn connect_restore -->
The signal `Window::restore` is emitted when a window
is restored.
