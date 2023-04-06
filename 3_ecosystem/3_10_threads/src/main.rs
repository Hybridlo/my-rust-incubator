use std::thread;

use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn produce() -> Vec<Vec<u8>> {
    let mut rng = thread_rng();

    (0..64)
        .map(|_| (0..64).map(|_| rng.gen()).collect())
        .collect()
}

fn consume(data: Vec<Vec<u8>>) {
    let res: u64 = data
        .into_par_iter()
        .map(|inner_vec| inner_vec.into_par_iter().map(|el| el as u64).sum::<u64>())
        .sum();

    println!("Computed sum: {res}");
}

fn main() {
    let (send, recv) = crossbeam_channel::bounded(10);

    let _consumers = (0..2)
        .map(|_| {
            let recv = recv.clone();
            thread::spawn(move || {
                while let Ok(val) = recv.recv() {
                    consume(val)
                }
            })
        })
        .collect::<Vec<_>>();

    loop {
        let new_val = produce();

        send.send(new_val).expect("Send to work");
    }
}
