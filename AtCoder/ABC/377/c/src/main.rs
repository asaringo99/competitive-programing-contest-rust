use std::collections::HashSet;

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

fn check(size: i32, x: i32, y: i32) -> bool {
    if 0 > x || x >= size || 0 > y || y >= size {
        false
    } else {
        true
    }
}

fn main() {
    let n: i32;
    let m: i32;
    scan!(n, m);
    let mut set = HashSet::new();
    for _ in 0..m {
        let mut x: i32;
        let mut y: i32;
        scan!(x, y);
        x -= 1; y -= 1;
        set.insert((x, y));
        for dx in -2i32..3 {
            if dx == 0 {
                continue;
            }
            for dy in -2i32..3 {
                if dy == 0 {
                    continue;
                }
                if dx.abs() == dy.abs() {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if check(n, nx, ny) {
                    set.insert((nx, ny));
                }
            }
        }
    }
    let res = (n as i64 * n as i64) - (set.len() as i64);
    pt!(res);
}
