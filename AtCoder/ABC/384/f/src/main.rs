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

fn rec(odd_v: Vec<i64>, eve_v: Vec<i64>) -> i64 {
    // println!("odd: {:?}, eve: {:?}", odd_v, eve_v);
    let mut ret = 0i64;
    let mut o1_v = Vec::<i64>::new();
    let mut e1_v = Vec::<i64>::new();
    let mut o2_v = Vec::<i64>::new();
    let mut e2_v = Vec::<i64>::new();
    for x in odd_v {
        if x / 2 & 1 == 1 {
            o1_v.push(x / 2);
        } else {
            e1_v.push(x / 2);
        }
    }
    for x in eve_v {
        if x / 2 & 1 == 1 {
            o2_v.push(x / 2);
        } else {
            e2_v.push(x / 2);
        }
    }
    ret += o1_v.iter().sum::<i64>() * o2_v.len() as i64 + o2_v.iter().sum::<i64>() * o1_v.len() as i64 + o1_v.len() as i64 * o2_v.len() as i64;
    ret += e1_v.iter().sum::<i64>() * e2_v.len() as i64 + e2_v.iter().sum::<i64>() * e1_v.len() as i64 + e1_v.len() as i64 * e2_v.len() as i64;
    // println!("ret: {}", ret);
    if !o1_v.is_empty() && !e2_v.is_empty() {
        ret += rec(o1_v, e2_v);
    }
    if !o2_v.is_empty() && !e1_v.is_empty() {
        ret += rec(o2_v, e1_v);
    }
    ret
}

fn f(vec: Vec<i64>) -> i64 {
    let mut ret = 0i64;
    let mut odd_v = Vec::<i64>::new();
    let mut eve_v = Vec::<i64>::new();
    for i in 0..vec.len() {
        if vec[i] / 2 & 1 == 1 {
            odd_v.push(vec[i] / 2);
        } else {
            eve_v.push(vec[i] / 2);
        }
    }
    ret += eve_v.iter().sum::<i64>() * (eve_v.len() + 1) as i64;
    ret += odd_v.iter().sum::<i64>() * (odd_v.len() + 1) as i64;
    ret += eve_v.len() as i64 * (eve_v.len() as i64 + 1) / 2 as i64;
    ret += odd_v.len() as i64 * (odd_v.len() as i64 + 1) / 2 as i64;
    ret += rec(odd_v, eve_v);
    ret
}

fn main() {
    let n: i64; let mut a_v: Vec<i64>;
    scan!(n); scanv!(n, a_v, i64);
    let mut res = 0i64;
    for i in 0..24 {
        let mut d_v = Vec::<i64>::new();
        let mut b_v = Vec::<i64>::new();
        for a in a_v {
            match a % 2 == 0 {
                true => d_v.push(a),
                false => b_v.push(a),
            }
        }
        let d_v_sum = d_v.iter().sum::<i64>();
        let b_v_sum = b_v.iter().sum::<i64>();
        res += d_v_sum * b_v.len() as i64 + b_v_sum * d_v.len() as i64;
        res += f(b_v);
        for i in 0..d_v.len() {
            d_v[i] /= 2;
        }
        a_v = d_v;
    }
    pt!(res)
} 
