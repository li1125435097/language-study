use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;
// 字符串读取
pub fn getfile(filename: &str) -> String {
    // let txt = fs::read_to_string(filename).expect("file not found");
    let txt = fs::read_to_string(filename).expect(filename);
    txt
}

// 二进制读取
pub fn getfileBuf(filename: &str) -> Vec<u8> {
    let buf = fs::read(filename).expect("file not found");
    buf
}

// 流读取
pub fn getfileChunk(filename: &str, buf:&mut [u8]) {
    let mut file = fs::File::open(filename).unwrap();
    file.read(buf).unwrap();
}

// 在原文基础上写入
pub fn writeFile(filename: &str, data: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        // .append(true)  // 追加写入
        .open(filename)
        .unwrap();
    file.write(data.as_bytes()).unwrap();
}

// 覆盖性写入
pub fn coverFile(filename: &str, data: &str) {
    fs::write(filename, data).unwrap();
}