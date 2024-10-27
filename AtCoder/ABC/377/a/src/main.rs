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
    let s: String;
    scan!(s);
    if &s == "ABC" {
        pt!("Yes")
    }
    else if &s == "ACB" {
        pt!("Yes")
    }
    else if &s == "BAC" {
        pt!("Yes")
    }
    else if &s == "BCA" {
        pt!("Yes")
    }
    else if &s == "CAB" {
        pt!("Yes")
    }
    else if &s == "CBA" {
        pt!("Yes")
    }
    else {
        pt!("No")
    }
}
