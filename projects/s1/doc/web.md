* Web Service
* 服务端 Web App
* 客户端 Web app (WebAssembly)
* Web框架 Actix
* 数据库： PostgreSQL
* 数据库链接： SQLx

## 1. TCP Server 

编写TCP Serve 和 Client

* std::net 网络的基本功能 
* TCP 和 UDP通信 
* TcpListener 和 TcpStream


## 2. Http Server

编写 Server 测试 Server

Rust没有内置的HTTP支持

### 组成部分
* Server 
  * 监听进来的TCP字节流
* Router
  * 接受http 请求，并决定调用哪个handler
* Handler
  * 处理http请求，构建http响应
* Http Library 
  * 解释字节流，把它转化为http请求
  * 把http响应转化为字节流

### 构建步骤

* 解析http请求消息
  * 三个数据结构
    * HttpRequest struct 表示http请求
    * Method enum 指定所允许的http方法
    * Version enum 指定所允许的http版本

  * 需要实现的trait
    From<&str> 用于吧传进来的字符串切片转化为HttpRequest
    Debug 打印调试信息
    PartialEq 用于解析和自动化测试脚本里做比较
* 构建http响应消息
* 路由与handler
* 测试web server



Http请求

* request line  `GET /greeting Http/1.1` Method path Version
* header line 1  `HOST:localhost:3000` Key: Value
* header line 2
* header line 3
* empty line
* message body `xxxx` Data


Http 响应

* status line `HTTP/1.1 200 OK` Version Status code Status text
* header line1 `Content-type: text/html` key value
* empty line
* message body 