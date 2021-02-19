<!-- file * -->
<!-- enum BlendMode -->
The blend modes available for render nodes.

The implementation of each blend mode is deferred to the
rendering pipeline.

See https://www.w3.org/TR/compositing-1/`blending` for more information
on blending and blend modes.
<!-- enum BlendMode::variant Default -->
The default blend mode, which specifies no blending
<!-- enum BlendMode::variant Multiply -->
The source color is multiplied by the destination
 and replaces the destination
<!-- enum BlendMode::variant Screen -->
Multiplies the complements of the destination and source
 color values, then complements the result.
<!-- enum BlendMode::variant Overlay -->
Multiplies or screens the colors, depending on the
 destination color value. This is the inverse of hard-list
<!-- enum BlendMode::variant Darken -->
Selects the darker of the destination and source colors
<!-- enum BlendMode::variant Lighten -->
Selects the lighter of the destination and source colors
<!-- enum BlendMode::variant ColorDodge -->
Brightens the destination color to reflect the source color
<!-- enum BlendMode::variant ColorBurn -->
Darkens the destination color to reflect the source color
<!-- enum BlendMode::variant HardLight -->
Multiplies or screens the colors, depending on the source color value
<!-- enum BlendMode::variant SoftLight -->
Darkens or lightens the colors, depending on the source color value
<!-- enum BlendMode::variant Difference -->
Subtracts the darker of the two constituent colors from the lighter color
<!-- enum BlendMode::variant Exclusion -->
Produces an effect similar to that of the difference mode but lower in contrast
<!-- enum BlendMode::variant Color -->
Creates a color with the hue and saturation of the source color and the luminosity of the destination color
<!-- enum BlendMode::variant Hue -->
Creates a color with the hue of the source color and the saturation and luminosity of the destination color
<!-- enum BlendMode::variant Saturation -->
Creates a color with the saturation of the source color and the hue and luminosity of the destination color
<!-- enum BlendMode::variant Luminosity -->
Creates a color with the luminosity of the source color and the hue and saturation of the destination color
<!-- struct BlendNode -->
A render node applying a blending function between its two child nodes.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl BlendNode::fn new -->
Creates a `RenderNode` that will use `blend_mode` to blend the `top`
node onto the `bottom` node.
## `bottom`
The bottom node to be drawn
## `top`
The node to be blended onto the `bottom` node
## `blend_mode`
The blend mode to use

# Returns

A new `RenderNode`
<!-- impl BlendNode::fn get_blend_mode -->
Retrieves the blend mode used by `self`.

# Returns

the blend mode
<!-- impl BlendNode::fn get_bottom_child -->
Retrieves the bottom `RenderNode` child of the `self`.

# Returns

the bottom child node
<!-- impl BlendNode::fn get_top_child -->
Retrieves the top `RenderNode` child of the `self`.

# Returns

the top child node
<!-- struct BlurNode -->
A render node applying a blur effect to its single child.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl BlurNode::fn new -->
Creates a render node that blurs the child.
## `child`
the child node to blur
## `radius`
the blur radius

# Returns

a new `RenderNode`
<!-- impl BlurNode::fn get_child -->
Retrieves the child `RenderNode` of the blur `self`.

# Returns

the blurred child node
<!-- impl BlurNode::fn get_radius -->
Retrieves the blur radius of the `self`.

# Returns

the blur radius
<!-- struct BorderNode -->
A render node for a border.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html), [`BorderNodeManualExt`](prelude/trait.BorderNodeManualExt.html)
<!-- impl BorderNode::fn new -->
Creates a `RenderNode` that will stroke a border rectangle inside the
given `outline`. The 4 sides of the border can have different widths and
colors.
## `outline`
a `RoundedRect` describing the outline of the border
## `border_width`
the stroke width of the border on
 the top, right, bottom and left side respectively.
## `border_color`
the color used on the top, right,
 bottom and left side.

# Returns

A new `RenderNode`
<!-- impl BorderNode::fn get_colors -->
Retrieves the colors of the border.

# Returns

an array of 4 `gdk::RGBA` structs
 for the top, right, bottom and left color of the border
<!-- impl BorderNode::fn get_outline -->
Retrieves the outline of the border.

# Returns

the outline of the border
<!-- impl BorderNode::fn get_widths -->
Retrieves the stroke widths of the border.

# Returns

an array of 4 floats
 for the top, right, bottom and left stroke width of the border,
 respectively
<!-- struct BroadwayRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- impl BroadwayRenderer::fn new -->
Creates a new Broadway renderer.

The Broadway renderer is the default renderer for the broadway backend.
It will only work with broadway surfaces, otherwise it will fail the
call to `RendererExt::realize`.

This function is only available when GTK was compiled with Broadway
support.

# Returns

a new Broadway renderer.
<!-- struct CairoNode -->
A render node for a Cairo surface.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl CairoNode::fn new -->
Creates a `RenderNode` that will render a cairo surface
into the area given by `bounds`. You can draw to the cairo
surface using `CairoNode::get_draw_context`
## `bounds`
the rectangle to render to

# Returns

A new `RenderNode`
<!-- impl CairoNode::fn get_draw_context -->
Creates a Cairo context for drawing using the surface associated
to the render node.

