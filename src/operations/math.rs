pub fn add(a: i32, b: i32) -> f32 {
    (a + b) as f32
}

pub fn sub(a: i32, b: i32) -> f32 {
    (a - b) as f32
}

pub fn mul(a: i32, b: i32) -> f32 {
    (a * b) as f32
}

pub fn div(a: i32, b: i32) -> f32 {
    (a as f32) / (b as f32)
}

pub fn square(a: i32) -> f32 {
    (a * a) as f32
}

pub fn sqrt(a: i32) -> f32 {
    (a as f32).sqrt()
}