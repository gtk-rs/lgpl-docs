<!-- file boxed.rs -->
<!-- file_comment -->
`IMPL` Boxed wrapper implementation.
<!-- macro glib_boxed_wrapper -->
Wrapper implementations for Boxed types. See `glib_wrapper!`.
<!-- trait BoxedMemoryManager -->
Memory management functions for a boxed type.
<!-- trait BoxedMemoryManager::fn copy -->
Makes a copy.
<!-- trait BoxedMemoryManager::fn free -->
Frees the object.
<!-- struct Boxed -->
Encapsulates memory management logic for boxed types.
<!-- file bytes.rs -->
<!-- macro glib_wrapper!::struct Bytes -->
A shared immutable byte slice (the equivalent of `Rc<[u8]>`).

`From` implementations that take references (e.g. `&[u8]`) copy the
data. The `from_static` constructor avoids copying static data.

```
use glib::Bytes;

let v = vec![1, 2, 3];
let b = Bytes::from(&v);
assert_eq!(v, b);

let s = b"xyz";
let b = Bytes::from_static(s);
assert_eq!(&s[..], b);
```
<!-- impl Bytes::fn new -->
Copies `data` into a new shared slice.
<!-- impl Bytes::fn from_static -->
Creates a view into static `data` without copying.
<!-- file enums.rs -->
<!-- struct EnumClass -->
Representation of an `enum` for dynamically, at runtime, querying the values of the enum and
using them.
<!-- impl EnumClass::fn new -->
Create a new `EnumClass` from a `Type`.

Returns `None` if `type_` is not representing an enum.
<!-- impl EnumClass::fn type_ -->
`Type` of the enum.
<!-- impl EnumClass::fn get_value -->
Gets `EnumValue` by integer `value`, if existing.

Returns `None` if the enum does not contain any value
with `value`.
<!-- impl EnumClass::fn get_value_by_name -->
Gets `EnumValue` by string name `name`, if existing.

Returns `None` if the enum does not contain any value
with name `name`.
<!-- impl EnumClass::fn get_value_by_nick -->
Gets `EnumValue` by string nick `nick`, if existing.

Returns `None` if the enum does not contain any value
with nick `nick`.
<!-- impl EnumClass::fn get_values -->
Gets all `EnumValue` of this `EnumClass`.
<!-- impl EnumClass::fn to_value -->
Converts integer `value` to a `Value`, if part of the enum.
<!-- impl EnumClass::fn to_value_by_name -->
Converts string name `name` to a `Value`, if part of the enum.
<!-- impl EnumClass::fn to_value_by_nick -->
Converts string nick `nick` to a `Value`, if part of the enum.
<!-- struct EnumValue -->
Representation of a single enum value of an `EnumClass`.
<!-- impl EnumValue::fn get_value -->
Get integer value corresponding to the value.
<!-- impl EnumValue::fn get_name -->
Get name corresponding to the value.
<!-- impl EnumValue::fn get_nick -->
Get nick corresponding to the value.
<!-- impl EnumValue::fn to_value -->
Convert enum value to a `Value`.
<!-- impl EnumValue::fn from_value -->
Convert enum value from a `Value`.
<!-- impl EnumValue::fn get_class -->
Get `EnumClass` to which the enum value belongs.
<!-- struct FlagsClass -->
Representation of a `flags` for dynamically, at runtime, querying the values of the enum and
using them
<!-- impl FlagsClass::fn new -->
Create a new `FlagsClass` from a `Type`

Returns `None` if `type_` is not representing a flags type.
<!-- impl FlagsClass::fn type_ -->
`Type` of the flags.
<!-- impl FlagsClass::fn get_value -->
Gets `FlagsValue` by integer `value`, if existing.

Returns `None` if the flags do not contain any value
with `value`.
<!-- impl FlagsClass::fn get_value_by_name -->
Gets `FlagsValue` by string name `name`, if existing.

Returns `None` if the flags do not contain any value
with name `name`.
<!-- impl FlagsClass::fn get_value_by_nick -->
Gets `FlagsValue` by string nick `nick`, if existing.

