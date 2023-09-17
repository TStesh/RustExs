use std::fmt::{Display, Formatter};

pub struct Board {
    max_r: usize,
    max_c: usize,
    brd: Vec<Vec<u8>>
}

fn gen_brd(max_r: usize, max_c: usize) -> Vec<Vec<u8>> {
    let mut brd = Vec::with_capacity(max_r + 2);
    let len = max_c + 2;
    for _ in 0..=max_r + 1 {
        let mut v = Vec::with_capacity(len);
        unsafe { v.set_len(len) }
        v.fill(0);
        brd.push(v);
    }
    brd
}

impl Board {
    pub fn new(max_r: usize, max_c: usize) -> Self {
        Self { max_r, max_c, brd: gen_brd(max_r, max_c) }
    }

    fn nbh_num(&self, i: usize, j: usize) -> u8 {
        let mut num = 0;
        for x in i - 1..=i + 1 {
            for y in j - 1 ..=j + 1 {
                if x == i && y == j { continue }
                num += self.brd[x][y];
            }
        }
        match num {
            2 | 3 => num,
            0 | 1 => 1,
            _ => 4
        }
    }

    fn state(&self, i: usize, j: usize) -> u8 {
        let transition = [0, 0, 1, 0, 0, 1, 1, 0];
        transition[((self.brd[i][j] << 2) + self.nbh_num(i, j) - 1) as usize]
    }

    pub fn next(&mut self) {
        let mut brd_new = gen_brd(self.max_r, self.max_c);
        for i in 1..=self.max_r {
            for j in 1..=self.max_c {
                brd_new[i][j] = self.state(i, j);
            }
        }
        self.brd = brd_new;
    }

    pub fn set(&mut self, i: usize, j: usize) { self.brd[i][j] = 1; }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 1..=self.max_r {
            write!(f, "|")?;
            for j in 1..=self.max_c {
                if self.brd[i][j] == 1 {
                    write!(f, "*|")?
                } else {
                    write!(f, " |")?
                }
            }
            writeln!(f)?
        }
        writeln!(f)
    }
}