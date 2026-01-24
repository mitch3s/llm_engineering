use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let iterations: u64 = 200_000_000;
    let mut result: f64 = 1.0;

    // j_minus = 4*i - 1, j_plus = 4*i + 1 for i starting at 1
    let mut j_minus: f64 = 3.0;
    let mut j_plus: f64 = 5.0;

    let mut i = 0u64;
    while i < iterations {
        result -= 1.0 / j_minus;
        result += 1.0 / j_plus;

        j_minus += 4.0;
        j_plus += 4.0;

        i += 1;
    }

    result *= 4.0;

    let duration = start_time.elapsed().as_secs_f64();

    println!("Result: {:.12}", result);
    println!("Execution Time: {:.6} seconds", duration);
}