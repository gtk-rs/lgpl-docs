<!-- file * -->
<!-- enum BackgroundPatternType -->
<!-- enum BackgroundPatternType::variant None -->
no pattern
<!-- enum BackgroundPatternType::variant Grid -->
grid pattern

Feature: `v3_16`

<!-- struct Buffer -->


# Implements

[`BufferExt`](trait.BufferExt.html), [`TextBufferExt`](trait.TextBufferExt.html)
<!-- trait BufferExt -->
Trait containing all `Buffer` methods.

# Implementors

[`Buffer`](struct.Buffer.html)
<!-- impl Buffer::fn new -->
Creates a new source buffer.
## `table`
a `gtk::TextTagTable`, or `None` to create a new one.

# Returns

a new source buffer.
<!-- impl Buffer::fn new_with_language -->
Creates a new source buffer using the highlighting patterns in
`language`. This is equivalent to creating a new source buffer with
a new tag table and then calling `BufferExt::set_language`.
## `language`
a `Language`.

# Returns

a new source buffer which will highlight text
according to the highlighting patterns in `language`.
<!-- trait BufferExt::fn backward_iter_to_source_mark -->
Moves `iter` to the position of the previous `Mark` of the given
category. Returns `true` if `iter` was moved. If `category` is NULL, the
previous source mark can be of any category.

Feature: `v2_2`

## `iter`
an iterator.
## `category`
category to search for, or `None`

# Returns

whether `iter` was moved.
<!-- trait BufferExt::fn begin_not_undoable_action -->
Marks the beginning of a not undoable action on the buffer,
disabling the undo manager. Typically you would call this function
before initially setting the contents of the buffer (e.g. when
loading a file in a text editor).

You may nest `BufferExt::begin_not_undoable_action` /
`BufferExt::end_not_undoable_action` blocks.
<!-- trait BufferExt::fn can_redo -->
Determines whether a source buffer can redo the last action
(i.e. if the last operation was an undo).

# Returns

`true` if a redo is possible.
<!-- trait BufferExt::fn can_undo -->
Determines whether a source buffer can undo the last action.

# Returns

`true` if it's possible to undo the last action.
<!-- trait BufferExt::fn change_case -->
Changes the case of the text between the specified iterators.

Feature: `v3_12`

## `case_type`
how to change the case.
## `start`
a `gtk::TextIter`.
## `end`
a `gtk::TextIter`.
<!-- trait BufferExt::fn create_source_mark -->
Creates a source mark in the `self` of category `category`. A source mark is
a `gtk::TextMark` but organised into categories. Depending on the category
a pixbuf can be specified that will be displayed along the line of the mark.

Like a `gtk::TextMark`, a `Mark` can be anonymous if the
passed `name` is `None`. Also, the buffer owns the marks so you
shouldn't unreference it.

Marks always have left gravity and are moved to the beginning of
the line when the user deletes the line they were in.

Typical uses for a source mark are bookmarks, breakpoints, current
executing instruction indication in a source file, etc..

Feature: `v2_2`

## `name`
the name of the mark, or `None`.
## `category`
a string defining the mark category.
## `where_`
location to place the mark.

# Returns

a new `Mark`, owned by the buffer.
<!-- trait BufferExt::fn create_source_tag -->
In short, this is the same function as `gtk::TextBufferExt::create_tag`, but
instead of creating a `gtk::TextTag`, this function creates a `Tag`.

This function creates a `Tag` and adds it to the tag table for
`self`. Equivalent to calling `gtk::TextTag::new` and then adding the tag to
the buffer’s tag table. The returned tag is owned by the buffer’s tag table,
so the ref count will be equal to one.

If `tag_name` is `None`, the tag is anonymous.

If `tag_name` is non-`None`, a tag called `tag_name` must not already
exist in the tag table for this buffer.

The `first_property_name` argument and subsequent arguments are a list
of properties to set on the tag, as with `gobject::Object::set`.

Feature: `v3_20`

## `tag_name`
name of the new tag, or `None`
## `first_property_name`
name of first property to set, or `None`

# Returns

a new `Tag`.
<!-- trait BufferExt::fn end_not_undoable_action -->
Marks the end of a not undoable action on the buffer. When the
last not undoable block is closed through the call to this
function, the list of undo actions is cleared and the undo manager
is re-enabled.
<!-- trait BufferExt::fn ensure_highlight -->
Forces buffer to analyze and highlight the given area synchronously.

`<note>`
 `<para>`
 This is a potentially slow operation and should be used only
 when you need to make sure that some text not currently
 visible is highlighted, for instance before printing.
 `</para>`
`</note>`
## `start`
start of the area to highlight.
## `end`
end of the area to highlight.
<!-- trait BufferExt::fn forward_iter_to_source_mark -->
Moves `iter` to the position of the next `Mark` of the given
`category`. Returns `true` if `iter` was moved. If `category` is NULL, the
next source mark can be of any category.

Feature: `v2_2`

## `iter`
an iterator.
## `category`
category to search for, or `None`

# Returns

whether `iter` was moved.
<!-- trait BufferExt::fn get_context_classes_at_iter -->
Get all defined context classes at `iter`.

See the `Buffer` description for the list of default context classes.

Feature: `v2_10`

## `iter`
a `gtk::TextIter`.

# Returns

a new `None`
terminated array of context class names.
Use `g_strfreev` to free the array if it is no longer needed.
<!-- trait BufferExt::fn get_highlight_matching_brackets -->
Determines whether bracket match highlighting is activated for the
source buffer.

# Returns

`true` if the source buffer will highlight matching
brackets.
<!-- trait BufferExt::fn get_highlight_syntax -->
Determines whether syntax highlighting is activated in the source
buffer.

# Returns

`true` if syntax highlighting is enabled, `false` otherwise.
<!-- trait BufferExt::fn get_implicit_trailing_newline -->

Feature: `v3_14`


# Returns

whether the `self` has an implicit trailing newline.
<!-- trait BufferExt::fn get_language -->
Returns the `Language` associated with the buffer,
see `BufferExt::set_language`. The returned object should not be
unreferenced by the user.

# Returns

the `Language` associated
with the buffer, or `None`.
<!-- trait BufferExt::fn get_max_undo_levels -->
Determines the number of undo levels the buffer will track for buffer edits.

# Returns

the maximum number of possible undo levels or -1 if no limit is set.
<!-- trait BufferExt::fn get_source_marks_at_iter -->
Returns the list of marks of the given category at `iter`. If `category`
is `None` it returns all marks at `iter`.

Feature: `v2_2`

## `iter`
an iterator.
## `category`
category to search for, or `None`

# Returns


a newly allocated `glib::SList`.
<!-- trait BufferExt::fn get_source_marks_at_line -->
Returns the list of marks of the given category at `line`.
If `category` is `None`, all marks at `line` are returned.

Feature: `v2_2`

## `line`
a line number.
## `category`
category to search for, or `None`

# Returns

a newly allocated `glib::SList`.
<!-- trait BufferExt::fn get_style_scheme -->
Returns the `StyleScheme` associated with the buffer,
see `BufferExt::set_style_scheme`.
The returned object should not be unreferenced by the user.

# Returns

the `StyleScheme`
associated with the buffer, or `None`.
<!-- trait BufferExt::fn get_undo_manager -->
Returns the `UndoManager` associated with the buffer,
see `BufferExt::set_undo_manager`. The returned object should not be
unreferenced by the user.

# Returns

the `UndoManager` associated
with the buffer, or `None`.
<!-- trait BufferExt::fn iter_backward_to_context_class_toggle -->
Moves backward to the next toggle (on or off) of the context class. If no
matching context class toggles are found, returns `false`, otherwise `true`.
Does not return toggles located at `iter`, only toggles after `iter`. Sets
`iter` to the location of the toggle, or to the end of the buffer if no
toggle is found.

See the `Buffer` description for the list of default context classes.

Feature: `v2_10`

## `iter`
a `gtk::TextIter`.
## `context_class`
the context class.

# Returns

whether we found a context class toggle before `iter`
<!-- trait BufferExt::fn iter_forward_to_context_class_toggle -->
Moves forward to the next toggle (on or off) of the context class. If no
matching context class toggles are found, returns `false`, otherwise `true`.
Does not return toggles located at `iter`, only toggles after `iter`. Sets
`iter` to the location of the toggle, or to the end of the buffer if no
toggle is found.

See the `Buffer` description for the list of default context classes.

Feature: `v2_10`

## `iter`
a `gtk::TextIter`.
## `context_class`
the context class.

# Returns

whether we found a context class toggle after `iter`
<!-- trait BufferExt::fn iter_has_context_class -->
Check if the class `context_class` is set on `iter`.

See the `Buffer` description for the list of default context classes.

Feature: `v2_10`

## `iter`
a `gtk::TextIter`.
## `context_class`
class to search for.

# Returns

whether `iter` has the context class.
<!-- trait BufferExt::fn join_lines -->
Joins the lines of text between the specified iterators.

Feature: `v3_16`

## `start`
a `gtk::TextIter`.
## `end`
a `gtk::TextIter`.
<!-- trait BufferExt::fn redo -->
Redoes the last undo operation. Use `BufferExt::can_redo`
to check whether a call to this function will have any effect.

This function emits the `Buffer::redo` signal.
<!-- trait BufferExt::fn remove_source_marks -->
Remove all marks of `category` between `start` and `end` from the buffer.
If `category` is NULL, all marks in the range will be removed.

Feature: `v2_2`

## `start`
a `gtk::TextIter`.
## `end`
a `gtk::TextIter`.
## `category`
category to search for, or `None`.
<!-- trait BufferExt::fn set_highlight_matching_brackets -->
Controls the bracket match highlighting function in the buffer. If
activated, when you position your cursor over a bracket character
(a parenthesis, a square bracket, etc.) the matching opening or
closing bracket character will be highlighted.
## `highlight`
`true` if you want matching brackets highlighted.
<!-- trait BufferExt::fn set_highlight_syntax -->
Controls whether syntax is highlighted in the buffer. If `highlight`
is `true`, the text will be highlighted according to the syntax
patterns specified in the language set with
`BufferExt::set_language`. If `highlight` is `false`, syntax highlighting
is disabled and all the `gtk::TextTag` objects that have been added by the
syntax highlighting engine are removed from the buffer.
## `highlight`
`true` to enable syntax highlighting, `false` to disable it.
<!-- trait BufferExt::fn set_implicit_trailing_newline -->
Sets whether the `self` has an implicit trailing newline.

If an explicit trailing newline is present in a `gtk::TextBuffer`, `gtk::TextView`
shows it as an empty line. This is generally not what the user expects.

If `implicit_trailing_newline` is `true` (the default value):
 - when a `FileLoader` loads the content of a file into the `self`,
 the trailing newline (if present in the file) is not inserted into the
 `self`.
 - when a `FileSaver` saves the content of the `self` into a file, a
 trailing newline is added to the file.

On the other hand, if `implicit_trailing_newline` is `false`, the file's
content is not modified when loaded into the `self`, and the `self`'s
content is not modified when saved into a file.

Feature: `v3_14`

## `implicit_trailing_newline`
the new value.
<!-- trait BufferExt::fn set_language -->
Associate a `Language` with the buffer. If `language` is
not-`None` and syntax highlighting is enabled (see `BufferExt::set_highlight_syntax`),
the syntax patterns defined in `language` will be used to highlight the text
contained in the buffer. If `language` is `None`, the text contained in the
buffer is not highlighted.

The buffer holds a reference to `language`.
## `language`
a `Language` to set, or `None`.
<!-- trait BufferExt::fn set_max_undo_levels -->
Sets the number of undo levels for user actions the buffer will
track. If the number of user actions exceeds the limit set by this
function, older actions will be discarded.

If `max_undo_levels` is -1, the undo/redo is unlimited.

If `max_undo_levels` is 0, the undo/redo is disabled.
## `max_undo_levels`
the desired maximum number of undo levels.
<!-- trait BufferExt::fn set_style_scheme -->
Sets style scheme used by the buffer. If `scheme` is `None` no
style scheme is used.
## `scheme`
a `StyleScheme` or `None`.
<!-- trait BufferExt::fn set_undo_manager -->
Set the buffer undo manager. If `manager` is `None` the default undo manager
will be set.
## `manager`
A `UndoManager` or `None`.
<!-- trait BufferExt::fn sort_lines -->
Sort the lines of text between the specified iterators.

Feature: `v3_18`

## `start`
a `gtk::TextIter`.
## `end`
a `gtk::TextIter`.
## `flags`
`SortFlags` specifying how the sort should behave
## `column`
sort considering the text starting at the given column
<!-- trait BufferExt::fn undo -->
Undoes the last user action which modified the buffer. Use
`BufferExt::can_undo` to check whether a call to this
function will have any effect.

This function emits the `Buffer::undo` signal.
<!-- enum ChangeCaseType -->
<!-- enum ChangeCaseType::variant Lower -->
change case to lowercase.
<!-- enum ChangeCaseType::variant Upper -->
change case to uppercase.
<!-- enum ChangeCaseType::variant Toggle -->
toggle case of each character.
<!-- enum ChangeCaseType::variant Title -->
capitalize each word.

Feature: `v3_12`

<!-- struct Completion -->


# Implements

[`CompletionExt`](trait.CompletionExt.html)
<!-- trait CompletionExt -->
Trait containing all `Completion` methods.

# Implementors

