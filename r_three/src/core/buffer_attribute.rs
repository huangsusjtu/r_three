pub struct BufferAttribute<T> {
    data: Vec<T>,
    normalized: bool,
}

impl<T> BufferAttribute<T> {
    pub fn new(data: Vec<T>) -> BufferAttribute<T> {
        BufferAttribute {
            data,
            normalized: false,
        }
    }

    //
    pub fn apply_matrix3(&mut self, m: glam::Mat3) {
        // todo
    }
    pub fn apply_matrix4(&mut self, m: glam::Mat4) {
        // todo
    }
    pub fn apply_normal_matrix4(&mut self, m: glam::Mat4) {
        // todo
    }
}

pub type Float16BufferAttribute = BufferAttribute<u16>;
pub type Float32BufferAttribute = BufferAttribute<f32>;

pub type Int8BufferAttribute = BufferAttribute<i8>;
pub type Int16BufferAttribute = BufferAttribute<i16>;
pub type Int32BufferAttribute = BufferAttribute<i32>;
pub type Int64BufferAttribute = BufferAttribute<i64>;

pub type Uint8BufferAttribute = BufferAttribute<u8>;
pub type Uint16BufferAttribute = BufferAttribute<u16>;
pub type Uint32BufferAttribute = BufferAttribute<u32>;
pub type Uint64BufferAttribute = BufferAttribute<u64>;
