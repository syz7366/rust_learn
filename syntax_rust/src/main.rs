

fn main(){


    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }
    assert_eq!(count, 5);

    // let mut n =1;
    // while n<10 {
    //     n+=1;
    // }
    // println!("n: {}", n);
    // let a = [4,3,2,1];
    // // 通过索引和值的方式迭代数组 `a` 
    // for (i,v) in a.iter().enumerate() {
    //     println!("第{}个元素是{}",i+1,v);
    // }
    // let names = [String::from("liming"),String::from("hanmeimei")];
    // // for name in &names {
    // //     println!("{}", name);
    // // }
    // for name in names.iter() {
    //     println!("{}", name);
    // }

    // println!("{:?}", names);

    // let numbers = [1, 2, 3];
    // for n in numbers {
    //     println!("{}", n);
    // }
    
    // println!("{:?}", numbers);
    // for n in 1..=100{
    //     println!("{}", n);
    //     if n == 100{
    //         panic!("NEVER LET THIS RUN！");
    //     }
    // }
    // let n = 5;
    // let big_n =
    //     if n < 10 && n>-10{
    //         println!("数字太小， 增大");
    //         10* n
    //     }else{
    //         println!("数字太大， 缩小范围");
    //         n / 2
    //     };
    // println!("{} -> {}", n, big_n);
    // let n =5;
    // if n<0 {
    //     println!("{} is negative", n);
    // }else if n>0{
    //     println!("{} is positive", n);
    // }else{
    //     println!("{} is zero", n);
    // }
}


// use crate::List::*;

// enum List{
//     // Cons 链表中包含有值的节点，第二个元素指向下一个节点的指针
//     Cons(u32, Box<List>),
//     // Nil 链表尾部的值
//     Nil,
// }

// impl List{
//     // 创建空的链表
//     fn new()->List{
//         Nil
//     }

//     fn prepend(self, elem: u32)->List{
//         Cons(elem, Box::new(self))
//     }

//     // 链表长度   模式匹配的写法
//     // fn len(&self) -> usize{
//     //     match *self{
//     //         Cons(_, ref tail)=>1+tail.len(),
//     //         Nil =>0
//     //     }
//     // }

//     fn len(&self) -> usize{
//         if let Cons(value, next)=self {
//             let tail = next;
//             1+tail.len()
//         }else{
//             0
//         }
//     }

//     fn stringify(&self)->String{
//         match *self{
//             Cons(head, ref tail)=>{
//                 format!("{}, {}", head, tail.stringify())
//             },
//             Nil =>{
//                 format!("Nil")
//             },
//         }
//     }
// }

// fn main(){
//     let mut list = List::new();
//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);
//     println!("list length: {}", list.len());
//     println!("list stringify: {}", list.stringify());
// }


// use num::complex::Complex;
// fn greetings(s: &str){
//     println!("{}",s);
// }
// fn create_arr(n: i32) {
//     let arr = [1; n];
//     println!("{:?}", arr);
// }
// fn type_name_of<T>(_: T) -> &'static str {
//     std::any::type_name::<T>()
// }
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// struct Unit;
// trait SomeTrait{
//     // ... 定义一些行为
// }

// impl SomeTrait for Unit{}

// fn do_something_with_unit(u: Unit) { 

//     println!("done!");
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
   
// fn check_color(p: Point) {
//     let Point(x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
//  }
// struct Person {
//     name: String,
//     age: u8,
// }

// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         name,
//     }
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }

// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }
// #[derive(Debug)]
// struct File {
//     name: String,
//     data: String,
// }
// enum Number{
//     Zero,
//     One,
//     Two,
// }
// enum Number1{
//     Zero = 0,
//     One,
//     Two,
// }
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }

