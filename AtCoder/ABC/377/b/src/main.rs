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

fn main() {
    let n: i32 = 8;
    let mut a_v: Vec<String>;
    scanv!(n, a_v, String);
    let mut set = HashSet::new();
    for i in 0..n {
        for (j, c) in a_v[i as usize].as_str().chars().enumerate() {
            if c == '#' {
                for x in 0..n {
                    set.insert((x as i32, j as i32));
                }
                for y in 0..n {
                    set.insert((i as i32, y as i32));
                }
            }
        }
    }
    let res = n * n - (set.len() as i32);
    pt!(res)
}