Returns `None` if the flags do not contain any value
with nick `nick`.
<!-- impl FlagsClass::fn get_values -->
Gets all `FlagsValue` of this `FlagsClass`.
<!-- impl FlagsClass::fn to_value -->
Converts integer `value` to a `Value`, if part of the flags.
<!-- impl FlagsClass::fn to_value_by_name -->
Converts string name `name` to a `Value`, if part of the flags.
<!-- impl FlagsClass::fn to_value_by_nick -->
Converts string nick `nick` to a `Value`, if part of the flags.
<!-- impl FlagsClass::fn is_set -->
Checks if the flags corresponding to integer `f` is set in `value`.
<!-- impl FlagsClass::fn is_set_by_name -->
Checks if the flags corresponding to string name `name` is set in `value`.
<!-- impl FlagsClass::fn is_set_by_nick -->
Checks if the flags corresponding to string nick `nick` is set in `value`.
<!-- impl FlagsClass::fn set -->
Sets flags value corresponding to integer `f` in `value`, if part of that flags. If the
flag is already set, it will succeed without doing any changes.

Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
value otherwise.
<!-- impl FlagsClass::fn set_by_name -->
Sets flags value corresponding to string name `name` in `value`, if part of that flags.
If the flag is already set, it will succeed without doing any changes.

Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
value otherwise.
<!-- impl FlagsClass::fn set_by_nick -->
Sets flags value corresponding to string nick `nick` in `value`, if part of that flags.
If the flag is already set, it will succeed without doing any changes.

Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
value otherwise.
<!-- impl FlagsClass::fn unset -->
Unsets flags value corresponding to integer `f` in `value`, if part of that flags.
If the flag is already unset, it will succeed without doing any changes.

Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
value otherwise.
<!-- impl FlagsClass::fn unset_by_name -->
Unsets flags value corresponding to string name `name` in `value`, if part of that flags.
If the flag is already unset, it will succeed without doing any changes.

Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
value otherwise.
<!-- impl FlagsClass::fn unset_by_nick -->
Unsets flags value corresponding to string nick `nick` in `value`, if part of that flags.
If the flag is already unset, it will succeed without doing any changes.

Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
value otherwise.
<!-- impl FlagsClass::fn builder -->
Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
and building a `Value`.
<!-- impl FlagsClass::fn builder_with_value -->
Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
and building a `Value`. The `Value` is initialized with `value`.
<!-- struct FlagsValue -->
Representation of a single flags value of a `FlagsClass`.
<!-- impl FlagsValue::fn get_value -->
Get integer value corresponding to the value.
<!-- impl FlagsValue::fn get_name -->
Get name corresponding to the value.
<!-- impl FlagsValue::fn get_nick -->
Get nick corresponding to the value.
<!-- impl FlagsValue::fn to_value -->
Convert flags value to a `Value`.
<!-- impl FlagsValue::fn from_value -->
Convert flags values from a `Value`. This returns all flags that are set.
<!-- impl FlagsValue::fn get_class -->
Get `FlagsClass` to which the flags value belongs.
<!-- struct FlagsBuilder -->
Builder for conveniently setting/unsetting flags and returning a `Value`.

Example for getting a flags property, unsetting some flags and setting the updated flags on the
object again:

```ignore
let flags = obj.get_property("flags").unwrap();
let flags_class = FlagsClass::new(flags.type_()).unwrap();
let flags = flags_class.builder_with_value(flags).unwrap()
    .unset_by_nick("some-flag")
    .unset_by_nick("some-other-flag")
    .build()
    .unwrap();
obj.set_property("flags", &flags).unwrap();
```

If setting/unsetting any value fails, `build()` returns `None`.
<!-- struct FlagsBuilder::fn set -->
Sets flags corresponding to integer value `f`.
<!-- struct FlagsBuilder::fn set_by_name -->
Sets flags corresponding to string name `name`.
<!-- struct FlagsBuilder::fn set_by_nick -->
Sets flags corresponding to string nick `nick`.
<!-- struct FlagsBuilder::fn unset -->
Unsets flags corresponding to integer value `f`.
<!-- struct FlagsBuilder::fn unset_by_name -->
Unsets flags corresponding to string name `name`.
<!-- struct FlagsBuilder::fn unset_by_nick -->
Unsets flags corresponding to string nick `nick`.
<!-- struct FlagsBuilder::fn build -->
Converts to the final `Value`, unless any previous setting/unsetting of flags failed.
<!-- file error.rs -->
<!-- file_comment -->
`Error` binding and helper trait.
<!-- macro glib_wrapper!::struct Error -->
A generic error capable of representing various error domains (types).
<!-- impl Error::fn new -->
Creates an error with supplied error enum variant and message.
<!-- impl Error::fn is -->
Checks if the error domain matches `T`.
<!-- impl Error::fn kind -->
Tries to convert to a specific error enum.

Returns `Some` if the error belongs to the enum's error domain and
`None` otherwise.

# Examples

```ignore
if let Some(file_error) = error.kind::<FileError>() {
    match file_error {
        FileError::Exist => ...
        FileError::Isdir => ...
        ...
    }
}
```

```ignore
match error {
    Some(FileError::Exist) => ...
    Some(FileError::Isdir) => ...
    ...
}
```
<!-- trait ErrorDomain -->
`GLib` error domain.

This trait is implemented by error enums that represent error domains (types).
<!-- trait ErrorDomain::fn domain -->
Returns the quark identifying the error domain.

