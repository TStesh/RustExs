mod calc2;
mod llist;
mod life;
use crate::calc2::Calc;
use crate::life::Board;
use crate::llist::LList;

fn main() {

    let mx = Matrix::new();
    let path = mx.path();
    println!("path max len = {}", path[path.len() - 1]);

    /*
    let tr = Triangle::new();

    let (xs, ys) = tr.path();
    let path_max_len = *xs[tr.size() - 1].iter().max().unwrap();
    println!("path max len = {path_max_len}");

    let mut b = Board::new(20, 10);

    b.set(1, 2);
    b.set(2, 3);
    b.set(3, 1);
    b.set(3, 2);
    b.set(3, 3);
    println!("{b}");

    for _ in 0..10 {
        b.next();
        println!("{b}");
    }

    let mut ll = LList::new(5);

    let mut x = 10;
    while !ll.full() {
        ll.add(x).unwrap();
        x += 1;
    }
    println!("{ll}");

    // прямой обход списка
    println!("Forward:");
    ll.start_ptr();
    while !ll.end() {
        println!("{}", ll.elem());
        ll.next();
    }

    // обратный обход списка
    println!("\nBack:");
    ll.end_ptr();
    while !ll.end() {
        println!("{}", ll.elem());
        ll.prev();
    }

    let s = "log(7)/log(2)";
    println!("{s}");
    let mut xs = Calc::new(s);
    let r = xs.calc_expr();
    println!("Answer: {:?}", r);
    */
}
