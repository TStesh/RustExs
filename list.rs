use std::fmt::{Display, format, Formatter};
use std::ops::Deref;
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct List<T> {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T> {
    item: T,
    next: Link<T>
}

impl<T> List<T> {
    fn new() -> Self { Self { head: None } }
    // Добавляет элемент в голову списка
    fn prepend(&self, item: T) -> List<T> {
        List { head: Some(Rc::new(Node { item, next: self.head.clone() })) }
    }
    // возвращает список без головы
    fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }
}

impl<T> Display for List<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut r = "".to_string();
        let mut link = self.head.as_ref();
        while let Some(x) = link {
            r.push_str(format!("{}, ", x.item).as_str());
            link = x.next.as_ref().clone()
        }
        if r.ends_with(", ") {
            r.truncate(r.len() - 2)
        }
        write!(f, "[{}]", r)
    }
}