If no surface exists yet, a surface will be created optimized for
rendering to `renderer`.

# Returns

a Cairo context used for drawing; use
 `cairo_destroy` when done drawing
<!-- impl CairoNode::fn get_surface -->
Retrieves the Cairo surface used by the render node.

# Returns

a Cairo surface
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
<!-- struct ClipNode -->
A render node applying a rectangular clip to its single child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl ClipNode::fn new -->
Creates a `RenderNode` that will clip the `child` to the area
given by `clip`.
## `child`
The node to draw
## `clip`
The clip to apply

# Returns

A new `RenderNode`
<!-- impl ClipNode::fn get_child -->
Gets the child node that is getting clipped by the given `self`.

# Returns

The child that is getting clipped
<!-- impl ClipNode::fn get_clip -->
Retrieves the clip rectangle for `self`.

# Returns

a clip rectangle
<!-- struct ColorMatrixNode -->
A render node controlling the color matrix of its single child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl ColorMatrixNode::fn new -->
Creates a `RenderNode` that will drawn the `child` with reduced
`color_matrix`.

In particular, the node will transform the operation


```plain
  pixel = color_matrix * pixel + color_offset
```

for every pixel.
## `child`
The node to draw
## `color_matrix`
The matrix to apply
## `color_offset`
Values to add to the color

# Returns

A new `RenderNode`
<!-- impl ColorMatrixNode::fn get_child -->
Gets the child node that is getting its colors modified by the given `self`.

# Returns

The child that is getting its colors modified
<!-- impl ColorMatrixNode::fn get_color_matrix -->
Retrieves the color matrix used by the `self`.

# Returns

a 4x4 color matrix
<!-- impl ColorMatrixNode::fn get_color_offset -->
Retrieves the color offset used by the `self`.

# Returns

a color vector
<!-- struct ColorNode -->
A render node for a solid color.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl ColorNode::fn new -->
Creates a `RenderNode` that will render the color specified by `rgba` into
the area given by `bounds`.
## `rgba`
a `gdk::RGBA` specifying a color
## `bounds`
the rectangle to render the color into

# Returns

A new `RenderNode`
<!-- impl ColorNode::fn get_color -->
Retrieves the color of the given `self`.

# Returns

the color of the node
<!-- struct ConicGradientNode -->
A render node for a conic gradient.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl ConicGradientNode::fn new -->
Creates a `RenderNode` that draws a conic gradient. The conic gradient
starts around `center` in the direction of `rotation`. A rotation of 0 means
that the gradient points up. Color stops are then added clockwise.
## `bounds`
the bounds of the node
## `center`
the center of the gradient
## `rotation`
the rotation of the gradient in degrees
## `color_stops`
a pointer to an array of `ColorStop` defining the gradient
 The offsets of all color steps must be increasing. The first stop's offset must be >= 0 and the last
 stop's offset must be <= 1.
## `n_color_stops`
the number of elements in `color_stops`

# Returns

A new `RenderNode`
<!-- impl ConicGradientNode::fn get_center -->
Retrieves the center pointer for the gradient.

# Returns

the center point for the gradient
<!-- impl ConicGradientNode::fn get_color_stops -->
Retrieves the color stops in the gradient.
## `n_stops`
the number of color stops in the returned array

# Returns

the color stops in the gradient
<!-- impl ConicGradientNode::fn get_n_color_stops -->
Retrieves the number of color stops in the gradient.

# Returns

the number of color stops
<!-- impl ConicGradientNode::fn get_rotation -->
Retrieves the rotation for the gradient in degrees.

# Returns

the rotation for the gradient
<!-- struct ContainerNode -->
A render node that can contain other render nodes.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl ContainerNode::fn new -->
Creates a new `RenderNode` instance for holding the given `children`.
The new node will acquire a reference to each of the children.
## `children`
The children of the node
## `n_children`
Number of children in the `children` array

# Returns

the new `RenderNode`
<!-- impl ContainerNode::fn get_child -->
Gets one of the children of `container`.
## `idx`
the position of the child to get

# Returns

the `idx`'th child of `container`
<!-- impl ContainerNode::fn get_n_children -->
Retrieves the number of direct children of `self`.

# Returns

the number of children of the `RenderNode`
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
<!-- struct CrossFadeNode -->
A render node cross fading between two child nodes.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl CrossFadeNode::fn new -->
Creates a `RenderNode` that will do a cross-fade between `start` and `end`.
## `start`
The start node to be drawn
## `end`
The node to be cross_fadeed onto the `start` node
## `progress`
How far the fade has progressed from start to end. The value will
 be clamped to the range [0 ... 1]

# Returns

A new `RenderNode`
<!-- impl CrossFadeNode::fn get_end_child -->
Retrieves the child `RenderNode` at the end of the cross-fade.

# Returns

a `RenderNode`
<!-- impl CrossFadeNode::fn get_progress -->
Retrieves the progress value of the cross fade.

# Returns

the progress value, between 0 and 1
<!-- impl CrossFadeNode::fn get_start_child -->
Retrieves the child `RenderNode` at the beginning of the cross-fade.

# Returns

a `RenderNode`
<!-- struct DebugNode -->
A render node that emits a debugging message when drawing its
child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl DebugNode::fn new -->
Creates a `RenderNode` that will add debug information about
the given `child`.

