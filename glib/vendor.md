<!-- file * -->
<!-- struct Bytes -->
A simple refcounted data type representing an immutable sequence of zero or
more bytes from an unspecified origin.

The purpose of a `Bytes` is to keep the memory region that it holds
alive for as long as anyone holds a reference to the bytes. When
the last reference count is dropped, the memory is released. Multiple
unrelated callers can use byte data in the `Bytes` without coordinating
their activities, resting assured that the byte data will not change or
move while they hold a reference.

A `Bytes` can come from many different origins that may have
different procedures for freeing the memory region. Examples are
memory from `g_malloc`, from memory slices, from a `MappedFile` or
memory from other allocators.

`Bytes` work well as keys in `HashTable`. Use `Bytes::equal` and
`Bytes::hash` as parameters to `HashTable::new` or `HashTable::new_full`.
`Bytes` can also be used as keys in a `Tree` by passing the `Bytes::compare`
function to `Tree::new`.

The data pointed to by this bytes must not be modified. For a mutable
array of bytes see `ByteArray`. Use `Bytes::unref_to_array` to create a
mutable array for a `Bytes` sequence. To create an immutable `Bytes` from
a mutable `ByteArray`, use the `ByteArray::free_to_bytes` function.
<!-- impl Bytes::fn new -->
Creates a new `Bytes` from `data`.

`data` is copied. If `size` is 0, `data` may be `None`.
## `data`

 the data to be used for the bytes
## `size`
the size of `data`

# Returns

a new `Bytes`
<!-- impl Bytes::fn new_static -->
Creates a new `Bytes` from static data.

`data` must be static (ie: never modified or freed). It may be `None` if `size`
is 0.
## `data`

 the data to be used for the bytes
## `size`
the size of `data`

# Returns

a new `Bytes`
<!-- impl Bytes::fn new_take -->
Creates a new `Bytes` from `data`.

After this call, `data` belongs to the bytes and may no longer be
modified by the caller. `g_free` will be called on `data` when the
bytes is no longer in use. Because of this `data` must have been created by
a call to `g_malloc`, `g_malloc0` or `g_realloc` or by one of the many
functions that wrap these calls (such as `g_new`, `g_strdup`, etc).

For creating `Bytes` with memory from other allocators, see
`Bytes::new_with_free_func`.

`data` may be `None` if `size` is 0.
## `data`

 the data to be used for the bytes
## `size`
the size of `data`

# Returns

a new `Bytes`
<!-- impl Bytes::fn new_with_free_func -->
Creates a `Bytes` from `data`.

When the last reference is dropped, `free_func` will be called with the
`user_data` argument.

`data` must not be modified after this call is made until `free_func` has
been called to indicate that the bytes is no longer in use.

`data` may be `None` if `size` is 0.
## `data`

 the data to be used for the bytes
## `size`
the size of `data`
## `free_func`
the function to call to release the data
## `user_data`
data to pass to `free_func`

# Returns

a new `Bytes`
<!-- impl Bytes::fn compare -->
Compares the two `Bytes` values.

This function can be used to sort GBytes instances in lexicographical order.

If `self` and `bytes2` have different length but the shorter one is a
prefix of the longer one then the shorter one is considered to be less than
the longer one. Otherwise the first byte where both differ is used for
comparison. If `self` has a smaller value at that position it is
considered less, otherwise greater than `bytes2`.
## `bytes2`
a pointer to a `Bytes` to compare with `self`

# Returns

a negative value if `self` is less than `bytes2`, a positive value
 if `self` is greater than `bytes2`, and zero if `self` is equal to
 `bytes2`
<!-- impl Bytes::fn equal -->
Compares the two `Bytes` values being pointed to and returns
`true` if they are equal.

This function can be passed to `HashTable::new` as the `key_equal_func`
parameter, when using non-`None` `Bytes` pointers as keys in a `HashTable`.
## `bytes2`
a pointer to a `Bytes` to compare with `self`

# Returns

`true` if the two keys match.
<!-- impl Bytes::fn get_data -->
Get the byte data in the `Bytes`. This data should not be modified.

This function will always return the same pointer for a given `Bytes`.

`None` may be returned if `size` is 0. This is not guaranteed, as the `Bytes`
may represent an empty string with `data` non-`None` and `size` as 0. `None` will
not be returned if `size` is non-zero.
## `size`
location to return size of byte data

# Returns


 a pointer to the byte data, or `None`
<!-- impl Bytes::fn get_size -->
Get the size of the byte data in the `Bytes`.

This function will always return the same value for a given `Bytes`.

# Returns

the size
<!-- impl Bytes::fn hash -->
Creates an integer hash code for the byte data in the `Bytes`.

This function can be passed to `HashTable::new` as the `key_hash_func`
parameter, when using non-`None` `Bytes` pointers as keys in a `HashTable`.

# Returns

a hash value corresponding to the key.
<!-- impl Bytes::fn new_from_bytes -->
Creates a `Bytes` which is a subsection of another `Bytes`. The `offset` +
`length` may not be longer than the size of `self`.

A reference to `self` will be held by the newly created `Bytes` until
the byte data is no longer needed.

Since 2.56, if `offset` is 0 and `length` matches the size of `self`, then
`self` will be returned with the reference count incremented by 1. If `self`
is a slice of another `Bytes`, then the resulting `Bytes` will reference
the same `Bytes` instead of `self`. This allows consumers to simplify the
usage of `Bytes` when asynchronously writing to streams.
## `offset`
offset which subsection starts at
## `length`
length of subsection

# Returns

a new `Bytes`
<!-- impl Bytes::fn ref -->
Increase the reference count on `self`.

# Returns

the `Bytes`
<!-- impl Bytes::fn unref -->
Releases a reference on `self`. This may result in the bytes being
freed. If `self` is `None`, it will return immediately.
<!-- impl Bytes::fn unref_to_array -->
Unreferences the bytes, and returns a new mutable `ByteArray` containing
the same byte data.

As an optimization, the byte data is transferred to the array without copying
if this was the last reference to bytes and bytes was created with
`Bytes::new`, `Bytes::new_take` or `ByteArray::free_to_bytes`. In all
other cases the data is copied.

# Returns

a new mutable `ByteArray` containing the same byte data
<!-- impl Bytes::fn unref_to_data -->
Unreferences the bytes, and returns a pointer the same byte data
contents.

As an optimization, the byte data is returned without copying if this was
the last reference to bytes and bytes was created with `Bytes::new`,
`Bytes::new_take` or `ByteArray::free_to_bytes`. In all other cases the
data is copied.
## `size`
location to place the length of the returned data

# Returns

a pointer to the same byte data, which should be
 freed with `g_free`
<!-- struct Checksum -->
An opaque structure representing a checksumming operation.
To create a new GChecksum, use `Checksum::new`. To free
a GChecksum, use `Checksum::free`.
<!-- impl Checksum::fn new -->
Creates a new `Checksum`, using the checksum algorithm `checksum_type`.
If the `checksum_type` is not known, `None` is returned.
A `Checksum` can be used to compute the checksum, or digest, of an
arbitrary binary blob, using different hashing algorithms.

A `Checksum` works by feeding a binary blob through `Checksum::update`
until there is data to be checked; the digest can then be extracted
using `Checksum::get_string`, which will return the checksum as a
hexadecimal string; or `Checksum::get_digest`, which will return a
vector of raw bytes. Once either `Checksum::get_string` or
`Checksum::get_digest` have been called on a `Checksum`, the checksum
will be closed and it won't be possible to call `Checksum::update`
on it anymore.
## `checksum_type`
the desired type of checksum

# Returns

the newly created `Checksum`, or `None`.
 Use `Checksum::free` to free the memory allocated by it.
<!-- impl Checksum::fn copy -->
Copies a `Checksum`. If `self` has been closed, by calling
`Checksum::get_string` or `Checksum::get_digest`, the copied
checksum will be closed as well.

# Returns

the copy of the passed `Checksum`. Use `Checksum::free`
 when finished using it.
<!-- impl Checksum::fn free -->
Frees the memory allocated for `self`.
<!-- impl Checksum::fn get_digest -->
Gets the digest from `self` as a raw binary vector and places it
into `buffer`. The size of the digest depends on the type of checksum.

Once this function has been called, the `Checksum` is closed and can
no longer be updated with `Checksum::update`.
## `buffer`
output buffer
## `digest_len`
an inout parameter. The caller initializes it to the size of `buffer`.
 After the call it contains the length of the digest.
<!-- impl Checksum::fn get_string -->
Gets the digest as an hexadecimal string.

Once this function has been called the `Checksum` can no longer be
updated with `Checksum::update`.

The hexadecimal characters will be lower case.

# Returns

the hexadecimal representation of the checksum. The
 returned string is owned by the checksum and should not be modified
 or freed.
<!-- impl Checksum::fn reset -->
Resets the state of the `self` back to its initial state.
<!-- impl Checksum::fn update -->
Feeds `data` into an existing `Checksum`. The checksum must still be
open, that is `Checksum::get_string` or `Checksum::get_digest` must
not have been called on `self`.
## `data`
buffer used to compute the checksum
## `length`
size of the buffer, or -1 if it is a null-terminated string.
<!-- impl Checksum::fn type_get_length -->
Gets the length in bytes of digests of type `checksum_type`
## `checksum_type`
a `ChecksumType`

# Returns

the checksum length, or -1 if `checksum_type` is
not supported.
<!-- enum ChecksumType -->
The hashing algorithm to be used by `Checksum` when performing the
digest of some data.

Note that the `ChecksumType` enumeration may be extended at a later
date to include new hashing algorithm types.
<!-- enum ChecksumType::variant Md5 -->
Use the MD5 hashing algorithm
<!-- enum ChecksumType::variant Sha1 -->
Use the SHA-1 hashing algorithm
<!-- enum ChecksumType::variant Sha256 -->
Use the SHA-256 hashing algorithm
<!-- enum ChecksumType::variant Sha512 -->
Use the SHA-512 hashing algorithm (Since: 2.36)
<!-- enum ChecksumType::variant Sha384 -->
Use the SHA-384 hashing algorithm (Since: 2.51)
<!-- enum DateMonth -->
Enumeration representing a month; values are `DateMonth::January`,
`DateMonth::February`, etc. `DateMonth::BadMonth` is the invalid value.
<!-- enum DateMonth::variant BadMonth -->
invalid value
<!-- enum DateMonth::variant January -->
January
<!-- enum DateMonth::variant February -->
February
<!-- enum DateMonth::variant March -->
March
<!-- enum DateMonth::variant April -->
April
<!-- enum DateMonth::variant May -->
May
<!-- enum DateMonth::variant June -->
June
<!-- enum DateMonth::variant July -->
July
<!-- enum DateMonth::variant August -->
August
<!-- enum DateMonth::variant September -->
September
<!-- enum DateMonth::variant October -->
October
<!-- enum DateMonth::variant November -->
November
<!-- enum DateMonth::variant December -->
December
<!-- struct DateTime -->
`GDateTime` is an opaque structure whose members
cannot be accessed directly.
<!-- impl DateTime::fn new -->
Creates a new `DateTime` corresponding to the given date and time in
the time zone `tz`.

The `year` must be between 1 and 9999, `month` between 1 and 12 and `day`
between 1 and 28, 29, 30 or 31 depending on the month and the year.

`hour` must be between 0 and 23 and `minute` must be between 0 and 59.

`seconds` must be at least 0.0 and must be strictly less than 60.0.
It will be rounded down to the nearest microsecond.

If the given time is not representable in the given time zone (for
example, 02:30 on March 14th 2010 in Toronto, due to daylight savings
time) then the time will be rounded up to the nearest existing time
(in this case, 03:00). If this matters to you then you should verify
the return value for containing the same as the numbers you gave.

In the case that the given time is ambiguous in the given time zone
(for example, 01:30 on November 7th 2010 in Toronto, due to daylight
savings time) then the time falling within standard (ie:
non-daylight) time is taken.

