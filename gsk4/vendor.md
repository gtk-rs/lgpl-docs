<!-- file * -->
<!-- enum BlendMode -->
The blend modes available for render nodes.

The implementation of each blend mode is deferred to the
rendering pipeline.
<!-- enum BlendMode::variant Default -->
The default blend mode, which specifies no blending
<!-- enum BlendMode::variant Multiply -->
The source color is multiplied by the destination
 and replaces the destination
<!-- enum BlendMode::variant Screen -->
...
<!-- enum BlendMode::variant Overlay -->
...
<!-- enum BlendMode::variant Darken -->
...
<!-- enum BlendMode::variant Lighten -->
...
<!-- enum BlendMode::variant ColorDodge -->
...
<!-- enum BlendMode::variant ColorBurn -->
...
<!-- enum BlendMode::variant HardLight -->
...
<!-- enum BlendMode::variant SoftLight -->
...
<!-- enum BlendMode::variant Difference -->
...
<!-- enum BlendMode::variant Exclusion -->
...
<!-- enum BlendMode::variant Color -->
...
<!-- enum BlendMode::variant Hue -->
...
<!-- enum BlendMode::variant Saturation -->
...
<!-- enum BlendMode::variant Luminosity -->
...
<!-- struct BroadwayRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- impl BroadwayRenderer::fn new -->
Creates a new Broadway renderer.

The Broadway renderer is the default renderer for the broadway backend.
It will only work with broadway surfaces, otherwise it will fail the
call to `gdk_renderer_realize`.

This function is only available when GTK was compiled with Broadway
support.

# Returns

a new Broadway renderer.
<!-- struct CairoRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- impl CairoRenderer::fn new -->
Creates a new Cairo renderer.

The Cairo renderer is the fallback renderer drawing in ways similar
to how GTK 3 drew its content. Its primary use is as comparison tool.

The Cairo renderer is incomplete. It cannot render 3D transformed
content and will instead render an error marker. Its usage should be
avoided.

# Returns

a new Cairo renderer.
<!-- struct ColorStop -->
<!-- enum Corner -->
The corner indices used by `RoundedRect`.
<!-- enum Corner::variant TopLeft -->
The top left corner
<!-- enum Corner::variant TopRight -->
The top right corner
<!-- enum Corner::variant BottomRight -->
The bottom right corner
<!-- enum Corner::variant BottomLeft -->
The bottom left corner
<!-- struct GLRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- impl GLRenderer::fn new -->
Creates a new `Renderer` using OpenGL. This is the default renderer
used by GTK.

# Returns

a new GL renderer
<!-- struct RenderNode -->
The `GskRenderNode` structure contains only private data.
<!-- impl RenderNode::fn draw -->
Draw the contents of `self` to the given cairo context.

Typically, you'll use this function to implement fallback rendering
of `GskRenderNodes` on an intermediate Cairo context, instead of using
the drawing context associated to a `gdk::Surface`'s rendering buffer.

For advanced nodes that cannot be supported using Cairo, in particular
for nodes doing 3D operations, this function may fail.
## `cr`
cairo context to draw to
<!-- impl RenderNode::fn get_bounds -->
Retrieves the boundaries of the `self`. The node will not draw outside
of its boundaries.
## `bounds`
return location for the boundaries
<!-- impl RenderNode::fn get_node_type -->
Returns the type of the `self`.

# Returns

the type of the `RenderNode`
<!-- impl RenderNode::fn ref -->
Acquires a reference on the given `RenderNode`.

# Returns

the `RenderNode` with an additional reference
<!-- impl RenderNode::fn serialize -->
Serializes the `self` for later deserialization via
`RenderNode::deserialize`. No guarantees are made about the format
used other than that the same version of GTK+ will be able to deserialize
the result of a call to `RenderNode::serialize` and
`RenderNode::deserialize` will correctly reject files it cannot open
that were created with previous versions of GTK+.

The intended use of this functions is testing, benchmarking and debugging.
The format is not meant as a permanent storage format.

# Returns

a `glib::Bytes` representing the node.
<!-- impl RenderNode::fn unref -->
Releases a reference on the given `RenderNode`.

If the reference was the last, the resources associated to the `self` are
freed.
<!-- impl RenderNode::fn write_to_file -->
This function is equivalent to calling `RenderNode::serialize`
followed by `g_file_set_contents`. See those two functions for details
on the arguments.

