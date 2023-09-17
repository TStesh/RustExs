/// Simple calculator
/// Supported operations: +, -, *, /, ^ (power)
/// May use (, ), unary minus

#[derive(Debug, Copy, Clone)]
struct Item {
    typ: u8,    // тип: 0 = число, 1 = операция, 2 = скобка
    code: u8,   // код операции или тип скобки - 0 ('(')или 1 (')')
    val: f64    // значение числа
}

impl Item {
    fn num(n: f64) -> Self { Self { typ: 0, code: 0, val: n} }
    fn opr(p: &Parser, c: char) -> Self { Self { typ: 1, code: p.oper_val(c), val: 0.} }
    fn pro() -> Self { Self { typ: 2, code: 0, val: 0.} }
    fn prc() -> Self { Self { typ: 2, code: 1, val: 0.} }
}

#[derive(Debug, PartialEq)]
enum Mode { START, NUM, OPER, PRO, PRC }

struct Parser {
    items: Vec<Item>,           // вектор распарсенных элементов
    opers: [char; 6],           // операции
    parentheses: Vec<u8>,       // скобки
    mode: Mode,                 // режим
    curr_num: String,           // текущее число
    curr_pos: usize,            // текущая позиция (начинаем считать с 1)
    curr_char: char,            // текущий символ
    unary_minus: bool,          // флажок унарного минуса
    err_code: u8                // код ошибки: 0 = нет ошибки
}

impl Parser {
    fn new() -> Self {
        Self {
            items: vec![],
            // операции приведены в порядка возрастания приоритета
            // + имеет приоритет меньше -, потому что в противном
            // случае в выражении 1-1+2 сначала будет выполнена
            // операция 1+2=3, а потом 1-3=-2
            opers: ['_', '+', '-', '/', '*', '^'],
            parentheses: vec![],
            mode: Mode::START,
            curr_num: "0".to_string(),
            curr_pos: 0,
            curr_char: '_',
            unary_minus: false,
            err_code: 0
        }
    }

    fn oper_val(&self, c: char) -> u8 {
        for (i, t) in self.opers.iter().enumerate() {
            if c == *t { return i as u8 }
        }
        0
    }

    fn val_oper(&self, v: u8) -> char {
        self.opers[v as usize]
    }

    fn err(&self, code: u8) {
        let err_msg = match self.err_code {
            1 => format!("invalid symbol '{}' in the position {}",
                         self.curr_char, self.curr_pos + 1),
            2 => format!("'{}' is invalid number", self.curr_num),
            3 => format!("something wrong with the parentheses"),
            _ => format!("unknown error")
        };
        println!("Syntax error #{code}: {err_msg}");
    }

    fn prev_op(&self) -> char {
        self.val_oper(self.items.last().unwrap().code)
    }

    fn get_num(&mut self) {
        match self.curr_num.parse::<f64>() {
            Ok(xs) => self.items.push(Item::num(xs)),
            _ => self.err_code = 2
        }
    }

    // Разбор '-'
    fn parse_sub(&mut self) {
        match self.mode {
            Mode::START | Mode::PRO => {
                if self.unary_minus {
                    self.err_code = 1;
                    return
                }
                // добавляем в начало выражения '0-'
                self.items.push(Item::num(0.));
                self.items.push(Item::opr(self, self.curr_char));
                // для защиты от --, (--, ...
                self.unary_minus = true;
            },
            Mode::NUM => {
                self.get_num();
                if self.err_code == 0 {
                    self.items.push(Item::opr(self, self.curr_char));
                    self.mode = Mode::OPER;
                }
            },
            Mode::OPER => {
                let op = self.prev_op();
                if self.unary_minus || op == '+' || op == '-' {
                    self.err_code = 1;
                    return
                }
                self.items.push(Item::num(-1.));
                self.items.push(Item::opr(self, '*'));
                self.unary_minus = true;
            },
            _ => self.err_code = 1
        }
    }

    // Разбор других операций: +, *, /, )
    fn parse_oper(&mut self) {
        // если это первый элемент или если предыдущий элемент - операция,
        // или открывающая скобка, то возвращаем синтаксическую ошибку
        if self.mode == Mode::START || self.mode == Mode::OPER || self.mode == Mode::PRO {
            self.err_code = 1;
            return
        }
        if self.mode == Mode::NUM {
            self.get_num();
            if self.err_code == 2 { return }
        }
        if self.curr_char == ')' {
            if self.parentheses.len() == 0 {
                self.err_code = 1;
                return
            }
            self.parentheses.pop().unwrap();
            self.items.push(Item::prc());
            self.mode = Mode::PRC;
        } else {
            self.items.push(Item::opr(self, self.curr_char));
            self.mode = Mode::OPER;
        }
    }

