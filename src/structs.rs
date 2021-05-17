    struct Color{
        red:u8,
        green:u8,
        blue:u8
    }
    struct Person{
        first_name:String,
        last_name:String
    }

    impl Person{
        fn new(first:&str,last:&str)->Person{
            Person{
                first_name:first.to_string(),
                last_name:last.to_string()
                
            }

        }
    }
    pub fn run(){
    let c = Color{
    red:255,
    blue:0,
    green:0
    };
    println!("{}",c.red)
    }