[`Completion`](struct.Completion.html)
<!-- trait CompletionExt::fn add_provider -->
Add a new `CompletionProvider` to the completion object. This will
add a reference `provider`, so make sure to unref your own copy when you
no longer need it.
## `provider`
a `CompletionProvider`.

# Returns

`true` if `provider` was successfully added, otherwise if `error`
 is provided, it will be set with the error and `false` is returned.
<!-- trait CompletionExt::fn block_interactive -->
Block interactive completion. This can be used to disable interactive
completion when inserting or deleting text from the buffer associated with
the completion. Use `CompletionExt::unblock_interactive` to enable
interactive completion again.

This function may be called multiple times. It will continue to block
interactive completion until `CompletionExt::unblock_interactive`
has been called the same number of times.
<!-- trait CompletionExt::fn create_context -->
Create a new `CompletionContext` for `self`. The position where
the completion occurs can be specified by `position`. If `position` is `None`,
the current cursor position will be used.
## `position`
a `gtk::TextIter`, or `None`.

# Returns

a new `CompletionContext`.
The reference being returned is a 'floating' reference,
so if you invoke `CompletionExt::show` with this context
you don't need to unref it.
<!-- trait CompletionExt::fn get_info_window -->
The info widget is the window where the completion displays optional extra
information of the proposal.

# Returns

The `CompletionInfo` window
 associated with `self`.
<!-- trait CompletionExt::fn get_providers -->
Get list of providers registered on `self`. The returned list is owned
by the completion and should not be freed.

# Returns


list of `CompletionProvider`.
<!-- trait CompletionExt::fn get_view -->
The `View` associated with `self`, or `None` if the view has been
destroyed.

# Returns


The `View` associated with `self`, or `None`.
<!-- trait CompletionExt::fn hide -->
Hides the completion if it is active (visible).
<!-- trait CompletionExt::fn move_window -->
Move the completion window to a specific iter.

# Deprecated since 3.8

Use `CompletionProvider::get_start_iter` instead.
## `iter`
a `gtk::TextIter`.
<!-- trait CompletionExt::fn remove_provider -->
Remove `provider` from the completion.
## `provider`
a `CompletionProvider`.

# Returns

`true` if `provider` was successfully removed, otherwise if `error`
 is provided, it will be set with the error and `false` is returned.
<!-- trait CompletionExt::fn show -->
Starts a new completion with the specified `CompletionContext` and
a list of potential candidate providers for completion.

It can be convenient for showing a completion on-the-fly, without the need to
add or remove providers to the `Completion`.

Another solution is to add providers with
`CompletionExt::add_provider`, and implement
`CompletionProvider::match` for each provider.
## `providers`

a list of `CompletionProvider`, or `None`.
## `context`
The `CompletionContext`
with which to start the completion.

# Returns

`true` if it was possible to the show completion window.
<!-- trait CompletionExt::fn unblock_interactive -->
Unblock interactive completion. This can be used after using
`CompletionExt::block_interactive` to enable interactive completion
again.
<!-- struct CompletionContext -->


# Implements

[`CompletionContextExt`](trait.CompletionContextExt.html)
<!-- trait CompletionContextExt -->
Trait containing all `CompletionContext` methods.

# Implementors

[`CompletionContext`](struct.CompletionContext.html)
<!-- trait CompletionContextExt::fn add_proposals -->
Providers can use this function to add proposals to the completion. They
can do so asynchronously by means of the `finished` argument. Providers must
ensure that they always call this function with `finished` set to `true`
once each population (even if no proposals need to be added).
Population occurs when the `CompletionProvider::populate`
function is called.
## `provider`
a `CompletionProvider`.
## `proposals`
The list of proposals to add.
## `finished`
Whether the provider is finished adding proposals.
<!-- trait CompletionContextExt::fn get_activation -->
Get the context activation.

# Returns

The context activation.
<!-- trait CompletionContextExt::fn get_iter -->
Get the iter at which the completion was invoked. Providers can use this
to determine how and if to match proposals.
## `iter`
a `gtk::TextIter`.

# Returns

`true` if `iter` is correctly set, `false` otherwise.
<!-- struct CompletionInfo -->


# Implements

[`CompletionInfoExt`](trait.CompletionInfoExt.html), [`WidgetExt`](trait.WidgetExt.html)
<!-- trait CompletionInfoExt -->
Trait containing all `CompletionInfo` methods.

# Implementors

[`CompletionInfo`](struct.CompletionInfo.html)
<!-- impl CompletionInfo::fn new -->

# Returns

a new `CompletionInfo`.
<!-- trait CompletionInfoExt::fn get_widget -->
Get the current content widget.

# Deprecated since 3.8

Use `gtk::Bin::get_child` instead.

# Returns

The current content widget.
<!-- trait CompletionInfoExt::fn move_to_iter -->
Moves the `CompletionInfo` to `iter`. If `iter` is `None` `self` is
moved to the cursor position. Moving will respect the `gdk::Gravity` setting
of the info window and will ensure the line at `iter` is not occluded by
the window.
## `view`
a `gtk::TextView` on which the info window should be positioned.
## `iter`
a `gtk::TextIter`.
<!-- trait CompletionInfoExt::fn set_widget -->
Sets the content widget of the info window. See that the previous widget will
lose a reference and it can be destroyed, so if you do not want this to
happen you must use `gobject::Object::ref` before calling this method.

# Deprecated since 3.8

Use `gtk::Container::add` instead. If there is already a child
widget, remove it with `gtk::Container::remove`.
## `widget`
a `gtk::Widget`.
<!-- struct CompletionItem -->


# Implements

[`CompletionItemExt`](trait.CompletionItemExt.html), [`CompletionProposalExt`](trait.CompletionProposalExt.html)
<!-- trait CompletionItemExt -->
Trait containing all `CompletionItem` methods.

# Implementors

[`CompletionItem`](struct.CompletionItem.html)
<!-- impl CompletionItem::fn new -->
Create a new `CompletionItem` with label `label`, icon `icon` and
extra information `info`. Both `icon` and `info` can be `None` in which case
there will be no icon shown and no extra information available.
## `label`
The item label.
## `text`
The item text.
## `icon`
The item icon.
## `info`
The item extra information.

# Returns

a new `CompletionItem`.
<!-- impl CompletionItem::fn new_from_stock -->
Creates a new `CompletionItem` from a stock item. If `label` is `None`,
the stock label will be used.

# Deprecated since 3.10

Use `CompletionItem::new` instead.
## `label`
The item label.
## `text`
The item text.
## `stock`
The stock icon.
## `info`
The item extra information.

# Returns

a new `CompletionItem`.
<!-- impl CompletionItem::fn new_with_markup -->
Create a new `CompletionItem` with markup label `markup`, icon
`icon` and extra information `info`. Both `icon` and `info` can be `None` in
which case there will be no icon shown and no extra information available.
## `markup`
The item markup label.
## `text`
The item text.
## `icon`
The item icon.
## `info`
The item extra information.

# Returns

a new `CompletionItem`.
<!-- struct CompletionProposal -->


# Implements

[`CompletionProposalExt`](trait.CompletionProposalExt.html)
<!-- trait CompletionProposalExt -->
Trait containing all `CompletionProposal` methods.

# Implementors

[`CompletionItem`](struct.CompletionItem.html), [`CompletionProposal`](struct.CompletionProposal.html)
<!-- trait CompletionProposalExt::fn changed -->
Emits the "changed" signal on `self`. This should be called by
implementations whenever the name, icon or info of the proposal has
changed.
<!-- trait CompletionProposalExt::fn equal -->
Get whether two proposal objects are the same. This is used to (together
with `CompletionProposal::hash`) to match proposals in the
completion model. By default, it uses direct equality (`g_direct_equal`).
## `other`
a `CompletionProposal`.

# Returns

`true` if `self` and `object` are the same proposal
<!-- trait CompletionProposalExt::fn get_gicon -->
Gets the `gio::Icon` for the icon of `self`.

Feature: `v3_18`


# Returns

A `gio::Icon` with the icon of `self`.
<!-- trait CompletionProposalExt::fn get_icon -->
Gets the `gdk_pixbuf::Pixbuf` for the icon of `self`.

# Returns

A `gdk_pixbuf::Pixbuf` with the icon of `self`.
<!-- trait CompletionProposalExt::fn get_icon_name -->
Gets the icon name of `self`.

Feature: `v3_18`


# Returns

The icon name of `self`.
<!-- trait CompletionProposalExt::fn get_info -->
Gets extra information associated to the proposal. This information will be
used to present the user with extra, detailed information about the
selected proposal. The returned string must be freed with `g_free`.

# Returns

a newly-allocated string containing
extra information of `self` or `None` if no extra information is associated
to `self`.
<!-- trait CompletionProposalExt::fn get_label -->
Gets the label of `self`. The label is shown in the list of proposals as
plain text. If you need any markup (such as bold or italic text), you have
to implement `CompletionProposal::get_markup`. The returned string
must be freed with `g_free`.

# Returns

a new string containing the label of `self`.
<!-- trait CompletionProposalExt::fn get_markup -->
Gets the label of `self` with markup. The label is shown in the list of
proposals and may contain markup. This will be used instead of
`CompletionProposal::get_label` if implemented. The returned string
must be freed with `g_free`.

# Returns

a new string containing the label of `self` with markup.
<!-- trait CompletionProposalExt::fn get_text -->
Gets the text of `self`. The text that is inserted into
the text buffer when the proposal is activated by the default activation.
You are free to implement a custom activation handler in the provider and
not implement this function. For more information, see
`CompletionProvider::activate_proposal`. The returned string must
be freed with `g_free`.

# Returns

a new string containing the text of `self`.
<!-- trait CompletionProposalExt::fn hash -->
Get the hash value of `self`. This is used to (together with
`CompletionProposal::equal`) to match proposals in the completion
model. By default, it uses a direct hash (`g_direct_hash`).

# Returns

The hash value of `self`.
<!-- struct CompletionProvider -->


# Implements

[`CompletionProviderExt`](trait.CompletionProviderExt.html)
<!-- trait CompletionProviderExt -->
Trait containing all `CompletionProvider` methods.

# Implementors

[`CompletionProvider`](struct.CompletionProvider.html), [`CompletionWords`](struct.CompletionWords.html)
<!-- trait CompletionProviderExt::fn activate_proposal -->
Activate `proposal` at `iter`. When this functions returns `false`, the default
activation of `proposal` will take place which replaces the word at `iter`
with the text of `proposal` (see `CompletionProposal::get_text`).

Here is how the default activation selects the boundaries of the word to
replace. The end of the word is `iter`. For the start of the word, it depends
on whether a start iter is defined for `proposal` (see
`CompletionProvider::get_start_iter`). If a start iter is defined,
the start of the word is the start iter. Else, the word (as long as possible)
will contain only alphanumerical and the "_" characters.
## `proposal`
a `CompletionProposal`.
## `iter`
a `gtk::TextIter`.

# Returns

`true` to indicate that the proposal activation has been handled,
 `false` otherwise.
<!-- trait CompletionProviderExt::fn get_activation -->
Get with what kind of activation the provider should be activated.

# Returns

a combination of `CompletionActivation`.
<!-- trait CompletionProviderExt::fn get_gicon -->
Gets the `gio::Icon` for the icon of `self`.

Feature: `v3_18`


# Returns

The icon to be used for the provider,
 or `None` if the provider does not have a special icon.
<!-- trait CompletionProviderExt::fn get_icon -->
Get the `gdk_pixbuf::Pixbuf` for the icon of the `self`.

# Returns

The icon to be used for the provider,
 or `None` if the provider does not have a special icon.
<!-- trait CompletionProviderExt::fn get_icon_name -->
Gets the icon name of `self`.

Feature: `v3_18`


# Returns

The icon name to be used for the provider,
 or `None` if the provider does not have a special icon.
<!-- trait CompletionProviderExt::fn get_info_widget -->
Get a customized info widget to show extra information of a proposal.
This allows for customized widgets on a proposal basis, although in general
providers will have the same custom widget for all their proposals and
`proposal` can be ignored. The implementation of this function is optional.

If this function is not implemented, the default widget is a `gtk::Label`. The
return value of `CompletionProposal::get_info` is used as the
content of the `gtk::Label`.

`<note>`
 `<para>`
 If implemented, `CompletionProvider::update_info`
 `<emphasis>`must`</emphasis>` also be implemented.
 `</para>`
`</note>`
## `proposal`
a currently selected `CompletionProposal`.

# Returns

a custom `gtk::Widget` to show extra
information about `proposal`, or `None` if the provider does not have a special
info widget.
<!-- trait CompletionProviderExt::fn get_interactive_delay -->
Get the delay in milliseconds before starting interactive completion for
this provider. A value of -1 indicates to use the default value as set
by the `Completion:auto-complete-delay` property.

# Returns

the interactive delay in milliseconds.
<!-- trait CompletionProviderExt::fn get_name -->
Get the name of the provider. This should be a translatable name for
display to the user. For example: _("Document word completion provider"). The
returned string must be freed with `g_free`.

# Returns

a new string containing the name of the provider.
<!-- trait CompletionProviderExt::fn get_priority -->
Get the provider priority. The priority determines the order in which
proposals appear in the completion popup. Higher priorities are sorted
before lower priorities. The default priority is 0.

# Returns

