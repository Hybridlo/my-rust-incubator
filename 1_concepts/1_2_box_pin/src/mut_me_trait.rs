use std::{rc::Rc, pin::Pin};

#[derive(Default, Debug)]
pub struct TestStruct {
    pub field: usize
}

trait TraitforMut {
    fn make_a_mut(&mut self);
}

impl TraitforMut for TestStruct {
    fn make_a_mut(&mut self) {
        self.field += 1;
    }
}

impl<T: TraitforMut> TraitforMut for Rc<T> {
    fn make_a_mut(&mut self) {
        Rc::get_mut(self).expect("Rc can be mutated only if there's a single owner").make_a_mut();
    }
}

impl<T: TraitforMut> TraitforMut for Box<T> {
    fn make_a_mut(&mut self) {
        (**self).make_a_mut();
    }
}

impl<T: TraitforMut> TraitforMut for Vec<T> {
    fn make_a_mut(&mut self) {
        for item in self.iter_mut() {
            item.make_a_mut();
        }
    }
}

impl TraitforMut for String {
    fn make_a_mut(&mut self) {
        *self += " mutated";
    }
}

impl TraitforMut for &[u8] {
    fn make_a_mut(&mut self) {
        *self = Default::default();
    }
}

pub trait MutMeSomehowTrait {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

/* impl<T: TraitforMut> MutMeSomehowTrait for Box<T> {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        (**self).make_a_mut();
    }
}

impl<T: TraitforMut> MutMeSomehowTrait for Rc<T> {
    /// safety: this function invokes make_a_mut of Rc implementation, that only
    /// replaces the Rc referenced by the borrow, the borrow itself is not moved
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let mut_self = unsafe { self.get_unchecked_mut() };
        mut_self.make_a_mut();
    }
}

impl<T: TraitforMut> MutMeSomehowTrait for Vec<T> {
    /// safety: this function invokes make_a_mut of Vec implementation, that only
    /// makes a mut iterable and changes values of the inner items, thus not moving the borrow
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let mut_self = unsafe { self.get_unchecked_mut() };
        mut_self.make_a_mut();
    }
}

impl MutMeSomehowTrait for String {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.make_a_mut();
    }
}

impl MutMeSomehowTrait for &[u8] {
    fn mut_me_somehow(mut self: Pin<&mut Self>) {
        self.make_a_mut();
    }
} */


// general case
impl<T: TraitforMut> MutMeSomehowTrait for T {
    /// safety: this function invokes make_a_mut of TraitforMut impls
    /// they don't move the &mut so it's save to get it from Pin
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let mut_self = unsafe { self.get_unchecked_mut() };
        mut_self.make_a_mut();
    }
}