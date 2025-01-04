use std::{cmp::{max, min}, collections::BTreeSet, iter};
use text_io::read;

macro_rules! scan {
    ($($var:ident),+ $(,)?) => {
        $(
            $var = read!();
        )+
    };
}

macro_rules! scanv {
    ($len:expr, $vec:ident, $ty:ty) => {
        {
            $vec = vec![];
            for _ in 0..$len {
                let value: $ty = read!();
                $vec.push(value);
            }
        }
    };
}

macro_rules! pt {
    ($($arg:expr),*) => {
        {
            let mut iter = vec![$($arg.to_string()),*].into_iter();
            if let Some(first) = iter.next() {
                print!("{}", first);
                for item in iter {
                    print!(" {}", item);
                }
            }
            println!();
        }
    };
}

macro_rules! ptv {
    ($vec:expr) => {
        {
            let mut iter = $vec.iter();
            if let Some(first) = iter.next() {
                print!("{}", first);
                for item in iter {
                    print!(" {}", item);
                }
            }
            println!();
        }
    };
}

trait Monoid: Copy {
    fn e() -> Self; // 単位元
    fn op(lhs: Self, rhs: Self) -> Self; // 演算
}

struct SegTree<S: Monoid> {
    n: usize,
    data: Vec<S>,
}

impl<S: Monoid> SegTree<S> {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        SegTree {
            n,
            data: vec![S::e(); 2 * n],
        }
    }

    // 0-indexed
    fn set(&mut self, mut idx: usize, value: S) {
        idx += self.n;
        self.data[idx] = value;
        while idx > 1 {
            idx /= 2;
            self.data[idx] = S::op(self.data[2 * idx], self.data[2 * idx + 1]);
        }
    }
    
    // 0-indexed
    fn get(&self, mut idx: usize) -> S {
        idx += self.n;
        self.data[idx]
    }
    
    // 0-indexed
    fn prod(&self, mut l: usize, mut r: usize) -> S {
        l += self.n;
        r += self.n;
        let mut res_left = S::e();
        let mut res_right = S::e();
        while l < r {
            if l % 2 == 1 {
                res_left = S::op(res_left, self.data[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                res_right = S::op(self.data[r], res_right);
            }
            l /= 2;
            r /= 2;
        }
        S::op(res_left, res_right)
    }
}


#[derive(Clone, Copy)]
struct Max(i32);

impl Monoid for Max {
    fn e() -> Self {
        Max(i32::MIN)
    }
    fn op(lhs: Self, rhs: Self) -> Self {
        Max(max(lhs.0, rhs.0))
    }
}

fn main() {
    let n: i32; let q: i32;
    let mut a_v: Vec<i32>;
    scan!(n, q); scanv!(n, a_v, i32);
    for i in 0..n {
        a_v[i as usize] -= 1;
    }
    let mut id_v = vec![0 as usize; n as usize];
    let mut segtree = SegTree::<Max>::new(n as usize);
    for (i, a) in a_v.iter().enumerate() {
        let a = *a;
        segtree.set(i, Max(a));
        id_v[a as usize] = i;
    }
    let mut sec = vec![Vec::<(i32, usize)>::new(); n as usize];
    let mut l_v = vec![0; q as usize];
    let mut r_v = vec![0; q as usize];
    for i in 0..q {
        let idx = i as usize;
        let l: i32; let r: i32;
        scan!(l, r);
        l_v[idx] = l - 1;
        r_v[idx] = r - 1;
        sec[r_v[idx] as usize].push((l_v[idx], i as usize));
    }
    let mut dp = vec![0; n as usize];
    let mut stk = Vec::<i32>::with_capacity(n as usize);
    for i in (0..n).rev() {
        let idx = i as usize;
        let a = a_v[idx];
        while stk.last().is_some_and(|x| *x < a) {
            stk.pop();
        }
        dp[idx] = stk.len();
        stk.push(a);
    }
    let mut set = BTreeSet::<(i32, usize)>::new();
    let mut res = vec![0; q as usize];
    for r in 0..n {
        while !set.is_empty() {
            let (first, idx) = set.first().unwrap();
            let first = *first;
            let idx = *idx;
            if a_v[r as usize] < first {
                break;
            }
            set.pop_first();
            res[idx] = dp[r as usize] + 1;
        }
        for (l, i) in &sec[r as usize] {
            let l = *l;
            let i = *i;
            let maxval = segtree.prod((l + 1) as usize, (r + 1) as usize).0;
            set.insert((maxval, i));
        }
    }
    ptv!(res);
}
