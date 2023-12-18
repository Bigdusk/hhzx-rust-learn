use std::{fmt, io};
use std::fmt::{Formatter, write};
use std::fs::File;
use std::io::Error;

#[test]
fn main() {
/*    //错误处理
    let a = Some("qwe");
    let b = Some("rty");
    println!("{:?}", a.or(b));
    println!("{:?}", b.and(a));
    //filter对值进行过滤
    println!("{:?}", a.filter(|v| {
        v.eq(&"qwe2")
    }));
    //map将值映射为另一个值
    println!("{:?}", a.map(|v| {
        v.to_owned() + "d"
    }))*/


    //自定义错误
    match produce_error() {
        Ok(_) => {}
        Err(e) => {
            println!("错误信息：{}", e)
        }
    }

    //?可以将错误类型强制转换为方法抛出类型
    match produce_error_one() {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

struct AppError;
#[derive(Debug)]
struct AppErrorOne {
    msg: String
}
impl From<io::Error> for AppErrorOne {
    fn from(value: Error) -> Self {
        AppErrorOne {
            msg: value.to_string()
        }
    }
}
fn produce_error_one() -> Result<(), AppErrorOne>{
/*    Err(AppErrorOne {
        msg:"错误了".parse().unwrap()
    })*/
    let _file = File::open("nonexistent_file.txt")?;

    Ok(())
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "自定义错误")
    }
}

fn produce_error() -> Result<(), AppError>{
    Err(AppError)
}