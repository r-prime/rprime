use std::io::{stdout, BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let numbers = Arc::new(Mutex::new((0..).into_iter()));
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let iterator = Arc::clone(&numbers);
        let count = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut nums = [0u32; 10000];
            let mut bufstdout = BufWriter::new(stdout());

            loop {
                {
                    let mut lock = iterator.lock().unwrap();
                    for j in 0..10000 {
                        nums[j] = lock.next().unwrap();
                    }
                }

                for n in nums {
                    if is_prime(n) {
                        writeln!(bufstdout, "{}", n).ok();
                        let mut c = count.lock().unwrap();
                        if *c == 1000000 {
                            std::process::exit(0);
                        }
                        *c += 1;
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

#[inline(always)]
fn is_prime(num: u32) -> bool {
    if num == 1 {
        return false;
    }

    let mut i: u32 = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
