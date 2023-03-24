use std::mem;

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s);


    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve_noclone();
    println!("noclone version: {:?}", s);
}

#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Trinity<T> {
    fn rotate(&mut self) {
        // self.b = self.a, self.a = self.b
        mem::swap(&mut self.a, &mut self.b);
        // self.c = self.b (has the initial value of self.a), self.b = self.c
        mem::swap(&mut self.b, &mut self.c);
        // result (right hand side is old/initial values):
        // self.a = self.b
        // self.b = self.c
        // self.c = self.a



        /* let a = self.a.clone();
        let b = self.b.clone();
        let c = self.c.clone();
        self.a = b;
        self.b = c;
        self.c = a; */
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: Clone + PartialEq> Solver<T> {
    /// optional: if we MUST keep self.expected unchanged;
    /// a noclone version is provided in resolve_noclone
    fn resolve(&mut self) {
        let mut expected = self.expected.clone();

        self
            .unsolved
            .retain(|unsolved| {
                for  _ in 0..3 {
                    if *unsolved == expected {
                        // it got solved - we can early return and not include this in result
                        return false;
                    }
                    // check all rotations
                    expected.rotate();
                }
                // all checks failed - not solved
                return true;
            }
        );



        /* let mut unsolved = Vec::with_capacity(self.unsolved.len());
        'l: for t in self.unsolved.iter_mut() {
            for _ in 0..3 {
                if *t == self.expected {
                    continue 'l;
                }
                t.rotate();
            }
            unsolved.push(t.clone())
        }
        self.unsolved = unsolved; */
    }
}

impl<T: PartialEq> Solver<T> {
    fn resolve_noclone(&mut self) {
        self
            .unsolved
            .retain(|unsolved| {
                for  _ in 0..3 {
                    if *unsolved == self.expected {
                        // it got solved - we can early return and not include this in result
                        return false;
                    }
                    // check all rotations
                    self.expected.rotate();
                }
                // all checks failed - not solved
                return true;
            }
        );
    }
}