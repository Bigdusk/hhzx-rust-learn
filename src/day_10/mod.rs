#[test]
fn main() {
    //模式匹配

    //单分支多模式|
    let a = 1;
    match a {
        1 | 2 => println!("one or tow"),
        3 => println!("three"),
        _ => println!("默认")
    };

    //范围匹配
    match a {
        1..=10 => println!("1..10"),
        _ => println!("默认")
    }
}