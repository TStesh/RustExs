use std::cmp::max;
use std::collections::HashMap;

fn prefix(s: &str) -> Vec<usize> {
    let mut v = Vec::with_capacity(s.len());
    unsafe { v.set_len(s.len()) };
    v.fill(0);
    for i in 1..s.len() {
        let mut j = 0;
        while i + j < s.len() && &s[j..=j] == &s[i + j..i + j + 1] {
            v[i + j] = max(v[i + j], j + 1);
            j += 1;
        }
    }
    v
}

// Алгоритм Кунта-Мориса-Пратта
pub fn kmp_search(s: &str, p: &str) -> Vec<usize> {
    let mut v = vec![];
    let pf = prefix(p);
    let mut i = 0;
    let mut j = 0;
    while i < s.len() {
        if &p[j..=j] == &s[i..=i] {
            i += 1;
            j += 1;
        }
        if j == p.len() {
            v.push(i - j);
            j = pf[j - 1];
        } else if i < s.len() && &p[j..=j] != &s[i..=i] {
            if j != 0 {
                j = pf[j - 1];
            } else {
                i += 1;
            }
        }
    }
    v
}
