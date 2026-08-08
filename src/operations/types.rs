pub enum OperationType {
    Unary(fn(i32) -> Result<f32, String>),
    Binary(fn(i32, i32) -> Result<f32, String>),
}