Adding this node has no visual effect.
## `child`
The child to add debug info for
## `message`
The debug message

# Returns

A new `RenderNode`
<!-- impl DebugNode::fn get_child -->
Gets the child node that is getting drawn by the given `self`.

# Returns

the child `RenderNode`
<!-- impl DebugNode::fn get_message -->
Gets the debug message that was set on this node

# Returns

The debug message
<!-- struct GLRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- impl GLRenderer::fn new -->
Creates a new `Renderer` using OpenGL. This is the default renderer
used by GTK.

# Returns

a new GL renderer
<!-- struct GLShader -->
An object representing a GL shader program.
<!-- impl GLShader::fn new_from_bytes -->
Creates a `GLShader` that will render pixels using the specified code.
## `sourcecode`
GLSL sourcecode for the shader, as a `glib::Bytes`

# Returns

A new `GLShader`
<!-- impl GLShader::fn new_from_resource -->
Creates a `GLShader` that will render pixels using the specified code.
## `resource_path`
path to a resource that contains the GLSL sourcecode for
 the shader

# Returns

A new `GLShader`
<!-- impl GLShader::fn compile -->
Tries to compile the `self` for the given `renderer`, and reports
`false` with an error if there is a problem. You should use this
function before relying on the shader for rendering and use a
fallback with a simpler shader or without shaders if it fails.

Note that this will modify the rendering state (for example
change the current GL context) and requires the renderer to be
set up. This means that the widget has to be realized. Commonly you
want to call this from the realize signal of a widget, or during
widget snapshot.
## `renderer`
a `Renderer`

# Returns

`true` on success, `false` if an error occurred
<!-- impl GLShader::fn find_uniform_by_name -->
Looks for a uniform by the name `name`, and returns the index
of the uniform, or -1 if it was not found.
## `name`
uniform name

# Returns

The index of the uniform, or -1
<!-- impl GLShader::fn format_args -->
Formats the uniform data as needed for feeding the named uniforms
values into the shader. The argument list is a list of pairs of
names, and values for the types that match the declared uniforms
(i.e. double/int/guint/gboolean for primitive values and
`graphene_vecN_t *` for vecN uniforms).

Any uniforms of the shader that are not included in the argument list
are zero-initialized.

# Returns

A newly allocated block of data which can be
 passed to `GLShaderNode::new`.
<!-- impl GLShader::fn format_args_va -->
Formats the uniform data as needed for feeding the named uniforms
values into the shader. The argument list is a list of pairs of
names, and values for the types that match the declared uniforms
(i.e. double/int/guint/gboolean for primitive values and
`graphene_vecN_t *` for vecN uniforms).

It is an error to pass a uniform name that is not declared by the shader.

Any uniforms of the shader that are not included in the argument list
are zero-initialized.
## `uniforms`
name-Value pairs for the uniforms of `self`, ending
 with a `None` name

# Returns

A newly allocated block of data which can be
 passed to `GLShaderNode::new`.
<!-- impl GLShader::fn get_arg_bool -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of bool type.
## `args`
uniform arguments
## `idx`
index of the uniform

# Returns

The value
<!-- impl GLShader::fn get_arg_float -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of float type.
## `args`
uniform arguments
## `idx`
index of the uniform

# Returns

The value
<!-- impl GLShader::fn get_arg_int -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of int type.
## `args`
uniform arguments
## `idx`
index of the uniform

# Returns

The value
<!-- impl GLShader::fn get_arg_uint -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of uint type.
## `args`
uniform arguments
## `idx`
index of the uniform

# Returns

The value
<!-- impl GLShader::fn get_arg_vec2 -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of vec2 type.
## `args`
uniform arguments
## `idx`
index of the uniform
## `out_value`
location to store the uniform value in
<!-- impl GLShader::fn get_arg_vec3 -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of vec3 type.
## `args`
uniform arguments
## `idx`
index of the uniform
## `out_value`
location to store the uniform value in
<!-- impl GLShader::fn get_arg_vec4 -->
Gets the value of the uniform `idx` in the `args` block.
The uniform must be of vec4 type.
## `args`
uniform arguments
## `idx`
index of the uniform
## `out_value`
location to store set the uniform value in
<!-- impl GLShader::fn get_args_size -->
Get the size of the data block used to specify arguments for this shader.

# Returns

The size of the data block
<!-- impl GLShader::fn get_n_textures -->
Returns the number of textures that the shader requires.

This can be used to check that the a passed shader works
in your usecase. It is determined by looking at the highest
u_textureN value that the shader defines.

# Returns

The number of texture inputs required by `self`
<!-- impl GLShader::fn get_n_uniforms -->
Get the number of declared uniforms for this shader.

# Returns

The number of declared uniforms
<!-- impl GLShader::fn get_resource -->
Gets the resource path for the GLSL sourcecode being used
to render this shader.

# Returns

The resource path for the shader,
 or `None` if none.
<!-- impl GLShader::fn get_source -->
Gets the GLSL sourcecode being used to render this shader.

# Returns

The source code for the shader
<!-- impl GLShader::fn get_uniform_name -->
Get the name of the declared uniform for this shader at index `idx`.
## `idx`
index of the uniform

# Returns

