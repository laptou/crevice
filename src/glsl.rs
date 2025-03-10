//! Defines traits and types for generating GLSL code from Rust definitions.

pub use crevice_derive::GlslStruct;

/// Trait for types that have a GLSL equivalent. Useful for generating GLSL code
/// from Rust structs.
pub unsafe trait Glsl {
    /// The name of this type in GLSL, like `vec2` or `mat4`.
    const NAME: &'static str;
}

/// A field contained within a GLSL struct definition.
pub struct GlslField {
    /// The type of the field, like `vec2` or `mat3`.
    pub ty: &'static str,

    /// The field's name. This must be a valid GLSL identifier.
    pub name: &'static str,
}

/// Trait for types that can be represented as a struct in GLSL.
///
/// This trait should not generally be implemented by hand, but can be derived.
pub unsafe trait GlslStruct: Glsl {
    /// The fields contained in this struct.
    const FIELDS: &'static [GlslField];

    /// Generates GLSL code that represents this struct and its fields.
    fn glsl_definition() -> String {
        let mut output = String::new();
        output.push_str("struct ");
        output.push_str(Self::NAME);
        output.push_str(" {\n");

        for field in Self::FIELDS {
            output.push('\t');
            output.push_str(field.ty);
            output.push(' ');
            output.push_str(field.name);
            output.push_str(";\n");
        }

        output.push_str("};");
        output
    }
}

unsafe impl Glsl for f32 {
    const NAME: &'static str = "float";
}

unsafe impl Glsl for f64 {
    const NAME: &'static str = "double";
}

unsafe impl Glsl for i32 {
    const NAME: &'static str = "int";
}

unsafe impl Glsl for u32 {
    const NAME: &'static str = "uint";
}