    // Разбор '('
    fn parse_pro(&mut self) {
        if self.mode == Mode::NUM || self.mode == Mode::PRC {
            self.err_code = 1;
            return
        }
        self.items.push(Item::pro());
        if self.unary_minus { self.unary_minus = false; }
        self.parentheses.push(1);
        self.mode = Mode::PRO;
    }

    // Разбор цифры
    fn parse_digit(&mut self) {
        if self.mode != Mode::NUM {
            if self.mode == Mode::PRC {
                self.err_code = 1;
                return
            }
            self.mode = Mode::NUM;
            self.curr_num = self.curr_char.to_string();
            if self.unary_minus { self.unary_minus = false; }
        } else {
            self.curr_num.push_str(
                self.curr_char.to_string().as_str()
            );
        }
    }

    // Разбор точки
    fn parse_dot(&mut self) {
        if self.mode != Mode::NUM {
            self.err_code = 1;
            return
        }
        self.curr_num.push_str(".");
    }

    fn parse_expr(&mut self, s: &str) {
        for (i, c) in s.chars().enumerate() {
            self.curr_char = c;
            self.curr_pos = i;
            match c {
                '-' => self.parse_sub(),
                '+' | '*' | '/' | '^' | ')' => self.parse_oper(),
                '(' => self.parse_pro(),
                '0'..='9' => self.parse_digit(),
                '.' => self.parse_dot(),
                ' ' => continue,
                _ => self.err_code = 1
            }
            if self.err_code > 0 { return }
        }
        if self.mode == Mode::NUM { self.get_num(); }
        if self.parentheses.len() > 0 { self.err_code = 3; }
    }
}

struct Calculator {
    parser: Option<Parser>,
    expr: Vec<Item>
}

impl Calculator {
    fn new(s: &str) -> Self {
        let mut parser = Parser::new();
        parser.parse_expr(s);
        if parser.err_code > 0 {
            parser.err(parser.err_code);
            return Self { parser: None, expr: vec![] }
        }
        if parser.items.len() == 0 {
            println!("Nothing to calculate!");
            return Self { parser: None, expr: vec![] }
        }
        Self {
            parser: Some(parser),
            expr: parser.items.clone()
        }
    }

    fn find_max_val_op(&self) -> (u8, usize) {
        let (mut m_code, mut pos) = (0, 0);
        for (i, item) in self.expr.iter().enumerate() {
            if *item.typ == 1 && *item.code > m_code {
                (m_code, pos) = (*item.code, i)
            }
        }
        (m_code, pos)
    }

    fn calc_single_expr(&mut self) -> Option<f64> {
        if self.expr.len() == 0 { return None }
        if self.expr.len() == 1 { return Some(self.expr[0].val) }
        // вычисления выполняются для операций с наибольшим значением val
        // задачу решаем рекурсивно
        // находим операцию с наибольшим приоритетом
        let (v, k) = self.find_max_val_op();
        // Выполняем операцию:
        let s = self.expr.as_slice();
        let lv = s[k - 1].val;
        let rv = s[k + 1].val;
        let r = match self.parser.unwrap().val_oper(v) {
            '^' => {
                if lv == 0. && rv == 0. {
                    println!("Warning: value is not defined / = 1");
                }
                lv.powf(rv)
            },
            '*' => lv * rv,
            '/' => {
                if rv == 0. {
                    println!("Division by zero!");
                    return None
                }
                lv / rv
            },
            '+' => lv + rv,
            '-' => lv - rv,
            _ => 0.
        };
        // Формируем новый вектор
        self.expr = s[..k - 1].to_vec();
        self.expr.push(Item::num(r));
        self.expr.append(&mut s[k + 2..].to_vec());
        self.calc_single_expr()
    }

    // Option, потому что скобок может не быть
    fn find_parentheses(&self) -> Option<(usize, usize)> {
        let (mut b_pos, mut e_pos) = (0, 0);
        let mut is_parentheses = false;
        for (i, item) in self.expr.iter().enumerate() {
            if item.typ == 2 {
                is_parentheses = true;
                if item.code == 0 { b_pos = i; }
                if item.code == 1 { e_pos = i; break }
            }
        }
        if is_parentheses { Some((b_pos, e_pos)) } else { None }
    }

