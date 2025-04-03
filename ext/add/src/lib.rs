use magnus::{define_class, function, method, prelude::*, Error};

fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<(), Error> {
    ruby.define_global_function("fib", magnus::function!(fib, 1));
    ruby.define_global_function("add", magnus::function!(add, 2));
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
