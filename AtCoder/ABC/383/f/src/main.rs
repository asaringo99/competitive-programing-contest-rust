use std::{cmp::{max, min}, collections::btree_map::Range, vec};
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

fn main() {
    let n: i32; let x: i32; let k: i64;
    scan!(n, x, k);
    let mut p_v = vec![Vec::<i32>::new(); n as usize];
    let mut u_v = vec![Vec::<i64>::new(); n as usize];
    for _ in 0..n {
        let p: i32; let u: i64; let c: usize;
        scan!(p, u, c);
        p_v[c - 1].push(p);
        u_v[c - 1].push(u);
    }
    let mut dp = vec![vec![0; 2]; (x + 1) as usize];
    for c in 0..n {
        let c = c as usize;
        for i in 0..p_v[c].len() {
            let p = p_v[c][i];
            let u = u_v[c][i];
            for tot in (0..(x+1)).rev() {
                if tot - p < 0 {
                    break;
                }
                dp[tot as usize][1] = max(dp[tot as usize][1], dp[(tot - p) as usize][0] + u + k);
                dp[tot as usize][1] = max(dp[tot as usize][1], dp[(tot - p) as usize][1] + u);
            }
        }
        for tot in 0..(x+1) {
            dp[tot as usize][0] = max(dp[tot as usize][1], dp[tot as usize][0]);
            dp[tot as usize][1] = 0;
        }
    }
    let binding = dp.iter().map(|dp| dp[0]).collect::<Vec<_>>();
    let res = binding.iter().max().unwrap();
    pt!(res)
}