As returned from `g_some_error_quark`.
<!-- trait ErrorDomain::fn code -->
Gets the integer representation of the variant.
<!-- trait ErrorDomain::fn from -->
Tries to convert an integer code to an enum variant.

By convention, the `Failed` variant, if present, is a catch-all,
i.e. any unrecognized codes map to it.
<!-- struct BoolError -->
Generic error used for functions that fail without any further information
<!-- file lib.rs -->
<!-- file_comment -->
# **glib**, **gobject** and **gio** bindings for Rust

This library contains

- bindings to some essential GLib, GObject, GIO types and APIs,

- common building blocks used in both handmade and machine generated
bindings to GTK+ and other GLib-based libraries.

It is the foundation for higher level libraries with uniform Rusty (safe and
strongly typed) APIs. It avoids exposing GLib-specific data types where
possible and is not meant to provide comprehensive GLib bindings, which
would often amount to duplicating the Rust Standard Library or other utility
crates.

The library is a work in progress: expect missing functionality and breaking
changes.

# Dynamic typing

Most types in the GLib family have type identifiers
([`Type`](types/enum.Type.html)). Their corresponding Rust types implement
the [`StaticType`](types/trait.StaticType.html) trait.

Dynamically typed [`Value`](value/index.html) can carry values of any `T:
StaticType`.

[`Variant`](variant/index.html) can carry values of `T: StaticVariantType`.

# Errors

Errors are represented by [`Error`](error/struct.Error.html), which can
carry values from various [error
domains](error/trait.ErrorDomain.html#implementors) (such as
[`FileError`](enum.FileError.html)).

# Objects

Each class and interface has a corresponding smart pointer struct
representing an instance of that type (e.g. `Object` for `GObject`,
`gtk::Widget` for `GtkWidget`). They are reference counted and feature
interior mutability similarly to Rust's `Rc<RefCell<T>>` idiom.
Consequently, cloning objects is cheap and their methods never require
mutable borrows. Two smart pointers are equal iff they point to the same
object.

The root of the object hierarchy is [`Object`](object/struct.Object.html).
Inheritance and subtyping is denoted with the [`IsA`](object/trait.IsA.html)
marker trait. The [`Cast`](object/trait.Cast.html) trait enables upcasting
and downcasting.

Interfaces and non-leaf classes also have corresponding traits (e.g.
`ObjectExt` and `gtk::WidgetExt`), which are blanketly implemented for all
their subtypes.

# Under the hood

GLib-based libraries largely operate on pointers to various boxed or
reference counted structures so the bindings have to implement corresponding
smart pointers (wrappers), which encapsulate resource management and safety
checks. Such wrappers are defined via the
[`glib_wrapper!`](macro.glib_wrapper!.html) macro, which uses abstractions
defined in the [`wrapper`](wrapper/index.html), [`boxed`](boxed/index.html),
[`shared`](shared/index.html) and [`object`](object/index.html) modules.

The [`translate`](translate/index.html) module defines and partly implements
conversions between high level Rust types (including the aforementioned
wrappers) and their FFI counterparts.
<!-- file object.rs -->
<!-- file_comment -->
`IMPL` Object wrapper implementation and `Object` binding.
<!-- trait Cast -->
Upcasting and downcasting support.

Provides conversions up and down the class hierarchy tree.
<!-- trait Cast::fn upcast -->
Upcasts an object to a superclass or interface `T`.

*NOTE*: This statically checks at compile-time if casting is possible. It is not always
known at compile-time, whether a specific object implements an interface or not, in which case
`upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
is checking the types at runtime.

# Example

```ignore
let button = gtk::Button::new();
let widget = button.upcast::<gtk::Widget>();
```
<!-- trait Cast::fn downcast -->
Tries to downcast to a subclass or interface implementor `T`.

Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
otherwise.

*NOTE*: This statically checks at compile-time if casting is possible. It is not always
known at compile-time, whether a specific object implements an interface or not, in which case
`upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
is checking the types at runtime.

# Example

```ignore
let button = gtk::Button::new();
let widget = button.upcast::<gtk::Widget>();
assert!(widget.downcast::<gtk::Button>().is_ok());
```
<!-- trait Cast::fn is -->
Returns `true` if the object is an instance of (can be cast to) `T`.
<!-- trait Cast::fn dynamic_cast -->
Tries to cast to an object of type `T`. This handles upcasting, downcasting
and casting between interface and interface implementors. All checks are performed at
runtime, while `downcast` and `upcast` will do many checks at compile-time already.

It is not always known at compile-time, whether a specific object implements an interface or
not, and checking as to be performed at runtime.

Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
otherwise.

# Example

```ignore
let button = gtk::Button::new();
let widget = button.dynamic_cast::<gtk::Widget>();
assert!(widget.is_ok);
let widget = widget.unwrap();
assert!(widget.dynamic_cast::<gtk::Button>().is_ok());
```
<!-- trait IsA -->
Declares the "is a" relationship.

