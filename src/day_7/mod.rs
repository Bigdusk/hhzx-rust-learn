use std::ops::Add;

#[test]
fn main() {
    //枚举，强大
    let m1 = Message::QUit;
    let m2 = Message::Move {x: 6, y: 6};
    let m3 = Message::ChangeColor(255, 255, 255);
    input_message(m1);
    input_message(m2);
    input_message(m3);

    //rust没有null
    //用Option代替
    let some_number = Some(5);
    let some_string = Some("你好呀");
    let absent_number: Option<i32> = None;
    //Option计算,some必须转换类型才能计算
    let number = 5;
    let num = some_number.unwrap().add(number);
    println!("{}", num);
    let v : i32= match some_number {
        None => {0}
        Some(v) => {v + 1}
    };
    println!("{:?}", v);
}

fn input_message(m: Message) {
    println!("{:?}", m);
}

//强大数据结构
#[derive(Debug)]
enum Message {
    QUit,
    Move {
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColor(i32, i32, i32)
}