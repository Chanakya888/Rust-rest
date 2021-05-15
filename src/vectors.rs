pub fn run(){
    let mut number : Vec<i32> = vec![1,2,3,4];

    number.push(3);
    println!("numbers are {:?}",number);

    //loop through values
    for value in number.iter(){
        println!("{}",value)
    }

    //loop and mutate values
    for value in number.iter_mut(){
        *value *= 2

    }
    println!("{:?}",number)
}