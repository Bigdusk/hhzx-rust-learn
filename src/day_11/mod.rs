#[test]
fn main() {
    //self, &self, &mut self,与java里面的this很像
    let mut cat = Cat::new(String::from("小梦"));
    cat.set_name(String::from("小黑"));
    cat.say_name();
    //为enum定义方法
    let dog = Animal::Dog;
    dog.get_type();
}
#[derive(Debug)]
enum Animal {
    Dog,
    Pig,
    Fowl
}

impl Animal {
    pub fn get_type(&self) {
        println!("{:?}", self);
    }
}
struct Cat {
    name: String
}

impl Cat {
    fn new(name: String) -> Cat{
        Cat {
            name
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn say_name(&self) {
        println!("{}", self.name);
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}