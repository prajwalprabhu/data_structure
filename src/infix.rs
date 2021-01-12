//Solve Post fix expression
fn to_result(result: Vec<char>) -> u32 {
    let mut stack: Vec<u32> = Vec::new(); //Stack
    let operator = "*/+-";
    for i in result {
        if operator.contains(i) {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            match i {
                '+' => stack.push(b + a),
                '-' => stack.push(b - a),
                '/' => stack.push(b / a),
                '*' => stack.push(b * a),
                _ => panic!("Something went wrong"),
            }
        } else {
            stack.push(i.to_digit(10).unwrap());
        }
    }
    stack.pop().unwrap()
}
//Program to Convert Infix to Postfix Expression
fn to_post(eq: String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new(); //Stack
    let mut output: Vec<char> = Vec::new(); //Output
    let operator = "*/+-";
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
    let infix = String::from("2*3/(2-1)+5*3");
    let result = to_post(infix.clone());
    println!(" Infix = {:?},PostFix = {:?}", infix, result);
    println!("Result = {:?}", to_result(result));
    let infix = String::from("(A+B)*C-(D-E)*(F+G)");
    let result = to_post(infix.clone());
    println!(" Infix = {:?},PostFix = {:?}", infix, result);
}
