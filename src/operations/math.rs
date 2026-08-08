pub fn add(a: i32, b: i32) -> Result<f32, String> {
    Ok((a + b) as f32)
}

pub fn sub(a: i32, b: i32) -> Result<f32, String> {
    Ok((a - b) as f32)
}

pub fn mul(a: i32, b: i32) -> Result<f32, String> {
    Ok((a * b) as f32)
}

pub fn div(a: i32, b: i32) -> Result<f32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok((a as f32) / (b as f32))
    }
}

pub fn square(a: i32) -> Result<f32, String> {
    Ok((a * a) as f32)
}

pub fn sqrt(a: i32) -> Result<f32, String> {
    if a < 0 {
        Err("Cannot take sqrt of negative number".to_string())
    } else {
        Ok((a as f32).sqrt())
    }
}