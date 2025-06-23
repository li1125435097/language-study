// 导出带外包装的模块
pub mod db{
    mod conn{
        pub fn connect(id:i32){
            println!("connect to db {}",id);
        }
    }
    mod pool{
        pub fn poo(id:i32){
            println!("connect to pool {}",id);
        }
    }
    pub mod handle{
        pub fn hand(id:i32){
            println!("connect to handle {}",id);
            super::conn::connect(id);
            super::pool::poo(id);
        }
    }
}