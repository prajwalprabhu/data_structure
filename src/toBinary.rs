pub fn run(a: i32) {
    let mut stack: Vec<i32> = Vec::new(); //Stack
    let mut a = a;
    while a >= 1 {
        let rem = a % 2;
        a /= 2;
        stack.push(rem);
    }
    let mut n = stack.len();
    if n % 2 == 0 {
        n /= 2;
    } else {
        n = (n - 1) / 2;
    }
    let nn = stack.len();
    for i in 0..n {
        stack.swap(i, nn - 1 - i);
    }
    println!("Result {:?}", stack);
}