It not considered a programmer error for the values to this function
to be out of range, but in the case that they are, the function will
return `None`.

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `tz`
a `TimeZone`
## `year`
the year component of the date
## `month`
the month component of the date
## `day`
the day component of the date
## `hour`
the hour component of the date
## `minute`
the minute component of the date
## `seconds`
the number of seconds past the minute

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_from_iso8601 -->
Creates a `DateTime` corresponding to the given
[ISO 8601 formatted string](https://en.wikipedia.org/wiki/ISO_8601)
`text`. ISO 8601 strings of the form `<date>``<sep>``<time>``<tz>` are supported.

`<sep>` is the separator and can be either 'T', 't' or ' '.

`<date>` is in the form:

- `YYYY-MM-DD` - Year/month/day, e.g. 2016-08-24.
- `YYYYMMDD` - Same as above without dividers.
- `YYYY-DDD` - Ordinal day where DDD is from 001 to 366, e.g. 2016-237.
- `YYYYDDD` - Same as above without dividers.
- `YYYY-Www-D` - Week day where ww is from 01 to 52 and D from 1-7,
 e.g. 2016-W34-3.
- `YYYYWwwD` - Same as above without dividers.

`<time>` is in the form:

- `hh:mm:ss(.sss)` - Hours, minutes, seconds (subseconds), e.g. 22:10:42.123.
- `hhmmss(.sss)` - Same as above without dividers.

`<tz>` is an optional timezone suffix of the form:

- `Z` - UTC.
- `+hh:mm` or `-hh:mm` - Offset from UTC in hours and minutes, e.g. +12:00.
- `+hh` or `-hh` - Offset from UTC in hours, e.g. +12.

If the timezone is not provided in `text` it must be provided in `default_tz`
(this field is otherwise ignored).

This call can fail (returning `None`) if `text` is not a valid ISO 8601
formatted string.

You should release the return value by calling `DateTime::unref`
when you are done with it.

Feature: `v2_56`

## `text`
an ISO 8601 formatted time string.
## `default_tz`
a `TimeZone` to use if the text doesn't contain a
 timezone, or `None`.

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_from_timeval_local -->
Creates a `DateTime` corresponding to the given `TimeVal` `tv` in the
local time zone.

The time contained in a `TimeVal` is always stored in the form of
seconds elapsed since 1970-01-01 00:00:00 UTC, regardless of the
local time offset.

This call can fail (returning `None`) if `tv` represents a time outside
of the supported range of `DateTime`.

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `tv`
a `TimeVal`

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_from_timeval_utc -->
Creates a `DateTime` corresponding to the given `TimeVal` `tv` in UTC.

The time contained in a `TimeVal` is always stored in the form of
seconds elapsed since 1970-01-01 00:00:00 UTC.

This call can fail (returning `None`) if `tv` represents a time outside
of the supported range of `DateTime`.

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `tv`
a `TimeVal`

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_from_unix_local -->
Creates a `DateTime` corresponding to the given Unix time `t` in the
local time zone.

Unix time is the number of seconds that have elapsed since 1970-01-01
00:00:00 UTC, regardless of the local time offset.

This call can fail (returning `None`) if `t` represents a time outside
of the supported range of `DateTime`.

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `t`
the Unix time

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_from_unix_utc -->
Creates a `DateTime` corresponding to the given Unix time `t` in UTC.

Unix time is the number of seconds that have elapsed since 1970-01-01
00:00:00 UTC.

This call can fail (returning `None`) if `t` represents a time outside
of the supported range of `DateTime`.

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `t`
the Unix time

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_local -->
Creates a new `DateTime` corresponding to the given date and time in
the local time zone.

This call is equivalent to calling `DateTime::new` with the time
zone returned by `TimeZone::new_local`.
## `year`
the year component of the date
## `month`
the month component of the date
## `day`
the day component of the date
## `hour`
the hour component of the date
## `minute`
the minute component of the date
## `seconds`
the number of seconds past the minute

# Returns

a `DateTime`, or `None`
<!-- impl DateTime::fn new_now -->
Creates a `DateTime` corresponding to this exact instant in the given
time zone `tz`. The time is as accurate as the system allows, to a
maximum accuracy of 1 microsecond.

This function will always succeed unless the system clock is set to
truly insane values (or unless GLib is still being used after the
year 9999).

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `tz`
a `TimeZone`

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_now_local -->
Creates a `DateTime` corresponding to this exact instant in the local
time zone.

This is equivalent to calling `DateTime::new_now` with the time
zone returned by `TimeZone::new_local`.

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_now_utc -->
Creates a `DateTime` corresponding to this exact instant in UTC.

This is equivalent to calling `DateTime::new_now` with the time
zone returned by `TimeZone::new_utc`.

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn new_utc -->
Creates a new `DateTime` corresponding to the given date and time in
UTC.

This call is equivalent to calling `DateTime::new` with the time
zone returned by `TimeZone::new_utc`.
## `year`
the year component of the date
## `month`
the month component of the date
## `day`
the day component of the date
## `hour`
the hour component of the date
## `minute`
the minute component of the date
## `seconds`
the number of seconds past the minute

# Returns

a `DateTime`, or `None`
<!-- impl DateTime::fn add -->
Creates a copy of `self` and adds the specified timespan to the copy.
## `timespan`
a `TimeSpan`

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_days -->
Creates a copy of `self` and adds the specified number of days to the
copy. Add negative values to subtract days.
## `days`
the number of days

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_full -->
Creates a new `DateTime` adding the specified values to the current date and
time in `self`. Add negative values to subtract.
## `years`
the number of years to add
## `months`
the number of months to add
## `days`
the number of days to add
## `hours`
the number of hours to add
## `minutes`
the number of minutes to add
## `seconds`
the number of seconds to add

# Returns

the newly created `DateTime` that should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_hours -->
Creates a copy of `self` and adds the specified number of hours.
Add negative values to subtract hours.
## `hours`
the number of hours to add

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_minutes -->
Creates a copy of `self` adding the specified number of minutes.
Add negative values to subtract minutes.
## `minutes`
the number of minutes to add

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_months -->
Creates a copy of `self` and adds the specified number of months to the
copy. Add negative values to subtract months.

The day of the month of the resulting `DateTime` is clamped to the number
of days in the updated calendar month. For example, if adding 1 month to
31st January 2018, the result would be 28th February 2018. In 2020 (a leap
year), the result would be 29th February.
## `months`
the number of months

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_seconds -->
Creates a copy of `self` and adds the specified number of seconds.
Add negative values to subtract seconds.
## `seconds`
the number of seconds to add

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_weeks -->
Creates a copy of `self` and adds the specified number of weeks to the
copy. Add negative values to subtract weeks.
## `weeks`
the number of weeks

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn add_years -->
Creates a copy of `self` and adds the specified number of years to the
copy. Add negative values to subtract years.

As with `DateTime::add_months`, if the resulting date would be 29th
February on a non-leap year, the day will be clamped to 28th February.
## `years`
the number of years

# Returns

the newly created `DateTime` which should be freed with
 `DateTime::unref`.
<!-- impl DateTime::fn difference -->
Calculates the difference in time between `self` and `begin`. The
`TimeSpan` that is returned is effectively `self` - `begin` (ie:
positive if the first parameter is larger).
## `begin`
a `DateTime`

# Returns

the difference between the two `DateTime`, as a time
 span expressed in microseconds.
<!-- impl DateTime::fn format -->
Creates a newly allocated string representing the requested `format`.

The format strings understood by this function are a subset of the
`strftime` format language as specified by C99. The \%D, \%U and \%W
conversions are not supported, nor is the 'E' modifier. The GNU
extensions \%k, \%l, \%s and \%P are supported, however, as are the
'0', '_' and '-' modifiers.

In contrast to `strftime`, this function always produces a UTF-8
string, regardless of the current locale. Note that the rendering of
many formats is locale-dependent and may not match the `strftime`
output exactly.

The following format specifiers are supported:

- \%a: the abbreviated weekday name according to the current locale
- \%A: the full weekday name according to the current locale
- \%b: the abbreviated month name according to the current locale
- \%B: the full month name according to the current locale
- \%c: the preferred date and time representation for the current locale
- \%C: the century number (year/100) as a 2-digit integer (00-99)
- \%d: the day of the month as a decimal number (range 01 to 31)
- \%e: the day of the month as a decimal number (range 1 to 31)
- \%F: equivalent to `%Y-%m-%d` (the ISO 8601 date format)
- \%g: the last two digits of the ISO 8601 week-based year as a
 decimal number (00-99). This works well with \%V and \%u.
- \%G: the ISO 8601 week-based year as a decimal number. This works
 well with \%V and \%u.
- \%h: equivalent to \%b
- \%H: the hour as a decimal number using a 24-hour clock (range 00 to 23)
- \%I: the hour as a decimal number using a 12-hour clock (range 01 to 12)
- \%j: the day of the year as a decimal number (range 001 to 366)
- \%k: the hour (24-hour clock) as a decimal number (range 0 to 23);
 single digits are preceded by a blank
- \%l: the hour (12-hour clock) as a decimal number (range 1 to 12);
 single digits are preceded by a blank
- \%m: the month as a decimal number (range 01 to 12)
- \%M: the minute as a decimal number (range 00 to 59)
- \%p: either "AM" or "PM" according to the given time value, or the
 corresponding strings for the current locale. Noon is treated as
 "PM" and midnight as "AM".
- \%P: like \%p but lowercase: "am" or "pm" or a corresponding string for
 the current locale
- \%r: the time in a.m. or p.m. notation
- \%R: the time in 24-hour notation (\%H:\%M)
- \%s: the number of seconds since the Epoch, that is, since 1970-01-01
 00:00:00 UTC
- \%S: the second as a decimal number (range 00 to 60)
- \%t: a tab character
- \%T: the time in 24-hour notation with seconds (\%H:\%M:\%S)
- \%u: the ISO 8601 standard day of the week as a decimal, range 1 to 7,
 Monday being 1. This works well with \%G and \%V.
- \%V: the ISO 8601 standard week number of the current year as a decimal
 number, range 01 to 53, where week 1 is the first week that has at
 least 4 days in the new year. See `DateTime::get_week_of_year`.
 This works well with \%G and \%u.
- \%w: the day of the week as a decimal, range 0 to 6, Sunday being 0.
 This is not the ISO 8601 standard format -- use \%u instead.
- \%x: the preferred date representation for the current locale without
 the time
- \%X: the preferred time representation for the current locale without
 the date
- \%y: the year as a decimal number without the century
- \%Y: the year as a decimal number including the century
- \%z: the time zone as an offset from UTC (+hhmm)
- \%:z: the time zone as an offset from UTC (+hh:mm).
 This is a gnulib `strftime` extension. Since: 2.38
- \%::z: the time zone as an offset from UTC (+hh:mm:ss). This is a
 gnulib `strftime` extension. Since: 2.38
- \%:::z: the time zone as an offset from UTC, with : to necessary
 precision (e.g., -04, +05:30). This is a gnulib `strftime` extension. Since: 2.38
- \%Z: the time zone or name or abbreviation
- \%\%: a literal \% character

Some conversion specifications can be modified by preceding the
conversion specifier by one or more modifier characters. The
following modifiers are supported for many of the numeric
conversions:

- O: Use alternative numeric symbols, if the current locale supports those.
- _: Pad a numeric result with spaces. This overrides the default padding
 for the specifier.
- -: Do not pad a numeric result. This overrides the default padding
 for the specifier.
- 0: Pad a numeric result with zeros. This overrides the default padding
 for the specifier.

Additionally, when O is used with B, b, or h, it produces the alternative
form of a month name. The alternative form should be used when the month
name is used without a day number (e.g., standalone). It is required in
some languages (Baltic, Slavic, Greek, and more) due to their grammatical
rules. For other languages there is no difference. \%OB is a GNU and BSD
`strftime` extension expected to be added to the future POSIX specification,
\%Ob and \%Oh are GNU `strftime` extensions. Since: 2.56
## `format`
a valid UTF-8 string, containing the format for the
 `DateTime`

# Returns

a newly allocated string formatted to the requested format
 or `None` in the case that there was an error (such as a format specifier
 not being supported in the current locale). The string
 should be freed with `g_free`.
<!-- impl DateTime::fn get_day_of_month -->
Retrieves the day of the month represented by `self` in the gregorian
calendar.

# Returns

the day of the month
<!-- impl DateTime::fn get_day_of_week -->
Retrieves the ISO 8601 day of the week on which `self` falls (1 is
Monday, 2 is Tuesday... 7 is Sunday).

# Returns

the day of the week
<!-- impl DateTime::fn get_day_of_year -->
Retrieves the day of the year represented by `self` in the Gregorian
calendar.

# Returns

the day of the year
<!-- impl DateTime::fn get_hour -->
Retrieves the hour of the day represented by `self`

# Returns

the hour of the day
<!-- impl DateTime::fn get_microsecond -->
Retrieves the microsecond of the date represented by `self`

# Returns

the microsecond of the second
<!-- impl DateTime::fn get_minute -->
Retrieves the minute of the hour represented by `self`

# Returns

the minute of the hour
<!-- impl DateTime::fn get_month -->
Retrieves the month of the year represented by `self` in the Gregorian
calendar.

# Returns

the month represented by `self`
<!-- impl DateTime::fn get_second -->
Retrieves the second of the minute represented by `self`

# Returns

the second represented by `self`
<!-- impl DateTime::fn get_seconds -->
Retrieves the number of seconds since the start of the last minute,
including the fractional part.

# Returns

the number of seconds
<!-- impl DateTime::fn get_timezone -->
Get the time zone for this `self`.

Feature: `v2_58`


# Returns

the time zone
<!-- impl DateTime::fn get_timezone_abbreviation -->
Determines the time zone abbreviation to be used at the time and in
the time zone of `self`.

For example, in Toronto this is currently "EST" during the winter
months and "EDT" during the summer months when daylight savings
time is in effect.

# Returns

the time zone abbreviation. The returned
 string is owned by the `DateTime` and it should not be
 modified or freed
<!-- impl DateTime::fn get_utc_offset -->
Determines the offset to UTC in effect at the time and in the time
zone of `self`.

The offset is the number of microseconds that you add to UTC time to
arrive at local time for the time zone (ie: negative numbers for time
zones west of GMT, positive numbers for east).

If `self` represents UTC time, then the offset is always zero.

# Returns

the number of microseconds that should be added to UTC to
 get the local time
<!-- impl DateTime::fn get_week_numbering_year -->
Returns the ISO 8601 week-numbering year in which the week containing
`self` falls.

This function, taken together with `DateTime::get_week_of_year` and
`DateTime::get_day_of_week` can be used to determine the full ISO
week date on which `self` falls.

This is usually equal to the normal Gregorian year (as returned by
`DateTime::get_year`), except as detailed below:

For Thursday, the week-numbering year is always equal to the usual
calendar year. For other days, the number is such that every day
within a complete week (Monday to Sunday) is contained within the
same week-numbering year.

For Monday, Tuesday and Wednesday occurring near the end of the year,
this may mean that the week-numbering year is one greater than the
calendar year (so that these days have the same week-numbering year
as the Thursday occurring early in the next year).

For Friday, Saturday and Sunday occurring near the start of the year,
this may mean that the week-numbering year is one less than the
calendar year (so that these days have the same week-numbering year
as the Thursday occurring late in the previous year).

An equivalent description is that the week-numbering year is equal to
the calendar year containing the majority of the days in the current
week (Monday to Sunday).

Note that January 1 0001 in the proleptic Gregorian calendar is a
Monday, so this function never returns 0.

# Returns

the ISO 8601 week-numbering year for `self`
<!-- impl DateTime::fn get_week_of_year -->
Returns the ISO 8601 week number for the week containing `self`.
The ISO 8601 week number is the same for every day of the week (from
Moday through Sunday). That can produce some unusual results
(described below).

The first week of the year is week 1. This is the week that contains
the first Thursday of the year. Equivalently, this is the first week
that has more than 4 of its days falling within the calendar year.

The value 0 is never returned by this function. Days contained
within a year but occurring before the first ISO 8601 week of that
year are considered as being contained in the last week of the
previous year. Similarly, the final days of a calendar year may be
considered as being part of the first ISO 8601 week of the next year
if 4 or more days of that week are contained within the new year.

# Returns

the ISO 8601 week number for `self`.
<!-- impl DateTime::fn get_year -->
Retrieves the year represented by `self` in the Gregorian calendar.

# Returns

the year represented by `self`
<!-- impl DateTime::fn get_ymd -->
Retrieves the Gregorian day, month, and year of a given `DateTime`.
## `year`
the return location for the gregorian year, or `None`.
## `month`
the return location for the month of the year, or `None`.
## `day`
the return location for the day of the month, or `None`.
<!-- impl DateTime::fn is_daylight_savings -->
Determines if daylight savings time is in effect at the time and in
the time zone of `self`.

# Returns

`true` if daylight savings time is in effect
<!-- impl DateTime::fn ref -->
Atomically increments the reference count of `self` by one.

# Returns

the `DateTime` with the reference count increased
<!-- impl DateTime::fn to_local -->
Creates a new `DateTime` corresponding to the same instant in time as
`self`, but in the local time zone.

This call is equivalent to calling `DateTime::to_timezone` with the
time zone returned by `TimeZone::new_local`.

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn to_timeval -->
Stores the instant in time that `self` represents into `tv`.

The time contained in a `TimeVal` is always stored in the form of
seconds elapsed since 1970-01-01 00:00:00 UTC, regardless of the time
zone associated with `self`.

On systems where 'long' is 32bit (ie: all 32bit systems and all
Windows systems), a `TimeVal` is incapable of storing the entire
range of values that `DateTime` is capable of expressing. On those
systems, this function returns `false` to indicate that the time is
out of range.

On systems where 'long' is 64bit, this function never fails.
## `tv`
a `TimeVal` to modify

# Returns

`true` if successful, else `false`
<!-- impl DateTime::fn to_timezone -->
Create a new `DateTime` corresponding to the same instant in time as
`self`, but in the time zone `tz`.

This call can fail in the case that the time goes out of bounds. For
example, converting 0001-01-01 00:00:00 UTC to a time zone west of
Greenwich will fail (due to the year 0 being out of range).

You should release the return value by calling `DateTime::unref`
when you are done with it.
## `tz`
the new `TimeZone`

# Returns

a new `DateTime`, or `None`
<!-- impl DateTime::fn to_unix -->
Gives the Unix time corresponding to `self`, rounding down to the
nearest second.

Unix time is the number of seconds that have elapsed since 1970-01-01
00:00:00 UTC, regardless of the time zone associated with `self`.

# Returns

the Unix time corresponding to `self`
<!-- impl DateTime::fn to_utc -->
Creates a new `DateTime` corresponding to the same instant in time as
`self`, but in UTC.

This call is equivalent to calling `DateTime::to_timezone` with the
time zone returned by `TimeZone::new_utc`.

# Returns

the newly created `DateTime`
<!-- impl DateTime::fn unref -->
Atomically decrements the reference count of `self` by one.

When the reference count reaches zero, the resources allocated by
`self` are freed
<!-- impl DateTime::fn compare -->
A comparison function for `GDateTimes` that is suitable
as a `GCompareFunc`. Both `GDateTimes` must be non-`None`.
## `dt1`
first `DateTime` to compare
## `dt2`
second `DateTime` to compare

# Returns

-1, 0 or 1 if `dt1` is less than, equal to or greater
 than `dt2`.
<!-- impl DateTime::fn equal -->
Checks to see if `dt1` and `dt2` are equal.

Equal here means that they represent the same moment after converting
them to the same time zone.
## `dt1`
a `DateTime`
## `dt2`
a `DateTime`

# Returns

`true` if `dt1` and `dt2` are equal
<!-- impl DateTime::fn hash -->
Hashes `datetime` into a `guint`, suitable for use within `HashTable`.
## `datetime`
a `DateTime`

# Returns

a `guint` containing the hash
<!-- enum DateWeekday -->
Enumeration representing a day of the week; `DateWeekday::Monday`,
`DateWeekday::Tuesday`, etc. `DateWeekday::BadWeekday` is an invalid weekday.
<!-- enum DateWeekday::variant BadWeekday -->
invalid value
<!-- enum DateWeekday::variant Monday -->
Monday
<!-- enum DateWeekday::variant Tuesday -->
Tuesday
<!-- enum DateWeekday::variant Wednesday -->
Wednesday
<!-- enum DateWeekday::variant Thursday -->
Thursday
<!-- enum DateWeekday::variant Friday -->
Friday
<!-- enum DateWeekday::variant Saturday -->
Saturday
<!-- enum DateWeekday::variant Sunday -->
Sunday
<!-- struct Error -->
The `GError` structure contains information about
an error that has occurred.
<!-- impl Error::fn new -->
Creates a new `Error` with the given `domain` and `code`,
and a message formatted with `format`.
## `domain`
error domain
## `code`
error code
## `format`
`printf`-style format for error message

# Returns

a new `Error`
<!-- impl Error::fn new_literal -->
Creates a new `Error`; unlike `Error::new`, `message` is
not a `printf`-style format string. Use this function if
`message` contains text you don't have control over,
that could include `printf` escape sequences.
## `domain`
error domain
## `code`
error code
## `message`
error message

# Returns

a new `Error`
<!-- impl Error::fn new_valist -->
Creates a new `Error` with the given `domain` and `code`,
and a message formatted with `format`.
## `domain`
error domain
## `code`
error code
## `format`
`printf`-style format for error message
## `args`
`va_list` of parameters for the message format

# Returns

a new `Error`
<!-- impl Error::fn copy -->
Makes a copy of `self`.

# Returns

a new `Error`
<!-- impl Error::fn free -->
Frees a `Error` and associated resources.
<!-- impl Error::fn matches -->
Returns `true` if `self` matches `domain` and `code`, `false`
otherwise. In particular, when `self` is `None`, `false` will
be returned.

If `domain` contains a `FAILED` (or otherwise generic) error code,
you should generally not check for it explicitly, but should
instead treat any not-explicitly-recognized error code as being
equivalent to the `FAILED` code. This way, if the domain is
extended in the future to provide a more specific error code for
a certain case, your code will still work.
## `domain`
an error domain
## `code`
an error code

# Returns

whether `self` has `domain` and `code`
<!-- struct KeyFile -->
The GKeyFile struct contains only private data
and should not be accessed directly.
<!-- impl KeyFile::fn new -->
Creates a new empty `KeyFile` object. Use
`KeyFile::load_from_file`, `KeyFile::load_from_data`,
`KeyFile::load_from_dirs` or `KeyFile::load_from_data_dirs` to
read an existing key file.

# Returns

an empty `KeyFile`.
<!-- impl KeyFile::fn free -->
Clears all keys and groups from `self`, and decreases the
reference count by 1. If the reference count reaches zero,
frees the key file and all its allocated memory.
<!-- impl KeyFile::fn get_boolean -->
Returns the value associated with `key` under `group_name` as a
boolean.

If `key` cannot be found then `false` is returned and `error` is set
to `KeyFileError::KeyNotFound`. Likewise, if the value
associated with `key` cannot be interpreted as a boolean then `false`
is returned and `error` is set to `KeyFileError::InvalidValue`.
## `group_name`
a group name
## `key`
a key

# Returns

the value associated with the key as a boolean,
 or `false` if the key was not found or could not be parsed.
<!-- impl KeyFile::fn get_boolean_list -->
Returns the values associated with `key` under `group_name` as
booleans.

If `key` cannot be found then `None` is returned and `error` is set to
`KeyFileError::KeyNotFound`. Likewise, if the values associated
with `key` cannot be interpreted as booleans then `None` is returned
and `error` is set to `KeyFileError::InvalidValue`.
## `group_name`
a group name
## `key`
a key
## `length`
the number of booleans returned

# Returns


 the values associated with the key as a list of booleans, or `None` if the
 key was not found or could not be parsed. The returned list of booleans
 should be freed with `g_free` when no longer needed.
<!-- impl KeyFile::fn get_comment -->
Retrieves a comment above `key` from `group_name`.
If `key` is `None` then `comment` will be read from above
`group_name`. If both `key` and `group_name` are `None`, then
`comment` will be read from above the first group in the file.

Note that the returned string includes the '#' comment markers.
## `group_name`
a group name, or `None`
## `key`
a key

# Returns

a comment that should be freed with `g_free`
<!-- impl KeyFile::fn get_double -->
Returns the value associated with `key` under `group_name` as a
double. If `group_name` is `None`, the start_group is used.

If `key` cannot be found then 0.0 is returned and `error` is set to
`KeyFileError::KeyNotFound`. Likewise, if the value associated
with `key` cannot be interpreted as a double then 0.0 is returned
and `error` is set to `KeyFileError::InvalidValue`.
## `group_name`
a group name
## `key`
a key

# Returns

the value associated with the key as a double, or
 0.0 if the key was not found or could not be parsed.
<!-- impl KeyFile::fn get_double_list -->
Returns the values associated with `key` under `group_name` as
doubles.

If `key` cannot be found then `None` is returned and `error` is set to
`KeyFileError::KeyNotFound`. Likewise, if the values associated
with `key` cannot be interpreted as doubles then `None` is returned
and `error` is set to `KeyFileError::InvalidValue`.
## `group_name`
a group name
## `key`
a key
## `length`
the number of doubles returned

# Returns


 the values associated with the key as a list of doubles, or `None` if the
 key was not found or could not be parsed. The returned list of doubles
 should be freed with `g_free` when no longer needed.
<!-- impl KeyFile::fn get_groups -->
Returns all groups in the key file loaded with `self`.
The array of returned groups will be `None`-terminated, so
`length` may optionally be `None`.
## `length`
return location for the number of returned groups, or `None`

# Returns

a newly-allocated `None`-terminated array of strings.
 Use `g_strfreev` to free it.
<!-- impl KeyFile::fn get_int64 -->
Returns the value associated with `key` under `group_name` as a signed
64-bit integer. This is similar to `KeyFile::get_integer` but can return
64-bit results without truncation.
## `group_name`
a non-`None` group name
## `key`
a non-`None` key

# Returns

the value associated with the key as a signed 64-bit integer, or
0 if the key was not found or could not be parsed.
<!-- impl KeyFile::fn get_integer -->
Returns the value associated with `key` under `group_name` as an
integer.

If `key` cannot be found then 0 is returned and `error` is set to
`KeyFileError::KeyNotFound`. Likewise, if the value associated
with `key` cannot be interpreted as an integer, or is out of range
for a `gint`, then 0 is returned
and `error` is set to `KeyFileError::InvalidValue`.
## `group_name`
a group name
## `key`
a key

# Returns

the value associated with the key as an integer, or
 0 if the key was not found or could not be parsed.
<!-- impl KeyFile::fn get_integer_list -->
Returns the values associated with `key` under `group_name` as
integers.

If `key` cannot be found then `None` is returned and `error` is set to
`KeyFileError::KeyNotFound`. Likewise, if the values associated
with `key` cannot be interpreted as integers, or are out of range for
`gint`, then `None` is returned
and `error` is set to `KeyFileError::InvalidValue`.
## `group_name`
a group name
## `key`
a key
## `length`
the number of integers returned

# Returns


 the values associated with the key as a list of integers, or `None` if
 the key was not found or could not be parsed. The returned list of
 integers should be freed with `g_free` when no longer needed.
<!-- impl KeyFile::fn get_keys -->
Returns all keys for the group name `group_name`. The array of
returned keys will be `None`-terminated, so `length` may
optionally be `None`. In the event that the `group_name` cannot
be found, `None` is returned and `error` is set to
`KeyFileError::GroupNotFound`.
## `group_name`
a group name
## `length`
return location for the number of keys returned, or `None`

# Returns

a newly-allocated `None`-terminated array of strings.
 Use `g_strfreev` to free it.
<!-- impl KeyFile::fn get_locale_for_key -->
Returns the actual locale which the result of
`KeyFile::get_locale_string` or `KeyFile::get_locale_string_list`
came from.

If calling `KeyFile::get_locale_string` or
`KeyFile::get_locale_string_list` with exactly the same `self`,
`group_name`, `key` and `locale`, the result of those functions will
have originally been tagged with the locale that is the result of
this function.

Feature: `v2_56`

## `group_name`
a group name
## `key`
a key
## `locale`
a locale identifier or `None`

# Returns

the locale from the file, or `None` if the key was not
 found or the entry in the file was was untranslated
<!-- impl KeyFile::fn get_locale_string -->
Returns the value associated with `key` under `group_name`
translated in the given `locale` if available. If `locale` is
`None` then the current locale is assumed.

If `locale` is to be non-`None`, or if the current locale will change over
the lifetime of the `KeyFile`, it must be loaded with
`KeyFileFlags::KeepTranslations` in order to load strings for all locales.

If `key` cannot be found then `None` is returned and `error` is set
to `KeyFileError::KeyNotFound`. If the value associated
with `key` cannot be interpreted or no suitable translation can
be found then the untranslated value is returned.
## `group_name`
a group name
## `key`
a key
## `locale`
a locale identifier or `None`

# Returns

a newly allocated string or `None` if the specified
 key cannot be found.
<!-- impl KeyFile::fn get_locale_string_list -->
Returns the values associated with `key` under `group_name`
translated in the given `locale` if available. If `locale` is
`None` then the current locale is assumed.

If `locale` is to be non-`None`, or if the current locale will change over
the lifetime of the `KeyFile`, it must be loaded with
`KeyFileFlags::KeepTranslations` in order to load strings for all locales.

If `key` cannot be found then `None` is returned and `error` is set
to `KeyFileError::KeyNotFound`. If the values associated
with `key` cannot be interpreted or no suitable translations
can be found then the untranslated values are returned. The
returned array is `None`-terminated, so `length` may optionally
be `None`.
## `group_name`
a group name
## `key`
a key
## `locale`
a locale identifier or `None`
## `length`
return location for the number of returned strings or `None`

# Returns

a newly allocated `None`-terminated string array
 or `None` if the key isn't found. The string array should be freed
 with `g_strfreev`.
<!-- impl KeyFile::fn get_start_group -->
Returns the name of the start group of the file.

# Returns

The start group of the key file.
<!-- impl KeyFile::fn get_string -->
Returns the string value associated with `key` under `group_name`.
Unlike `KeyFile::get_value`, this function handles escape sequences
like \s.

In the event the key cannot be found, `None` is returned and
`error` is set to `KeyFileError::KeyNotFound`. In the
event that the `group_name` cannot be found, `None` is returned
and `error` is set to `KeyFileError::GroupNotFound`.
## `group_name`
a group name
## `key`
a key

# Returns

a newly allocated string or `None` if the specified
 key cannot be found.
<!-- impl KeyFile::fn get_string_list -->
Returns the values associated with `key` under `group_name`.

In the event the key cannot be found, `None` is returned and
`error` is set to `KeyFileError::KeyNotFound`. In the
event that the `group_name` cannot be found, `None` is returned
and `error` is set to `KeyFileError::GroupNotFound`.
## `group_name`
a group name
## `key`
a key
## `length`
return location for the number of returned strings, or `None`

# Returns


 a `None`-terminated string array or `None` if the specified
 key cannot be found. The array should be freed with `g_strfreev`.
<!-- impl KeyFile::fn get_uint64 -->
Returns the value associated with `key` under `group_name` as an unsigned
64-bit integer. This is similar to `KeyFile::get_integer` but can return
large positive results without truncation.
## `group_name`
a non-`None` group name
## `key`
a non-`None` key

# Returns

the value associated with the key as an unsigned 64-bit integer,
or 0 if the key was not found or could not be parsed.
<!-- impl KeyFile::fn get_value -->
Returns the raw value associated with `key` under `group_name`.
Use `KeyFile::get_string` to retrieve an unescaped UTF-8 string.

In the event the key cannot be found, `None` is returned and
`error` is set to `KeyFileError::KeyNotFound`. In the
event that the `group_name` cannot be found, `None` is returned
and `error` is set to `KeyFileError::GroupNotFound`.
## `group_name`
a group name
## `key`
a key

# Returns

a newly allocated string or `None` if the specified
 key cannot be found.
<!-- impl KeyFile::fn has_group -->
Looks whether the key file has the group `group_name`.
## `group_name`
a group name

# Returns

`true` if `group_name` is a part of `self`, `false`
otherwise.
<!-- impl KeyFile::fn has_key -->
Looks whether the key file has the key `key` in the group
`group_name`.

Note that this function does not follow the rules for `Error` strictly;
the return value both carries meaning and signals an error. To use
this function, you must pass a `Error` pointer in `error`, and check
whether it is not `None` to see if an error occurred.

Language bindings should use `KeyFile::get_value` to test whether
or not a key exists.
## `group_name`
a group name
## `key`
a key name

# Returns

`true` if `key` is a part of `group_name`, `false` otherwise
<!-- impl KeyFile::fn load_from_bytes -->
Loads a key file from the data in `bytes` into an empty `KeyFile` structure.
If the object cannot be created then `error` is set to a `KeyFileError`.

Feature: `v2_50`

## `bytes`
a `Bytes`
## `flags`
flags from `KeyFileFlags`

# Returns

`true` if a key file could be loaded, `false` otherwise
<!-- impl KeyFile::fn load_from_data -->
Loads a key file from memory into an empty `KeyFile` structure.
If the object cannot be created then `error` is set to a `KeyFileError`.
## `data`
key file loaded in memory
## `length`
the length of `data` in bytes (or (gsize)-1 if data is nul-terminated)
## `flags`
flags from `KeyFileFlags`

# Returns

`true` if a key file could be loaded, `false` otherwise
<!-- impl KeyFile::fn load_from_data_dirs -->
This function looks for a key file named `file` in the paths
returned from `g_get_user_data_dir` and `g_get_system_data_dirs`,
loads the file into `self` and returns the file's full path in
`full_path`. If the file could not be loaded then an `error` is
set to either a `FileError` or `KeyFileError`.
## `file`
a relative path to a filename to open and parse
## `full_path`
return location for a string containing the full path
 of the file, or `None`
## `flags`
flags from `KeyFileFlags`

# Returns

`true` if a key file could be loaded, `false` othewise
<!-- impl KeyFile::fn load_from_dirs -->
This function looks for a key file named `file` in the paths
specified in `search_dirs`, loads the file into `self` and
returns the file's full path in `full_path`.

If the file could not be found in any of the `search_dirs`,
`KeyFileError::NotFound` is returned. If
the file is found but the OS returns an error when opening or reading the
file, a `G_FILE_ERROR` is returned. If there is a problem parsing the file, a
`G_KEY_FILE_ERROR` is returned.
## `file`
a relative path to a filename to open and parse
## `search_dirs`
`None`-terminated array of directories to search
## `full_path`
return location for a string containing the full path
 of the file, or `None`
## `flags`
flags from `KeyFileFlags`

# Returns

`true` if a key file could be loaded, `false` otherwise
<!-- impl KeyFile::fn load_from_file -->
Loads a key file into an empty `KeyFile` structure.

If the OS returns an error when opening or reading the file, a
`G_FILE_ERROR` is returned. If there is a problem parsing the file, a
`G_KEY_FILE_ERROR` is returned.

This function will never return a `KeyFileError::NotFound` error. If the
`file` is not found, `FileError::Noent` is returned.
## `file`
the path of a filename to load, in the GLib filename encoding
## `flags`
flags from `KeyFileFlags`

# Returns

`true` if a key file could be loaded, `false` otherwise
<!-- impl KeyFile::fn ref -->
Increases the reference count of `self`.

# Returns

the same `self`.
<!-- impl KeyFile::fn remove_comment -->
Removes a comment above `key` from `group_name`.
If `key` is `None` then `comment` will be removed above `group_name`.
If both `key` and `group_name` are `None`, then `comment` will
be removed above the first group in the file.
## `group_name`
a group name, or `None`
## `key`
a key

# Returns

`true` if the comment was removed, `false` otherwise
<!-- impl KeyFile::fn remove_group -->
Removes the specified group, `group_name`,
from the key file.
## `group_name`
a group name

# Returns

`true` if the group was removed, `false` otherwise
<!-- impl KeyFile::fn remove_key -->
Removes `key` in `group_name` from the key file.
## `group_name`
a group name
## `key`
a key name to remove

# Returns

`true` if the key was removed, `false` otherwise
<!-- impl KeyFile::fn save_to_file -->
Writes the contents of `self` to `filename` using
`g_file_set_contents`.

This function can fail for any of the reasons that
`g_file_set_contents` may fail.
## `filename`
the name of the file to write to

# Returns

`true` if successful, else `false` with `error` set
<!-- impl KeyFile::fn set_boolean -->
Associates a new boolean value with `key` under `group_name`.
If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `value`
`true` or `false`
<!-- impl KeyFile::fn set_boolean_list -->
Associates a list of boolean values with `key` under `group_name`.
If `key` cannot be found then it is created.
If `group_name` is `None`, the start_group is used.
## `group_name`
a group name
## `key`
a key
## `list`
an array of boolean values
## `length`
length of `list`
<!-- impl KeyFile::fn set_comment -->
Places a comment above `key` from `group_name`.

If `key` is `None` then `comment` will be written above `group_name`.
If both `key` and `group_name` are `None`, then `comment` will be
written above the first group in the file.

Note that this function prepends a '#' comment marker to
each line of `comment`.
## `group_name`
a group name, or `None`
## `key`
a key
## `comment`
a comment

# Returns

`true` if the comment was written, `false` otherwise
<!-- impl KeyFile::fn set_double -->
Associates a new double value with `key` under `group_name`.
If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `value`
an double value
<!-- impl KeyFile::fn set_double_list -->
Associates a list of double values with `key` under
`group_name`. If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `list`
an array of double values
## `length`
number of double values in `list`
<!-- impl KeyFile::fn set_int64 -->
Associates a new integer value with `key` under `group_name`.
If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `value`
an integer value
<!-- impl KeyFile::fn set_integer -->
Associates a new integer value with `key` under `group_name`.
If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `value`
an integer value
<!-- impl KeyFile::fn set_integer_list -->
Associates a list of integer values with `key` under `group_name`.
If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `list`
an array of integer values
## `length`
number of integer values in `list`
<!-- impl KeyFile::fn set_list_separator -->
Sets the character which is used to separate
values in lists. Typically ';' or ',' are used
as separators. The default list separator is ';'.
## `separator`
the separator
<!-- impl KeyFile::fn set_locale_string -->
Associates a string value for `key` and `locale` under `group_name`.
If the translation for `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `locale`
a locale identifier
## `string`
a string
<!-- impl KeyFile::fn set_locale_string_list -->
Associates a list of string values for `key` and `locale` under
`group_name`. If the translation for `key` cannot be found then
it is created.
## `group_name`
a group name
## `key`
a key
## `locale`
a locale identifier
## `list`
a `None`-terminated array of locale string values
## `length`
the length of `list`
<!-- impl KeyFile::fn set_string -->
Associates a new string value with `key` under `group_name`.
If `key` cannot be found then it is created.
If `group_name` cannot be found then it is created.
Unlike `KeyFile::set_value`, this function handles characters
that need escaping, such as newlines.
## `group_name`
a group name
## `key`
a key
## `string`
a string
<!-- impl KeyFile::fn set_string_list -->
Associates a list of string values for `key` under `group_name`.
If `key` cannot be found then it is created.
If `group_name` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `list`
an array of string values
## `length`
number of string values in `list`
<!-- impl KeyFile::fn set_uint64 -->
Associates a new integer value with `key` under `group_name`.
If `key` cannot be found then it is created.
## `group_name`
a group name
## `key`
a key
## `value`
an integer value
<!-- impl KeyFile::fn set_value -->
Associates a new value with `key` under `group_name`.

If `key` cannot be found then it is created. If `group_name` cannot
be found then it is created. To set an UTF-8 string which may contain
characters that need escaping (such as newlines or spaces), use
`KeyFile::set_string`.
## `group_name`
a group name
## `key`
a key
## `value`
a string
<!-- impl KeyFile::fn to_data -->
This function outputs `self` as a string.

Note that this function never reports an error,
so it is safe to pass `None` as `error`.
## `length`
return location for the length of the
 returned string, or `None`

# Returns

a newly allocated string holding
 the contents of the `KeyFile`
<!-- impl KeyFile::fn unref -->
Decreases the reference count of `self` by 1. If the reference count
reaches zero, frees the key file and all its allocated memory.
<!-- enum KeyFileError -->
Error codes returned by key file parsing.
<!-- enum KeyFileError::variant UnknownEncoding -->
the text being parsed was in
 an unknown encoding
<!-- enum KeyFileError::variant Parse -->
document was ill-formed
<!-- enum KeyFileError::variant NotFound -->
the file was not found
<!-- enum KeyFileError::variant KeyNotFound -->
a requested key was not found
<!-- enum KeyFileError::variant GroupNotFound -->
a requested group was not found
<!-- enum KeyFileError::variant InvalidValue -->
a value could not be parsed
<!-- struct MainContext -->
The `GMainContext` struct is an opaque data
type representing a set of sources to be handled in a main loop.
<!-- impl MainContext::fn new -->
Creates a new `MainContext` structure.

# Returns

the new `MainContext`
<!-- impl MainContext::fn acquire -->
Tries to become the owner of the specified context.
If some other thread is the owner of the context,
returns `false` immediately. Ownership is properly
recursive: the owner can require ownership again
and will release ownership when `MainContext::release`
is called as many times as `MainContext::acquire`.

You must be the owner of a context before you
can call `MainContext::prepare`, `MainContext::query`,
`MainContext::check`, `MainContext::dispatch`.

# Returns

`true` if the operation succeeded, and
 this thread is now the owner of `self`.
<!-- impl MainContext::fn add_poll -->
Adds a file descriptor to the set of file descriptors polled for
this context. This will very seldom be used directly. Instead
a typical event source will use `Source::add_unix_fd` instead.
## `fd`
a `PollFD` structure holding information about a file
 descriptor to watch.
## `priority`
the priority for this file descriptor which should be
 the same as the priority used for `Source::attach` to ensure that the
 file descriptor is polled whenever the results may be needed.
<!-- impl MainContext::fn check -->
Passes the results of polling back to the main loop.

You must have successfully acquired the context with
`MainContext::acquire` before you may call this function.
## `max_priority`
the maximum numerical priority of sources to check
## `fds`
array of `PollFD`'s that was passed to
 the last call to `MainContext::query`
## `n_fds`
return value of `MainContext::query`

# Returns

`true` if some sources are ready to be dispatched.
<!-- impl MainContext::fn dispatch -->
Dispatches all pending sources.

You must have successfully acquired the context with
`MainContext::acquire` before you may call this function.
<!-- impl MainContext::fn find_source_by_funcs_user_data -->
Finds a source with the given source functions and user data. If
multiple sources exist with the same source function and user data,
the first one found will be returned.
## `funcs`
the `source_funcs` passed to `Source::new`.
## `user_data`
the user data from the callback.

# Returns

the source, if one was found, otherwise `None`
<!-- impl MainContext::fn find_source_by_id -->
Finds a `Source` given a pair of context and ID.

It is a programmer error to attempt to lookup a non-existent source.

More specifically: source IDs can be reissued after a source has been
destroyed and therefore it is never valid to use this function with a
source ID which may have already been removed. An example is when
scheduling an idle to run in another thread with `g_idle_add`: the
idle may already have run and been removed by the time this function
is called on its (now invalid) source ID. This source ID may have
been reissued, leading to the operation being performed against the
wrong source.
## `source_id`
the source ID, as returned by `Source::get_id`.

# Returns

the `Source`
<!-- impl MainContext::fn find_source_by_user_data -->
Finds a source with the given user data for the callback. If
multiple sources exist with the same user data, the first
one found will be returned.
## `user_data`
the user_data for the callback.

# Returns

the source, if one was found, otherwise `None`
<!-- impl MainContext::fn get_poll_func -->
Gets the poll function set by `MainContext::set_poll_func`.

# Returns

the poll function
<!-- impl MainContext::fn invoke -->
Invokes a function in such a way that `self` is owned during the
invocation of `function`.

If `self` is `None` then the global default main context — as
returned by `MainContext::default` — is used.

If `self` is owned by the current thread, `function` is called
directly. Otherwise, if `self` is the thread-default main context
of the current thread and `MainContext::acquire` succeeds, then
`function` is called and `MainContext::release` is called
afterwards.

In any other case, an idle source is created to call `function` and
that source is attached to `self` (presumably to be run in another
thread). The idle source is attached with `G_PRIORITY_DEFAULT`
priority. If you want a different priority, use
`MainContext::invoke_full`.

Note that, as with normal idle functions, `function` should probably
return `false`. If it returns `true`, it will be continuously run in a
loop (and may prevent this call from returning).
## `function`
function to call
## `data`
data to pass to `function`
<!-- impl MainContext::fn invoke_full -->
Invokes a function in such a way that `self` is owned during the
invocation of `function`.

This function is the same as `MainContext::invoke` except that it
lets you specify the priority in case `function` ends up being
scheduled as an idle and also lets you give a `GDestroyNotify` for `data`.

`notify` should not assume that it is called from any particular
thread or with any particular context acquired.
## `priority`
the priority at which to run `function`
## `function`
function to call
## `data`
data to pass to `function`
## `notify`
a function to call when `data` is no longer in use, or `None`.
<!-- impl MainContext::fn is_owner -->
Determines whether this thread holds the (recursive)
ownership of this `MainContext`. This is useful to
know before waiting on another thread that may be
blocking to get ownership of `self`.

# Returns

`true` if current thread is owner of `self`.
<!-- impl MainContext::fn iteration -->
Runs a single iteration for the given main loop. This involves
checking to see if any event sources are ready to be processed,
then if no events sources are ready and `may_block` is `true`, waiting
for a source to become ready, then dispatching the highest priority
events sources that are ready. Otherwise, if `may_block` is `false`
sources are not waited to become ready, only those highest priority
events sources will be dispatched (if any), that are ready at this
given moment without further waiting.

Note that even when `may_block` is `true`, it is still possible for
`MainContext::iteration` to return `false`, since the wait may
be interrupted for other reasons than an event source becoming ready.
## `may_block`
whether the call may block.

# Returns

`true` if events were dispatched.
<!-- impl MainContext::fn pending -->
Checks if any sources have pending events for the given context.

# Returns

`true` if events are pending.
<!-- impl MainContext::fn pop_thread_default -->
Pops `self` off the thread-default context stack (verifying that
it was on the top of the stack).
<!-- impl MainContext::fn prepare -->
Prepares to poll sources within a main loop. The resulting information
for polling is determined by calling g_main_context_query ().

You must have successfully acquired the context with
`MainContext::acquire` before you may call this function.
## `priority`
location to store priority of highest priority
 source already ready.

# Returns

`true` if some source is ready to be dispatched
 prior to polling.
<!-- impl MainContext::fn push_thread_default -->
Acquires `self` and sets it as the thread-default context for the
current thread. This will cause certain asynchronous operations
(such as most [gio][gio]-based I/O) which are
started in this thread to run under `self` and deliver their
results to its main loop, rather than running under the global
default context in the main thread. Note that calling this function
changes the context returned by `MainContext::get_thread_default`,
not the one returned by `MainContext::default`, so it does not affect
the context used by functions like `g_idle_add`.

Normally you would call this function shortly after creating a new
thread, passing it a `MainContext` which will be run by a
`MainLoop` in that thread, to set a new default context for all
async operations in that thread. In this case you may not need to
ever call `MainContext::pop_thread_default`, assuming you want the
new `MainContext` to be the default for the whole lifecycle of the
thread.

If you don't have control over how the new thread was created (e.g.
in the new thread isn't newly created, or if the thread life
cycle is managed by a `ThreadPool`), it is always suggested to wrap
the logic that needs to use the new `MainContext` inside a
`MainContext::push_thread_default` / `MainContext::pop_thread_default`
pair, otherwise threads that are re-used will end up never explicitly
releasing the `MainContext` reference they hold.

In some cases you may want to schedule a single operation in a
non-default context, or temporarily use a non-default context in
the main thread. In that case, you can wrap the call to the
asynchronous operation inside a
`MainContext::push_thread_default` /
`MainContext::pop_thread_default` pair, but it is up to you to
ensure that no other asynchronous operations accidentally get
started while the non-default context is active.

Beware that libraries that predate this function may not correctly
handle being used from a thread with a thread-default context. Eg,
see `g_file_supports_thread_contexts`.
<!-- impl MainContext::fn query -->
Determines information necessary to poll this main loop.

You must have successfully acquired the context with
`MainContext::acquire` before you may call this function.
## `max_priority`
maximum priority source to check
## `timeout_`
location to store timeout to be used in polling
## `fds`
location to
 store `PollFD` records that need to be polled.
## `n_fds`
length of `fds`.

# Returns

the number of records actually stored in `fds`,
 or, if more than `n_fds` records need to be stored, the number
 of records that need to be stored.
<!-- impl MainContext::fn ref -->
Increases the reference count on a `MainContext` object by one.

# Returns

the `self` that was passed in (since 2.6)
<!-- impl MainContext::fn release -->
Releases ownership of a context previously acquired by this thread
with `MainContext::acquire`. If the context was acquired multiple
times, the ownership will be released only when `MainContext::release`
is called as many times as it was acquired.
<!-- impl MainContext::fn remove_poll -->
Removes file descriptor from the set of file descriptors to be
polled for a particular context.
## `fd`
a `PollFD` descriptor previously added with `MainContext::add_poll`
<!-- impl MainContext::fn set_poll_func -->
Sets the function to use to handle polling of file descriptors. It
will be used instead of the `poll` system call
(or GLib's replacement function, which is used where
`poll` isn't available).

This function could possibly be used to integrate the GLib event
loop with an external event loop.
## `func`
the function to call to poll all file descriptors
<!-- impl MainContext::fn unref -->
Decreases the reference count on a `MainContext` object by one. If
the result is zero, free the context and free all associated memory.
<!-- impl MainContext::fn wait -->
Tries to become the owner of the specified context,
as with `MainContext::acquire`. But if another thread
is the owner, atomically drop `mutex` and wait on `cond` until
that owner releases ownership or until `cond` is signaled, then
try again (once) to become the owner.

# Deprecated since 2.58

Use `MainContext::is_owner` and separate locking instead.
## `cond`
a condition variable
## `mutex`
a mutex, currently held

# Returns

`true` if the operation succeeded, and
 this thread is now the owner of `self`.
<!-- impl MainContext::fn wakeup -->
If `self` is currently blocking in `MainContext::iteration`
waiting for a source to become ready, cause it to stop blocking
and return. Otherwise, cause the next invocation of
`MainContext::iteration` to return without blocking.

This API is useful for low-level control over `MainContext`; for
example, integrating it with main loop implementations such as
`MainLoop`.

Another related use for this function is when implementing a main
loop with a termination condition, computed from multiple threads:


```C
  #define NUM_TASKS 10
  static volatile gint tasks_remaining = NUM_TASKS;
  ...
 
  while (g_atomic_int_get (&tasks_remaining) != 0)
    g_main_context_iteration (NULL, TRUE);
```
 
Then in a thread:

```C
  perform_work();

  if (g_atomic_int_dec_and_test (&tasks_remaining))
    g_main_context_wakeup (NULL);
```
<!-- impl MainContext::fn default -->
Returns the global default main context. This is the main context
used for main loop functions when a main loop is not explicitly
specified, and corresponds to the "main" main loop. See also
`MainContext::get_thread_default`.

# Returns

the global default main context.
<!-- impl MainContext::fn get_thread_default -->
Gets the thread-default `MainContext` for this thread. Asynchronous
operations that want to be able to be run in contexts other than
the default one should call this method or
`MainContext::ref_thread_default` to get a `MainContext` to add
their `GSources` to. (Note that even in single-threaded
programs applications may sometimes want to temporarily push a
non-default context, so it is not safe to assume that this will
always return `None` if you are running in the default thread.)

If you need to hold a reference on the context, use
`MainContext::ref_thread_default` instead.

# Returns

the thread-default `MainContext`, or
`None` if the thread-default context is the global default context.
<!-- impl MainContext::fn ref_thread_default -->
Gets the thread-default `MainContext` for this thread, as with
`MainContext::get_thread_default`, but also adds a reference to
it with `MainContext::ref`. In addition, unlike
`MainContext::get_thread_default`, if the thread-default context
is the global default context, this will return that `MainContext`
(with a ref added to it) rather than returning `None`.

# Returns

the thread-default `MainContext`. Unref
 with `MainContext::unref` when you are done with it.
<!-- struct MainLoop -->
The `GMainLoop` struct is an opaque data type
representing the main event loop of a GLib or GTK+ application.
<!-- impl MainLoop::fn new -->
Creates a new `MainLoop` structure.
## `context`
a `MainContext` (if `None`, the default context will be used).
## `is_running`
set to `true` to indicate that the loop is running. This
is not very important since calling `MainLoop::run` will set this to
`true` anyway.

# Returns

a new `MainLoop`.
<!-- impl MainLoop::fn get_context -->
Returns the `MainContext` of `self`.

# Returns

the `MainContext` of `self`
<!-- impl MainLoop::fn is_running -->
Checks to see if the main loop is currently being run via `MainLoop::run`.

# Returns

`true` if the mainloop is currently being run.
<!-- impl MainLoop::fn quit -->
Stops a `MainLoop` from running. Any calls to `MainLoop::run`
for the loop will return.

Note that sources that have already been dispatched when
`MainLoop::quit` is called will still be executed.
<!-- impl MainLoop::fn ref -->
Increases the reference count on a `MainLoop` object by one.

# Returns

`self`
<!-- impl MainLoop::fn run -->
Runs a main loop until `MainLoop::quit` is called on the loop.
If this is called for the thread of the loop's `MainContext`,
it will process events from the loop, otherwise it will
simply wait.
<!-- impl MainLoop::fn unref -->
Decreases the reference count on a `MainLoop` object by one. If
the result is zero, free the loop and free all associated memory.
<!-- enum OptionArg -->
The `OptionArg` enum values determine which type of extra argument the
options expect to find. If an option expects an extra argument, it can
be specified in several ways; with a short option: `-x arg`, with a long
option: `--name arg` or combined in a single argument: `--name=arg`.
<!-- enum OptionArg::variant None -->
No extra argument. This is useful for simple flags.
<!-- enum OptionArg::variant String -->
The option takes a string argument.
<!-- enum OptionArg::variant Int -->
The option takes an integer argument.
<!-- enum OptionArg::variant Callback -->
The option provides a callback (of type
 `GOptionArgFunc`) to parse the extra argument.
<!-- enum OptionArg::variant Filename -->
The option takes a filename as argument.
<!-- enum OptionArg::variant StringArray -->
The option takes a string argument, multiple
 uses of the option are collected into an array of strings.
<!-- enum OptionArg::variant FilenameArray -->
The option takes a filename as argument,
 multiple uses of the option are collected into an array of strings.
<!-- enum OptionArg::variant Double -->
The option takes a double argument. The argument
 can be formatted either for the user's locale or for the "C" locale.
 Since 2.12
<!-- enum OptionArg::variant Int64 -->
The option takes a 64-bit integer. Like
 `OptionArg::Int` but for larger numbers. The number can be in
 decimal base, or in hexadecimal (when prefixed with `0x`, for
 example, `0xffffffff`). Since 2.12
<!-- enum SeekType -->
An enumeration specifying the base position for a
`IOChannel::seek_position` operation.
<!-- enum SeekType::variant Cur -->
the current position in the file.
<!-- enum SeekType::variant Set -->
the start of the file.
<!-- enum SeekType::variant End -->
the end of the file.
<!-- struct Source -->
The `GSource` struct is an opaque data type
representing an event source.
<!-- impl Source::fn new -->
Creates a new `Source` structure. The size is specified to
allow creating structures derived from `Source` that contain
additional data. The size passed in must be at least
`sizeof (GSource)`.

The source will not initially be associated with any `MainContext`
and must be added to one with `Source::attach` before it will be
executed.
## `source_funcs`
structure containing functions that implement
 the sources behavior.
## `struct_size`
size of the `Source` structure to create.

# Returns

the newly-created `Source`.
<!-- impl Source::fn add_child_source -->
Adds `child_source` to `self` as a "polled" source; when `self` is
added to a `MainContext`, `child_source` will be automatically added
with the same priority, when `child_source` is triggered, it will
cause `self` to dispatch (in addition to calling its own
callback), and when `self` is destroyed, it will destroy
`child_source` as well. (`self` will also still be dispatched if
its own prepare/check functions indicate that it is ready.)

If you don't need `child_source` to do anything on its own when it
triggers, you can call `g_source_set_dummy_callback` on it to set a
callback that does nothing (except return `true` if appropriate).

`self` will hold a reference on `child_source` while `child_source`
is attached to it.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.
## `child_source`
a second `Source` that `self` should "poll"
<!-- impl Source::fn add_poll -->
Adds a file descriptor to the set of file descriptors polled for
this source. This is usually combined with `Source::new` to add an
event source. The event source's check function will typically test
the `revents` field in the `PollFD` struct and return `true` if events need
to be processed.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.

Using this API forces the linear scanning of event sources on each
main loop iteration. Newly-written event sources should try to use
`Source::add_unix_fd` instead of this API.
## `fd`
a `PollFD` structure holding information about a file
 descriptor to watch.
<!-- impl Source::fn add_unix_fd -->
Monitors `fd` for the IO events in `events`.

The tag returned by this function can be used to remove or modify the
monitoring of the fd using `Source::remove_unix_fd` or
`Source::modify_unix_fd`.

It is not necessary to remove the fd before destroying the source; it
will be cleaned up automatically.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.

As the name suggests, this function is not available on Windows.
## `fd`
the fd to monitor
## `events`
an event mask

# Returns

an opaque tag
<!-- impl Source::fn attach -->
Adds a `Source` to a `context` so that it will be executed within
that context. Remove it by calling `Source::destroy`.
## `context`
a `MainContext` (if `None`, the default context will be used)

# Returns

the ID (greater than 0) for the source within the
 `MainContext`.
<!-- impl Source::fn destroy -->
Removes a source from its `MainContext`, if any, and mark it as
destroyed. The source cannot be subsequently added to another
context. It is safe to call this on sources which have already been
removed from their context.
<!-- impl Source::fn get_can_recurse -->
Checks whether a source is allowed to be called recursively.
see `Source::set_can_recurse`.

# Returns

whether recursion is allowed.
<!-- impl Source::fn get_context -->
Gets the `MainContext` with which the source is associated.

You can call this on a source that has been destroyed, provided
that the `MainContext` it was attached to still exists (in which
case it will return that `MainContext`). In particular, you can
always call this function on the source returned from
`g_main_current_source`. But calling this function on a source
whose `MainContext` has been destroyed is an error.

# Returns

the `MainContext` with which the
 source is associated, or `None` if the context has not
 yet been added to a source.
<!-- impl Source::fn get_id -->
Returns the numeric ID for a particular source. The ID of a source
is a positive integer which is unique within a particular main loop
context. The reverse
mapping from ID to source is done by `MainContext::find_source_by_id`.

You can only call this function while the source is associated to a
`MainContext` instance; calling this function before `Source::attach`
or after `Source::destroy` yields undefined behavior. The ID returned
is unique within the `MainContext` instance passed to `Source::attach`.

# Returns

the ID (greater than 0) for the source
<!-- impl Source::fn get_name -->
Gets a name for the source, used in debugging and profiling. The
name may be `None` if it has never been set with `Source::set_name`.

# Returns

the name of the source
<!-- impl Source::fn get_priority -->
Gets the priority of a source.

# Returns

the priority of the source
<!-- impl Source::fn get_ready_time -->
Gets the "ready time" of `self`, as set by
`Source::set_ready_time`.

Any time before the current monotonic time (including 0) is an
indication that the source will fire immediately.

# Returns

the monotonic ready time, -1 for "never"
<!-- impl Source::fn get_time -->
Gets the time to be used when checking this source. The advantage of
calling this function over calling `g_get_monotonic_time` directly is
that when checking multiple sources, GLib can cache a single value
instead of having to repeatedly get the system monotonic time.

The time here is the system monotonic time, if available, or some
other reasonable alternative otherwise. See `g_get_monotonic_time`.

# Returns

the monotonic time in microseconds
<!-- impl Source::fn is_destroyed -->
Returns whether `self` has been destroyed.

This is important when you operate upon your objects
from within idle handlers, but may have freed the object
before the dispatch of your idle handler.


```C
static gboolean
idle_callback (gpointer data)
{
  SomeWidget *self = data;
   
  GDK_THREADS_ENTER ();
  // do stuff with self
  self->idle_id = 0;
  GDK_THREADS_LEAVE ();
   
  return G_SOURCE_REMOVE;
}
 
static void
some_widget_do_stuff_later (SomeWidget *self)
{
  self->idle_id = g_idle_add (idle_callback, self);
}
 
static void
some_widget_finalize (GObject *object)
{
  SomeWidget *self = SOME_WIDGET (object);
   
  if (self->idle_id)
    g_source_remove (self->idle_id);
   
  G_OBJECT_CLASS (parent_class)->finalize (object);
}
```

This will fail in a multi-threaded application if the
widget is destroyed before the idle handler fires due
to the use after free in the callback. A solution, to
this particular problem, is to check to if the source
has already been destroy within the callback.


```C
static gboolean
idle_callback (gpointer data)
{
  SomeWidget *self = data;
  
  GDK_THREADS_ENTER ();
  if (!g_source_is_destroyed (g_main_current_source ()))
    {
      // do stuff with self
    }
  GDK_THREADS_LEAVE ();
  
  return FALSE;
}
```

Calls to this function from a thread other than the one acquired by the
`MainContext` the `Source` is attached to are typically redundant, as the
source could be destroyed immediately after this function returns. However,
once a source is destroyed it cannot be un-destroyed, so this function can be
used for opportunistic checks from any thread.

# Returns

`true` if the source has been destroyed
<!-- impl Source::fn modify_unix_fd -->
Updates the event mask to watch for the fd identified by `tag`.

`tag` is the tag returned from `Source::add_unix_fd`.

If you want to remove a fd, don't set its event mask to zero.
Instead, call `Source::remove_unix_fd`.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.

As the name suggests, this function is not available on Windows.
## `tag`
the tag from `Source::add_unix_fd`
## `new_events`
the new event mask to watch
<!-- impl Source::fn query_unix_fd -->
Queries the events reported for the fd corresponding to `tag` on
`self` during the last poll.

The return value of this function is only defined when the function
is called from the check or dispatch functions for `self`.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.

As the name suggests, this function is not available on Windows.
## `tag`
the tag from `Source::add_unix_fd`

# Returns

the conditions reported on the fd
<!-- impl Source::fn ref -->
Increases the reference count on a source by one.

# Returns

`self`
<!-- impl Source::fn remove_child_source -->
Detaches `child_source` from `self` and destroys it.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.
## `child_source`
a `Source` previously passed to
 `Source::add_child_source`.
<!-- impl Source::fn remove_poll -->
Removes a file descriptor from the set of file descriptors polled for
this source.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.
## `fd`
a `PollFD` structure previously passed to `Source::add_poll`.
<!-- impl Source::fn remove_unix_fd -->
Reverses the effect of a previous call to `Source::add_unix_fd`.

You only need to call this if you want to remove an fd from being
watched while keeping the same source around. In the normal case you
will just want to destroy the source.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.

As the name suggests, this function is not available on Windows.
## `tag`
the tag from `Source::add_unix_fd`
<!-- impl Source::fn set_callback -->
Sets the callback function for a source. The callback for a source is
called from the source's dispatch function.

The exact type of `func` depends on the type of source; ie. you
should not count on `func` being called with `data` as its first
parameter. Cast `func` with G_SOURCE_FUNC() to avoid warnings about
incompatible function types.

See [memory management of sources][mainloop-memory-management] for details
on how to handle memory management of `data`.

Typically, you won't use this function. Instead use functions specific
to the type of source you are using.
## `func`
a callback function
## `data`
the data to pass to callback function
## `notify`
a function to call when `data` is no longer in use, or `None`.
<!-- impl Source::fn set_callback_indirect -->
Sets the callback function storing the data as a refcounted callback
"object". This is used internally. Note that calling
`Source::set_callback_indirect` assumes
an initial reference count on `callback_data`, and thus
`callback_funcs`->unref will eventually be called once more
than `callback_funcs`->ref.
## `callback_data`
pointer to callback data "object"
## `callback_funcs`
functions for reference counting `callback_data`
 and getting the callback and data
<!-- impl Source::fn set_can_recurse -->
Sets whether a source can be called recursively. If `can_recurse` is
`true`, then while the source is being dispatched then this source
will be processed normally. Otherwise, all processing of this
source is blocked until the dispatch function returns.
## `can_recurse`
whether recursion is allowed for this source
<!-- impl Source::fn set_funcs -->
Sets the source functions (can be used to override
default implementations) of an unattached source.
## `funcs`
the new `SourceFuncs`
<!-- impl Source::fn set_name -->
Sets a name for the source, used in debugging and profiling.
The name defaults to `None`.

The source name should describe in a human-readable way
what the source does. For example, "X11 event queue"
or "GTK+ repaint idle handler" or whatever it is.

It is permitted to call this function multiple times, but is not
recommended due to the potential performance impact. For example,
one could change the name in the "check" function of a `SourceFuncs`
to include details like the event type in the source name.

Use caution if changing the name while another thread may be
accessing it with `Source::get_name`; that function does not copy
the value, and changing the value will free it while the other thread
may be attempting to use it.
## `name`
debug name for the source
<!-- impl Source::fn set_priority -->
Sets the priority of a source. While the main loop is being run, a
source will be dispatched if it is ready to be dispatched and no
sources at a higher (numerically smaller) priority are ready to be
dispatched.

A child source always has the same priority as its parent. It is not
permitted to change the priority of a source once it has been added
as a child of another source.
## `priority`
the new priority.
<!-- impl Source::fn set_ready_time -->
Sets a `Source` to be dispatched when the given monotonic time is
reached (or passed). If the monotonic time is in the past (as it
always will be if `ready_time` is 0) then the source will be
dispatched immediately.

If `ready_time` is -1 then the source is never woken up on the basis
of the passage of time.

Dispatching the source does not reset the ready time. You should do
so yourself, from the source dispatch function.

Note that if you have a pair of sources where the ready time of one
suggests that it will be delivered first but the priority for the
other suggests that it would be delivered first, and the ready time
for both sources is reached during the same main context iteration,
then the order of dispatch is undefined.

It is a no-op to call this function on a `Source` which has already been
destroyed with `Source::destroy`.

This API is only intended to be used by implementations of `Source`.
Do not call this API on a `Source` that you did not create.
## `ready_time`
the monotonic time at which the source will be ready,
 0 for "immediately", -1 for "never"
<!-- impl Source::fn unref -->
Decreases the reference count of a source by one. If the
resulting reference count is zero the source and associated
memory will be destroyed.
<!-- impl Source::fn remove -->
Removes the source with the given ID from the default main context. You must
use `Source::destroy` for sources added to a non-default main context.

The ID of a `Source` is given by `Source::get_id`, or will be
returned by the functions `Source::attach`, `g_idle_add`,
`g_idle_add_full`, `g_timeout_add`, `g_timeout_add_full`,
`g_child_watch_add`, `g_child_watch_add_full`, `g_io_add_watch`, and
`g_io_add_watch_full`.

It is a programmer error to attempt to remove a non-existent source.

More specifically: source IDs can be reissued after a source has been
destroyed and therefore it is never valid to use this function with a
source ID which may have already been removed. An example is when
scheduling an idle to run in another thread with `g_idle_add`: the
idle may already have run and been removed by the time this function
is called on its (now invalid) source ID. This source ID may have
been reissued, leading to the operation being performed against the
wrong source.
## `tag`
the ID of the source to remove.

# Returns

For historical reasons, this function always returns `true`
<!-- impl Source::fn remove_by_funcs_user_data -->
Removes a source from the default main loop context given the
source functions and user data. If multiple sources exist with the
same source functions and user data, only one will be destroyed.
## `funcs`
The `source_funcs` passed to `Source::new`
## `user_data`
the user data for the callback

# Returns

`true` if a source was found and removed.
<!-- impl Source::fn remove_by_user_data -->
Removes a source from the default main loop context given the user
data for the callback. If multiple sources exist with the same user
data, only one will be destroyed.
## `user_data`
the user_data for the callback.

# Returns

`true` if a source was found and removed.
<!-- impl Source::fn set_name_by_id -->
Sets the name of a source using its ID.

This is a convenience utility to set source names from the return
value of `g_idle_add`, `g_timeout_add`, etc.

It is a programmer error to attempt to set the name of a non-existent
source.

More specifically: source IDs can be reissued after a source has been
destroyed and therefore it is never valid to use this function with a
source ID which may have already been removed. An example is when
scheduling an idle to run in another thread with `g_idle_add`: the
idle may already have run and been removed by the time this function
is called on its (now invalid) source ID. This source ID may have
been reissued, leading to the operation being performed against the
wrong source.
## `tag`
a `Source` ID
## `name`
debug name for the source
<!-- enum TimeType -->
Disambiguates a given time in two ways.

First, specifies if the given time is in universal or local time.

Second, if the time is in local time, specifies if it is local
standard time or local daylight time. This is important for the case
where the same local time occurs twice (during daylight savings time
transitions, for example).
<!-- enum TimeType::variant Standard -->
the time is in local standard time
<!-- enum TimeType::variant Daylight -->
the time is in local daylight time
<!-- enum TimeType::variant Universal -->
the time is in UTC
<!-- struct TimeZone -->
`TimeZone` is an opaque structure whose members cannot be accessed
directly.
<!-- impl TimeZone::fn new -->
Creates a `TimeZone` corresponding to `identifier`.

`identifier` can either be an RFC3339/ISO 8601 time offset or
something that would pass as a valid value for the `TZ` environment
variable (including `None`).

In Windows, `identifier` can also be the unlocalized name of a time
zone for standard time, for example "Pacific Standard Time".

Valid RFC3339 time offsets are `"Z"` (for UTC) or
`"±hh:mm"`. ISO 8601 additionally specifies
`"±hhmm"` and `"±hh"`. Offsets are
time values to be added to Coordinated Universal Time (UTC) to get
the local time.

In UNIX, the `TZ` environment variable typically corresponds
to the name of a file in the zoneinfo database, or string in
"std offset [dst [offset],start[/time],end[/time]]" (POSIX) format.
There are no spaces in the specification. The name of standard
and daylight savings time zone must be three or more alphabetic
characters. Offsets are time values to be added to local time to
get Coordinated Universal Time (UTC) and should be
`"[±]hh[[:]mm[:ss]]"`. Dates are either
`"Jn"` (Julian day with n between 1 and 365, leap
years not counted), `"n"` (zero-based Julian day
with n between 0 and 365) or `"Mm.w.d"` (day d
(0 <= d <= 6) of week w (1 <= w <= 5) of month m (1 <= m <= 12), day
0 is a Sunday). Times are in local wall clock time, the default is
02:00:00.

In Windows, the "tzn[+|–]hh[:mm[:ss]][dzn]" format is used, but also
accepts POSIX format. The Windows format uses US rules for all time
zones; daylight savings time is 60 minutes behind the standard time
with date and time of change taken from Pacific Standard Time.
Offsets are time values to be added to the local time to get
Coordinated Universal Time (UTC).

`TimeZone::new_local` calls this function with the value of the
`TZ` environment variable. This function itself is independent of
the value of `TZ`, but if `identifier` is `None` then `/etc/localtime`
will be consulted to discover the correct time zone on UNIX and the
registry will be consulted or GetTimeZoneInformation() will be used
to get the local time zone on Windows.

If intervals are not available, only time zone rules from `TZ`
environment variable or other means, then they will be computed
from year 1900 to 2037. If the maximum year for the rules is
available and it is greater than 2037, then it will followed
instead.

See
[RFC3339 §5.6](http://tools.ietf.org/html/rfc3339`section`-5.6)
for a precise definition of valid RFC3339 time offsets
(the `time-offset` expansion) and ISO 8601 for the
full list of valid time offsets. See
[The GNU C Library manual](http://www.gnu.org/s/libc/manual/html_node/TZ-Variable.html)
for an explanation of the possible
values of the `TZ` environment variable. See
[Microsoft Time Zone Index Values](http://msdn.microsoft.com/en-us/library/ms912391`28v`=winembedded.11`29.aspx`)
for the list of time zones on Windows.

You should release the return value by calling `TimeZone::unref`
when you are done with it.
## `identifier`
a timezone identifier

# Returns

the requested timezone
<!-- impl TimeZone::fn new_local -->
Creates a `TimeZone` corresponding to local time. The local time
zone may change between invocations to this function; for example,
if the system administrator changes it.

This is equivalent to calling `TimeZone::new` with the value of
the `TZ` environment variable (including the possibility of `None`).

You should release the return value by calling `TimeZone::unref`
when you are done with it.

# Returns

the local timezone
<!-- impl TimeZone::fn new_offset -->
Creates a `TimeZone` corresponding to the given constant offset from UTC,
in seconds.

This is equivalent to calling `TimeZone::new` with a string in the form
`[+|-]hh[:mm[:ss]]`.

Feature: `v2_58`

## `seconds`
offset to UTC, in seconds

# Returns

a timezone at the given offset from UTC
<!-- impl TimeZone::fn new_utc -->
Creates a `TimeZone` corresponding to UTC.

This is equivalent to calling `TimeZone::new` with a value like
"Z", "UTC", "+00", etc.

You should release the return value by calling `TimeZone::unref`
when you are done with it.

# Returns

the universal timezone
<!-- impl TimeZone::fn adjust_time -->
Finds an interval within `self` that corresponds to the given `time_`,
possibly adjusting `time_` if required to fit into an interval.
The meaning of `time_` depends on `type_`.

This function is similar to `TimeZone::find_interval`, with the
difference that it always succeeds (by making the adjustments
described below).

In any of the cases where `TimeZone::find_interval` succeeds then
this function returns the same value, without modifying `time_`.

This function may, however, modify `time_` in order to deal with
non-existent times. If the non-existent local `time_` of 02:30 were
requested on March 14th 2010 in Toronto then this function would
adjust `time_` to be 03:00 and return the interval containing the
adjusted time.
## `type_`
the `TimeType` of `time_`
## `time_`
a pointer to a number of seconds since January 1, 1970

# Returns

the interval containing `time_`, never -1
<!-- impl TimeZone::fn find_interval -->
Finds an the interval within `self` that corresponds to the given `time_`.
The meaning of `time_` depends on `type_`.

If `type_` is `TimeType::Universal` then this function will always
succeed (since universal time is monotonic and continuous).

Otherwise `time_` is treated as local time. The distinction between
`TimeType::Standard` and `TimeType::Daylight` is ignored except in
the case that the given `time_` is ambiguous. In Toronto, for example,
01:30 on November 7th 2010 occurred twice (once inside of daylight
savings time and the next, an hour later, outside of daylight savings
time). In this case, the different value of `type_` would result in a
different interval being returned.

It is still possible for this function to fail. In Toronto, for
example, 02:00 on March 14th 2010 does not exist (due to the leap
forward to begin daylight savings time). -1 is returned in that
case.
## `type_`
the `TimeType` of `time_`
## `time_`
a number of seconds since January 1, 1970

# Returns

the interval containing `time_`, or -1 in case of failure
<!-- impl TimeZone::fn get_abbreviation -->
Determines the time zone abbreviation to be used during a particular
`interval` of time in the time zone `self`.

For example, in Toronto this is currently "EST" during the winter
months and "EDT" during the summer months when daylight savings time
is in effect.
## `interval`
an interval within the timezone

# Returns

the time zone abbreviation, which belongs to `self`
<!-- impl TimeZone::fn get_identifier -->
Get the identifier of this `TimeZone`, as passed to `TimeZone::new`.
If the identifier passed at construction time was not recognised, `UTC` will
be returned. If it was `None`, the identifier of the local timezone at
construction time will be returned.

The identifier will be returned in the same format as provided at
construction time: if provided as a time offset, that will be returned by
this function.

Feature: `v2_58`


# Returns

identifier for this timezone
<!-- impl TimeZone::fn get_offset -->
Determines the offset to UTC in effect during a particular `interval`
of time in the time zone `self`.

The offset is the number of seconds that you add to UTC time to
arrive at local time for `self` (ie: negative numbers for time zones
west of GMT, positive numbers for east).
## `interval`
an interval within the timezone

# Returns

the number of seconds that should be added to UTC to get the
 local time in `self`
<!-- impl TimeZone::fn is_dst -->
Determines if daylight savings time is in effect during a particular
`interval` of time in the time zone `self`.
## `interval`
an interval within the timezone

# Returns

`true` if daylight savings time is in effect
<!-- impl TimeZone::fn ref -->
Increases the reference count on `self`.

# Returns

a new reference to `self`.
<!-- impl TimeZone::fn unref -->
Decreases the reference count on `self`.
<!-- enum UserDirectory -->
These are logical ids for special directories which are defined
depending on the platform used. You should use `g_get_user_special_dir`
to retrieve the full path associated to the logical id.

The `UserDirectory` enumeration can be extended at later date. Not
every platform has a directory for every logical id in this
enumeration.
<!-- enum UserDirectory::variant DirectoryDesktop -->
the user's Desktop directory
<!-- enum UserDirectory::variant DirectoryDocuments -->
the user's Documents directory
<!-- enum UserDirectory::variant DirectoryDownload -->
the user's Downloads directory
<!-- enum UserDirectory::variant DirectoryMusic -->
the user's Music directory
<!-- enum UserDirectory::variant DirectoryPictures -->
the user's Pictures directory
<!-- enum UserDirectory::variant DirectoryPublicShare -->
the user's shared directory
<!-- enum UserDirectory::variant DirectoryTemplates -->
the user's Templates directory
<!-- enum UserDirectory::variant DirectoryVideos -->
the user's Movies directory
<!-- enum UserDirectory::variant NDirectories -->
the number of enum values
<!-- struct Variant -->
`Variant` is a variant datatype; it can contain one or more values
along with information about the type of the values.

A `Variant` may contain simple types, like an integer, or a boolean value;
or complex types, like an array of two strings, or a dictionary of key
value pairs. A `Variant` is also immutable: once it's been created neither
its type nor its content can be modified further.

GVariant is useful whenever data needs to be serialized, for example when
sending method parameters in DBus, or when saving settings using GSettings.

When creating a new `Variant`, you pass the data you want to store in it
along with a string representing the type of data you wish to pass to it.

For instance, if you want to create a `Variant` holding an integer value you
can use:


```C
  GVariant *v = g_variant_new ("u", 40);
```

The string "u" in the first argument tells `Variant` that the data passed to
the constructor (40) is going to be an unsigned integer.

More advanced examples of `Variant` in use can be found in documentation for
[GVariant format strings][gvariant-format-strings-pointers].

The range of possible values is determined by the type.

The type system used by `Variant` is `VariantType`.

`Variant` instances always have a type and a value (which are given
at construction time). The type and value of a `Variant` instance
can never change other than by the `Variant` itself being
destroyed. A `Variant` cannot contain a pointer.

`Variant` is reference counted using `Variant::ref` and
`Variant::unref`. `Variant` also has floating reference counts --
see `Variant::ref_sink`.

`Variant` is completely threadsafe. A `Variant` instance can be
concurrently accessed in any way from any number of threads without
problems.

`Variant` is heavily optimised for dealing with data in serialised
form. It works particularly well with data located in memory-mapped
files. It can perform nearly all deserialisation operations in a
small constant time, usually touching only a single memory page.
Serialised `Variant` data can also be sent over the network.

`Variant` is largely compatible with D-Bus. Almost all types of
`Variant` instances can be sent over D-Bus. See `VariantType` for
exceptions. (However, `Variant`'s serialisation format is not the same
as the serialisation format of a D-Bus message body: use `GDBusMessage`,
in the gio library, for those.)

For space-efficiency, the `Variant` serialisation format does not
automatically include the variant's length, type or endianness,
which must either be implied from context (such as knowledge that a
particular file format always contains a little-endian
`G_VARIANT_TYPE_VARIANT` which occupies the whole length of the file)
or supplied out-of-band (for instance, a length, type and/or endianness
indicator could be placed at the beginning of a file, network message
or network stream).

A `Variant`'s size is limited mainly by any lower level operating
system constraints, such as the number of bits in `gsize`. For
example, it is reasonable to have a 2GB file mapped into memory
with `MappedFile`, and call `Variant::new_from_data` on it.

For convenience to C programmers, `Variant` features powerful
varargs-based value construction and destruction. This feature is
designed to be embedded in other libraries.

There is a Python-inspired text language for describing `Variant`
values. `Variant` includes a printer for this language and a parser
with type inferencing.

## Memory Use

`Variant` tries to be quite efficient with respect to memory use.
This section gives a rough idea of how much memory is used by the
current implementation. The information here is subject to change
in the future.

The memory allocated by `Variant` can be grouped into 4 broad
purposes: memory for serialised data, memory for the type
information cache, buffer management memory and memory for the
`Variant` structure itself.

## Serialised Data Memory

This is the memory that is used for storing GVariant data in
serialised form. This is what would be sent over the network or
what would end up on disk, not counting any indicator of the
endianness, or of the length or type of the top-level variant.

The amount of memory required to store a boolean is 1 byte. 16,
32 and 64 bit integers and double precision floating point numbers
use their "natural" size. Strings (including object path and
signature strings) are stored with a nul terminator, and as such
use the length of the string plus 1 byte.

Maybe types use no space at all to represent the null value and
use the same amount of space (sometimes plus one byte) as the
equivalent non-maybe-typed value to represent the non-null case.

Arrays use the amount of space required to store each of their
members, concatenated. Additionally, if the items stored in an
array are not of a fixed-size (ie: strings, other arrays, etc)
then an additional framing offset is stored for each item. The
size of this offset is either 1, 2 or 4 bytes depending on the
overall size of the container. Additionally, extra padding bytes
are added as required for alignment of child values.

Tuples (including dictionary entries) use the amount of space
required to store each of their members, concatenated, plus one
framing offset (as per arrays) for each non-fixed-sized item in
the tuple, except for the last one. Additionally, extra padding
bytes are added as required for alignment of child values.

Variants use the same amount of space as the item inside of the
variant, plus 1 byte, plus the length of the type string for the
item inside the variant.

As an example, consider a dictionary mapping strings to variants.
In the case that the dictionary is empty, 0 bytes are required for
the serialisation.

If we add an item "width" that maps to the int32 value of 500 then
we will use 4 byte to store the int32 (so 6 for the variant
containing it) and 6 bytes for the string. The variant must be
aligned to 8 after the 6 bytes of the string, so that's 2 extra
bytes. 6 (string) + 2 (padding) + 6 (variant) is 14 bytes used
for the dictionary entry. An additional 1 byte is added to the
array as a framing offset making a total of 15 bytes.

If we add another entry, "title" that maps to a nullable string
that happens to have a value of null, then we use 0 bytes for the
null value (and 3 bytes for the variant to contain it along with
its type string) plus 6 bytes for the string. Again, we need 2
padding bytes. That makes a total of 6 + 2 + 3 = 11 bytes.

We now require extra padding between the two items in the array.
After the 14 bytes of the first item, that's 2 bytes required.
We now require 2 framing offsets for an extra two
bytes. 14 + 2 + 11 + 2 = 29 bytes to encode the entire two-item
dictionary.

## Type Information Cache

For each GVariant type that currently exists in the program a type
information structure is kept in the type information cache. The
type information structure is required for rapid deserialisation.

Continuing with the above example, if a `Variant` exists with the
type "a{sv}" then a type information struct will exist for
"a{sv}", "{sv}", "s", and "v". Multiple uses of the same type
will share the same type information. Additionally, all
single-digit types are stored in read-only static memory and do
not contribute to the writable memory footprint of a program using
`Variant`.

Aside from the type information structures stored in read-only
memory, there are two forms of type information. One is used for
container types where there is a single element type: arrays and
maybe types. The other is used for container types where there
are multiple element types: tuples and dictionary entries.

Array type info structures are 6 * sizeof (void *), plus the
memory required to store the type string itself. This means that
on 32-bit systems, the cache entry for "a{sv}" would require 30
bytes of memory (plus malloc overhead).

Tuple type info structures are 6 * sizeof (void *), plus 4 *
sizeof (void *) for each item in the tuple, plus the memory
required to store the type string itself. A 2-item tuple, for
example, would have a type information structure that consumed
writable memory in the size of 14 * sizeof (void *) (plus type
string) This means that on 32-bit systems, the cache entry for
"{sv}" would require 61 bytes of memory (plus malloc overhead).

This means that in total, for our "a{sv}" example, 91 bytes of
type information would be allocated.

The type information cache, additionally, uses a `HashTable` to
store and lookup the cached items and stores a pointer to this
hash table in static storage. The hash table is freed when there
are zero items in the type cache.

Although these sizes may seem large it is important to remember
that a program will probably only have a very small number of
different types of values in it and that only one type information
structure is required for many different values of the same type.

## Buffer Management Memory

`Variant` uses an internal buffer management structure to deal
with the various different possible sources of serialised data
that it uses. The buffer is responsible for ensuring that the
correct call is made when the data is no longer in use by
`Variant`. This may involve a `g_free` or a `g_slice_free` or
even `MappedFile::unref`.

One buffer management structure is used for each chunk of
serialised data. The size of the buffer management structure
is 4 * (void *). On 32-bit systems, that's 16 bytes.

## GVariant structure

The size of a `Variant` structure is 6 * (void *). On 32-bit
systems, that's 24 bytes.

`Variant` structures only exist if they are explicitly created
with API calls. For example, if a `Variant` is constructed out of
serialised data for the example given above (with the dictionary)
then although there are 9 individual values that comprise the
entire dictionary (two keys, two values, two variants containing
the values, two dictionary entries, plus the dictionary itself),
only 1 `Variant` instance exists -- the one referring to the
dictionary.

If calls are made to start accessing the other values then
`Variant` instances will exist for those values only for as long
as they are in use (ie: until you call `Variant::unref`). The
type information is shared. The serialised data and the buffer
management structure for that serialised data is shared by the
child.

## Summary

To put the entire example together, for our dictionary mapping
strings to variants (with two entries, as given above), we are
using 91 bytes of memory for type information, 29 bytes of memory
for the serialised data, 16 bytes for buffer management and 24
bytes for the `Variant` instance, or a total of 160 bytes, plus
malloc overhead. If we were to use `Variant::get_child_value` to
access the two dictionary entries, we would use an additional 48
bytes. If we were to have other dictionaries of the same type, we
would use more memory for the serialised data and buffer
management for those dictionaries, but the type information would
be shared.
<!-- impl Variant::fn new -->
Creates a new `Variant` instance.

Think of this function as an analogue to `g_strdup_printf`.

The type of the created instance and the arguments that are expected
by this function are determined by `format_string`. See the section on
[GVariant format strings][gvariant-format-strings]. Please note that
the syntax of the format string is very likely to be extended in the
future.

The first character of the format string must not be '*' '?' '@' or
'r'; in essence, a new `Variant` must always be constructed by this
function (and not merely passed through it unmodified).

Note that the arguments must be of the correct width for their types
specified in `format_string`. This can be achieved by casting them. See
the [GVariant varargs documentation][gvariant-varargs].


```C
MyFlags some_flags = FLAG_ONE | FLAG_TWO;
const gchar *some_strings[] = { "a", "b", "c", NULL };
GVariant *new_variant;

new_variant = g_variant_new ("(t^as)",
                             // This cast is required.
                             (guint64) some_flags,
                             some_strings);
```
## `format_string`
a `Variant` format string

# Returns

a new floating `Variant` instance
<!-- impl Variant::fn new_array -->
Creates a new `Variant` array from `children`.

`child_type` must be non-`None` if `n_children` is zero. Otherwise, the
child type is determined by inspecting the first element of the
`children` array. If `child_type` is non-`None` then it must be a
definite type.

The items of the array are taken from the `children` array. No entry
in the `children` array may be `None`.

All items in the array must have the same type, which must be the
same as `child_type`, if given.

If the `children` are floating references (see `Variant::ref_sink`), the
new instance takes ownership of them as if via `Variant::ref_sink`.
## `child_type`
the element type of the new array
## `children`
an array of
 `Variant` pointers, the children
## `n_children`
the length of `children`

# Returns

a floating reference to a new `Variant` array
<!-- impl Variant::fn new_boolean -->
Creates a new boolean `Variant` instance -- either `true` or `false`.
## `value`
a `gboolean` value

# Returns

a floating reference to a new boolean `Variant` instance
<!-- impl Variant::fn new_byte -->
Creates a new byte `Variant` instance.
## `value`
a `guint8` value

# Returns

a floating reference to a new byte `Variant` instance
<!-- impl Variant::fn new_bytestring -->
Creates an array-of-bytes `Variant` with the contents of `string`.
This function is just like `Variant::new_string` except that the
string need not be valid UTF-8.

The nul terminator character at the end of the string is stored in
the array.
## `string`
a normal
 nul-terminated string in no particular encoding

# Returns

a floating reference to a new bytestring `Variant` instance
<!-- impl Variant::fn new_bytestring_array -->
Constructs an array of bytestring `Variant` from the given array of
strings.

If `length` is -1 then `strv` is `None`-terminated.
## `strv`
an array of strings
## `length`
the length of `strv`, or -1

# Returns

a new floating `Variant` instance
<!-- impl Variant::fn new_dict_entry -->
Creates a new dictionary entry `Variant`. `key` and `value` must be
non-`None`. `key` must be a value of a basic type (ie: not a container).

If the `key` or `value` are floating references (see `Variant::ref_sink`),
the new instance takes ownership of them as if via `Variant::ref_sink`.
## `key`
a basic `Variant`, the key
## `value`
a `Variant`, the value

# Returns

a floating reference to a new dictionary entry `Variant`
<!-- impl Variant::fn new_double -->
Creates a new double `Variant` instance.
## `value`
a `gdouble` floating point value

# Returns

a floating reference to a new double `Variant` instance
<!-- impl Variant::fn new_fixed_array -->
Constructs a new array `Variant` instance, where the elements are
of `element_type` type.

`elements` must be an array with fixed-sized elements. Numeric types are
fixed-size as are tuples containing only other fixed-sized types.

`element_size` must be the size of a single element in the array.
For example, if calling this function for an array of 32-bit integers,
you might say sizeof(gint32). This value isn't used except for the purpose
of a double-check that the form of the serialised data matches the caller's
expectation.

`n_elements` must be the length of the `elements` array.
## `element_type`
the `VariantType` of each element
## `elements`
a pointer to the fixed array of contiguous elements
## `n_elements`
the number of elements
## `element_size`
the size of each element

# Returns

a floating reference to a new array `Variant` instance
<!-- impl Variant::fn new_from_bytes -->
Constructs a new serialised-mode `Variant` instance. This is the
inner interface for creation of new serialised values that gets
called from various functions in gvariant.c.

A reference is taken on `bytes`.
## `type_`
a `VariantType`
## `bytes`
a `Bytes`
## `trusted`
if the contents of `bytes` are trusted

# Returns

a new `Variant` with a floating reference
<!-- impl Variant::fn new_from_data -->
Creates a new `Variant` instance from serialised data.

`type_` is the type of `Variant` instance that will be constructed.
The interpretation of `data` depends on knowing the type.

`data` is not modified by this function and must remain valid with an
unchanging value until such a time as `notify` is called with
`user_data`. If the contents of `data` change before that time then
the result is undefined.

If `data` is trusted to be serialised data in normal form then
`trusted` should be `true`. This applies to serialised data created
within this process or read from a trusted location on the disk (such
as a file installed in /usr/lib alongside your application). You
should set trusted to `false` if `data` is read from the network, a
file in the user's home directory, etc.

If `data` was not stored in this machine's native endianness, any multi-byte
numeric values in the returned variant will also be in non-native
endianness. `Variant::byteswap` can be used to recover the original values.

`notify` will be called with `user_data` when `data` is no longer
needed. The exact time of this call is unspecified and might even be
before this function returns.
## `type_`
a definite `VariantType`
## `data`
the serialised data
## `size`
the size of `data`
## `trusted`
`true` if `data` is definitely in normal form
## `notify`
function to call when `data` is no longer needed
## `user_data`
data for `notify`

# Returns

a new floating `Variant` of type `type_`
<!-- impl Variant::fn new_handle -->
Creates a new handle `Variant` instance.

By convention, handles are indexes into an array of file descriptors
that are sent alongside a D-Bus message. If you're not interacting
with D-Bus, you probably don't need them.
## `value`
a `gint32` value

# Returns

a floating reference to a new handle `Variant` instance
<!-- impl Variant::fn new_int16 -->
Creates a new int16 `Variant` instance.
## `value`
a `gint16` value

# Returns

a floating reference to a new int16 `Variant` instance
<!-- impl Variant::fn new_int32 -->
Creates a new int32 `Variant` instance.
## `value`
a `gint32` value

# Returns

a floating reference to a new int32 `Variant` instance
<!-- impl Variant::fn new_int64 -->
Creates a new int64 `Variant` instance.
## `value`
a `gint64` value

# Returns

a floating reference to a new int64 `Variant` instance
<!-- impl Variant::fn new_maybe -->
Depending on if `child` is `None`, either wraps `child` inside of a
maybe container or creates a Nothing instance for the given `type_`.

At least one of `child_type` and `child` must be non-`None`.
If `child_type` is non-`None` then it must be a definite type.
If they are both non-`None` then `child_type` must be the type
of `child`.

If `child` is a floating reference (see `Variant::ref_sink`), the new
instance takes ownership of `child`.
## `child_type`
the `VariantType` of the child, or `None`
## `child`
the child value, or `None`

# Returns

a floating reference to a new `Variant` maybe instance
<!-- impl Variant::fn new_object_path -->
Creates a D-Bus object path `Variant` with the contents of `string`.
`string` must be a valid D-Bus object path. Use
`Variant::is_object_path` if you're not sure.
## `object_path`
a normal C nul-terminated string

# Returns

a floating reference to a new object path `Variant` instance
<!-- impl Variant::fn new_objv -->
Constructs an array of object paths `Variant` from the given array of
strings.

Each string must be a valid `Variant` object path; see
`Variant::is_object_path`.

If `length` is -1 then `strv` is `None`-terminated.
## `strv`
an array of strings
## `length`
the length of `strv`, or -1

# Returns

a new floating `Variant` instance
<!-- impl Variant::fn new_parsed -->
Parses `format` and returns the result.

`format` must be a text format `Variant` with one extension: at any
point that a value may appear in the text, a '%' character followed
by a GVariant format string (as per `Variant::new`) may appear. In
that case, the same arguments are collected from the argument list as
`Variant::new` would have collected.

Note that the arguments must be of the correct width for their types
specified in `format`. This can be achieved by casting them. See
the [GVariant varargs documentation][gvariant-varargs].

Consider this simple example:

```C
 g_variant_new_parsed ("[('one', 1), ('two', %i), (%s, 3)]", 2, "three");
```

In the example, the variable argument parameters are collected and
filled in as if they were part of the original string to produce the
result of

```C
[('one', 1), ('two', 2), ('three', 3)]
```

This function is intended only to be used with `format` as a string
literal. Any parse error is fatal to the calling process. If you
want to parse data from untrusted sources, use `Variant::parse`.

You may not use this function to return, unmodified, a single
`Variant` pointer from the argument list. ie: `format` may not solely
be anything along the lines of "%*", "%?", "\%r", or anything starting
with "%@".
## `format`
a text format `Variant`

# Returns

a new floating `Variant` instance
<!-- impl Variant::fn new_parsed_va -->
Parses `format` and returns the result.

This is the version of `Variant::new_parsed` intended to be used
from libraries.

The return value will be floating if it was a newly created GVariant
instance. In the case that `format` simply specified the collection
of a `Variant` pointer (eg: `format` was "%*") then the collected
`Variant` pointer will be returned unmodified, without adding any
additional references.

Note that the arguments in `app` must be of the correct width for their types
specified in `format` when collected into the `va_list`. See
the [GVariant varargs documentation][gvariant-varargs].

In order to behave correctly in all cases it is necessary for the
calling function to `Variant::ref_sink` the return result before
returning control to the user that originally provided the pointer.
At this point, the caller will have their own full reference to the
result. This can also be done by adding the result to a container,
or by passing it to another `Variant::new` call.
## `format`
a text format `Variant`
## `app`
a pointer to a `va_list`

# Returns

a new, usually floating, `Variant`
<!-- impl Variant::fn new_printf -->
Creates a string-type GVariant using printf formatting.

This is similar to calling `g_strdup_printf` and then
`Variant::new_string` but it saves a temporary variable and an
unnecessary copy.
## `format_string`
a printf-style format string

# Returns

a floating reference to a new string
 `Variant` instance
<!-- impl Variant::fn new_signature -->
Creates a D-Bus type signature `Variant` with the contents of
`string`. `string` must be a valid D-Bus type signature. Use
`Variant::is_signature` if you're not sure.
## `signature`
a normal C nul-terminated string

# Returns

a floating reference to a new signature `Variant` instance
<!-- impl Variant::fn new_string -->
Creates a string `Variant` with the contents of `string`.

`string` must be valid UTF-8, and must not be `None`. To encode
potentially-`None` strings, use `Variant::new` with `ms` as the
[format string][gvariant-format-strings-maybe-types].
## `string`
a normal UTF-8 nul-terminated string

# Returns

a floating reference to a new string `Variant` instance
<!-- impl Variant::fn new_strv -->
Constructs an array of strings `Variant` from the given array of
strings.

If `length` is -1 then `strv` is `None`-terminated.
## `strv`
an array of strings
## `length`
the length of `strv`, or -1

# Returns

a new floating `Variant` instance
<!-- impl Variant::fn new_take_string -->
Creates a string `Variant` with the contents of `string`.

`string` must be valid UTF-8, and must not be `None`. To encode
potentially-`None` strings, use this with `Variant::new_maybe`.

This function consumes `string`. `g_free` will be called on `string`
when it is no longer required.

You must not modify or access `string` in any other way after passing
it to this function. It is even possible that `string` is immediately
freed.
## `string`
a normal UTF-8 nul-terminated string

# Returns

a floating reference to a new string
 `Variant` instance
<!-- impl Variant::fn new_tuple -->
Creates a new tuple `Variant` out of the items in `children`. The
type is determined from the types of `children`. No entry in the
`children` array may be `None`.

If `n_children` is 0 then the unit tuple is constructed.

If the `children` are floating references (see `Variant::ref_sink`), the
new instance takes ownership of them as if via `Variant::ref_sink`.
## `children`
the items to make the tuple out of
## `n_children`
the length of `children`

# Returns

a floating reference to a new `Variant` tuple
<!-- impl Variant::fn new_uint16 -->
Creates a new uint16 `Variant` instance.
## `value`
a `guint16` value

# Returns

a floating reference to a new uint16 `Variant` instance
<!-- impl Variant::fn new_uint32 -->
Creates a new uint32 `Variant` instance.
## `value`
a `guint32` value

# Returns

a floating reference to a new uint32 `Variant` instance
<!-- impl Variant::fn new_uint64 -->
Creates a new uint64 `Variant` instance.
## `value`
a `guint64` value

# Returns

a floating reference to a new uint64 `Variant` instance
<!-- impl Variant::fn new_va -->
This function is intended to be used by libraries based on
`Variant` that want to provide `Variant::new`-like functionality
to their users.

The API is more general than `Variant::new` to allow a wider range
of possible uses.

`format_string` must still point to a valid format string, but it only
needs to be nul-terminated if `endptr` is `None`. If `endptr` is
non-`None` then it is updated to point to the first character past the
end of the format string.

`app` is a pointer to a `va_list`. The arguments, according to
`format_string`, are collected from this `va_list` and the list is left
pointing to the argument following the last.

Note that the arguments in `app` must be of the correct width for their
types specified in `format_string` when collected into the `va_list`.
See the [GVariant varargs documentation][gvariant-varargs].

These two generalisations allow mixing of multiple calls to
`Variant::new_va` and `Variant::get_va` within a single actual
varargs call by the user.

The return value will be floating if it was a newly created GVariant
instance (for example, if the format string was "(ii)"). In the case
that the format_string was '*', '?', 'r', or a format starting with
'@' then the collected `Variant` pointer will be returned unmodified,
without adding any additional references.

In order to behave correctly in all cases it is necessary for the
calling function to `Variant::ref_sink` the return result before
returning control to the user that originally provided the pointer.
At this point, the caller will have their own full reference to the
result. This can also be done by adding the result to a container,
or by passing it to another `Variant::new` call.
## `format_string`
a string that is prefixed with a format string
## `endptr`
location to store the end pointer,
 or `None`
## `app`
a pointer to a `va_list`

# Returns

a new, usually floating, `Variant`
<!-- impl Variant::fn new_variant -->
Boxes `value`. The result is a `Variant` instance representing a
variant containing the original value.

If `child` is a floating reference (see `Variant::ref_sink`), the new
instance takes ownership of `child`.
## `value`
a `Variant` instance

# Returns

a floating reference to a new variant `Variant` instance
<!-- impl Variant::fn byteswap -->
Performs a byteswapping operation on the contents of `self`. The
result is that all multi-byte numeric data contained in `self` is
byteswapped. That includes 16, 32, and 64bit signed and unsigned
integers as well as file handles and double precision floating point
values.

This function is an identity mapping on any value that does not
contain multi-byte numeric data. That include strings, booleans,
bytes and containers containing only these things (recursively).

The returned value is always in normal form and is marked as trusted.

# Returns

the byteswapped form of `self`
<!-- impl Variant::fn check_format_string -->
Checks if calling `Variant::get` with `format_string` on `self` would
be valid from a type-compatibility standpoint. `format_string` is
assumed to be a valid format string (from a syntactic standpoint).

If `copy_only` is `true` then this function additionally checks that it
would be safe to call `Variant::unref` on `self` immediately after
the call to `Variant::get` without invalidating the result. This is
only possible if deep copies are made (ie: there are no pointers to
the data inside of the soon-to-be-freed `Variant` instance). If this
check fails then a `g_critical` is printed and `false` is returned.

This function is meant to be used by functions that wish to provide
varargs accessors to `Variant` values of uncertain values (eg:
`Variant::lookup` or `g_menu_model_get_item_attribute`).
## `format_string`
a valid `Variant` format string
## `copy_only`
`true` to ensure the format string makes deep copies

# Returns

`true` if `format_string` is safe to use
<!-- impl Variant::fn classify -->
Classifies `self` according to its top-level type.

# Returns

the `VariantClass` of `self`
<!-- impl Variant::fn compare -->
Compares `self` and `two`.

The types of `self` and `two` are `gconstpointer` only to allow use of
this function with `Tree`, `PtrArray`, etc. They must each be a
`Variant`.

Comparison is only defined for basic types (ie: booleans, numbers,
strings). For booleans, `false` is less than `true`. Numbers are
ordered in the usual way. Strings are in ASCII lexographical order.

It is a programmer error to attempt to compare container values or
two values that have types that are not exactly equal. For example,
you cannot compare a 32-bit signed integer with a 32-bit unsigned
integer. Also note that this function is not particularly
well-behaved when it comes to comparison of doubles; in particular,
the handling of incomparable values (ie: NaN) is undefined.

If you only require an equality comparison, `Variant::equal` is more
general.
## `two`
a `Variant` instance of the same type

# Returns

negative value if a < b;
 zero if a = b;
 positive value if a > b.
<!-- impl Variant::fn dup_bytestring -->
Similar to `Variant::get_bytestring` except that instead of
returning a constant string, the string is duplicated.

The return value must be freed using `g_free`.
## `length`
a pointer to a `gsize`, to store
 the length (not including the nul terminator)

# Returns


 a newly allocated string
<!-- impl Variant::fn dup_bytestring_array -->
Gets the contents of an array of array of bytes `Variant`. This call
makes a deep copy; the return result should be released with
`g_strfreev`.

If `length` is non-`None` then the number of elements in the result is
stored there. In any case, the resulting array will be
`None`-terminated.

For an empty array, `length` will be set to 0 and a pointer to a
`None` pointer will be returned.
## `length`
the length of the result, or `None`

# Returns

an array of strings
<!-- impl Variant::fn dup_objv -->
Gets the contents of an array of object paths `Variant`. This call
makes a deep copy; the return result should be released with
`g_strfreev`.

If `length` is non-`None` then the number of elements in the result
is stored there. In any case, the resulting array will be
`None`-terminated.

For an empty array, `length` will be set to 0 and a pointer to a
`None` pointer will be returned.
## `length`
the length of the result, or `None`

# Returns

an array of strings
<!-- impl Variant::fn dup_string -->
Similar to `Variant::get_string` except that instead of returning
a constant string, the string is duplicated.

The string will always be UTF-8 encoded.

The return value must be freed using `g_free`.
## `length`
a pointer to a `gsize`, to store the length

# Returns

a newly allocated string, UTF-8 encoded
<!-- impl Variant::fn dup_strv -->
Gets the contents of an array of strings `Variant`. This call
makes a deep copy; the return result should be released with
`g_strfreev`.

If `length` is non-`None` then the number of elements in the result
is stored there. In any case, the resulting array will be
`None`-terminated.

For an empty array, `length` will be set to 0 and a pointer to a
`None` pointer will be returned.
## `length`
the length of the result, or `None`

# Returns

an array of strings
<!-- impl Variant::fn equal -->
Checks if `self` and `two` have the same type and value.

The types of `self` and `two` are `gconstpointer` only to allow use of
this function with `HashTable`. They must each be a `Variant`.
## `two`
a `Variant` instance

# Returns

`true` if `self` and `two` are equal
<!-- impl Variant::fn get -->
Deconstructs a `Variant` instance.

Think of this function as an analogue to `scanf`.

The arguments that are expected by this function are entirely
determined by `format_string`. `format_string` also restricts the
permissible types of `self`. It is an error to give a value with
an incompatible type. See the section on
[GVariant format strings][gvariant-format-strings].
Please note that the syntax of the format string is very likely to be
extended in the future.

`format_string` determines the C types that are used for unpacking
the values and also determines if the values are copied or borrowed,
see the section on
[GVariant format strings][gvariant-format-strings-pointers].
## `format_string`
a `Variant` format string
<!-- impl Variant::fn get_boolean -->
Returns the boolean value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_BOOLEAN`.

# Returns

`true` or `false`
<!-- impl Variant::fn get_byte -->
Returns the byte value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_BYTE`.

# Returns

a `guint8`
<!-- impl Variant::fn get_bytestring -->
Returns the string value of a `Variant` instance with an
array-of-bytes type. The string has no particular encoding.

If the array does not end with a nul terminator character, the empty
string is returned. For this reason, you can always trust that a
non-`None` nul-terminated string will be returned by this function.

If the array contains a nul terminator character somewhere other than
the last byte then the returned string is the string, up to the first
such nul character.

`Variant::get_fixed_array` should be used instead if the array contains
arbitrary data that could not be nul-terminated or could contain nul bytes.

It is an error to call this function with a `self` that is not an
array of bytes.

The return value remains valid as long as `self` exists.

# Returns


 the constant string
<!-- impl Variant::fn get_bytestring_array -->
Gets the contents of an array of array of bytes `Variant`. This call
makes a shallow copy; the return result should be released with
`g_free`, but the individual strings must not be modified.

If `length` is non-`None` then the number of elements in the result is
stored there. In any case, the resulting array will be
`None`-terminated.

For an empty array, `length` will be set to 0 and a pointer to a
`None` pointer will be returned.
## `length`
the length of the result, or `None`

# Returns

an array of constant strings
<!-- impl Variant::fn get_child -->
Reads a child item out of a container `Variant` instance and
deconstructs it according to `format_string`. This call is
essentially a combination of `Variant::get_child_value` and
`Variant::get`.

`format_string` determines the C types that are used for unpacking
the values and also determines if the values are copied or borrowed,
see the section on
[GVariant format strings][gvariant-format-strings-pointers].
## `index_`
the index of the child to deconstruct
## `format_string`
a `Variant` format string
<!-- impl Variant::fn get_child_value -->
Reads a child item out of a container `Variant` instance. This
includes variants, maybes, arrays, tuples and dictionary
entries. It is an error to call this function on any other type of
`Variant`.

It is an error if `index_` is greater than the number of child items
in the container. See `Variant::n_children`.

The returned value is never floating. You should free it with
`Variant::unref` when you're done with it.

There may be implementation specific restrictions on deeply nested values,
which would result in the unit tuple being returned as the child value,
instead of further nested children. `Variant` is guaranteed to handle
nesting up to at least 64 levels.

This function is O(1).
## `index_`
the index of the child to fetch

# Returns

the child at the specified index
<!-- impl Variant::fn get_data -->
Returns a pointer to the serialised form of a `Variant` instance.
The returned data may not be in fully-normalised form if read from an
untrusted source. The returned data must not be freed; it remains
valid for as long as `self` exists.

If `self` is a fixed-sized value that was deserialised from a
corrupted serialised container then `None` may be returned. In this
case, the proper thing to do is typically to use the appropriate
number of nul bytes in place of `self`. If `self` is not fixed-sized
then `None` is never returned.

In the case that `self` is already in serialised form, this function
is O(1). If the value is not already in serialised form,
serialisation occurs implicitly and is approximately O(n) in the size
of the result.

To deserialise the data returned by this function, in addition to the
serialised data, you must know the type of the `Variant`, and (if the
machine might be different) the endianness of the machine that stored
it. As a result, file formats or network messages that incorporate
serialised `GVariants` must include this information either
implicitly (for instance "the file always contains a
`G_VARIANT_TYPE_VARIANT` and it is always in little-endian order") or
explicitly (by storing the type and/or endianness in addition to the
serialised data).

# Returns

the serialised form of `self`, or `None`
<!-- impl Variant::fn get_data_as_bytes -->
Returns a pointer to the serialised form of a `Variant` instance.
The semantics of this function are exactly the same as
`Variant::get_data`, except that the returned `Bytes` holds
a reference to the variant data.

# Returns

A new `Bytes` representing the variant data
<!-- impl Variant::fn get_double -->
Returns the double precision floating point value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_DOUBLE`.

# Returns

a `gdouble`
<!-- impl Variant::fn get_fixed_array -->
Provides access to the serialised data for an array of fixed-sized
items.

`self` must be an array with fixed-sized elements. Numeric types are
fixed-size, as are tuples containing only other fixed-sized types.

`element_size` must be the size of a single element in the array,
as given by the section on
[serialized data memory][gvariant-serialised-data-memory].

In particular, arrays of these fixed-sized types can be interpreted
as an array of the given C type, with `element_size` set to the size
the appropriate type:
- `G_VARIANT_TYPE_INT16` (etc.): `gint16` (etc.)
- `G_VARIANT_TYPE_BOOLEAN`: `guchar` (not `gboolean`!)
- `G_VARIANT_TYPE_BYTE`: `guint8`
- `G_VARIANT_TYPE_HANDLE`: `guint32`
- `G_VARIANT_TYPE_DOUBLE`: `gdouble`

For example, if calling this function for an array of 32-bit integers,
you might say `sizeof(gint32)`. This value isn't used except for the purpose
of a double-check that the form of the serialised data matches the caller's
expectation.

`n_elements`, which must be non-`None`, is set equal to the number of
items in the array.
## `n_elements`
a pointer to the location to store the number of items
## `element_size`
the size of each element

# Returns

a pointer to
 the fixed array
<!-- impl Variant::fn get_handle -->
Returns the 32-bit signed integer value of `self`.

It is an error to call this function with a `self` of any type other
than `G_VARIANT_TYPE_HANDLE`.

By convention, handles are indexes into an array of file descriptors
that are sent alongside a D-Bus message. If you're not interacting
with D-Bus, you probably don't need them.

# Returns

a `gint32`
<!-- impl Variant::fn get_int16 -->
Returns the 16-bit signed integer value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_INT16`.

# Returns

a `gint16`
<!-- impl Variant::fn get_int32 -->
Returns the 32-bit signed integer value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_INT32`.

# Returns

a `gint32`
<!-- impl Variant::fn get_int64 -->
Returns the 64-bit signed integer value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_INT64`.

# Returns

a `gint64`
<!-- impl Variant::fn get_maybe -->
Given a maybe-typed `Variant` instance, extract its value. If the
value is Nothing, then this function returns `None`.

# Returns

the contents of `self`, or `None`
<!-- impl Variant::fn get_normal_form -->
Gets a `Variant` instance that has the same value as `self` and is
trusted to be in normal form.

If `self` is already trusted to be in normal form then a new
reference to `self` is returned.

If `self` is not already trusted, then it is scanned to check if it
is in normal form. If it is found to be in normal form then it is
marked as trusted and a new reference to it is returned.

If `self` is found not to be in normal form then a new trusted
`Variant` is created with the same value as `self`.

It makes sense to call this function if you've received `Variant`
data from untrusted sources and you want to ensure your serialised
output is definitely in normal form.

If `self` is already in normal form, a new reference will be returned
(which will be floating if `self` is floating). If it is not in normal form,
the newly created `Variant` will be returned with a single non-floating
reference. Typically, `Variant::take_ref` should be called on the return
value from this function to guarantee ownership of a single non-floating
reference to it.

# Returns

a trusted `Variant`
<!-- impl Variant::fn get_objv -->
Gets the contents of an array of object paths `Variant`. This call
makes a shallow copy; the return result should be released with
`g_free`, but the individual strings must not be modified.

If `length` is non-`None` then the number of elements in the result
is stored there. In any case, the resulting array will be
`None`-terminated.

For an empty array, `length` will be set to 0 and a pointer to a
`None` pointer will be returned.
## `length`
the length of the result, or `None`

# Returns

an array of constant strings
<!-- impl Variant::fn get_size -->
Determines the number of bytes that would be required to store `self`
with `Variant::store`.

If `self` has a fixed-sized type then this function always returned
that fixed size.

In the case that `self` is already in serialised form or the size has
already been calculated (ie: this function has been called before)
then this function is O(1). Otherwise, the size is calculated, an
operation which is approximately O(n) in the number of values
involved.

# Returns

the serialised size of `self`
<!-- impl Variant::fn get_string -->
Returns the string value of a `Variant` instance with a string
type. This includes the types `G_VARIANT_TYPE_STRING`,
`G_VARIANT_TYPE_OBJECT_PATH` and `G_VARIANT_TYPE_SIGNATURE`.

The string will always be UTF-8 encoded, and will never be `None`.

If `length` is non-`None` then the length of the string (in bytes) is
returned there. For trusted values, this information is already
known. For untrusted values, a `strlen` will be performed.

It is an error to call this function with a `self` of any type
other than those three.

The return value remains valid as long as `self` exists.
## `length`
a pointer to a `gsize`,
 to store the length

# Returns

the constant string, UTF-8 encoded
<!-- impl Variant::fn get_strv -->
Gets the contents of an array of strings `Variant`. This call
makes a shallow copy; the return result should be released with
`g_free`, but the individual strings must not be modified.

If `length` is non-`None` then the number of elements in the result
is stored there. In any case, the resulting array will be
`None`-terminated.

For an empty array, `length` will be set to 0 and a pointer to a
`None` pointer will be returned.
## `length`
the length of the result, or `None`

# Returns

an array of constant strings
<!-- impl Variant::fn get_type -->
Determines the type of `self`.

The return value is valid for the lifetime of `self` and must not
be freed.

# Returns

a `VariantType`
<!-- impl Variant::fn get_type_string -->
Returns the type string of `self`. Unlike the result of calling
`VariantType::peek_string`, this string is nul-terminated. This
string belongs to `Variant` and must not be freed.

# Returns

the type string for the type of `self`
<!-- impl Variant::fn get_uint16 -->
Returns the 16-bit unsigned integer value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_UINT16`.

# Returns

a `guint16`
<!-- impl Variant::fn get_uint32 -->
Returns the 32-bit unsigned integer value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_UINT32`.

# Returns

a `guint32`
<!-- impl Variant::fn get_uint64 -->
Returns the 64-bit unsigned integer value of `self`.

It is an error to call this function with a `self` of any type
other than `G_VARIANT_TYPE_UINT64`.

# Returns

a `guint64`
<!-- impl Variant::fn get_va -->
This function is intended to be used by libraries based on `Variant`
that want to provide `Variant::get`-like functionality to their
users.

The API is more general than `Variant::get` to allow a wider range
of possible uses.

`format_string` must still point to a valid format string, but it only
need to be nul-terminated if `endptr` is `None`. If `endptr` is
non-`None` then it is updated to point to the first character past the
end of the format string.

`app` is a pointer to a `va_list`. The arguments, according to
`format_string`, are collected from this `va_list` and the list is left
pointing to the argument following the last.

These two generalisations allow mixing of multiple calls to
`Variant::new_va` and `Variant::get_va` within a single actual
varargs call by the user.

`format_string` determines the C types that are used for unpacking
the values and also determines if the values are copied or borrowed,
see the section on
[GVariant format strings][gvariant-format-strings-pointers].
## `format_string`
a string that is prefixed with a format string
## `endptr`
location to store the end pointer,
 or `None`
## `app`
a pointer to a `va_list`
<!-- impl Variant::fn get_variant -->
Unboxes `self`. The result is the `Variant` instance that was
contained in `self`.

# Returns

the item contained in the variant
<!-- impl Variant::fn hash -->
Generates a hash value for a `Variant` instance.

The output of this function is guaranteed to be the same for a given
value only per-process. It may change between different processor
architectures or even different versions of GLib. Do not use this
function as a basis for building protocols or file formats.

The type of `self` is `gconstpointer` only to allow use of this
function with `HashTable`. `self` must be a `Variant`.

# Returns

a hash value corresponding to `self`
<!-- impl Variant::fn is_container -->
Checks if `self` is a container.

# Returns

`true` if `self` is a container
<!-- impl Variant::fn is_floating -->
Checks whether `self` has a floating reference count.

This function should only ever be used to assert that a given variant
is or is not floating, or for debug purposes. To acquire a reference
to a variant that might be floating, always use `Variant::ref_sink`
or `Variant::take_ref`.

See `Variant::ref_sink` for more information about floating reference
counts.

# Returns

whether `self` is floating
<!-- impl Variant::fn is_normal_form -->
Checks if `self` is in normal form.

The main reason to do this is to detect if a given chunk of
serialised data is in normal form: load the data into a `Variant`
using `Variant::new_from_data` and then use this function to
check.

If `self` is found to be in normal form then it will be marked as
being trusted. If the value was already marked as being trusted then
this function will immediately return `true`.

There may be implementation specific restrictions on deeply nested values.
GVariant is guaranteed to handle nesting up to at least 64 levels.

# Returns

`true` if `self` is in normal form
<!-- impl Variant::fn is_of_type -->
Checks if a value has a type matching the provided type.
## `type_`
a `VariantType`

# Returns

`true` if the type of `self` matches `type_`
<!-- impl Variant::fn iter_new -->
Creates a heap-allocated `VariantIter` for iterating over the items
in `self`.

Use `VariantIter::free` to free the return value when you no longer
need it.

A reference is taken to `self` and will be released only when
`VariantIter::free` is called.

# Returns

a new heap-allocated `VariantIter`
<!-- impl Variant::fn lookup -->
Looks up a value in a dictionary `Variant`.

This function is a wrapper around `Variant::lookup_value` and
`Variant::get`. In the case that `None` would have been returned,
this function returns `false`. Otherwise, it unpacks the returned
value and returns `true`.

`format_string` determines the C types that are used for unpacking
the values and also determines if the values are copied or borrowed,
see the section on
[GVariant format strings][gvariant-format-strings-pointers].

This function is currently implemented with a linear scan. If you
plan to do many lookups then `VariantDict` may be more efficient.
## `key`
the key to lookup in the dictionary
## `format_string`
a GVariant format string

# Returns

`true` if a value was unpacked
<!-- impl Variant::fn lookup_value -->
Looks up a value in a dictionary `Variant`.

This function works with dictionaries of the type a{s*} (and equally
well with type a{o*}, but we only further discuss the string case
for sake of clarity).

In the event that `self` has the type a{sv}, the `expected_type`
string specifies what type of value is expected to be inside of the
variant. If the value inside the variant has a different type then
`None` is returned. In the event that `self` has a value type other
than v then `expected_type` must directly match the key type and it is
used to unpack the value directly or an error occurs.

In either case, if `key` is not found in `self`, `None` is returned.

If the key is found and the value has the correct type, it is
returned. If `expected_type` was specified then any non-`None` return
value will have this type.

This function is currently implemented with a linear scan. If you
plan to do many lookups then `VariantDict` may be more efficient.
## `key`
the key to lookup in the dictionary
## `expected_type`
a `VariantType`, or `None`

# Returns

the value of the dictionary key, or `None`
<!-- impl Variant::fn n_children -->
Determines the number of children in a container `Variant` instance.
This includes variants, maybes, arrays, tuples and dictionary
entries. It is an error to call this function on any other type of
`Variant`.

For variants, the return value is always 1. For values with maybe
types, it is always zero or one. For arrays, it is the length of the
array. For tuples it is the number of tuple items (which depends
only on the type). For dictionary entries, it is always 2

This function is O(1).

# Returns

the number of children in the container
<!-- impl Variant::fn print -->
Pretty-prints `self` in the format understood by `Variant::parse`.

The format is described [here][gvariant-text].

If `type_annotate` is `true`, then type information is included in
the output.
## `type_annotate`
`true` if type information should be included in
 the output

# Returns

a newly-allocated string holding the result.
<!-- impl Variant::fn print_string -->
Behaves as `Variant::print`, but operates on a `String`.

If `string` is non-`None` then it is appended to and returned. Else,
a new empty `String` is allocated and it is returned.
## `string`
a `String`, or `None`
## `type_annotate`
`true` if type information should be included in
 the output

# Returns

a `String` containing the string
<!-- impl Variant::fn ref -->
Increases the reference count of `self`.

# Returns

the same `self`
<!-- impl Variant::fn ref_sink -->
`Variant` uses a floating reference count system. All functions with
names starting with `g_variant_new_` return floating
references.

Calling `Variant::ref_sink` on a `Variant` with a floating reference
will convert the floating reference into a full reference. Calling
`Variant::ref_sink` on a non-floating `Variant` results in an
additional normal reference being added.

In other words, if the `self` is floating, then this call "assumes
ownership" of the floating reference, converting it to a normal
reference. If the `self` is not floating, then this call adds a
new normal reference increasing the reference count by one.

All calls that result in a `Variant` instance being inserted into a
container will call `Variant::ref_sink` on the instance. This means
that if the value was just created (and has only its floating
reference) then the container will assume sole ownership of the value
at that point and the caller will not need to unreference it. This
makes certain common styles of programming much easier while still
maintaining normal refcounting semantics in situations where values
are not floating.

# Returns

the same `self`
<!-- impl Variant::fn store -->
Stores the serialised form of `self` at `data`. `data` should be
large enough. See `Variant::get_size`.

The stored data is in machine native byte order but may not be in
fully-normalised form if read from an untrusted source. See
`Variant::get_normal_form` for a solution.

As with `Variant::get_data`, to be able to deserialise the
serialised variant successfully, its type and (if the destination
machine might be different) its endianness must also be available.

This function is approximately O(n) in the size of `data`.
## `data`
the location to store the serialised data at
<!-- impl Variant::fn take_ref -->
If `self` is floating, sink it. Otherwise, do nothing.

Typically you want to use `Variant::ref_sink` in order to
automatically do the correct thing with respect to floating or
non-floating references, but there is one specific scenario where
this function is helpful.

The situation where this function is helpful is when creating an API
that allows the user to provide a callback function that returns a
`Variant`. We certainly want to allow the user the flexibility to
return a non-floating reference from this callback (for the case
where the value that is being returned already exists).

At the same time, the style of the `Variant` API makes it likely that
for newly-created `Variant` instances, the user can be saved some
typing if they are allowed to return a `Variant` with a floating
reference.

Using this function on the return value of the user's callback allows
the user to do whichever is more convenient for them. The caller
will alway receives exactly one full reference to the value: either
the one that was returned in the first place, or a floating reference
that has been converted to a full reference.

This function has an odd interaction when combined with
`Variant::ref_sink` running at the same time in another thread on
the same `Variant` instance. If `Variant::ref_sink` runs first then
the result will be that the floating reference is converted to a hard
reference. If `Variant::take_ref` runs first then the result will
be that the floating reference is converted to a hard reference and
an additional reference on top of that one is added. It is best to
avoid this situation.

# Returns

the same `self`
<!-- impl Variant::fn unref -->
Decreases the reference count of `self`. When its reference count
drops to 0, the memory used by the variant is freed.
<!-- impl Variant::fn is_object_path -->
Determines if a given string is a valid D-Bus object path. You
should ensure that a string is a valid D-Bus object path before
passing it to `Variant::new_object_path`.

A valid object path starts with `/` followed by zero or more
sequences of characters separated by `/` characters. Each sequence
must contain only the characters `[A-Z][a-z][0-9]_`. No sequence
(including the one following the final `/` character) may be empty.
## `string`
a normal C nul-terminated string

# Returns

`true` if `string` is a D-Bus object path
<!-- impl Variant::fn is_signature -->
Determines if a given string is a valid D-Bus type signature. You
should ensure that a string is a valid D-Bus type signature before
passing it to `Variant::new_signature`.

D-Bus type signatures consist of zero or more definite `VariantType`
strings in sequence.
## `string`
a normal C nul-terminated string

# Returns

`true` if `string` is a D-Bus type signature
<!-- impl Variant::fn parse -->
Parses a `Variant` from a text representation.

A single `Variant` is parsed from the content of `text`.

The format is described [here][gvariant-text].

The memory at `limit` will never be accessed and the parser behaves as
if the character at `limit` is the nul terminator. This has the
effect of bounding `text`.

If `endptr` is non-`None` then `text` is permitted to contain data
following the value that this function parses and `endptr` will be
updated to point to the first character past the end of the text
parsed by this function. If `endptr` is `None` and there is extra data
then an error is returned.

If `type_` is non-`None` then the value will be parsed to have that
type. This may result in additional parse errors (in the case that
the parsed value doesn't fit the type) but may also result in fewer
errors (in the case that the type would have been ambiguous, such as
with empty arrays).

In the event that the parsing is successful, the resulting `Variant`
is returned. It is never floating, and must be freed with
`Variant::unref`.

In case of any error, `None` will be returned. If `error` is non-`None`
then it will be set to reflect the error that occurred.

Officially, the language understood by the parser is "any string
produced by `Variant::print`".
## `type_`
a `VariantType`, or `None`
## `text`
a string containing a GVariant in text form
## `limit`
a pointer to the end of `text`, or `None`
## `endptr`
a location to store the end pointer, or `None`

# Returns

a non-floating reference to a `Variant`, or `None`
<!-- impl Variant::fn parse_error_print_context -->
Pretty-prints a message showing the context of a `Variant` parse
error within the string for which parsing was attempted.

The resulting string is suitable for output to the console or other
monospace media where newlines are treated in the usual way.

The message will typically look something like one of the following:


```text
unterminated string constant:
  (1, 2, 3, 'abc
            ^^^^
```

or


```text
unable to find a common type:
  [1, 2, 3, 'str']
   ^        ^^^^^
```

The format of the message may change in a future version.

`error` must have come from a failed attempt to `Variant::parse` and
`source_str` must be exactly the same string that caused the error.
If `source_str` was not nul-terminated when you passed it to
`Variant::parse` then you must add nul termination before using this
function.
## `error`
a `Error` from the `VariantParseError` domain
## `source_str`
the string that was given to the parser

# Returns

the printed message
<!-- impl Variant::fn parser_get_error_quark -->
Same as `g_variant_error_quark`.

# Deprecated

Use `Variant::parse_error_quark` instead.
<!-- struct VariantType -->
This section introduces the GVariant type system. It is based, in
large part, on the D-Bus type system, with two major changes and
some minor lifting of restrictions. The
[D-Bus specification](http://dbus.freedesktop.org/doc/dbus-specification.html),
therefore, provides a significant amount of
information that is useful when working with GVariant.

The first major change with respect to the D-Bus type system is the
introduction of maybe (or "nullable") types. Any type in GVariant can be
converted to a maybe type, in which case, "nothing" (or "null") becomes a
valid value. Maybe types have been added by introducing the
character "m" to type strings.

The second major change is that the GVariant type system supports the
concept of "indefinite types" -- types that are less specific than
the normal types found in D-Bus. For example, it is possible to speak
of "an array of any type" in GVariant, where the D-Bus type system
would require you to speak of "an array of integers" or "an array of
strings". Indefinite types have been added by introducing the
characters "*", "?" and "r" to type strings.

Finally, all arbitrary restrictions relating to the complexity of
types are lifted along with the restriction that dictionary entries
may only appear nested inside of arrays.

Just as in D-Bus, GVariant types are described with strings ("type
strings"). Subject to the differences mentioned above, these strings
are of the same form as those found in DBus. Note, however: D-Bus
always works in terms of messages and therefore individual type
strings appear nowhere in its interface. Instead, "signatures"
are a concatenation of the strings of the type of each argument in a
message. GVariant deals with single values directly so GVariant type
strings always describe the type of exactly one value. This means
that a D-Bus signature string is generally not a valid GVariant type
string -- except in the case that it is the signature of a message
containing exactly one argument.

An indefinite type is similar in spirit to what may be called an
abstract type in other type systems. No value can exist that has an
indefinite type as its type, but values can exist that have types
that are subtypes of indefinite types. That is to say,
`Variant::get_type` will never return an indefinite type, but
calling `Variant::is_of_type` with an indefinite type may return
`true`. For example, you cannot have a value that represents "an
array of no particular type", but you can have an "array of integers"
which certainly matches the type of "an array of no particular type",
since "array of integers" is a subtype of "array of no particular
type".

This is similar to how instances of abstract classes may not
directly exist in other type systems, but instances of their
non-abstract subtypes may. For example, in GTK, no object that has
the type of ``GtkBin`` can exist (since ``GtkBin`` is an abstract class),
but a ``GtkWindow`` can certainly be instantiated, and you would say
that the ``GtkWindow`` is a ``GtkBin`` (since ``GtkWindow`` is a subclass of
``GtkBin``).

## GVariant Type Strings

A GVariant type string can be any of the following:

- any basic type string (listed below)

- "v", "r" or "*"

- one of the characters 'a' or 'm', followed by another type string

- the character '(', followed by a concatenation of zero or more other
 type strings, followed by the character ')'

- the character '{', followed by a basic type string (see below),
 followed by another type string, followed by the character '}'

A basic type string describes a basic type (as per
`VariantType::is_basic`) and is always a single character in length.
The valid basic type strings are "b", "y", "n", "q", "i", "u", "x", "t",
"h", "d", "s", "o", "g" and "?".

The above definition is recursive to arbitrary depth. "aaaaai" and
"(ui(nq((y)))s)" are both valid type strings, as is
"a(aa(ui)(qna{ya(yd)}))". In order to not hit memory limits, `Variant`
imposes a limit on recursion depth of 65 nested containers. This is the
limit in the D-Bus specification (64) plus one to allow a `GDBusMessage` to
be nested in a top-level tuple.

The meaning of each of the characters is as follows:
- `b`: the type string of `G_VARIANT_TYPE_BOOLEAN`; a boolean value.
- `y`: the type string of `G_VARIANT_TYPE_BYTE`; a byte.
- `n`: the type string of `G_VARIANT_TYPE_INT16`; a signed 16 bit integer.
- `q`: the type string of `G_VARIANT_TYPE_UINT16`; an unsigned 16 bit integer.
- `i`: the type string of `G_VARIANT_TYPE_INT32`; a signed 32 bit integer.
- `u`: the type string of `G_VARIANT_TYPE_UINT32`; an unsigned 32 bit integer.
- `x`: the type string of `G_VARIANT_TYPE_INT64`; a signed 64 bit integer.
- `t`: the type string of `G_VARIANT_TYPE_UINT64`; an unsigned 64 bit integer.
- `h`: the type string of `G_VARIANT_TYPE_HANDLE`; a signed 32 bit value
 that, by convention, is used as an index into an array of file
 descriptors that are sent alongside a D-Bus message.
- `d`: the type string of `G_VARIANT_TYPE_DOUBLE`; a double precision
 floating point value.
- `s`: the type string of `G_VARIANT_TYPE_STRING`; a string.
- `o`: the type string of `G_VARIANT_TYPE_OBJECT_PATH`; a string in the form
 of a D-Bus object path.
- `g`: the type string of `G_VARIANT_TYPE_SIGNATURE`; a string in the form of
 a D-Bus type signature.
- `?`: the type string of `G_VARIANT_TYPE_BASIC`; an indefinite type that
 is a supertype of any of the basic types.
- `v`: the type string of `G_VARIANT_TYPE_VARIANT`; a container type that
 contain any other type of value.
- `a`: used as a prefix on another type string to mean an array of that
 type; the type string "ai", for example, is the type of an array of
 signed 32-bit integers.
- `m`: used as a prefix on another type string to mean a "maybe", or
 "nullable", version of that type; the type string "ms", for example,
 is the type of a value that maybe contains a string, or maybe contains
 nothing.
- `()`: used to enclose zero or more other concatenated type strings to
 create a tuple type; the type string "(is)", for example, is the type of
 a pair of an integer and a string.
- `r`: the type string of `G_VARIANT_TYPE_TUPLE`; an indefinite type that is
 a supertype of any tuple type, regardless of the number of items.
- `{}`: used to enclose a basic type string concatenated with another type
 string to create a dictionary entry type, which usually appears inside of
 an array to form a dictionary; the type string "a{sd}", for example, is
 the type of a dictionary that maps strings to double precision floating
 point values.

 The first type (the basic type) is the key type and the second type is
 the value type. The reason that the first type is restricted to being a
 basic type is so that it can easily be hashed.
- `*`: the type string of `G_VARIANT_TYPE_ANY`; the indefinite type that is
 a supertype of all types. Note that, as with all type strings, this
 character represents exactly one type. It cannot be used inside of tuples
 to mean "any number of items".

Any type string of a container that contains an indefinite type is,
itself, an indefinite type. For example, the type string "a*"
(corresponding to `G_VARIANT_TYPE_ARRAY`) is an indefinite type
that is a supertype of every array type. "(*s)" is a supertype
of all tuples that contain exactly two items where the second
item is a string.

"a{?*}" is an indefinite type that is a supertype of all arrays
containing dictionary entries where the key is any basic type and
the value is any type at all. This is, by definition, a dictionary,
so this type string corresponds to `G_VARIANT_TYPE_DICTIONARY`. Note
that, due to the restriction that the key of a dictionary entry must
be a basic type, "{**}" is not a valid type string.
<!-- impl VariantType::fn new -->
Creates a new `VariantType` corresponding to the type string given
by `type_string`. It is appropriate to call `VariantType::free` on
the return value.

It is a programmer error to call this function with an invalid type
string. Use `VariantType::string_is_valid` if you are unsure.
## `type_string`
a valid GVariant type string

# Returns

a new `VariantType`
<!-- impl VariantType::fn new_array -->
Constructs the type corresponding to an array of elements of the
type `type_`.

It is appropriate to call `VariantType::free` on the return value.
## `element`
a `VariantType`

# Returns

a new array `VariantType`

Since 2.24
<!-- impl VariantType::fn new_dict_entry -->
Constructs the type corresponding to a dictionary entry with a key
of type `key` and a value of type `value`.

It is appropriate to call `VariantType::free` on the return value.
## `key`
a basic `VariantType`
## `value`
a `VariantType`

# Returns

a new dictionary entry `VariantType`

Since 2.24
<!-- impl VariantType::fn new_maybe -->
Constructs the type corresponding to a maybe instance containing
type `type_` or Nothing.

It is appropriate to call `VariantType::free` on the return value.
## `element`
a `VariantType`

# Returns

a new maybe `VariantType`

Since 2.24
<!-- impl VariantType::fn new_tuple -->
Constructs a new tuple type, from `items`.

`length` is the number of items in `items`, or -1 to indicate that
`items` is `None`-terminated.

It is appropriate to call `VariantType::free` on the return value.
## `items`
an array of `GVariantTypes`, one for each item
## `length`
the length of `items`, or -1

# Returns

a new tuple `VariantType`

Since 2.24
<!-- impl VariantType::fn copy -->
Makes a copy of a `VariantType`. It is appropriate to call
`VariantType::free` on the return value. `self` may not be `None`.

# Returns

a new `VariantType`

Since 2.24
<!-- impl VariantType::fn dup_string -->
Returns a newly-allocated copy of the type string corresponding to
`self`. The returned string is nul-terminated. It is appropriate to
call `g_free` on the return value.

# Returns

the corresponding type string

Since 2.24
<!-- impl VariantType::fn element -->
Determines the element type of an array or maybe type.

This function may only be used with array or maybe types.

# Returns

the element type of `self`

Since 2.24
<!-- impl VariantType::fn equal -->
Compares `self` and `type2` for equality.

Only returns `true` if the types are exactly equal. Even if one type
is an indefinite type and the other is a subtype of it, `false` will
be returned if they are not exactly equal. If you want to check for
subtypes, use `VariantType::is_subtype_of`.

The argument types of `self` and `type2` are only `gconstpointer` to
allow use with `HashTable` without function pointer casting. For
both arguments, a valid `VariantType` must be provided.
## `type2`
a `VariantType`

# Returns

`true` if `self` and `type2` are exactly equal

Since 2.24
<!-- impl VariantType::fn first -->
Determines the first item type of a tuple or dictionary entry
type.

This function may only be used with tuple or dictionary entry types,
but must not be used with the generic tuple type
`G_VARIANT_TYPE_TUPLE`.

In the case of a dictionary entry type, this returns the type of
the key.

`None` is returned in case of `self` being `G_VARIANT_TYPE_UNIT`.

This call, together with `VariantType::next` provides an iterator
interface over tuple and dictionary entry types.

# Returns

the first item type of `self`, or `None`

Since 2.24
<!-- impl VariantType::fn free -->
Frees a `VariantType` that was allocated with
`VariantType::copy`, `VariantType::new` or one of the container
type constructor functions.

In the case that `self` is `None`, this function does nothing.

Since 2.24
<!-- impl VariantType::fn get_string_length -->
Returns the length of the type string corresponding to the given
`self`. This function must be used to determine the valid extent of
the memory region returned by `VariantType::peek_string`.

# Returns

the length of the corresponding type string

Since 2.24
<!-- impl VariantType::fn hash -->
Hashes `self`.

The argument type of `self` is only `gconstpointer` to allow use with
`HashTable` without function pointer casting. A valid
`VariantType` must be provided.

# Returns

the hash value

Since 2.24
<!-- impl VariantType::fn is_array -->
Determines if the given `self` is an array type. This is true if the
type string for `self` starts with an 'a'.

This function returns `true` for any indefinite type for which every
definite subtype is an array type -- `G_VARIANT_TYPE_ARRAY`, for
example.

# Returns

`true` if `self` is an array type

Since 2.24
<!-- impl VariantType::fn is_basic -->
Determines if the given `self` is a basic type.

Basic types are booleans, bytes, integers, doubles, strings, object
paths and signatures.

Only a basic type may be used as the key of a dictionary entry.

This function returns `false` for all indefinite types except
`G_VARIANT_TYPE_BASIC`.

# Returns

`true` if `self` is a basic type

Since 2.24
<!-- impl VariantType::fn is_container -->
Determines if the given `self` is a container type.

Container types are any array, maybe, tuple, or dictionary
entry types plus the variant type.

This function returns `true` for any indefinite type for which every
definite subtype is a container -- `G_VARIANT_TYPE_ARRAY`, for
example.

# Returns

`true` if `self` is a container type

Since 2.24
<!-- impl VariantType::fn is_definite -->
Determines if the given `self` is definite (ie: not indefinite).

A type is definite if its type string does not contain any indefinite
type characters ('*', '?', or 'r').

A `Variant` instance may not have an indefinite type, so calling
this function on the result of `Variant::get_type` will always
result in `true` being returned. Calling this function on an
indefinite type like `G_VARIANT_TYPE_ARRAY`, however, will result in
`false` being returned.

# Returns

`true` if `self` is definite

Since 2.24
<!-- impl VariantType::fn is_dict_entry -->
Determines if the given `self` is a dictionary entry type. This is
true if the type string for `self` starts with a '{'.

This function returns `true` for any indefinite type for which every
definite subtype is a dictionary entry type --
`G_VARIANT_TYPE_DICT_ENTRY`, for example.

# Returns

`true` if `self` is a dictionary entry type

Since 2.24
<!-- impl VariantType::fn is_maybe -->
Determines if the given `self` is a maybe type. This is true if the
type string for `self` starts with an 'm'.

This function returns `true` for any indefinite type for which every
definite subtype is a maybe type -- `G_VARIANT_TYPE_MAYBE`, for
example.

# Returns

`true` if `self` is a maybe type

Since 2.24
<!-- impl VariantType::fn is_subtype_of -->
Checks if `self` is a subtype of `supertype`.

This function returns `true` if `self` is a subtype of `supertype`. All
types are considered to be subtypes of themselves. Aside from that,
only indefinite types can have subtypes.
## `supertype`
a `VariantType`

# Returns

`true` if `self` is a subtype of `supertype`

Since 2.24
<!-- impl VariantType::fn is_tuple -->
Determines if the given `self` is a tuple type. This is true if the
type string for `self` starts with a '(' or if `self` is
`G_VARIANT_TYPE_TUPLE`.

This function returns `true` for any indefinite type for which every
definite subtype is a tuple type -- `G_VARIANT_TYPE_TUPLE`, for
example.

# Returns

`true` if `self` is a tuple type

Since 2.24
<!-- impl VariantType::fn is_variant -->
Determines if the given `self` is the variant type.

# Returns

`true` if `self` is the variant type

Since 2.24
<!-- impl VariantType::fn key -->
Determines the key type of a dictionary entry type.

This function may only be used with a dictionary entry type. Other
than the additional restriction, this call is equivalent to
`VariantType::first`.

# Returns

the key type of the dictionary entry

Since 2.24
<!-- impl VariantType::fn n_items -->
Determines the number of items contained in a tuple or
dictionary entry type.

This function may only be used with tuple or dictionary entry types,
but must not be used with the generic tuple type
`G_VARIANT_TYPE_TUPLE`.

In the case of a dictionary entry type, this function will always
return 2.

# Returns

the number of items in `self`

Since 2.24
<!-- impl VariantType::fn next -->
Determines the next item type of a tuple or dictionary entry
type.

`self` must be the result of a previous call to
`VariantType::first` or `VariantType::next`.

If called on the key type of a dictionary entry then this call
returns the value type. If called on the value type of a dictionary
entry then this call returns `None`.

For tuples, `None` is returned when `self` is the last item in a tuple.

# Returns

the next `VariantType` after `self`, or `None`

Since 2.24
<!-- impl VariantType::fn peek_string -->
Returns the type string corresponding to the given `self`. The
result is not nul-terminated; in order to determine its length you
must call `VariantType::get_string_length`.

To get a nul-terminated string, see `VariantType::dup_string`.

# Returns

the corresponding type string (not nul-terminated)

Since 2.24
<!-- impl VariantType::fn value -->
Determines the value type of a dictionary entry type.

This function may only be used with a dictionary entry type.

# Returns

the value type of the dictionary entry

Since 2.24
<!-- impl VariantType::fn string_is_valid -->
Checks if `type_string` is a valid GVariant type string. This call is
equivalent to calling `VariantType::string_scan` and confirming
that the following character is a nul terminator.
## `type_string`
a pointer to any string

# Returns

`true` if `type_string` is exactly one valid type string

Since 2.24
<!-- impl VariantType::fn string_scan -->
Scan for a single complete and valid GVariant type string in `string`.
The memory pointed to by `limit` (or bytes beyond it) is never
accessed.

If a valid type string is found, `endptr` is updated to point to the
first character past the end of the string that was found and `true`
is returned.

If there is no valid type string starting at `string`, or if the type
string does not end before `limit` then `false` is returned.

For the simple case of checking if a string is a valid type string,
see `VariantType::string_is_valid`.
## `string`
a pointer to any string
## `limit`
the end of `string`, or `None`
## `endptr`
location to store the end pointer, or `None`

# Returns

`true` if a valid type string was found
