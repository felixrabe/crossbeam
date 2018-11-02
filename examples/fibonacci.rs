//! An asynchronous fibonacci sequence generator.

#[macro_use]
extern crate crossbeam_channel;

use std::thread;

use crossbeam_channel::{bounded, Sender};

// Sends the Fibonacci sequence into the channel until it becomes disconnected.
fn fibonacci(sender: Sender<u64>) {
    let (mut x, mut y) = (0, 1);
    loop {
        select! {
            send(sender, x) -> res => match res {
                Err(_) => return,
                Ok(()) => {
                    let tmp = x;
                    x = y;
                    y = tmp + y;
                }
            }
        }
    }
}

fn main() {
    let (s, r) = bounded(0);

    thread::spawn(move || {
        // Print the first 20 Fibonacci numbers.
        for num in r.iter().take(20) {
            println!("{}", num);
        }
        drop(r);
    });

    fibonacci(s);
}
