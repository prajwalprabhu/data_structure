pub fn run(a:i32){
    let mut stack: Vec<i32>= Vec::new(); //Stack
    let mut output: Vec<i32> = Vec::new(); //Output
    let b=a;
    let mut a = a;
    while a>=1{
        let rem = a%2;
        a=a/2;
        stack.push(rem);

    }
    for _ in 0..stack.len(){
        output.push(stack.pop().unwrap());
    }
    
    println!(" Binary equivalent of  {} is {:?} ", b,output);

}