It is mostly intended for use inside a debugger to quickly dump a render
node to a file for later inspection.
## `filename`
the file to save it to.

# Returns

`true` if saving was successful
<!-- impl RenderNode::fn deserialize -->
Loads data previously created via `RenderNode::serialize`. For a
discussion of the supported format, see that function.
## `bytes`
the bytes containing the data

# Returns

a new `RenderNode` or `None` on
 error.
<!-- enum RenderNodeType -->
The type of a node determines what the node is rendering.
<!-- enum RenderNodeType::variant NotARenderNode -->
Error type. No node will ever have this type.
<!-- enum RenderNodeType::variant ContainerNode -->
A node containing a stack of children
<!-- enum RenderNodeType::variant CairoNode -->
A node drawing a `cairo::Surface`
<!-- enum RenderNodeType::variant ColorNode -->
A node drawing a single color rectangle
<!-- enum RenderNodeType::variant LinearGradientNode -->
A node drawing a linear gradient
<!-- enum RenderNodeType::variant RepeatingLinearGradientNode -->
A node drawing a repeating linear gradient
<!-- enum RenderNodeType::variant BorderNode -->
A node stroking a border around an area
<!-- enum RenderNodeType::variant TextureNode -->
A node drawing a `gdk::Texture`
<!-- enum RenderNodeType::variant InsetShadowNode -->
A node drawing an inset shadow
<!-- enum RenderNodeType::variant OutsetShadowNode -->
A node drawing an outset shadow
<!-- enum RenderNodeType::variant TransformNode -->
A node that renders its child after applying a matrix transform
<!-- enum RenderNodeType::variant OpacityNode -->
A node that changes the opacity of its child
<!-- enum RenderNodeType::variant ColorMatrixNode -->
A node that applies a color matrix to every pixel
<!-- enum RenderNodeType::variant RepeatNode -->
A node that repeats the child's contents
<!-- enum RenderNodeType::variant ClipNode -->
A node that clips its child to a rectangular area
<!-- enum RenderNodeType::variant RoundedClipNode -->
A node that clips its child to a rounded rectangle
<!-- enum RenderNodeType::variant ShadowNode -->
A node that draws a shadow below its child
<!-- enum RenderNodeType::variant BlendNode -->
A node that blends two children together
<!-- enum RenderNodeType::variant CrossFadeNode -->
A node that cross-fades between two children
<!-- enum RenderNodeType::variant TextNode -->
A node containing a glyph string
<!-- enum RenderNodeType::variant BlurNode -->
A node that applies a blur
<!-- struct Renderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- trait RendererExt -->
Trait containing all `Renderer` methods.

# Implementors

[`BroadwayRenderer`](struct.BroadwayRenderer.html), [`CairoRenderer`](struct.CairoRenderer.html), [`GLRenderer`](struct.GLRenderer.html), [`Renderer`](struct.Renderer.html), [`VulkanRenderer`](struct.VulkanRenderer.html)
<!-- impl Renderer::fn new_for_surface -->
Creates an appropriate `Renderer` instance for the given `surface`.

The renderer will be realized when it is returned.
## `surface`
a `gdk::Surface`

# Returns

a `Renderer`
<!-- trait RendererExt::fn get_surface -->
Retrieves the `gdk::Surface` set using `RendererExt::realize`. If the renderer
has not been realized yet, `None` will be returned.

# Returns

a `gdk::Surface`
<!-- trait RendererExt::fn is_realized -->
Checks whether the `self` is realized or not.

# Returns

`true` if the `Renderer` was realized, and `false` otherwise
<!-- trait RendererExt::fn realize -->
Creates the resources needed by the `self` to render the scene
graph.
## `surface`
the `gdk::Surface` renderer will be used on
<!-- trait RendererExt::fn render -->
Renders the scene graph, described by a tree of `RenderNode` instances,
ensuring that the given `region` gets redrawn.

Renderers must ensure that changes of the contents given by the `root`
node as well as the area given by `region` are redrawn. They are however
free to not redraw any pixel outside of `region` if they can guarantee that
it didn't change.

The `self` will acquire a reference on the `RenderNode` tree while
the rendering is in progress.
## `root`
a `RenderNode`
## `region`
the `cairo::Region` that must be redrawn or `None`
 for the whole window
<!-- trait RendererExt::fn render_texture -->
Renders the scene graph, described by a tree of `RenderNode` instances,
to a `gdk::Texture`.

