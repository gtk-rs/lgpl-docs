<!-- file * -->
<!-- struct Box -->
A 3D box, described as the volume between a minimum and
a maximum vertices.
<!-- impl Box::fn alloc -->
Allocates a new `Box`.

The contents of the returned structure are undefined.

# Returns

the newly allocated `Box` structure.
 Use `Box::free` to free the resources allocated by this function
<!-- impl Box::fn contains_box -->
Checks whether the `Box` `self` contains the given
`Box` `b`.
## `b`
a `Box`

# Returns

`true` if the box is contained in the given box
<!-- impl Box::fn contains_point -->
Checks whether `self` contains the given `point`.
## `point`
the coordinates to check

# Returns

`true` if the point is contained in the given box
<!-- impl Box::fn equal -->
Checks whether the two given boxes are equal.
## `b`
a `Box`

# Returns

`true` if the boxes are equal
<!-- impl Box::fn expand -->
Expands the dimensions of `self` to include the coordinates at `point`.
## `point`
the coordinates of the point to include
## `res`
return location for the expanded box
<!-- impl Box::fn expand_scalar -->
Expands the dimensions of `self` by the given `scalar` value.

If `scalar` is positive, the `Box` will grow; if `scalar` is
negative, the `Box` will shrink.
## `scalar`
a scalar value
## `res`
return location for the expanded box
<!-- impl Box::fn expand_vec3 -->
Expands the dimensions of `self` to include the coordinates of the
given vector.
## `vec`
the coordinates of the point to include, as a `Vec3`
## `res`
return location for the expanded box
<!-- impl Box::fn free -->
Frees the resources allocated by `Box::alloc`.
<!-- impl Box::fn get_bounding_sphere -->
Computes the bounding `Sphere` capable of containing the given
`Box`.
## `sphere`
return location for the bounding sphere
<!-- impl Box::fn get_center -->
Retrieves the coordinates of the center of a `Box`.
## `center`
return location for the coordinates of
 the center
<!-- impl Box::fn get_depth -->
Retrieves the size of the `self` on the Z axis.

# Returns

the depth of the box
<!-- impl Box::fn get_height -->
Retrieves the size of the `self` on the Y axis.

# Returns

the height of the box
<!-- impl Box::fn get_max -->
Retrieves the coordinates of the maximum point of the given
`Box`.
## `max`
return location for the maximum point
<!-- impl Box::fn get_min -->
Retrieves the coordinates of the minimum point of the given
`Box`.
## `min`
return location for the minimum point
<!-- impl Box::fn get_size -->
Retrieves the size of the box on all three axes, and stores
it into the given `size` vector.
## `size`
return location for the size
<!-- impl Box::fn get_vertices -->
Computes the vertices of the given `Box`.
## `vertices`
return location for an array
 of 8 `Vec3`
<!-- impl Box::fn get_width -->
Retrieves the size of the `self` on the X axis.

# Returns

the width of the box
<!-- impl Box::fn init -->
Initializes the given `Box` with two vertices.
## `min`
the coordinates of the minimum vertex
## `max`
the coordinates of the maximum vertex

# Returns

the initialized `Box`
<!-- impl Box::fn init_from_box -->
Initializes the given `Box` with the vertices of
another `Box`.
## `src`
a `Box`

# Returns

the initialized `Box`
<!-- impl Box::fn init_from_points -->
Initializes the given `Box` with the given array
of vertices.

If `n_points` is 0, the returned box is initialized with
`Box::empty`.
## `n_points`
the number `Point3D` in the `points` array
## `points`
an array of `Point3D`

# Returns

the initialized `Box`
<!-- impl Box::fn init_from_vec3 -->
Initializes the given `Box` with two vertices
stored inside `Vec3`.
## `min`
the coordinates of the minimum vertex
## `max`
the coordinates of the maximum vertex

# Returns

the initialized `Box`
<!-- impl Box::fn init_from_vectors -->
Initializes the given `Box` with the given array
of vertices.

If `n_vectors` is 0, the returned box is initialized with
`Box::empty`.
## `n_vectors`
the number `Point3D` in the `vectors` array
## `vectors`
an array of `Vec3`

# Returns

the initialized `Box`
<!-- impl Box::fn intersection -->
Intersects the two given `Box`.

If the two boxes do not intersect, `res` will contain a degenerate box
initialized with `Box::empty`.
## `b`
a `Box`
## `res`
return location for the result

# Returns

true if the two boxes intersect
<!-- impl Box::fn union -->
Unions the two given `Box`.
## `b`
the box to union to `self`
## `res`
return location for the result
<!-- impl Box::fn empty -->
A degenerate `Box` that can only be expanded.

The returned value is owned by Graphene and should not be modified or freed.

# Returns

a `Box`
<!-- impl Box::fn infinite -->
A degenerate `Box` that cannot be expanded.

The returned value is owned by Graphene and should not be modified or freed.

# Returns

a `Box`
<!-- impl Box::fn minus_one -->
A `Box` with the minimum vertex set at (-1, -1, -1) and the
maximum vertex set at (0, 0, 0).

The returned value is owned by Graphene and should not be modified or freed.

# Returns

a `Box`
<!-- impl Box::fn one -->
A `Box` with the minimum vertex set at (0, 0, 0) and the
maximum vertex set at (1, 1, 1).

The returned value is owned by Graphene and should not be modified or freed.

# Returns

a `Box`
<!-- impl Box::fn one_minus_one -->
A `Box` with the minimum vertex set at (-1, -1, -1) and the
maximum vertex set at (1, 1, 1).

The returned value is owned by Graphene and should not be modified or freed.

# Returns

a `Box`
<!-- impl Box::fn zero -->
A `Box` with both the minimum and maximum vertices set at (0, 0, 0).

The returned value is owned by Graphene and should not be modified or freed.

# Returns

a `Box`
<!-- struct Euler -->
Describe a rotation using Euler angles.

The contents of the `Euler` structure are private
and should never be accessed directly.
<!-- impl Euler::fn alloc -->
Allocates a new `Euler`.

The contents of the returned structure are undefined.

# Returns

the newly allocated `Euler`
<!-- impl Euler::fn equal -->
Checks if two `Euler` are equal.
## `b`
a `Euler`

# Returns

`true` if the two `Euler` are equal
<!-- impl Euler::fn free -->
Frees the resources allocated by `Euler::alloc`.
<!-- impl Euler::fn get_order -->
Retrieves the order used to apply the rotations described in the
`Euler` structure, when converting to and from other
structures, like `Quaternion` and `Matrix`.

This function does not return the `EulerOrder::Default`
enumeration value; it will return the effective order of rotation
instead.

# Returns

the order used to apply the rotations
<!-- impl Euler::fn get_x -->
Retrieves the rotation angle on the X axis, in degrees.

# Returns

the rotation angle
<!-- impl Euler::fn get_y -->
Retrieves the rotation angle on the Y axis, in degrees.

# Returns

the rotation angle
<!-- impl Euler::fn get_z -->
Retrieves the rotation angle on the Z axis, in degrees.

# Returns

the rotation angle
<!-- impl Euler::fn init -->
Initializes a `Euler` using the given angles.

The order of the rotations is `EulerOrder::Default`.
## `x`
rotation angle on the X axis, in degrees
## `y`
rotation angle on the Y axis, in degrees
## `z`
rotation angle on the Z axis, in degrees

# Returns

the initialized `Euler`
<!-- impl Euler::fn init_from_euler -->
Initializes a `Euler` using the angles and order of
another `Euler`.

If the `Euler` `src` is `None`, this function is equivalent
to calling `Euler::init` with all angles set to 0.
## `src`
a `Euler`

# Returns

the initialized `Euler`
<!-- impl Euler::fn init_from_matrix -->
Initializes a `Euler` using the given rotation matrix.

If the `Matrix` `m` is `None`, the `Euler` will
be initialized with all angles set to 0.
## `m`
a rotation matrix
## `order`
the order used to apply the rotations

# Returns

the initialized `Euler`
<!-- impl Euler::fn init_from_quaternion -->
Initializes a `Euler` using the given normalized quaternion.

If the `Quaternion` `q` is `None`, the `Euler` will
be initialized with all angles set to 0.
## `q`
a normalized `Quaternion`
## `order`
the order used to apply the rotations

# Returns

the initialized `Euler`
<!-- impl Euler::fn init_from_vec3 -->
Initializes a `Euler` using the angles contained in a
`Vec3`.

If the `Vec3` `v` is `None`, the `Euler` will be
initialized with all angles set to 0.
## `v`
a `Vec3` containing the rotation
 angles in degrees
## `order`
the order used to apply the rotations

# Returns

the initialized `Euler`
<!-- impl Euler::fn init_with_order -->
Initializes a `Euler` with the given angles and `order`.
## `x`
rotation angle on the X axis, in degrees
## `y`
rotation angle on the Y axis, in degrees
## `z`
rotation angle on the Z axis, in degrees
## `order`
the order used to apply the rotations

# Returns

the initialized `Euler`
<!-- impl Euler::fn reorder -->
Reorders a `Euler` using `order`.

This function is equivalent to creating a `Quaternion` from the
given `Euler`, and then converting the quaternion into another
`Euler`.
## `order`
the new order
## `res`
return location for the reordered
 `Euler`
<!-- impl Euler::fn to_matrix -->
Converts a `Euler` into a transformation matrix expressing
the extrinsic composition of rotations described by the Euler angles.

The rotations are applied over the reference frame axes in the order
associated with the `Euler`; for instance, if the order
used to initialize `self` is `EulerOrder::Xyz`:

 * the first rotation moves the body around the X axis with
 an angle φ
 * the second rotation moves the body around the Y axis with
 an angle of ϑ
 * the third rotation moves the body around the Z axis with
 an angle of ψ

