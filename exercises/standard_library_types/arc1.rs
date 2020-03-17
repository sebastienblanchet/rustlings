// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted'.
    // avoids copying numbers
    let shared_numbers = Arc::new(numbers);

    // used to keep track of threads
    // same as Vec::new()
    let mut joinhandles = vec![];

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);

        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;

            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }

    for handle in joinhandles {
        handle.join().unwrap();
    }
}