The name of the declared uniform
<!-- impl GLShader::fn get_uniform_offset -->
Get the offset into the data block where data for this uniforms is stored.
## `idx`
index of the uniform

# Returns

The data offset
<!-- impl GLShader::fn get_uniform_type -->
Get the type of the declared uniform for this shader at index `idx`.
## `idx`
index of the uniform

# Returns

The type of the declared uniform
<!-- impl GLShader::fn get_property_resource -->
Resource containing the source code for the shader.

If the shader source is not coming from a resource, this
will be `None`.
<!-- impl GLShader::fn set_property_resource -->
Resource containing the source code for the shader.

If the shader source is not coming from a resource, this
will be `None`.
<!-- struct GLShaderNode -->
A render node using a GL shader when drawing its children nodes.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl GLShaderNode::fn new -->
Creates a `RenderNode` that will render the given `shader` into the
area given by `bounds`. The `args` is a block of data to use for uniform
input, as per types and offsets defined by the `shader`. Normally this
is generated by `GLShader::format_args` or `GskGLShaderArgBuilder`.

See `GLShader` for details about how the shader should be written.

All the children will be rendered into textures (if they aren't already
`GskTextureNodes`, which will be used directly). These textures will be
sent as input to the shader.

If the renderer doesn't support GL shaders, or if there is any problem
when compiling the shader, then the node will draw pink. You should use
`GLShader::compile` to ensure the `shader` will work for the
renderer before using it.
## `shader`
the `GLShader`
## `bounds`
the rectangle to render the shader into
## `args`
Arguments for the uniforms
## `children`
array of child nodes, these will
 be rendered to textures and used as input.
## `n_children`
Length of `children` (currenly the GL backend supports
 up to 4 children)

# Returns

A new `RenderNode`
<!-- impl GLShaderNode::fn get_args -->
Gets args for the node.

# Returns

A `glib::Bytes` with the uniform arguments
<!-- impl GLShaderNode::fn get_child -->
Gets one of the children.
## `idx`
the position of the child to get

# Returns

the `idx`'th child of `self`
<!-- impl GLShaderNode::fn get_n_children -->
Returns the number of children

# Returns

The number of children
<!-- impl GLShaderNode::fn get_shader -->
Gets shader code for the node.

# Returns

the `GLShader` shader
<!-- enum GLUniformType -->
This defines the types of the uniforms that `GskGLShaders`
declare. It defines both what the type is called in the GLSL shader
code, and what the corresponding C type is on the Gtk side.
<!-- enum GLUniformType::variant None -->
No type, used for uninitialized or unspecified values.
<!-- enum GLUniformType::variant Float -->
A float uniform
<!-- enum GLUniformType::variant Int -->
A GLSL int / gint32 uniform
<!-- enum GLUniformType::variant Uint -->
A GLSL uint / guint32 uniform
<!-- enum GLUniformType::variant Bool -->
A GLSL bool / gboolean uniform
<!-- enum GLUniformType::variant Vec2 -->
A GLSL vec2 / graphene_vec2_t uniform
<!-- enum GLUniformType::variant Vec3 -->
A GLSL vec3 / graphene_vec3_t uniform
<!-- enum GLUniformType::variant Vec4 -->
A GLSL vec4 / graphene_vec4_t uniform
<!-- struct InsetShadowNode -->
A render node for an inset shadow.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl InsetShadowNode::fn new -->
Creates a `RenderNode` that will render an inset shadow
into the box given by `outline`.
## `outline`
outline of the region containing the shadow
## `color`
color of the shadow
## `dx`
horizontal offset of shadow
## `dy`
vertical offset of shadow
## `spread`
how far the shadow spreads towards the inside
## `blur_radius`
how much blur to apply to the shadow

# Returns

A new `RenderNode`
<!-- impl InsetShadowNode::fn get_blur_radius -->
Retrieves the blur radius to apply to the shadow.

# Returns

the blur radius, in pixels
<!-- impl InsetShadowNode::fn get_color -->
Retrieves the color of the inset shadow.

# Returns

the color of the shadow
<!-- impl InsetShadowNode::fn get_dx -->
Retrieves the horizontal offset of the inset shadow.

# Returns

an offset, in pixels
<!-- impl InsetShadowNode::fn get_dy -->
Retrieves the vertical offset of the inset shadow.

# Returns

an offset, in pixels
<!-- impl InsetShadowNode::fn get_outline -->
Retrieves the outline rectangle of the inset shadow.

# Returns

a rounded rectangle
<!-- impl InsetShadowNode::fn get_spread -->
Retrieves how much the shadow spreads inwards.

# Returns

the size of the shadow, in pixels
<!-- struct LinearGradientNode -->
A render node for a linear gradient.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl LinearGradientNode::fn new -->
Creates a `RenderNode` that will create a linear gradient from the given
points and color stops, and render that into the area given by `bounds`.
## `bounds`
the rectangle to render the linear gradient into
## `start`
the point at which the linear gradient will begin
## `end`
the point at which the linear gradient will finish
## `color_stops`
a pointer to an array of `ColorStop` defining the gradient
 The offsets of all color steps must be increasing. The first stop's offset must be >= 0 and the last
 stop's offset must be <= 1.
## `n_color_stops`
the number of elements in `color_stops`

# Returns