// fn plus_one(x: Option<i32>) -> Option<i32>{
//     match x{
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main(){

//     let five =Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     if let Some(n) = six {
//         println!("{}", n)
//     } 

//     panic!("NEVER LET THIS RUN！");
    // panic!("不要让这行代码运行！");
    // let msgs: [Message; 3] = [
    //     Message::Quit,
    //     Message::Move{x:1, y:3},
    //     Message::ChangeColor(255,255,0)
    // ];
    // for msg in msgs {
    //     show_message(msg)
    // }

    // let msg = Message::Move{x: 1, y: 2};

    // if let Message::Move{x:a,y:b} = msg {
    //     assert_eq!(a, b);
    // } else {
    //     panic!("随便打印点什么");
    // }

    // let msg1 = Message::Move { x:1, y:2 };
    // let msg2 = Message::Write(String::from("hello"));
    // println!("msg1: {:?}", msg1);
    // println!("msg2: {:?}", msg2);
    // println!("Number: {:?}", Number::One as i32);
    // println!("Number1: {:?}", Number1::One as i32);
    // println!("Number2: {:?}", Number2::One);
    // assert_eq!(Number::One as i32, Number1::One as i32);
    // assert_eq!(Number1::One, Number2::One);
    // let f = File {
    //     name: String::from("readme.md"),
    //     data: "Rust By Practice".to_string()
    // };
    // let _name = f.name;
    // // 只能修改这一行
    // println!("{}",f.data);

    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: Box<u8>,
    // }
    // let person = Person {
    //     name: String::from("Alice"),
    //     age: Box::new(18),
    // };
    // 结构体所有权问题
    // let Person { name, ref age } = person;
    // println!("person: {:?}", person);
    // println!("person.age: {}", *person.age);
    // println!("person.name: {}", person.name);
    // println!("The person's age from person struct is {}", person.age);

    // let u1 = User {
    //     email: String::from("someone"),
    //     username: String::from("sunface"),
    //     active: true,
    //     sign_in_count: 1,
    // };
    // let u2 = set_email(u1);
    // println!("u2: {:?}", u2);
    // let u1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("sunface"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let u2 = set_email(u1);
    // println!("u2: {:?}", u2);
    // let age = 18;
    // let mut p = Person {
    //     name: String::from("sunface"),
    //     age,
    // };
    // p.age = 30;
    // p.name = String::from("sunfei");

    // println!("{}", p.name);
    // println!("{}", p.age);
    // let v = Point(0, 127, 255);
    // check_color(v);
    // let u = Unit;
    // do_something_with_unit(u);
    // 填空，让代码工作
    // let age = 30;
    // let p = Person{
    //     name: String::from("sunface"),
    //     age,
    //     hobby: String::from("coding"),
    // };
    // println!("{}", p.name);
    // println!("{}", p.age);
    // println!("{}", p.hobby);

    // let tup = (1, 6.4, "hello");
    // // 填空
    // let (x,z,y) = tup;
    // assert_eq!(x, 1);
    // assert_eq!(y, "hello");
    // assert_eq!(z, 6.4);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("{:?}",too_long_tuple);

    // let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // println!("{:?}",_t1);
    // println!("{}",_t1.0);
    // println!("{}",_t1.1.0);
    // println!("{}",_t1.1.1);

    // let t0: (u8,i16) = (0,-1);
    // println!("t0:{}",t0.0);
    // println!("t0:{}",t0.1);
    // let s = "你好，世界";
    // let slice = &s[0..3];
    // assert!(slice == "你");
    // let s = String::from("hello");
    // let slice1 = &s[0..2];

    // let slice2 = &s[..2];
    // assert_eq!(slice1, slice2);


    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let slice: &[i32] = &arr[1..4];
    // assert_eq!(slice, &[2, 3, 4]);
    // println!("slice:{}",type_name_of(&slice));


    // let arr: [char;3] = ['中', '国', '人'];
    // let slice = &arr[..2];
    // println!("{:?}", slice);
    // assert!(std::mem::size_of_val(&slice) == 16);

    // let arr = [1,2,3];
    // let s1: &[i32] = &arr[0..2];
    // let s2: &str = "hello, world";
    // println!("{:?}", s1);
    // println!("{}", s2);
    // let names = [String::from("Sunfei"),"Sunface".to_string()];
    // let name0 = names.get(0).unwrap();
    // println!("{}", name0);
    // let _name1 = &names[1];
    // println!("{}", _name1);

    // let arr = ['a', 'b', 'c'];
    // let ele = arr[0]; // 只修改此行来让代码工作
    // assert!(ele == 'a');
    // let list: [i32; 100] = [1; 100];
    // println!("{:?}", list);
    // assert!(list[0] == 1);
    // assert!(list.len() == 100);
    // let arr0 = [1, 2, 3];
    // let arr: [_; 3] = ['a', 'b', 'c'];
    // println!("{:?}", arr0);
    // println!("{:?}", arr);
    // assert!(std::mem::size_of_val(&arr) == 12);

    // let arr:[i32; 5] = [1, 2, 3, 4, 5];
    // println!("{:?}", arr);
    // println!("{}", arr.len());


    // for c in "你好，世界".chars() {
    //     println!("{}", c)
    // }
    // let s1 = String::from("hi,中国");
    // let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    // assert_eq!(h, "h");
    // println!("{}",h);

    // let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    // assert_eq!(h1, "中");
    // println!("{}",h1);

    // let raw_bytestring = br"\u{211D} is not escaped here";
    // println!("{:?}", raw_bytestring);
    // if let Ok(my_str) = str::from_utf8(raw_bytestring) {
    //     println!("And the same as text: '{}'", my_str);
    // }

    // let bytestring: &[u8; 21] = b"this is a byte string";
    // println!("A byte string: {:?}", bytestring);

    // let escaped = b"\x52\x75\x73\x74 as bytes";
    // println!("Some escaped bytes: {:?}", escaped);

    // let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // assert_eq!(raw_str, "Escapes don't work here: ? ℝ");
    // println!("{}",raw_str);

    // let quotes = r#"And then I said: "There is no escape!""#;
    // println!("{}", quotes);
    // let long_string = "String literals
    // can span multiple lines.
    // The linebreak and indentation here \
    //  can be escaped too!";
    // println!("{}", long_string);
    
    // let byte_escape = "I'm writing Ru\x73\x74!";
    // println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // let s1 = String::from("hello,");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; 
    // assert_eq!(s3,"hello,world!");
    // // println!("{}",s1);
    // println!("{}",s2);
    // println!("{}",s3);

    // let s = String::from("I like dogs");
    // // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    // let s1 = s.replace("dogs", "cats");

    // assert_eq!(s1, "I like cats")

    // let mut s = String::new();
    // s.push_str("hello, world");
    // s.push('!');

    // assert_eq!(s, "hello, world!");

    // let s:Box<str> = "hello, world".into();
    // greetings(&s);

    // for c in "中国人".bytes() {
    //     println!("{}", c);
    // }
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    // let byte_escape = "I'm writing \x52\x75\x73\x74!";
    // println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // // \u 可以输出一个 unicode 字符
    // let unicode_codepoint = "\u{211D}";
    // let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    // println!(
    //     "Unicode character {} (U+211D) is called {}",
    //     unicode_codepoint, character_name
    // );
    // // 换行了也会保持之前的字符串格式
    // // \n可以忽略换行符
    // let long_string = "String literals
    //                     can span multiple lines.
    //                     The linebreak and indentation here ->\
    //                     <- can be escaped too!";
    // println!("{}", long_string);

    // 字符串转义
    // let s1 = "hello";
    // let s2 = String::from("rust");
    // let s3 = "world";
    // let s = format!("{} {}!", s1, s3);
    // println!("{}", s);
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);
    // let s1 = String::from("hello,");
    // let s2 = String::from("world!");
    // // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    // let s3 = s1 + &s2;
    // assert_eq!(s3,"hello,world!");

    // // 字符串拼接
    // let string_append = String::from("hello ");
    // let string_rust = String::from("rust");
    // // &string_rust 会自动解引用为&str
    // let result = string_append + &string_rust;
    // let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    // result += "!!!";

    // println!("连接字符串 + -> {}", result);
    // 所有权已经被转移
    // println!("连接字符串 += -> {}", string_append);
    // let mut string_clear = String::from("string clear");
    // string_clear.clear();
    // dbg!(string_clear);

    // let mut string_truncate = String::from("测试truncate方法");     这里是以字节为单位的
    // string_truncate.truncate(3);
    // dbg!(string_truncate);
    // let mut string_remove = String::from(
    //     "测试remove方法"
    // );
    // println!(
    //     "string_remove 占 {} 个字节",
    //     std::mem::size_of_val(string_remove.as_str())
    // );
    // 删除第一个汉字
    // string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    // dbg!(string_remove);

    // let mut string_pop = String::from("rust pop 中文！");
    // let p1 = string_pop.pop();
    // let p2 = string_pop.pop();
    // dbg!(p1);
    // dbg!(p2);
    // dbg!(string_pop);

    // let string_replace = String::from("I like rust. Learning rust is my favorite!");
    // let mut string_replace_range = String::from("I like rust. Learning rust is my favorite!");
    // // let new_string_replace = string_replace.replace("rust", "RUST");
    // // let new_string_replace = string_replace.replacen("rust", "RUST", 1);
    // string_replace_range.replace_range(0..1,"R");
    // dbg!(string_replace_range);
    // dbg!(new_string_replace);

    // let mut s = String::from("Hello rust!");
    // s.insert(5, ',');
    // println!("插入字符 insert() -> {}", s);
    // s.insert_str(6, " I like");
    // println!("插入字符串 insert_str() -> {}", s);

    // let mut s = String::from("Hello ");

    // s.push_str("rust");
    // println!("追加字符串 push_str() -> {}", s);

    // s.push('!');
    // println!("追加字符 push() -> {}", s);
    // let s = String::from("hello world!");
    // say_hello(&s);
    // say_hello(&s[..]);
    // say_hello(s.as_str());

    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3];
    // assert_eq!(slice, &[2, 3]);
