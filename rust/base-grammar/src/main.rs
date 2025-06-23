
use std::{collections::HashMap, fmt, mem}; // 引入标准库中的fmt模块
use regex::Regex;

fn main(){
    // 定义一个变量a并赋值为12
    let a = 12;
    // 打印a的值
    println!("a is {0} {0}",a);

    // 定义一个可变变量b并赋值为4
    let mut b = 4;
    // 将b的值修改为5
    b = 5;
    // 打印b的值
    println!("b is {0} {0}",b);
    // 定义一个变量c并赋值为b
    let c = b;
    // 打印c的值
    println!("c is {0} {0}",c);
    // 打印b的值
    // println!("b is {0} {0}",b);

    // 调用add函数，将1和2作为参数传入，并将返回值赋值给d
    let d = add(1,2);
    // 打印d的值
    println!("d is {0} {0}",d);
    // 判断d是否大于3
    if d > 3 {
        // 如果d大于3，打印"d greater 3"
        println!("d greater 3");
    }else {
        // 如果d小于等于3，打印"d less 3"
        println!("d less 3");
    }

    // 调用fbnqc函数，将15作为参数传入，并将返回值打印出来
    println!("fbnqc(5) is {}",fbnqc(15));
    // 调用jiecheng函数，将5作为参数传入，并将返回值打印出来
    println!("jiecheng(5) is {}",jiecheng(5));

    // 定义一个固定数组arr，并赋值为[1,2,3,4,5]
    let mut arr = [1,2,3,4,5];
    // 遍历数组arr，并打印出每个元素的值
    for i in arr.iter() {
        println!("arr is {0}",i);
    }

    // 定义一个可变变量students，并赋值为一个空的Vec<Student>
    let mut students:Vec<Student> = Vec::new();

    // 向students中添加一个Student对象
    students.push(Student { name: String::from("zhangsan"), age: 10, gender: 1 });
    students.push(Student { name: String::from("zhangsan"), age: 10, gender: 1 });

    // 参数传students为所有权转移，传&students为只读不转移引用，传&mut students为可读可写不转移引用
    arradd(  & students);
    for i in students.iter() {
        println!("student is {} {} {}",i.name,i.age,i.gender);
    }

    // 元祖
    let tmp: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tmp is: {:?}",tmp);

    // 块级表达式,相当于vue的计算属性
    let y = {
        let x = 3;
        x + 1 + d +b
    };
    println!("The value of y is: {0}",y);
    
    // 三元操作符
    let number = if a > 0 { 1 } else { -1 };
    println!("number is {0}",number);

    // while循环
    let mut index:f32 = 5.0;
    while index > 0.0 {
        index -= 1.0;
        println!("index is {0}",index);
    }

    // 数组及数组的一些方法,数组方法要转成迭代器调用,比如map,filter等
    let numbers = vec![1, 2, 3, 4, 5];
    let squared_numbers: Vec<i32> = numbers.iter().map(|x| x*x*x).collect();
    println!("squared_numbers is {:?}",squared_numbers);
    let filtered_vec: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("cc is {:?}",filtered_vec);

    // 闭包
    let add_one = |x: i32| 3*x + x*x + 11;
    println!("add_one is {}",add_one(11));

    // 字符串切片
    let name = "zhang san";
    let name1 = name.to_string();
    let s = String::from("broadcast");
    println!("Hello, {}!", &name[0..5]);
    println!("Hello, {}!", &name1[0..5]);
    println!("Hello, {}!", &s[0..5]);

    // 结构体
    struct Point {
        x: i32,
        y: i32,
    }
    // 结构体序列化实现
    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }
    // 结构体相当于类了,类方法实现
    impl Point {
        // 结构体实例方法
        fn aaa(&self) {
            println!("aaa {}.{}",self.x,self.y);
        }
        // 结构体静态方法
        fn bbb(a:String) {
            println!("bbb {}",a)
        }
    }
    let p = Point { x: 10, y: 20 };
    println!("{:?}", p);
    // 类方法调用,可传入其他实例的指针,相当于this
    p.aaa();
    // 静态方法调用
    Point::bbb(String::from("1324"));

    // 枚举
    enum Book{
        Paper(u32),
        Electronic(String)
    }
    let book = Book::Paper(123);
    let book1 = Book::Electronic(String::from("www.baidu.com"));
    match book {
        Book::Paper(num) => println!("Paper book with id: {}", num),
        Book::Electronic(url) => println!("Electronic book with url: {}", url),
    }
    match book1 {
        Book::Paper(num) => println!("Paper book with id: {}", num),
        Book::Electronic(url) => println!("Electronic book with url: {}", url),
    }

    // Some枚举
    let c = Some(63);
    if let Some(63) = c {
        println!("c is 63");
    }
    match c {
        Some(index) => println!("c is {}",index),
        None => println!("c is None")
    }

    // 数据类型，内存大小，指针
    println!("输出char类型内存占用 {}",mem::size_of::<char>());
    println!("输出u8类型内存占用 {}",mem::size_of::<u8>());
    println!("输出i8类型内存占用 {}",mem::size_of::<i8>());
    println!("输出u16类型内存占用 {}",mem::size_of::<u16>());
    println!("输出u32类型内存占用 {}",mem::size_of::<u32>());
    println!("输出u64类型内存占用 {}",mem::size_of::<u64>());
    println!("输出u128类型内存占用 {}",mem::size_of::<u128>());
    println!("输出f32类型内存占用 {}",mem::size_of::<f32>());
    println!("输出f64类型内存占用 {}",mem::size_of::<f64>());
    println!("输出bool类型内存占用 {}",mem::size_of::<bool>());
    println!("系统带宽位 {}",mem::size_of::<usize>());

    // 特殊类型
    let tup: (i32, f64, char) = (500, 6.4, 'a');
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let mut str = "hello world斗法";
    let mut str1 = "";
    let mut name = String::from("hello world封控");
    let mut name1 = String::from("");
    println!("输出元祖变量内存占用 {}",mem::size_of_val(&tup));
    println!("输出数组变量内存占用 {}",mem::size_of_val(&arr));
    println!("输出字符串变量内存占用 {}",mem::size_of_val(&str));
    println!("输出字符串引用变量内存占用 {}",mem::size_of_val(&&str));
    println!("输出字符串可变引用变量内存占用 {}",mem::size_of_val(&&mut str));
    println!("输出字符串变量内存占用 {}",mem::size_of_val(&name));
    println!("输出字符串引用变量内存占用 {}",mem::size_of_val(&&name));
    println!("输出字符串可变引用变量内存占用 {}",mem::size_of_val(&&mut name));
    println!("输出空串变量内存占用 {}",mem::size_of_val(&str1));
    println!("输出空串变量内存占用 {}",mem::size_of_val(&name1));
    println!("输出字符串变量字符长度 {}",str.len());
    println!("输出字符串变量字符长度 {}",name.len());
    println!("输出字符串截取前12个 {:?}",str.chars().take(12).collect::<String>());
    name.push_str("hello world  ");
    println!("字符串拼接 {}",&name);
    println!("字符串分割 {:?}",name.split("").collect::<Vec<&str>>());
    println!("字符串分割 {:?}",name.split_whitespace().next());
    println!("模版字符串 {}",format!("{}------{}",&str,&name));
    println!("字符串替换 {:?}",name.replace("hello","world"));
    println!("字符串查找 {:?}",name.contains("hello1"));
    println!("字符串查找位置 {:?}",name.find("world"));
    println!("字符串大写 {:?}",name.to_uppercase());
    println!("字符串删空格 {:?}",name.trim());
    println!("字符串转数字 {:?}","58".parse::<u16>().unwrap());
    println!("字符串正则匹配 {:?}",Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap().captures("Today is 2025-06-13").unwrap());

    // 向量
    let mut v1 = vec![1, 2, 3, 4, 5];
    let mut v2 = vec![6, 7, 8, 9, 10];
    v1.push(6);
    println!("输出向量v1 {:?}",v1);
    v1.append(&mut v2);
    println!("输出向量v1 {:?}",v1);
    println!("输出向量v2 {:?}",v2);

    // hashMap
    let mut scores = HashMap::new();
    scores.insert("red", 10);
    scores.insert("blue", 30);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
}

// 求和函数
fn add(a:i32,b:i32) -> i32{
    a+b
}

// 递归
fn fbnqc(n:u32) -> u32{
    if n == 0 {
        0
    }else if n == 1 {
        1
    }else {
        fbnqc(n-1) + fbnqc(n-2)
    }
}

// 阶乘
fn jiecheng(mut n:u32) -> u32{
    let mut result = 1;
    loop {
        result = result * n;
        n -= 1;
        if n == 1 {
            break;
        }
    }
    result
}

struct Student{
    name:String,
    age:i32,
    gender:u8
}

// 数组修改泛型函数头
// fn arradd<T>(arr:& Vec<T>) {
fn arradd(arr:& Vec<Student>) {
    // 数组修改
    // for i in 0..5 {  // 预先获取长度，避免迭代时修改
    //     arr.push(Student {
    //         name: String::from("zhangsan"),
    //         age: 10 + i as i32,
    //         gender: 1
    //     });
    // }
    // 数组读取打印
    for i in arr.iter(){
        println!("student is {} {} {}",i.name,i.age,i.gender);
    }
}