A new `RenderNode`
<!-- impl LinearGradientNode::fn get_color_stops -->
Retrieves the color stops in the gradient.
## `n_stops`
the number of color stops in the returned array

# Returns

the color stops in the gradient
<!-- impl LinearGradientNode::fn get_end -->
Retrieves the final point of the linear gradient.

# Returns

the final point
<!-- impl LinearGradientNode::fn get_n_color_stops -->
Retrieves the number of color stops in the gradient.

# Returns

the number of color stops
<!-- impl LinearGradientNode::fn get_start -->
Retrieves the initial point of the linear gradient.

# Returns

the initial point
<!-- struct OpacityNode -->
A render node controlling the opacity of its single child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl OpacityNode::fn new -->
Creates a `RenderNode` that will drawn the `child` with reduced
`opacity`.
## `child`
The node to draw
## `opacity`
The opacity to apply

# Returns

A new `RenderNode`
<!-- impl OpacityNode::fn get_child -->
Gets the child node that is getting opacityed by the given `self`.

# Returns

The child that is getting opacityed
<!-- impl OpacityNode::fn get_opacity -->
Gets the transparency factor for an opacity node.

# Returns

the opacity factor
<!-- struct OutsetShadowNode -->
A render node for an outset shadow.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl OutsetShadowNode::fn new -->
Creates a `RenderNode` that will render an outset shadow
around the box given by `outline`.
## `outline`
outline of the region surrounded by shadow
## `color`
color of the shadow
## `dx`
horizontal offset of shadow
## `dy`
vertical offset of shadow
## `spread`
how far the shadow spreads towards the inside
## `blur_radius`
how much blur to apply to the shadow

# Returns

A new `RenderNode`
<!-- impl OutsetShadowNode::fn get_blur_radius -->
Retrieves the blur radius of the shadow.

# Returns

the blur radius, in pixels
<!-- impl OutsetShadowNode::fn get_color -->
Retrieves the color of the outset shadow.

# Returns

a color
<!-- impl OutsetShadowNode::fn get_dx -->
Retrieves the horizontal offset of the outset shadow.

# Returns

an offset, in pixels
<!-- impl OutsetShadowNode::fn get_dy -->
Retrieves the vertical offset of the outset shadow.

# Returns

an offset, in pixels
<!-- impl OutsetShadowNode::fn get_outline -->
Retrieves the outline rectangle of the outset shadow.

# Returns

a rounded rectangle
<!-- impl OutsetShadowNode::fn get_spread -->
Retrieves how much the shadow spreads outwards.

# Returns

the size of the shadow, in pixels
<!-- struct RadialGradientNode -->
A render node for a radial gradient.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl RadialGradientNode::fn new -->
Creates a `RenderNode` that draws a radial gradient. The radial gradient
starts around `center`. The size of the gradient is dictated by `hradius`
in horizontal orientation and by `vradius` in vertial orientation.
## `bounds`
the bounds of the node
## `center`
the center of the gradient
## `hradius`
the horizontal radius
## `vradius`
the vertical radius
## `start`
a percentage >= 0 that defines the start of the gradient around `center`
## `end`
a percentage >= 0 that defines the end of the gradient around `center`
## `color_stops`
a pointer to an array of `ColorStop` defining the gradient
 The offsets of all color steps must be increasing. The first stop's offset must be >= 0 and the last
 stop's offset must be <= 1.
## `n_color_stops`
the number of elements in `color_stops`

# Returns

A new `RenderNode`
<!-- impl RadialGradientNode::fn get_center -->
Retrieves the center pointer for the gradient.

# Returns

the center point for the gradient
<!-- impl RadialGradientNode::fn get_color_stops -->
Retrieves the color stops in the gradient.
## `n_stops`
the number of color stops in the returned array

# Returns

the color stops in the gradient
<!-- impl RadialGradientNode::fn get_end -->
Retrieves the end value for the gradient.

# Returns

the end value for the gradient
<!-- impl RadialGradientNode::fn get_hradius -->
Retrieves the horizonal radius for the gradient.

# Returns

the horizontal radius for the gradient
<!-- impl RadialGradientNode::fn get_n_color_stops -->
Retrieves the number of color stops in the gradient.

# Returns

the number of color stops
<!-- impl RadialGradientNode::fn get_start -->
Retrieves the start value for the gradient.

# Returns

the start value for the gradient
<!-- impl RadialGradientNode::fn get_vradius -->
Retrieves the vertical radius for the gradient.

# Returns

the vertical radius for the gradient
<!-- struct RenderNode -->
A node in the render tree.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- trait RenderNodeExt -->
Trait containing all `RenderNode` methods.

# Implementors

