// 第二课：所有权、错误处理和 TCP 服务器

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// 第二课主函数
pub fn run_lesson2() {
    println!("\n🚀 第二课：TCP 服务器与 Rust 核心特性");
    println!("==========================================\n");
    
    // 课程 1：所有权系统
    lesson_ownership();
    
    // 课程 2：错误处理
    lesson_error_handling();
    
    // 课程 3：启动 TCP 服务器
    println!("\n📖 课程 3：启动 TCP 服务器");
    println!("----------------------------");
    println!("准备启动服务器...");
    
    // 启动服务器（这会阻塞，实际运行时需要在单独的终端测试）
    if let Err(e) = start_simple_server() {
        eprintln!("❌ 服务器错误: {}", e);
    }
}

/// 课程 1：理解所有权系统
/// 
/// 所有权是 Rust 最独特和核心的特性，让 Rust 无需垃圾回收器就能保证内存安全
/// 
/// 三条规则：
/// 1. Rust 中的每个值都有一个所有者（owner）
/// 2. 值在任一时刻只能有一个所有者
/// 3. 当所有者离开作用域，值将被丢弃（drop）
fn lesson_ownership() {
    println!("📖 课程 1：所有权系统（Ownership）");
    println!("----------------------------");
    
    // === 1. 基本的所有权转移（Move） ===
    let message1 = String::from("Hello, Rust!");
    println!("原始消息: {}", message1);
    
    // 所有权转移：message1 的所有权转移给了 message2
    let message2 = message1;
    println!("转移后: {}", message2);
    
    // ❌ 这行会编译错误，因为 message1 的所有权已经被转移
    // println!("原始消息: {}", message1);  // 取消注释会报错
    
    println!("✅ 所有权转移后，原变量不能再使用\n");
    
    // === 2. 克隆（Clone） ===
    let user1 = String::from("Alice");
    let user2 = user1.clone();  // 深拷贝，两个独立的值
    println!("用户1: {}, 用户2: {}", user1, user2);
    println!("✅ clone() 创建了深拷贝，两个变量都可用\n");
    
    // === 3. Copy 类型（基本类型） ===
    let x = 5;
    let y = x;  // 简单类型实现了 Copy trait，是值拷贝而非移动
    println!("x = {}, y = {}", x, y);
    println!("✅ 整数等基本类型是 Copy 的，赋值时会复制\n");
    
    // === 4. 函数与所有权 ===
    let greeting = String::from("你好");
    take_ownership(greeting);  // greeting 的所有权转移到函数内
    // println!("{}", greeting);  // ❌ 编译错误：greeting 已被移动
    
    let number = 42;
    makes_copy(number);  // 整数是 Copy，所以这里是复制
    println!("数字仍然可用: {}", number);  // ✅ 可以使用
    
    // === 5. 借用（Borrowing）- 不转移所有权 ===
    let chat_msg = String::from("大家好！");
    let len = calculate_length(&chat_msg);  // &表示借用，不转移所有权
    println!("消息 '{}' 的长度是 {}", chat_msg, len);  // ✅ 仍然可以使用
    
    // === 6. 可变借用 ===
    let mut user_name = String::from("Bob");
    println!("修改前: {}", user_name);
    change_string(&mut user_name);  // 可变借用
    println!("修改后: {}", user_name);
    
    println!("\n💡 所有权总结：");
    println!("  - 默认情况下，赋值/传参会转移所有权（Move）");
    println!("  - 使用 & 进行借用，不转移所有权");
    println!("  - 使用 &mut 进行可变借用");
    println!("  - 一个值同时只能有一个可变借用，或多个不可变借用");
}

// 辅助函数：接受所有权
fn take_ownership(s: String) {
    println!("函数接收到: {}", s);
    // s 在这里离开作用域并被 drop
}

// 辅助函数：Copy 类型
fn makes_copy(num: i32) {
    println!("函数接收到数字: {}", num);
}

// 辅助函数：借用（不可变引用）
fn calculate_length(s: &String) -> usize {
    s.len()  // 可以读取，但不能修改
}

// 辅助函数：可变借用
fn change_string(s: &mut String) {
    s.push_str(" (已修改)");
}