// }
// fn say_hello(s: &str){
//     println!("Hello, {}!", s);
// }


// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error!

//     println!("the first word is: {}", word);
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }

// fn main(){
//     let s = String::from("hello");

//     let len = s.len();
    
//     // let slice = &s[4..len];
//     let slice = &s[4..];
//     println!("{}", slice);

//     // let s = String::from("hello world");
//     // let hello = &s[0..5];
//     // let world = &s[6..11];

//     // println!("{} {}", hello, world);
// }


// #![allow(unused_variables)]
// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }

// fn close(f: &mut File)->bool{
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main(){
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     read(&mut f1, &mut vec![]);
//     close(&mut f1);
// }


// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn main(){
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     println!("{} and {}", r1, r2);
//     // 新编译器中，r1,r2作用域在这里结束

//     let r3 = &mut s;
//     println!("{}", r3);
// }


// fn main() {
    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    //     println!("{}", r1);
    // } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    // let r2 = &mut s;
    // println!("{}", r2);
    // println!("{}, {}", r1, r2);
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("s: {}", s);
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    
    // println!("{}, {}", r1, r2);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main(){
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
//     // assert_eq!(5, y);
// }

// fn main() {
//     let s1 = gives_ownership();         
                                        

//     let s2 = String::from("hello");     

//     let s3 = takes_and_gives_back(s2);  
// } 

// fn gives_ownership() -> String {             

//     let some_string = String::from("hello"); 

//     some_string                              
// }

// fn takes_and_gives_back(a_string: String) -> String { 

//     a_string  
// }

// fn main(){
//     let s = String::from("hello");
//     takes_ownership(s);
//     println!("s: {}", s);
//     let x = 5;
//     makes_copy(x);
//     println!("x: {}", x);
// }

// fn takes_ownership(s: String){
//     println!("s: {}", s);
// }

// fn makes_copy(x: i32){
//     println!("x: {}", x);
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn plus_five(x:i32) -> i32 {
//     x + 5
// }

// fn main() {
//     let x = plus_five(5);

//     println!("The value of x is: {}", x);
// }

// fn main(){
//     another_function(5, 6.4);
// }

// fn another_function(x:i32, y:f32){
//     // 不接受匿名参数了
//     println!("x: {}", x);
//     println!("y: {}", y);
// }

// fn main(){
//     // let x = '中';
//     // println!("字符中'中'占用了{}字节的内存大小", size_of_val(&x));
//     let t = true;

//     let f: bool = false; // 使用类型标注,显式指定f的类型

//     if f {
//         println!("这是段毫无意义的代码");
//     }

// }


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
