use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
fn spawn_function(index:u16,task:u16) {
    let time = rand(10000);
    thread::sleep(Duration::from_millis(time));
    println!("thread {} print {} time {}", index, task,time);
}

// 定义一个名为start的函数
pub fn start(num: Option<u16>) {
    let mut snum:u16 = if Some(num) == None {50000 } else { num.unwrap()};
    let mut taskQueue:Vec<u16> = (1..=snum).collect();
    let mut jj = Vec::new();
    let l = taskQueue.len() as u16;
    
    // 创建五万个线程测试   
    loop {
        let len = taskQueue.len() as u16;
        let task = taskQueue.pop().unwrap();
        jj.push(thread::spawn(move || {
            spawn_function(l-len,task);
        }));
        if taskQueue.len() == 0 {
            break;
        }
    }

    // 阻塞等待线程执行完毕
    for i in jj {
        i.join().unwrap();
    }
}

// 获取随机数
fn rand(num: u128) -> u64 {
    // 获取当前时间
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    // 映射到 0-500 范围
    (now % (num+1)) as u64
}