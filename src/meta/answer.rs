pub trait Answer: std::fmt::Display {}
impl Answer for i32 {}
impl Answer for i64 {}
impl Answer for usize {}
