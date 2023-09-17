// Грамматика арифметических формул с поддержкой функций
// -----------------------------------------------------
// expr := [plusminus]*
// plusminus := muldiv [('+' | '-') muldiv]*
// muldiv := multiplier [('*' | '/') multiplier]*
// multiplier := factor ['^' factor]*
// factor := ['-']? pfactor
// pfactor := NUMBER | function | '(' expr ')'
// function := FUNCNAME '(' [args]* ')'
// args := expr | [',' expr]*

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum TokenType {
    LeftBracket, RightBracket, OpPlus, OpMinus,
    OpMul, OpDiv, Caret, NUMBER, FUNCNAME, Comma, EOF
}

#[derive(Debug)]
struct Token {
    lex_type: TokenType,
    lex_num_value: f64,
    lex_str_value: String
}

impl Token {
    fn null() -> Self { Self { lex_type: TokenType::EOF,
        lex_num_value: 0., lex_str_value: "".to_string() } }
    fn oper(t: TokenType) -> Self { Self { lex_type: t,
        lex_num_value: 0., lex_str_value: "".to_string() } }
    fn num(n: f64) -> Self { Self { lex_type: TokenType::NUMBER,
        lex_num_value: n, lex_str_value: "".to_string() } }
    fn func(s: &String) -> Self { Self { lex_type: TokenType::FUNCNAME,
        lex_num_value: 0., lex_str_value: s.to_string() } }
}

#[derive(Debug)]
struct Formula {
    items: Vec<Token>
}

impl Formula {
    // Лексический анализ
    // Разбиваем выражение на лексемы
    // Проверяем на корректность числа, баланс скобок
    fn new(s: &str) -> Self {
        let mut items = vec![];
        let mut bn = 0;
        let mut i = 0;
        while i < s.len() {
            let xc = FromStr::from_str(&s[i..i + 1]);
            if xc.is_err() {
                println!("Syntax error: {:?}", xc.err().unwrap());
                return Self { items: vec![Token::null()] }
            }
            let c = xc.unwrap();
            let mut t = Token::null();
            match c {
                '+' => t = Token::oper(TokenType::OpPlus),
                '-' => t = Token::oper(TokenType::OpMinus),
                '*' => t = Token::oper(TokenType::OpMul),
                '/' => t = Token::oper(TokenType::OpDiv),
                '^' => t = Token::oper(TokenType::Caret),
                '(' => {
                    bn += 1;
                    t = Token::oper(TokenType::LeftBracket);
                },
                ')' => {
                    bn -= 1;
                    if bn < 0 {
                        println!("Error: invalid symbol ')' in the position {}", i + 1);
                        return Self { items: vec![Token::null()] }
                    }
                    t = Token::oper(TokenType::RightBracket);
                },
                '0'..='9' | '.' => {
                    let mut num_val = c.to_string();
                    i += 1;
                    while i < s.len() {
                        let xc = FromStr::from_str(&s[i..i + 1]);
                        if xc.is_err() {
                            println!("Syntax error: {:?}", xc.err().unwrap());
                            return Self { items: vec![Token::null()] }
                        }
                        let c = xc.unwrap();
                        match c {
                            '0'..='9' | '.' => {
                                num_val.push_str(c.to_string().as_str());
                                i += 1;
                                continue
                            },
                            ' ' => { i += 1; continue },
                            _ => {
                                i -= 1;
                                break;
                            }
                        }
                    }
                    if let Ok(x) = num_val.parse::<f64>() {
                        t = Token::num(x);
                    } else {
                        println!("Syntax error: invalid number '{num_val}'");
                        return Self { items: vec![Token::null()] }
                    }
                },
                ',' => t = Token::oper(TokenType::Comma),
                'a'..='z' | 'A'..='Z' => {
                    let mut fname = c.to_string();
                    i += 1;
                    while i < s.len() {
                        let xc = FromStr::from_str(&s[i..i + 1]);
                        if xc.is_err() {
                            println!("Syntax error: {:?}", xc.err().unwrap());
                            return Self { items: vec![Token::null()] }
                        }
                        let c = xc.unwrap();
                        match c {
                            'a'..='z' | 'A'..='Z' => {
                                fname.push_str(c.to_string().as_str());
                                i += 1;
                                continue
                            },
                            ' ' => { i += 1; continue },
                            _ => {
                                i -= 1;
                                break;
                            }
                        }
                    }
                    t = Token::func(&fname);
                },
                ' ' => { i += 1; continue; },
                _ => {
                    println!("Syntax error: unknown symbol '{c}' in the position {}", i + 1);
                    return Self { items: vec![Token::null()] }
                }
            }
            items.push(t);
            i += 1;
        }
        // проверяем баланс скобок
        if bn != 0 {
            println!("Error: invalid symbol '('");
            return Self { items: vec![Token::null()] }
        }
        items.push(Token::null());
        Self { items }
    }
}

pub struct Calc {
    tokens: Formula,
    pos: usize
}

impl Calc {
    pub fn new(s: &str) -> Self {
        let tokens = Formula::new(s);
        Self { tokens, pos: 0 }
    }

    pub fn calc_expr(&mut self) -> f64 {
        if self.tokens.items[self.pos].lex_type != TokenType::EOF {
            self.calc_plusminus()
        } else { 0. }
    }

    fn calc_plusminus(&mut self) -> f64 {
        let mut x = self.calc_muldiv();
        loop {
            match self.tokens.items[self.pos].lex_type {
                TokenType::OpPlus => {
                    self.pos += 1;
                    x += self.calc_muldiv();
                },
                TokenType::OpMinus => {
                    self.pos += 1;
                    x -= self.calc_muldiv();
                },
                _ => return x
            }
        }
    }

