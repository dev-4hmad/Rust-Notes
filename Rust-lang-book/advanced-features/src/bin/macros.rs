//! Declarative macros with `macro_rules!`
//! This file demonstrates simple patterns, repetitions, and hygiene.

// 1) A simple vec-like macro
#[macro_export]
macro_rules! v {
    ( $( $x:expr ),* $(,)? ) => {{
        let mut temp_vec = Vec::new();
        $( temp_vec.push($x); )*
        temp_vec
    }};
}

// 2) A logging macro with optional target
macro_rules! logln {
    (target: $target:expr, $($arg:tt)*) => {{
        println!("[{}] {}", $target, format!($($arg)*));
    }};
    ($($arg:tt)*) => {{
        println!("{}", format!($($arg)*));
    }};
}

// 3) Count identifiers
macro_rules! count_idents {
    ( $($name:ident),* $(,)? ) => {
        {
            let mut n = 0usize;
            $( let _ = stringify!($name); n += 1; )*
            n
        }
    };
}

fn main() {
    // Using v!
    let a = v![1, 2, 3, 4];
    println!("a = {:?}", a);

    // Using logln!
    logln!("plain message");
    logln!(target: "AUTH", "user {} logged in", "ada");

    // Using count_idents!
    let n = count_idents!(alpha, beta, gamma);
    println!("count_idents = {}", n);
}
