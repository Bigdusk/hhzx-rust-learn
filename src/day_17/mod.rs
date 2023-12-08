use std::collections::HashMap;
#[test]
fn main() {
    //创建hashmap
    let mut hashmap = HashMap::new();
    hashmap.insert("红宝石1", 1);
    hashmap.insert("红宝石2", 2);
    hashmap.insert("红宝石3", 3);
    //迭代器
    let map: HashMap<_, _> = hashmap.clone().into_iter().collect();
    println!("{:?}", map);
    println!("{:#?}", hashmap);
}