the provider priority.
<!-- trait CompletionProviderExt::fn get_start_iter -->
Get the `gtk::TextIter` at which the completion for `proposal` starts. When
implemented, this information is used to position the completion window
accordingly when a proposal is selected in the completion window. The
`proposal` text inside the completion window is aligned on `iter`.

If this function is not implemented, the word boundary is taken to position
the completion window. See `CompletionProvider::activate_proposal`
for an explanation on the word boundaries.

When the `proposal` is activated, the default handler uses `iter` as the start
of the word to replace. See
`CompletionProvider::activate_proposal` for more information.
## `context`
a `CompletionContext`.
## `proposal`
a `CompletionProposal`.
## `iter`
a `gtk::TextIter`.

# Returns

`true` if `iter` was set for `proposal`, `false` otherwise.
<!-- trait CompletionProviderExt::fn match -->
Get whether the provider match the context of completion detailed in
`context`.
## `context`
a `CompletionContext`.

# Returns

`true` if `self` matches the completion context, `false` otherwise.
<!-- trait CompletionProviderExt::fn populate -->
Populate `context` with proposals from `self` added with the
`CompletionContextExt::add_proposals` function.
## `context`
a `CompletionContext`.
<!-- trait CompletionProviderExt::fn update_info -->
Update extra information shown in `info` for `proposal`.

`<note>`
 `<para>`
 This function `<emphasis>`must`</emphasis>` be implemented when
 `CompletionProvider::get_info_widget` is implemented.
 `</para>`
`</note>`
## `proposal`
a `CompletionProposal`.
## `info`
a `CompletionInfo`.
<!-- struct CompletionWords -->


# Implements

[`CompletionWordsExt`](trait.CompletionWordsExt.html), [`CompletionProviderExt`](trait.CompletionProviderExt.html)
<!-- trait CompletionWordsExt -->
Trait containing all `CompletionWords` methods.

# Implementors

[`CompletionWords`](struct.CompletionWords.html)
<!-- impl CompletionWords::fn new -->
## `name`
The name for the provider
## `icon`
A specific icon for the provider

# Returns

a new `CompletionWords` provider
<!-- trait CompletionWordsExt::fn register -->
Registers `buffer` in the `self` provider.
## `buffer`
a `gtk::TextBuffer`
<!-- trait CompletionWordsExt::fn unregister -->
Unregisters `buffer` from the `self` provider.
## `buffer`
a `gtk::TextBuffer`
<!-- enum CompressionType -->
<!-- enum CompressionType::variant None -->
plain text.
<!-- enum CompressionType::variant Gzip -->
gzip compression.

Feature: `v3_14`

<!-- struct Encoding -->

Feature: `v3_14`
<!-- impl Encoding::fn copy -->
Used by language bindings.

Feature: `v3_14`


# Returns

a copy of `self`.
<!-- impl Encoding::fn free -->
Used by language bindings.

Feature: `v3_14`

<!-- impl Encoding::fn get_charset -->
Gets the character set of the `Encoding`, such as "UTF-8" or
"ISO-8859-1".

Feature: `v3_14`


# Returns

the character set of the `Encoding`.
<!-- impl Encoding::fn get_name -->
Gets the name of the `Encoding` such as "Unicode" or "Western".

Feature: `v3_14`


# Returns

the name of the `Encoding`.
<!-- impl Encoding::fn to_string -->

Feature: `v3_14`


# Returns

a string representation. Free with `g_free` when no longer needed.
<!-- impl Encoding::fn get_all -->
Gets all encodings.

Feature: `v3_14`


# Returns

a list of
all `Encoding`'s. Free with `glib::SList::free`.
<!-- impl Encoding::fn get_current -->
Gets the `Encoding` for the current locale. See also `g_get_charset`.

Feature: `v3_14`


# Returns

the current locale encoding.
<!-- impl Encoding::fn get_default_candidates -->
Gets the list of default candidate encodings to try when loading a file. See
`FileLoaderExt::set_candidate_encodings`.

This function returns a different list depending on the current locale (i.e.
language, country and default encoding). The UTF-8 encoding and the current
locale encoding are guaranteed to be present in the returned list.

Feature: `v3_18`


# Returns

the list of
default candidate encodings. Free with `glib::SList::free`.
<!-- impl Encoding::fn get_from_charset -->
Gets a `Encoding` from a character set such as "UTF-8" or
"ISO-8859-1".

Feature: `v3_14`

## `charset`
a character set.

# Returns

the corresponding `Encoding`, or `None`
if not found.
<!-- impl Encoding::fn get_utf8 -->

Feature: `v3_14`


# Returns

the UTF-8 encoding.
<!-- struct File -->


Feature: `v3_14`

# Implements

[`FileExt`](trait.FileExt.html)
<!-- trait FileExt -->
Trait containing all `File` methods.

Feature: `v3_14`

# Implementors

[`File`](struct.File.html)
<!-- impl File::fn new -->

Feature: `v3_14`


# Returns

a new `File` object.
<!-- trait FileExt::fn check_file_on_disk -->
Checks synchronously the file on disk, to know whether the file is externally
modified, or has been deleted, and whether the file is read-only.

`File` doesn't create a `gio::FileMonitor` to track those properties, so
this function needs to be called instead. Creating lots of `gio::FileMonitor`'s
would take lots of resources.

Since this function is synchronous, it is advised to call it only on local
files. See `FileExt::is_local`.

Feature: `v3_18`

<!-- trait FileExt::fn get_compression_type -->

Feature: `v3_14`


# Returns

the compression type.
<!-- trait FileExt::fn get_encoding -->
The encoding is initially `None`. After a successful file loading or saving
operation, the encoding is non-`None`.

Feature: `v3_14`


# Returns

the character encoding.
<!-- trait FileExt::fn get_location -->

Feature: `v3_14`


# Returns

the `gio::File`.
<!-- trait FileExt::fn get_newline_type -->

Feature: `v3_14`


# Returns

the newline type.
<!-- trait FileExt::fn is_deleted -->
Returns whether the file has been deleted. If the
`File:location` is `None`, returns `false`.

To have an up-to-date value, you must first call
`FileExt::check_file_on_disk`.

Feature: `v3_18`


# Returns

whether the file has been deleted.
<!-- trait FileExt::fn is_externally_modified -->
Returns whether the file is externally modified. If the
`File:location` is `None`, returns `false`.

To have an up-to-date value, you must first call
`FileExt::check_file_on_disk`.

Feature: `v3_18`


# Returns

whether the file is externally modified.
<!-- trait FileExt::fn is_local -->
Returns whether the file is local. If the `File:location` is `None`,
returns `false`.

Feature: `v3_18`


# Returns

whether the file is local.
<!-- trait FileExt::fn is_readonly -->
Returns whether the file is read-only. If the
`File:location` is `None`, returns `false`.

To have an up-to-date value, you must first call
`FileExt::check_file_on_disk`.

Feature: `v3_18`


# Returns

whether the file is read-only.
<!-- trait FileExt::fn set_location -->
Sets the location.

Feature: `v3_14`

## `location`
the new `gio::File`, or `None`.
<!-- trait FileExt::fn set_mount_operation_factory -->
Sets a ``GtkSourceMountOperationFactory`` function that will be called when a
`gio::MountOperation` must be created. This is useful for creating a
`gtk::MountOperation` with the parent `gtk::Window`.

If a mount operation factory isn't set, `gio::MountOperation::new` will be
called.

Feature: `v3_14`

## `callback`
a ``GtkSourceMountOperationFactory`` to call when a
 `gio::MountOperation` is needed.
## `user_data`
the data to pass to the `callback` function.
## `notify`
function to call on `user_data` when the `callback` is no
 longer needed, or `None`.
<!-- struct FileLoader -->


Feature: `v3_14`

# Implements

[`FileLoaderExt`](trait.FileLoaderExt.html)
<!-- trait FileLoaderExt -->
Trait containing all `FileLoader` methods.

Feature: `v3_14`

# Implementors

[`FileLoader`](struct.FileLoader.html)
<!-- impl FileLoader::fn new -->
Creates a new `FileLoader` object. The contents is read from the
`File`'s location. If not already done, call
`FileExt::set_location` before calling this constructor. The previous
location is anyway not needed, because as soon as the file loading begins,
the `buffer` is emptied.

Feature: `v3_14`

## `buffer`
the `Buffer` to load the contents into.
## `file`
the `File`.

# Returns

a new `FileLoader` object.
<!-- impl FileLoader::fn new_from_stream -->
Creates a new `FileLoader` object. The contents is read from `stream`.

Feature: `v3_14`

## `buffer`
the `Buffer` to load the contents into.
## `file`
the `File`.
## `stream`
the `gio::InputStream` to load, e.g. stdin.

# Returns

a new `FileLoader` object.
<!-- trait FileLoaderExt::fn get_buffer -->

Feature: `v3_14`


# Returns

the `Buffer` to load the contents into.
<!-- trait FileLoaderExt::fn get_compression_type -->

Feature: `v3_14`


# Returns

the detected compression type.
<!-- trait FileLoaderExt::fn get_encoding -->

Feature: `v3_14`


# Returns

the detected file encoding.
<!-- trait FileLoaderExt::fn get_file -->

Feature: `v3_14`


# Returns

the `File`.
<!-- trait FileLoaderExt::fn get_input_stream -->

Feature: `v3_14`


# Returns

the `gio::InputStream` to load, or `None`
if a `gio::File` is used.
<!-- trait FileLoaderExt::fn get_location -->

Feature: `v3_14`


# Returns

the `gio::File` to load, or `None`
if an input stream is used.
<!-- trait FileLoaderExt::fn get_newline_type -->

Feature: `v3_14`


# Returns

the detected newline type.
<!-- trait FileLoaderExt::fn load_async -->
Loads asynchronously the file or input stream contents into the
`Buffer`. See the `gio::AsyncResult` documentation to know how to use this
function.

Feature: `v3_14`

## `io_priority`
the I/O priority of the request. E.g. `G_PRIORITY_LOW`,
 `G_PRIORITY_DEFAULT` or `G_PRIORITY_HIGH`.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `progress_callback`
function to call back with
 progress information, or `None` if progress information is not needed.
## `progress_callback_data`
user data to pass to `progress_callback`.
## `progress_callback_notify`
function to call on
 `progress_callback_data` when the `progress_callback` is no longer needed, or
 `None`.
## `callback`
a `GAsyncReadyCallback` to call when the request is
 satisfied.
## `user_data`
user data to pass to `callback`.
<!-- trait FileLoaderExt::fn load_finish -->
Finishes a file loading started with `FileLoaderExt::load_async`.

If the contents has been loaded, the following `File` properties will
be updated: the location, the encoding, the newline type and the compression
type.

Feature: `v3_14`

## `result`
a `gio::AsyncResult`.

# Returns

whether the contents has been loaded successfully.
<!-- trait FileLoaderExt::fn set_candidate_encodings -->
Sets the candidate encodings for the file loading. The encodings are tried in
the same order as the list.

For convenience, `candidate_encodings` can contain duplicates. Only the first
occurrence of a duplicated encoding is kept in the list.

By default the candidate encodings are (in that order in the list):
1. If set, the `File`'s encoding as returned by
`FileExt::get_encoding`.
2. The default candidates as returned by
`Encoding::get_default_candidates`.

Feature: `v3_14`

## `candidate_encodings`
a list of
 `Encoding`<!-- -->s.
<!-- struct FileSaver -->


Feature: `v3_14`

# Implements

[`FileSaverExt`](trait.FileSaverExt.html)
<!-- trait FileSaverExt -->
Trait containing all `FileSaver` methods.

Feature: `v3_14`

# Implementors

[`FileSaver`](struct.FileSaver.html)
<!-- impl FileSaver::fn new -->
Creates a new `FileSaver` object. The `buffer` will be saved to the
`File`'s location.

This constructor is suitable for a simple "save" operation, when the `file`
already contains a non-`None` `File:location`.

Feature: `v3_14`

## `buffer`
the `Buffer` to save.
## `file`
the `File`.

# Returns

a new `FileSaver` object.
<!-- impl FileSaver::fn new_with_target -->
Creates a new `FileSaver` object with a target location. When the
file saving is finished successfully, `target_location` is set to the `file`'s
`File:location` property. If an error occurs, the previous valid
location is still available in `File`.

This constructor is suitable for a "save as" operation, or for saving a new
buffer for the first time.

Feature: `v3_14`

## `buffer`
the `Buffer` to save.
## `file`
the `File`.
## `target_location`
the `gio::File` where to save the buffer to.

# Returns

a new `FileSaver` object.
<!-- trait FileSaverExt::fn get_buffer -->

Feature: `v3_14`


# Returns

the `Buffer` to save.
<!-- trait FileSaverExt::fn get_compression_type -->

Feature: `v3_14`


# Returns

the compression type.
<!-- trait FileSaverExt::fn get_encoding -->

Feature: `v3_14`


# Returns

the encoding.
<!-- trait FileSaverExt::fn get_file -->

Feature: `v3_14`


# Returns

the `File`.
<!-- trait FileSaverExt::fn get_flags -->

Feature: `v3_14`


# Returns

the flags.
<!-- trait FileSaverExt::fn get_location -->

Feature: `v3_14`


# Returns

the `gio::File` where to save the buffer to.
<!-- trait FileSaverExt::fn get_newline_type -->

Feature: `v3_14`


# Returns

the newline type.
<!-- trait FileSaverExt::fn save_async -->
Saves asynchronously the buffer into the file. See the `gio::AsyncResult`
documentation to know how to use this function.

