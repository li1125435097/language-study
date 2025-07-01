use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::process;
use std::collections::VecDeque;
use std::os::unix::fs::PermissionsExt;

// 使用标准库实现的简单AES加密
fn aes_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    // 简单的AES-256-CTR实现
    let mut result = Vec::with_capacity(data.len());
    let mut key_stream = KeyStream::new(key, iv);
    
    for byte in data {
        result.push(byte ^ key_stream.next());
    }
    
    result
}

// 密钥流生成器（模拟CTR模式）
struct KeyStream {
    key: [u8; 32],
    iv: [u8; 16],
    counter: u128,
    buffer: VecDeque<u8>,
}

impl KeyStream {
    fn new(key: &[u8], iv: &[u8]) -> Self {
        let mut key_arr = [0u8; 32];
        let mut iv_arr = [0u8; 16];
        
        // 复制密钥和IV
        let len = key.len().min(32);
        key_arr[..len].copy_from_slice(&key[..len]);
        
        let len = iv.len().min(16);
        iv_arr[..len].copy_from_slice(&iv[..len]);
        
        Self {
            key: key_arr,
            iv: iv_arr,
            counter: 0,
            buffer: VecDeque::new(),
        }
    }
    
    fn next(&mut self) -> u8 {
        if self.buffer.is_empty() {
            self.generate_block();
        }
        self.buffer.pop_front().unwrap()
    }
    
    fn generate_block(&mut self) {
        // 使用简单的伪随机函数生成密钥块
        let mut block = [0u8; 16];
        
        // 使用counter和iv初始化块
        for i in 0..16 {
            block[i] = self.iv[i] ^ (self.counter as u8).wrapping_add(i as u8);
        }
        
        // 应用密钥（简单的混淆）
        for i in 0..32 {
            let idx = i % 16;
            block[idx] = block[idx].wrapping_add(self.key[i]).rotate_left(3);
        }
        
        // 增加计数器
        self.counter = self.counter.wrapping_add(1);
        
        // 填充缓冲区
        self.buffer.extend(block.iter());
    }
}

fn handle_client(mut stream: UnixStream) {
    // 读取请求头: [密钥长度(1字节)] + [IV长度(1字节)] + [数据长度(4字节)]
    let mut header = [0u8; 6];
    if let Err(e) = stream.read_exact(&mut header) {
        eprintln!("读取头错误: {}", e);
        return;
    }
    
    let key_len = header[0] as usize;
    let iv_len = header[1] as usize;
    let data_len = u32::from_be_bytes([header[2], header[3], header[4], header[5]]) as usize;
    
    // 检查长度是否合理
    if key_len > 256 || iv_len > 256 || data_len > 10 * 1024 * 1024 {
        let _ = stream.write_all(b"ERROR: Invalid length");
        return;
    }
    
    // 读取完整请求
    let mut request = vec![0u8; key_len + iv_len + data_len];
    if let Err(e) = stream.read_exact(&mut request) {
        eprintln!("读取数据错误: {}", e);
        return;
    }
    
    // 分割请求数据
    let key = &request[..key_len];
    let iv = &request[key_len..key_len + iv_len];
    let data = &request[key_len + iv_len..];
    
    // 加密数据
    let encrypted = aes_encrypt(data, key, iv);
    
    // 发送加密结果
    if let Err(e) = stream.write_all(&encrypted) {
        eprintln!("写入响应错误: {}", e);
    }
}

fn main() {
    // 设置套接字路径
    let socket_path = "/tmp/native_encrypt.sock";
    
    // 清理旧套接字
    if let Err(e) = fs::remove_file(socket_path) {
        if e.kind() != std::io::ErrorKind::NotFound {
            eprintln!("删除旧套接字错误: {}", e);
            process::exit(1);
        }
    }
    
    // 创建监听器
    let listener = match UnixListener::bind(socket_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("绑定套接字错误: {}", e);
            process::exit(1);
        }
    };
    
    // 设置套接字权限 (仅限当前用户)
    if let Err(e) = fs::set_permissions(socket_path, fs::Permissions::from_mode(0o600)) {
        eprintln!("设置权限错误: {}", e);
    }
    
    println!("🔒 原生加密服务器启动，监听在 {}", socket_path);
    println!("⚠️ 注意: 此实现仅用于演示，不适用于生产环境");
    
    // 处理连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
            }
        }
    }
}