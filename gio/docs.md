<!-- file socket.rs -->
<!-- trait IntoRawFd -->
This is stub for doc generation
[Actual docs](https://doc.rust-lang.org/std/os/unix/io/trait.IntoRawFd.html)
<!-- trait FromRawFd -->
This is stub for doc generation
[Actual docs](https://doc.rust-lang.org/std/os/unix/io/trait.FromRawFd.html)
<!-- trait IntoRawSocket -->
This is stub for doc generation
[Actual docs](https://doc.rust-lang.org/std/os/windows/io/trait.IntoRawSocket.html)
<!-- trait FromRawSocket -->
This is stub for doc generation
[Actual docs](https://doc.rust-lang.org/std/os/windows/io/trait.FromRawSocket.html)
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

# Safety

As stated, this function takes ownership of the file descriptor on successful
creation of a `Socket`.  You must ensure you do not `close()` or perform other
actions on the file descriptor which could disrupt the `Socket`'s behaviour.
<!-- file unix_input_stream.rs -->
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

# Safety

If `close_fd` is `false` then the caller is responsible for dealing with the `fd`
once the `UnixInputStream` is cleaned up.  It is important that the caller not
close the `fd` before then.  If `close_fd` is `true` then instead the caller must
not close the `fd` as the `UnixInputStream` has taken ownership of it and ideally
the caller must not do any direct operations on `fd` at all subsequent to calling
this function.
<!-- trait UnixInputStreamExtManual::fn set_close_fd -->
Sets whether the file descriptor of `self` shall be closed
when the stream is closed.
## `close_fd`
`true` to close the file descriptor when done

# Safety

If by calling this, the value of `close_fd` changes in the underlying `UnixInputStream`
then it is essential that the caller subsequently obeys the safety information
from [`UnixInputStream::new()`](struct.UnixInputStream.html#method.new)
<!-- file unix_output_stream.rs -->
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

# Safety

If `close_fd` is `false` then the caller is responsible for dealing with the `fd`
once the `UnixOutputStream` is cleaned up.  It is important that the caller not
close the `fd` before then.  If `close_fd` is `true` then instead the caller must
not close the `fd` as the `UnixOutputStream` has taken ownership of it and ideally
the caller must not do any direct operations on `fd` at all subsequent to calling
this function.
<!-- trait UnixOutputStreamExtManual::fn set_close_fd -->
Sets whether the file descriptor of `self` shall be closed
when the stream is closed.
## `close_fd`
`true` to close the file descriptor when done

# Safety

If by calling this, the value of `close_fd` changes in the underlying `UnixOutputStream`
then it is essential that the caller subsequently obeys the safety information
from [`UnixOutputStream::new()`](struct.UnixOutputStream.html#method.new)
