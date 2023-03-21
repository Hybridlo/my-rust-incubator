use std::{ops::Deref, borrow::Borrow, fmt::Display};

use rand::Rng;

#[derive(Clone)]
pub struct Random<T>(Box<T>, Box<T>, Box<T>);

impl<T> Random<T> {
    pub fn new(val_one: T, val_two: T, val_three: T) -> Self {
        Self(Box::new(val_one), Box::new(val_two), Box::new(val_three))
    }

    fn get_random(&self) -> &T {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..=2) {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!("Rand returned an invalid value"),
        }
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_random()
    }
}

impl<T> AsRef<T> for Random<T> {
    fn as_ref(&self) -> &T {
        self    // uses Deref for coersion
    }
}

impl<T> Borrow<T> for Random<T> {
    fn borrow(&self) -> &T {
        self    // uses Deref for coersion
    }
}

impl<T: Display> Display for Random<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}