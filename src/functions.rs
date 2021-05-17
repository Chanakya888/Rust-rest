pub fn run(){
greeting("chan","yo");
println!("{}",add(2, 2));
}

pub fn greeting(name:&str,greet:&str){
    println!("{} {}",name,greet)

}

pub fn add(n1:i32,n2:i32)->i32{
    n1+n2
}