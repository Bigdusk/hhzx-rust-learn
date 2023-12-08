#[test]
fn main() {
    //集合类型
    let v1:Vec<i32> = Vec::new();
    //利用宏vec!初始化创建
    let v2 = vec![1,2,3,4];

    //读取数据
    println!("{}", &v2[1]);
    //发生越界
    //println!("{}", &v2[6]);

    //调用get方法解决，个体方法有默认处理
    println!("{:?}", &v2.get(6));

    //常用方法
    let mut  vec = vec![9,12,3,5,2,1,2,3,4];
    //调整容积
    vec.reserve(100);

    //释放容量
    vec.shrink_to_fit();


    //插入
    vec.insert(2, 100);
    //排序
    vec.sort_unstable();
    //检查是否为空，
    println!("{}", vec.is_empty());
    for v in &vec {
        println!("{}", v);
    }
    //删除
    vec.remove(0);

    //返回尾部
    println!("弹出{:?}", vec.pop());
    println!("长度{}， 容量{}", &vec.len(), &vec.capacity());
}