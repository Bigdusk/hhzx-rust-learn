#[test]
fn main() {
    //trait类似java里面的接口，为解耦而生
    let cat = Cat {name: String::from("猫")};
    let dog = Dog {name: String::from("狗")};
    cat.say();
    dog.say();
}

trait Say {
    //在方法里面添加&self之后才能直接调用
    fn say(&self);
    fn hi(&self) {
        println!("hi 我是宠物");
    }
}

struct Cat {
    name: String
}
struct Dog {
    name: String
}

impl Say for Cat {
   fn say(&self) {
        println!("喵喵")
    }
    fn hi(&self) {
        println!("{}", self.name)
    }
}
impl Cat {
    //这种方法使我们传入的类型必须实现对应接口
    //也可以返回实现类型
    fn notify(item: &impl Say) -> &impl Say{
        item
    }
}
impl Say for Dog {
    fn say(&self) {
        println!("汪汪")
    }
}