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

[`Action`](struct.Action.html), [`SimpleAction`](struct.SimpleAction.html)
<!-- impl Action::fn name_is_valid -->
Checks if `action_name` is valid.

`action_name` is valid if it consists only of alphanumeric characters,
plus '-' and '.'. The empty string is not a valid action name.

It is an error to call this function with a non-utf8 `action_name`.
`action_name` must not be `None`.

Feature: `v2_38`

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

Feature: `v2_38`

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

Feature: `v2_38`

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
<!-- trait ActionExt::fn get_property_parameter-type -->
The type of the parameter that must be given when activating the
action. This is immutable, and may be `None` if no parameter is needed when
activating the action.
<!-- trait ActionExt::fn get_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- trait ActionExt::fn get_property_state-type -->
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

[`ActionGroup`](struct.ActionGroup.html), [`Application`](struct.Application.html), [`SimpleActionGroup`](struct.SimpleActionGroup.html)
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

[`AppInfo`](struct.AppInfo.html)
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
## `launch_context`
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

Note that the check `<em>`may not`</em>` compare each individual field, and
only does an identity check. In case detecting changes in the contents
is needed, program code must additionally compare relevant fields.
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

Feature: `v2_34`


# Returns


 a list of content types.
<!-- trait AppInfoExt::fn launch -->
Launches the application. Passes `files` to the launched application
as arguments, using the optional `launch_context` to get information
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
on information provided in `launch_context`.
## `files`
a `glib::List` of `File` objects
## `launch_context`
a `AppLaunchContext` or `None`

# Returns

`true` on successful launch, `false` otherwise.
<!-- trait AppInfoExt::fn launch_uris -->
Launches the application. This passes the `uris` to the launched application
as arguments, using the optional `launch_context` to get information
about the details of the launcher (like what screen it is on).
On error, `error` will be set accordingly.

To launch the application without arguments pass a `None` `uris` list.

Note that even if the launch is successful the application launched
can fail to start if it runs into problems during startup. There is
no way to detect this.
## `uris`
a `glib::List` containing URIs to launch.
## `launch_context`
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
<!-- struct AppLaunchContext -->
Integrating the launch with the launching application. This is used to
handle for instance startup notification and launching the new application
on the same screen as the launching window.

# Implements

[`AppLaunchContextExt`](trait.AppLaunchContextExt.html), [`ObjectExt`](trait.ObjectExt.html)
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

the
 child's environment
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

Feature: `v2_36`

## `startup_notify_id`
the startup notification id for the failed launch
<!-- trait AppLaunchContextExt::fn connect_launched -->
The ::launched signal is emitted when a `AppInfo` is successfully
launched. The `platform_data` is an GVariant dictionary mapping
strings to variants (ie a{sv}), which contains additional,
platform-specific data about this launch. On UNIX, at least the
"pid" and "startup-notification-id" keys will be present.

Feature: `v2_36`

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
calling `ApplicationExt::run`. All checks for uniqueness are done
internally. If the application is the primary instance then the
startup signal is emitted and the mainloop runs. If the application
is not the primary instance then a signal is sent to the primary
instance and `ApplicationExt::run` promptly returns. See the code
examples below.