The `self` will acquire a reference on the `RenderNode` tree while
the rendering is in progress.

If you want to apply any transformations to `root`, you should put it into a
transform node and pass that node instead.
## `root`
a `RenderNode`
## `viewport`
the section to draw or `None` to use `root`'s bounds

# Returns

a `gdk::Texture` with the rendered contents of `root`.
<!-- trait RendererExt::fn unrealize -->
Releases all the resources created by `RendererExt::realize`.
<!-- struct RoundedRect -->
A rectangular region with rounded corners.

Application code should normalize rectangles using `RoundedRect::normalize`;
this function will ensure that the bounds of the rectanlge are normalized
and ensure that the corner values are positive and the corners do not overlap.
All functions taking a `RoundedRect` as an argument will internally operate on
a normalized copy; all functions returning a `RoundedRect` will always return
a normalized one.
<!-- impl RoundedRect::fn contains_point -->
Checks if the given `point` is inside the rounded rectangle. This function
returns `false` if the point is in the rounded corner areas.
## `point`
the point to check

# Returns

`true` if the `point` is inside the rounded rectangle
<!-- impl RoundedRect::fn contains_rect -->
Checks if the given `rect` is contained inside the rounded rectangle.
This function returns `false` if `rect` extends into one of the rounded
corner areas.
## `rect`
the rectangle to check

# Returns

`true` if the `rect` is fully contained inside the rounded rectangle
<!-- impl RoundedRect::fn init -->
Initializes the given `RoundedRect` with the given values.

This function will implicitly normalize the `RoundedRect`
before returning.
## `bounds`
a `graphene::Rect` describing the bounds
## `top_left`
the rounding radius of the top left corner
## `top_right`
the rounding radius of the top right corner
## `bottom_right`
the rounding radius of the bottom right corner
## `bottom_left`
the rounding radius of the bottom left corner

# Returns

the initialized rectangle
<!-- impl RoundedRect::fn init_copy -->
Initializes `self` using the given `src` rectangle.

This function will not normalize the `RoundedRect`, so
make sure the source is normalized.
## `src`
a `RoundedRect`

# Returns

the initialized rectangle
<!-- impl RoundedRect::fn init_from_rect -->
Initializes `self` to the given `bounds` and sets the radius of all
four corners to `radius`.
## `bounds`
a `graphene::Rect`
## `radius`
the border radius

# Returns

the initialized rectangle
<!-- impl RoundedRect::fn intersects_rect -->
Checks if part of the given `rect` is contained inside the rounded rectangle.
This function returns `false` if `rect` only extends into one of the rounded
corner areas but not into the rounded rectangle itself.
## `rect`
the rectangle to check

# Returns

`true` if the `rect` intersects with the rounded rectangle
<!-- impl RoundedRect::fn is_rectilinear -->
Checks if all corners of `self` are right angles and the
rectangle covers all of its bounds.

This information can be used to decide if `gsk_clip_node_new`
or `gsk_rounded_clip_node_new` should be called.

# Returns

`true` if the rectangle is rectilinear
<!-- impl RoundedRect::fn normalize -->
Normalizes the passed rectangle.

this function will ensure that the bounds of the rectanlge are normalized
and ensure that the corner values are positive and the corners do not overlap.

# Returns

the normalized rectangle
<!-- impl RoundedRect::fn offset -->
Offsets the bound's origin by `dx` and `dy`.

The size and corners of the rectangle are unchanged.
## `dx`
the horizontal offset
## `dy`
the vertical offset

# Returns

the offset rectangle
<!-- impl RoundedRect::fn shrink -->
Shrinks (or grows) the given rectangle by moving the 4 sides
according to the offsets given. The corner radii will be changed
in a way that tries to keep the center of the corner circle intact.
This emulates CSS behavior.

This function also works for growing rectangles if you pass
negative values for the `top`, `right`, `bottom` or `left`.
## `top`
How far to move the top side downwards
## `right`
How far to move the right side to the left
## `bottom`
How far to move the bottom side upwards
## `left`
How far to move the left side to the right

# Returns

the resized `RoundedRect`
<!-- enum ScalingFilter -->
The filters used when scaling texture data.

The actual implementation of each filter is deferred to the
rendering pipeline.
<!-- enum ScalingFilter::variant Linear -->
linear interpolation filter
<!-- enum ScalingFilter::variant Nearest -->
nearest neighbor interpolation filter
<!-- enum ScalingFilter::variant Trilinear -->
linear interpolation along each axis,
 plus mipmap generation, with linear interpolation along the mipmap
 levels
