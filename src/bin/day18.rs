use aoc_2020::read_input_map;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    OpenP,
    CloseP,
    Num(u64),
    Add,
    Mul,
}

fn main() {
    let input = read_input_map(|e| parse(e.chars().collect()));
    // input.iter().for_each(|e| {
    //     println!("{:?}", e);
    // });
    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<Token>]) {
    let answer = input.iter().map(|e| evaluate(e)).sum::<u64>();
    println!("part 1: {}", answer);
}

fn part2(input: &[Vec<Token>]) {
    let mut input = input.to_vec();
    input.iter_mut().for_each(|e| add_parens_for_plus(e));
    let answer = input.iter().map(|e| evaluate(e)).sum::<u64>();
    println!("part 2: {}", answer);
}

fn add_parens_for_plus(expr: &mut Vec<Token>) {
    let mut i = 0;
    while i < expr.len() {
        let token = expr[i];
        match token {
            Token::Add => {
                let mut l = i - 1;
                while l > 0 {
                    match expr[l] {
                        Token::CloseP => {
                            l = open_paren_pos(expr, l - 1).unwrap_or(0);
                            break;
                        }
                        Token::OpenP => {
                            break;
                        }
                        Token::Mul => {
                            l += 1;
                            break;
                        }
                        _ => {}
                    }

                    l -= 1;
                }

                let mut r = i + 1;
                while r < expr.len() {
                    match expr[r] {
                        Token::OpenP => {
                            r = close_paren_pos(expr, r + 1).unwrap_or(expr.len() - 1) + 1;
                            break;
                        }
                        Token::CloseP => {
                            break;
                        }
                        Token::Mul => {
                            break;
                        }
                        _ => {}
                    }
                    r += 1;
                }

                expr.insert(l, Token::OpenP);
                if r + 1 >= expr.len() {
                    expr.push(Token::CloseP);
                } else {
                    expr.insert(r + 1, Token::CloseP);
                }

                i += 1;
            }
            _ => {}
        }

        i += 1;
    }
}

#[allow(dead_code)]
fn print_expr(expr: &[Token]) {
    for i in expr {
        match i {
            Token::OpenP => {
                print!("(");
            }
            Token::CloseP => {
                print!(")");
            }
            Token::Add => {
                print!("+");
            }
            Token::Mul => {
                print!("*");
            }
            Token::Num(n) => {
                print!("{}", n);
            }
        }
    }
    println!()
}

/// Searches to the right.
fn close_paren_pos(expression: &[Token], start: usize) -> Option<usize> {
    let mut open_count = 0;
    for k in start..expression.len() {
        if expression[k] == Token::OpenP {
            open_count += 1;
        } else if expression[k] == Token::CloseP {
            if open_count == 0 {
                return Some(k);
            } else {
                open_count -= 1;
            }
        }
    }

    None
}

/// Searches to the left.
fn open_paren_pos(expression: &[Token], start: usize) -> Option<usize> {
    let mut close_count = 0;
    for k in (0..=start).rev() {
        if expression[k] == Token::CloseP {
            close_count += 1;
        } else if expression[k] == Token::OpenP {
            if close_count == 0 {
                return Some(k);
            } else {
                close_count -= 1;
            }
        }
    }
    None
}

fn evaluate(expression: &[Token]) -> u64 {
    let mut i = 0;
    let mut acc = 0;
    let mut op = None;
    while i < expression.len() {
        let a = expression[i];
        match a {
            Token::OpenP => {
                let close_paren_pos = close_paren_pos(expression, i + 1).unwrap();
                match op {
                    Some(op) => {
                        match op {
                            Token::Add => {
                                acc += evaluate(&expression[i + 1..close_paren_pos]);
                            }
                            Token::Mul => {
                                acc *= evaluate(&expression[i + 1..close_paren_pos]);
                            }
                            _ => unreachable!(),
                        }
                        i = close_paren_pos;
                    }
                    None => {
                        acc = evaluate(&expression[i + 1..close_paren_pos]);
                        i = close_paren_pos;
                    }
                }
                //
            }
            Token::CloseP => {}
            Token::Num(n) => {
                match op {
                    Some(op) => match op {
                        Token::Add => {
                            acc += n;
                        }
                        Token::Mul => {
                            acc *= n;
                        }
                        _ => {}
                    },
                    None => {
                        acc = n;
                    }
                }
                op = None;
            }
            Token::Add | Token::Mul => {
                op = Some(a);
            }
        }

        i += 1;
    }

    acc
}

fn parse(input: Vec<char>) -> Vec<Token> {
    let mut result = vec![];

    let mut i = 0;
    while i < input.len() {
        match &input[i] {
            ' ' => {
                i += 1;
            }
            '(' => {
                result.push(Token::OpenP);
                i += 1;
            }
            ')' => {
                result.push(Token::CloseP);
                i += 1;
            }
            '+' => {
                result.push(Token::Add);
                i += 1;
            }
            '*' => {
                result.push(Token::Mul);
                i += 1;
            }
            _ => {
                let mut k = i + 1;
                while k < input.len() {
                    match input[k] {
                        '0'..='9' => {
                            k += 1;
                        }
                        _ => break,
                    }
                }

                result.push(Token::Num(
                    input[i..k]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                ));

                i = k;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(expr: &str, value: u64) {
        let mut parsed = parse(expr.chars().collect());
        add_parens_for_plus(&mut parsed);
        assert_eq!(value, evaluate(&parsed));
    }

    #[test]
    fn test1() {
        assert("1 + (2 * 3) + (4 * (5 + 6))", 51);
    }

    #[test]
    fn test2() {
        assert("2 * 3 + (4 * 5)", 46);
    }

    #[test]
    fn test3() {
        assert("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445);
    }

    #[test]
    fn test4() {
        assert("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060);
    }

    #[test]
    fn test5() {
        assert("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340);
    }
}