[`BlendNode`](struct.BlendNode.html), [`BlurNode`](struct.BlurNode.html), [`BorderNode`](struct.BorderNode.html), [`CairoNode`](struct.CairoNode.html), [`ClipNode`](struct.ClipNode.html), [`ColorMatrixNode`](struct.ColorMatrixNode.html), [`ColorNode`](struct.ColorNode.html), [`ConicGradientNode`](struct.ConicGradientNode.html), [`ContainerNode`](struct.ContainerNode.html), [`CrossFadeNode`](struct.CrossFadeNode.html), [`DebugNode`](struct.DebugNode.html), [`GLShaderNode`](struct.GLShaderNode.html), [`InsetShadowNode`](struct.InsetShadowNode.html), [`LinearGradientNode`](struct.LinearGradientNode.html), [`OpacityNode`](struct.OpacityNode.html), [`OutsetShadowNode`](struct.OutsetShadowNode.html), [`RadialGradientNode`](struct.RadialGradientNode.html), [`RenderNode`](struct.RenderNode.html), [`RepeatNode`](struct.RepeatNode.html), [`RepeatingLinearGradientNode`](struct.RepeatingLinearGradientNode.html), [`RepeatingRadialGradientNode`](struct.RepeatingRadialGradientNode.html), [`RoundedClipNode`](struct.RoundedClipNode.html), [`ShadowNode`](struct.ShadowNode.html), [`TextNode`](struct.TextNode.html), [`TextureNode`](struct.TextureNode.html), [`TransformNode`](struct.TransformNode.html)
<!-- impl RenderNode::fn deserialize -->
Loads data previously created via `RenderNodeExt::serialize`. For a
discussion of the supported format, see that function.
## `bytes`
the bytes containing the data
## `error_func`
Callback on parsing errors or `None`
## `user_data`
user_data for `error_func`

# Returns

a new `RenderNode` or `None` on
 error.
<!-- trait RenderNodeExt::fn draw -->
Draw the contents of `self` to the given cairo context.

Typically, you'll use this function to implement fallback rendering
of `GskRenderNodes` on an intermediate Cairo context, instead of using
the drawing context associated to a `gdk::Surface`'s rendering buffer.

For advanced nodes that cannot be supported using Cairo, in particular
for nodes doing 3D operations, this function may fail.
## `cr`
cairo context to draw to
<!-- trait RenderNodeExt::fn get_bounds -->
Retrieves the boundaries of the `self`. The node will not draw outside
of its boundaries.
## `bounds`
return location for the boundaries
<!-- trait RenderNodeExt::fn get_node_type -->
Returns the type of the `self`.

# Returns

the type of the `RenderNode`
<!-- trait RenderNodeExt::fn ref -->
Acquires a reference on the given `RenderNode`.

# Returns

the `RenderNode` with an additional reference
<!-- trait RenderNodeExt::fn serialize -->
Serializes the `self` for later deserialization via
`RenderNode::deserialize`. No guarantees are made about the format
used other than that the same version of GTK will be able to deserialize
the result of a call to `RenderNodeExt::serialize` and
`RenderNode::deserialize` will correctly reject files it cannot open
that were created with previous versions of GTK.

The intended use of this functions is testing, benchmarking and debugging.
The format is not meant as a permanent storage format.

# Returns

a `glib::Bytes` representing the node.
<!-- trait RenderNodeExt::fn unref -->
Releases a reference on the given `RenderNode`.

If the reference was the last, the resources associated to the `self` are
freed.
<!-- trait RenderNodeExt::fn write_to_file -->
This function is equivalent to calling `RenderNodeExt::serialize`
followed by `g_file_set_contents`. See those two functions for details
on the arguments.

It is mostly intended for use inside a debugger to quickly dump a render
node to a file for later inspection.
## `filename`
the file to save it to.

# Returns

`true` if saving was successful
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
<!-- enum RenderNodeType::variant RadialGradientNode -->
A node drawing a radial gradient
<!-- enum RenderNodeType::variant RepeatingRadialGradientNode -->
A node drawing a repeating radial gradient
<!-- enum RenderNodeType::variant ConicGradientNode -->
A node drawing a conic gradient
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
<!-- enum RenderNodeType::variant DebugNode -->
Debug information that does not affect the rendering
<!-- enum RenderNodeType::variant GlShaderNode -->
A node that uses OpenGL fragment shaders to render
<!-- struct Renderer -->
Base type for the object managing the rendering pipeline
for a `gdk::Surface`.

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`RendererExt`](trait.RendererExt.html)
<!-- trait RendererExt -->
Trait containing all `Renderer` methods.

# Implementors

[`BroadwayRenderer`](struct.BroadwayRenderer.html), [`CairoRenderer`](struct.CairoRenderer.html), [`GLRenderer`](struct.GLRenderer.html), [`Renderer`](struct.Renderer.html), [`VulkanRenderer`](struct.VulkanRenderer.html)
<!-- impl Renderer::fn new_for_surface -->
Creates an appropriate `Renderer` instance for the given `surface`.

If the `GSK_RENDERER` environment variable is set, GSK will
try that renderer first, before trying the backend-specific
default. The ultimate fallback is the cairo renderer.

The renderer will be realized before it is returned.
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
<!-- struct RepeatNode -->
A render node repeating its single child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl RepeatNode::fn new -->
Creates a `RenderNode` that will repeat the drawing of `child` across
the given `bounds`.
## `bounds`
The bounds of the area to be painted
## `child`
The child to repeat
## `child_bounds`
The area of the child to repeat or `None` to
 use the child's bounds

# Returns

A new `RenderNode`
<!-- impl RepeatNode::fn get_child -->
Retrieves the child of `self`.

# Returns

a `RenderNode`
<!-- impl RepeatNode::fn get_child_bounds -->
Retrieves the bounding rectangle of the child of `self`.