The rotation sign convention is left-handed, to preserve compatibility
between Euler-based, quaternion-based, and angle-axis-based rotations.
## `res`
return location for a `Matrix`
<!-- impl Euler::fn to_vec3 -->
Retrieves the angles of a `Euler` and initializes a
`Vec3` with them.
## `res`
return location for a `Vec3`
<!-- enum EulerOrder -->
Specify the order of the rotations on each axis.

The `EulerOrder::Default` value is special, and is used
as an alias for one of the other orders.
<!-- enum EulerOrder::variant Default -->
Rotate in the default order; the
 default order is one of the following enumeration values
<!-- enum EulerOrder::variant Xyz -->
Rotate in the X, Y, and Z order
<!-- enum EulerOrder::variant Yzx -->
Rotate in the Y, Z, and X order
<!-- enum EulerOrder::variant Zxy -->
Rotate in the Z, X, and Y order
<!-- enum EulerOrder::variant Xzy -->
Rotate in the X, Z, and Y order
<!-- enum EulerOrder::variant Yxz -->
Rotate in the Y, X, and Z order
<!-- enum EulerOrder::variant Zyx -->
Rotate in the Z, Y, and X order
<!-- struct Frustum -->
A 3D volume delimited by 2D clip planes.

The contents of the `graphene_frustum_t` are private, and should not be
modified directly.
<!-- impl Frustum::fn alloc -->
Allocates a new `Frustum` structure.

The contents of the returned structure are undefined.

# Returns

the newly allocated `Frustum`
 structure. Use `Frustum::free` to free the resources
 allocated by this function.
<!-- impl Frustum::fn contains_point -->
Checks whether a point is inside the volume defined by the given
`Frustum`.
## `point`
a `Point3D`

# Returns

`true` if the point is inside the frustum
<!-- impl Frustum::fn equal -->
Checks whether the two given `Frustum` are equal.
## `b`
a `Frustum`

# Returns

`true` if the given frustums are equal
<!-- impl Frustum::fn free -->
Frees the resources allocated by `Frustum::alloc`.
<!-- impl Frustum::fn get_planes -->
Retrieves the planes that define the given `Frustum`.
## `planes`
return location for an array
 of 6 `Plane`
<!-- impl Frustum::fn init -->
Initializes the given `Frustum` using the provided
clipping planes.
## `p0`
a clipping plane
## `p1`
a clipping plane
## `p2`
a clipping plane
## `p3`
a clipping plane
## `p4`
a clipping plane
## `p5`
a clipping plane

# Returns

the initialized frustum
<!-- impl Frustum::fn init_from_frustum -->
Initializes the given `Frustum` using the clipping
planes of another `Frustum`.
## `src`
a `Frustum`

# Returns

the initialized frustum
<!-- impl Frustum::fn init_from_matrix -->
Initializes a `Frustum` using the given `matrix`.
## `matrix`
a `Matrix`

# Returns

the initialized frustum
<!-- impl Frustum::fn intersects_box -->
Checks whether the given `box_` intersects a plane of
a `Frustum`.
## `box_`
a `Box`

# Returns

`true` if the box intersects the frustum
<!-- impl Frustum::fn intersects_sphere -->
Checks whether the given `sphere` intersects a plane of
a `Frustum`.
## `sphere`
a `Sphere`

# Returns

`true` if the sphere intersects the frustum
<!-- struct Matrix -->
A structure capable of holding a 4x4 matrix.

The contents of the `Matrix` structure are private and
should never be accessed directly.
<!-- impl Matrix::fn alloc -->
Allocates a new `Matrix`.

# Returns

the newly allocated matrix
<!-- impl Matrix::fn determinant -->
Computes the determinant of the given matrix.

# Returns

the value of the determinant
<!-- impl Matrix::fn equal -->
Checks whether the two given `Matrix` matrices are equal.

Feature: `v1_10`

## `b`
a `Matrix`

# Returns

`true` if the two matrices are equal, and `false` otherwise
<!-- impl Matrix::fn equal_fast -->
Checks whether the two given `Matrix` matrices are
byte-by-byte equal.

While this function is faster than `Matrix::equal`, it
can also return false negatives, so it should be used in
conjuction with either `Matrix::equal` or
`Matrix::near`. For instance:


```C
  if (graphene_matrix_equal_fast (a, b))
    {
      // matrices are definitely the same
    }
  else
    {
      if (graphene_matrix_equal (a, b))
        // matrices contain the same values within an epsilon of FLT_EPSILON
      else if (graphene_matrix_near (a, b, 0.0001))
        // matrices contain the same values within an epsilon of 0.0001
      else
        // matrices are not equal
    }
```

Feature: `v1_10`

## `b`
a `Matrix`

# Returns

`true` if the matrices are equal. and `false` otherwise
<!-- impl Matrix::fn free -->
Frees the resources allocated by `Matrix::alloc`.
<!-- impl Matrix::fn get_row -->
Retrieves the given row vector at `index_` inside a matrix.
## `index_`
the index of the row vector, between 0 and 3
## `res`
return location for the `Vec4`
 that is used to store the row vector
<!-- impl Matrix::fn get_value -->
Retrieves the value at the given `row` and `col` index.
## `row`
the row index
## `col`
the column index

# Returns

the value at the given indices
<!-- impl Matrix::fn get_x_scale -->
Retrieves the scaling factor on the X axis in `self`.

# Returns

the value of the scaling factor
<!-- impl Matrix::fn get_x_translation -->
Retrieves the translation component on the X axis from `self`.

Feature: `v1_10`


# Returns

the translation component
<!-- impl Matrix::fn get_y_scale -->
Retrieves the scaling factor on the Y axis in `self`.

# Returns

the value of the scaling factor
<!-- impl Matrix::fn get_y_translation -->
Retrieves the translation component on the Y axis from `self`.

Feature: `v1_10`


# Returns

the translation component
<!-- impl Matrix::fn get_z_scale -->
Retrieves the scaling factor on the Z axis in `self`.

# Returns

the value of the scaling factor
<!-- impl Matrix::fn get_z_translation -->
Retrieves the translation component on the Z axis from `self`.

Feature: `v1_10`


# Returns

the translation component
<!-- impl Matrix::fn init_from_2d -->
Initializes a `Matrix` from the values of an affine
transformation matrix.

The arguments map to the following matrix layout:


```plain
  ⎛ xx  yx ⎞   ⎛  a   b  0 ⎞
  ⎜ xy  yy ⎟ = ⎜  c   d  0 ⎟
  ⎝ x0  y0 ⎠   ⎝ tx  ty  1 ⎠
```

This function can be used to convert between an affine matrix type
from other libraries and a `Matrix`.
## `xx`
the xx member
## `yx`
the yx member
## `xy`
the xy member
## `yy`
the yy member
## `x_0`
the x0 member
## `y_0`
the y0 member

# Returns

the initialized matrix
<!-- impl Matrix::fn init_from_float -->
Initializes a `Matrix` with the given array of floating
point values.
## `v`
an array of at least 16 floating
 point values

# Returns

the initialized matrix
<!-- impl Matrix::fn init_from_matrix -->
Initializes a `Matrix` using the values of the
given matrix.
## `src`
a `Matrix`

# Returns

the initialized matrix
<!-- impl Matrix::fn init_from_vec4 -->
Initializes a `Matrix` with the given four row
vectors.
## `v0`
the first row vector
## `v1`
the second row vector
## `v2`
the third row vector
## `v3`
the fourth row vector

# Returns

the initialized matrix
<!-- impl Matrix::fn init_frustum -->
Initializes a `Matrix` compatible with `Frustum`.

See also: `Frustum::init_from_matrix`
## `left`
distance of the left clipping plane
## `right`
distance of the right clipping plane
## `bottom`
distance of the bottom clipping plane
## `top`
distance of the top clipping plane
## `z_near`
distance of the near clipping plane
## `z_far`
distance of the far clipping plane

# Returns

the initialized matrix
<!-- impl Matrix::fn init_identity -->
Initializes a `Matrix` with the identity matrix.

# Returns

the initialized matrix
<!-- impl Matrix::fn init_look_at -->
Initializes a `Matrix` so that it positions the "camera"
at the given `eye` coordinates towards an object at the `center`
coordinates. The top of the camera is aligned to the direction
of the `up` vector.
## `eye`
the vector describing the position to look from
## `center`
the vector describing the position to look at
## `up`
the vector describing the world's upward direction; usually,
 this is the `Vec3::y_axis` vector

# Returns

the initialized matrix
<!-- impl Matrix::fn init_ortho -->
Initializes a `Matrix` with an orthographic projection.
## `left`
the left edge of the clipping plane
## `right`
the right edge of the clipping plane
## `top`
the top edge of the clipping plane
## `bottom`
the bottom edge of the clipping plane
## `z_near`
the distance of the near clipping plane
## `z_far`
the distance of the far clipping plane

# Returns

the initialized matrix
<!-- impl Matrix::fn init_perspective -->
Initializes a `Matrix` with a perspective projection.
## `fovy`
the field of view angle, in degrees
## `aspect`
the aspect value
## `z_near`
the near Z plane
## `z_far`
the far Z plane

# Returns

the initialized matrix
<!-- impl Matrix::fn init_rotate -->
Initializes `self` to represent a rotation of `angle` degrees on
the axis represented by the `axis` vector.
## `angle`
the rotation angle, in degrees
## `axis`
the axis vector as a `Vec3`

# Returns

the initialized matrix
<!-- impl Matrix::fn init_scale -->
Initializes a `Matrix` with the given scaling factors.
## `x`
the scale factor on the X axis
## `y`
the scale factor on the Y axis
## `z`
the scale factor on the Z axis

