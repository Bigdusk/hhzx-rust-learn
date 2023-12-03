#[test]
fn main() {
    //定义结构体
    let user = User {
        username: String::from("用户"),
        password: String::from("密码"),
        email: String::from("邮箱"),
        age: 21,
        money: 3000.00,
    };
    println!("{:?}", user);

    //实现new结构体方法
    let yxp = User::new(
        String::from("yxp"),
        String::from("123456"),
        String::from("2831828656@qq.com"),
        21,
        3000.00
    );
    println!("{:?}", yxp);

    //创建新结构体用上一个数据
    let user2 = User {
        username: String::from("新用户"),
        ..user
    };
    println!("{:?}", user2);
    //结构体里面的数据所有权可以转移
    let name = &user2.username;
    println!("{}", user2.username);


    //元组结构体，struct tuple
    //结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体
    let my_tuple = MyTuple(-32, 32, 32.0);
    println!("{:?}", my_tuple);


    //单元结构
    //如果你定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体
    let date = AlwaysEqual::new();
}
struct AlwaysEqual;

//不关注数据只关注行为
impl AlwaysEqual {
    fn new() {}
}



#[derive(Debug)]
struct MyTuple(i32, u32, f32);


#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    age: u32,
    money: f32,
}

impl User {
    fn new(
        username: String,
        password: String,
        email: String,
        age: u32,
        money: f32,
    ) -> User {
        User {
            username,
            password,
            email,
            age,
            money,
        }
    }
}