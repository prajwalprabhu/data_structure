pub fn convert(a: i32) -> Vec<i8> {
    let mut stack = Vec::<i8>::new(); //Stack
    let mut a = a;
    if a == 0 {
        return vec![0];
    }
    while a >= 1 {
        let rem: i8 = (a % 2) as i8;
        a /= 2;
        stack.push(rem);
    }
    stack = stack.into_iter().rev().collect();
    stack
    // println!("Result {:?}", stack);
}

#[cfg(test)]
mod test_binary {
    use super::*;

    #[test]
    fn to_binary() {
        let inputs = [0, 2, 5, 44, 45];
        let outputs = [
            vec![0],
            vec![1, 0],
            vec![1, 0, 1],
            vec![1, 0, 1, 1, 0, 0],
            vec![1, 0, 1, 1, 0, 1],
        ];
        assert_eq!(inputs.len(), outputs.len());
        for i in 0..inputs.len() {
            assert_eq!(convert(inputs[i]), outputs[i]);
        }
    }
}
