// use num::complex::Complex;



fn main(){
    // let x = '中';
    // println!("字符中'中'占用了{}字节的内存大小", size_of_val(&x));
    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }

}


// fn main() {
//     let a = Complex {re:2.1, im: -1.2};
//     let b =Complex::new(11.1, 22.2);
//     let result = a+b;

//     println!("{} + {}i", result.re, result.im);
// }

// fn main() {
//     println!("Hello, world!");
// }

// fn greet_world(){
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好！";
//     let english = "Hello, world!";
//     let regions = [southern_germany, chinese, english];
//     // for region in regions.iter(){
//     //     println!("{}", &region);
//     // }
//     for region in regions{
//         println!("{}", &region);
//     }
// }


// fn main() {
//     // greet_world();
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,63.5
//     Yellow-eyed penguin,100.5
//     Fiordland penguin,60.1
//     Invalid,data
//     ";
//     let records = penguin_data.lines();  // 这一步是什么意思？为什么要把penguin_data转换成lines？
//     for (i, record) in records.enumerate(){
//         if i==0 || record.trim().len()==0{
//             continue;
//         }
//         // 声明一个 fields变量， 类型是Vec
//         let fields: Vec<_> = record
//             .split(',')
//             .map(|field| field.trim())
//             .collect();
//         if cfg!(debug_assertions){
//             //输出到标准错误输出
//             eprintln!("debug: {:?}, {:?}",fields,record);
//         }
//         let name = fields[0];
//         if let Ok(length) = fields[1].parse::<f32>(){
//             println!("{}, {}cm", name, length);
//         }
//     }
// }

// fn main() {
//     let a = 10;
//     let b: i32 = 20;
//     let mut c = 30i32;
//     let d = 30_i32;
//     let e = add(a, b);
//     println!("e: {}", e);

//     let e = add(add(a, b), add(c, d));
//     println!("({} + {}) + ({} + {}) = {}", a, b, c, d, e);
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     // let mut x = 5;
//     // println!("The value of x is: {}", x);
//     // x = 6;
//     // println!("The value of x is: {}", x);
// }

// fn main() {
//     let a : u8 = 255;
//     let b = a.wrapping_add(20);
//     println!("{}", b);  // 19
// }

// fn main() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//     println!("abc (f32)");
//     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("         0.3: {:x}", (abc.2).to_bits());
//     println!();

//     println!("xyz (f64)");
//     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!("         0.3: {:x}", (xyz.2).to_bits());
//     println!();


//     let num = 10;
//     println!("Number is {}", num);
//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }


// fn main() {
    // // 无符号8位整数，二进制为00000010
    // let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

    // // 二进制为00000011
    // let b: u8 = 3;

    // // {:08b}：左高右低输出二进制01，不足8位则高位补0
    // println!("a value is        {:08b}", a);
    // println!("b value is        {:08b}", b);
    // println!("(a & b) value is  {:08b}", a & b);
    // println!("(a | b) value is  {:08b}", a | b);
    // println!("(a ^ b) value is  {:08b}", a ^ b);
    // println!("(!b) value is     {:08b}", !b);
    // println!("(a << b) value is {:08b}", a << b);
    // println!("(a >> b) value is {:08b}", a >> b);
    // let mut a = a;
    // // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    // a <<= b;
    // println!("(a << b) value is {:08b}", a);

    // for i in 1..=5{
    //     println!("i: {}", i);
    // }
    // for i in 'a'..='z'{
    //     println!("char: {}", i);
    // }
// }
