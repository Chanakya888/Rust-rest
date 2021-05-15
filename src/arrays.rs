pub fn run(){
    let numbers : [i32;2]=[1,2];
    let slice : &[i32] = &numbers[0..2];
    println!("{:?}",slice)
}