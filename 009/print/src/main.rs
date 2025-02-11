fn main() {
    println!("\n**************1、默认输出****************");
    println!("Hello, world!");

    println!("\n**************2、通配符****************");
    println!("今天是 {} 年 {} 月 {} 日", 2021, 6, 22);

    println!("\n**************3、通配符 + 位置****************");
    println!("{0} 的平方是 {0}, {0} 的相反数是 {1}", 1, -1);

    println!("\n**************4、通配符 + 命名参数****************");
    println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}", hobby = "打篮球", name = "张三", age = 18);

    println!("\n**************5、通配符 + 其它对齐方式****************");
    let a = 31;
    let b = [1, 3, 5, 7, 9];

    println!("二进制 {:b}", a);
    println!("八进制 {:o}", a);
    println!("十六进制(小写) {:x}", a);
    println!("十六进制(大写) {:X}", a);
    println!("科学计数(小写) {:e}", 100000_f32);
    println!("科学计数(大写) {:E}", 100000_f32);
    println!("打印Debug {:?}", b);
    println!("输出标点 {:+}", 5);

    println!("前置符二进制 {:#b}", a);
    println!("前置符八进制 {:#o}", a);
    println!("前置符十六进制(小写) {:#x}", a);
    println!("前置符十六进制(大写) {:#X}", a);
    println!("带换行和缩进的Debug打印 {:#?}", b);

    println!("使用大于号右对齐 {:>6}{:>6}{:>6}", 1, 2, 3);
    println!("使用小于号左对齐 {:<6}{:<6}{:<6}", 1, 2, 3);
    println!("省略大于号右对齐 {:6}{:6}{:6}", 1, 2, 3);
    println!("居中对齐 {:^6}{:^6}{:^6}", 1, 2, 3);
    println!("填充任意字符居中对齐 {:-^6}{:*^6}{:1^6}", 1, 2, 3);

    println!("二进制8位补零 {:08b}", a);
    println!("八进制8位补零 {:08o}", a);
    println!("十六进制16位补零 {:016b}", a);

    println!("宽度 {:1$}", 0.0123, 6);
    println!("宽度 {1:0$}", 6, 0.0123);
    println!("宽度 {:width$}", 0.0123, width = 6);

    println!("保留3位小数 {:.3} ", 0.01);
    println!("保留3位小数 {1:.0$} ", 3, 0.01);
    println!("{}小数保留3位数 {:.*} --- 保留4位数 {:.*} ", 0.01, 3, 0.01, 4, 0.10);

    println!("\n**************6、输出花括号****************");
    println!("左边的括号  {{");
    println!("右边的括号  }}");
    println!("全括号  {{}}");

    println!("\n**************7、输入****************");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read_line error!");
    println!("你输入的内容是 : {}", input);
}
