<!-- file * -->
<!-- struct TextBuffer -->
You may wish to begin by reading the
[text widget conceptual overview](https://developer.gnome.org/gtk3/stable/TextWidget.html)
which gives an overview of all the objects and data
types related to the text widget and how they work together.
<!-- impl TextBuffer::fn insert_markup -->
Inserts the text in `markup` at position `iter`. `markup` will be inserted
in its entirety. Emits the `TextBuffer::insert-text` signal, possibly
multiple times; insertion actually occurs in the default handler for the
signal. `iter` will point to the end of the inserted text on return.

Since: 3.16

## `iter`
location to insert the markup
## `markup`
a string containing [Pango markup](https://developer.gnome.org/pygtk/stable/pango-markup-language.html)
<!-- struct TextIter -->
You may wish to begin by reading the
[text widget conceptual overview](https://developer.gnome.org/gtk3/stable/TextWidget.html)
which gives an overview of all the objects and data
types related to the text widget and how they work together.
<!-- struct Container -->
A GTK+ user interface is constructed by nesting widgets inside widgets.
Container widgets are the inner nodes in the resulting tree of widgets:
they contain other widgets. So, for example, you might have a `Window`
containing a `Frame` containing a `Label`. If you wanted an image instead
of a textual label inside the frame, you might replace the `Label` widget
with a `Image` widget.

There are two major kinds of container widgets in GTK+. Both are subclasses
of the abstract `Container` base class.

The first type of container widget has a single child widget and derives
from `Bin`. These containers are decorators, which
add some kind of functionality to the child. For example, a `Button` makes
its child into a clickable button; a `Frame` draws a frame around its child
and a `Window` places its child widget inside a top-level window.

The second type of container can have more than one child; its purpose is to
manage layout. This means that these containers assign
sizes and positions to their children. For example, a `HBox` arranges its
children in a horizontal row, and a `Grid` arranges the widgets it contains
in a two-dimensional grid.

# Child properties

`Container` introduces child properties.
These are object properties that are not specific
to either the container or the contained widget, but rather to their relation.
Typical examples of child properties are the position or pack-type of a widget
which is contained in a `Box`.

# `Container` as `Buildable`

The `Container` implementation of the `Buildable` interface supports
a `<packing>` element for children, which can contain multiple `<property>`
elements that specify child properties for the child.

Since 2.16, child properties can also be marked as translatable using
the same “translatable”, “comments” and “context” attributes that are used
for regular properties.

Since 3.16, containers can have a `<focus-chain>` element containing multiple
`<widget>` elements, one for each child that should be added to the focus
chain. The ”name” attribute gives the id of the widget.

An example of these properties in UI definitions:

```xml
<object class="GtkBox">
  <child>
    <object class="GtkEntry" id="entry1"/>
    <packing>
      <property name="pack-type">start</property>
    </packing>
  </child>
  <child>
    <object class="GtkEntry" id="entry2"/>
  </child>
  <focus-chain>
    <widget name="entry1"/>
    <widget name="entry2"/>
  </focus-chain>
</object>
```

# Implements

[`ContainerExt`](trait.ContainerExt.html), [`WidgetExt`](trait.WidgetExt.html)
<!-- trait ContainerExt::fn child_notify -->
Emits a `Widget::child-notify` signal for the
[child property](struct.Container.html#child-properties)
`child_property` on widget.

This is an analogue of `gobject::Object::notify` for child properties.

Also see `Widget::child_notify`.
## `child`
the child widget
## `child_property`
the name of a child property installed on
 the class of `self`
<!-- impl Expander::fn get_use_markup -->
Returns whether the label’s text is interpreted as marked up with
the [Pango text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).
See `Expander::set_use_markup`.

# Returns

`true` if the label’s text will be parsed for markup
<!-- impl Expander::fn set_use_markup -->
Sets whether the text of the label contains markup in
[Pango’s text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).
See `Label::set_markup`.
## `use_markup`
`true` if the label’s text should be parsed for markup
<!-- struct Label -->
The `Label` widget displays a small amount of text. As the name
implies, most labels are used to label another widget such as a
`Button`, a `MenuItem`, or a `ComboBox`.

# `Label` as `Buildable`

The `Label` implementation of the `Buildable` interface supports a
custom `<attributes>` element, which supports any number of `<attribute>`
elements. The `<attribute>` element has attributes named “name“, “value“,
“start“ and “end“ and allows you to specify `pango::Attribute` values for
this label.

An example of a UI definition fragment specifying Pango attributes:

```no_run
    use gtk::{Button, Label};
    use gtk::prelude::*;

    // Pressing Alt+H will activate this button
    let button = Button::new();
    let label = Label::new_with_mnemonic(Some("_Hello"));
    button.add(&label);
```

There’s a convenience function to create buttons with a mnemonic label
already inside:


```no_run
    use gtk::Button;

    // Pressing Alt+H will activate this button
    let button = Button::new_with_mnemonic("_Hello");
```

To create a mnemonic for a widget alongside the label, such as a
`Entry`, you have to point the label at the entry with
`Label::set_mnemonic_widget`:


```no_run
    use gtk::{Entry, Label};
    use gtk::prelude::*;

    // Pressing Alt+H will focus the entry
    let entry = Entry::new();
    let label = Label::new_with_mnemonic(Some("_Hello"));
    label.set_mnemonic_widget(Some(&entry));
```

# Markup (styled text)

To make it easy to format text in a label (changing colors,
fonts, etc.), label text can be provided in a simple
[markup format](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).

Here’s how to create a label with a small font:

```no_run
    use gtk::Label;

    let label = Label::new(None);
    label.set_markup("<small>Small text</small>");
```

(See [complete documentation](https://developer.gnome.org/pygtk/stable/pango-markup-language.html) of available
tags in the Pango manual.)

The markup passed to `Label::set_markup` must be valid; for example,
literal <, > and & characters must be escaped as `&lt;`, `&gt;`, and `&amp;`.
If you pass text obtained from the user, file, or a network to
`Label::set_markup`, you’ll want to escape it with
`g_markup_escape_text` or `g_markup_printf_escaped`.

Markup strings are just a convenient way to set the `pango::AttrList` on
a label; `Label::set_attributes` may be a simpler way to set
attributes in some cases. Be careful though; `pango::AttrList` tends to
cause internationalization problems, unless you’re applying attributes
to the entire string (i.e. unless you set the range of each attribute
to [0, `G_MAXINT`)). The reason is that specifying the start_index and
end_index for a `pango::Attribute` requires knowledge of the exact string
being displayed, so translations will cause problems.

# Selectable labels

Labels can be made selectable with `Label::set_selectable`.
Selectable labels allow the user to copy the label contents to
the clipboard. Only labels that contain useful-to-copy information
— such as error messages — should be made selectable.

# Text layout # {`label`-text-layout}

A label can contain any number of paragraphs, but will have
performance problems if it contains more than a small number.
Paragraphs are separated by newlines or other paragraph separators
understood by Pango.

Labels can automatically wrap text if you call
`Label::set_line_wrap`.

`Label::set_justify` sets how the lines in a label align
with one another. If you want to set how the label as a whole
aligns in its available space, see the `Widget::halign` and
`Widget:valign` properties.

The `Label:width-chars` and `Label:max-width-chars` properties
can be used to control the size allocation of ellipsized or wrapped
labels. For ellipsizing labels, if either is specified (and less
than the actual text size), it is used as the minimum width, and the actual
text size is used as the natural width of the label. For wrapping labels,
width-chars is used as the minimum width, if specified, and max-width-chars
is used as the natural width. Even if max-width-chars specified, wrapping
labels will be rewrapped to use all of the available width.

Note that the interpretation of `Label:width-chars` and
`Label:max-width-chars` has changed a bit with the introduction of
[width-for-height geometry management.](geometry-management)

# Links

Since 2.18, GTK+ supports markup for clickable hyperlinks in addition
to regular Pango markup. The markup for links is borrowed from HTML,
using the `<a>` with “href“ and “title“ attributes. GTK+ renders links
similar to the way they appear in web browsers, with colored, underlined
text. The “title“ attribute is displayed as a tooltip on the link.

An example looks like this:

```no_run
    use gtk::Label;

    let label = Label::new(None);
    let text =
r#"Go to the
 <a href="http://www.gtk.org" title="&lt;i&gt;Our&lt;/i&gt; website">
 GTK+ website</a> for more..."#;
    label.set_markup(text);
```

It is possible to implement custom handling for links and their tooltips with
the `Label::activate-link` signal and the `Label::get_current_uri` function.

# Implements

[`MiscExt`](trait.MiscExt.html), [`WidgetExt`](trait.WidgetExt.html)
<!-- impl Label::fn get_use_markup -->
Returns whether the label’s text is interpreted as marked up with
the [Pango text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).
See `Label::set_use_markup`.

# Returns

`true` if the label’s text will be parsed for markup.
<!-- impl Label::fn set_markup -->
Parses `str` which is marked up with the
[Pango text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html), setting the
label’s text and attribute list based on the parse results. If the `str` is
external data, you may need to escape it with `g_markup_escape_text` or
`g_markup_printf_escaped`:

```C
const char *format = "<span style=\"italic\">\%s</span>";
char *markup;

markup = g_markup_printf_escaped (format, str);
gtk_label_set_markup (GTK_LABEL (label), markup);
g_free (markup);
```
## `str`
a markup string (see [Pango markup format](https://developer.gnome.org/pygtk/stable/pango-markup-language.html))
<!-- impl Label::fn set_markup_with_mnemonic -->
Parses `str` which is marked up with the
[Pango text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html),
setting the label’s text and attribute list based on the parse results.
If characters in `str` are preceded by an underscore, they are underlined
indicating that they represent a keyboard accelerator called a mnemonic.

The mnemonic key can be used to activate another widget, chosen
automatically, or explicitly using `Label::set_mnemonic_widget`.
## `str`
a markup string (see
 [Pango markup format](https://developer.gnome.org/pygtk/stable/pango-markup-language.html))
<!-- impl Label::fn set_use_markup -->
Sets whether the text of the label contains markup in
[Pango’s text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).
See `Label::set_markup`.
## `setting`
`true` if the label’s text should be parsed for markup.
<!-- impl Scale::fn add_mark -->
Adds a mark at `value`.

A mark is indicated visually by drawing a tick mark next to the scale,
and GTK+ makes it easy for the user to position the scale exactly at the
marks value.

If `markup` is not `None`, text is shown next to the tick mark.

To remove marks from a scale, use `Scale::clear_marks`.
## `value`
the value at which the mark is placed, must be between
 the lower and upper limits of the scales’ adjustment
## `position`
where to draw the mark. For a horizontal scale, `PositionType::Top`
 and `PositionType::Left` are drawn above the scale, anything else below.
 For a vertical scale, `PositionType::Left` and `PositionType::Top` are drawn to
 the left of the scale, anything else to the right.
## `markup`
Text to be shown at the mark, using [Pango markup](https://developer.gnome.org/pygtk/stable/pango-markup-language.html), or `None`
<!-- impl StatusIcon::fn set_tooltip_markup -->
Sets `markup` as the contents of the tooltip, which is marked up with
 the [Pango text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).

This function will take care of setting `StatusIcon:has-tooltip` to `true`
and of the default handler for the `StatusIcon::query-tooltip` signal.

See also the `StatusIcon:tooltip-markup` property and
`Tooltip::set_markup`.

# Deprecated since 3.14

Use notifications
## `markup`
the contents of the tooltip for `self`, or `None`
<!-- trait WidgetExt::fn set_tooltip_markup -->
Sets `markup` as the contents of the tooltip, which is marked up with
 the [Pango text markup language](https://developer.gnome.org/pygtk/stable/pango-markup-language.html).

This function will take care of setting `Widget:has-tooltip` to `true`
and of the default handler for the `Widget::query-tooltip` signal.

See also the `Widget:tooltip-markup` property and
`Tooltip::set_markup`.
## `markup`
the contents of the tooltip for `self`, or `None`