# Returns

a bounding rectangle
<!-- struct RepeatingLinearGradientNode -->
A render node for a repeating linear gradient.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl RepeatingLinearGradientNode::fn new -->
Creates a `RenderNode` that will create a repeating linear gradient
from the given points and color stops, and render that into the area
given by `bounds`.
## `bounds`
the rectangle to render the linear gradient into
## `start`
the point at which the linear gradient will begin
## `end`
the point at which the linear gradient will finish
## `color_stops`
a pointer to an array of `ColorStop` defining the gradient
 The offsets of all color steps must be increasing. The first stop's offset must be >= 0 and the last
 stop's offset must be <= 1.
## `n_color_stops`
the number of elements in `color_stops`

# Returns

A new `RenderNode`
<!-- struct RepeatingRadialGradientNode -->
A render node for a repeating radial gradient.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl RepeatingRadialGradientNode::fn new -->
Creates a `RenderNode` that draws a repeating radial gradient. The radial gradient
starts around `center`. The size of the gradient is dictated by `hradius`
in horizontal orientation and by `vradius` in vertial orientation.
## `bounds`
the bounds of the node
## `center`
the center of the gradient
## `hradius`
the horizontal radius
## `vradius`
the vertical radius
## `start`
a percentage >= 0 that defines the start of the gradient around `center`
## `end`
a percentage >= 0 that defines the end of the gradient around `center`
## `color_stops`
a pointer to an array of `ColorStop` defining the gradient
 The offsets of all color steps must be increasing. The first stop's offset must be >= 0 and the last
 stop's offset must be <= 1.
## `n_color_stops`
the number of elements in `color_stops`

# Returns

A new `RenderNode`
<!-- struct RoundedClipNode -->
A render node applying a rounded rectangle clip to its single child.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl RoundedClipNode::fn new -->
Creates a `RenderNode` that will clip the `child` to the area
given by `clip`.
## `child`
The node to draw
## `clip`
The clip to apply

# Returns

A new `RenderNode`
<!-- impl RoundedClipNode::fn get_child -->
Gets the child node that is getting clipped by the given `self`.

# Returns

The child that is getting clipped
<!-- impl RoundedClipNode::fn get_clip -->
Retrievs the rounded rectangle used to clip the contents of the `self`.

# Returns

a rounded rectangle
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
<!-- struct ShaderArgsBuilder -->
An object to build the uniforms data for a `GLShader`.
<!-- impl ShaderArgsBuilder::fn new -->
Allocates a builder that can be used to construct a new uniform data
chunk.
## `shader`
a `GLShader`
## `initial_values`
optional `Bytes` with initial values

# Returns

The newly allocated builder, free with
 `ShaderArgsBuilder::unref`
<!-- impl ShaderArgsBuilder::fn free_to_args -->
Creates a new `glib::Bytes` args from the current state of the
given `self`, and frees the `self` instance. Any uniforms
of the shader that have not been explicitly set on the `self`
are zero-initialized.

# Returns

the newly created `glib::Bytes`
 with all the args added to `self`
<!-- impl ShaderArgsBuilder::fn ref -->
Increases the reference count of a `ShaderArgsBuilder` by one.

# Returns

the passed in `ShaderArgsBuilder`
<!-- impl ShaderArgsBuilder::fn set_bool -->
Sets the value of the uniform `idx`.
The uniform must be of bool type.
## `idx`
index of the uniform
## `value`
value to set the uniform to
<!-- impl ShaderArgsBuilder::fn set_float -->
Sets the value of the uniform `idx`.
The uniform must be of float type.
## `idx`
index of the uniform
## `value`
value to set the uniform to
<!-- impl ShaderArgsBuilder::fn set_int -->
Sets the value of the uniform `idx`.
The uniform must be of int type.
## `idx`
index of the uniform
## `value`
value to set the uniform to
<!-- impl ShaderArgsBuilder::fn set_uint -->
Sets the value of the uniform `idx`.
The uniform must be of uint type.
## `idx`
index of the uniform
## `value`
value to set the uniform to
<!-- impl ShaderArgsBuilder::fn set_vec2 -->
Sets the value of the uniform `idx`.
The uniform must be of vec2 type.
## `idx`
index of the uniform
## `value`
value to set the uniform too
<!-- impl ShaderArgsBuilder::fn set_vec3 -->
Sets the value of the uniform `idx`.
The uniform must be of vec3 type.
## `idx`
index of the uniform
## `value`
value to set the uniform too
<!-- impl ShaderArgsBuilder::fn set_vec4 -->
Sets the value of the uniform `idx`.
The uniform must be of vec4 type.
## `idx`
index of the uniform
## `value`
value to set the uniform too
<!-- impl ShaderArgsBuilder::fn to_args -->
Creates a new `glib::Bytes` args from the current state of the
given `self`. Any uniforms of the shader that have not
been explicitly set on the `self` are zero-initialized.

The given `ShaderArgsBuilder` is reset once this function returns;
you cannot call this function multiple times on the same `self` instance.

This function is intended primarily for bindings. C code should use
`ShaderArgsBuilder::free_to_args`.

# Returns

the newly allocated builder, free with
 `gsk_shader_args_builder_free`