# Returns

the initialized matrix
<!-- impl Matrix::fn init_skew -->
Initializes a `Matrix` with a skew transformation
with the given factors.
## `x_skew`
skew factor, in radians, on the X axis
## `y_skew`
skew factor, in radians, on the Y axis

# Returns

the initialized matrix
<!-- impl Matrix::fn init_translate -->
Initializes a `Matrix` with a translation to the
given coordinates.
## `p`
the translation coordinates

# Returns

the initialized matrix
<!-- impl Matrix::fn interpolate -->
Linearly interpolates the two given `Matrix` by
interpolating the decomposed transformations separately.

If either matrix cannot be reduced to their transformations
then the interpolation cannot be performed, and this function
will return an identity matrix.
## `b`
a `Matrix`
## `factor`
the linear interpolation factor
## `res`
return location for the
 interpolated matrix
<!-- impl Matrix::fn inverse -->
Inverts the given matrix.
## `res`
return location for the
 inverse matrix

# Returns

`true` if the matrix is invertible
<!-- impl Matrix::fn is_2d -->
Checks whether the given `Matrix` is compatible with an
a 2D affine transformation matrix.

# Returns

`true` if the matrix is compatible with an affine
 transformation matrix
<!-- impl Matrix::fn is_backface_visible -->
Checks whether a `Matrix` has a visible back face.

# Returns

`true` if the back face of the matrix is visible
<!-- impl Matrix::fn is_identity -->
Checks whether the given `Matrix` is the identity matrix.

# Returns

`true` if the matrix is the identity matrix
<!-- impl Matrix::fn is_singular -->
Checks whether a matrix is singular.

# Returns

`true` if the matrix is singular
<!-- impl Matrix::fn multiply -->
Multiplies two `Matrix`.

Matrix multiplication is not commutative in general; the order of the factors matters.
The product of this multiplication is (`self` × `b`)
## `b`
a `Matrix`
## `res`
return location for the matrix
 result
<!-- impl Matrix::fn near -->
Compares the two given `Matrix` matrices and check
whether their values are within the given `epsilon` of each
other.

Feature: `v1_10`

## `b`
a `Matrix`

# Returns

`true` if the two matrices are near each other, and
 `false` otherwise
<!-- impl Matrix::fn normalize -->
Normalizes the given `Matrix`.
## `res`
return location for the normalized matrix
<!-- impl Matrix::fn perspective -->
Applies a perspective of `depth` to the matrix.
## `depth`
the depth of the perspective
## `res`
return location for the
 perspective matrix
<!-- impl Matrix::fn print -->
Prints the contents of a matrix.
<!-- impl Matrix::fn project_point -->
Projects a `Point` using the matrix `self`.
## `p`
a `Point`
## `res`
return location for the projected
 point
<!-- impl Matrix::fn project_rect -->
Projects a `Rect` using the given matrix.
## `r`
a `Rect`
## `res`
return location for the projected
 rectangle
<!-- impl Matrix::fn project_rect_bounds -->
Projects a `Rect` using the given matrix.

The resulting rectangle is the axis aligned bounding rectangle capable
of containing fully the projected rectangle.
## `r`
a `Rect`
## `res`
return location for the projected
 rectangle
<!-- impl Matrix::fn rotate -->
Adds a rotation transformation to `self`, using the given `angle`
and `axis` vector.

This is the equivalent of calling `Matrix::init_rotate` and
then multiplying the matrix `self` with the rotation matrix.
## `angle`
the rotation angle, in degrees
## `axis`
the rotation axis, as a `Vec3`
<!-- impl Matrix::fn rotate_euler -->
Adds a rotation transformation to `self`, using the given
`Euler`.
## `e`
a rotation described by a `Euler`
<!-- impl Matrix::fn rotate_quaternion -->
Adds a rotation transformation to `self`, using the given
`Quaternion`.

This is the equivalent of calling `Quaternion::to_matrix` and
then multiplying `self` with the rotation matrix.
## `q`
a rotation described by a `Quaternion`
<!-- impl Matrix::fn rotate_x -->
Adds a rotation transformation around the X axis to `self`, using
the given `angle`.

See also: `Matrix::rotate`
## `angle`
the rotation angle, in degrees
<!-- impl Matrix::fn rotate_y -->
Adds a rotation transformation around the Y axis to `self`, using
the given `angle`.

See also: `Matrix::rotate`
## `angle`
the rotation angle, in degrees
<!-- impl Matrix::fn rotate_z -->
Adds a rotation transformation around the Z axis to `self`, using
the given `angle`.

See also: `Matrix::rotate`
## `angle`
the rotation angle, in degrees
<!-- impl Matrix::fn scale -->
Adds a scaling transformation to `self`, using the three
given factors.

This is the equivalent of calling `Matrix::init_scale` and then
multiplying the matrix `self` with the scale matrix.
## `factor_x`
scaling factor on the X axis
## `factor_y`
scaling factor on the Y axis
## `factor_z`
scaling factor on the Z axis
<!-- impl Matrix::fn skew_xy -->
Adds a skew of `factor` on the X and Y axis to the given matrix.
## `factor`
skew factor
<!-- impl Matrix::fn skew_xz -->
Adds a skew of `factor` on the X and Z axis to the given matrix.
## `factor`
skew factor
<!-- impl Matrix::fn skew_yz -->
Adds a skew of `factor` on the Y and Z axis to the given matrix.
## `factor`
skew factor
<!-- impl Matrix::fn to_2d -->
Converts a `Matrix` to an affine transformation
matrix, if the given matrix is compatible.

The returned values have the following layout:


```plain
  ⎛ xx  yx ⎞   ⎛  a   b  0 ⎞
  ⎜ xy  yy ⎟ = ⎜  c   d  0 ⎟
  ⎝ x0  y0 ⎠   ⎝ tx  ty  1 ⎠
```

This function can be used to convert between a `Matrix`
and an affine matrix type from other libraries.
## `xx`
return location for the xx member
## `yx`
return location for the yx member
## `xy`
return location for the xy member
## `yy`
return location for the yy member
## `x_0`
return location for the x0 member
## `y_0`
return location for the y0 member

# Returns

`true` if the matrix is compatible with an affine
 transformation matrix
<!-- impl Matrix::fn to_float -->
Converts a `Matrix` to an array of floating point
values.
## `v`
return location
 for an array of floating point values. The array must be capable
 of holding at least 16 values.
<!-- impl Matrix::fn transform_bounds -->
Transforms each corner of a `Rect` using the given matrix `self`.

The result is the axis aligned bounding rectangle containing the coplanar
quadrilateral.
## `r`
a `Rect`
## `res`
return location for the bounds
 of the transformed rectangle
<!-- impl Matrix::fn transform_box -->
Transforms the vertices of a `Box` using the given matrix `self`.

The result is the axis aligned bounding box containing the transformed
vertices.
## `b`
a `Box`
## `res`
return location for the bounds
 of the transformed box
<!-- impl Matrix::fn transform_point -->
Transforms the given `Point` using the matrix `self`.

Unlike `Matrix::transform_vec3`, this function will take into
account the fourth row vector of the `Matrix` when computing
the dot product of each row vector of the matrix.

See also: `graphene_simd4x4f_point3_mul`
## `p`
a `Point`
## `res`
return location for the
 transformed `Point`
<!-- impl Matrix::fn transform_point3d -->
Transforms the given `Point3D` using the matrix `self`.

Unlike `Matrix::transform_vec3`, this function will take into
account the fourth row vector of the `Matrix` when computing
the dot product of each row vector of the matrix.
## `p`
a `Point3D`
## `res`
return location for the result
<!-- impl Matrix::fn transform_ray -->
Transform a `Ray` using the given matrix `self`.
## `r`
a `Ray`
## `res`
return location for the
 transformed ray
<!-- impl Matrix::fn transform_rect -->
Transforms each corner of a `Rect` using the given matrix `self`.

The result is a coplanar quadrilateral.
## `r`
a `Rect`
## `res`
return location for the
 transformed quad
<!-- impl Matrix::fn transform_sphere -->
Transforms a `Sphere` using the given matrix `self`. The
result is the bounding sphere containing the transformed sphere.
## `s`
a `Sphere`
## `res`
return location for the bounds
 of the transformed sphere
<!-- impl Matrix::fn transform_vec3 -->
Transforms the given `Vec3` using the matrix `self`.

This function will multiply the X, Y, and Z row vectors of the matrix `self`
with the corresponding components of the vector `v`. The W row vector will
be ignored.

See also: `graphene_simd4x4f_vec3_mul`
## `v`
a `Vec3`
## `res`
return location for a `Vec3`
<!-- impl Matrix::fn transform_vec4 -->
Transforms the given `Vec4` using the matrix `self`.

See also: `graphene_simd4x4f_vec4_mul`
## `v`
a `Vec4`
## `res`
return location for a `Vec4`
<!-- impl Matrix::fn translate -->
Adds a translation transformation to `self` using the coordinates
of the given `Point3D`.

This is the equivalent of calling `Matrix::init_translate` and
then multiplying `self` with the translation matrix.
## `pos`
a `Point3D`
<!-- impl Matrix::fn transpose -->
Transposes the given matrix.
## `res`
return location for the
 transposed matrix
<!-- impl Matrix::fn unproject_point3d -->
Unprojects the given `point` using the `self` matrix and
a `modelview` matrix.
## `modelview`
a `Matrix` for the modelview matrix; this is
 the inverse of the modelview used when projecting the point
## `point`
a `Point3D` with the coordinates of the point
## `res`
return location for the unprojected
 point
<!-- impl Matrix::fn untransform_bounds -->
Undoes the transformation on the corners of a `Rect` using the
given matrix, within the given axis aligned rectangular `bounds`.
## `r`
a `Rect`
## `bounds`
the bounds of the transformation
## `res`
return location for the
 untransformed rectangle