Feature: `v3_14`

## `io_priority`
the I/O priority of the request. E.g. `G_PRIORITY_LOW`,
 `G_PRIORITY_DEFAULT` or `G_PRIORITY_HIGH`.
## `cancellable`
optional `gio::Cancellable` object, `None` to ignore.
## `progress_callback`
function to call back with
 progress information, or `None` if progress information is not needed.
## `progress_callback_data`
user data to pass to `progress_callback`.
## `progress_callback_notify`
function to call on
 `progress_callback_data` when the `progress_callback` is no longer needed, or
 `None`.
## `callback`
a `GAsyncReadyCallback` to call when the request is
 satisfied.
## `user_data`
user data to pass to `callback`.
<!-- trait FileSaverExt::fn save_finish -->
Finishes a file saving started with `FileSaverExt::save_async`.

If the file has been saved successfully, the following `File`
properties will be updated: the location, the encoding, the newline type and
the compression type.

Since the 3.20 version, `gtk::TextBufferExt::set_modified` is called with `false`
if the file has been saved successfully.

Feature: `v3_14`

## `result`
a `gio::AsyncResult`.

# Returns

whether the file was saved successfully.
<!-- trait FileSaverExt::fn set_compression_type -->
Sets the compression type. By default the compression type is taken from the
`File`.

Feature: `v3_14`

## `compression_type`
the new compression type.
<!-- trait FileSaverExt::fn set_encoding -->
Sets the encoding. If `encoding` is `None`, the UTF-8 encoding will be set.
By default the encoding is taken from the `File`.

Feature: `v3_14`

## `encoding`
the new encoding, or `None` for UTF-8.
<!-- trait FileSaverExt::fn set_flags -->

Feature: `v3_14`

## `flags`
the new flags.
<!-- trait FileSaverExt::fn set_newline_type -->
Sets the newline type. By default the newline type is taken from the
`File`.

Feature: `v3_14`

## `newline_type`
the new newline type.
<!-- struct Gutter -->


# Implements

[`GutterExt`](trait.GutterExt.html)
<!-- trait GutterExt -->
Trait containing all `Gutter` methods.

# Implementors

[`Gutter`](struct.Gutter.html)
<!-- trait GutterExt::fn get_padding -->

# Deprecated since 3.12

Use `GutterRendererExt::get_padding` instead.
<!-- trait GutterExt::fn get_renderer_at_pos -->
Finds the `GutterRenderer` at (x, y).
## `x`
The x position to get identified.
## `y`
The y position to get identified.

# Returns

the renderer at (x, y) or `None`.
<!-- trait GutterExt::fn get_window -->
Get the `gdk::Window` of the gutter. The window will only be available when the
gutter has at least one, non-zero width, cell renderer packed.

Feature: `v2_8`


# Deprecated since 3.12

Use `gtk::TextViewExt::get_window` instead.

# Returns

the `gdk::Window` of the gutter, or `None`
if the gutter has no window.
<!-- trait GutterExt::fn insert -->
Insert `renderer` into the gutter. If `renderer` is yet unowned then gutter
claims its ownership. Otherwise just increases renderer's reference count.
`renderer` cannot be already inserted to another gutter.

Feature: `v3_0`

## `renderer`
a gutter renderer (must inherit from `GutterRenderer`).
## `position`
the renderer position.

# Returns

`true` if operation succeeded. Otherwise `false`.
<!-- trait GutterExt::fn queue_draw -->
Invalidates the drawable area of the gutter. You can use this to force a
redraw of the gutter if something has changed and needs to be redrawn.

Feature: `v2_8`

<!-- trait GutterExt::fn remove -->
Removes `renderer` from `self`.

Feature: `v2_8`

## `renderer`
a `GutterRenderer`.
<!-- trait GutterExt::fn reorder -->
Reorders `renderer` in `self` to new `position`.

Feature: `v2_8`

## `renderer`
a `gtk::CellRenderer`.
## `position`
the new renderer position.
<!-- trait GutterExt::fn set_padding -->

# Deprecated since 3.12

Use `GutterRendererExt::set_padding` instead.
<!-- struct GutterRenderer -->


# Implements

[`GutterRendererExt`](trait.GutterRendererExt.html)
<!-- trait GutterRendererExt -->
Trait containing all `GutterRenderer` methods.

# Implementors

[`GutterRendererPixbuf`](struct.GutterRendererPixbuf.html), [`GutterRendererText`](struct.GutterRendererText.html), [`GutterRenderer`](struct.GutterRenderer.html)
<!-- trait GutterRendererExt::fn activate -->
Emits the `GutterRenderer::activate` signal of the renderer. This is
called from `Gutter` and should never have to be called manually.
## `iter`
a `gtk::TextIter` at the start of the line where the renderer is activated
## `area`
a `gdk::Rectangle` of the cell area where the renderer is activated
## `event`
the event that triggered the activation
<!-- trait GutterRendererExt::fn begin -->
Called when drawing a region begins. The region to be drawn is indicated
by `start` and `end`. The purpose is to allow the implementation to precompute
some state before the draw method is called for each cell.
## `cr`
a `cairo::Context`
## `background_area`
a `gdk::Rectangle`
## `cell_area`
a `gdk::Rectangle`
## `start`
a `gtk::TextIter`
## `end`
a `gtk::TextIter`
<!-- trait GutterRendererExt::fn draw -->
Main renderering method. Implementations should implement this method to draw
onto the cairo context. The `background_area` indicates the total area of the
cell to be drawn. The `cell_area` indicates the area where content can be
drawn (text, images, etc).

The `background_area` is the `cell_area` plus the padding on each side (two
times the `GutterRenderer:xpad` horizontally and two times the
`GutterRenderer:ypad` vertically, so that the `cell_area` is centered
inside `background_area`).

The `state` argument indicates the current state of the renderer and should
be taken into account to properly draw the different possible states
(cursor, prelit, selected) if appropriate.
## `cr`
the cairo render context
## `background_area`
a `gdk::Rectangle` indicating the total area to be drawn
## `cell_area`
a `gdk::Rectangle` indicating the area to draw content
## `start`
a `gtk::TextIter`
## `end`
a `gtk::TextIter`
## `state`
a `GutterRendererState`
<!-- trait GutterRendererExt::fn end -->
Called when drawing a region of lines has ended.
<!-- trait GutterRendererExt::fn get_alignment -->
Get the x-alignment and y-alignment of the gutter renderer.
## `xalign`
return location for the x-alignment (can be `None`)
## `yalign`
return location for the y-alignment (can be `None`)
<!-- trait GutterRendererExt::fn get_alignment_mode -->
Get the alignment mode. The alignment mode describes the manner in which the
renderer is aligned (see :xalign and :yalign).

# Returns

a `GutterRendererAlignmentMode`
<!-- trait GutterRendererExt::fn get_background -->
Get the background color of the renderer.
## `color`
return value for a `gdk::RGBA`

# Returns

`true` if the background color is set, `false` otherwise
<!-- trait GutterRendererExt::fn get_padding -->
Get the x-padding and y-padding of the gutter renderer.
## `xpad`
return location for the x-padding (can be `None`)
## `ypad`
return location for the y-padding (can be `None`)
<!-- trait GutterRendererExt::fn get_size -->
Get the size of the renderer.

# Returns

the size of the renderer.
<!-- trait GutterRendererExt::fn get_view -->
Get the view associated to the gutter renderer

# Returns

a `gtk::TextView`
<!-- trait GutterRendererExt::fn get_visible -->
Get whether the gutter renderer is visible.

# Returns

`true` if the renderer is visible, `false` otherwise
<!-- trait GutterRendererExt::fn get_window_type -->
Get the `gtk::TextWindowType` associated with the gutter renderer.

# Returns

a `gtk::TextWindowType`
<!-- trait GutterRendererExt::fn query_activatable -->
Get whether the renderer is activatable at the location in `event`. This is
called from `Gutter` to determine whether a renderer is activatable
using the mouse pointer.
## `iter`
a `gtk::TextIter` at the start of the line to be activated
## `area`
a `gdk::Rectangle` of the cell area to be activated
## `event`
the event that triggered the query

# Returns

`true` if the renderer can be activated, `false` otherwise
<!-- trait GutterRendererExt::fn query_data -->
Emit the `GutterRenderer::query-data` signal. This function is called
to query for data just before rendering a cell. This is called from the
`Gutter`. Implementations can override the default signal handler or
can connect a signal handler externally to the
`GutterRenderer::query-data` signal.
## `start`
a `gtk::TextIter`.
## `end`
a `gtk::TextIter`.
## `state`
a `GutterRendererState`.
<!-- trait GutterRendererExt::fn query_tooltip -->
Emits the `GutterRenderer::query-tooltip` signal. This function is
called from `Gutter`. Implementations can override the default signal
handler or can connect to the signal externally.
## `iter`
a `gtk::TextIter`.
## `area`
a `gdk::Rectangle`.
## `x`
The x position of the tooltip.
## `y`
The y position of the tooltip.
## `tooltip`
a `gtk::Tooltip`.

# Returns

`true` if the tooltip has been set, `false` otherwise
<!-- trait GutterRendererExt::fn queue_draw -->
Emits the `GutterRenderer::queue-draw` signal of the renderer. Call
this from an implementation to inform that the renderer has changed such that
it needs to redraw.
<!-- trait GutterRendererExt::fn set_alignment -->
Set the alignment of the gutter renderer. Both `xalign` and `yalign` can be
-1, which means the values will not be changed (this allows changing only
one of the values).

`xalign` is the horizontal alignment. Set to 0 for a left alignment. 1 for a
right alignment. And 0.5 for centering the cells. `yalign` is the vertical
alignment. Set to 0 for a top alignment. 1 for a bottom alignment.
## `xalign`
the x-alignment
## `yalign`
the y-alignment
<!-- trait GutterRendererExt::fn set_alignment_mode -->
Set the alignment mode. The alignment mode describes the manner in which the
renderer is aligned (see :xalign and :yalign).
## `mode`
a `GutterRendererAlignmentMode`
<!-- trait GutterRendererExt::fn set_background -->
Set the background color of the renderer. If `color` is set to `None`, the
renderer will not have a background color.
## `color`
a `gdk::RGBA` or `None`
<!-- trait GutterRendererExt::fn set_padding -->
Set the padding of the gutter renderer. Both `xpad` and `ypad` can be
-1, which means the values will not be changed (this allows changing only
one of the values).

`xpad` is the left and right padding. `ypad` is the top and bottom padding.
## `xpad`
the x-padding
## `ypad`
the y-padding
<!-- trait GutterRendererExt::fn set_size -->
Sets the size of the renderer. A value of -1 specifies that the size
is to be determined dynamically.
## `size`
the size
<!-- trait GutterRendererExt::fn set_visible -->
Set whether the gutter renderer is visible.
## `visible`
the visibility
<!-- enum GutterRendererAlignmentMode -->
The alignment mode of the renderer, when a cell spans multiple lines (due to
text wrapping).
<!-- enum GutterRendererAlignmentMode::variant Cell -->
The full cell.
<!-- enum GutterRendererAlignmentMode::variant First -->
The first line.
<!-- enum GutterRendererAlignmentMode::variant Last -->
The last line.
<!-- struct GutterRendererPixbuf -->


# Implements

[`GutterRendererPixbufExt`](trait.GutterRendererPixbufExt.html), [`GutterRendererExt`](trait.GutterRendererExt.html)
<!-- trait GutterRendererPixbufExt -->
Trait containing all `GutterRendererPixbuf` methods.

# Implementors

[`GutterRendererPixbuf`](struct.GutterRendererPixbuf.html)
<!-- impl GutterRendererPixbuf::fn new -->
Create a new `GutterRendererPixbuf`.

# Returns

A `GutterRenderer`
<!-- trait GutterRendererPixbufExt::fn get_gicon -->
Get the gicon of the renderer

# Returns

a `gio::Icon`
<!-- trait GutterRendererPixbufExt::fn get_pixbuf -->
Get the pixbuf of the renderer.

# Returns

a `gdk_pixbuf::Pixbuf`
<!-- trait GutterRendererPixbufExt::fn get_stock_id -->

# Deprecated since 3.10

Don't use this function.

# Returns

the stock id.
<!-- trait GutterRendererPixbufExt::fn set_gicon -->
## `icon`
the icon
<!-- trait GutterRendererPixbufExt::fn set_icon_name -->
## `icon_name`
the icon name
<!-- trait GutterRendererPixbufExt::fn set_pixbuf -->
## `pixbuf`
the pixbuf
<!-- trait GutterRendererPixbufExt::fn set_stock_id -->

# Deprecated since 3.10

Don't use this function.
## `stock_id`
the stock id
<!-- struct GutterRendererText -->


# Implements

[`GutterRendererTextExt`](trait.GutterRendererTextExt.html), [`GutterRendererExt`](trait.GutterRendererExt.html)
<!-- trait GutterRendererTextExt -->
Trait containing all `GutterRendererText` methods.

# Implementors

[`GutterRendererText`](struct.GutterRendererText.html)
<!-- impl GutterRendererText::fn new -->
Create a new `GutterRendererText`.

# Returns

A `GutterRenderer`
<!-- trait GutterRendererTextExt::fn measure -->
Measures the text provided using the pango layout used by the
`GutterRendererText`.
## `text`
the text to measure.
## `width`
location to store the width of the text in pixels,
 or `None`.
## `height`
location to store the height of the text in
 pixels, or `None`.
