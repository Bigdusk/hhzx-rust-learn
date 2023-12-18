#[test]
fn main() {
    //智能智能指针，box将数据存放到堆上
    let b = Box::new(100);
    //智能指针都实现了deref和drop，但在表达式中我们无法隐式的执行解引用deref，所以我们需要*
    let num = *b + 100;
    println!("{}", num);

    //避免堆数据的拷贝

    // 在栈上创建一个长度为1000的数组
    let arr = [0;1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0;1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    //一个集合存放同种类型数据用box解决无法判断空间大小问题
    let box_vec: Vec<Box<dyn Animal>> = vec![Box::new(Cat),Box::new(Dog)];
    for a in box_vec {
        a.say();
    }

    //解引用*获取指针里面的值
    let x = Box::new(1);
    let sum = *x + 1;
    println!("{}", sum);
}

trait Animal {
    fn say(&self);
}

struct Cat;

impl Animal for Cat {
    fn say(&self) {
        println!("猫猫");
    }
}
struct Dog;

impl Animal for Dog {
    fn say(&self) {
        println!("狗狗")
    }
}