<!-- enum SerializationError -->
Errors that can happen during (de)serialization.
<!-- enum SerializationError::variant UnsupportedFormat -->
The format can not be
 identified
<!-- enum SerializationError::variant UnsupportedVersion -->
The version of the data
 is not understood
<!-- enum SerializationError::variant InvalidData -->
The given data may not exist in
 a proper serialization
<!-- struct Shadow -->
<!-- impl Shadow::fn node_new -->
Creates a `RenderNode` that will draw a `child` with the given
`shadows` below it.
## `child`
The node to draw
## `shadows`
The shadows to apply
## `n_shadows`
number of entries in the `shadows` array

# Returns

A new `RenderNode`
<!-- struct Transform -->
The `GskTransform` structure contains only private data.
<!-- impl Transform::fn equal -->
Checks two matrices for equality. Note that matrices need to be literally
identical in their operations, it is not enough that they return the
same result in `Transform::to_matrix`.
## `second`
the second matrix

# Returns

`true` if the two matrices can be proven to be equal
<!-- impl Transform::fn get_category -->
Returns the category this transform belongs to.

# Returns

The category of the transform
<!-- impl Transform::fn invert -->
Inverts the given transform.

If `self` is not invertible, `None` is returned.
Note that inverting `None` also returns `None`, which is
the correct inverse of `None`. If you need to differentiate
between those cases, you should check `self` is not `None`
before calling this function.

# Returns

The inverted transform or `None` if the transform
 cannot be inverted.
<!-- impl Transform::fn matrix -->
Multiplies `self` with the given `matrix`.
## `matrix`
the matrix to multiply `self` with

# Returns

The new matrix
<!-- impl Transform::fn perspective -->
Applies a perspective projection transform. This transform
scales points in X and Y based on their Z value, scaling
points with positive Z values away from the origin, and
those with negative Z values towards the origin. Points
on the z=0 plane are unchanged.
## `depth`
distance of the z=0 plane. Lower values give a more
 flattened pyramid and therefore a more pronounced
 perspective effect.

# Returns

The new matrix
<!-- impl Transform::fn print -->
Converts `self` into a human-readable string representation suitable
for printing that can later be parsed with `Transform::parse`.
## `string`
The string to print into
<!-- impl Transform::fn ref -->
Acquires a reference on the given `Transform`.

# Returns

the `Transform` with an additional reference
<!-- impl Transform::fn rotate -->
Rotates `self` `angle` degrees in 2D - or in 3Dspeak, around the z axis.
## `angle`
the rotation angle, in degrees (clockwise)

# Returns

The new matrix
<!-- impl Transform::fn rotate_3d -->
Rotates `self` `angle` degrees around `axis`.

For a rotation in 2D space, use `Transform::rotate`.
## `angle`
the rotation angle, in degrees (clockwise)
## `axis`
The rotation axis

# Returns

The new matrix
<!-- impl Transform::fn scale -->
Scales `self` in 2-dimensional space by the given factors.
Use `Transform::scale_3d` to scale in all 3 dimensions.
## `factor_x`
scaling factor on the X axis
## `factor_y`
scaling factor on the Y axis

# Returns

The new matrix
<!-- impl Transform::fn scale_3d -->
Scales `self` by the given factors.
## `factor_x`
scaling factor on the X axis
## `factor_y`
scaling factor on the Y axis
## `factor_z`
scaling factor on the Z axis

# Returns

The new matrix
<!-- impl Transform::fn to_2d -->
Converts a `Transform` to a 2D transformation
matrix.
`self` must be a 2D transformation. If you are not
sure, use `Transform::get_category` >=
`TransformCategory::2d` to check.

The returned values have the following layout:


```plain
  | xx yx |   |  a  b  0 |
  | xy yy | = |  c  d  0 |
  | x0 y0 |   | tx ty  1 |
```

This function can be used to convert between a `Transform`
and a matrix type from other 2D drawing libraries, in particular
Cairo.
## `out_xx`
return location for the xx member
## `out_yx`
return location for the yx member
## `out_xy`
return location for the xy member
## `out_yy`
return location for the yy member
## `out_dx`
return location for the x0 member
## `out_dy`
return location for the y0 member
<!-- impl Transform::fn to_affine -->
Converts a `Transform` to 2D affine transformation
factors.
`self` must be a 2D transformation. If you are not
sure, use `Transform::get_category` >=
`TransformCategory::2dAffine` to check.
## `out_scale_x`
return location for the scale
 factor in the x direction