<!-- impl Matrix::fn untransform_point -->
Undoes the transformation of a `Point` using the
given matrix, within the given axis aligned rectangular `bounds`.
## `p`
a `Point`
## `bounds`
the bounds of the transformation
## `res`
return location for the
 untransformed point

# Returns

`true` if the point was successfully untransformed
<!-- struct Plane -->
A 2D plane that extends infinitely in a 3D volume.

The contents of the `graphene_plane_t` are private, and should not be
modified directly.
<!-- impl Plane::fn alloc -->
Allocates a new `Plane` structure.

The contents of the returned structure are undefined.

# Returns

the newly allocated `Plane`.
 Use `Plane::free` to free the resources allocated by
 this function
<!-- impl Plane::fn distance -->
Computes the distance of `point` from a `Plane`.
## `point`
a `Point3D`

# Returns

the distance of the given `Point3D` from the plane
<!-- impl Plane::fn equal -->
Checks whether the two given `Plane` are equal.
## `b`
a `Plane`

# Returns

`true` if the given planes are equal
<!-- impl Plane::fn free -->
Frees the resources allocated by `Plane::alloc`.
<!-- impl Plane::fn get_constant -->
Retrieves the distance along the normal vector of the
given `Plane` from the origin.

# Returns

the constant value of the plane
<!-- impl Plane::fn get_normal -->
Retrieves the normal vector pointing towards the origin of the
given `Plane`.
## `normal`
return location for the normal vector
<!-- impl Plane::fn init -->
Initializes the given `Plane` using the given `normal` vector
and `constant` values.
## `normal`
a unit length normal vector defining the plane
 pointing towards the origin; if unset, we use the X axis by default
## `constant`
the distance from the origin to the plane along the
 normal vector; the sign determines the half-space occupied by the
 plane

# Returns

the initialized plane
<!-- impl Plane::fn init_from_plane -->
Initializes the given `Plane` using the normal
vector and constant of another `Plane`.
## `src`
a `Plane`

# Returns

the initialized plane
<!-- impl Plane::fn init_from_point -->
Initializes the given `Plane` using the given normal vector
and an arbitrary co-planar point.
## `normal`
a normal vector defining the plane pointing towards the origin
## `point`
a `Point3D`

# Returns

the initialized plane
<!-- impl Plane::fn init_from_points -->
Initializes the given `Plane` using the 3 provided co-planar
points.

The winding order is counter-clockwise, and determines which direction
the normal vector will point.
## `a`
a `Point3D`
## `b`
a `Point3D`
## `c`
a `Point3D`

# Returns

the initialized plane
<!-- impl Plane::fn init_from_vec4 -->
Initializes the given `Plane` using the components of
the given `Vec4` vector.
## `src`
a `Vec4` containing the normal vector in its first
 three components, and the distance in its fourth component

# Returns

the initialized plane
<!-- impl Plane::fn negate -->
Negates the normal vector and constant of a `Plane`, effectively
mirroring the plane across the origin.
## `res`
return location for the negated plane
<!-- impl Plane::fn normalize -->
Normalizes the vector of the given `Plane`,
and adjusts the constant accordingly.
## `res`
return location for the normalized plane
<!-- struct Point -->
A point with two coordinates.
<!-- impl Point::fn alloc -->
Allocates a new `Point` structure.

The coordinates of the returned point are (0, 0).

It's possible to chain this function with `Point::init`
or `Point::init_from_point`, e.g.:


```C
  graphene_point_t *
  point_new (float x, float y)
  {
    return graphene_point_init (graphene_point_alloc (), x, y);
  }

  graphene_point_t *
  point_copy (const graphene_point_t *p)
  {
    return graphene_point_init_from_point (graphene_point_alloc (), p);
  }
```

# Returns

the newly allocated `Point`.
 Use `Point::free` to free the resources allocated by
 this function.
<!-- impl Point::fn distance -->
Computes the distance between `self` and `b`.
## `b`
a `Point`
## `d_x`
distance component on the X axis
## `d_y`
distance component on the Y axis

# Returns

the distance between the two points
<!-- impl Point::fn equal -->
Checks if the two points `self` and `b` point to the same
coordinates.

This function accounts for floating point fluctuations; if
you want to control the fuzziness of the match, you can use
`Point::near` instead.
## `b`
a `Point`

# Returns

`true` if the points have the same coordinates
<!-- impl Point::fn free -->
Frees the resources allocated by `Point::alloc`.
<!-- impl Point::fn init -->
Initializes `self` to the given `x` and `y` coordinates.

It's safe to call this function multiple times.
## `x`
the X coordinate
## `y`
the Y coordinate

# Returns

the initialized point
<!-- impl Point::fn init_from_point -->
Initializes `self` with the same coordinates of `src`.
## `src`
the `Point` to use

# Returns

the initialized point
<!-- impl Point::fn init_from_vec2 -->
Initializes `self` with the coordinates inside the given `Vec2`.
## `src`
a `Vec2`

# Returns

the initialized point
<!-- impl Point::fn interpolate -->
Linearly interpolates the coordinates of `self` and `b` using the
given `factor`.
## `b`
a `Point`
## `factor`
the linear interpolation factor
## `res`
return location for the interpolated
 point
<!-- impl Point::fn near -->
Checks whether the two points `self` and `b` are within
the threshold of `epsilon`.
## `b`
a `Point`
## `epsilon`
threshold between the two points

# Returns

`true` if the distance is within `epsilon`
<!-- impl Point::fn to_vec2 -->
Stores the coordinates of the given `Point` into a
`Vec2`.
## `v`
return location for the vertex
<!-- impl Point::fn zero -->
Returns a point fixed at (0, 0).

# Returns

a fixed point
<!-- struct Point3D -->
A point with three components: X, Y, and Z.
<!-- impl Point3D::fn alloc -->
Allocates a `Point3D` structure.

# Returns

the newly allocated structure.
 Use `Point3D::free` to free the resources
 allocated by this function.
<!-- impl Point3D::fn cross -->
Computes the cross product of the two given `Point3D`.
## `b`
a `Point3D`
## `res`
return location for the cross
 product
<!-- impl Point3D::fn distance -->
Computes the distance between the two given `Point3D`.
## `b`
a `Point3D`
## `delta`
return location for the distance
 components on the X, Y, and Z axis

# Returns

the distance between two points
<!-- impl Point3D::fn dot -->
Computes the dot product of the two given `Point3D`.
## `b`
a `Point3D`

# Returns

the value of the dot product
<!-- impl Point3D::fn equal -->
Checks whether two given points are equal.
## `b`
a `Point3D`

# Returns

`true` if the points are equal
<!-- impl Point3D::fn free -->
Frees the resources allocated via `Point3D::alloc`.
<!-- impl Point3D::fn init -->
Initializes a `Point3D` with the given coordinates.
## `x`
the X coordinate of the point
## `y`
the Y coordinate of the point
## `z`
the Z coordinate of the point

# Returns

the initialized `Point3D`
<!-- impl Point3D::fn init_from_point -->
Initializes a `Point3D` using the coordinates of
another `Point3D`.
## `src`
a `Point3D`

# Returns

the initialized point
<!-- impl Point3D::fn init_from_vec3 -->
Initializes a `Point3D` using the components
of a `Vec3`.
## `v`
a `Vec3`

# Returns

the initialized `Point3D`
<!-- impl Point3D::fn interpolate -->
Linearly interpolates each component of `self` and `b` using the
provided `factor`, and places the result in `res`.
## `b`
a `Point3D`
## `factor`
the interpolation factor
## `res`
the return location for the
 interpolated `Point3D`
<!-- impl Point3D::fn length -->
Computes the length of the vector represented by the
coordinates of the given `Point3D`.

# Returns

the length of the vector represented by the point
<!-- impl Point3D::fn near -->
Checks whether the two points are near each other, within
an `epsilon` factor.
## `b`
a `Point3D`
## `epsilon`
fuzzyness factor

# Returns

`true` if the points are near each other
<!-- impl Point3D::fn normalize -->
Computes the normalization of the vector represented by the
coordinates of the given `Point3D`.
## `res`
return location for the normalized
 `Point3D`
<!-- impl Point3D::fn normalize_viewport -->
Normalizes the coordinates of a `Point3D` using the
given viewport and clipping planes.

The coordinates of the resulting `Point3D` will be
in the [ -1, 1 ] range.
## `viewport`
a `Rect` representing a viewport
## `z_near`
the coordinate of the near clipping plane, or 0 for
 the default near clipping plane
## `z_far`
the coordinate of the far clipping plane, or 1 for the
 default far clipping plane
## `res`
the return location for the
 normalized `Point3D`
<!-- impl Point3D::fn scale -->
Scales the coordinates of the given `Point3D` by
the given `factor`.
## `factor`
the scaling factor
## `res`
return location for the scaled point
<!-- impl Point3D::fn to_vec3 -->
Stores the coordinates of a `Point3D` into a
`Vec3`.
## `v`
return location for a `Vec3`
<!-- impl Point3D::fn zero -->
Retrieves a constant point with all three coordinates set to 0.

# Returns

a zero point
<!-- struct Quad -->
A 4 vertex quadrilateral, as represented by four `Point`.

The contents of a `Quad` are private and should never be
accessed directly.
<!-- impl Quad::fn alloc -->
Allocates a new `Quad` instance.

The contents of the returned instance are undefined.

# Returns

the newly created `Quad` instance
<!-- impl Quad::fn bounds -->
Computes the bounding rectangle of `self` and places it into `r`.
## `r`
return location for a `Rect`
<!-- impl Quad::fn contains -->
Checks if the given `Quad` contains the given `Point`.
## `p`
a `Point`

