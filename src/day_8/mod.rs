#[test]
fn main() {
    //创建数组
    //长度固定
    //元素必须有相同的类型
    //依次线性排列
    let arr:[i32; 6] = [1,2,3,4,5,6];
    //简便方法,生成重复数据
    /*let arr2 = [6; 99];
    for a in arr2 {
        println!("{}", a);
    }*/
    //数组切片
    let arr3 = &arr[4..5];
    println!("{:?}", arr3);
}