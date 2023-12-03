#[test]
fn main() {
    //字符切片，集合也可以切片
    let hi = String::from("hello world");

    let hello = &hi[0..5];
    let world = &hi[6..11];
    println!("{}{}", hello, world);
    //在对字符串使用切片语法时需要格外小心，
    // 切片的索引必须落在字符之间的边界位置，
    // 也就是 UTF-8 字符的边界，
    // 例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：

/*    let s = "中国人";
    let a = &s[0..3];
    println!("{}",a);
*/

    //string转str
    let string = String::from("string");
    let str = string.as_str();

    //字符串操作

    let mut text = String::from("你好，世界。我是rust代码。");
    //字符串追加
    text.push_str("世界");

    //字符追加
    text.push('w');

    //插入字符串
    text.insert_str(3, "插入数据");

    //插入字符
    text.insert(0, 'l');

    //替换,替换需要重新赋值
    let text2 = text.replace("rust", "RUST");
    println!("{}", text2);
    println!("{}", text);

}