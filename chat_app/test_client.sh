#!/bin/bash
# 简单的测试客户端脚本

echo "🔌 连接到 Rust 聊天服务器..."
echo "💡 提示：输入消息后按回车发送，输入 'quit' 退出"
echo ""

# 使用 nc (netcat) 连接服务器
# 如果系统没有 nc，可以用 telnet 127.0.0.1 8080
nc 127.0.0.1 8080