# Returns

`true` if the point is inside the `Quad`
<!-- impl Quad::fn free -->
Frees the resources allocated by `Quad::alloc`
<!-- impl Quad::fn get_point -->
Retrieves the point of a `Quad` at the given index.
## `index_`
the index of the point to retrieve

# Returns

a `Point`
<!-- impl Quad::fn init -->
Initializes a `Quad` with the given points.
## `p1`
the first point of the quadrilateral
## `p2`
the second point of the quadrilateral
## `p3`
the third point of the quadrilateral
## `p4`
the fourth point of the quadrilateral

# Returns

the initialized `Quad`
<!-- impl Quad::fn init_from_points -->
Initializes a `Quad` using an array of points.
## `points`
an array of 4 `Point`

# Returns

the initialized `Quad`
<!-- impl Quad::fn init_from_rect -->
Initializes a `Quad` using the four corners of the
given `Rect`.
## `r`
a `Rect`

# Returns

the initialized `Quad`
<!-- struct Quaternion -->
A quaternion.

The contents of the `Quaternion` structure are private
and should never be accessed directly.
<!-- impl Quaternion::fn alloc -->
Allocates a new `Quaternion`.

The contents of the returned value are undefined.

# Returns

the newly allocated `Quaternion`
<!-- impl Quaternion::fn dot -->
Computes the dot product of two `Quaternion`.
## `b`
a `Quaternion`

# Returns

the value of the dot products
<!-- impl Quaternion::fn equal -->
Checks whether the given quaternions are equal.
## `b`
a `Quaternion`

# Returns