`Self` is said to implement `T`.

For instance, since originally `GtkWidget` is a subclass of `GObject` and
implements the `GtkBuildable` interface, `gtk::Widget` implements
`IsA<glib::Object>` and `IsA<gtk::Buildable>`.


The trait can only be implemented if the appropriate `ToGlibPtr`
implementations exist.

`T` always implements `IsA<T>`.
<!-- trait Downcast -->
Downcasts support.
<!-- trait Downcast::fn can_downcast -->
Checks if it's possible to downcast to `T`.

Returns `true` if the instance implements `T` and `false` otherwise.
<!-- trait Downcast::fn downcast -->
Tries to downcast to `T`.

Returns `Ok(T)` if the instance implements `T` and `Err(Self)` otherwise.
<!-- trait Downcast::fn downcast_unchecked -->
Downcasts to `T` unconditionally.

Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
<!-- macro glib_object_wrapper -->
Wrapper implementations for Object types. See `glib_wrapper!`.
<!-- file prelude.rs -->
<!-- file_comment -->
Traits and essential types intended for blanket imports.
<!-- file shared.rs -->
<!-- file_comment -->
`IMPL` Shared (reference counted) wrapper implementation.
<!-- macro glib_shared_wrapper -->
Wrapper implementations for shared types. See `glib_wrapper!`.
<!-- struct Shared -->
Encapsulates memory management logic for shared types.
<!-- file signal.rs -->
<!-- file_comment -->
`IMPL` Low level signal support.
<!-- struct Inhibit -->
Whether to propagate the signal to the default handler.

Don't inhibit default handlers without a reason, they're usually helpful.
<!-- file source.rs -->
<!-- struct SourceId -->
The id of a source that is returned by `idle_add` and `timeout_add`.

A value of 0 is a good default as it is never a valid source ID.
<!-- struct Continue -->
Continue calling the closure in the future iterations or drop it.

This is the return type of `idle_add` and `timeout_add` closures.

`Continue(true)` keeps the closure assigned, to be rerun when appropriate.

`Continue(false)` disconnects and drops it.
<!-- struct CallbackGuard -->
Unwinding propagation guard. Aborts the process if destroyed while
panicking.
<!-- fn idle_add -->
Adds a closure to be called by the default main loop when it's idle.

`func` will be called repeatedly until it returns `Continue(false)`.

The default main loop almost always is the main loop of the main thread.
Thus the closure is called on the main thread.
<!-- fn timeout_add -->
Adds a closure to be called by the default main loop at regular intervals
with millisecond granularity.

`func` will be called repeatedly every `interval` milliseconds until it
returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
be delayed by other events. Prefer `timeout_add_seconds` when millisecond
precision is not necessary.

The default main loop almost always is the main loop of the main thread.
Thus the closure is called on the main thread.
<!-- fn timeout_add_seconds -->
Adds a closure to be called by the default main loop at regular intervals
with second granularity.

`func` will be called repeatedly every `interval` seconds until it
returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
be delayed by other events.

The default main loop almost always is the main loop of the main thread.
Thus the closure is called on the main thread.
<!-- fn child_watch_add -->
Adds a closure to be called by the main loop the returned `Source` is attached to when a child
process exits.

`func` will be called when `pid` exits
<!-- fn unix_signal_add -->
Adds a closure to be called by the default main loop whenever a UNIX signal is raised.

`func` will be called repeatedly every time `signum` is raised until it
returns `Continue(false)`.

The default main loop almost always is the main loop of the main thread.
Thus the closure is called on the main thread.
<!-- fn source_remove -->
Removes the source with the given id `source_id` from the default main context.

It is a programmer error to attempt to remove a non-existent source.
Note: source id are reused.

For historical reasons, the native function always returns true, so we
ignore it here.
<!-- struct Priority -->
The priority of sources

<!-- fn idle_source_new -->
Adds a closure to be called by the main loop the return `Source` is attached to when it's idle.

`func` will be called repeatedly until it returns `Continue(false)`.
<!-- fn timeout_source_new -->
Adds a closure to be called by the main loop the returned `Source` is attached to at regular
intervals with millisecond granularity.

`func` will be called repeatedly every `interval` milliseconds until it
returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
be delayed by other events. Prefer `timeout_add_seconds` when millisecond
precision is not necessary.
<!-- fn timeout_source_new_seconds -->
Adds a closure to be called by the main loop the returned `Source` is attached to at regular
intervals with second granularity.

`func` will be called repeatedly every `interval` seconds until it
returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
be delayed by other events.
<!-- fn child_watch_source_new -->
Adds a closure to be called by the main loop the returned `Source` is attached to when a child
process exits.

`func` will be called when `pid` exits
<!-- fn unix_signal_source_new -->
Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
UNIX signal is raised.