    fn calc_muldiv(&mut self) -> f64 {
        let mut x = self.calc_multiplier();
        loop {
            match self.tokens.items[self.pos].lex_type {
                TokenType::OpMul => {
                    self.pos += 1;
                    x *= self.calc_multiplier();
                },
                TokenType::OpDiv => {
                    self.pos += 1;
                    x /= self.calc_multiplier();
                },
                _ => return x
            }
        }
    }

    fn calc_multiplier(&mut self) -> f64 {
        let mut x = self.calc_factor();
        loop {
            match self.tokens.items[self.pos].lex_type {
                TokenType::Caret => {
                    self.pos += 1;
                    x = x.powf(self.calc_factor());
                }
                _ => return x
            }
        }
    }

    fn calc_factor(&mut self) -> f64 {
        if self.tokens.items[self.pos].lex_type == TokenType::OpMinus {
            self.pos += 1;
            -self.calc_pfactor()
        } else {
            self.calc_pfactor()
        }
    }

    fn calc_pfactor(&mut self) -> f64 {
        match self.tokens.items[self.pos].lex_type {
            TokenType::NUMBER => {
                let x = self.tokens.items[self.pos].lex_num_value;
                self.pos += 1;
                return x
            },
            TokenType::FUNCNAME => {
                // function := FUNCNAME '(' [args]* ')'
                // args := factor | [',' factor]*
                let fname = self.tokens.items[self.pos].lex_str_value
                    .to_ascii_lowercase().clone();
                let mut args = vec![];
                self.pos += 1;
                if self.tokens.items[self.pos].lex_type == TokenType::LeftBracket {
                    self.pos += 1;
                    if self.tokens.items[self.pos].lex_type != TokenType::RightBracket {
                        loop {
                            args.push(self.calc_expr());
                            if self.tokens.items[self.pos].lex_type == TokenType::Comma {
                                self.pos += 1;
                            } else {
                                break;
                            }
                        }
                    }
                    if self.tokens.items[self.pos].lex_type == TokenType::RightBracket {
                        self.pos += 1;
                        // Вычисляем функцию
                        return self.calc_func(fname.as_str(), args);
                    } else {
                        self.out(0)
                    }
                } else {
                    self.out(0)
                }
            }
            TokenType::LeftBracket => {
                self.pos += 1;
                let x = self.calc_expr();
                if self.tokens.items[self.pos].lex_type == TokenType::RightBracket {
                    self.pos += 1;
                    return x
                } else {
                    self.out(0)
                }
            },
            _ => self.out(0)
        }
    }

    // ["pow", "ln", "log", "sqr", "e", "sin", "cos", "tan"]
    fn calc_func(&self, fname: &str, args: Vec<f64>) -> f64 {
        match fname {
            // --> Функции с произвольным количеством аргументов
            "min" => {
                if args.len() == 0 { self.out1(fname, usize::MAX, 0, 0); }
                let mut m = args[0];
                for x in args { if x < m { m = x; } }
                m
            },
            "max" => {
                if args.len() == 0 { self.out1(fname, usize::MAX, 0, 0); }
                let mut m = args[0];
                for x in args { if x > m { m = x; } }
                m
            },
            "avg" => {
                if args.len() == 0 { self.out1(fname, usize::MAX, 0, 0); }
                args.iter().sum::<f64>() / args.len() as f64
            },
            // --> Функции с двумя аргументами
            "pow" => {
                if args.len() != 2 { self.out1(fname, 2, args.len(), 0); }
                args[0].powf(args[1])
            },
            // --> Функции с одним аргументом
            "sqr" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].powf(0.5)
            },
            "exp" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].exp()
            },
            "ln" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].ln()
            },
            "log" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].log10()
            },
            "sin" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].sin()
            },
            "cos" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].cos()
            },
            "tan" => {
                if args.len() != 1 { self.out1(fname, 1, args.len(), 0); }
                args[0].tan()
            },
            // --> Функции без аргументов
            "pi" => {
                if args.len() > 0 { self.out1(fname, 0, args.len(), 0); }
                std::f64::consts::PI
            },
            "e" => {
                if args.len() > 0 { self.out1(fname, 0, args.len(), 0); }
                std::f64::consts::E
            },
            _ => {
                println!("Syntax error: unknown function '{fname}'");
                std::process::exit(0);
            }
        }
    }

    fn out(&self, c: i32) -> f64 {
        let xs = format!("Ошибка в позиции {}: invalid token {:?}",
                             self.pos, self.tokens.items[self.pos].lex_type);
        let ys = if self.tokens.items[self.pos].lex_type == TokenType::NUMBER {
            format!("('{}')", self.tokens.items[self.pos].lex_num_value)
        } else {
            if self.tokens.items[self.pos].lex_str_value.len() > 0 {
                format!(" ('{}')", self.tokens.items[self.pos].lex_str_value)
            } else {
                "".to_string()
            }
        };
        println!("{xs}{ys}");
        std::process::exit(c);
    }

    fn out1(&self, fname: &str, n: usize, m: usize, c: i32) -> f64 {
        if n != usize::MAX {
            println!("Error: invalid argument's list for the function '{fname}'");
            println!("(must be {n} arguments, but was taken {m} arguments)");
        } else {
            println!("Error: empty argument's list for the function '{fname}'");
        }
        std::process::exit(c);
    }
}