<!-- trait GutterRendererTextExt::fn measure_markup -->
Measures the pango markup provided using the pango layout used by the
`GutterRendererText`.
## `markup`
the pango markup to measure.
## `width`
location to store the width of the text in pixels,
 or `None`.
## `height`
location to store the height of the text in
 pixels, or `None`.
<!-- struct Language -->


# Implements

[`LanguageExt`](trait.LanguageExt.html)
<!-- trait LanguageExt -->
Trait containing all `Language` methods.

# Implementors

[`Language`](struct.Language.html)
<!-- trait LanguageExt::fn get_globs -->
Returns the globs associated to this language. This is just
an utility wrapper around `LanguageExt::get_metadata` to
retrieve the "globs" metadata property and split it into an array.

# Returns


a newly-allocated `None` terminated array containing the globs or `None`
if no globs are found.
The returned array must be freed with `g_strfreev`.
<!-- trait LanguageExt::fn get_hidden -->
Returns whether the language should be hidden from the user.

# Returns

`true` if the language should be hidden, `false` otherwise.
<!-- trait LanguageExt::fn get_id -->
Returns the ID of the language. The ID is not locale-dependent.
The returned string is owned by `self` and should not be freed
or modified.

# Returns

the ID of `self`.
<!-- trait LanguageExt::fn get_metadata -->
## `name`
metadata property name.

# Returns

value of property `name` stored in
the metadata of `self` or `None` if language does not contain the
specified metadata property.
The returned string is owned by `self` and should not be freed
or modified.
<!-- trait LanguageExt::fn get_mime_types -->
Returns the mime types associated to this language. This is just
an utility wrapper around `LanguageExt::get_metadata` to
retrieve the "mimetypes" metadata property and split it into an
array.

# Returns


a newly-allocated `None` terminated array containing the mime types
or `None` if no mime types are found.
The returned array must be freed with `g_strfreev`.
<!-- trait LanguageExt::fn get_name -->
Returns the localized name of the language.
The returned string is owned by `self` and should not be freed
or modified.

# Returns

the name of `self`.
<!-- trait LanguageExt::fn get_section -->
Returns the localized section of the language.
Each language belong to a section (ex. HTML belogs to the
Markup section).
The returned string is owned by `self` and should not be freed
or modified.

# Returns

the section of `self`.
<!-- trait LanguageExt::fn get_style_fallback -->
Returns the ID of the style to use if the specified `style_id`
is not present in the current style scheme.

Feature: `v3_4`

## `style_id`
a style ID.

# Returns

the ID of the style to use if the
specified `style_id` is not present in the current style scheme or `None`
if the style has no fallback defined.
The returned string is owned by the `self` and must not be modified.
<!-- trait LanguageExt::fn get_style_ids -->
Returns the ids of the styles defined by this `self`.

# Returns


a newly-allocated `None` terminated array containing ids of the
styles defined by this `self` or `None` if no style is defined.
The returned array must be freed with `g_strfreev`.
<!-- trait LanguageExt::fn get_style_name -->
Returns the name of the style with ID `style_id` defined by this `self`.
## `style_id`
a style ID.

# Returns

the name of the style with ID `style_id`
defined by this `self` or `None` if the style has no name or there is no
style with ID `style_id` defined by this `self`.
The returned string is owned by the `self` and must not be modified.
<!-- struct LanguageManager -->


# Implements

[`LanguageManagerExt`](trait.LanguageManagerExt.html)
<!-- trait LanguageManagerExt -->
Trait containing all `LanguageManager` methods.

# Implementors

[`LanguageManager`](struct.LanguageManager.html)
<!-- impl LanguageManager::fn new -->
Creates a new language manager. If you do not need more than one language
manager or a private language manager instance then use
`LanguageManager::get_default` instead.

# Returns

a new `LanguageManager`.
<!-- impl LanguageManager::fn get_default -->
Returns the default `LanguageManager` instance.

# Returns

a `LanguageManager`.
Return value is owned by `View` library and must not be unref'ed.
<!-- trait LanguageManagerExt::fn get_language -->
Gets the `Language` identified by the given `id` in the language
manager.
## `id`
a language id.

# Returns

a `Language`, or `None`
if there is no language identified by the given `id`. Return value is
owned by `self` and should not be freed.
<!-- trait LanguageManagerExt::fn get_language_ids -->
Returns the ids of the available languages.

# Returns


a `None`-terminated array of strings containing the ids of the available
languages or `None` if no language is available.
The array is sorted alphabetically according to the language name.
The array is owned by `self` and must not be modified.
<!-- trait LanguageManagerExt::fn get_search_path -->
Gets the list directories where `self` looks for language files.

# Returns

`None`-terminated array
containg a list of language files directories.
The array is owned by `self` and must not be modified.
<!-- trait LanguageManagerExt::fn guess_language -->
Picks a `Language` for given file name and content type,
according to the information in lang files. Either `filename` or
`content_type` may be `None`. This function can be used as follows:

`<informalexample>``<programlisting>`
 `Language` *lang;
 lang = gtk_source_language_manager_guess_language (filename, NULL);
 gtk_source_buffer_set_language (buffer, lang);
`</programlisting>``</informalexample>`

or

`<informalexample>``<programlisting>`
 `Language` *lang = NULL;
 gboolean result_uncertain;
 gchar *content_type;

 content_type = g_content_type_guess (filename, NULL, 0, &result_uncertain);
 if (result_uncertain)
 {
 g_free (content_type);
 content_type = NULL;
 }

 lang = gtk_source_language_manager_guess_language (manager, filename, content_type);
 gtk_source_buffer_set_language (buffer, lang);

 g_free (content_type);
`</programlisting>``</informalexample>`

etc. Use `LanguageExt::get_mime_types` and `LanguageExt::get_globs`
if you need full control over file -> language mapping.

Feature: `v2_4`

## `filename`
a filename in Glib filename encoding, or `None`.
## `content_type`
a content type (as in GIO API), or `None`.

# Returns

a `Language`, or `None` if there
is no suitable language for given `filename` and/or `content_type`. Return
value is owned by `self` and should not be freed.
<!-- trait LanguageManagerExt::fn set_search_path -->
Sets the list of directories where the `self` looks for
language files.
If `dirs` is `None`, the search path is reset to default.

`<note>`
 `<para>`
 At the moment this function can be called only before the
 language files are loaded for the first time. In practice
 to set a custom search path for a `LanguageManager`,
 you have to call this function right after creating it.
 `</para>`
`</note>`
## `dirs`

a `None`-terminated array of strings or `None`.
<!-- struct Map -->


Feature: `v3_18`

# Implements

[`MapExt`](trait.MapExt.html), [`ViewExt`](trait.ViewExt.html), [`TextViewExt`](trait.TextViewExt.html), [`WidgetExt`](trait.WidgetExt.html)
<!-- trait MapExt -->
Trait containing all `Map` methods.

Feature: `v3_18`

# Implementors

[`Map`](struct.Map.html)
<!-- impl Map::fn new -->
Creates a new `Map`.

Feature: `v3_18`


# Returns

a new `Map`.
<!-- trait MapExt::fn get_view -->
Gets the `Map:view` property, which is the view this widget is mapping.

Feature: `v3_18`


# Returns

a `View` or `None`.
<!-- trait MapExt::fn set_view -->
Sets the view that `self` will be doing the mapping to.

Feature: `v3_18`

## `view`
a `View`
<!-- struct Mark -->


Feature: `v2_2`

# Implements

[`MarkExt`](trait.MarkExt.html), [`TextMarkExt`](trait.TextMarkExt.html)
<!-- trait MarkExt -->
Trait containing all `Mark` methods.

Feature: `v2_2`

# Implementors

[`Mark`](struct.Mark.html)
<!-- impl Mark::fn new -->
Creates a text mark. Add it to a buffer using `gtk::TextBufferExt::add_mark`.
If name is NULL, the mark is anonymous; otherwise, the mark can be retrieved
by name using `gtk::TextBufferExt::get_mark`.
Normally marks are created using the utility function
`BufferExt::create_source_mark`.

Feature: `v2_2`

## `name`
Name of the `Mark`, can be NULL when not using a name
## `category`
is used to classify marks according to common characteristics
(e.g. all the marks representing a bookmark could belong to the "bookmark"
category, or all the marks representing a compilation error could belong to
"error" category).

# Returns

a new `Mark` that can be added using `gtk::TextBufferExt::add_mark`.
<!-- trait MarkExt::fn get_category -->
Returns the mark category.

Feature: `v2_2`


# Returns

the category of the `Mark`.
<!-- trait MarkExt::fn next -->
Returns the next `Mark` in the buffer or `None` if the mark
was not added to a buffer. If there is no next mark, `None` will be returned.

If `category` is `None`, looks for marks of any category.

Feature: `v2_2`

## `category`
a string specifying the mark category, or `None`.

# Returns

the next `Mark`, or `None`.
<!-- trait MarkExt::fn prev -->
Returns the previous `Mark` in the buffer or `None` if the mark
was not added to a buffer. If there is no previous mark, `None` is returned.

If `category` is `None`, looks for marks of any category

Feature: `v2_2`

## `category`
a string specifying the mark category, or `None`.

# Returns

the previous `Mark`, or `None`.
<!-- struct MarkAttributes -->


Feature: `v2_2`

# Implements

[`MarkAttributesExt`](trait.MarkAttributesExt.html)
<!-- trait MarkAttributesExt -->
Trait containing all `MarkAttributes` methods.

Feature: `v2_2`

# Implementors

[`MarkAttributes`](struct.MarkAttributes.html)
<!-- impl MarkAttributes::fn new -->
Creates a new source mark attributes.

# Returns

a new source mark attributes.
<!-- trait MarkAttributesExt::fn get_background -->
Stores background color in `background`.
## `background`
a `gdk::RGBA`.

# Returns

whether background color for `self` was set.
<!-- trait MarkAttributesExt::fn get_gicon -->
Gets a `gio::Icon` to be used as a base for rendered icon. Note that the icon can
be `None` if it wasn't set earlier.

# Returns

An icon. The icon belongs to `self` and should
not be unreffed.
<!-- trait MarkAttributesExt::fn get_icon_name -->
Gets a name of an icon to be used as a base for rendered icon. Note that the
icon name can be `None` if it wasn't set earlier.

# Returns

An icon name. The string belongs to `self` and
should not be freed.
<!-- trait MarkAttributesExt::fn get_pixbuf -->
Gets a `gdk_pixbuf::Pixbuf` to be used as a base for rendered icon. Note that the
pixbuf can be `None` if it wasn't set earlier.

# Returns

A pixbuf. The pixbuf belongs to `self` and
should not be unreffed.
<!-- trait MarkAttributesExt::fn get_stock_id -->
Gets a stock id of an icon used by this attributes. Note that the stock id can
be `None` if it wasn't set earlier.

# Deprecated since 3.10

Don't use this function.

# Returns

Stock id. Returned string is owned by `self` and
shouldn't be freed.
<!-- trait MarkAttributesExt::fn get_tooltip_markup -->
Queries for a tooltip by emitting
a `MarkAttributes::query-tooltip-markup` signal. The tooltip may contain
a markup.
## `mark`
a `Mark`.

# Returns

A tooltip. The returned string should be freed by
using `g_free` when done with it.
<!-- trait MarkAttributesExt::fn get_tooltip_text -->
Queries for a tooltip by emitting
a `MarkAttributes::query-tooltip-text` signal. The tooltip is a plain
text.
## `mark`
a `Mark`.

# Returns

A tooltip. The returned string should be freed by
using `g_free` when done with it.
<!-- trait MarkAttributesExt::fn render_icon -->
Renders an icon of given size. The base of the icon is set by the last call
to one of: `MarkAttributesExt::set_pixbuf`,
`MarkAttributesExt::set_gicon`,
`MarkAttributesExt::set_icon_name` or
`MarkAttributesExt::set_stock_id`. `size` cannot be lower than 1.
## `widget`
widget of which style settings may be used.
## `size`
size of the rendered icon.

# Returns

A rendered pixbuf. The pixbuf belongs to `self`
and should not be unreffed.
<!-- trait MarkAttributesExt::fn set_background -->
Sets background color to the one given in `background`.
## `background`
a `gdk::RGBA`.
<!-- trait MarkAttributesExt::fn set_gicon -->
Sets an icon to be used as a base for rendered icon.
## `gicon`
a `gio::Icon` to be used.
<!-- trait MarkAttributesExt::fn set_icon_name -->
Sets a name of an icon to be used as a base for rendered icon.
## `icon_name`
name of an icon to be used.
<!-- trait MarkAttributesExt::fn set_pixbuf -->
Sets a pixbuf to be used as a base for rendered icon.
## `pixbuf`
a `gdk_pixbuf::Pixbuf` to be used.
<!-- trait MarkAttributesExt::fn set_stock_id -->
Sets stock id to be used as a base for rendered icon.

# Deprecated since 3.10

Don't use this function.
## `stock_id`
a stock id.
<!-- enum NewlineType -->
<!-- enum NewlineType::variant Lf -->
line feed, used on UNIX.
<!-- enum NewlineType::variant Cr -->
carriage return, used on Mac.
<!-- enum NewlineType::variant CrLf -->
carriage return followed by a line feed, used
 on Windows.

Feature: `v3_14`

<!-- struct PrintCompositor -->


# Implements