`func` will be called repeatedly every time `signum` is raised until it
returns `Continue(false)`.
<!-- file translate.rs -->
<!-- file_comment -->
Translation between GLib/GLib-based FFI types and their Rust counterparts.

This module allows library bindings authors to decouple type translation
logic and use unified idioms at FFI boundaries. It also implements
translation of GLib core data types.

`FromGlib`, `from_glib` and `ToGlib` translate simple types like `bool`.

```ignore
    pub fn set_accept_focus(&self, accept_focus: bool) {
        unsafe { glib_ffi::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
    }

    pub fn get_accept_focus(&self) -> bool {
        unsafe { from_glib(glib_ffi::gdk_window_get_accept_focus(self.pointer)) }
    }
```

`ToGlibPtr`, `FromGlibPtrNone`, `FromGlibPtrFull` and `FromGlibPtrBorrow` work on `gpointer`s
and support different modes of ownership transfer.

```ignore
    fn get_title(&self) -> Option<String> {
        unsafe {
            let title = glib_ffi::gtk_window_get_title(self.pointer);
            from_glib_none(title)
        }
    }
```

Letting the foreign library borrow pointers from the Rust side often
requires having a temporary variable of an intermediate type (e.g. `CString`).
A `Stash` contains the temporary storage and a pointer into it that
is valid for the lifetime of the `Stash`. As the lifetime of the `Stash` returned
from `to_glib_none` is at least the enclosing statement, you can avoid explicitly
binding the stash in most cases and just take the pointer out of it:

```ignore
    pub fn set_icon_name(&self, name: &str) {
        unsafe {
            glib_ffi::gdk_window_set_icon_name(self.pointer, name.to_glib_none().0)
        }
    }
```
<!-- trait Ptr -->
A pointer
<!-- fn mut_override -->
Overrides pointer mutability.

Use when the C API should be specifying a const pointer but doesn't.
<!-- trait Uninitialized -->
A trait for creating an uninitialized value. Handy for receiving outparams.
<!-- trait Uninitialized::fn uninitialized -->
Returns an uninitialized value.
<!-- fn uninitialized -->
Returns an uninitialized value.
<!-- fn some_if -->
Returns `Some(val)` if the condition is true and `None` otherwise.
<!-- struct Stash -->
Helper type that stores temporary values used for translation.

`P` is the foreign type pointer and the first element of the tuple.

`T` is the Rust type that is translated.

The second element of the tuple is the temporary storage defined
by the implementation of `ToGlibPtr<P> for T`

Say you want to pass a `*mut GdkWindowAttr` to a foreign function. The `Stash`
will own a `GdkWindowAttr` and a `CString` that `GdkWindowAttr::title` points into.

```ignore
impl <'a> ToGlibPtr<'a, *mut glib_ffi::GdkWindowAttr> for WindowAttr {
    type Storage = (Box<glib_ffi::GdkWindowAttr>, Stash<'a, *const c_char, Option<String>>);

    fn to_glib_none(&'a self) -> Stash<*mut glib_ffi::GdkWindowAttr, WindowAttr> {
        let title = self.title.to_glib_none();

        let mut attrs = Box::new(glib_ffi::GdkWindowAttr {
            title: title.0,
            // ....
        });

        Stash(&mut *attrs, (attrs, title))
    }
}
```
<!-- trait ToGlib -->
Translate a simple type.
<!-- trait GlibPtrDefault -->
Provides the default pointer type to be used in some container conversions.

It's `*mut c_char` for `String`, `*mut GtkButton` for `gtk::Button`, etc.
<!-- trait ToGlibPtr -->
Translate to a pointer.
<!-- trait ToGlibPtr::fn to_glib_none -->
Transfer: none.

The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
<!-- trait ToGlibPtr::fn to_glib_container -->
Transfer: container.

We transfer the container ownership to the foreign library retaining
the elements ownership.
<!-- trait ToGlibPtr::fn to_glib_full -->
Transfer: full.

We transfer the ownership to the foreign library.
<!-- trait ToGlibPtrMut -->

Translate to a pointer with a mutable borrow.
<!-- trait ToGlibPtrMut::fn to_glib_none_mut -->
Transfer: none.

The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
<!-- trait FromGlib -->
Translate a simple type.
<!-- fn from_glib -->
Translate a simple type.
<!-- trait FromGlibPtrNone -->
Translate from a pointer type without taking ownership, transfer: none.
<!-- trait FromGlibPtrFull -->
Translate from a pointer type taking ownership, transfer: full.
<!-- trait FromGlibPtrBorrow -->
Translate from a pointer type by borrowing. Don't increase the refcount
<!-- fn from_glib_none -->
Translate from a pointer type, transfer: none.
<!-- fn from_glib_full -->
Translate from a pointer type, transfer: full (assume ownership).
<!-- fn from_glib_borrow -->
Translate from a pointer type, borrowing the pointer.
<!-- trait FromGlibContainer -->
Translate from a container.
<!-- trait FromGlibContainer::fn from_glib_none_num -->
Transfer: none.