`true` if the quaternions are equal
<!-- impl Quaternion::fn free -->
Releases the resources allocated by `Quaternion::alloc`.
<!-- impl Quaternion::fn init -->
Initializes a `Quaternion` using the given four values.
## `x`
the first component of the quaternion
## `y`
the second component of the quaternion
## `z`
the third component of the quaternion
## `w`
the fourth component of the quaternion

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_from_angle_vec3 -->
Initializes a `Quaternion` using an `angle` on a
specific `axis`.
## `angle`
the rotation on a given axis, in degrees
## `axis`
the axis of rotation, expressed as a vector

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_from_angles -->
Initializes a `Quaternion` using the values of
the [Euler angles](http://en.wikipedia.org/wiki/Euler_angles)
on each axis.

See also: `Quaternion::init_from_euler`
## `deg_x`
rotation angle on the X axis (yaw), in degrees
## `deg_y`
rotation angle on the Y axis (pitch), in degrees
## `deg_z`
rotation angle on the Z axis (roll), in degrees

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_from_euler -->
Initializes a `Quaternion` using the given `Euler`.
## `e`
a `Euler`

# Returns

the initialized `Quaternion`
<!-- impl Quaternion::fn init_from_matrix -->
Initializes a `Quaternion` using the rotation components
of a transformation matrix.
## `m`
a `Matrix`

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_from_quaternion -->
Initializes a `Quaternion` with the values from `src`.
## `src`
a `Quaternion`

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_from_radians -->
Initializes a `Quaternion` using the values of
the [Euler angles](http://en.wikipedia.org/wiki/Euler_angles)
on each axis.

See also: `Quaternion::init_from_euler`
## `rad_x`
rotation angle on the X axis (yaw), in radians
## `rad_y`
rotation angle on the Y axis (pitch), in radians
## `rad_z`
rotation angle on the Z axis (roll), in radians

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_from_vec4 -->
Initializes a `Quaternion` with the values from `src`.
## `src`
a `Vec4`

# Returns

the initialized quaternion
<!-- impl Quaternion::fn init_identity -->
Initializes a `Quaternion` using the identity
transformation.

# Returns

the initialized quaternion
<!-- impl Quaternion::fn invert -->
Inverts a `Quaternion`.
## `res`
return location for the inverted
 quaternion
<!-- impl Quaternion::fn normalize -->
Normalizes a `Quaternion`.
## `res`
return location for the normalized
 quaternion
<!-- impl Quaternion::fn slerp -->
Interpolates between the two given quaternions using a spherical
linear interpolation, or [SLERP](http://en.wikipedia.org/wiki/Slerp),
using the given interpolation `factor`.
## `b`
a `Quaternion`
## `factor`
the linear interpolation factor
## `res`
return location for the interpolated
 quaternion
<!-- impl Quaternion::fn to_angle_vec3 -->
Converts a quaternion into an `angle`, `axis` pair.
## `angle`
return location for the angle, in degrees
## `axis`
return location for the rotation axis
<!-- impl Quaternion::fn to_angles -->
Converts a `Quaternion` to its corresponding rotations
on the [Euler angles](http://en.wikipedia.org/wiki/Euler_angles)
on each axis.
## `deg_x`
return location for the rotation angle on
 the X axis (yaw), in degrees
## `deg_y`
return location for the rotation angle on
 the Y axis (pitch), in degrees
## `deg_z`
return location for the rotation angle on
 the Z axis (roll), in degrees
<!-- impl Quaternion::fn to_matrix -->
Converts a quaternion into a transformation matrix expressing
the rotation defined by the `Quaternion`.
## `m`
a `Matrix`
<!-- impl Quaternion::fn to_radians -->
Converts a `Quaternion` to its corresponding rotations
on the [Euler angles](http://en.wikipedia.org/wiki/Euler_angles)
on each axis.
## `rad_x`
return location for the rotation angle on
 the X axis (yaw), in radians
## `rad_y`
return location for the rotation angle on
 the Y axis (pitch), in radians
## `rad_z`
return location for the rotation angle on
 the Z axis (roll), in radians
<!-- impl Quaternion::fn to_vec4 -->
Copies the components of a `Quaternion` into a
`Vec4`.
## `res`
return location for a
 `Vec4`
<!-- struct Ray -->
A ray emitted from an origin in a given direction.

The contents of the `graphene_ray_t` structure are private, and should not
be modified directly.
<!-- impl Ray::fn alloc -->
Allocates a new `Ray` structure.

The contents of the returned structure are undefined.

# Returns

the newly allocated `Ray`.
 Use `Ray::free` to free the resources allocated by
 this function
<!-- impl Ray::fn equal -->
Checks whether the two given `Ray` are equal.
## `b`
a `Ray`

# Returns

`true` if the given rays are equal
<!-- impl Ray::fn free -->
Frees the resources allocated by `Ray::alloc`.
<!-- impl Ray::fn get_closest_point_to_point -->
Computes the point on the given `Ray` that is closest to the
given point `p`.
## `p`
a `Point3D`
## `res`
return location for the closest point3d
<!-- impl Ray::fn get_direction -->
Retrieves the direction of the given `Ray`.
## `direction`
return location for the direction
<!-- impl Ray::fn get_distance_to_plane -->
Computes the distance of the origin of the given `Ray` from the
given plane.

If the ray does not intersect the plane, this function returns `INFINITY`.
## `p`
a `Plane`

# Returns

the distance of the origin of the ray from the plane
<!-- impl Ray::fn get_distance_to_point -->
Computes the distance from the origin of the given ray to the given point.
## `p`
a `Point3D`

# Returns

the distance of the point
<!-- impl Ray::fn get_origin -->
Retrieves the origin of the given `Ray`.
## `origin`
return location for the origin
<!-- impl Ray::fn get_position_at -->
Retrieves the coordinates of a point at the distance `t` along the
given `Ray`.
## `t`
the distance along the ray
## `position`
return location for the position
<!-- impl Ray::fn init -->
Initializes the given `Ray` using the given `origin`
and `direction` values.
## `origin`
the origin of the ray
## `direction`
the direction vector

# Returns

the initialized ray
<!-- impl Ray::fn init_from_ray -->
Initializes the given `Ray` using the origin and direction
values of another `Ray`.
## `src`
a `Ray`

# Returns

the initialized ray
<!-- impl Ray::fn init_from_vec3 -->
Initializes the given `Ray` using the given vectors.
## `origin`
a `Vec3`
## `direction`
a `Vec3`

# Returns

the initialized ray
<!-- struct Rect -->
The location and size of a rectangle region.

The width and height of a `Rect` can be negative; for instance,
a `Rect` with an origin of [ 0, 0 ] and a size of [ 10, 10 ] is
equivalent to a `Rect` with an origin of [ 10, 10 ] and a size
of [ -10, -10 ].

Application code can normalize rectangles using `Rect::normalize`;
this function will ensure that the width and height of a rectangle are
positive values. All functions taking a `Rect` as an argument
will internally operate on a normalized copy; all functions returning a
`Rect` will always return a normalized rectangle.
<!-- impl Rect::fn contains_point -->
Checks whether a `Rect` contains the given coordinates.
## `p`
a `Point`

# Returns

`true` if the rectangle contains the point
<!-- impl Rect::fn contains_rect -->
Checks whether a `Rect` fully contains the given
rectangle.
## `b`
a `Rect`

# Returns

`true` if the rectangle `self` fully contains `b`
<!-- impl Rect::fn equal -->
Checks whether the two given rectangle are equal.
## `b`
a `Rect`

# Returns

`true` if the rectangles are equal
<!-- impl Rect::fn expand -->
Expands a `Rect` to contain the given `Point`.
## `p`
a `Point`
## `res`
return location for the expanded rectangle
<!-- impl Rect::fn free -->
Frees the resources allocated by `Rect::alloc`.
<!-- impl Rect::fn get_bottom_left -->
Retrieves the coordinates of the bottom-left corner of the given rectangle.
## `p`
return location for a `Point`
<!-- impl Rect::fn get_bottom_right -->
Retrieves the coordinates of the bottom-right corner of the given rectangle.
## `p`
return location for a `Point`
<!-- impl Rect::fn get_center -->
Retrieves the coordinates of the center of the given rectangle.
## `p`
return location for a `Point`
<!-- impl Rect::fn get_height -->
Retrieves the normalized height of the given rectangle.

# Returns

the normalized height of the rectangle
<!-- impl Rect::fn get_top_left -->
Retrieves the coordinates of the top-left corner of the given rectangle.
## `p`
return location for a `Point`
<!-- impl Rect::fn get_top_right -->
Retrieves the coordinates of the top-right corner of the given rectangle.
## `p`
return location for a `Point`
<!-- impl Rect::fn get_vertices -->
Computes the four vertices of a `Rect`.
## `vertices`
return location for an array
 of 4 `Vec2`
<!-- impl Rect::fn get_width -->
Retrieves the normalized width of the given rectangle.

# Returns

the normalized width of the rectangle
<!-- impl Rect::fn get_x -->
Retrieves the normalized X coordinate of the origin of the given
rectangle.

# Returns

the normalized X coordinate of the rectangle
<!-- impl Rect::fn get_y -->
Retrieves the normalized Y coordinate of the origin of the given
rectangle.

# Returns

the normalized Y coordinate of the rectangle
<!-- impl Rect::fn init -->
Initializes the given `Rect` with the given values.

This function will implicitly normalize the `Rect`
before returning.
## `x`
the X coordinate of the `Rect.origin`
## `y`
the Y coordinate of the `Rect.origin`
## `width`
the width of the `Rect.size`
## `height`
the height of the `Rect.size`

# Returns

the initialized rectangle
<!-- impl Rect::fn init_from_rect -->
Initializes `self` using the given `src` rectangle.

This function will implicitly normalize the `Rect`
before returning.
## `src`
a `Rect`

# Returns

the initialized rectangle
<!-- impl Rect::fn inset -->
Changes the given rectangle to be smaller, or larger depending on the
given inset parameters.

To create an inset rectangle, use positive `d_x` or `d_y` values; to
create a larger, encompassing rectangle, use negative `d_x` or `d_y`
values.

The origin of the rectangle is offset by `d_x` and `d_y`, while the size
is adjusted by `(2 * @d_x, 2 * @d_y)`. If `d_x` and `d_y` are positive
values, the size of the rectangle is decreased; if `d_x` and `d_y` are
negative values, the size of the rectangle is increased.

If the size of the resulting inset rectangle has a negative width or
height then the size will be set to zero.
## `d_x`
the horizontal inset
## `d_y`
the vertical inset

# Returns

the inset rectangle
<!-- impl Rect::fn inset_r -->
Changes the given rectangle to be smaller, or larger depending on the
given inset parameters.

To create an inset rectangle, use positive `d_x` or `d_y` values; to
create a larger, encompassing rectangle, use negative `d_x` or `d_y`
values.

The origin of the rectangle is offset by `d_x` and `d_y`, while the size
is adjusted by `(2 * @d_x, 2 * @d_y)`. If `d_x` and `d_y` are positive
values, the size of the rectangle is decreased; if `d_x` and `d_y` are
negative values, the size of the rectangle is increased.

If the size of the resulting inset rectangle has a negative width or
height then the size will be set to zero.
## `d_x`
the horizontal inset
## `d_y`
the vertical inset
## `res`
return location for the inset rectangle
<!-- impl Rect::fn interpolate -->
Linearly interpolates the origin and size of the two given
rectangles.
## `b`
a `Rect`
## `factor`
the linear interpolation factor
## `res`
return location for the
 interpolated rectangle
<!-- impl Rect::fn intersection -->
Computes the intersection of the two given rectangles.

![](rectangle-intersection.png)

The intersection in the image above is the blue outline.

If the two rectangles do not intersect, `res` will contain
a degenerate rectangle with origin in (0, 0) and a size of 0.
## `b`
a `Rect`
## `res`
return location for
 a `Rect`

# Returns

`true` if the two rectangles intersect
<!-- impl Rect::fn normalize -->
Normalizes the passed rectangle.

This function ensures that the size of the rectangle is made of
positive values, and that the origin is the top-left corner of
the rectangle.

# Returns

the normalized rectangle
<!-- impl Rect::fn normalize_r -->
Normalizes the passed rectangle.

This function ensures that the size of the rectangle is made of
positive values, and that the origin is in the top-left corner
of the rectangle.
## `res`
the return location for the
 normalized rectangle
<!-- impl Rect::fn offset -->
Offsets the origin by `d_x` and `d_y`.

The size of the rectangle is unchanged.
## `d_x`
the horizontal offset
## `d_y`
the vertical offset

# Returns

the offset rectangle
<!-- impl Rect::fn offset_r -->
Offsets the origin of the given rectangle by `d_x` and `d_y`.

The size of the rectangle is left unchanged.
## `d_x`
the horizontal offset
## `d_y`
the vertical offset
## `res`
return location for the offset
 rectangle
<!-- impl Rect::fn round -->
Rounds the origin and size of the given rectangle to
their nearest integer values; the rounding is guaranteed
to be large enough to contain the original rectangle.

This function is the equivalent of calling `floor` on
the coordinates of the origin, and `ceil` on the size.
## `res`
return location for the
 rounded rectangle
<!-- impl Rect::fn scale -->
Scales the size and origin of a rectangle horizontaly by `s_h`,
and vertically by `s_v`. The result `res` is normalized.

Feature: `v1_10`

## `s_h`
horizontal scale factor
## `s_v`
vertical scale factor
## `res`
return location for the
 scaled rectangle
<!-- impl Rect::fn union -->
Computes the union of the two given rectangles.

![](rectangle-union.png)

The union in the image above is the blue outline.
## `b`
a `Rect`
## `res`
return location for a `Rect`
<!-- impl Rect::fn alloc -->
Allocates a new `Rect`.

The contents of the returned rectangle are undefined.

# Returns

the newly allocated rectangle
<!-- impl Rect::fn zero -->
Returns a degenerate rectangle with origin fixed at (0, 0) and
a size of 0, 0.

# Returns

a fixed rectangle
<!-- struct Size -->
A size.
<!-- impl Size::fn alloc -->
Allocates a new `Size`.

The contents of the returned value are undefined.

# Returns

the newly allocated `Size`
<!-- impl Size::fn equal -->
Checks whether the two give `Size` are equal.
## `b`
a `Size`

# Returns

`true` if the sizes are equal
<!-- impl Size::fn free -->
Frees the resources allocated by `Size::alloc`.
<!-- impl Size::fn init -->
Initializes a `Size` using the given `width` and `height`.
## `width`
the width
## `height`
the height

# Returns

the initialized `Size`
<!-- impl Size::fn init_from_size -->
Initializes a `Size` using the width and height of
the given `src`.
## `src`
a `Size`

# Returns

the initialized `Size`
<!-- impl Size::fn interpolate -->
Linearly interpolates the two given `Size` using the given
interpolation `factor`.
## `b`
a `Size`
## `factor`
the linear interpolation factor
## `res`
return location for the interpolated size
<!-- impl Size::fn scale -->
Scales the components of a `Size` using the given `factor`.
## `factor`
the scaling factor
## `res`
return location for the scaled size
<!-- impl Size::fn zero -->
A constant pointer to a zero `Size`, useful for
equality checks and interpolations.

# Returns

a constant size
<!-- struct Sphere -->
A sphere, represented by its center and radius.
<!-- impl Sphere::fn alloc -->
Allocates a new `Sphere`.

The contents of the newly allocated structure are undefined.

# Returns

the newly allocated `Sphere`. Use
 `Sphere::free` to free the resources allocated by this function
<!-- impl Sphere::fn contains_point -->
Checks whether the given `point` is contained in the volume
of a `Sphere`.
## `point`
a `Point3D`

# Returns

`true` if the sphere contains the point
<!-- impl Sphere::fn distance -->
Computes the distance of the given `point` from the surface of
a `Sphere`.
## `point`
a `Point3D`

# Returns

the distance of the point
<!-- impl Sphere::fn equal -->
Checks whether two `Sphere` are equal.
## `b`
a `Sphere`

# Returns

`true` if the spheres are equal
<!-- impl Sphere::fn free -->
Frees the resources allocated by `Sphere::alloc`.
<!-- impl Sphere::fn get_bounding_box -->
Computes the bounding box capable of containing the
given `Sphere`.
## `box_`
return location for the bounding box
<!-- impl Sphere::fn get_center -->
Retrieves the coordinates of the center of a `Sphere`.
## `center`
return location for the coordinates of
 the center
<!-- impl Sphere::fn get_radius -->
Retrieves the radius of a `Sphere`.
<!-- impl Sphere::fn init -->
Initializes the given `Sphere` with the given `center` and `radius`.
## `center`
the coordinates of the center of the sphere, or `None`
 for a center in (0, 0, 0)
## `radius`
the radius of the sphere

# Returns

the initialized `Sphere`
<!-- impl Sphere::fn init_from_points -->
Initializes the given `Sphere` using the given array
of 3D coordinates so that the sphere includes them.

The center of the sphere can either be specified, or will be center
of the 3D volume that encompasses all `points`.
## `n_points`
the number of `Point3D` in the `points` array
## `points`
an array of `Point3D`
## `center`
the center of the sphere

# Returns

the initialized `Sphere`
<!-- impl Sphere::fn init_from_vectors -->
Initializes the given `Sphere` using the given array
of 3D coordinates so that the sphere includes them.

The center of the sphere can either be specified, or will be center
of the 3D volume that encompasses all `vectors`.
## `n_vectors`
the number of `Vec3` in the `vectors` array
## `vectors`
an array of `Vec3`
## `center`
the center of the sphere

# Returns

the initialized `Sphere`
<!-- impl Sphere::fn is_empty -->
Checks whether the sphere has a zero radius.

# Returns

`true` if the sphere is empty
<!-- impl Sphere::fn translate -->
Translates the center of the given `Sphere` using the `point`
coordinates as the delta of the translation.
## `point`
the coordinates of the translation
## `res`
return location for the translated sphere
<!-- struct Triangle -->
A triangle.
<!-- impl Triangle::fn alloc -->
Allocates a new `Triangle`.

The contents of the returned structure are undefined.

# Returns

the newly allocated `Triangle`
 structure. Use `Triangle::free` to free the resources
 allocated by this function
<!-- impl Triangle::fn contains_point -->
Checks whether the given triangle `self` contains the point `p`.
## `p`
a `Point3D`

# Returns

`true` if the point is inside the triangle
<!-- impl Triangle::fn equal -->
Checks whether the two given `Triangle` are equal.
## `b`
a `Triangle`

# Returns

`true` if the triangles are equal
<!-- impl Triangle::fn free -->
Frees the resources allocated by `Triangle::alloc`.
<!-- impl Triangle::fn get_area -->
Computes the area of the given `Triangle`.

# Returns

the area of the triangle
<!-- impl Triangle::fn get_barycoords -->
Computes the [barycentric coordinates](http://en.wikipedia.org/wiki/Barycentric_coordinate_system)
of the given point `p`.

The point `p` must lie on the same plane as the triangle `self`; if the
point is not coplanar, the result of this function is undefined.

If we place the origin in the coordinates of the triangle's A point,
the barycentric coordinates are `u`, which is on the AC vector; and `v`
which is on the AB vector:

![](triangle-barycentric.png)

The returned `Vec2` contains the following values, in order:

 - `res.x = u`
 - `res.y = v`
## `p`
a `Point3D`
## `res`
return location for the vector
 with the barycentric coordinates

# Returns

`true` if the barycentric coordinates are valid
<!-- impl Triangle::fn get_bounding_box -->
Computes the bounding box of the given `Triangle`.
## `res`
return location for the box
<!-- impl Triangle::fn get_midpoint -->
Computes the coordinates of the midpoint of the given `Triangle`.

The midpoint G is the [centroid](https://en.wikipedia.org/wiki/Centroid`Triangle_centroid`)
of the triangle, i.e. the intersection of its medians.
## `res`
return location for the coordinates of
 the midpoint
<!-- impl Triangle::fn get_normal -->
Computes the normal vector of the given `Triangle`.
## `res`
return location for the normal vector
<!-- impl Triangle::fn get_plane -->
Computes the plane based on the vertices of the given `Triangle`.
## `res`
return location for the plane
<!-- impl Triangle::fn get_points -->
Retrieves the three vertices of the given `Triangle` and returns
their coordinates as `Point3D`.
## `a`
return location for the coordinates
 of the first vertex
## `b`
return location for the coordinates
 of the second vertex
## `c`
return location for the coordinates
 of the third vertex
<!-- impl Triangle::fn get_vertices -->
Retrieves the three vertices of the given `Triangle`.
## `a`
return location for the first vertex
## `b`
return location for the second vertex
## `c`
return location for the third vertex
<!-- impl Triangle::fn init_from_point3d -->
Initializes a `Triangle` using the three given 3D points.
## `a`
a `Point3D`
## `b`
a `Point3D`
## `c`
a `Point3D`

# Returns

the initialized `Triangle`
<!-- impl Triangle::fn init_from_vec3 -->
Initializes a `Triangle` using the three given vectors.
## `a`
a `Vec3`
## `b`
a `Vec3`
## `c`
a `Vec3`

# Returns

the initialized `Triangle`
<!-- struct Vec2 -->
A structure capable of holding a vector with two dimensions, x and y.

The contents of the `Vec2` structure are private and should
never be accessed directly.
<!-- impl Vec2::fn alloc -->
Allocates a new `Vec2` structure.

The contents of the returned structure are undefined.

Use `Vec2::init` to initialize the vector.

# Returns

the newly allocated `Vec2`
 structure. Use `Vec2::free` to free the resources allocated
 by this function.
<!-- impl Vec2::fn add -->
Adds each component of the two passed vectors and places
each result into the components of `res`.
## `b`
a `Vec2`
## `res`
return location for the result
<!-- impl Vec2::fn divide -->
Divides each component of the first operand `self` by the corresponding
component of the second operand `b`, and places the results into the
vector `res`.
## `b`
a `Vec2`
## `res`
return location for the result
<!-- impl Vec2::fn dot -->
Computes the dot product of the two given vectors.
## `b`
a `Vec2`

# Returns

the dot product of the vectors
<!-- impl Vec2::fn equal -->
Checks whether the two given `Vec2` are equal.
## `v2`
a `Vec2`

# Returns

`true` if the two vectors are equal, and false otherwise
<!-- impl Vec2::fn free -->
Frees the resources allocated by `self`
<!-- impl Vec2::fn get_x -->
Retrieves the X component of the `Vec2`.

# Returns

the value of the X component
<!-- impl Vec2::fn get_y -->
Retrieves the Y component of the `Vec2`.

# Returns

the value of the Y component
<!-- impl Vec2::fn init -->
Initializes a `Vec2` using the given values.

This function can be called multiple times.
## `x`
the X field of the vector
## `y`
the Y field of the vector

# Returns

the initialized vector
<!-- impl Vec2::fn init_from_float -->
Initializes `self` with the contents of the given array.
## `src`
an array of floating point values
 with at least two elements

# Returns

the initialized vector
<!-- impl Vec2::fn init_from_vec2 -->
Copies the contents of `src` into `self`.
## `src`
a `Vec2`

# Returns

the initialized vector
<!-- impl Vec2::fn length -->
Computes the length of the given vector.

# Returns

the length of the vector
<!-- impl Vec2::fn max -->
Compares the two given vectors and places the maximum
values of each component into `res`.
## `b`
a `Vec2`
## `res`
the resulting vector
<!-- impl Vec2::fn min -->
Compares the two given vectors and places the minimum
values of each component into `res`.
## `b`
a `Vec2`
## `res`
the resulting vector
<!-- impl Vec2::fn multiply -->
Multiplies each component of the two passed vectors and places
each result into the components of `res`.
## `b`
a `Vec2`
## `res`
return location for the result
<!-- impl Vec2::fn near -->
Compares the two given `Vec2` vectors and checks
whether their values are within the given `epsilon`.
## `v2`
a `Vec2`
## `epsilon`
the threshold between the two vectors

# Returns

`true` if the two vectors are near each other
<!-- impl Vec2::fn negate -->
Negates the given `Vec2`.
## `res`
return location for the result vector
<!-- impl Vec2::fn normalize -->
Computes the normalized vector for the given vector `self`.
## `res`
return location for the
 normalized vector
<!-- impl Vec2::fn scale -->
Multiplies all components of the given vector with the given scalar `factor`.
## `factor`
the scalar factor
## `res`
return location for the result vector
<!-- impl Vec2::fn subtract -->
Subtracts from each component of the first operand `self` the
corresponding component of the second operand `b` and places
each result into the components of `res`.
## `b`
a `Vec2`
## `res`
return location for the result
<!-- impl Vec2::fn to_float -->
Stores the components of `self` into an array.
## `dest`
return location
 for an array of floating point values with at least 2 elements
<!-- impl Vec2::fn one -->
Retrieves a constant vector with (1, 1) components.

# Returns

the one vector
<!-- impl Vec2::fn x_axis -->
Retrieves a constant vector with (1, 0) components.

# Returns

the X axis vector
<!-- impl Vec2::fn y_axis -->
Retrieves a constant vector with (0, 1) components.

# Returns

the Y axis vector
<!-- impl Vec2::fn zero -->
Retrieves a constant vector with (0, 0) components.

# Returns

the zero vector
<!-- struct Vec3 -->
A structure capable of holding a vector with three dimensions: x, y, and z.

The contents of the `Vec3` structure are private and should
never be accessed directly.
<!-- impl Vec3::fn alloc -->
Allocates a new `Vec3` structure.

The contents of the returned structure are undefined.

Use `Vec3::init` to initialize the vector.

# Returns

the newly allocated `Vec3`
 structure. Use `Vec3::free` to free the resources allocated
 by this function.
<!-- impl Vec3::fn add -->
Adds each component of the two given vectors.
## `b`
a `Vec3`
## `res`
return location for the resulting vector
<!-- impl Vec3::fn cross -->
Computes the cross product of the two given vectors.
## `b`
a `Vec3`
## `res`
return location for the resulting vector
<!-- impl Vec3::fn divide -->
Divides each component of the first operand `self` by the corresponding
component of the second operand `b`, and places the results into the
vector `res`.
## `b`
a `Vec3`
## `res`
return location for the resulting vector
<!-- impl Vec3::fn dot -->
Computes the dot product of the two given vectors.
## `b`
a `Vec3`

# Returns

the value of the dot product
<!-- impl Vec3::fn equal -->
Checks whether the two given `Vec3` are equal.
## `v2`
a `Vec3`

# Returns

`true` if the two vectors are equal, and false otherwise
<!-- impl Vec3::fn free -->
Frees the resources allocated by `self`
<!-- impl Vec3::fn get_x -->
Retrieves the first component of the given vector `self`.

# Returns

the value of the first component of the vector
<!-- impl Vec3::fn get_xy -->
Creates a `Vec2` that contains the first and second
components of the given `Vec3`.
## `res`
return location for a `Vec2`
<!-- impl Vec3::fn get_xy0 -->
Creates a `Vec3` that contains the first two components of
the given `Vec3`, and the third component set to 0.
## `res`
return location for a `Vec3`
<!-- impl Vec3::fn get_xyz0 -->
Converts a `Vec3` in a `Vec4` using 0.0
as the value for the fourth component of the resulting vector.
## `res`
return location for the vector
<!-- impl Vec3::fn get_xyz1 -->
Converts a `Vec3` in a `Vec4` using 1.0
as the value for the fourth component of the resulting vector.
## `res`
return location for the vector
<!-- impl Vec3::fn get_xyzw -->
Converts a `Vec3` in a `Vec4` using `w` as
the value of the fourth component of the resulting vector.
## `w`
the value of the W component
## `res`
return location for the vector
<!-- impl Vec3::fn get_y -->
Retrieves the second component of the given vector `self`.

# Returns

the value of the second component of the vector
<!-- impl Vec3::fn get_z -->
Retrieves the third component of the given vector `self`.

# Returns

the value of the third component of the vector
<!-- impl Vec3::fn init -->
Initializes a `Vec3` using the given values.

This function can be called multiple times.
## `x`
the X field of the vector
## `y`
the Y field of the vector
## `z`
the Z field of the vector

# Returns

a pointer to the initialized
 vector
<!-- impl Vec3::fn init_from_float -->
Initializes a `Vec3` with the values from an array.
## `src`
an array of 3 floating point values

# Returns

the initialized vector
<!-- impl Vec3::fn init_from_vec3 -->
Initializes a `Vec3` with the values of another
`Vec3`.
## `src`
a `Vec3`

# Returns

the initialized vector
<!-- impl Vec3::fn length -->
Retrieves the length of the given vector `self`.

# Returns

the value of the length of the vector
<!-- impl Vec3::fn max -->
Compares each component of the two given vectors and creates a
vector that contains the maximum values.
## `b`
a `Vec3`
## `res`
return location for the result vector
<!-- impl Vec3::fn min -->
Compares each component of the two given vectors and creates a
vector that contains the minimum values.
## `b`
a `Vec3`
## `res`
return location for the result vector
<!-- impl Vec3::fn multiply -->
Multiplies each component of the two given vectors.
## `b`
a `Vec3`
## `res`
return location for the resulting vector
<!-- impl Vec3::fn near -->
Compares the two given `Vec3` vectors and checks
whether their values are within the given `epsilon`.
## `v2`
a `Vec3`
## `epsilon`
the threshold between the two vectors

# Returns

`true` if the two vectors are near each other
<!-- impl Vec3::fn negate -->
Negates the given `Vec3`.
## `res`
return location for the result vector
<!-- impl Vec3::fn normalize -->
Normalizes the given `Vec3`.
## `res`
return location for the normalized vector
<!-- impl Vec3::fn scale -->
Multiplies all components of the given vector with the given scalar `factor`.
## `factor`
the scalar factor
## `res`
return location for the result vector
<!-- impl Vec3::fn subtract -->
Subtracts from each component of the first operand `self` the
corresponding component of the second operand `b` and places
each result into the components of `res`.
## `b`
a `Vec3`
## `res`
return location for the resulting vector
<!-- impl Vec3::fn to_float -->
Copies the components of a `Vec3` into the given array.
## `dest`
return location for
 an array of floating point values
<!-- impl Vec3::fn one -->
Provides a constant pointer to a vector with three components,
all sets to 1.

# Returns

a constant vector
<!-- impl Vec3::fn x_axis -->
Provides a constant pointer to a vector with three components
with values set to (1, 0, 0).

# Returns

a constant vector
<!-- impl Vec3::fn y_axis -->
Provides a constant pointer to a vector with three components
with values set to (0, 1, 0).

# Returns

a constant vector
<!-- impl Vec3::fn z_axis -->
Provides a constant pointer to a vector with three components
with values set to (0, 0, 1).

# Returns

a constant vector
<!-- impl Vec3::fn zero -->
Provides a constant pointer to a vector with three components,
all sets to 0.

# Returns

a constant vector
<!-- struct Vec4 -->
A structure capable of holding a vector with four dimensions: x, y, z, and w.

The contents of the `Vec4` structure are private and should
never be accessed directly.
<!-- impl Vec4::fn alloc -->
Allocates a new `Vec4` structure.

The contents of the returned structure are undefined.

Use `Vec4::init` to initialize the vector.

# Returns

the newly allocated `Vec4`
 structure. Use `Vec4::free` to free the resources allocated
 by this function.
<!-- impl Vec4::fn add -->
Adds each component of the two given vectors.
## `b`
a `Vec4`
## `res`
return location for the resulting vector
<!-- impl Vec4::fn divide -->
Divides each component of the first operand `self` by the corresponding
component of the second operand `b`, and places the results into the
vector `res`.
## `b`
a `Vec4`
## `res`
return location for the resulting vector
<!-- impl Vec4::fn dot -->
Computes the dot product of the two given vectors.
## `b`
a `Vec4`

# Returns

the value of the dot product
<!-- impl Vec4::fn equal -->
Checks whether the two given `Vec4` are equal.
## `v2`
a `Vec4`

# Returns

`true` if the two vectors are equal, and false otherwise
<!-- impl Vec4::fn free -->
Frees the resources allocated by `self`
<!-- impl Vec4::fn get_w -->
Retrieves the value of the fourth component of the given `Vec4`.

# Returns

the value of the fourth component
<!-- impl Vec4::fn get_x -->
Retrieves the value of the first component of the given `Vec4`.

# Returns

the value of the first component
<!-- impl Vec4::fn get_xy -->
Creates a `Vec2` that contains the first two components
of the given `Vec4`.
## `res`
return location for a `Vec2`
<!-- impl Vec4::fn get_xyz -->
Creates a `Vec3` that contains the first three components
of the given `Vec4`.
## `res`
return location for a graphene_vec3_t
<!-- impl Vec4::fn get_y -->
Retrieves the value of the second component of the given `Vec4`.

# Returns

the value of the second component
<!-- impl Vec4::fn get_z -->
Retrieves the value of the third component of the given `Vec4`.

# Returns

the value of the third component
<!-- impl Vec4::fn init -->
Initializes a `Vec4` using the given values.

This function can be called multiple times.
## `x`
the X field of the vector
## `y`
the Y field of the vector
## `z`
the Z field of the vector
## `w`
the W field of the vector

# Returns

a pointer to the initialized
 vector
<!-- impl Vec4::fn init_from_float -->
Initializes a `Vec4` with the values inside the given array.
## `src`
an array of four floating point values

# Returns

the initialized vector
<!-- impl Vec4::fn init_from_vec2 -->
Initializes a `Vec4` using the components of a
`Vec2` and the values of `z` and `w`.
## `src`
a `Vec2`
## `z`
the value for the third component of `self`
## `w`
the value for the fourth component of `self`

# Returns

the initialized vector
<!-- impl Vec4::fn init_from_vec3 -->
Initializes a `Vec4` using the components of a
`Vec3` and the value of `w`.
## `src`
a `Vec3`
## `w`
the value for the fourth component of `self`

# Returns

the initialized vector
<!-- impl Vec4::fn init_from_vec4 -->
Initializes a `Vec4` using the components of
another `Vec4`.
## `src`
a `Vec4`

# Returns

the initialized vector
<!-- impl Vec4::fn length -->
Computes the length of the given `Vec4`.

# Returns

the length of the vector
<!-- impl Vec4::fn max -->
Compares each component of the two given vectors and creates a
vector that contains the maximum values.
## `b`
a `Vec4`
## `res`
return location for the result vector
<!-- impl Vec4::fn min -->
Compares each component of the two given vectors and creates a
vector that contains the minimum values.
## `b`
a `Vec4`
## `res`
return location for the result vector
<!-- impl Vec4::fn multiply -->
Multiplies each component of the two given vectors.
## `b`
a `Vec4`
## `res`
return location for the resulting vector
<!-- impl Vec4::fn near -->
Compares the two given `Vec4` vectors and checks
whether their values are within the given `epsilon`.
## `v2`
a `Vec4`
## `epsilon`
the threshold between the two vectors

# Returns

`true` if the two vectors are near each other
<!-- impl Vec4::fn negate -->
Negates the given `Vec4`.
## `res`
return location for the result vector
<!-- impl Vec4::fn normalize -->
Normalizes the given `Vec4`.
## `res`
return location for the normalized
 vector
<!-- impl Vec4::fn scale -->
Multiplies all components of the given vector with the given scalar `factor`.
## `factor`
the scalar factor
## `res`
return location for the result vector
<!-- impl Vec4::fn subtract -->
Subtracts from each component of the first operand `self` the
corresponding component of the second operand `b` and places
each result into the components of `res`.
## `b`
a `Vec4`
## `res`
return location for the resulting vector
<!-- impl Vec4::fn to_float -->
Stores the components of the given `Vec4` into an array
of floating point values.
## `dest`
return location for
 an array of floating point values
<!-- impl Vec4::fn one -->
Retrieves a pointer to a `Vec4` with all its
components set to 1.

# Returns

a constant vector
<!-- impl Vec4::fn w_axis -->
Retrieves a pointer to a `Vec4` with its
components set to (0, 0, 0, 1).

# Returns

a constant vector
<!-- impl Vec4::fn x_axis -->
Retrieves a pointer to a `Vec4` with its
components set to (1, 0, 0, 0).

# Returns

a constant vector
<!-- impl Vec4::fn y_axis -->
Retrieves a pointer to a `Vec4` with its
components set to (0, 1, 0, 0).

# Returns

a constant vector
<!-- impl Vec4::fn z_axis -->
Retrieves a pointer to a `Vec4` with its
components set to (0, 0, 1, 0).

# Returns

a constant vector
<!-- impl Vec4::fn zero -->
Retrieves a pointer to a `Vec4` with all its
components set to 0.

# Returns

a constant vector