/// 课程 2：错误处理
/// 
/// Rust 使用 Result<T, E> 和 Option<T> 来处理错误，而非异常机制
/// 
/// Result<T, E>：
///   - Ok(T): 成功，包含值
///   - Err(E): 失败，包含错误信息
/// 
/// Option<T>：
///   - Some(T): 有值
///   - None: 无值
fn lesson_error_handling() {
    println!("\n📖 课程 2：错误处理");
    println!("----------------------------");
    
    // === 1. Option<T> - 处理可能不存在的值 ===
    let users = vec!["Alice", "Bob", "Charlie"];
    
    // 安全地获取元素
    match users.get(1) {
        Some(name) => println!("找到用户: {}", name),
        None => println!("用户不存在"),
    }
    
    match users.get(10) {
        Some(name) => println!("找到用户: {}", name),
        None => println!("索引 10 的用户不存在"),
    }
    
    // === 2. Result<T, E> - 处理可能失败的操作 ===
    println!("\n测试端口解析：");
    
    // 成功的情况
    match parse_port("8080") {
        Ok(port) => println!("✅ 解析成功: {}", port),
        Err(e) => println!("❌ 解析失败: {}", e),
    }
    
    // 失败的情况
    match parse_port("invalid") {
        Ok(port) => println!("✅ 解析成功: {}", port),
        Err(e) => println!("❌ 解析失败: {}", e),
    }
    
    // === 3. if let - 简化的 match ===
    println!("\n使用 if let 简化代码：");
    if let Ok(port) = parse_port("9000") {
        println!("端口号: {}", port);
    }
    
    // === 4. unwrap 和 expect（谨慎使用）===
    println!("\n使用 unwrap（生产环境要小心）：");
    let port = parse_port("3000").unwrap();  // 如果是 Err 会 panic
    println!("端口: {}", port);
    
    // expect 可以自定义错误消息
    let port2 = parse_port("4000").expect("端口解析失败！");
    println!("端口: {}", port2);
    
    // === 5. ? 操作符 - 错误传播 ===
    println!("\n测试错误传播：");
    match connect_to_port("8080") {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }
    
    println!("\n💡 错误处理总结：");
    println!("  - 使用 Result<T, E> 处理可能失败的操作");
    println!("  - 使用 Option<T> 处理可能不存在的值");
    println!("  - match 表达式进行模式匹配");
    println!("  - ? 操作符简化错误传播");
    println!("  - unwrap/expect 在确定不会失败时使用（或原型开发）");
}

// 辅助函数：解析端口号
fn parse_port(s: &str) -> Result<u16, String> {
    match s.parse::<u16>() {
        Ok(port) => Ok(port),
        Err(_) => Err(format!("'{}' 不是有效的端口号", s)),
    }
}

// 辅助函数：演示 ? 操作符
fn connect_to_port(port_str: &str) -> Result<String, String> {
    let port = parse_port(port_str)?;  // ? 会在错误时提前返回
    Ok(format!("成功连接到端口 {}", port))
}

/// 课程 3：创建简单的 TCP 服务器
/// 
/// 使用标准库的 TcpListener 创建一个能接收消息的服务器
fn start_simple_server() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    
    // 绑定到地址，返回 Result<TcpListener, Error>
    let listener = TcpListener::bind(address)?;
    
    println!("🌐 服务器启动成功！");
    println!("📍 监听地址: {}", address);
    println!("💡 测试方法：打开新终端，运行：");
    println!("   telnet 127.0.0.1 8080");
    println!("   或者：nc 127.0.0.1 8080");
    println!("\n等待客户端连接...\n");
    
    // 只接受一个连接（下节课会改进为多连接）
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("✅ 新客户端连接: {}", stream.peer_addr()?);
                handle_client(stream)?;
                println!("\n继续等待连接...");
            }
            Err(e) => {
                eprintln!("❌ 连接错误: {}", e);
            }
        }
    }
    
    Ok(())
}

/// 处理单个客户端连接
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // 发送欢迎消息
    let welcome = "欢迎来到 Rust 聊天服务器！\n请输入消息（输入 'quit' 退出）：\n";
    stream.write_all(welcome.as_bytes())?;
    
    // 读取缓冲区
    let mut buffer = [0; 512];
    
    loop {
        // 读取客户端消息
        match stream.read(&mut buffer) {
            Ok(0) => {
                // 0 字节表示客户端断开连接
                println!("👋 客户端断开连接");
                break;
            }
            Ok(n) => {
                // 转换为字符串
                let message = String::from_utf8_lossy(&buffer[..n]);
                let message = message.trim();
                
                println!("📨 收到消息: {}", message);
                
                // 检查退出命令
                if message.eq_ignore_ascii_case("quit") {
                    let goodbye = "再见！\n";
                    stream.write_all(goodbye.as_bytes())?;
                    println!("👋 客户端主动断开");
                    break;
                }
                
                // 回显消息
                let response = format!("服务器收到: {}\n", message);
                stream.write_all(response.as_bytes())?;
                
                // 清空缓冲区
                buffer = [0; 512];
            }
            Err(e) => {
                eprintln!("❌ 读取错误: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

