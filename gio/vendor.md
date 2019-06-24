<!-- file * -->
<!-- struct Action -->
`Action` represents a single named action.

The main interface to an action is that it can be activated with
`Action::activate`. This results in the 'activate' signal being
emitted. An activation has a `glib::Variant` parameter (which may be
`None`). The correct type for the parameter is determined by a static
parameter type (which is given at construction time).

An action may optionally have a state, in which case the state may be
set with `Action::change_state`. This call takes a `glib::Variant`. The
correct type for the state is determined by a static state type
(which is given at construction time).

The state may have a hint associated with it, specifying its valid
range.

`Action` is merely the interface to the concept of an action, as
described above. Various implementations of actions exist, including
`SimpleAction`.

In all cases, the implementing class is responsible for storing the
name of the action, the parameter type, the enabled state, the
optional state type and the state and emitting the appropriate
signals when these change. The implementor is responsible for filtering
calls to `Action::activate` and `Action::change_state` for type
safety and for the state being enabled.

Probably the only useful thing to do with a `Action` is to put it
inside of a `SimpleActionGroup`.

# Implements

[`ActionExt`](trait.ActionExt.html)
<!-- trait ActionExt -->
Trait containing all `Action` methods.

# Implementors

[`Action`](struct.Action.html), [`PropertyAction`](struct.PropertyAction.html), [`SimpleAction`](struct.SimpleAction.html)
<!-- impl Action::fn name_is_valid -->
Checks if `action_name` is valid.

`action_name` is valid if it consists only of alphanumeric characters,
plus '-' and '.'. The empty string is not a valid action name.

It is an error to call this function with a non-utf8 `action_name`.
`action_name` must not be `None`.
## `action_name`
an potential action name

# Returns

`true` if `action_name` is valid
<!-- impl Action::fn parse_detailed_name -->
Parses a detailed action name into its separate name and target
components.

Detailed action names can have three formats.

The first format is used to represent an action name with no target
value and consists of just an action name containing no whitespace
nor the characters ':', '(' or ')'. For example: "app.action".

The second format is used to represent an action with a target value
that is a non-empty string consisting only of alphanumerics, plus '-'
and '.'. In that case, the action name and target value are
separated by a double colon ("::"). For example:
"app.action::target".

The third format is used to represent an action with any type of
target value, including strings. The target value follows the action
name, surrounded in parens. For example: "app.action(42)". The
target value is parsed using `glib::Variant::parse`. If a tuple-typed
value is desired, it must be specified in the same way, resulting in
two sets of parens, for example: "app.action((1,2,3))". A string
target can be specified this way as well: "app.action('target')".
For strings, this third format must be used if * target value is
empty or contains characters other than alphanumerics, '-' and '.'.
## `detailed_name`
a detailed action name
## `action_name`
the action name
## `target_value`
the target value, or `None` for no target

# Returns

`true` if successful, else `false` with `error` set
<!-- impl Action::fn print_detailed_name -->
Formats a detailed action name from `action_name` and `target_value`.

It is an error to call this function with an invalid action name.

This function is the opposite of `Action::parse_detailed_name`.
It will produce a string that can be parsed back to the `action_name`
and `target_value` by that function.

See that function for the types of strings that will be printed by
this function.
## `action_name`
a valid action name
## `target_value`
a `glib::Variant` target value, or `None`

# Returns

a detailed format string
<!-- trait ActionExt::fn activate -->
Activates the action.

`parameter` must be the correct type of parameter for the action (ie:
the parameter type given at construction time). If the parameter
type was `None` then `parameter` must also be `None`.

If the `parameter` GVariant is floating, it is consumed.
## `parameter`
the parameter to the activation
<!-- trait ActionExt::fn change_state -->
Request for the state of `self` to be changed to `value`.

The action must be stateful and `value` must be of the correct type.
See `Action::get_state_type`.

This call merely requests a change. The action may refuse to change
its state or may change its state to something other than `value`.
See `Action::get_state_hint`.

If the `value` GVariant is floating, it is consumed.
## `value`
the new state
<!-- trait ActionExt::fn get_enabled -->
Checks if `self` is currently enabled.

An action must be enabled in order to be activated or in order to
have its state changed from outside callers.

# Returns

whether the action is enabled
<!-- trait ActionExt::fn get_name -->
Queries the name of `self`.

# Returns

the name of the action
<!-- trait ActionExt::fn get_parameter_type -->
Queries the type of the parameter that must be given when activating
`self`.

When activating the action using `Action::activate`, the `glib::Variant`
given to that function must be of the type returned by this function.

In the case that this function returns `None`, you must not give any
`glib::Variant`, but `None` instead.

# Returns

the parameter type
<!-- trait ActionExt::fn get_state -->
Queries the current state of `self`.

If the action is not stateful then `None` will be returned. If the
action is stateful then the type of the return value is the type
given by `Action::get_state_type`.

The return value (if non-`None`) should be freed with
`glib::Variant::unref` when it is no longer required.

# Returns

the current state of the action
<!-- trait ActionExt::fn get_state_hint -->
Requests a hint about the valid range of values for the state of
`self`.

If `None` is returned it either means that the action is not stateful
or that there is no hint about the valid range of values for the
state of the action.

If a `glib::Variant` array is returned then each item in the array is a
possible value for the state. If a `glib::Variant` pair (ie: two-tuple) is
returned then the tuple specifies the inclusive lower and upper bound
of valid values for the state.

In any case, the information is merely a hint. It may be possible to
have a state value outside of the hinted range and setting a value
within the range may fail.

The return value (if non-`None`) should be freed with
`glib::Variant::unref` when it is no longer required.

# Returns

the state range hint
<!-- trait ActionExt::fn get_state_type -->
Queries the type of the state of `self`.

If the action is stateful (e.g. created with
`SimpleAction::new_stateful`) then this function returns the
`glib::VariantType` of the state. This is the type of the initial value
given as the state. All calls to `Action::change_state` must give a
`glib::Variant` of this type and `Action::get_state` will return a
`glib::Variant` of the same type.

If the action is not stateful (e.g. created with `SimpleAction::new`)
then this function will return `None`. In that case, `Action::get_state`
will return `None` and you must not call `Action::change_state`.

# Returns

the state type, if the action is stateful
<!-- trait ActionExt::fn get_property_enabled -->
If `action` is currently enabled.

If the action is disabled then calls to `Action::activate` and
`Action::change_state` have no effect.
<!-- trait ActionExt::fn get_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `ActionGroup`. It is immutable.
<!-- trait ActionExt::fn get_property_parameter_type -->
The type of the parameter that must be given when activating the
action. This is immutable, and may be `None` if no parameter is needed when
activating the action.
<!-- trait ActionExt::fn get_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- trait ActionExt::fn get_property_state_type -->
The `glib::VariantType` of the state that the action has, or `None` if the
action is stateless. This is immutable.
<!-- struct ActionGroup -->
`ActionGroup` represents a group of actions. Actions can be used to
expose functionality in a structured way, either from one part of a
program to another, or to the outside world. Action groups are often
used together with a `MenuModel` that provides additional
representation data for displaying the actions to the user, e.g. in
a menu.

The main way to interact with the actions in a GActionGroup is to
activate them with `ActionGroup::activate_action`. Activating an
action may require a `glib::Variant` parameter. The required type of the
parameter can be inquired with `ActionGroup::get_action_parameter_type`.
Actions may be disabled, see `ActionGroup::get_action_enabled`.
Activating a disabled action has no effect.

Actions may optionally have a state in the form of a `glib::Variant`. The
current state of an action can be inquired with
`ActionGroup::get_action_state`. Activating a stateful action may
change its state, but it is also possible to set the state by calling
`ActionGroup::change_action_state`.

As typical example, consider a text editing application which has an
option to change the current font to 'bold'. A good way to represent
this would be a stateful action, with a boolean state. Activating the
action would toggle the state.

Each action in the group has a unique name (which is a string). All
method calls, except `ActionGroup::list_actions` take the name of
an action as an argument.

The `ActionGroup` API is meant to be the 'public' API to the action
group. The calls here are exactly the interaction that 'external
forces' (eg: UI, incoming D-Bus messages, etc.) are supposed to have
with actions. 'Internal' APIs (ie: ones meant only to be accessed by
the action group implementation) are found on subclasses. This is
why you will find - for example - `ActionGroup::get_action_enabled`
but not an equivalent `set` call.

Signals are emitted on the action group in response to state changes
on individual actions.

Implementations of `ActionGroup` should provide implementations for
the virtual functions `ActionGroup::list_actions` and
`ActionGroup::query_action`. The other virtual functions should
not be implemented - their "wrappers" are actually implemented with
calls to `ActionGroup::query_action`.

# Implements

[`ActionGroupExt`](trait.ActionGroupExt.html)
<!-- trait ActionGroupExt -->
Trait containing all `ActionGroup` methods.

# Implementors

[`ActionGroup`](struct.ActionGroup.html), [`Application`](struct.Application.html), [`RemoteActionGroup`](struct.RemoteActionGroup.html), [`SimpleActionGroup`](struct.SimpleActionGroup.html)
<!-- trait ActionGroupExt::fn action_added -->
Emits the `ActionGroup::action-added` signal on `self`.

This function should only be called by `ActionGroup` implementations.
## `action_name`
the name of an action in the group
<!-- trait ActionGroupExt::fn action_enabled_changed -->
Emits the `ActionGroup::action-enabled-changed` signal on `self`.

This function should only be called by `ActionGroup` implementations.
## `action_name`
the name of an action in the group
## `enabled`
whether or not the action is now enabled
<!-- trait ActionGroupExt::fn action_removed -->
Emits the `ActionGroup::action-removed` signal on `self`.

This function should only be called by `ActionGroup` implementations.
## `action_name`
the name of an action in the group
<!-- trait ActionGroupExt::fn action_state_changed -->
Emits the `ActionGroup::action-state-changed` signal on `self`.

This function should only be called by `ActionGroup` implementations.
## `action_name`
the name of an action in the group
## `state`
the new state of the named action
<!-- trait ActionGroupExt::fn activate_action -->
Activate the named action within `self`.

If the action is expecting a parameter, then the correct type of
parameter must be given as `parameter`. If the action is expecting no
parameters then `parameter` must be `None`. See
`ActionGroup::get_action_parameter_type`.
## `action_name`
the name of the action to activate
## `parameter`
parameters to the activation
<!-- trait ActionGroupExt::fn change_action_state -->
Request for the state of the named action within `self` to be
changed to `value`.

The action must be stateful and `value` must be of the correct type.
See `ActionGroup::get_action_state_type`.

This call merely requests a change. The action may refuse to change
its state or may change its state to something other than `value`.
See `ActionGroup::get_action_state_hint`.

If the `value` GVariant is floating, it is consumed.
## `action_name`
the name of the action to request the change on
## `value`
the new state
<!-- trait ActionGroupExt::fn get_action_enabled -->
Checks if the named action within `self` is currently enabled.

An action must be enabled in order to be activated or in order to
have its state changed from outside callers.
## `action_name`
the name of the action to query

# Returns

whether or not the action is currently enabled
<!-- trait ActionGroupExt::fn get_action_parameter_type -->
Queries the type of the parameter that must be given when activating
the named action within `self`.

When activating the action using `ActionGroup::activate_action`,
the `glib::Variant` given to that function must be of the type returned
by this function.

In the case that this function returns `None`, you must not give any
`glib::Variant`, but `None` instead.

The parameter type of a particular action will never change but it is
possible for an action to be removed and for a new action to be added
with the same name but a different parameter type.
## `action_name`
the name of the action to query

# Returns

the parameter type
<!-- trait ActionGroupExt::fn get_action_state -->
Queries the current state of the named action within `self`.

If the action is not stateful then `None` will be returned. If the
action is stateful then the type of the return value is the type
given by `ActionGroup::get_action_state_type`.

The return value (if non-`None`) should be freed with
`glib::Variant::unref` when it is no longer required.
## `action_name`
the name of the action to query

# Returns

the current state of the action
<!-- trait ActionGroupExt::fn get_action_state_hint -->
Requests a hint about the valid range of values for the state of the
named action within `self`.

If `None` is returned it either means that the action is not stateful
or that there is no hint about the valid range of values for the
state of the action.

If a `glib::Variant` array is returned then each item in the array is a
possible value for the state. If a `glib::Variant` pair (ie: two-tuple) is
returned then the tuple specifies the inclusive lower and upper bound
of valid values for the state.

In any case, the information is merely a hint. It may be possible to
have a state value outside of the hinted range and setting a value
within the range may fail.

The return value (if non-`None`) should be freed with
`glib::Variant::unref` when it is no longer required.
## `action_name`
the name of the action to query

# Returns

the state range hint
<!-- trait ActionGroupExt::fn get_action_state_type -->
Queries the type of the state of the named action within
`self`.

If the action is stateful then this function returns the
`glib::VariantType` of the state. All calls to
`ActionGroup::change_action_state` must give a `glib::Variant` of this
type and `ActionGroup::get_action_state` will return a `glib::Variant`
of the same type.

If the action is not stateful then this function will return `None`.
In that case, `ActionGroup::get_action_state` will return `None`
and you must not call `ActionGroup::change_action_state`.

The state type of a particular action will never change but it is
possible for an action to be removed and for a new action to be added
with the same name but a different state type.
## `action_name`
the name of the action to query

# Returns

the state type, if the action is stateful
<!-- trait ActionGroupExt::fn has_action -->
Checks if the named action exists within `self`.
## `action_name`
the name of the action to check for

# Returns

whether the named action exists
<!-- trait ActionGroupExt::fn list_actions -->
Lists the actions contained within `self`.

The caller is responsible for freeing the list with `g_strfreev` when
it is no longer required.

# Returns

a `None`-terminated array of the names of the
actions in the group
<!-- trait ActionGroupExt::fn query_action -->
Queries all aspects of the named action within an `self`.

This function acquires the information available from
`ActionGroup::has_action`, `ActionGroup::get_action_enabled`,
`ActionGroup::get_action_parameter_type`,
`ActionGroup::get_action_state_type`,
`ActionGroup::get_action_state_hint` and
`ActionGroup::get_action_state` with a single function call.

This provides two main benefits.

The first is the improvement in efficiency that comes with not having
to perform repeated lookups of the action in order to discover
different things about it. The second is that implementing
`ActionGroup` can now be done by only overriding this one virtual
function.

The interface provides a default implementation of this function that
calls the individual functions, as required, to fetch the
information. The interface also provides default implementations of
those functions that call this function. All implementations,
therefore, must override either this function or all of the others.

If the action exists, `true` is returned and any of the requested
fields (as indicated by having a non-`None` reference passed in) are
filled. If the action doesn't exist, `false` is returned and the
fields may or may not have been modified.
## `action_name`
the name of an action in the group
## `enabled`
if the action is presently enabled
## `parameter_type`
the parameter type, or `None` if none needed
## `state_type`
the state type, or `None` if stateless
## `state_hint`
the state hint, or `None` if none
## `state`
the current state, or `None` if stateless

# Returns

`true` if the action exists, else `false`
<!-- trait ActionGroupExt::fn connect_action_added -->
Signals that a new action was just added to the group.
This signal is emitted after the action has been added
and is now visible.
## `action_name`
the name of the action in `action_group`
<!-- trait ActionGroupExt::fn connect_action_enabled_changed -->
Signals that the enabled status of the named action has changed.
## `action_name`
the name of the action in `action_group`
## `enabled`
whether the action is enabled or not
<!-- trait ActionGroupExt::fn connect_action_removed -->
Signals that an action is just about to be removed from the group.
This signal is emitted before the action is removed, so the action
is still visible and can be queried from the signal handler.
## `action_name`
the name of the action in `action_group`
<!-- trait ActionGroupExt::fn connect_action_state_changed -->
Signals that the state of the named action has changed.
## `action_name`
the name of the action in `action_group`
## `value`
the new value of the state
<!-- struct ActionMap -->
The GActionMap interface is implemented by `ActionGroup`
implementations that operate by containing a number of
named `Action` instances, such as `SimpleActionGroup`.

One useful application of this interface is to map the
names of actions from various action groups to unique,
prefixed names (e.g. by prepending "app." or "win.").
This is the motivation for the 'Map' part of the interface
name.

# Implements

[`ActionMapExt`](trait.ActionMapExt.html)
<!-- trait ActionMapExt -->
Trait containing all `ActionMap` methods.

# Implementors

[`ActionMap`](struct.ActionMap.html), [`Application`](struct.Application.html), [`SimpleActionGroup`](struct.SimpleActionGroup.html)
<!-- trait ActionMapExt::fn add_action -->
Adds an action to the `self`.

If the action map already contains an action with the same name
as `action` then the old action is dropped from the action map.

The action map takes its own reference on `action`.
## `action`
a `Action`
<!-- trait ActionMapExt::fn add_action_entries -->
A convenience function for creating multiple `SimpleAction` instances
and adding them to a `ActionMap`.

Each action is constructed as per one `ActionEntry`.


```C
static void
activate_quit (GSimpleAction *simple,
               GVariant      *parameter,
               gpointer       user_data)
{
  exit (0);
}

static void
activate_print_string (GSimpleAction *simple,
                       GVariant      *parameter,
                       gpointer       user_data)
{
  g_print ("%s\n", g_variant_get_string (parameter, NULL));
}

static GActionGroup *
create_action_group (void)
{
  const GActionEntry entries[] = {
    { "quit",         activate_quit              },
    { "print-string", activate_print_string, "s" }
  };
  GSimpleActionGroup *group;

  group = g_simple_action_group_new ();
  g_action_map_add_action_entries (G_ACTION_MAP (group), entries, G_N_ELEMENTS (entries), NULL);

  return G_ACTION_GROUP (group);
}
```
## `entries`
a pointer to
 the first item in an array of `ActionEntry` structs
## `n_entries`
the length of `entries`, or -1 if `entries` is `None`-terminated
## `user_data`
the user data for signal connections
<!-- trait ActionMapExt::fn lookup_action -->
Looks up the action with the name `action_name` in `self`.

If no such action exists, returns `None`.
## `action_name`
the name of an action

# Returns

a `Action`, or `None`
<!-- trait ActionMapExt::fn remove_action -->
Removes the named action from the action map.

If no action of this name is in the map then nothing happens.
## `action_name`
the name of the action
<!-- struct AppInfo -->
`AppInfo` and `AppLaunchContext` are used for describing and launching
applications installed on the system.

As of GLib 2.20, URIs will always be converted to POSIX paths
(using `File::get_path`) when using `AppInfo::launch` even if
the application requested an URI and not a POSIX path. For example
for an desktop-file based application with Exec key `totem
%U` and a single URI, `sftp://foo/file.avi`, then
`/home/user/.gvfs/sftp on foo/file.avi` will be passed. This will
only work if a set of suitable GIO extensions (such as gvfs 2.26
compiled with FUSE support), is available and operational; if this
is not the case, the URI will be passed unmodified to the application.
Some URIs, such as `mailto:`, of course cannot be mapped to a POSIX
path (in gvfs there's no FUSE mount for it); such URIs will be
passed unmodified to the application.

Specifically for gvfs 2.26 and later, the POSIX URI will be mapped
back to the GIO URI in the `File` constructors (since gvfs
implements the `Vfs` extension point). As such, if the application
needs to examine the URI, it needs to use `File::get_uri` or
similar on `File`. In other words, an application cannot assume
that the URI passed to e.g. `File::new_for_commandline_arg` is
equal to the result of `File::get_uri`. The following snippet
illustrates this:


```text
GFile *f;
char *uri;

file = g_file_new_for_commandline_arg (uri_from_commandline);

uri = g_file_get_uri (file);
strcmp (uri, uri_from_commandline) == 0;
g_free (uri);

if (g_file_has_uri_scheme (file, "cdda"))
  {
    // do something special with uri
  }
g_object_unref (file);
```

This code will work when both `cdda://sr0/Track 1.wav` and
`/home/user/.gvfs/cdda on sr0/Track 1.wav` is passed to the
application. It should be noted that it's generally not safe
for applications to rely on the format of a particular URIs.
Different launcher applications (e.g. file managers) may have
different ideas of what a given URI means.

# Implements

[`AppInfoExt`](trait.AppInfoExt.html)
<!-- trait AppInfoExt -->
Trait containing all `AppInfo` methods.

# Implementors

[`AppInfo`](struct.AppInfo.html), [`DesktopAppInfo`](struct.DesktopAppInfo.html)
<!-- impl AppInfo::fn create_from_commandline -->
Creates a new `AppInfo` from the given information.

Note that for `commandline`, the quoting rules of the Exec key of the
[freedesktop.org Desktop Entry Specification](http://freedesktop.org/Standards/desktop-entry-spec)
are applied. For example, if the `commandline` contains
percent-encoded URIs, the percent-character must be doubled in order to prevent it from
being swallowed by Exec key unquoting. See the specification for exact quoting rules.
## `commandline`
the commandline to use
## `application_name`
the application name, or `None` to use `commandline`
## `flags`
flags that can specify details of the created `AppInfo`

# Returns

new `AppInfo` for given command.
<!-- impl AppInfo::fn get_all -->
Gets a list of all of the applications currently registered
on this system.

For desktop files, this includes applications that have
`NoDisplay=true` set or are excluded from display by means
of `OnlyShowIn` or `NotShowIn`. See `AppInfo::should_show`.
The returned list does not include applications which have
the `Hidden` key set.

# Returns

a newly allocated `glib::List` of references to `GAppInfos`.
<!-- impl AppInfo::fn get_all_for_type -->
Gets a list of all `GAppInfos` for a given content type,
including the recommended and fallback `GAppInfos`. See
`AppInfo::get_recommended_for_type` and
`AppInfo::get_fallback_for_type`.
## `content_type`
the content type to find a `AppInfo` for

# Returns

`glib::List` of `GAppInfos`
 for given `content_type` or `None` on error.
<!-- impl AppInfo::fn get_default_for_type -->
Gets the default `AppInfo` for a given content type.
## `content_type`
the content type to find a `AppInfo` for
## `must_support_uris`
if `true`, the `AppInfo` is expected to
 support URIs

# Returns

`AppInfo` for given `content_type` or
 `None` on error.
<!-- impl AppInfo::fn get_default_for_uri_scheme -->
Gets the default application for handling URIs with
the given URI scheme. A URI scheme is the initial part
of the URI, up to but not including the ':', e.g. "http",
"ftp" or "sip".
## `uri_scheme`
a string containing a URI scheme.

# Returns

`AppInfo` for given `uri_scheme` or `None` on error.
<!-- impl AppInfo::fn get_fallback_for_type -->
Gets a list of fallback `GAppInfos` for a given content type, i.e.
those applications which claim to support the given content type
by MIME type subclassing and not directly.
## `content_type`
the content type to find a `AppInfo` for

# Returns

`glib::List` of `GAppInfos`
 for given `content_type` or `None` on error.
<!-- impl AppInfo::fn get_recommended_for_type -->
Gets a list of recommended `GAppInfos` for a given content type, i.e.
those applications which claim to support the given content type exactly,
and not by MIME type subclassing.
Note that the first application of the list is the last used one, i.e.
the last one for which `AppInfo::set_as_last_used_for_type` has been
called.
## `content_type`
the content type to find a `AppInfo` for

# Returns

`glib::List` of `GAppInfos`
 for given `content_type` or `None` on error.
<!-- impl AppInfo::fn launch_default_for_uri -->
Utility function that launches the default application
registered to handle the specified uri. Synchronous I/O
is done on the uri to detect the type of the file if
required.
## `uri`
the uri to show
## `context`
an optional `AppLaunchContext`

# Returns

`true` on success, `false` on error.
<!-- impl AppInfo::fn launch_default_for_uri_async -->
Async version of `AppInfo::launch_default_for_uri`.

This version is useful if you are interested in receiving
error information in the case where the application is
sandboxed and the portal may present an application chooser
dialog to the user.

Feature: `v2_50`

## `uri`
the uri to show
## `context`
an optional `AppLaunchContext`
## `cancellable`
a `Cancellable`
## `callback`
a `GASyncReadyCallback` to call when the request is done
## `user_data`
data to pass to `callback`
<!-- impl AppInfo::fn launch_default_for_uri_finish -->
Finishes an asynchronous launch-default-for-uri operation.

Feature: `v2_50`

## `result`
a `AsyncResult`

# Returns

`true` if the launch was successful, `false` if `error` is set
<!-- impl AppInfo::fn reset_type_associations -->
Removes all changes to the type associations done by
`AppInfo::set_as_default_for_type`,
`AppInfo::set_as_default_for_extension`,
`AppInfo::add_supports_type` or
`AppInfo::remove_supports_type`.
## `content_type`
a content type
<!-- trait AppInfoExt::fn add_supports_type -->
Adds a content type to the application information to indicate the
application is capable of opening files with the given content type.
## `content_type`
a string.

# Returns

`true` on success, `false` on error.
<!-- trait AppInfoExt::fn can_delete -->
Obtains the information whether the `AppInfo` can be deleted.
See `AppInfo::delete`.

# Returns

`true` if `self` can be deleted
<!-- trait AppInfoExt::fn can_remove_supports_type -->
Checks if a supported content type can be removed from an application.

# Returns

`true` if it is possible to remove supported
 content types from a given `self`, `false` if not.
<!-- trait AppInfoExt::fn delete -->
Tries to delete a `AppInfo`.

On some platforms, there may be a difference between user-defined
`GAppInfos` which can be deleted, and system-wide ones which cannot.
See `AppInfo::can_delete`.

# Returns

`true` if `self` has been deleted
<!-- trait AppInfoExt::fn dup -->
Creates a duplicate of a `AppInfo`.

# Returns

a duplicate of `self`.
<!-- trait AppInfoExt::fn equal -->
Checks if two `GAppInfos` are equal.

Note that the check `<emphasis>`may not`</emphasis>` compare each individual
field, and only does an identity check. In case detecting changes in the
contents is needed, program code must additionally compare relevant fields.
## `appinfo2`
the second `AppInfo`.

# Returns

`true` if `self` is equal to `appinfo2`. `false` otherwise.
<!-- trait AppInfoExt::fn get_commandline -->
Gets the commandline with which the application will be
started.

# Returns

a string containing the `self`'s commandline,
 or `None` if this information is not available
<!-- trait AppInfoExt::fn get_description -->
Gets a human-readable description of an installed application.

# Returns

a string containing a description of the
application `self`, or `None` if none.
<!-- trait AppInfoExt::fn get_display_name -->
Gets the display name of the application. The display name is often more
descriptive to the user than the name itself.

# Returns

the display name of the application for `self`, or the name if
no display name is available.
<!-- trait AppInfoExt::fn get_executable -->
Gets the executable's name for the installed application.

# Returns

a string containing the `self`'s application
binaries name
<!-- trait AppInfoExt::fn get_icon -->
Gets the icon for the application.

# Returns

the default `Icon` for `self` or `None`
if there is no default icon.
<!-- trait AppInfoExt::fn get_id -->
Gets the ID of an application. An id is a string that
identifies the application. The exact format of the id is
platform dependent. For instance, on Unix this is the
desktop file id from the xdg menu specification.

Note that the returned ID may be `None`, depending on how
the `self` has been constructed.

# Returns

a string containing the application's ID.
<!-- trait AppInfoExt::fn get_name -->
Gets the installed name of the application.

# Returns

the name of the application for `self`.
<!-- trait AppInfoExt::fn get_supported_types -->
Retrieves the list of content types that `app_info` claims to support.
If this information is not provided by the environment, this function
will return `None`.
This function does not take in consideration associations added with
`AppInfo::add_supports_type`, but only those exported directly by
the application.

# Returns


 a list of content types.
<!-- trait AppInfoExt::fn launch -->
Launches the application. Passes `files` to the launched application
as arguments, using the optional `context` to get information
about the details of the launcher (like what screen it is on).
On error, `error` will be set accordingly.

To launch the application without arguments pass a `None` `files` list.

Note that even if the launch is successful the application launched
can fail to start if it runs into problems during startup. There is
no way to detect this.

Some URIs can be changed when passed through a GFile (for instance
unsupported URIs with strange formats like mailto:), so if you have
a textual URI you want to pass in as argument, consider using
`AppInfo::launch_uris` instead.

The launched application inherits the environment of the launching
process, but it can be modified with `AppLaunchContextExt::setenv`
and `AppLaunchContextExt::unsetenv`.

On UNIX, this function sets the `GIO_LAUNCHED_DESKTOP_FILE`
environment variable with the path of the launched desktop file and
`GIO_LAUNCHED_DESKTOP_FILE_PID` to the process id of the launched
process. This can be used to ignore `GIO_LAUNCHED_DESKTOP_FILE`,
should it be inherited by further processes. The `DISPLAY` and
`DESKTOP_STARTUP_ID` environment variables are also set, based
on information provided in `context`.
## `files`
a `glib::List` of `File` objects
## `context`
a `AppLaunchContext` or `None`

# Returns

`true` on successful launch, `false` otherwise.
<!-- trait AppInfoExt::fn launch_uris -->
Launches the application. This passes the `uris` to the launched application
as arguments, using the optional `context` to get information
about the details of the launcher (like what screen it is on).
On error, `error` will be set accordingly.

To launch the application without arguments pass a `None` `uris` list.

Note that even if the launch is successful the application launched
can fail to start if it runs into problems during startup. There is
no way to detect this.
## `uris`
a `glib::List` containing URIs to launch.
## `context`
a `AppLaunchContext` or `None`

# Returns

`true` on successful launch, `false` otherwise.
<!-- trait AppInfoExt::fn remove_supports_type -->
Removes a supported type from an application, if possible.
## `content_type`
a string.

# Returns

`true` on success, `false` on error.
<!-- trait AppInfoExt::fn set_as_default_for_extension -->
Sets the application as the default handler for the given file extension.
## `extension`
a string containing the file extension
 (without the dot).

# Returns

`true` on success, `false` on error.
<!-- trait AppInfoExt::fn set_as_default_for_type -->
Sets the application as the default handler for a given type.
## `content_type`
the content type.

# Returns

`true` on success, `false` on error.
<!-- trait AppInfoExt::fn set_as_last_used_for_type -->
Sets the application as the last used application for a given type.
This will make the application appear as first in the list returned
by `AppInfo::get_recommended_for_type`, regardless of the default
application for that content type.
## `content_type`
the content type.

# Returns

`true` on success, `false` on error.
<!-- trait AppInfoExt::fn should_show -->
Checks if the application info should be shown in menus that
list available applications.

# Returns

`true` if the `self` should be shown, `false` otherwise.
<!-- trait AppInfoExt::fn supports_files -->
Checks if the application accepts files as arguments.

# Returns

`true` if the `self` supports files.
<!-- trait AppInfoExt::fn supports_uris -->
Checks if the application supports reading files and directories from URIs.

# Returns

`true` if the `self` supports URIs.
<!-- struct AppInfoMonitor -->
`AppInfoMonitor` is a very simple object used for monitoring the app
info database for changes (ie: newly installed or removed
applications).

Call `AppInfoMonitor::get` to get a `AppInfoMonitor` and connect
to the "changed" signal.

In the usual case, applications should try to make note of the change
(doing things like invalidating caches) but not act on it. In
particular, applications should avoid making calls to `AppInfo` APIs
in response to the change signal, deferring these until the time that
the data is actually required. The exception to this case is when
application information is actually being displayed on the screen
(eg: during a search or when the list of all applications is shown).
The reason for this is that changes to the list of installed
applications often come in groups (like during system updates) and
rescanning the list on every change is pointless and expensive.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl AppInfoMonitor::fn get -->
Gets the `AppInfoMonitor` for the current thread-default main
context.

The `AppInfoMonitor` will emit a "changed" signal in the
thread-default main context whenever the list of installed
applications (as reported by `AppInfo::get_all`) may have changed.

You must only call `gobject::ObjectExt::unref` on the return value from under
the same main context as you created it.

# Returns

a reference to a `AppInfoMonitor`
<!-- impl AppInfoMonitor::fn connect_changed -->
Signal emitted when the app info database for changes (ie: newly installed
or removed applications).
<!-- struct AppLaunchContext -->
Integrating the launch with the launching application. This is used to
handle for instance startup notification and launching the new application
on the same screen as the launching window.

# Implements

[`AppLaunchContextExt`](trait.AppLaunchContextExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait AppLaunchContextExt -->
Trait containing all `AppLaunchContext` methods.

# Implementors

[`AppLaunchContext`](struct.AppLaunchContext.html)
<!-- impl AppLaunchContext::fn new -->
Creates a new application launch context. This is not normally used,
instead you instantiate a subclass of this, such as ``GdkAppLaunchContext``.

# Returns

a `AppLaunchContext`.
<!-- trait AppLaunchContextExt::fn get_display -->
Gets the display string for the `self`. This is used to ensure new
applications are started on the same display as the launching
application, by setting the `DISPLAY` environment variable.
## `info`
a `AppInfo`
## `files`
a `glib::List` of `File` objects

# Returns

a display string for the display.
<!-- trait AppLaunchContextExt::fn get_environment -->
Gets the complete environment variable list to be passed to
the child process when `self` is used to launch an application.
This is a `None`-terminated array of strings, where each string has
the form `KEY=VALUE`.

# Returns


 the child's environment
<!-- trait AppLaunchContextExt::fn get_startup_notify_id -->
Initiates startup notification for the application and returns the
`DESKTOP_STARTUP_ID` for the launched operation, if supported.

Startup notification IDs are defined in the
[FreeDesktop.Org Startup Notifications standard](http://standards.freedesktop.org/startup-notification-spec/startup-notification-latest.txt").
## `info`
a `AppInfo`
## `files`
a `glib::List` of of `File` objects

# Returns

a startup notification ID for the application, or `None` if
 not supported.
<!-- trait AppLaunchContextExt::fn launch_failed -->
Called when an application has failed to launch, so that it can cancel
the application startup notification started in `AppLaunchContextExt::get_startup_notify_id`.
## `startup_notify_id`
the startup notification id that was returned by `AppLaunchContextExt::get_startup_notify_id`.
<!-- trait AppLaunchContextExt::fn setenv -->
Arranges for `variable` to be set to `value` in the child's
environment when `self` is used to launch an application.
## `variable`
the environment variable to set
## `value`
the value for to set the variable to.
<!-- trait AppLaunchContextExt::fn unsetenv -->
Arranges for `variable` to be unset in the child's environment
when `self` is used to launch an application.
## `variable`
the environment variable to remove
<!-- trait AppLaunchContextExt::fn connect_launch_failed -->
The ::launch-failed signal is emitted when a `AppInfo` launch
fails. The startup notification id is provided, so that the launcher
can cancel the startup notification.
## `startup_notify_id`
the startup notification id for the failed launch
<!-- trait AppLaunchContextExt::fn connect_launched -->
The ::launched signal is emitted when a `AppInfo` is successfully
launched. The `platform_data` is an GVariant dictionary mapping
strings to variants (ie a{sv}), which contains additional,
platform-specific data about this launch. On UNIX, at least the
"pid" and "startup-notification-id" keys will be present.
## `info`
the `AppInfo` that was just launched
## `platform_data`
additional platform-specific data for this launch
<!-- struct Application -->
A `Application` is the foundation of an application. It wraps some
low-level platform-specific services and is intended to act as the
foundation for higher-level application classes such as
``GtkApplication`` or `MxApplication`. In general, you should not use
this class outside of a higher level framework.

GApplication provides convenient life cycle management by maintaining
a "use count" for the primary application instance. The use count can
be changed using `ApplicationExt::hold` and `ApplicationExt::release`. If
it drops to zero, the application exits. Higher-level classes such as
``GtkApplication`` employ the use count to ensure that the application
stays alive as long as it has any opened windows.

Another feature that GApplication (optionally) provides is process
uniqueness. Applications can make use of this functionality by
providing a unique application ID. If given, only one application
with this ID can be running at a time per session. The session
concept is platform-dependent, but corresponds roughly to a graphical
desktop login. When your application is launched again, its
arguments are passed through platform communication to the already
running program. The already running instance of the program is
called the "primary instance"; for non-unique applications this is
the always the current instance. On Linux, the D-Bus session bus
is used for communication.

The use of `Application` differs from some other commonly-used
uniqueness libraries (such as libunique) in important ways. The
application is not expected to manually register itself and check
if it is the primary instance. Instead, the `main` function of a
`Application` should do very little more than instantiating the
application instance, possibly connecting signal handlers, then
calling `Application::run`. All checks for uniqueness are done
internally. If the application is the primary instance then the
startup signal is emitted and the mainloop runs. If the application
is not the primary instance then a signal is sent to the primary
instance and `Application::run` promptly returns. See the code
examples below.

If used, the expected form of an application identifier is the same as
that of of a
[D-Bus well-known bus name](https://dbus.freedesktop.org/doc/dbus-specification.html`message`-protocol-names-bus).
Examples include: `com.example.MyApp`, `org.example.internal_apps.Calculator`,
`org._7_zip.Archiver`.
For details on valid application identifiers, see `Application::id_is_valid`.

On Linux, the application identifier is claimed as a well-known bus name
on the user's session bus. This means that the uniqueness of your
application is scoped to the current session. It also means that your
application may provide additional services (through registration of other
object paths) at that bus name. The registration of these object paths
should be done with the shared GDBus session bus. Note that due to the
internal architecture of GDBus, method calls can be dispatched at any time
(even if a main loop is not running). For this reason, you must ensure that
any object paths that you wish to register are registered before `Application`
attempts to acquire the bus name of your application (which happens in
`ApplicationExt::register`). Unfortunately, this means that you cannot use
`ApplicationExt::get_is_remote` to decide if you want to register object paths.

GApplication also implements the `ActionGroup` and `ActionMap`
interfaces and lets you easily export actions by adding them with
`ActionMap::add_action`. When invoking an action by calling
`ActionGroup::activate_action` on the application, it is always
invoked in the primary instance. The actions are also exported on
the session bus, and GIO provides the `DBusActionGroup` wrapper to
conveniently access them remotely. GIO provides a `DBusMenuModel` wrapper
for remote access to exported `GMenuModels`.

There is a number of different entry points into a GApplication:

- via 'Activate' (i.e. just starting the application)

- via 'Open' (i.e. opening some files)

- by handling a command-line

- via activating an action

The `Application::startup` signal lets you handle the application
initialization for all of these in a single place.

Regardless of which of these entry points is used to start the
application, GApplication passes some "platform data from the
launching instance to the primary instance, in the form of a
`glib::Variant` dictionary mapping strings to variants. To use platform
data, override the `before_emit` or `after_emit` virtual functions
in your `Application` subclass. When dealing with
`ApplicationCommandLine` objects, the platform data is
directly available via `ApplicationCommandLineExt::get_cwd`,
`ApplicationCommandLineExt::get_environ` and
`ApplicationCommandLineExt::get_platform_data`.

As the name indicates, the platform data may vary depending on the
operating system, but it always includes the current directory (key
"cwd"), and optionally the environment (ie the set of environment
variables and their values) of the calling process (key "environ").
The environment is only added to the platform data if the
`ApplicationFlags::SendEnvironment` flag is set. `Application` subclasses
can add their own platform data by overriding the `add_platform_data`
virtual function. For instance, ``GtkApplication`` adds startup notification
data in this way.

To parse commandline arguments you may handle the
`Application::command-line` signal or override the `local_command_line`
vfunc, to parse them in either the primary instance or the local instance,
respectively.

For an example of opening files with a GApplication, see
[gapplication-example-open.c](https://git.gnome.org/browse/glib/tree/gio/tests/gapplication-example-open.c).

For an example of using actions with GApplication, see
[gapplication-example-actions.c](https://git.gnome.org/browse/glib/tree/gio/tests/gapplication-example-actions.c).

For an example of using extra D-Bus hooks with GApplication, see
[gapplication-example-dbushooks.c](https://git.gnome.org/browse/glib/tree/gio/tests/gapplication-example-dbushooks.c).

# Implements

[`ApplicationExt`](trait.ApplicationExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ActionGroupExt`](trait.ActionGroupExt.html), [`ActionMapExt`](trait.ActionMapExt.html), [`ApplicationExtManual`](prelude/trait.ApplicationExtManual.html)
<!-- trait ApplicationExt -->
Trait containing all `Application` methods.

# Implementors

[`Application`](struct.Application.html)
<!-- impl Application::fn new -->
Creates a new `Application` instance.

If non-`None`, the application id must be valid. See
`Application::id_is_valid`.

If no application ID is given then some features of `Application`
(most notably application uniqueness) will be disabled.
## `application_id`
the application id
## `flags`
the application flags

# Returns

a new `Application` instance
<!-- impl Application::fn get_default -->
Returns the default `Application` instance for this process.

Normally there is only one `Application` per process and it becomes
the default when it is created. You can exercise more control over
this by using `ApplicationExt::set_default`.

If there is no default application then `None` is returned.

# Returns

the default application for this process, or `None`
<!-- impl Application::fn id_is_valid -->
Checks if `application_id` is a valid application identifier.

A valid ID is required for calls to `Application::new` and
`ApplicationExt::set_application_id`.

Application identifiers follow the same format as
[D-Bus well-known bus names](https://dbus.freedesktop.org/doc/dbus-specification.html`message`-protocol-names-bus).
For convenience, the restrictions on application identifiers are
reproduced here:

- Application identifiers are composed of 1 or more elements separated by a
 period (`.`) character. All elements must contain at least one character.

- Each element must only contain the ASCII characters `[A-Z][a-z][0-9]_-`,
 with `-` discouraged in new application identifiers. Each element must not
 begin with a digit.

- Application identifiers must contain at least one `.` (period) character
 (and thus at least two elements).

- Application identifiers must not begin with a `.` (period) character.

- Application identifiers must not exceed 255 characters.

Note that the hyphen (`-`) character is allowed in application identifiers,
but is problematic or not allowed in various specifications and APIs that
refer to D-Bus, such as
[Flatpak application IDs](http://docs.flatpak.org/en/latest/introduction.html`identifiers`),
the
[`DBusActivatable` interface in the Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html`dbus`),
and the convention that an application's "main" interface and object path
resemble its application identifier and bus name. To avoid situations that
require special-case handling, it is recommended that new application
identifiers consistently replace hyphens with underscores.

Like D-Bus interface names, application identifiers should start with the
reversed DNS domain name of the author of the interface (in lower-case), and
it is conventional for the rest of the application identifier to consist of
words run together, with initial capital letters.

As with D-Bus interface names, if the author's DNS domain name contains
hyphen/minus characters they should be replaced by underscores, and if it
contains leading digits they should be escaped by prepending an underscore.
For example, if the owner of 7-zip.org used an application identifier for an
archiving application, it might be named `org._7_zip.Archiver`.
## `application_id`
a potential application identifier

# Returns

`true` if `application_id` is valid
<!-- trait ApplicationExt::fn activate -->
Activates the application.

In essence, this results in the `Application::activate` signal being
emitted in the primary instance.

The application must be registered before calling this function.
<!-- trait ApplicationExt::fn add_main_option -->
Add an option to be handled by `self`.

Calling this function is the equivalent of calling
`ApplicationExt::add_main_option_entries` with a single `glib::OptionEntry`
that has its arg_data member set to `None`.

The parsed arguments will be packed into a `glib::VariantDict` which
is passed to `Application::handle-local-options`. If
`ApplicationFlags::HandlesCommandLine` is set, then it will also
be sent to the primary instance. See
`ApplicationExt::add_main_option_entries` for more details.

See `glib::OptionEntry` for more documentation of the arguments.
## `long_name`
the long name of an option used to specify it in a commandline
## `short_name`
the short name of an option
## `flags`
flags from `glib::OptionFlags`
## `arg`
the type of the option, as a `glib::OptionArg`
## `description`
the description for the option in `--help` output
## `arg_description`
the placeholder to use for the extra argument
 parsed by the option in `--help` output
<!-- trait ApplicationExt::fn add_main_option_entries -->
Adds main option entries to be handled by `self`.

This function is comparable to `glib::OptionContext::add_main_entries`.

After the commandline arguments are parsed, the
`Application::handle-local-options` signal will be emitted. At this
point, the application can inspect the values pointed to by `arg_data`
in the given `GOptionEntrys`.

Unlike `glib::OptionContext`, `Application` supports giving a `None`
`arg_data` for a non-callback `glib::OptionEntry`. This results in the
argument in question being packed into a `glib::VariantDict` which is also
passed to `Application::handle-local-options`, where it can be
inspected and modified. If `ApplicationFlags::HandlesCommandLine` is
set, then the resulting dictionary is sent to the primary instance,
where `ApplicationCommandLineExt::get_options_dict` will return it.
This "packing" is done according to the type of the argument --
booleans for normal flags, strings for strings, bytestrings for
filenames, etc. The packing only occurs if the flag is given (ie: we
do not pack a "false" `glib::Variant` in the case that a flag is missing).

In general, it is recommended that all commandline arguments are
parsed locally. The options dictionary should then be used to
transmit the result of the parsing to the primary instance, where
`glib::VariantDict::lookup` can be used. For local options, it is
possible to either use `arg_data` in the usual way, or to consult (and
potentially remove) the option from the options dictionary.

This function is new in GLib 2.40. Before then, the only real choice
was to send all of the commandline arguments (options and all) to the
primary instance for handling. `Application` ignored them completely
on the local side. Calling this function "opts in" to the new
behaviour, and in particular, means that unrecognised options will be
treated as errors. Unrecognised options have never been ignored when
`ApplicationFlags::HandlesCommandLine` is unset.

If `Application::handle-local-options` needs to see the list of
filenames, then the use of `G_OPTION_REMAINING` is recommended. If
`arg_data` is `None` then `G_OPTION_REMAINING` can be used as a key into
the options dictionary. If you do use `G_OPTION_REMAINING` then you
need to handle these arguments for yourself because once they are
consumed, they will no longer be visible to the default handling
(which treats them as filenames to be opened).

It is important to use the proper GVariant format when retrieving
the options with `glib::VariantDict::lookup`:
- for `glib::OptionArg::None`, use b
- for `glib::OptionArg::String`, use &s
- for `glib::OptionArg::Int`, use i
- for `glib::OptionArg::Int64`, use x
- for `glib::OptionArg::Double`, use d
- for `glib::OptionArg::Filename`, use ^ay
- for `glib::OptionArg::StringArray`, use &as
- for `glib::OptionArg::FilenameArray`, use ^aay
## `entries`
a
 `None`-terminated list of `GOptionEntrys`
<!-- trait ApplicationExt::fn add_option_group -->
Adds a `glib::OptionGroup` to the commandline handling of `self`.

This function is comparable to `glib::OptionContext::add_group`.

Unlike `ApplicationExt::add_main_option_entries`, this function does
not deal with `None` `arg_data` and never transmits options to the
primary instance.

The reason for that is because, by the time the options arrive at the
primary instance, it is typically too late to do anything with them.
Taking the GTK option group as an example: GTK will already have been
initialised by the time the `Application::command-line` handler runs.
In the case that this is not the first-running instance of the
application, the existing instance may already have been running for
a very long time.

This means that the options from `glib::OptionGroup` are only really usable
in the case that the instance of the application being run is the
first instance. Passing options like `--display=` or `--gdk-debug=`
on future runs will have no effect on the existing primary instance.

Calling this function will cause the options in the supplied option
group to be parsed, but it does not cause you to be "opted in" to the
new functionality whereby unrecognised options are rejected even if
`ApplicationFlags::HandlesCommandLine` was given.
## `group`
a `glib::OptionGroup`
<!-- trait ApplicationExt::fn bind_busy_property -->
Marks `self` as busy (see `ApplicationExt::mark_busy`) while
`property` on `object` is `true`.

The binding holds a reference to `self` while it is active, but
not to `object`. Instead, the binding is destroyed when `object` is
finalized.

Feature: `v2_44`

## `object`
a `gobject::Object`
## `property`
the name of a boolean property of `object`
<!-- trait ApplicationExt::fn get_application_id -->
Gets the unique identifier for `self`.

# Returns

the identifier for `self`, owned by `self`
<!-- trait ApplicationExt::fn get_dbus_connection -->
Gets the `DBusConnection` being used by the application, or `None`.

If `Application` is using its D-Bus backend then this function will
return the `DBusConnection` being used for uniqueness and
communication with the desktop environment and other instances of the
application.

If `Application` is not using D-Bus then this function will return
`None`. This includes the situation where the D-Bus backend would
normally be in use but we were unable to connect to the bus.

This function must not be called before the application has been
registered. See `ApplicationExt::get_is_registered`.

# Returns

a `DBusConnection`, or `None`
<!-- trait ApplicationExt::fn get_dbus_object_path -->
Gets the D-Bus object path being used by the application, or `None`.

If `Application` is using its D-Bus backend then this function will
return the D-Bus object path that `Application` is using. If the
application is the primary instance then there is an object published
at this path. If the application is not the primary instance then
the result of this function is undefined.

If `Application` is not using D-Bus then this function will return
`None`. This includes the situation where the D-Bus backend would
normally be in use but we were unable to connect to the bus.

This function must not be called before the application has been
registered. See `ApplicationExt::get_is_registered`.

# Returns

the object path, or `None`
<!-- trait ApplicationExt::fn get_flags -->
Gets the flags for `self`.

See `ApplicationFlags`.

# Returns

the flags for `self`
<!-- trait ApplicationExt::fn get_inactivity_timeout -->
Gets the current inactivity timeout for the application.

This is the amount of time (in milliseconds) after the last call to
`ApplicationExt::release` before the application stops running.

# Returns

the timeout, in milliseconds
<!-- trait ApplicationExt::fn get_is_busy -->
Gets the application's current busy state, as set through
`ApplicationExt::mark_busy` or `ApplicationExt::bind_busy_property`.

Feature: `v2_44`


# Returns

`true` if `self` is currenty marked as busy
<!-- trait ApplicationExt::fn get_is_registered -->
Checks if `self` is registered.

An application is registered if `ApplicationExt::register` has been
successfully called.

# Returns

`true` if `self` is registered
<!-- trait ApplicationExt::fn get_is_remote -->
Checks if `self` is remote.

If `self` is remote then it means that another instance of
application already exists (the 'primary' instance). Calls to
perform actions on `self` will result in the actions being
performed by the primary instance.

The value of this property cannot be accessed before
`ApplicationExt::register` has been called. See
`ApplicationExt::get_is_registered`.

# Returns

`true` if `self` is remote
<!-- trait ApplicationExt::fn get_resource_base_path -->
Gets the resource base path of `self`.

See `ApplicationExt::set_resource_base_path` for more information.

# Returns

the base resource path, if one is set
<!-- trait ApplicationExt::fn hold -->
Increases the use count of `self`.

Use this function to indicate that the application has a reason to
continue to run. For example, `ApplicationExt::hold` is called by GTK+
when a toplevel window is on the screen.

To cancel the hold, call `ApplicationExt::release`.
<!-- trait ApplicationExt::fn mark_busy -->
Increases the busy count of `self`.

Use this function to indicate that the application is busy, for instance
while a long running operation is pending.

The busy state will be exposed to other processes, so a session shell will
use that information to indicate the state to the user (e.g. with a
spinner).

To cancel the busy indication, use `ApplicationExt::unmark_busy`.
<!-- trait ApplicationExt::fn open -->
Opens the given files.

In essence, this results in the `Application::open` signal being emitted
in the primary instance.

`n_files` must be greater than zero.

`hint` is simply passed through to the ::open signal. It is
intended to be used by applications that have multiple modes for
opening files (eg: "view" vs "edit", etc). Unless you have a need
for this functionality, you should use "".

The application must be registered before calling this function
and it must have the `ApplicationFlags::HandlesOpen` flag set.
## `files`
an array of `GFiles` to open
## `n_files`
the length of the `files` array
## `hint`
a hint (or ""), but never `None`
<!-- trait ApplicationExt::fn quit -->
Immediately quits the application.

Upon return to the mainloop, `Application::run` will return,
calling only the 'shutdown' function before doing so.

The hold count is ignored.
Take care if your code has called `ApplicationExt::hold` on the application and
is therefore still expecting it to exist.
(Note that you may have called `ApplicationExt::hold` indirectly, for example
through `gtk_application_add_window`.)

The result of calling `Application::run` again after it returns is
unspecified.
<!-- trait ApplicationExt::fn register -->
Attempts registration of the application.

This is the point at which the application discovers if it is the
primary instance or merely acting as a remote for an already-existing
primary instance. This is implemented by attempting to acquire the
application identifier as a unique bus name on the session bus using
GDBus.

If there is no application ID or if `ApplicationFlags::NonUnique` was
given, then this process will always become the primary instance.

Due to the internal architecture of GDBus, method calls can be
dispatched at any time (even if a main loop is not running). For
this reason, you must ensure that any object paths that you wish to
register are registered before calling this function.

If the application has already been registered then `true` is
returned with no work performed.

The `Application::startup` signal is emitted if registration succeeds
and `self` is the primary instance (including the non-unique
case).

In the event of an error (such as `cancellable` being cancelled, or a
failure to connect to the session bus), `false` is returned and `error`
is set appropriately.

Note: the return value of this function is not an indicator that this
instance is or is not the primary instance of the application. See
`ApplicationExt::get_is_remote` for that.
## `cancellable`
a `Cancellable`, or `None`

# Returns

`true` if registration succeeded
<!-- trait ApplicationExt::fn release -->
Decrease the use count of `self`.

When the use count reaches zero, the application will stop running.

Never call this function except to cancel the effect of a previous
call to `ApplicationExt::hold`.
<!-- trait ApplicationExtManual::fn run -->
Runs the application.

This function is intended to be run from `main` and its return value
is intended to be returned by `main`. Although you are expected to pass
the `argc`, `argv` parameters from `main` to this function, it is possible
to pass `None` if `argv` is not available or commandline handling is not
required. Note that on Windows, `argc` and `argv` are ignored, and
`g_win32_get_command_line` is called internally (for proper support
of Unicode commandline arguments).

`Application` will attempt to parse the commandline arguments. You
can add commandline flags to the list of recognised options by way of
`ApplicationExt::add_main_option_entries`. After this, the
`Application::handle-local-options` signal is emitted, from which the
application can inspect the values of its `GOptionEntrys`.

`Application::handle-local-options` is a good place to handle options
such as `--version`, where an immediate reply from the local process is
desired (instead of communicating with an already-running instance).
A `Application::handle-local-options` handler can stop further processing
by returning a non-negative value, which then becomes the exit status of
the process.

What happens next depends on the flags: if
`ApplicationFlags::HandlesCommandLine` was specified then the remaining
commandline arguments are sent to the primary instance, where a
`Application::command-line` signal is emitted. Otherwise, the
remaining commandline arguments are assumed to be a list of files.
If there are no files listed, the application is activated via the
`Application::activate` signal. If there are one or more files, and
`ApplicationFlags::HandlesOpen` was specified then the files are opened
via the `Application::open` signal.

If you are interested in doing more complicated local handling of the
commandline then you should implement your own `Application` subclass
and override `local_command_line`. In this case, you most likely want
to return `true` from your `local_command_line` implementation to
suppress the default handling. See
[gapplication-example-cmdline2.c][gapplication-example-cmdline2]
for an example.

If, after the above is done, the use count of the application is zero
then the exit status is returned immediately. If the use count is
non-zero then the default main context is iterated until the use count
falls to zero, at which point 0 is returned.

If the `ApplicationFlags::IsService` flag is set, then the service will
run for as much as 10 seconds with a use count of zero while waiting
for the message that caused the activation to arrive. After that,
if the use count falls to zero the application will exit immediately,
except in the case that `ApplicationExt::set_inactivity_timeout` is in
use.

This function sets the prgname (`g_set_prgname`), if not already set,
to the basename of argv[0].

Much like `glib::MainLoop::run`, this function will acquire the main context
for the duration that the application is running.

Since 2.40, applications that are not explicitly flagged as services
or launchers (ie: neither `ApplicationFlags::IsService` or
`ApplicationFlags::IsLauncher` are given as flags) will check (from the
default handler for local_command_line) if "--gapplication-service"
was given in the command line. If this flag is present then normal
commandline processing is interrupted and the
`ApplicationFlags::IsService` flag is set. This provides a "compromise"
solution whereby running an application directly from the commandline
will invoke it in the normal way (which can be useful for debugging)
while still allowing applications to be D-Bus activated in service
mode. The D-Bus service file should invoke the executable with
"--gapplication-service" as the sole commandline argument. This
approach is suitable for use by most graphical applications but
should not be used from applications like editors that need precise
control over when processes invoked via the commandline will exit and
what their exit status will be.
## `argc`
the argc from `main` (or 0 if `argv` is `None`)
## `argv`

 the argv from `main`, or `None`

# Returns

the exit status
<!-- trait ApplicationExt::fn send_notification -->
Sends a notification on behalf of `self` to the desktop shell.
There is no guarantee that the notification is displayed immediately,
or even at all.

Notifications may persist after the application exits. It will be
D-Bus-activated when the notification or one of its actions is
activated.

Modifying `notification` after this call has no effect. However, the
object can be reused for a later call to this function.

`id` may be any string that uniquely identifies the event for the
application. It does not need to be in any special format. For
example, "new-message" might be appropriate for a notification about
new messages.

If a previous notification was sent with the same `id`, it will be
replaced with `notification` and shown again as if it was a new
notification. This works even for notifications sent from a previous
execution of the application, as long as `id` is the same string.

`id` may be `None`, but it is impossible to replace or withdraw
notifications without an id.

If `notification` is no longer relevant, it can be withdrawn with
`ApplicationExt::withdraw_notification`.
## `id`
id of the notification, or `None`
## `notification`
the `Notification` to send
<!-- trait ApplicationExt::fn set_application_id -->
Sets the unique identifier for `self`.

The application id can only be modified if `self` has not yet
been registered.

If non-`None`, the application id must be valid. See
`Application::id_is_valid`.
## `application_id`
the identifier for `self`
<!-- trait ApplicationExt::fn set_default -->
Sets or unsets the default application for the process, as returned
by `Application::get_default`.

This function does not take its own reference on `self`. If
`self` is destroyed then the default application will revert
back to `None`.
<!-- trait ApplicationExt::fn set_flags -->
Sets the flags for `self`.

The flags can only be modified if `self` has not yet been
registered.

See `ApplicationFlags`.
## `flags`
the flags for `self`
<!-- trait ApplicationExt::fn set_inactivity_timeout -->
Sets the current inactivity timeout for the application.

This is the amount of time (in milliseconds) after the last call to
`ApplicationExt::release` before the application stops running.

This call has no side effects of its own. The value set here is only
used for next time `ApplicationExt::release` drops the use count to
zero. Any timeouts currently in progress are not impacted.
## `inactivity_timeout`
the timeout, in milliseconds
<!-- trait ApplicationExt::fn set_option_context_description -->
Adds a description to the `self` option context.

See `glib::OptionContext::set_description` for more information.

Feature: `v2_56`

## `description`
a string to be shown in `--help` output
 after the list of options, or `None`
<!-- trait ApplicationExt::fn set_option_context_parameter_string -->
Sets the parameter string to be used by the commandline handling of `self`.

This function registers the argument to be passed to `glib::OptionContext::new`
when the internal `glib::OptionContext` of `self` is created.

See `glib::OptionContext::new` for more information about `parameter_string`.

Feature: `v2_56`

## `parameter_string`
a string which is displayed
 in the first line of `--help` output, after the usage summary `programname [OPTION...]`.
<!-- trait ApplicationExt::fn set_option_context_summary -->
Adds a summary to the `self` option context.

See `glib::OptionContext::set_summary` for more information.

Feature: `v2_56`

## `summary`
a string to be shown in `--help` output
 before the list of options, or `None`
<!-- trait ApplicationExt::fn set_resource_base_path -->
Sets (or unsets) the base resource path of `self`.

The path is used to automatically load various [application
resources][gresource] such as menu layouts and action descriptions.
The various types of resources will be found at fixed names relative
to the given base path.

By default, the resource base path is determined from the application
ID by prefixing '/' and replacing each '.' with '/'. This is done at
the time that the `Application` object is constructed. Changes to
the application ID after that point will not have an impact on the
resource base path.

As an example, if the application has an ID of "org.example.app" then
the default resource base path will be "/org/example/app". If this
is a ``GtkApplication`` (and you have not manually changed the path)
then Gtk will then search for the menus of the application at
"/org/example/app/gtk/menus.ui".

See `Resource` for more information about adding resources to your
application.

You can disable automatic resource loading functionality by setting
the path to `None`.

Changing the resource base path once the application is running is
not recommended. The point at which the resource path is consulted
for forming paths for various purposes is unspecified. When writing
a sub-class of `Application` you should either set the
`Application:resource-base-path` property at construction time, or call
this function during the instance initialization. Alternatively, you
can call this function in the `ApplicationClass.startup` virtual function,
before chaining up to the parent implementation.
## `resource_path`
the resource path to use
<!-- trait ApplicationExt::fn unbind_busy_property -->
Destroys a binding between `property` and the busy state of
`self` that was previously created with
`ApplicationExt::bind_busy_property`.

Feature: `v2_44`

## `object`
a `gobject::Object`
## `property`
the name of a boolean property of `object`
<!-- trait ApplicationExt::fn unmark_busy -->
Decreases the busy count of `self`.

When the busy count reaches zero, the new state will be propagated
to other processes.

This function must only be called to cancel the effect of a previous
call to `ApplicationExt::mark_busy`.
<!-- trait ApplicationExt::fn withdraw_notification -->
Withdraws a notification that was sent with
`ApplicationExt::send_notification`.

This call does nothing if a notification with `id` doesn't exist or
the notification was never sent.

This function works even for notifications sent in previous
executions of this application, as long `id` is the same as it was for
the sent notification.

Note that notifications are dismissed when the user clicks on one
of the buttons in a notification or triggers its default action, so
there is no need to explicitly withdraw the notification in that case.
## `id`
id of a previously sent notification
<!-- trait ApplicationExt::fn connect_activate -->
The ::activate signal is emitted on the primary instance when an
activation occurs. See `ApplicationExt::activate`.
<!-- trait ApplicationExt::fn connect_command_line -->
The ::command-line signal is emitted on the primary instance when
a commandline is not handled locally. See `Application::run` and
the `ApplicationCommandLine` documentation for more information.
## `command_line`
a `ApplicationCommandLine` representing the
 passed commandline

# Returns

An integer that is set as the exit status for the calling
 process. See `ApplicationCommandLineExt::set_exit_status`.
<!-- trait ApplicationExt::fn connect_handle_local_options -->
The ::handle-local-options signal is emitted on the local instance
after the parsing of the commandline options has occurred.

You can add options to be recognised during commandline option
parsing using `ApplicationExt::add_main_option_entries` and
`ApplicationExt::add_option_group`.

Signal handlers can inspect `options` (along with values pointed to
from the `arg_data` of an installed `GOptionEntrys`) in order to
decide to perform certain actions, including direct local handling
(which may be useful for options like --version).

In the event that the application is marked
`ApplicationFlags::HandlesCommandLine` the "normal processing" will
send the `options` dictionary to the primary instance where it can be
read with `ApplicationCommandLineExt::get_options_dict`. The signal
handler can modify the dictionary before returning, and the
modified dictionary will be sent.

In the event that `ApplicationFlags::HandlesCommandLine` is not set,
"normal processing" will treat the remaining uncollected command
line arguments as filenames or URIs. If there are no arguments,
the application is activated by `ApplicationExt::activate`. One or
more arguments results in a call to `ApplicationExt::open`.

If you want to handle the local commandline arguments for yourself
by converting them to calls to `ApplicationExt::open` or
`ActionGroup::activate_action` then you must be sure to register
the application first. You should probably not call
`ApplicationExt::activate` for yourself, however: just return -1 and
allow the default handler to do it for you. This will ensure that
the `--gapplication-service` switch works properly (i.e. no activation
in that case).

Note that this signal is emitted from the default implementation of
`local_command_line`. If you override that function and don't
chain up then this signal will never be emitted.

You can override `local_command_line` if you need more powerful
capabilities than what is provided here, but this should not
normally be required.
## `options`
the options dictionary

# Returns

an exit code. If you have handled your options and want
to exit the process, return a non-negative option, 0 for success,
and a positive value for failure. To continue, return -1 to let
the default option processing continue.
<!-- trait ApplicationExtManual::fn connect_open -->
The ::open signal is emitted on the primary instance when there are
files to open. See `ApplicationExt::open` for more information.
## `files`
an array of `GFiles`
## `n_files`
the length of `files`
## `hint`
a hint provided by the calling instance
<!-- trait ApplicationExt::fn connect_shutdown -->
The ::shutdown signal is emitted only on the registered primary instance
immediately after the main loop terminates.
<!-- trait ApplicationExt::fn connect_startup -->
The ::startup signal is emitted on the primary instance immediately
after registration. See `ApplicationExt::register`.
<!-- trait ApplicationExt::fn get_property_is_busy -->
Whether the application is currently marked as busy through
`ApplicationExt::mark_busy` or `ApplicationExt::bind_busy_property`.

Feature: `v2_44`

<!-- struct ApplicationCommandLine -->
`ApplicationCommandLine` represents a command-line invocation of
an application. It is created by `Application` and emitted
in the `Application::command-line` signal and virtual function.

The class contains the list of arguments that the program was invoked
with. It is also possible to query if the commandline invocation was
local (ie: the current process is running in direct response to the
invocation) or remote (ie: some other process forwarded the
commandline to this process).

The GApplicationCommandLine object can provide the `argc` and `argv`
parameters for use with the `glib::OptionContext` command-line parsing API,
with the `ApplicationCommandLineExt::get_arguments` function. See
[gapplication-example-cmdline3.c][gapplication-example-cmdline3]
for an example.

The exit status of the originally-invoked process may be set and
messages can be printed to stdout or stderr of that process. The
lifecycle of the originally-invoked process is tied to the lifecycle
of this object (ie: the process exits when the last reference is
dropped).

The main use for `ApplicationCommandLine` (and the
`Application::command-line` signal) is 'Emacs server' like use cases:
You can set the `EDITOR` environment variable to have e.g. git use
your favourite editor to edit commit messages, and if you already
have an instance of the editor running, the editing will happen
in the running instance, instead of opening a new one. An important
aspect of this use case is that the process that gets started by git
does not return until the editing is done.

Normally, the commandline is completely handled in the
`Application::command-line` handler. The launching instance exits
once the signal handler in the primary instance has returned, and
the return value of the signal handler becomes the exit status
of the launching instance.

```C
static int
command_line (GApplication            *application,
              GApplicationCommandLine *cmdline)
{
  gchar **argv;
  gint argc;
  gint i;

  argv = g_application_command_line_get_arguments (cmdline, &argc);

  g_application_command_line_print (cmdline,
                                    "This text is written back\n"
                                    "to stdout of the caller\n");

  for (i = 0; i < argc; i++)
    g_print ("argument %d: %s\n", i, argv[i]);

  g_strfreev (argv);

  return 0;
}
```
The complete example can be found here:
[gapplication-example-cmdline.c](https://git.gnome.org/browse/glib/tree/gio/tests/gapplication-example-cmdline.c)

In more complicated cases, the handling of the comandline can be
split between the launcher and the primary instance.

```C
static gboolean
 test_local_cmdline (GApplication   *application,
                     gchar        ***arguments,
                     gint           *exit_status)
{
  gint i, j;
  gchar **argv;

  argv = *arguments;

  i = 1;
  while (argv[i])
    {
      if (g_str_has_prefix (argv[i], "--local-"))
        {
          g_print ("handling argument %s locally\n", argv[i]);
          g_free (argv[i]);
          for (j = i; argv[j]; j++)
            argv[j] = argv[j + 1];
        }
      else
        {
          g_print ("not handling argument %s locally\n", argv[i]);
          i++;
        }
    }

  *exit_status = 0;

  return FALSE;
}

static void
test_application_class_init (TestApplicationClass *class)
{
  G_APPLICATION_CLASS (class)->local_command_line = test_local_cmdline;

  ...
}
```
In this example of split commandline handling, options that start
with `--local-` are handled locally, all other options are passed
to the `Application::command-line` handler which runs in the primary
instance.

The complete example can be found here:
[gapplication-example-cmdline2.c](https://git.gnome.org/browse/glib/tree/gio/tests/gapplication-example-cmdline2.c)

If handling the commandline requires a lot of work, it may
be better to defer it.

```C
static gboolean
my_cmdline_handler (gpointer data)
{
  GApplicationCommandLine *cmdline = data;

  // do the heavy lifting in an idle

  g_application_command_line_set_exit_status (cmdline, 0);
  g_object_unref (cmdline); // this releases the application

  return G_SOURCE_REMOVE;
}

static int
command_line (GApplication            *application,
              GApplicationCommandLine *cmdline)
{
  // keep the application running until we are done with this commandline
  g_application_hold (application);

  g_object_set_data_full (G_OBJECT (cmdline),
                          "application", application,
                          (GDestroyNotify)g_application_release);

  g_object_ref (cmdline);
  g_idle_add (my_cmdline_handler, cmdline);

  return 0;
}
```
In this example the commandline is not completely handled before
the `Application::command-line` handler returns. Instead, we keep
a reference to the `ApplicationCommandLine` object and handle it
later (in this example, in an idle). Note that it is necessary to
hold the application until you are done with the commandline.

The complete example can be found here:
[gapplication-example-cmdline3.c](https://git.gnome.org/browse/glib/tree/gio/tests/gapplication-example-cmdline3.c)

# Implements

[`ApplicationCommandLineExt`](trait.ApplicationCommandLineExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait ApplicationCommandLineExt -->
Trait containing all `ApplicationCommandLine` methods.

# Implementors

[`ApplicationCommandLine`](struct.ApplicationCommandLine.html)
<!-- trait ApplicationCommandLineExt::fn create_file_for_arg -->
Creates a `File` corresponding to a filename that was given as part
of the invocation of `self`.

This differs from `File::new_for_commandline_arg` in that it
resolves relative pathnames using the current working directory of
the invoking process rather than the local process.
## `arg`
an argument from `self`

# Returns

a new `File`
<!-- trait ApplicationCommandLineExt::fn get_arguments -->
Gets the list of arguments that was passed on the command line.

The strings in the array may contain non-UTF-8 data on UNIX (such as
filenames or arguments given in the system locale) but are always in
UTF-8 on Windows.

If you wish to use the return value with `glib::OptionContext`, you must
use `glib::OptionContext::parse_strv`.

The return value is `None`-terminated and should be freed using
`g_strfreev`.
## `argc`
the length of the arguments array, or `None`

# Returns


 the string array containing the arguments (the argv)
<!-- trait ApplicationCommandLineExt::fn get_cwd -->
Gets the working directory of the command line invocation.
The string may contain non-utf8 data.

It is possible that the remote application did not send a working
directory, so this may be `None`.

The return value should not be modified or freed and is valid for as
long as `self` exists.

# Returns

the current directory, or `None`
<!-- trait ApplicationCommandLineExt::fn get_environ -->
Gets the contents of the 'environ' variable of the command line
invocation, as would be returned by `g_get_environ`, ie as a
`None`-terminated list of strings in the form 'NAME=VALUE'.
The strings may contain non-utf8 data.

The remote application usually does not send an environment. Use
`ApplicationFlags::SendEnvironment` to affect that. Even with this flag
set it is possible that the environment is still not available (due
to invocation messages from other applications).

The return value should not be modified or freed and is valid for as
long as `self` exists.

See `ApplicationCommandLineExt::getenv` if you are only interested
in the value of a single environment variable.

# Returns


 the environment strings, or `None` if they were not sent
<!-- trait ApplicationCommandLineExt::fn get_exit_status -->
Gets the exit status of `self`. See
`ApplicationCommandLineExt::set_exit_status` for more information.

# Returns

the exit status
<!-- trait ApplicationCommandLineExt::fn get_is_remote -->
Determines if `self` represents a remote invocation.

# Returns

`true` if the invocation was remote
<!-- trait ApplicationCommandLineExt::fn get_options_dict -->
Gets the options there were passed to `g_application_command_line`.

If you did not override `local_command_line` then these are the same
options that were parsed according to the `GOptionEntrys` added to the
application with `ApplicationExt::add_main_option_entries` and possibly
modified from your GApplication::handle-local-options handler.

If no options were sent then an empty dictionary is returned so that
you don't need to check for `None`.

# Returns

a `glib::VariantDict` with the options
<!-- trait ApplicationCommandLineExt::fn get_platform_data -->
Gets the platform data associated with the invocation of `self`.

This is a `glib::Variant` dictionary containing information about the
context in which the invocation occurred. It typically contains
information like the current working directory and the startup
notification ID.

For local invocation, it will be `None`.

# Returns

the platform data, or `None`
<!-- trait ApplicationCommandLineExt::fn get_stdin -->
Gets the stdin of the invoking process.

The `InputStream` can be used to read data passed to the standard
input of the invoking process.
This doesn't work on all platforms. Presently, it is only available
on UNIX when using a DBus daemon capable of passing file descriptors.
If stdin is not available then `None` will be returned. In the
future, support may be expanded to other platforms.

You must only call this function once per commandline invocation.

# Returns

a `InputStream` for stdin
<!-- trait ApplicationCommandLineExt::fn getenv -->
Gets the value of a particular environment variable of the command
line invocation, as would be returned by `g_getenv`. The strings may
contain non-utf8 data.

The remote application usually does not send an environment. Use
`ApplicationFlags::SendEnvironment` to affect that. Even with this flag
set it is possible that the environment is still not available (due
to invocation messages from other applications).

The return value should not be modified or freed and is valid for as
long as `self` exists.
## `name`
the environment variable to get

# Returns

the value of the variable, or `None` if unset or unsent
<!-- trait ApplicationCommandLineExt::fn print -->
Formats a message and prints it using the stdout print handler in the
invoking process.

If `self` is a local invocation then this is exactly equivalent to
`g_print`. If `self` is remote then this is equivalent to calling
`g_print` in the invoking process.
## `format`
a printf-style format string
<!-- trait ApplicationCommandLineExt::fn printerr -->
Formats a message and prints it using the stderr print handler in the
invoking process.

If `self` is a local invocation then this is exactly equivalent to
`g_printerr`. If `self` is remote then this is equivalent to
calling `g_printerr` in the invoking process.
## `format`
a printf-style format string
<!-- trait ApplicationCommandLineExt::fn set_exit_status -->
Sets the exit status that will be used when the invoking process
exits.

The return value of the `Application::command-line` signal is
passed to this function when the handler returns. This is the usual
way of setting the exit status.

In the event that you want the remote invocation to continue running
and want to decide on the exit status in the future, you can use this
call. For the case of a remote invocation, the remote process will
typically exit when the last reference is dropped on `self`. The
exit status of the remote process will be equal to the last value
that was set with this function.

In the case that the commandline invocation is local, the situation
is slightly more complicated. If the commandline invocation results
in the mainloop running (ie: because the use-count of the application
increased to a non-zero value) then the application is considered to
have been 'successful' in a certain sense, and the exit status is
always zero. If the application use count is zero, though, the exit
status of the local `ApplicationCommandLine` is used.
## `exit_status`
the exit status
<!-- struct BufferedInputStream -->
Buffered input stream implements `FilterInputStream` and provides
for buffered reads.

By default, `BufferedInputStream`'s buffer size is set at 4 kilobytes.

To create a buffered input stream, use `BufferedInputStream::new`,
or `BufferedInputStream::new_sized` to specify the buffer's size at
construction.

To get the size of a buffer within a buffered input stream, use
`BufferedInputStreamExt::get_buffer_size`. To change the size of a
buffered input stream's buffer, use
`BufferedInputStreamExt::set_buffer_size`. Note that the buffer's size
cannot be reduced below the size of the data within the buffer.

# Implements

[`BufferedInputStreamExt`](trait.BufferedInputStreamExt.html), [`FilterInputStreamExt`](trait.FilterInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html)
<!-- trait BufferedInputStreamExt -->
Trait containing all `BufferedInputStream` methods.

# Implementors

[`BufferedInputStream`](struct.BufferedInputStream.html), [`DataInputStream`](struct.DataInputStream.html)
<!-- impl BufferedInputStream::fn new -->
Creates a new `InputStream` from the given `base_stream`, with
a buffer set to the default size (4 kilobytes).
## `base_stream`
a `InputStream`

# Returns

a `InputStream` for the given `base_stream`.
<!-- impl BufferedInputStream::fn new_sized -->
Creates a new `BufferedInputStream` from the given `base_stream`,
with a buffer set to `size`.
## `base_stream`
a `InputStream`
## `size`
a `gsize`

# Returns

a `InputStream`.
<!-- trait BufferedInputStreamExt::fn fill -->
Tries to read `count` bytes from the stream into the buffer.
Will block during this read.

If `count` is zero, returns zero and does nothing. A value of `count`
larger than `G_MAXSSIZE` will cause a `IOErrorEnum::InvalidArgument` error.

On success, the number of bytes read into the buffer is returned.
It is not an error if this is not the same as the requested size, as it
can happen e.g. near the end of a file. Zero is returned on end of file
(or if `count` is zero), but never otherwise.

If `count` is -1 then the attempted read size is equal to the number of
bytes that are required to fill the buffer.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.

On error -1 is returned and `error` is set accordingly.

For the asynchronous, non-blocking, version of this function, see
`BufferedInputStreamExt::fill_async`.
## `count`
the number of bytes that will be read from the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

the number of bytes read into `self`'s buffer, up to `count`,
 or -1 on error.
<!-- trait BufferedInputStreamExt::fn fill_async -->
Reads data into `self`'s buffer asynchronously, up to `count` size.
`io_priority` can be used to prioritize reads. For the synchronous
version of this function, see `BufferedInputStreamExt::fill`.

If `count` is -1 then the attempted read size is equal to the number
of bytes that are required to fill the buffer.
## `count`
the number of bytes that will be read from the stream
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object
## `callback`
a `GAsyncReadyCallback`
## `user_data`
a `gpointer`
<!-- trait BufferedInputStreamExt::fn fill_finish -->
Finishes an asynchronous read.
## `result`
a `AsyncResult`

# Returns

a `gssize` of the read stream, or `-1` on an error.
<!-- trait BufferedInputStreamExt::fn get_available -->
Gets the size of the available data within the stream.

# Returns

size of the available stream.
<!-- trait BufferedInputStreamExt::fn get_buffer_size -->
Gets the size of the input buffer.

# Returns

the current buffer size.
<!-- trait BufferedInputStreamExt::fn peek -->
Peeks in the buffer, copying data of size `count` into `buffer`,
offset `offset` bytes.
## `buffer`
a pointer to
 an allocated chunk of memory
## `offset`
a `gsize`
## `count`
a `gsize`

# Returns

a `gsize` of the number of bytes peeked, or -1 on error.
<!-- trait BufferedInputStreamExt::fn peek_buffer -->
Returns the buffer with the currently available bytes. The returned
buffer must not be modified and will become invalid when reading from
the stream or filling the buffer.
## `count`
a `gsize` to get the number of bytes available in the buffer

# Returns


 read-only buffer
<!-- trait BufferedInputStreamExt::fn read_byte -->
Tries to read a single byte from the stream or the buffer. Will block
during this read.

On success, the byte read from the stream is returned. On end of stream
-1 is returned but it's not an exceptional error and `error` is not set.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.

On error -1 is returned and `error` is set accordingly.
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

the byte read from the `self`, or -1 on end of stream or error.
<!-- trait BufferedInputStreamExt::fn set_buffer_size -->
Sets the size of the internal buffer of `self` to `size`, or to the
size of the contents of the buffer. The buffer can never be resized
smaller than its current contents.
## `size`
a `gsize`
<!-- struct BufferedOutputStream -->
Buffered output stream implements `FilterOutputStream` and provides
for buffered writes.

By default, `BufferedOutputStream`'s buffer size is set at 4 kilobytes.

To create a buffered output stream, use `BufferedOutputStream::new`,
or `BufferedOutputStream::new_sized` to specify the buffer's size
at construction.

To get the size of a buffer within a buffered input stream, use
`BufferedOutputStreamExt::get_buffer_size`. To change the size of a
buffered output stream's buffer, use
`BufferedOutputStreamExt::set_buffer_size`. Note that the buffer's
size cannot be reduced below the size of the data within the buffer.

# Implements

[`BufferedOutputStreamExt`](trait.BufferedOutputStreamExt.html), [`FilterOutputStreamExt`](trait.FilterOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html)
<!-- trait BufferedOutputStreamExt -->
Trait containing all `BufferedOutputStream` methods.

# Implementors

[`BufferedOutputStream`](struct.BufferedOutputStream.html)
<!-- impl BufferedOutputStream::fn new -->
Creates a new buffered output stream for a base stream.
## `base_stream`
a `OutputStream`.

# Returns

a `OutputStream` for the given `base_stream`.
<!-- impl BufferedOutputStream::fn new_sized -->
Creates a new buffered output stream with a given buffer size.
## `base_stream`
a `OutputStream`.
## `size`
a `gsize`.

# Returns

a `OutputStream` with an internal buffer set to `size`.
<!-- trait BufferedOutputStreamExt::fn get_auto_grow -->
Checks if the buffer automatically grows as data is added.

# Returns

`true` if the `self`'s buffer automatically grows,
`false` otherwise.
<!-- trait BufferedOutputStreamExt::fn get_buffer_size -->
Gets the size of the buffer in the `self`.

# Returns

the current size of the buffer.
<!-- trait BufferedOutputStreamExt::fn set_auto_grow -->
Sets whether or not the `self`'s buffer should automatically grow.
If `auto_grow` is true, then each write will just make the buffer
larger, and you must manually flush the buffer to actually write out
the data to the underlying stream.
## `auto_grow`
a `gboolean`.
<!-- trait BufferedOutputStreamExt::fn set_buffer_size -->
Sets the size of the internal buffer to `size`.
## `size`
a `gsize`.
<!-- struct BytesIcon -->
`BytesIcon` specifies an image held in memory in a common format (usually
png) to be used as icon.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`IconExt`](trait.IconExt.html), [`LoadableIconExt`](trait.LoadableIconExt.html)
<!-- impl BytesIcon::fn new -->
Creates a new icon for a bytes.
## `bytes`
a `glib::Bytes`.

# Returns

a `Icon` for the given
 `bytes`, or `None` on error.
<!-- impl BytesIcon::fn get_bytes -->
Gets the `glib::Bytes` associated with the given `self`.

# Returns

a `glib::Bytes`, or `None`.
<!-- impl BytesIcon::fn get_property_bytes -->
The bytes containing the icon.
<!-- impl BytesIcon::fn set_property_bytes -->
The bytes containing the icon.
<!-- struct Cancellable -->
GCancellable is a thread-safe operation cancellation stack used
throughout GIO to allow for cancellation of synchronous and
asynchronous operations.

# Implements

[`CancellableExt`](trait.CancellableExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait CancellableExt -->
Trait containing all `Cancellable` methods.

# Implementors

[`Cancellable`](struct.Cancellable.html)
<!-- impl Cancellable::fn new -->
Creates a new `Cancellable` object.

Applications that want to start one or more operations
that should be cancellable should create a `Cancellable`
and pass it to the operations.

One `Cancellable` can be used in multiple consecutive
operations or in multiple concurrent operations.

# Returns

a `Cancellable`.
<!-- impl Cancellable::fn get_current -->
Gets the top cancellable from the stack.

# Returns

a `Cancellable` from the top
of the stack, or `None` if the stack is empty.
<!-- trait CancellableExt::fn cancel -->
Will set `self` to cancelled, and will emit the
`Cancellable::cancelled` signal. (However, see the warning about
race conditions in the documentation for that signal if you are
planning to connect to it.)

This function is thread-safe. In other words, you can safely call
it from a thread other than the one running the operation that was
passed the `self`.

If `self` is `None`, this function returns immediately for convenience.

The convention within GIO is that cancelling an asynchronous
operation causes it to complete asynchronously. That is, if you
cancel the operation from the same thread in which it is running,
then the operation's `GAsyncReadyCallback` will not be invoked until
the application returns to the main loop.
<!-- trait CancellableExt::fn connect -->
Convenience function to connect to the `Cancellable::cancelled`
signal. Also handles the race condition that may happen
if the cancellable is cancelled right before connecting.

`callback` is called at most once, either directly at the
time of the connect if `self` is already cancelled,
or when `self` is cancelled in some thread.

`data_destroy_func` will be called when the handler is
disconnected, or immediately if the cancellable is already
cancelled.

See `Cancellable::cancelled` for details on how to use this.

Since GLib 2.40, the lock protecting `self` is not held when
`callback` is invoked. This lifts a restriction in place for
earlier GLib versions which now makes it easier to write cleanup
code that unconditionally invokes e.g. `CancellableExt::cancel`.
## `callback`
The `GCallback` to connect.
## `data`
Data to pass to `callback`.
## `data_destroy_func`
Free function for `data` or `None`.

# Returns

The id of the signal handler or 0 if `self` has already
 been cancelled.
<!-- trait CancellableExt::fn disconnect -->
Disconnects a handler from a cancellable instance similar to
`g_signal_handler_disconnect`. Additionally, in the event that a
signal handler is currently running, this call will block until the
handler has finished. Calling this function from a
`Cancellable::cancelled` signal handler will therefore result in a
deadlock.

This avoids a race condition where a thread cancels at the
same time as the cancellable operation is finished and the
signal handler is removed. See `Cancellable::cancelled` for
details on how to use this.

If `self` is `None` or `handler_id` is `0` this function does
nothing.
## `handler_id`
Handler id of the handler to be disconnected, or `0`.
<!-- trait CancellableExt::fn get_fd -->
Gets the file descriptor for a cancellable job. This can be used to
implement cancellable operations on Unix systems. The returned fd will
turn readable when `self` is cancelled.

You are not supposed to read from the fd yourself, just check for
readable status. Reading to unset the readable status is done
with `Cancellable::reset`.

After a successful return from this function, you should use
`CancellableExt::release_fd` to free up resources allocated for
the returned file descriptor.

See also `CancellableExt::make_pollfd`.

# Returns

A valid file descriptor. %-1 if the file descriptor
is not supported, or on errors.
<!-- trait CancellableExt::fn is_cancelled -->
Checks if a cancellable job has been cancelled.

# Returns

`true` if `self` is cancelled,
FALSE if called with `None` or if item is not cancelled.
<!-- trait CancellableExt::fn make_pollfd -->
Creates a `glib::PollFD` corresponding to `self`; this can be passed
to `g_poll` and used to poll for cancellation. This is useful both
for unix systems without a native poll and for portability to
windows.

When this function returns `true`, you should use
`CancellableExt::release_fd` to free up resources allocated for the
`pollfd`. After a `false` return, do not call `CancellableExt::release_fd`.

If this function returns `false`, either no `self` was given or
resource limits prevent this function from allocating the necessary
structures for polling. (On Linux, you will likely have reached
the maximum number of file descriptors.) The suggested way to handle
these cases is to ignore the `self`.

You are not supposed to read from the fd yourself, just check for
readable status. Reading to unset the readable status is done
with `Cancellable::reset`.
## `pollfd`
a pointer to a `glib::PollFD`

# Returns

`true` if `pollfd` was successfully initialized, `false` on
 failure to prepare the cancellable.
<!-- trait CancellableExt::fn pop_current -->
Pops `self` off the cancellable stack (verifying that `self`
is on the top of the stack).
<!-- trait CancellableExt::fn push_current -->
Pushes `self` onto the cancellable stack. The current
cancellable can then be received using `Cancellable::get_current`.

This is useful when implementing cancellable operations in
code that does not allow you to pass down the cancellable object.

This is typically called automatically by e.g. `File` operations,
so you rarely have to call this yourself.
<!-- trait CancellableExt::fn release_fd -->
Releases a resources previously allocated by `CancellableExt::get_fd`
or `CancellableExt::make_pollfd`.

For compatibility reasons with older releases, calling this function
is not strictly required, the resources will be automatically freed
when the `self` is finalized. However, the `self` will
block scarce file descriptors until it is finalized if this function
is not called. This can cause the application to run out of file
descriptors when many `GCancellables` are used at the same time.
<!-- trait CancellableExt::fn reset -->
Resets `self` to its uncancelled state.

If cancellable is currently in use by any cancellable operation
then the behavior of this function is undefined.

Note that it is generally not a good idea to reuse an existing
cancellable for more operations after it has been cancelled once,
as this function might tempt you to do. The recommended practice
is to drop the reference to a cancellable after cancelling it,
and let it die with the outstanding async operations. You should
create a fresh cancellable for further async operations.
<!-- trait CancellableExt::fn set_error_if_cancelled -->
If the `self` is cancelled, sets the error to notify
that the operation was cancelled.

# Returns

`true` if `self` was cancelled, `false` if it was not
<!-- trait CancellableExt::fn source_new -->
Creates a source that triggers if `self` is cancelled and
calls its callback of type `GCancellableSourceFunc`. This is
primarily useful for attaching to another (non-cancellable) source
with `glib::Source::add_child_source` to add cancellability to it.

For convenience, you can call this with a `None` `Cancellable`,
in which case the source will never trigger.

The new `glib::Source` will hold a reference to the `Cancellable`.

# Returns

the new `glib::Source`.
<!-- trait CancellableExt::fn connect_cancelled -->
Emitted when the operation has been cancelled.

Can be used by implementations of cancellable operations. If the
operation is cancelled from another thread, the signal will be
emitted in the thread that cancelled the operation, not the
thread that is running the operation.

Note that disconnecting from this signal (or any signal) in a
multi-threaded program is prone to race conditions. For instance
it is possible that a signal handler may be invoked even after
a call to `g_signal_handler_disconnect` for that handler has
already returned.

There is also a problem when cancellation happens right before
connecting to the signal. If this happens the signal will
unexpectedly not be emitted, and checking before connecting to
the signal leaves a race condition where this is still happening.

In order to make it safe and easy to connect handlers there
are two helper functions: `CancellableExt::connect` and
`CancellableExt::disconnect` which protect against problems
like this.

An example of how to us this:

```C
    // Make sure we don't do unnecessary work if already cancelled
    if (g_cancellable_set_error_if_cancelled (cancellable, error))
      return;

    // Set up all the data needed to be able to handle cancellation
    // of the operation
    my_data = my_data_new (...);

    id = 0;
    if (cancellable)
      id = g_cancellable_connect (cancellable,
                      G_CALLBACK (cancelled_handler)
                      data, NULL);

    // cancellable operation here...

    g_cancellable_disconnect (cancellable, id);

    // cancelled_handler is never called after this, it is now safe
    // to free the data
    my_data_free (my_data);
```

Note that the cancelled signal is emitted in the thread that
the user cancelled from, which may be the main thread. So, the
cancellable signal should not do something that can block.
<!-- struct CharsetConverter -->
`CharsetConverter` is an implementation of `Converter` based on
GIConv.

# Implements

[`CharsetConverterExt`](trait.CharsetConverterExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ConverterExt`](trait.ConverterExt.html), [`ConverterExtManual`](prelude/trait.ConverterExtManual.html)
<!-- trait CharsetConverterExt -->
Trait containing all `CharsetConverter` methods.

# Implementors

[`CharsetConverter`](struct.CharsetConverter.html)
<!-- impl CharsetConverter::fn new -->
Creates a new `CharsetConverter`.
## `to_charset`
destination charset
## `from_charset`
source charset

# Returns

a new `CharsetConverter` or `None` on error.
<!-- trait CharsetConverterExt::fn get_num_fallbacks -->
Gets the number of fallbacks that `self` has applied so far.

# Returns

the number of fallbacks that `self` has applied
<!-- trait CharsetConverterExt::fn get_use_fallback -->
Gets the `CharsetConverter:use-fallback` property.

# Returns

`true` if fallbacks are used by `self`
<!-- trait CharsetConverterExt::fn set_use_fallback -->
Sets the `CharsetConverter:use-fallback` property.
## `use_fallback`
`true` to use fallbacks
<!-- struct Converter -->
`Converter` is implemented by objects that convert
binary data in various ways. The conversion can be
stateful and may fail at any place.

Some example conversions are: character set conversion,
compression, decompression and regular expression
replace.

# Implements

[`ConverterExt`](trait.ConverterExt.html), [`ConverterExtManual`](prelude/trait.ConverterExtManual.html)
<!-- trait ConverterExt -->
Trait containing all `Converter` methods.

# Implementors

[`CharsetConverter`](struct.CharsetConverter.html), [`Converter`](struct.Converter.html), [`ZlibCompressor`](struct.ZlibCompressor.html), [`ZlibDecompressor`](struct.ZlibDecompressor.html)
<!-- trait ConverterExtManual::fn convert -->
This is the main operation used when converting data. It is to be called
multiple times in a loop, and each time it will do some work, i.e.
producing some output (in `outbuf`) or consuming some input (from `inbuf`) or
both. If its not possible to do any work an error is returned.

Note that a single call may not consume all input (or any input at all).
Also a call may produce output even if given no input, due to state stored
in the converter producing output.

If any data was either produced or consumed, and then an error happens, then
only the successful conversion is reported and the error is returned on the
next call.

A full conversion loop involves calling this method repeatedly, each time
giving it new input and space output space. When there is no more input
data after the data in `inbuf`, the flag `ConverterFlags::InputAtEnd` must be set.
The loop will be (unless some error happens) returning `ConverterResult::Converted`
each time until all data is consumed and all output is produced, then
`ConverterResult::Finished` is returned instead. Note, that `ConverterResult::Finished`
may be returned even if `ConverterFlags::InputAtEnd` is not set, for instance
in a decompression converter where the end of data is detectable from the
data (and there might even be other data after the end of the compressed data).

When some data has successfully been converted `bytes_read` and is set to
the number of bytes read from `inbuf`, and `bytes_written` is set to indicate
how many bytes was written to `outbuf`. If there are more data to output
or consume (i.e. unless the `ConverterFlags::InputAtEnd` is specified) then
`ConverterResult::Converted` is returned, and if no more data is to be output
then `ConverterResult::Finished` is returned.

On error `ConverterResult::Error` is returned and `error` is set accordingly.
Some errors need special handling:

`IOErrorEnum::NoSpace` is returned if there is not enough space
to write the resulting converted data, the application should
call the function again with a larger `outbuf` to continue.

`IOErrorEnum::PartialInput` is returned if there is not enough
input to fully determine what the conversion should produce,
and the `ConverterFlags::InputAtEnd` flag is not set. This happens for
example with an incomplete multibyte sequence when converting text,
or when a regexp matches up to the end of the input (and may match
further input). It may also happen when `inbuf_size` is zero and
there is no more data to produce.

When this happens the application should read more input and then
call the function again. If further input shows that there is no
more data call the function again with the same data but with
the `ConverterFlags::InputAtEnd` flag set. This may cause the conversion
to finish as e.g. in the regexp match case (or, to fail again with
`IOErrorEnum::PartialInput` in e.g. a charset conversion where the
input is actually partial).

After `Converter::convert` has returned `ConverterResult::Finished` the
converter object is in an invalid state where its not allowed
to call `Converter::convert` anymore. At this time you can only
free the object or call `Converter::reset` to reset it to the
initial state.

If the flag `ConverterFlags::Flush` is set then conversion is modified
to try to write out all internal state to the output. The application
has to call the function multiple times with the flag set, and when
the available input has been consumed and all internal state has
been produced then `ConverterResult::Flushed` (or `ConverterResult::Finished` if
really at the end) is returned instead of `ConverterResult::Converted`.
This is somewhat similar to what happens at the end of the input stream,
but done in the middle of the data.

This has different meanings for different conversions. For instance
in a compression converter it would mean that we flush all the
compression state into output such that if you uncompress the
compressed data you get back all the input data. Doing this may
make the final file larger due to padding though. Another example
is a regexp conversion, where if you at the end of the flushed data
have a match, but there is also a potential longer match. In the
non-flushed case we would ask for more input, but when flushing we
treat this as the end of input and do the match.

Flushing is not always possible (like if a charset converter flushes
at a partial multibyte sequence). Converters are supposed to try
to produce as much output as possible and then return an error
(typically `IOErrorEnum::PartialInput`).
## `inbuf`
the buffer
 containing the data to convert.
## `inbuf_size`
the number of bytes in `inbuf`
## `outbuf`
a buffer to write
 converted data in.
## `outbuf_size`
the number of bytes in `outbuf`, must be at least one
## `flags`
a `ConverterFlags` controlling the conversion details
## `bytes_read`
will be set to the number of bytes read from `inbuf` on success
## `bytes_written`
will be set to the number of bytes written to `outbuf` on success

# Returns

a `ConverterResult`, `ConverterResult::Error` on error.
<!-- trait ConverterExt::fn reset -->
Resets all internal state in the converter, making it behave
as if it was just created. If the converter has any internal
state that would produce output then that output is lost.
<!-- struct ConverterInputStream -->
Converter input stream implements `InputStream` and allows
conversion of data of various types during reading.

As of GLib 2.34, `ConverterInputStream` implements
`PollableInputStream`.

# Implements

[`ConverterInputStreamExt`](trait.ConverterInputStreamExt.html), [`FilterInputStreamExt`](trait.FilterInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableInputStreamExt`](trait.PollableInputStreamExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html), [`PollableInputStreamExtManual`](prelude/trait.PollableInputStreamExtManual.html)
<!-- trait ConverterInputStreamExt -->
Trait containing all `ConverterInputStream` methods.

# Implementors

[`ConverterInputStream`](struct.ConverterInputStream.html)
<!-- impl ConverterInputStream::fn new -->
Creates a new converter input stream for the `base_stream`.
## `base_stream`
a `InputStream`
## `converter`
a `Converter`

# Returns

a new `InputStream`.
<!-- trait ConverterInputStreamExt::fn get_converter -->
Gets the `Converter` that is used by `self`.

# Returns

the converter of the converter input stream
<!-- struct ConverterOutputStream -->
Converter output stream implements `OutputStream` and allows
conversion of data of various types during reading.

As of GLib 2.34, `ConverterOutputStream` implements
`PollableOutputStream`.

# Implements

[`ConverterOutputStreamExt`](trait.ConverterOutputStreamExt.html), [`FilterOutputStreamExt`](trait.FilterOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableOutputStreamExt`](trait.PollableOutputStreamExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html), [`PollableOutputStreamExtManual`](prelude/trait.PollableOutputStreamExtManual.html)
<!-- trait ConverterOutputStreamExt -->
Trait containing all `ConverterOutputStream` methods.

# Implementors

[`ConverterOutputStream`](struct.ConverterOutputStream.html)
<!-- impl ConverterOutputStream::fn new -->
Creates a new converter output stream for the `base_stream`.
## `base_stream`
a `OutputStream`
## `converter`
a `Converter`

# Returns

a new `OutputStream`.
<!-- trait ConverterOutputStreamExt::fn get_converter -->
Gets the `Converter` that is used by `self`.

# Returns

the converter of the converter output stream
<!-- enum ConverterResult -->
Results returned from `Converter::convert`.
<!-- enum ConverterResult::variant Error -->
There was an error during conversion.
<!-- enum ConverterResult::variant Converted -->
Some data was consumed or produced
<!-- enum ConverterResult::variant Finished -->
The conversion is finished
<!-- enum ConverterResult::variant Flushed -->
Flushing is finished
<!-- struct Credentials -->
The `Credentials` type is a reference-counted wrapper for native
credentials. This information is typically used for identifying,
authenticating and authorizing other processes.

Some operating systems supports looking up the credentials of the
remote peer of a communication endpoint - see e.g.
`SocketExt::get_credentials`.

Some operating systems supports securely sending and receiving
credentials over a Unix Domain Socket, see
`UnixCredentialsMessage`, `UnixConnection::send_credentials` and
`UnixConnection::receive_credentials` for details.

On Linux, the native credential type is a struct ucred - see the
unix(7) man page for details. This corresponds to
`CredentialsType::LinuxUcred`.

On FreeBSD, Debian GNU/kFreeBSD, and GNU/Hurd, the native
credential type is a struct cmsgcred. This corresponds
to `CredentialsType::FreebsdCmsgcred`.

On NetBSD, the native credential type is a struct unpcbid.
This corresponds to `CredentialsType::NetbsdUnpcbid`.

On OpenBSD, the native credential type is a struct sockpeercred.
This corresponds to `CredentialsType::OpenbsdSockpeercred`.

On Solaris (including OpenSolaris and its derivatives), the native
credential type is a ucred_t. This corresponds to
`CredentialsType::SolarisUcred`.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl Credentials::fn new -->
Creates a new `Credentials` object with credentials matching the
the current process.

# Returns

A `Credentials`. Free with `gobject::ObjectExt::unref`.
<!-- impl Credentials::fn get_native -->
Gets a pointer to native credentials of type `native_type` from
`self`.

It is a programming error (which will cause an warning to be
logged) to use this method if there is no `Credentials` support for
the OS or if `native_type` isn't supported by the OS.
## `native_type`
The type of native credentials to get.

# Returns

The pointer to native credentials or `None` if the
operation there is no `Credentials` support for the OS or if
`native_type` isn't supported by the OS. Do not free the returned
data, it is owned by `self`.
<!-- impl Credentials::fn get_unix_pid -->
Tries to get the UNIX process identifier from `self`. This
method is only available on UNIX platforms.

This operation can fail if `Credentials` is not supported on the
OS or if the native credentials type does not contain information
about the UNIX process ID.

# Returns

The UNIX process ID, or -1 if `error` is set.
<!-- impl Credentials::fn get_unix_user -->
Tries to get the UNIX user identifier from `self`. This
method is only available on UNIX platforms.

This operation can fail if `Credentials` is not supported on the
OS or if the native credentials type does not contain information
about the UNIX user.

# Returns

The UNIX user identifier or -1 if `error` is set.
<!-- impl Credentials::fn is_same_user -->
Checks if `self` and `other_credentials` is the same user.

This operation can fail if `Credentials` is not supported on the
the OS.
## `other_credentials`
A `Credentials`.

# Returns

`true` if `self` and `other_credentials` has the same
user, `false` otherwise or if `error` is set.
<!-- impl Credentials::fn set_native -->
Copies the native credentials of type `native_type` from `native`
into `self`.

It is a programming error (which will cause an warning to be
logged) to use this method if there is no `Credentials` support for
the OS or if `native_type` isn't supported by the OS.
## `native_type`
The type of native credentials to set.
## `native`
A pointer to native credentials.
<!-- impl Credentials::fn set_unix_user -->
Tries to set the UNIX user identifier on `self`. This method
is only available on UNIX platforms.

This operation can fail if `Credentials` is not supported on the
OS or if the native credentials type does not contain information
about the UNIX user. It can also fail if the OS does not allow the
use of "spoofed" credentials.
## `uid`
The UNIX user identifier to set.

# Returns

`true` if `uid` was set, `false` if error is set.
<!-- impl Credentials::fn to_string -->
Creates a human-readable textual representation of `self`
that can be used in logging and debug messages. The format of the
returned string may change in future GLib release.

# Returns

A string that should be freed with `g_free`.
<!-- enum CredentialsType -->
Enumeration describing different kinds of native credential types.
<!-- enum CredentialsType::variant Invalid -->
Indicates an invalid native credential type.
<!-- enum CredentialsType::variant LinuxUcred -->
The native credentials type is a struct ucred.
<!-- enum CredentialsType::variant FreebsdCmsgcred -->
The native credentials type is a struct cmsgcred.
<!-- enum CredentialsType::variant OpenbsdSockpeercred -->
The native credentials type is a struct sockpeercred. Added in 2.30.
<!-- enum CredentialsType::variant SolarisUcred -->
The native credentials type is a ucred_t. Added in 2.40.
<!-- enum CredentialsType::variant NetbsdUnpcbid -->
The native credentials type is a struct unpcbid.
<!-- struct DataInputStream -->
Data input stream implements `InputStream` and includes functions for
reading structured data directly from a binary input stream.

# Implements

[`DataInputStreamExt`](trait.DataInputStreamExt.html), [`BufferedInputStreamExt`](trait.BufferedInputStreamExt.html), [`FilterInputStreamExt`](trait.FilterInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html)
<!-- trait DataInputStreamExt -->
Trait containing all `DataInputStream` methods.

# Implementors

[`DataInputStream`](struct.DataInputStream.html)
<!-- impl DataInputStream::fn new -->
Creates a new data input stream for the `base_stream`.
## `base_stream`
a `InputStream`.

# Returns

a new `DataInputStream`.
<!-- trait DataInputStreamExt::fn get_byte_order -->
Gets the byte order for the data input stream.

# Returns

the `self`'s current `DataStreamByteOrder`.
<!-- trait DataInputStreamExt::fn get_newline_type -->
Gets the current newline type for the `self`.

# Returns

`DataStreamNewlineType` for the given `self`.
<!-- trait DataInputStreamExt::fn read_byte -->
Reads an unsigned 8-bit/1-byte value from `self`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

an unsigned 8-bit/1-byte value read from the `self` or `0`
if an error occurred.
<!-- trait DataInputStreamExt::fn read_int16 -->
Reads a 16-bit/2-byte value from `self`.

In order to get the correct byte order for this read operation,
see `DataInputStreamExt::get_byte_order` and `DataInputStreamExt::set_byte_order`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a signed 16-bit/2-byte value read from `self` or `0` if
an error occurred.
<!-- trait DataInputStreamExt::fn read_int32 -->
Reads a signed 32-bit/4-byte value from `self`.

In order to get the correct byte order for this read operation,
see `DataInputStreamExt::get_byte_order` and `DataInputStreamExt::set_byte_order`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a signed 32-bit/4-byte value read from the `self` or `0` if
an error occurred.
<!-- trait DataInputStreamExt::fn read_int64 -->
Reads a 64-bit/8-byte value from `self`.

In order to get the correct byte order for this read operation,
see `DataInputStreamExt::get_byte_order` and `DataInputStreamExt::set_byte_order`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a signed 64-bit/8-byte value read from `self` or `0` if
an error occurred.
<!-- trait DataInputStreamExt::fn read_line -->
Reads a line from the data input stream. Note that no encoding
checks or conversion is performed; the input is not guaranteed to
be UTF-8, and may in fact have embedded NUL characters.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `length`
a `gsize` to get the length of the data read in.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns


 a NUL terminated byte array with the line that was read in
 (without the newlines). Set `length` to a `gsize` to get the length
 of the read line. On an error, it will return `None` and `error`
 will be set. If there's no content to read, it will still return
 `None`, but `error` won't be set.
<!-- trait DataInputStreamExt::fn read_line_async -->
The asynchronous version of `DataInputStream::read_line`. It is
an error to have two outstanding calls to this function.

When the operation is finished, `callback` will be called. You
can then call `DataInputStreamExt::read_line_finish` to get
the result of the operation.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied.
## `user_data`
the data to pass to callback function.
<!-- trait DataInputStreamExt::fn read_line_finish -->
Finish an asynchronous call started by
`DataInputStream::read_line_async`. Note the warning about
string encoding in `DataInputStream::read_line` applies here as
well.
## `result`
the `AsyncResult` that was provided to the callback.
## `length`
a `gsize` to get the length of the data read in.

# Returns


 a NUL-terminated byte array with the line that was read in
 (without the newlines). Set `length` to a `gsize` to get the length
 of the read line. On an error, it will return `None` and `error`
 will be set. If there's no content to read, it will still return
 `None`, but `error` won't be set.
<!-- trait DataInputStreamExt::fn read_line_finish_utf8 -->
Finish an asynchronous call started by
`DataInputStream::read_line_async`.
## `result`
the `AsyncResult` that was provided to the callback.
## `length`
a `gsize` to get the length of the data read in.

# Returns

a string with the line that
 was read in (without the newlines). Set `length` to a `gsize` to
 get the length of the read line. On an error, it will return
 `None` and `error` will be set. For UTF-8 conversion errors, the set
 error domain is `G_CONVERT_ERROR`. If there's no content to read,
 it will still return `None`, but `error` won't be set.
<!-- trait DataInputStreamExt::fn read_line_utf8 -->
Reads a UTF-8 encoded line from the data input stream.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `length`
a `gsize` to get the length of the data read in.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a NUL terminated UTF-8 string
 with the line that was read in (without the newlines). Set
 `length` to a `gsize` to get the length of the read line. On an
 error, it will return `None` and `error` will be set. For UTF-8
 conversion errors, the set error domain is `G_CONVERT_ERROR`. If
 there's no content to read, it will still return `None`, but `error`
 won't be set.
<!-- trait DataInputStreamExt::fn read_uint16 -->
Reads an unsigned 16-bit/2-byte value from `self`.

In order to get the correct byte order for this read operation,
see `DataInputStreamExt::get_byte_order` and `DataInputStreamExt::set_byte_order`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

an unsigned 16-bit/2-byte value read from the `self` or `0` if
an error occurred.
<!-- trait DataInputStreamExt::fn read_uint32 -->
Reads an unsigned 32-bit/4-byte value from `self`.

In order to get the correct byte order for this read operation,
see `DataInputStreamExt::get_byte_order` and `DataInputStreamExt::set_byte_order`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

an unsigned 32-bit/4-byte value read from the `self` or `0` if
an error occurred.
<!-- trait DataInputStreamExt::fn read_uint64 -->
Reads an unsigned 64-bit/8-byte value from `self`.

In order to get the correct byte order for this read operation,
see `DataInputStreamExt::get_byte_order`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

an unsigned 64-bit/8-byte read from `self` or `0` if
an error occurred.
<!-- trait DataInputStreamExt::fn read_until -->
Reads a string from the data input stream, up to the first
occurrence of any of the stop characters.

Note that, in contrast to `DataInputStreamExt::read_until_async`,
this function consumes the stop character that it finds.

Don't use this function in new code. Its functionality is
inconsistent with `DataInputStreamExt::read_until_async`. Both
functions will be marked as deprecated in a future release. Use
`DataInputStreamExt::read_upto` instead, but note that that function
does not consume the stop character.

# Deprecated since 2.56

Use `DataInputStreamExt::read_upto` instead, which has more
 consistent behaviour regarding the stop character.
## `stop_chars`
characters to terminate the read.
## `length`
a `gsize` to get the length of the data read in.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a string with the data that was read
 before encountering any of the stop characters. Set `length` to
 a `gsize` to get the length of the string. This function will
 return `None` on an error.
<!-- trait DataInputStreamExt::fn read_until_async -->
The asynchronous version of `DataInputStreamExt::read_until`.
It is an error to have two outstanding calls to this function.

Note that, in contrast to `DataInputStreamExt::read_until`,
this function does not consume the stop character that it finds. You
must read it for yourself.

When the operation is finished, `callback` will be called. You
can then call `DataInputStreamExt::read_until_finish` to get
the result of the operation.

Don't use this function in new code. Its functionality is
inconsistent with `DataInputStreamExt::read_until`. Both functions
will be marked as deprecated in a future release. Use
`DataInputStreamExt::read_upto_async` instead.

# Deprecated since 2.56

Use `DataInputStreamExt::read_upto_async` instead, which
 has more consistent behaviour regarding the stop character.
## `stop_chars`
characters to terminate the read.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied.
## `user_data`
the data to pass to callback function.
<!-- trait DataInputStreamExt::fn read_until_finish -->
Finish an asynchronous call started by
`DataInputStreamExt::read_until_async`.

# Deprecated since 2.56

Use `DataInputStreamExt::read_upto_finish` instead, which
 has more consistent behaviour regarding the stop character.
## `result`
the `AsyncResult` that was provided to the callback.
## `length`
a `gsize` to get the length of the data read in.

# Returns

a string with the data that was read
 before encountering any of the stop characters. Set `length` to
 a `gsize` to get the length of the string. This function will
 return `None` on an error.
<!-- trait DataInputStreamExt::fn read_upto -->
Reads a string from the data input stream, up to the first
occurrence of any of the stop characters.

In contrast to `DataInputStreamExt::read_until`, this function
does not consume the stop character. You have to use
`DataInputStreamExt::read_byte` to get it before calling
`DataInputStreamExt::read_upto` again.

Note that `stop_chars` may contain '\0' if `stop_chars_len` is
specified.

The returned string will always be nul-terminated on success.
## `stop_chars`
characters to terminate the read
## `stop_chars_len`
length of `stop_chars`. May be -1 if `stop_chars` is
 nul-terminated
## `length`
a `gsize` to get the length of the data read in
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

a string with the data that was read
 before encountering any of the stop characters. Set `length` to
 a `gsize` to get the length of the string. This function will
 return `None` on an error
<!-- trait DataInputStreamExt::fn read_upto_async -->
The asynchronous version of `DataInputStreamExt::read_upto`.
It is an error to have two outstanding calls to this function.

In contrast to `DataInputStreamExt::read_until`, this function
does not consume the stop character. You have to use
`DataInputStreamExt::read_byte` to get it before calling
`DataInputStreamExt::read_upto` again.

Note that `stop_chars` may contain '\0' if `stop_chars_len` is
specified.

When the operation is finished, `callback` will be called. You
can then call `DataInputStreamExt::read_upto_finish` to get
the result of the operation.
## `stop_chars`
characters to terminate the read
## `stop_chars_len`
length of `stop_chars`. May be -1 if `stop_chars` is
 nul-terminated
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait DataInputStreamExt::fn read_upto_finish -->
Finish an asynchronous call started by
`DataInputStreamExt::read_upto_async`.

Note that this function does not consume the stop character. You
have to use `DataInputStreamExt::read_byte` to get it before calling
`DataInputStreamExt::read_upto_async` again.

The returned string will always be nul-terminated on success.
## `result`
the `AsyncResult` that was provided to the callback
## `length`
a `gsize` to get the length of the data read in

# Returns

a string with the data that was read
 before encountering any of the stop characters. Set `length` to
 a `gsize` to get the length of the string. This function will
 return `None` on an error.
<!-- trait DataInputStreamExt::fn set_byte_order -->
This function sets the byte order for the given `self`. All subsequent
reads from the `self` will be read in the given `order`.
## `order`
a `DataStreamByteOrder` to set.
<!-- trait DataInputStreamExt::fn set_newline_type -->
Sets the newline type for the `self`.

Note that using G_DATA_STREAM_NEWLINE_TYPE_ANY is slightly unsafe. If a read
chunk ends in "CR" we must read an additional byte to know if this is "CR" or
"CR LF", and this might block if there is no more data available.
## `type_`
the type of new line return as `DataStreamNewlineType`.
<!-- struct DataOutputStream -->
Data output stream implements `OutputStream` and includes functions for
writing data directly to an output stream.

# Implements

[`DataOutputStreamExt`](trait.DataOutputStreamExt.html), [`FilterOutputStreamExt`](trait.FilterOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html)
<!-- trait DataOutputStreamExt -->
Trait containing all `DataOutputStream` methods.

# Implementors

[`DataOutputStream`](struct.DataOutputStream.html)
<!-- impl DataOutputStream::fn new -->
Creates a new data output stream for `base_stream`.
## `base_stream`
a `OutputStream`.

# Returns

`DataOutputStream`.
<!-- trait DataOutputStreamExt::fn get_byte_order -->
Gets the byte order for the stream.

# Returns

the `DataStreamByteOrder` for the `self`.
<!-- trait DataOutputStreamExt::fn put_byte -->
Puts a byte into the output stream.
## `data`
a `guchar`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_int16 -->
Puts a signed 16-bit integer into the output stream.
## `data`
a `gint16`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_int32 -->
Puts a signed 32-bit integer into the output stream.
## `data`
a `gint32`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_int64 -->
Puts a signed 64-bit integer into the stream.
## `data`
a `gint64`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_string -->
Puts a string into the output stream.
## `str`
a string.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `string` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_uint16 -->
Puts an unsigned 16-bit integer into the output stream.
## `data`
a `guint16`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_uint32 -->
Puts an unsigned 32-bit integer into the stream.
## `data`
a `guint32`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn put_uint64 -->
Puts an unsigned 64-bit integer into the stream.
## `data`
a `guint64`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if `data` was successfully added to the `self`.
<!-- trait DataOutputStreamExt::fn set_byte_order -->
Sets the byte order of the data output stream to `order`.
## `order`
a `DataStreamByteOrder`.
<!-- trait DataOutputStreamExt::fn get_property_byte_order -->
Determines the byte ordering that is used when writing
multi-byte entities (such as integers) to the stream.
<!-- trait DataOutputStreamExt::fn set_property_byte_order -->
Determines the byte ordering that is used when writing
multi-byte entities (such as integers) to the stream.
<!-- enum DataStreamByteOrder -->
`DataStreamByteOrder` is used to ensure proper endianness of streaming data sources
across various machine architectures.
<!-- enum DataStreamByteOrder::variant BigEndian -->
Selects Big Endian byte order.
<!-- enum DataStreamByteOrder::variant LittleEndian -->
Selects Little Endian byte order.
<!-- enum DataStreamByteOrder::variant HostEndian -->
Selects endianness based on host machine's architecture.
<!-- enum DataStreamNewlineType -->
`DataStreamNewlineType` is used when checking for or setting the line endings for a given file.
<!-- enum DataStreamNewlineType::variant Lf -->
Selects "LF" line endings, common on most modern UNIX platforms.
<!-- enum DataStreamNewlineType::variant Cr -->
Selects "CR" line endings.
<!-- enum DataStreamNewlineType::variant CrLf -->
Selects "CR, LF" line ending, common on Microsoft Windows.
<!-- enum DataStreamNewlineType::variant Any -->
Automatically try to handle any line ending type.
<!-- struct DesktopAppInfo -->
`DesktopAppInfo` is an implementation of `AppInfo` based on
desktop files.

Note that `<gio/gdesktopappinfo.h>` belongs to the UNIX-specific
GIO interfaces, thus you have to use the `gio-unix-2.0.pc` pkg-config
file when using it.

# Implements

[`DesktopAppInfoExt`](trait.DesktopAppInfoExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`AppInfoExt`](trait.AppInfoExt.html)
<!-- trait DesktopAppInfoExt -->
Trait containing all `DesktopAppInfo` methods.

# Implementors

[`DesktopAppInfo`](struct.DesktopAppInfo.html)
<!-- impl DesktopAppInfo::fn new -->
Creates a new `DesktopAppInfo` based on a desktop file id.

A desktop file id is the basename of the desktop file, including the
.desktop extension. GIO is looking for a desktop file with this name
in the `applications` subdirectories of the XDG
data directories (i.e. the directories specified in the `XDG_DATA_HOME`
and `XDG_DATA_DIRS` environment variables). GIO also supports the
prefix-to-subdirectory mapping that is described in the
[Menu Spec](http://standards.freedesktop.org/menu-spec/latest/)
(i.e. a desktop id of kde-foo.desktop will match
`/usr/share/applications/kde/foo.desktop`).
## `desktop_id`
the desktop file id

# Returns

a new `DesktopAppInfo`, or `None` if no desktop
 file with that id exists.
<!-- impl DesktopAppInfo::fn new_from_filename -->
Creates a new `DesktopAppInfo`.
## `filename`
the path of a desktop file, in the GLib
 filename encoding

# Returns

a new `DesktopAppInfo` or `None` on error.
<!-- impl DesktopAppInfo::fn new_from_keyfile -->
Creates a new `DesktopAppInfo`.
## `key_file`
an opened `glib::KeyFile`

# Returns

a new `DesktopAppInfo` or `None` on error.
<!-- impl DesktopAppInfo::fn get_implementations -->
Gets all applications that implement `interface`.

An application implements an interface if that interface is listed in
the Implements= line of the desktop file of the application.
## `interface`
the name of the interface

# Returns

a list of `DesktopAppInfo`
objects.
<!-- impl DesktopAppInfo::fn search -->
Searches desktop files for ones that match `search_string`.

The return value is an array of strvs. Each strv contains a list of
applications that matched `search_string` with an equal score. The
outer list is sorted by score so that the first strv contains the
best-matching applications, and so on.
The algorithm for determining matches is undefined and may change at
any time.
## `search_string`
the search string to use

# Returns

a
 list of strvs. Free each item with `g_strfreev` and free the outer
 list with `g_free`.
<!-- trait DesktopAppInfoExt::fn get_action_name -->
Gets the user-visible display name of the "additional application
action" specified by `action_name`.

This corresponds to the "Name" key within the keyfile group for the
action.
## `action_name`
the name of the action as from
 `DesktopAppInfoExt::list_actions`

# Returns

the locale-specific action name
<!-- trait DesktopAppInfoExt::fn get_boolean -->
Looks up a boolean value in the keyfile backing `self`.

The `key` is looked up in the "Desktop Entry" group.
## `key`
the key to look up

# Returns

the boolean value, or `false` if the key
 is not found
<!-- trait DesktopAppInfoExt::fn get_categories -->
Gets the categories from the desktop file.

# Returns

The unparsed Categories key from the desktop file;
 i.e. no attempt is made to split it by ';' or validate it.
<!-- trait DesktopAppInfoExt::fn get_filename -->
When `self` was created from a known filename, return it. In some
situations such as the `DesktopAppInfo` returned from
`DesktopAppInfo::new_from_keyfile`, this function will return `None`.

# Returns

The full path to the file for `self`,
 or `None` if not known.
<!-- trait DesktopAppInfoExt::fn get_generic_name -->
Gets the generic name from the destkop file.

# Returns

The value of the GenericName key
<!-- trait DesktopAppInfoExt::fn get_is_hidden -->
A desktop file is hidden if the Hidden key in it is
set to True.

# Returns

`true` if hidden, `false` otherwise.
<!-- trait DesktopAppInfoExt::fn get_keywords -->
Gets the keywords from the desktop file.

# Returns

The value of the Keywords key
<!-- trait DesktopAppInfoExt::fn get_locale_string -->
Looks up a localized string value in the keyfile backing `self`
translated to the current locale.

The `key` is looked up in the "Desktop Entry" group.

Feature: `v2_56`

## `key`
the key to look up

# Returns

a newly allocated string, or `None` if the key
 is not found
<!-- trait DesktopAppInfoExt::fn get_nodisplay -->
Gets the value of the NoDisplay key, which helps determine if the
application info should be shown in menus. See
`G_KEY_FILE_DESKTOP_KEY_NO_DISPLAY` and `AppInfo::should_show`.

# Returns

The value of the NoDisplay key
<!-- trait DesktopAppInfoExt::fn get_show_in -->
Checks if the application info should be shown in menus that list available
applications for a specific name of the desktop, based on the
`OnlyShowIn` and `NotShowIn` keys.

`desktop_env` should typically be given as `None`, in which case the
`XDG_CURRENT_DESKTOP` environment variable is consulted. If you want
to override the default mechanism then you may specify `desktop_env`,
but this is not recommended.

Note that `AppInfo::should_show` for `self` will include this check (with
`None` for `desktop_env`) as well as additional checks.
## `desktop_env`
a string specifying a desktop name

# Returns

`true` if the `self` should be shown in `desktop_env` according to the
`OnlyShowIn` and `NotShowIn` keys, `false`
otherwise.
<!-- trait DesktopAppInfoExt::fn get_startup_wm_class -->
Retrieves the StartupWMClass field from `self`. This represents the
WM_CLASS property of the main window of the application, if launched
through `self`.

# Returns

the startup WM class, or `None` if none is set
in the desktop file.
<!-- trait DesktopAppInfoExt::fn get_string -->
Looks up a string value in the keyfile backing `self`.

The `key` is looked up in the "Desktop Entry" group.
## `key`
the key to look up

# Returns

a newly allocated string, or `None` if the key
 is not found
<!-- trait DesktopAppInfoExt::fn has_key -->
Returns whether `key` exists in the "Desktop Entry" group
of the keyfile backing `self`.
## `key`
the key to look up

# Returns

`true` if the `key` exists
<!-- trait DesktopAppInfoExt::fn launch_action -->
Activates the named application action.

You may only call this function on action names that were
returned from `DesktopAppInfoExt::list_actions`.

Note that if the main entry of the desktop file indicates that the
application supports startup notification, and `launch_context` is
non-`None`, then startup notification will be used when activating the
action (and as such, invocation of the action on the receiving side
must signal the end of startup notification when it is completed).
This is the expected behaviour of applications declaring additional
actions, as per the desktop file specification.

As with `AppInfo::launch` there is no way to detect failures that
occur while using this function.
## `action_name`
the name of the action as from
 `DesktopAppInfoExt::list_actions`
## `launch_context`
a `AppLaunchContext`
<!-- trait DesktopAppInfoExt::fn launch_uris_as_manager -->
This function performs the equivalent of `AppInfo::launch_uris`,
but is intended primarily for operating system components that
launch applications. Ordinary applications should use
`AppInfo::launch_uris`.

If the application is launched via GSpawn, then `spawn_flags`, `user_setup`
and `user_setup_data` are used for the call to `g_spawn_async`.
Additionally, `pid_callback` (with `pid_callback_data`) will be called to
inform about the PID of the created process. See `g_spawn_async_with_pipes`
for information on certain parameter conditions that can enable an
optimized `posix_spawn` codepath to be used.

If application launching occurs via some other mechanism (eg: D-Bus
activation) then `spawn_flags`, `user_setup`, `user_setup_data`,
`pid_callback` and `pid_callback_data` are ignored.
## `uris`
List of URIs
## `launch_context`
a `AppLaunchContext`
## `spawn_flags`
`glib::SpawnFlags`, used for each process
## `user_setup`
a `GSpawnChildSetupFunc`, used once
 for each process.
## `user_setup_data`
User data for `user_setup`
## `pid_callback`
Callback for child processes
## `pid_callback_data`
User data for `callback`

# Returns

`true` on successful launch, `false` otherwise.
<!-- trait DesktopAppInfoExt::fn launch_uris_as_manager_with_fds -->
Equivalent to `DesktopAppInfoExt::launch_uris_as_manager` but allows
you to pass in file descriptors for the stdin, stdout and stderr streams
of the launched process.

If application launching occurs via some non-spawn mechanism (e.g. D-Bus
activation) then `stdin_fd`, `stdout_fd` and `stderr_fd` are ignored.

Feature: `v2_58`

## `uris`
List of URIs
## `launch_context`
a `AppLaunchContext`
## `spawn_flags`
`glib::SpawnFlags`, used for each process
## `user_setup`
a `GSpawnChildSetupFunc`, used once
 for each process.
## `user_setup_data`
User data for `user_setup`
## `pid_callback`
Callback for child processes
## `pid_callback_data`
User data for `callback`
## `stdin_fd`
file descriptor to use for child's stdin, or -1
## `stdout_fd`
file descriptor to use for child's stdout, or -1
## `stderr_fd`
file descriptor to use for child's stderr, or -1

# Returns

`true` on successful launch, `false` otherwise.
<!-- trait DesktopAppInfoExt::fn list_actions -->
Returns the list of "additional application actions" supported on the
desktop file, as per the desktop file specification.

As per the specification, this is the list of actions that are
explicitly listed in the "Actions" key of the [Desktop Entry] group.

# Returns

a list of strings, always non-`None`
<!-- trait DesktopAppInfoExt::fn get_property_filename -->
The origin filename of this `DesktopAppInfo`
<!-- trait DesktopAppInfoExt::fn set_property_filename -->
The origin filename of this `DesktopAppInfo`
<!-- struct Drive -->
`Drive` - this represent a piece of hardware connected to the machine.
It's generally only created for removable hardware or hardware with
removable media.

`Drive` is a container class for `Volume` objects that stem from
the same piece of media. As such, `Drive` abstracts a drive with
(or without) removable media and provides operations for querying
whether media is available, determining whether media change is
automatically detected and ejecting the media.

If the `Drive` reports that media isn't automatically detected, one
can poll for media; typically one should not do this periodically
as a poll for media operation is potententially expensive and may
spin up the drive creating noise.

`Drive` supports starting and stopping drives with authentication
support for the former. This can be used to support a diverse set
of use cases including connecting/disconnecting iSCSI devices,
powering down external disk enclosures and starting/stopping
multi-disk devices such as RAID devices. Note that the actual
semantics and side-effects of starting/stopping a `Drive` may vary
according to implementation. To choose the correct verbs in e.g. a
file manager, use `Drive::get_start_stop_type`.

For porting from GnomeVFS note that there is no equivalent of
`Drive` in that API.

# Implements

[`DriveExt`](trait.DriveExt.html)
<!-- trait DriveExt -->
Trait containing all `Drive` methods.

# Implementors

[`Drive`](struct.Drive.html)
<!-- trait DriveExt::fn can_eject -->
Checks if a drive can be ejected.

# Returns

`true` if the `self` can be ejected, `false` otherwise.
<!-- trait DriveExt::fn can_poll_for_media -->
Checks if a drive can be polled for media changes.

# Returns

`true` if the `self` can be polled for media changes,
 `false` otherwise.
<!-- trait DriveExt::fn can_start -->
Checks if a drive can be started.

# Returns

`true` if the `self` can be started, `false` otherwise.
<!-- trait DriveExt::fn can_start_degraded -->
Checks if a drive can be started degraded.

# Returns

`true` if the `self` can be started degraded, `false` otherwise.
<!-- trait DriveExt::fn can_stop -->
Checks if a drive can be stopped.

# Returns

`true` if the `self` can be stopped, `false` otherwise.
<!-- trait DriveExt::fn eject_with_operation -->
Ejects a drive. This is an asynchronous operation, and is
finished by calling `Drive::eject_with_operation_finish` with the `self`
and `AsyncResult` data returned in the `callback`.
## `flags`
flags affecting the unmount if required for eject
## `mount_operation`
a `MountOperation` or `None` to avoid
 user interaction.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data passed to `callback`.
<!-- trait DriveExt::fn eject_with_operation_finish -->
Finishes ejecting a drive. If any errors occurred during the operation,
`error` will be set to contain the errors and `false` will be returned.
## `result`
a `AsyncResult`.

# Returns

`true` if the drive was successfully ejected. `false` otherwise.
<!-- trait DriveExt::fn enumerate_identifiers -->
Gets the kinds of identifiers that `self` has.
Use `Drive::get_identifier` to obtain the identifiers
themselves.

# Returns

a `None`-terminated
 array of strings containing kinds of identifiers. Use `g_strfreev`
 to free.
<!-- trait DriveExt::fn get_icon -->
Gets the icon for `self`.

# Returns

`Icon` for the `self`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait DriveExt::fn get_identifier -->
Gets the identifier of the given kind for `self`. The only
identifier currently available is
`G_DRIVE_IDENTIFIER_KIND_UNIX_DEVICE`.
## `kind`
the kind of identifier to return

# Returns

a newly allocated string containing the
 requested identifier, or `None` if the `Drive`
 doesn't have this kind of identifier.
<!-- trait DriveExt::fn get_name -->
Gets the name of `self`.

# Returns

a string containing `self`'s name. The returned
 string should be freed when no longer needed.
<!-- trait DriveExt::fn get_sort_key -->
Gets the sort key for `self`, if any.

# Returns

Sorting key for `self` or `None` if no such key is available.
<!-- trait DriveExt::fn get_start_stop_type -->
Gets a hint about how a drive can be started/stopped.

# Returns

A value from the `DriveStartStopType` enumeration.
<!-- trait DriveExt::fn get_symbolic_icon -->
Gets the icon for `self`.

# Returns

symbolic `Icon` for the `self`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait DriveExt::fn get_volumes -->
Get a list of mountable volumes for `self`.

The returned list should be freed with `glib::List::free`, after
its elements have been unreffed with `gobject::ObjectExt::unref`.

# Returns

`glib::List` containing any `Volume` objects on the given `self`.
<!-- trait DriveExt::fn has_media -->
Checks if the `self` has media. Note that the OS may not be polling
the drive for media changes; see `Drive::is_media_check_automatic`
for more details.

# Returns

`true` if `self` has media, `false` otherwise.
<!-- trait DriveExt::fn has_volumes -->
Check if `self` has any mountable volumes.

# Returns

`true` if the `self` contains volumes, `false` otherwise.
<!-- trait DriveExt::fn is_media_check_automatic -->
Checks if `self` is capabable of automatically detecting media changes.

# Returns

`true` if the `self` is capabable of automatically detecting
 media changes, `false` otherwise.
<!-- trait DriveExt::fn is_media_removable -->
Checks if the `self` supports removable media.

# Returns

`true` if `self` supports removable media, `false` otherwise.
<!-- trait DriveExt::fn is_removable -->
Checks if the `Drive` and/or its media is considered removable by the user.
See `Drive::is_media_removable`.

Feature: `v2_50`


# Returns

`true` if `self` and/or its media is considered removable, `false` otherwise.
<!-- trait DriveExt::fn poll_for_media -->
Asynchronously polls `self` to see if media has been inserted or removed.

When the operation is finished, `callback` will be called.
You can then call `Drive::poll_for_media_finish` to obtain the
result of the operation.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data to pass to `callback`
<!-- trait DriveExt::fn poll_for_media_finish -->
Finishes an operation started with `Drive::poll_for_media` on a drive.
## `result`
a `AsyncResult`.

# Returns

`true` if the drive has been poll_for_mediaed successfully,
 `false` otherwise.
<!-- trait DriveExt::fn start -->
Asynchronously starts a drive.

When the operation is finished, `callback` will be called.
You can then call `Drive::start_finish` to obtain the
result of the operation.
## `flags`
flags affecting the start operation.
## `mount_operation`
a `MountOperation` or `None` to avoid
 user interaction.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data to pass to `callback`
<!-- trait DriveExt::fn start_finish -->
Finishes starting a drive.
## `result`
a `AsyncResult`.

# Returns

`true` if the drive has been started successfully,
 `false` otherwise.
<!-- trait DriveExt::fn stop -->
Asynchronously stops a drive.

When the operation is finished, `callback` will be called.
You can then call `Drive::stop_finish` to obtain the
result of the operation.
## `flags`
flags affecting the unmount if required for stopping.
## `mount_operation`
a `MountOperation` or `None` to avoid
 user interaction.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data to pass to `callback`
<!-- trait DriveExt::fn stop_finish -->
Finishes stopping a drive.
## `result`
a `AsyncResult`.

# Returns

`true` if the drive has been stopped successfully,
 `false` otherwise.
<!-- trait DriveExt::fn connect_changed -->
Emitted when the drive's state has changed.
<!-- trait DriveExt::fn connect_disconnected -->
This signal is emitted when the `Drive` have been
disconnected. If the recipient is holding references to the
object they should release them so the object can be
finalized.
<!-- trait DriveExt::fn connect_eject_button -->
Emitted when the physical eject button (if any) of a drive has
been pressed.
<!-- trait DriveExt::fn connect_stop_button -->
Emitted when the physical stop button (if any) of a drive has
been pressed.
<!-- enum DriveStartStopType -->
Enumeration describing how a drive can be started/stopped.
<!-- enum DriveStartStopType::variant Unknown -->
Unknown or drive doesn't support
 start/stop.
<!-- enum DriveStartStopType::variant Shutdown -->
The stop method will physically
 shut down the drive and e.g. power down the port the drive is
 attached to.
<!-- enum DriveStartStopType::variant Network -->
The start/stop methods are used
 for connecting/disconnect to the drive over the network.
<!-- enum DriveStartStopType::variant Multidisk -->
The start/stop methods will
 assemble/disassemble a virtual drive from several physical
 drives.
<!-- enum DriveStartStopType::variant Password -->
The start/stop methods will
 unlock/lock the disk (for example using the ATA `<quote>`SECURITY
 UNLOCK DEVICE`</quote>` command)
<!-- struct Emblem -->
`Emblem` is an implementation of `Icon` that supports
having an emblem, which is an icon with additional properties.
It can than be added to a `EmblemedIcon`.

Currently, only metainformation about the emblem's origin is
supported. More may be added in the future.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`IconExt`](trait.IconExt.html)
<!-- impl Emblem::fn new -->
Creates a new emblem for `icon`.
## `icon`
a GIcon containing the icon.

# Returns

a new `Emblem`.
<!-- impl Emblem::fn new_with_origin -->
Creates a new emblem for `icon`.
## `icon`
a GIcon containing the icon.
## `origin`
a GEmblemOrigin enum defining the emblem's origin

# Returns

a new `Emblem`.
<!-- impl Emblem::fn get_icon -->
Gives back the icon from `self`.

# Returns

a `Icon`. The returned object belongs to
 the emblem and should not be modified or freed.
<!-- impl Emblem::fn get_origin -->
Gets the origin of the emblem.

# Returns

the origin of the emblem
<!-- enum EmblemOrigin -->
GEmblemOrigin is used to add information about the origin of the emblem
to `Emblem`.
<!-- enum EmblemOrigin::variant Unknown -->
Emblem of unknown origin
<!-- enum EmblemOrigin::variant Device -->
Emblem adds device-specific information
<!-- enum EmblemOrigin::variant Livemetadata -->
Emblem depicts live metadata, such as "readonly"
<!-- enum EmblemOrigin::variant Tag -->
Emblem comes from a user-defined tag, e.g. set by nautilus (in the future)
<!-- struct EmblemedIcon -->
`EmblemedIcon` is an implementation of `Icon` that supports
adding an emblem to an icon. Adding multiple emblems to an
icon is ensured via `EmblemedIconExt::add_emblem`.

Note that `EmblemedIcon` allows no control over the position
of the emblems. See also `Emblem` for more information.

# Implements

[`EmblemedIconExt`](trait.EmblemedIconExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`IconExt`](trait.IconExt.html)
<!-- trait EmblemedIconExt -->
Trait containing all `EmblemedIcon` methods.

# Implementors

[`EmblemedIcon`](struct.EmblemedIcon.html)
<!-- impl EmblemedIcon::fn new -->
Creates a new emblemed icon for `icon` with the emblem `emblem`.
## `icon`
a `Icon`
## `emblem`
a `Emblem`, or `None`

# Returns

a new `Icon`
<!-- trait EmblemedIconExt::fn add_emblem -->
Adds `emblem` to the `glib::List` of `GEmblems`.
## `emblem`
a `Emblem`
<!-- trait EmblemedIconExt::fn clear_emblems -->
Removes all the emblems from `icon`.
<!-- trait EmblemedIconExt::fn get_emblems -->
Gets the list of emblems for the `icon`.

# Returns

a `glib::List` of
 `GEmblems` that is owned by `self`
<!-- trait EmblemedIconExt::fn get_icon -->
Gets the main icon for `self`.

# Returns

a `Icon` that is owned by `self`
<!-- struct File -->
`File` is a high level abstraction for manipulating files on a
virtual file system. `GFiles` are lightweight, immutable objects
that do no I/O upon creation. It is necessary to understand that
`File` objects do not represent files, merely an identifier for a
file. All file content I/O is implemented as streaming operations
(see `InputStream` and `OutputStream`).

To construct a `File`, you can use:
- `File::new_for_path` if you have a path.
- `File::new_for_uri` if you have a URI.
- `File::new_for_commandline_arg` for a command line argument.
- `File::new_tmp` to create a temporary file from a template.
- `File::parse_name` from a UTF-8 string gotten from `File::get_parse_name`.
- `File::new_build_filename` to create a file from path elements.

One way to think of a `File` is as an abstraction of a pathname. For
normal files the system pathname is what is stored internally, but as
`GFiles` are extensible it could also be something else that corresponds
to a pathname in a userspace implementation of a filesystem.

`GFiles` make up hierarchies of directories and files that correspond to
the files on a filesystem. You can move through the file system with
`File` using `File::get_parent` to get an identifier for the parent
directory, `File::get_child` to get a child within a directory,
`File::resolve_relative_path` to resolve a relative path between two
`GFiles`. There can be multiple hierarchies, so you may not end up at
the same root if you repeatedly call `File::get_parent` on two different
files.

All `GFiles` have a basename (get with `File::get_basename`). These names
are byte strings that are used to identify the file on the filesystem
(relative to its parent directory) and there is no guarantees that they
have any particular charset encoding or even make any sense at all. If
you want to use filenames in a user interface you should use the display
name that you can get by requesting the
`G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME` attribute with `File::query_info`.
This is guaranteed to be in UTF-8 and can be used in a user interface.
But always store the real basename or the `File` to use to actually
access the file, because there is no way to go from a display name to
the actual name.

Using `File` as an identifier has the same weaknesses as using a path
in that there may be multiple aliases for the same file. For instance,
hard or soft links may cause two different `GFiles` to refer to the same
file. Other possible causes for aliases are: case insensitive filesystems,
short and long names on FAT/NTFS, or bind mounts in Linux. If you want to
check if two `GFiles` point to the same file you can query for the
`G_FILE_ATTRIBUTE_ID_FILE` attribute. Note that `File` does some trivial
canonicalization of pathnames passed in, so that trivial differences in
the path string used at creation (duplicated slashes, slash at end of
path, "." or ".." path segments, etc) does not create different `GFiles`.

Many `File` operations have both synchronous and asynchronous versions
to suit your application. Asynchronous versions of synchronous functions
simply have `_async` appended to their function names. The asynchronous
I/O functions call a `GAsyncReadyCallback` which is then used to finalize
the operation, producing a GAsyncResult which is then passed to the
function's matching `_finish` operation.

It is highly recommended to use asynchronous calls when running within a
shared main loop, such as in the main thread of an application. This avoids
I/O operations blocking other sources on the main loop from being dispatched.
Synchronous I/O operations should be performed from worker threads. See the
[introduction to asynchronous programming section][async-programming] for
more.

Some `File` operations almost always take a noticeable amount of time, and
so do not have synchronous analogs. Notable cases include:
- `File::mount_mountable` to mount a mountable file.
- `File::unmount_mountable_with_operation` to unmount a mountable file.
- `File::eject_mountable_with_operation` to eject a mountable file.

## Entity Tags # {`gfile`-etag}

One notable feature of `GFiles` are entity tags, or "etags" for
short. Entity tags are somewhat like a more abstract version of the
traditional mtime, and can be used to quickly determine if the file
has been modified from the version on the file system. See the
HTTP 1.1
[specification](http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html)
for HTTP Etag headers, which are a very similar concept.

# Implements

[`FileExt`](trait.FileExt.html), [`FileExtManual`](prelude/trait.FileExtManual.html)
<!-- trait FileExt -->
Trait containing all `File` methods.

# Implementors

[`File`](struct.File.html)
<!-- impl File::fn new_build_filename -->
Constructs a `File` from a series of elements using the correct
separator for filenames.

Using this function is equivalent to calling `g_build_filename`,
followed by `File::new_for_path` on the result.

Feature: `v2_56`

## `first_element`
the first element in the path

# Returns

a new `File`
<!-- impl File::fn new_for_commandline_arg -->
Creates a `File` with the given argument from the command line.
The value of `arg` can be either a URI, an absolute path or a
relative path resolved relative to the current working directory.
This operation never fails, but the returned object might not
support any I/O operation if `arg` points to a malformed path.

Note that on Windows, this function expects its argument to be in
UTF-8 -- not the system code page. This means that you
should not use this function with string from argv as it is passed
to `main`. `g_win32_get_command_line` will return a UTF-8 version of
the commandline. `Application` also uses UTF-8 but
`ApplicationCommandLineExt::create_file_for_arg` may be more useful
for you there. It is also always possible to use this function with
`glib::OptionContext` arguments of type `glib::OptionArg::Filename`.
## `arg`
a command line string

# Returns

a new `File`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl File::fn new_for_commandline_arg_and_cwd -->
Creates a `File` with the given argument from the command line.

This function is similar to `File::new_for_commandline_arg` except
that it allows for passing the current working directory as an
argument instead of using the current working directory of the
process.

This is useful if the commandline argument was given in a context
other than the invocation of the current process.

See also `ApplicationCommandLineExt::create_file_for_arg`.
## `arg`
a command line string
## `cwd`
the current working directory of the commandline

# Returns

a new `File`
<!-- impl File::fn new_for_path -->
Constructs a `File` for a given path. This operation never
fails, but the returned object might not support any I/O
operation if `path` is malformed.
## `path`
a string containing a relative or absolute path.
 The string must be encoded in the glib filename encoding.

# Returns

a new `File` for the given `path`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl File::fn new_for_uri -->
Constructs a `File` for a given URI. This operation never
fails, but the returned object might not support any I/O
operation if `uri` is malformed or if the uri type is
not supported.
## `uri`
a UTF-8 string containing a URI

# Returns

a new `File` for the given `uri`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl File::fn new_tmp -->
Opens a file in the preferred directory for temporary files (as
returned by `g_get_tmp_dir`) and returns a `File` and
`FileIOStream` pointing to it.

`tmpl` should be a string in the GLib file name encoding
containing a sequence of six 'X' characters, and containing no
directory components. If it is `None`, a default template is used.

Unlike the other `File` constructors, this will return `None` if
a temporary file could not be created.
## `tmpl`
Template for the file
 name, as in `g_file_open_tmp`, or `None` for a default template
## `iostream`
on return, a `FileIOStream` for the created file

# Returns

a new `File`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl File::fn parse_name -->
Constructs a `File` with the given `parse_name` (i.e. something
given by `File::get_parse_name`). This operation never fails,
but the returned object might not support any I/O operation if
the `parse_name` cannot be parsed.
## `parse_name`
a file name or path to be parsed

# Returns

a new `File`.
<!-- trait FileExt::fn append_to -->
Gets an output stream for appending data to the file.
If the file doesn't already exist it is created.

By default files created are generally readable by everyone,
but if you pass `FileCreateFlags::Private` in `flags` the file
will be made readable only to the current user, to the level that
is supported on the target filesystem.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

Some file systems don't allow all file names, and may return an
`IOErrorEnum::InvalidFilename` error. If the file is a directory the
`IOErrorEnum::IsDirectory` error will be returned. Other errors are
possible too, and depend on what kind of filesystem the file is on.
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileOutputStream`, or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn append_to_async -->
Asynchronously opens `self` for appending.

For more details, see `File::append_to` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::append_to_finish` to get the result
of the operation.
## `flags`
a set of `FileCreateFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn append_to_finish -->
Finishes an asynchronous file append operation started with
`File::append_to_async`.
## `res`
`AsyncResult`

# Returns

a valid `FileOutputStream`
 or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn copy -->
Copies the file `self` to the location specified by `destination`.
Can not handle recursive copies of directories.

If the flag `FileCopyFlags::Overwrite` is specified an already
existing `destination` file is overwritten.

If the flag `FileCopyFlags::NofollowSymlinks` is specified then symlinks
will be copied as symlinks, otherwise the target of the
`self` symlink will be copied.

If the flag `FileCopyFlags::AllMetadata` is specified then all the metadata
that is possible to copy is copied, not just the default subset (which,
for instance, does not include the owner, see `FileInfo`).

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

If `progress_callback` is not `None`, then the operation can be monitored
by setting this to a `GFileProgressCallback` function.
`progress_callback_data` will be passed to this function. It is guaranteed
that this callback will be called after all data has been transferred with
the total number of bytes copied during the operation.

If the `self` file does not exist, then the `IOErrorEnum::NotFound` error
is returned, independent on the status of the `destination`.

If `FileCopyFlags::Overwrite` is not specified and the target exists, then
the error `IOErrorEnum::Exists` is returned.

If trying to overwrite a file over a directory, the `IOErrorEnum::IsDirectory`
error is returned. If trying to overwrite a directory with a directory the
`IOErrorEnum::WouldMerge` error is returned.

If the source is a directory and the target does not exist, or
`FileCopyFlags::Overwrite` is specified and the target is a file, then the
`IOErrorEnum::WouldRecurse` error is returned.

If you are interested in copying the `File` object itself (not the on-disk
file), see `File::dup`.
## `destination`
destination `File`
## `flags`
set of `FileCopyFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `progress_callback`
function to callback with
 progress information, or `None` if progress information is not needed
## `progress_callback_data`
user data to pass to `progress_callback`

# Returns

`true` on success, `false` otherwise.
<!-- trait FileExt::fn copy_async -->
Copies the file `self` to the location specified by `destination`
asynchronously. For details of the behaviour, see `File::copy`.

If `progress_callback` is not `None`, then that function that will be called
just like in `File::copy`. The callback will run in the default main context
of the thread calling `File::copy_async` — the same context as `callback` is
run in.

When the operation is finished, `callback` will be called. You can then call
`File::copy_finish` to get the result of the operation.
## `destination`
destination `File`
## `flags`
set of `FileCopyFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `progress_callback`
function to callback with progress
 information, or `None` if progress information is not needed
## `progress_callback_data`
user data to pass to `progress_callback`
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn copy_attributes -->
Copies the file attributes from `self` to `destination`.

Normally only a subset of the file attributes are copied,
those that are copies in a normal file copy operation
(which for instance does not include e.g. owner). However
if `FileCopyFlags::AllMetadata` is specified in `flags`, then
all the metadata that is possible to copy is copied. This
is useful when implementing move by copy + delete source.
## `destination`
a `File` to copy attributes to
## `flags`
a set of `FileCopyFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the attributes were copied successfully,
 `false` otherwise.
<!-- trait FileExt::fn copy_finish -->
Finishes copying the file started with `File::copy_async`.
## `res`
a `AsyncResult`

# Returns

a `true` on success, `false` on error.
<!-- trait FileExt::fn create -->
Creates a new file and returns an output stream for writing to it.
The file must not already exist.

By default files created are generally readable by everyone,
but if you pass `FileCreateFlags::Private` in `flags` the file
will be made readable only to the current user, to the level
that is supported on the target filesystem.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

If a file or directory with this name already exists the
`IOErrorEnum::Exists` error will be returned. Some file systems don't
allow all file names, and may return an `IOErrorEnum::InvalidFilename`
error, and if the name is to long `IOErrorEnum::FilenameTooLong` will
be returned. Other errors are possible too, and depend on what kind
of filesystem the file is on.
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileOutputStream` for the newly created
 file, or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn create_async -->
Asynchronously creates a new file and returns an output stream
for writing to it. The file must not already exist.

For more details, see `File::create` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::create_finish` to get the result
of the operation.
## `flags`
a set of `FileCreateFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn create_finish -->
Finishes an asynchronous file create operation started with
`File::create_async`.
## `res`
a `AsyncResult`

# Returns

a `FileOutputStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn create_readwrite -->
Creates a new file and returns a stream for reading and
writing to it. The file must not already exist.

By default files created are generally readable by everyone,
but if you pass `FileCreateFlags::Private` in `flags` the file
will be made readable only to the current user, to the level
that is supported on the target filesystem.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

If a file or directory with this name already exists, the
`IOErrorEnum::Exists` error will be returned. Some file systems don't
allow all file names, and may return an `IOErrorEnum::InvalidFilename`
error, and if the name is too long, `IOErrorEnum::FilenameTooLong`
will be returned. Other errors are possible too, and depend on what
kind of filesystem the file is on.

Note that in many non-local file cases read and write streams are
not supported, so make sure you really need to do read and write
streaming, rather than just opening for reading or writing.
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileIOStream` for the newly created
 file, or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn create_readwrite_async -->
Asynchronously creates a new file and returns a stream
for reading and writing to it. The file must not already exist.

For more details, see `File::create_readwrite` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::create_readwrite_finish` to get
the result of the operation.
## `flags`
a set of `FileCreateFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn create_readwrite_finish -->
Finishes an asynchronous file create operation started with
`File::create_readwrite_async`.
## `res`
a `AsyncResult`

# Returns

a `FileIOStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn delete -->
Deletes a file. If the `self` is a directory, it will only be
deleted if it is empty. This has the same semantics as `g_unlink`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the file was deleted. `false` otherwise.
<!-- trait FileExt::fn delete_async -->
Asynchronously delete a file. If the `self` is a directory, it will
only be deleted if it is empty. This has the same semantics as
`g_unlink`.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn delete_finish -->
Finishes deleting a file started with `File::delete_async`.
## `result`
a `AsyncResult`

# Returns

`true` if the file was deleted. `false` otherwise.
<!-- trait FileExt::fn dup -->
Duplicates a `File` handle. This operation does not duplicate
the actual file or directory represented by the `File`; see
`File::copy` if attempting to copy a file.

`File::dup` is useful when a second handle is needed to the same underlying
file, for use in a separate thread (`File` is not thread-safe). For use
within the same thread, use `gobject::ObjectExt::ref` to increment the existing object’s
reference count.

This call does no blocking I/O.

# Returns

a new `File` that is a duplicate
 of the given `File`.
<!-- trait FileExt::fn eject_mountable_with_operation -->
Starts an asynchronous eject on a mountable.
When this operation has completed, `callback` will be called with
`user_user` data, and the operation can be finalized with
`File::eject_mountable_with_operation_finish`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation`,
 or `None` to avoid user interaction
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn eject_mountable_with_operation_finish -->
Finishes an asynchronous eject operation started by
`File::eject_mountable_with_operation`.
## `result`
a `AsyncResult`

# Returns

`true` if the `self` was ejected successfully.
 `false` otherwise.
<!-- trait FileExt::fn enumerate_children -->
Gets the requested information about the files in a directory.
The result is a `FileEnumerator` object that will give out
`FileInfo` objects for all the files in the directory.

The `attributes` value is a string that specifies the file
attributes that should be gathered. It is not an error if
it's not possible to read a particular requested attribute
from a file - it just won't be set. `attributes` should
be a comma-separated list of attributes or attribute wildcards.
The wildcard "*" means all attributes, and a wildcard like
"standard::*" means all attributes in the standard namespace.
An example attribute query be "standard::*,owner::user".
The standard attributes are available as defines, like
`G_FILE_ATTRIBUTE_STANDARD_NAME`.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

If the file does not exist, the `IOErrorEnum::NotFound` error will
be returned. If the file is not a directory, the `IOErrorEnum::NotDirectory`
error will be returned. Other errors are possible too.
## `attributes`
an attribute query string
## `flags`
a set of `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

A `FileEnumerator` if successful,
 `None` on error. Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn enumerate_children_async -->
Asynchronously gets the requested information about the files
in a directory. The result is a `FileEnumerator` object that will
give out `FileInfo` objects for all the files in the directory.

For more details, see `File::enumerate_children` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called. You can
then call `File::enumerate_children_finish` to get the result of
the operation.
## `attributes`
an attribute query string
## `flags`
a set of `FileQueryInfoFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the
 request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn enumerate_children_finish -->
Finishes an async enumerate children operation.
See `File::enumerate_children_async`.
## `res`
a `AsyncResult`

# Returns

a `FileEnumerator` or `None`
 if an error occurred.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn equal -->
Checks if the two given `GFiles` refer to the same file.

Note that two `GFiles` that differ can still refer to the same
file on the filesystem due to various forms of filename
aliasing.

This call does no blocking I/O.
## `file2`
the second `File`

# Returns

`true` if `self` and `file2` are equal.
<!-- trait FileExt::fn find_enclosing_mount -->
Gets a `Mount` for the `File`.

If the `FileIface` for `self` does not have a mount (e.g.
possibly a remote share), `error` will be set to `IOErrorEnum::NotFound`
and `None` will be returned.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `Mount` where the `self` is located
 or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn find_enclosing_mount_async -->
Asynchronously gets the mount for the file.

For more details, see `File::find_enclosing_mount` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::find_enclosing_mount_finish` to
get the result of the operation.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn find_enclosing_mount_finish -->
Finishes an asynchronous find mount request.
See `File::find_enclosing_mount_async`.
## `res`
a `AsyncResult`

# Returns

`Mount` for given `self` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn get_basename -->
Gets the base name (the last component of the path) for a given `File`.

If called for the top level of a system (such as the filesystem root
or a uri like sftp://host/) it will return a single directory separator
(and on Windows, possibly a drive letter).

The base name is a byte string (not UTF-8). It has no defined encoding
or rules other than it may not contain zero bytes. If you want to use
filenames in a user interface you should use the display name that you
can get by requesting the `G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME`
attribute with `File::query_info`.

This call does no blocking I/O.

# Returns

string containing the `File`'s
 base name, or `None` if given `File` is invalid. The returned string
 should be freed with `g_free` when no longer needed.
<!-- trait FileExt::fn get_child -->
Gets a child of `self` with basename equal to `name`.

Note that the file with that specific name might not exist, but
you can still have a `File` that points to it. You can use this
for instance to create that file.

This call does no blocking I/O.
## `name`
string containing the child's basename

# Returns

a `File` to a child specified by `name`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn get_child_for_display_name -->
Gets the child of `self` for a given `display_name` (i.e. a UTF-8
version of the name). If this function fails, it returns `None`
and `error` will be set. This is very useful when constructing a
`File` for a new file and the user entered the filename in the
user interface, for instance when you select a directory and
type a filename in the file selector.

This call does no blocking I/O.
## `display_name`
string to a possible child

# Returns

a `File` to the specified child, or
 `None` if the display name couldn't be converted.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn get_parent -->
Gets the parent directory for the `self`.
If the `self` represents the root directory of the
file system, then `None` will be returned.

This call does no blocking I/O.

# Returns

a `File` structure to the
 parent of the given `File` or `None` if there is no parent. Free
 the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn get_parse_name -->
Gets the parse name of the `self`.
A parse name is a UTF-8 string that describes the
file such that one can get the `File` back using
`File::parse_name`.

This is generally used to show the `File` as a nice
full-pathname kind of string in a user interface,
like in a location entry.

For local files with names that can safely be converted
to UTF-8 the pathname is used, otherwise the IRI is used
(a form of URI that allows UTF-8 characters unescaped).

This call does no blocking I/O.

# Returns

a string containing the `File`'s parse name.
 The returned string should be freed with `g_free`
 when no longer needed.
<!-- trait FileExt::fn get_path -->
Gets the local pathname for `File`, if one exists. If non-`None`, this is
guaranteed to be an absolute, canonical path. It might contain symlinks.

This call does no blocking I/O.

# Returns

string containing the `File`'s path,
 or `None` if no such path exists. The returned string should be freed
 with `g_free` when no longer needed.
<!-- trait FileExt::fn get_relative_path -->
Gets the path for `descendant` relative to `self`.

This call does no blocking I/O.
## `descendant`
input `File`

# Returns

string with the relative path from
 `descendant` to `self`, or `None` if `descendant` doesn't have `self` as
 prefix. The returned string should be freed with `g_free` when
 no longer needed.
<!-- trait FileExt::fn get_uri -->
Gets the URI for the `self`.

This call does no blocking I/O.

# Returns

a string containing the `File`'s URI.
 The returned string should be freed with `g_free`
 when no longer needed.
<!-- trait FileExt::fn get_uri_scheme -->
Gets the URI scheme for a `File`.
RFC 3986 decodes the scheme as:

```text
URI = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
```
Common schemes include "file", "http", "ftp", etc.

This call does no blocking I/O.

# Returns

a string containing the URI scheme for the given
 `File`. The returned string should be freed with `g_free`
 when no longer needed.
<!-- trait FileExt::fn has_parent -->
Checks if `self` has a parent, and optionally, if it is `parent`.

If `parent` is `None` then this function returns `true` if `self` has any
parent at all. If `parent` is non-`None` then `true` is only returned
if `self` is an immediate child of `parent`.
## `parent`
the parent to check for, or `None`

# Returns

`true` if `self` is an immediate child of `parent` (or any parent in
 the case that `parent` is `None`).
<!-- trait FileExt::fn has_prefix -->
Checks whether `self` has the prefix specified by `prefix`.

In other words, if the names of initial elements of `self`'s
pathname match `prefix`. Only full pathname elements are matched,
so a path like /foo is not considered a prefix of /foobar, only
of /foo/bar.

A `File` is not a prefix of itself. If you want to check for
equality, use `File::equal`.

This call does no I/O, as it works purely on names. As such it can
sometimes return `false` even if `self` is inside a `prefix` (from a
filesystem point of view), because the prefix of `self` is an alias
of `prefix`.
## `prefix`
input `File`

# Returns

`true` if the `files`'s parent, grandparent, etc is `prefix`,
 `false` otherwise.
<!-- trait FileExt::fn has_uri_scheme -->
Checks to see if a `File` has a given URI scheme.

This call does no blocking I/O.
## `uri_scheme`
a string containing a URI scheme

# Returns

`true` if `File`'s backend supports the
 given URI scheme, `false` if URI scheme is `None`,
 not supported, or `File` is invalid.
<!-- trait FileExt::fn hash -->
Creates a hash value for a `File`.

This call does no blocking I/O.

# Returns

0 if `self` is not a valid `File`, otherwise an
 integer that can be used as hash value for the `File`.
 This function is intended for easily hashing a `File` to
 add to a `glib::HashTable` or similar data structure.
<!-- trait FileExt::fn is_native -->
Checks to see if a file is native to the platform.

A native file is one expressed in the platform-native filename format,
e.g. "C:\Windows" or "/usr/bin/". This does not mean the file is local,
as it might be on a locally mounted remote filesystem.

On some systems non-native files may be available using the native
filesystem via a userspace filesystem (FUSE), in these cases this call
will return `false`, but `File::get_path` will still return a native path.

This call does no blocking I/O.

# Returns

`true` if `self` is native
<!-- trait FileExt::fn load_bytes -->
Loads the contents of `self` and returns it as `glib::Bytes`.

If `self` is a resource:// based URI, the resulting bytes will reference the
embedded resource instead of a copy. Otherwise, this is equivalent to calling
`File::load_contents` and `glib::Bytes::new_take`.

For resources, `etag_out` will be set to `None`.

The data contained in the resulting `glib::Bytes` is always zero-terminated, but
this is not included in the `glib::Bytes` length. The resulting `glib::Bytes` should be
freed with `glib::Bytes::unref` when no longer in use.

Feature: `v2_56`

## `cancellable`
a `Cancellable` or `None`
## `etag_out`
a location to place the current
 entity tag for the file, or `None` if the entity tag is not needed

# Returns

a `glib::Bytes` or `None` and `error` is set
<!-- trait FileExt::fn load_bytes_async -->
Asynchronously loads the contents of `self` as `glib::Bytes`.

If `self` is a resource:// based URI, the resulting bytes will reference the
embedded resource instead of a copy. Otherwise, this is equivalent to calling
`File::load_contents_async` and `glib::Bytes::new_take`.

`callback` should call `File::load_bytes_finish` to get the result of this
asynchronous operation.

See `File::load_bytes` for more information.

Feature: `v2_56`

## `cancellable`
a `Cancellable` or `None`
## `callback`
a `GAsyncReadyCallback` to call when the
 request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn load_bytes_finish -->
Completes an asynchronous request to `File::load_bytes_async`.

For resources, `etag_out` will be set to `None`.

The data contained in the resulting `glib::Bytes` is always zero-terminated, but
this is not included in the `glib::Bytes` length. The resulting `glib::Bytes` should be
freed with `glib::Bytes::unref` when no longer in use.

See `File::load_bytes` for more information.

Feature: `v2_56`

## `result`
a `AsyncResult` provided to the callback
## `etag_out`
a location to place the current
 entity tag for the file, or `None` if the entity tag is not needed

# Returns

a `glib::Bytes` or `None` and `error` is set
<!-- trait FileExt::fn load_contents -->
Loads the content of the file into memory. The data is always
zero-terminated, but this is not included in the resultant `length`.
The returned `content` should be freed with `g_free` when no longer
needed.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `contents`
a location to place the contents of the file
## `length`
a location to place the length of the contents of the file,
 or `None` if the length is not needed
## `etag_out`
a location to place the current entity tag for the file,
 or `None` if the entity tag is not needed

# Returns

`true` if the `self`'s contents were successfully loaded.
 `false` if there were errors.
<!-- trait FileExt::fn load_contents_async -->
Starts an asynchronous load of the `self`'s contents.

For more details, see `File::load_contents` which is
the synchronous version of this call.

When the load operation has completed, `callback` will be called
with `user` data. To finish the operation, call
`File::load_contents_finish` with the `AsyncResult` returned by
the `callback`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn load_contents_finish -->
Finishes an asynchronous load of the `self`'s contents.
The contents are placed in `contents`, and `length` is set to the
size of the `contents` string. The `content` should be freed with
`g_free` when no longer needed. If `etag_out` is present, it will be
set to the new entity tag for the `self`.
## `res`
a `AsyncResult`
## `contents`
a location to place the contents of the file
## `length`
a location to place the length of the contents of the file,
 or `None` if the length is not needed
## `etag_out`
a location to place the current entity tag for the file,
 or `None` if the entity tag is not needed

# Returns

`true` if the load was successful. If `false` and `error` is
 present, it will be set appropriately.
<!-- trait FileExt::fn load_partial_contents_async -->
Reads the partial contents of a file. A `GFileReadMoreCallback` should
be used to stop reading from the file when appropriate, else this
function will behave exactly as `File::load_contents_async`. This
operation can be finished by `File::load_partial_contents_finish`.

Users of this function should be aware that `user_data` is passed to
both the `read_more_callback` and the `callback`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `read_more_callback`
a
 `GFileReadMoreCallback` to receive partial data
 and to specify whether further data should be read
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to the callback functions
<!-- trait FileExt::fn load_partial_contents_finish -->
Finishes an asynchronous partial load operation that was started
with `File::load_partial_contents_async`. The data is always
zero-terminated, but this is not included in the resultant `length`.
The returned `content` should be freed with `g_free` when no longer
needed.
## `res`
a `AsyncResult`
## `contents`
a location to place the contents of the file
## `length`
a location to place the length of the contents of the file,
 or `None` if the length is not needed
## `etag_out`
a location to place the current entity tag for the file,
 or `None` if the entity tag is not needed

# Returns

`true` if the load was successful. If `false` and `error` is
 present, it will be set appropriately.
<!-- trait FileExt::fn make_directory -->
Creates a directory. Note that this will only create a child directory
of the immediate parent directory of the path or URI given by the `File`.
To recursively create directories, see `File::make_directory_with_parents`.
This function will fail if the parent directory does not exist, setting
`error` to `IOErrorEnum::NotFound`. If the file system doesn't support
creating directories, this function will fail, setting `error` to
`IOErrorEnum::NotSupported`.

For a local `File` the newly created directory will have the default
(current) ownership and permissions of the current process.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` on successful creation, `false` otherwise.
<!-- trait FileExt::fn make_directory_async -->
Asynchronously creates a directory.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn make_directory_finish -->
Finishes an asynchronous directory creation, started with
`File::make_directory_async`.
## `result`
a `AsyncResult`

# Returns

`true` on successful directory creation, `false` otherwise.
<!-- trait FileExt::fn make_directory_with_parents -->
Creates a directory and any parent directories that may not
exist similar to 'mkdir -p'. If the file system does not support
creating directories, this function will fail, setting `error` to
`IOErrorEnum::NotSupported`. If the directory itself already exists,
this function will fail setting `error` to `IOErrorEnum::Exists`, unlike
the similar `g_mkdir_with_parents`.

For a local `File` the newly created directories will have the default
(current) ownership and permissions of the current process.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if all directories have been successfully created, `false`
otherwise.
<!-- trait FileExt::fn make_symbolic_link -->
Creates a symbolic link named `self` which contains the string
`symlink_value`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `symlink_value`
a string with the path for the target
 of the new symlink
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` on the creation of a new symlink, `false` otherwise.
<!-- trait FileExt::fn measure_disk_usage -->
Recursively measures the disk usage of `self`.

This is essentially an analog of the 'du' command, but it also
reports the number of directories and non-directory files encountered
(including things like symbolic links).

By default, errors are only reported against the toplevel file
itself. Errors found while recursing are silently ignored, unless
`G_FILE_DISK_USAGE_REPORT_ALL_ERRORS` is given in `flags`.

The returned size, `disk_usage`, is in bytes and should be formatted
with `g_format_size` in order to get something reasonable for showing
in a user interface.

`progress_callback` and `progress_data` can be given to request
periodic progress updates while scanning. See the documentation for
`GFileMeasureProgressCallback` for information about when and how the
callback will be invoked.
## `flags`
`FileMeasureFlags`
## `cancellable`
optional `Cancellable`
## `progress_callback`
a `GFileMeasureProgressCallback`
## `progress_data`
user_data for `progress_callback`
## `disk_usage`
the number of bytes of disk space used
## `num_dirs`
the number of directories encountered
## `num_files`
the number of non-directories encountered

# Returns

`true` if successful, with the out parameters set.
 `false` otherwise, with `error` set.
<!-- trait FileExt::fn measure_disk_usage_async -->
Recursively measures the disk usage of `self`.

This is the asynchronous version of `File::measure_disk_usage`. See
there for more information.
## `flags`
`FileMeasureFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable`
## `progress_callback`
a `GFileMeasureProgressCallback`
## `progress_data`
user_data for `progress_callback`
## `callback`
a `GAsyncReadyCallback` to call when complete
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn measure_disk_usage_finish -->
Collects the results from an earlier call to
`File::measure_disk_usage_async`. See `File::measure_disk_usage` for
more information.
## `result`
the `AsyncResult` passed to your `GAsyncReadyCallback`
## `disk_usage`
the number of bytes of disk space used
## `num_dirs`
the number of directories encountered
## `num_files`
the number of non-directories encountered

# Returns

`true` if successful, with the out parameters set.
 `false` otherwise, with `error` set.
<!-- trait FileExt::fn monitor -->
Obtains a file or directory monitor for the given file,
depending on the type of the file.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `flags`
a set of `FileMonitorFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileMonitor` for the given `self`,
 or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn monitor_directory -->
Obtains a directory monitor for the given file.
This may fail if directory monitoring is not supported.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

It does not make sense for `flags` to contain
`FileMonitorFlags::WatchHardLinks`, since hard links can not be made to
directories. It is not possible to monitor all the files in a
directory for changes made via hard links; if you want to do this then
you must register individual watches with `File::monitor`.
## `flags`
a set of `FileMonitorFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileMonitor` for the given `self`,
 or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn monitor_file -->
Obtains a file monitor for the given file. If no file notification
mechanism exists, then regular polling of the file is used.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

If `flags` contains `FileMonitorFlags::WatchHardLinks` then the monitor
will also attempt to report changes made to the file via another
filename (ie, a hard link). Without this flag, you can only rely on
changes made through the filename contained in `self` to be
reported. Using this flag may result in an increase in resource
usage, and may not have any effect depending on the `FileMonitor`
backend and/or filesystem type.
## `flags`
a set of `FileMonitorFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileMonitor` for the given `self`,
 or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn mount_enclosing_volume -->
Starts a `mount_operation`, mounting the volume that contains
the file `self`.

When this operation has completed, `callback` will be called with
`user_user` data, and the operation can be finalized with
`File::mount_enclosing_volume_finish`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation`
 or `None` to avoid user interaction
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn mount_enclosing_volume_finish -->
Finishes a mount operation started by `File::mount_enclosing_volume`.
## `result`
a `AsyncResult`

# Returns

`true` if successful. If an error has occurred,
 this function will return `false` and set `error`
 appropriately if present.
<!-- trait FileExt::fn mount_mountable -->
Mounts a file of type G_FILE_TYPE_MOUNTABLE.
Using `mount_operation`, you can request callbacks when, for instance,
passwords are needed during authentication.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

When the operation is finished, `callback` will be called.
You can then call `File::mount_mountable_finish` to get
the result of the operation.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation`,
 or `None` to avoid user interaction
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn mount_mountable_finish -->
Finishes a mount operation. See `File::mount_mountable` for details.

Finish an asynchronous mount operation that was started
with `File::mount_mountable`.
## `result`
a `AsyncResult`

# Returns

a `File` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn move -->
Tries to move the file or directory `self` to the location specified
by `destination`. If native move operations are supported then this is
used, otherwise a copy + delete fallback is used. The native
implementation may support moving directories (for instance on moves
inside the same filesystem), but the fallback code does not.

If the flag `FileCopyFlags::Overwrite` is specified an already
existing `destination` file is overwritten.

If the flag `FileCopyFlags::NofollowSymlinks` is specified then symlinks
will be copied as symlinks, otherwise the target of the
`self` symlink will be copied.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

If `progress_callback` is not `None`, then the operation can be monitored
by setting this to a `GFileProgressCallback` function.
`progress_callback_data` will be passed to this function. It is
guaranteed that this callback will be called after all data has been
transferred with the total number of bytes copied during the operation.

If the `self` file does not exist, then the `IOErrorEnum::NotFound`
error is returned, independent on the status of the `destination`.

If `FileCopyFlags::Overwrite` is not specified and the target exists,
then the error `IOErrorEnum::Exists` is returned.

If trying to overwrite a file over a directory, the `IOErrorEnum::IsDirectory`
error is returned. If trying to overwrite a directory with a directory the
`IOErrorEnum::WouldMerge` error is returned.

If the source is a directory and the target does not exist, or
`FileCopyFlags::Overwrite` is specified and the target is a file, then
the `IOErrorEnum::WouldRecurse` error may be returned (if the native
move operation isn't available).
## `destination`
`File` pointing to the destination location
## `flags`
set of `FileCopyFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `progress_callback`
`GFileProgressCallback`
 function for updates
## `progress_callback_data`
gpointer to user data for
 the callback function

# Returns

`true` on successful move, `false` otherwise.
<!-- trait FileExt::fn open_readwrite -->
Opens an existing file for reading and writing. The result is
a `FileIOStream` that can be used to read and write the contents
of the file.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

If the file does not exist, the `IOErrorEnum::NotFound` error will
be returned. If the file is a directory, the `IOErrorEnum::IsDirectory`
error will be returned. Other errors are possible too, and depend on
what kind of filesystem the file is on. Note that in many non-local
file cases read and write streams are not supported, so make sure you
really need to do read and write streaming, rather than just opening
for reading or writing.
## `cancellable`
a `Cancellable`

# Returns

`FileIOStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn open_readwrite_async -->
Asynchronously opens `self` for reading and writing.

For more details, see `File::open_readwrite` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::open_readwrite_finish` to get
the result of the operation.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn open_readwrite_finish -->
Finishes an asynchronous file read operation started with
`File::open_readwrite_async`.
## `res`
a `AsyncResult`

# Returns

a `FileIOStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn peek_path -->
Exactly like `File::get_path`, but caches the result via
`gobject::ObjectExt::set_qdata_full`. This is useful for example in C
applications which mix `g_file_*` APIs with native ones. It
also avoids an extra duplicated string when possible, so will be
generally more efficient.

This call does no blocking I/O.

Feature: `v2_56`


# Returns

string containing the `File`'s path,
 or `None` if no such path exists. The returned string is owned by `self`.
<!-- trait FileExt::fn poll_mountable -->
Polls a file of type `FileType::Mountable`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

When the operation is finished, `callback` will be called.
You can then call `File::mount_mountable_finish` to get
the result of the operation.
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn poll_mountable_finish -->
Finishes a poll operation. See `File::poll_mountable` for details.

Finish an asynchronous poll operation that was polled
with `File::poll_mountable`.
## `result`
a `AsyncResult`

# Returns

`true` if the operation finished successfully. `false`
otherwise.
<!-- trait FileExt::fn query_default_handler -->
Returns the `AppInfo` that is registered as the default
application to handle the file specified by `self`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

a `AppInfo` if the handle was found,
 `None` if there were errors.
 When you are done with it, release it with `gobject::ObjectExt::unref`
<!-- trait FileExt::fn query_exists -->
Utility function to check if a particular file exists. This is
implemented using `File::query_info` and as such does blocking I/O.

Note that in many cases it is [racy to first check for file existence](https://en.wikipedia.org/wiki/Time_of_check_to_time_of_use)
and then execute something based on the outcome of that, because the
file might have been created or removed in between the operations. The
general approach to handling that is to not check, but just do the
operation and handle the errors as they come.

As an example of race-free checking, take the case of reading a file,
and if it doesn't exist, creating it. There are two racy versions: read
it, and on error create it; and: check if it exists, if not create it.
These can both result in two processes creating the file (with perhaps
a partially written file as the result). The correct approach is to
always try to create the file with `File::create` which will either
atomically create the file or fail with a `IOErrorEnum::Exists` error.

However, in many cases an existence check is useful in a user interface,
for instance to make a menu item sensitive/insensitive, so that you don't
have to fool users that something is possible and then just show an error
dialog. If you do this, you should make sure to also handle the errors
that can happen due to races when you execute the operation.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the file exists (and can be detected without error),
 `false` otherwise (or if cancelled).
<!-- trait FileExt::fn query_file_type -->
Utility function to inspect the `FileType` of a file. This is
implemented using `File::query_info` and as such does blocking I/O.

The primary use case of this method is to check if a file is
a regular file, directory, or symlink.
## `flags`
a set of `FileQueryInfoFlags` passed to `File::query_info`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

The `FileType` of the file and `FileType::Unknown`
 if the file does not exist
<!-- trait FileExt::fn query_filesystem_info -->
Similar to `File::query_info`, but obtains information
about the filesystem the `self` is on, rather than the file itself.
For instance the amount of space available and the type of
the filesystem.

The `attributes` value is a string that specifies the attributes
that should be gathered. It is not an error if it's not possible
to read a particular requested attribute from a file - it just
won't be set. `attributes` should be a comma-separated list of
attributes or attribute wildcards. The wildcard "*" means all
attributes, and a wildcard like "filesystem::*" means all attributes
in the filesystem namespace. The standard namespace for filesystem
attributes is "filesystem". Common attributes of interest are
`G_FILE_ATTRIBUTE_FILESYSTEM_SIZE` (the total size of the filesystem
in bytes), `G_FILE_ATTRIBUTE_FILESYSTEM_FREE` (number of bytes available),
and `G_FILE_ATTRIBUTE_FILESYSTEM_TYPE` (type of the filesystem).

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

If the file does not exist, the `IOErrorEnum::NotFound` error will
be returned. Other errors are possible too, and depend on what
kind of filesystem the file is on.
## `attributes`
an attribute query string
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileInfo` or `None` if there was an error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn query_filesystem_info_async -->
Asynchronously gets the requested information about the filesystem
that the specified `self` is on. The result is a `FileInfo` object
that contains key-value attributes (such as type or size for the
file).

For more details, see `File::query_filesystem_info` which is the
synchronous version of this call.

When the operation is finished, `callback` will be called. You can
then call `File::query_info_finish` to get the result of the
operation.
## `attributes`
an attribute query string
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn query_filesystem_info_finish -->
Finishes an asynchronous filesystem info query.
See `File::query_filesystem_info_async`.
## `res`
a `AsyncResult`

# Returns

`FileInfo` for given `self`
 or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn query_info -->
Gets the requested information about specified `self`.
The result is a `FileInfo` object that contains key-value
attributes (such as the type or size of the file).

The `attributes` value is a string that specifies the file
attributes that should be gathered. It is not an error if
it's not possible to read a particular requested attribute
from a file - it just won't be set. `attributes` should be a
comma-separated list of attributes or attribute wildcards.
The wildcard "*" means all attributes, and a wildcard like
"standard::*" means all attributes in the standard namespace.
An example attribute query be "standard::*,owner::user".
The standard attributes are available as defines, like
`G_FILE_ATTRIBUTE_STANDARD_NAME`.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

For symlinks, normally the information about the target of the
symlink is returned, rather than information about the symlink
itself. However if you pass `FileQueryInfoFlags::NofollowSymlinks`
in `flags` the information about the symlink itself will be returned.
Also, for symlinks that point to non-existing files the information
about the symlink itself will be returned.

If the file does not exist, the `IOErrorEnum::NotFound` error will be
returned. Other errors are possible too, and depend on what kind of
filesystem the file is on.
## `attributes`
an attribute query string
## `flags`
a set of `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileInfo` for the given `self`, or `None`
 on error. Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn query_info_async -->
Asynchronously gets the requested information about specified `self`.
The result is a `FileInfo` object that contains key-value attributes
(such as type or size for the file).

For more details, see `File::query_info` which is the synchronous
version of this call.

When the operation is finished, `callback` will be called. You can
then call `File::query_info_finish` to get the result of the operation.
## `attributes`
an attribute query string
## `flags`
a set of `FileQueryInfoFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the
 request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn query_info_finish -->
Finishes an asynchronous file info query.
See `File::query_info_async`.
## `res`
a `AsyncResult`

# Returns

`FileInfo` for given `self`
 or `None` on error. Free the returned object with
 `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn query_settable_attributes -->
Obtain the list of settable attributes for the file.

Returns the type and full attribute name of all the attributes
that can be set on this file. This doesn't mean setting it will
always succeed though, you might get an access failure, or some
specific file may not support a specific attribute.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileAttributeInfoList` describing the settable attributes.
 When you are done with it, release it with
 `FileAttributeInfoList::unref`
<!-- trait FileExt::fn query_writable_namespaces -->
Obtain the list of attribute namespaces where new attributes
can be created by a user. An example of this is extended
attributes (in the "xattr" namespace).

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileAttributeInfoList` describing the writable namespaces.
 When you are done with it, release it with
 `FileAttributeInfoList::unref`
<!-- trait FileExt::fn read -->
Opens a file for reading. The result is a `FileInputStream` that
can be used to read the contents of the file.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

If the file does not exist, the `IOErrorEnum::NotFound` error will be
returned. If the file is a directory, the `IOErrorEnum::IsDirectory`
error will be returned. Other errors are possible too, and depend
on what kind of filesystem the file is on.
## `cancellable`
a `Cancellable`

# Returns

`FileInputStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn read_async -->
Asynchronously opens `self` for reading.

For more details, see `File::read` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::read_finish` to get the result
of the operation.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn read_finish -->
Finishes an asynchronous file read operation started with
`File::read_async`.
## `res`
a `AsyncResult`

# Returns

a `FileInputStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn replace -->
Returns an output stream for overwriting the file, possibly
creating a backup copy of the file first. If the file doesn't exist,
it will be created.

This will try to replace the file in the safest way possible so
that any errors during the writing will not affect an already
existing copy of the file. For instance, for local files it
may write to a temporary file and then atomically rename over
the destination when the stream is closed.

By default files created are generally readable by everyone,
but if you pass `FileCreateFlags::Private` in `flags` the file
will be made readable only to the current user, to the level that
is supported on the target filesystem.

If `cancellable` is not `None`, then the operation can be cancelled
by triggering the cancellable object from another thread. If the
operation was cancelled, the error `IOErrorEnum::Cancelled` will be
returned.

If you pass in a non-`None` `etag` value and `self` already exists, then
this value is compared to the current entity tag of the file, and if
they differ an `IOErrorEnum::WrongEtag` error is returned. This
generally means that the file has been changed since you last read
it. You can get the new etag from `FileOutputStreamExt::get_etag`
after you've finished writing and closed the `FileOutputStream`. When
you load a new file you can use `FileInputStreamExt::query_info` to
get the etag of the file.

If `make_backup` is `true`, this function will attempt to make a
backup of the current file before overwriting it. If this fails
a `IOErrorEnum::CantCreateBackup` error will be returned. If you
want to replace anyway, try again with `make_backup` set to `false`.

If the file is a directory the `IOErrorEnum::IsDirectory` error will
be returned, and if the file is some other form of non-regular file
then a `IOErrorEnum::NotRegularFile` error will be returned. Some
file systems don't allow all file names, and may return an
`IOErrorEnum::InvalidFilename` error, and if the name is to long
`IOErrorEnum::FilenameTooLong` will be returned. Other errors are
possible too, and depend on what kind of filesystem the file is on.
## `etag`
an optional [entity tag][gfile-etag]
 for the current `File`, or `None` to ignore
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileOutputStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn replace_async -->
Asynchronously overwrites the file, replacing the contents,
possibly creating a backup copy of the file first.

For more details, see `File::replace` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::replace_finish` to get the result
of the operation.
## `etag`
an [entity tag][gfile-etag] for the current `File`,
 or `None` to ignore
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn replace_contents -->
Replaces the contents of `self` with `contents` of `length` bytes.

If `etag` is specified (not `None`), any existing file must have that etag,
or the error `IOErrorEnum::WrongEtag` will be returned.

If `make_backup` is `true`, this function will attempt to make a backup
of `self`. Internally, it uses `File::replace`, so will try to replace the
file contents in the safest way possible. For example, atomic renames are
used when replacing local files’ contents.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

The returned `new_etag` can be used to verify that the file hasn't
changed the next time it is saved over.
## `contents`
a string containing the new contents for `self`
## `length`
the length of `contents` in bytes
## `etag`
the old [entity-tag][gfile-etag] for the document,
 or `None`
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `new_etag`
a location to a new [entity tag][gfile-etag]
 for the document. This should be freed with `g_free` when no longer
 needed, or `None`
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

`true` if successful. If an error has occurred, this function
 will return `false` and set `error` appropriately if present.
<!-- trait FileExtManual::fn replace_contents_async -->
Starts an asynchronous replacement of `self` with the given
`contents` of `length` bytes. `etag` will replace the document's
current entity tag.

When this operation has completed, `callback` will be called with
`user_user` data, and the operation can be finalized with
`File::replace_contents_finish`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

If `make_backup` is `true`, this function will attempt to
make a backup of `self`.

Note that no copy of `content` will be made, so it must stay valid
until `callback` is called. See `File::replace_contents_bytes_async`
for a `glib::Bytes` version that will automatically hold a reference to the
contents (without copying) for the duration of the call.
## `contents`
string of contents to replace the file with
## `length`
the length of `contents` in bytes
## `etag`
a new [entity tag][gfile-etag] for the `self`, or `None`
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn replace_contents_bytes_async -->
Same as `File::replace_contents_async` but takes a `glib::Bytes` input instead.
This function will keep a ref on `contents` until the operation is done.
Unlike `File::replace_contents_async` this allows forgetting about the
content without waiting for the callback.

When this operation has completed, `callback` will be called with
`user_user` data, and the operation can be finalized with
`File::replace_contents_finish`.
## `contents`
a `glib::Bytes`
## `etag`
a new [entity tag][gfile-etag] for the `self`, or `None`
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn replace_contents_finish -->
Finishes an asynchronous replace of the given `self`. See
`File::replace_contents_async`. Sets `new_etag` to the new entity
tag for the document, if present.
## `res`
a `AsyncResult`
## `new_etag`
a location of a new [entity tag][gfile-etag]
 for the document. This should be freed with `g_free` when it is no
 longer needed, or `None`

# Returns

`true` on success, `false` on failure.
<!-- trait FileExt::fn replace_finish -->
Finishes an asynchronous file replace operation started with
`File::replace_async`.
## `res`
a `AsyncResult`

# Returns

a `FileOutputStream`, or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn replace_readwrite -->
Returns an output stream for overwriting the file in readwrite mode,
possibly creating a backup copy of the file first. If the file doesn't
exist, it will be created.

For details about the behaviour, see `File::replace` which does the
same thing but returns an output stream only.

Note that in many non-local file cases read and write streams are not
supported, so make sure you really need to do read and write streaming,
rather than just opening for reading or writing.
## `etag`
an optional [entity tag][gfile-etag]
 for the current `File`, or `None` to ignore
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `FileIOStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn replace_readwrite_async -->
Asynchronously overwrites the file in read-write mode,
replacing the contents, possibly creating a backup copy
of the file first.

For more details, see `File::replace_readwrite` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::replace_readwrite_finish` to get
the result of the operation.
## `etag`
an [entity tag][gfile-etag] for the current `File`,
 or `None` to ignore
## `make_backup`
`true` if a backup should be created
## `flags`
a set of `FileCreateFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn replace_readwrite_finish -->
Finishes an asynchronous file replace operation started with
`File::replace_readwrite_async`.
## `res`
a `AsyncResult`

# Returns

a `FileIOStream`, or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn resolve_relative_path -->
Resolves a relative path for `self` to an absolute path.

This call does no blocking I/O.
## `relative_path`
a given relative path string

# Returns

`File` to the resolved path.
 `None` if `relative_path` is `None` or if `self` is invalid.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn set_attribute -->
Sets an attribute in the file with attribute name `attribute` to `value`.

Some attributes can be unset by setting `type_` to
`FileAttributeType::Invalid` and `value_p` to `None`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `type_`
The type of the attribute
## `value_p`
a pointer to the value (or the pointer
 itself if the type is a pointer type)
## `flags`
a set of `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the attribute was set, `false` otherwise.
<!-- trait FileExt::fn set_attribute_byte_string -->
Sets `attribute` of type `FileAttributeType::ByteString` to `value`.
If `attribute` is of a different type, this operation will fail,
returning `false`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `value`
a string containing the attribute's new value
## `flags`
a `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the `attribute` was successfully set to `value`
 in the `self`, `false` otherwise.
<!-- trait FileExt::fn set_attribute_int32 -->
Sets `attribute` of type `FileAttributeType::Int32` to `value`.
If `attribute` is of a different type, this operation will fail.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `value`
a `gint32` containing the attribute's new value
## `flags`
a `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the `attribute` was successfully set to `value`
 in the `self`, `false` otherwise.
<!-- trait FileExt::fn set_attribute_int64 -->
Sets `attribute` of type `FileAttributeType::Int64` to `value`.
If `attribute` is of a different type, this operation will fail.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `value`
a `guint64` containing the attribute's new value
## `flags`
a `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the `attribute` was successfully set, `false` otherwise.
<!-- trait FileExt::fn set_attribute_string -->
Sets `attribute` of type `FileAttributeType::String` to `value`.
If `attribute` is of a different type, this operation will fail.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `value`
a string containing the attribute's value
## `flags`
`FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the `attribute` was successfully set, `false` otherwise.
<!-- trait FileExt::fn set_attribute_uint32 -->
Sets `attribute` of type `FileAttributeType::Uint32` to `value`.
If `attribute` is of a different type, this operation will fail.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `value`
a `guint32` containing the attribute's new value
## `flags`
a `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the `attribute` was successfully set to `value`
 in the `self`, `false` otherwise.
<!-- trait FileExt::fn set_attribute_uint64 -->
Sets `attribute` of type `FileAttributeType::Uint64` to `value`.
If `attribute` is of a different type, this operation will fail.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `attribute`
a string containing the attribute's name
## `value`
a `guint64` containing the attribute's new value
## `flags`
a `FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` if the `attribute` was successfully set to `value`
 in the `self`, `false` otherwise.
<!-- trait FileExt::fn set_attributes_async -->
Asynchronously sets the attributes of `self` with `info`.

For more details, see `File::set_attributes_from_info`,
which is the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::set_attributes_finish` to get
the result of the operation.
## `info`
a `FileInfo`
## `flags`
a `FileQueryInfoFlags`
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback`
## `user_data`
a `gpointer`
<!-- trait FileExt::fn set_attributes_finish -->
Finishes setting an attribute started in `File::set_attributes_async`.
## `result`
a `AsyncResult`
## `info`
a `FileInfo`

# Returns

`true` if the attributes were set correctly, `false` otherwise.
<!-- trait FileExt::fn set_attributes_from_info -->
Tries to set all attributes in the `FileInfo` on the target
values, not stopping on the first error.

If there is any error during this operation then `error` will
be set to the first error. Error on particular fields are flagged
by setting the "status" field in the attribute value to
`FileAttributeStatus::ErrorSetting`, which means you can
also detect further errors.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `info`
a `FileInfo`
## `flags`
`FileQueryInfoFlags`
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`false` if there was any error, `true` otherwise.
<!-- trait FileExt::fn set_display_name -->
Renames `self` to the specified display name.

The display name is converted from UTF-8 to the correct encoding
for the target filesystem if possible and the `self` is renamed to this.

If you want to implement a rename operation in the user interface the
edit name (`G_FILE_ATTRIBUTE_STANDARD_EDIT_NAME`) should be used as the
initial value in the rename widget, and then the result after editing
should be passed to `File::set_display_name`.

On success the resulting converted filename is returned.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `display_name`
a string
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

a `File` specifying what `self` was renamed to,
 or `None` if there was an error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn set_display_name_async -->
Asynchronously sets the display name for a given `File`.

For more details, see `File::set_display_name` which is
the synchronous version of this call.

When the operation is finished, `callback` will be called.
You can then call `File::set_display_name_finish` to get
the result of the operation.
## `display_name`
a string
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn set_display_name_finish -->
Finishes setting a display name started with
`File::set_display_name_async`.
## `res`
a `AsyncResult`

# Returns

a `File` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait FileExt::fn start_mountable -->
Starts a file of type `FileType::Mountable`.
Using `start_operation`, you can request callbacks when, for instance,
passwords are needed during authentication.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

When the operation is finished, `callback` will be called.
You can then call `File::mount_mountable_finish` to get
the result of the operation.
## `flags`
flags affecting the operation
## `start_operation`
a `MountOperation`, or `None` to avoid user interaction
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn start_mountable_finish -->
Finishes a start operation. See `File::start_mountable` for details.

Finish an asynchronous start operation that was started
with `File::start_mountable`.
## `result`
a `AsyncResult`

# Returns

`true` if the operation finished successfully. `false`
otherwise.
<!-- trait FileExt::fn stop_mountable -->
Stops a file of type `FileType::Mountable`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

When the operation is finished, `callback` will be called.
You can then call `File::stop_mountable_finish` to get
the result of the operation.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation`,
 or `None` to avoid user interaction.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn stop_mountable_finish -->
Finishes an stop operation, see `File::stop_mountable` for details.

Finish an asynchronous stop operation that was started
with `File::stop_mountable`.
## `result`
a `AsyncResult`

# Returns

`true` if the operation finished successfully.
 `false` otherwise.
<!-- trait FileExt::fn supports_thread_contexts -->
Checks if `self` supports
[thread-default contexts][g-main-context-push-thread-default-context].
If this returns `false`, you cannot perform asynchronous operations on
`self` in a thread that has a thread-default context.

# Returns

Whether or not `self` supports thread-default contexts.
<!-- trait FileExt::fn trash -->
Sends `self` to the "Trashcan", if possible. This is similar to
deleting it, but the user can recover it before emptying the trashcan.
Not all file systems support trashing, so this call can return the
`IOErrorEnum::NotSupported` error.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional `Cancellable` object,
 `None` to ignore

# Returns

`true` on successful trash, `false` otherwise.
<!-- trait FileExt::fn trash_async -->
Asynchronously sends `self` to the Trash location, if possible.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn trash_finish -->
Finishes an asynchronous file trashing operation, started with
`File::trash_async`.
## `result`
a `AsyncResult`

# Returns

`true` on successful trash, `false` otherwise.
<!-- trait FileExt::fn unmount_mountable_with_operation -->
Unmounts a file of type `FileType::Mountable`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

When the operation is finished, `callback` will be called.
You can then call `File::unmount_mountable_finish` to get
the result of the operation.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation`,
 or `None` to avoid user interaction
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn unmount_mountable_with_operation_finish -->
Finishes an unmount operation,
see `File::unmount_mountable_with_operation` for details.

Finish an asynchronous unmount operation that was started
with `File::unmount_mountable_with_operation`.
## `result`
a `AsyncResult`

# Returns

`true` if the operation finished successfully.
 `false` otherwise.
<!-- struct FileAttributeMatcher -->
Determines if a string matches a file attribute.
<!-- impl FileAttributeMatcher::fn new -->
Creates a new file attribute matcher, which matches attributes
against a given string. `GFileAttributeMatchers` are reference
counted structures, and are created with a reference count of 1. If
the number of references falls to 0, the `FileAttributeMatcher` is
automatically destroyed.

The `attribute` string should be formatted with specific keys separated
from namespaces with a double colon. Several "namespace::key" strings may be
concatenated with a single comma (e.g. "standard::type,standard::is-hidden").
The wildcard "*" may be used to match all keys and namespaces, or
"namespace::*" will match all keys in a given namespace.

## Examples of file attribute matcher strings and results

- `"*"`: matches all attributes.
- `"standard::is-hidden"`: matches only the key is-hidden in the
 standard namespace.
- `"standard::type,unix::*"`: matches the type key in the standard
 namespace and all keys in the unix namespace.
## `attributes`
an attribute string to match.

# Returns

a `FileAttributeMatcher`
<!-- impl FileAttributeMatcher::fn enumerate_namespace -->
Checks if the matcher will match all of the keys in a given namespace.
This will always return `true` if a wildcard character is in use (e.g. if
matcher was created with "standard::*" and `ns` is "standard", or if matcher was created
using "*" and namespace is anything.)

TODO: this is awkwardly worded.
## `ns`
a string containing a file attribute namespace.

# Returns

`true` if the matcher matches all of the entries
in the given `ns`, `false` otherwise.
<!-- impl FileAttributeMatcher::fn enumerate_next -->
Gets the next matched attribute from a `FileAttributeMatcher`.

# Returns

a string containing the next attribute or `None` if
no more attribute exist.
<!-- impl FileAttributeMatcher::fn matches -->
Checks if an attribute will be matched by an attribute matcher. If
the matcher was created with the "*" matching string, this function
will always return `true`.
## `attribute`
a file attribute key.

# Returns

`true` if `attribute` matches `self`. `false` otherwise.
<!-- impl FileAttributeMatcher::fn matches_only -->
Checks if a attribute matcher only matches a given attribute. Always
returns `false` if "*" was used when creating the matcher.
## `attribute`
a file attribute key.

# Returns

`true` if the matcher only matches `attribute`. `false` otherwise.
<!-- impl FileAttributeMatcher::fn ref -->
References a file attribute matcher.

# Returns

a `FileAttributeMatcher`.
<!-- impl FileAttributeMatcher::fn subtract -->
Subtracts all attributes of `subtract` from `self` and returns
a matcher that supports those attributes.

Note that currently it is not possible to remove a single
attribute when the `self` matches the whole namespace - or remove
a namespace or attribute when the matcher matches everything. This
is a limitation of the current implementation, but may be fixed
in the future.
## `subtract`
The matcher to subtract

# Returns

A file attribute matcher matching all attributes of
 `self` that are not matched by `subtract`
<!-- impl FileAttributeMatcher::fn to_string -->
Prints what the matcher is matching against. The format will be
equal to the format passed to `FileAttributeMatcher::new`.
The output however, might not be identical, as the matcher may
decide to use a different order or omit needless parts.

# Returns

a string describing the attributes the matcher matches
 against or `None` if `self` was `None`.
<!-- impl FileAttributeMatcher::fn unref -->
Unreferences `self`. If the reference count falls below 1,
the `self` is automatically freed.
<!-- enum FileAttributeStatus -->
Used by `File::set_attributes_from_info` when setting file attributes.
<!-- enum FileAttributeStatus::variant Unset -->
Attribute value is unset (empty).
<!-- enum FileAttributeStatus::variant Set -->
Attribute value is set.
<!-- enum FileAttributeStatus::variant ErrorSetting -->
Indicates an error in setting the value.
<!-- enum FileAttributeType -->
The data types for file attributes.
<!-- enum FileAttributeType::variant Invalid -->
indicates an invalid or uninitalized type.
<!-- enum FileAttributeType::variant String -->
a null terminated UTF8 string.
<!-- enum FileAttributeType::variant ByteString -->
a zero terminated string of non-zero bytes.
<!-- enum FileAttributeType::variant Boolean -->
a boolean value.
<!-- enum FileAttributeType::variant Uint32 -->
an unsigned 4-byte/32-bit integer.
<!-- enum FileAttributeType::variant Int32 -->
a signed 4-byte/32-bit integer.
<!-- enum FileAttributeType::variant Uint64 -->
an unsigned 8-byte/64-bit integer.
<!-- enum FileAttributeType::variant Int64 -->
a signed 8-byte/64-bit integer.
<!-- enum FileAttributeType::variant Object -->
a `gobject::Object`.
<!-- enum FileAttributeType::variant Stringv -->
a `None` terminated char **. Since 2.22
<!-- struct FileIOStream -->
GFileIOStream provides io streams that both read and write to the same
file handle.

GFileIOStream implements `Seekable`, which allows the io
stream to jump to arbitrary positions in the file and to truncate
the file, provided the filesystem of the file supports these
operations.

To find the position of a file io stream, use
`Seekable::tell`.

To find out if a file io stream supports seeking, use `Seekable::can_seek`.
To position a file io stream, use `Seekable::seek`.
To find out if a file io stream supports truncating, use
`Seekable::can_truncate`. To truncate a file io
stream, use `Seekable::truncate`.

The default implementation of all the `FileIOStream` operations
and the implementation of `Seekable` just call into the same operations
on the output stream.

# Implements

[`FileIOStreamExt`](trait.FileIOStreamExt.html), [`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html)
<!-- trait FileIOStreamExt -->
Trait containing all `FileIOStream` methods.

# Implementors

[`FileIOStream`](struct.FileIOStream.html)
<!-- trait FileIOStreamExt::fn get_etag -->
Gets the entity tag for the file when it has been written.
This must be called after the stream has been written
and closed, as the etag can change while writing.

# Returns

the entity tag for the stream.
<!-- trait FileIOStreamExt::fn query_info -->
Queries a file io stream for the given `attributes`.
This function blocks while querying the stream. For the asynchronous
version of this function, see `FileIOStreamExt::query_info_async`.
While the stream is blocked, the stream will set the pending flag
internally, and any other operations on the stream will fail with
`IOErrorEnum::Pending`.

Can fail if the stream was already closed (with `error` being set to
`IOErrorEnum::Closed`), the stream has pending operations (with `error` being
set to `IOErrorEnum::Pending`), or if querying info is not supported for
the stream's interface (with `error` being set to `IOErrorEnum::NotSupported`). I
all cases of failure, `None` will be returned.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be set, and `None` will
be returned.
## `attributes`
a file attribute query string.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `FileInfo` for the `self`, or `None` on error.
<!-- trait FileIOStreamExt::fn query_info_async -->
Asynchronously queries the `self` for a `FileInfo`. When completed,
`callback` will be called with a `AsyncResult` which can be used to
finish the operation with `FileIOStreamExt::query_info_finish`.

For the synchronous version of this function, see
`FileIOStreamExt::query_info`.
## `attributes`
a file attribute query string.
## `io_priority`
the [I/O priority][gio-GIOScheduler] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileIOStreamExt::fn query_info_finish -->
Finalizes the asynchronous query started
by `FileIOStreamExt::query_info_async`.
## `result`
a `AsyncResult`.

# Returns

A `FileInfo` for the finished query.
<!-- struct FileIcon -->
`FileIcon` specifies an icon by pointing to an image file
to be used as icon.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`IconExt`](trait.IconExt.html), [`LoadableIconExt`](trait.LoadableIconExt.html)
<!-- impl FileIcon::fn new -->
Creates a new icon for a file.
## `file`
a `File`.

# Returns

a `Icon` for the given
 `file`, or `None` on error.
<!-- impl FileIcon::fn get_file -->
Gets the `File` associated with the given `self`.

# Returns

a `File`, or `None`.
<!-- impl FileIcon::fn get_property_file -->
The file containing the icon.
<!-- impl FileIcon::fn set_property_file -->
The file containing the icon.
<!-- struct FileInfo -->
Functionality for manipulating basic metadata for files. `FileInfo`
implements methods for getting information that all files should
contain, and allows for manipulation of extended attributes.

See [GFileAttribute][gio-GFileAttribute] for more information on how
GIO handles file attributes.

To obtain a `FileInfo` for a `File`, use `File::query_info` (or its
async variant). To obtain a `FileInfo` for a file input or output
stream, use `FileInputStreamExt::query_info` or
`FileOutputStreamExt::query_info` (or their async variants).

To change the actual attributes of a file, you should then set the
attribute in the `FileInfo` and call `File::set_attributes_from_info`
or `File::set_attributes_async` on a GFile.

However, not all attributes can be changed in the file. For instance,
the actual size of a file cannot be changed via `FileInfo::set_size`.
You may call `File::query_settable_attributes` and
`File::query_writable_namespaces` to discover the settable attributes
of a particular file at runtime.

`FileAttributeMatcher` allows for searching through a `FileInfo` for
attributes.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl FileInfo::fn new -->
Creates a new file info structure.

# Returns

a `FileInfo`.
<!-- impl FileInfo::fn clear_status -->
Clears the status information from `self`.
<!-- impl FileInfo::fn copy_into -->
First clears all of the [GFileAttribute][gio-GFileAttribute] of `dest_info`,
and then copies all of the file attributes from `self` to `dest_info`.
## `dest_info`
destination to copy attributes to.
<!-- impl FileInfo::fn dup -->
Duplicates a file info structure.

# Returns

a duplicate `FileInfo` of `self`.
<!-- impl FileInfo::fn get_attribute_as_string -->
Gets the value of a attribute, formated as a string.
This escapes things as needed to make the string valid
utf8.
## `attribute`
a file attribute key.

# Returns

a UTF-8 string associated with the given `attribute`.
 When you're done with the string it must be freed with `g_free`.
<!-- impl FileInfo::fn get_attribute_boolean -->
Gets the value of a boolean attribute. If the attribute does not
contain a boolean value, `false` will be returned.
## `attribute`
a file attribute key.

# Returns

the boolean value contained within the attribute.
<!-- impl FileInfo::fn get_attribute_byte_string -->
Gets the value of a byte string attribute. If the attribute does
not contain a byte string, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

the contents of the `attribute` value as a byte string, or
`None` otherwise.
<!-- impl FileInfo::fn get_attribute_data -->
Gets the attribute type, value and status for an attribute key.
## `attribute`
a file attribute key
## `type_`
return location for the attribute type, or `None`
## `value_pp`
return location for the
 attribute value, or `None`; the attribute value will not be `None`
## `status`
return location for the attribute status, or `None`

# Returns

`true` if `self` has an attribute named `attribute`,
 `false` otherwise.
<!-- impl FileInfo::fn get_attribute_int32 -->
Gets a signed 32-bit integer contained within the attribute. If the
attribute does not contain a signed 32-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

a signed 32-bit integer from the attribute.
<!-- impl FileInfo::fn get_attribute_int64 -->
Gets a signed 64-bit integer contained within the attribute. If the
attribute does not contain an signed 64-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

a signed 64-bit integer from the attribute.
<!-- impl FileInfo::fn get_attribute_object -->
Gets the value of a `gobject::Object` attribute. If the attribute does
not contain a `gobject::Object`, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

a `gobject::Object` associated with the given `attribute`, or
`None` otherwise.
<!-- impl FileInfo::fn get_attribute_status -->
Gets the attribute status for an attribute key.
## `attribute`
a file attribute key

# Returns

a `FileAttributeStatus` for the given `attribute`, or
 `FileAttributeStatus::Unset` if the key is invalid.
<!-- impl FileInfo::fn get_attribute_string -->
Gets the value of a string attribute. If the attribute does
not contain a string, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

the contents of the `attribute` value as a UTF-8 string, or
`None` otherwise.
<!-- impl FileInfo::fn get_attribute_stringv -->
Gets the value of a stringv attribute. If the attribute does
not contain a stringv, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

the contents of the `attribute` value as a stringv, or
`None` otherwise. Do not free. These returned strings are UTF-8.
<!-- impl FileInfo::fn get_attribute_type -->
Gets the attribute type for an attribute key.
## `attribute`
a file attribute key.

# Returns

a `FileAttributeType` for the given `attribute`, or
`FileAttributeType::Invalid` if the key is not set.
<!-- impl FileInfo::fn get_attribute_uint32 -->
Gets an unsigned 32-bit integer contained within the attribute. If the
attribute does not contain an unsigned 32-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

an unsigned 32-bit integer from the attribute.
<!-- impl FileInfo::fn get_attribute_uint64 -->
Gets a unsigned 64-bit integer contained within the attribute. If the
attribute does not contain an unsigned 64-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

a unsigned 64-bit integer from the attribute.
<!-- impl FileInfo::fn get_content_type -->
Gets the file's content type.

# Returns

a string containing the file's content type.
<!-- impl FileInfo::fn get_deletion_date -->
Returns the `glib::DateTime` representing the deletion date of the file, as
available in G_FILE_ATTRIBUTE_TRASH_DELETION_DATE. If the
G_FILE_ATTRIBUTE_TRASH_DELETION_DATE attribute is unset, `None` is returned.

# Returns

a `glib::DateTime`, or `None`.
<!-- impl FileInfo::fn get_display_name -->
Gets a display name for a file.

# Returns

a string containing the display name.
<!-- impl FileInfo::fn get_edit_name -->
Gets the edit name for a file.

# Returns

a string containing the edit name.
<!-- impl FileInfo::fn get_etag -->
Gets the [entity tag][gfile-etag] for a given
`FileInfo`. See `G_FILE_ATTRIBUTE_ETAG_VALUE`.

# Returns

a string containing the value of the "etag:value" attribute.
<!-- impl FileInfo::fn get_file_type -->
Gets a file's type (whether it is a regular file, symlink, etc).
This is different from the file's content type, see `FileInfo::get_content_type`.

# Returns

a `FileType` for the given file.
<!-- impl FileInfo::fn get_icon -->
Gets the icon for a file.

# Returns

`Icon` for the given `self`.
<!-- impl FileInfo::fn get_is_backup -->
Checks if a file is a backup file.

# Returns

`true` if file is a backup file, `false` otherwise.
<!-- impl FileInfo::fn get_is_hidden -->
Checks if a file is hidden.

# Returns

`true` if the file is a hidden file, `false` otherwise.
<!-- impl FileInfo::fn get_is_symlink -->
Checks if a file is a symlink.

# Returns

`true` if the given `self` is a symlink.
<!-- impl FileInfo::fn get_modification_time -->
Gets the modification time of the current `self` and sets it
in `result`.
## `result`
a `glib::TimeVal`.
<!-- impl FileInfo::fn get_name -->
Gets the name for a file.

# Returns

a string containing the file name.
<!-- impl FileInfo::fn get_size -->
Gets the file's size.

# Returns

a `goffset` containing the file's size.
<!-- impl FileInfo::fn get_sort_order -->
Gets the value of the sort_order attribute from the `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER`.

# Returns

a `gint32` containing the value of the "standard::sort_order" attribute.
<!-- impl FileInfo::fn get_symbolic_icon -->
Gets the symbolic icon for a file.

# Returns

`Icon` for the given `self`.
<!-- impl FileInfo::fn get_symlink_target -->
Gets the symlink target for a given `FileInfo`.

# Returns

a string containing the symlink target.
<!-- impl FileInfo::fn has_attribute -->
Checks if a file info structure has an attribute named `attribute`.
## `attribute`
a file attribute key.

# Returns

`true` if `Ginfo` has an attribute named `attribute`,
 `false` otherwise.
<!-- impl FileInfo::fn has_namespace -->
Checks if a file info structure has an attribute in the
specified `name_space`.
## `name_space`
a file attribute namespace.

# Returns

`true` if `Ginfo` has an attribute in `name_space`,
 `false` otherwise.
<!-- impl FileInfo::fn list_attributes -->
Lists the file info structure's attributes.
## `name_space`
a file attribute key's namespace, or `None` to list
 all attributes.

# Returns

a
null-terminated array of strings of all of the possible attribute
types for the given `name_space`, or `None` on error.
<!-- impl FileInfo::fn remove_attribute -->
Removes all cases of `attribute` from `self` if it exists.
## `attribute`
a file attribute key.
<!-- impl FileInfo::fn set_attribute -->
Sets the `attribute` to contain the given value, if possible. To unset the
attribute, use `FileAttributeType::Invalid` for `type_`.
## `attribute`
a file attribute key.
## `type_`
a `FileAttributeType`
## `value_p`
pointer to the value
<!-- impl FileInfo::fn set_attribute_boolean -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a boolean value.
<!-- impl FileInfo::fn set_attribute_byte_string -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a byte string.
<!-- impl FileInfo::fn set_attribute_int32 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a signed 32-bit integer
<!-- impl FileInfo::fn set_attribute_int64 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
attribute name to set.
## `attr_value`
int64 value to set attribute to.
<!-- impl FileInfo::fn set_attribute_mask -->
Sets `mask` on `self` to match specific attribute types.
## `mask`
a `FileAttributeMatcher`.
<!-- impl FileInfo::fn set_attribute_object -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a `gobject::Object`.
<!-- impl FileInfo::fn set_attribute_status -->
Sets the attribute status for an attribute key. This is only
needed by external code that implement `File::set_attributes_from_info`
or similar functions.

The attribute must exist in `self` for this to work. Otherwise `false`
is returned and `self` is unchanged.
## `attribute`
a file attribute key
## `status`
a `FileAttributeStatus`

# Returns

`true` if the status was changed, `false` if the key was not set.
<!-- impl FileInfo::fn set_attribute_string -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a UTF-8 string.
<!-- impl FileInfo::fn set_attribute_stringv -->
Sets the `attribute` to contain the given `attr_value`,
if possible.

Sinze: 2.22
## `attribute`
a file attribute key
## `attr_value`
a `None` terminated array of UTF-8 strings.
<!-- impl FileInfo::fn set_attribute_uint32 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
an unsigned 32-bit integer.
<!-- impl FileInfo::fn set_attribute_uint64 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
an unsigned 64-bit integer.
<!-- impl FileInfo::fn set_content_type -->
Sets the content type attribute for a given `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE`.
## `content_type`
a content type. See [GContentType][gio-GContentType]
<!-- impl FileInfo::fn set_display_name -->
Sets the display name for the current `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME`.
## `display_name`
a string containing a display name.
<!-- impl FileInfo::fn set_edit_name -->
Sets the edit name for the current file.
See `G_FILE_ATTRIBUTE_STANDARD_EDIT_NAME`.
## `edit_name`
a string containing an edit name.
<!-- impl FileInfo::fn set_file_type -->
Sets the file type in a `FileInfo` to `type_`.
See `G_FILE_ATTRIBUTE_STANDARD_TYPE`.
## `type_`
a `FileType`.
<!-- impl FileInfo::fn set_icon -->
Sets the icon for a given `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_ICON`.
## `icon`
a `Icon`.
<!-- impl FileInfo::fn set_is_hidden -->
Sets the "is_hidden" attribute in a `FileInfo` according to `is_hidden`.
See `G_FILE_ATTRIBUTE_STANDARD_IS_HIDDEN`.
## `is_hidden`
a `gboolean`.
<!-- impl FileInfo::fn set_is_symlink -->
Sets the "is_symlink" attribute in a `FileInfo` according to `is_symlink`.
See `G_FILE_ATTRIBUTE_STANDARD_IS_SYMLINK`.
## `is_symlink`
a `gboolean`.
<!-- impl FileInfo::fn set_modification_time -->
Sets the `G_FILE_ATTRIBUTE_TIME_MODIFIED` attribute in the file
info to the given time value.
## `mtime`
a `glib::TimeVal`.
<!-- impl FileInfo::fn set_name -->
Sets the name attribute for the current `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_NAME`.
## `name`
a string containing a name.
<!-- impl FileInfo::fn set_size -->
Sets the `G_FILE_ATTRIBUTE_STANDARD_SIZE` attribute in the file info
to the given size.
## `size`
a `goffset` containing the file's size.
<!-- impl FileInfo::fn set_sort_order -->
Sets the sort order attribute in the file info structure. See
`G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER`.
## `sort_order`
a sort order integer.
<!-- impl FileInfo::fn set_symbolic_icon -->
Sets the symbolic icon for a given `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON`.
## `icon`
a `Icon`.
<!-- impl FileInfo::fn set_symlink_target -->
Sets the `G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET` attribute in the file info
to the given symlink target.
## `symlink_target`
a static string containing a path to a symlink target.
<!-- impl FileInfo::fn unset_attribute_mask -->
Unsets a mask set by `FileInfo::set_attribute_mask`, if one
is set.
<!-- struct FileInputStream -->
GFileInputStream provides input streams that take their
content from a file.

GFileInputStream implements `Seekable`, which allows the input
stream to jump to arbitrary positions in the file, provided the
filesystem of the file allows it. To find the position of a file
input stream, use `Seekable::tell`. To find out if a file input
stream supports seeking, use `Seekable::can_seek`.
To position a file input stream, use `Seekable::seek`.

# Implements

[`FileInputStreamExt`](trait.FileInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html)
<!-- trait FileInputStreamExt -->
Trait containing all `FileInputStream` methods.

# Implementors

[`FileInputStream`](struct.FileInputStream.html)
<!-- trait FileInputStreamExt::fn query_info -->
Queries a file input stream the given `attributes`. This function blocks
while querying the stream. For the asynchronous (non-blocking) version
of this function, see `FileInputStreamExt::query_info_async`. While the
stream is blocked, the stream will set the pending flag internally, and
any other operations on the stream will fail with `IOErrorEnum::Pending`.
## `attributes`
a file attribute query string.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `FileInfo`, or `None` on error.
<!-- trait FileInputStreamExt::fn query_info_async -->
Queries the stream information asynchronously.
When the operation is finished `callback` will be called.
You can then call `FileInputStreamExt::query_info_finish`
to get the result of the operation.

For the synchronous version of this function,
see `FileInputStreamExt::query_info`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be set
## `attributes`
a file attribute query string.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileInputStreamExt::fn query_info_finish -->
Finishes an asynchronous info query operation.
## `result`
a `AsyncResult`.

# Returns

`FileInfo`.
<!-- struct FileMonitor -->
Monitors a file or directory for changes.

To obtain a `FileMonitor` for a file or directory, use
`File::monitor`, `File::monitor_file`, or
`File::monitor_directory`.

To get informed about changes to the file or directory you are
monitoring, connect to the `FileMonitor::changed` signal. The
signal will be emitted in the
[thread-default main context][g-main-context-push-thread-default]
of the thread that the monitor was created in
(though if the global default main context is blocked, this may
cause notifications to be blocked even if the thread-default
context is still running).

# Implements

[`FileMonitorExt`](trait.FileMonitorExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait FileMonitorExt -->
Trait containing all `FileMonitor` methods.

# Implementors

[`FileMonitor`](struct.FileMonitor.html)
<!-- trait FileMonitorExt::fn cancel -->
Cancels a file monitor.

# Returns

always `true`
<!-- trait FileMonitorExt::fn emit_event -->
Emits the `FileMonitor::changed` signal if a change
has taken place. Should be called from file monitor
implementations only.

Implementations are responsible to call this method from the
[thread-default main context][g-main-context-push-thread-default] of the
thread that the monitor was created in.
## `child`
a `File`.
## `other_file`
a `File`.
## `event_type`
a set of `FileMonitorEvent` flags.
<!-- trait FileMonitorExt::fn is_cancelled -->
Returns whether the monitor is canceled.

# Returns

`true` if monitor is canceled. `false` otherwise.
<!-- trait FileMonitorExt::fn set_rate_limit -->
Sets the rate limit to which the `self` will report
consecutive change events to the same file.
## `limit_msecs`
a non-negative integer with the limit in milliseconds
 to poll for changes
<!-- trait FileMonitorExt::fn connect_changed -->
Emitted when `file` has been changed.

If using `FileMonitorFlags::WatchMoves` on a directory monitor, and
the information is available (and if supported by the backend),
`event_type` may be `FileMonitorEvent::Renamed`,
`FileMonitorEvent::MovedIn` or `FileMonitorEvent::MovedOut`.

In all cases `file` will be a child of the monitored directory. For
renames, `file` will be the old name and `other_file` is the new
name. For "moved in" events, `file` is the name of the file that
appeared and `other_file` is the old name that it was moved from (in
another directory). For "moved out" events, `file` is the name of
the file that used to be in this directory and `other_file` is the
name of the file at its new location.

It makes sense to treat `FileMonitorEvent::MovedIn` as
equivalent to `FileMonitorEvent::Created` and
`FileMonitorEvent::MovedOut` as equivalent to
`FileMonitorEvent::Deleted`, with extra information.
`FileMonitorEvent::Renamed` is equivalent to a delete/create
pair. This is exactly how the events will be reported in the case
that the `FileMonitorFlags::WatchMoves` flag is not in use.

If using the deprecated flag `FileMonitorFlags::SendMoved` flag and `event_type` is
`FileMonitorEvent::Moved`, `file` will be set to a `File` containing the
old path, and `other_file` will be set to a `File` containing the new path.

In all the other cases, `other_file` will be set to `None`.
## `file`
a `File`.
## `other_file`
a `File` or `None`.
## `event_type`
a `FileMonitorEvent`.
<!-- enum FileMonitorEvent -->
Specifies what type of event a monitor event is.
<!-- enum FileMonitorEvent::variant Changed -->
a file changed.
<!-- enum FileMonitorEvent::variant ChangesDoneHint -->
a hint that this was probably the last change in a set of changes.
<!-- enum FileMonitorEvent::variant Deleted -->
a file was deleted.
<!-- enum FileMonitorEvent::variant Created -->
a file was created.
<!-- enum FileMonitorEvent::variant AttributeChanged -->
a file attribute was changed.
<!-- enum FileMonitorEvent::variant PreUnmount -->
the file location will soon be unmounted.
<!-- enum FileMonitorEvent::variant Unmounted -->
the file location was unmounted.
<!-- enum FileMonitorEvent::variant Moved -->
the file was moved -- only sent if the
 (deprecated) `FileMonitorFlags::SendMoved` flag is set
<!-- enum FileMonitorEvent::variant Renamed -->
the file was renamed within the
 current directory -- only sent if the `FileMonitorFlags::WatchMoves`
 flag is set. Since: 2.46.
<!-- enum FileMonitorEvent::variant MovedIn -->
the file was moved into the
 monitored directory from another location -- only sent if the
 `FileMonitorFlags::WatchMoves` flag is set. Since: 2.46.
<!-- enum FileMonitorEvent::variant MovedOut -->
the file was moved out of the
 monitored directory to another location -- only sent if the
 `FileMonitorFlags::WatchMoves` flag is set. Since: 2.46
<!-- struct FileOutputStream -->
GFileOutputStream provides output streams that write their
content to a file.

GFileOutputStream implements `Seekable`, which allows the output
stream to jump to arbitrary positions in the file and to truncate
the file, provided the filesystem of the file supports these
operations.

To find the position of a file output stream, use `Seekable::tell`.
To find out if a file output stream supports seeking, use
`Seekable::can_seek`.To position a file output stream, use
`Seekable::seek`. To find out if a file output stream supports
truncating, use `Seekable::can_truncate`. To truncate a file output
stream, use `Seekable::truncate`.

# Implements

[`FileOutputStreamExt`](trait.FileOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SeekableExt`](trait.SeekableExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html)
<!-- trait FileOutputStreamExt -->
Trait containing all `FileOutputStream` methods.

# Implementors

[`FileOutputStream`](struct.FileOutputStream.html)
<!-- trait FileOutputStreamExt::fn get_etag -->
Gets the entity tag for the file when it has been written.
This must be called after the stream has been written
and closed, as the etag can change while writing.

# Returns

the entity tag for the stream.
<!-- trait FileOutputStreamExt::fn query_info -->
Queries a file output stream for the given `attributes`.
This function blocks while querying the stream. For the asynchronous
version of this function, see `FileOutputStreamExt::query_info_async`.
While the stream is blocked, the stream will set the pending flag
internally, and any other operations on the stream will fail with
`IOErrorEnum::Pending`.

Can fail if the stream was already closed (with `error` being set to
`IOErrorEnum::Closed`), the stream has pending operations (with `error` being
set to `IOErrorEnum::Pending`), or if querying info is not supported for
the stream's interface (with `error` being set to `IOErrorEnum::NotSupported`). In
all cases of failure, `None` will be returned.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be set, and `None` will
be returned.
## `attributes`
a file attribute query string.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `FileInfo` for the `self`, or `None` on error.
<!-- trait FileOutputStreamExt::fn query_info_async -->
Asynchronously queries the `self` for a `FileInfo`. When completed,
`callback` will be called with a `AsyncResult` which can be used to
finish the operation with `FileOutputStreamExt::query_info_finish`.

For the synchronous version of this function, see
`FileOutputStreamExt::query_info`.
## `attributes`
a file attribute query string.
## `io_priority`
the [I/O priority][gio-GIOScheduler] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait FileOutputStreamExt::fn query_info_finish -->
Finalizes the asynchronous query started
by `FileOutputStreamExt::query_info_async`.
## `result`
a `AsyncResult`.

# Returns

A `FileInfo` for the finished query.
<!-- enum FileType -->
Indicates the file's on-disk type.
<!-- enum FileType::variant Unknown -->
File's type is unknown.
<!-- enum FileType::variant Regular -->
File handle represents a regular file.
<!-- enum FileType::variant Directory -->
File handle represents a directory.
<!-- enum FileType::variant SymbolicLink -->
File handle represents a symbolic link
 (Unix systems).
<!-- enum FileType::variant Special -->
File is a "special" file, such as a socket, fifo,
 block device, or character device.
<!-- enum FileType::variant Shortcut -->
File is a shortcut (Windows systems).
<!-- enum FileType::variant Mountable -->
File is a mountable location.
<!-- struct FilenameCompleter -->
Completes partial file and directory names given a partial string by
looking in the file system for clues. Can return a list of possible
completion strings for widget implementations.

# Implements

[`FilenameCompleterExt`](trait.FilenameCompleterExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait FilenameCompleterExt -->
Trait containing all `FilenameCompleter` methods.

# Implementors

[`FilenameCompleter`](struct.FilenameCompleter.html)
<!-- impl FilenameCompleter::fn new -->
Creates a new filename completer.

# Returns

a `FilenameCompleter`.
<!-- trait FilenameCompleterExt::fn get_completion_suffix -->
Obtains a completion for `initial_text` from `self`.
## `initial_text`
text to be completed.

# Returns

a completed string, or `None` if no completion exists.
 This string is not owned by GIO, so remember to `g_free` it
 when finished.
<!-- trait FilenameCompleterExt::fn get_completions -->
Gets an array of completion strings for a given initial text.
## `initial_text`
text to be completed.

# Returns

array of strings with possible completions for `initial_text`.
This array must be freed by `g_strfreev` when finished.
<!-- trait FilenameCompleterExt::fn set_dirs_only -->
If `dirs_only` is `true`, `self` will only
complete directory names, and not file names.
## `dirs_only`
a `gboolean`.
<!-- trait FilenameCompleterExt::fn connect_got_completion_data -->
Emitted when the file name completion information comes available.
<!-- struct FilterInputStream -->
Base class for input stream implementations that perform some
kind of filtering operation on a base stream. Typical examples
of filtering operations are character set conversion, compression
and byte order flipping.

# Implements

[`FilterInputStreamExt`](trait.FilterInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html)
<!-- trait FilterInputStreamExt -->
Trait containing all `FilterInputStream` methods.

# Implementors

[`BufferedInputStream`](struct.BufferedInputStream.html), [`ConverterInputStream`](struct.ConverterInputStream.html), [`FilterInputStream`](struct.FilterInputStream.html)
<!-- trait FilterInputStreamExt::fn get_base_stream -->
Gets the base stream for the filter stream.

# Returns

a `InputStream`.
<!-- trait FilterInputStreamExt::fn get_close_base_stream -->
Returns whether the base stream will be closed when `self` is
closed.

# Returns

`true` if the base stream will be closed.
<!-- trait FilterInputStreamExt::fn set_close_base_stream -->
Sets whether the base stream will be closed when `self` is closed.
## `close_base`
`true` to close the base stream.
<!-- struct FilterOutputStream -->
Base class for output stream implementations that perform some
kind of filtering operation on a base stream. Typical examples
of filtering operations are character set conversion, compression
and byte order flipping.

# Implements

[`FilterOutputStreamExt`](trait.FilterOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html)
<!-- trait FilterOutputStreamExt -->
Trait containing all `FilterOutputStream` methods.

# Implementors

[`BufferedOutputStream`](struct.BufferedOutputStream.html), [`ConverterOutputStream`](struct.ConverterOutputStream.html), [`DataOutputStream`](struct.DataOutputStream.html), [`FilterOutputStream`](struct.FilterOutputStream.html)
<!-- trait FilterOutputStreamExt::fn get_base_stream -->
Gets the base stream for the filter stream.

# Returns

a `OutputStream`.
<!-- trait FilterOutputStreamExt::fn get_close_base_stream -->
Returns whether the base stream will be closed when `self` is
closed.

# Returns

`true` if the base stream will be closed.
<!-- trait FilterOutputStreamExt::fn set_close_base_stream -->
Sets whether the base stream will be closed when `self` is closed.
## `close_base`
`true` to close the base stream.
<!-- enum IOErrorEnum -->
Error codes returned by GIO functions.

Note that this domain may be extended in future GLib releases. In
general, new error codes either only apply to new APIs, or else
replace `IOErrorEnum::Failed` in cases that were not explicitly
distinguished before. You should therefore avoid writing code like

```C
if (g_error_matches (error, G_IO_ERROR, G_IO_ERROR_FAILED))
  {
    // Assume that this is EPRINTERONFIRE
    ...
  }
```
but should instead treat all unrecognized error codes the same as
`IOErrorEnum::Failed`.
<!-- enum IOErrorEnum::variant Failed -->
Generic error condition for when an operation fails
 and no more specific `IOErrorEnum` value is defined.
<!-- enum IOErrorEnum::variant NotFound -->
File not found.
<!-- enum IOErrorEnum::variant Exists -->
File already exists.
<!-- enum IOErrorEnum::variant IsDirectory -->
File is a directory.
<!-- enum IOErrorEnum::variant NotDirectory -->
File is not a directory.
<!-- enum IOErrorEnum::variant NotEmpty -->
File is a directory that isn't empty.
<!-- enum IOErrorEnum::variant NotRegularFile -->
File is not a regular file.
<!-- enum IOErrorEnum::variant NotSymbolicLink -->
File is not a symbolic link.
<!-- enum IOErrorEnum::variant NotMountableFile -->
File cannot be mounted.
<!-- enum IOErrorEnum::variant FilenameTooLong -->
Filename is too many characters.
<!-- enum IOErrorEnum::variant InvalidFilename -->
Filename is invalid or contains invalid characters.
<!-- enum IOErrorEnum::variant TooManyLinks -->
File contains too many symbolic links.
<!-- enum IOErrorEnum::variant NoSpace -->
No space left on drive.
<!-- enum IOErrorEnum::variant InvalidArgument -->
Invalid argument.
<!-- enum IOErrorEnum::variant PermissionDenied -->
Permission denied.
<!-- enum IOErrorEnum::variant NotSupported -->
Operation (or one of its parameters) not supported
<!-- enum IOErrorEnum::variant NotMounted -->
File isn't mounted.
<!-- enum IOErrorEnum::variant AlreadyMounted -->
File is already mounted.
<!-- enum IOErrorEnum::variant Closed -->
File was closed.
<!-- enum IOErrorEnum::variant Cancelled -->
Operation was cancelled. See `Cancellable`.
<!-- enum IOErrorEnum::variant Pending -->
Operations are still pending.
<!-- enum IOErrorEnum::variant ReadOnly -->
File is read only.
<!-- enum IOErrorEnum::variant CantCreateBackup -->
Backup couldn't be created.
<!-- enum IOErrorEnum::variant WrongEtag -->
File's Entity Tag was incorrect.
<!-- enum IOErrorEnum::variant TimedOut -->
Operation timed out.
<!-- enum IOErrorEnum::variant WouldRecurse -->
Operation would be recursive.
<!-- enum IOErrorEnum::variant Busy -->
File is busy.
<!-- enum IOErrorEnum::variant WouldBlock -->
Operation would block.
<!-- enum IOErrorEnum::variant HostNotFound -->
Host couldn't be found (remote operations).
<!-- enum IOErrorEnum::variant WouldMerge -->
Operation would merge files.
<!-- enum IOErrorEnum::variant FailedHandled -->
Operation failed and a helper program has
 already interacted with the user. Do not display any error dialog.
<!-- enum IOErrorEnum::variant TooManyOpenFiles -->
The current process has too many files
 open and can't open any more. Duplicate descriptors do count toward
 this limit. Since 2.20
<!-- enum IOErrorEnum::variant NotInitialized -->
The object has not been initialized. Since 2.22
<!-- enum IOErrorEnum::variant AddressInUse -->
The requested address is already in use. Since 2.22
<!-- enum IOErrorEnum::variant PartialInput -->
Need more input to finish operation. Since 2.24
<!-- enum IOErrorEnum::variant InvalidData -->
The input data was invalid. Since 2.24
<!-- enum IOErrorEnum::variant DbusError -->
A remote object generated an error that
 doesn't correspond to a locally registered `glib::Error` error
 domain. Use `DBusError::get_remote_error` to extract the D-Bus
 error name and `DBusError::strip_remote_error` to fix up the
 message so it matches what was received on the wire. Since 2.26.
<!-- enum IOErrorEnum::variant HostUnreachable -->
Host unreachable. Since 2.26
<!-- enum IOErrorEnum::variant NetworkUnreachable -->
Network unreachable. Since 2.26
<!-- enum IOErrorEnum::variant ConnectionRefused -->
Connection refused. Since 2.26
<!-- enum IOErrorEnum::variant ProxyFailed -->
Connection to proxy server failed. Since 2.26
<!-- enum IOErrorEnum::variant ProxyAuthFailed -->
Proxy authentication failed. Since 2.26
<!-- enum IOErrorEnum::variant ProxyNeedAuth -->
Proxy server needs authentication. Since 2.26
<!-- enum IOErrorEnum::variant ProxyNotAllowed -->
Proxy connection is not allowed by ruleset.
 Since 2.26
<!-- enum IOErrorEnum::variant BrokenPipe -->
Broken pipe. Since 2.36
<!-- enum IOErrorEnum::variant ConnectionClosed -->
Connection closed by peer. Note that this
 is the same code as `IOErrorEnum::BrokenPipe`; before 2.44 some
 "connection closed" errors returned `IOErrorEnum::BrokenPipe`, but others
 returned `IOErrorEnum::Failed`. Now they should all return the same
 value, which has this more logical name. Since 2.44.
<!-- enum IOErrorEnum::variant NotConnected -->
Transport endpoint is not connected. Since 2.44
<!-- enum IOErrorEnum::variant MessageTooLarge -->
Message too large. Since 2.48.
<!-- struct IOStream -->
GIOStream represents an object that has both read and write streams.
Generally the two streams act as separate input and output streams,
but they share some common resources and state. For instance, for
seekable streams, both streams may use the same position.

Examples of `IOStream` objects are `SocketConnection`, which represents
a two-way network connection; and `FileIOStream`, which represents a
file handle opened in read-write mode.

To do the actual reading and writing you need to get the substreams
with `IOStreamExt::get_input_stream` and `IOStreamExt::get_output_stream`.

The `IOStream` object owns the input and the output streams, not the other
way around, so keeping the substreams alive will not keep the `IOStream`
object alive. If the `IOStream` object is freed it will be closed, thus
closing the substreams, so even if the substreams stay alive they will
always return `IOErrorEnum::Closed` for all operations.

To close a stream use `IOStreamExt::close` which will close the common
stream object and also the individual substreams. You can also close
the substreams themselves. In most cases this only marks the
substream as closed, so further I/O on it fails but common state in the
`IOStream` may still be open. However, some streams may support
"half-closed" states where one direction of the stream is actually shut down.

Operations on `GIOStreams` cannot be started while another operation on the
`IOStream` or its substreams is in progress. Specifically, an application can
read from the `InputStream` and write to the `OutputStream` simultaneously
(either in separate threads, or as asynchronous operations in the same
thread), but an application cannot start any `IOStream` operation while there
is a `IOStream`, `InputStream` or `OutputStream` operation in progress, and
an application can’t start any `InputStream` or `OutputStream` operation
while there is a `IOStream` operation in progress.

This is a product of individual stream operations being associated with a
given `glib::MainContext` (the thread-default context at the time the operation was
started), rather than entire streams being associated with a single
`glib::MainContext`.

GIO may run operations on `GIOStreams` from other (worker) threads, and this
may be exposed to application code in the behaviour of wrapper streams, such
as `BufferedInputStream` or `TlsConnection`. With such wrapper APIs,
application code may only run operations on the base (wrapped) stream when
the wrapper stream is idle. Note that the semantics of such operations may
not be well-defined due to the state the wrapper stream leaves the base
stream in (though they are guaranteed not to crash).

# Implements

[`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait IOStreamExt -->
Trait containing all `IOStream` methods.

# Implementors

[`FileIOStream`](struct.FileIOStream.html), [`IOStream`](struct.IOStream.html), [`SimpleIOStream`](struct.SimpleIOStream.html), [`SocketConnection`](struct.SocketConnection.html), [`TlsConnection`](struct.TlsConnection.html)
<!-- impl IOStream::fn splice_finish -->
Finishes an asynchronous io stream splice operation.
## `result`
a `AsyncResult`.

# Returns

`true` on success, `false` otherwise.
<!-- trait IOStreamExt::fn clear_pending -->
Clears the pending flag on `self`.
<!-- trait IOStreamExt::fn close -->
Closes the stream, releasing resources related to it. This will also
close the individual input and output streams, if they are not already
closed.

Once the stream is closed, all other operations will return
`IOErrorEnum::Closed`. Closing a stream multiple times will not
return an error.

Closing a stream will automatically flush any outstanding buffers
in the stream.

Streams will be automatically closed when the last reference
is dropped, but you might want to call this function to make sure
resources are released as early as possible.

Some streams might keep the backing store of the stream (e.g. a file
descriptor) open after the stream is closed. See the documentation for
the individual stream for details.

On failure the first error that happened will be reported, but the
close operation will finish as much as possible. A stream that failed
to close will still return `IOErrorEnum::Closed` for all operations.
Still, it is important to check and report the error to the user,
otherwise there might be a loss of data as all data might not be written.

If `cancellable` is not NULL, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
Cancelling a close will still leave the stream closed, but some streams
can use a faster close that doesn't block to e.g. check errors.

The default implementation of this method just calls close on the
individual input/output streams.
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

`true` on success, `false` on failure
<!-- trait IOStreamExt::fn close_async -->
Requests an asynchronous close of the stream, releasing resources
related to it. When the operation is finished `callback` will be
called. You can then call `IOStreamExt::close_finish` to get
the result of the operation.

For behaviour details see `IOStreamExt::close`.

The asynchronous methods have a default fallback that uses threads
to implement asynchronicity, so they are optional for inheriting
classes. However, if you override one you must override all.
## `io_priority`
the io priority of the request
## `cancellable`
optional cancellable object
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait IOStreamExt::fn close_finish -->
Closes a stream.
## `result`
a `AsyncResult`

# Returns

`true` if stream was successfully closed, `false` otherwise.
<!-- trait IOStreamExt::fn get_input_stream -->
Gets the input stream for this object. This is used
for reading.

# Returns

a `InputStream`, owned by the `IOStream`.
Do not free.
<!-- trait IOStreamExt::fn get_output_stream -->
Gets the output stream for this object. This is used for
writing.

# Returns

a `OutputStream`, owned by the `IOStream`.
Do not free.
<!-- trait IOStreamExt::fn has_pending -->
Checks if a stream has pending actions.

# Returns

`true` if `self` has pending actions.
<!-- trait IOStreamExt::fn is_closed -->
Checks if a stream is closed.

# Returns

`true` if the stream is closed.
<!-- trait IOStreamExt::fn set_pending -->
Sets `self` to have actions pending. If the pending flag is
already set or `self` is closed, it will return `false` and set
`error`.

# Returns

`true` if pending was previously unset and is now set.
<!-- trait IOStreamExt::fn splice_async -->
Asyncronously splice the output stream of `self` to the input stream of
`stream2`, and splice the output stream of `stream2` to the input stream of
`self`.

When the operation is finished `callback` will be called.
You can then call `IOStream::splice_finish` to get the
result of the operation.
## `stream2`
a `IOStream`.
## `flags`
a set of `IOStreamSpliceFlags`.
## `io_priority`
the io priority of the request.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`.
## `user_data`
user data passed to `callback`.
<!-- struct Icon -->
`Icon` is a very minimal interface for icons. It provides functions
for checking the equality of two icons, hashing of icons and
serializing an icon to and from strings.

`Icon` does not provide the actual pixmap for the icon as this is out
of GIO's scope, however implementations of `Icon` may contain the name
of an icon (see `ThemedIcon`), or the path to an icon (see `LoadableIcon`).

To obtain a hash of a `Icon`, see `Icon::hash`.

To check if two `GIcons` are equal, see `Icon::equal`.

For serializing a `Icon`, use `Icon::serialize` and
`Icon::deserialize`.

If you want to consume `Icon` (for example, in a toolkit) you must
be prepared to handle at least the three following cases:
`LoadableIcon`, `ThemedIcon` and `EmblemedIcon`. It may also make
sense to have fast-paths for other cases (like handling ``GdkPixbuf``
directly, for example) but all compliant `Icon` implementations
outside of GIO must implement `LoadableIcon`.

If your application or library provides one or more `Icon`
implementations you need to ensure that your new implementation also
implements `LoadableIcon`. Additionally, you must provide an
implementation of `Icon::serialize` that gives a result that is
understood by `Icon::deserialize`, yielding one of the built-in icon
types.

# Implements

[`IconExt`](trait.IconExt.html)
<!-- trait IconExt -->
Trait containing all `Icon` methods.

# Implementors

[`BytesIcon`](struct.BytesIcon.html), [`Emblem`](struct.Emblem.html), [`EmblemedIcon`](struct.EmblemedIcon.html), [`FileIcon`](struct.FileIcon.html), [`Icon`](struct.Icon.html), [`LoadableIcon`](struct.LoadableIcon.html), [`ThemedIcon`](struct.ThemedIcon.html)
<!-- impl Icon::fn deserialize -->
Deserializes a `Icon` previously serialized using `Icon::serialize`.
## `value`
a `glib::Variant` created with `Icon::serialize`

# Returns

a `Icon`, or `None` when deserialization fails.
<!-- impl Icon::fn hash -->
Gets a hash for an icon.
## `icon`
`gconstpointer` to an icon object.

# Returns

a `guint` containing a hash for the `icon`, suitable for
use in a `glib::HashTable` or similar data structure.
<!-- impl Icon::fn new_for_string -->
Generate a `Icon` instance from `str`. This function can fail if
`str` is not valid - see `Icon::to_string` for discussion.

If your application or library provides one or more `Icon`
implementations you need to ensure that each `glib::Type` is registered
with the type system prior to calling `Icon::new_for_string`.
## `str`
A string obtained via `Icon::to_string`.

# Returns

An object implementing the `Icon`
 interface or `None` if `error` is set.
<!-- trait IconExt::fn equal -->
Checks if two icons are equal.
## `icon2`
pointer to the second `Icon`.

# Returns

`true` if `self` is equal to `icon2`. `false` otherwise.
<!-- trait IconExt::fn serialize -->
Serializes a `Icon` into a `glib::Variant`. An equivalent `Icon` can be retrieved
back by calling `Icon::deserialize` on the returned value.
As serialization will avoid using raw icon data when possible, it only
makes sense to transfer the `glib::Variant` between processes on the same machine,
(as opposed to over the network), and within the same file system namespace.

# Returns

a `glib::Variant`, or `None` when serialization fails.
<!-- trait IconExt::fn to_string -->
Generates a textual representation of `self` that can be used for
serialization such as when passing `self` to a different process or
saving it to persistent storage. Use `Icon::new_for_string` to
get `self` back from the returned string.

The encoding of the returned string is proprietary to `Icon` except
in the following two cases

- If `self` is a `FileIcon`, the returned string is a native path
 (such as `/path/to/my icon.png`) without escaping
 if the `File` for `self` is a native file. If the file is not
 native, the returned string is the result of `File::get_uri`
 (such as `sftp://path/to/my%20icon.png`).

- If `self` is a `ThemedIcon` with exactly one name and no fallbacks,
 the encoding is simply the name (such as `network-server`).

# Returns

An allocated NUL-terminated UTF8 string or
`None` if `self` can't be serialized. Use `g_free` to free.
<!-- struct InetAddress -->
`InetAddress` represents an IPv4 or IPv6 internet address. Use
`ResolverExt::lookup_by_name` or `ResolverExt::lookup_by_name_async` to
look up the `InetAddress` for a hostname. Use
`ResolverExt::lookup_by_address` or
`ResolverExt::lookup_by_address_async` to look up the hostname for a
`InetAddress`.

To actually connect to a remote host, you will need a
`InetSocketAddress` (which includes a `InetAddress` as well as a
port number).

# Implements

[`InetAddressExt`](trait.InetAddressExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait InetAddressExt -->
Trait containing all `InetAddress` methods.

# Implementors

[`InetAddress`](struct.InetAddress.html)
<!-- impl InetAddress::fn new_any -->
Creates a `InetAddress` for the "any" address (unassigned/"don't
care") for `family`.
## `family`
the address family

# Returns

a new `InetAddress` corresponding to the "any" address
for `family`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl InetAddress::fn new_from_bytes -->
Creates a new `InetAddress` from the given `family` and `bytes`.
`bytes` should be 4 bytes for `SocketFamily::Ipv4` and 16 bytes for
`SocketFamily::Ipv6`.
## `bytes`
raw address data
## `family`
the address family of `bytes`

# Returns

a new `InetAddress` corresponding to `family` and `bytes`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl InetAddress::fn new_from_string -->
Parses `string` as an IP address and creates a new `InetAddress`.
## `string`
a string representation of an IP address

# Returns

a new `InetAddress` corresponding to `string`, or `None` if
`string` could not be parsed.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl InetAddress::fn new_loopback -->
Creates a `InetAddress` for the loopback address for `family`.
## `family`
the address family

# Returns

a new `InetAddress` corresponding to the loopback address
for `family`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait InetAddressExt::fn equal -->
Checks if two `InetAddress` instances are equal, e.g. the same address.
## `other_address`
Another `InetAddress`.

# Returns

`true` if `self` and `other_address` are equal, `false` otherwise.
<!-- trait InetAddressExt::fn get_family -->
Gets `self`'s family

# Returns

`self`'s family
<!-- trait InetAddressExt::fn get_is_any -->
Tests whether `self` is the "any" address for its family.

# Returns

`true` if `self` is the "any" address for its family.
<!-- trait InetAddressExt::fn get_is_link_local -->
Tests whether `self` is a link-local address (that is, if it
identifies a host on a local network that is not connected to the
Internet).

# Returns

`true` if `self` is a link-local address.
<!-- trait InetAddressExt::fn get_is_loopback -->
Tests whether `self` is the loopback address for its family.

# Returns

`true` if `self` is the loopback address for its family.
<!-- trait InetAddressExt::fn get_is_mc_global -->
Tests whether `self` is a global multicast address.

# Returns

`true` if `self` is a global multicast address.
<!-- trait InetAddressExt::fn get_is_mc_link_local -->
Tests whether `self` is a link-local multicast address.

# Returns

`true` if `self` is a link-local multicast address.
<!-- trait InetAddressExt::fn get_is_mc_node_local -->
Tests whether `self` is a node-local multicast address.

# Returns

`true` if `self` is a node-local multicast address.
<!-- trait InetAddressExt::fn get_is_mc_org_local -->
Tests whether `self` is an organization-local multicast address.

# Returns

`true` if `self` is an organization-local multicast address.
<!-- trait InetAddressExt::fn get_is_mc_site_local -->
Tests whether `self` is a site-local multicast address.

# Returns

`true` if `self` is a site-local multicast address.
<!-- trait InetAddressExt::fn get_is_multicast -->
Tests whether `self` is a multicast address.

# Returns

`true` if `self` is a multicast address.
<!-- trait InetAddressExt::fn get_is_site_local -->
Tests whether `self` is a site-local address such as 10.0.0.1
(that is, the address identifies a host on a local network that can
not be reached directly from the Internet, but which may have
outgoing Internet connectivity via a NAT or firewall).

# Returns

`true` if `self` is a site-local address.
<!-- trait InetAddressExt::fn get_native_size -->
Gets the size of the native raw binary address for `self`. This
is the size of the data that you get from `InetAddress::to_bytes`.

# Returns

the number of bytes used for the native version of `self`.
<!-- trait InetAddressExt::fn to_bytes -->
Gets the raw binary address data from `self`.

# Returns

a pointer to an internal array of the bytes in `self`,
which should not be modified, stored, or freed. The size of this
array can be gotten with `InetAddressExt::get_native_size`.
<!-- trait InetAddressExt::fn to_string -->
Converts `self` to string form.

# Returns

a representation of `self` as a string, which should be
freed after use.
<!-- trait InetAddressExt::fn get_property_is_any -->
Whether this is the "any" address for its family.
See `InetAddressExt::get_is_any`.
<!-- trait InetAddressExt::fn get_property_is_link_local -->
Whether this is a link-local address.
See `InetAddressExt::get_is_link_local`.
<!-- trait InetAddressExt::fn get_property_is_loopback -->
Whether this is the loopback address for its family.
See `InetAddressExt::get_is_loopback`.
<!-- trait InetAddressExt::fn get_property_is_mc_global -->
Whether this is a global multicast address.
See `InetAddressExt::get_is_mc_global`.
<!-- trait InetAddressExt::fn get_property_is_mc_link_local -->
Whether this is a link-local multicast address.
See `InetAddressExt::get_is_mc_link_local`.
<!-- trait InetAddressExt::fn get_property_is_mc_node_local -->
Whether this is a node-local multicast address.
See `InetAddressExt::get_is_mc_node_local`.
<!-- trait InetAddressExt::fn get_property_is_mc_org_local -->
Whether this is an organization-local multicast address.
See `InetAddressExt::get_is_mc_org_local`.
<!-- trait InetAddressExt::fn get_property_is_mc_site_local -->
Whether this is a site-local multicast address.
See `InetAddressExt::get_is_mc_site_local`.
<!-- trait InetAddressExt::fn get_property_is_multicast -->
Whether this is a multicast address.
See `InetAddressExt::get_is_multicast`.
<!-- trait InetAddressExt::fn get_property_is_site_local -->
Whether this is a site-local address.
See `InetAddressExt::get_is_loopback`.
<!-- struct InetAddressMask -->
`InetAddressMask` represents a range of IPv4 or IPv6 addresses
described by a base address and a length indicating how many bits
of the base address are relevant for matching purposes. These are
often given in string form. Eg, "10.0.0.0/8", or "fe80::/10".

# Implements

[`InetAddressMaskExt`](trait.InetAddressMaskExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait InetAddressMaskExt -->
Trait containing all `InetAddressMask` methods.

# Implementors

[`InetAddressMask`](struct.InetAddressMask.html)
<!-- impl InetAddressMask::fn new -->
Creates a new `InetAddressMask` representing all addresses whose
first `length` bits match `addr`.
## `addr`
a `InetAddress`
## `length`
number of bits of `addr` to use

# Returns

a new `InetAddressMask`, or `None` on error
<!-- impl InetAddressMask::fn new_from_string -->
Parses `mask_string` as an IP address and (optional) length, and
creates a new `InetAddressMask`. The length, if present, is
delimited by a "/". If it is not present, then the length is
assumed to be the full length of the address.
## `mask_string`
an IP address or address/length string

# Returns

a new `InetAddressMask` corresponding to `string`, or `None`
on error.
<!-- trait InetAddressMaskExt::fn equal -->
Tests if `self` and `mask2` are the same mask.
## `mask2`
another `InetAddressMask`

# Returns

whether `self` and `mask2` are the same mask
<!-- trait InetAddressMaskExt::fn get_address -->
Gets `self`'s base address

# Returns

`self`'s base address
<!-- trait InetAddressMaskExt::fn get_family -->
Gets the `SocketFamily` of `self`'s address

# Returns

the `SocketFamily` of `self`'s address
<!-- trait InetAddressMaskExt::fn get_length -->
Gets `self`'s length

# Returns

`self`'s length
<!-- trait InetAddressMaskExt::fn matches -->
Tests if `address` falls within the range described by `self`.
## `address`
a `InetAddress`

# Returns

whether `address` falls within the range described by
`self`.
<!-- trait InetAddressMaskExt::fn to_string -->
Converts `self` back to its corresponding string form.

# Returns

a string corresponding to `self`.
<!-- struct InetSocketAddress -->
An IPv4 or IPv6 socket address; that is, the combination of a
`InetAddress` and a port number.

# Implements

[`InetSocketAddressExt`](trait.InetSocketAddressExt.html), [`SocketAddressExt`](trait.SocketAddressExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketConnectableExt`](trait.SocketConnectableExt.html)
<!-- trait InetSocketAddressExt -->
Trait containing all `InetSocketAddress` methods.

# Implementors

[`InetSocketAddress`](struct.InetSocketAddress.html), [`ProxyAddress`](struct.ProxyAddress.html)
<!-- impl InetSocketAddress::fn new -->
Creates a new `InetSocketAddress` for `address` and `port`.
## `address`
a `InetAddress`
## `port`
a port number

# Returns

a new `InetSocketAddress`
<!-- impl InetSocketAddress::fn new_from_string -->
Creates a new `InetSocketAddress` for `address` and `port`.

If `address` is an IPv6 address, it can also contain a scope ID
(separated from the address by a `%`).
## `address`
the string form of an IP address
## `port`
a port number

# Returns

a new `InetSocketAddress`, or `None` if `address` cannot be
parsed.
<!-- trait InetSocketAddressExt::fn get_address -->
Gets `self`'s `InetAddress`.

# Returns

the `InetAddress` for `self`, which must be
`gobject::ObjectExt::ref`'d if it will be stored
<!-- trait InetSocketAddressExt::fn get_flowinfo -->
Gets the `sin6_flowinfo` field from `self`,
which must be an IPv6 address.

# Returns

the flowinfo field
<!-- trait InetSocketAddressExt::fn get_port -->
Gets `self`'s port.

# Returns

the port for `self`
<!-- trait InetSocketAddressExt::fn get_scope_id -->
Gets the `sin6_scope_id` field from `self`,
which must be an IPv6 address.

# Returns

the scope id field
<!-- trait InetSocketAddressExt::fn get_property_flowinfo -->
The `sin6_flowinfo` field, for IPv6 addresses.
<!-- trait InetSocketAddressExt::fn set_property_flowinfo -->
The `sin6_flowinfo` field, for IPv6 addresses.
<!-- struct InputStream -->
`InputStream` has functions to read from a stream (`InputStream::read`),
to close a stream (`InputStreamExt::close`) and to skip some content
(`InputStreamExt::skip`).

To copy the content of an input stream to an output stream without
manually handling the reads and writes, use `OutputStreamExt::splice`.

See the documentation for `IOStream` for details of thread safety of
streaming APIs.

All of these functions have async variants too.

# Implements

[`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html)
<!-- trait InputStreamExt -->
Trait containing all `InputStream` methods.

# Implementors

[`FileInputStream`](struct.FileInputStream.html), [`FilterInputStream`](struct.FilterInputStream.html), [`InputStream`](struct.InputStream.html), [`MemoryInputStream`](struct.MemoryInputStream.html), [`PollableInputStream`](struct.PollableInputStream.html), [`UnixInputStream`](struct.UnixInputStream.html)
<!-- trait InputStreamExt::fn clear_pending -->
Clears the pending flag on `self`.
<!-- trait InputStreamExt::fn close -->
Closes the stream, releasing resources related to it.

Once the stream is closed, all other operations will return `IOErrorEnum::Closed`.
Closing a stream multiple times will not return an error.

Streams will be automatically closed when the last reference
is dropped, but you might want to call this function to make sure
resources are released as early as possible.

Some streams might keep the backing store of the stream (e.g. a file descriptor)
open after the stream is closed. See the documentation for the individual
stream for details.

On failure the first error that happened will be reported, but the close
operation will finish as much as possible. A stream that failed to
close will still return `IOErrorEnum::Closed` for all operations. Still, it
is important to check and report the error to the user.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
Cancelling a close will still leave the stream closed, but some streams
can use a faster close that doesn't block to e.g. check errors.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` on success, `false` on failure
<!-- trait InputStreamExt::fn close_async -->
Requests an asynchronous closes of the stream, releasing resources related to it.
When the operation is finished `callback` will be called.
You can then call `InputStreamExt::close_finish` to get the result of the
operation.

For behaviour details see `InputStreamExt::close`.

The asynchronous methods have a default fallback that uses threads to implement
asynchronicity, so they are optional for inheriting classes. However, if you
override one you must override all.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional cancellable object
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait InputStreamExt::fn close_finish -->
Finishes closing a stream asynchronously, started from `InputStreamExt::close_async`.
## `result`
a `AsyncResult`.

# Returns

`true` if the stream was closed successfully.
<!-- trait InputStreamExt::fn has_pending -->
Checks if an input stream has pending actions.

# Returns

`true` if `self` has pending actions.
<!-- trait InputStreamExt::fn is_closed -->
Checks if an input stream is closed.

# Returns

`true` if the stream is closed.
<!-- trait InputStreamExtManual::fn read -->
Tries to read `count` bytes from the stream into the buffer starting at
`buffer`. Will block during this read.

If count is zero returns zero and does nothing. A value of `count`
larger than `G_MAXSSIZE` will cause a `IOErrorEnum::InvalidArgument` error.

On success, the number of bytes read into the buffer is returned.
It is not an error if this is not the same as the requested size, as it
can happen e.g. near the end of a file. Zero is returned on end of file
(or if `count` is zero), but never otherwise.

The returned `buffer` is not a nul-terminated string, it can contain nul bytes
at any position, and this function doesn't nul-terminate the `buffer`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.

On error -1 is returned and `error` is set accordingly.
## `buffer`
a buffer to
 read data into (which should be at least count bytes long).
## `count`
the number of bytes that will be read from the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

Number of bytes read, or -1 on error, or 0 on end of file.
<!-- trait InputStreamExtManual::fn read_all -->
Tries to read `count` bytes from the stream into the buffer starting at
`buffer`. Will block during this read.

This function is similar to `InputStream::read`, except it tries to
read as many bytes as requested, only stopping on an error or end of stream.

On a successful read of `count` bytes, or if we reached the end of the
stream, `true` is returned, and `bytes_read` is set to the number of bytes
read into `buffer`.

If there is an error during the operation `false` is returned and `error`
is set to indicate the error status.

As a special exception to the normal conventions for functions that
use `glib::Error`, if this function returns `false` (and sets `error`) then
`bytes_read` will be set to the number of bytes that were successfully
read before the error was encountered. This functionality is only
available from C. If you need it from another language then you must
write your own loop around `InputStream::read`.
## `buffer`
a buffer to
 read data into (which should be at least count bytes long).
## `count`
the number of bytes that will be read from the stream
## `bytes_read`
location to store the number of bytes that was read from the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` on success, `false` if there was an error
<!-- trait InputStreamExtManual::fn read_all_async -->
Request an asynchronous read of `count` bytes from the stream into the
buffer starting at `buffer`.

This is the asynchronous equivalent of `InputStream::read_all`.

Call `InputStreamExt::read_all_finish` to collect the result.

Any outstanding I/O request with higher priority (lower numerical
value) will be executed before an outstanding request with lower
priority. Default priority is `G_PRIORITY_DEFAULT`.

Feature: `v2_44`

## `buffer`
a buffer to
 read data into (which should be at least count bytes long)
## `count`
the number of bytes that will be read from the stream
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait InputStreamExt::fn read_all_finish -->
Finishes an asynchronous stream read operation started with
`InputStream::read_all_async`.

As a special exception to the normal conventions for functions that
use `glib::Error`, if this function returns `false` (and sets `error`) then
`bytes_read` will be set to the number of bytes that were successfully
read before the error was encountered. This functionality is only
available from C. If you need it from another language then you must
write your own loop around `InputStream::read_async`.

Feature: `v2_44`

## `result`
a `AsyncResult`
## `bytes_read`
location to store the number of bytes that was read from the stream

# Returns

`true` on success, `false` if there was an error
<!-- trait InputStreamExtManual::fn read_async -->
Request an asynchronous read of `count` bytes from the stream into the buffer
starting at `buffer`. When the operation is finished `callback` will be called.
You can then call `InputStreamExt::read_finish` to get the result of the
operation.

During an async request no other sync and async calls are allowed on `self`, and will
result in `IOErrorEnum::Pending` errors.

A value of `count` larger than `G_MAXSSIZE` will cause a `IOErrorEnum::InvalidArgument` error.

On success, the number of bytes read into the buffer will be passed to the
callback. It is not an error if this is not the same as the requested size, as it
can happen e.g. near the end of a file, but generally we try to read
as many bytes as requested. Zero is returned on end of file
(or if `count` is zero), but never otherwise.

Any outstanding i/o request with higher priority (lower numerical value) will
be executed before an outstanding request with lower priority. Default
priority is `G_PRIORITY_DEFAULT`.

The asynchronous methods have a default fallback that uses threads to implement
asynchronicity, so they are optional for inheriting classes. However, if you
override one you must override all.
## `buffer`
a buffer to
 read data into (which should be at least count bytes long).
## `count`
the number of bytes that will be read from the stream
## `io_priority`
the [I/O priority][io-priority]
of the request.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait InputStreamExt::fn read_bytes -->
Like `InputStream::read`, this tries to read `count` bytes from
the stream in a blocking fashion. However, rather than reading into
a user-supplied buffer, this will create a new `glib::Bytes` containing
the data that was read. This may be easier to use from language
bindings.

If count is zero, returns a zero-length `glib::Bytes` and does nothing. A
value of `count` larger than `G_MAXSSIZE` will cause a
`IOErrorEnum::InvalidArgument` error.

On success, a new `glib::Bytes` is returned. It is not an error if the
size of this object is not the same as the requested size, as it
can happen e.g. near the end of a file. A zero-length `glib::Bytes` is
returned on end of file (or if `count` is zero), but never
otherwise.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.

On error `None` is returned and `error` is set accordingly.
## `count`
maximum number of bytes that will be read from the stream. Common
values include 4096 and 8192.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a new `glib::Bytes`, or `None` on error
<!-- trait InputStreamExt::fn read_bytes_async -->
Request an asynchronous read of `count` bytes from the stream into a
new `glib::Bytes`. When the operation is finished `callback` will be
called. You can then call `InputStreamExt::read_bytes_finish` to get the
result of the operation.

During an async request no other sync and async calls are allowed
on `self`, and will result in `IOErrorEnum::Pending` errors.

A value of `count` larger than `G_MAXSSIZE` will cause a
`IOErrorEnum::InvalidArgument` error.

On success, the new `glib::Bytes` will be passed to the callback. It is
not an error if this is smaller than the requested size, as it can
happen e.g. near the end of a file, but generally we try to read as
many bytes as requested. Zero is returned on end of file (or if
`count` is zero), but never otherwise.

Any outstanding I/O request with higher priority (lower numerical
value) will be executed before an outstanding request with lower
priority. Default priority is `G_PRIORITY_DEFAULT`.
## `count`
the number of bytes that will be read from the stream
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait InputStreamExt::fn read_bytes_finish -->
Finishes an asynchronous stream read-into-`glib::Bytes` operation.
## `result`
a `AsyncResult`.

# Returns

the newly-allocated `glib::Bytes`, or `None` on error
<!-- trait InputStreamExt::fn read_finish -->
Finishes an asynchronous stream read operation.
## `result`
a `AsyncResult`.

# Returns

number of bytes read in, or -1 on error, or 0 on end of file.
<!-- trait InputStreamExt::fn set_pending -->
Sets `self` to have actions pending. If the pending flag is
already set or `self` is closed, it will return `false` and set
`error`.

# Returns

`true` if pending was previously unset and is now set.
<!-- trait InputStreamExt::fn skip -->
Tries to skip `count` bytes from the stream. Will block during the operation.

This is identical to `InputStream::read`, from a behaviour standpoint,
but the bytes that are skipped are not returned to the user. Some
streams have an implementation that is more efficient than reading the data.

This function is optional for inherited classes, as the default implementation
emulates it using read.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.
## `count`
the number of bytes that will be skipped from the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

Number of bytes skipped, or -1 on error
<!-- trait InputStreamExt::fn skip_async -->
Request an asynchronous skip of `count` bytes from the stream.
When the operation is finished `callback` will be called.
You can then call `InputStreamExt::skip_finish` to get the result
of the operation.

During an async request no other sync and async calls are allowed,
and will result in `IOErrorEnum::Pending` errors.

A value of `count` larger than `G_MAXSSIZE` will cause a `IOErrorEnum::InvalidArgument` error.

On success, the number of bytes skipped will be passed to the callback.
It is not an error if this is not the same as the requested size, as it
can happen e.g. near the end of a file, but generally we try to skip
as many bytes as requested. Zero is returned on end of file
(or if `count` is zero), but never otherwise.

Any outstanding i/o request with higher priority (lower numerical value)
will be executed before an outstanding request with lower priority.
Default priority is `G_PRIORITY_DEFAULT`.

The asynchronous methods have a default fallback that uses threads to
implement asynchronicity, so they are optional for inheriting classes.
However, if you override one, you must override all.
## `count`
the number of bytes that will be skipped from the stream
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait InputStreamExt::fn skip_finish -->
Finishes a stream skip operation.
## `result`
a `AsyncResult`.

# Returns

the size of the bytes skipped, or %-1 on error.
<!-- struct ListModel -->
`ListModel` is an interface that represents a mutable list of
`GObjects`. Its main intention is as a model for various widgets in
user interfaces, such as list views, but it can also be used as a
convenient method of returning lists of data, with support for
updates.

Each object in the list may also report changes in itself via some
mechanism (normally the `gobject::Object::notify` signal). Taken together
with the `ListModel::items-changed` signal, this provides for a list
that can change its membership, and in which the members can change
their individual properties.

A good example would be the list of visible wireless network access
points, where each access point can report dynamic properties such as
signal strength.

It is important to note that the `ListModel` itself does not report
changes to the individual items. It only reports changes to the list
membership. If you want to observe changes to the objects themselves
then you need to connect signals to the objects that you are
interested in.

All items in a `ListModel` are of (or derived from) the same type.
`ListModel::get_item_type` returns that type. The type may be an
interface, in which case all objects in the list must implement it.

The semantics are close to that of an array:
`ListModel::get_n_items` returns the number of items in the list and
`ListModel::get_item` returns an item at a (0-based) position. In
order to allow implementations to calculate the list length lazily,
you can also iterate over items: starting from 0, repeatedly call
`ListModel::get_item` until it returns `None`.

An implementation may create objects lazily, but must take care to
return the same object for a given position until all references to
it are gone.

On the other side, a consumer is expected only to hold references on
objects that are currently "user visible", in order to faciliate the
maximum level of laziness in the implementation of the list and to
reduce the required number of signal connections at a given time.

This interface is intended only to be used from a single thread. The
thread in which it is appropriate to use it depends on the particular
implementation, but typically it will be from the thread that owns
the [thread-default main context][g-main-context-push-thread-default]
in effect at the time that the model was created.

Feature: `v2_44`

# Implements

[`ListModelExt`](trait.ListModelExt.html)
<!-- trait ListModelExt -->
Trait containing all `ListModel` methods.

Feature: `v2_44`

# Implementors

[`ListModel`](struct.ListModel.html), [`ListStore`](struct.ListStore.html)
<!-- trait ListModelExt::fn get_item -->
Get the item at `position`. If `position` is greater than the number of
items in `self`, `None` is returned.

`None` is never returned for an index that is smaller than the length
of the list. See `ListModel::get_n_items`.

Feature: `v2_44`

## `position`
the position of the item to fetch

# Returns

the item at `position`.
<!-- trait ListModelExt::fn get_item_type -->
Gets the type of the items in `self`. All items returned from
`g_list_model_get_type` are of that type or a subtype, or are an
implementation of that interface.

The item type of a `ListModel` can not change during the life of the
model.

Feature: `v2_44`


# Returns

the `glib::Type` of the items contained in `self`.
<!-- trait ListModelExt::fn get_n_items -->
Gets the number of items in `self`.

Depending on the model implementation, calling this function may be
less efficient than iterating the list with increasing values for
`position` until `ListModel::get_item` returns `None`.

Feature: `v2_44`


# Returns

the number of items in `self`.
<!-- trait ListModelExt::fn get_object -->
Get the item at `position`. If `position` is greater than the number of
items in `self`, `None` is returned.

`None` is never returned for an index that is smaller than the length
of the list. See `ListModel::get_n_items`.

Feature: `v2_44`

## `position`
the position of the item to fetch

# Returns

the object at `position`.
<!-- trait ListModelExt::fn items_changed -->
Emits the `ListModel::items-changed` signal on `self`.

This function should only be called by classes implementing
`ListModel`. It has to be called after the internal representation
of `self` has been updated, because handlers connected to this signal
might query the new state of the list.

Implementations must only make changes to the model (as visible to
its consumer) in places that will not cause problems for that
consumer. For models that are driven directly by a write API (such
as `ListStore`), changes can be reported in response to uses of that
API. For models that represent remote data, changes should only be
made from a fresh mainloop dispatch. It is particularly not
permitted to make changes in response to a call to the `ListModel`
consumer API.

Stated another way: in general, it is assumed that code making a
series of accesses to the model via the API, without returning to the
mainloop, and without calling other code, will continue to view the
same contents of the model.

Feature: `v2_44`

## `position`
the position at which `self` changed
## `removed`
the number of items removed
## `added`
the number of items added
<!-- trait ListModelExt::fn connect_items_changed -->
This signal is emitted whenever items were added or removed to
`list`. At `position`, `removed` items were removed and `added` items
were added in their place.

Feature: `v2_44`

## `position`
the position at which `list` changed
## `removed`
the number of items removed
## `added`
the number of items added
<!-- struct ListStore -->
`ListStore` is a simple implementation of `ListModel` that stores all
items in memory.

It provides insertions, deletions, and lookups in logarithmic time
with a fast path for the common case of iterating the list linearly.

Feature: `v2_44`

# Implements

[`ListStoreExt`](trait.ListStoreExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ListModelExt`](trait.ListModelExt.html), [`ListStoreExtManual`](prelude/trait.ListStoreExtManual.html)
<!-- trait ListStoreExt -->
Trait containing all `ListStore` methods.

Feature: `v2_44`

# Implementors

[`ListStore`](struct.ListStore.html)
<!-- impl ListStore::fn new -->
Creates a new `ListStore` with items of type `item_type`. `item_type`
must be a subclass of `gobject::Object`.

Feature: `v2_44`

## `item_type`
the `glib::Type` of items in the list

# Returns

a new `ListStore`
<!-- trait ListStoreExt::fn append -->
Appends `item` to `self`. `item` must be of type `ListStore:item-type`.

This function takes a ref on `item`.

Use `ListStoreExt::splice` to append multiple items at the same time
efficiently.

Feature: `v2_44`

## `item`
the new item
<!-- trait ListStoreExt::fn insert -->
Inserts `item` into `self` at `position`. `item` must be of type
`ListStore:item-type` or derived from it. `position` must be smaller
than the length of the list, or equal to it to append.

This function takes a ref on `item`.

Use `ListStoreExt::splice` to insert multiple items at the same time
efficiently.

Feature: `v2_44`

## `position`
the position at which to insert the new item
## `item`
the new item
<!-- trait ListStoreExtManual::fn insert_sorted -->
Inserts `item` into `self` at a position to be determined by the
`compare_func`.

The list must already be sorted before calling this function or the
result is undefined. Usually you would approach this by only ever
inserting items by way of this function.

This function takes a ref on `item`.

Feature: `v2_44`

## `item`
the new item
## `compare_func`
pairwise comparison function for sorting
## `user_data`
user data for `compare_func`

# Returns

the position at which `item` was inserted
<!-- trait ListStoreExt::fn remove -->
Removes the item from `self` that is at `position`. `position` must be
smaller than the current length of the list.

Use `ListStoreExt::splice` to remove multiple items at the same time
efficiently.

Feature: `v2_44`

## `position`
the position of the item that is to be removed
<!-- trait ListStoreExt::fn remove_all -->
Removes all items from `self`.

Feature: `v2_44`

<!-- trait ListStoreExtManual::fn sort -->
Sort the items in `self` according to `compare_func`.

Feature: `v2_46`

## `compare_func`
pairwise comparison function for sorting
## `user_data`
user data for `compare_func`
<!-- trait ListStoreExt::fn splice -->
Changes `self` by removing `n_removals` items and adding `n_additions`
items to it. `additions` must contain `n_additions` items of type
`ListStore:item-type`. `None` is not permitted.

This function is more efficient than `ListStoreExt::insert` and
`ListStoreExt::remove`, because it only emits
`ListModel::items-changed` once for the change.

This function takes a ref on each item in `additions`.

The parameters `position` and `n_removals` must be correct (ie:
`position` + `n_removals` must be less than or equal to the length of
the list at the time this function is called).

Feature: `v2_44`

## `position`
the position at which to make the change
## `n_removals`
the number of items to remove
## `additions`
the items to add
## `n_additions`
the number of items to add
<!-- trait ListStoreExt::fn get_property_item_type -->
The type of items contained in this list store. Items must be
subclasses of `gobject::Object`.

Feature: `v2_44`

<!-- trait ListStoreExt::fn set_property_item_type -->
The type of items contained in this list store. Items must be
subclasses of `gobject::Object`.

Feature: `v2_44`

<!-- struct LoadableIcon -->
Extends the `Icon` interface and adds the ability to
load icons from streams.

# Implements

[`LoadableIconExt`](trait.LoadableIconExt.html), [`IconExt`](trait.IconExt.html)
<!-- trait LoadableIconExt -->
Trait containing all `LoadableIcon` methods.

# Implementors

[`BytesIcon`](struct.BytesIcon.html), [`FileIcon`](struct.FileIcon.html), [`LoadableIcon`](struct.LoadableIcon.html)
<!-- trait LoadableIconExt::fn load -->
Loads a loadable icon. For the asynchronous version of this function,
see `LoadableIcon::load_async`.
## `size`
an integer.
## `type_`
a location to store the type of the loaded
icon, `None` to ignore.
## `cancellable`
optional `Cancellable` object, `None` to
ignore.

# Returns

a `InputStream` to read the icon from.
<!-- trait LoadableIconExt::fn load_async -->
Loads an icon asynchronously. To finish this function, see
`LoadableIcon::load_finish`. For the synchronous, blocking
version of this function, see `LoadableIcon::load`.
## `size`
an integer.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback` to call when the
 request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait LoadableIconExt::fn load_finish -->
Finishes an asynchronous icon load started in `LoadableIcon::load_async`.
## `res`
a `AsyncResult`.
## `type_`
a location to store the type of the loaded
 icon, `None` to ignore.

# Returns

a `InputStream` to read the icon from.
<!-- struct MemoryInputStream -->
`MemoryInputStream` is a class for using arbitrary
memory chunks as input for GIO streaming input operations.

As of GLib 2.34, `MemoryInputStream` implements
`PollableInputStream`.

# Implements

[`MemoryInputStreamExt`](trait.MemoryInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableInputStreamExt`](trait.PollableInputStreamExt.html), [`SeekableExt`](trait.SeekableExt.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html), [`PollableInputStreamExtManual`](prelude/trait.PollableInputStreamExtManual.html)
<!-- trait MemoryInputStreamExt -->
Trait containing all `MemoryInputStream` methods.

# Implementors

[`MemoryInputStream`](struct.MemoryInputStream.html)
<!-- impl MemoryInputStream::fn new -->
Creates a new empty `MemoryInputStream`.

# Returns

a new `InputStream`
<!-- impl MemoryInputStream::fn new_from_bytes -->
Creates a new `MemoryInputStream` with data from the given `bytes`.
## `bytes`
a `glib::Bytes`

# Returns

new `InputStream` read from `bytes`
<!-- impl MemoryInputStream::fn new_from_data -->
Creates a new `MemoryInputStream` with data in memory of a given size.
## `data`
input data
## `len`
length of the data, may be -1 if `data` is a nul-terminated string
## `destroy`
function that is called to free `data`, or `None`

# Returns

new `InputStream` read from `data` of `len` bytes.
<!-- trait MemoryInputStreamExt::fn add_bytes -->
Appends `bytes` to data that can be read from the input stream.
## `bytes`
input data
<!-- trait MemoryInputStreamExt::fn add_data -->
Appends `data` to data that can be read from the input stream
## `data`
input data
## `len`
length of the data, may be -1 if `data` is a nul-terminated string
## `destroy`
function that is called to free `data`, or `None`
<!-- struct MemoryOutputStream -->
`MemoryOutputStream` is a class for using arbitrary
memory chunks as output for GIO streaming output operations.

As of GLib 2.34, `MemoryOutputStream` trivially implements
`PollableOutputStream`: it always polls as ready.

# Implements

[`MemoryOutputStreamExt`](trait.MemoryOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableOutputStreamExt`](trait.PollableOutputStreamExt.html), [`SeekableExt`](trait.SeekableExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html), [`PollableOutputStreamExtManual`](prelude/trait.PollableOutputStreamExtManual.html)
<!-- trait MemoryOutputStreamExt -->
Trait containing all `MemoryOutputStream` methods.

# Implementors

[`MemoryOutputStream`](struct.MemoryOutputStream.html)
<!-- impl MemoryOutputStream::fn new -->
Creates a new `MemoryOutputStream`.

In most cases this is not the function you want. See
`MemoryOutputStream::new_resizable` instead.

If `data` is non-`None`, the stream will use that for its internal storage.

If `realloc_fn` is non-`None`, it will be used for resizing the internal
storage when necessary and the stream will be considered resizable.
In that case, the stream will start out being (conceptually) empty.
`size` is used only as a hint for how big `data` is. Specifically,
seeking to the end of a newly-created stream will seek to zero, not
`size`. Seeking past the end of the stream and then writing will
introduce a zero-filled gap.

If `realloc_fn` is `None` then the stream is fixed-sized. Seeking to
the end will seek to `size` exactly. Writing past the end will give
an 'out of space' error. Attempting to seek past the end will fail.
Unlike the resizable case, seeking to an offset within the stream and
writing will preserve the bytes passed in as `data` before that point
and will return them as part of `MemoryOutputStream::steal_data`.
If you intend to seek you should probably therefore ensure that `data`
is properly initialised.

It is probably only meaningful to provide `data` and `size` in the case
that you want a fixed-sized stream. Put another way: if `realloc_fn`
is non-`None` then it makes most sense to give `data` as `None` and
`size` as 0 (allowing `MemoryOutputStream` to do the initial
allocation for itself).


```C
// a stream that can grow
stream = g_memory_output_stream_new (NULL, 0, realloc, free);

// another stream that can grow
stream2 = g_memory_output_stream_new (NULL, 0, g_realloc, g_free);

// a fixed-size stream
data = malloc (200);
stream3 = g_memory_output_stream_new (data, 200, NULL, free);
```
## `data`
pointer to a chunk of memory to use, or `None`
## `size`
the size of `data`
## `realloc_function`
a function with `realloc` semantics (like `g_realloc`)
 to be called when `data` needs to be grown, or `None`
## `destroy_function`
a function to be called on `data` when the stream is
 finalized, or `None`

# Returns

A newly created `MemoryOutputStream` object.
<!-- impl MemoryOutputStream::fn new_resizable -->
Creates a new `MemoryOutputStream`, using `g_realloc` and `g_free`
for memory allocation.
<!-- trait MemoryOutputStreamExt::fn get_data -->
Gets any loaded data from the `self`.

Note that the returned pointer may become invalid on the next
write or truncate operation on the stream.

# Returns

pointer to the stream's data, or `None` if the data
 has been stolen
<!-- trait MemoryOutputStreamExt::fn get_data_size -->
Returns the number of bytes from the start up to including the last
byte written in the stream that has not been truncated away.

# Returns

the number of bytes written to the stream
<!-- trait MemoryOutputStreamExt::fn get_size -->
Gets the size of the currently allocated data area (available from
`MemoryOutputStream::get_data`).

You probably don't want to use this function on resizable streams.
See `MemoryOutputStreamExt::get_data_size` instead. For resizable
streams the size returned by this function is an implementation
detail and may be change at any time in response to operations on the
stream.

If the stream is fixed-sized (ie: no realloc was passed to
`MemoryOutputStream::new`) then this is the maximum size of the
stream and further writes will return `IOErrorEnum::NoSpace`.

In any case, if you want the number of bytes currently written to the
stream, use `MemoryOutputStreamExt::get_data_size`.

# Returns

the number of bytes allocated for the data buffer
<!-- trait MemoryOutputStreamExt::fn steal_as_bytes -->
Returns data from the `self` as a `glib::Bytes`. `self` must be
closed before calling this function.

# Returns

the stream's data
<!-- trait MemoryOutputStreamExt::fn steal_data -->
Gets any loaded data from the `self`. Ownership of the data
is transferred to the caller; when no longer needed it must be
freed using the free function set in `self`'s
`MemoryOutputStream:destroy-function` property.

`self` must be closed before calling this function.

# Returns

the stream's data, or `None` if it has previously
 been stolen
<!-- trait MemoryOutputStreamExt::fn get_property_data -->
Pointer to buffer where data will be written.
<!-- trait MemoryOutputStreamExt::fn set_property_data -->
Pointer to buffer where data will be written.
<!-- trait MemoryOutputStreamExt::fn get_property_data_size -->
Size of data written to the buffer.
<!-- trait MemoryOutputStreamExt::fn get_property_destroy_function -->
Function called with the buffer as argument when the stream is destroyed.
<!-- trait MemoryOutputStreamExt::fn set_property_destroy_function -->
Function called with the buffer as argument when the stream is destroyed.
<!-- trait MemoryOutputStreamExt::fn get_property_realloc_function -->
Function with realloc semantics called to enlarge the buffer.
<!-- trait MemoryOutputStreamExt::fn set_property_realloc_function -->
Function with realloc semantics called to enlarge the buffer.
<!-- trait MemoryOutputStreamExt::fn get_property_size -->
Current size of the data buffer.
<!-- trait MemoryOutputStreamExt::fn set_property_size -->
Current size of the data buffer.
<!-- struct Menu -->
`Menu` is a simple implementation of `MenuModel`.
You populate a `Menu` by adding `MenuItem` instances to it.

There are some convenience functions to allow you to directly
add items (avoiding `MenuItem`) for the common cases. To add
a regular item, use `Menu::insert`. To add a section, use
`Menu::insert_section`. To add a submenu, use
`Menu::insert_submenu`.

# Implements

[`MenuModelExt`](trait.MenuModelExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl Menu::fn new -->
Creates a new `Menu`.

The new menu has no items.

# Returns

a new `Menu`
<!-- impl Menu::fn append -->
Convenience function for appending a normal menu item to the end of
`self`. Combine `MenuItem::new` and `Menu::insert_item` for a more
flexible alternative.
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`
<!-- impl Menu::fn append_item -->
Appends `item` to the end of `self`.

See `Menu::insert_item` for more information.
## `item`
a `MenuItem` to append
<!-- impl Menu::fn append_section -->
Convenience function for appending a section menu item to the end of
`self`. Combine `MenuItem::new_section` and `Menu::insert_item` for a
more flexible alternative.
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section
<!-- impl Menu::fn append_submenu -->
Convenience function for appending a submenu menu item to the end of
`self`. Combine `MenuItem::new_submenu` and `Menu::insert_item` for a
more flexible alternative.
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu
<!-- impl Menu::fn freeze -->
Marks `self` as frozen.

After the menu is frozen, it is an error to attempt to make any
changes to it. In effect this means that the `Menu` API must no
longer be used.

This function causes `MenuModelExt::is_mutable` to begin returning
`false`, which has some positive performance implications.
<!-- impl Menu::fn insert -->
Convenience function for inserting a normal menu item into `self`.
Combine `MenuItem::new` and `Menu::insert_item` for a more flexible
alternative.
## `position`
the position at which to insert the item
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`
<!-- impl Menu::fn insert_item -->
Inserts `item` into `self`.

The "insertion" is actually done by copying all of the attribute and
link values of `item` and using them to form a new item within `self`.
As such, `item` itself is not really inserted, but rather, a menu item
that is exactly the same as the one presently described by `item`.

This means that `item` is essentially useless after the insertion
occurs. Any changes you make to it are ignored unless it is inserted
again (at which point its updated values will be copied).

You should probably just free `item` once you're done.

There are many convenience functions to take care of common cases.
See `Menu::insert`, `Menu::insert_section` and
`Menu::insert_submenu` as well as "prepend" and "append" variants of
each of these functions.
## `position`
the position at which to insert the item
## `item`
the `MenuItem` to insert
<!-- impl Menu::fn insert_section -->
Convenience function for inserting a section menu item into `self`.
Combine `MenuItem::new_section` and `Menu::insert_item` for a more
flexible alternative.
## `position`
the position at which to insert the item
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section
<!-- impl Menu::fn insert_submenu -->
Convenience function for inserting a submenu menu item into `self`.
Combine `MenuItem::new_submenu` and `Menu::insert_item` for a more
flexible alternative.
## `position`
the position at which to insert the item
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu
<!-- impl Menu::fn prepend -->
Convenience function for prepending a normal menu item to the start
of `self`. Combine `MenuItem::new` and `Menu::insert_item` for a more
flexible alternative.
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`
<!-- impl Menu::fn prepend_item -->
Prepends `item` to the start of `self`.

See `Menu::insert_item` for more information.
## `item`
a `MenuItem` to prepend
<!-- impl Menu::fn prepend_section -->
Convenience function for prepending a section menu item to the start
of `self`. Combine `MenuItem::new_section` and `Menu::insert_item` for
a more flexible alternative.
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section
<!-- impl Menu::fn prepend_submenu -->
Convenience function for prepending a submenu menu item to the start
of `self`. Combine `MenuItem::new_submenu` and `Menu::insert_item` for
a more flexible alternative.
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu
<!-- impl Menu::fn remove -->
Removes an item from the menu.

`position` gives the index of the item to remove.

It is an error if position is not in range the range from 0 to one
less than the number of items in the menu.

It is not possible to remove items by identity since items are added
to the menu simply by copying their links and attributes (ie:
identity of the item itself is not preserved).
## `position`
the position of the item to remove
<!-- impl Menu::fn remove_all -->
Removes all items in the menu.
<!-- struct MenuAttributeIter -->
`MenuAttributeIter` is an opaque structure type. You must access it
using the functions below.

# Implements

[`MenuAttributeIterExt`](trait.MenuAttributeIterExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait MenuAttributeIterExt -->
Trait containing all `MenuAttributeIter` methods.

# Implementors

[`MenuAttributeIter`](struct.MenuAttributeIter.html)
<!-- trait MenuAttributeIterExt::fn get_name -->
Gets the name of the attribute at the current iterator position, as
a string.

The iterator is not advanced.

# Returns

the name of the attribute
<!-- trait MenuAttributeIterExt::fn get_next -->
This function combines `MenuAttributeIterExt::next` with
`MenuAttributeIterExt::get_name` and `MenuAttributeIterExt::get_value`.

First the iterator is advanced to the next (possibly first) attribute.
If that fails, then `false` is returned and there are no other
effects.

If successful, `name` and `value` are set to the name and value of the
attribute that has just been advanced to. At this point,
`MenuAttributeIterExt::get_name` and `MenuAttributeIterExt::get_value` will
return the same values again.

The value returned in `name` remains valid for as long as the iterator
remains at the current position. The value returned in `value` must
be unreffed using `glib::Variant::unref` when it is no longer in use.
## `out_name`
the type of the attribute
## `value`
the attribute value

# Returns

`true` on success, or `false` if there is no additional
 attribute
<!-- trait MenuAttributeIterExt::fn get_value -->
Gets the value of the attribute at the current iterator position.

The iterator is not advanced.

# Returns

the value of the current attribute
<!-- trait MenuAttributeIterExt::fn next -->
Attempts to advance the iterator to the next (possibly first)
attribute.

`true` is returned on success, or `false` if there are no more
attributes.

You must call this function when you first acquire the iterator
to advance it to the first attribute (and determine if the first
attribute exists at all).

# Returns

`true` on success, or `false` when there are no more attributes
<!-- struct MenuItem -->
`MenuItem` is an opaque structure type. You must access it using the
functions below.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl MenuItem::fn new -->
Creates a new `MenuItem`.

If `label` is non-`None` it is used to set the "label" attribute of the
new item.

If `detailed_action` is non-`None` it is used to set the "action" and
possibly the "target" attribute of the new item. See
`MenuItem::set_detailed_action` for more information.
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`

# Returns

a new `MenuItem`
<!-- impl MenuItem::fn new_from_model -->
Creates a `MenuItem` as an exact copy of an existing menu item in a
`MenuModel`.

`item_index` must be valid (ie: be sure to call
`MenuModelExt::get_n_items` first).
## `model`
a `MenuModel`
## `item_index`
the index of an item in `model`

# Returns

a new `MenuItem`.
<!-- impl MenuItem::fn new_section -->
Creates a new `MenuItem` representing a section.

This is a convenience API around `MenuItem::new` and
`MenuItem::set_section`.

The effect of having one menu appear as a section of another is
exactly as it sounds: the items from `section` become a direct part of
the menu that `menu_item` is added to.

Visual separation is typically displayed between two non-empty
sections. If `label` is non-`None` then it will be encorporated into
this visual indication. This allows for labeled subsections of a
menu.

As a simple example, consider a typical "Edit" menu from a simple
program. It probably contains an "Undo" and "Redo" item, followed by
a separator, followed by "Cut", "Copy" and "Paste".

This would be accomplished by creating three `Menu` instances. The
first would be populated with the "Undo" and "Redo" items, and the
second with the "Cut", "Copy" and "Paste" items. The first and
second menus would then be added as submenus of the third. In XML
format, this would look something like the following:

```text
<menu id='edit-menu'>
  <section>
    <item label='Undo'/>
    <item label='Redo'/>
  </section>
  <section>
    <item label='Cut'/>
    <item label='Copy'/>
    <item label='Paste'/>
  </section>
</menu>
```

The following example is exactly equivalent. It is more illustrative
of the exact relationship between the menus and items (keeping in
mind that the 'link' element defines a new menu that is linked to the
containing one). The style of the second example is more verbose and
difficult to read (and therefore not recommended except for the
purpose of understanding what is really going on).

```text
<menu id='edit-menu'>
  <item>
    <link name='section'>
      <item label='Undo'/>
      <item label='Redo'/>
    </link>
  </item>
  <item>
    <link name='section'>
      <item label='Cut'/>
      <item label='Copy'/>
      <item label='Paste'/>
    </link>
  </item>
</menu>
```
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section

# Returns

a new `MenuItem`
<!-- impl MenuItem::fn new_submenu -->
Creates a new `MenuItem` representing a submenu.

This is a convenience API around `MenuItem::new` and
`MenuItem::set_submenu`.
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu

# Returns

a new `MenuItem`
<!-- impl MenuItem::fn get_attribute -->
Queries the named `attribute` on `self`.

If the attribute exists and matches the `glib::VariantType` corresponding
to `format_string` then `format_string` is used to deconstruct the
value into the positional parameters and `true` is returned.

If the attribute does not exist, or it does exist but has the wrong
type, then the positional parameters are ignored and `false` is
returned.
## `attribute`
the attribute name to query
## `format_string`
a `glib::Variant` format string

# Returns

`true` if the named attribute was found with the expected
 type
<!-- impl MenuItem::fn get_attribute_value -->
Queries the named `attribute` on `self`.

If `expected_type` is specified and the attribute does not have this
type, `None` is returned. `None` is also returned if the attribute
simply does not exist.
## `attribute`
the attribute name to query
## `expected_type`
the expected type of the attribute

# Returns

the attribute value, or `None`
<!-- impl MenuItem::fn get_link -->
Queries the named `link` on `self`.
## `link`
the link name to query

# Returns

the link, or `None`
<!-- impl MenuItem::fn set_action_and_target -->
Sets or unsets the "action" and "target" attributes of `self`.

If `action` is `None` then both the "action" and "target" attributes
are unset (and `format_string` is ignored along with the positional
parameters).

If `action` is non-`None` then the "action" attribute is set.
`format_string` is then inspected. If it is non-`None` then the proper
position parameters are collected to create a `glib::Variant` instance to
use as the target value. If it is `None` then the positional
parameters are ignored and the "target" attribute is unset.

See also `MenuItem::set_action_and_target_value` for an equivalent
call that directly accepts a `glib::Variant`. See
`MenuItem::set_detailed_action` for a more convenient version that
works with string-typed targets.

See also `MenuItem::set_action_and_target_value` for a
description of the semantics of the action and target attributes.
## `action`
the name of the action for this item
## `format_string`
a GVariant format string
<!-- impl MenuItem::fn set_action_and_target_value -->
Sets or unsets the "action" and "target" attributes of `self`.

If `action` is `None` then both the "action" and "target" attributes
are unset (and `target_value` is ignored).

If `action` is non-`None` then the "action" attribute is set. The
"target" attribute is then set to the value of `target_value` if it is
non-`None` or unset otherwise.

Normal menu items (ie: not submenu, section or other custom item
types) are expected to have the "action" attribute set to identify
the action that they are associated with. The state type of the
action help to determine the disposition of the menu item. See
`Action` and `ActionGroup` for an overview of actions.

In general, clicking on the menu item will result in activation of
the named action with the "target" attribute given as the parameter
to the action invocation. If the "target" attribute is not set then
the action is invoked with no parameter.

If the action has no state then the menu item is usually drawn as a
plain menu item (ie: with no additional decoration).

If the action has a boolean state then the menu item is usually drawn
as a toggle menu item (ie: with a checkmark or equivalent
indication). The item should be marked as 'toggled' or 'checked'
when the boolean state is `true`.

If the action has a string state then the menu item is usually drawn
as a radio menu item (ie: with a radio bullet or equivalent
indication). The item should be marked as 'selected' when the string
state is equal to the value of the `target` property.

See `MenuItem::set_action_and_target` or
`MenuItem::set_detailed_action` for two equivalent calls that are
probably more convenient for most uses.
## `action`
the name of the action for this item
## `target_value`
a `glib::Variant` to use as the action target
<!-- impl MenuItem::fn set_attribute -->
Sets or unsets an attribute on `self`.

The attribute to set or unset is specified by `attribute`. This
can be one of the standard attribute names `G_MENU_ATTRIBUTE_LABEL`,
`G_MENU_ATTRIBUTE_ACTION`, `G_MENU_ATTRIBUTE_TARGET`, or a custom
attribute name.
Attribute names are restricted to lowercase characters, numbers
and '-'. Furthermore, the names must begin with a lowercase character,
must not end with a '-', and must not contain consecutive dashes.

If `format_string` is non-`None` then the proper position parameters
are collected to create a `glib::Variant` instance to use as the attribute
value. If it is `None` then the positional parameterrs are ignored
and the named attribute is unset.

See also `MenuItem::set_attribute_value` for an equivalent call
that directly accepts a `glib::Variant`.
## `attribute`
the attribute to set
## `format_string`
a `glib::Variant` format string, or `None`
<!-- impl MenuItem::fn set_attribute_value -->
Sets or unsets an attribute on `self`.

The attribute to set or unset is specified by `attribute`. This
can be one of the standard attribute names `G_MENU_ATTRIBUTE_LABEL`,
`G_MENU_ATTRIBUTE_ACTION`, `G_MENU_ATTRIBUTE_TARGET`, or a custom
attribute name.
Attribute names are restricted to lowercase characters, numbers
and '-'. Furthermore, the names must begin with a lowercase character,
must not end with a '-', and must not contain consecutive dashes.

must consist only of lowercase
ASCII characters, digits and '-'.

If `value` is non-`None` then it is used as the new value for the
attribute. If `value` is `None` then the attribute is unset. If
the `value` `glib::Variant` is floating, it is consumed.

See also `MenuItem::set_attribute` for a more convenient way to do
the same.
## `attribute`
the attribute to set
## `value`
a `glib::Variant` to use as the value, or `None`
<!-- impl MenuItem::fn set_detailed_action -->
Sets the "action" and possibly the "target" attribute of `self`.

The format of `detailed_action` is the same format parsed by
`Action::parse_detailed_name`.

See `MenuItem::set_action_and_target` or
`MenuItem::set_action_and_target_value` for more flexible (but
slightly less convenient) alternatives.

See also `MenuItem::set_action_and_target_value` for a description of
the semantics of the action and target attributes.
## `detailed_action`
the "detailed" action string
<!-- impl MenuItem::fn set_icon -->
Sets (or unsets) the icon on `self`.

This call is the same as calling `Icon::serialize` and using the
result as the value to `MenuItem::set_attribute_value` for
`G_MENU_ATTRIBUTE_ICON`.

This API is only intended for use with "noun" menu items; things like
bookmarks or applications in an "Open With" menu. Don't use it on
menu items corresponding to verbs (eg: stock icons for 'Save' or
'Quit').

If `icon` is `None` then the icon is unset.
## `icon`
a `Icon`, or `None`
<!-- impl MenuItem::fn set_label -->
Sets or unsets the "label" attribute of `self`.

If `label` is non-`None` it is used as the label for the menu item. If
it is `None` then the label attribute is unset.
## `label`
the label to set, or `None` to unset
<!-- impl MenuItem::fn set_link -->
Creates a link from `self` to `model` if non-`None`, or unsets it.

Links are used to establish a relationship between a particular menu
item and another menu. For example, `G_MENU_LINK_SUBMENU` is used to
associate a submenu with a particular menu item, and `G_MENU_LINK_SECTION`
is used to create a section. Other types of link can be used, but there
is no guarantee that clients will be able to make sense of them.
Link types are restricted to lowercase characters, numbers
and '-'. Furthermore, the names must begin with a lowercase character,
must not end with a '-', and must not contain consecutive dashes.
## `link`
type of link to establish or unset
## `model`
the `MenuModel` to link to (or `None` to unset)
<!-- impl MenuItem::fn set_section -->
Sets or unsets the "section" link of `self` to `section`.

The effect of having one menu appear as a section of another is
exactly as it sounds: the items from `section` become a direct part of
the menu that `self` is added to. See `MenuItem::new_section`
for more information about what it means for a menu item to be a
section.
## `section`
a `MenuModel`, or `None`
<!-- impl MenuItem::fn set_submenu -->
Sets or unsets the "submenu" link of `self` to `submenu`.

If `submenu` is non-`None`, it is linked to. If it is `None` then the
link is unset.

The effect of having one menu appear as a submenu of another is
exactly as it sounds.
## `submenu`
a `MenuModel`, or `None`
<!-- struct MenuLinkIter -->
`MenuLinkIter` is an opaque structure type. You must access it using
the functions below.

# Implements

[`MenuLinkIterExt`](trait.MenuLinkIterExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait MenuLinkIterExt -->
Trait containing all `MenuLinkIter` methods.

# Implementors

[`MenuLinkIter`](struct.MenuLinkIter.html)
<!-- trait MenuLinkIterExt::fn get_name -->
Gets the name of the link at the current iterator position.

The iterator is not advanced.

# Returns

the type of the link
<!-- trait MenuLinkIterExt::fn get_next -->
This function combines `MenuLinkIterExt::next` with
`MenuLinkIterExt::get_name` and `MenuLinkIterExt::get_value`.

First the iterator is advanced to the next (possibly first) link.
If that fails, then `false` is returned and there are no other effects.

If successful, `out_link` and `value` are set to the name and `MenuModel`
of the link that has just been advanced to. At this point,
`MenuLinkIterExt::get_name` and `MenuLinkIterExt::get_value` will return the
same values again.

The value returned in `out_link` remains valid for as long as the iterator
remains at the current position. The value returned in `value` must
be unreffed using `gobject::ObjectExt::unref` when it is no longer in use.
## `out_link`
the name of the link
## `value`
the linked `MenuModel`

# Returns

`true` on success, or `false` if there is no additional link
<!-- trait MenuLinkIterExt::fn get_value -->
Gets the linked `MenuModel` at the current iterator position.

The iterator is not advanced.

# Returns

the `MenuModel` that is linked to
<!-- trait MenuLinkIterExt::fn next -->
Attempts to advance the iterator to the next (possibly first)
link.

`true` is returned on success, or `false` if there are no more links.

You must call this function when you first acquire the iterator to
advance it to the first link (and determine if the first link exists
at all).

# Returns

`true` on success, or `false` when there are no more links
<!-- struct MenuModel -->
`MenuModel` represents the contents of a menu -- an ordered list of
menu items. The items are associated with actions, which can be
activated through them. Items can be grouped in sections, and may
have submenus associated with them. Both items and sections usually
have some representation data, such as labels or icons. The type of
the associated action (ie whether it is stateful, and what kind of
state it has) can influence the representation of the item.

The conceptual model of menus in `MenuModel` is hierarchical:
sections and submenus are again represented by `GMenuModels`.
Menus themselves do not define their own roles. Rather, the role
of a particular `MenuModel` is defined by the item that references
it (or, in the case of the 'root' menu, is defined by the context
in which it is used).

As an example, consider the visible portions of this menu:

## An example menu # {`menu`-example}

![](menu-example.png)

There are 8 "menus" visible in the screenshot: one menubar, two
submenus and 5 sections:

- the toplevel menubar (containing 4 items)
- the View submenu (containing 3 sections)
- the first section of the View submenu (containing 2 items)
- the second section of the View submenu (containing 1 item)
- the final section of the View submenu (containing 1 item)
- the Highlight Mode submenu (containing 2 sections)
- the Sources section (containing 2 items)
- the Markup section (containing 2 items)

The [example][menu-model] illustrates the conceptual connection between
these 8 menus. Each large block in the figure represents a menu and the
smaller blocks within the large block represent items in that menu. Some
items contain references to other menus.

## A menu example # {`menu`-model}

![](menu-model.png)

Notice that the separators visible in the [example][menu-example]
appear nowhere in the [menu model][menu-model]. This is because
separators are not explicitly represented in the menu model. Instead,
a separator is inserted between any two non-empty sections of a menu.
Section items can have labels just like any other item. In that case,
a display system may show a section header instead of a separator.

The motivation for this abstract model of application controls is
that modern user interfaces tend to make these controls available
outside the application. Examples include global menus, jumplists,
dash boards, etc. To support such uses, it is necessary to 'export'
information about actions and their representation in menus, which
is exactly what the [GActionGroup exporter][gio-GActionGroup-exporter]
and the [GMenuModel exporter][gio-GMenuModel-exporter] do for
`ActionGroup` and `MenuModel`. The client-side counterparts to
make use of the exported information are `DBusActionGroup` and
`DBusMenuModel`.

The API of `MenuModel` is very generic, with iterators for the
attributes and links of an item, see `MenuModelExt::iterate_item_attributes`
and `MenuModelExt::iterate_item_links`. The 'standard' attributes and
link types have predefined names: `G_MENU_ATTRIBUTE_LABEL`,
`G_MENU_ATTRIBUTE_ACTION`, `G_MENU_ATTRIBUTE_TARGET`, `G_MENU_LINK_SECTION`
and `G_MENU_LINK_SUBMENU`.

Items in a `MenuModel` represent active controls if they refer to
an action that can get activated when the user interacts with the
menu item. The reference to the action is encoded by the string id
in the `G_MENU_ATTRIBUTE_ACTION` attribute. An action id uniquely
identifies an action in an action group. Which action group(s) provide
actions depends on the context in which the menu model is used.
E.g. when the model is exported as the application menu of a
``GtkApplication``, actions can be application-wide or window-specific
(and thus come from two different action groups). By convention, the
application-wide actions have names that start with "app.", while the
names of window-specific actions start with "win.".

While a wide variety of stateful actions is possible, the following
is the minimum that is expected to be supported by all users of exported
menu information:
- an action with no parameter type and no state
- an action with no parameter type and boolean state
- an action with string parameter type and string state

## Stateless

A stateless action typically corresponds to an ordinary menu item.

Selecting such a menu item will activate the action (with no parameter).

## Boolean State

An action with a boolean state will most typically be used with a "toggle"
or "switch" menu item. The state can be set directly, but activating the
action (with no parameter) results in the state being toggled.

Selecting a toggle menu item will activate the action. The menu item should
be rendered as "checked" when the state is true.

## String Parameter and State

Actions with string parameters and state will most typically be used to
represent an enumerated choice over the items available for a group of
radio menu items. Activating the action with a string parameter is
equivalent to setting that parameter as the state.

Radio menu items, in addition to being associated with the action, will
have a target value. Selecting that menu item will result in activation
of the action with the target value as the parameter. The menu item should
be rendered as "selected" when the state of the action is equal to the
target value of the menu item.

# Implements

[`MenuModelExt`](trait.MenuModelExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait MenuModelExt -->
Trait containing all `MenuModel` methods.

# Implementors

[`MenuModel`](struct.MenuModel.html), [`Menu`](struct.Menu.html)
<!-- trait MenuModelExt::fn get_item_attribute -->
Queries item at position `item_index` in `self` for the attribute
specified by `attribute`.

If the attribute exists and matches the `glib::VariantType` corresponding
to `format_string` then `format_string` is used to deconstruct the
value into the positional parameters and `true` is returned.

If the attribute does not exist, or it does exist but has the wrong
type, then the positional parameters are ignored and `false` is
returned.

This function is a mix of `MenuModelExt::get_item_attribute_value` and
`glib::Variant::get`, followed by a `glib::Variant::unref`. As such,
`format_string` must make a complete copy of the data (since the
`glib::Variant` may go away after the call to `glib::Variant::unref`). In
particular, no '&' characters are allowed in `format_string`.
## `item_index`
the index of the item
## `attribute`
the attribute to query
## `format_string`
a `glib::Variant` format string

# Returns

`true` if the named attribute was found with the expected
 type
<!-- trait MenuModelExt::fn get_item_attribute_value -->
Queries the item at position `item_index` in `self` for the attribute
specified by `attribute`.

If `expected_type` is non-`None` then it specifies the expected type of
the attribute. If it is `None` then any type will be accepted.

If the attribute exists and matches `expected_type` (or if the
expected type is unspecified) then the value is returned.

If the attribute does not exist, or does not match the expected type
then `None` is returned.
## `item_index`
the index of the item
## `attribute`
the attribute to query
## `expected_type`
the expected type of the attribute, or
 `None`

# Returns

the value of the attribute
<!-- trait MenuModelExt::fn get_item_link -->
Queries the item at position `item_index` in `self` for the link
specified by `link`.

If the link exists, the linked `MenuModel` is returned. If the link
does not exist, `None` is returned.
## `item_index`
the index of the item
## `link`
the link to query

# Returns

the linked `MenuModel`, or `None`
<!-- trait MenuModelExt::fn get_n_items -->
Query the number of items in `self`.

# Returns

the number of items
<!-- trait MenuModelExt::fn is_mutable -->
Queries if `self` is mutable.

An immutable `MenuModel` will never emit the `MenuModel::items-changed`
signal. Consumers of the model may make optimisations accordingly.

# Returns

`true` if the model is mutable (ie: "items-changed" may be
 emitted).
<!-- trait MenuModelExt::fn items_changed -->
Requests emission of the `MenuModel::items-changed` signal on `self`.

This function should never be called except by `MenuModel`
subclasses. Any other calls to this function will very likely lead
to a violation of the interface of the model.

The implementation should update its internal representation of the
menu before emitting the signal. The implementation should further
expect to receive queries about the new state of the menu (and
particularly added menu items) while signal handlers are running.

The implementation must dispatch this call directly from a mainloop
entry and not in response to calls -- particularly those from the
`MenuModel` API. Said another way: the menu must not change while
user code is running without returning to the mainloop.
## `position`
the position of the change
## `removed`
the number of items removed
## `added`
the number of items added
<!-- trait MenuModelExt::fn iterate_item_attributes -->
Creates a `MenuAttributeIter` to iterate over the attributes of
the item at position `item_index` in `self`.

You must free the iterator with `gobject::ObjectExt::unref` when you are done.
## `item_index`
the index of the item

# Returns

a new `MenuAttributeIter`
<!-- trait MenuModelExt::fn iterate_item_links -->
Creates a `MenuLinkIter` to iterate over the links of the item at
position `item_index` in `self`.

You must free the iterator with `gobject::ObjectExt::unref` when you are done.
## `item_index`
the index of the item

# Returns

a new `MenuLinkIter`
<!-- trait MenuModelExt::fn connect_items_changed -->
Emitted when a change has occured to the menu.

The only changes that can occur to a menu is that items are removed
or added. Items may not change (except by being removed and added
back in the same location). This signal is capable of describing
both of those changes (at the same time).

The signal means that starting at the index `position`, `removed`
items were removed and `added` items were added in their place. If
`removed` is zero then only items were added. If `added` is zero
then only items were removed.

As an example, if the menu contains items a, b, c, d (in that
order) and the signal (2, 1, 3) occurs then the new composition of
the menu will be a, b, _, _, _, d (with each _ representing some
new item).

Signal handlers may query the model (particularly the added items)
and expect to see the results of the modification that is being
reported. The signal is emitted after the modification.
## `position`
the position of the change
## `removed`
the number of items removed
## `added`
the number of items added
<!-- struct Mount -->
The `Mount` interface represents user-visible mounts. Note, when
porting from GnomeVFS, `Mount` is the moral equivalent of `GnomeVFSVolume`.

`Mount` is a "mounted" filesystem that you can access. Mounted is in
quotes because it's not the same as a unix mount, it might be a gvfs
mount, but you can still access the files on it if you use GIO. Might or
might not be related to a volume object.

Unmounting a `Mount` instance is an asynchronous operation. For
more information about asynchronous operations, see `AsyncResult`
and `Task`. To unmount a `Mount` instance, first call
`Mount::unmount_with_operation` with (at least) the `Mount` instance and a
`GAsyncReadyCallback`. The callback will be fired when the
operation has resolved (either with success or failure), and a
`AsyncResult` structure will be passed to the callback. That
callback should then call `Mount::unmount_with_operation_finish` with the `Mount`
and the `AsyncResult` data to see if the operation was completed
successfully. If an `error` is present when `Mount::unmount_with_operation_finish`
is called, then it will be filled with any error information.

# Implements

[`MountExt`](trait.MountExt.html)
<!-- trait MountExt -->
Trait containing all `Mount` methods.

# Implementors

[`Mount`](struct.Mount.html)
<!-- trait MountExt::fn can_eject -->
Checks if `self` can be ejected.

# Returns

`true` if the `self` can be ejected.
<!-- trait MountExt::fn can_unmount -->
Checks if `self` can be unmounted.

# Returns

`true` if the `self` can be unmounted.
<!-- trait MountExt::fn eject_with_operation -->
Ejects a mount. This is an asynchronous operation, and is
finished by calling `Mount::eject_with_operation_finish` with the `self`
and `AsyncResult` data returned in the `callback`.
## `flags`
flags affecting the unmount if required for eject
## `mount_operation`
a `MountOperation` or `None` to avoid
 user interaction.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data passed to `callback`.
<!-- trait MountExt::fn eject_with_operation_finish -->
Finishes ejecting a mount. If any errors occurred during the operation,
`error` will be set to contain the errors and `false` will be returned.
## `result`
a `AsyncResult`.

# Returns

`true` if the mount was successfully ejected. `false` otherwise.
<!-- trait MountExt::fn get_default_location -->
Gets the default location of `self`. The default location of the given
`self` is a path that reflects the main entry point for the user (e.g.
the home directory, or the root of the volume).

# Returns

a `File`.
 The returned object should be unreffed with
 `gobject::ObjectExt::unref` when no longer needed.
<!-- trait MountExt::fn get_drive -->
Gets the drive for the `self`.

This is a convenience method for getting the `Volume` and then
using that object to get the `Drive`.

# Returns

a `Drive` or `None` if `self` is not
 associated with a volume or a drive.
 The returned object should be unreffed with
 `gobject::ObjectExt::unref` when no longer needed.
<!-- trait MountExt::fn get_icon -->
Gets the icon for `self`.

# Returns

a `Icon`.
 The returned object should be unreffed with
 `gobject::ObjectExt::unref` when no longer needed.
<!-- trait MountExt::fn get_name -->
Gets the name of `self`.

# Returns

the name for the given `self`.
 The returned string should be freed with `g_free`
 when no longer needed.
<!-- trait MountExt::fn get_root -->
Gets the root directory on `self`.

# Returns

a `File`.
 The returned object should be unreffed with
 `gobject::ObjectExt::unref` when no longer needed.
<!-- trait MountExt::fn get_sort_key -->
Gets the sort key for `self`, if any.

# Returns

Sorting key for `self` or `None` if no such key is available.
<!-- trait MountExt::fn get_symbolic_icon -->
Gets the symbolic icon for `self`.

# Returns

a `Icon`.
 The returned object should be unreffed with
 `gobject::ObjectExt::unref` when no longer needed.
<!-- trait MountExt::fn get_uuid -->
Gets the UUID for the `self`. The reference is typically based on
the file system UUID for the mount in question and should be
considered an opaque string. Returns `None` if there is no UUID
available.

# Returns

the UUID for `self` or `None` if no UUID
 can be computed.
 The returned string should be freed with `g_free`
 when no longer needed.
<!-- trait MountExt::fn get_volume -->
Gets the volume for the `self`.

# Returns

a `Volume` or `None` if `self` is not
 associated with a volume.
 The returned object should be unreffed with
 `gobject::ObjectExt::unref` when no longer needed.
<!-- trait MountExt::fn guess_content_type -->
Tries to guess the type of content stored on `self`. Returns one or
more textual identifiers of well-known content types (typically
prefixed with "x-content/"), e.g. x-content/image-dcf for camera
memory cards. See the
[shared-mime-info](http://www.freedesktop.org/wiki/Specifications/shared-mime-info-spec)
specification for more on x-content types.

This is an asynchronous operation (see
`Mount::guess_content_type_sync` for the synchronous version), and
is finished by calling `Mount::guess_content_type_finish` with the
`self` and `AsyncResult` data returned in the `callback`.
## `force_rescan`
Whether to force a rescan of the content.
 Otherwise a cached result will be used if available
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data passed to `callback`
<!-- trait MountExt::fn guess_content_type_finish -->
Finishes guessing content types of `self`. If any errors occurred
during the operation, `error` will be set to contain the errors and
`false` will be returned. In particular, you may get an
`IOErrorEnum::NotSupported` if the mount does not support content
guessing.
## `result`
a `AsyncResult`

# Returns

a `None`-terminated array of content types or `None` on error.
 Caller should free this array with `g_strfreev` when done with it.
<!-- trait MountExt::fn guess_content_type_sync -->
Tries to guess the type of content stored on `self`. Returns one or
more textual identifiers of well-known content types (typically
prefixed with "x-content/"), e.g. x-content/image-dcf for camera
memory cards. See the
[shared-mime-info](http://www.freedesktop.org/wiki/Specifications/shared-mime-info-spec)
specification for more on x-content types.

This is an synchronous operation and as such may block doing IO;
see `Mount::guess_content_type` for the asynchronous version.
## `force_rescan`
Whether to force a rescan of the content.
 Otherwise a cached result will be used if available
## `cancellable`
optional `Cancellable` object, `None` to ignore

# Returns

a `None`-terminated array of content types or `None` on error.
 Caller should free this array with `g_strfreev` when done with it.
<!-- trait MountExt::fn is_shadowed -->
Determines if `self` is shadowed. Applications or libraries should
avoid displaying `self` in the user interface if it is shadowed.

A mount is said to be shadowed if there exists one or more user
visible objects (currently `Mount` objects) with a root that is
inside the root of `self`.

One application of shadow mounts is when exposing a single file
system that is used to address several logical volumes. In this
situation, a `VolumeMonitor` implementation would create two
`Volume` objects (for example, one for the camera functionality of
the device and one for a SD card reader on the device) with
activation URIs `gphoto2://[usb:001,002]/store1/`
and `gphoto2://[usb:001,002]/store2/`. When the
underlying mount (with root
`gphoto2://[usb:001,002]/`) is mounted, said
`VolumeMonitor` implementation would create two `Mount` objects
(each with their root matching the corresponding volume activation
root) that would shadow the original mount.

The proxy monitor in GVfs 2.26 and later, automatically creates and
manage shadow mounts (and shadows the underlying mount) if the
activation root on a `Volume` is set.

# Returns

`true` if `self` is shadowed.
<!-- trait MountExt::fn remount -->
Remounts a mount. This is an asynchronous operation, and is
finished by calling `Mount::remount_finish` with the `self`
and `GAsyncResults` data returned in the `callback`.

Remounting is useful when some setting affecting the operation
of the volume has been changed, as these may need a remount to
take affect. While this is semantically equivalent with unmounting
and then remounting not all backends might need to actually be
unmounted.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation` or `None` to avoid
 user interaction.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data passed to `callback`.
<!-- trait MountExt::fn remount_finish -->
Finishes remounting a mount. If any errors occurred during the operation,
`error` will be set to contain the errors and `false` will be returned.
## `result`
a `AsyncResult`.

# Returns

`true` if the mount was successfully remounted. `false` otherwise.
<!-- trait MountExt::fn shadow -->
Increments the shadow count on `self`. Usually used by
`VolumeMonitor` implementations when creating a shadow mount for
`self`, see `Mount::is_shadowed` for more information. The caller
will need to emit the `Mount::changed` signal on `self` manually.
<!-- trait MountExt::fn unmount_with_operation -->
Unmounts a mount. This is an asynchronous operation, and is
finished by calling `Mount::unmount_with_operation_finish` with the `self`
and `AsyncResult` data returned in the `callback`.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation` or `None` to avoid
 user interaction.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`, or `None`.
## `user_data`
user data passed to `callback`.
<!-- trait MountExt::fn unmount_with_operation_finish -->
Finishes unmounting a mount. If any errors occurred during the operation,
`error` will be set to contain the errors and `false` will be returned.
## `result`
a `AsyncResult`.

# Returns

`true` if the mount was successfully unmounted. `false` otherwise.
<!-- trait MountExt::fn unshadow -->
Decrements the shadow count on `self`. Usually used by
`VolumeMonitor` implementations when destroying a shadow mount for
`self`, see `Mount::is_shadowed` for more information. The caller
will need to emit the `Mount::changed` signal on `self` manually.
<!-- trait MountExt::fn connect_changed -->
Emitted when the mount has been changed.
<!-- trait MountExt::fn connect_pre_unmount -->
This signal may be emitted when the `Mount` is about to be
unmounted.

This signal depends on the backend and is only emitted if
GIO was used to unmount.
<!-- trait MountExt::fn connect_unmounted -->
This signal is emitted when the `Mount` have been
unmounted. If the recipient is holding references to the
object they should release them so the object can be
finalized.
<!-- struct MountOperation -->
`MountOperation` provides a mechanism for interacting with the user.
It can be used for authenticating mountable operations, such as loop
mounting files, hard drive partitions or server locations. It can
also be used to ask the user questions or show a list of applications
preventing unmount or eject operations from completing.

Note that `MountOperation` is used for more than just `Mount`
objects – for example it is also used in `Drive::start` and
`Drive::stop`.

Users should instantiate a subclass of this that implements all the
various callbacks to show the required dialogs, such as
``GtkMountOperation``. If no user interaction is desired (for example
when automounting filesystems at login time), usually `None` can be
passed, see each method taking a `MountOperation` for details.

The term ‘TCRYPT’ is used to mean ‘compatible with TrueCrypt and VeraCrypt’.
[TrueCrypt](https://en.wikipedia.org/wiki/TrueCrypt) is a discontinued system for
encrypting file containers, partitions or whole disks, typically used with Windows.
[VeraCrypt](https://www.veracrypt.fr/) is a maintained fork of TrueCrypt with various
improvements and auditing fixes.

# Implements

[`MountOperationExt`](trait.MountOperationExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait MountOperationExt -->
Trait containing all `MountOperation` methods.

# Implementors

[`MountOperation`](struct.MountOperation.html)
<!-- impl MountOperation::fn new -->
Creates a new mount operation.

# Returns

a `MountOperation`.
<!-- trait MountOperationExt::fn get_anonymous -->
Check to see whether the mount operation is being used
for an anonymous user.

# Returns

`true` if mount operation is anonymous.
<!-- trait MountOperationExt::fn get_choice -->
Gets a choice from the mount operation.

# Returns

an integer containing an index of the user's choice from
the choice's list, or `0`.
<!-- trait MountOperationExt::fn get_domain -->
Gets the domain of the mount operation.

# Returns

a string set to the domain.
<!-- trait MountOperationExt::fn get_is_tcrypt_hidden_volume -->
Check to see whether the mount operation is being used
for a TCRYPT hidden volume.

Feature: `v2_58`


# Returns

`true` if mount operation is for hidden volume.
<!-- trait MountOperationExt::fn get_is_tcrypt_system_volume -->
Check to see whether the mount operation is being used
for a TCRYPT system volume.

Feature: `v2_58`


# Returns

`true` if mount operation is for system volume.
<!-- trait MountOperationExt::fn get_password -->
Gets a password from the mount operation.

# Returns

a string containing the password within `self`.
<!-- trait MountOperationExt::fn get_password_save -->
Gets the state of saving passwords for the mount operation.

# Returns

a `PasswordSave` flag.
<!-- trait MountOperationExt::fn get_pim -->
Gets a PIM from the mount operation.

Feature: `v2_58`


# Returns

The VeraCrypt PIM within `self`.
<!-- trait MountOperationExt::fn get_username -->
Get the user name from the mount operation.

# Returns

a string containing the user name.
<!-- trait MountOperationExt::fn reply -->
Emits the `MountOperation::reply` signal.
## `result`
a `MountOperationResult`
<!-- trait MountOperationExt::fn set_anonymous -->
Sets the mount operation to use an anonymous user if `anonymous` is `true`.
## `anonymous`
boolean value.
<!-- trait MountOperationExt::fn set_choice -->
Sets a default choice for the mount operation.
## `choice`
an integer.
<!-- trait MountOperationExt::fn set_domain -->
Sets the mount operation's domain.
## `domain`
the domain to set.
<!-- trait MountOperationExt::fn set_is_tcrypt_hidden_volume -->
Sets the mount operation to use a hidden volume if `hidden_volume` is `true`.

Feature: `v2_58`

## `hidden_volume`
boolean value.
<!-- trait MountOperationExt::fn set_is_tcrypt_system_volume -->
Sets the mount operation to use a system volume if `system_volume` is `true`.

Feature: `v2_58`

## `system_volume`
boolean value.
<!-- trait MountOperationExt::fn set_password -->
Sets the mount operation's password to `password`.
## `password`
password to set.
<!-- trait MountOperationExt::fn set_password_save -->
Sets the state of saving passwords for the mount operation.
## `save`
a set of `PasswordSave` flags.
<!-- trait MountOperationExt::fn set_pim -->
Sets the mount operation's PIM to `pim`.

Feature: `v2_58`

## `pim`
an unsigned integer.
<!-- trait MountOperationExt::fn set_username -->
Sets the user name within `self` to `username`.
## `username`
input username.
<!-- trait MountOperationExt::fn connect_aborted -->
Emitted by the backend when e.g. a device becomes unavailable
while a mount operation is in progress.

Implementations of GMountOperation should handle this signal
by dismissing open password dialogs.
<!-- trait MountOperationExt::fn connect_ask_password -->
Emitted when a mount operation asks the user for a password.

If the message contains a line break, the first line should be
presented as a heading. For example, it may be used as the
primary text in a ``GtkMessageDialog``.
## `message`
string containing a message to display to the user.
## `default_user`
string containing the default user name.
## `default_domain`
string containing the default domain.
## `flags`
a set of `AskPasswordFlags`.
<!-- trait MountOperationExt::fn connect_ask_question -->
Emitted when asking the user a question and gives a list of
choices for the user to choose from.

If the message contains a line break, the first line should be
presented as a heading. For example, it may be used as the
primary text in a ``GtkMessageDialog``.
## `message`
string containing a message to display to the user.
## `choices`
an array of strings for each possible choice.
<!-- trait MountOperationExt::fn connect_reply -->
Emitted when the user has replied to the mount operation.
## `result`
a `MountOperationResult` indicating how the request was handled
<!-- trait MountOperationExt::fn connect_show_processes -->
Emitted when one or more processes are blocking an operation
e.g. unmounting/ejecting a `Mount` or stopping a `Drive`.

Note that this signal may be emitted several times to update the
list of blocking processes as processes close files. The
application should only respond with `MountOperationExt::reply` to
the latest signal (setting `MountOperation:choice` to the choice
the user made).

If the message contains a line break, the first line should be
presented as a heading. For example, it may be used as the
primary text in a ``GtkMessageDialog``.
## `message`
string containing a message to display to the user.
## `processes`
an array of `glib::Pid` for processes
 blocking the operation.
## `choices`
an array of strings for each possible choice.
<!-- trait MountOperationExt::fn connect_show_unmount_progress -->
Emitted when an unmount operation has been busy for more than some time
(typically 1.5 seconds).

When unmounting or ejecting a volume, the kernel might need to flush
pending data in its buffers to the volume stable storage, and this operation
can take a considerable amount of time. This signal may be emitted several
times as long as the unmount operation is outstanding, and then one
last time when the operation is completed, with `bytes_left` set to zero.

Implementations of GMountOperation should handle this signal by
showing an UI notification, and then dismiss it, or show another notification
of completion, when `bytes_left` reaches zero.

If the message contains a line break, the first line should be
presented as a heading. For example, it may be used as the
primary text in a ``GtkMessageDialog``.
## `message`
string containing a mesage to display to the user
## `time_left`
the estimated time left before the operation completes,
 in microseconds, or -1
## `bytes_left`
the amount of bytes to be written before the operation
 completes (or -1 if such amount is not known), or zero if the operation
 is completed
<!-- trait MountOperationExt::fn get_property_anonymous -->
Whether to use an anonymous user when authenticating.
<!-- trait MountOperationExt::fn set_property_anonymous -->
Whether to use an anonymous user when authenticating.
<!-- trait MountOperationExt::fn get_property_choice -->
The index of the user's choice when a question is asked during the
mount operation. See the `MountOperation::ask-question` signal.
<!-- trait MountOperationExt::fn set_property_choice -->
The index of the user's choice when a question is asked during the
mount operation. See the `MountOperation::ask-question` signal.
<!-- trait MountOperationExt::fn get_property_domain -->
The domain to use for the mount operation.
<!-- trait MountOperationExt::fn set_property_domain -->
The domain to use for the mount operation.
<!-- trait MountOperationExt::fn get_property_is_tcrypt_hidden_volume -->
Whether the device to be unlocked is a TCRYPT hidden volume.
See https://www.veracrypt.fr/en/Hidden`20Volume.html`.

Feature: `v2_58`

<!-- trait MountOperationExt::fn set_property_is_tcrypt_hidden_volume -->
Whether the device to be unlocked is a TCRYPT hidden volume.
See https://www.veracrypt.fr/en/Hidden`20Volume.html`.

Feature: `v2_58`

<!-- trait MountOperationExt::fn get_property_is_tcrypt_system_volume -->
Whether the device to be unlocked is a TCRYPT system volume.
In this context, a system volume is a volume with a bootloader
and operating system installed. This is only supported for Windows
operating systems. For further documentation, see
https://www.veracrypt.fr/en/System`20Encryption.html`.

Feature: `v2_58`

<!-- trait MountOperationExt::fn set_property_is_tcrypt_system_volume -->
Whether the device to be unlocked is a TCRYPT system volume.
In this context, a system volume is a volume with a bootloader
and operating system installed. This is only supported for Windows
operating systems. For further documentation, see
https://www.veracrypt.fr/en/System`20Encryption.html`.

Feature: `v2_58`

<!-- trait MountOperationExt::fn get_property_password -->
The password that is used for authentication when carrying out
the mount operation.
<!-- trait MountOperationExt::fn set_property_password -->
The password that is used for authentication when carrying out
the mount operation.
<!-- trait MountOperationExt::fn get_property_password_save -->
Determines if and how the password information should be saved.
<!-- trait MountOperationExt::fn set_property_password_save -->
Determines if and how the password information should be saved.
<!-- trait MountOperationExt::fn get_property_pim -->
The VeraCrypt PIM value, when unlocking a VeraCrypt volume. See
https://www.veracrypt.fr/en/Personal`20Iterations`%20Multiplier`20`(PIM).html.

Feature: `v2_58`

<!-- trait MountOperationExt::fn set_property_pim -->
The VeraCrypt PIM value, when unlocking a VeraCrypt volume. See
https://www.veracrypt.fr/en/Personal`20Iterations`%20Multiplier`20`(PIM).html.

Feature: `v2_58`

<!-- trait MountOperationExt::fn get_property_username -->
The user name that is used for authentication when carrying out
the mount operation.
<!-- trait MountOperationExt::fn set_property_username -->
The user name that is used for authentication when carrying out
the mount operation.
<!-- enum MountOperationResult -->
`MountOperationResult` is returned as a result when a request for
information is send by the mounting operation.
<!-- enum MountOperationResult::variant Handled -->
The request was fulfilled and the
 user specified data is now available
<!-- enum MountOperationResult::variant Aborted -->
The user requested the mount operation
 to be aborted
<!-- enum MountOperationResult::variant Unhandled -->
The request was unhandled (i.e. not
 implemented)
<!-- struct NetworkAddress -->
`NetworkAddress` provides an easy way to resolve a hostname and
then attempt to connect to that host, handling the possibility of
multiple IP addresses and multiple address families.

See `SocketConnectable` for and example of using the connectable
interface.

# Implements

[`NetworkAddressExt`](trait.NetworkAddressExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketConnectableExt`](trait.SocketConnectableExt.html)
<!-- trait NetworkAddressExt -->
Trait containing all `NetworkAddress` methods.

# Implementors

[`NetworkAddress`](struct.NetworkAddress.html)
<!-- impl NetworkAddress::fn new -->
Creates a new `SocketConnectable` for connecting to the given
`hostname` and `port`.

Note that depending on the configuration of the machine, a
`hostname` of `localhost` may refer to the IPv4 loopback address
only, or to both IPv4 and IPv6; use
`NetworkAddress::new_loopback` to create a `NetworkAddress` that
is guaranteed to resolve to both addresses.
## `hostname`
the hostname
## `port`
the port

# Returns

the new `NetworkAddress`
<!-- impl NetworkAddress::fn new_loopback -->
Creates a new `SocketConnectable` for connecting to the local host
over a loopback connection to the given `port`. This is intended for
use in connecting to local services which may be running on IPv4 or
IPv6.

The connectable will return IPv4 and IPv6 loopback addresses,
regardless of how the host resolves `localhost`. By contrast,
`NetworkAddress::new` will often only return an IPv4 address when
resolving `localhost`, and an IPv6 address for `localhost6`.

`NetworkAddressExt::get_hostname` will always return `localhost` for
`GNetworkAddresses` created with this constructor.

Feature: `v2_44`

## `port`
the port

# Returns

the new `NetworkAddress`
<!-- impl NetworkAddress::fn parse -->
Creates a new `SocketConnectable` for connecting to the given
`hostname` and `port`. May fail and return `None` in case
parsing `host_and_port` fails.

`host_and_port` may be in any of a number of recognised formats; an IPv6
address, an IPv4 address, or a domain name (in which case a DNS
lookup is performed). Quoting with [] is supported for all address
types. A port override may be specified in the usual way with a
colon.

If no port is specified in `host_and_port` then `default_port` will be
used as the port number to connect to.

In general, `host_and_port` is expected to be provided by the user
(allowing them to give the hostname, and a port override if necessary)
and `default_port` is expected to be provided by the application.

(The port component of `host_and_port` can also be specified as a
service name rather than as a numeric port, but this functionality
is deprecated, because it depends on the contents of /etc/services,
which is generally quite sparse on platforms other than Linux.)
## `host_and_port`
the hostname and optionally a port
## `default_port`
the default port if not in `host_and_port`

# Returns

the new
 `NetworkAddress`, or `None` on error
<!-- impl NetworkAddress::fn parse_uri -->
Creates a new `SocketConnectable` for connecting to the given
`uri`. May fail and return `None` in case parsing `uri` fails.

Using this rather than `NetworkAddress::new` or
`NetworkAddress::parse` allows `SocketClient` to determine
when to use application-specific proxy protocols.
## `uri`
the hostname and optionally a port
## `default_port`
The default port if none is found in the URI

# Returns

the new
 `NetworkAddress`, or `None` on error
<!-- trait NetworkAddressExt::fn get_hostname -->
Gets `self`'s hostname. This might be either UTF-8 or ASCII-encoded,
depending on what `self` was created with.

# Returns

`self`'s hostname
<!-- trait NetworkAddressExt::fn get_port -->
Gets `self`'s port number

# Returns

`self`'s port (which may be 0)
<!-- trait NetworkAddressExt::fn get_scheme -->
Gets `self`'s scheme

# Returns

`self`'s scheme (`None` if not built from URI)
<!-- enum NetworkConnectivity -->
The host's network connectivity state, as reported by `NetworkMonitor`.
<!-- enum NetworkConnectivity::variant Local -->
The host is not configured with a
 route to the Internet; it may or may not be connected to a local
 network.
<!-- enum NetworkConnectivity::variant Limited -->
The host is connected to a network, but
 does not appear to be able to reach the full Internet, perhaps
 due to upstream network problems.
<!-- enum NetworkConnectivity::variant Portal -->
The host is behind a captive portal and
 cannot reach the full Internet.
<!-- enum NetworkConnectivity::variant Full -->
The host is connected to a network, and
 appears to be able to reach the full Internet.

Feature: `v2_44`

<!-- struct NetworkMonitor -->
`NetworkMonitor` provides an easy-to-use cross-platform API
for monitoring network connectivity. On Linux, the available
implementations are based on the kernel's netlink interface and
on NetworkManager.

There is also an implementation for use inside Flatpak sandboxes.

# Implements

[`NetworkMonitorExt`](trait.NetworkMonitorExt.html)
<!-- trait NetworkMonitorExt -->
Trait containing all `NetworkMonitor` methods.

# Implementors

[`NetworkMonitor`](struct.NetworkMonitor.html)
<!-- impl NetworkMonitor::fn get_default -->
Gets the default `NetworkMonitor` for the system.

# Returns

a `NetworkMonitor`
<!-- trait NetworkMonitorExt::fn can_reach -->
Attempts to determine whether or not the host pointed to by
`connectable` can be reached, without actually trying to connect to
it.

This may return `true` even when `NetworkMonitor:network-available`
is `false`, if, for example, `self` can determine that
`connectable` refers to a host on a local network.

If `self` believes that an attempt to connect to `connectable`
will succeed, it will return `true`. Otherwise, it will return
`false` and set `error` to an appropriate error (such as
`IOErrorEnum::HostUnreachable`).

Note that although this does not attempt to connect to
`connectable`, it may still block for a brief period of time (eg,
trying to do multicast DNS on the local network), so if you do not
want to block, you should use `NetworkMonitor::can_reach_async`.
## `connectable`
a `SocketConnectable`
## `cancellable`
a `Cancellable`, or `None`

# Returns

`true` if `connectable` is reachable, `false` if not.
<!-- trait NetworkMonitorExt::fn can_reach_async -->
Asynchronously attempts to determine whether or not the host
pointed to by `connectable` can be reached, without actually
trying to connect to it.

For more details, see `NetworkMonitor::can_reach`.

When the operation is finished, `callback` will be called.
You can then call `NetworkMonitor::can_reach_finish`
to get the result of the operation.
## `connectable`
a `SocketConnectable`
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback` to call when the
 request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait NetworkMonitorExt::fn can_reach_finish -->
Finishes an async network connectivity test.
See `NetworkMonitor::can_reach_async`.
## `result`
a `AsyncResult`

# Returns

`true` if network is reachable, `false` if not.
<!-- trait NetworkMonitorExt::fn get_connectivity -->
Gets a more detailed networking state than
`NetworkMonitor::get_network_available`.

If `NetworkMonitor:network-available` is `false`, then the
connectivity state will be `NetworkConnectivity::Local`.

If `NetworkMonitor:network-available` is `true`, then the
connectivity state will be `NetworkConnectivity::Full` (if there
is full Internet connectivity), `NetworkConnectivity::Limited` (if
the host has a default route, but appears to be unable to actually
reach the full Internet), or `NetworkConnectivity::Portal` (if the
host is trapped behind a "captive portal" that requires some sort
of login or acknowledgement before allowing full Internet access).

Note that in the case of `NetworkConnectivity::Limited` and
`NetworkConnectivity::Portal`, it is possible that some sites are
reachable but others are not. In this case, applications can
attempt to connect to remote servers, but should gracefully fall
back to their "offline" behavior if the connection attempt fails.

Feature: `v2_44`


# Returns

the network connectivity state
<!-- trait NetworkMonitorExt::fn get_network_available -->
Checks if the network is available. "Available" here means that the
system has a default route available for at least one of IPv4 or
IPv6. It does not necessarily imply that the public Internet is
reachable. See `NetworkMonitor:network-available` for more details.

# Returns

whether the network is available
<!-- trait NetworkMonitorExt::fn get_network_metered -->
Checks if the network is metered.
See `NetworkMonitor:network-metered` for more details.

Feature: `v2_46`


# Returns

whether the connection is metered
<!-- trait NetworkMonitorExt::fn connect_network_changed -->
Emitted when the network configuration changes.
## `network_available`
the current value of `NetworkMonitor:network-available`
<!-- trait NetworkMonitorExt::fn get_property_connectivity -->
More detailed information about the host's network connectivity.
See `NetworkMonitor::get_connectivity` and
`NetworkConnectivity` for more details.

Feature: `v2_44`

<!-- trait NetworkMonitorExt::fn get_property_network_available -->
Whether the network is considered available. That is, whether the
system has a default route for at least one of IPv4 or IPv6.

Real-world networks are of course much more complicated than
this; the machine may be connected to a wifi hotspot that
requires payment before allowing traffic through, or may be
connected to a functioning router that has lost its own upstream
connectivity. Some hosts might only be accessible when a VPN is
active. Other hosts might only be accessible when the VPN is
not active. Thus, it is best to use `NetworkMonitor::can_reach`
or `NetworkMonitor::can_reach_async` to test for reachability
on a host-by-host basis. (On the other hand, when the property is
`false`, the application can reasonably expect that no remote
hosts at all are reachable, and should indicate this to the user
in its UI.)

See also `NetworkMonitor::network-changed`.
<!-- trait NetworkMonitorExt::fn get_property_network_metered -->
Whether the network is considered metered. That is, whether the
system has traffic flowing through the default connection that is
subject to limitations set by service providers. For example, traffic
might be billed by the amount of data transmitted, or there might be a
quota on the amount of traffic per month. This is typical with tethered
connections (3G and 4G) and in such situations, bandwidth intensive
applications may wish to avoid network activity where possible if it will
cost the user money or use up their limited quota.

If more information is required about specific devices then the
system network management API should be used instead (for example,
NetworkManager or ConnMan).

If this information is not available then no networks will be
marked as metered.

See also `NetworkMonitor:network-available`.

Feature: `v2_46`

<!-- struct NetworkService -->
Like `NetworkAddress` does with hostnames, `NetworkService`
provides an easy way to resolve a SRV record, and then attempt to
connect to one of the hosts that implements that service, handling
service priority/weighting, multiple IP addresses, and multiple
address families.

See `SrvTarget` for more information about SRV records, and see
`SocketConnectable` for and example of using the connectable
interface.

# Implements

[`NetworkServiceExt`](trait.NetworkServiceExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketConnectableExt`](trait.SocketConnectableExt.html)
<!-- trait NetworkServiceExt -->
Trait containing all `NetworkService` methods.

# Implementors

[`NetworkService`](struct.NetworkService.html)
<!-- impl NetworkService::fn new -->
Creates a new `NetworkService` representing the given `service`,
`protocol`, and `domain`. This will initially be unresolved; use the
`SocketConnectable` interface to resolve it.
## `service`
the service type to look up (eg, "ldap")
## `protocol`
the networking protocol to use for `service` (eg, "tcp")
## `domain`
the DNS domain to look up the service in

# Returns

a new `NetworkService`
<!-- trait NetworkServiceExt::fn get_domain -->
Gets the domain that `self` serves. This might be either UTF-8 or
ASCII-encoded, depending on what `self` was created with.

# Returns

`self`'s domain name
<!-- trait NetworkServiceExt::fn get_protocol -->
Gets `self`'s protocol name (eg, "tcp").

# Returns

`self`'s protocol name
<!-- trait NetworkServiceExt::fn get_scheme -->
Get's the URI scheme used to resolve proxies. By default, the service name
is used as scheme.

# Returns

`self`'s scheme name
<!-- trait NetworkServiceExt::fn get_service -->
Gets `self`'s service name (eg, "ldap").

# Returns

`self`'s service name
<!-- trait NetworkServiceExt::fn set_scheme -->
Set's the URI scheme used to resolve proxies. By default, the service name
is used as scheme.
## `scheme`
a URI scheme
<!-- struct Notification -->
`Notification` is a mechanism for creating a notification to be shown
to the user -- typically as a pop-up notification presented by the
desktop environment shell.

The key difference between `Notification` and other similar APIs is
that, if supported by the desktop environment, notifications sent
with `Notification` will persist after the application has exited,
and even across system reboots.

Since the user may click on a notification while the application is
not running, applications using `Notification` should be able to be
started as a D-Bus service, using `Application`.

User interaction with a notification (either the default action, or
buttons) must be associated with actions on the application (ie:
"app." actions). It is not possible to route user interaction
through the notification itself, because the object will not exist if
the application is autostarted as a result of a notification being
clicked.

A notification can be sent with `ApplicationExt::send_notification`.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl Notification::fn new -->
Creates a new `Notification` with `title` as its title.

After populating `notification` with more details, it can be sent to
the desktop shell with `ApplicationExt::send_notification`. Changing
any properties after this call will not have any effect until
resending `notification`.
## `title`
the title of the notification

# Returns

a new `Notification` instance
<!-- impl Notification::fn add_button -->
Adds a button to `self` that activates the action in
`detailed_action` when clicked. That action must be an
application-wide action (starting with "app."). If `detailed_action`
contains a target, the action will be activated with that target as
its parameter.

See `Action::parse_detailed_name` for a description of the format
for `detailed_action`.
## `label`
label of the button
## `detailed_action`
a detailed action name
<!-- impl Notification::fn add_button_with_target -->
Adds a button to `self` that activates `action` when clicked.
`action` must be an application-wide action (it must start with "app.").

If `target_format` is given, it is used to collect remaining
positional parameters into a `glib::Variant` instance, similar to
`glib::Variant::new`. `action` will be activated with that `glib::Variant` as its
parameter.
## `label`
label of the button
## `action`
an action name
## `target_format`
a `glib::Variant` format string, or `None`
<!-- impl Notification::fn add_button_with_target_value -->
Adds a button to `self` that activates `action` when clicked.
`action` must be an application-wide action (it must start with "app.").

If `target` is non-`None`, `action` will be activated with `target` as
its parameter.
## `label`
label of the button
## `action`
an action name
## `target`
a `glib::Variant` to use as `action`'s parameter, or `None`
<!-- impl Notification::fn set_body -->
Sets the body of `self` to `body`.
## `body`
the new body for `self`, or `None`
<!-- impl Notification::fn set_default_action -->
Sets the default action of `self` to `detailed_action`. This
action is activated when the notification is clicked on.

The action in `detailed_action` must be an application-wide action (it
must start with "app."). If `detailed_action` contains a target, the
given action will be activated with that target as its parameter.
See `Action::parse_detailed_name` for a description of the format
for `detailed_action`.

When no default action is set, the application that the notification
was sent on is activated.
## `detailed_action`
a detailed action name
<!-- impl Notification::fn set_default_action_and_target -->
Sets the default action of `self` to `action`. This action is
activated when the notification is clicked on. It must be an
application-wide action (it must start with "app.").

If `target_format` is given, it is used to collect remaining
positional parameters into a `glib::Variant` instance, similar to
`glib::Variant::new`. `action` will be activated with that `glib::Variant` as its
parameter.

When no default action is set, the application that the notification
was sent on is activated.
## `action`
an action name
## `target_format`
a `glib::Variant` format string, or `None`
<!-- impl Notification::fn set_default_action_and_target_value -->
Sets the default action of `self` to `action`. This action is
activated when the notification is clicked on. It must be an
application-wide action (start with "app.").

If `target` is non-`None`, `action` will be activated with `target` as
its parameter.

When no default action is set, the application that the notification
was sent on is activated.
## `action`
an action name
## `target`
a `glib::Variant` to use as `action`'s parameter, or `None`
<!-- impl Notification::fn set_icon -->
Sets the icon of `self` to `icon`.
## `icon`
the icon to be shown in `self`, as a `Icon`
<!-- impl Notification::fn set_priority -->
Sets the priority of `self` to `priority`. See
`NotificationPriority` for possible values.
## `priority`
a `NotificationPriority`
<!-- impl Notification::fn set_title -->
Sets the title of `self` to `title`.
## `title`
the new title for `self`
<!-- enum NotificationPriority -->
Priority levels for `GNotifications`.
<!-- enum NotificationPriority::variant Normal -->
the default priority, to be used for the
 majority of notifications (for example email messages, software updates,
 completed download/sync operations)
<!-- enum NotificationPriority::variant Low -->
for notifications that do not require
 immediate attention - typically used for contextual background
 information, such as contact birthdays or local weather
<!-- enum NotificationPriority::variant High -->
for events that require more attention,
 usually because responses are time-sensitive (for example chat and SMS
 messages or alarms)
<!-- enum NotificationPriority::variant Urgent -->
for urgent notifications, or notifications
 that require a response in a short space of time (for example phone calls
 or emergency warnings)
<!-- struct OutputStream -->
`OutputStream` has functions to write to a stream (`OutputStreamExt::write`),
to close a stream (`OutputStreamExt::close`) and to flush pending writes
(`OutputStreamExt::flush`).

To copy the content of an input stream to an output stream without
manually handling the reads and writes, use `OutputStreamExt::splice`.

See the documentation for `IOStream` for details of thread safety of
streaming APIs.

All of these functions have async variants too.

# Implements

[`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html)
<!-- trait OutputStreamExt -->
Trait containing all `OutputStream` methods.

# Implementors

[`FileOutputStream`](struct.FileOutputStream.html), [`FilterOutputStream`](struct.FilterOutputStream.html), [`MemoryOutputStream`](struct.MemoryOutputStream.html), [`OutputStream`](struct.OutputStream.html), [`PollableOutputStream`](struct.PollableOutputStream.html), [`UnixOutputStream`](struct.UnixOutputStream.html)
<!-- trait OutputStreamExt::fn clear_pending -->
Clears the pending flag on `self`.
<!-- trait OutputStreamExt::fn close -->
Closes the stream, releasing resources related to it.

Once the stream is closed, all other operations will return `IOErrorEnum::Closed`.
Closing a stream multiple times will not return an error.

Closing a stream will automatically flush any outstanding buffers in the
stream.

Streams will be automatically closed when the last reference
is dropped, but you might want to call this function to make sure
resources are released as early as possible.

Some streams might keep the backing store of the stream (e.g. a file descriptor)
open after the stream is closed. See the documentation for the individual
stream for details.

On failure the first error that happened will be reported, but the close
operation will finish as much as possible. A stream that failed to
close will still return `IOErrorEnum::Closed` for all operations. Still, it
is important to check and report the error to the user, otherwise
there might be a loss of data as all data might not be written.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
Cancelling a close will still leave the stream closed, but there some streams
can use a faster close that doesn't block to e.g. check errors. On
cancellation (as with any error) there is no guarantee that all written
data will reach the target.
## `cancellable`
optional cancellable object

# Returns

`true` on success, `false` on failure
<!-- trait OutputStreamExt::fn close_async -->
Requests an asynchronous close of the stream, releasing resources
related to it. When the operation is finished `callback` will be
called. You can then call `OutputStreamExt::close_finish` to get
the result of the operation.

For behaviour details see `OutputStreamExt::close`.

The asynchronous methods have a default fallback that uses threads
to implement asynchronicity, so they are optional for inheriting
classes. However, if you override one you must override all.
## `io_priority`
the io priority of the request.
## `cancellable`
optional cancellable object
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait OutputStreamExt::fn close_finish -->
Closes an output stream.
## `result`
a `AsyncResult`.

# Returns

`true` if stream was successfully closed, `false` otherwise.
<!-- trait OutputStreamExt::fn flush -->
Forces a write of all user-space buffered data for the given
`self`. Will block during the operation. Closing the stream will
implicitly cause a flush.

This function is optional for inherited classes.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `cancellable`
optional cancellable object

# Returns

`true` on success, `false` on error
<!-- trait OutputStreamExt::fn flush_async -->
Forces an asynchronous write of all user-space buffered data for
the given `self`.
For behaviour details see `OutputStreamExt::flush`.

When the operation is finished `callback` will be
called. You can then call `OutputStreamExt::flush_finish` to get the
result of the operation.
## `io_priority`
the io priority of the request.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait OutputStreamExt::fn flush_finish -->
Finishes flushing an output stream.
## `result`
a GAsyncResult.

# Returns

`true` if flush operation succeeded, `false` otherwise.
<!-- trait OutputStreamExt::fn has_pending -->
Checks if an output stream has pending actions.

# Returns

`true` if `self` has pending actions.
<!-- trait OutputStreamExt::fn is_closed -->
Checks if an output stream has already been closed.

# Returns

`true` if `self` is closed. `false` otherwise.
<!-- trait OutputStreamExt::fn is_closing -->
Checks if an output stream is being closed. This can be
used inside e.g. a flush implementation to see if the
flush (or other i/o operation) is called from within
the closing operation.

# Returns

`true` if `self` is being closed. `false` otherwise.
<!-- trait OutputStreamExt::fn printf -->
This is a utility function around `OutputStream::write_all`. It
uses `g_strdup_vprintf` to turn `format` and @... into a string that
is then written to `self`.

See the documentation of `OutputStream::write_all` about the
behavior of the actual write operation.

Note that partial writes cannot be properly checked with this
function due to the variable length of the written string, if you
need precise control over partial write failures, you need to
create you own `printf`-like wrapper around `OutputStreamExt::write`
or `OutputStream::write_all`.
## `bytes_written`
location to store the number of bytes that was
 written to the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `error`
location to store the error occurring, or `None` to ignore
## `format`
the format string. See the `printf` documentation

# Returns

`true` on success, `false` if there was an error
<!-- trait OutputStreamExt::fn set_pending -->
Sets `self` to have actions pending. If the pending flag is
already set or `self` is closed, it will return `false` and set
`error`.

# Returns

`true` if pending was previously unset and is now set.
<!-- trait OutputStreamExt::fn splice -->
Splices an input stream into an output stream.
## `source`
a `InputStream`.
## `flags`
a set of `OutputStreamSpliceFlags`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `gssize` containing the size of the data spliced, or
 -1 if an error occurred. Note that if the number of bytes
 spliced is greater than `G_MAXSSIZE`, then that will be
 returned, and there is no way to determine the actual number
 of bytes spliced.
<!-- trait OutputStreamExt::fn splice_async -->
Splices a stream asynchronously.
When the operation is finished `callback` will be called.
You can then call `OutputStreamExt::splice_finish` to get the
result of the operation.

For the synchronous, blocking version of this function, see
`OutputStreamExt::splice`.
## `source`
a `InputStream`.
## `flags`
a set of `OutputStreamSpliceFlags`.
## `io_priority`
the io priority of the request.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback`.
## `user_data`
user data passed to `callback`.
<!-- trait OutputStreamExt::fn splice_finish -->
Finishes an asynchronous stream splice operation.
## `result`
a `AsyncResult`.

# Returns

a `gssize` of the number of bytes spliced. Note that if the
 number of bytes spliced is greater than `G_MAXSSIZE`, then that
 will be returned, and there is no way to determine the actual
 number of bytes spliced.
<!-- trait OutputStreamExt::fn vprintf -->
This is a utility function around `OutputStream::write_all`. It
uses `g_strdup_vprintf` to turn `format` and `args` into a string that
is then written to `self`.

See the documentation of `OutputStream::write_all` about the
behavior of the actual write operation.

Note that partial writes cannot be properly checked with this
function due to the variable length of the written string, if you
need precise control over partial write failures, you need to
create you own `printf`-like wrapper around `OutputStreamExt::write`
or `OutputStream::write_all`.
## `bytes_written`
location to store the number of bytes that was
 written to the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `error`
location to store the error occurring, or `None` to ignore
## `format`
the format string. See the `printf` documentation
## `args`
the parameters to insert into the format string

# Returns

`true` on success, `false` if there was an error
<!-- trait OutputStreamExt::fn write -->
Tries to write `count` bytes from `buffer` into the stream. Will block
during the operation.

If count is 0, returns 0 and does nothing. A value of `count`
larger than `G_MAXSSIZE` will cause a `IOErrorEnum::InvalidArgument` error.

On success, the number of bytes written to the stream is returned.
It is not an error if this is not the same as the requested size, as it
can happen e.g. on a partial I/O error, or if there is not enough
storage in the stream. All writes block until at least one byte
is written or an error occurs; 0 is never returned (unless
`count` is 0).

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.

On error -1 is returned and `error` is set accordingly.
## `buffer`
the buffer containing the data to write.
## `count`
the number of bytes to write
## `cancellable`
optional cancellable object

# Returns

Number of bytes written, or -1 on error
<!-- trait OutputStreamExtManual::fn write_all -->
Tries to write `count` bytes from `buffer` into the stream. Will block
during the operation.

This function is similar to `OutputStreamExt::write`, except it tries to
write as many bytes as requested, only stopping on an error.

On a successful write of `count` bytes, `true` is returned, and `bytes_written`
is set to `count`.

If there is an error during the operation `false` is returned and `error`
is set to indicate the error status.

As a special exception to the normal conventions for functions that
use `glib::Error`, if this function returns `false` (and sets `error`) then
`bytes_written` will be set to the number of bytes that were
successfully written before the error was encountered. This
functionality is only available from C. If you need it from another
language then you must write your own loop around
`OutputStreamExt::write`.
## `buffer`
the buffer containing the data to write.
## `count`
the number of bytes to write
## `bytes_written`
location to store the number of bytes that was
 written to the stream
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` on success, `false` if there was an error
<!-- trait OutputStreamExtManual::fn write_all_async -->
Request an asynchronous write of `count` bytes from `buffer` into
the stream. When the operation is finished `callback` will be called.
You can then call `OutputStreamExt::write_all_finish` to get the result of the
operation.

This is the asynchronous version of `OutputStream::write_all`.

Call `OutputStreamExt::write_all_finish` to collect the result.

Any outstanding I/O request with higher priority (lower numerical
value) will be executed before an outstanding request with lower
priority. Default priority is `G_PRIORITY_DEFAULT`.

Note that no copy of `buffer` will be made, so it must stay valid
until `callback` is called.

Feature: `v2_44`

## `buffer`
the buffer containing the data to write
## `count`
the number of bytes to write
## `io_priority`
the io priority of the request
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait OutputStreamExt::fn write_all_finish -->
Finishes an asynchronous stream write operation started with
`OutputStream::write_all_async`.

As a special exception to the normal conventions for functions that
use `glib::Error`, if this function returns `false` (and sets `error`) then
`bytes_written` will be set to the number of bytes that were
successfully written before the error was encountered. This
functionality is only available from C. If you need it from another
language then you must write your own loop around
`OutputStream::write_async`.

Feature: `v2_44`

## `result`
a `AsyncResult`
## `bytes_written`
location to store the number of bytes that was written to the stream

# Returns

`true` on success, `false` if there was an error
<!-- trait OutputStreamExtManual::fn write_async -->
Request an asynchronous write of `count` bytes from `buffer` into
the stream. When the operation is finished `callback` will be called.
You can then call `OutputStreamExt::write_finish` to get the result of the
operation.

During an async request no other sync and async calls are allowed,
and will result in `IOErrorEnum::Pending` errors.

A value of `count` larger than `G_MAXSSIZE` will cause a
`IOErrorEnum::InvalidArgument` error.

On success, the number of bytes written will be passed to the
`callback`. It is not an error if this is not the same as the
requested size, as it can happen e.g. on a partial I/O error,
but generally we try to write as many bytes as requested.

You are guaranteed that this method will never fail with
`IOErrorEnum::WouldBlock` - if `self` can't accept more data, the
method will just wait until this changes.

Any outstanding I/O request with higher priority (lower numerical
value) will be executed before an outstanding request with lower
priority. Default priority is `G_PRIORITY_DEFAULT`.

The asynchronous methods have a default fallback that uses threads
to implement asynchronicity, so they are optional for inheriting
classes. However, if you override one you must override all.

For the synchronous, blocking version of this function, see
`OutputStreamExt::write`.

Note that no copy of `buffer` will be made, so it must stay valid
until `callback` is called. See `OutputStreamExt::write_bytes_async`
for a `glib::Bytes` version that will automatically hold a reference to
the contents (without copying) for the duration of the call.
## `buffer`
the buffer containing the data to write.
## `count`
the number of bytes to write
## `io_priority`
the io priority of the request.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait OutputStreamExt::fn write_bytes -->
A wrapper function for `OutputStreamExt::write` which takes a
`glib::Bytes` as input. This can be more convenient for use by language
bindings or in other cases where the refcounted nature of `glib::Bytes`
is helpful over a bare pointer interface.

However, note that this function may still perform partial writes,
just like `OutputStreamExt::write`. If that occurs, to continue
writing, you will need to create a new `glib::Bytes` containing just the
remaining bytes, using `glib::Bytes::new_from_bytes`. Passing the same
`glib::Bytes` instance multiple times potentially can result in duplicated
data in the output stream.
## `bytes`
the `glib::Bytes` to write
## `cancellable`
optional cancellable object

# Returns

Number of bytes written, or -1 on error
<!-- trait OutputStreamExt::fn write_bytes_async -->
This function is similar to `OutputStream::write_async`, but
takes a `glib::Bytes` as input. Due to the refcounted nature of `glib::Bytes`,
this allows the stream to avoid taking a copy of the data.

However, note that this function may still perform partial writes,
just like `OutputStream::write_async`. If that occurs, to continue
writing, you will need to create a new `glib::Bytes` containing just the
remaining bytes, using `glib::Bytes::new_from_bytes`. Passing the same
`glib::Bytes` instance multiple times potentially can result in duplicated
data in the output stream.

For the synchronous, blocking version of this function, see
`OutputStreamExt::write_bytes`.
## `bytes`
The bytes to write
## `io_priority`
the io priority of the request.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
callback to call when the request is satisfied
## `user_data`
the data to pass to callback function
<!-- trait OutputStreamExt::fn write_bytes_finish -->
Finishes a stream write-from-`glib::Bytes` operation.
## `result`
a `AsyncResult`.

# Returns

a `gssize` containing the number of bytes written to the stream.
<!-- trait OutputStreamExt::fn write_finish -->
Finishes a stream write operation.
## `result`
a `AsyncResult`.

# Returns

a `gssize` containing the number of bytes written to the stream.
<!-- enum PasswordSave -->
`PasswordSave` is used to indicate the lifespan of a saved password.

`Gvfs` stores passwords in the Gnome keyring when this flag allows it
to, and later retrieves it again from there.
<!-- enum PasswordSave::variant Never -->
never save a password.
<!-- enum PasswordSave::variant ForSession -->
save a password for the session.
<!-- enum PasswordSave::variant Permanently -->
save a password permanently.
<!-- struct Permission -->
A `Permission` represents the status of the caller's permission to
perform a certain action.

You can query if the action is currently allowed and if it is
possible to acquire the permission so that the action will be allowed
in the future.

There is also an API to actually acquire the permission and one to
release it.

As an example, a `Permission` might represent the ability for the
user to write to a `Settings` object. This `Permission` object could
then be used to decide if it is appropriate to show a "Click here to
unlock" button in a dialog and to provide the mechanism to invoke
when that button is clicked.

# Implements

[`PermissionExt`](trait.PermissionExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait PermissionExt -->
Trait containing all `Permission` methods.

# Implementors

[`Permission`](struct.Permission.html), [`SimplePermission`](struct.SimplePermission.html)
<!-- trait PermissionExt::fn acquire -->
Attempts to acquire the permission represented by `self`.

The precise method by which this happens depends on the permission
and the underlying authentication mechanism. A simple example is
that a dialog may appear asking the user to enter their password.

You should check with `PermissionExt::get_can_acquire` before calling
this function.

If the permission is acquired then `true` is returned. Otherwise,
`false` is returned and `error` is set appropriately.

This call is blocking, likely for a very long time (in the case that
user interaction is required). See `PermissionExt::acquire_async` for
the non-blocking version.
## `cancellable`
a `Cancellable`, or `None`

# Returns

`true` if the permission was successfully acquired
<!-- trait PermissionExt::fn acquire_async -->
Attempts to acquire the permission represented by `self`.

This is the first half of the asynchronous version of
`PermissionExt::acquire`.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
the `GAsyncReadyCallback` to call when done
## `user_data`
the user data to pass to `callback`
<!-- trait PermissionExt::fn acquire_finish -->
Collects the result of attempting to acquire the permission
represented by `self`.

This is the second half of the asynchronous version of
`PermissionExt::acquire`.
## `result`
the `AsyncResult` given to the `GAsyncReadyCallback`

# Returns

`true` if the permission was successfully acquired
<!-- trait PermissionExt::fn get_allowed -->
Gets the value of the 'allowed' property. This property is `true` if
the caller currently has permission to perform the action that
`self` represents the permission to perform.

# Returns

the value of the 'allowed' property
<!-- trait PermissionExt::fn get_can_acquire -->
Gets the value of the 'can-acquire' property. This property is `true`
if it is generally possible to acquire the permission by calling
`PermissionExt::acquire`.

# Returns

the value of the 'can-acquire' property
<!-- trait PermissionExt::fn get_can_release -->
Gets the value of the 'can-release' property. This property is `true`
if it is generally possible to release the permission by calling
`PermissionExt::release`.

# Returns

the value of the 'can-release' property
<!-- trait PermissionExt::fn impl_update -->
This function is called by the `Permission` implementation to update
the properties of the permission. You should never call this
function except from a `Permission` implementation.

GObject notify signals are generated, as appropriate.
## `allowed`
the new value for the 'allowed' property
## `can_acquire`
the new value for the 'can-acquire' property
## `can_release`
the new value for the 'can-release' property
<!-- trait PermissionExt::fn release -->
Attempts to release the permission represented by `self`.

The precise method by which this happens depends on the permission
and the underlying authentication mechanism. In most cases the
permission will be dropped immediately without further action.

You should check with `PermissionExt::get_can_release` before calling
this function.

If the permission is released then `true` is returned. Otherwise,
`false` is returned and `error` is set appropriately.

This call is blocking, likely for a very long time (in the case that
user interaction is required). See `PermissionExt::release_async` for
the non-blocking version.
## `cancellable`
a `Cancellable`, or `None`

# Returns

`true` if the permission was successfully released
<!-- trait PermissionExt::fn release_async -->
Attempts to release the permission represented by `self`.

This is the first half of the asynchronous version of
`PermissionExt::release`.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
the `GAsyncReadyCallback` to call when done
## `user_data`
the user data to pass to `callback`
<!-- trait PermissionExt::fn release_finish -->
Collects the result of attempting to release the permission
represented by `self`.

This is the second half of the asynchronous version of
`PermissionExt::release`.
## `result`
the `AsyncResult` given to the `GAsyncReadyCallback`

# Returns

`true` if the permission was successfully released
<!-- trait PermissionExt::fn get_property_allowed -->
`true` if the caller currently has permission to perform the action that
`permission` represents the permission to perform.
<!-- trait PermissionExt::fn get_property_can_acquire -->
`true` if it is generally possible to acquire the permission by calling
`PermissionExt::acquire`.
<!-- trait PermissionExt::fn get_property_can_release -->
`true` if it is generally possible to release the permission by calling
`PermissionExt::release`.
<!-- struct PollableInputStream -->
`PollableInputStream` is implemented by `GInputStreams` that
can be polled for readiness to read. This can be used when
interfacing with a non-GIO API that expects
UNIX-file-descriptor-style asynchronous I/O rather than GIO-style.

# Implements

[`PollableInputStreamExt`](trait.PollableInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableInputStreamExtManual`](prelude/trait.PollableInputStreamExtManual.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html)
<!-- trait PollableInputStreamExt -->
Trait containing all `PollableInputStream` methods.

# Implementors

[`ConverterInputStream`](struct.ConverterInputStream.html), [`MemoryInputStream`](struct.MemoryInputStream.html), [`PollableInputStream`](struct.PollableInputStream.html), [`UnixInputStream`](struct.UnixInputStream.html)
<!-- trait PollableInputStreamExt::fn can_poll -->
Checks if `self` is actually pollable. Some classes may implement
`PollableInputStream` but have only certain instances of that class
be pollable. If this method returns `false`, then the behavior of
other `PollableInputStream` methods is undefined.

For any given stream, the value returned by this method is constant;
a stream cannot switch from pollable to non-pollable or vice versa.

# Returns

`true` if `self` is pollable, `false` if not.
<!-- trait PollableInputStreamExtManual::fn create_source -->
Creates a `glib::Source` that triggers when `self` can be read, or
`cancellable` is triggered or an error occurs. The callback on the
source is of the `GPollableSourceFunc` type.

As with `PollableInputStream::is_readable`, it is possible that
the stream may not actually be readable even after the source
triggers, so you should use `PollableInputStream::read_nonblocking`
rather than `InputStream::read` from the callback.
## `cancellable`
a `Cancellable`, or `None`

# Returns

a new `glib::Source`
<!-- trait PollableInputStreamExt::fn is_readable -->
Checks if `self` can be read.

Note that some stream types may not be able to implement this 100%
reliably, and it is possible that a call to `InputStream::read`
after this returns `true` would still block. To guarantee
non-blocking behavior, you should always use
`PollableInputStream::read_nonblocking`, which will return a
`IOErrorEnum::WouldBlock` error rather than blocking.

# Returns

`true` if `self` is readable, `false` if not. If an error
 has occurred on `self`, this will result in
 `PollableInputStream::is_readable` returning `true`, and the
 next attempt to read will return the error.
<!-- trait PollableInputStreamExtManual::fn read_nonblocking -->
Attempts to read up to `count` bytes from `self` into `buffer`, as
with `InputStream::read`. If `self` is not currently readable,
this will immediately return `IOErrorEnum::WouldBlock`, and you can
use `PollableInputStream::create_source` to create a `glib::Source`
that will be triggered when `self` is readable.

Note that since this method never blocks, you cannot actually
use `cancellable` to cancel it. However, it will return an error
if `cancellable` has already been cancelled when you call, which
may happen if you call this method after a source triggers due
to having been cancelled.
## `buffer`
a buffer to
 read data into (which should be at least `count` bytes long).
## `count`
the number of bytes you want to read
## `cancellable`
a `Cancellable`, or `None`

# Returns

the number of bytes read, or -1 on error (including
 `IOErrorEnum::WouldBlock`).
<!-- struct PollableOutputStream -->
`PollableOutputStream` is implemented by `GOutputStreams` that
can be polled for readiness to write. This can be used when
interfacing with a non-GIO API that expects
UNIX-file-descriptor-style asynchronous I/O rather than GIO-style.

# Implements

[`PollableOutputStreamExt`](trait.PollableOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableOutputStreamExtManual`](prelude/trait.PollableOutputStreamExtManual.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html)
<!-- trait PollableOutputStreamExt -->
Trait containing all `PollableOutputStream` methods.

# Implementors

[`ConverterOutputStream`](struct.ConverterOutputStream.html), [`MemoryOutputStream`](struct.MemoryOutputStream.html), [`PollableOutputStream`](struct.PollableOutputStream.html), [`UnixOutputStream`](struct.UnixOutputStream.html)
<!-- trait PollableOutputStreamExt::fn can_poll -->
Checks if `self` is actually pollable. Some classes may implement
`PollableOutputStream` but have only certain instances of that
class be pollable. If this method returns `false`, then the behavior
of other `PollableOutputStream` methods is undefined.

For any given stream, the value returned by this method is constant;
a stream cannot switch from pollable to non-pollable or vice versa.

# Returns

`true` if `self` is pollable, `false` if not.
<!-- trait PollableOutputStreamExtManual::fn create_source -->
Creates a `glib::Source` that triggers when `self` can be written, or
`cancellable` is triggered or an error occurs. The callback on the
source is of the `GPollableSourceFunc` type.

As with `PollableOutputStream::is_writable`, it is possible that
the stream may not actually be writable even after the source
triggers, so you should use `PollableOutputStream::write_nonblocking`
rather than `OutputStreamExt::write` from the callback.
## `cancellable`
a `Cancellable`, or `None`

# Returns

a new `glib::Source`
<!-- trait PollableOutputStreamExt::fn is_writable -->
Checks if `self` can be written.

Note that some stream types may not be able to implement this 100%
reliably, and it is possible that a call to `OutputStreamExt::write`
after this returns `true` would still block. To guarantee
non-blocking behavior, you should always use
`PollableOutputStream::write_nonblocking`, which will return a
`IOErrorEnum::WouldBlock` error rather than blocking.

# Returns

`true` if `self` is writable, `false` if not. If an error
 has occurred on `self`, this will result in
 `PollableOutputStream::is_writable` returning `true`, and the
 next attempt to write will return the error.
<!-- trait PollableOutputStreamExt::fn write_nonblocking -->
Attempts to write up to `count` bytes from `buffer` to `self`, as
with `OutputStreamExt::write`. If `self` is not currently writable,
this will immediately return `IOErrorEnum::WouldBlock`, and you can
use `PollableOutputStream::create_source` to create a `glib::Source`
that will be triggered when `self` is writable.

Note that since this method never blocks, you cannot actually
use `cancellable` to cancel it. However, it will return an error
if `cancellable` has already been cancelled when you call, which
may happen if you call this method after a source triggers due
to having been cancelled.

Also note that if `IOErrorEnum::WouldBlock` is returned some underlying
transports like D/TLS require that you send the same `buffer` and `count`.
## `buffer`
a buffer to write
 data from
## `count`
the number of bytes you want to write
## `cancellable`
a `Cancellable`, or `None`

# Returns

the number of bytes written, or -1 on error (including
 `IOErrorEnum::WouldBlock`).
<!-- struct PropertyAction -->
A `PropertyAction` is a way to get a `Action` with a state value
reflecting and controlling the value of a `gobject::Object` property.

The state of the action will correspond to the value of the property.
Changing it will change the property (assuming the requested value
matches the requirements as specified in the `gobject::ParamSpec`).

Only the most common types are presently supported. Booleans are
mapped to booleans, strings to strings, signed/unsigned integers to
int32/uint32 and floats and doubles to doubles.

If the property is an enum then the state will be string-typed and
conversion will automatically be performed between the enum value and
"nick" string as per the `gobject::EnumValue` table.

Flags types are not currently supported.

Properties of object types, boxed types and pointer types are not
supported and probably never will be.

Properties of `glib::Variant` types are not currently supported.

If the property is boolean-valued then the action will have a NULL
parameter type, and activating the action (with no parameter) will
toggle the value of the property.

In all other cases, the parameter type will correspond to the type of
the property.

The general idea here is to reduce the number of locations where a
particular piece of state is kept (and therefore has to be synchronised
between). `PropertyAction` does not have a separate state that is kept
in sync with the property value -- its state is the property value.

For example, it might be useful to create a `Action` corresponding to
the "visible-child-name" property of a ``GtkStack`` so that the current
page can be switched from a menu. The active radio indication in the
menu is then directly determined from the active page of the
``GtkStack``.

An anti-example would be binding the "active-id" property on a
``GtkComboBox``. This is because the state of the combobox itself is
probably uninteresting and is actually being used to control
something else.

Another anti-example would be to bind to the "visible-child-name"
property of a ``GtkStack`` if this value is actually stored in
`Settings`. In that case, the real source of the value is
`Settings`. If you want a `Action` to control a setting stored in
`Settings`, see `SettingsExt::create_action` instead, and possibly
combine its use with `SettingsExt::bind`.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ActionExt`](trait.ActionExt.html)
<!-- impl PropertyAction::fn new -->
Creates a `Action` corresponding to the value of property
`property_name` on `object`.

The property must be existent and readable and writable (and not
construct-only).

This function takes a reference on `object` and doesn't release it
until the action is destroyed.
## `name`
the name of the action to create
## `object`
the object that has the property
 to wrap
## `property_name`
the name of the property

# Returns

a new `PropertyAction`
<!-- impl PropertyAction::fn get_property_enabled -->
If `action` is currently enabled.

If the action is disabled then calls to `Action::activate` and
`Action::change_state` have no effect.
<!-- impl PropertyAction::fn get_property_invert_boolean -->
If `true`, the state of the action will be the negation of the
property value, provided the property is boolean.

Feature: `v2_46`

<!-- impl PropertyAction::fn set_property_invert_boolean -->
If `true`, the state of the action will be the negation of the
property value, provided the property is boolean.

Feature: `v2_46`

<!-- impl PropertyAction::fn get_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `ActionMap`.
<!-- impl PropertyAction::fn set_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `ActionMap`.
<!-- impl PropertyAction::fn set_property_object -->
The object to wrap a property on.

The object must be a non-`None` `gobject::Object` with properties.
<!-- impl PropertyAction::fn get_property_parameter_type -->
The type of the parameter that must be given when activating the
action.
<!-- impl PropertyAction::fn set_property_property_name -->
The name of the property to wrap on the object.

The property must exist on the passed-in object and it must be
readable and writable (and not construct-only).
<!-- impl PropertyAction::fn get_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- impl PropertyAction::fn get_property_state_type -->
The `glib::VariantType` of the state that the action has, or `None` if the
action is stateless.
<!-- struct Proxy -->
A `Proxy` handles connecting to a remote host via a given type of
proxy server. It is implemented by the 'gio-proxy' extension point.
The extensions are named after their proxy protocol name. As an
example, a SOCKS5 proxy implementation can be retrieved with the
name 'socks5' using the function
`IOExtensionPoint::get_extension_by_name`.

# Implements

[`ProxyExt`](trait.ProxyExt.html)
<!-- trait ProxyExt -->
Trait containing all `Proxy` methods.

# Implementors

[`Proxy`](struct.Proxy.html)
<!-- impl Proxy::fn get_default_for_protocol -->
Lookup "gio-proxy" extension point for a proxy implementation that supports
specified protocol.
## `protocol`
the proxy protocol name (e.g. http, socks, etc)

# Returns

return a `Proxy` or NULL if protocol
 is not supported.
<!-- trait ProxyExt::fn connect -->
Given `connection` to communicate with a proxy (eg, a
`SocketConnection` that is connected to the proxy server), this
does the necessary handshake to connect to `proxy_address`, and if
required, wraps the `IOStream` to handle proxy payload.
## `connection`
a `IOStream`
## `proxy_address`
a `ProxyAddress`
## `cancellable`
a `Cancellable`

# Returns

a `IOStream` that will replace `connection`. This might
 be the same as `connection`, in which case a reference
 will be added.
<!-- trait ProxyExt::fn connect_async -->
Asynchronous version of `Proxy::connect`.
## `connection`
a `IOStream`
## `proxy_address`
a `ProxyAddress`
## `cancellable`
a `Cancellable`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
callback data
<!-- trait ProxyExt::fn connect_finish -->
See `Proxy::connect`.
## `result`
a `AsyncResult`

# Returns

a `IOStream`.
<!-- trait ProxyExt::fn supports_hostname -->
Some proxy protocols expect to be passed a hostname, which they
will resolve to an IP address themselves. Others, like SOCKS4, do
not allow this. This function will return `false` if `self` is
implementing such a protocol. When `false` is returned, the caller
should resolve the destination hostname first, and then pass a
`ProxyAddress` containing the stringified IP address to
`Proxy::connect` or `Proxy::connect_async`.

# Returns

`true` if hostname resolution is supported.
<!-- struct ProxyAddress -->
Support for proxied `InetSocketAddress`.

# Implements

[`ProxyAddressExt`](trait.ProxyAddressExt.html), [`InetSocketAddressExt`](trait.InetSocketAddressExt.html), [`SocketAddressExt`](trait.SocketAddressExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketConnectableExt`](trait.SocketConnectableExt.html)
<!-- trait ProxyAddressExt -->
Trait containing all `ProxyAddress` methods.

# Implementors

[`ProxyAddress`](struct.ProxyAddress.html)
<!-- impl ProxyAddress::fn new -->
Creates a new `ProxyAddress` for `inetaddr` with `protocol` that should
tunnel through `dest_hostname` and `dest_port`.

(Note that this method doesn't set the `ProxyAddress:uri` or
`ProxyAddress:destination-protocol` fields; use `gobject::Object::new`
directly if you want to set those.)
## `inetaddr`
The proxy server `InetAddress`.
## `port`
The proxy server port.
## `protocol`
The proxy protocol to support, in lower case (e.g. socks, http).
## `dest_hostname`
The destination hostname the proxy should tunnel to.
## `dest_port`
The destination port to tunnel to.
## `username`
The username to authenticate to the proxy server
 (or `None`).
## `password`
The password to authenticate to the proxy server
 (or `None`).

# Returns

a new `ProxyAddress`
<!-- trait ProxyAddressExt::fn get_destination_hostname -->
Gets `self`'s destination hostname; that is, the name of the host
that will be connected to via the proxy, not the name of the proxy
itself.

# Returns

the `self`'s destination hostname
<!-- trait ProxyAddressExt::fn get_destination_port -->
Gets `self`'s destination port; that is, the port on the
destination host that will be connected to via the proxy, not the
port number of the proxy itself.

# Returns

the `self`'s destination port
<!-- trait ProxyAddressExt::fn get_destination_protocol -->
Gets the protocol that is being spoken to the destination
server; eg, "http" or "ftp".

# Returns

the `self`'s destination protocol
<!-- trait ProxyAddressExt::fn get_password -->
Gets `self`'s password.

# Returns

the `self`'s password
<!-- trait ProxyAddressExt::fn get_protocol -->
Gets `self`'s protocol. eg, "socks" or "http"

# Returns

the `self`'s protocol
<!-- trait ProxyAddressExt::fn get_uri -->
Gets the proxy URI that `self` was constructed from.

# Returns

the `self`'s URI, or `None` if unknown
<!-- trait ProxyAddressExt::fn get_username -->
Gets `self`'s username.

# Returns

the `self`'s username
<!-- trait ProxyAddressExt::fn get_property_destination_protocol -->
The protocol being spoke to the destination host, or `None` if
the `ProxyAddress` doesn't know.
<!-- trait ProxyAddressExt::fn set_property_destination_protocol -->
The protocol being spoke to the destination host, or `None` if
the `ProxyAddress` doesn't know.
<!-- trait ProxyAddressExt::fn get_property_uri -->
The URI string that the proxy was constructed from (or `None`
if the creator didn't specify this).
<!-- trait ProxyAddressExt::fn set_property_uri -->
The URI string that the proxy was constructed from (or `None`
if the creator didn't specify this).
<!-- struct ProxyResolver -->
`ProxyResolver` provides synchronous and asynchronous network proxy
resolution. `ProxyResolver` is used within `SocketClient` through
the method `SocketConnectable::proxy_enumerate`.

Implementations of `ProxyResolver` based on libproxy and GNOME settings can
be found in glib-networking. GIO comes with an implementation for use inside
Flatpak portals.

# Implements

[`ProxyResolverExt`](trait.ProxyResolverExt.html)
<!-- trait ProxyResolverExt -->
Trait containing all `ProxyResolver` methods.

# Implementors

[`ProxyResolver`](struct.ProxyResolver.html)
<!-- impl ProxyResolver::fn get_default -->
Gets the default `ProxyResolver` for the system.

# Returns

the default `ProxyResolver`.
<!-- trait ProxyResolverExt::fn is_supported -->
Checks if `self` can be used on this system. (This is used
internally; `ProxyResolver::get_default` will only return a proxy
resolver that returns `true` for this method.)

# Returns

`true` if `self` is supported.
<!-- trait ProxyResolverExt::fn lookup -->
Looks into the system proxy configuration to determine what proxy,
if any, to use to connect to `uri`. The returned proxy URIs are of
the form `<protocol>://[user[:password]@]host:port` or
`direct://`, where `<protocol>` could be http, rtsp, socks
or other proxying protocol.

If you don't know what network protocol is being used on the
socket, you should use `none` as the URI protocol.
In this case, the resolver might still return a generic proxy type
(such as SOCKS), but would not return protocol-specific proxy types
(such as http).

`direct://` is used when no proxy is needed.
Direct connection should not be attempted unless it is part of the
returned array of proxies.
## `uri`
a URI representing the destination to connect to
## `cancellable`
a `Cancellable`, or `None`

# Returns

A
 NULL-terminated array of proxy URIs. Must be freed
 with `g_strfreev`.
<!-- trait ProxyResolverExt::fn lookup_async -->
Asynchronous lookup of proxy. See `ProxyResolver::lookup` for more
details.
## `uri`
a URI representing the destination to connect to
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call after resolution completes
## `user_data`
data for `callback`
<!-- trait ProxyResolverExt::fn lookup_finish -->
Call this function to obtain the array of proxy URIs when
`ProxyResolver::lookup_async` is complete. See
`ProxyResolver::lookup` for more details.
## `result`
the result passed to your `GAsyncReadyCallback`

# Returns

A
 NULL-terminated array of proxy URIs. Must be freed
 with `g_strfreev`.
<!-- struct RemoteActionGroup -->
The GRemoteActionGroup interface is implemented by `ActionGroup`
instances that either transmit action invocations to other processes
or receive action invocations in the local process from other
processes.

The interface has `_full` variants of the two
methods on `ActionGroup` used to activate actions:
`ActionGroup::activate_action` and
`ActionGroup::change_action_state`. These variants allow a
"platform data" `glib::Variant` to be specified: a dictionary providing
context for the action invocation (for example: timestamps, startup
notification IDs, etc).

`DBusActionGroup` implements `RemoteActionGroup`. This provides a
mechanism to send platform data for action invocations over D-Bus.

Additionally, `DBusConnection::export_action_group` will check if
the exported `ActionGroup` implements `RemoteActionGroup` and use the
`_full` variants of the calls if available. This
provides a mechanism by which to receive platform data for action
invocations that arrive by way of D-Bus.

# Implements

[`RemoteActionGroupExt`](trait.RemoteActionGroupExt.html), [`ActionGroupExt`](trait.ActionGroupExt.html)
<!-- trait RemoteActionGroupExt -->
Trait containing all `RemoteActionGroup` methods.

# Implementors

[`RemoteActionGroup`](struct.RemoteActionGroup.html)
<!-- trait RemoteActionGroupExt::fn activate_action_full -->
Activates the remote action.

This is the same as `ActionGroup::activate_action` except that it
allows for provision of "platform data" to be sent along with the
activation request. This typically contains details such as the user
interaction timestamp or startup notification information.

`platform_data` must be non-`None` and must have the type
`G_VARIANT_TYPE_VARDICT`. If it is floating, it will be consumed.
## `action_name`
the name of the action to activate
## `parameter`
the optional parameter to the activation
## `platform_data`
the platform data to send
<!-- trait RemoteActionGroupExt::fn change_action_state_full -->
Changes the state of a remote action.

This is the same as `ActionGroup::change_action_state` except that
it allows for provision of "platform data" to be sent along with the
state change request. This typically contains details such as the
user interaction timestamp or startup notification information.

`platform_data` must be non-`None` and must have the type
`G_VARIANT_TYPE_VARDICT`. If it is floating, it will be consumed.
## `action_name`
the name of the action to change the state of
## `value`
the new requested value for the state
## `platform_data`
the platform data to send
<!-- struct Resolver -->
`Resolver` provides cancellable synchronous and asynchronous DNS
resolution, for hostnames (`ResolverExt::lookup_by_address`,
`ResolverExt::lookup_by_name` and their async variants) and SRV
(service) records (`ResolverExt::lookup_service`).

`NetworkAddress` and `NetworkService` provide wrappers around
`Resolver` functionality that also implement `SocketConnectable`,
making it easy to connect to a remote host/service.

# Implements

[`ResolverExt`](trait.ResolverExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait ResolverExt -->
Trait containing all `Resolver` methods.

# Implementors

[`Resolver`](struct.Resolver.html)
<!-- impl Resolver::fn free_addresses -->
Frees `addresses` (which should be the return value from
`ResolverExt::lookup_by_name` or `ResolverExt::lookup_by_name_finish`).
(This is a convenience method; you can also simply free the results
by hand.)
## `addresses`
a `glib::List` of `InetAddress`
<!-- impl Resolver::fn free_targets -->
Frees `targets` (which should be the return value from
`ResolverExt::lookup_service` or `ResolverExt::lookup_service_finish`).
(This is a convenience method; you can also simply free the
results by hand.)
## `targets`
a `glib::List` of `SrvTarget`
<!-- impl Resolver::fn get_default -->
Gets the default `Resolver`. You should unref it when you are done
with it. `Resolver` may use its reference count as a hint about how
many threads it should allocate for concurrent DNS resolutions.

# Returns

the default `Resolver`.
<!-- trait ResolverExt::fn lookup_by_address -->
Synchronously reverse-resolves `address` to determine its
associated hostname.

If the DNS resolution fails, `error` (if non-`None`) will be set to
a value from `ResolverError`.

If `cancellable` is non-`None`, it can be used to cancel the
operation, in which case `error` (if non-`None`) will be set to
`IOErrorEnum::Cancelled`.
## `address`
the address to reverse-resolve
## `cancellable`
a `Cancellable`, or `None`

# Returns

a hostname (either ASCII-only, or in ASCII-encoded
 form), or `None` on error.
<!-- trait ResolverExt::fn lookup_by_address_async -->
Begins asynchronously reverse-resolving `address` to determine its
associated hostname, and eventually calls `callback`, which must
call `ResolverExt::lookup_by_address_finish` to get the final result.
## `address`
the address to reverse-resolve
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call after resolution completes
## `user_data`
data for `callback`
<!-- trait ResolverExt::fn lookup_by_address_finish -->
Retrieves the result of a previous call to
`ResolverExt::lookup_by_address_async`.

If the DNS resolution failed, `error` (if non-`None`) will be set to
a value from `ResolverError`. If the operation was cancelled,
`error` will be set to `IOErrorEnum::Cancelled`.
## `result`
the result passed to your `GAsyncReadyCallback`

# Returns

a hostname (either ASCII-only, or in ASCII-encoded
form), or `None` on error.
<!-- trait ResolverExt::fn lookup_by_name -->
Synchronously resolves `hostname` to determine its associated IP
address(es). `hostname` may be an ASCII-only or UTF-8 hostname, or
the textual form of an IP address (in which case this just becomes
a wrapper around `InetAddress::new_from_string`).

On success, `ResolverExt::lookup_by_name` will return a non-empty `glib::List` of
`InetAddress`, sorted in order of preference and guaranteed to not
contain duplicates. That is, if using the result to connect to
`hostname`, you should attempt to connect to the first address
first, then the second if the first fails, etc. If you are using
the result to listen on a socket, it is appropriate to add each
result using e.g. `SocketListenerExt::add_address`.

If the DNS resolution fails, `error` (if non-`None`) will be set to a
value from `ResolverError` and `None` will be returned.

If `cancellable` is non-`None`, it can be used to cancel the
operation, in which case `error` (if non-`None`) will be set to
`IOErrorEnum::Cancelled`.

If you are planning to connect to a socket on the resolved IP
address, it may be easier to create a `NetworkAddress` and use its
`SocketConnectable` interface.
## `hostname`
the hostname to look up
## `cancellable`
a `Cancellable`, or `None`

# Returns

a non-empty `glib::List`
of `InetAddress`, or `None` on error. You
must unref each of the addresses and free the list when you are
done with it. (You can use `Resolver::free_addresses` to do this.)
<!-- trait ResolverExt::fn lookup_by_name_async -->
Begins asynchronously resolving `hostname` to determine its
associated IP address(es), and eventually calls `callback`, which
must call `ResolverExt::lookup_by_name_finish` to get the result.
See `ResolverExt::lookup_by_name` for more details.
## `hostname`
the hostname to look up the address of
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call after resolution completes
## `user_data`
data for `callback`
<!-- trait ResolverExt::fn lookup_by_name_finish -->
Retrieves the result of a call to
`ResolverExt::lookup_by_name_async`.

If the DNS resolution failed, `error` (if non-`None`) will be set to
a value from `ResolverError`. If the operation was cancelled,
`error` will be set to `IOErrorEnum::Cancelled`.
## `result`
the result passed to your `GAsyncReadyCallback`

# Returns

a `glib::List`
of `InetAddress`, or `None` on error. See `ResolverExt::lookup_by_name`
for more details.
<!-- trait ResolverExt::fn lookup_records -->
Synchronously performs a DNS record lookup for the given `rrname` and returns
a list of records as `glib::Variant` tuples. See `ResolverRecordType` for
information on what the records contain for each `record_type`.

If the DNS resolution fails, `error` (if non-`None`) will be set to
a value from `ResolverError` and `None` will be returned.

If `cancellable` is non-`None`, it can be used to cancel the
operation, in which case `error` (if non-`None`) will be set to
`IOErrorEnum::Cancelled`.
## `rrname`
the DNS name to lookup the record for
## `record_type`
the type of DNS record to lookup
## `cancellable`
a `Cancellable`, or `None`

# Returns

a non-empty `glib::List` of
`glib::Variant`, or `None` on error. You must free each of the records and the list
when you are done with it. (You can use `glib::List::free_full` with
`glib::Variant::unref` to do this.)
<!-- trait ResolverExt::fn lookup_records_async -->
Begins asynchronously performing a DNS lookup for the given
`rrname`, and eventually calls `callback`, which must call
`ResolverExt::lookup_records_finish` to get the final result. See
`ResolverExt::lookup_records` for more details.
## `rrname`
the DNS name to lookup the record for
## `record_type`
the type of DNS record to lookup
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call after resolution completes
## `user_data`
data for `callback`
<!-- trait ResolverExt::fn lookup_records_finish -->
Retrieves the result of a previous call to
`ResolverExt::lookup_records_async`. Returns a non-empty list of records as
`glib::Variant` tuples. See `ResolverRecordType` for information on what the
records contain.

If the DNS resolution failed, `error` (if non-`None`) will be set to
a value from `ResolverError`. If the operation was cancelled,
`error` will be set to `IOErrorEnum::Cancelled`.
## `result`
the result passed to your `GAsyncReadyCallback`

# Returns

a non-empty `glib::List` of
`glib::Variant`, or `None` on error. You must free each of the records and the list
when you are done with it. (You can use `glib::List::free_full` with
`glib::Variant::unref` to do this.)
<!-- trait ResolverExt::fn lookup_service -->
Synchronously performs a DNS SRV lookup for the given `service` and
`protocol` in the given `domain` and returns an array of `SrvTarget`.
`domain` may be an ASCII-only or UTF-8 hostname. Note also that the
`service` and `protocol` arguments do not include the leading underscore
that appears in the actual DNS entry.

On success, `ResolverExt::lookup_service` will return a non-empty `glib::List` of
`SrvTarget`, sorted in order of preference. (That is, you should
attempt to connect to the first target first, then the second if
the first fails, etc.)

If the DNS resolution fails, `error` (if non-`None`) will be set to
a value from `ResolverError` and `None` will be returned.

If `cancellable` is non-`None`, it can be used to cancel the
operation, in which case `error` (if non-`None`) will be set to
`IOErrorEnum::Cancelled`.

If you are planning to connect to the service, it is usually easier
to create a `NetworkService` and use its `SocketConnectable`
interface.
## `service`
the service type to look up (eg, "ldap")
## `protocol`
the networking protocol to use for `service` (eg, "tcp")
## `domain`
the DNS domain to look up the service in
## `cancellable`
a `Cancellable`, or `None`

# Returns

a non-empty `glib::List` of
`SrvTarget`, or `None` on error. You must free each of the targets and the
list when you are done with it. (You can use `Resolver::free_targets` to do
this.)
<!-- trait ResolverExt::fn lookup_service_async -->
Begins asynchronously performing a DNS SRV lookup for the given
`service` and `protocol` in the given `domain`, and eventually calls
`callback`, which must call `ResolverExt::lookup_service_finish` to
get the final result. See `ResolverExt::lookup_service` for more
details.
## `service`
the service type to look up (eg, "ldap")
## `protocol`
the networking protocol to use for `service` (eg, "tcp")
## `domain`
the DNS domain to look up the service in
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call after resolution completes
## `user_data`
data for `callback`
<!-- trait ResolverExt::fn lookup_service_finish -->
Retrieves the result of a previous call to
`ResolverExt::lookup_service_async`.

If the DNS resolution failed, `error` (if non-`None`) will be set to
a value from `ResolverError`. If the operation was cancelled,
`error` will be set to `IOErrorEnum::Cancelled`.
## `result`
the result passed to your `GAsyncReadyCallback`

# Returns

a non-empty `glib::List` of
`SrvTarget`, or `None` on error. See `ResolverExt::lookup_service` for more
details.
<!-- trait ResolverExt::fn set_default -->
Sets `self` to be the application's default resolver (reffing
`self`, and unreffing the previous default resolver, if any).
Future calls to `Resolver::get_default` will return this resolver.

This can be used if an application wants to perform any sort of DNS
caching or "pinning"; it can implement its own `Resolver` that
calls the original default resolver for DNS operations, and
implements its own cache policies on top of that, and then set
itself as the default resolver for all later code to use.
<!-- trait ResolverExt::fn connect_reload -->
Emitted when the resolver notices that the system resolver
configuration has changed.
<!-- enum ResolverRecordType -->
The type of record that `ResolverExt::lookup_records` or
`ResolverExt::lookup_records_async` should retrieve. The records are returned
as lists of `glib::Variant` tuples. Each record type has different values in
the variant tuples returned.

`ResolverRecordType::Srv` records are returned as variants with the signature
'(qqqs)', containing a guint16 with the priority, a guint16 with the
weight, a guint16 with the port, and a string of the hostname.

`ResolverRecordType::Mx` records are returned as variants with the signature
'(qs)', representing a guint16 with the preference, and a string containing
the mail exchanger hostname.

`ResolverRecordType::Txt` records are returned as variants with the signature
'(as)', representing an array of the strings in the text record.

`ResolverRecordType::Soa` records are returned as variants with the signature
'(ssuuuuu)', representing a string containing the primary name server, a
string containing the administrator, the serial as a guint32, the refresh
interval as guint32, the retry interval as a guint32, the expire timeout
as a guint32, and the ttl as a guint32.

`ResolverRecordType::Ns` records are returned as variants with the signature
'(s)', representing a string of the hostname of the name server.
<!-- enum ResolverRecordType::variant Srv -->
lookup DNS SRV records for a domain
<!-- enum ResolverRecordType::variant Mx -->
lookup DNS MX records for a domain
<!-- enum ResolverRecordType::variant Txt -->
lookup DNS TXT records for a name
<!-- enum ResolverRecordType::variant Soa -->
lookup DNS SOA records for a zone
<!-- enum ResolverRecordType::variant Ns -->
lookup DNS NS records for a domain
<!-- struct Resource -->
Applications and libraries often contain binary or textual data that is
really part of the application, rather than user data. For instance
``GtkBuilder`` .ui files, splashscreen images, GMenu markup XML, CSS files,
icons, etc. These are often shipped as files in `$datadir/appname`, or
manually included as literal strings in the code.

The `Resource` API and the [glib-compile-resources][glib-compile-resources] program
provide a convenient and efficient alternative to this which has some nice properties. You
maintain the files as normal files, so its easy to edit them, but during the build the files
are combined into a binary bundle that is linked into the executable. This means that loading
the resource files are efficient (as they are already in memory, shared with other instances) and
simple (no need to check for things like I/O errors or locate the files in the filesystem). It
also makes it easier to create relocatable applications.

Resource files can also be marked as compressed. Such files will be included in the resource bundle
in a compressed form, but will be automatically uncompressed when the resource is used. This
is very useful e.g. for larger text files that are parsed once (or rarely) and then thrown away.

Resource files can also be marked to be preprocessed, by setting the value of the
`preprocess` attribute to a comma-separated list of preprocessing options.
The only options currently supported are:

`xml-stripblanks` which will use the xmllint command
to strip ignorable whitespace from the XML file. For this to work,
the `XMLLINT` environment variable must be set to the full path to
the xmllint executable, or xmllint must be in the `PATH`; otherwise
the preprocessing step is skipped.

`to-pixdata` which will use the gdk-pixbuf-pixdata command to convert
images to the `GdkPixdata` format, which allows you to create pixbufs directly using the data inside
the resource file, rather than an (uncompressed) copy if it. For this, the gdk-pixbuf-pixdata
program must be in the PATH, or the `GDK_PIXBUF_PIXDATA` environment variable must be
set to the full path to the gdk-pixbuf-pixdata executable; otherwise the resource compiler will
abort.

Resource files will be exported in the GResource namespace using the
combination of the given `prefix` and the filename from the `file` element.
The `alias` attribute can be used to alter the filename to expose them at a
different location in the resource namespace. Typically, this is used to
include files from a different source directory without exposing the source
directory in the resource namespace, as in the example below.

Resource bundles are created by the [glib-compile-resources][glib-compile-resources] program
which takes an XML file that describes the bundle, and a set of files that the XML references. These
are combined into a binary resource bundle.

An example resource description:

```text
<?xml version="1.0" encoding="UTF-8"?>
<gresources>
  <gresource prefix="/org/gtk/Example">
    <file>data/splashscreen.png</file>
    <file compressed="true">dialog.ui</file>
    <file preprocess="xml-stripblanks">menumarkup.xml</file>
    <file alias="example.css">data/example.css</file>
  </gresource>
</gresources>
```

This will create a resource bundle with the following files:

```text
/org/gtk/Example/data/splashscreen.png
/org/gtk/Example/dialog.ui
/org/gtk/Example/menumarkup.xml
/org/gtk/Example/example.css
```

Note that all resources in the process share the same namespace, so use Java-style
path prefixes (like in the above example) to avoid conflicts.

You can then use [glib-compile-resources][glib-compile-resources] to compile the XML to a
binary bundle that you can load with `Resource::load`. However, its more common to use the --generate-source and
--generate-header arguments to create a source file and header to link directly into your application.
This will generate `get_resource()`, `register_resource()` and
`unregister_resource()` functions, prefixed by the `--c-name` argument passed
to [glib-compile-resources][glib-compile-resources]. `get_resource()` returns
the generated `Resource` object. The register and unregister functions
register the resource so its files can be accessed using
`g_resources_lookup_data`.

Once a `Resource` has been created and registered all the data in it can be accessed globally in the process by
using API calls like `g_resources_open_stream` to stream the data or `g_resources_lookup_data` to get a direct pointer
to the data. You can also use URIs like "resource:///org/gtk/Example/data/splashscreen.png" with `File` to access
the resource data.

Some higher-level APIs, such as ``GtkApplication``, will automatically load
resources from certain well-known paths in the resource namespace as a
convenience. See the documentation for those APIs for details.

There are two forms of the generated source, the default version uses the compiler support for constructor
and destructor functions (where available) to automatically create and register the `Resource` on startup
or library load time. If you pass `--manual-register`, two functions to register/unregister the resource are created
instead. This requires an explicit initialization call in your application/library, but it works on all platforms,
even on the minor ones where constructors are not supported. (Constructor support is available for at least Win32, Mac OS and Linux.)

Note that resource data can point directly into the data segment of e.g. a library, so if you are unloading libraries
during runtime you need to be very careful with keeping around pointers to data from a resource, as this goes away
when the library is unloaded. However, in practice this is not generally a problem, since most resource accesses
are for your own resources, and resource data is often used once, during parsing, and then released.

When debugging a program or testing a change to an installed version, it is often useful to be able to
replace resources in the program or library, without recompiling, for debugging or quick hacking and testing
purposes. Since GLib 2.50, it is possible to use the `G_RESOURCE_OVERLAYS` environment variable to selectively overlay
resources with replacements from the filesystem. It is a colon-separated list of substitutions to perform
during resource lookups.

A substitution has the form


```text
   /org/gtk/libgtk=/home/desrt/gtk-overlay
```

The part before the `=` is the resource subpath for which the overlay applies. The part after is a
filesystem path which contains files and subdirectories as you would like to be loaded as resources with the
equivalent names.

In the example above, if an application tried to load a resource with the resource path
`/org/gtk/libgtk/ui/gtkdialog.ui` then GResource would check the filesystem path
`/home/desrt/gtk-overlay/ui/gtkdialog.ui`. If a file was found there, it would be used instead. This is an
overlay, not an outright replacement, which means that if a file is not found at that path, the built-in
version will be used instead. Whiteouts are not currently supported.

Substitutions must start with a slash, and must not contain a trailing slash before the '='. The path after
the slash should ideally be absolute, but this is not strictly required. It is possible to overlay the
location of a single resource with an individual file.
<!-- impl Resource::fn new_from_data -->
Creates a GResource from a reference to the binary resource bundle.
This will keep a reference to `data` while the resource lives, so
the data should not be modified or freed.

If you want to use this resource in the global resource namespace you need
to register it with `g_resources_register`.

Note: `data` must be backed by memory that is at least pointer aligned.
Otherwise this function will internally create a copy of the memory since
GLib 2.56, or in older versions fail and exit the process.

If `data` is empty or corrupt, `ResourceError::Internal` will be returned.
## `data`
A `glib::Bytes`

# Returns

a new `Resource`, or `None` on error
<!-- impl Resource::fn enumerate_children -->
Returns all the names of children at the specified `path` in the resource.
The return result is a `None` terminated list of strings which should
be released with `g_strfreev`.

If `path` is invalid or does not exist in the `Resource`,
`ResourceError::NotFound` will be returned.

`lookup_flags` controls the behaviour of the lookup.
## `path`
A pathname inside the resource
## `lookup_flags`
A `ResourceLookupFlags`

# Returns

an array of constant strings
<!-- impl Resource::fn get_info -->
Looks for a file at the specified `path` in the resource and
if found returns information about it.

`lookup_flags` controls the behaviour of the lookup.
## `path`
A pathname inside the resource
## `lookup_flags`
A `ResourceLookupFlags`
## `size`
a location to place the length of the contents of the file,
 or `None` if the length is not needed
## `flags`
a location to place the flags about the file,
 or `None` if the length is not needed

# Returns

`true` if the file was found. `false` if there were errors
<!-- impl Resource::fn lookup_data -->
Looks for a file at the specified `path` in the resource and
returns a `glib::Bytes` that lets you directly access the data in
memory.

The data is always followed by a zero byte, so you
can safely use the data as a C string. However, that byte
is not included in the size of the GBytes.

For uncompressed resource files this is a pointer directly into
the resource bundle, which is typically in some readonly data section
in the program binary. For compressed files we allocate memory on
the heap and automatically uncompress the data.

`lookup_flags` controls the behaviour of the lookup.
## `path`
A pathname inside the resource
## `lookup_flags`
A `ResourceLookupFlags`

# Returns

`glib::Bytes` or `None` on error.
 Free the returned object with `glib::Bytes::unref`
<!-- impl Resource::fn open_stream -->
Looks for a file at the specified `path` in the resource and
returns a `InputStream` that lets you read the data.

`lookup_flags` controls the behaviour of the lookup.
## `path`
A pathname inside the resource
## `lookup_flags`
A `ResourceLookupFlags`

# Returns

`InputStream` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`
<!-- impl Resource::fn ref -->
Atomically increments the reference count of `self` by one. This
function is MT-safe and may be called from any thread.

# Returns

The passed in `Resource`
<!-- impl Resource::fn unref -->
Atomically decrements the reference count of `self` by one. If the
reference count drops to 0, all memory allocated by the resource is
released. This function is MT-safe and may be called from any
thread.
<!-- impl Resource::fn load -->
Loads a binary resource bundle and creates a `Resource` representation of it, allowing
you to query it for data.

If you want to use this resource in the global resource namespace you need
to register it with `g_resources_register`.

If `filename` is empty or the data in it is corrupt,
`ResourceError::Internal` will be returned. If `filename` doesn’t exist, or
there is an error in reading it, an error from `glib::MappedFile::new` will be
returned.
## `filename`
the path of a filename to load, in the GLib filename encoding

# Returns

a new `Resource`, or `None` on error
<!-- enum ResourceError -->
An error code used with `G_RESOURCE_ERROR` in a `glib::Error` returned
from a `Resource` routine.
<!-- enum ResourceError::variant NotFound -->
no file was found at the requested path
<!-- enum ResourceError::variant Internal -->
unknown error
<!-- struct Seekable -->
`Seekable` is implemented by streams (implementations of
`InputStream` or `OutputStream`) that support seeking.

Seekable streams largely fall into two categories: resizable and
fixed-size.

`Seekable` on fixed-sized streams is approximately the same as POSIX
`lseek` on a block device (for example: attmepting to seek past the
end of the device is an error). Fixed streams typically cannot be
truncated.

`Seekable` on resizable streams is approximately the same as POSIX
`lseek` on a normal file. Seeking past the end and writing data will
usually cause the stream to resize by introducing zero bytes.

# Implements

[`SeekableExt`](trait.SeekableExt.html)
<!-- trait SeekableExt -->
Trait containing all `Seekable` methods.

# Implementors

[`BufferedInputStream`](struct.BufferedInputStream.html), [`BufferedOutputStream`](struct.BufferedOutputStream.html), [`DataInputStream`](struct.DataInputStream.html), [`DataOutputStream`](struct.DataOutputStream.html), [`FileIOStream`](struct.FileIOStream.html), [`FileInputStream`](struct.FileInputStream.html), [`FileOutputStream`](struct.FileOutputStream.html), [`MemoryInputStream`](struct.MemoryInputStream.html), [`MemoryOutputStream`](struct.MemoryOutputStream.html), [`Seekable`](struct.Seekable.html)
<!-- trait SeekableExt::fn can_seek -->
Tests if the stream supports the `SeekableIface`.

# Returns

`true` if `self` can be seeked. `false` otherwise.
<!-- trait SeekableExt::fn can_truncate -->
Tests if the length of the stream can be adjusted with
`Seekable::truncate`.

# Returns

`true` if the stream can be truncated, `false` otherwise.
<!-- trait SeekableExt::fn seek -->
Seeks in the stream by the given `offset`, modified by `type_`.

Attempting to seek past the end of the stream will have different
results depending on if the stream is fixed-sized or resizable. If
the stream is resizable then seeking past the end and then writing
will result in zeros filling the empty space. Seeking past the end
of a resizable stream and reading will result in EOF. Seeking past
the end of a fixed-sized stream will fail.

Any operation that would result in a negative offset will fail.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `offset`
a `goffset`.
## `type_`
a `glib::SeekType`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if successful. If an error
 has occurred, this function will return `false` and set `error`
 appropriately if present.
<!-- trait SeekableExt::fn tell -->
Tells the current position within the stream.

# Returns

the offset from the beginning of the buffer.
<!-- trait SeekableExt::fn truncate -->
Sets the length of the stream to `offset`. If the stream was previously
larger than `offset`, the extra data is discarded. If the stream was
previouly shorter than `offset`, it is extended with NUL ('\0') bytes.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned. If an
operation was partially finished when the operation was cancelled the
partial result will be returned, without an error.
## `offset`
new length for `self`, in bytes.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

`true` if successful. If an error
 has occurred, this function will return `false` and set `error`
 appropriately if present.
<!-- struct Settings -->
The `Settings` class provides a convenient API for storing and retrieving
application settings.

Reads and writes can be considered to be non-blocking. Reading
settings with `Settings` is typically extremely fast: on
approximately the same order of magnitude (but slower than) a
`glib::HashTable` lookup. Writing settings is also extremely fast in terms
of time to return to your application, but can be extremely expensive
for other threads and other processes. Many settings backends
(including dconf) have lazy initialisation which means in the common
case of the user using their computer without modifying any settings
a lot of work can be avoided. For dconf, the D-Bus service doesn't
even need to be started in this case. For this reason, you should
only ever modify `Settings` keys in response to explicit user action.
Particular care should be paid to ensure that modifications are not
made during startup -- for example, when setting the initial value
of preferences widgets. The built-in `SettingsExt::bind` functionality
is careful not to write settings in response to notify signals as a
result of modifications that it makes to widgets.

When creating a GSettings instance, you have to specify a schema
that describes the keys in your settings and their types and default
values, as well as some other information.

Normally, a schema has a fixed path that determines where the settings
are stored in the conceptual global tree of settings. However, schemas
can also be '[relocatable][gsettings-relocatable]', i.e. not equipped with
a fixed path. This is
useful e.g. when the schema describes an 'account', and you want to be
able to store a arbitrary number of accounts.

Paths must start with and end with a forward slash character ('/')
and must not contain two sequential slash characters. Paths should
be chosen based on a domain name associated with the program or
library to which the settings belong. Examples of paths are
"/org/gtk/settings/file-chooser/" and "/ca/desrt/dconf-editor/".
Paths should not start with "/apps/", "/desktop/" or "/system/" as
they often did in GConf.

Unlike other configuration systems (like GConf), GSettings does not
restrict keys to basic types like strings and numbers. GSettings stores
values as `glib::Variant`, and allows any `glib::VariantType` for keys. Key names
are restricted to lowercase characters, numbers and '-'. Furthermore,
the names must begin with a lowercase character, must not end
with a '-', and must not contain consecutive dashes.

Similar to GConf, the default values in GSettings schemas can be
localized, but the localized values are stored in gettext catalogs
and looked up with the domain that is specified in the
`gettext-domain` attribute of the `<schemalist>` or `<schema>`
elements and the category that is specified in the `l10n` attribute of
the `<default>` element. The string which is translated includes all text in
the `<default>` element, including any surrounding quotation marks.

The `l10n` attribute must be set to `messages` or `time`, and sets the
[locale category for
translation](https://www.gnu.org/software/gettext/manual/html_node/Aspects.html`index`-locale-categories-1).
The `messages` category should be used by default; use `time` for
translatable date or time formats. A translation comment can be added as an
XML comment immediately above the `<default>` element — it is recommended to
add these comments to aid translators understand the meaning and
implications of the default value. An optional translation `context`
attribute can be set on the `<default>` element to disambiguate multiple
defaults which use the same string.

For example:

```text
 <!-- Translators: A list of words which are not allowed to be typed, in
      GVariant serialization syntax.
      See: https://developer.gnome.org/glib/stable/gvariant-text.html -->
 <default l10n='messages' context='Banned words'>['bad', 'words']</default>
```

Translations of default values must remain syntactically valid serialized
`GVariants` (e.g. retaining any surrounding quotation marks) or runtime
errors will occur.

GSettings uses schemas in a compact binary form that is created
by the [glib-compile-schemas][glib-compile-schemas]
utility. The input is a schema description in an XML format.

A DTD for the gschema XML format can be found here:
[gschema.dtd](https://git.gnome.org/browse/glib/tree/gio/gschema.dtd)

The [glib-compile-schemas][glib-compile-schemas] tool expects schema
files to have the extension `.gschema.xml`.

At runtime, schemas are identified by their id (as specified in the
id attribute of the `<schema>` element). The convention for schema
ids is to use a dotted name, similar in style to a D-Bus bus name,
e.g. "org.gnome.SessionManager". In particular, if the settings are
for a specific service that owns a D-Bus bus name, the D-Bus bus name
and schema id should match. For schemas which deal with settings not
associated with one named application, the id should not use
StudlyCaps, e.g. "org.gnome.font-rendering".

In addition to `glib::Variant` types, keys can have types that have
enumerated types. These can be described by a `<choice>`,
`<enum>` or `<flags>` element, as seen in the
[example][schema-enumerated]. The underlying type of such a key
is string, but you can use `SettingsExt::get_enum`, `SettingsExt::set_enum`,
`SettingsExt::get_flags`, `SettingsExt::set_flags` access the numeric values
corresponding to the string value of enum and flags keys.

An example for default value:

```text
<schemalist>
  <schema id="org.gtk.Test" path="/org/gtk/Test/" gettext-domain="test">

    <key name="greeting" type="s">
      <default l10n="messages">"Hello, earthlings"</default>
      <summary>A greeting</summary>
      <description>
        Greeting of the invading martians
      </description>
    </key>

    <key name="box" type="(ii)">
      <default>(20,30)</default>
    </key>

  </schema>
</schemalist>
```

An example for ranges, choices and enumerated types:

```text
<schemalist>

  <enum id="org.gtk.Test.myenum">
    <value nick="first" value="1"/>
    <value nick="second" value="2"/>
  </enum>

  <flags id="org.gtk.Test.myflags">
    <value nick="flag1" value="1"/>
    <value nick="flag2" value="2"/>
    <value nick="flag3" value="4"/>
  </flags>

  <schema id="org.gtk.Test">

    <key name="key-with-range" type="i">
      <range min="1" max="100"/>
      <default>10</default>
    </key>

    <key name="key-with-choices" type="s">
      <choices>
        <choice value='Elisabeth'/>
        <choice value='Annabeth'/>
        <choice value='Joe'/>
      </choices>
      <aliases>
        <alias value='Anna' target='Annabeth'/>
        <alias value='Beth' target='Elisabeth'/>
      </aliases>
      <default>'Joe'</default>
    </key>

    <key name='enumerated-key' enum='org.gtk.Test.myenum'>
      <default>'first'</default>
    </key>

    <key name='flags-key' flags='org.gtk.Test.myflags'>
      <default>["flag1","flag2"]</default>
    </key>
  </schema>
</schemalist>
```

## Vendor overrides

Default values are defined in the schemas that get installed by
an application. Sometimes, it is necessary for a vendor or distributor
to adjust these defaults. Since patching the XML source for the schema
is inconvenient and error-prone,
[glib-compile-schemas][glib-compile-schemas] reads so-called vendor
override' files. These are keyfiles in the same directory as the XML
schema sources which can override default values. The schema id serves
as the group name in the key file, and the values are expected in
serialized GVariant form, as in the following example:

```text
    [org.gtk.Example]
    key1='string'
    key2=1.5
```

glib-compile-schemas expects schema files to have the extension
`.gschema.override`.

## Binding

A very convenient feature of GSettings lets you bind `gobject::Object` properties
directly to settings, using `SettingsExt::bind`. Once a GObject property
has been bound to a setting, changes on either side are automatically
propagated to the other side. GSettings handles details like mapping
between GObject and GVariant types, and preventing infinite cycles.

This makes it very easy to hook up a preferences dialog to the
underlying settings. To make this even more convenient, GSettings
looks for a boolean property with the name "sensitivity" and
automatically binds it to the writability of the bound setting.
If this 'magic' gets in the way, it can be suppressed with the
`SettingsBindFlags::NoSensitivity` flag.

## Relocatable schemas # {`gsettings`-relocatable}

A relocatable schema is one with no `path` attribute specified on its
`<schema>` element. By using `Settings::new_with_path`, a `Settings` object
can be instantiated for a relocatable schema, assigning a path to the
instance. Paths passed to `Settings::new_with_path` will typically be
constructed dynamically from a constant prefix plus some form of instance
identifier; but they must still be valid GSettings paths. Paths could also
be constant and used with a globally installed schema originating from a
dependency library.

For example, a relocatable schema could be used to store geometry information
for different windows in an application. If the schema ID was
`org.foo.MyApp.Window`, it could be instantiated for paths
`/org/foo/MyApp/main/`, `/org/foo/MyApp/document-1/`,
`/org/foo/MyApp/document-2/`, etc. If any of the paths are well-known
they can be specified as `<child>` elements in the parent schema, e.g.:

```text
<schema id="org.foo.MyApp" path="/org/foo/MyApp/">
  <child name="main" schema="org.foo.MyApp.Window"/>
</schema>
```

## Build system integration # {`gsettings`-build-system}

GSettings comes with autotools integration to simplify compiling and
installing schemas. To add GSettings support to an application, add the
following to your `configure.ac`:

```text
GLIB_GSETTINGS
```

In the appropriate `Makefile.am`, use the following snippet to compile and
install the named schema:

```text
gsettings_SCHEMAS = org.foo.MyApp.gschema.xml
EXTRA_DIST = $(gsettings_SCHEMAS)

@GSETTINGS_RULES@
```

No changes are needed to the build system to mark a schema XML file for
translation. Assuming it sets the `gettext-domain` attribute, a schema may
be marked for translation by adding it to `POTFILES.in`, assuming gettext
0.19 is in use (the preferred method for translation):

```text
data/org.foo.MyApp.gschema.xml
```

Alternatively, if intltool 0.50.1 is in use:

```text
[type: gettext/gsettings]data/org.foo.MyApp.gschema.xml
```

GSettings will use gettext to look up translations for the `<summary>` and
`<description>` elements, and also any `<default>` elements which have a `l10n`
attribute set. Translations must not be included in the `.gschema.xml` file
by the build system, for example by using intltool XML rules with a
`.gschema.xml.in` template.

If an enumerated type defined in a C header file is to be used in a GSettings
schema, it can either be defined manually using an `<enum>` element in the
schema XML, or it can be extracted automatically from the C header. This
approach is preferred, as it ensures the two representations are always
synchronised. To do so, add the following to the relevant `Makefile.am`:

```text
gsettings_ENUM_NAMESPACE = org.foo.MyApp
gsettings_ENUM_FILES = my-app-enums.h my-app-misc.h
```

`gsettings_ENUM_NAMESPACE` specifies the schema namespace for the enum files,
which are specified in `gsettings_ENUM_FILES`. This will generate a
`org.foo.MyApp.enums.xml` file containing the extracted enums, which will be
automatically included in the schema compilation, install and uninstall
rules. It should not be committed to version control or included in
`EXTRA_DIST`.

# Implements

[`SettingsExt`](trait.SettingsExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait SettingsExt -->
Trait containing all `Settings` methods.

# Implementors

[`Settings`](struct.Settings.html)
<!-- impl Settings::fn new -->
Creates a new `Settings` object with the schema specified by
`schema_id`.

Signals on the newly created `Settings` object will be dispatched
via the thread-default `glib::MainContext` in effect at the time of the
call to `Settings::new`. The new `Settings` will hold a reference
on the context. See `glib::MainContext::push_thread_default`.
## `schema_id`
the id of the schema

# Returns

a new `Settings` object
<!-- impl Settings::fn new_full -->
Creates a new `Settings` object with a given schema, backend and
path.

It should be extremely rare that you ever want to use this function.
It is made available for advanced use-cases (such as plugin systems
that want to provide access to schemas loaded from custom locations,
etc).

At the most basic level, a `Settings` object is a pure composition of
4 things: a `SettingsSchema`, a `SettingsBackend`, a path within that
backend, and a `glib::MainContext` to which signals are dispatched.

This constructor therefore gives you full control over constructing
`Settings` instances. The first 3 parameters are given directly as
`schema`, `backend` and `path`, and the main context is taken from the
thread-default (as per `Settings::new`).

If `backend` is `None` then the default backend is used.

If `path` is `None` then the path from the schema is used. It is an
error if `path` is `None` and the schema has no path of its own or if
`path` is non-`None` and not equal to the path that the schema does
have.
## `schema`
a `SettingsSchema`
## `backend`
a `SettingsBackend`
## `path`
the path to use

# Returns

a new `Settings` object
<!-- impl Settings::fn new_with_backend -->
Creates a new `Settings` object with the schema specified by
`schema_id` and a given `SettingsBackend`.

Creating a `Settings` object with a different backend allows accessing
settings from a database other than the usual one. For example, it may make
sense to pass a backend corresponding to the "defaults" settings database on
the system to get a settings object that modifies the system default
settings instead of the settings for this user.
## `schema_id`
the id of the schema
## `backend`
the `SettingsBackend` to use

# Returns

a new `Settings` object
<!-- impl Settings::fn new_with_backend_and_path -->
Creates a new `Settings` object with the schema specified by
`schema_id` and a given `SettingsBackend` and path.

This is a mix of `Settings::new_with_backend` and
`Settings::new_with_path`.
## `schema_id`
the id of the schema
## `backend`
the `SettingsBackend` to use
## `path`
the path to use

# Returns

a new `Settings` object
<!-- impl Settings::fn new_with_path -->
Creates a new `Settings` object with the relocatable schema specified
by `schema_id` and a given path.

You only need to do this if you want to directly create a settings
object with a schema that doesn't have a specified path of its own.
That's quite rare.

It is a programmer error to call this function for a schema that
has an explicitly specified path.

It is a programmer error if `path` is not a valid path. A valid path
begins and ends with '/' and does not contain two consecutive '/'
characters.
## `schema_id`
the id of the schema
## `path`
the path to use

# Returns

a new `Settings` object
<!-- impl Settings::fn sync -->
Ensures that all pending operations are complete for the default backend.

Writes made to a `Settings` are handled asynchronously. For this
reason, it is very unlikely that the changes have it to disk by the
time `SettingsExt::set` returns.

This call will block until all of the writes have made it to the
backend. Since the mainloop is not running, no change notifications
will be dispatched during this call (but some may be queued by the
time the call is done).
<!-- impl Settings::fn unbind -->
Removes an existing binding for `property` on `object`.

Note that bindings are automatically removed when the
object is finalized, so it is rarely necessary to call this
function.
## `object`
the object
## `property`
the property whose binding is removed
<!-- trait SettingsExt::fn apply -->
Applies any changes that have been made to the settings. This
function does nothing unless `self` is in 'delay-apply' mode;
see `SettingsExt::delay`. In the normal case settings are always
applied immediately.
<!-- trait SettingsExt::fn bind -->
Create a binding between the `key` in the `self` object
and the property `property` of `object`.

The binding uses the default GIO mapping functions to map
between the settings and property values. These functions
handle booleans, numeric types and string types in a
straightforward way. Use `SettingsExt::bind_with_mapping` if
you need a custom mapping, or map between types that are not
supported by the default mapping functions.

Unless the `flags` include `SettingsBindFlags::NoSensitivity`, this
function also establishes a binding between the writability of
`key` and the "sensitive" property of `object` (if `object` has
a boolean property by that name). See `SettingsExt::bind_writable`
for more details about writable bindings.

Note that the lifecycle of the binding is tied to `object`,
and that you can have only one binding per object property.
If you bind the same property twice on the same object, the second
binding overrides the first one.
## `key`
the key to bind
## `object`
a `gobject::Object`
## `property`
the name of the property to bind
## `flags`
flags for the binding
<!-- trait SettingsExt::fn bind_with_mapping -->
Create a binding between the `key` in the `self` object
and the property `property` of `object`.

The binding uses the provided mapping functions to map between
settings and property values.

Note that the lifecycle of the binding is tied to `object`,
and that you can have only one binding per object property.
If you bind the same property twice on the same object, the second
binding overrides the first one.
## `key`
the key to bind
## `object`
a `gobject::Object`
## `property`
the name of the property to bind
## `flags`
flags for the binding
## `get_mapping`
a function that gets called to convert values
 from `self` to `object`, or `None` to use the default GIO mapping
## `set_mapping`
a function that gets called to convert values
 from `object` to `self`, or `None` to use the default GIO mapping
## `user_data`
data that gets passed to `get_mapping` and `set_mapping`
## `destroy`
`GDestroyNotify` function for `user_data`
<!-- trait SettingsExt::fn bind_writable -->
Create a binding between the writability of `key` in the
`self` object and the property `property` of `object`.
The property must be boolean; "sensitive" or "visible"
properties of widgets are the most likely candidates.

Writable bindings are always uni-directional; changes of the
writability of the setting will be propagated to the object
property, not the other way.

When the `inverted` argument is `true`, the binding inverts the
value as it passes from the setting to the object, i.e. `property`
will be set to `true` if the key is not writable.

Note that the lifecycle of the binding is tied to `object`,
and that you can have only one binding per object property.
If you bind the same property twice on the same object, the second
binding overrides the first one.
## `key`
the key to bind
## `object`
a `gobject::Object`
## `property`
the name of a boolean property to bind
## `inverted`
whether to 'invert' the value
<!-- trait SettingsExt::fn create_action -->
Creates a `Action` corresponding to a given `Settings` key.

The action has the same name as the key.

The value of the key becomes the state of the action and the action
is enabled when the key is writable. Changing the state of the
action results in the key being written to. Changes to the value or
writability of the key cause appropriate change notifications to be
emitted for the action.

For boolean-valued keys, action activations take no parameter and
result in the toggling of the value. For all other types,
activations take the new value for the key (which must have the
correct type).
## `key`
the name of a key in `self`

# Returns

a new `Action`
<!-- trait SettingsExt::fn delay -->
Changes the `Settings` object into 'delay-apply' mode. In this
mode, changes to `self` are not immediately propagated to the
backend, but kept locally until `SettingsExt::apply` is called.
<!-- trait SettingsExt::fn get -->
Gets the value that is stored at `key` in `self`.

A convenience function that combines `SettingsExt::get_value` with
`glib::Variant::get`.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or for the `glib::VariantType` of `format` to mismatch
the type given in the schema.
## `key`
the key to get the value for
## `format`
a `glib::Variant` format string
<!-- trait SettingsExt::fn get_boolean -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for booleans.

It is a programmer error to give a `key` that isn't specified as
having a boolean type in the schema for `self`.
## `key`
the key to get the value for

# Returns

a boolean
<!-- trait SettingsExt::fn get_child -->
Creates a child settings object which has a base path of
`base-path/@name`, where `base-path` is the base path of
`self`.

The schema for the child settings object must have been declared
in the schema of `self` using a `<child>` element.
## `name`
the name of the child schema

# Returns

a 'child' settings object
<!-- trait SettingsExt::fn get_default_value -->
Gets the "default value" of a key.

This is the value that would be read if `SettingsExt::reset` were to be
called on the key.

Note that this may be a different value than returned by
`SettingsSchemaKey::get_default_value` if the system administrator
has provided a default value.

Comparing the return values of `SettingsExt::get_default_value` and
`SettingsExt::get_value` is not sufficient for determining if a value
has been set because the user may have explicitly set the value to
something that happens to be equal to the default. The difference
here is that if the default changes in the future, the user's key
will still be set.

This function may be useful for adding an indication to a UI of what
the default value was before the user set it.

It is a programmer error to give a `key` that isn't contained in the
schema for `self`.
## `key`
the key to get the default value for

# Returns

the default value
<!-- trait SettingsExt::fn get_double -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for doubles.

It is a programmer error to give a `key` that isn't specified as
having a 'double' type in the schema for `self`.
## `key`
the key to get the value for

# Returns

a double
<!-- trait SettingsExt::fn get_enum -->
Gets the value that is stored in `self` for `key` and converts it
to the enum value that it represents.

In order to use this function the type of the value must be a string
and it must be marked in the schema file as an enumerated type.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or is not marked as an enumerated type.

If the value stored in the configuration database is not a valid
value for the enumerated type then this function will return the
default value.
## `key`
the key to get the value for

# Returns

the enum value
<!-- trait SettingsExt::fn get_flags -->
Gets the value that is stored in `self` for `key` and converts it
to the flags value that it represents.

In order to use this function the type of the value must be an array
of strings and it must be marked in the schema file as an flags type.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or is not marked as a flags type.

If the value stored in the configuration database is not a valid
value for the flags type then this function will return the default
value.
## `key`
the key to get the value for

# Returns

the flags value
<!-- trait SettingsExt::fn get_has_unapplied -->
Returns whether the `Settings` object has any unapplied
changes. This can only be the case if it is in 'delayed-apply' mode.

# Returns

`true` if `self` has unapplied changes
<!-- trait SettingsExt::fn get_int -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for 32-bit integers.

It is a programmer error to give a `key` that isn't specified as
having a int32 type in the schema for `self`.
## `key`
the key to get the value for

# Returns

an integer
<!-- trait SettingsExt::fn get_int64 -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for 64-bit integers.

It is a programmer error to give a `key` that isn't specified as
having a int64 type in the schema for `self`.

Feature: `v2_50`

## `key`
the key to get the value for

# Returns

a 64-bit integer
<!-- trait SettingsExt::fn get_mapped -->
Gets the value that is stored at `key` in `self`, subject to
application-level validation/mapping.

You should use this function when the application needs to perform
some processing on the value of the key (for example, parsing). The
`mapping` function performs that processing. If the function
indicates that the processing was unsuccessful (due to a parse error,
for example) then the mapping is tried again with another value.

This allows a robust 'fall back to defaults' behaviour to be
implemented somewhat automatically.

The first value that is tried is the user's setting for the key. If
the mapping function fails to map this value, other values may be
tried in an unspecified order (system or site defaults, translated
schema default values, untranslated schema default values, etc).

If the mapping function fails for all possible values, one additional
attempt is made: the mapping function is called with a `None` value.
If the mapping function still indicates failure at this point then
the application will be aborted.

The result parameter for the `mapping` function is pointed to a
`gpointer` which is initially set to `None`. The same pointer is given
to each invocation of `mapping`. The final value of that `gpointer` is
what is returned by this function. `None` is valid; it is returned
just as any other value would be.
## `key`
the key to get the value for
## `mapping`
the function to map the value in the
 settings database to the value used by the application
## `user_data`
user data for `mapping`

# Returns

the result, which may be `None`
<!-- trait SettingsExt::fn get_string -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for strings.

It is a programmer error to give a `key` that isn't specified as
having a string type in the schema for `self`.
## `key`
the key to get the value for

# Returns

a newly-allocated string
<!-- trait SettingsExt::fn get_strv -->
A convenience variant of `SettingsExt::get` for string arrays.

It is a programmer error to give a `key` that isn't specified as
having an array of strings type in the schema for `self`.
## `key`
the key to get the value for

# Returns

a
newly-allocated, `None`-terminated array of strings, the value that
is stored at `key` in `self`.
<!-- trait SettingsExt::fn get_uint -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for 32-bit unsigned
integers.

It is a programmer error to give a `key` that isn't specified as
having a uint32 type in the schema for `self`.
## `key`
the key to get the value for

# Returns

an unsigned integer
<!-- trait SettingsExt::fn get_uint64 -->
Gets the value that is stored at `key` in `self`.

A convenience variant of `SettingsExt::get` for 64-bit unsigned
integers.

It is a programmer error to give a `key` that isn't specified as
having a uint64 type in the schema for `self`.

Feature: `v2_50`

## `key`
the key to get the value for

# Returns

a 64-bit unsigned integer
<!-- trait SettingsExt::fn get_user_value -->
Checks the "user value" of a key, if there is one.

The user value of a key is the last value that was set by the user.

After calling `SettingsExt::reset` this function should always return
`None` (assuming something is not wrong with the system
configuration).

It is possible that `SettingsExt::get_value` will return a different
value than this function. This can happen in the case that the user
set a value for a key that was subsequently locked down by the system
administrator -- this function will return the user's old value.

This function may be useful for adding a "reset" option to a UI or
for providing indication that a particular value has been changed.

It is a programmer error to give a `key` that isn't contained in the
schema for `self`.
## `key`
the key to get the user value for

# Returns

the user's value, if set
<!-- trait SettingsExt::fn get_value -->
Gets the value that is stored in `self` for `key`.

It is a programmer error to give a `key` that isn't contained in the
schema for `self`.
## `key`
the key to get the value for

# Returns

a new `glib::Variant`
<!-- trait SettingsExt::fn is_writable -->
Finds out if a key can be written or not
## `name`
the name of a key

# Returns

`true` if the key `name` is writable
<!-- trait SettingsExt::fn list_children -->
Gets the list of children on `self`.

The list is exactly the list of strings for which it is not an error
to call `SettingsExt::get_child`.

For GSettings objects that are lists, this value can change at any
time. Note that there is a race condition here: you may
request a child after listing it only for it to have been destroyed
in the meantime. For this reason, `SettingsExt::get_child` may return
`None` even for a child that was listed by this function.

For GSettings objects that are not lists, you should probably not be
calling this function from "normal" code (since you should already
know what children are in your schema). This function may still be
useful there for introspection reasons, however.

You should free the return value with `g_strfreev` when you are done
with it.

# Returns

a list of the children on `self`
<!-- trait SettingsExt::fn list_keys -->
Introspects the list of keys on `self`.

You should probably not be calling this function from "normal" code
(since you should already know what keys are in your schema). This
function is intended for introspection reasons.

You should free the return value with `g_strfreev` when you are done
with it.

# Returns

a list of the keys on `self`
<!-- trait SettingsExt::fn reset -->
Resets `key` to its default value.

This call resets the key, as much as possible, to its default value.
That might the value specified in the schema or the one set by the
administrator.
## `key`
the name of a key
<!-- trait SettingsExt::fn revert -->
Reverts all non-applied changes to the settings. This function
does nothing unless `self` is in 'delay-apply' mode; see
`SettingsExt::delay`. In the normal case settings are always applied
immediately.

Change notifications will be emitted for affected keys.
<!-- trait SettingsExt::fn set -->
Sets `key` in `self` to `value`.

A convenience function that combines `SettingsExt::set_value` with
`glib::Variant::new`.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or for the `glib::VariantType` of `format` to mismatch
the type given in the schema.
## `key`
the name of the key to set
## `format`
a `glib::Variant` format string

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_boolean -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for booleans.

It is a programmer error to give a `key` that isn't specified as
having a boolean type in the schema for `self`.
## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_double -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for doubles.

It is a programmer error to give a `key` that isn't specified as
having a 'double' type in the schema for `self`.
## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_enum -->
Looks up the enumerated type nick for `value` and writes it to `key`,
within `self`.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or is not marked as an enumerated type, or for
`value` not to be a valid value for the named type.

After performing the write, accessing `key` directly with
`SettingsExt::get_string` will return the 'nick' associated with
`value`.
## `key`
a key, within `self`
## `value`
an enumerated value

# Returns

`true`, if the set succeeds
<!-- trait SettingsExt::fn set_flags -->
Looks up the flags type nicks for the bits specified by `value`, puts
them in an array of strings and writes the array to `key`, within
`self`.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or is not marked as a flags type, or for `value`
to contain any bits that are not value for the named type.

After performing the write, accessing `key` directly with
`SettingsExt::get_strv` will return an array of 'nicks'; one for each
bit in `value`.
## `key`
a key, within `self`
## `value`
a flags value

# Returns

`true`, if the set succeeds
<!-- trait SettingsExt::fn set_int -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for 32-bit integers.

It is a programmer error to give a `key` that isn't specified as
having a int32 type in the schema for `self`.
## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_int64 -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for 64-bit integers.

It is a programmer error to give a `key` that isn't specified as
having a int64 type in the schema for `self`.

Feature: `v2_50`

## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_string -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for strings.

It is a programmer error to give a `key` that isn't specified as
having a string type in the schema for `self`.
## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_strv -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for string arrays. If
`value` is `None`, then `key` is set to be the empty array.

It is a programmer error to give a `key` that isn't specified as
having an array of strings type in the schema for `self`.
## `key`
the name of the key to set
## `value`
the value to set it to, or `None`

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_uint -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for 32-bit unsigned
integers.

It is a programmer error to give a `key` that isn't specified as
having a uint32 type in the schema for `self`.
## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_uint64 -->
Sets `key` in `self` to `value`.

A convenience variant of `SettingsExt::set` for 64-bit unsigned
integers.

It is a programmer error to give a `key` that isn't specified as
having a uint64 type in the schema for `self`.

Feature: `v2_50`

## `key`
the name of the key to set
## `value`
the value to set it to

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn set_value -->
Sets `key` in `self` to `value`.

It is a programmer error to give a `key` that isn't contained in the
schema for `self` or for `value` to have the incorrect type, per
the schema.

If `value` is floating then this function consumes the reference.
## `key`
the name of the key to set
## `value`
a `glib::Variant` of the correct type

# Returns

`true` if setting the key succeeded,
 `false` if the key was not writable
<!-- trait SettingsExt::fn connect_change_event -->
The "change-event" signal is emitted once per change event that
affects this settings object. You should connect to this signal
only if you are interested in viewing groups of changes before they
are split out into multiple emissions of the "changed" signal.
For most use cases it is more appropriate to use the "changed" signal.

In the event that the change event applies to one or more specified
keys, `keys` will be an array of `glib::Quark` of length `n_keys`. In the
event that the change event applies to the `Settings` object as a
whole (ie: potentially every key has been changed) then `keys` will
be `None` and `n_keys` will be 0.

The default handler for this signal invokes the "changed" signal
for each affected key. If any other connected handler returns
`true` then this default functionality will be suppressed.
## `keys`

 an array of `GQuarks` for the changed keys, or `None`
## `n_keys`
the length of the `keys` array, or 0

# Returns

`true` to stop other handlers from being invoked for the
 event. FALSE to propagate the event further.
<!-- trait SettingsExt::fn connect_changed -->
The "changed" signal is emitted when a key has potentially changed.
You should call one of the `SettingsExt::get` calls to check the new
value.

This signal supports detailed connections. You can connect to the
detailed signal "changed::x" in order to only receive callbacks
when key "x" changes.

Note that `settings` only emits this signal if you have read `key` at
least once while a signal handler was already connected for `key`.
## `key`
the name of the key that changed
<!-- trait SettingsExt::fn connect_writable_change_event -->
The "writable-change-event" signal is emitted once per writability
change event that affects this settings object. You should connect
to this signal if you are interested in viewing groups of changes
before they are split out into multiple emissions of the
"writable-changed" signal. For most use cases it is more
appropriate to use the "writable-changed" signal.

In the event that the writability change applies only to a single
key, `key` will be set to the `glib::Quark` for that key. In the event
that the writability change affects the entire settings object,
`key` will be 0.

The default handler for this signal invokes the "writable-changed"
and "changed" signals for each affected key. This is done because
changes in writability might also imply changes in value (if for
example, a new mandatory setting is introduced). If any other
connected handler returns `true` then this default functionality
will be suppressed.
## `key`
the quark of the key, or 0

# Returns

`true` to stop other handlers from being invoked for the
 event. FALSE to propagate the event further.
<!-- trait SettingsExt::fn connect_writable_changed -->
The "writable-changed" signal is emitted when the writability of a
key has potentially changed. You should call
`SettingsExt::is_writable` in order to determine the new status.

This signal supports detailed connections. You can connect to the
detailed signal "writable-changed::x" in order to only receive
callbacks when the writability of "x" changes.
## `key`
the key
<!-- trait SettingsExt::fn get_property_backend -->
The name of the context that the settings are stored in.
<!-- trait SettingsExt::fn set_property_backend -->
The name of the context that the settings are stored in.
<!-- trait SettingsExt::fn get_property_delay_apply -->
Whether the `Settings` object is in 'delay-apply' mode. See
`SettingsExt::delay` for details.
<!-- trait SettingsExt::fn get_property_has_unapplied -->
If this property is `true`, the `Settings` object has outstanding
changes that will be applied when `SettingsExt::apply` is called.
<!-- trait SettingsExt::fn get_property_path -->
The path within the backend where the settings are stored.
<!-- trait SettingsExt::fn set_property_path -->
The path within the backend where the settings are stored.
<!-- trait SettingsExt::fn get_property_schema_id -->
The name of the schema that describes the types of keys
for this `Settings` object.
<!-- trait SettingsExt::fn set_property_schema_id -->
The name of the schema that describes the types of keys
for this `Settings` object.
<!-- trait SettingsExt::fn get_property_settings_schema -->
The `SettingsSchema` describing the types of keys for this
`Settings` object.

Ideally, this property would be called 'schema'. `SettingsSchema`
has only existed since version 2.32, however, and before then the
'schema' property was used to refer to the ID of the schema rather
than the schema itself. Take care.
<!-- trait SettingsExt::fn set_property_settings_schema -->
The `SettingsSchema` describing the types of keys for this
`Settings` object.

Ideally, this property would be called 'schema'. `SettingsSchema`
has only existed since version 2.32, however, and before then the
'schema' property was used to refer to the ID of the schema rather
than the schema itself. Take care.
<!-- struct SettingsBackend -->
The `SettingsBackend` interface defines a generic interface for
non-strictly-typed data that is stored in a hierarchy. To implement
an alternative storage backend for `Settings`, you need to implement
the `SettingsBackend` interface and then make it implement the
extension point `G_SETTINGS_BACKEND_EXTENSION_POINT_NAME`.

The interface defines methods for reading and writing values, a
method for determining if writing of certain values will fail
(lockdown) and a change notification mechanism.

The semantics of the interface are very precisely defined and
implementations must carefully adhere to the expectations of
callers that are documented on each of the interface methods.

Some of the `SettingsBackend` functions accept or return a `glib::Tree`.
These trees always have strings as keys and `glib::Variant` as values.
`g_settings_backend_create_tree` is a convenience function to create
suitable trees.

The `SettingsBackend` API is exported to allow third-party
implementations, but does not carry the same stability guarantees
as the public GIO API. For this reason, you have to define the
C preprocessor symbol `G_SETTINGS_ENABLE_BACKEND` before including
`gio/gsettingsbackend.h`.

# Implements

[`SettingsBackendExt`](trait.SettingsBackendExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait SettingsBackendExt -->
Trait containing all `SettingsBackend` methods.

# Implementors

[`SettingsBackend`](struct.SettingsBackend.html)
<!-- impl SettingsBackend::fn flatten_tree -->
Calculate the longest common prefix of all keys in a tree and write
out an array of the key names relative to that prefix and,
optionally, the value to store at each of those keys.

You must free the value returned in `path`, `keys` and `values` using
`g_free`. You should not attempt to free or unref the contents of
`keys` or `values`.
## `tree`
a `glib::Tree` containing the changes
## `path`
the location to save the path
## `keys`
the
 location to save the relative keys
## `values`

 the location to save the values, or `None`
<!-- impl SettingsBackend::fn get_default -->
Returns the default `SettingsBackend`. It is possible to override
the default by setting the `GSETTINGS_BACKEND` environment variable
to the name of a settings backend.

The user gets a reference to the backend.

# Returns

the default `SettingsBackend`
<!-- trait SettingsBackendExt::fn changed -->
Signals that a single key has possibly changed. Backend
implementations should call this if a key has possibly changed its
value.

`key` must be a valid key (ie starting with a slash, not containing
'//', and not ending with a slash).

The implementation must call this function during any call to
`g_settings_backend_write`, before the call returns (except in the
case that no keys are actually changed and it cares to detect this
fact). It may not rely on the existence of a mainloop for
dispatching the signal later.

The implementation may call this function at any other time it likes
in response to other events (such as changes occurring outside of the
program). These calls may originate from a mainloop or may originate
in response to any other action (including from calls to
`g_settings_backend_write`).

In the case that this call is in response to a call to
`g_settings_backend_write` then `origin_tag` must be set to the same
value that was passed to that call.
## `key`
the name of the key
## `origin_tag`
the origin tag
<!-- trait SettingsBackendExt::fn changed_tree -->
This call is a convenience wrapper. It gets the list of changes from
`tree`, computes the longest common prefix and calls
`SettingsBackendExt::changed`.
## `tree`
a `glib::Tree` containing the changes
## `origin_tag`
the origin tag
<!-- trait SettingsBackendExt::fn keys_changed -->
Signals that a list of keys have possibly changed. Backend
implementations should call this if keys have possibly changed their
values.

`path` must be a valid path (ie starting and ending with a slash and
not containing '//'). Each string in `items` must form a valid key
name when `path` is prefixed to it (ie: each item must not start or
end with '/' and must not contain '//').

The meaning of this signal is that any of the key names resulting
from the contatenation of `path` with each item in `items` may have
changed.

The same rules for when notifications must occur apply as per
`SettingsBackendExt::changed`. These two calls can be used
interchangeably if exactly one item has changed (although in that
case `SettingsBackendExt::changed` is definitely preferred).

For efficiency reasons, the implementation should strive for `path` to
be as long as possible (ie: the longest common prefix of all of the
keys that were changed) but this is not strictly required.
## `path`
the path containing the changes
## `items`
the `None`-terminated list of changed keys
## `origin_tag`
the origin tag
<!-- trait SettingsBackendExt::fn path_changed -->
Signals that all keys below a given path may have possibly changed.
Backend implementations should call this if an entire path of keys
have possibly changed their values.

`path` must be a valid path (ie starting and ending with a slash and
not containing '//').

The meaning of this signal is that any of the key which has a name
starting with `path` may have changed.

The same rules for when notifications must occur apply as per
`SettingsBackendExt::changed`. This call might be an appropriate
reasponse to a 'reset' call but implementations are also free to
explicitly list the keys that were affected by that call if they can
easily do so.

For efficiency reasons, the implementation should strive for `path` to
be as long as possible (ie: the longest common prefix of all of the
keys that were changed) but this is not strictly required. As an
example, if this function is called with the path of "/" then every
single key in the application will be notified of a possible change.
## `path`
the path containing the changes
## `origin_tag`
the origin tag
<!-- trait SettingsBackendExt::fn path_writable_changed -->
Signals that the writability of all keys below a given path may have
changed.

Since GSettings performs no locking operations for itself, this call
will always be made in response to external events.
## `path`
the name of the path
<!-- trait SettingsBackendExt::fn writable_changed -->
Signals that the writability of a single key has possibly changed.

Since GSettings performs no locking operations for itself, this call
will always be made in response to external events.
## `key`
the name of the key
<!-- struct SettingsSchema -->
The `SettingsSchemaSource` and `SettingsSchema` APIs provide a
mechanism for advanced control over the loading of schemas and a
mechanism for introspecting their content.

Plugin loading systems that wish to provide plugins a way to access
settings face the problem of how to make the schemas for these
settings visible to GSettings. Typically, a plugin will want to ship
the schema along with itself and it won't be installed into the
standard system directories for schemas.

`SettingsSchemaSource` provides a mechanism for dealing with this by
allowing the creation of a new 'schema source' from which schemas can
be acquired. This schema source can then become part of the metadata
associated with the plugin and queried whenever the plugin requires
access to some settings.

Consider the following example:


```C
typedef struct
{
   ...
   GSettingsSchemaSource *schema_source;
   ...
} Plugin;

Plugin *
initialise_plugin (const gchar *dir)
{
  Plugin *plugin;

  ...

  plugin->schema_source =
    g_settings_schema_source_new_from_directory (dir,
      g_settings_schema_source_get_default (), FALSE, NULL);

  ...

  return plugin;
}

...

GSettings *
plugin_get_settings (Plugin      *plugin,
                     const gchar *schema_id)
{
  GSettingsSchema *schema;

  if (schema_id == NULL)
    schema_id = plugin->identifier;

  schema = g_settings_schema_source_lookup (plugin->schema_source,
                                            schema_id, FALSE);

  if (schema == NULL)
    {
      ... disable the plugin or abort, etc ...
    }

  return g_settings_new_full (schema, NULL, NULL);
}
```

The code above shows how hooks should be added to the code that
initialises (or enables) the plugin to create the schema source and
how an API can be added to the plugin system to provide a convenient
way for the plugin to access its settings, using the schemas that it
ships.

From the standpoint of the plugin, it would need to ensure that it
ships a gschemas.compiled file as part of itself, and then simply do
the following:


```C
{
  GSettings *settings;
  gint some_value;

  settings = plugin_get_settings (self, NULL);
  some_value = g_settings_get_int (settings, "some-value");
  ...
}
```

It's also possible that the plugin system expects the schema source
files (ie: .gschema.xml files) instead of a gschemas.compiled file.
In that case, the plugin loading system must compile the schemas for
itself before attempting to create the settings source.
<!-- impl SettingsSchema::fn get_id -->
Get the ID of `self`.

# Returns

the ID
<!-- impl SettingsSchema::fn get_key -->
Gets the key named `name` from `self`.

It is a programmer error to request a key that does not exist. See
`SettingsSchema::list_keys`.
## `name`
the name of a key

# Returns

the `SettingsSchemaKey` for `name`
<!-- impl SettingsSchema::fn get_path -->
Gets the path associated with `self`, or `None`.

Schemas may be single-instance or relocatable. Single-instance
schemas correspond to exactly one set of keys in the backend
database: those located at the path returned by this function.

Relocatable schemas can be referenced by other schemas and can
threfore describe multiple sets of keys at different locations. For
relocatable schemas, this function will return `None`.

# Returns

the path of the schema, or `None`
<!-- impl SettingsSchema::fn has_key -->
Checks if `self` has a key named `name`.
## `name`
the name of a key

# Returns

`true` if such a key exists
<!-- impl SettingsSchema::fn list_children -->
Gets the list of children in `self`.

You should free the return value with `g_strfreev` when you are done
with it.

Feature: `v2_44`


# Returns

a list of the children on `settings`
<!-- impl SettingsSchema::fn list_keys -->
Introspects the list of keys on `self`.

You should probably not be calling this function from "normal" code
(since you should already know what keys are in your schema). This
function is intended for introspection reasons.

Feature: `v2_46`


# Returns

a list of the keys on
 `self`
<!-- impl SettingsSchema::fn ref -->
Increase the reference count of `self`, returning a new reference.

# Returns

a new reference to `self`
<!-- impl SettingsSchema::fn unref -->
Decrease the reference count of `self`, possibly freeing it.
<!-- struct SettingsSchemaKey -->
`SettingsSchemaKey` is an opaque data structure and can only be accessed
using the following functions.
<!-- impl SettingsSchemaKey::fn get_default_value -->
Gets the default value for `self`.

Note that this is the default value according to the schema. System
administrator defaults and lockdown are not visible via this API.

# Returns

the default value for the key
<!-- impl SettingsSchemaKey::fn get_description -->
Gets the description for `self`.

If no description has been provided in the schema for `self`, returns
`None`.

The description can be one sentence to several paragraphs in length.
Paragraphs are delimited with a double newline. Descriptions can be
translated and the value returned from this function is is the
current locale.

This function is slow. The summary and description information for
the schemas is not stored in the compiled schema database so this
function has to parse all of the source XML files in the schema
directory.

# Returns

the description for `self`, or `None`
<!-- impl SettingsSchemaKey::fn get_name -->
Gets the name of `self`.

Feature: `v2_44`


# Returns

the name of `self`.
<!-- impl SettingsSchemaKey::fn get_range -->
Queries the range of a key.

This function will return a `glib::Variant` that fully describes the range
of values that are valid for `self`.

The type of `glib::Variant` returned is `(sv)`. The string describes
the type of range restriction in effect. The type and meaning of
the value contained in the variant depends on the string.

If the string is `'type'` then the variant contains an empty array.
The element type of that empty array is the expected type of value
and all values of that type are valid.

If the string is `'enum'` then the variant contains an array
enumerating the possible values. Each item in the array is
a possible valid value and no other values are valid.

If the string is `'flags'` then the variant contains an array. Each
item in the array is a value that may appear zero or one times in an
array to be used as the value for this key. For example, if the
variant contained the array `['x', 'y']` then the valid values for
the key would be `[]`, `['x']`, `['y']`, `['x', 'y']` and
`['y', 'x']`.

Finally, if the string is `'range'` then the variant contains a pair
of like-typed values -- the minimum and maximum permissible values
for this key.

This information should not be used by normal programs. It is
considered to be a hint for introspection purposes. Normal programs
should already know what is permitted by their own schema. The
format may change in any way in the future -- but particularly, new
forms may be added to the possibilities described above.

You should free the returned value with `glib::Variant::unref` when it is
no longer needed.

# Returns

a `glib::Variant` describing the range
<!-- impl SettingsSchemaKey::fn get_summary -->
Gets the summary for `self`.

If no summary has been provided in the schema for `self`, returns
`None`.

The summary is a short description of the purpose of the key; usually
one short sentence. Summaries can be translated and the value
returned from this function is is the current locale.

This function is slow. The summary and description information for
the schemas is not stored in the compiled schema database so this
function has to parse all of the source XML files in the schema
directory.

# Returns

the summary for `self`, or `None`
<!-- impl SettingsSchemaKey::fn get_value_type -->
Gets the `glib::VariantType` of `self`.

# Returns

the type of `self`
<!-- impl SettingsSchemaKey::fn range_check -->
Checks if the given `value` is of the correct type and within the
permitted range for `self`.

It is a programmer error if `value` is not of the correct type -- you
must check for this first.
## `value`
the value to check

# Returns

`true` if `value` is valid for `self`
<!-- impl SettingsSchemaKey::fn ref -->
Increase the reference count of `self`, returning a new reference.

# Returns

a new reference to `self`
<!-- impl SettingsSchemaKey::fn unref -->
Decrease the reference count of `self`, possibly freeing it.
<!-- struct SettingsSchemaSource -->
This is an opaque structure type. You may not access it directly.
<!-- impl SettingsSchemaSource::fn new_from_directory -->
Attempts to create a new schema source corresponding to the contents
of the given directory.

This function is not required for normal uses of `Settings` but it
may be useful to authors of plugin management systems.

The directory should contain a file called `gschemas.compiled` as
produced by the [glib-compile-schemas][glib-compile-schemas] tool.

If `trusted` is `true` then `gschemas.compiled` is trusted not to be
corrupted. This assumption has a performance advantage, but can result
in crashes or inconsistent behaviour in the case of a corrupted file.
Generally, you should set `trusted` to `true` for files installed by the
system and to `false` for files in the home directory.

In either case, an empty file or some types of corruption in the file will
result in `glib::FileError::Inval` being returned.

If `parent` is non-`None` then there are two effects.

First, if `SettingsSchemaSource::lookup` is called with the
`recursive` flag set to `true` and the schema can not be found in the
source, the lookup will recurse to the parent.

Second, any references to other schemas specified within this
source (ie: `child` or `extends`) references may be resolved
from the `parent`.

For this second reason, except in very unusual situations, the
`parent` should probably be given as the default schema source, as
returned by `SettingsSchemaSource::get_default`.
## `directory`
the filename of a directory
## `parent`
a `SettingsSchemaSource`, or `None`
## `trusted`
`true`, if the directory is trusted
<!-- impl SettingsSchemaSource::fn list_schemas -->
Lists the schemas in a given source.

If `recursive` is `true` then include parent sources. If `false` then
only include the schemas from one source (ie: one directory). You
probably want `true`.

Non-relocatable schemas are those for which you can call
`Settings::new`. Relocatable schemas are those for which you must
use `Settings::new_with_path`.

Do not call this function from normal programs. This is designed for
use by database editors, commandline tools, etc.
## `recursive`
if we should recurse
## `non_relocatable`
the
 list of non-relocatable schemas
## `relocatable`
the list
 of relocatable schemas
<!-- impl SettingsSchemaSource::fn lookup -->
Looks up a schema with the identifier `schema_id` in `self`.

This function is not required for normal uses of `Settings` but it
may be useful to authors of plugin management systems or to those who
want to introspect the content of schemas.

If the schema isn't found directly in `self` and `recursive` is `true`
then the parent sources will also be checked.

If the schema isn't found, `None` is returned.
## `schema_id`
a schema ID
## `recursive`
`true` if the lookup should be recursive

# Returns

a new `SettingsSchema`
<!-- impl SettingsSchemaSource::fn ref -->
Increase the reference count of `self`, returning a new reference.

# Returns

a new reference to `self`
<!-- impl SettingsSchemaSource::fn unref -->
Decrease the reference count of `self`, possibly freeing it.
<!-- impl SettingsSchemaSource::fn get_default -->
Gets the default system schema source.

This function is not required for normal uses of `Settings` but it
may be useful to authors of plugin management systems or to those who
want to introspect the content of schemas.

If no schemas are installed, `None` will be returned.

The returned source may actually consist of multiple schema sources
from different directories, depending on which directories were given
in `XDG_DATA_DIRS` and `GSETTINGS_SCHEMA_DIR`. For this reason, all
lookups performed against the default source should probably be done
recursively.

# Returns

the default schema source
<!-- struct SimpleAction -->
A `SimpleAction` is the obvious simple implementation of the `Action`
interface. This is the easiest way to create an action for purposes of
adding it to a `SimpleActionGroup`.

See also ``GtkAction``.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ActionExt`](trait.ActionExt.html)
<!-- impl SimpleAction::fn new -->
Creates a new action.

The created action is stateless. See `SimpleAction::new_stateful` to create
an action that has state.
## `name`
the name of the action
## `parameter_type`
the type of parameter that will be passed to
 handlers for the `SimpleAction::activate` signal, or `None` for no parameter

# Returns

a new `SimpleAction`
<!-- impl SimpleAction::fn new_stateful -->
Creates a new stateful action.

All future state values must have the same `glib::VariantType` as the initial
`state`.

If the `state` `glib::Variant` is floating, it is consumed.
## `name`
the name of the action
## `parameter_type`
the type of the parameter that will be passed to
 handlers for the `SimpleAction::activate` signal, or `None` for no parameter
## `state`
the initial state of the action

# Returns

a new `SimpleAction`
<!-- impl SimpleAction::fn set_enabled -->
Sets the action as enabled or not.

An action must be enabled in order to be activated or in order to
have its state changed from outside callers.

This should only be called by the implementor of the action. Users
of the action should not attempt to modify its enabled flag.
## `enabled`
whether the action is enabled
<!-- impl SimpleAction::fn set_state -->
Sets the state of the action.

This directly updates the 'state' property to the given value.

This should only be called by the implementor of the action. Users
of the action should not attempt to directly modify the 'state'
property. Instead, they should call `Action::change_state` to
request the change.

If the `value` GVariant is floating, it is consumed.
## `value`
the new `glib::Variant` for the state
<!-- impl SimpleAction::fn set_state_hint -->
Sets the state hint for the action.

See `Action::get_state_hint` for more information about
action state hints.

Feature: `v2_44`

## `state_hint`
a `glib::Variant` representing the state hint
<!-- impl SimpleAction::fn connect_activate -->
Indicates that the action was just activated.

`parameter` will always be of the expected type, i.e. the parameter type
specified when the action was created. If an incorrect type is given when
activating the action, this signal is not emitted.

Since GLib 2.40, if no handler is connected to this signal then the
default behaviour for boolean-stated actions with a `None` parameter
type is to toggle them via the `SimpleAction::change-state` signal.
For stateful actions where the state type is equal to the parameter
type, the default is to forward them directly to
`SimpleAction::change-state`. This should allow almost all users
of `SimpleAction` to connect only one handler or the other.
## `parameter`
the parameter to the activation, or `None` if it has
 no parameter
<!-- impl SimpleAction::fn connect_change_state -->
Indicates that the action just received a request to change its
state.

`value` will always be of the correct state type, i.e. the type of the
initial state passed to `SimpleAction::new_stateful`. If an incorrect
type is given when requesting to change the state, this signal is not
emitted.

If no handler is connected to this signal then the default
behaviour is to call `SimpleAction::set_state` to set the state
to the requested value. If you connect a signal handler then no
default action is taken. If the state should change then you must
call `SimpleAction::set_state` from the handler.

An example of a 'change-state' handler:

```C
static void
change_volume_state (GSimpleAction *action,
                     GVariant      *value,
                     gpointer       user_data)
{
  gint requested;

  requested = g_variant_get_int32 (value);

  // Volume only goes from 0 to 10
  if (0 <= requested && requested <= 10)
    g_simple_action_set_state (action, value);
}
```

The handler need not set the state to the requested value.
It could set it to any value at all, or take some other action.
## `value`
the requested value for the state
<!-- impl SimpleAction::fn get_property_enabled -->
If `action` is currently enabled.

If the action is disabled then calls to `Action::activate` and
`Action::change_state` have no effect.
<!-- impl SimpleAction::fn set_property_enabled -->
If `action` is currently enabled.

If the action is disabled then calls to `Action::activate` and
`Action::change_state` have no effect.
<!-- impl SimpleAction::fn get_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `SimpleActionGroup`.
<!-- impl SimpleAction::fn set_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `SimpleActionGroup`.
<!-- impl SimpleAction::fn get_property_parameter_type -->
The type of the parameter that must be given when activating the
action.
<!-- impl SimpleAction::fn set_property_parameter_type -->
The type of the parameter that must be given when activating the
action.
<!-- impl SimpleAction::fn get_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- impl SimpleAction::fn set_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- impl SimpleAction::fn get_property_state_type -->
The `glib::VariantType` of the state that the action has, or `None` if the
action is stateless.
<!-- struct SimpleActionGroup -->
`SimpleActionGroup` is a hash table filled with `Action` objects,
implementing the `ActionGroup` and `ActionMap` interfaces.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ActionGroupExt`](trait.ActionGroupExt.html), [`ActionMapExt`](trait.ActionMapExt.html)
<!-- impl SimpleActionGroup::fn new -->
Creates a new, empty, `SimpleActionGroup`.

# Returns

a new `SimpleActionGroup`
<!-- struct SimpleIOStream -->
GSimpleIOStream creates a `IOStream` from an arbitrary `InputStream` and
`OutputStream`. This allows any pair of input and output streams to be used
with `IOStream` methods.

This is useful when you obtained a `InputStream` and a `OutputStream`
by other means, for instance creating them with platform specific methods as
`UnixInputStream::new` or `g_win32_input_stream_new`, and you want
to take advantage of the methods provided by `IOStream`.

Feature: `v2_44`

# Implements

[`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl SimpleIOStream::fn new -->
Creates a new `SimpleIOStream` wrapping `input_stream` and `output_stream`.
See also `IOStream`.

Feature: `v2_44`

## `input_stream`
a `InputStream`.
## `output_stream`
a `OutputStream`.

# Returns

a new `SimpleIOStream` instance.
<!-- struct SimplePermission -->
`SimplePermission` is a trivial implementation of `Permission` that
represents a permission that is either always or never allowed. The
value is given at construction and doesn't change.

Calling request or release will result in errors.

# Implements

[`PermissionExt`](trait.PermissionExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl SimplePermission::fn new -->
Creates a new `Permission` instance that represents an action that is
either always or never allowed.
## `allowed`
`true` if the action is allowed

# Returns

the `SimplePermission`, as a `Permission`
<!-- struct Socket -->
A `Socket` is a low-level networking primitive. It is a more or less
direct mapping of the BSD socket API in a portable GObject based API.
It supports both the UNIX socket implementations and winsock2 on Windows.

`Socket` is the platform independent base upon which the higher level
network primitives are based. Applications are not typically meant to
use it directly, but rather through classes like `SocketClient`,
`SocketService` and `SocketConnection`. However there may be cases where
direct use of `Socket` is useful.

`Socket` implements the `Initable` interface, so if it is manually constructed
by e.g. `gobject::Object::new` you must call `Initable::init` and check the
results before using the object. This is done automatically in
`Socket::new` and `Socket::new_from_fd`, so these functions can return
`None`.

Sockets operate in two general modes, blocking or non-blocking. When
in blocking mode all operations (which don’t take an explicit blocking
parameter) block until the requested operation
is finished or there is an error. In non-blocking mode all calls that
would block return immediately with a `IOErrorEnum::WouldBlock` error.
To know when a call would successfully run you can call `SocketExt::condition_check`,
or `SocketExt::condition_wait`. You can also use `Socket::create_source` and
attach it to a `glib::MainContext` to get callbacks when I/O is possible.
Note that all sockets are always set to non blocking mode in the system, and
blocking mode is emulated in GSocket.

When working in non-blocking mode applications should always be able to
handle getting a `IOErrorEnum::WouldBlock` error even when some other
function said that I/O was possible. This can easily happen in case
of a race condition in the application, but it can also happen for other
reasons. For instance, on Windows a socket is always seen as writable
until a write returns `IOErrorEnum::WouldBlock`.

`GSockets` can be either connection oriented or datagram based.
For connection oriented types you must first establish a connection by
either connecting to an address or accepting a connection from another
address. For connectionless socket types the target/source address is
specified or received in each I/O operation.

All socket file descriptors are set to be close-on-exec.

Note that creating a `Socket` causes the signal `SIGPIPE` to be
ignored for the remainder of the program. If you are writing a
command-line utility that uses `Socket`, you may need to take into
account the fact that your program will not automatically be killed
if it tries to write to `stdout` after it has been closed.

Like most other APIs in GLib, `Socket` is not inherently thread safe. To use
a `Socket` concurrently from multiple threads, you must implement your own
locking.

# Implements

[`SocketExt`](trait.SocketExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketExtManual`](prelude/trait.SocketExtManual.html)
<!-- trait SocketExt -->
Trait containing all `Socket` methods.

# Implementors

[`Socket`](struct.Socket.html)
<!-- impl Socket::fn new -->
Creates a new `Socket` with the defined family, type and protocol.
If `protocol` is 0 (`SocketProtocol::Default`) the default protocol type
for the family and type is used.

The `protocol` is a family and type specific int that specifies what
kind of protocol to use. `SocketProtocol` lists several common ones.
Many families only support one protocol, and use 0 for this, others
support several and using 0 means to use the default protocol for
the family and type.

The protocol id is passed directly to the operating
system, so you can use protocols not listed in `SocketProtocol` if you
know the protocol number used for it.
## `family`
the socket family to use, e.g. `SocketFamily::Ipv4`.
## `type_`
the socket type to use.
## `protocol`
the id of the protocol to use, or 0 for default.

# Returns

a `Socket` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- impl Socket::fn new_from_fd -->
Creates a new `Socket` from a native file descriptor
or winsock SOCKET handle.

This reads all the settings from the file descriptor so that
all properties should work. Note that the file descriptor
will be set to non-blocking mode, independent on the blocking
mode of the `Socket`.

On success, the returned `Socket` takes ownership of `fd`. On failure, the
caller must close `fd` themselves.

Since GLib 2.46, it is no longer a fatal error to call this on a non-socket
descriptor. Instead, a GError will be set with code `IOErrorEnum::Failed`
## `fd`
a native socket file descriptor.

# Returns

a `Socket` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketExt::fn accept -->
Accept incoming connections on a connection-based socket. This removes
the first outstanding connection request from the listening socket and
creates a `Socket` object for it.

The `self` must be bound to a local address with `SocketExt::bind` and
must be listening for incoming connections (`SocketExt::listen`).

If there are no outstanding connections then the operation will block
or return `IOErrorEnum::WouldBlock` if non-blocking I/O is enabled.
To be notified of an incoming connection, wait for the `glib::IOCondition::In` condition.
## `cancellable`
a `Cancellable` or `None`

# Returns

a new `Socket`, or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketExt::fn bind -->
When a socket is created it is attached to an address family, but it
doesn't have an address in this family. `SocketExt::bind` assigns the
address (sometimes called name) of the socket.

It is generally required to bind to a local address before you can
receive connections. (See `SocketExt::listen` and `SocketExt::accept` ).
In certain situations, you may also want to bind a socket that will be
used to initiate connections, though this is not normally required.

If `self` is a TCP socket, then `allow_reuse` controls the setting
of the `SO_REUSEADDR` socket option; normally it should be `true` for
server sockets (sockets that you will eventually call
`SocketExt::accept` on), and `false` for client sockets. (Failing to
set this flag on a server socket may cause `SocketExt::bind` to return
`IOErrorEnum::AddressInUse` if the server program is stopped and then
immediately restarted.)

If `self` is a UDP socket, then `allow_reuse` determines whether or
not other UDP sockets can be bound to the same address at the same
time. In particular, you can have several UDP sockets bound to the
same address, and they will all receive all of the multicast and
broadcast packets sent to that address. (The behavior of unicast
UDP packets to an address with multiple listeners is not defined.)
## `address`
a `SocketAddress` specifying the local address.
## `allow_reuse`
whether to allow reusing this address

# Returns

`true` on success, `false` on error.
<!-- trait SocketExt::fn check_connect_result -->
Checks and resets the pending connect error for the socket.
This is used to check for errors when `SocketExt::connect` is
used in non-blocking mode.

# Returns

`true` if no error, `false` otherwise, setting `error` to the error
<!-- trait SocketExt::fn close -->
Closes the socket, shutting down any active connection.

Closing a socket does not wait for all outstanding I/O operations
to finish, so the caller should not rely on them to be guaranteed
to complete even if the close returns with no error.

Once the socket is closed, all other operations will return
`IOErrorEnum::Closed`. Closing a socket multiple times will not
return an error.

Sockets will be automatically closed when the last reference
is dropped, but you might want to call this function to make sure
resources are released as early as possible.

Beware that due to the way that TCP works, it is possible for
recently-sent data to be lost if either you close a socket while the
`glib::IOCondition::In` condition is set, or else if the remote connection tries to
send something to you after you close the socket but before it has
finished reading all of the data you sent. There is no easy generic
way to avoid this problem; the easiest fix is to design the network
protocol such that the client will never send data "out of turn".
Another solution is for the server to half-close the connection by
calling `SocketExt::shutdown` with only the `shutdown_write` flag set,
and then wait for the client to notice this and close its side of the
connection, after which the server can safely call `SocketExt::close`.
(This is what `TcpConnection` does if you call
`TcpConnectionExt::set_graceful_disconnect`. But of course, this
only works if the client will close its connection after the server
does.)

# Returns

`true` on success, `false` on error
<!-- trait SocketExt::fn condition_check -->
Checks on the readiness of `self` to perform operations.
The operations specified in `condition` are checked for and masked
against the currently-satisfied conditions on `self`. The result
is returned.

Note that on Windows, it is possible for an operation to return
`IOErrorEnum::WouldBlock` even immediately after
`SocketExt::condition_check` has claimed that the socket is ready for
writing. Rather than calling `SocketExt::condition_check` and then
writing to the socket if it succeeds, it is generally better to
simply try writing to the socket right away, and try again later if
the initial attempt returns `IOErrorEnum::WouldBlock`.

It is meaningless to specify `glib::IOCondition::Err` or `glib::IOCondition::Hup` in condition;
these conditions will always be set in the output if they are true.

This call never blocks.
## `condition`
a `glib::IOCondition` mask to check

# Returns

the `glib::IOCondition` mask of the current state
<!-- trait SocketExt::fn condition_timed_wait -->
Waits for up to `timeout` microseconds for `condition` to become true
on `self`. If the condition is met, `true` is returned.

If `cancellable` is cancelled before the condition is met, or if
`timeout` (or the socket's `Socket:timeout`) is reached before the
condition is met, then `false` is returned and `error`, if non-`None`,
is set to the appropriate value (`IOErrorEnum::Cancelled` or
`IOErrorEnum::TimedOut`).

If you don't want a timeout, use `SocketExt::condition_wait`.
(Alternatively, you can pass -1 for `timeout`.)

Note that although `timeout` is in microseconds for consistency with
other GLib APIs, this function actually only has millisecond
resolution, and the behavior is undefined if `timeout` is not an
exact number of milliseconds.
## `condition`
a `glib::IOCondition` mask to wait for
## `timeout`
the maximum time (in microseconds) to wait, or -1
## `cancellable`
a `Cancellable`, or `None`

# Returns

`true` if the condition was met, `false` otherwise
<!-- trait SocketExt::fn condition_wait -->
Waits for `condition` to become true on `self`. When the condition
is met, `true` is returned.

If `cancellable` is cancelled before the condition is met, or if the
socket has a timeout set and it is reached before the condition is
met, then `false` is returned and `error`, if non-`None`, is set to
the appropriate value (`IOErrorEnum::Cancelled` or
`IOErrorEnum::TimedOut`).

See also `SocketExt::condition_timed_wait`.
## `condition`
a `glib::IOCondition` mask to wait for
## `cancellable`
a `Cancellable`, or `None`

# Returns

`true` if the condition was met, `false` otherwise
<!-- trait SocketExt::fn connect -->
Connect the socket to the specified remote address.

For connection oriented socket this generally means we attempt to make
a connection to the `address`. For a connection-less socket it sets
the default address for `Socket::send` and discards all incoming datagrams
from other sources.

Generally connection oriented sockets can only connect once, but
connection-less sockets can connect multiple times to change the
default address.

If the connect call needs to do network I/O it will block, unless
non-blocking I/O is enabled. Then `IOErrorEnum::Pending` is returned
and the user can be notified of the connection finishing by waiting
for the G_IO_OUT condition. The result of the connection must then be
checked with `SocketExt::check_connect_result`.
## `address`
a `SocketAddress` specifying the remote address.
## `cancellable`
a `Cancellable` or `None`

# Returns

`true` if connected, `false` on error.
<!-- trait SocketExt::fn connection_factory_create_connection -->
Creates a `SocketConnection` subclass of the right type for
`self`.

# Returns

a `SocketConnection`
<!-- trait SocketExtManual::fn create_source -->
Creates a `glib::Source` that can be attached to a `glib::MainContext` to monitor
for the availability of the specified `condition` on the socket. The `glib::Source`
keeps a reference to the `self`.

The callback on the source is of the `GSocketSourceFunc` type.

It is meaningless to specify `glib::IOCondition::Err` or `glib::IOCondition::Hup` in `condition`;
these conditions will always be reported output if they are true.

`cancellable` if not `None` can be used to cancel the source, which will
cause the source to trigger, reporting the current condition (which
is likely 0 unless cancellation happened at the same time as a
condition change). You can check for this in the callback using
`CancellableExt::is_cancelled`.

If `self` has a timeout set, and it is reached before `condition`
occurs, the source will then trigger anyway, reporting `glib::IOCondition::In` or
`glib::IOCondition::Out` depending on `condition`. However, `self` will have been
marked as having had a timeout, and so the next `Socket` I/O method
you call will then fail with a `IOErrorEnum::TimedOut`.
## `condition`
a `glib::IOCondition` mask to monitor
## `cancellable`
a `Cancellable` or `None`

# Returns

a newly allocated `glib::Source`, free with `glib::Source::unref`.
<!-- trait SocketExt::fn get_available_bytes -->
Get the amount of data pending in the OS input buffer, without blocking.

If `self` is a UDP or SCTP socket, this will return the size of
just the next packet, even if additional packets are buffered after
that one.

Note that on Windows, this function is rather inefficient in the
UDP case, and so if you know any plausible upper bound on the size
of the incoming packet, it is better to just do a
`Socket::receive` with a buffer of that size, rather than calling
`SocketExt::get_available_bytes` first and then doing a receive of
exactly the right size.

# Returns

the number of bytes that can be read from the socket
without blocking or truncating, or -1 on error.
<!-- trait SocketExt::fn get_blocking -->
Gets the blocking mode of the socket. For details on blocking I/O,
see `SocketExt::set_blocking`.

# Returns

`true` if blocking I/O is used, `false` otherwise.
<!-- trait SocketExt::fn get_broadcast -->
Gets the broadcast setting on `self`; if `true`,
it is possible to send packets to broadcast
addresses.

# Returns

the broadcast setting on `self`
<!-- trait SocketExt::fn get_credentials -->
Returns the credentials of the foreign process connected to this
socket, if any (e.g. it is only supported for `SocketFamily::Unix`
sockets).

If this operation isn't supported on the OS, the method fails with
the `IOErrorEnum::NotSupported` error. On Linux this is implemented
by reading the `SO_PEERCRED` option on the underlying socket.

Other ways to obtain credentials from a foreign peer includes the
`UnixCredentialsMessage` type and
`UnixConnection::send_credentials` /
`UnixConnection::receive_credentials` functions.

# Returns

`None` if `error` is set, otherwise a `Credentials` object
that must be freed with `gobject::ObjectExt::unref`.
<!-- trait SocketExt::fn get_family -->
Gets the socket family of the socket.

# Returns

a `SocketFamily`
<!-- trait SocketExtManual::fn get_fd -->
Returns the underlying OS socket object. On unix this
is a socket file descriptor, and on Windows this is
a Winsock2 SOCKET handle. This may be useful for
doing platform specific or otherwise unusual operations
on the socket.

# Returns

the file descriptor of the socket.
<!-- trait SocketExt::fn get_keepalive -->
Gets the keepalive mode of the socket. For details on this,
see `SocketExt::set_keepalive`.

# Returns

`true` if keepalive is active, `false` otherwise.
<!-- trait SocketExt::fn get_listen_backlog -->
Gets the listen backlog setting of the socket. For details on this,
see `SocketExt::set_listen_backlog`.

# Returns

the maximum number of pending connections.
<!-- trait SocketExt::fn get_local_address -->
Try to get the local address of a bound socket. This is only
useful if the socket has been bound to a local address,
either explicitly or implicitly when connecting.

# Returns

a `SocketAddress` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketExt::fn get_multicast_loopback -->
Gets the multicast loopback setting on `self`; if `true` (the
default), outgoing multicast packets will be looped back to
multicast listeners on the same host.

# Returns

the multicast loopback setting on `self`
<!-- trait SocketExt::fn get_multicast_ttl -->
Gets the multicast time-to-live setting on `self`; see
`SocketExt::set_multicast_ttl` for more details.

# Returns

the multicast time-to-live setting on `self`
<!-- trait SocketExt::fn get_option -->
Gets the value of an integer-valued option on `self`, as with
`getsockopt`. (If you need to fetch a non-integer-valued option,
you will need to call `getsockopt` directly.)

The [<gio/gnetworking.h>][gio-gnetworking.h]
header pulls in system headers that will define most of the
standard/portable socket options. For unusual socket protocols or
platform-dependent options, you may need to include additional
headers.

Note that even for socket options that are a single byte in size,
`value` is still a pointer to a `gint` variable, not a `guchar`;
`SocketExt::get_option` will handle the conversion internally.
## `level`
the "API level" of the option (eg, `SOL_SOCKET`)
## `optname`
the "name" of the option (eg, `SO_BROADCAST`)
## `value`
return location for the option value

# Returns

success or failure. On failure, `error` will be set, and
 the system error value (`errno` or WSAGetLastError()) will still
 be set to the result of the `getsockopt` call.
<!-- trait SocketExt::fn get_protocol -->
Gets the socket protocol id the socket was created with.
In case the protocol is unknown, -1 is returned.

# Returns

a protocol id, or -1 if unknown
<!-- trait SocketExt::fn get_remote_address -->
Try to get the remote address of a connected socket. This is only
useful for connection oriented sockets that have been connected.

# Returns

a `SocketAddress` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketExt::fn get_socket_type -->
Gets the socket type of the socket.

# Returns

a `SocketType`
<!-- trait SocketExt::fn get_timeout -->
Gets the timeout setting of the socket. For details on this, see
`SocketExt::set_timeout`.

# Returns

the timeout in seconds
<!-- trait SocketExt::fn get_ttl -->
Gets the unicast time-to-live setting on `self`; see
`SocketExt::set_ttl` for more details.

# Returns

the time-to-live setting on `self`
<!-- trait SocketExt::fn is_closed -->
Checks whether a socket is closed.

# Returns

`true` if socket is closed, `false` otherwise
<!-- trait SocketExt::fn is_connected -->
Check whether the socket is connected. This is only useful for
connection-oriented sockets.

If using `SocketExt::shutdown`, this function will return `true` until the
socket has been shut down for reading and writing. If you do a non-blocking
connect, this function will not return `true` until after you call
`SocketExt::check_connect_result`.

# Returns

`true` if socket is connected, `false` otherwise.
<!-- trait SocketExt::fn join_multicast_group -->
Registers `self` to receive multicast messages sent to `group`.
`self` must be a `SocketType::Datagram` socket, and must have
been bound to an appropriate interface and port with
`SocketExt::bind`.

If `iface` is `None`, the system will automatically pick an interface
to bind to based on `group`.

If `source_specific` is `true`, source-specific multicast as defined
in RFC 4604 is used. Note that on older platforms this may fail
with a `IOErrorEnum::NotSupported` error.

To bind to a given source-specific multicast address, use
`SocketExt::join_multicast_group_ssm` instead.
## `group`
a `InetAddress` specifying the group address to join.
## `source_specific`
`true` if source-specific multicast should be used
## `iface`
Name of the interface to use, or `None`

# Returns

`true` on success, `false` on error.
<!-- trait SocketExt::fn join_multicast_group_ssm -->
Registers `self` to receive multicast messages sent to `group`.
`self` must be a `SocketType::Datagram` socket, and must have
been bound to an appropriate interface and port with
`SocketExt::bind`.

If `iface` is `None`, the system will automatically pick an interface
to bind to based on `group`.

If `source_specific` is not `None`, use source-specific multicast as
defined in RFC 4604. Note that on older platforms this may fail
with a `IOErrorEnum::NotSupported` error.

Note that this function can be called multiple times for the same
`group` with different `source_specific` in order to receive multicast
packets from more than one source.

Feature: `v2_56`

## `group`
a `InetAddress` specifying the group address to join.
## `source_specific`
a `InetAddress` specifying the
source-specific multicast address or `None` to ignore.
## `iface`
Name of the interface to use, or `None`

# Returns

`true` on success, `false` on error.
<!-- trait SocketExt::fn leave_multicast_group -->
Removes `self` from the multicast group defined by `group`, `iface`,
and `source_specific` (which must all have the same values they had
when you joined the group).

`self` remains bound to its address and port, and can still receive
unicast messages after calling this.

To unbind to a given source-specific multicast address, use
`SocketExt::leave_multicast_group_ssm` instead.
## `group`
a `InetAddress` specifying the group address to leave.
## `source_specific`
`true` if source-specific multicast was used
## `iface`
Interface used

# Returns

`true` on success, `false` on error.
<!-- trait SocketExt::fn leave_multicast_group_ssm -->
Removes `self` from the multicast group defined by `group`, `iface`,
and `source_specific` (which must all have the same values they had
when you joined the group).

`self` remains bound to its address and port, and can still receive
unicast messages after calling this.

Feature: `v2_56`

## `group`
a `InetAddress` specifying the group address to leave.
## `source_specific`
a `InetAddress` specifying the
source-specific multicast address or `None` to ignore.
## `iface`
Name of the interface to use, or `None`

# Returns

`true` on success, `false` on error.
<!-- trait SocketExt::fn listen -->
Marks the socket as a server socket, i.e. a socket that is used
to accept incoming requests using `SocketExt::accept`.

Before calling this the socket must be bound to a local address using
`SocketExt::bind`.

To set the maximum amount of outstanding clients, use
`SocketExt::set_listen_backlog`.

# Returns

`true` on success, `false` on error.
<!-- trait SocketExtManual::fn receive -->
Receive data (up to `size` bytes) from a socket. This is mainly used by
connection-oriented sockets; it is identical to `Socket::receive_from`
with `address` set to `None`.

For `SocketType::Datagram` and `SocketType::Seqpacket` sockets,
`Socket::receive` will always read either 0 or 1 complete messages from
the socket. If the received message is too large to fit in `buffer`, then
the data beyond `size` bytes will be discarded, without any explicit
indication that this has occurred.

For `SocketType::Stream` sockets, `Socket::receive` can return any
number of bytes, up to `size`. If more than `size` bytes have been
received, the additional data will be returned in future calls to
`Socket::receive`.

If the socket is in blocking mode the call will block until there
is some data to receive, the connection is closed, or there is an
error. If there is no data available and the socket is in
non-blocking mode, a `IOErrorEnum::WouldBlock` error will be
returned. To be notified when data is available, wait for the
`glib::IOCondition::In` condition.

On error -1 is returned and `error` is set accordingly.
## `buffer`
a buffer to
 read data into (which should be at least `size` bytes long).
## `size`
the number of bytes you want to read from the socket
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes read, or 0 if the connection was closed by
the peer, or -1 on error
<!-- trait SocketExtManual::fn receive_from -->
Receive data (up to `size` bytes) from a socket.

If `address` is non-`None` then `address` will be set equal to the
source address of the received packet.
`address` is owned by the caller.

See `Socket::receive` for additional information.
## `address`
a pointer to a `SocketAddress`
 pointer, or `None`
## `buffer`
a buffer to
 read data into (which should be at least `size` bytes long).
## `size`
the number of bytes you want to read from the socket
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes read, or 0 if the connection was closed by
the peer, or -1 on error
<!-- trait SocketExtManual::fn receive_message -->
Receive data from a socket. For receiving multiple messages, see
`Socket::receive_messages`; for easier use, see
`Socket::receive` and `Socket::receive_from`.

If `address` is non-`None` then `address` will be set equal to the
source address of the received packet.
`address` is owned by the caller.

`vector` must point to an array of `InputVector` structs and
`num_vectors` must be the length of this array. These structs
describe the buffers that received data will be scattered into.
If `num_vectors` is -1, then `vectors` is assumed to be terminated
by a `InputVector` with a `None` buffer pointer.

As a special case, if `num_vectors` is 0 (in which case, `vectors`
may of course be `None`), then a single byte is received and
discarded. This is to facilitate the common practice of sending a
single '\0' byte for the purposes of transferring ancillary data.

`messages`, if non-`None`, will be set to point to a newly-allocated
array of `SocketControlMessage` instances or `None` if no such
messages was received. These correspond to the control messages
received from the kernel, one `SocketControlMessage` per message
from the kernel. This array is `None`-terminated and must be freed
by the caller using `g_free` after calling `gobject::ObjectExt::unref` on each
element. If `messages` is `None`, any control messages received will
be discarded.

`num_messages`, if non-`None`, will be set to the number of control
messages received.

If both `messages` and `num_messages` are non-`None`, then
`num_messages` gives the number of `SocketControlMessage` instances
in `messages` (ie: not including the `None` terminator).

`flags` is an in/out parameter. The commonly available arguments
for this are available in the `SocketMsgFlags` enum, but the
values there are the same as the system values, and the flags
are passed in as-is, so you can pass in system-specific flags too
(and `Socket::receive_message` may pass system-specific flags out).
Flags passed in to the parameter affect the receive operation; flags returned
out of it are relevant to the specific returned message.

As with `Socket::receive`, data may be discarded if `self` is
`SocketType::Datagram` or `SocketType::Seqpacket` and you do not
provide enough buffer space to read a complete message. You can pass
`SocketMsgFlags::Peek` in `flags` to peek at the current message without
removing it from the receive queue, but there is no portable way to find
out the length of the message other than by reading it into a
sufficiently-large buffer.

If the socket is in blocking mode the call will block until there
is some data to receive, the connection is closed, or there is an
error. If there is no data available and the socket is in
non-blocking mode, a `IOErrorEnum::WouldBlock` error will be
returned. To be notified when data is available, wait for the
`glib::IOCondition::In` condition.

On error -1 is returned and `error` is set accordingly.
## `address`
a pointer to a `SocketAddress`
 pointer, or `None`
## `vectors`
an array of `InputVector` structs
## `num_vectors`
the number of elements in `vectors`, or -1
## `messages`
a pointer
 which may be filled with an array of `GSocketControlMessages`, or `None`
## `num_messages`
a pointer which will be filled with the number of
 elements in `messages`, or `None`
## `flags`
a pointer to an int containing `SocketMsgFlags` flags
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes read, or 0 if the connection was closed by
the peer, or -1 on error
<!-- trait SocketExtManual::fn receive_messages -->
Receive multiple data messages from `self` in one go. This is the most
complicated and fully-featured version of this call. For easier use, see
`Socket::receive`, `Socket::receive_from`, and `Socket::receive_message`.

`messages` must point to an array of `InputMessage` structs and
`num_messages` must be the length of this array. Each `InputMessage`
contains a pointer to an array of `InputVector` structs describing the
buffers that the data received in each message will be written to. Using
multiple `GInputVectors` is more memory-efficient than manually copying data
out of a single buffer to multiple sources, and more system-call-efficient
than making multiple calls to `Socket::receive`, such as in scenarios where
a lot of data packets need to be received (e.g. high-bandwidth video
streaming over RTP/UDP).

`flags` modify how all messages are received. The commonly available
arguments for this are available in the `SocketMsgFlags` enum, but the
values there are the same as the system values, and the flags
are passed in as-is, so you can pass in system-specific flags too. These
flags affect the overall receive operation. Flags affecting individual
messages are returned in `InputMessage.flags`.

The other members of `InputMessage` are treated as described in its
documentation.

If `Socket:blocking` is `true` the call will block until `num_messages` have
been received, or the end of the stream is reached.

If `Socket:blocking` is `false` the call will return up to `num_messages`
without blocking, or `IOErrorEnum::WouldBlock` if no messages are queued in the
operating system to be received.

In blocking mode, if `Socket:timeout` is positive and is reached before any
messages are received, `IOErrorEnum::TimedOut` is returned, otherwise up to
`num_messages` are returned. (Note: This is effectively the
behaviour of `MSG_WAITFORONE` with `recvmmsg`.)

To be notified when messages are available, wait for the
`glib::IOCondition::In` condition. Note though that you may still receive
`IOErrorEnum::WouldBlock` from `Socket::receive_messages` even if you were
previously notified of a `glib::IOCondition::In` condition.

If the remote peer closes the connection, any messages queued in the
operating system will be returned, and subsequent calls to
`Socket::receive_messages` will return 0 (with no error set).

On error -1 is returned and `error` is set accordingly. An error will only
be returned if zero messages could be received; otherwise the number of
messages successfully received before the error will be returned.

Feature: `v2_48`

## `messages`
an array of `InputMessage` structs
## `num_messages`
the number of elements in `messages`
## `flags`
an int containing `SocketMsgFlags` flags for the overall operation
## `cancellable`
a `Cancellable` or `None`

# Returns

number of messages received, or -1 on error. Note that the number
 of messages received may be smaller than `num_messages` if in non-blocking
 mode, if the peer closed the connection, or if `num_messages`
 was larger than `UIO_MAXIOV` (1024), in which case the caller may re-try
 to receive the remaining messages.
<!-- trait SocketExtManual::fn receive_with_blocking -->
This behaves exactly the same as `Socket::receive`, except that
the choice of blocking or non-blocking behavior is determined by
the `blocking` argument rather than by `self`'s properties.
## `buffer`
a buffer to
 read data into (which should be at least `size` bytes long).
## `size`
the number of bytes you want to read from the socket
## `blocking`
whether to do blocking or non-blocking I/O
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes read, or 0 if the connection was closed by
the peer, or -1 on error
<!-- trait SocketExtManual::fn send -->
Tries to send `size` bytes from `buffer` on the socket. This is
mainly used by connection-oriented sockets; it is identical to
`Socket::send_to` with `address` set to `None`.

If the socket is in blocking mode the call will block until there is
space for the data in the socket queue. If there is no space available
and the socket is in non-blocking mode a `IOErrorEnum::WouldBlock` error
will be returned. To be notified when space is available, wait for the
`glib::IOCondition::Out` condition. Note though that you may still receive
`IOErrorEnum::WouldBlock` from `Socket::send` even if you were previously
notified of a `glib::IOCondition::Out` condition. (On Windows in particular, this is
very common due to the way the underlying APIs work.)

On error -1 is returned and `error` is set accordingly.
## `buffer`
the buffer
 containing the data to send.
## `size`
the number of bytes to send
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes written (which may be less than `size`), or -1
on error
<!-- trait SocketExtManual::fn send_message -->
Send data to `address` on `self`. For sending multiple messages see
`Socket::send_messages`; for easier use, see
`Socket::send` and `Socket::send_to`.

If `address` is `None` then the message is sent to the default receiver
(set by `SocketExt::connect`).

`vectors` must point to an array of `OutputVector` structs and
`num_vectors` must be the length of this array. (If `num_vectors` is -1,
then `vectors` is assumed to be terminated by a `OutputVector` with a
`None` buffer pointer.) The `OutputVector` structs describe the buffers
that the sent data will be gathered from. Using multiple
`GOutputVectors` is more memory-efficient than manually copying
data from multiple sources into a single buffer, and more
network-efficient than making multiple calls to `Socket::send`.

`messages`, if non-`None`, is taken to point to an array of `num_messages`
`SocketControlMessage` instances. These correspond to the control
messages to be sent on the socket.
If `num_messages` is -1 then `messages` is treated as a `None`-terminated
array.

`flags` modify how the message is sent. The commonly available arguments
for this are available in the `SocketMsgFlags` enum, but the
values there are the same as the system values, and the flags
are passed in as-is, so you can pass in system-specific flags too.

If the socket is in blocking mode the call will block until there is
space for the data in the socket queue. If there is no space available
and the socket is in non-blocking mode a `IOErrorEnum::WouldBlock` error
will be returned. To be notified when space is available, wait for the
`glib::IOCondition::Out` condition. Note though that you may still receive
`IOErrorEnum::WouldBlock` from `Socket::send` even if you were previously
notified of a `glib::IOCondition::Out` condition. (On Windows in particular, this is
very common due to the way the underlying APIs work.)

On error -1 is returned and `error` is set accordingly.
## `address`
a `SocketAddress`, or `None`
## `vectors`
an array of `OutputVector` structs
## `num_vectors`
the number of elements in `vectors`, or -1
## `messages`
a pointer to an
 array of `GSocketControlMessages`, or `None`.
## `num_messages`
number of elements in `messages`, or -1.
## `flags`
an int containing `SocketMsgFlags` flags
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes written (which may be less than `size`), or -1
on error
<!-- trait SocketExtManual::fn send_messages -->
Send multiple data messages from `self` in one go. This is the most
complicated and fully-featured version of this call. For easier use, see
`Socket::send`, `Socket::send_to`, and `Socket::send_message`.

`messages` must point to an array of `OutputMessage` structs and
`num_messages` must be the length of this array. Each `OutputMessage`
contains an address to send the data to, and a pointer to an array of
`OutputVector` structs to describe the buffers that the data to be sent
for each message will be gathered from. Using multiple `GOutputVectors` is
more memory-efficient than manually copying data from multiple sources
into a single buffer, and more network-efficient than making multiple
calls to `Socket::send`. Sending multiple messages in one go avoids the
overhead of making a lot of syscalls in scenarios where a lot of data
packets need to be sent (e.g. high-bandwidth video streaming over RTP/UDP),
or where the same data needs to be sent to multiple recipients.

`flags` modify how the message is sent. The commonly available arguments
for this are available in the `SocketMsgFlags` enum, but the
values there are the same as the system values, and the flags
are passed in as-is, so you can pass in system-specific flags too.

If the socket is in blocking mode the call will block until there is
space for all the data in the socket queue. If there is no space available
and the socket is in non-blocking mode a `IOErrorEnum::WouldBlock` error
will be returned if no data was written at all, otherwise the number of
messages sent will be returned. To be notified when space is available,
wait for the `glib::IOCondition::Out` condition. Note though that you may still receive
`IOErrorEnum::WouldBlock` from `Socket::send` even if you were previously
notified of a `glib::IOCondition::Out` condition. (On Windows in particular, this is
very common due to the way the underlying APIs work.)

On error -1 is returned and `error` is set accordingly. An error will only
be returned if zero messages could be sent; otherwise the number of messages
successfully sent before the error will be returned.

Feature: `v2_44`

## `messages`
an array of `OutputMessage` structs
## `num_messages`
the number of elements in `messages`
## `flags`
an int containing `SocketMsgFlags` flags
## `cancellable`
a `Cancellable` or `None`

# Returns

number of messages sent, or -1 on error. Note that the number of
 messages sent may be smaller than `num_messages` if the socket is
 non-blocking or if `num_messages` was larger than UIO_MAXIOV (1024),
 in which case the caller may re-try to send the remaining messages.
<!-- trait SocketExtManual::fn send_to -->
Tries to send `size` bytes from `buffer` to `address`. If `address` is
`None` then the message is sent to the default receiver (set by
`SocketExt::connect`).

See `Socket::send` for additional information.
## `address`
a `SocketAddress`, or `None`
## `buffer`
the buffer
 containing the data to send.
## `size`
the number of bytes to send
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes written (which may be less than `size`), or -1
on error
<!-- trait SocketExtManual::fn send_with_blocking -->
This behaves exactly the same as `Socket::send`, except that
the choice of blocking or non-blocking behavior is determined by
the `blocking` argument rather than by `self`'s properties.
## `buffer`
the buffer
 containing the data to send.
## `size`
the number of bytes to send
## `blocking`
whether to do blocking or non-blocking I/O
## `cancellable`
a `Cancellable` or `None`

# Returns

Number of bytes written (which may be less than `size`), or -1
on error
<!-- trait SocketExt::fn set_blocking -->
Sets the blocking mode of the socket. In blocking mode
all operations (which don’t take an explicit blocking parameter) block until
they succeed or there is an error. In
non-blocking mode all functions return results immediately or
with a `IOErrorEnum::WouldBlock` error.

All sockets are created in blocking mode. However, note that the
platform level socket is always non-blocking, and blocking mode
is a GSocket level feature.
## `blocking`
Whether to use blocking I/O or not.
<!-- trait SocketExt::fn set_broadcast -->
Sets whether `self` should allow sending to broadcast addresses.
This is `false` by default.
## `broadcast`
whether `self` should allow sending to broadcast
 addresses
<!-- trait SocketExt::fn set_keepalive -->
Sets or unsets the `SO_KEEPALIVE` flag on the underlying socket. When
this flag is set on a socket, the system will attempt to verify that the
remote socket endpoint is still present if a sufficiently long period of
time passes with no data being exchanged. If the system is unable to
verify the presence of the remote endpoint, it will automatically close
the connection.

This option is only functional on certain kinds of sockets. (Notably,
`SocketProtocol::Tcp` sockets.)

The exact time between pings is system- and protocol-dependent, but will
normally be at least two hours. Most commonly, you would set this flag
on a server socket if you want to allow clients to remain idle for long
periods of time, but also want to ensure that connections are eventually
garbage-collected if clients crash or become unreachable.
## `keepalive`
Value for the keepalive flag
<!-- trait SocketExt::fn set_listen_backlog -->
Sets the maximum number of outstanding connections allowed
when listening on this socket. If more clients than this are
connecting to the socket and the application is not handling them
on time then the new connections will be refused.

Note that this must be called before `SocketExt::listen` and has no
effect if called after that.
## `backlog`
the maximum number of pending connections.
<!-- trait SocketExt::fn set_multicast_loopback -->
Sets whether outgoing multicast packets will be received by sockets
listening on that multicast address on the same host. This is `true`
by default.
## `loopback`
whether `self` should receive messages sent to its
 multicast groups from the local host
<!-- trait SocketExt::fn set_multicast_ttl -->
Sets the time-to-live for outgoing multicast datagrams on `self`.
By default, this is 1, meaning that multicast packets will not leave
the local network.
## `ttl`
the time-to-live value for all multicast datagrams on `self`
<!-- trait SocketExt::fn set_option -->
Sets the value of an integer-valued option on `self`, as with
`setsockopt`. (If you need to set a non-integer-valued option,
you will need to call `setsockopt` directly.)

The [<gio/gnetworking.h>][gio-gnetworking.h]
header pulls in system headers that will define most of the
standard/portable socket options. For unusual socket protocols or
platform-dependent options, you may need to include additional
headers.
## `level`
the "API level" of the option (eg, `SOL_SOCKET`)
## `optname`
the "name" of the option (eg, `SO_BROADCAST`)
## `value`
the value to set the option to

# Returns

success or failure. On failure, `error` will be set, and
 the system error value (`errno` or WSAGetLastError()) will still
 be set to the result of the `setsockopt` call.
<!-- trait SocketExt::fn set_timeout -->
Sets the time in seconds after which I/O operations on `self` will
time out if they have not yet completed.

On a blocking socket, this means that any blocking `Socket`
operation will time out after `timeout` seconds of inactivity,
returning `IOErrorEnum::TimedOut`.

On a non-blocking socket, calls to `SocketExt::condition_wait` will
also fail with `IOErrorEnum::TimedOut` after the given time. Sources
created with `Socket::create_source` will trigger after
`timeout` seconds of inactivity, with the requested condition
set, at which point calling `Socket::receive`, `Socket::send`,
`SocketExt::check_connect_result`, etc, will fail with
`IOErrorEnum::TimedOut`.

If `timeout` is 0 (the default), operations will never time out
on their own.

Note that if an I/O operation is interrupted by a signal, this may
cause the timeout to be reset.
## `timeout`
the timeout for `self`, in seconds, or 0 for none
<!-- trait SocketExt::fn set_ttl -->
Sets the time-to-live for outgoing unicast packets on `self`.
By default the platform-specific default value is used.
## `ttl`
the time-to-live value for all unicast packets on `self`
<!-- trait SocketExt::fn shutdown -->
Shut down part or all of a full-duplex connection.

If `shutdown_read` is `true` then the receiving side of the connection
is shut down, and further reading is disallowed.

If `shutdown_write` is `true` then the sending side of the connection
is shut down, and further writing is disallowed.

It is allowed for both `shutdown_read` and `shutdown_write` to be `true`.

One example where it is useful to shut down only one side of a connection is
graceful disconnect for TCP connections where you close the sending side,
then wait for the other side to close the connection, thus ensuring that the
other side saw all sent data.
## `shutdown_read`
whether to shut down the read side
## `shutdown_write`
whether to shut down the write side

# Returns

`true` on success, `false` on error
<!-- trait SocketExt::fn speaks_ipv4 -->
Checks if a socket is capable of speaking IPv4.

IPv4 sockets are capable of speaking IPv4. On some operating systems
and under some combinations of circumstances IPv6 sockets are also
capable of speaking IPv4. See RFC 3493 section 3.7 for more
information.

No other types of sockets are currently considered as being capable
of speaking IPv4.

# Returns

`true` if this socket can be used with IPv4.
<!-- trait SocketExt::fn get_property_broadcast -->
Whether the socket should allow sending to broadcast addresses.
<!-- trait SocketExt::fn set_property_broadcast -->
Whether the socket should allow sending to broadcast addresses.
<!-- trait SocketExt::fn get_property_multicast_loopback -->
Whether outgoing multicast packets loop back to the local host.
<!-- trait SocketExt::fn set_property_multicast_loopback -->
Whether outgoing multicast packets loop back to the local host.
<!-- trait SocketExt::fn get_property_multicast_ttl -->
Time-to-live out outgoing multicast packets
<!-- trait SocketExt::fn set_property_multicast_ttl -->
Time-to-live out outgoing multicast packets
<!-- trait SocketExt::fn get_property_timeout -->
The timeout in seconds on socket I/O
<!-- trait SocketExt::fn set_property_timeout -->
The timeout in seconds on socket I/O
<!-- trait SocketExt::fn get_property_ttl -->
Time-to-live for outgoing unicast packets
<!-- trait SocketExt::fn set_property_ttl -->
Time-to-live for outgoing unicast packets
<!-- struct SocketAddress -->
`SocketAddress` is the equivalent of struct sockaddr in the BSD
sockets API. This is an abstract class; use `InetSocketAddress`
for internet sockets, or `UnixSocketAddress` for UNIX domain sockets.

# Implements

[`SocketAddressExt`](trait.SocketAddressExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketConnectableExt`](trait.SocketConnectableExt.html)
<!-- trait SocketAddressExt -->
Trait containing all `SocketAddress` methods.

# Implementors

[`InetSocketAddress`](struct.InetSocketAddress.html), [`SocketAddress`](struct.SocketAddress.html), [`UnixSocketAddress`](struct.UnixSocketAddress.html)
<!-- impl SocketAddress::fn new_from_native -->
Creates a `SocketAddress` subclass corresponding to the native
struct sockaddr `native`.
## `native`
a pointer to a struct sockaddr
## `len`
the size of the memory location pointed to by `native`

# Returns

a new `SocketAddress` if `native` could successfully
 be converted, otherwise `None`
<!-- trait SocketAddressExt::fn get_family -->
Gets the socket family type of `self`.

# Returns

the socket family type of `self`
<!-- trait SocketAddressExt::fn get_native_size -->
Gets the size of `self`'s native struct sockaddr.
You can use this to allocate memory to pass to
`SocketAddressExt::to_native`.

# Returns

the size of the native struct sockaddr that
 `self` represents
<!-- trait SocketAddressExt::fn to_native -->
Converts a `SocketAddress` to a native struct sockaddr, which can
be passed to low-level functions like `connect` or `bind`.

If not enough space is available, a `IOErrorEnum::NoSpace` error
is returned. If the address type is not known on the system
then a `IOErrorEnum::NotSupported` error is returned.
## `dest`
a pointer to a memory location that will contain the native
struct sockaddr
## `destlen`
the size of `dest`. Must be at least as large as
 `SocketAddressExt::get_native_size`

# Returns

`true` if `dest` was filled in, `false` on error
<!-- struct SocketAddressEnumerator -->
Enumerator type for objects that contain or generate
`SocketAddress` instances.

# Implements

[`SocketAddressEnumeratorExt`](trait.SocketAddressEnumeratorExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait SocketAddressEnumeratorExt -->
Trait containing all `SocketAddressEnumerator` methods.

# Implementors

[`SocketAddressEnumerator`](struct.SocketAddressEnumerator.html)
<!-- trait SocketAddressEnumeratorExt::fn next -->
Retrieves the next `SocketAddress` from `self`. Note that this
may block for some amount of time. (Eg, a `NetworkAddress` may need
to do a DNS lookup before it can return an address.) Use
`SocketAddressEnumeratorExt::next_async` if you need to avoid
blocking.

If `self` is expected to yield addresses, but for some reason
is unable to (eg, because of a DNS error), then the first call to
`SocketAddressEnumeratorExt::next` will return an appropriate error
in *`error`. However, if the first call to
`SocketAddressEnumeratorExt::next` succeeds, then any further
internal errors (other than `cancellable` being triggered) will be
ignored.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `SocketAddress` (owned by the caller), or `None` on
 error (in which case *`error` will be set) or if there are no
 more addresses.
<!-- trait SocketAddressEnumeratorExt::fn next_async -->
Asynchronously retrieves the next `SocketAddress` from `self`
and then calls `callback`, which must call
`SocketAddressEnumeratorExt::next_finish` to get the result.
## `cancellable`
optional `Cancellable` object, `None` to ignore.
## `callback`
a `GAsyncReadyCallback` to call when the request
 is satisfied
## `user_data`
the data to pass to callback function
<!-- trait SocketAddressEnumeratorExt::fn next_finish -->
Retrieves the result of a completed call to
`SocketAddressEnumeratorExt::next_async`. See
`SocketAddressEnumeratorExt::next` for more information about
error handling.
## `result`
a `AsyncResult`

# Returns

a `SocketAddress` (owned by the caller), or `None` on
 error (in which case *`error` will be set) or if there are no
 more addresses.
<!-- struct SocketClient -->
`SocketClient` is a lightweight high-level utility class for connecting to
a network host using a connection oriented socket type.

You create a `SocketClient` object, set any options you want, and then
call a sync or async connect operation, which returns a `SocketConnection`
subclass on success.

The type of the `SocketConnection` object returned depends on the type of
the underlying socket that is in use. For instance, for a TCP/IP connection
it will be a `TcpConnection`.

As `SocketClient` is a lightweight object, you don't need to cache it. You
can just create a new one any time you need one.

# Implements

[`SocketClientExt`](trait.SocketClientExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait SocketClientExt -->
Trait containing all `SocketClient` methods.

# Implementors

[`SocketClient`](struct.SocketClient.html)
<!-- impl SocketClient::fn new -->
Creates a new `SocketClient` with the default options.

# Returns

a `SocketClient`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketClientExt::fn add_application_proxy -->
Enable proxy protocols to be handled by the application. When the
indicated proxy protocol is returned by the `ProxyResolver`,
`SocketClient` will consider this protocol as supported but will
not try to find a `Proxy` instance to handle handshaking. The
application must check for this case by calling
`SocketConnectionExt::get_remote_address` on the returned
`SocketConnection`, and seeing if it's a `ProxyAddress` of the
appropriate type, to determine whether or not it needs to handle
the proxy handshaking itself.

This should be used for proxy protocols that are dialects of
another protocol such as HTTP proxy. It also allows cohabitation of
proxy protocols that are reused between protocols. A good example
is HTTP. It can be used to proxy HTTP, FTP and Gopher and can also
be use as generic socket proxy through the HTTP CONNECT method.

When the proxy is detected as being an application proxy, TLS handshake
will be skipped. This is required to let the application do the proxy
specific handshake.
## `protocol`
The proxy protocol
<!-- trait SocketClientExt::fn connect -->
Tries to resolve the `connectable` and make a network connection to it.

Upon a successful connection, a new `SocketConnection` is constructed
and returned. The caller owns this new object and must drop their
reference to it when finished with it.

The type of the `SocketConnection` object returned depends on the type of
the underlying socket that is used. For instance, for a TCP/IP connection
it will be a `TcpConnection`.

The socket created will be the same family as the address that the
`connectable` resolves to, unless family is set with `SocketClientExt::set_family`
or indirectly via `SocketClientExt::set_local_address`. The socket type
defaults to `SocketType::Stream` but can be set with
`SocketClientExt::set_socket_type`.

If a local address is specified with `SocketClientExt::set_local_address` the
socket will be bound to this address before connecting.
## `connectable`
a `SocketConnectable` specifying the remote address.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn connect_async -->
This is the asynchronous version of `SocketClientExt::connect`.

When the operation is finished `callback` will be
called. You can then call `SocketClientExt::connect_finish` to get
the result of the operation.
## `connectable`
a `SocketConnectable` specifying the remote address.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketClientExt::fn connect_finish -->
Finishes an async connect operation. See `SocketClientExt::connect_async`
## `result`
a `AsyncResult`.

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn connect_to_host -->
This is a helper function for `SocketClientExt::connect`.

Attempts to create a TCP connection to the named host.

`host_and_port` may be in any of a number of recognized formats; an IPv6
address, an IPv4 address, or a domain name (in which case a DNS
lookup is performed). Quoting with [] is supported for all address
types. A port override may be specified in the usual way with a
colon. Ports may be given as decimal numbers or symbolic names (in
which case an /etc/services lookup is performed).

If no port override is given in `host_and_port` then `default_port` will be
used as the port number to connect to.

In general, `host_and_port` is expected to be provided by the user (allowing
them to give the hostname, and a port override if necessary) and
`default_port` is expected to be provided by the application.

In the case that an IP address is given, a single connection
attempt is made. In the case that a name is given, multiple
connection attempts may be made, in turn and according to the
number of address records in DNS, until a connection succeeds.

Upon a successful connection, a new `SocketConnection` is constructed
and returned. The caller owns this new object and must drop their
reference to it when finished with it.

In the event of any failure (DNS error, service not found, no hosts
connectable) `None` is returned and `error` (if non-`None`) is set
accordingly.
## `host_and_port`
the name and optionally port of the host to connect to
## `default_port`
the default port to connect to
## `cancellable`
a `Cancellable`, or `None`

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn connect_to_host_async -->
This is the asynchronous version of `SocketClientExt::connect_to_host`.

When the operation is finished `callback` will be
called. You can then call `SocketClientExt::connect_to_host_finish` to get
the result of the operation.
## `host_and_port`
the name and optionally the port of the host to connect to
## `default_port`
the default port to connect to
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketClientExt::fn connect_to_host_finish -->
Finishes an async connect operation. See `SocketClientExt::connect_to_host_async`
## `result`
a `AsyncResult`.

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn connect_to_service -->
Attempts to create a TCP connection to a service.

This call looks up the SRV record for `service` at `domain` for the
"tcp" protocol. It then attempts to connect, in turn, to each of
the hosts providing the service until either a connection succeeds
or there are no hosts remaining.

Upon a successful connection, a new `SocketConnection` is constructed
and returned. The caller owns this new object and must drop their
reference to it when finished with it.

In the event of any failure (DNS error, service not found, no hosts
connectable) `None` is returned and `error` (if non-`None`) is set
accordingly.
## `domain`
a domain name
## `service`
the name of the service to connect to
## `cancellable`
a `Cancellable`, or `None`

# Returns

a `SocketConnection` if successful, or `None` on error
<!-- trait SocketClientExt::fn connect_to_service_async -->
This is the asynchronous version of
`SocketClientExt::connect_to_service`.
## `domain`
a domain name
## `service`
the name of the service to connect to
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketClientExt::fn connect_to_service_finish -->
Finishes an async connect operation. See `SocketClientExt::connect_to_service_async`
## `result`
a `AsyncResult`.

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn connect_to_uri -->
This is a helper function for `SocketClientExt::connect`.

Attempts to create a TCP connection with a network URI.

`uri` may be any valid URI containing an "authority" (hostname/port)
component. If a port is not specified in the URI, `default_port`
will be used. TLS will be negotiated if `SocketClient:tls` is `true`.
(`SocketClient` does not know to automatically assume TLS for
certain URI schemes.)

Using this rather than `SocketClientExt::connect` or
`SocketClientExt::connect_to_host` allows `SocketClient` to
determine when to use application-specific proxy protocols.

Upon a successful connection, a new `SocketConnection` is constructed
and returned. The caller owns this new object and must drop their
reference to it when finished with it.

In the event of any failure (DNS error, service not found, no hosts
connectable) `None` is returned and `error` (if non-`None`) is set
accordingly.
## `uri`
A network URI
## `default_port`
the default port to connect to
## `cancellable`
a `Cancellable`, or `None`

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn connect_to_uri_async -->
This is the asynchronous version of `SocketClientExt::connect_to_uri`.

When the operation is finished `callback` will be
called. You can then call `SocketClientExt::connect_to_uri_finish` to get
the result of the operation.
## `uri`
a network uri
## `default_port`
the default port to connect to
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketClientExt::fn connect_to_uri_finish -->
Finishes an async connect operation. See `SocketClientExt::connect_to_uri_async`
## `result`
a `AsyncResult`.

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketClientExt::fn get_enable_proxy -->
Gets the proxy enable state; see `SocketClientExt::set_enable_proxy`

# Returns

whether proxying is enabled
<!-- trait SocketClientExt::fn get_family -->
Gets the socket family of the socket client.

See `SocketClientExt::set_family` for details.

# Returns

a `SocketFamily`
<!-- trait SocketClientExt::fn get_local_address -->
Gets the local address of the socket client.

See `SocketClientExt::set_local_address` for details.

# Returns

a `SocketAddress` or `None`. Do not free.
<!-- trait SocketClientExt::fn get_protocol -->
Gets the protocol name type of the socket client.

See `SocketClientExt::set_protocol` for details.

# Returns

a `SocketProtocol`
<!-- trait SocketClientExt::fn get_proxy_resolver -->
Gets the `ProxyResolver` being used by `self`. Normally, this will
be the resolver returned by `ProxyResolver::get_default`, but you
can override it with `SocketClientExt::set_proxy_resolver`.

# Returns

The `ProxyResolver` being used by
 `self`.
<!-- trait SocketClientExt::fn get_socket_type -->
Gets the socket type of the socket client.

See `SocketClientExt::set_socket_type` for details.

# Returns

a `SocketFamily`
<!-- trait SocketClientExt::fn get_timeout -->
Gets the I/O timeout time for sockets created by `self`.

See `SocketClientExt::set_timeout` for details.

# Returns

the timeout in seconds
<!-- trait SocketClientExt::fn get_tls -->
Gets whether `self` creates TLS connections. See
`SocketClientExt::set_tls` for details.

# Returns

whether `self` uses TLS
<!-- trait SocketClientExt::fn get_tls_validation_flags -->
Gets the TLS validation flags used creating TLS connections via
`self`.

# Returns

the TLS validation flags
<!-- trait SocketClientExt::fn set_enable_proxy -->
Sets whether or not `self` attempts to make connections via a
proxy server. When enabled (the default), `SocketClient` will use a
`ProxyResolver` to determine if a proxy protocol such as SOCKS is
needed, and automatically do the necessary proxy negotiation.

See also `SocketClientExt::set_proxy_resolver`.
## `enable`
whether to enable proxies
<!-- trait SocketClientExt::fn set_family -->
Sets the socket family of the socket client.
If this is set to something other than `SocketFamily::Invalid`
then the sockets created by this object will be of the specified
family.

This might be useful for instance if you want to force the local
connection to be an ipv4 socket, even though the address might
be an ipv6 mapped to ipv4 address.
## `family`
a `SocketFamily`
<!-- trait SocketClientExt::fn set_local_address -->
Sets the local address of the socket client.
The sockets created by this object will bound to the
specified address (if not `None`) before connecting.

This is useful if you want to ensure that the local
side of the connection is on a specific port, or on
a specific interface.
## `address`
a `SocketAddress`, or `None`
<!-- trait SocketClientExt::fn set_protocol -->
Sets the protocol of the socket client.
The sockets created by this object will use of the specified
protocol.

If `protocol` is `0` that means to use the default
protocol for the socket family and type.
## `protocol`
a `SocketProtocol`
<!-- trait SocketClientExt::fn set_proxy_resolver -->
Overrides the `ProxyResolver` used by `self`. You can call this if
you want to use specific proxies, rather than using the system
default proxy settings.

Note that whether or not the proxy resolver is actually used
depends on the setting of `SocketClient:enable-proxy`, which is not
changed by this function (but which is `true` by default)
## `proxy_resolver`
a `ProxyResolver`, or `None` for the
 default.
<!-- trait SocketClientExt::fn set_socket_type -->
Sets the socket type of the socket client.
The sockets created by this object will be of the specified
type.

It doesn't make sense to specify a type of `SocketType::Datagram`,
as GSocketClient is used for connection oriented services.
## `type_`
a `SocketType`
<!-- trait SocketClientExt::fn set_timeout -->
Sets the I/O timeout for sockets created by `self`. `timeout` is a
time in seconds, or 0 for no timeout (the default).

The timeout value affects the initial connection attempt as well,
so setting this may cause calls to `SocketClientExt::connect`, etc,
to fail with `IOErrorEnum::TimedOut`.
## `timeout`
the timeout
<!-- trait SocketClientExt::fn set_tls -->
Sets whether `self` creates TLS (aka SSL) connections. If `tls` is
`true`, `self` will wrap its connections in a `TlsClientConnection`
and perform a TLS handshake when connecting.

Note that since `SocketClient` must return a `SocketConnection`,
but `TlsClientConnection` is not a `SocketConnection`, this
actually wraps the resulting `TlsClientConnection` in a
`TcpWrapperConnection` when returning it. You can use
`TcpWrapperConnection::get_base_io_stream` on the return value
to extract the `TlsClientConnection`.

If you need to modify the behavior of the TLS handshake (eg, by
setting a client-side certificate to use, or connecting to the
`TlsConnection::accept-certificate` signal), you can connect to
`self`'s `SocketClient::event` signal and wait for it to be
emitted with `SocketClientEvent::TlsHandshaking`, which will give you
a chance to see the `TlsClientConnection` before the handshake
starts.
## `tls`
whether to use TLS
<!-- trait SocketClientExt::fn set_tls_validation_flags -->
Sets the TLS validation flags used when creating TLS connections
via `self`. The default value is `TlsCertificateFlags::ValidateAll`.
## `flags`
the validation flags
<!-- trait SocketClientExt::fn connect_event -->
Emitted when `client`'s activity on `connectable` changes state.
Among other things, this can be used to provide progress
information about a network connection in the UI. The meanings of
the different `event` values are as follows:

- `SocketClientEvent::Resolving`: `client` is about to look up `connectable`
 in DNS. `connection` will be `None`.

- `SocketClientEvent::Resolved`: `client` has successfully resolved
 `connectable` in DNS. `connection` will be `None`.

- `SocketClientEvent::Connecting`: `client` is about to make a connection
 to a remote host; either a proxy server or the destination server
 itself. `connection` is the `SocketConnection`, which is not yet
 connected. Since GLib 2.40, you can access the remote
 address via `SocketConnectionExt::get_remote_address`.

- `SocketClientEvent::Connected`: `client` has successfully connected
 to a remote host. `connection` is the connected `SocketConnection`.

- `SocketClientEvent::ProxyNegotiating`: `client` is about to negotiate
 with a proxy to get it to connect to `connectable`. `connection` is
 the `SocketConnection` to the proxy server.

- `SocketClientEvent::ProxyNegotiated`: `client` has negotiated a
 connection to `connectable` through a proxy server. `connection` is
 the stream returned from `Proxy::connect`, which may or may not
 be a `SocketConnection`.

- `SocketClientEvent::TlsHandshaking`: `client` is about to begin a TLS
 handshake. `connection` is a `TlsClientConnection`.

- `SocketClientEvent::TlsHandshaked`: `client` has successfully completed
 the TLS handshake. `connection` is a `TlsClientConnection`.

- `SocketClientEvent::Complete`: `client` has either successfully connected
 to `connectable` (in which case `connection` is the `SocketConnection`
 that it will be returning to the caller) or has failed (in which
 case `connection` is `None` and the client is about to return an error).

Each event except `SocketClientEvent::Complete` may be emitted
multiple times (or not at all) for a given connectable (in
particular, if `client` ends up attempting to connect to more than
one address). However, if `client` emits the `SocketClient::event`
signal at all for a given connectable, that it will always emit
it with `SocketClientEvent::Complete` when it is done.

Note that there may be additional `SocketClientEvent` values in
the future; unrecognized `event` values should be ignored.
## `event`
the event that is occurring
## `connectable`
the `SocketConnectable` that `event` is occurring on
## `connection`
the current representation of the connection
<!-- trait SocketClientExt::fn get_property_proxy_resolver -->
The proxy resolver to use
<!-- trait SocketClientExt::fn set_property_proxy_resolver -->
The proxy resolver to use
<!-- enum SocketClientEvent -->
Describes an event occurring on a `SocketClient`. See the
`SocketClient::event` signal for more details.

Additional values may be added to this type in the future.
<!-- enum SocketClientEvent::variant Resolving -->
The client is doing a DNS lookup.
<!-- enum SocketClientEvent::variant Resolved -->
The client has completed a DNS lookup.
<!-- enum SocketClientEvent::variant Connecting -->
The client is connecting to a remote
 host (either a proxy or the destination server).
<!-- enum SocketClientEvent::variant Connected -->
The client has connected to a remote
 host.
<!-- enum SocketClientEvent::variant ProxyNegotiating -->
The client is negotiating
 with a proxy to connect to the destination server.
<!-- enum SocketClientEvent::variant ProxyNegotiated -->
The client has negotiated
 with the proxy server.
<!-- enum SocketClientEvent::variant TlsHandshaking -->
The client is performing a
 TLS handshake.
<!-- enum SocketClientEvent::variant TlsHandshaked -->
The client has performed a
 TLS handshake.
<!-- enum SocketClientEvent::variant Complete -->
The client is done with a particular
 `SocketConnectable`.
<!-- struct SocketConnectable -->
Objects that describe one or more potential socket endpoints
implement `SocketConnectable`. Callers can then use
`SocketConnectable::enumerate` to get a `SocketAddressEnumerator`
to try out each socket address in turn until one succeeds, as shown
in the sample code below.


```C
MyConnectionType *
connect_to_host (const char    *hostname,
                 guint16        port,
                 GCancellable  *cancellable,
                 GError       **error)
{
  MyConnection *conn = NULL;
  GSocketConnectable *addr;
  GSocketAddressEnumerator *enumerator;
  GSocketAddress *sockaddr;
  GError *conn_error = NULL;

  addr = g_network_address_new (hostname, port);
  enumerator = g_socket_connectable_enumerate (addr);
  g_object_unref (addr);

  // Try each sockaddr until we succeed. Record the first connection error,
  // but not any further ones (since they'll probably be basically the same
  // as the first).
  while (!conn && (sockaddr = g_socket_address_enumerator_next (enumerator, cancellable, error))
    {
      conn = connect_to_sockaddr (sockaddr, conn_error ? NULL : &conn_error);
      g_object_unref (sockaddr);
    }
  g_object_unref (enumerator);

  if (conn)
    {
      if (conn_error)
        {
          // We couldn't connect to the first address, but we succeeded
          // in connecting to a later address.
          g_error_free (conn_error);
        }
      return conn;
    }
  else if (error)
    {
      /// Either initial lookup failed, or else the caller cancelled us.
      if (conn_error)
        g_error_free (conn_error);
      return NULL;
    }
  else
    {
      g_error_propagate (error, conn_error);
      return NULL;
    }
}
```

# Implements

[`SocketConnectableExt`](trait.SocketConnectableExt.html)
<!-- trait SocketConnectableExt -->
Trait containing all `SocketConnectable` methods.

# Implementors

[`InetSocketAddress`](struct.InetSocketAddress.html), [`NetworkAddress`](struct.NetworkAddress.html), [`NetworkService`](struct.NetworkService.html), [`ProxyAddress`](struct.ProxyAddress.html), [`SocketAddress`](struct.SocketAddress.html), [`SocketConnectable`](struct.SocketConnectable.html), [`UnixSocketAddress`](struct.UnixSocketAddress.html)
<!-- trait SocketConnectableExt::fn enumerate -->
Creates a `SocketAddressEnumerator` for `self`.

# Returns

a new `SocketAddressEnumerator`.
<!-- trait SocketConnectableExt::fn proxy_enumerate -->
Creates a `SocketAddressEnumerator` for `self` that will
return `GProxyAddresses` for addresses that you must connect
to via a proxy.

If `self` does not implement
`SocketConnectable::proxy_enumerate`, this will fall back to
calling `SocketConnectable::enumerate`.

# Returns

a new `SocketAddressEnumerator`.
<!-- trait SocketConnectableExt::fn to_string -->
Format a `SocketConnectable` as a string. This is a human-readable format for
use in debugging output, and is not a stable serialization format. It is not
suitable for use in user interfaces as it exposes too much information for a
user.

If the `SocketConnectable` implementation does not support string formatting,
the implementation’s type name will be returned as a fallback.

Feature: `v2_48`


# Returns

the formatted string
<!-- struct SocketConnection -->
`SocketConnection` is a `IOStream` for a connected socket. They
can be created either by `SocketClient` when connecting to a host,
or by `SocketListener` when accepting a new client.

The type of the `SocketConnection` object returned from these calls
depends on the type of the underlying socket that is in use. For
instance, for a TCP/IP connection it will be a `TcpConnection`.

Choosing what type of object to construct is done with the socket
connection factory, and it is possible for 3rd parties to register
custom socket connection types for specific combination of socket
family/type/protocol using `SocketConnection::factory_register_type`.

To close a `SocketConnection`, use `IOStreamExt::close`. Closing both
substreams of the `IOStream` separately will not close the underlying
`Socket`.

# Implements

[`SocketConnectionExt`](trait.SocketConnectionExt.html), [`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait SocketConnectionExt -->
Trait containing all `SocketConnection` methods.

# Implementors

[`SocketConnection`](struct.SocketConnection.html), [`TcpConnection`](struct.TcpConnection.html)
<!-- impl SocketConnection::fn factory_lookup_type -->
Looks up the `glib::Type` to be used when creating socket connections on
sockets with the specified `family`, `type_` and `protocol_id`.

If no type is registered, the `SocketConnection` base type is returned.
## `family`
a `SocketFamily`
## `type_`
a `SocketType`
## `protocol_id`
a protocol id

# Returns

a `glib::Type`
<!-- impl SocketConnection::fn factory_register_type -->
Looks up the `glib::Type` to be used when creating socket connections on
sockets with the specified `family`, `type_` and `protocol`.

If no type is registered, the `SocketConnection` base type is returned.
## `g_type`
a `glib::Type`, inheriting from `G_TYPE_SOCKET_CONNECTION`
## `family`
a `SocketFamily`
## `type_`
a `SocketType`
## `protocol`
a protocol id
<!-- trait SocketConnectionExt::fn connect -->
Connect `self` to the specified remote address.
## `address`
a `SocketAddress` specifying the remote address.
## `cancellable`
a `Cancellable` or `None`

# Returns

`true` if the connection succeeded, `false` on error
<!-- trait SocketConnectionExt::fn connect_async -->
Asynchronously connect `self` to the specified remote address.

This clears the `Socket:blocking` flag on `self`'s underlying
socket if it is currently set.

Use `SocketConnectionExt::connect_finish` to retrieve the result.
## `address`
a `SocketAddress` specifying the remote address.
## `cancellable`
a `Cancellable` or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketConnectionExt::fn connect_finish -->
Gets the result of a `SocketConnectionExt::connect_async` call.
## `result`
the `AsyncResult`

# Returns

`true` if the connection succeeded, `false` on error
<!-- trait SocketConnectionExt::fn get_local_address -->
Try to get the local address of a socket connection.

# Returns

a `SocketAddress` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketConnectionExt::fn get_remote_address -->
Try to get the remote address of a socket connection.

Since GLib 2.40, when used with `SocketClientExt::connect` or
`SocketClientExt::connect_async`, during emission of
`SocketClientEvent::Connecting`, this function will return the remote
address that will be used for the connection. This allows
applications to print e.g. "Connecting to example.com
(10.42.77.3)...".

# Returns

a `SocketAddress` or `None` on error.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait SocketConnectionExt::fn get_socket -->
Gets the underlying `Socket` object of the connection.
This can be useful if you want to do something unusual on it
not supported by the `SocketConnection` APIs.

# Returns

a `Socket` or `None` on error.
<!-- trait SocketConnectionExt::fn is_connected -->
Checks if `self` is connected. This is equivalent to calling
`SocketExt::is_connected` on `self`'s underlying `Socket`.

# Returns

whether `self` is connected
<!-- enum SocketFamily -->
The protocol family of a `SocketAddress`. (These values are
identical to the system defines `AF_INET`, `AF_INET6` and `AF_UNIX`,
if available.)
<!-- enum SocketFamily::variant Invalid -->
no address family
<!-- enum SocketFamily::variant Unix -->
the UNIX domain family
<!-- enum SocketFamily::variant Ipv4 -->
the IPv4 family
<!-- enum SocketFamily::variant Ipv6 -->
the IPv6 family
<!-- struct SocketListener -->
A `SocketListener` is an object that keeps track of a set
of server sockets and helps you accept sockets from any of the
socket, either sync or async.

Add addresses and ports to listen on using `SocketListenerExt::add_address`
and `SocketListenerExt::add_inet_port`. These will be listened on until
`SocketListenerExt::close` is called. Dropping your final reference to the
`SocketListener` will not cause `SocketListenerExt::close` to be called
implicitly, as some references to the `SocketListener` may be held
internally.

If you want to implement a network server, also look at `SocketService`
and `ThreadedSocketService` which are subclasses of `SocketListener`
that make this even easier.

# Implements

[`SocketListenerExt`](trait.SocketListenerExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketListenerExtManual`](prelude/trait.SocketListenerExtManual.html)
<!-- trait SocketListenerExt -->
Trait containing all `SocketListener` methods.

# Implementors

[`SocketListener`](struct.SocketListener.html), [`SocketService`](struct.SocketService.html)
<!-- impl SocketListener::fn new -->
Creates a new `SocketListener` with no sockets to listen for.
New listeners can be added with e.g. `SocketListenerExt::add_address`
or `SocketListenerExt::add_inet_port`.

# Returns

a new `SocketListener`.
<!-- trait SocketListenerExt::fn accept -->
Blocks waiting for a client to connect to any of the sockets added
to the listener. Returns a `SocketConnection` for the socket that was
accepted.

If `source_object` is not `None` it will be filled out with the source
object specified when the corresponding socket or address was added
to the listener.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `source_object`
location where `gobject::Object` pointer will be stored, or `None`
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketListenerExtManual::fn accept_async -->
This is the asynchronous version of `SocketListenerExt::accept`.

When the operation is finished `callback` will be
called. You can then call `SocketListenerExt::accept_socket`
to get the result of the operation.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketListenerExt::fn accept_finish -->
Finishes an async accept operation. See `SocketListener::accept_async`
## `result`
a `AsyncResult`.
## `source_object`
Optional `gobject::Object` identifying this source

# Returns

a `SocketConnection` on success, `None` on error.
<!-- trait SocketListenerExt::fn accept_socket -->
Blocks waiting for a client to connect to any of the sockets added
to the listener. Returns the `Socket` that was accepted.

If you want to accept the high-level `SocketConnection`, not a `Socket`,
which is often the case, then you should use `SocketListenerExt::accept`
instead.

If `source_object` is not `None` it will be filled out with the source
object specified when the corresponding socket or address was added
to the listener.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.
## `source_object`
location where `gobject::Object` pointer will be stored, or `None`.
## `cancellable`
optional `Cancellable` object, `None` to ignore.

# Returns

a `Socket` on success, `None` on error.
<!-- trait SocketListenerExtManual::fn accept_socket_async -->
This is the asynchronous version of `SocketListenerExt::accept_socket`.

When the operation is finished `callback` will be
called. You can then call `SocketListenerExt::accept_socket_finish`
to get the result of the operation.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback`
## `user_data`
user data for the callback
<!-- trait SocketListenerExt::fn accept_socket_finish -->
Finishes an async accept operation. See `SocketListener::accept_socket_async`
## `result`
a `AsyncResult`.
## `source_object`
Optional `gobject::Object` identifying this source

# Returns

a `Socket` on success, `None` on error.
<!-- trait SocketListenerExt::fn add_address -->
Creates a socket of type `type_` and protocol `protocol`, binds
it to `address` and adds it to the set of sockets we're accepting
sockets from.

Note that adding an IPv6 address, depending on the platform,
may or may not result in a listener that also accepts IPv4
connections. For more deterministic behavior, see
`SocketListenerExt::add_inet_port`.

`source_object` will be passed out in the various calls
to accept to identify this particular source, which is
useful if you're listening on multiple addresses and do
different things depending on what address is connected to.

If successful and `effective_address` is non-`None` then it will
be set to the address that the binding actually occurred at. This
is helpful for determining the port number that was used for when
requesting a binding to port 0 (ie: "any port"). This address, if
requested, belongs to the caller and must be freed.

Call `SocketListenerExt::close` to stop listening on `address`; this will not
be done automatically when you drop your final reference to `self`, as
references may be held internally.
## `address`
a `SocketAddress`
## `type_`
a `SocketType`
## `protocol`
a `SocketProtocol`
## `source_object`
Optional `gobject::Object` identifying this source
## `effective_address`
location to store the address that was bound to, or `None`.

# Returns

`true` on success, `false` on error.
<!-- trait SocketListenerExt::fn add_any_inet_port -->
Listens for TCP connections on any available port number for both
IPv6 and IPv4 (if each is available).

This is useful if you need to have a socket for incoming connections
but don't care about the specific port number.

`source_object` will be passed out in the various calls
to accept to identify this particular source, which is
useful if you're listening on multiple addresses and do
different things depending on what address is connected to.
## `source_object`
Optional `gobject::Object` identifying this source

# Returns

the port number, or 0 in case of failure.
<!-- trait SocketListenerExt::fn add_inet_port -->
Helper function for `SocketListenerExt::add_address` that
creates a TCP/IP socket listening on IPv4 and IPv6 (if
supported) on the specified port on all interfaces.

`source_object` will be passed out in the various calls
to accept to identify this particular source, which is
useful if you're listening on multiple addresses and do
different things depending on what address is connected to.

Call `SocketListenerExt::close` to stop listening on `port`; this will not
be done automatically when you drop your final reference to `self`, as
references may be held internally.
## `port`
an IP port number (non-zero)
## `source_object`
Optional `gobject::Object` identifying this source

# Returns

`true` on success, `false` on error.
<!-- trait SocketListenerExt::fn add_socket -->
Adds `socket` to the set of sockets that we try to accept
new clients from. The socket must be bound to a local
address and listened to.

`source_object` will be passed out in the various calls
to accept to identify this particular source, which is
useful if you're listening on multiple addresses and do
different things depending on what address is connected to.

The `socket` will not be automatically closed when the `self` is finalized
unless the listener held the final reference to the socket. Before GLib 2.42,
the `socket` was automatically closed on finalization of the `self`, even
if references to it were held elsewhere.
## `socket`
a listening `Socket`
## `source_object`
Optional `gobject::Object` identifying this source

# Returns

`true` on success, `false` on error.
<!-- trait SocketListenerExt::fn close -->
Closes all the sockets in the listener.
<!-- trait SocketListenerExt::fn set_backlog -->
Sets the listen backlog on the sockets in the listener.

See `SocketExt::set_listen_backlog` for details
## `listen_backlog`
an integer
<!-- trait SocketListenerExt::fn connect_event -->
Emitted when `listener`'s activity on `socket` changes state.
Note that when `listener` is used to listen on both IPv4 and
IPv6, a separate set of signals will be emitted for each, and
the order they happen in is undefined.

Feature: `v2_46`

## `event`
the event that is occurring
## `socket`
the `Socket` the event is occurring on
<!-- enum SocketListenerEvent -->
Describes an event occurring on a `SocketListener`. See the
`SocketListener::event` signal for more details.

Additional values may be added to this type in the future.
<!-- enum SocketListenerEvent::variant Binding -->
The listener is about to bind a socket.
<!-- enum SocketListenerEvent::variant Bound -->
The listener has bound a socket.
<!-- enum SocketListenerEvent::variant Listening -->
The listener is about to start
 listening on this socket.
<!-- enum SocketListenerEvent::variant Listened -->
The listener is now listening on
 this socket.

Feature: `v2_46`

<!-- enum SocketProtocol -->
A protocol identifier is specified when creating a `Socket`, which is a
family/type specific identifier, where 0 means the default protocol for
the particular family/type.

This enum contains a set of commonly available and used protocols. You
can also pass any other identifiers handled by the platform in order to
use protocols not listed here.
<!-- enum SocketProtocol::variant Unknown -->
The protocol type is unknown
<!-- enum SocketProtocol::variant Default -->
The default protocol for the family/type
<!-- enum SocketProtocol::variant Tcp -->
TCP over IP
<!-- enum SocketProtocol::variant Udp -->
UDP over IP
<!-- enum SocketProtocol::variant Sctp -->
SCTP over IP
<!-- struct SocketService -->
A `SocketService` is an object that represents a service that
is provided to the network or over local sockets. When a new
connection is made to the service the `SocketService::incoming`
signal is emitted.

A `SocketService` is a subclass of `SocketListener` and you need
to add the addresses you want to accept connections on with the
`SocketListener` APIs.

There are two options for implementing a network service based on
`SocketService`. The first is to create the service using
`SocketService::new` and to connect to the `SocketService::incoming`
signal. The second is to subclass `SocketService` and override the
default signal handler implementation.

In either case, the handler must immediately return, or else it
will block additional incoming connections from being serviced.
If you are interested in writing connection handlers that contain
blocking code then see `ThreadedSocketService`.

The socket service runs on the main loop of the
[thread-default context][g-main-context-push-thread-default-context]
of the thread it is created in, and is not
threadsafe in general. However, the calls to start and stop the
service are thread-safe so these can be used from threads that
handle incoming clients.

# Implements

[`SocketServiceExt`](trait.SocketServiceExt.html), [`SocketListenerExt`](trait.SocketListenerExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketListenerExtManual`](prelude/trait.SocketListenerExtManual.html)
<!-- trait SocketServiceExt -->
Trait containing all `SocketService` methods.

# Implementors

[`SocketService`](struct.SocketService.html), [`ThreadedSocketService`](struct.ThreadedSocketService.html)
<!-- impl SocketService::fn new -->
Creates a new `SocketService` with no sockets to listen for.
New listeners can be added with e.g. `SocketListenerExt::add_address`
or `SocketListenerExt::add_inet_port`.

New services are created active, there is no need to call
`SocketServiceExt::start`, unless `SocketServiceExt::stop` has been
called before.

# Returns

a new `SocketService`.
<!-- trait SocketServiceExt::fn is_active -->
Check whether the service is active or not. An active
service will accept new clients that connect, while
a non-active service will let connecting clients queue
up until the service is started.

# Returns

`true` if the service is active, `false` otherwise
<!-- trait SocketServiceExt::fn start -->
Restarts the service, i.e. start accepting connections
from the added sockets when the mainloop runs. This only needs
to be called after the service has been stopped from
`SocketServiceExt::stop`.

This call is thread-safe, so it may be called from a thread
handling an incoming client request.
<!-- trait SocketServiceExt::fn stop -->
Stops the service, i.e. stops accepting connections
from the added sockets when the mainloop runs.

This call is thread-safe, so it may be called from a thread
handling an incoming client request.

Note that this only stops accepting new connections; it does not
close the listening sockets, and you can call
`SocketServiceExt::start` again later to begin listening again. To
close the listening sockets, call `SocketListenerExt::close`. (This
will happen automatically when the `SocketService` is finalized.)

This must be called before calling `SocketListenerExt::close` as
the socket service will start accepting connections immediately
when a new socket is added.
<!-- trait SocketServiceExt::fn connect_incoming -->
The ::incoming signal is emitted when a new incoming connection
to `service` needs to be handled. The handler must initiate the
handling of `connection`, but may not block; in essence,
asynchronous operations must be used.

`connection` will be unreffed once the signal handler returns,
so you need to ref it yourself if you are planning to use it.
## `connection`
a new `SocketConnection` object
## `source_object`
the source_object passed to
 `SocketListenerExt::add_address`

# Returns

`true` to stop other handlers from being called
<!-- trait SocketServiceExt::fn get_property_active -->
Whether the service is currently accepting connections.

Feature: `v2_46`

<!-- trait SocketServiceExt::fn set_property_active -->
Whether the service is currently accepting connections.

Feature: `v2_46`

<!-- enum SocketType -->
Flags used when creating a `Socket`. Some protocols may not implement
all the socket types.
<!-- enum SocketType::variant Invalid -->
Type unknown or wrong
<!-- enum SocketType::variant Stream -->
Reliable connection-based byte streams (e.g. TCP).
<!-- enum SocketType::variant Datagram -->
Connectionless, unreliable datagram passing.
 (e.g. UDP)
<!-- enum SocketType::variant Seqpacket -->
Reliable connection-based passing of datagrams
 of fixed maximum length (e.g. SCTP).
<!-- struct SrvTarget -->
SRV (service) records are used by some network protocols to provide
service-specific aliasing and load-balancing. For example, XMPP
(Jabber) uses SRV records to locate the XMPP server for a domain;
rather than connecting directly to "example.com" or assuming a
specific server hostname like "xmpp.example.com", an XMPP client
would look up the "xmpp-client" SRV record for "example.com", and
then connect to whatever host was pointed to by that record.

You can use `ResolverExt::lookup_service` or
`ResolverExt::lookup_service_async` to find the `GSrvTargets`
for a given service. However, if you are simply planning to connect
to the remote service, you can use `NetworkService`'s
`SocketConnectable` interface and not need to worry about
`SrvTarget` at all.
<!-- impl SrvTarget::fn new -->
Creates a new `SrvTarget` with the given parameters.

You should not need to use this; normally `GSrvTargets` are
created by `Resolver`.
## `hostname`
the host that the service is running on
## `port`
the port that the service is running on
## `priority`
the target's priority
## `weight`
the target's weight

# Returns

a new `SrvTarget`.
<!-- impl SrvTarget::fn copy -->
Copies `self`

# Returns

a copy of `self`
<!-- impl SrvTarget::fn free -->
Frees `self`
<!-- impl SrvTarget::fn get_hostname -->
Gets `self`'s hostname (in ASCII form; if you are going to present
this to the user, you should use `g_hostname_is_ascii_encoded` to
check if it contains encoded Unicode segments, and use
`g_hostname_to_unicode` to convert it if it does.)

# Returns

`self`'s hostname
<!-- impl SrvTarget::fn get_port -->
Gets `self`'s port

# Returns

`self`'s port
<!-- impl SrvTarget::fn get_priority -->
Gets `self`'s priority. You should not need to look at this;
`Resolver` already sorts the targets according to the algorithm in
RFC 2782.

# Returns

`self`'s priority
<!-- impl SrvTarget::fn get_weight -->
Gets `self`'s weight. You should not need to look at this;
`Resolver` already sorts the targets according to the algorithm in
RFC 2782.

# Returns

`self`'s weight
<!-- impl SrvTarget::fn list_sort -->
Sorts `targets` in place according to the algorithm in RFC 2782.
## `targets`
a `glib::List` of `SrvTarget`

# Returns

the head of the sorted list.
<!-- struct Subprocess -->
`Subprocess` allows the creation of and interaction with child
processes.

Processes can be communicated with using standard GIO-style APIs (ie:
`InputStream`, `OutputStream`). There are GIO-style APIs to wait for
process termination (ie: cancellable and with an asynchronous
variant).

There is an API to force a process to terminate, as well as a
race-free API for sending UNIX signals to a subprocess.

One major advantage that GIO brings over the core GLib library is
comprehensive API for asynchronous I/O, such
`OutputStreamExt::splice_async`. This makes GSubprocess
significantly more powerful and flexible than equivalent APIs in
some other languages such as the `subprocess.py`
included with Python. For example, using `Subprocess` one could
create two child processes, reading standard output from the first,
processing it, and writing to the input stream of the second, all
without blocking the main loop.

A powerful `Subprocess::communicate` API is provided similar to the
`communicate()` method of `subprocess.py`. This enables very easy
interaction with a subprocess that has been opened with pipes.

`Subprocess` defaults to tight control over the file descriptors open
in the child process, avoiding dangling-fd issues that are caused by
a simple `fork`/`exec`. The only open file descriptors in the
spawned process are ones that were explicitly specified by the
`Subprocess` API (unless `SubprocessFlags::InheritFds` was
specified).

`Subprocess` will quickly reap all child processes as they exit,
avoiding "zombie processes" remaining around for long periods of
time. `Subprocess::wait` can be used to wait for this to happen,
but it will happen even without the call being explicitly made.

As a matter of principle, `Subprocess` has no API that accepts
shell-style space-separated strings. It will, however, match the
typical shell behaviour of searching the PATH for executables that do
not contain a directory separator in their name.

`Subprocess` attempts to have a very simple API for most uses (ie:
spawning a subprocess with arguments and support for most typical
kinds of input and output redirection). See `Subprocess::new`. The
`SubprocessLauncher` API is provided for more complicated cases
(advanced types of redirection, environment variable manipulation,
change of working directory, child setup functions, etc).

A typical use of `Subprocess` will involve calling
`Subprocess::new`, followed by `Subprocess::wait_async` or
`Subprocess::wait`. After the process exits, the status can be
checked using functions such as `Subprocess::get_if_exited` (which
are similar to the familiar WIFEXITED-style POSIX macros).

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl Subprocess::fn new -->
Create a new process with the given flags and varargs argument
list. By default, matching the `g_spawn_async` defaults, the
child's stdin will be set to the system null device, and
stdout/stderr will be inherited from the parent. You can use
`flags` to control this behavior.

The argument list must be terminated with `None`.
## `flags`
flags that define the behaviour of the subprocess
## `error`
return location for an error, or `None`
## `argv0`
first commandline argument to pass to the subprocess

# Returns

A newly created `Subprocess`, or `None` on error (and `error`
 will be set)
<!-- impl Subprocess::fn newv -->
Create a new process with the given flags and argument list.

The argument list is expected to be `None`-terminated.
## `argv`
commandline arguments for the subprocess
## `flags`
flags that define the behaviour of the subprocess

# Returns

A newly created `Subprocess`, or `None` on error (and `error`
 will be set)
<!-- impl Subprocess::fn communicate -->
Communicate with the subprocess until it terminates, and all input
and output has been completed.

If `stdin_buf` is given, the subprocess must have been created with
`SubprocessFlags::StdinPipe`. The given data is fed to the
stdin of the subprocess and the pipe is closed (ie: EOF).

At the same time (as not to cause blocking when dealing with large
amounts of data), if `SubprocessFlags::StdoutPipe` or
`SubprocessFlags::StderrPipe` were used, reads from those
streams. The data that was read is returned in `stdout` and/or
the `stderr`.

If the subprocess was created with `SubprocessFlags::StdoutPipe`,
`stdout_buf` will contain the data read from stdout. Otherwise, for
subprocesses not created with `SubprocessFlags::StdoutPipe`,
`stdout_buf` will be set to `None`. Similar provisions apply to
`stderr_buf` and `SubprocessFlags::StderrPipe`.

As usual, any output variable may be given as `None` to ignore it.

If you desire the stdout and stderr data to be interleaved, create
the subprocess with `SubprocessFlags::StdoutPipe` and
`SubprocessFlags::StderrMerge`. The merged result will be returned
in `stdout_buf` and `stderr_buf` will be set to `None`.

In case of any error (including cancellation), `false` will be
returned with `error` set. Some or all of the stdin data may have
been written. Any stdout or stderr data that has been read will be
discarded. None of the out variables (aside from `error`) will have
been set to anything in particular and should not be inspected.

In the case that `true` is returned, the subprocess has exited and the
exit status inspection APIs (eg: `Subprocess::get_if_exited`,
`Subprocess::get_exit_status`) may be used.

You should not attempt to use any of the subprocess pipes after
starting this function, since they may be left in strange states,
even if the operation was cancelled. You should especially not
attempt to interact with the pipes while the operation is in progress
(either from another thread or if using the asynchronous version).
## `stdin_buf`
data to send to the stdin of the subprocess, or `None`
## `cancellable`
a `Cancellable`
## `stdout_buf`
data read from the subprocess stdout
## `stderr_buf`
data read from the subprocess stderr

# Returns

`true` if successful
<!-- impl Subprocess::fn communicate_async -->
Asynchronous version of `Subprocess::communicate`. Complete
invocation with `Subprocess::communicate_finish`.
## `stdin_buf`
Input data, or `None`
## `cancellable`
Cancellable
## `callback`
Callback
## `user_data`
User data
<!-- impl Subprocess::fn communicate_finish -->
Complete an invocation of `Subprocess::communicate_async`.
## `result`
Result
## `stdout_buf`
Return location for stdout data
## `stderr_buf`
Return location for stderr data
<!-- impl Subprocess::fn communicate_utf8 -->
Like `Subprocess::communicate`, but validates the output of the
process as UTF-8, and returns it as a regular NUL terminated string.
## `stdin_buf`
data to send to the stdin of the subprocess, or `None`
## `cancellable`
a `Cancellable`
## `stdout_buf`
data read from the subprocess stdout
## `stderr_buf`
data read from the subprocess stderr
<!-- impl Subprocess::fn communicate_utf8_async -->
Asynchronous version of `Subprocess::communicate_utf8`. Complete
invocation with `Subprocess::communicate_utf8_finish`.
## `stdin_buf`
Input data, or `None`
## `cancellable`
Cancellable
## `callback`
Callback
## `user_data`
User data
<!-- impl Subprocess::fn communicate_utf8_finish -->
Complete an invocation of `Subprocess::communicate_utf8_async`.
## `result`
Result
## `stdout_buf`
Return location for stdout data
## `stderr_buf`
Return location for stderr data
<!-- impl Subprocess::fn force_exit -->
Use an operating-system specific method to attempt an immediate,
forceful termination of the process. There is no mechanism to
determine whether or not the request itself was successful;
however, you can use `Subprocess::wait` to monitor the status of
the process after calling this function.

On Unix, this function sends `SIGKILL`.
<!-- impl Subprocess::fn get_exit_status -->
Check the exit status of the subprocess, given that it exited
normally. This is the value passed to the `exit` system call or the
return value from main.

This is equivalent to the system WEXITSTATUS macro.

It is an error to call this function before `Subprocess::wait` and
unless `Subprocess::get_if_exited` returned `true`.

# Returns

the exit status
<!-- impl Subprocess::fn get_identifier -->
On UNIX, returns the process ID as a decimal string.
On Windows, returns the result of GetProcessId() also as a string.
<!-- impl Subprocess::fn get_if_exited -->
Check if the given subprocess exited normally (ie: by way of `exit`
or return from `main`).

This is equivalent to the system WIFEXITED macro.

It is an error to call this function before `Subprocess::wait` has
returned.

# Returns

`true` if the case of a normal exit
<!-- impl Subprocess::fn get_if_signaled -->
Check if the given subprocess terminated in response to a signal.

This is equivalent to the system WIFSIGNALED macro.

It is an error to call this function before `Subprocess::wait` has
returned.

# Returns

`true` if the case of termination due to a signal
<!-- impl Subprocess::fn get_status -->
Gets the raw status code of the process, as from `waitpid`.

This value has no particular meaning, but it can be used with the
macros defined by the system headers such as WIFEXITED. It can also
be used with `g_spawn_check_exit_status`.

It is more likely that you want to use `Subprocess::get_if_exited`
followed by `Subprocess::get_exit_status`.

It is an error to call this function before `Subprocess::wait` has
returned.

# Returns

the (meaningless) `waitpid` exit status from the kernel
<!-- impl Subprocess::fn get_stderr_pipe -->
Gets the `InputStream` from which to read the stderr output of
`self`.

The process must have been created with
`SubprocessFlags::StderrPipe`.

# Returns

the stderr pipe
<!-- impl Subprocess::fn get_stdin_pipe -->
Gets the `OutputStream` that you can write to in order to give data
to the stdin of `self`.

The process must have been created with
`SubprocessFlags::StdinPipe`.

# Returns

the stdout pipe
<!-- impl Subprocess::fn get_stdout_pipe -->
Gets the `InputStream` from which to read the stdout output of
`self`.

The process must have been created with
`SubprocessFlags::StdoutPipe`.

# Returns

the stdout pipe
<!-- impl Subprocess::fn get_successful -->
Checks if the process was "successful". A process is considered
successful if it exited cleanly with an exit status of 0, either by
way of the `exit` system call or return from `main`.

It is an error to call this function before `Subprocess::wait` has
returned.

# Returns

`true` if the process exited cleanly with a exit status of 0
<!-- impl Subprocess::fn get_term_sig -->
Get the signal number that caused the subprocess to terminate, given
that it terminated due to a signal.

This is equivalent to the system WTERMSIG macro.

It is an error to call this function before `Subprocess::wait` and
unless `Subprocess::get_if_signaled` returned `true`.

# Returns

the signal causing termination
<!-- impl Subprocess::fn send_signal -->
Sends the UNIX signal `signal_num` to the subprocess, if it is still
running.

This API is race-free. If the subprocess has terminated, it will not
be signalled.

This API is not available on Windows.
## `signal_num`
the signal number to send
<!-- impl Subprocess::fn wait -->
Synchronously wait for the subprocess to terminate.

After the process terminates you can query its exit status with
functions such as `Subprocess::get_if_exited` and
`Subprocess::get_exit_status`.

This function does not fail in the case of the subprocess having
abnormal termination. See `Subprocess::wait_check` for that.

Cancelling `cancellable` doesn't kill the subprocess. Call
`Subprocess::force_exit` if it is desirable.
## `cancellable`
a `Cancellable`

# Returns

`true` on success, `false` if `cancellable` was cancelled
<!-- impl Subprocess::fn wait_async -->
Wait for the subprocess to terminate.

This is the asynchronous version of `Subprocess::wait`.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback` to call when the operation is complete
## `user_data`
user_data for `callback`
<!-- impl Subprocess::fn wait_check -->
Combines `Subprocess::wait` with `g_spawn_check_exit_status`.
## `cancellable`
a `Cancellable`

# Returns

`true` on success, `false` if process exited abnormally, or
`cancellable` was cancelled
<!-- impl Subprocess::fn wait_check_async -->
Combines `Subprocess::wait_async` with `g_spawn_check_exit_status`.

This is the asynchronous version of `Subprocess::wait_check`.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
a `GAsyncReadyCallback` to call when the operation is complete
## `user_data`
user_data for `callback`
<!-- impl Subprocess::fn wait_check_finish -->
Collects the result of a previous call to
`Subprocess::wait_check_async`.
## `result`
the `AsyncResult` passed to your `GAsyncReadyCallback`

# Returns

`true` if successful, or `false` with `error` set
<!-- impl Subprocess::fn wait_finish -->
Collects the result of a previous call to
`Subprocess::wait_async`.
## `result`
the `AsyncResult` passed to your `GAsyncReadyCallback`

# Returns

`true` if successful, or `false` with `error` set
<!-- struct SubprocessLauncher -->
This class contains a set of options for launching child processes,
such as where its standard input and output will be directed, the
argument list, the environment, and more.

While the `Subprocess` class has high level functions covering
popular cases, use of this class allows access to more advanced
options. It can also be used to launch multiple subprocesses with
a similar configuration.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- impl SubprocessLauncher::fn new -->
Creates a new `SubprocessLauncher`.

The launcher is created with the default options. A copy of the
environment of the calling process is made at the time of this call
and will be used as the environment that the process is launched in.
## `flags`
`SubprocessFlags`
<!-- impl SubprocessLauncher::fn getenv -->
Returns the value of the environment variable `variable` in the
environment of processes launched from this launcher.

On UNIX, the returned string can be an arbitrary byte string.
On Windows, it will be UTF-8.
## `variable`
the environment variable to get

# Returns

the value of the environment variable,
 `None` if unset
<!-- impl SubprocessLauncher::fn set_child_setup -->
Sets up a child setup function.

The child setup function will be called after `fork` but before
`exec` on the child's side.

`destroy_notify` will not be automatically called on the child's side
of the `fork`. It will only be called when the last reference on the
`SubprocessLauncher` is dropped or when a new child setup function is
given.

`None` can be given as `child_setup` to disable the functionality.

Child setup functions are only available on UNIX.
## `child_setup`
a `GSpawnChildSetupFunc` to use as the child setup function
## `user_data`
user data for `child_setup`
## `destroy_notify`
a `GDestroyNotify` for `user_data`
<!-- impl SubprocessLauncher::fn set_cwd -->
Sets the current working directory that processes will be launched
with.

By default processes are launched with the current working directory
of the launching process at the time of launch.
## `cwd`
the cwd for launched processes
<!-- impl SubprocessLauncher::fn set_environ -->
Replace the entire environment of processes launched from this
launcher with the given 'environ' variable.

Typically you will build this variable by using `g_listenv` to copy
the process 'environ' and using the functions `g_environ_setenv`,
`g_environ_unsetenv`, etc.

As an alternative, you can use `SubprocessLauncher::setenv`,
`SubprocessLauncher::unsetenv`, etc.

Pass an empty array to set an empty environment. Pass `None` to inherit the
parent process’ environment. As of GLib 2.54, the parent process’ environment
will be copied when `SubprocessLauncher::set_environ` is called.
Previously, it was copied when the subprocess was executed. This means the
copied environment may now be modified (using `SubprocessLauncher::setenv`,
etc.) before launching the subprocess.

On UNIX, all strings in this array can be arbitrary byte strings.
On Windows, they should be in UTF-8.
## `env`

 the replacement environment
<!-- impl SubprocessLauncher::fn set_flags -->
Sets the flags on the launcher.

The default flags are `SubprocessFlags::None`.

You may not set flags that specify conflicting options for how to
handle a particular stdio stream (eg: specifying both
`SubprocessFlags::StdinPipe` and
`SubprocessFlags::StdinInherit`).

You may also not set a flag that conflicts with a previous call to a
function like `SubprocessLauncher::set_stdin_file_path` or
`SubprocessLauncher::take_stdout_fd`.
## `flags`
`SubprocessFlags`
<!-- impl SubprocessLauncher::fn set_stderr_file_path -->
Sets the file path to use as the stderr for spawned processes.

If `path` is `None` then any previously given path is unset.

The file will be created or truncated when the process is spawned, as
would be the case if using '2>' at the shell.

If you want to send both stdout and stderr to the same file then use
`SubprocessFlags::StderrMerge`.

You may not set a stderr file path if a stderr fd is already set or
if the launcher flags contain any flags directing stderr elsewhere.

This feature is only available on UNIX.
## `path`
a filename or `None`
<!-- impl SubprocessLauncher::fn set_stdin_file_path -->
Sets the file path to use as the stdin for spawned processes.

If `path` is `None` then any previously given path is unset.

The file must exist or spawning the process will fail.

You may not set a stdin file path if a stdin fd is already set or if
the launcher flags contain any flags directing stdin elsewhere.

This feature is only available on UNIX.
<!-- impl SubprocessLauncher::fn set_stdout_file_path -->
Sets the file path to use as the stdout for spawned processes.

If `path` is `None` then any previously given path is unset.

The file will be created or truncated when the process is spawned, as
would be the case if using '>' at the shell.

You may not set a stdout file path if a stdout fd is already set or
if the launcher flags contain any flags directing stdout elsewhere.

This feature is only available on UNIX.
## `path`
a filename or `None`
<!-- impl SubprocessLauncher::fn setenv -->
Sets the environment variable `variable` in the environment of
processes launched from this launcher.

On UNIX, both the variable's name and value can be arbitrary byte
strings, except that the variable's name cannot contain '='.
On Windows, they should be in UTF-8.
## `variable`
the environment variable to set,
 must not contain '='
## `value`
the new value for the variable
## `overwrite`
whether to change the variable if it already exists
<!-- impl SubprocessLauncher::fn spawn -->
Creates a `Subprocess` given a provided varargs list of arguments.
## `error`
Error
## `argv0`
Command line arguments

# Returns

A new `Subprocess`, or `None` on error (and `error` will be set)
<!-- impl SubprocessLauncher::fn spawnv -->
Creates a `Subprocess` given a provided array of arguments.
## `argv`
Command line arguments

# Returns

A new `Subprocess`, or `None` on error (and `error` will be set)
<!-- impl SubprocessLauncher::fn take_fd -->
Transfer an arbitrary file descriptor from parent process to the
child. This function takes "ownership" of the fd; it will be closed
in the parent when `self` is freed.

By default, all file descriptors from the parent will be closed.
This function allows you to create (for example) a custom `pipe` or
`socketpair` before launching the process, and choose the target
descriptor in the child.

An example use case is GNUPG, which has a command line argument
--passphrase-fd providing a file descriptor number where it expects
the passphrase to be written.
## `source_fd`
File descriptor in parent process
## `target_fd`
Target descriptor for child process
<!-- impl SubprocessLauncher::fn take_stderr_fd -->
Sets the file descriptor to use as the stderr for spawned processes.

If `fd` is -1 then any previously given fd is unset.

Note that the default behaviour is to pass stderr through to the
stderr of the parent process.

The passed `fd` belongs to the `SubprocessLauncher`. It will be
automatically closed when the launcher is finalized. The file
descriptor will also be closed on the child side when executing the
spawned process.

You may not set a stderr fd if a stderr file path is already set or
if the launcher flags contain any flags directing stderr elsewhere.

This feature is only available on UNIX.
## `fd`
a file descriptor, or -1
<!-- impl SubprocessLauncher::fn take_stdin_fd -->
Sets the file descriptor to use as the stdin for spawned processes.

If `fd` is -1 then any previously given fd is unset.

Note that if your intention is to have the stdin of the calling
process inherited by the child then `SubprocessFlags::StdinInherit`
is a better way to go about doing that.

The passed `fd` is noted but will not be touched in the current
process. It is therefore necessary that it be kept open by the
caller until the subprocess is spawned. The file descriptor will
also not be explicitly closed on the child side, so it must be marked
O_CLOEXEC if that's what you want.

You may not set a stdin fd if a stdin file path is already set or if
the launcher flags contain any flags directing stdin elsewhere.

This feature is only available on UNIX.
## `fd`
a file descriptor, or -1
<!-- impl SubprocessLauncher::fn take_stdout_fd -->
Sets the file descriptor to use as the stdout for spawned processes.

If `fd` is -1 then any previously given fd is unset.

Note that the default behaviour is to pass stdout through to the
stdout of the parent process.

The passed `fd` is noted but will not be touched in the current
process. It is therefore necessary that it be kept open by the
caller until the subprocess is spawned. The file descriptor will
also not be explicitly closed on the child side, so it must be marked
O_CLOEXEC if that's what you want.

You may not set a stdout fd if a stdout file path is already set or
if the launcher flags contain any flags directing stdout elsewhere.

This feature is only available on UNIX.
## `fd`
a file descriptor, or -1
<!-- impl SubprocessLauncher::fn unsetenv -->
Removes the environment variable `variable` from the environment of
processes launched from this launcher.

On UNIX, the variable's name can be an arbitrary byte string not
containing '='. On Windows, it should be in UTF-8.
## `variable`
the environment variable to unset,
 must not contain '='
<!-- struct TcpConnection -->
This is the subclass of `SocketConnection` that is created
for TCP/IP sockets.

# Implements

[`TcpConnectionExt`](trait.TcpConnectionExt.html), [`SocketConnectionExt`](trait.SocketConnectionExt.html), [`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TcpConnectionExt -->
Trait containing all `TcpConnection` methods.

# Implementors

[`TcpConnection`](struct.TcpConnection.html)
<!-- trait TcpConnectionExt::fn get_graceful_disconnect -->
Checks if graceful disconnects are used. See
`TcpConnectionExt::set_graceful_disconnect`.

# Returns

`true` if graceful disconnect is used on close, `false` otherwise
<!-- trait TcpConnectionExt::fn set_graceful_disconnect -->
This enables graceful disconnects on close. A graceful disconnect
means that we signal the receiving end that the connection is terminated
and wait for it to close the connection before closing the connection.

A graceful disconnect means that we can be sure that we successfully sent
all the outstanding data to the other end, or get an error reported.
However, it also means we have to wait for all the data to reach the
other side and for it to acknowledge this by closing the socket, which may
take a while. For this reason it is disabled by default.
## `graceful_disconnect`
Whether to do graceful disconnects or not
<!-- struct ThemedIcon -->
`ThemedIcon` is an implementation of `Icon` that supports icon themes.
`ThemedIcon` contains a list of all of the icons present in an icon
theme, so that icons can be looked up quickly. `ThemedIcon` does
not provide actual pixmaps for icons, just the icon names.
Ideally something like `gtk_icon_theme_choose_icon` should be used to
resolve the list of names so that fallback icons work nicely with
themes that inherit other themes.

# Implements

[`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`IconExt`](trait.IconExt.html)
<!-- impl ThemedIcon::fn new -->
Creates a new themed icon for `iconname`.
## `iconname`
a string containing an icon name.

# Returns

a new `ThemedIcon`.
<!-- impl ThemedIcon::fn new_from_names -->
Creates a new themed icon for `iconnames`.
## `iconnames`
an array of strings containing icon names.
## `len`
the length of the `iconnames` array, or -1 if `iconnames` is
 `None`-terminated

# Returns

a new `ThemedIcon`
<!-- impl ThemedIcon::fn new_with_default_fallbacks -->
Creates a new themed icon for `iconname`, and all the names
that can be created by shortening `iconname` at '-' characters.

In the following example, `icon1` and `icon2` are equivalent:

```C
const char *names[] = {
  "gnome-dev-cdrom-audio",
  "gnome-dev-cdrom",
  "gnome-dev",
  "gnome"
};

icon1 = g_themed_icon_new_from_names (names, 4);
icon2 = g_themed_icon_new_with_default_fallbacks ("gnome-dev-cdrom-audio");
```
## `iconname`
a string containing an icon name

# Returns

a new `ThemedIcon`.
<!-- impl ThemedIcon::fn append_name -->
Append a name to the list of icons from within `self`.

Note that doing so invalidates the hash computed by prior calls
to `Icon::hash`.
## `iconname`
name of icon to append to list of icons from within `self`.
<!-- impl ThemedIcon::fn get_names -->
Gets the names of icons from within `self`.

# Returns

a list of icon names.
<!-- impl ThemedIcon::fn prepend_name -->
Prepend a name to the list of icons from within `self`.

Note that doing so invalidates the hash computed by prior calls
to `Icon::hash`.
## `iconname`
name of icon to prepend to list of icons from within `self`.
<!-- impl ThemedIcon::fn set_property_name -->
The icon name.
<!-- impl ThemedIcon::fn get_property_names -->
A `None`-terminated array of icon names.
<!-- impl ThemedIcon::fn set_property_names -->
A `None`-terminated array of icon names.
<!-- impl ThemedIcon::fn get_property_use_default_fallbacks -->
Whether to use the default fallbacks found by shortening the icon name
at '-' characters. If the "names" array has more than one element,
ignores any past the first.

For example, if the icon name was "gnome-dev-cdrom-audio", the array
would become

```C
{
  "gnome-dev-cdrom-audio",
  "gnome-dev-cdrom",
  "gnome-dev",
  "gnome",
  NULL
};
```
<!-- impl ThemedIcon::fn set_property_use_default_fallbacks -->
Whether to use the default fallbacks found by shortening the icon name
at '-' characters. If the "names" array has more than one element,
ignores any past the first.

For example, if the icon name was "gnome-dev-cdrom-audio", the array
would become

```C
{
  "gnome-dev-cdrom-audio",
  "gnome-dev-cdrom",
  "gnome-dev",
  "gnome",
  NULL
};
```
<!-- struct ThreadedSocketService -->
A `ThreadedSocketService` is a simple subclass of `SocketService`
that handles incoming connections by creating a worker thread and
dispatching the connection to it by emitting the
`ThreadedSocketService::run` signal in the new thread.

The signal handler may perform blocking IO and need not return
until the connection is closed.

The service is implemented using a thread pool, so there is a
limited amount of threads available to serve incoming requests.
The service automatically stops the `SocketService` from accepting
new connections when all threads are busy.

As with `SocketService`, you may connect to `ThreadedSocketService::run`,
or subclass and override the default handler.

# Implements

[`ThreadedSocketServiceExt`](trait.ThreadedSocketServiceExt.html), [`SocketServiceExt`](trait.SocketServiceExt.html), [`SocketListenerExt`](trait.SocketListenerExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketListenerExtManual`](prelude/trait.SocketListenerExtManual.html)
<!-- trait ThreadedSocketServiceExt -->
Trait containing all `ThreadedSocketService` methods.

# Implementors

[`ThreadedSocketService`](struct.ThreadedSocketService.html)
<!-- impl ThreadedSocketService::fn new -->
Creates a new `ThreadedSocketService` with no listeners. Listeners
must be added with one of the `SocketListener` "add" methods.
## `max_threads`
the maximal number of threads to execute concurrently
 handling incoming clients, -1 means no limit

# Returns

a new `SocketService`.
<!-- trait ThreadedSocketServiceExt::fn connect_run -->
The ::run signal is emitted in a worker thread in response to an
incoming connection. This thread is dedicated to handling
`connection` and may perform blocking IO. The signal handler need
not return until the connection is closed.
## `connection`
a new `SocketConnection` object.
## `source_object`
the source_object passed to `SocketListenerExt::add_address`.

# Returns

`true` to stop further signal handlers from being called
<!-- enum TlsAuthenticationMode -->
The client authentication mode for a `TlsServerConnection`.
<!-- enum TlsAuthenticationMode::variant None -->
client authentication not required
<!-- enum TlsAuthenticationMode::variant Requested -->
client authentication is requested
<!-- enum TlsAuthenticationMode::variant Required -->
client authentication is required
<!-- struct TlsCertificate -->
A certificate used for TLS authentication and encryption.
This can represent either a certificate only (eg, the certificate
received by a client from a server), or the combination of
a certificate and a private key (which is needed when acting as a
`TlsServerConnection`).

# Implements

[`TlsCertificateExt`](trait.TlsCertificateExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsCertificateExt -->
Trait containing all `TlsCertificate` methods.

# Implementors

[`TlsCertificate`](struct.TlsCertificate.html)
<!-- impl TlsCertificate::fn new_from_file -->
Creates a `TlsCertificate` from the PEM-encoded data in `file`. The
returned certificate will be the first certificate found in `file`. As
of GLib 2.44, if `file` contains more certificates it will try to load
a certificate chain. All certificates will be verified in the order
found (top-level certificate should be the last one in the file) and
the `TlsCertificate:issuer` property of each certificate will be set
accordingly if the verification succeeds. If any certificate in the
chain cannot be verified, the first certificate in the file will
still be returned.

If `file` cannot be read or parsed, the function will return `None` and
set `error`. Otherwise, this behaves like
`TlsCertificate::new_from_pem`.
## `file`
file containing a PEM-encoded certificate to import

# Returns

the new certificate, or `None` on error
<!-- impl TlsCertificate::fn new_from_files -->
Creates a `TlsCertificate` from the PEM-encoded data in `cert_file`
and `key_file`. The returned certificate will be the first certificate
found in `cert_file`. As of GLib 2.44, if `cert_file` contains more
certificates it will try to load a certificate chain. All
certificates will be verified in the order found (top-level
certificate should be the last one in the file) and the
`TlsCertificate:issuer` property of each certificate will be set
accordingly if the verification succeeds. If any certificate in the
chain cannot be verified, the first certificate in the file will
still be returned.

If either file cannot be read or parsed, the function will return
`None` and set `error`. Otherwise, this behaves like
`TlsCertificate::new_from_pem`.
## `cert_file`
file containing one or more PEM-encoded
 certificates to import
## `key_file`
file containing a PEM-encoded private key
 to import

# Returns

the new certificate, or `None` on error
<!-- impl TlsCertificate::fn new_from_pem -->
Creates a `TlsCertificate` from the PEM-encoded data in `data`. If
`data` includes both a certificate and a private key, then the
returned certificate will include the private key data as well. (See
the `TlsCertificate:private-key-pem` property for information about
supported formats.)

The returned certificate will be the first certificate found in
`data`. As of GLib 2.44, if `data` contains more certificates it will
try to load a certificate chain. All certificates will be verified in
the order found (top-level certificate should be the last one in the
file) and the `TlsCertificate:issuer` property of each certificate
will be set accordingly if the verification succeeds. If any
certificate in the chain cannot be verified, the first certificate in
the file will still be returned.
## `data`
PEM-encoded certificate data
## `length`
the length of `data`, or -1 if it's 0-terminated.

# Returns

the new certificate, or `None` if `data` is invalid
<!-- impl TlsCertificate::fn list_new_from_file -->
Creates one or more `GTlsCertificates` from the PEM-encoded
data in `file`. If `file` cannot be read or parsed, the function will
return `None` and set `error`. If `file` does not contain any
PEM-encoded certificates, this will return an empty list and not
set `error`.
## `file`
file containing PEM-encoded certificates to import

# Returns

a
`glib::List` containing `TlsCertificate` objects. You must free the list
and its contents when you are done with it.
<!-- trait TlsCertificateExt::fn get_issuer -->
Gets the `TlsCertificate` representing `self`'s issuer, if known

# Returns

The certificate of `self`'s issuer,
or `None` if `self` is self-signed or signed with an unknown
certificate.
<!-- trait TlsCertificateExt::fn is_same -->
Check if two `TlsCertificate` objects represent the same certificate.
The raw DER byte data of the two certificates are checked for equality.
This has the effect that two certificates may compare equal even if
their `TlsCertificate:issuer`, `TlsCertificate:private-key`, or
`TlsCertificate:private-key-pem` properties differ.
## `cert_two`
second certificate to compare

# Returns

whether the same or not
<!-- trait TlsCertificateExt::fn verify -->
This verifies `self` and returns a set of `TlsCertificateFlags`
indicating any problems found with it. This can be used to verify a
certificate outside the context of making a connection, or to
check a certificate against a CA that is not part of the system
CA database.

If `identity` is not `None`, `self`'s name(s) will be compared against
it, and `TlsCertificateFlags::BadIdentity` will be set in the return
value if it does not match. If `identity` is `None`, that bit will
never be set in the return value.

If `trusted_ca` is not `None`, then `self` (or one of the certificates
in its chain) must be signed by it, or else
`TlsCertificateFlags::UnknownCa` will be set in the return value. If
`trusted_ca` is `None`, that bit will never be set in the return
value.

(All other `TlsCertificateFlags` values will always be set or unset
as appropriate.)
## `identity`
the expected peer identity
## `trusted_ca`
the certificate of a trusted authority

# Returns

the appropriate `TlsCertificateFlags`
<!-- trait TlsCertificateExt::fn get_property_certificate -->
The DER (binary) encoded representation of the certificate.
This property and the `TlsCertificate:certificate-pem` property
represent the same data, just in different forms.
<!-- trait TlsCertificateExt::fn set_property_certificate -->
The DER (binary) encoded representation of the certificate.
This property and the `TlsCertificate:certificate-pem` property
represent the same data, just in different forms.
<!-- trait TlsCertificateExt::fn get_property_certificate_pem -->
The PEM (ASCII) encoded representation of the certificate.
This property and the `TlsCertificate:certificate`
property represent the same data, just in different forms.
<!-- trait TlsCertificateExt::fn set_property_certificate_pem -->
The PEM (ASCII) encoded representation of the certificate.
This property and the `TlsCertificate:certificate`
property represent the same data, just in different forms.
<!-- trait TlsCertificateExt::fn get_property_issuer -->
A `TlsCertificate` representing the entity that issued this
certificate. If `None`, this means that the certificate is either
self-signed, or else the certificate of the issuer is not
available.
<!-- trait TlsCertificateExt::fn set_property_issuer -->
A `TlsCertificate` representing the entity that issued this
certificate. If `None`, this means that the certificate is either
self-signed, or else the certificate of the issuer is not
available.
<!-- trait TlsCertificateExt::fn set_property_private_key -->
The DER (binary) encoded representation of the certificate's
private key, in either PKCS`1` format or unencrypted PKCS`8`
format. This property (or the `TlsCertificate:private-key-pem`
property) can be set when constructing a key (eg, from a file),
but cannot be read.

PKCS`8` format is supported since 2.32; earlier releases only
support PKCS`1`. You can use the `openssl rsa`
tool to convert PKCS`8` keys to PKCS`1`.
<!-- trait TlsCertificateExt::fn set_property_private_key_pem -->
The PEM (ASCII) encoded representation of the certificate's
private key in either PKCS`1` format ("`BEGIN RSA PRIVATE
KEY`") or unencrypted PKCS`8` format ("`BEGIN
PRIVATE KEY`"). This property (or the
`TlsCertificate:private-key` property) can be set when
constructing a key (eg, from a file), but cannot be read.

PKCS`8` format is supported since 2.32; earlier releases only
support PKCS`1`. You can use the `openssl rsa`
tool to convert PKCS`8` keys to PKCS`1`.
<!-- enum TlsCertificateRequestFlags -->
Flags for `TlsInteractionExt::request_certificate`,
`TlsInteractionExt::request_certificate_async`, and
`TlsInteractionExt::invoke_request_certificate`.
<!-- enum TlsCertificateRequestFlags::variant None -->
No flags
<!-- struct TlsClientConnection -->
`TlsClientConnection` is the client-side subclass of
`TlsConnection`, representing a client-side TLS connection.

# Implements

[`TlsClientConnectionExt`](trait.TlsClientConnectionExt.html), [`TlsConnectionExt`](trait.TlsConnectionExt.html), [`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsClientConnectionExt -->
Trait containing all `TlsClientConnection` methods.

# Implementors

[`TlsClientConnection`](struct.TlsClientConnection.html)
<!-- impl TlsClientConnection::fn new -->
Creates a new `TlsClientConnection` wrapping `base_io_stream` (which
must have pollable input and output streams) which is assumed to
communicate with the server identified by `server_identity`.

See the documentation for `TlsConnection:base-io-stream` for restrictions
on when application code can run operations on the `base_io_stream` after
this function has returned.
## `base_io_stream`
the `IOStream` to wrap
## `server_identity`
the expected identity of the server

# Returns

the new
`TlsClientConnection`, or `None` on error
<!-- trait TlsClientConnectionExt::fn copy_session_state -->
Copies session state from one connection to another. This is
not normally needed, but may be used when the same session
needs to be used between different endpoints as is required
by some protocols such as FTP over TLS. `source` should have
already completed a handshake, and `self` should not have
completed a handshake.

Feature: `v2_46`

## `source`
a `TlsClientConnection`
<!-- trait TlsClientConnectionExt::fn get_accepted_cas -->
Gets the list of distinguished names of the Certificate Authorities
that the server will accept certificates from. This will be set
during the TLS handshake if the server requests a certificate.
Otherwise, it will be `None`.

Each item in the list is a `glib::ByteArray` which contains the complete
subject DN of the certificate authority.

# Returns

the list of
CA DNs. You should unref each element with `glib::ByteArray::unref` and then
the free the list with `glib::List::free`.
<!-- trait TlsClientConnectionExt::fn get_server_identity -->
Gets `self`'s expected server identity

# Returns

a `SocketConnectable` describing the
expected server identity, or `None` if the expected identity is not
known.
<!-- trait TlsClientConnectionExt::fn get_use_ssl3 -->
Gets whether `self` will force the lowest-supported TLS protocol
version rather than attempt to negotiate the highest mutually-
supported version of TLS; see `TlsClientConnection::set_use_ssl3`.

# Deprecated since 2.56

SSL 3.0 is insecure, and this function does not
actually indicate whether it is enabled.

# Returns

whether `self` will use the lowest-supported TLS protocol version
<!-- trait TlsClientConnectionExt::fn get_validation_flags -->
Gets `self`'s validation flags

# Returns

the validation flags
<!-- trait TlsClientConnectionExt::fn set_server_identity -->
Sets `self`'s expected server identity, which is used both to tell
servers on virtual hosts which certificate to present, and also
to let `self` know what name to look for in the certificate when
performing `TlsCertificateFlags::BadIdentity` validation, if enabled.
## `identity`
a `SocketConnectable` describing the expected server identity
<!-- trait TlsClientConnectionExt::fn set_use_ssl3 -->
Since 2.42.1, if `use_ssl3` is `true`, this forces `self` to use the
lowest-supported TLS protocol version rather than trying to properly
negotiate the highest mutually-supported protocol version with the
peer. Be aware that SSL 3.0 is generally disabled by the
`TlsBackend`, so the lowest-supported protocol version is probably
not SSL 3.0.

Since 2.58, this may additionally cause an RFC 7507 fallback SCSV to
be sent to the server, causing modern TLS servers to immediately
terminate the connection. You should generally only use this function
if you need to connect to broken servers that exhibit TLS protocol
version intolerance, and when an initial attempt to connect to a
server normally has already failed.

# Deprecated since 2.56

SSL 3.0 is insecure, and this function does not
generally enable or disable it, despite its name.
## `use_ssl3`
whether to use the lowest-supported protocol version
<!-- trait TlsClientConnectionExt::fn set_validation_flags -->
Sets `self`'s validation flags, to override the default set of
checks performed when validating a server certificate. By default,
`TlsCertificateFlags::ValidateAll` is used.
## `flags`
the `TlsCertificateFlags` to use
<!-- trait TlsClientConnectionExt::fn get_property_accepted_cas -->
A list of the distinguished names of the Certificate Authorities
that the server will accept client certificates signed by. If the
server requests a client certificate during the handshake, then
this property will be set after the handshake completes.

Each item in the list is a `glib::ByteArray` which contains the complete
subject DN of the certificate authority.
<!-- trait TlsClientConnectionExt::fn get_property_server_identity -->
A `SocketConnectable` describing the identity of the server that
is expected on the other end of the connection.

If the `TlsCertificateFlags::BadIdentity` flag is set in
`TlsClientConnection:validation-flags`, this object will be used
to determine the expected identify of the remote end of the
connection; if `TlsClientConnection:server-identity` is not set,
or does not match the identity presented by the server, then the
`TlsCertificateFlags::BadIdentity` validation will fail.

In addition to its use in verifying the server certificate,
this is also used to give a hint to the server about what
certificate we expect, which is useful for servers that serve
virtual hosts.
<!-- trait TlsClientConnectionExt::fn set_property_server_identity -->
A `SocketConnectable` describing the identity of the server that
is expected on the other end of the connection.

If the `TlsCertificateFlags::BadIdentity` flag is set in
`TlsClientConnection:validation-flags`, this object will be used
to determine the expected identify of the remote end of the
connection; if `TlsClientConnection:server-identity` is not set,
or does not match the identity presented by the server, then the
`TlsCertificateFlags::BadIdentity` validation will fail.

In addition to its use in verifying the server certificate,
this is also used to give a hint to the server about what
certificate we expect, which is useful for servers that serve
virtual hosts.
<!-- trait TlsClientConnectionExt::fn get_property_use_ssl3 -->
If `true`, forces the connection to use a fallback version of TLS
or SSL, rather than trying to negotiate the best version of TLS
to use. See `TlsClientConnection::set_use_ssl3`.

# Deprecated since 2.56

SSL 3.0 is insecure, and this property does not
generally enable or disable it, despite its name.
<!-- trait TlsClientConnectionExt::fn set_property_use_ssl3 -->
If `true`, forces the connection to use a fallback version of TLS
or SSL, rather than trying to negotiate the best version of TLS
to use. See `TlsClientConnection::set_use_ssl3`.

# Deprecated since 2.56

SSL 3.0 is insecure, and this property does not
generally enable or disable it, despite its name.
<!-- trait TlsClientConnectionExt::fn get_property_validation_flags -->
What steps to perform when validating a certificate received from
a server. Server certificates that fail to validate in all of the
ways indicated here will be rejected unless the application
overrides the default via `TlsConnection::accept-certificate`.
<!-- trait TlsClientConnectionExt::fn set_property_validation_flags -->
What steps to perform when validating a certificate received from
a server. Server certificates that fail to validate in all of the
ways indicated here will be rejected unless the application
overrides the default via `TlsConnection::accept-certificate`.
<!-- struct TlsConnection -->
`TlsConnection` is the base TLS connection class type, which wraps
a `IOStream` and provides TLS encryption on top of it. Its
subclasses, `TlsClientConnection` and `TlsServerConnection`,
implement client-side and server-side TLS, respectively.

For DTLS (Datagram TLS) support, see `DtlsConnection`.

# Implements

[`TlsConnectionExt`](trait.TlsConnectionExt.html), [`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsConnectionExt -->
Trait containing all `TlsConnection` methods.

# Implementors

[`TlsClientConnection`](struct.TlsClientConnection.html), [`TlsConnection`](struct.TlsConnection.html), [`TlsServerConnection`](struct.TlsServerConnection.html)
<!-- trait TlsConnectionExt::fn emit_accept_certificate -->
Used by `TlsConnection` implementations to emit the
`TlsConnection::accept-certificate` signal.
## `peer_cert`
the peer's `TlsCertificate`
## `errors`
the problems with `peer_cert`

# Returns

`true` if one of the signal handlers has returned
 `true` to accept `peer_cert`
<!-- trait TlsConnectionExt::fn get_certificate -->
Gets `self`'s certificate, as set by
`TlsConnectionExt::set_certificate`.

# Returns

`self`'s certificate, or `None`
<!-- trait TlsConnectionExt::fn get_database -->
Gets the certificate database that `self` uses to verify
peer certificates. See `TlsConnectionExt::set_database`.

# Returns

the certificate database that `self` uses or `None`
<!-- trait TlsConnectionExt::fn get_interaction -->
Get the object that will be used to interact with the user. It will be used
for things like prompting the user for passwords. If `None` is returned, then
no user interaction will occur for this connection.

# Returns

The interaction object.
<!-- trait TlsConnectionExt::fn get_peer_certificate -->
Gets `self`'s peer's certificate after the handshake has completed.
(It is not set during the emission of
`TlsConnection::accept-certificate`.)

# Returns

`self`'s peer's certificate, or `None`
<!-- trait TlsConnectionExt::fn get_peer_certificate_errors -->
Gets the errors associated with validating `self`'s peer's
certificate, after the handshake has completed. (It is not set
during the emission of `TlsConnection::accept-certificate`.)

# Returns

`self`'s peer's certificate errors
<!-- trait TlsConnectionExt::fn get_rehandshake_mode -->
Gets `self` rehandshaking mode. See
`TlsConnectionExt::set_rehandshake_mode` for details.

# Returns

`self`'s rehandshaking mode
<!-- trait TlsConnectionExt::fn get_require_close_notify -->
Tests whether or not `self` expects a proper TLS close notification
when the connection is closed. See
`TlsConnectionExt::set_require_close_notify` for details.

# Returns

`true` if `self` requires a proper TLS close
notification.
<!-- trait TlsConnectionExt::fn handshake -->
Attempts a TLS handshake on `self`.

On the client side, it is never necessary to call this method;
although the connection needs to perform a handshake after
connecting (or after sending a "STARTTLS"-type command) and may
need to rehandshake later if the server requests it,
`TlsConnection` will handle this for you automatically when you try
to send or receive data on the connection. However, you can call
`TlsConnectionExt::handshake` manually if you want to know for sure
whether the initial handshake succeeded or failed (as opposed to
just immediately trying to write to `self`'s output stream, in which
case if it fails, it may not be possible to tell if it failed
before or after completing the handshake).

Likewise, on the server side, although a handshake is necessary at
the beginning of the communication, you do not need to call this
function explicitly unless you want clearer error reporting.
However, you may call `TlsConnectionExt::handshake` later on to
rehandshake, if TLS 1.2 or older is in use. With TLS 1.3, the
behavior is undefined but guaranteed to be reasonable and
nondestructive, so most older code should be expected to continue to
work without changes.

`TlsConnection::accept_certificate` may be emitted during the
handshake.
## `cancellable`
a `Cancellable`, or `None`

# Returns

success or failure
<!-- trait TlsConnectionExt::fn handshake_async -->
Asynchronously performs a TLS handshake on `self`. See
`TlsConnectionExt::handshake` for more information.
## `io_priority`
the [I/O priority][io-priority] of the request
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call when the handshake is complete
## `user_data`
the data to pass to the callback function
<!-- trait TlsConnectionExt::fn handshake_finish -->
Finish an asynchronous TLS handshake operation. See
`TlsConnectionExt::handshake` for more information.
## `result`
a `AsyncResult`.

# Returns

`true` on success, `false` on failure, in which
case `error` will be set.
<!-- trait TlsConnectionExt::fn set_certificate -->
This sets the certificate that `self` will present to its peer
during the TLS handshake. For a `TlsServerConnection`, it is
mandatory to set this, and that will normally be done at construct
time.

For a `TlsClientConnection`, this is optional. If a handshake fails
with `TlsError::CertificateRequired`, that means that the server
requires a certificate, and if you try connecting again, you should
call this method first. You can call
`TlsClientConnection::get_accepted_cas` on the failed connection
to get a list of Certificate Authorities that the server will
accept certificates from.

(It is also possible that a server will allow the connection with
or without a certificate; in that case, if you don't provide a
certificate, you can tell that the server requested one by the fact
that `TlsClientConnection::get_accepted_cas` will return
non-`None`.)
## `certificate`
the certificate to use for `self`
<!-- trait TlsConnectionExt::fn set_database -->
Sets the certificate database that is used to verify peer certificates.
This is set to the default database by default. See
`TlsBackend::get_default_database`. If set to `None`, then
peer certificate validation will always set the
`TlsCertificateFlags::UnknownCa` error (meaning
`TlsConnection::accept-certificate` will always be emitted on
client-side connections, unless that bit is not set in
`TlsClientConnection:validation-flags`).
## `database`
a `TlsDatabase`
<!-- trait TlsConnectionExt::fn set_interaction -->
Set the object that will be used to interact with the user. It will be used
for things like prompting the user for passwords.

The `interaction` argument will normally be a derived subclass of
`TlsInteraction`. `None` can also be provided if no user interaction
should occur for this connection.
## `interaction`
an interaction object, or `None`
<!-- trait TlsConnectionExt::fn set_rehandshake_mode -->
Sets how `self` behaves with respect to rehandshaking requests, when
TLS 1.2 or older is in use.

`TlsRehandshakeMode::Never` means that it will never agree to
rehandshake after the initial handshake is complete. (For a client,
this means it will refuse rehandshake requests from the server, and
for a server, this means it will close the connection with an error
if the client attempts to rehandshake.)

`TlsRehandshakeMode::Safely` means that the connection will allow a
rehandshake only if the other end of the connection supports the
TLS `renegotiation_info` extension. This is the default behavior,
but means that rehandshaking will not work against older
implementations that do not support that extension.

`TlsRehandshakeMode::Unsafely` means that the connection will allow
rehandshaking even without the `renegotiation_info` extension. On
the server side in particular, this is not recommended, since it
leaves the server open to certain attacks. However, this mode is
necessary if you need to allow renegotiation with older client
software.
## `mode`
the rehandshaking mode
<!-- trait TlsConnectionExt::fn set_require_close_notify -->
Sets whether or not `self` expects a proper TLS close notification
before the connection is closed. If this is `true` (the default),
then `self` will expect to receive a TLS close notification from its
peer before the connection is closed, and will return a
`TlsError::Eof` error if the connection is closed without proper
notification (since this may indicate a network error, or
man-in-the-middle attack).

In some protocols, the application will know whether or not the
connection was closed cleanly based on application-level data
(because the application-level data includes a length field, or is
somehow self-delimiting); in this case, the close notify is
redundant and sometimes omitted. (TLS 1.1 explicitly allows this;
in TLS 1.0 it is technically an error, but often done anyway.) You
can use `TlsConnectionExt::set_require_close_notify` to tell `self`
to allow an "unannounced" connection close, in which case the close
will show up as a 0-length read, as in a non-TLS
`SocketConnection`, and it is up to the application to check that
the data has been fully received.

Note that this only affects the behavior when the peer closes the
connection; when the application calls `IOStreamExt::close` itself
on `self`, this will send a close notification regardless of the
setting of this property. If you explicitly want to do an unclean
close, you can close `self`'s `TlsConnection:base-io-stream` rather
than closing `self` itself, but note that this may only be done when no other
operations are pending on `self` or the base I/O stream.
## `require_close_notify`
whether or not to require close notification
<!-- trait TlsConnectionExt::fn connect_accept_certificate -->
Emitted during the TLS handshake after the peer certificate has
been received. You can examine `peer_cert`'s certification path by
calling `TlsCertificateExt::get_issuer` on it.

For a client-side connection, `peer_cert` is the server's
certificate, and the signal will only be emitted if the
certificate was not acceptable according to `conn`'s
`TlsClientConnection:validation_flags`. If you would like the
certificate to be accepted despite `errors`, return `true` from the
signal handler. Otherwise, if no handler accepts the certificate,
the handshake will fail with `TlsError::BadCertificate`.

For a server-side connection, `peer_cert` is the certificate
presented by the client, if this was requested via the server's
`TlsServerConnection:authentication_mode`. On the server side,
the signal is always emitted when the client presents a
certificate, and the certificate will only be accepted if a
handler returns `true`.

Note that if this signal is emitted as part of asynchronous I/O
in the main thread, then you should not attempt to interact with
the user before returning from the signal handler. If you want to
let the user decide whether or not to accept the certificate, you
would have to return `false` from the signal handler on the first
attempt, and then after the connection attempt returns a
`TlsError::Handshake`, you can interact with the user, and if
the user decides to accept the certificate, remember that fact,
create a new connection, and return `true` from the signal handler
the next time.

If you are doing I/O in another thread, you do not
need to worry about this, and can simply block in the signal
handler until the UI thread returns an answer.
## `peer_cert`
the peer's `TlsCertificate`
## `errors`
the problems with `peer_cert`.

# Returns

`true` to accept `peer_cert` (which will also
immediately end the signal emission). `false` to allow the signal
emission to continue, which will cause the handshake to fail if
no one else overrides it.
<!-- trait TlsConnectionExt::fn get_property_base_io_stream -->
The `IOStream` that the connection wraps. The connection holds a reference
to this stream, and may run operations on the stream from other threads
throughout its lifetime. Consequently, after the `IOStream` has been
constructed, application code may only run its own operations on this
stream when no `IOStream` operations are running.
<!-- trait TlsConnectionExt::fn set_property_base_io_stream -->
The `IOStream` that the connection wraps. The connection holds a reference
to this stream, and may run operations on the stream from other threads
throughout its lifetime. Consequently, after the `IOStream` has been
constructed, application code may only run its own operations on this
stream when no `IOStream` operations are running.
<!-- trait TlsConnectionExt::fn get_property_certificate -->
The connection's certificate; see
`TlsConnectionExt::set_certificate`.
<!-- trait TlsConnectionExt::fn set_property_certificate -->
The connection's certificate; see
`TlsConnectionExt::set_certificate`.
<!-- trait TlsConnectionExt::fn get_property_database -->
The certificate database to use when verifying this TLS connection.
If no certificate database is set, then the default database will be
used. See `TlsBackend::get_default_database`.
<!-- trait TlsConnectionExt::fn set_property_database -->
The certificate database to use when verifying this TLS connection.
If no certificate database is set, then the default database will be
used. See `TlsBackend::get_default_database`.
<!-- trait TlsConnectionExt::fn get_property_interaction -->
A `TlsInteraction` object to be used when the connection or certificate
database need to interact with the user. This will be used to prompt the
user for passwords where necessary.
<!-- trait TlsConnectionExt::fn set_property_interaction -->
A `TlsInteraction` object to be used when the connection or certificate
database need to interact with the user. This will be used to prompt the
user for passwords where necessary.
<!-- trait TlsConnectionExt::fn get_property_peer_certificate -->
The connection's peer's certificate, after the TLS handshake has
completed and the certificate has been accepted. Note in
particular that this is not yet set during the emission of
`TlsConnection::accept-certificate`.

(You can watch for a `gobject::Object::notify` signal on this property to
detect when a handshake has occurred.)
<!-- trait TlsConnectionExt::fn get_property_peer_certificate_errors -->
The errors noticed-and-ignored while verifying
`TlsConnection:peer-certificate`. Normally this should be 0, but
it may not be if `TlsClientConnection:validation-flags` is not
`TlsCertificateFlags::ValidateAll`, or if
`TlsConnection::accept-certificate` overrode the default
behavior.
<!-- trait TlsConnectionExt::fn get_property_rehandshake_mode -->
The rehandshaking mode. See
`TlsConnectionExt::set_rehandshake_mode`.
<!-- trait TlsConnectionExt::fn set_property_rehandshake_mode -->
The rehandshaking mode. See
`TlsConnectionExt::set_rehandshake_mode`.
<!-- trait TlsConnectionExt::fn get_property_require_close_notify -->
Whether or not proper TLS close notification is required.
See `TlsConnectionExt::set_require_close_notify`.
<!-- trait TlsConnectionExt::fn set_property_require_close_notify -->
Whether or not proper TLS close notification is required.
See `TlsConnectionExt::set_require_close_notify`.
<!-- struct TlsDatabase -->
`TlsDatabase` is used to lookup certificates and other information
from a certificate or key store. It is an abstract base class which
TLS library specific subtypes override.

Most common client applications will not directly interact with
`TlsDatabase`. It is used internally by `TlsConnection`.

# Implements

[`TlsDatabaseExt`](trait.TlsDatabaseExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsDatabaseExt -->
Trait containing all `TlsDatabase` methods.

# Implementors

[`TlsDatabase`](struct.TlsDatabase.html), [`TlsFileDatabase`](struct.TlsFileDatabase.html)
<!-- trait TlsDatabaseExt::fn create_certificate_handle -->
Create a handle string for the certificate. The database will only be able
to create a handle for certificates that originate from the database. In
cases where the database cannot create a handle for a certificate, `None`
will be returned.

This handle should be stable across various instances of the application,
and between applications. If a certificate is modified in the database,
then it is not guaranteed that this handle will continue to point to it.
## `certificate`
certificate for which to create a handle.

# Returns

a newly allocated string containing the
handle.
<!-- trait TlsDatabaseExt::fn lookup_certificate_for_handle -->
Lookup a certificate by its handle.

The handle should have been created by calling
`TlsDatabaseExt::create_certificate_handle` on a `TlsDatabase` object of
the same TLS backend. The handle is designed to remain valid across
instantiations of the database.

If the handle is no longer valid, or does not point to a certificate in
this database, then `None` will be returned.

This function can block, use `TlsDatabaseExt::lookup_certificate_for_handle_async` to perform
the lookup operation asynchronously.
## `handle`
a certificate handle
## `interaction`
used to interact with the user if necessary
## `flags`
Flags which affect the lookup.
## `cancellable`
a `Cancellable`, or `None`

# Returns

a newly allocated
`TlsCertificate`, or `None`. Use `gobject::ObjectExt::unref` to release the certificate.
<!-- trait TlsDatabaseExt::fn lookup_certificate_for_handle_async -->
Asynchronously lookup a certificate by its handle in the database. See
`TlsDatabaseExt::lookup_certificate_for_handle` for more information.
## `handle`
a certificate handle
## `interaction`
used to interact with the user if necessary
## `flags`
Flags which affect the lookup.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call when the operation completes
## `user_data`
the data to pass to the callback function
<!-- trait TlsDatabaseExt::fn lookup_certificate_for_handle_finish -->
Finish an asynchronous lookup of a certificate by its handle. See
`g_tls_database_lookup_certificate_by_handle` for more information.

If the handle is no longer valid, or does not point to a certificate in
this database, then `None` will be returned.
## `result`
a `AsyncResult`.

# Returns

a newly allocated `TlsCertificate` object.
Use `gobject::ObjectExt::unref` to release the certificate.
<!-- trait TlsDatabaseExt::fn lookup_certificate_issuer -->
Lookup the issuer of `certificate` in the database.

The `issuer` property
of `certificate` is not modified, and the two certificates are not hooked
into a chain.

This function can block, use `TlsDatabaseExt::lookup_certificate_issuer_async` to perform
the lookup operation asynchronously.
## `certificate`
a `TlsCertificate`
## `interaction`
used to interact with the user if necessary
## `flags`
flags which affect the lookup operation
## `cancellable`
a `Cancellable`, or `None`

# Returns

a newly allocated issuer `TlsCertificate`,
or `None`. Use `gobject::ObjectExt::unref` to release the certificate.
<!-- trait TlsDatabaseExt::fn lookup_certificate_issuer_async -->
Asynchronously lookup the issuer of `certificate` in the database. See
`TlsDatabaseExt::lookup_certificate_issuer` for more information.
## `certificate`
a `TlsCertificate`
## `interaction`
used to interact with the user if necessary
## `flags`
flags which affect the lookup operation
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call when the operation completes
## `user_data`
the data to pass to the callback function
<!-- trait TlsDatabaseExt::fn lookup_certificate_issuer_finish -->
Finish an asynchronous lookup issuer operation. See
`TlsDatabaseExt::lookup_certificate_issuer` for more information.
## `result`
a `AsyncResult`.

# Returns

a newly allocated issuer `TlsCertificate`,
or `None`. Use `gobject::ObjectExt::unref` to release the certificate.
<!-- trait TlsDatabaseExt::fn lookup_certificates_issued_by -->
Lookup certificates issued by this issuer in the database.

This function can block, use `TlsDatabaseExt::lookup_certificates_issued_by_async` to perform
the lookup operation asynchronously.
## `issuer_raw_dn`
a `glib::ByteArray` which holds the DER encoded issuer DN.
## `interaction`
used to interact with the user if necessary
## `flags`
Flags which affect the lookup operation.
## `cancellable`
a `Cancellable`, or `None`

# Returns

a newly allocated list of `TlsCertificate`
objects. Use `gobject::ObjectExt::unref` on each certificate, and `glib::List::free` on the release the list.
<!-- trait TlsDatabaseExt::fn lookup_certificates_issued_by_async -->
Asynchronously lookup certificates issued by this issuer in the database. See
`TlsDatabaseExt::lookup_certificates_issued_by` for more information.

The database may choose to hold a reference to the issuer byte array for the duration
of of this asynchronous operation. The byte array should not be modified during
this time.
## `issuer_raw_dn`
a `glib::ByteArray` which holds the DER encoded issuer DN.
## `interaction`
used to interact with the user if necessary
## `flags`
Flags which affect the lookup operation.
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call when the operation completes
## `user_data`
the data to pass to the callback function
<!-- trait TlsDatabaseExt::fn lookup_certificates_issued_by_finish -->
Finish an asynchronous lookup of certificates. See
`TlsDatabaseExt::lookup_certificates_issued_by` for more information.
## `result`
a `AsyncResult`.

# Returns

a newly allocated list of `TlsCertificate`
objects. Use `gobject::ObjectExt::unref` on each certificate, and `glib::List::free` on the release the list.
<!-- trait TlsDatabaseExt::fn verify_chain -->
Determines the validity of a certificate chain after looking up and
adding any missing certificates to the chain.

`chain` is a chain of `TlsCertificate` objects each pointing to the next
certificate in the chain by its `TlsCertificate:issuer` property. The chain may initially
consist of one or more certificates. After the verification process is
complete, `chain` may be modified by adding missing certificates, or removing
extra certificates. If a certificate anchor was found, then it is added to
the `chain`.

`purpose` describes the purpose (or usage) for which the certificate
is being used. Typically `purpose` will be set to `G_TLS_DATABASE_PURPOSE_AUTHENTICATE_SERVER`
which means that the certificate is being used to authenticate a server
(and we are acting as the client).

The `identity` is used to check for pinned certificates (trust exceptions)
in the database. These will override the normal verification process on a
host by host basis.

Currently there are no `flags`, and `TlsDatabaseVerifyFlags::None` should be
used.

If `chain` is found to be valid, then the return value will be 0. If
`chain` is found to be invalid, then the return value will indicate
the problems found. If the function is unable to determine whether
`chain` is valid or not (eg, because `cancellable` is triggered
before it completes) then the return value will be
`TlsCertificateFlags::GenericError` and `error` will be set
accordingly. `error` is not set when `chain` is successfully analyzed
but found to be invalid.

This function can block, use `TlsDatabaseExt::verify_chain_async` to perform
the verification operation asynchronously.
## `chain`
a `TlsCertificate` chain
## `purpose`
the purpose that this certificate chain will be used for.
## `identity`
the expected peer identity
## `interaction`
used to interact with the user if necessary
## `flags`
additional verify flags
## `cancellable`
a `Cancellable`, or `None`

# Returns

the appropriate `TlsCertificateFlags` which represents the
result of verification.
<!-- trait TlsDatabaseExt::fn verify_chain_async -->
Asynchronously determines the validity of a certificate chain after
looking up and adding any missing certificates to the chain. See
`TlsDatabaseExt::verify_chain` for more information.
## `chain`
a `TlsCertificate` chain
## `purpose`
the purpose that this certificate chain will be used for.
## `identity`
the expected peer identity
## `interaction`
used to interact with the user if necessary
## `flags`
additional verify flags
## `cancellable`
a `Cancellable`, or `None`
## `callback`
callback to call when the operation completes
## `user_data`
the data to pass to the callback function
<!-- trait TlsDatabaseExt::fn verify_chain_finish -->
Finish an asynchronous verify chain operation. See
`TlsDatabaseExt::verify_chain` for more information.

If `chain` is found to be valid, then the return value will be 0. If
`chain` is found to be invalid, then the return value will indicate
the problems found. If the function is unable to determine whether
`chain` is valid or not (eg, because `cancellable` is triggered
before it completes) then the return value will be
`TlsCertificateFlags::GenericError` and `error` will be set
accordingly. `error` is not set when `chain` is successfully analyzed
but found to be invalid.
## `result`
a `AsyncResult`.

# Returns

the appropriate `TlsCertificateFlags` which represents the
result of verification.
<!-- enum TlsDatabaseLookupFlags -->
Flags for `TlsDatabaseExt::lookup_certificate_for_handle`,
`TlsDatabaseExt::lookup_certificate_issuer`,
and `TlsDatabaseExt::lookup_certificates_issued_by`.
<!-- enum TlsDatabaseLookupFlags::variant None -->
No lookup flags
<!-- enum TlsDatabaseLookupFlags::variant Keypair -->
Restrict lookup to certificates that have
 a private key.
<!-- struct TlsFileDatabase -->
`TlsFileDatabase` is implemented by `TlsDatabase` objects which load
their certificate information from a file. It is an interface which
TLS library specific subtypes implement.

# Implements

[`TlsFileDatabaseExt`](trait.TlsFileDatabaseExt.html), [`TlsDatabaseExt`](trait.TlsDatabaseExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsFileDatabaseExt -->
Trait containing all `TlsFileDatabase` methods.

# Implementors

[`TlsFileDatabase`](struct.TlsFileDatabase.html)
<!-- impl TlsFileDatabase::fn new -->
Creates a new `TlsFileDatabase` which uses anchor certificate authorities
in `anchors` to verify certificate chains.

The certificates in `anchors` must be PEM encoded.
## `anchors`
filename of anchor certificate authorities.

# Returns

the new
`TlsFileDatabase`, or `None` on error
<!-- trait TlsFileDatabaseExt::fn get_property_anchors -->
The path to a file containing PEM encoded certificate authority
root anchors. The certificates in this file will be treated as
root authorities for the purpose of verifying other certificates
via the `TlsDatabaseExt::verify_chain` operation.
<!-- trait TlsFileDatabaseExt::fn set_property_anchors -->
The path to a file containing PEM encoded certificate authority
root anchors. The certificates in this file will be treated as
root authorities for the purpose of verifying other certificates
via the `TlsDatabaseExt::verify_chain` operation.
<!-- struct TlsInteraction -->
`TlsInteraction` provides a mechanism for the TLS connection and database
code to interact with the user. It can be used to ask the user for passwords.

To use a `TlsInteraction` with a TLS connection use
`TlsConnectionExt::set_interaction`.

Callers should instantiate a derived class that implements the various
interaction methods to show the required dialogs.

Callers should use the 'invoke' functions like
`TlsInteractionExt::invoke_ask_password` to run interaction methods. These
functions make sure that the interaction is invoked in the main loop
and not in the current thread, if the current thread is not running the
main loop.

Derived classes can choose to implement whichever interactions methods they'd
like to support by overriding those virtual methods in their class
initialization function. Any interactions not implemented will return
`TlsInteractionResult::Unhandled`. If a derived class implements an async method,
it must also implement the corresponding finish method.

# Implements

[`TlsInteractionExt`](trait.TlsInteractionExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsInteractionExt -->
Trait containing all `TlsInteraction` methods.

# Implementors

[`TlsInteraction`](struct.TlsInteraction.html)
<!-- trait TlsInteractionExt::fn ask_password -->
Run synchronous interaction to ask the user for a password. In general,
`TlsInteractionExt::invoke_ask_password` should be used instead of this
function.

Derived subclasses usually implement a password prompt, although they may
also choose to provide a password from elsewhere. The `password` value will
be filled in and then `callback` will be called. Alternatively the user may
abort this password request, which will usually abort the TLS connection.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code. Certain implementations may
not support immediate cancellation.
## `password`
a `TlsPassword` object
## `cancellable`
an optional `Cancellable` cancellation object

# Returns

The status of the ask password interaction.
<!-- trait TlsInteractionExt::fn ask_password_async -->
Run asynchronous interaction to ask the user for a password. In general,
`TlsInteractionExt::invoke_ask_password` should be used instead of this
function.

Derived subclasses usually implement a password prompt, although they may
also choose to provide a password from elsewhere. The `password` value will
be filled in and then `callback` will be called. Alternatively the user may
abort this password request, which will usually abort the TLS connection.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code. Certain implementations may
not support immediate cancellation.

Certain implementations may not support immediate cancellation.
## `password`
a `TlsPassword` object
## `cancellable`
an optional `Cancellable` cancellation object
## `callback`
will be called when the interaction completes
## `user_data`
data to pass to the `callback`
<!-- trait TlsInteractionExt::fn ask_password_finish -->
Complete an ask password user interaction request. This should be once
the `TlsInteractionExt::ask_password_async` completion callback is called.

If `TlsInteractionResult::Handled` is returned, then the `TlsPassword` passed
to `TlsInteractionExt::ask_password` will have its password filled in.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code.
## `result`
the result passed to the callback

# Returns

The status of the ask password interaction.
<!-- trait TlsInteractionExt::fn invoke_ask_password -->
Invoke the interaction to ask the user for a password. It invokes this
interaction in the main loop, specifically the `glib::MainContext` returned by
`glib::MainContext::get_thread_default` when the interaction is created. This
is called by called by `TlsConnection` or `TlsDatabase` to ask the user
for a password.

Derived subclasses usually implement a password prompt, although they may
also choose to provide a password from elsewhere. The `password` value will
be filled in and then `callback` will be called. Alternatively the user may
abort this password request, which will usually abort the TLS connection.

The implementation can either be a synchronous (eg: modal dialog) or an
asynchronous one (eg: modeless dialog). This function will take care of
calling which ever one correctly.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code. Certain implementations may
not support immediate cancellation.
## `password`
a `TlsPassword` object
## `cancellable`
an optional `Cancellable` cancellation object

# Returns

The status of the ask password interaction.
<!-- trait TlsInteractionExt::fn invoke_request_certificate -->
Invoke the interaction to ask the user to choose a certificate to
use with the connection. It invokes this interaction in the main
loop, specifically the `glib::MainContext` returned by
`glib::MainContext::get_thread_default` when the interaction is
created. This is called by called by `TlsConnection` when the peer
requests a certificate during the handshake.

Derived subclasses usually implement a certificate selector,
although they may also choose to provide a certificate from
elsewhere. Alternatively the user may abort this certificate
request, which may or may not abort the TLS connection.

The implementation can either be a synchronous (eg: modal dialog) or an
asynchronous one (eg: modeless dialog). This function will take care of
calling which ever one correctly.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code. Certain implementations may
not support immediate cancellation.
## `connection`
a `TlsConnection` object
## `flags`
flags providing more information about the request
## `cancellable`
an optional `Cancellable` cancellation object

# Returns

The status of the certificate request interaction.
<!-- trait TlsInteractionExt::fn request_certificate -->
Run synchronous interaction to ask the user to choose a certificate to use
with the connection. In general, `TlsInteractionExt::invoke_request_certificate`
should be used instead of this function.

Derived subclasses usually implement a certificate selector, although they may
also choose to provide a certificate from elsewhere. Alternatively the user may
abort this certificate request, which will usually abort the TLS connection.

If `TlsInteractionResult::Handled` is returned, then the `TlsConnection`
passed to `TlsInteractionExt::request_certificate` will have had its
`TlsConnection:certificate` filled in.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code. Certain implementations may
not support immediate cancellation.
## `connection`
a `TlsConnection` object
## `flags`
flags providing more information about the request
## `cancellable`
an optional `Cancellable` cancellation object

# Returns

The status of the request certificate interaction.
<!-- trait TlsInteractionExt::fn request_certificate_async -->
Run asynchronous interaction to ask the user for a certificate to use with
the connection. In general, `TlsInteractionExt::invoke_request_certificate` should
be used instead of this function.

Derived subclasses usually implement a certificate selector, although they may
also choose to provide a certificate from elsewhere. `callback` will be called
when the operation completes. Alternatively the user may abort this certificate
request, which will usually abort the TLS connection.
## `connection`
a `TlsConnection` object
## `flags`
flags providing more information about the request
## `cancellable`
an optional `Cancellable` cancellation object
## `callback`
will be called when the interaction completes
## `user_data`
data to pass to the `callback`
<!-- trait TlsInteractionExt::fn request_certificate_finish -->
Complete an request certificate user interaction request. This should be once
the `TlsInteractionExt::request_certificate_async` completion callback is called.

If `TlsInteractionResult::Handled` is returned, then the `TlsConnection`
passed to `TlsInteractionExt::request_certificate_async` will have had its
`TlsConnection:certificate` filled in.

If the interaction is cancelled by the cancellation object, or by the
user then `TlsInteractionResult::Failed` will be returned with an error that
contains a `IOErrorEnum::Cancelled` error code.
## `result`
the result passed to the callback

# Returns

The status of the request certificate interaction.
<!-- enum TlsInteractionResult -->
`TlsInteractionResult` is returned by various functions in `TlsInteraction`
when finishing an interaction request.
<!-- enum TlsInteractionResult::variant Unhandled -->
The interaction was unhandled (i.e. not
 implemented).
<!-- enum TlsInteractionResult::variant Handled -->
The interaction completed, and resulting data
 is available.
<!-- enum TlsInteractionResult::variant Failed -->
The interaction has failed, or was cancelled.
 and the operation should be aborted.
<!-- struct TlsPassword -->
Holds a password used in TLS.

# Implements

[`TlsPasswordExt`](trait.TlsPasswordExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsPasswordExt -->
Trait containing all `TlsPassword` methods.

# Implementors

[`TlsPassword`](struct.TlsPassword.html)
<!-- impl TlsPassword::fn new -->
Create a new `TlsPassword` object.
## `flags`
the password flags
## `description`
description of what the password is for

# Returns

The newly allocated password object
<!-- trait TlsPasswordExt::fn get_description -->
Get a description string about what the password will be used for.

# Returns

The description of the password.
<!-- trait TlsPasswordExt::fn get_flags -->
Get flags about the password.

# Returns

The flags about the password.
<!-- trait TlsPasswordExt::fn get_value -->
Get the password value. If `length` is not `None` then it will be
filled in with the length of the password value. (Note that the
password value is not nul-terminated, so you can only pass `None`
for `length` in contexts where you know the password will have a
certain fixed length.)
## `length`
location to place the length of the password.

# Returns

The password value (owned by the password object).
<!-- trait TlsPasswordExt::fn get_warning -->
Get a user readable translated warning. Usually this warning is a
representation of the password flags returned from
`TlsPasswordExt::get_flags`.

# Returns

The warning.
<!-- trait TlsPasswordExt::fn set_description -->
Set a description string about what the password will be used for.
## `description`
The description of the password
<!-- trait TlsPasswordExt::fn set_flags -->
Set flags about the password.
## `flags`
The flags about the password
<!-- trait TlsPasswordExt::fn set_value -->
Set the value for this password. The `value` will be copied by the password
object.

Specify the `length`, for a non-nul-terminated password. Pass -1 as
`length` if using a nul-terminated password, and `length` will be
calculated automatically. (Note that the terminating nul is not
considered part of the password in this case.)
## `value`
the new password value
## `length`
the length of the password, or -1
<!-- trait TlsPasswordExt::fn set_value_full -->
Provide the value for this password.

The `value` will be owned by the password object, and later freed using
the `destroy` function callback.

Specify the `length`, for a non-nul-terminated password. Pass -1 as
`length` if using a nul-terminated password, and `length` will be
calculated automatically. (Note that the terminating nul is not
considered part of the password in this case.)
## `value`
the value for the password
## `length`
the length of the password, or -1
## `destroy`
a function to use to free the password.
<!-- trait TlsPasswordExt::fn set_warning -->
Set a user readable translated warning. Usually this warning is a
representation of the password flags returned from
`TlsPasswordExt::get_flags`.
## `warning`
The user readable warning
<!-- enum TlsRehandshakeMode -->
When to allow rehandshaking. See
`TlsConnectionExt::set_rehandshake_mode`.
<!-- enum TlsRehandshakeMode::variant Never -->
Never allow rehandshaking
<!-- enum TlsRehandshakeMode::variant Safely -->
Allow safe rehandshaking only
<!-- enum TlsRehandshakeMode::variant Unsafely -->
Allow unsafe rehandshaking
<!-- struct TlsServerConnection -->
`TlsServerConnection` is the server-side subclass of `TlsConnection`,
representing a server-side TLS connection.

# Implements

[`TlsServerConnectionExt`](trait.TlsServerConnectionExt.html), [`TlsConnectionExt`](trait.TlsConnectionExt.html), [`IOStreamExt`](trait.IOStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait TlsServerConnectionExt -->
Trait containing all `TlsServerConnection` methods.

# Implementors

[`TlsServerConnection`](struct.TlsServerConnection.html)
<!-- impl TlsServerConnection::fn new -->
Creates a new `TlsServerConnection` wrapping `base_io_stream` (which
must have pollable input and output streams).

See the documentation for `TlsConnection:base-io-stream` for restrictions
on when application code can run operations on the `base_io_stream` after
this function has returned.
## `base_io_stream`
the `IOStream` to wrap
## `certificate`
the default server certificate, or `None`

# Returns

the new
`TlsServerConnection`, or `None` on error
<!-- trait TlsServerConnectionExt::fn get_property_authentication_mode -->
The `TlsAuthenticationMode` for the server. This can be changed
before calling `TlsConnectionExt::handshake` if you want to
rehandshake with a different mode from the initial handshake.
<!-- trait TlsServerConnectionExt::fn set_property_authentication_mode -->
The `TlsAuthenticationMode` for the server. This can be changed
before calling `TlsConnectionExt::handshake` if you want to
rehandshake with a different mode from the initial handshake.
<!-- struct UnixInputStream -->
`UnixInputStream` implements `InputStream` for reading from a UNIX
file descriptor, including asynchronous operations. (If the file
descriptor refers to a socket or pipe, this will use `poll` to do
asynchronous I/O. If it refers to a regular file, it will fall back
to doing asynchronous I/O in another thread.)

Note that `<gio/gunixinputstream.h>` belongs to the UNIX-specific GIO
interfaces, thus you have to use the `gio-unix-2.0.pc` pkg-config
file when using it.

# Implements

[`UnixInputStreamExt`](trait.UnixInputStreamExt.html), [`InputStreamExt`](trait.InputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableInputStreamExt`](trait.PollableInputStreamExt.html), [`UnixInputStreamExtManual`](prelude/trait.UnixInputStreamExtManual.html), [`InputStreamExtManual`](prelude/trait.InputStreamExtManual.html), [`PollableInputStreamExtManual`](prelude/trait.PollableInputStreamExtManual.html)
<!-- trait UnixInputStreamExt -->
Trait containing all `UnixInputStream` methods.

# Implementors

[`UnixInputStream`](struct.UnixInputStream.html)
<!-- impl UnixInputStream::fn new -->
Creates a new `UnixInputStream` for the given `fd`.

If `close_fd` is `true`, the file descriptor will be closed
when the stream is closed.
## `fd`
a UNIX file descriptor
## `close_fd`
`true` to close the file descriptor when done

# Returns

a new `UnixInputStream`
<!-- trait UnixInputStreamExt::fn get_close_fd -->
Returns whether the file descriptor of `self` will be
closed when the stream is closed.

# Returns

`true` if the file descriptor is closed when done
<!-- trait UnixInputStreamExtManual::fn get_fd -->
Return the UNIX file descriptor that the stream reads from.

# Returns

The file descriptor of `self`
<!-- trait UnixInputStreamExtManual::fn set_close_fd -->
Sets whether the file descriptor of `self` shall be closed
when the stream is closed.
## `close_fd`
`true` to close the file descriptor when done
<!-- trait UnixInputStreamExt::fn get_property_close_fd -->
Whether to close the file descriptor when the stream is closed.
<!-- trait UnixInputStreamExt::fn set_property_close_fd -->
Whether to close the file descriptor when the stream is closed.
<!-- trait UnixInputStreamExt::fn get_property_fd -->
The file descriptor that the stream reads from.
<!-- trait UnixInputStreamExt::fn set_property_fd -->
The file descriptor that the stream reads from.
<!-- struct UnixOutputStream -->
`UnixOutputStream` implements `OutputStream` for writing to a UNIX
file descriptor, including asynchronous operations. (If the file
descriptor refers to a socket or pipe, this will use `poll` to do
asynchronous I/O. If it refers to a regular file, it will fall back
to doing asynchronous I/O in another thread.)

Note that `<gio/gunixoutputstream.h>` belongs to the UNIX-specific GIO
interfaces, thus you have to use the `gio-unix-2.0.pc` pkg-config file
when using it.

# Implements

[`UnixOutputStreamExt`](trait.UnixOutputStreamExt.html), [`OutputStreamExt`](trait.OutputStreamExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`PollableOutputStreamExt`](trait.PollableOutputStreamExt.html), [`UnixOutputStreamExtManual`](prelude/trait.UnixOutputStreamExtManual.html), [`OutputStreamExtManual`](prelude/trait.OutputStreamExtManual.html), [`PollableOutputStreamExtManual`](prelude/trait.PollableOutputStreamExtManual.html)
<!-- trait UnixOutputStreamExt -->
Trait containing all `UnixOutputStream` methods.

# Implementors

[`UnixOutputStream`](struct.UnixOutputStream.html)
<!-- impl UnixOutputStream::fn new -->
Creates a new `UnixOutputStream` for the given `fd`.

If `close_fd`, is `true`, the file descriptor will be closed when
the output stream is destroyed.
## `fd`
a UNIX file descriptor
## `close_fd`
`true` to close the file descriptor when done

# Returns

a new `OutputStream`
<!-- trait UnixOutputStreamExt::fn get_close_fd -->
Returns whether the file descriptor of `self` will be
closed when the stream is closed.

# Returns

`true` if the file descriptor is closed when done
<!-- trait UnixOutputStreamExtManual::fn get_fd -->
Return the UNIX file descriptor that the stream writes to.

# Returns

The file descriptor of `self`
<!-- trait UnixOutputStreamExtManual::fn set_close_fd -->
Sets whether the file descriptor of `self` shall be closed
when the stream is closed.
## `close_fd`
`true` to close the file descriptor when done
<!-- trait UnixOutputStreamExt::fn get_property_close_fd -->
Whether to close the file descriptor when the stream is closed.
<!-- trait UnixOutputStreamExt::fn set_property_close_fd -->
Whether to close the file descriptor when the stream is closed.
<!-- trait UnixOutputStreamExt::fn get_property_fd -->
The file descriptor that the stream writes to.
<!-- trait UnixOutputStreamExt::fn set_property_fd -->
The file descriptor that the stream writes to.
<!-- struct UnixSocketAddress -->
Support for UNIX-domain (also known as local) sockets.

UNIX domain sockets are generally visible in the filesystem.
However, some systems support abstract socket names which are not
visible in the filesystem and not affected by the filesystem
permissions, visibility, etc. Currently this is only supported
under Linux. If you attempt to use abstract sockets on other
systems, function calls may return `IOErrorEnum::NotSupported`
errors. You can use `UnixSocketAddress::abstract_names_supported`
to see if abstract names are supported.

Note that `<gio/gunixsocketaddress.h>` belongs to the UNIX-specific GIO
interfaces, thus you have to use the `gio-unix-2.0.pc` pkg-config file
when using it.

# Implements

[`UnixSocketAddressExt`](trait.UnixSocketAddressExt.html), [`SocketAddressExt`](trait.SocketAddressExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`SocketConnectableExt`](trait.SocketConnectableExt.html), [`UnixSocketAddressExtManual`](prelude/trait.UnixSocketAddressExtManual.html)
<!-- trait UnixSocketAddressExt -->
Trait containing all `UnixSocketAddress` methods.

# Implementors

[`UnixSocketAddress`](struct.UnixSocketAddress.html)
<!-- impl UnixSocketAddress::fn new -->
Creates a new `UnixSocketAddress` for `path`.

To create abstract socket addresses, on systems that support that,
use `UnixSocketAddress::new_abstract`.
## `path`
the socket path

# Returns

a new `UnixSocketAddress`
<!-- impl UnixSocketAddress::fn new_abstract -->
Creates a new `UnixSocketAddressType::AbstractPadded`
`UnixSocketAddress` for `path`.

# Deprecated

Use `UnixSocketAddress::new_with_type`.
## `path`
the abstract name
## `path_len`
the length of `path`, or -1

# Returns

a new `UnixSocketAddress`
<!-- impl UnixSocketAddress::fn new_with_type -->
Creates a new `UnixSocketAddress` of type `type_` with name `path`.

If `type_` is `UnixSocketAddressType::Path`, this is equivalent to
calling `UnixSocketAddress::new`.

If `type_` is `UnixSocketAddressType::Anonymous`, `path` and `path_len` will be
ignored.

If `path_type` is `UnixSocketAddressType::Abstract`, then `path_len`
bytes of `path` will be copied to the socket's path, and only those
bytes will be considered part of the name. (If `path_len` is -1,
then `path` is assumed to be NUL-terminated.) For example, if `path`
was "test", then calling `SocketAddressExt::get_native_size` on the
returned socket would return 7 (2 bytes of overhead, 1 byte for the
abstract-socket indicator byte, and 4 bytes for the name "test").

If `path_type` is `UnixSocketAddressType::AbstractPadded`, then
`path_len` bytes of `path` will be copied to the socket's path, the
rest of the path will be padded with 0 bytes, and the entire
zero-padded buffer will be considered the name. (As above, if
`path_len` is -1, then `path` is assumed to be NUL-terminated.) In
this case, `SocketAddressExt::get_native_size` will always return
the full size of a `struct sockaddr_un`, although
`UnixSocketAddressExt::get_path_len` will still return just the
length of `path`.

`UnixSocketAddressType::Abstract` is preferred over
`UnixSocketAddressType::AbstractPadded` for new programs. Of course,
when connecting to a server created by another process, you must
use the appropriate type corresponding to how that process created
its listening socket.
## `path`
the name
## `path_len`
the length of `path`, or -1
## `type_`
a `UnixSocketAddressType`

# Returns

a new `UnixSocketAddress`
<!-- impl UnixSocketAddress::fn abstract_names_supported -->
Checks if abstract UNIX domain socket names are supported.

# Returns

`true` if supported, `false` otherwise
<!-- trait UnixSocketAddressExt::fn get_address_type -->
Gets `self`'s type.

# Returns

a `UnixSocketAddressType`
<!-- trait UnixSocketAddressExt::fn get_is_abstract -->
Tests if `self` is abstract.

# Deprecated

Use `UnixSocketAddressExt::get_address_type`

# Returns

`true` if the address is abstract, `false` otherwise
<!-- trait UnixSocketAddressExtManual::fn get_path -->
Gets `self`'s path, or for abstract sockets the "name".

Guaranteed to be zero-terminated, but an abstract socket
may contain embedded zeros, and thus you should use
`UnixSocketAddressExt::get_path_len` to get the true length
of this string.

# Returns

the path for `self`
<!-- trait UnixSocketAddressExt::fn get_path_len -->
Gets the length of `self`'s path.

For details, see `UnixSocketAddress::get_path`.

# Returns

the length of the path
<!-- trait UnixSocketAddressExt::fn get_property_abstract -->
Whether or not this is an abstract address

# Deprecated

Use `UnixSocketAddress:address-type`, which
distinguishes between zero-padded and non-zero-padded
abstract addresses.
<!-- trait UnixSocketAddressExt::fn set_property_abstract -->
Whether or not this is an abstract address

# Deprecated

Use `UnixSocketAddress:address-type`, which
distinguishes between zero-padded and non-zero-padded
abstract addresses.
<!-- enum UnixSocketAddressType -->
The type of name used by a `UnixSocketAddress`.
`UnixSocketAddressType::Path` indicates a traditional unix domain
socket bound to a filesystem path. `UnixSocketAddressType::Anonymous`
indicates a socket not bound to any name (eg, a client-side socket,
or a socket created with `socketpair`).

For abstract sockets, there are two incompatible ways of naming
them; the man pages suggest using the entire `struct sockaddr_un`
as the name, padding the unused parts of the `sun_path` field with
zeroes; this corresponds to `UnixSocketAddressType::AbstractPadded`.
However, many programs instead just use a portion of `sun_path`, and
pass an appropriate smaller length to `bind` or `connect`. This is
`UnixSocketAddressType::Abstract`.
<!-- enum UnixSocketAddressType::variant Invalid -->
invalid
<!-- enum UnixSocketAddressType::variant Anonymous -->
anonymous
<!-- enum UnixSocketAddressType::variant Path -->
a filesystem path
<!-- enum UnixSocketAddressType::variant Abstract -->
an abstract name
<!-- enum UnixSocketAddressType::variant AbstractPadded -->
an abstract name, 0-padded
 to the full length of a unix socket name
<!-- struct Vfs -->
Entry point for using GIO functionality.

# Implements

[`VfsExt`](trait.VfsExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait VfsExt -->
Trait containing all `Vfs` methods.

# Implementors

[`Vfs`](struct.Vfs.html)
<!-- impl Vfs::fn get_default -->
Gets the default `Vfs` for the system.

# Returns

a `Vfs`.
<!-- impl Vfs::fn get_local -->
Gets the local `Vfs` for the system.

# Returns

a `Vfs`.
<!-- trait VfsExt::fn get_file_for_path -->
Gets a `File` for `path`.
## `path`
a string containing a VFS path.

# Returns

a `File`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait VfsExt::fn get_file_for_uri -->
Gets a `File` for `uri`.

This operation never fails, but the returned object
might not support any I/O operation if the URI
is malformed or if the URI scheme is not supported.
## `uri`
a string containing a URI

# Returns

a `File`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait VfsExt::fn get_supported_uri_schemes -->
Gets a list of URI schemes supported by `self`.

# Returns

a `None`-terminated array of strings.
 The returned array belongs to GIO and must
 not be freed or modified.
<!-- trait VfsExt::fn is_active -->
Checks if the VFS is active.

# Returns

`true` if construction of the `self` was successful
 and it is now active.
<!-- trait VfsExt::fn parse_name -->
This operation never fails, but the returned object might
not support any I/O operations if the `parse_name` cannot
be parsed by the `Vfs` module.
## `parse_name`
a string to be parsed by the VFS module.

# Returns

a `File` for the given `parse_name`.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait VfsExt::fn register_uri_scheme -->
Registers `uri_func` and `parse_name_func` as the `File` URI and parse name
lookup functions for URIs with a scheme matching `scheme`.
Note that `scheme` is registered only within the running application, as
opposed to desktop-wide as it happens with GVfs backends.

When a `File` is requested with an URI containing `scheme` (e.g. through
`File::new_for_uri`), `uri_func` will be called to allow a custom
constructor. The implementation of `uri_func` should not be blocking, and
must not call `VfsExt::register_uri_scheme` or `VfsExt::unregister_uri_scheme`.

When `File::parse_name` is called with a parse name obtained from such file,
`parse_name_func` will be called to allow the `File` to be created again. In
that case, it's responsibility of `parse_name_func` to make sure the parse
name matches what the custom `File` implementation returned when
`File::get_parse_name` was previously called. The implementation of
`parse_name_func` should not be blocking, and must not call
`VfsExt::register_uri_scheme` or `VfsExt::unregister_uri_scheme`.

It's an error to call this function twice with the same scheme. To unregister
a custom URI scheme, use `VfsExt::unregister_uri_scheme`.

Feature: `v2_50`

## `scheme`
an URI scheme, e.g. "http"
## `uri_func`
a `GVfsFileLookupFunc`
## `uri_data`
custom data passed to be passed to `uri_func`, or `None`
## `uri_destroy`
function to be called when unregistering the
 URI scheme, or when `self` is disposed, to free the resources used
 by the URI lookup function
## `parse_name_func`
a `GVfsFileLookupFunc`
## `parse_name_data`
custom data passed to be passed to
 `parse_name_func`, or `None`
## `parse_name_destroy`
function to be called when unregistering the
 URI scheme, or when `self` is disposed, to free the resources used
 by the parse name lookup function

# Returns

`true` if `scheme` was successfully registered, or `false` if a handler
 for `scheme` already exists.
<!-- trait VfsExt::fn unregister_uri_scheme -->
Unregisters the URI handler for `scheme` previously registered with
`VfsExt::register_uri_scheme`.

Feature: `v2_50`

## `scheme`
an URI scheme, e.g. "http"

# Returns

`true` if `scheme` was successfully unregistered, or `false` if a
 handler for `scheme` does not exist.
<!-- struct Volume -->
The `Volume` interface represents user-visible objects that can be
mounted. Note, when porting from GnomeVFS, `Volume` is the moral
equivalent of `GnomeVFSDrive`.

Mounting a `Volume` instance is an asynchronous operation. For more
information about asynchronous operations, see `AsyncResult` and
`Task`. To mount a `Volume`, first call `Volume::mount` with (at
least) the `Volume` instance, optionally a `MountOperation` object
and a `GAsyncReadyCallback`.

Typically, one will only want to pass `None` for the
`MountOperation` if automounting all volumes when a desktop session
starts since it's not desirable to put up a lot of dialogs asking
for credentials.

The callback will be fired when the operation has resolved (either
with success or failure), and a `GAsyncReady` structure will be
passed to the callback. That callback should then call
`Volume::mount_finish` with the `Volume` instance and the
`GAsyncReady` data to see if the operation was completed
successfully. If an `error` is present when `Volume::mount_finish`
is called, then it will be filled with any error information.

## Volume Identifiers # {`volume`-identifier}

It is sometimes necessary to directly access the underlying
operating system object behind a volume (e.g. for passing a volume
to an application via the commandline). For this purpose, GIO
allows to obtain an 'identifier' for the volume. There can be
different kinds of identifiers, such as Hal UDIs, filesystem labels,
traditional Unix devices (e.g. `/dev/sda2`), UUIDs. GIO uses predefined
strings as names for the different kinds of identifiers:
`G_VOLUME_IDENTIFIER_KIND_UUID`, `G_VOLUME_IDENTIFIER_KIND_LABEL`, etc.
Use `Volume::get_identifier` to obtain an identifier for a volume.


Note that `G_VOLUME_IDENTIFIER_KIND_HAL_UDI` will only be available
when the gvfs hal volume monitor is in use. Other volume monitors
will generally be able to provide the `G_VOLUME_IDENTIFIER_KIND_UNIX_DEVICE`
identifier, which can be used to obtain a hal device by means of
`libhal_manager_find_device_string_match`.

# Implements

[`VolumeExt`](trait.VolumeExt.html)
<!-- trait VolumeExt -->
Trait containing all `Volume` methods.

# Implementors

[`Volume`](struct.Volume.html)
<!-- trait VolumeExt::fn can_eject -->
Checks if a volume can be ejected.

# Returns

`true` if the `self` can be ejected. `false` otherwise
<!-- trait VolumeExt::fn can_mount -->
Checks if a volume can be mounted.

# Returns

`true` if the `self` can be mounted. `false` otherwise
<!-- trait VolumeExt::fn eject_with_operation -->
Ejects a volume. This is an asynchronous operation, and is
finished by calling `Volume::eject_with_operation_finish` with the `self`
and `AsyncResult` data returned in the `callback`.
## `flags`
flags affecting the unmount if required for eject
## `mount_operation`
a `MountOperation` or `None` to
 avoid user interaction
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback`, or `None`
## `user_data`
user data passed to `callback`
<!-- trait VolumeExt::fn eject_with_operation_finish -->
Finishes ejecting a volume. If any errors occurred during the operation,
`error` will be set to contain the errors and `false` will be returned.
## `result`
a `AsyncResult`

# Returns

`true` if the volume was successfully ejected. `false` otherwise
<!-- trait VolumeExt::fn enumerate_identifiers -->
Gets the kinds of [identifiers][volume-identifier] that `self` has.
Use `Volume::get_identifier` to obtain the identifiers themselves.

# Returns

a `None`-terminated array
 of strings containing kinds of identifiers. Use `g_strfreev` to free.
<!-- trait VolumeExt::fn get_activation_root -->
Gets the activation root for a `Volume` if it is known ahead of
mount time. Returns `None` otherwise. If not `None` and if `self`
is mounted, then the result of `Mount::get_root` on the
`Mount` object obtained from `Volume::get_mount` will always
either be equal or a prefix of what this function returns. In
other words, in code


```C
  GMount *mount;
  GFile *mount_root
  GFile *volume_activation_root;

  mount = g_volume_get_mount (volume); // mounted, so never NULL
  mount_root = g_mount_get_root (mount);
  volume_activation_root = g_volume_get_activation_root (volume); // assume not NULL
```
then the expression

```C
  (g_file_has_prefix (volume_activation_root, mount_root) ||
   g_file_equal (volume_activation_root, mount_root))
```
will always be `true`.

Activation roots are typically used in `VolumeMonitor`
implementations to find the underlying mount to shadow, see
`Mount::is_shadowed` for more details.

# Returns

the activation root of `self`
 or `None`. Use `gobject::ObjectExt::unref` to free.
<!-- trait VolumeExt::fn get_drive -->
Gets the drive for the `self`.

# Returns

a `Drive` or `None` if `self` is not
 associated with a drive. The returned object should be unreffed
 with `gobject::ObjectExt::unref` when no longer needed.
<!-- trait VolumeExt::fn get_icon -->
Gets the icon for `self`.

# Returns

a `Icon`.
 The returned object should be unreffed with `gobject::ObjectExt::unref`
 when no longer needed.
<!-- trait VolumeExt::fn get_identifier -->
Gets the identifier of the given kind for `self`.
See the [introduction][volume-identifier] for more
information about volume identifiers.
## `kind`
the kind of identifier to return

# Returns

a newly allocated string containing the
 requested identifier, or `None` if the `Volume`
 doesn't have this kind of identifier
<!-- trait VolumeExt::fn get_mount -->
Gets the mount for the `self`.

# Returns

a `Mount` or `None` if `self` isn't mounted.
 The returned object should be unreffed with `gobject::ObjectExt::unref`
 when no longer needed.
<!-- trait VolumeExt::fn get_name -->
Gets the name of `self`.

# Returns

the name for the given `self`. The returned string should
 be freed with `g_free` when no longer needed.
<!-- trait VolumeExt::fn get_sort_key -->
Gets the sort key for `self`, if any.

# Returns

Sorting key for `self` or `None` if no such key is available
<!-- trait VolumeExt::fn get_symbolic_icon -->
Gets the symbolic icon for `self`.

# Returns

a `Icon`.
 The returned object should be unreffed with `gobject::ObjectExt::unref`
 when no longer needed.
<!-- trait VolumeExt::fn get_uuid -->
Gets the UUID for the `self`. The reference is typically based on
the file system UUID for the volume in question and should be
considered an opaque string. Returns `None` if there is no UUID
available.

# Returns

the UUID for `self` or `None` if no UUID
 can be computed.
 The returned string should be freed with `g_free`
 when no longer needed.
<!-- trait VolumeExt::fn mount -->
Mounts a volume. This is an asynchronous operation, and is
finished by calling `Volume::mount_finish` with the `self`
and `AsyncResult` returned in the `callback`.
## `flags`
flags affecting the operation
## `mount_operation`
a `MountOperation` or `None` to avoid user interaction
## `cancellable`
optional `Cancellable` object, `None` to ignore
## `callback`
a `GAsyncReadyCallback`, or `None`
## `user_data`
user data that gets passed to `callback`
<!-- trait VolumeExt::fn mount_finish -->
Finishes mounting a volume. If any errors occurred during the operation,
`error` will be set to contain the errors and `false` will be returned.

If the mount operation succeeded, `Volume::get_mount` on `self`
is guaranteed to return the mount right after calling this
function; there's no need to listen for the 'mount-added' signal on
`VolumeMonitor`.
## `result`
a `AsyncResult`

# Returns

`true`, `false` if operation failed
<!-- trait VolumeExt::fn should_automount -->
Returns whether the volume should be automatically mounted.

# Returns

`true` if the volume should be automatically mounted
<!-- trait VolumeExt::fn connect_changed -->
Emitted when the volume has been changed.
<!-- trait VolumeExt::fn connect_removed -->
This signal is emitted when the `Volume` have been removed. If
the recipient is holding references to the object they should
release them so the object can be finalized.
<!-- struct VolumeMonitor -->
`VolumeMonitor` is for listing the user interesting devices and volumes
on the computer. In other words, what a file selector or file manager
would show in a sidebar.

`VolumeMonitor` is not
[thread-default-context aware][g-main-context-push-thread-default],
and so should not be used other than from the main thread, with no
thread-default-context active.

# Implements

[`VolumeMonitorExt`](trait.VolumeMonitorExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html)
<!-- trait VolumeMonitorExt -->
Trait containing all `VolumeMonitor` methods.

# Implementors

[`VolumeMonitor`](struct.VolumeMonitor.html)
<!-- impl VolumeMonitor::fn get -->
Gets the volume monitor used by gio.

# Returns

a reference to the `VolumeMonitor` used by gio. Call
 `gobject::ObjectExt::unref` when done with it.
<!-- trait VolumeMonitorExt::fn get_connected_drives -->
Gets a list of drives connected to the system.

The returned list should be freed with `glib::List::free`, after
its elements have been unreffed with `gobject::ObjectExt::unref`.

# Returns

a `glib::List` of connected `Drive` objects.
<!-- trait VolumeMonitorExt::fn get_mount_for_uuid -->
Finds a `Mount` object by its UUID (see `Mount::get_uuid`)
## `uuid`
the UUID to look for

# Returns

a `Mount` or `None` if no such mount is available.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait VolumeMonitorExt::fn get_mounts -->
Gets a list of the mounts on the system.

The returned list should be freed with `glib::List::free`, after
its elements have been unreffed with `gobject::ObjectExt::unref`.

# Returns

a `glib::List` of `Mount` objects.
<!-- trait VolumeMonitorExt::fn get_volume_for_uuid -->
Finds a `Volume` object by its UUID (see `Volume::get_uuid`)
## `uuid`
the UUID to look for

# Returns

a `Volume` or `None` if no such volume is available.
 Free the returned object with `gobject::ObjectExt::unref`.
<!-- trait VolumeMonitorExt::fn get_volumes -->
Gets a list of the volumes on the system.

The returned list should be freed with `glib::List::free`, after
its elements have been unreffed with `gobject::ObjectExt::unref`.

# Returns

a `glib::List` of `Volume` objects.
<!-- trait VolumeMonitorExt::fn connect_drive_changed -->
Emitted when a drive changes.
## `drive`
the drive that changed
<!-- trait VolumeMonitorExt::fn connect_drive_connected -->
Emitted when a drive is connected to the system.
## `drive`
a `Drive` that was connected.
<!-- trait VolumeMonitorExt::fn connect_drive_disconnected -->
Emitted when a drive is disconnected from the system.
## `drive`
a `Drive` that was disconnected.
<!-- trait VolumeMonitorExt::fn connect_drive_eject_button -->
Emitted when the eject button is pressed on `drive`.
## `drive`
the drive where the eject button was pressed
<!-- trait VolumeMonitorExt::fn connect_drive_stop_button -->
Emitted when the stop button is pressed on `drive`.
## `drive`
the drive where the stop button was pressed
<!-- trait VolumeMonitorExt::fn connect_mount_added -->
Emitted when a mount is added.
## `mount`
a `Mount` that was added.
<!-- trait VolumeMonitorExt::fn connect_mount_changed -->
Emitted when a mount changes.
## `mount`
a `Mount` that changed.
<!-- trait VolumeMonitorExt::fn connect_mount_pre_unmount -->
May be emitted when a mount is about to be removed.

This signal depends on the backend and is only emitted if
GIO was used to unmount.
## `mount`
a `Mount` that is being unmounted.
<!-- trait VolumeMonitorExt::fn connect_mount_removed -->
Emitted when a mount is removed.
## `mount`
a `Mount` that was removed.
<!-- trait VolumeMonitorExt::fn connect_volume_added -->
Emitted when a mountable volume is added to the system.
## `volume`
a `Volume` that was added.
<!-- trait VolumeMonitorExt::fn connect_volume_changed -->
Emitted when mountable volume is changed.
## `volume`
a `Volume` that changed.
<!-- trait VolumeMonitorExt::fn connect_volume_removed -->
Emitted when a mountable volume is removed from the system.
## `volume`
a `Volume` that was removed.
<!-- struct ZlibCompressor -->
Zlib decompression

# Implements

[`ZlibCompressorExt`](trait.ZlibCompressorExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ConverterExt`](trait.ConverterExt.html), [`ConverterExtManual`](prelude/trait.ConverterExtManual.html)
<!-- trait ZlibCompressorExt -->
Trait containing all `ZlibCompressor` methods.

# Implementors

[`ZlibCompressor`](struct.ZlibCompressor.html)
<!-- impl ZlibCompressor::fn new -->
Creates a new `ZlibCompressor`.
## `format`
The format to use for the compressed data
## `level`
compression level (0-9), -1 for default

# Returns

a new `ZlibCompressor`
<!-- trait ZlibCompressorExt::fn get_file_info -->
Returns the `ZlibCompressor:file-info` property.

# Returns

a `FileInfo`, or `None`
<!-- trait ZlibCompressorExt::fn set_file_info -->
Sets `file_info` in `self`. If non-`None`, and `self`'s
`ZlibCompressor:format` property is `ZlibCompressorFormat::Gzip`,
it will be used to set the file name and modification time in
the GZIP header of the compressed data.

Note: it is an error to call this function while a compression is in
progress; it may only be called immediately after creation of `self`,
or after resetting it with `Converter::reset`.
## `file_info`
a `FileInfo`
<!-- trait ZlibCompressorExt::fn get_property_file_info -->
If set to a non-`None` `FileInfo` object, and `ZlibCompressor:format` is
`ZlibCompressorFormat::Gzip`, the compressor will write the file name
and modification time from the file info to the GZIP header.
<!-- trait ZlibCompressorExt::fn set_property_file_info -->
If set to a non-`None` `FileInfo` object, and `ZlibCompressor:format` is
`ZlibCompressorFormat::Gzip`, the compressor will write the file name
and modification time from the file info to the GZIP header.
<!-- enum ZlibCompressorFormat -->
Used to select the type of data format to use for `ZlibDecompressor`
and `ZlibCompressor`.
<!-- enum ZlibCompressorFormat::variant Zlib -->
deflate compression with zlib header
<!-- enum ZlibCompressorFormat::variant Gzip -->
gzip file format
<!-- enum ZlibCompressorFormat::variant Raw -->
deflate compression with no header
<!-- struct ZlibDecompressor -->
Zlib decompression

# Implements

[`ZlibDecompressorExt`](trait.ZlibDecompressorExt.html), [`glib::object::ObjectExt`](../glib/object/trait.ObjectExt.html), [`ConverterExt`](trait.ConverterExt.html), [`ConverterExtManual`](prelude/trait.ConverterExtManual.html)
<!-- trait ZlibDecompressorExt -->
Trait containing all `ZlibDecompressor` methods.

# Implementors

[`ZlibDecompressor`](struct.ZlibDecompressor.html)
<!-- impl ZlibDecompressor::fn new -->
Creates a new `ZlibDecompressor`.
## `format`
The format to use for the compressed data

# Returns

a new `ZlibDecompressor`
<!-- trait ZlibDecompressorExt::fn get_file_info -->
Retrieves the `FileInfo` constructed from the GZIP header data
of compressed data processed by `compressor`, or `None` if `self`'s
`ZlibDecompressor:format` property is not `ZlibCompressorFormat::Gzip`,
or the header data was not fully processed yet, or it not present in the
data stream at all.

# Returns

a `FileInfo`, or `None`
<!-- trait ZlibDecompressorExt::fn get_property_file_info -->
A `FileInfo` containing the information found in the GZIP header
of the data stream processed, or `None` if the header was not yet
fully processed, is not present at all, or the compressor's
`ZlibDecompressor:format` property is not `ZlibCompressorFormat::Gzip`.
