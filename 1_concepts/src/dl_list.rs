use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Mutex;

type Link<T> = AtomicPtr<Node<T>>;

struct Node<T> {
    value: Mutex<T>,
    next: Link<T>,
    previous: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            value: Mutex::new(val),
            next: AtomicPtr::new(ptr::null_mut()),
            previous: AtomicPtr::new(ptr::null_mut()),
        }
    }

    fn set_value(&self, new_val: T) {
        if let Ok(mut value_guard) = self.value.lock() {
            *value_guard = new_val;
        }
    }

    fn inspect_value(&self, f: impl FnOnce(&T)) {
        if let Ok(value_guard) = self.value.lock() {
            f(&*value_guard)
        }
    }

    fn check_value(&self, f: impl FnOnce(&T) -> bool) -> bool {
        if let Ok(value_guard) = self.value.lock() {
            f(&*value_guard)
        } else {
            panic!("Another thread paniced while holding the mutex");
        }
    }

    fn next(&self) -> Option<&Node<T>> {
        unsafe { self.next.load(Ordering::Acquire).as_ref() }
    }

    fn previous(&self) -> Option<&Node<T>> {
        unsafe { self.previous.load(Ordering::Acquire).as_ref() }
    }

    fn add_after(&self, new_node: *mut Node<T>) {
        let next = self.next.load(Ordering::Acquire);

        // link new_node between self and self.next
        {
            let new_node =
                unsafe { new_node.as_ref() }.expect("We created the node, must be initialized");

            new_node
                .previous
                .store(self as *const _ as *mut _, Ordering::Release);
            new_node.next.store(next, Ordering::Release);
        }

        // change the links of self and self.next to have the new_node
        if let Some(next) = unsafe { next.as_ref() } {
            next.previous.store(new_node, Ordering::Release);
        }

        self.next.store(new_node, Ordering::Release);
    }

    fn add_before(&self, new_node: *mut Node<T>) {
        let previous = self.previous.load(Ordering::Acquire);

        // link new_node between self and self.previous
        {
            let new_node =
                unsafe { new_node.as_ref() }.expect("We created the node, must be initialized");

            new_node
                .next
                .store(self as *const _ as *mut _, Ordering::Release);
            new_node.previous.store(previous, Ordering::Release);
        }

        // change the links of self and self.previous to have the new_node
        if let Some(previous) = unsafe { previous.as_ref() } {
            previous.next.store(new_node, Ordering::Release);
        }

        self.previous.store(new_node, Ordering::Release);
    }

    /// You can't use self after this call
    /// but since we operate in a hard to reason environment -
    /// we can't take ownership of self
    unsafe fn remove(&self) -> T {
        let next = self.next.load(Ordering::Acquire);
        let previous = self.previous.load(Ordering::Acquire);

        // change pointers of next and previous that pointed to self
        // to now point to each other
        if let Some(next) = unsafe { next.as_ref() } {
            next.previous.store(previous, Ordering::Release);
        }

        if let Some(previous) = unsafe { previous.as_ref() } {
            previous.next.store(next, Ordering::Release);
        }

        // self is removed from the structure - Box the pointer, get the value from it,
        // and since the Box is owned and consumed - self is dropped after this
        let self_box = unsafe { Box::from_raw(self as *const Node<T> as *mut Node<T>) };

        self_box
            .value
            .into_inner()
            .expect("Another thread paniced while holding the mutex")
    }
}

impl<T: Clone> Node<T> {
    fn get_value(&self) -> T {
        if let Ok(value_guard) = self.value.lock() {
            (*value_guard).clone()
        } else {
            panic!("Another thread paniced while holding the mutex");
        }
    }
}

pub struct DoubleLinkList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: AtomicUsize,
}

