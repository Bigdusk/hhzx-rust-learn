#[test]
fn main() {
    //元组是由多种类型组合到一起形成的，因此它是复合类型，元组的长度是固定的，元组中元素的顺序也是固定的
    //创建元组
    let tup = (100, 0.1, 1);
    //解构
    let (x, y, z) = tup;
    //用.直接调用
    println!("{} {} {}", tup.0, tup.1, tup.2);

    //测试函数返回元组
    let (a, b) = test_tuple();
    println!("{} {}", a, b);
}

fn test_tuple() -> (i32, i32){
    (1, 2)
}