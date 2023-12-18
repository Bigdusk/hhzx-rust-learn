use std::fmt::{Display, Formatter, write};

#[test]
fn main() {
    //类型转换
    //1.只能进行同类型转换，大范围类型转小范围类型会报错
    let a:f32 = 1.1;
    let b:i32 = 300;
    let c:char = 'a';
    println!("{}", a as i32);
    println!("{}", b as i8);
    //字符转数字ASC-LL码
    println!("{}", c as u8);

    //向上转型try_into
    let d:u32 = 120;
    //用unwrap捕获错误
    let e:u8 = d.try_into().unwrap();
    println!("{}", e);

    //自定义类型newType
    println!("{}", Wrapper(vec![String::from("sada"), String::from("dasda")]));

    //进行类型判断!一种不反回类型
    let v1 = 100;
    let i = match v1 {
        0..=100 => v1 + 1,
        _ => panic!("不符合条件")
    };

    println!("{}", i);
}

//直接将定义类型
type MyType = u32;

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}