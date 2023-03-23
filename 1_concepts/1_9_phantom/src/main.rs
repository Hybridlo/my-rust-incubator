use std::{marker::PhantomData, collections::HashMap};

use rand::Rng;

struct Fact<T: ?Sized> {
    _data: PhantomData<T>
}

impl<T: ?Sized> Fact<T> {
    pub fn new() -> Self {
        Self {
            _data: PhantomData,
        }
    }
}

impl Fact<String> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "String is a wrapper around a Vec<u8>",
            "String doesn't copy data it holds when changing it",
            "String, just like str, guarantees valid UTF-8"
        ];

        let mut rng = rand::thread_rng();

        facts[rng.gen_range(0..=2)]
    }
}

impl<T> Fact<[T]> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "Slices are only accessible through a pointer",
            "Slices can refer to data from any source: heap, stack, read-only memory",
            "Slices are always a continuous block of data"
        ];

        let mut rng = rand::thread_rng();

        facts[rng.gen_range(0..=2)]
    }
}

impl<K, V> Fact<HashMap<K, V>> {
    pub fn fact(&self) -> &'static str {
        let facts = [
            "HashMap requires the key to implement Hash trait",
            "HashMap takes the owned value of the key to set an item, but to get an item - a borrow is enough",
            "HashMap has O(1) lookup, but it has a big constant because of hashing, so sometimes searching a Vec can be faster"
        ];

        let mut rng = rand::thread_rng();

        facts[rng.gen_range(0..=2)]
    }
}

fn main() {
    let fact: Fact<String> = Fact::new();

    for _ in 0..5 {
        println!("Here's a fact about String: {}", fact.fact());
    }
    println!("\n");


    let fact: Fact<[i32]> = Fact::new();

    for _ in 0..5 {
        println!("Here's a fact about slices: {}", fact.fact());
    }
    println!("\n");


    let fact: Fact<HashMap<i32, i32>> = Fact::new();

    for _ in 0..5 {
        println!("Here's a fact about HashMap: {}", fact.fact());
    }
}
