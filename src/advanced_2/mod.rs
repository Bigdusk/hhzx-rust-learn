use std::collections::HashMap;

#[test]
fn main() {
    //迭代器,尽量使用迭代器
    let mut v = vec![1, 2, 3, 4, 5, 6];
    //into_iter 会夺走所有权
    let mut iter = v.iter();
    //iter 是借用
    //let mut iter = v.iter();
    //iter_mut 是可变借用
    //let mut iter = v.iter_mut();

    //map方法取出值与闭包配合使用,定义处理方法，还需要调用收集方法collect
    let vo: Vec<_>= v.iter().map(|x| x + 100).collect();
    println!("{:?}", vo);

    //将俩个数组压缩成hashmap
    let arr_string = ["修猫", "修狗"];
    let arr_name = ["cat", "dog"];

    let hash_map: HashMap<_, _> = arr_string.iter().zip(arr_name.iter()).collect();
    let arr_index = vec![1,2,3,5];
    //用enumerate,将集合里面的索引通过元组提取出来
    for (index, value) in arr_index.iter().enumerate() {
        println!("{} {}", index, value);
    }
    println!("{:#?}", hash_map);
}