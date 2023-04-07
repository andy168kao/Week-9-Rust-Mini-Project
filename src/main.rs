use std::io;

fn main() {
    println!("Enter the number of parentheses pairs:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    let res = generate_parenthesis(n);
    println!("All combinations of well-formed parentheses with {} pairs:", n);
    for s in res {
        println!("{}", s);
    }
}

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut curr = String::new();
    backtrack(&mut res, &mut curr, n, n);
    res
}

fn backtrack(res: &mut Vec<String>, curr: &mut String, open: i32, close: i32) {
    if open == 0 && close == 0 {
        res.push(curr.clone());
        return;
    }
    if open > 0 {
        curr.push('(');
        backtrack(res, curr, open - 1, close);
        curr.pop();
    }
    if close > open {
        curr.push(')');
        backtrack(res, curr, open, close - 1);
        curr.pop();
    }
}
