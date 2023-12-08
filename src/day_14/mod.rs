#[test]
fn main() {
    //特征对象
    //Box<dyn T>,接收实现T的类型，解析出来对应类型，运行实现方法
    let s = Screen {
        components:vec![
            Box::new(Cat),
            Box::new(Dog)
        ]
    };
    s.run();

    //不是所有特征都能拥有特征对象，只有对象安全的特征才行。当一个特征的所有方法都有如下属性时，它的对象才是安全的：
    //
    // 方法的返回类型不能是 Self
    // 方法没有任何泛型参数
    // 对象安全对于特征对象是必须的，因为一旦有了特征对象，
    // 就不再需要知道实现该特征的具体类型是什么了。
    // 如果特征方法返回了具体的 Self 类型，
    // 但是特征对象忘记了其真正的类型，
    // 那这个 Self 就非常尴尬，因为没人知道它是谁了。
    // 但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：
    // 此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。
}

//创建绘画
pub struct Screen {
    components:Vec<Box<dyn Draw>>
}
impl Screen {
    pub fn run(&self) {
        //读取所以绘画实现方法
        for component in self.components.iter() {
            //运行实现的绘画方法
            component.draw();
        }
    }
}
//self代表当前对象与Self当前对象类型,在trait起作用，用于指定返回类型
trait Draw {
    fn draw(&self);
}
struct Cat;
struct Dog;

impl Draw for Cat {
    fn draw(&self) {
        println!("画猫");
    }
}
impl Draw for Dog {
    fn draw(&self) {

        println!("画狗")
    }
}