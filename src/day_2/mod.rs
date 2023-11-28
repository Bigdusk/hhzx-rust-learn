use num::Complex;

#[test]
fn main() {
    //整型溢出
    //test1();
    //浮点数
    //test2();

    //位运算
    //test3();

    //序列 range
    //test4();

    //常用库num，用于货币计算场景
    //test5();

    //表达式
    test6();
}

fn test6() {
    //表达式不能包含分号。这一点非常重要，一旦你在表达式后加上分号，它就会变成一条语句，再也不会返回一个值，请牢记！
    let a = {
        1 + 1
    };
    //最后，表达式如果不返回任何值，会隐式地返回一个 () 。比如在表达式里面结尾添加上分号
    println!("{}", a);
}

fn test5() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im)
}

fn test4() {
    //1..5 为 1,2,3,4
    for i in 1..5 {
        println!("{}", i);
    }
    //1..=5 为 1,2,3,4,5

    for n in 1..=5 {
        println!("{}", n);
    }

    //字母
    for i in 'a'..='z' {
        println!("{}",i);
    }
}

fn test3() {
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn test2() {
    //f32 与f64速度是差不多的，而且f64精度更高所以rust默认是f32
    let _a = 2.0;
    let _b: f32 = 2.0;

    //感受浮点计算的精度问题
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    //对于数学上未定义的结果，
    // 例如对负数取平方根 -42.1.sqrt() ，
    // 会产生一个特殊的结果：
    // Rust 的浮点数类型使用 NaN (not a number)来处理这些情况。
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为")
    }

    //数学计算
    // 加法
    let _sum = 5 + 10;

    // 减法
    let _difference = 95.5 - 4.3;

    // 乘法
    let _product = 4 * 30;

    // 除法
    let _quotient = 56.7 / 32.2;

    // 求余
    let _remainder = 43 % 5;


}

fn test1() {
/*  要显式处理可能的溢出，可以使用标准库针对原始数字类型提供的这些方法：

    使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    如果使用 checked_* 方法时发生溢出，则返回 None 值
    使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    使用 saturating_* 方法使值达到最小值或最大值*/

    let a:u8 = 255;
    println!("{}", a.wrapping_add(20));
}
