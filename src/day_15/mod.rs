#[test]
fn main() {
    //关联类型，具体到对应的类型。防止手贱传类型，于简化代码


}
struct Cat;
struct Dog;
trait Animal {
    type Cat;
    type Dog;
    //强制指定特征里面方法的类型
    fn animal(&self, a: &Self::Cat, b: &Self::Dog) -> bool;
}