    fn calc_expr(&mut self) -> Option<f64> {
        // находим ближайшую пару скобок
        if let Some((bp, ep)) = self.find_parentheses() {
            // вычисляем выражение внутри скобок
            self.expr = 
            if let Some(r) = self.calc_single_expr(p,&s[bp + 1..ep]) {
                // Заменяем скобки на вычисленное
                // выражение и повторяем расчет
                let mut xs = s[..bp].to_vec();
                xs.push(Item{ typ: 0, code: 0, val: r });
                xs.append(&mut s[ep + 1..].to_vec());
                calc_expr(p,xs.as_slice())
            } else {
                None
            }
        } else {
            calc_single_expr(p, s)
        }
    }
}

/*
fn find_max_val_op(v: &[Item]) -> (u8, usize) {
    let (mut m_code, mut pos) = (0, 0);
    for (i, item) in v.iter().enumerate() {
        if item.typ == 1 && item.code > m_code {
            (m_code, pos) = (item.code, i)
        }
    }
    (m_code, pos)
}

// Вычисляем выражение, в котором нет скобок
// Option - в случае деления на 0
fn calc_single_expr(p: &Parser, s: &[Item]) -> Option<f64> {
    if s.len() == 0 { return None }
    if s.len() == 1 {
        // в синтаксически верном выражении это может быть только число
        return Some(s[0].val)
    }
    // вычисления выполняются для операций с наибольшим значением val
    // задачу решаем рекурсивно
    // находим операцию с наибольшим приоритетом
    let (v, k) = find_max_val_op(s);
    // Выполняем операцию:
    let lv = s[k - 1].val;
    let rv = s[k + 1].val;
    let r = match p.val_oper(v) {
        '^' => {
            if lv == 0. && rv == 0. {
                println!("Warning: value is not defined / = 1");
            }
            lv.powf(rv)
        },
        '*' => lv * rv,
        '/' => {
            if rv == 0. {
                println!("Division by zero!");
                return None
            }
            lv / rv
        },
        '+' => lv + rv,
        '-' => lv - rv,
        _ => 0.
    };
    // Формируем новый вектор
    let mut xs = s[..k - 1].to_vec();
    xs.push(Item{ typ: 0, code: 0, val: r });
    xs.append(&mut s[k + 2..].to_vec());
    calc_single_expr(p, xs.as_slice())
}

// Option, потому что скобок может не быть
fn find_parentheses(s: &[Item]) -> Option<(usize, usize)> {
    let (mut b_pos, mut e_pos) = (0, 0);
    let mut is_parentheses = false;
    for (i, item) in s.iter().enumerate() {
        if item.typ == 2 {
            is_parentheses = true;
            if item.code == 0 { b_pos = i; }
            if item.code == 1 {
                e_pos = i;
                break
            }
        }
    }
    if is_parentheses {
        Some((b_pos, e_pos))
    } else {
        None
    }
}

fn calc_expr(p: &Parser, s: &[Item]) -> Option<f64> {
    // находим ближайшую пару скобок
    if let Some((bp, ep)) = find_parentheses(s) {
        // вычисляем выражение внутри скобок
        if let Some(r) = calc_single_expr(p,&s[bp + 1..ep]) {
            // Заменяем скобки на вычисленное
            // выражение и повторяем расчет
            let mut xs = s[..bp].to_vec();
            xs.push(Item{ typ: 0, code: 0, val: r });
            xs.append(&mut s[ep + 1..].to_vec());
            calc_expr(p,xs.as_slice())
        } else {
            None
        }
    } else {
        calc_single_expr(p, s)
    }
}
*/

pub fn parse_and_calc_expr(s: &str) -> Option<f64> {
    let mut parser = Parser::new();
    parser.parse_expr(s);
    if parser.err_code > 0 {
        parser.err(parser.err_code);
        return None
    }
    if parser.items.len() == 0 {
        println!("Nothing to calculate!");
        return None
    }
    calc_expr(&parser, parser.items.as_slice())
}

/*
mod calc;
use crate::calc::parse_and_calc_expr;

fn main() {
    let expr = "0^0";
    println!("{expr}");
    let r = parse_and_calc_expr(expr);
    if r.is_some() {
        println!("Answer: {}", r.unwrap());
    }
}
*/
