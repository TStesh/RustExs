use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point{ x: f64, y: f64 }

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }
}

#[derive(Debug)]
pub struct Segment { p: Point, q: Point, v: (f64, f64) }

impl Segment {
    // конструктор
    fn new(p: Point, q: Point) -> Self {
        Self { p, q, v: (q.x - p.x, q.y - p.y) }
    }

    // скалярное произведение
    fn dot(&self, that: &Segment) -> f64 {
        self.v.0 * that.v.0 + self.v.1 * that.v.1
    }

    // модуль векторного произведения
    fn mul(&self, that: &Segment) -> f64 {
        self.v.0 * that.v.1 - self.v.1 * that.v.0
    }

    // длина
    fn length(&self) -> f64 { self.dot(self).sqrt() }

    // p лежит на линии отрезка?
    fn inline(&self, p: Point) -> bool {
        self.mul(&Segment::new(self.p, p)) == 0.
    }

    // p лежит внутри отрезка?
    fn within(&self, p: Point) -> bool {
        self.length() == Segment::new(self.p, p).length() +
            Segment::new(self.q, p).length()
    }

    // отрезок освещен из p?
    // ориентация self.p -> self.q
    fn is_light(&self, p: Point) -> bool {
        let pa = Segment::new(p, self.p);
        let pb = Segment::new(p, self.q);
        let s = pa.mul(&pb);
        s < 0. || (s == 0. && !self.inline(p))
    }
}

pub struct ConvexHull {
    vertices: VecDeque<Point>,
    perimeter: f64,
    area: f64
}

impl ConvexHull {
    // пустая оболочка
    pub fn new() -> Self {
        Self { vertices: VecDeque::new(), perimeter: 0., area: 0. }
    }
    // Добавить точку в оболочку
    pub fn add_point(&mut self, p: Point) {
        match self.vertices.len() {
            0 => self.process_empty(p),
            1 => self.process_point(p),
            2 => self.process_segment(p),
            _ => self.process_polygon(p)
        }
    }

    // Оболочка пустая, просто добавляем точку
    fn process_empty(&mut self, p: Point) {
        self.vertices.push_front(p);
        self.perimeter = 0.;
        self.area = 0.;
    }

    // В оболочке одна точка
    // Проверяем, что новая не совпадает с ней
    // И добавляем новую точку
    fn process_point(&mut self, p: Point) {
        let a = self.vertices[0];
        if p != a {
            self.vertices.push_front(p);
            self.perimeter = 2. * Segment::new(a, p).length();
            self.area = 0.;
        }
    }

    // В оболочке одно ребро => может три случая:
    // 2) точка лежит на продолжении отрезка => удаляем ненужную точку, добавляем новую точку,
    // 3) точка лежит где-то вне отрезка => просто добавляем новую точку
    fn process_segment(&mut self, p: Point) {
        let a = self.vertices[0];
        let b = self.vertices[1];
        let ab = Segment::new(a, b);
        // Первый вариант: точка P лежит в границах отрезка AB
        // Оболочка не меняется
        if ab.within(p) {
            println!("Точка {:?} внутри оболочки", p);
            return
        }
        // Второй вариант: точка P лежит на продолжении отрезка AB
        // Заменяем ненужную точку (A или B) на новую точку P
        // В оболочке по прежнему одно ребро
        if ab.inline(p) {
            // какую точку оставить?
            let x = if Segment::new(a, p).within(b) { a } else { b };
            self.vertices = VecDeque::from([x, p]);
            self.perimeter = 2. * Segment::new(x, p).length();
            self.area = 0.;
            return
        }
        // Третий вариант: a, b, p образуют треугольник
        // формируем новый дек с учетом правильной ориентации
        let pa = Segment::new(p, a);
        let pb = Segment::new(p, b);
        let s = pa.mul(&pb);
        self.vertices = VecDeque::from([p]);
        if s < 0. {
            // ребро (a->b) освещено
            self.vertices.push_front(a);
            self.vertices.push_back(b);
        } else {
            // ребро (a->b) не освещено
            self.vertices.push_front(b);
            self.vertices.push_back(a);
        }
        self.perimeter = pa.length() + pb.length() + ab.length();
        self.area = s.abs() / 2.;
    }

    // Обработать ребро
    fn process_edge(&mut self, a: Point, b: Point, c: Point) {
        self.perimeter -= Segment::new(a, b).length();
        let ca = Segment::new(c, a);
        let cb = Segment::new(c, b);
        self.area += ca.mul(&cb).abs() / 2.;
    }

    // Удалить вершины освещенных ребер
    fn del_edges(&mut self, p: Point) {
        // обработаем начальную и конечную засвеченные вершины
        let a = self.vertices[self.vertices.len() - 1];
        let b = self.vertices[0];
        self.process_edge(a, b, p);
        // [ОВ, ?, ?, ..., ?, ОВ]
        // берем 0 и 1 вершины, проверяем на засветку.
        // Если не засвечено, то выходим из цикла
        // Если засвечено, то обработаем и удалим первый элемент
        loop {
            let a = self.vertices[0];
            let b = self.vertices[1];
            let ab = Segment::new(a, b);
            if ab.is_light(p) {
                // пересчитываем площадь и периметр
                self.process_edge(a, b, p);
                // удаляем вершину
                self.vertices.pop_front();
            } else {
                break;
            }
        }
    }

    // В оболочке не менее 3 точек
    fn process_polygon(&mut self, p: Point) {
        let vn = self.vertices.len();
        // освещено хотя бы одно ребро?
        for _ in 0..vn {
            // текущее ребро с учетом ориентации
            let a = self.vertices[vn - 1];
            let b = self.vertices[0];
            if Segment::new(a, b).is_light(p) {
                // удаляем освещенные ребра
                self.del_edges(p);
                // Пересчитываем периметр
                let a = self.vertices[0];
                let b = self.vertices[self.vertices.len() - 1];
                self.perimeter += Segment::new(a, p).length() +
                    Segment::new(b, p).length();
                // Добавляем Р в дек
                self.vertices.push_front(p);
                return
            }
            // "прокручиваем" дек влево на 1 позицию
            self.vertices.rotate_left(1);
        }
        // P внутри оболочки
        println!("Точка {:?} внутри оболочки", p);
    }
}

impl Display for ConvexHull {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\nPerimeter: {}", self.perimeter)?;
        writeln!(f, "Area: {}", self.area)?;
        writeln!(f, "Vertices: {}", self.vertices.len())?;
        for (i, p) in self.vertices.iter().enumerate() {
            write!(f, "{}: (x={}, y={}); ", i + 1, p.x, p.y)?;
        }
        writeln!(f)
    }
}

/*
mod conv;
use crate::conv::{ConvexHull, Point};

fn main() {
    let mut conv = ConvexHull::new();
    conv.add_point(Point::new(0., 0.));
    conv.add_point(Point::new(1., 0.));
    conv.add_point(Point::new(1., 1.));
    conv.add_point(Point::new(0., 1.));
    //conv.add_point(Point::new(0.9, 0.1));
    println!("{conv}");
}
*/