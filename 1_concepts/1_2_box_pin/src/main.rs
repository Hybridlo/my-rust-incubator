mod say_hi;
mod mut_me_trait;
mod measure_future;

use std::pin::Pin;

use say_hi::SayHi;
use mut_me_trait::{MutMeSomehowTrait, TestStruct};
use measure_future::MeasurableFuture;



#[tokio::main]
async fn main() {
    let mut item = TestStruct { field: 10 };
    Pin::new(&item).say_hi();
    Pin::new(&mut item).mut_me_somehow();
    Pin::new(&item).say_hi();

    MeasurableFuture::new(
        tokio::time::sleep(std::time::Duration::from_secs(1))
    ).await;
}
