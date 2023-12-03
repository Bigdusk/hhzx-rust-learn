#[test]
fn main() {
    //流程控制跳过...
    //模式匹配
    let a = Direction::South("东".parse().unwrap());
    //没有对应选项默到_选项,在没有穷举所以的情况下使用，当然也可以提取变量
    match a {
        Direction::East(v) => println!("east{}", v),
        Direction::West(_) => println!("west"),
        Direction::North => println!("North"),
        v => println!("默认处理{:?}", v)
    }

    //当选项只有一个的时候
    //当你只要匹配一个条件，且忽略其他条件时就用
    if let p @ Some(num) = Some(3){
        println!("if let{} {:?}", num, p);
    }
    //matches常用宏
    let v = vec![
        Direction::East("东".parse().unwrap()),
        Direction::East("东".parse().unwrap()),
        Direction::East("东".parse().unwrap()),
        Direction::West("西".parse().unwrap()),
        Direction::West("西".parse().unwrap()),
        Direction::West("西".parse().unwrap()),
        Direction::North
    ];
    //进行过滤
    let _ = v.iter().filter(|v| matches!(v, Direction::North));

    //模式匹配
    let vs = vec![1,2,3,4,5,6];
    //这里使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组
    for (index, value) in vs.iter().enumerate() {
        println!("{:?} {:?}", index, value);
    }
}
#[derive(Debug)]
enum Direction {
    East(String),
    West(String),
    North,
    South(String)
}