## `out_scale_y`
return location for the scale
 factor in the y direction
## `out_dx`
return location for the translation
 in the x direction
## `out_dy`
return location for the translation
 in the y direction
<!-- impl Transform::fn to_matrix -->
Computes the actual value of `self` and stores it in `out_matrix`.
The previous value of `out_matrix` will be ignored.
## `out_matrix`
The matrix to set
<!-- impl Transform::fn to_string -->
Converts a matrix into a string that is suitable for
printing and can later be parsed with `Transform::parse`.

This is a wrapper around `Transform::print`, see that function
for details.

# Returns

A new string for `self`
<!-- impl Transform::fn to_translate -->
Converts a `Transform` to a translation operation.
`self` must be a 2D transformation. If you are not
sure, use `Transform::get_category` >=
`TransformCategory::2dTranslate` to check.
## `out_dx`
return location for the translation
 in the x direction
## `out_dy`
return location for the translation
 in the y direction
<!-- impl Transform::fn transform -->
Applies all the operations from `other` to `self`.
## `other`
Transform to apply

# Returns

The new matrix
<!-- impl Transform::fn transform_bounds -->
Transforms a `graphene::Rect` using the given matrix `m`. The
result is the bounding box containing the coplanar quad.
## `rect`
a `graphene::Rect`
## `out_rect`
return location for the bounds
 of the transformed rectangle
<!-- impl Transform::fn translate -->
Translates `self` in 2dimensional space by `point`.
## `point`
the point to translate the matrix by

# Returns

The new matrix
<!-- impl Transform::fn translate_3d -->
Translates `self` by `point`.
## `point`
the point to translate the matrix by

# Returns

The new matrix
<!-- impl Transform::fn unref -->
Releases a reference on the given `Transform`.

If the reference was the last, the resources associated to the `self` are
freed.
<!-- impl Transform::fn node_get_child -->
Gets the child node that is getting transformed by the given `node`.
## `node`
a transform `RenderNode`

# Returns

The child that is getting transformed
<!-- impl Transform::fn node_new -->
Creates a `RenderNode` that will transform the given `child`
with the given `transform`.
## `child`
The node to transform
## `transform`
The transform to apply

# Returns

A new `RenderNode`
<!-- impl Transform::fn parse -->
Parses the given `string` into a transform and puts it in
`out_transform`. Strings printed via `Transform::to_string`
can be read in again successfully using this function.

If `string` does not describe a valid transform, `false` is
returned and `None` is put in `out_transform`.
## `string`
the string to parse
## `out_transform`
The location to put the transform in

# Returns

`true` if `string` described a valid transform.
<!-- enum TransformCategory -->
The categories of matrices relevant for GSK and GTK. Note that any
category includes matrices of all later categories. So if you want
to for example check if a matrix is a 2D matrix,
`category >= GSK_TRANSFORM_CATEGORY_2D` is the way to do this.

Also keep in mind that rounding errors may cause matrices to not
conform to their categories. Otherwise, matrix operations done via
mutliplication will not worsen categories. So for the matrix
multiplication `C = A * B`, `category(C) = MIN (category(A), category(B))`.
<!-- enum TransformCategory::variant Unknown -->
The category of the matrix has not been
 determined.
<!-- enum TransformCategory::variant Any -->
Analyzing the matrix concluded that it does
 not fit in any other category.
<!-- enum TransformCategory::variant 3d -->
The matrix is a 3D matrix. This means that
 the w column (the last column) has the values (0, 0, 0, 1).
<!-- enum TransformCategory::variant 2d -->
The matrix is a 2D matrix. This is equivalent
 to `graphene::Matrix::is_2d` returning `true`. In particular, this
 means that Cairo can deal with the matrix.
<!-- enum TransformCategory::variant 2dAffine -->
The matrix is a combination of 2D scale
 and 2D translation operations. In particular, this means that any
 rectangle can be transformed exactly using this matrix.
<!-- enum TransformCategory::variant 2dTranslate -->
The matrix is a 2D translation.
<!-- enum TransformCategory::variant Identity -->
The matrix is the identity matrix.
<!-- struct VulkanRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
