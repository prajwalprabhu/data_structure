//Solve Post fix expression
fn to_result(result: Vec<char>) -> u32 {
    let mut stack: Vec<u32> = Vec::new(); //Stack
    let operator = "*/+-%^";
    for i in result {
        if operator.contains(i) {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            match i {
                '+' => stack.push(b + a),
                '-' => stack.push(b - a),
                '/' => stack.push(b / a),
                '*' => stack.push(b * a),
                '^' => stack.push(b.pow(a)),
                _ => panic!("Something went wrong"),
            }
        } else {
            stack.push(i.to_digit(10).unwrap());
        }
    }
    stack.pop().unwrap()
}
fn check(eq: String) -> u32 {
    let mut eq: Vec<char> = eq.chars().collect();
    let mut result: u32;
    if eq.contains(&'^') {
        for i in 0..eq.len() - 1 {
            if eq[i] == '^' {
                let a: char = eq[i - 1];
                let b: char = eq[i + 1];
                let a: u32 = a.to_digit(10).unwrap();
                let b: u32 = b.to_digit(10).unwrap();
                result = a.pow(b);
                let result = format!("{}", result);
                // println!("{}",result);
                println!("Checked {:?}", eq);
                eq.remove(i - 1);
                eq.remove(i - 1);
                eq.remove(i - 1);
                // println!("Checked {:?}",eq);
                eq.insert(i - 1, result.chars().next().unwrap());
            }
        }
        // println!("Checked {:?}",eq);
    }
    let mut result = String::new();
    eq.iter().for_each(|i| {
        result.push(*i);
    });
    // let a = String::from_iter(eq.iter());
    to_result(to_post(result))
}
//Program to Convert Infix to Postfix Expression
fn to_post(eq: String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new(); //Stack
    let mut output: Vec<char> = Vec::new(); //Output
    let operator = "*/+-^";
    for i in eq.chars() {
        if i == '(' {
            stack.push(i);
        } else if operator.contains(i) {
            loop {
                if stack.len() == 0 {
                    stack.push(i);
                    break;
                } else if stack[stack.len() - 1] == '(' {
                    stack.push(i);
                    break;
                } else if i == '*' || i == '/' {
                    if stack[stack.len() - 1] == '+' || stack[stack.len() - 1] == '-' {
                        stack.push(i);
                        break;
                    } else {
                        output.push(stack.pop().unwrap());
                        stack.push(i);
                    }
                    break;
                } else {
                    output.push(stack.pop().unwrap());
                }
            }
        } else if i == ')' {
            // println!("{}",eq.Index(i));
            while stack.len() != 0 {
                let a = stack.pop().unwrap();
                if a == '(' {
                    break;
                }
                output.push(a);
            }
        } else {
            output.push(i);
        }

        println!(" Stack = {:?} , output is {:?}", stack, output);
    }
    for i in stack.iter().rev() {
        output.push(*i);
    }
    output
}
pub fn run() {
    let infix = String::from("2*3/(4-1)+5*3^2");
    // let result = to_post(infix.clone());
    println!("infix = {:?}\nPostfix = {:?}",infix, check(infix.clone()));
    // println!(" Infix = {:?},PostFix = {:?}", infix, result);
    // println!("Result = {:?}", to_result(result));
    // let infix = String::from("(A+B)*C-(D-E)*(F+G)");
    // let result = to_post(infix.clone());
    // println!(" Infix = {:?},PostFix = {:?}", infix, result);
}
