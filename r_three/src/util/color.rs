// use glam::{vec3, vec4, Vec3, Vec4};
//
// /// Represents a color composed of a red, green and blue component in the sRGB color space.
// /// In addition, the alpha value determines the how transparent the color is (0 is fully transparent and 255 is fully opaque).
// pub type Color = Srgba;
//
// /// Represents a color composed of a red, green and blue component in the sRGB color space.
// /// In addition, the alpha value determines the how transparent the color is (0 is fully transparent and 255 is fully opaque).
// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct Srgba {
//     /// Red component
//     pub r: u8,
//     /// Green component
//     pub g: u8,
//     /// Blue component
//     pub b: u8,
//     /// Alpha component
//     pub a: u8,
// }
//
// impl Srgba {
//     ///
//     /// Creates a new sRGBA color with the given values.
//     ///
//     pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
//         Self { r, g, b, a }
//     }
//
//     ///
//     /// Creates a new sRGB color with the given red, green and blue values and an alpha value of 255.
//     ///
//     pub const fn new_opaque(r: u8, g: u8, b: u8) -> Self {
//         Self { r, g, b, a: 255 }
//     }
//
//     ///
//     /// Returns the color in linear sRGB color space.
//     ///
//     pub fn to_linear_srgb(&self) -> Vec4 {
//         let convert = |c: u8| {
//             let c = c as f32 / 255.0;
//             if c < 0.04045 {
//                 c / 12.92
//             } else {
//                 ((c + 0.055) / 1.055).powf(2.4)
//             }
//         };
//         vec4(
//             convert(self.r),
//             convert(self.g),
//             convert(self.b),
//             self.a as f32 / 255.0,
//         )
//     }
//
//     /// Opaque red
//     pub const RED: Self = Self::new_opaque(255, 0, 0);
//     /// Opaque green
//     pub const GREEN: Self = Self::new_opaque(0, 255, 0);
//     /// Opaque blue
//     pub const BLUE: Self = Self::new_opaque(0, 0, 255);
//     /// Opaque white
//     pub const WHITE: Self = Self::new_opaque(255, 255, 255);
//     /// Opaque black
//     pub const BLACK: Self = Self::new_opaque(0, 0, 0);
// }
//
// impl From<[f32; 3]> for Srgba {
//     fn from(value: [f32; 3]) -> Self {
//         Self {
//             r: (value[0] * 255.0) as u8,
//             g: (value[1] * 255.0) as u8,
//             b: (value[2] * 255.0) as u8,
//             a: 255,
//         }
//     }
// }
//
// impl From<[f32; 4]> for Srgba {
//     fn from(value: [f32; 4]) -> Self {
//         Self {
//             r: (value[0] * 255.0) as u8,
//             g: (value[1] * 255.0) as u8,
//             b: (value[2] * 255.0) as u8,
//             a: (value[3] * 255.0) as u8,
//         }
//     }
// }
// impl From<Vec3> for Srgba {
//     fn from(value: Vec3) -> Self {
//         Self {
//             r: (value.x * 255.0) as u8,
//             g: (value.y * 255.0) as u8,
//             b: (value.z * 255.0) as u8,
//             a: 255,
//         }
//     }
// }
//
// impl From<Vec4> for Srgba {
//     fn from(value: Vec4) -> Self {
//         Self {
//             r: (value.x * 255.0) as u8,
//             g: (value.y * 255.0) as u8,
//             b: (value.z * 255.0) as u8,
//             a: (value.w * 255.0) as u8,
//         }
//     }
// }
//
// impl From<[u8; 3]> for Srgba {
//     fn from(value: [u8; 3]) -> Self {
//         Self {
//             r: value[0],
//             g: value[1],
//             b: value[2],
//             a: 255,
//         }
//     }
// }
//
// impl From<[u8; 4]> for Srgba {
//     fn from(value: [u8; 4]) -> Self {
//         Self {
//             r: value[0],
//             g: value[1],
//             b: value[2],
//             a: value[3],
//         }
//     }
// }
//
// impl From<Srgba> for [f32; 3] {
//     fn from(value: Srgba) -> Self {
//         [
//             value.r as f32 / 255.0,
//             value.g as f32 / 255.0,
//             value.b as f32 / 255.0,
//         ]
//     }
// }
//
// impl From<Srgba> for [f32; 4] {
//     fn from(value: Srgba) -> Self {
//         [
//             value.r as f32 / 255.0,
//             value.g as f32 / 255.0,
//             value.b as f32 / 255.0,
//             value.a as f32 / 255.0,
//         ]
//     }
// }
//
// impl From<Srgba> for Vec3 {
//     fn from(value: Srgba) -> Self {
//         vec3(
//             value.r as f32 / 255.0,
//             value.g as f32 / 255.0,
//             value.b as f32 / 255.0,
//         )
//     }
// }
//
// impl From<Srgba> for Vec4 {
//     fn from(value: Srgba) -> Self {
//         vec4(
//             value.r as f32 / 255.0,
//             value.g as f32 / 255.0,
//             value.b as f32 / 255.0,
//             value.a as f32 / 255.0,
//         )
//     }
// }
//
// impl From<Srgba> for [u8; 3] {
//     fn from(value: Srgba) -> Self {
//         [value.r, value.g, value.b]
//     }
// }
//
// impl From<Srgba> for [u8; 4] {
//     fn from(value: Srgba) -> Self {
//         [value.r, value.g, value.b, value.a]
//     }
// }
//
// impl From<Srgba> for [f64; 4] {
//     fn from(value: Srgba) -> Self {
//         [
//             value.r as f64 / 255.0,
//             value.g as f64 / 255.0,
//             value.b as f64 / 255.0,
//             value.a as f64 / 255.0,
//         ]
//     }
// }
//
// impl Default for Srgba {
//     fn default() -> Self {
//         Self::WHITE
//     }
// }

use glam::{Vec3, Vec4};

/// RGBA double precision color.
///
/// This is not to be used as a generic color type, only for specific wgpu interfaces.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Color {
    /// Red component of the color
    pub r: f64,
    /// Green component of the color
    pub g: f64,
    /// Blue component of the color
    pub b: f64,
    /// Alpha component of the color
    pub a: f64,
}

#[allow(missing_docs)]
impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Self {
            r: value[0] as f64,
            g: value[1] as f64,
            b: value[2] as f64,
            a: 1.0,
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self {
            r: value[0] as f64,
            g: value[1] as f64,
            b: value[2] as f64,
            a: value[3] as f64,
        }
    }
}
impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Self {
            r: value.x as f64,
            g: value.y as f64,
            b: value.z as f64,
            a: 1.0,
        }
    }
}

impl From<Vec4> for Color {
    fn from(value: Vec4) -> Self {
        Self {
            r: value.x as f64,
            g: value.y as f64,
            b: value.z as f64,
            a: value.w as f64,
        }
    }
}


impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.r as f32, self.g as f32, self.b as f32]
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r as f32, self.g as f32, self.b as f32, self.a as f32]
    }
}
impl Into<Vec3> for Color {
    fn into(self) -> Vec3 {
        Vec3::new(self.r as f32, self.g as f32, self.b as f32)
    }
}

impl Into<Vec4> for Color {
    fn into(self) -> Vec4 {
        Vec4::new(self.r as f32, self.g as f32, self.b as f32, self.a as f32)
    }
}