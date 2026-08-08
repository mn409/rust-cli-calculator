pub enum OperationType {
    Unary(fn(i32) -> f32),
    Binary(fn(i32, i32) -> f32),
}