[`PrintCompositorExt`](trait.PrintCompositorExt.html)
<!-- trait PrintCompositorExt -->
Trait containing all `PrintCompositor` methods.

# Implementors

[`PrintCompositor`](struct.PrintCompositor.html)
<!-- impl PrintCompositor::fn new -->
Creates a new print compositor that can be used to print `buffer`.

Feature: `v2_2`

## `buffer`
the `Buffer` to print.

# Returns

a new print compositor object.
<!-- impl PrintCompositor::fn new_from_view -->
Creates a new print compositor that can be used to print the buffer
associated with `view`.
This constructor sets some configuration properties to make the
printed output match `view` as much as possible. The properties set are
`PrintCompositor:tab-width`, `PrintCompositor:highlight-syntax`,
`PrintCompositor:wrap-mode`, `PrintCompositor:body-font-name` and
`PrintCompositor:print-line-numbers`.

Feature: `v2_2`

## `view`
a `View` to get configuration from.

# Returns

a new print compositor object.
<!-- trait PrintCompositorExt::fn draw_page -->
Draw page `page_nr` for printing on the the Cairo context encapsuled in `context`.

This method has been designed to be called in the handler of the `gtk::PrintOperation::draw_page` signal
as shown in the following example:

`<informalexample>``<programlisting>`
// Signal handler for the `gtk::PrintOperation`::draw_page signal

static void
draw_page (`gtk::PrintOperation` *operation,
 `gtk::PrintContext` *context,
 gint page_nr,
 gpointer user_data)
{
 `PrintCompositor` *compositor;

 compositor = GTK_SOURCE_PRINT_COMPOSITOR (user_data);

 gtk_source_print_compositor_draw_page (compositor,
 context,
 page_nr);
}
`</programlisting>``</informalexample>`
## `context`
the `gtk::PrintContext` encapsulating the context information that is required when
 drawing the page for printing.
## `page_nr`
the number of the page to print.
<!-- trait PrintCompositorExt::fn get_body_font_name -->
Returns the name of the font used to print the text body. The returned string
must be freed with `g_free`.

Feature: `v2_2`


# Returns

a new string containing the name of the font used to print the
text body.
<!-- trait PrintCompositorExt::fn get_bottom_margin -->
Gets the bottom margin in units of `unit`.

Feature: `v2_2`

## `unit`
the unit for the return value.

# Returns

the bottom margin.
<!-- trait PrintCompositorExt::fn get_buffer -->
Gets the `Buffer` associated with the compositor. The returned
object reference is owned by the compositor object and
should not be unreferenced.

Feature: `v2_2`


# Returns

the `Buffer` associated with the compositor.
<!-- trait PrintCompositorExt::fn get_footer_font_name -->
Returns the name of the font used to print the page footer.
The returned string must be freed with `g_free`.

Feature: `v2_2`


# Returns

a new string containing the name of the font used to print
the page footer.
<!-- trait PrintCompositorExt::fn get_header_font_name -->
Returns the name of the font used to print the page header.
The returned string must be freed with `g_free`.

Feature: `v2_2`


# Returns

a new string containing the name of the font used to print
the page header.
<!-- trait PrintCompositorExt::fn get_highlight_syntax -->
Determines whether the printed text will be highlighted according to the
buffer rules. Note that highlighting will happen
only if the buffer to print has highlighting activated.

Feature: `v2_2`


# Returns

`true` if the printed output will be highlighted.
<!-- trait PrintCompositorExt::fn get_left_margin -->
Gets the left margin in units of `unit`.

Feature: `v2_2`

## `unit`
the unit for the return value.

# Returns

the left margin
<!-- trait PrintCompositorExt::fn get_line_numbers_font_name -->
Returns the name of the font used to print line numbers on the left margin.
The returned string must be freed with `g_free`.

Feature: `v2_2`


# Returns

a new string containing the name of the font used to print
line numbers on the left margin.
<!-- trait PrintCompositorExt::fn get_n_pages -->
Returns the number of pages in the document or `<code>`-1`</code>` if the
document has not been completely paginated.

Feature: `v2_2`


# Returns

the number of pages in the document or `<code>`-1`</code>` if the
document has not been completely paginated.
<!-- trait PrintCompositorExt::fn get_pagination_progress -->
Returns the current fraction of the document pagination that has been completed.

Feature: `v2_2`


# Returns

a fraction from 0.0 to 1.0 inclusive.
<!-- trait PrintCompositorExt::fn get_print_footer -->
Determines if a footer is set to be printed for each page. A
footer will be printed if this function returns `true`
`<emphasis>`and`</emphasis>` some format strings have been specified
with `PrintCompositorExt::set_footer_format`.

Feature: `v2_2`


# Returns

`true` if the footer is set to be printed.
<!-- trait PrintCompositorExt::fn get_print_header -->
Determines if a header is set to be printed for each page. A
header will be printed if this function returns `true`
`<emphasis>`and`</emphasis>` some format strings have been specified
with `PrintCompositorExt::set_header_format`.

Feature: `v2_2`


# Returns

`true` if the header is set to be printed.
<!-- trait PrintCompositorExt::fn get_print_line_numbers -->
Returns the interval used for line number printing. If the
value is 0, no line numbers will be printed. The default value is
1 (i.e. numbers printed in all lines).

Feature: `v2_2`


# Returns

the interval of printed line numbers.
<!-- trait PrintCompositorExt::fn get_right_margin -->
Gets the right margin in units of `unit`.

Feature: `v2_2`

## `unit`
the unit for the return value.

# Returns

the right margin.
<!-- trait PrintCompositorExt::fn get_tab_width -->
Returns the width of tabulation in characters for printed text.

Feature: `v2_2`


# Returns

width of tab.
<!-- trait PrintCompositorExt::fn get_top_margin -->
Gets the top margin in units of `unit`.

Feature: `v2_2`

## `unit`
the unit for the return value.

# Returns

the top margin.
<!-- trait PrintCompositorExt::fn get_wrap_mode -->
Gets the line wrapping mode for the printed text.

Feature: `v2_2`


# Returns

the line wrap mode.
<!-- trait PrintCompositorExt::fn paginate -->
Paginate the document associated with the `self`.

In order to support non-blocking pagination, document is paginated in small chunks.
Each time `PrintCompositorExt::paginate` is invoked, a chunk of the document
is paginated. To paginate the entire document, `PrintCompositorExt::paginate`
must be invoked multiple times.
It returns `true` if the document has been completely paginated, otherwise it returns `false`.

This method has been designed to be invoked in the handler of the `gtk::PrintOperation::paginate` signal,
as shown in the following example:

`<informalexample>``<programlisting>`
// Signal handler for the `gtk::PrintOperation`::paginate signal

static gboolean
paginate (`gtk::PrintOperation` *operation,
 `gtk::PrintContext` *context,
 gpointer user_data)
{
 `PrintCompositor` *compositor;

 compositor = GTK_SOURCE_PRINT_COMPOSITOR (user_data);

 if (gtk_source_print_compositor_paginate (compositor, context))
 {
 gint n_pages;

 n_pages = gtk_source_print_compositor_get_n_pages (compositor);
 gtk_print_operation_set_n_pages (operation, n_pages);

 return TRUE;
 }

 return FALSE;
}
`</programlisting>``</informalexample>`

If you don't need to do pagination in chunks, you can simply do it all in the
`gtk::PrintOperation::begin-print` handler, and set the number of pages from there, like
in the following example:

`<informalexample>``<programlisting>`
// Signal handler for the `gtk::PrintOperation`::begin-print signal

static void
begin_print (`gtk::PrintOperation` *operation,
 `gtk::PrintContext` *context,
 gpointer user_data)
{
 `PrintCompositor` *compositor;
 gint n_pages;

 compositor = GTK_SOURCE_PRINT_COMPOSITOR (user_data);

 while (!gtk_source_print_compositor_paginate (compositor, context));

 n_pages = gtk_source_print_compositor_get_n_pages (compositor);
 gtk_print_operation_set_n_pages (operation, n_pages);
}
`</programlisting>``</informalexample>`

Feature: `v2_2`

## `context`
the `gtk::PrintContext` whose parameters (e.g. paper size, print margins, etc.)
are used by the the `self` to paginate the document.

# Returns

`true` if the document has been completely paginated, `false` otherwise.
<!-- trait PrintCompositorExt::fn set_body_font_name -->
Sets the default font for the printed text.

`font_name` should be a
string representation of a font description Pango can understand.
(e.g. &quot;Monospace 10&quot;). See `pango::FontDescription::from_string`
for a description of the format of the string representation.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `font_name`
the name of the default font for the body text.
<!-- trait PrintCompositorExt::fn set_bottom_margin -->
Sets the bottom margin used by `self`.

Feature: `v2_2`

## `margin`
the new bottom margin in units of `unit`.
## `unit`
the units for `margin`.
<!-- trait PrintCompositorExt::fn set_footer_font_name -->
Sets the font for printing the page footer. If
`None` is supplied, the default font (i.e. the one being used for the
text) will be used instead.

`font_name` should be a
string representation of a font description Pango can understand.
(e.g. &quot;Monospace 10&quot;). See `pango::FontDescription::from_string`
for a description of the format of the string representation.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `font_name`
the name of the font for the footer text, or `None`.
<!-- trait PrintCompositorExt::fn set_footer_format -->
See `PrintCompositorExt::set_header_format` for more information
about the parameters.

Feature: `v2_2`

## `separator`
`true` if you want a separator line to be printed.
## `left`
a format string to print on the left of the footer.
## `center`
a format string to print on the center of the footer.
## `right`
a format string to print on the right of the footer.
<!-- trait PrintCompositorExt::fn set_header_font_name -->
Sets the font for printing the page header. If
`None` is supplied, the default font (i.e. the one being used for the
text) will be used instead.

`font_name` should be a
string representation of a font description Pango can understand.
(e.g. &quot;Monospace 10&quot;). See `pango::FontDescription::from_string`
for a description of the format of the string representation.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `font_name`
the name of the font for header text, or `None`.
<!-- trait PrintCompositorExt::fn set_header_format -->
Sets strftime like header format strings, to be printed on the
left, center and right of the top of each page. The strings may
include strftime(3) codes which will be expanded at print time.
A subset of `strftime` codes are accepted, see `glib::DateTime::format`
for more details on the accepted format specifiers.
Additionally the following format specifiers are accepted:
- `N`: the page number
- `Q`: the page count.

`separator` specifies if a solid line should be drawn to separate
the header from the document text.

If `None` is given for any of the three arguments, that particular
string will not be printed.

For the header to be printed, in
addition to specifying format strings, you need to enable header
printing with `PrintCompositorExt::set_print_header`.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `separator`
`true` if you want a separator line to be printed.
## `left`
a format string to print on the left of the header.
## `center`
a format string to print on the center of the header.
## `right`
a format string to print on the right of the header.
<!-- trait PrintCompositorExt::fn set_highlight_syntax -->
Sets whether the printed text will be highlighted according to the
buffer rules. Both color and font style are applied.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `highlight`
whether syntax should be highlighted.
<!-- trait PrintCompositorExt::fn set_left_margin -->
Sets the left margin used by `self`.

Feature: `v2_2`

## `margin`
the new left margin in units of `unit`.
## `unit`
the units for `margin`.
<!-- trait PrintCompositorExt::fn set_line_numbers_font_name -->
Sets the font for printing line numbers on the left margin. If
`None` is supplied, the default font (i.e. the one being used for the
text) will be used instead.

`font_name` should be a
string representation of a font description Pango can understand.
(e.g. &quot;Monospace 10&quot;). See `pango::FontDescription::from_string`
for a description of the format of the string representation.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `font_name`
the name of the font for line numbers, or `None`.
<!-- trait PrintCompositorExt::fn set_print_footer -->
Sets whether you want to print a footer in each page. The
footer consists of three pieces of text and an optional line
separator, configurable with
`PrintCompositorExt::set_footer_format`.

Note that by default the footer format is unspecified, and if it's
empty it will not be printed, regardless of this setting.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `print`
`true` if you want the footer to be printed.
<!-- trait PrintCompositorExt::fn set_print_header -->
Sets whether you want to print a header in each page. The
header consists of three pieces of text and an optional line
separator, configurable with
`PrintCompositorExt::set_header_format`.

Note that by default the header format is unspecified, and if it's
empty it will not be printed, regardless of this setting.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `print`
`true` if you want the header to be printed.
<!-- trait PrintCompositorExt::fn set_print_line_numbers -->
Sets the interval for printed line numbers. If `interval` is 0 no
numbers will be printed. If greater than 0, a number will be
printed every `interval` lines (i.e. 1 will print all line numbers).

Maximum accepted value for `interval` is 100.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `interval`
interval for printed line numbers.
<!-- trait PrintCompositorExt::fn set_right_margin -->
Sets the right margin used by `self`.

Feature: `v2_2`

## `margin`
the new right margin in units of `unit`.
## `unit`
the units for `margin`.
<!-- trait PrintCompositorExt::fn set_tab_width -->
Sets the width of tabulation in characters for printed text.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `width`
width of tab in characters.
<!-- trait PrintCompositorExt::fn set_top_margin -->
Sets the top margin used by `self`.

Feature: `v2_2`

## `margin`
the new top margin in units of `unit`
## `unit`
the units for `margin`
<!-- trait PrintCompositorExt::fn set_wrap_mode -->
Sets the line wrapping mode for the printed text.

This function cannot be called anymore after the first call to the
`PrintCompositorExt::paginate` function.

