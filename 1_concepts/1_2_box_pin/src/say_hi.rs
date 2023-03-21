use std::pin::Pin;

pub trait SayHi: std::fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

// impl<T> SayHi for Box<T> {}
// impl<T> SayHi for Rc<T> {}
// impl<T> SayHi for Vec<T> {}
// impl SayHi for String {}
// impl SayHi for &[u8] {}

// this impl covers all of the above
impl<T: std::fmt::Debug> SayHi for T {}