`num` is the advised number of elements.
<!-- trait FromGlibContainer::fn from_glib_container_num -->
Transfer: container.

`num` is the advised number of elements.
<!-- trait FromGlibContainer::fn from_glib_full_num -->
Transfer: full.

`num` is the advised number of elements.
<!-- trait FromGlibPtrContainer -->
Translate from a container of pointers.
<!-- trait FromGlibPtrContainer::fn from_glib_none -->
Transfer: none.
<!-- trait FromGlibPtrContainer::fn from_glib_container -->
Transfer: container.
<!-- trait FromGlibPtrContainer::fn from_glib_full -->
Transfer: full.
<!-- file types.rs -->
<!-- file_comment -->
Runtime type information.
<!-- enum Type -->
A GLib or GLib-based library type
<!-- enum Type::variant Invalid, -->
An invalid `Type` used as error return value in some functions
<!-- enum Type::variant Unit, -->
The fundamental type corresponding to the unit type `()`
<!-- enum Type::variant I8, -->
The fundamental type corresponding to `i8`
<!-- enum Type::variant U8, -->
The fundamental type corresponding to `u8`
<!-- enum Type::variant Bool, -->
The fundamental type corresponding to `bool`
<!-- enum Type::variant I32, -->
The fundamental type corresponding to `i32`
<!-- enum Type::variant U32, -->
The fundamental type corresponding to `u32`
<!-- enum Type::variant ILong, -->
The fundamental type corresponding to C `long`
<!-- enum Type::variant ULong, -->
The fundamental type corresponding to C `unsigned long`
<!-- enum Type::variant I64, -->
The fundamental type corresponding to `i64`
<!-- enum Type::variant U64, -->
The fundamental type corresponding to `u64`
<!-- enum Type::variant F32, -->
The fundamental type corresponding to `f32`
<!-- enum Type::variant F64, -->
The fundamental type corresponding to `f64`
<!-- enum Type::variant String, -->
The fundamental type corresponding to `String`
<!-- enum Type::variant Pointer, -->
The fundamental type corresponding to a pointer
<!-- enum Type::variant Variant, -->
The fundamental type of GVariant
<!-- enum Type::variant BaseInterface, -->
The fundamental type from which all interfaces are derived
<!-- enum Type::variant BaseEnum, -->
The fundamental type from which all enumeration types are derived
<!-- enum Type::variant BaseFlags, -->
The fundamental type from which all flags types are derived
<!-- enum Type::variant BaseBoxed, -->
The fundamental type from which all boxed types are derived
<!-- enum Type::variant BaseParamSpec, -->
The fundamental type from which all `GParamSpec` types are derived
<!-- enum Type::variant BaseObject, -->
The fundamental type from which all objects are derived
<!-- enum Type::variant Other -->
A non-fundamental type identified by value of type `usize`
<!-- trait StaticType -->
Types that are supported by GLib dynamic typing.
<!-- trait StaticType::fn static_type -->
Returns the type identifier of `Self`.
<!-- file utils.rs -->
<!-- fn get_program_name -->
Same as [`get_prgname()`].

[`get_prgname()`]: fn.get_prgname.html
<!-- fn set_program_name -->
Same as [`set_prgname()`].

[`set_prgname()`]: fn.set_prgname.html
<!-- file value.rs -->
<!-- file_comment -->
`Value` binding and helper traits.

The type of a [`Value`](struct.Value.html) is dynamic in that it generally
isn't known at compile time but once created a `Value` can't change its
type.

[`TypedValue`](struct.TypedValue.html) has a statically known type and
dereferences to `Value` so it can be used everywhere `Value` references are
accepted.

Supported types are `bool`, `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `f32`,
`f64`, `String` and objects (`T: IsA<Object>`).

# Examples

```
use glib::prelude::*; // or `use gtk::prelude::*;`
use glib::{TypedValue, Value};

// Value and TypedValue implement From<&i32>, From<&str>
// and From<Option<&str>>. Another option is the `ToValue` trait.
let mut num = 10.to_value();
let mut hello = Value::from("Hello!");
let none: Option<&str> = None;
let str_none = Value::from(none.clone());
let typed_str_none = TypedValue::from(none);

// `is` tests the type of the value.
assert!(num.is::<i32>());
assert!(hello.is::<String>());

// `get` tries to get a value of specific type and returns None
// if the type doesn't match or the value is None.
assert_eq!(num.get(), Some(10));
assert_eq!(num.get::<String>(), None);
assert_eq!(hello.get(), Some(String::from("Hello!")));
assert_eq!(hello.get::<String>(), Some(String::from("Hello!")));
assert_eq!(str_none.get::<String>(), None);

