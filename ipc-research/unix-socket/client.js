const net = require('net');
const crypto = require('crypto');
const fs = require('fs');

const SOCKET_PATH = '/tmp/native_encrypt.sock';

// 加密函数
async function encryptData(data) {
    return new Promise((resolve, reject) => {
        // 生成随机密钥和IV
        const key = crypto.randomBytes(32); // 32字节密钥
        const iv = crypto.randomBytes(16);  // 16字节IV
        
        const client = net.createConnection(SOCKET_PATH, () => {
            // 准备请求头
            const keyLen = Buffer.from([key.length]);
            const ivLen = Buffer.from([iv.length]);
            const dataLen = Buffer.alloc(4);
            dataLen.writeUInt32BE(data.length, 0);
            
            // 组合请求数据
            const request = Buffer.concat([key, iv, data]);
            
            // 发送请求
            client.write(Buffer.concat([keyLen, ivLen, dataLen, request]));
        });
        
        // 接收响应
        const chunks = [];
        client.on('data', (chunk) => {
            chunks.push(chunk);
        });
        
        client.on('end', () => {
            const encrypted = Buffer.concat(chunks);
            resolve({ key, iv, encrypted });
        });
        
        client.on('error', (err) => {
            reject(`连接错误: ${err.message}`);
        });
        
        client.on('timeout', () => {
            client.destroy();
            reject('请求超时');
        });
    });
}

// 示例使用
async function main() {
    try {
        // 检查服务器是否运行
        if (!fs.existsSync(SOCKET_PATH)) {
            console.error('错误: 加密服务器未运行');
            process.exit(1);
        }
        
        const plaintext = Buffer.from('使用纯Rust标准库实现的加密');
        console.log('📝 原始数据:', plaintext.toString());
        
        // 加密数据
        const { key, iv, encrypted } = await encryptData(plaintext);
        
        console.log('🔑 密钥:', key.toString('hex'));
        console.log('🔄 IV:', iv.toString('hex'));
        console.log('🔒 加密数据:', encrypted.toString('hex'));
        console.log(`📏 加密结果大小: ${encrypted.length} 字节`);
        
        // 在Node.js中解密（使用相同的算法）
        const decrypted = decryptData(encrypted, key, iv);
        console.log('🔓 解密验证:', decrypted.toString());
    } catch (err) {
        console.error('❌ 错误:', err);
    }
}

// 与Rust服务器相同的解密算法
function decryptData(encrypted, key, iv) {
    const result = Buffer.alloc(encrypted.length);
    const keyStream = createKeyStream(key, iv);
    
    for (let i = 0; i < encrypted.length; i++) {
        result[i] = encrypted[i] ^ keyStream.next().value;
    }
    
    return result;
}

// JavaScript版本的KeyStream
function* createKeyStream(key, iv) {
    const keyArr = new Uint8Array(32);
    const ivArr = new Uint8Array(16);
    
    // 复制密钥和IV
    keyArr.set(key.subarray(0, 32));
    ivArr.set(iv.subarray(0, 16));
    
    let counter = 0;
    
    while (true) {
        const block = new Uint8Array(16);
        
        // 初始化块
        for (let i = 0; i < 16; i++) {
            block[i] = ivArr[i] ^ (counter & 0xFF) + i;
        }
        
        // 应用密钥
        for (let i = 0; i < 32; i++) {
            const idx = i % 16;
            block[idx] = (block[idx] + keyArr[i]) << 3 | (block[idx] + keyArr[i]) >> 5;
        }
        
        // 产生密钥流字节
        for (const byte of block) {
            yield byte;
        }
        
        counter++;
    }
}

main();