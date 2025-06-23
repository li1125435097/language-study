// 导出一个模拟orm的模块
pub mod Orm{
    pub struct Handle{
        connid:i32,
        single:bool
    }

    impl Handle{
        pub fn new(connid:i32,single:bool)->Handle{
            Handle{
                connid,
                single
            }
        }

        pub fn create(&self,a:i32)->i32{
            println!("create {}",self.connid);
            a+1
        }
    
        pub fn del(&self,id:i32)->i32{
            println!("del {}",self.connid);
            id
        }
    
        pub fn update(&self,a:i32)->i32{
            println!("update {}",self.connid);
            a
        }
    
        pub fn read(&self,id:i32)->i32{
            println!("read {}",self.connid);
            id
        }

    }
    
}