<!-- impl ShaderArgsBuilder::fn unref -->
Decreases the reference count of a `GskShaderArgBuilder` by one.
If the resulting reference count is zero, frees the builder.
<!-- struct ShadowNode -->
A render node drawing one or more shadows behind its single child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl ShadowNode::fn new -->
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
<!-- impl ShadowNode::fn get_child -->
Retrieves the child `RenderNode` of the shadow `self`.

# Returns

the child render node
<!-- impl ShadowNode::fn get_n_shadows -->
Retrieves the number of shadows in the `self`.

# Returns

the number of shadows.
<!-- impl ShadowNode::fn get_shadow -->
Retrieves the shadow data at the given index `i`.
## `i`
the given index

# Returns

the shadow data
<!-- struct TextNode -->
A render node drawing a set of glyphs.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl TextNode::fn new -->
Creates a render node that renders the given glyphs,
Note that `color` may not be used if the font contains
color glyphs.
## `font`
the `pango::Font` containing the glyphs
## `glyphs`
the `pango::GlyphString` to render
## `color`
the foreground color to render with
## `offset`
offset of the baseline

# Returns

a new `RenderNode`
<!-- impl TextNode::fn get_color -->
Retrieves the color used by the text `self`.

# Returns

the text color
<!-- impl TextNode::fn get_font -->
Returns the font used by the text `self`.

# Returns

the font
<!-- impl TextNode::fn get_glyphs -->
Retrieves the glyph information in the `self`.
## `n_glyphs`
the number of glyphs returned

# Returns

the glyph information
<!-- impl TextNode::fn get_num_glyphs -->
Retrieves the number of glyphs in the text node.

# Returns

the number of glyphs
<!-- impl TextNode::fn get_offset -->
Retrieves the offset applied to the text.

# Returns

a point with the horizontal and vertical offsets
<!-- impl TextNode::fn has_color_glyphs -->
Checks whether the text `self` has color glyphs.

# Returns

`true` if the text node has color glyphs
<!-- struct TextureNode -->
A render node for a `gdk::Texture`.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl TextureNode::fn new -->
Creates a `RenderNode` that will render the given
`texture` into the area given by `bounds`.
## `texture`
the `gdk::Texture`
## `bounds`
the rectangle to render the texture into

# Returns

A new `RenderNode`
<!-- impl TextureNode::fn get_texture -->
Retrieves the `gdk::Texture` used when creating this `RenderNode`.

# Returns

the `gdk::Texture`
<!-- struct Transform -->
The `GskTransform` structure contains only private data.
<!-- impl Transform::fn equal -->
Checks two transforms for equality.
## `second`
the second transform

# Returns

`true` if the two transforms perform the same operation.
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

The new transform
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

The new transform
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

The new transform
<!-- impl Transform::fn rotate_3d -->
Rotates `self` `angle` degrees around `axis`.

For a rotation in 2D space, use `Transform::rotate`.
## `angle`
the rotation angle, in degrees (clockwise)
## `axis`
The rotation axis

# Returns

The new transform
<!-- impl Transform::fn scale -->
Scales `self` in 2-dimensional space by the given factors.
Use `Transform::scale_3d` to scale in all 3 dimensions.
## `factor_x`
scaling factor on the X axis
## `factor_y`
scaling factor on the Y axis

# Returns

The new transform
<!-- impl Transform::fn scale_3d -->
Scales `self` by the given factors.
## `factor_x`
scaling factor on the X axis
## `factor_y`
scaling factor on the Y axis
## `factor_z`
scaling factor on the Z axis

# Returns

The new transform
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
  | dx dy |   | tx ty  1 |
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

The new transform
<!-- impl Transform::fn transform_bounds -->
Transforms a `graphene::Rect` using the given transform `self`.
The result is the bounding box containing the coplanar quad.
## `rect`
a `graphene::Rect`
## `out_rect`
return location for the bounds
 of the transformed rectangle
<!-- impl Transform::fn transform_point -->
Transforms a `graphene::Point` using the given transform `self`.
## `point`
a `graphene::Point`
## `out_point`
return location for
 the transformed point
<!-- impl Transform::fn translate -->
Translates `self` in 2dimensional space by `point`.
## `point`
the point to translate the transform by

# Returns

The new transform
<!-- impl Transform::fn translate_3d -->
Translates `self` by `point`.
## `point`
the point to translate the transform by

# Returns

The new transform
<!-- impl Transform::fn unref -->
Releases a reference on the given `Transform`.

If the reference was the last, the resources associated to the `self` are
freed.
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
<!-- struct TransformNode -->
A render node applying a `Transform` to its single child node.

# Implements

[`RenderNodeExt`](trait.RenderNodeExt.html)
<!-- impl TransformNode::fn new -->
Creates a `RenderNode` that will transform the given `child`
with the given `transform`.
## `child`
The node to transform
## `transform`
The transform to apply

# Returns

A new `RenderNode`
<!-- impl TransformNode::fn get_child -->
Gets the child node that is getting transformed by the given `self`.

# Returns

The child that is getting transformed
<!-- impl TransformNode::fn get_transform -->
Retrieves the `Transform` used by the `self`.

# Returns

a `Transform`
<!-- struct VulkanRenderer -->


# Implements

[`RendererExt`](trait.RendererExt.html)
