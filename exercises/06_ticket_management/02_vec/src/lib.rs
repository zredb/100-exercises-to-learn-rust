// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.

    match n {
        0 => 0,
        1 => 1,
        n => {
            let mut fib = Vec::new();
            fib.push(0);
            fib.push(1);
            for i in 2..=n {
                let x = fib.get((i - 2) as usize).unwrap() + fib.get((i - 1) as usize).unwrap();

                fib.push(x);
                println!("{} --> {} ==>{:?}", i, x, fib);
            }
            *fib.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirthieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
