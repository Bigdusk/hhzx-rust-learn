#[test]
fn main() {
    //创建泛型，主要是因为懒
    println!("{}", add(1, 2))
}
//替代
fn add<T: std::ops::Add<Output = T>>(num1: T, num2: T) -> T{
    num1 + num2
}
fn add_i8(a:i8, b:i8) -> i8 {
    a + b
}
fn add_i32(a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
    a + b
}