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
    
    fn get(&self, mut idx: usize) -> S {
        idx += self.n;
        self.data[idx]
    }

    // 区間クエリ（[l, r)）
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
struct Sum(i32);

impl Monoid for Sum {
    fn e() -> Self {
        Sum(0)
    }
    fn op(lhs: Self, rhs: Self) -> Self {
        Sum(lhs.0 + rhs.0)
    }
}

fn f(a_v: Vec<i64>, n: i32, m: i32) -> i64    {
    let mut res = 0i64;
    let a_v = a_v.clone();
    let mut segtree = SegTree::<Sum>::new(m as usize);
    let mut sum = 0;
    let mut tot = 0i64;
    for a in &a_v {
        sum += a;
        sum %= m as i64;
        tot += sum as i64;
        segtree.set(sum as usize, Sum(segtree.get(sum as usize).0 + 1i32));
    }
    let mut sum = 0;
    for a in &a_v {
        res += tot;
        let cnt = segtree.prod(0 as usize, sum as usize);
        res += cnt.0 as i64 * (m as i64 - sum as i64);
        let cnt = segtree.prod(sum as usize, m as usize);
        res -= cnt.0 as i64 * sum as i64;
        sum += a;
        sum %= m as i64;
        tot -= sum;
        segtree.set(sum as usize, Sum(segtree.get(sum as usize).0 - 1i32));
    }
    res
}

fn main() {
    let n: i32; let m: i32; let mut a_v: Vec<i64>;
    scan!(n, m); scanv!(n, a_v, i64);
    pt!(f(a_v, n, m))
}
