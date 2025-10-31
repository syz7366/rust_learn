// Rust 聊天应用 - 教学项目
// 通过构建完整的聊天应用学习 Rust

// 声明模块
mod lesson2;

use std::io::{self, Write};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   🦀 Rust 聊天应用 - 学习项目 🦀    ║");
    println!("╚════════════════════════════════════════╝");
    
    loop {
        println!("\n请选择要运行的课程：");
        println!("  1. 第一课：Rust 基础语法");
        println!("  2. 第二课：TCP 服务器与核心特性");
        println!("  0. 退出");
        print!("\n请输入选项 (0-2): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => run_lesson1(),
            "2" => lesson2::run_lesson2(),
            "0" => {
                println!("👋 再见！继续加油学习 Rust！");
                break;
            }
            _ => println!("❌ 无效选项，请输入 0-2"),
        }
    }
}

/// 第一课：Rust 基础语法
fn run_lesson1() {
    println!("\n🚀 第一课：Rust 基础语法");
    println!("================================");
    
    // 第一课：变量和不可变性
    lesson_1_variables();
    
    // 第二课：数据类型
    lesson_2_types();
    
    // 第三课：函数和返回值
    lesson_3_functions();
    
    println!("\n✅ 第一课完成！");
}

/// 课程 1：Rust 的变量和不可变性
fn lesson_1_variables() {
    println!("\n📖 课程 1：变量和不可变性");
    println!("----------------------------");
    
    // 不可变变量
    let user_name = "Alice";
    println!("用户名: {}", user_name);
    
    // 可变变量
    let mut online_users = 0;
    println!("在线用户: {}", online_users);
    online_users = 5;
    println!("在线用户更新为: {}", online_users);
    
    // 变量遮蔽
    let message = "Hello";
    let message = message.len();
    println!("消息长度: {}", message);
}

/// 课程 2：Rust 的基本数据类型
fn lesson_2_types() {
    println!("\n📖 课程 2：数据类型");
    println!("----------------------------");
    
    // 基本类型
    let user_id: u32 = 1001;
    let port: u16 = 8080;
    let is_online: bool = true;
    let emoji: char = '🎉';
    
    println!("用户 ID: {}, 端口: {}", user_id, port);
    println!("在线: {}, 表情: {}", is_online, emoji);
    
    // 字符串类型
    let owned_string: String = String::from("这是 String 类型");
    let string_slice: &str = "这是 &str 类型";
    println!("{}", owned_string);
    println!("{}", string_slice);
    
    // 元组和数组
    let user_info: (&str, u32, bool) = ("Bob", 25, true);
    let ports: [u16; 3] = [8080, 8081, 8082];
    
    println!("用户: {}, 年龄: {}, 在线: {}", 
             user_info.0, user_info.1, user_info.2);
    println!("端口列表: {:?}", ports);
}

/// 课程 3：函数和返回值
fn lesson_3_functions() {
    println!("\n📖 课程 3：函数和返回值");
    println!("----------------------------");
    
    greet_user("Charlie");
    
    let sum = add_numbers(10, 20);
    println!("10 + 20 = {}", sum);
    
    // 代码块表达式
    let result = {
        let a = 3;
        let b = 4;
        a * a + b * b
    };
    println!("3² + 4² = {}", result);
}

fn greet_user(name: &str) {
    println!("你好, {}! 欢迎来到聊天室", name);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
