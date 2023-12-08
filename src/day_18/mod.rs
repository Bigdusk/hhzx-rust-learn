use std::fs::File;
use std::io::Read;

#[test]
fn main() {
    //恐慌，抛出异常panic
    //panic!("直接恐慌")

    //可恢复的错误
    let mut open_file = File::open("src/text.txt").expect("读取失败");
    //捕获错误
    /*let f = match open_file {
        Ok(file) => file,
        Err(_) => panic!("读取失败")
    };
    println!("{:?}", f);*/
    let mut s = String::new();
    open_file.read_to_string(&mut s).expect("TODO: panic message");
    println!("{:?}", s);
}