impl<T> DoubleLinkList<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            tail: AtomicPtr::new(ptr::null_mut()),
            len: AtomicUsize::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn head(&self) -> Option<&Node<T>> {
        unsafe { self.head.load(Ordering::Acquire).as_ref() }
    }

    fn tail(&self) -> Option<&Node<T>> {
        unsafe { self.tail.load(Ordering::Acquire).as_ref() }
    }

    pub fn head_inspect(&self, f: impl FnOnce(&T)) {
        if let Some(node) = self.head() {
            node.inspect_value(f)
        }
    }

    pub fn head_check(&self, f: impl FnOnce(&T) -> bool) -> bool {
        if let Some(node) = self.head() {
            return node.check_value(f);
        }

        false
    }

    pub fn tail_inspect(&self, f: impl FnOnce(&T)) {
        if let Some(node) = self.tail() {
            node.inspect_value(f)
        }
    }

    pub fn tail_check(&self, f: impl FnOnce(&T) -> bool) -> bool {
        if let Some(node) = self.tail() {
            return node.check_value(f);
        }

        false
    }

    pub fn pop_head(&self) -> Option<T> {
        // load a null into tail if it points to the same node as head
        match self.tail.compare_exchange(
            self.head.load(Ordering::Acquire),
            ptr::null_mut(),
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            // if they are the same - remove and update the head, and tail is nulled already - the same node won't be freed twice
            Ok(_) => {
                // check the integrity of head
                if let Some(head) = unsafe { self.head.load(Ordering::Acquire).as_ref() } {
                    self.head.store(ptr::null_mut(), Ordering::Release);

                    self.len.fetch_sub(1, Ordering::Release);
                    return Some(unsafe { head.remove() });
                }
            }
            // if they're not the same - load the head and just remove and update it
            Err(_) => {
                if let Some(head) = unsafe { self.head.load(Ordering::Acquire).as_ref() } {
                    let head_next = head.next.load(Ordering::Acquire);

                    self.head.store(head_next, Ordering::Release);

                    self.len.fetch_sub(1, Ordering::Release);
                    return Some(unsafe { head.remove() });
                }
            }
        }

        None
    }

    pub fn pop_tail(&self) -> Option<T> {
        match self.head.compare_exchange(
            self.tail.load(Ordering::Acquire),
            ptr::null_mut(),
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                // check the integrity of tail as well
                if let Some(tail) = unsafe { self.tail.load(Ordering::Acquire).as_ref() } {
                    self.tail.store(ptr::null_mut(), Ordering::Release);

                    self.len.fetch_sub(1, Ordering::Release);
                    return Some(unsafe { tail.remove() });
                }
            }
            Err(_) => {
                if let Some(tail) = unsafe { self.tail.load(Ordering::Acquire).as_ref() } {
                    let tail_previous = tail.previous.load(Ordering::Acquire);

                    self.tail.store(tail_previous, Ordering::Release);

                    self.len.fetch_sub(1, Ordering::Release);
                    return Some(unsafe { tail.remove() });
                }
            }
        }

        None
    }

    pub fn push_head(&self, new_val: T) {
        // create the node and move to heap
        let new_node = Box::into_raw(Box::new(Node::new(new_val)));

        match self.head.compare_exchange(
            ptr::null_mut(),
            new_node,
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            // head was null, then the tail is null as well
            Ok(_) => {
                // make sure the removals aren't in progress when assigning new node
                loop {
                    if self
                        .tail
                        .compare_exchange(
                            ptr::null_mut(),
                            new_node,
                            Ordering::Release,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                    {
                        break;
                    };
                }
                self.len.fetch_add(1, Ordering::Release);
            }
            Err(head_ptr) => {
                let head = unsafe { head_ptr.as_ref() }.expect("Just checked for null");
                head.add_before(new_node);

                self.head
                    .store(head.previous.load(Ordering::Acquire), Ordering::Release);
                self.len.fetch_add(1, Ordering::Release);
            }
        };
    }

    pub fn push_tail(&self, new_val: T) {
        // create the node and move to heap
        let new_node = Box::into_raw(Box::new(Node::new(new_val)));

        match self.tail.compare_exchange(
            ptr::null_mut(),
            new_node,
            Ordering::AcqRel,
            Ordering::Relaxed,
        ) {
            // tail was null, then the head is null as well
            Ok(_) => {
                // make sure the removals aren't in progress when assigning new node
                loop {
                    if self
                        .head
                        .compare_exchange(
                            ptr::null_mut(),
                            new_node,
                            Ordering::Release,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                    {
                        break;
                    };
                }
                self.len.fetch_add(1, Ordering::Release);
            }
            Err(tail_ptr) => {
                let tail = unsafe { tail_ptr.as_ref() }.expect("Just checked for null");
                tail.add_after(new_node);

                self.tail.store(new_node, Ordering::Release);
                self.len.fetch_add(1, Ordering::Release);
            }
        };
    }
}

impl<T: Clone> DoubleLinkList<T> {
    pub fn get_head(&self) -> Option<T> {
        self.head().map(|node| node.get_value())
    }

    pub fn get_tail(&self) -> Option<T> {
        self.tail().map(|node| node.get_value())
    }
}

impl<T: PartialEq> DoubleLinkList<T> {
    pub fn contains(&self, item: &T) -> bool {
        let mut node_ref = self.head();
        while let Some(node) = node_ref {
            if node.check_value(|val| *val == *item) {
                return true;
            }

            node_ref = node.next();
        }

        false
    }
}

impl<T> Drop for DoubleLinkList<T> {
    fn drop(&mut self) {
        while !self.is_empty() {
            self.pop_head();
        }
    }
}