Feature: `v2_2`

## `wrap_mode`
a `gtk::WrapMode`.
<!-- struct SearchContext -->


Feature: `v3_10`

# Implements

[`SearchContextExt`](trait.SearchContextExt.html)
<!-- trait SearchContextExt -->
Trait containing all `SearchContext` methods.

Feature: `v3_10`

# Implementors

[`SearchContext`](struct.SearchContext.html)
<!-- impl SearchContext::fn new -->
Creates a new search context, associated with `buffer`, and customized with
`settings`. If `settings` is `None`, a new `SearchSettings` object will
be created, that you can retrieve with
`SearchContextExt::get_settings`.

Feature: `v3_10`

## `buffer`
a `Buffer`.
## `settings`
a `SearchSettings`, or `None`.

# Returns

a new search context.
<!-- trait SearchContextExt::fn backward -->
Synchronous backward search. It is recommended to use the asynchronous
functions instead, to not block the user interface. However, if you are sure
that the `buffer` is small, this function is more convenient to use.

Feature: `v3_10`

## `iter`
start of search.
## `match_start`
return location for start of match, or `None`.
## `match_end`
return location for end of match, or `None`.

# Returns

whether a match was found.
<!-- trait SearchContextExt::fn backward_async -->
Asynchronous backward search. See the `gio::AsyncResult` documentation to know
how to use this function.

If the operation is cancelled, the `callback` will only be called if
`cancellable` was not `None`. `SearchContextExt::backward_async` takes
ownership of `cancellable`, so you can unref it after calling this function.

Feature: `v3_10`

## `iter`
start of search.
## `cancellable`
a `gio::Cancellable`, or `None`.
## `callback`
a `GAsyncReadyCallback` to call when the operation is finished.
## `user_data`
the data to pass to the `callback` function.
<!-- trait SearchContextExt::fn backward_finish -->
Finishes a backward search started with
`SearchContextExt::backward_async`.

Feature: `v3_10`

## `result`
a `gio::AsyncResult`.
## `match_start`
return location for start of match, or `None`.
## `match_end`
return location for end of match, or `None`.

# Returns

whether a match was found.
<!-- trait SearchContextExt::fn forward -->
Synchronous forward search. It is recommended to use the asynchronous
functions instead, to not block the user interface. However, if you are sure
that the `buffer` is small, this function is more convenient to use.

Feature: `v3_10`

## `iter`
start of search.
## `match_start`
return location for start of match, or `None`.
## `match_end`
return location for end of match, or `None`.

# Returns

whether a match was found.
<!-- trait SearchContextExt::fn forward_async -->
Asynchronous forward search. See the `gio::AsyncResult` documentation to know
how to use this function.

If the operation is cancelled, the `callback` will only be called if
`cancellable` was not `None`. `SearchContextExt::forward_async` takes
ownership of `cancellable`, so you can unref it after calling this function.

Feature: `v3_10`

## `iter`
start of search.
## `cancellable`
a `gio::Cancellable`, or `None`.
## `callback`
a `GAsyncReadyCallback` to call when the operation is finished.
## `user_data`
the data to pass to the `callback` function.
<!-- trait SearchContextExt::fn forward_finish -->
Finishes a forward search started with
`SearchContextExt::forward_async`.

Feature: `v3_10`

## `result`
a `gio::AsyncResult`.
## `match_start`
return location for start of match, or `None`.
## `match_end`
return location for end of match, or `None`.

# Returns

whether a match was found.
<!-- trait SearchContextExt::fn get_buffer -->

Feature: `v3_10`


# Returns

the associated buffer.
<!-- trait SearchContextExt::fn get_highlight -->

Feature: `v3_10`


# Returns

whether to highlight the search occurrences.
<!-- trait SearchContextExt::fn get_match_style -->

Feature: `v3_16`


# Returns

the `Style` to apply on search matches.
<!-- trait SearchContextExt::fn get_occurrence_position -->
Gets the position of a search occurrence. If the buffer is not already fully
scanned, the position may be unknown, and -1 is returned. If 0 is returned,
it means that this part of the buffer has already been scanned, and that
`match_start` and `match_end` don't delimit an occurrence.

Feature: `v3_10`

## `match_start`
the start of the occurrence.
## `match_end`
the end of the occurrence.

# Returns

the position of the search occurrence. The first occurrence has the
position 1 (not 0). Returns 0 if `match_start` and `match_end` don't delimit
an occurrence. Returns -1 if the position is not yet known.
<!-- trait SearchContextExt::fn get_occurrences_count -->
Gets the total number of search occurrences. If the buffer is not already
fully scanned, the total number of occurrences is unknown, and -1 is
returned.

Feature: `v3_10`


# Returns

the total number of search occurrences, or -1 if unknown.
<!-- trait SearchContextExt::fn get_regex_error -->
Regular expression patterns must follow certain rules. If
`SearchSettings:search-text` breaks a rule, the error can be retrieved
with this function. The error domain is `G_REGEX_ERROR`.

Free the return value with `glib::Error::free`.

Feature: `v3_10`


# Returns

the `glib::Error`, or `None` if the pattern is valid.
<!-- trait SearchContextExt::fn get_settings -->

Feature: `v3_10`


# Returns

the search settings.
<!-- trait SearchContextExt::fn replace -->
Replaces a search match by another text. If `match_start` and `match_end`
doesn't correspond to a search match, `false` is returned.

For a regular expression replacement, you can check if `replace` is valid by
calling `glib::Regex::check_replacement`. The `replace` text can contain
backreferences; read the `glib::Regex::replace` documentation for more details.

Feature: `v3_10`

## `match_start`
the start of the match to replace.
## `match_end`
the end of the match to replace.
## `replace`
the replacement text.
## `replace_length`
the length of `replace` in bytes, or -1.

# Returns

whether the match has been replaced.
<!-- trait SearchContextExt::fn replace_all -->
Replaces all search matches by another text. It is a synchronous function, so
it can block the user interface.

For a regular expression replacement, you can check if `replace` is valid by
calling `glib::Regex::check_replacement`. The `replace` text can contain
backreferences; read the `glib::Regex::replace` documentation for more details.

Feature: `v3_10`

## `replace`
the replacement text.
## `replace_length`
the length of `replace` in bytes, or -1.

# Returns

the number of replaced matches.
<!-- trait SearchContextExt::fn set_highlight -->
Enables or disables the search occurrences highlighting.

Feature: `v3_10`

## `highlight`
the setting.
<!-- trait SearchContextExt::fn set_match_style -->
Set the style to apply on search matches. If `match_style` is `None`, default
theme's scheme 'match-style' will be used.
To enable or disable the search highlighting, use
`SearchContextExt::set_highlight`.

Feature: `v3_16`

## `match_style`
a `Style`.
<!-- trait SearchContextExt::fn set_settings -->
Associate a `SearchSettings` with the search context. If `settings` is
`None`, a new one will be created.

The search context holds a reference to `settings`.

Feature: `v3_10`

## `settings`
the new `SearchSettings`, or `None`.
<!-- struct SearchSettings -->


Feature: `v3_10`

# Implements

[`SearchSettingsExt`](trait.SearchSettingsExt.html)
<!-- trait SearchSettingsExt -->
Trait containing all `SearchSettings` methods.

Feature: `v3_10`

# Implementors

[`SearchSettings`](struct.SearchSettings.html)
<!-- impl SearchSettings::fn new -->
Creates a new search settings object.

Feature: `v3_10`


# Returns

a new search settings object.
<!-- trait SearchSettingsExt::fn get_at_word_boundaries -->

Feature: `v3_10`


# Returns

whether to search at word boundaries.
<!-- trait SearchSettingsExt::fn get_case_sensitive -->

Feature: `v3_10`


# Returns

whether the search is case sensitive.
<!-- trait SearchSettingsExt::fn get_regex_enabled -->

Feature: `v3_10`


# Returns

whether to search by regular expressions.
<!-- trait SearchSettingsExt::fn get_search_text -->
Gets the text to search. The return value must not be freed.

You may be interested to call `gtk_source_utils_escape_search_text` after
this function.

Feature: `v3_10`


# Returns

the text to search, or `None` if the search is disabled.
<!-- trait SearchSettingsExt::fn get_wrap_around -->

Feature: `v3_10`


# Returns

whether to wrap around the search.
<!-- trait SearchSettingsExt::fn set_at_word_boundaries -->
Change whether the search is done at word boundaries. If `at_word_boundaries`
is `true`, a search match must start and end a word. The match can span
multiple words. See also `gtk::TextIter::starts_word` and
`gtk::TextIter::ends_word`.

Feature: `v3_10`

## `at_word_boundaries`
the setting.
<!-- trait SearchSettingsExt::fn set_case_sensitive -->
Enables or disables the case sensitivity for the search.

Feature: `v3_10`

## `case_sensitive`
the setting.
<!-- trait SearchSettingsExt::fn set_regex_enabled -->
Enables or disables whether to search by regular expressions.
If enabled, the `SearchSettings:search-text` property contains the
pattern of the regular expression.

Feature: `v3_10`

## `regex_enabled`
the setting.
<!-- trait SearchSettingsExt::fn set_search_text -->
Sets the text to search. If `text` is `None` or is empty, the search will be
disabled. A copy of `text` will be made, so you can safely free `text` after
a call to this function.

You may be interested to call `gtk_source_utils_unescape_search_text` before
this function.

Feature: `v3_10`

## `search_text`
the nul-terminated text to search, or `None` to disable the search.
<!-- trait SearchSettingsExt::fn set_wrap_around -->
Enables or disables the wrap around search. If `wrap_around` is `true`, the
forward search continues at the beginning of the buffer if no search
occurrences are found. Similarly, the backward search continues to search at
the end of the buffer.

Feature: `v3_10`

## `wrap_around`
the setting.
<!-- enum SmartHomeEndType -->
<!-- enum SmartHomeEndType::variant Disabled -->
smart-home-end disabled.
<!-- enum SmartHomeEndType::variant Before -->
move to the first/last
non-whitespace character on the first press of the HOME/END keys and
to the beginning/end of the line on the second press.
<!-- enum SmartHomeEndType::variant After -->
move to the beginning/end of the
line on the first press of the HOME/END keys and to the first/last
non-whitespace character on the second press.
<!-- enum SmartHomeEndType::variant Always -->
always move to the first/last
non-whitespace character when the HOME/END keys are pressed.
<!-- struct Style -->


# Implements

[`StyleExt`](trait.StyleExt.html)
<!-- trait StyleExt -->
Trait containing all `Style` methods.

# Implementors

[`Style`](struct.Style.html)
<!-- trait StyleExt::fn copy -->
Creates a copy of `self`, that is a new `Style` instance which
has the same attributes set.

Feature: `v2_0`


# Returns

copy of `self`, call `gobject::Object::unref`
when you are done with it.
<!-- struct StyleScheme -->


Feature: `v2_0`

# Implements

[`StyleSchemeExt`](trait.StyleSchemeExt.html)
<!-- trait StyleSchemeExt -->
Trait containing all `StyleScheme` methods.

Feature: `v2_0`

# Implementors

[`StyleScheme`](struct.StyleScheme.html)
<!-- trait StyleSchemeExt::fn get_authors -->

Feature: `v2_0`


# Returns

a
`None`-terminated array containing the `self` authors or `None` if
no author is specified by the style scheme.
<!-- trait StyleSchemeExt::fn get_description -->

Feature: `v2_0`


# Returns

`self` description (if defined), or `None`.
<!-- trait StyleSchemeExt::fn get_filename -->

Feature: `v2_0`


# Returns

`self` file name if the scheme was created
parsing a style scheme file or `None` in the other cases.
<!-- trait StyleSchemeExt::fn get_id -->

Feature: `v2_0`


# Returns

`self` id.
<!-- trait StyleSchemeExt::fn get_name -->

Feature: `v2_0`


# Returns

`self` name.
<!-- trait StyleSchemeExt::fn get_style -->

Feature: `v2_0`

## `style_id`
id of the style to retrieve.

# Returns

style which corresponds to `style_id` in
the `self`, or `None` when no style with this name found. It is owned by
`self` and may not be unref'ed.
<!-- struct StyleSchemeChooser -->


Feature: `v3_16`

# Implements

[`StyleSchemeChooserExt`](trait.StyleSchemeChooserExt.html)
<!-- trait StyleSchemeChooserExt -->
Trait containing all `StyleSchemeChooser` methods.

Feature: `v3_16`

# Implementors

[`StyleSchemeChooser`](struct.StyleSchemeChooser.html)
<!-- trait StyleSchemeChooserExt::fn get_style_scheme -->
Gets the currently-selected scheme.

Feature: `v3_16`


# Returns

the currently-selected scheme.
<!-- trait StyleSchemeChooserExt::fn set_style_scheme -->
Sets the scheme.

Feature: `v3_16`

## `scheme`
a `StyleScheme`
<!-- struct StyleSchemeManager -->


# Implements

[`StyleSchemeManagerExt`](trait.StyleSchemeManagerExt.html)
<!-- trait StyleSchemeManagerExt -->
Trait containing all `StyleSchemeManager` methods.

# Implementors

[`StyleSchemeManager`](struct.StyleSchemeManager.html)
<!-- impl StyleSchemeManager::fn new -->
Creates a new style manager. If you do not need more than one style
manager then use `StyleSchemeManager::get_default` instead.

# Returns