// `typed` tries to convert a `Value` to `TypedValue`.
let mut typed_num = num.downcast::<i32>().unwrap();
let mut typed_hello = hello.downcast::<String>().unwrap();

// `str_none` is not an `i32`
assert!(str_none.downcast::<i32>().is_err());

// `get`
assert!(typed_hello.get().unwrap() == "Hello!");
assert!(typed_str_none.get() == None);

// Numeric types can't have value `None`, `get` always returns `Some`.
// Such types have `get_some`, which avoids unnecessary `unwrap`ping.
assert_eq!(typed_num.get().unwrap(), 10);
assert_eq!(typed_num.get_some(), 10);

// `set_none` sets the value to `None` if the type supports it.
typed_hello.set_none();
assert!(typed_hello.get().is_none());

// `set` takes an optional reference for types that support `None`.
typed_hello.set(Some("Hello again!"));
assert!(typed_hello.get().unwrap() == "Hello again!");

// `set_some` is the only setter for types that don't support `None`.
typed_num.set_some(&20);
assert_eq!(typed_num.get_some(), 20);
```
<!-- struct Value -->
A generic value capable of carrying various types.

Once created the type of the value can't be changed.

Some types (e.g. `String` and objects) support `None` values while others
(e.g. numeric types) don't.

See the [module documentation](index.html) for more details.
<!-- impl Value::fn downcast -->
Tries to downcast to a `TypedValue`.

Returns `Ok(TypedValue<T>)` if the value carries a type corresponding
to `T` and `Err(self)` otherwise.
<!-- impl Value::fn get -->
Tries to get a value of type `T`.

Returns `Some` if the type is correct and the value is not `None`.

This function doesn't distinguish between type mismatches and correctly
typed `None` values. Use `downcast` or `is` for that.
<!-- impl Value::fn is -->
Returns `true` if the type of the value corresponds to `T`.
<!-- impl Value::fn type_ -->
Returns the type of the value.
<!-- impl Value::fn type_transformable -->
Returns whether `Value`s of type `src` can be transformed to type `dst`.
<!-- struct TypedValue -->
A statically typed [`Value`](struct.Value.html).

It dereferences to `Value` and can be used everywhere `Value` references are
accepted.

See the [module documentation](index.html) for more details.
<!-- struct TypedValue::fn get -->
Returns the value.

Types that don't support a `None` value always return `Some`. See
`get_some`.
<!-- struct TypedValue::fn get_some -->
Returns the value.

This method is only available for types that don't support a `None`
value.
<!-- struct TypedValue::fn set -->
Sets the value.

This method is only available for types that support a `None` value.
<!-- struct TypedValue::fn set_none -->
Sets the value to `None`.

This method is only available for types that support a `None` value.
<!-- struct TypedValue::fn set_some -->
Sets the value.
<!-- trait ToValue -->
Converts to `Value`.
<!-- trait ToValue::fn to_value -->
Returns a `Value` clone of `self`.
<!-- trait ToValue::fn to_value_type -->
Returns the type identifer of `self`.

This is the type of the value to be returned by `to_value`.
<!-- trait FromValueOptional -->
Extracts a value.

Types that don't support a `None` value always return `Some`.
<!-- trait FromValue -->
Extracts a value.

Only implemented for types that don't support a `None` value.
<!-- trait SetValueOptional -->
Sets a value.

Only implemented for types that support a `None` value.
<!-- trait SetValue -->
Sets a value.
<!-- file variant.rs -->
<!-- file_comment -->
`Variant` binding and helper traits.

[`Variant`](struct.Variant.html) is an immutable dynamically-typed generic
container. Its type and value are defined at construction and never change.

`Variant` types are described by [`VariantType`](../struct.VariantType.html)
"type strings".

Although `GVariant` supports arbitrarily complex types, this binding is
currently limited to the basic ones: `bool`, `u8`, `i16`, `u16`, `i32`,
`u32`, `i64`, `u64`, `f64` and `&str`/`String`.

# Examples

```
use glib::prelude::*; // or `use gtk::prelude::*;`
use glib::Variant;

// Using the `ToVariant` trait.
let num = 10.to_variant();

// `is` tests the type of the value.
assert!(num.is::<i32>());

// `get` tries to extract the value.
assert_eq!(num.get::<i32>(), Some(10));
assert_eq!(num.get::<u32>(), None);

// `Variant` implements `From`
let hello = Variant::from("Hello!");

