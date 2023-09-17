/// L2-список на базе двух векторов
use std::fmt::{Display, Formatter};

const NIL: usize = 0;
const FREE_NIL: usize = 1;

pub struct LList<T> {
    items: Vec<T>,
    links: Vec<usize>,
    blinks: Vec<usize>,
    // ptr(x): (@prev(x), x, @next(x))
    ptr: (usize, usize, usize)
}

impl<T> LList<T> {
    // конструктор
    pub fn new(max_elems: usize) -> Self {
        let max_size = max_elems + 2;
        let mut items = Vec::with_capacity(max_size);
        unsafe { items.set_len(max_size); }
        let mut links = Vec::with_capacity(max_size);
        unsafe { links.set_len(max_size); }
        let mut blinks = Vec::with_capacity(max_size);
        unsafe { blinks.set_len(max_size); }
        // инициализировать список элементов:
        links[NIL] = NIL;
        blinks[NIL] = NIL;
        // инициализировать список свободных элементов:
        for i in FREE_NIL..max_size - 1 {
            links[i] = i + 1;
            blinks[i + 1] = i;
        }
        links[max_size - 1] = FREE_NIL;
        blinks[FREE_NIL] = max_size - 1;
        Self { items, links, blinks, ptr: (NIL, NIL, NIL) }
    }

    // Список пуст?
    pub fn empty(&self) -> bool { self.links[NIL] == NIL }

    // Список полон?
    pub fn full(&self) -> bool { self.links[FREE_NIL] == FREE_NIL }

    // Указатель в конце списка?
    pub fn end(&self) -> bool { self.ptr.1 == NIL }

    // Установить указатель в начало списка
    pub fn start_ptr(&mut self) {
        let xs = self.links[NIL];
        self.ptr = (NIL, xs, self.links[xs]);
    }

    // Установить указатель в начало списка
    pub fn end_ptr(&mut self) {
        let xs = self.blinks[NIL];
        self.ptr = (self.blinks[xs], xs, NIL);
    }

    // Переместить указатель вперед
    pub fn next(&mut self) {
        self.ptr.0 = self.ptr.1;
        self.ptr.1 = self.ptr.2;
        self.ptr.2 = self.links[self.ptr.1];
    }

    // Переместить указатель назад
    pub fn prev(&mut self) {
        self.ptr.2 = self.ptr.1;
        self.ptr.1 = self.ptr.0;
        self.ptr.0 = self.blinks[self.ptr.1];
    }

    // получить элемент за указателем
    pub fn elem(&self) -> &T { &self.items[self.ptr.1] }

    fn alloc(&mut self) -> Result<usize, String> {
        if self.full() { return Err("Недостаточно памяти".to_string()) }
        let closest_free = self.links[FREE_NIL];
        let xs = self.links[closest_free];
        // корректируем прямые связи:
        self.links[FREE_NIL] = xs;
        // корректируем обратные связи:
        self.blinks[xs] = FREE_NIL;
        Ok(closest_free)
    }

    fn free(&mut self, x: usize) {
        let xs = self.links[FREE_NIL];
        // корректируем прямые связи:
        self.links[FREE_NIL] = x;
        self.links[x] = xs;
        // корректируем обратные связи:
        self.blinks[x] = FREE_NIL;
        self.blinks[xs] = x;
    }

    // Добавить элемент за указателем
    pub fn add(&mut self, elem: T) -> Result<(), String> {
        // получить свободный узел:
        let new_index= self.alloc()?;
        // добавить узел в список за указателем:
        self.links[self.ptr.1] = new_index;
        self.blinks[new_index] = self.ptr.1;
        self.links[new_index] = self.ptr.2;
        self.blinks[self.ptr.2] = new_index;
        // корректируем текущий указатель:
        self.ptr.2 = new_index;
        // разместить элемент по указанному индексу:
        self.items[new_index] = elem;
        Ok(())
    }

    // Удалить элемент за указателем
    pub fn del(&mut self) {
        // удалить узел из списка за указателем:
        let del_index = self.ptr.2;
        self.ptr.2 = self.links[del_index];
        self.links[self.ptr.1] = self.ptr.2;
        self.blinks[self.ptr.2] = self.ptr.1;
        // добавить удаленный узел в список свободных:
        self.free(del_index);
    }
}

impl<T> Display for LList<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = self.links[0];
        write!(f, "[")?;
        let mut first = true;
        while i != NIL {
            if first {
                write!(f, "{}", self.items[i])?;
                first = false;
            } else {
                write!(f, ", {}", self.items[i])?;
            }
            i = self.links[i];
        }
        write!(f, "]")
    }
}

/*
fn main() {

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
}
*/