a new `StyleSchemeManager`.
<!-- impl StyleSchemeManager::fn get_default -->
Returns the default `StyleSchemeManager` instance.

# Returns

a `StyleSchemeManager`. Return value
is owned by `View` library and must not be unref'ed.
<!-- trait StyleSchemeManagerExt::fn append_search_path -->
Appends `path` to the list of directories where the `self` looks for
style scheme files.
See `StyleSchemeManagerExt::set_search_path` for details.
## `path`
a directory or a filename.
<!-- trait StyleSchemeManagerExt::fn force_rescan -->
Mark any currently cached information about the available style scehems
as invalid. All the available style schemes will be reloaded next time
the `self` is accessed.
<!-- trait StyleSchemeManagerExt::fn get_scheme -->
Looks up style scheme by id.
## `scheme_id`
style scheme id to find.

# Returns

a `StyleScheme` object. Returned value is owned by
`self` and must not be unref'ed.
<!-- trait StyleSchemeManagerExt::fn get_scheme_ids -->
Returns the ids of the available style schemes.

# Returns


a `None`-terminated array of strings containing the ids of the available
style schemes or `None` if no style scheme is available.
The array is sorted alphabetically according to the scheme name.
The array is owned by the `self` and must not be modified.
<!-- trait StyleSchemeManagerExt::fn get_search_path -->
Returns the current search path for the `self`.
See `StyleSchemeManagerExt::set_search_path` for details.

# Returns

a `None`-terminated array
of string containing the search path.
The array is owned by the `self` and must not be modified.
<!-- trait StyleSchemeManagerExt::fn prepend_search_path -->
Prepends `path` to the list of directories where the `self` looks
for style scheme files.
See `StyleSchemeManagerExt::set_search_path` for details.
## `path`
a directory or a filename.
<!-- trait StyleSchemeManagerExt::fn set_search_path -->
Sets the list of directories where the `self` looks for
style scheme files.
If `path` is `None`, the search path is reset to default.
## `path`

a `None`-terminated array of strings or `None`.
<!-- struct Tag -->


# Implements

[`TagExt`](trait.TagExt.html)
<!-- trait TagExt -->
Trait containing all `Tag` methods.

# Implementors

[`Tag`](struct.Tag.html)
<!-- impl Tag::fn new -->
Creates a `Tag`. Configure the tag using object arguments,
i.e. using `gobject::Object::set`.

For usual cases, `BufferExt::create_source_tag` is more convenient to
use.

Feature: `v3_20`

## `name`
tag name, or `None`.

# Returns

a new `Tag`.
<!-- struct UndoManager -->


Feature: `v2_10`

# Implements

[`UndoManagerExt`](trait.UndoManagerExt.html)
<!-- trait UndoManagerExt -->
Trait containing all `UndoManager` methods.

Feature: `v2_10`

# Implementors

[`UndoManager`](struct.UndoManager.html)
<!-- trait UndoManagerExt::fn begin_not_undoable_action -->
Begin a not undoable action on the buffer. All changes between this call
and the call to `UndoManager::end_not_undoable_action` cannot
be undone. This function should be re-entrant.

Feature: `v2_10`

<!-- trait UndoManagerExt::fn can_redo -->
Get whether there are redo operations available.

Feature: `v2_10`


# Returns

`true` if there are redo operations available, `false` otherwise
<!-- trait UndoManagerExt::fn can_redo_changed -->
Emits the `UndoManager::can-redo-changed` signal.

Feature: `v2_10`

<!-- trait UndoManagerExt::fn can_undo -->
Get whether there are undo operations available.

Feature: `v2_10`


# Returns

`true` if there are undo operations available, `false` otherwise
<!-- trait UndoManagerExt::fn can_undo_changed -->
Emits the `UndoManager::can-undo-changed` signal.

Feature: `v2_10`

<!-- trait UndoManagerExt::fn end_not_undoable_action -->
Ends a not undoable action on the buffer.

Feature: `v2_10`

<!-- trait UndoManagerExt::fn redo -->
Perform a single redo. Calling this function when there are no redo operations
available is an error. Use `UndoManager::can_redo` to find out
if there are redo operations available.

Feature: `v2_10`

<!-- trait UndoManagerExt::fn undo -->
Perform a single undo. Calling this function when there are no undo operations
available is an error. Use `UndoManager::can_undo` to find out
if there are undo operations available.

Feature: `v2_10`

<!-- struct View -->


# Implements

[`ViewExt`](trait.ViewExt.html), [`TextViewExt`](trait.TextViewExt.html), [`WidgetExt`](trait.WidgetExt.html)
<!-- trait ViewExt -->
Trait containing all `View` methods.

# Implementors

[`Map`](struct.Map.html), [`View`](struct.View.html)
<!-- impl View::fn new -->
Creates a new `View`. An empty default `Buffer` will be
created for you and can be retrieved with `gtk::TextViewExt::get_buffer`. If you
want to specify your own buffer, consider `View::new_with_buffer`.

# Returns

a new `View`.
<!-- impl View::fn new_with_buffer -->
Creates a new `View` widget displaying the buffer
`buffer`. One buffer can be shared among many widgets.
## `buffer`
a `Buffer`.

# Returns

a new `View`.
<!-- trait ViewExt::fn get_auto_indent -->
Returns whether auto-indentation of text is enabled.

# Returns

`true` if auto indentation is enabled.
<!-- trait ViewExt::fn get_background_pattern -->
Returns the `BackgroundPatternType` specifying if and how
the background pattern should be displayed for this `self`.

Feature: `v3_16`


# Returns

the `BackgroundPatternType`.
<!-- trait ViewExt::fn get_completion -->
Gets the `Completion` associated with `self`.

# Returns


the `Completion` associated with `self`.
<!-- trait ViewExt::fn get_draw_spaces -->
Returns the `DrawSpacesFlags` specifying if and how spaces
should be displayed for this `self`.

# Returns

the `DrawSpacesFlags`, 0 if no spaces should be drawn.
<!-- trait ViewExt::fn get_gutter -->
Returns the `Gutter` object associated with `window_type` for `self`.
Only GTK_TEXT_WINDOW_LEFT and GTK_TEXT_WINDOW_RIGHT are supported,
respectively corresponding to the left and right gutter. The line numbers
and mark category icons are rendered in the left gutter.

Feature: `v2_8`

## `window_type`
the gutter window type.

# Returns

the `Gutter`.
<!-- trait ViewExt::fn get_highlight_current_line -->
Returns whether the current line is highlighted.

# Returns

`true` if the current line is highlighted.
<!-- trait ViewExt::fn get_indent_on_tab -->
Returns whether when the tab key is pressed the current selection
should get indented instead of replaced with the \t character.

# Returns

`true` if the selection is indented when tab is pressed.
<!-- trait ViewExt::fn get_indent_width -->
Returns the number of spaces to use for each step of indent.
See `ViewExt::set_indent_width` for details.

# Returns

indent width.
<!-- trait ViewExt::fn get_insert_spaces_instead_of_tabs -->
Returns whether when inserting a tabulator character it should
be replaced by a group of space characters.

# Returns

`true` if spaces are inserted instead of tabs.
<!-- trait ViewExt::fn get_mark_attributes -->
Gets attributes and priority for the `category`.
## `category`
the category.
## `priority`
place where priority of the category will be stored.

# Returns

`MarkAttributes` for the `category`.
The object belongs to `self`, so it must not be unreffed.
<!-- trait ViewExt::fn get_right_margin_position -->
Gets the position of the right margin in the given `self`.

# Returns

the position of the right margin.
<!-- trait ViewExt::fn get_show_line_marks -->
Returns whether line marks are displayed beside the text.

Feature: `v2_2`


# Returns

`true` if the line marks are displayed.
<!-- trait ViewExt::fn get_show_line_numbers -->
Returns whether line numbers are displayed beside the text.

# Returns

`true` if the line numbers are displayed.
<!-- trait ViewExt::fn get_show_right_margin -->
Returns whether a right margin is displayed.

# Returns

`true` if the right margin is shown.
<!-- trait ViewExt::fn get_smart_backspace -->
Returns `true` if pressing the Backspace key will try to delete spaces
up to the previous tab stop.

Feature: `v3_18`


# Returns

`true` if smart Backspace handling is enabled.
<!-- trait ViewExt::fn get_smart_home_end -->
Returns a `SmartHomeEndType` end value specifying
how the cursor will move when HOME and END keys are pressed.

# Returns

a `SmartHomeEndType` value.
<!-- trait ViewExt::fn get_tab_width -->
Returns the width of tabulation in characters.

# Returns

width of tab.
<!-- trait ViewExt::fn get_visual_column -->
Determines the visual column at `iter` taking into consideration the
`View:tab-width` of `self`.
## `iter`
a position in `self`.

# Returns

the visual column at `iter`.
<!-- trait ViewExt::fn indent_lines -->
Insert one indentation level at the beginning of the
specified lines.

Feature: `v3_16`

## `start`
`gtk::TextIter` of the first line to indent
## `end`
`gtk::TextIter` of the last line to indent
<!-- trait ViewExt::fn set_auto_indent -->
If `true` auto-indentation of text is enabled.

When Enter is pressed to create a new line, the auto-indentation inserts the
same indentation as the previous line. This is `<emphasis>`not`</emphasis>` a
"smart indentation" where an indentation level is added or removed depending
on the context.
## `enable`
whether to enable auto indentation.
<!-- trait ViewExt::fn set_background_pattern -->
Set if and how the background pattern should be displayed.

Feature: `v3_16`

## `background_pattern`
the `BackgroundPatternType`.
<!-- trait ViewExt::fn set_draw_spaces -->
Set if and how the spaces should be visualized. Specifying `flags` as 0 will
disable display of spaces.

For a finer-grained method, there is also the `Tag`'s
`Tag:draw-spaces` property.
## `flags`
`DrawSpacesFlags` specifing how white spaces should
be displayed
<!-- trait ViewExt::fn set_highlight_current_line -->
If `highlight` is `true` the current line will be highlighted.
## `highlight`
whether to highlight the current line.
<!-- trait ViewExt::fn set_indent_on_tab -->
If `true`, when the tab key is pressed when several lines are selected, the
selected lines are indented of one level instead of being replaced with a \t
character. Shift+Tab unindents the selection.

If the first or last line is not selected completely, it is also indented or
unindented.

When the selection doesn't span several lines, the tab key always replaces
the selection with a normal \t character.
## `enable`
whether to indent a block when tab is pressed.
<!-- trait ViewExt::fn set_indent_width -->
Sets the number of spaces to use for each step of indent when the tab key is
pressed. If `width` is -1, the value of the `View:tab-width` property
will be used.

The `View:indent-width` interacts with the
`View:insert-spaces-instead-of-tabs` property and
`View:tab-width`. An example will be clearer: if the
`View:indent-width` is 4 and
`View:tab-width` is 8 and
`View:insert-spaces-instead-of-tabs` is `false`, then pressing the tab
key at the beginning of a line will insert 4 spaces. So far so good. Pressing
the tab key a second time will remove the 4 spaces and insert a \t character
instead (since `View:tab-width` is 8). On the other hand, if
`View:insert-spaces-instead-of-tabs` is `true`, the second tab key
pressed will insert 4 more spaces for a total of 8 spaces in the
`gtk::TextBuffer`.

The test-widget program (available in the `View` repository) may be
useful to better understand the indentation settings (enable the space
drawing!).
## `width`
indent width in characters.
<!-- trait ViewExt::fn set_insert_spaces_instead_of_tabs -->
If `true` a tab key pressed is replaced by a group of space characters. Of
course it is still possible to insert a real \t programmatically with the
`gtk::TextBuffer` API.
## `enable`
whether to insert spaces instead of tabs.
<!-- trait ViewExt::fn set_mark_attributes -->
Sets attributes and priority for the `category`.
## `category`
the category.
## `attributes`
mark attributes.
## `priority`
priority of the category.
<!-- trait ViewExt::fn set_right_margin_position -->
Sets the position of the right margin in the given `self`.
## `pos`
the width in characters where to position the right margin.
<!-- trait ViewExt::fn set_show_line_marks -->
If `true` line marks will be displayed beside the text.

Feature: `v2_2`

## `show`
whether line marks should be displayed.
<!-- trait ViewExt::fn set_show_line_numbers -->
If `true` line numbers will be displayed beside the text.
## `show`
whether line numbers should be displayed.
<!-- trait ViewExt::fn set_show_right_margin -->
If `true` a right margin is displayed.
## `show`
whether to show a right margin.
<!-- trait ViewExt::fn set_smart_backspace -->
When set to `true`, pressing the Backspace key will try to delete spaces
up to the previous tab stop.

Feature: `v3_18`

## `smart_backspace`
whether to enable smart Backspace handling.
<!-- trait ViewExt::fn set_smart_home_end -->
Set the desired movement of the cursor when HOME and END keys
are pressed.
## `smart_home_end`
the desired behavior among `SmartHomeEndType`.
<!-- trait ViewExt::fn set_tab_width -->
Sets the width of tabulation in characters. The `gtk::TextBuffer` still contains
\t characters, but they can take a different visual width in a `View`
widget.
## `width`
width of tab in characters.
<!-- trait ViewExt::fn unindent_lines -->
Removes one indentation level at the beginning of the
specified lines.

Feature: `v3_16`

## `start`
`gtk::TextIter` of the first line to indent
## `end`
`gtk::TextIter` of the last line to indent
