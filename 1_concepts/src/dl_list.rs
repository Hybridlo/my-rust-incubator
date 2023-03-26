use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::{Mutex, Arc, Weak};

type Link<T> = Option<Arc<Mutex<Node<T>>>>;
type LinkWeak<T> = Option<Weak<Mutex<Node<T>>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
    previous: LinkWeak<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            value: val,
            next: None,
            previous: None,
        }
    }

    fn set_value(&mut self, new_val: T) {
        self.value = new_val;
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn remove(self) -> T {
        if let Some(next) = &self.next {
            if let Ok(mut next_guard) = next.lock() {
                next_guard.previous = self.previous.clone();
            }
        }

        if let Some(previous) = self.previous.and_then(|prev| prev.upgrade()) {
            if let Ok(mut previous_guard) = previous.lock() {
                previous_guard.next = self.next.clone();
            }
        }

        self.value

        // self was pointed at by next.previous and previous.next, these were changed so self is gone from the chain by the end of the function
    }
}

pub struct DoubleLinkList<T> {
    head: Mutex<Link<T>>,
    tail: Mutex<LinkWeak<T>>,
    len: AtomicUsize,
}

impl<T> DoubleLinkList<T> {
    pub fn new() -> Self {
        Self {
            head: Mutex::new(None),
            tail: Mutex::new(None),
            len: AtomicUsize::new(0),
        }
    }

    fn head(&self) -> Link<T> {
        self.head.lock().ok().and_then(|link| link.clone())
    }

    fn tail(&self) -> Link<T> {
        self.tail.lock().ok().and_then(|guard| guard.as_ref().and_then(|link| link.upgrade()))
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::Acquire)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn head_inspect(&self, f: impl FnOnce(&T)) {
        if let Some(head) = self.head() {
            if let Ok(head_guard) = head.lock() {
                f(&head_guard.value)
            }
        }
    }

    pub fn head_check(&self, f: impl FnOnce(&T) -> bool) -> bool {
        if let Some(head) = self.head() {
            if let Ok(head_guard) = head.lock() {
                return f(&head_guard.value);
            }
        }

        false
    }

    pub fn tail_inspect(&self, f: impl FnOnce(&T)) {
        if let Some(tail) = self.tail() {
            if let Ok(tail_guard) = tail.lock() {
                f(&tail_guard.value)
            }
        }
    }

    pub fn tail_check(&self, f: impl FnOnce(&T) -> bool) -> bool {
        if let Some(tail) = self.tail() {
            if let Ok(tail_guard) = tail.lock() {
                return f(&tail_guard.value);
            }
        }

        false
    }

    pub fn pop_head(&self) -> Option<T> {
        let mut head_next = None;
        let mut res = None;

        let mut node_guard = self.head.lock().unwrap();
        if let Some(head) = node_guard.take() {
            let head = Arc::try_unwrap(head).unwrap_or_else(|_| panic!("Arc was held by more than one thread"));

            if let Ok(head_guard) = head.lock() {
                head_next = head_guard.next.clone();
            }

            res = Some(head.into_inner().expect("Another thread held a mutex and paniced").value);

            self.len.fetch_sub(1, Ordering::Release);
        }

        // replace head with the next one
        *node_guard = head_next.clone();

        // if next of head is None - tail is also head, but tail is weakref, so it's cleaned automatically

        //the previous of head.next was weak, so the only strong ref to old head was from self.head, which is now different, so that node is dropped
        res
    }

    pub fn pop_tail(&self) -> Option<T> {
        let mut tail_previous = None;

        let mut node_guard_head = self.head.lock().unwrap();
        let mut node_guard = self.tail.lock().unwrap();
        if let Some(tail) = (node_guard.as_ref())?.upgrade() {
            if let Ok(tail_guard) = tail.lock() {
                tail_previous = tail_guard.previous.clone();

                // if previous of tail is None - head is also tail, and should be made None
                if tail_previous.is_none() {
                    *node_guard_head = None;
                }
            }

            self.len.fetch_sub(1, Ordering::Release);
        }

        // replace tail with the previous one
        let tail = node_guard.take().and_then(|link| link.upgrade());
        *node_guard = tail_previous;

        tail.map(
            |node| Arc::try_unwrap(node)
                                            .unwrap_or_else(|_| panic!("Arc was held by more than one thread"))
                                            .into_inner()
                                            .expect("Another thread held a mutex and paniced")
                                            .value
        )
    }

    pub fn push_head(&self, new_val: T) {
        if let Ok(mut node_guard) = self.head.lock() {
            let prev_head = node_guard.take();

            let mut new_node = Node::new(new_val);

            new_node.next = prev_head.clone();

            let new_node_link = Arc::new(Mutex::new(new_node));

            if let Ok(mut node_guard) = self.tail.lock() {
                if node_guard.is_none() {
                    *node_guard = Some(Arc::downgrade(&new_node_link));
                }
            }

            *node_guard = Some(new_node_link);

            self.len.fetch_add(1, Ordering::Release);

            if let Some(prev_head) = &prev_head {
                if let Ok(mut prev_head_guard) = prev_head.lock() {
                    prev_head_guard.previous = node_guard.as_ref().map(|link| Arc::downgrade(link));
                }
            }
        }
    }

    pub fn push_tail(&self, new_val: T) {
        if let Ok(mut node_guard) = self.tail.lock() {
            let prev_tail = node_guard.take();

            let mut new_node = Node::new(new_val);

            new_node.previous = prev_tail.clone();

            let link = Arc::new(Mutex::new(new_node));
            *node_guard = Some(Arc::downgrade(&link));

            // also set head if it's None
            if let Ok(mut node_guard) = self.head.lock() {
                if node_guard.is_none() {
                    *node_guard = Some(link.clone());
                }
            }

            self.len.fetch_add(1, Ordering::Release);

            if let Some(prev_tail) = &prev_tail.and_then(|link| link.upgrade()) {
                if let Ok(mut prev_tail_guard) = prev_tail.lock() {
                    prev_tail_guard.next = Some(link);
                }
            }
        }
    }
}

impl<T: Clone> DoubleLinkList<T> {
    pub fn get_head(&self) -> Option<T> {
        if let Ok(node_guard) = self.head.lock() {
            if let Some(head) = &*node_guard {
                if let Ok(head_guard) = head.lock() {
                    return Some(head_guard.value.clone());
                }
            }
        }

        None
    }

    pub fn get_tail(&self) -> Option<T> {
        if let Ok(node_guard) = self.tail.lock() {
            if let Some(tail) = &node_guard.as_ref().and_then(|link| link.upgrade()) {
                if let Ok(tail_guard) = tail.lock() {
                    return Some(tail_guard.value.clone());
                }
            }
        }

        None
    }
}

impl<T: PartialEq> DoubleLinkList<T> {
    pub fn contains(&self, item: &T) -> bool {
        let mut node_ref = None;

        if let Ok(head_guard) = self.head.lock() {
            node_ref = (*head_guard).clone();
        }
        while let Some(node) = node_ref.clone() {
            if let Ok(node_guard) = node.lock() {
                if node_guard.value == *item {
                    return true;
                }

                node_ref = node_guard.next.clone();
            }
        }

        false
    }
}

/* impl<T> Drop for DoubleLinkList<T> {
    fn drop(&mut self) {
        while !self.is_empty() {
            println!("{:?}", self.head.lock().unwrap().is_some());
            println!("{:?}", self.tail.lock().unwrap().is_some());
            self.pop_head();
        }
    }
}
 */