// `get_str` tries to borrow a string slice.
assert_eq!(hello.get_str(), Some("Hello!"));
assert_eq!(num.get_str(), None);
```
<!-- macro glib_wrapper!::struct Variant -->
A generic immutable value capable of carrying various types.

See the [module documentation](index.html) for more details.
<!-- impl Variant::fn type_ -->
Returns the type of the value.
<!-- impl Variant::fn is -->
Returns `true` if the type of the value corresponds to `T`.
<!-- impl Variant::fn get -->
Tries to extract a value of type `T`.

Returns `Some` if `T` matches the variant's type.
<!-- impl Variant::fn get_str -->
Tries to extract a `&str`.

Returns `Some` if the variant has a string type (`s`, `o` or `g` type
strings).
<!-- trait ToVariant -->
Converts to `Variant`.
<!-- trait ToVariant::fn to_variant -->
Returns a `Variant` clone of `self`.
<!-- trait FromVariant -->
Extracts a value.
<!-- trait FromVariant::fn from_variant -->
Tries to extract a value.

Returns `Some` if the variant's type matches `Self`.
<!-- trait StaticVariantType -->
Returns `VariantType` of `Self`.
<!-- trait StaticVariantType::fn static_variant_type -->
Returns the `VariantType` corresponding to `Self`.
<!-- file variant_type.rs -->
<!-- struct VariantType -->
Describes `Variant` types.

The `Variant` type system (based on the D-Bus one) describes types with
"type strings". `VariantType` is an owned immutable type string (you can
think of it as a `Box<str>` statically guaranteed to be a valid type
string), `&VariantTy` is a borrowed one (like `&str`).
<!-- impl VariantType::fn new -->
Tries to create a `VariantType` from a string slice.

Returns `Ok` if the string is a valid type string, `Err` otherwise.
<!-- struct VariantTy -->
Describes `Variant` types.

This is a borrowed counterpart of [`VariantType`](struct.VariantType.html).
Essentially it's a `str` statically guaranteed to be a valid type string.
<!-- impl VariantTy::fn new -->
Tries to create a `&VariantTy` from a string slice.

Returns `Ok` if the string is a valid type string, `Err` otherwise.
<!-- impl VariantTy::fn from_str_unchecked -->
Converts a type string into `&VariantTy` without any checks.
<!-- impl VariantTy::fn from_ptr -->
Creates `&VariantTy` with a wildcard lifetime from a `GVariantType`
pointer.
<!-- impl VariantTy::fn as_ptr -->
Returns a `GVariantType` pointer.
<!-- impl VariantTy::fn to_str -->
Converts to a string slice.
<!-- file wrapper.rs -->
<!-- file_comment -->
`IMPL` The `glib_wrapper!` macro and miscellaneous wrapper traits.
<!-- macro glib_wrapper -->
Defines a wrapper type and implements the appropriate traits.

The basic syntax is

```ignore
glib_wrapper! {
     Documentation
    pub struct $name($kind<$foreign>);

    match fn {
        $fn_name => /* a closure-like expression ,
        ...
    }
}
```

This creates a wrapper named `$name` around the foreign type `$foreign`
of `$kind` (one of `Boxed`, `Shared`, `Object`) using expressions from the `match fn`
block to implement type-specific low-level operations (the expression
will be evaluated in `unsafe` context).

### Boxed

Boxed records with single ownership.

```ignore
glib_wrapper! {
     Text buffer iterator
    pub struct TextIter(Boxed<ffi::GtkTextIter>);

    match fn {
        copy => |ptr| ffi::gtk_text_iter_copy(ptr),
        free => |ptr| ffi::gtk_text_iter_free(ptr),
    }
}
```

`copy`: `|*const $foreign| -> *mut $foreign` creates a copy of the value.

`free`: `|*mut $foreign|` frees the value.

`get_type`: `||` (optional) returns the `Type`, if any

### Shared

Records with reference counted shared ownership.

```ignore
glib_wrapper! {
     Object holding timing information for a single frame.
    pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);

    match fn {
        ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
        unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
    }
}
```

`ref`: `|*mut $foreign|` increases the refcount.

`unref`: `|*mut $foreign|` decreases the refcount.

`get_type`: `||` (optional) returns the `Type`, if any

### Object

Objects -- classes and interfaces.

```ignore
glib_wrapper! {
     Object representing an input device.
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        get_type => || ffi::gdk_device_get_type(),
    }
}
```

```ignore
glib_wrapper! {
     A container with just one child.
    pub struct Bin(Object<ffi::GtkBin>): Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_bin_get_type(),
    }
}
```

Implementing types from other crates requires specifying their FFI
counterparts as well:

```ignore
glib_wrapper! {
    pub struct Application(Object<ffi::GtkApplication>): [
        gio::Application => gio_ffi::GApplication,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_get_type(),
    }
}
```

`get_type: || -> GType` returns the type identifier of the class or interface.
<!-- trait Wrapper -->
A wrapper struct.
<!-- trait Wrapper::type GlibType -->
Foreign type represented by the struct.
