pub fn run(){
    //strings use the following to modify, builds it as a heap allcoated data structure
    let mut hello =String::from("chanakya");

    hello.push_str(" world");
    println!("{}",hello);

    //assertion testing
    assert_eq!(2,hello.len());
}