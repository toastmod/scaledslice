# scaledslice
Safely compress or stretch the image of an array.

# What?
This crate provides structs that wrap around `&[f32]`, `&[f64]`, or `&[Complex<Float>]`.\
Slices are not copied, interpolation is applied as you index the slice.\
Undefined math results like `NaN` or `inf` are worked around internally.

# To-Do
- `.len()`
- `impl Index`
- Examples
- Tests
