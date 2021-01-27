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
                // '^' => stack.push(b.pow(a)),
                _ => panic!("Something went wrong"),
            }
        } else {
            stack.push(i.to_digit(10).unwrap());
        }
    }
    stack.pop().unwrap()
}
//Checks whether there is any power equatio in goven equatio i.e 2^3=8 or (1+1)^3=8
fn eval(eq: String) -> u32 {
    println!("eq = {:?}", eq);
    let mut eq: Vec<char> = eq.chars().collect();
    let result: u32;
    let a: char;
    let b: char;
    let mut eq2: Vec<char> = Vec::new();
    if eq.contains(&'^') {
        for i in 0..eq.len() - 1 {
            if eq[i] == '^' {
                a = eq[i - 1];

                b = eq[i + 1];
                if a == ')' {
                    let mut j = i - 2;

                    let mut open = 0;
                    eq.remove(i - 1);
                    while j > 0 {
                        if eq[j] == ')' {
                            open += 1;
                        }
                        if eq[j] == '(' {
                            if open > 0 {
                                open -= 1;
                            } else {
                                eq.remove(j);

                                break;
                            }
                        }
                        eq2.insert(0, eq.remove(j));

                        j -= 1;
                    }
                    let mut eq2_str = String::new();
                    eq2.iter().for_each(|x| eq2_str.push(*x));
                    let res2 = eval(eq2_str);
                    let res2 = format!("{}", res2);
                    eq.insert(j, res2.chars().next().unwrap());

                    break;
                } else {
                    let a: u32 = a.to_digit(10).unwrap();
                    let b: u32 = b.to_digit(10).unwrap();
                    result = a.pow(b);
                    let result = format!("{}", result);
                    eq.remove(i - 1);
                    eq.remove(i - 1);
                    eq.remove(i - 1);
                    eq.insert(i - 1, result.chars().next().unwrap());

                    break;
                }
            }
        }
    }
    let mut result = String::new();
    eq.iter().for_each(|i| {
        result.push(*i);
    });
    if eq.contains(&'^') {
        return eval(result);
    }
    to_result(to_post(result))
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
    for _ in 0..stack.len() {
        output.push(stack.pop().unwrap());
    }

    println!(" Stack = {:?} , output is {:?}", stack, output);
    output
}
pub fn run() {
    // let infix = String::from("2*3/(
    // 4-1)^2+5*3");
    let infix = String::from("2+3*4+(3-(4-(2-(2-1))))^2");
    // let infix = String::from("1+23");
    // let infix = String::from("2*3^2/(4-1)+5*3");
    // let result = to_post(infix.clone());
    println!("infix = {:?}\nPostfix = {:?}", infix, eval(infix.clone()));
    // println!(" Infix = {:?},PostFix = {:?}", infix, result);
    // println!("Result = {:?}", to_result(result));
    // let infix = String::from("(A+B)*C-(D-E)*(F+G)");
    // let result = to_post(infix.clone());
    // println!(" Infix = {:?},PostFix = {:?}", infix, result);
}