If used, the expected form of an application identifier is very close
to that of of a
[D-Bus bus name](http://dbus.freedesktop.org/doc/dbus-specification.html`message`-protocol-names-interface).
Examples include: "com.example.MyApp", "org.example.internal-apps.Calculator".
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
directly available via `ApplicationCommandLine::get_cwd`,
`ApplicationCommandLine::get_environ` and
`ApplicationCommandLine::get_platform_data`.

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

[`ApplicationExt`](trait.ApplicationExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ActionGroupExt`](trait.ActionGroupExt.html), [`ActionMapExt`](trait.ActionMapExt.html)
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

For convenience, the restrictions on application identifiers are
reproduced here:

- Application identifiers must contain only the ASCII characters
 "[A-Z][a-z][0-9]_-." and must not begin with a digit.

- Application identifiers must contain at least one '.' (period)
 character (and thus at least three elements).

- Application identifiers must not begin or end with a '.' (period)
 character.

- Application identifiers must not contain consecutive '.' (period)
 characters.

- Application identifiers must not exceed 255 characters.
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

Feature: `v2_42`

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
where `ApplicationCommandLine::get_options_dict` will return it.
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

Feature: `v2_40`

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

Feature: `v2_40`

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

Feature: `v2_34`


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

Feature: `v2_34`


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

Feature: `v2_42`


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

Feature: `v2_38`

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

Upon return to the mainloop, `ApplicationExt::run` will return,
calling only the 'shutdown' function before doing so.

The hold count is ignored.

The result of calling `ApplicationExt::run` again after it returns is
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
<!-- trait ApplicationExt::fn run -->
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

Feature: `v2_40`

## `id`
id of the notification, or `None`
## `notification`
the `Notification` to send
<!-- trait ApplicationExt::fn set_action_group -->
This used to be how actions were associated with a `Application`.
Now there is `ActionMap` for that.

# Deprecated since 2.32

Use the `ActionMap` interface instead. Never ever
mix use of this API with use of `ActionMap` on the same `self`
or things will go very badly wrong. This function is known to
introduce buggy behaviour (ie: signals not emitted on changes to the
action group), so you should really use `ActionMap` instead.
## `action_group`
a `ActionGroup`, or `None`
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

Feature: `v2_42`

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

Feature: `v2_38`

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

Feature: `v2_40`

## `id`
id of a previously sent notification
<!-- trait ApplicationExt::fn connect_activate -->
The ::activate signal is emitted on the primary instance when an
activation occurs. See `ApplicationExt::activate`.
<!-- trait ApplicationExt::fn connect_command_line -->
The ::command-line signal is emitted on the primary instance when
a commandline is not handled locally. See `ApplicationExt::run` and
the `ApplicationCommandLine` documentation for more information.
## `command_line`
a `ApplicationCommandLine` representing the
 passed commandline

# Returns

An integer that is set as the exit status for the calling
 process. See `ApplicationCommandLine::set_exit_status`.
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
read with `ApplicationCommandLine::get_options_dict`. The signal
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

Feature: `v2_40`

## `options`
the options dictionary

# Returns

an exit code. If you have handled your options and want
to exit the process, return a non-negative option, 0 for success,
and a positive value for failure. To continue, return -1 to let
the default option processing continue.
<!-- trait ApplicationExt::fn connect_open -->
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
<!-- trait ApplicationExt::fn get_property_is-busy -->
Whether the application is currently marked as busy through
`ApplicationExt::mark_busy` or `ApplicationExt::bind_busy_property`.

Feature: `v2_44`

<!-- struct Cancellable -->
GCancellable is a thread-safe operation cancellation stack used
throughout GIO to allow for cancellation of synchronous and
asynchronous operations.

# Implements

[`CancellableExt`](trait.CancellableExt.html), [`ObjectExt`](trait.ObjectExt.html)
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
with `CancellableExt::reset`.

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
with `CancellableExt::reset`.
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

[`FileExt`](trait.FileExt.html)
<!-- trait FileExt -->
Trait containing all `File` methods.

# Implementors

[`File`](struct.File.html)
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
`ApplicationCommandLine::create_file_for_arg` may be more useful
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

See also `ApplicationCommandLine::create_file_for_arg`.

Feature: `v2_36`

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

Feature: `v2_34`

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

Feature: `v2_34`

## `result`
a `AsyncResult`

# Returns

`true` if the file was deleted. `false` otherwise.
<!-- trait FileExt::fn dup -->
Duplicates a `File` handle. This operation does not duplicate
the actual file or directory represented by the `File`; see
`File::copy` if attempting to copy a file.

This call does no blocking I/O.

# Returns

a new `File` that is a duplicate
 of the given `File`.
<!-- trait FileExt::fn eject_mountable -->
Starts an asynchronous eject on a mountable.
When this operation has completed, `callback` will be called with
`user_user` data, and the operation can be finalized with
`File::eject_mountable_finish`.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

# Deprecated since 2.22

Use `File::eject_mountable_with_operation` instead.
## `flags`
flags affecting the operation
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn eject_mountable_finish -->
Finishes an asynchronous eject operation started by
`File::eject_mountable`.

# Deprecated since 2.22

Use `File::eject_mountable_with_operation_finish`
 instead.
## `result`
a `AsyncResult`

# Returns

`true` if the `self` was ejected successfully.
 `false` otherwise.
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

A native file s one expressed in the platform-native filename format,
e.g. "C:\Windows" or "/usr/bin/". This does not mean the file is local,
as it might be on a locally mounted remote filesystem.

On some systems non-native files may be available using the native
filesystem via a userspace filesystem (FUSE), in these cases this call
will return `false`, but `File::get_path` will still return a native path.

This call does no blocking I/O.

# Returns

`true` if `self` is native
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
a `GFileReadMoreCallback` to receive partial data
 and to specify whether further data should be read
## `callback`
a `GAsyncReadyCallback` to call when the request is satisfied
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

Feature: `v2_38`

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

Feature: `v2_38`

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

Feature: `v2_38`

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

Feature: `v2_38`

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

Feature: `v2_38`

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

Note that in many cases it is racy to first check for file existence
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
it. You can get the new etag from `FileOutputStream::get_etag`
after you've finished writing and closed the `FileOutputStream`. When
you load a new file you can use `FileInputStream::query_info` to
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
<!-- trait FileExt::fn replace_contents_async -->
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

Feature: `v2_40`

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

Some attributes can be unset by setting `attribute` to
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

Feature: `v2_38`

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

Feature: `v2_38`

## `result`
a `AsyncResult`

# Returns

`true` on successful trash, `false` otherwise.
<!-- trait FileExt::fn unmount_mountable -->
Unmounts a file of type G_FILE_TYPE_MOUNTABLE.

If `cancellable` is not `None`, then the operation can be cancelled by
triggering the cancellable object from another thread. If the operation
was cancelled, the error `IOErrorEnum::Cancelled` will be returned.

When the operation is finished, `callback` will be called.
You can then call `File::unmount_mountable_finish` to get
the result of the operation.

# Deprecated since 2.22

Use `File::unmount_mountable_with_operation` instead.
## `flags`
flags affecting the operation
## `cancellable`
optional `Cancellable` object,
 `None` to ignore
## `callback`
a `GAsyncReadyCallback` to call
 when the request is satisfied, or `None`
## `user_data`
the data to pass to callback function
<!-- trait FileExt::fn unmount_mountable_finish -->
Finishes an unmount operation, see `File::unmount_mountable` for details.

Finish an asynchronous unmount operation that was started
with `File::unmount_mountable`.

# Deprecated since 2.22

Use `File::unmount_mountable_with_operation_finish`
 instead.
## `result`
a `AsyncResult`

# Returns

`true` if the operation finished successfully.
 `false` otherwise.
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
<!-- struct FileInfo -->
Functionality for manipulating basic metadata for files. `FileInfo`
implements methods for getting information that all files should
contain, and allows for manipulation of extended attributes.

See [GFileAttribute][gio-GFileAttribute] for more information on how
GIO handles file attributes.

To obtain a `FileInfo` for a `File`, use `File::query_info` (or its
async variant). To obtain a `FileInfo` for a file input or output
stream, use `FileInputStream::query_info` or
`FileOutputStream::query_info` (or their async variants).

To change the actual attributes of a file, you should then set the
attribute in the `FileInfo` and call `File::set_attributes_from_info`
or `File::set_attributes_async` on a GFile.

However, not all attributes can be changed in the file. For instance,
the actual size of a file cannot be changed via `FileInfoExt::set_size`.
You may call `File::query_settable_attributes` and
`File::query_writable_namespaces` to discover the settable attributes
of a particular file at runtime.

`FileAttributeMatcher` allows for searching through a `FileInfo` for
attributes.

# Implements

[`FileInfoExt`](trait.FileInfoExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait FileInfoExt -->
Trait containing all `FileInfo` methods.

# Implementors

[`FileInfo`](struct.FileInfo.html)
<!-- impl FileInfo::fn new -->
Creates a new file info structure.

# Returns

a `FileInfo`.
<!-- trait FileInfoExt::fn clear_status -->
Clears the status information from `self`.
<!-- trait FileInfoExt::fn copy_into -->
First clears all of the [GFileAttribute][gio-GFileAttribute] of `dest_info`,
and then copies all of the file attributes from `self` to `dest_info`.
## `dest_info`
destination to copy attributes to.
<!-- trait FileInfoExt::fn dup -->
Duplicates a file info structure.

# Returns

a duplicate `FileInfo` of `self`.
<!-- trait FileInfoExt::fn get_attribute_as_string -->
Gets the value of a attribute, formated as a string.
This escapes things as needed to make the string valid
utf8.
## `attribute`
a file attribute key.

# Returns

a UTF-8 string associated with the given `attribute`.
 When you're done with the string it must be freed with `g_free`.
<!-- trait FileInfoExt::fn get_attribute_boolean -->
Gets the value of a boolean attribute. If the attribute does not
contain a boolean value, `false` will be returned.
## `attribute`
a file attribute key.

# Returns

the boolean value contained within the attribute.
<!-- trait FileInfoExt::fn get_attribute_byte_string -->
Gets the value of a byte string attribute. If the attribute does
not contain a byte string, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

the contents of the `attribute` value as a byte string, or
`None` otherwise.
<!-- trait FileInfoExt::fn get_attribute_data -->
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
<!-- trait FileInfoExt::fn get_attribute_int32 -->
Gets a signed 32-bit integer contained within the attribute. If the
attribute does not contain a signed 32-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

a signed 32-bit integer from the attribute.
<!-- trait FileInfoExt::fn get_attribute_int64 -->
Gets a signed 64-bit integer contained within the attribute. If the
attribute does not contain an signed 64-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

a signed 64-bit integer from the attribute.
<!-- trait FileInfoExt::fn get_attribute_object -->
Gets the value of a `gobject::Object` attribute. If the attribute does
not contain a `gobject::Object`, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

a `gobject::Object` associated with the given `attribute`, or
`None` otherwise.
<!-- trait FileInfoExt::fn get_attribute_status -->
Gets the attribute status for an attribute key.
## `attribute`
a file attribute key

# Returns

a `FileAttributeStatus` for the given `attribute`, or
 `FileAttributeStatus::Unset` if the key is invalid.
<!-- trait FileInfoExt::fn get_attribute_string -->
Gets the value of a string attribute. If the attribute does
not contain a string, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

the contents of the `attribute` value as a UTF-8 string, or
`None` otherwise.
<!-- trait FileInfoExt::fn get_attribute_stringv -->
Gets the value of a stringv attribute. If the attribute does
not contain a stringv, `None` will be returned.
## `attribute`
a file attribute key.

# Returns

the contents of the `attribute` value as a stringv, or
`None` otherwise. Do not free. These returned strings are UTF-8.
<!-- trait FileInfoExt::fn get_attribute_type -->
Gets the attribute type for an attribute key.
## `attribute`
a file attribute key.

# Returns

a `FileAttributeType` for the given `attribute`, or
`FileAttributeType::Invalid` if the key is not set.
<!-- trait FileInfoExt::fn get_attribute_uint32 -->
Gets an unsigned 32-bit integer contained within the attribute. If the
attribute does not contain an unsigned 32-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

an unsigned 32-bit integer from the attribute.
<!-- trait FileInfoExt::fn get_attribute_uint64 -->
Gets a unsigned 64-bit integer contained within the attribute. If the
attribute does not contain an unsigned 64-bit integer, or is invalid,
0 will be returned.
## `attribute`
a file attribute key.

# Returns

a unsigned 64-bit integer from the attribute.
<!-- trait FileInfoExt::fn get_content_type -->
Gets the file's content type.

# Returns

a string containing the file's content type.
<!-- trait FileInfoExt::fn get_deletion_date -->
Returns the `glib::DateTime` representing the deletion date of the file, as
available in G_FILE_ATTRIBUTE_TRASH_DELETION_DATE. If the
G_FILE_ATTRIBUTE_TRASH_DELETION_DATE attribute is unset, `None` is returned.

Feature: `v2_36`


# Returns

a `glib::DateTime`, or `None`.
<!-- trait FileInfoExt::fn get_display_name -->
Gets a display name for a file.

# Returns

a string containing the display name.
<!-- trait FileInfoExt::fn get_edit_name -->
Gets the edit name for a file.

# Returns

a string containing the edit name.
<!-- trait FileInfoExt::fn get_etag -->
Gets the [entity tag][gfile-etag] for a given
`FileInfo`. See `G_FILE_ATTRIBUTE_ETAG_VALUE`.

# Returns

a string containing the value of the "etag:value" attribute.
<!-- trait FileInfoExt::fn get_file_type -->
Gets a file's type (whether it is a regular file, symlink, etc).
This is different from the file's content type, see `FileInfoExt::get_content_type`.

# Returns

a `FileType` for the given file.
<!-- trait FileInfoExt::fn get_icon -->
Gets the icon for a file.

# Returns

`Icon` for the given `self`.
<!-- trait FileInfoExt::fn get_is_backup -->
Checks if a file is a backup file.

# Returns

`true` if file is a backup file, `false` otherwise.
<!-- trait FileInfoExt::fn get_is_hidden -->
Checks if a file is hidden.

# Returns

`true` if the file is a hidden file, `false` otherwise.
<!-- trait FileInfoExt::fn get_is_symlink -->
Checks if a file is a symlink.

# Returns

`true` if the given `self` is a symlink.
<!-- trait FileInfoExt::fn get_modification_time -->
Gets the modification time of the current `self` and sets it
in `result`.
## `result`
a `glib::TimeVal`.
<!-- trait FileInfoExt::fn get_name -->
Gets the name for a file.

# Returns

a string containing the file name.
<!-- trait FileInfoExt::fn get_size -->
Gets the file's size.

# Returns

a `goffset` containing the file's size.
<!-- trait FileInfoExt::fn get_sort_order -->
Gets the value of the sort_order attribute from the `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER`.

# Returns

a `gint32` containing the value of the "standard::sort_order" attribute.
<!-- trait FileInfoExt::fn get_symbolic_icon -->
Gets the symbolic icon for a file.

Feature: `v2_34`


# Returns

`Icon` for the given `self`.
<!-- trait FileInfoExt::fn get_symlink_target -->
Gets the symlink target for a given `FileInfo`.

# Returns

a string containing the symlink target.
<!-- trait FileInfoExt::fn has_attribute -->
Checks if a file info structure has an attribute named `attribute`.
## `attribute`
a file attribute key.

# Returns

`true` if `Ginfo` has an attribute named `attribute`,
 `false` otherwise.
<!-- trait FileInfoExt::fn has_namespace -->
Checks if a file info structure has an attribute in the
specified `name_space`.
## `name_space`
a file attribute namespace.

# Returns

`true` if `Ginfo` has an attribute in `name_space`,
 `false` otherwise.
<!-- trait FileInfoExt::fn list_attributes -->
Lists the file info structure's attributes.
## `name_space`
a file attribute key's namespace, or `None` to list
 all attributes.

# Returns

a
null-terminated array of strings of all of the possible attribute
types for the given `name_space`, or `None` on error.
<!-- trait FileInfoExt::fn remove_attribute -->
Removes all cases of `attribute` from `self` if it exists.
## `attribute`
a file attribute key.
<!-- trait FileInfoExt::fn set_attribute -->
Sets the `attribute` to contain the given value, if possible. To unset the
attribute, use `G_ATTRIBUTE_TYPE_INVALID` for `type_`.
## `attribute`
a file attribute key.
## `type_`
a `FileAttributeType`
## `value_p`
pointer to the value
<!-- trait FileInfoExt::fn set_attribute_boolean -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a boolean value.
<!-- trait FileInfoExt::fn set_attribute_byte_string -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a byte string.
<!-- trait FileInfoExt::fn set_attribute_int32 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a signed 32-bit integer
<!-- trait FileInfoExt::fn set_attribute_int64 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
attribute name to set.
## `attr_value`
int64 value to set attribute to.
<!-- trait FileInfoExt::fn set_attribute_mask -->
Sets `mask` on `self` to match specific attribute types.
## `mask`
a `FileAttributeMatcher`.
<!-- trait FileInfoExt::fn set_attribute_object -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a `gobject::Object`.
<!-- trait FileInfoExt::fn set_attribute_status -->
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
<!-- trait FileInfoExt::fn set_attribute_string -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
a UTF-8 string.
<!-- trait FileInfoExt::fn set_attribute_stringv -->
Sets the `attribute` to contain the given `attr_value`,
if possible.

Sinze: 2.22
## `attribute`
a file attribute key
## `attr_value`
a `None` terminated array of UTF-8 strings.
<!-- trait FileInfoExt::fn set_attribute_uint32 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
an unsigned 32-bit integer.
<!-- trait FileInfoExt::fn set_attribute_uint64 -->
Sets the `attribute` to contain the given `attr_value`,
if possible.
## `attribute`
a file attribute key.
## `attr_value`
an unsigned 64-bit integer.
<!-- trait FileInfoExt::fn set_content_type -->
Sets the content type attribute for a given `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE`.
## `content_type`
a content type. See [GContentType][gio-GContentType]
<!-- trait FileInfoExt::fn set_display_name -->
Sets the display name for the current `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME`.
## `display_name`
a string containing a display name.
<!-- trait FileInfoExt::fn set_edit_name -->
Sets the edit name for the current file.
See `G_FILE_ATTRIBUTE_STANDARD_EDIT_NAME`.
## `edit_name`
a string containing an edit name.
<!-- trait FileInfoExt::fn set_file_type -->
Sets the file type in a `FileInfo` to `type_`.
See `G_FILE_ATTRIBUTE_STANDARD_TYPE`.
## `type_`
a `FileType`.
<!-- trait FileInfoExt::fn set_icon -->
Sets the icon for a given `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_ICON`.
## `icon`
a `Icon`.
<!-- trait FileInfoExt::fn set_is_hidden -->
Sets the "is_hidden" attribute in a `FileInfo` according to `is_hidden`.
See `G_FILE_ATTRIBUTE_STANDARD_IS_HIDDEN`.
## `is_hidden`
a `gboolean`.
<!-- trait FileInfoExt::fn set_is_symlink -->
Sets the "is_symlink" attribute in a `FileInfo` according to `is_symlink`.
See `G_FILE_ATTRIBUTE_STANDARD_IS_SYMLINK`.
## `is_symlink`
a `gboolean`.
<!-- trait FileInfoExt::fn set_modification_time -->
Sets the `G_FILE_ATTRIBUTE_TIME_MODIFIED` attribute in the file
info to the given time value.
## `mtime`
a `glib::TimeVal`.
<!-- trait FileInfoExt::fn set_name -->
Sets the name attribute for the current `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_NAME`.
## `name`
a string containing a name.
<!-- trait FileInfoExt::fn set_size -->
Sets the `G_FILE_ATTRIBUTE_STANDARD_SIZE` attribute in the file info
to the given size.
## `size`
a `goffset` containing the file's size.
<!-- trait FileInfoExt::fn set_sort_order -->
Sets the sort order attribute in the file info structure. See
`G_FILE_ATTRIBUTE_STANDARD_SORT_ORDER`.
## `sort_order`
a sort order integer.
<!-- trait FileInfoExt::fn set_symbolic_icon -->
Sets the symbolic icon for a given `FileInfo`.
See `G_FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON`.

Feature: `v2_34`

## `icon`
a `Icon`.
<!-- trait FileInfoExt::fn set_symlink_target -->
Sets the `G_FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET` attribute in the file info
to the given symlink target.
## `symlink_target`
a static string containing a path to a symlink target.
<!-- trait FileInfoExt::fn unset_attribute_mask -->
Unsets a mask set by `FileInfoExt::set_attribute_mask`, if one
is set.
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

[`Icon`](struct.Icon.html), [`ThemedIcon`](struct.ThemedIcon.html)
<!-- impl Icon::fn deserialize -->
Deserializes a `Icon` previously serialized using `Icon::serialize`.

Feature: `v2_38`

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

Feature: `v2_38`


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

- If `self` is a `ThemedIcon` with exactly one name, the encoding is
 simply the name (such as `network-server`).

# Returns

An allocated NUL-terminated UTF8 string or
`None` if `self` can't be serialized. Use `g_free` to free.
<!-- struct Menu -->
`Menu` is a simple implementation of `MenuModel`.
You populate a `Menu` by adding `MenuItem` instances to it.

There are some convenience functions to allow you to directly
add items (avoiding `MenuItem`) for the common cases. To add
a regular item, use `MenuExt::insert`. To add a section, use
`MenuExt::insert_section`. To add a submenu, use
`MenuExt::insert_submenu`.

# Implements

[`MenuExt`](trait.MenuExt.html), [`MenuModelExt`](trait.MenuModelExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MenuExt -->
Trait containing all `Menu` methods.

# Implementors

[`Menu`](struct.Menu.html)
<!-- impl Menu::fn new -->
Creates a new `Menu`.

The new menu has no items.

# Returns

a new `Menu`
<!-- trait MenuExt::fn append -->
Convenience function for appending a normal menu item to the end of
`self`. Combine `MenuItem::new` and `MenuExt::insert_item` for a more
flexible alternative.
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`
<!-- trait MenuExt::fn append_item -->
Appends `item` to the end of `self`.

See `MenuExt::insert_item` for more information.
## `item`
a `MenuItem` to append
<!-- trait MenuExt::fn append_section -->
Convenience function for appending a section menu item to the end of
`self`. Combine `MenuItem::new_section` and `MenuExt::insert_item` for a
more flexible alternative.
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section
<!-- trait MenuExt::fn append_submenu -->
Convenience function for appending a submenu menu item to the end of
`self`. Combine `MenuItem::new_submenu` and `MenuExt::insert_item` for a
more flexible alternative.
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu
<!-- trait MenuExt::fn freeze -->
Marks `self` as frozen.

After the menu is frozen, it is an error to attempt to make any
changes to it. In effect this means that the `Menu` API must no
longer be used.

This function causes `MenuModelExt::is_mutable` to begin returning
`false`, which has some positive performance implications.
<!-- trait MenuExt::fn insert -->
Convenience function for inserting a normal menu item into `self`.
Combine `MenuItem::new` and `MenuExt::insert_item` for a more flexible
alternative.
## `position`
the position at which to insert the item
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`
<!-- trait MenuExt::fn insert_item -->
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
See `MenuExt::insert`, `MenuExt::insert_section` and
`MenuExt::insert_submenu` as well as "prepend" and "append" variants of
each of these functions.
## `position`
the position at which to insert the item
## `item`
the `MenuItem` to insert
<!-- trait MenuExt::fn insert_section -->
Convenience function for inserting a section menu item into `self`.
Combine `MenuItem::new_section` and `MenuExt::insert_item` for a more
flexible alternative.
## `position`
the position at which to insert the item
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section
<!-- trait MenuExt::fn insert_submenu -->
Convenience function for inserting a submenu menu item into `self`.
Combine `MenuItem::new_submenu` and `MenuExt::insert_item` for a more
flexible alternative.
## `position`
the position at which to insert the item
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu
<!-- trait MenuExt::fn prepend -->
Convenience function for prepending a normal menu item to the start
of `self`. Combine `MenuItem::new` and `MenuExt::insert_item` for a more
flexible alternative.
## `label`
the section label, or `None`
## `detailed_action`
the detailed action string, or `None`
<!-- trait MenuExt::fn prepend_item -->
Prepends `item` to the start of `self`.

See `MenuExt::insert_item` for more information.
## `item`
a `MenuItem` to prepend
<!-- trait MenuExt::fn prepend_section -->
Convenience function for prepending a section menu item to the start
of `self`. Combine `MenuItem::new_section` and `MenuExt::insert_item` for
a more flexible alternative.
## `label`
the section label, or `None`
## `section`
a `MenuModel` with the items of the section
<!-- trait MenuExt::fn prepend_submenu -->
Convenience function for prepending a submenu menu item to the start
of `self`. Combine `MenuItem::new_submenu` and `MenuExt::insert_item` for
a more flexible alternative.
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu
<!-- trait MenuExt::fn remove -->
Removes an item from the menu.

`position` gives the index of the item to remove.

It is an error if position is not in range the range from 0 to one
less than the number of items in the menu.

It is not possible to remove items by identity since items are added
to the menu simply by copying their links and attributes (ie:
identity of the item itself is not preserved).
## `position`
the position of the item to remove
<!-- trait MenuExt::fn remove_all -->
Removes all items in the menu.

Feature: `v2_38`

<!-- struct MenuAttributeIter -->
`MenuAttributeIter` is an opaque structure type. You must access it
using the functions below.

# Implements

[`MenuAttributeIterExt`](trait.MenuAttributeIterExt.html), [`ObjectExt`](trait.ObjectExt.html)
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

[`MenuItemExt`](trait.MenuItemExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait MenuItemExt -->
Trait containing all `MenuItem` methods.

# Implementors

[`MenuItem`](struct.MenuItem.html)
<!-- impl MenuItem::fn new -->
Creates a new `MenuItem`.

If `label` is non-`None` it is used to set the "label" attribute of the
new item.

If `detailed_action` is non-`None` it is used to set the "action" and
possibly the "target" attribute of the new item. See
`MenuItemExt::set_detailed_action` for more information.
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

Feature: `v2_34`

## `model`
a `MenuModel`
## `item_index`
the index of an item in `model`

# Returns

a new `MenuItem`.
<!-- impl MenuItem::fn new_section -->
Creates a new `MenuItem` representing a section.

This is a convenience API around `MenuItem::new` and
`MenuItemExt::set_section`.

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
`MenuItemExt::set_submenu`.
## `label`
the section label, or `None`
## `submenu`
a `MenuModel` with the items of the submenu

# Returns

a new `MenuItem`
<!-- trait MenuItemExt::fn get_attribute -->
Queries the named `attribute` on `self`.

If the attribute exists and matches the `glib::VariantType` corresponding
to `format_string` then `format_string` is used to deconstruct the
value into the positional parameters and `true` is returned.

If the attribute does not exist, or it does exist but has the wrong
type, then the positional parameters are ignored and `false` is
returned.

Feature: `v2_34`

## `attribute`
the attribute name to query
## `format_string`
a `glib::Variant` format string

# Returns

`true` if the named attribute was found with the expected
 type
<!-- trait MenuItemExt::fn get_attribute_value -->
Queries the named `attribute` on `self`.

If `expected_type` is specified and the attribute does not have this
type, `None` is returned. `None` is also returned if the attribute
simply does not exist.

Feature: `v2_34`

## `attribute`
the attribute name to query
## `expected_type`
the expected type of the attribute

# Returns

the attribute value, or `None`
<!-- trait MenuItemExt::fn get_link -->
Queries the named `link` on `self`.

Feature: `v2_34`

## `link`
the link name to query

# Returns

the link, or `None`
<!-- trait MenuItemExt::fn set_action_and_target -->
Sets or unsets the "action" and "target" attributes of `self`.

If `action` is `None` then both the "action" and "target" attributes
are unset (and `format_string` is ignored along with the positional
parameters).

If `action` is non-`None` then the "action" attribute is set.
`format_string` is then inspected. If it is non-`None` then the proper
position parameters are collected to create a `glib::Variant` instance to
use as the target value. If it is `None` then the positional
parameters are ignored and the "target" attribute is unset.

See also `MenuItemExt::set_action_and_target_value` for an equivalent
call that directly accepts a `glib::Variant`. See
`MenuItemExt::set_detailed_action` for a more convenient version that
works with string-typed targets.

See also `MenuItemExt::set_action_and_target_value` for a
description of the semantics of the action and target attributes.
## `action`
the name of the action for this item
## `format_string`
a GVariant format string
<!-- trait MenuItemExt::fn set_action_and_target_value -->
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

See `MenuItemExt::set_action_and_target` or
`MenuItemExt::set_detailed_action` for two equivalent calls that are
probably more convenient for most uses.
## `action`
the name of the action for this item
## `target_value`
a `glib::Variant` to use as the action target
<!-- trait MenuItemExt::fn set_attribute -->
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

See also `MenuItemExt::set_attribute_value` for an equivalent call
that directly accepts a `glib::Variant`.
## `attribute`
the attribute to set
## `format_string`
a `glib::Variant` format string, or `None`
<!-- trait MenuItemExt::fn set_attribute_value -->
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

See also `MenuItemExt::set_attribute` for a more convenient way to do
the same.
## `attribute`
the attribute to set
## `value`
a `glib::Variant` to use as the value, or `None`
<!-- trait MenuItemExt::fn set_detailed_action -->
Sets the "action" and possibly the "target" attribute of `self`.

The format of `detailed_action` is the same format parsed by
`Action::parse_detailed_name`.

See `MenuItemExt::set_action_and_target` or
`MenuItemExt::set_action_and_target_value` for more flexible (but
slightly less convenient) alternatives.

See also `MenuItemExt::set_action_and_target_value` for a description of
the semantics of the action and target attributes.
## `detailed_action`
the "detailed" action string
<!-- trait MenuItemExt::fn set_icon -->
Sets (or unsets) the icon on `self`.

This call is the same as calling `Icon::serialize` and using the
result as the value to `MenuItemExt::set_attribute_value` for
`G_MENU_ATTRIBUTE_ICON`.

This API is only intended for use with "noun" menu items; things like
bookmarks or applications in an "Open With" menu. Don't use it on
menu items corresponding to verbs (eg: stock icons for 'Save' or
'Quit').

If `icon` is `None` then the icon is unset.

Feature: `v2_38`

## `icon`
a `Icon`, or `None`
<!-- trait MenuItemExt::fn set_label -->
Sets or unsets the "label" attribute of `self`.

If `label` is non-`None` it is used as the label for the menu item. If
it is `None` then the label attribute is unset.
## `label`
the label to set, or `None` to unset
<!-- trait MenuItemExt::fn set_link -->
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
<!-- trait MenuItemExt::fn set_section -->
Sets or unsets the "section" link of `self` to `section`.

The effect of having one menu appear as a section of another is
exactly as it sounds: the items from `section` become a direct part of
the menu that `self` is added to. See `MenuItem::new_section`
for more information about what it means for a menu item to be a
section.
## `section`
a `MenuModel`, or `None`
<!-- trait MenuItemExt::fn set_submenu -->
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

[`MenuLinkIterExt`](trait.MenuLinkIterExt.html), [`ObjectExt`](trait.ObjectExt.html)
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

[`MenuModelExt`](trait.MenuModelExt.html), [`ObjectExt`](trait.ObjectExt.html)
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

Feature: `v2_40`

# Implements

[`NotificationExt`](trait.NotificationExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait NotificationExt -->
Trait containing all `Notification` methods.

Feature: `v2_40`

# Implementors

[`Notification`](struct.Notification.html)
<!-- impl Notification::fn new -->
Creates a new `Notification` with `title` as its title.

After populating `notification` with more details, it can be sent to
the desktop shell with `ApplicationExt::send_notification`. Changing
any properties after this call will not have any effect until
resending `notification`.

Feature: `v2_40`

## `title`
the title of the notification

# Returns

a new `Notification` instance
<!-- trait NotificationExt::fn add_button -->
Adds a button to `self` that activates the action in
`detailed_action` when clicked. That action must be an
application-wide action (starting with "app."). If `detailed_action`
contains a target, the action will be activated with that target as
its parameter.

See `Action::parse_detailed_name` for a description of the format
for `detailed_action`.

Feature: `v2_40`

## `label`
label of the button
## `detailed_action`
a detailed action name
<!-- trait NotificationExt::fn add_button_with_target -->
Adds a button to `self` that activates `action` when clicked.
`action` must be an application-wide action (it must start with "app.").

If `target_format` is given, it is used to collect remaining
positional parameters into a `glib::Variant` instance, similar to
`glib::Variant::new`. `action` will be activated with that `glib::Variant` as its
parameter.

Feature: `v2_40`

## `label`
label of the button
## `action`
an action name
## `target_format`
a `glib::Variant` format string, or `None`
<!-- trait NotificationExt::fn add_button_with_target_value -->
Adds a button to `self` that activates `action` when clicked.
`action` must be an application-wide action (it must start with "app.").

If `target` is non-`None`, `action` will be activated with `target` as
its parameter.

Feature: `v2_40`

## `label`
label of the button
## `action`
an action name
## `target`
a `glib::Variant` to use as `action`'s parameter, or `None`
<!-- trait NotificationExt::fn set_body -->
Sets the body of `self` to `body`.

Feature: `v2_40`

## `body`
the new body for `self`, or `None`
<!-- trait NotificationExt::fn set_default_action -->
Sets the default action of `self` to `detailed_action`. This
action is activated when the notification is clicked on.

The action in `detailed_action` must be an application-wide action (it
must start with "app."). If `detailed_action` contains a target, the
given action will be activated with that target as its parameter.
See `Action::parse_detailed_name` for a description of the format
for `detailed_action`.

When no default action is set, the application that the notification
was sent on is activated.

Feature: `v2_40`

## `detailed_action`
a detailed action name
<!-- trait NotificationExt::fn set_default_action_and_target -->
Sets the default action of `self` to `action`. This action is
activated when the notification is clicked on. It must be an
application-wide action (it must start with "app.").

If `target_format` is given, it is used to collect remaining
positional parameters into a `glib::Variant` instance, similar to
`glib::Variant::new`. `action` will be activated with that `glib::Variant` as its
parameter.

When no default action is set, the application that the notification
was sent on is activated.

Feature: `v2_40`

## `action`
an action name
## `target_format`
a `glib::Variant` format string, or `None`
<!-- trait NotificationExt::fn set_default_action_and_target_value -->
Sets the default action of `self` to `action`. This action is
activated when the notification is clicked on. It must be an
application-wide action (start with "app.").

If `target` is non-`None`, `action` will be activated with `target` as
its parameter.

When no default action is set, the application that the notification
was sent on is activated.

Feature: `v2_40`

## `action`
an action name
## `target`
a `glib::Variant` to use as `action`'s parameter, or `None`
<!-- trait NotificationExt::fn set_icon -->
Sets the icon of `self` to `icon`.

Feature: `v2_40`

## `icon`
the icon to be shown in `self`, as a `Icon`
<!-- trait NotificationExt::fn set_priority -->
Sets the priority of `self` to `priority`. See
`NotificationPriority` for possible values.

Feature: `v2_42`

## `priority`
a `NotificationPriority`
<!-- trait NotificationExt::fn set_title -->
Sets the title of `self` to `title`.

Feature: `v2_40`

## `title`
the new title for `self`
<!-- trait NotificationExt::fn set_urgent -->
Deprecated in favor of `NotificationExt::set_priority`.

Feature: `v2_40`

## `urgent`
`true` if `self` is urgent
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

Feature: `v2_42`

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

[`PermissionExt`](trait.PermissionExt.html), [`ObjectExt`](trait.ObjectExt.html)
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
<!-- trait PermissionExt::fn get_property_can-acquire -->
`true` if it is generally possible to acquire the permission by calling
`PermissionExt::acquire`.
<!-- trait PermissionExt::fn get_property_can-release -->
`true` if it is generally possible to release the permission by calling
`PermissionExt::release`.
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
  </gresource>
</gresources>
```

This will create a resource bundle with the following files:

```text
/org/gtk/Example/data/splashscreen.png
/org/gtk/Example/dialog.ui
/org/gtk/Example/menumarkup.xml
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

There are two forms of the generated source, the default version uses the compiler support for constructor
and destructor functions (where available) to automatically create and register the `Resource` on startup
or library load time. If you pass --manual-register two functions to register/unregister the resource is instead
created. This requires an explicit initialization call in your application/library, but it works on all platforms,
even on the minor ones where this is not available. (Constructor support is available for at least Win32, Mac OS and Linux.)

Note that resource data can point directly into the data segment of e.g. a library, so if you are unloading libraries
during runtime you need to be very careful with keeping around pointers to data from a resource, as this goes away
when the library is unloaded. However, in practice this is not generally a problem, since most resource accesses
is for your own resources, and resource data is often used once, during parsing, and then released.

When debugging a program or testing a change to an installed version, it is often useful to be able to
replace resources in the program or library, without recompiling, for debugging or quick hacking and testing
purposes.

Since GLib 2.50, it is possible to use the `G_RESOURCE_OVERLAYS` environment variable to selectively overlay
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

Normally, a schema has as fixed path that determines where the settings
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

[`SettingsExt`](trait.SettingsExt.html), [`ObjectExt`](trait.ObjectExt.html)
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
<!-- impl Settings::fn list_relocatable_schemas -->
<!-- -->

# Deprecated since 2.40

Use `SettingsSchemaSource::list_schemas` instead

# Returns

a list of relocatable
 `Settings` schemas that are available. The list must not be
 modified or freed.
<!-- impl Settings::fn list_schemas -->
<!-- -->

# Deprecated since 2.40

Use `SettingsSchemaSource::list_schemas` instead.
If you used `Settings::list_schemas` to check for the presence of
a particular schema, use `SettingsSchemaSource::lookup` instead
of your whole loop.

# Returns

a list of `Settings`
 schemas that are available. The list must not be modified or
 freed.
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

Note that the lifecycle of the binding is tied to the object,
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

Note that the lifecycle of the binding is tied to the object,
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

Note that the lifecycle of the binding is tied to the object,
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

Feature: `v2_40`

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
<!-- trait SettingsExt::fn get_range -->
Queries the range of a key.

# Deprecated since 2.40

Use `SettingsSchemaKey::get_range` instead.
## `key`
the key to query the range of
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

Feature: `v2_40`

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
time and you should connect to the "children-changed" signal to watch
for those changes. Note that there is a race condition here: you may
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
<!-- trait SettingsExt::fn range_check -->
Checks if the given `value` is of the correct type and within the
permitted range for `key`.

# Deprecated since 2.40

Use `SettingsSchemaKey::range_check` instead.
## `key`
the key to check
## `value`
the value to check

# Returns

`true` if `value` is valid for `key`
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
<!-- trait SettingsExt::fn get_property_delay-apply -->
Whether the `Settings` object is in 'delay-apply' mode. See
`SettingsExt::delay` for details.
<!-- trait SettingsExt::fn get_property_has-unapplied -->
If this property is `true`, the `Settings` object has outstanding
changes that will be applied when `SettingsExt::apply` is called.
<!-- trait SettingsExt::fn get_property_path -->
The path within the backend where the settings are stored.
<!-- trait SettingsExt::fn set_property_path -->
The path within the backend where the settings are stored.
<!-- trait SettingsExt::fn get_property_schema -->
The name of the schema that describes the types of keys
for this `Settings` object.

The type of this property is *not* `SettingsSchema`.
`SettingsSchema` has only existed since version 2.32 and
unfortunately this name was used in previous versions to refer to
the schema ID rather than the schema itself. Take care to use the
'settings-schema' property if you wish to pass in a
`SettingsSchema`.

# Deprecated since 2.32

Use the 'schema-id' property instead. In a future
version, this property may instead refer to a `SettingsSchema`.
<!-- trait SettingsExt::fn set_property_schema -->
The name of the schema that describes the types of keys
for this `Settings` object.

The type of this property is *not* `SettingsSchema`.
`SettingsSchema` has only existed since version 2.32 and
unfortunately this name was used in previous versions to refer to
the schema ID rather than the schema itself. Take care to use the
'settings-schema' property if you wish to pass in a
`SettingsSchema`.

# Deprecated since 2.32

Use the 'schema-id' property instead. In a future
version, this property may instead refer to a `SettingsSchema`.
<!-- trait SettingsExt::fn get_property_schema-id -->
The name of the schema that describes the types of keys
for this `Settings` object.
<!-- trait SettingsExt::fn set_property_schema-id -->
The name of the schema that describes the types of keys
for this `Settings` object.
<!-- trait SettingsExt::fn get_property_settings-schema -->
The `SettingsSchema` describing the types of keys for this
`Settings` object.

Ideally, this property would be called 'schema'. `SettingsSchema`
has only existed since version 2.32, however, and before then the
'schema' property was used to refer to the ID of the schema rather
than the schema itself. Take care.
<!-- trait SettingsExt::fn set_property_settings-schema -->
The `SettingsSchema` describing the types of keys for this
`Settings` object.

Ideally, this property would be called 'schema'. `SettingsSchema`
has only existed since version 2.32, however, and before then the
'schema' property was used to refer to the ID of the schema rather
than the schema itself. Take care.
<!-- struct SimpleAction -->
A `SimpleAction` is the obvious simple implementation of the `Action`
interface. This is the easiest way to create an action for purposes of
adding it to a `SimpleActionGroup`.

See also ``GtkAction``.

# Implements

[`SimpleActionExt`](trait.SimpleActionExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ActionExt`](trait.ActionExt.html)
<!-- trait SimpleActionExt -->
Trait containing all `SimpleAction` methods.

# Implementors

[`SimpleAction`](struct.SimpleAction.html)
<!-- impl SimpleAction::fn new -->
Creates a new action.

The created action is stateless. See `SimpleAction::new_stateful`.
## `name`
the name of the action
## `parameter_type`
the type of parameter to the activate function

# Returns

a new `SimpleAction`
<!-- impl SimpleAction::fn new_stateful -->
Creates a new stateful action.

`state` is the initial state of the action. All future state values
must have the same `glib::VariantType` as the initial state.

If the `state` GVariant is floating, it is consumed.
## `name`
the name of the action
## `parameter_type`
the type of the parameter to the activate function
## `state`
the initial state of the action

# Returns

a new `SimpleAction`
<!-- trait SimpleActionExt::fn set_enabled -->
Sets the action as enabled or not.

An action must be enabled in order to be activated or in order to
have its state changed from outside callers.

This should only be called by the implementor of the action. Users
of the action should not attempt to modify its enabled flag.
## `enabled`
whether the action is enabled
<!-- trait SimpleActionExt::fn set_state -->
Sets the state of the action.

This directly updates the 'state' property to the given value.

This should only be called by the implementor of the action. Users
of the action should not attempt to directly modify the 'state'
property. Instead, they should call `Action::change_state` to
request the change.

If the `value` GVariant is floating, it is consumed.
## `value`
the new `glib::Variant` for the state
<!-- trait SimpleActionExt::fn set_state_hint -->
Sets the state hint for the action.

See `Action::get_state_hint` for more information about
action state hints.

Feature: `v2_44`

## `state_hint`
a `glib::Variant` representing the state hint
<!-- trait SimpleActionExt::fn connect_activate -->
Indicates that the action was just activated.

`parameter` will always be of the expected type. In the event that
an incorrect type was given, no signal will be emitted.

Since GLib 2.40, if no handler is connected to this signal then the
default behaviour for boolean-stated actions with a `None` parameter
type is to toggle them via the `SimpleAction::change-state` signal.
For stateful actions where the state type is equal to the parameter
type, the default is to forward them directly to
`SimpleAction::change-state`. This should allow almost all users
of `SimpleAction` to connect only one handler or the other.
## `parameter`
the parameter to the activation
<!-- trait SimpleActionExt::fn connect_change_state -->
Indicates that the action just received a request to change its
state.

`value` will always be of the correct state type. In the event that
an incorrect type was given, no signal will be emitted.

If no handler is connected to this signal then the default
behaviour is to call `SimpleActionExt::set_state` to set the state
to the requested value. If you connect a signal handler then no
default action is taken. If the state should change then you must
call `SimpleActionExt::set_state` from the handler.

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
<!-- trait SimpleActionExt::fn get_property_enabled -->
If `action` is currently enabled.

If the action is disabled then calls to `Action::activate` and
`Action::change_state` have no effect.
<!-- trait SimpleActionExt::fn set_property_enabled -->
If `action` is currently enabled.

If the action is disabled then calls to `Action::activate` and
`Action::change_state` have no effect.
<!-- trait SimpleActionExt::fn get_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `SimpleActionGroup`.
<!-- trait SimpleActionExt::fn set_property_name -->
The name of the action. This is mostly meaningful for identifying
the action once it has been added to a `SimpleActionGroup`.
<!-- trait SimpleActionExt::fn get_property_parameter-type -->
The type of the parameter that must be given when activating the
action.
<!-- trait SimpleActionExt::fn set_property_parameter-type -->
The type of the parameter that must be given when activating the
action.
<!-- trait SimpleActionExt::fn get_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- trait SimpleActionExt::fn set_property_state -->
The state of the action, or `None` if the action is stateless.
<!-- trait SimpleActionExt::fn get_property_state-type -->
The `glib::VariantType` of the state that the action has, or `None` if the
action is stateless.
<!-- struct SimpleActionGroup -->
`SimpleActionGroup` is a hash table filled with `Action` objects,
implementing the `ActionGroup` and `ActionMap` interfaces.

# Implements

[`SimpleActionGroupExt`](trait.SimpleActionGroupExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ActionGroupExt`](trait.ActionGroupExt.html), [`ActionMapExt`](trait.ActionMapExt.html)
<!-- trait SimpleActionGroupExt -->
Trait containing all `SimpleActionGroup` methods.

# Implementors

[`SimpleActionGroup`](struct.SimpleActionGroup.html)
<!-- impl SimpleActionGroup::fn new -->
Creates a new, empty, `SimpleActionGroup`.

# Returns

a new `SimpleActionGroup`
<!-- trait SimpleActionGroupExt::fn add_entries -->
A convenience function for creating multiple `SimpleAction` instances
and adding them to the action group.

# Deprecated since 2.38

Use `ActionMap::add_action_entries`
## `entries`
a pointer to the first item in
 an array of `ActionEntry` structs
## `n_entries`
the length of `entries`, or -1
## `user_data`
the user data for signal connections
<!-- trait SimpleActionGroupExt::fn insert -->
Adds an action to the action group.

If the action group already contains an action with the same name as
`action` then the old action is dropped from the group.

The action group takes its own reference on `action`.

# Deprecated since 2.38

Use `ActionMap::add_action`
## `action`
a `Action`
<!-- trait SimpleActionGroupExt::fn lookup -->
Looks up the action with the name `action_name` in the group.

If no such action exists, returns `None`.

# Deprecated since 2.38

Use `ActionMap::lookup_action`
## `action_name`
the name of an action

# Returns

a `Action`, or `None`
<!-- trait SimpleActionGroupExt::fn remove -->
Removes the named action from the action group.

If no action of this name is in the group then nothing happens.

# Deprecated since 2.38

Use `ActionMap::remove_action`
## `action_name`
the name of the action
<!-- struct SimplePermission -->
`SimplePermission` is a trivial implementation of `Permission` that
represents a permission that is either always or never allowed. The
value is given at construction and doesn't change.

Calling request or release will result in errors.

# Implements

[`PermissionExt`](trait.PermissionExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- impl SimplePermission::fn new -->
Creates a new `Permission` instance that represents an action that is
either always or never allowed.
## `allowed`
`true` if the action is allowed

# Returns

the `SimplePermission`, as a `Permission`
<!-- struct ThemedIcon -->
`ThemedIcon` is an implementation of `Icon` that supports icon themes.
`ThemedIcon` contains a list of all of the icons present in an icon
theme, so that icons can be looked up quickly. `ThemedIcon` does
not provide actual pixmaps for icons, just the icon names.
Ideally something like `gtk_icon_theme_choose_icon` should be used to
resolve the list of names so that fallback icons work nicely with
themes that inherit other themes.

# Implements

[`ThemedIconExt`](trait.ThemedIconExt.html), [`ObjectExt`](trait.ObjectExt.html), [`IconExt`](trait.IconExt.html)
<!-- trait ThemedIconExt -->
Trait containing all `ThemedIcon` methods.

# Implementors

[`ThemedIcon`](struct.ThemedIcon.html)
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
<!-- trait ThemedIconExt::fn append_name -->
Append a name to the list of icons from within `self`.

Note that doing so invalidates the hash computed by prior calls
to `Icon::hash`.
## `iconname`
name of icon to append to list of icons from within `self`.
<!-- trait ThemedIconExt::fn get_names -->
Gets the names of icons from within `self`.

# Returns

a list of icon names.
<!-- trait ThemedIconExt::fn prepend_name -->
Prepend a name to the list of icons from within `self`.

Note that doing so invalidates the hash computed by prior calls
to `Icon::hash`.
## `iconname`
name of icon to prepend to list of icons from within `self`.
<!-- trait ThemedIconExt::fn set_property_name -->
The icon name.
<!-- trait ThemedIconExt::fn get_property_names -->
A `None`-terminated array of icon names.
<!-- trait ThemedIconExt::fn set_property_names -->
A `None`-terminated array of icon names.
<!-- trait ThemedIconExt::fn get_property_use-default-fallbacks -->
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
<!-- trait ThemedIconExt::fn set_property_use-default-fallbacks -->
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
<!-- struct TlsCertificate -->
A certificate used for TLS authentication and encryption.
This can represent either a certificate only (eg, the certificate
received by a client from a server), or the combination of
a certificate and a private key (which is needed when acting as a
`TlsServerConnection`).

# Implements

[`TlsCertificateExt`](trait.TlsCertificateExt.html), [`ObjectExt`](trait.ObjectExt.html)
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

Feature: `v2_34`

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
<!-- trait TlsCertificateExt::fn get_property_certificate-pem -->
The PEM (ASCII) encoded representation of the certificate.
This property and the `TlsCertificate:certificate`
property represent the same data, just in different forms.
<!-- trait TlsCertificateExt::fn set_property_certificate-pem -->
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
<!-- trait TlsCertificateExt::fn set_property_private-key -->
The DER (binary) encoded representation of the certificate's
private key, in either PKCS`1` format or unencrypted PKCS`8`
format. This property (or the `TlsCertificate:private-key-pem`
property) can be set when constructing a key (eg, from a file),
but cannot be read.

PKCS`8` format is supported since 2.32; earlier releases only
support PKCS`1`. You can use the `openssl rsa`
tool to convert PKCS`8` keys to PKCS`1`.
<!-- trait TlsCertificateExt::fn set_property_private-key-pem -->
The PEM (ASCII) encoded representation of the certificate's
private key in either PKCS`1` format ("`BEGIN RSA PRIVATE
KEY`") or unencrypted PKCS`8` format ("`BEGIN
PRIVATE KEY`"). This property (or the
`TlsCertificate:private-key` property) can be set when
constructing a key (eg, from a file), but cannot be read.

PKCS`8` format is supported since 2.32; earlier releases only
support PKCS`1`. You can use the `openssl rsa`
tool to convert PKCS`8` keys to PKCS`1`.
