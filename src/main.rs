/**
 * 块状注释
 */
// 主函数,普通注释
fn main() {
    // 带单号的是类函数宏
    println!("Hello, world!");

    // 注释不影响运行
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // 打印变量，可以使用下标，也可以具名，但是不可以混用
    println!("{0}->这个是下标", "第一个参数");
    println!("{first}->这个是具名", first = "第一个参数");
    // println!("{1}->这个是下标，{first}->这个是具名",first="第一个参数", "第二个参数")

    // 可以在 `:` 后面指定特殊的格式。
    println!("{}:{{}}  {:b}:{{:b}}", 1, 2);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number = 100, width = 6);

    // 自定义 #[derive] 宏,过程宏，给下面结构体添加一些通用的功能，而无需手动编写
    #[derive(Debug)]
    struct Structure(i32);
    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
    #[derive(Debug)]
    struct Deep(Structure);
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    println!("Now {:?} will print!", Deep(Structure(7)));

    // 假如我只想展示一个 `7` 怎么办？,自定义{}打印内容
    impl std::fmt::Display for Deep {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "-->{}", self.0 .0)
        }
    }
    println!("Now {} will print!", Deep(Structure(7)));
}
