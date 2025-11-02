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

fn main() {
    let a = 10;
    let b: i32 = 20;
    let mut c = 30i32;
    let d = 30_i32;
    let e = add(a, b);
    println!("e: {}", e);

    let e = add(add(a, b), add(c, d));
    println!("({} + {}) + ({} + {}) = {}", a, b, c, d, e);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}


