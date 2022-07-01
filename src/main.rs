mod client;
mod models;

use std::thread;

use client::methods::transaction_stream;

fn main() {
    let txn_stream_thrd = thread::spawn(|| {
        transaction_stream();
    });
    loop {
        continue;
    }
}
