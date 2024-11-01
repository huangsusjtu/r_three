use crate::color::Color;
use crate::materials::Material;

pub struct LineBasicMaterial {
    pub color: Color,
    pub line_width: f32,

    /// 'butt'：线条端点是平的，直接结束，没有额外的形状。
    /// 'round'：线条端点是圆形的，会有一个半圆的形状来结束线条。
    /// 'square'：线条端点是方形的，会有一个矩形的形状来结束线条，这个矩形的长度是线条宽度的一半。
    pub linecap: &'static str,

    ///'round'：在连接点处使用圆形过渡，使线条连接更加平滑。
    /// 'bevel'：在连接点处使用斜角过渡，会有一个平面的斜角形状。
    /// 'miter'：在连接点处使用斜接过渡，会根据线条的角度形成一个尖锐的连接。
    pub linejoin: &'static str,

    /// 用于设置线条的透明度。取值范围是从 0（完全透明）到 1（完全不透明）
    pub alpha: f32,
}

impl Material for LineBasicMaterial {}
