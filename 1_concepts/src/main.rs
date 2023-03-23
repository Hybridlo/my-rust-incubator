mod dl_list;

fn main() {}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate::dl_list::DoubleLinkList;

    #[test]
    fn test_ddl_single_thread() {
        let ddl = DoubleLinkList::new();

        for i in 0..10 {
            //println!("{}", i);
            ddl.push_tail(i);
        }

        for i in 0..10 {
            ddl.contains(&i);
        }

        assert_eq!(ddl.len(), 10);

        ddl.push_head(10);

        assert!(ddl.head_check(|&val| val == 10));
        assert_eq!(ddl.len(), 11);
    }

    #[test]
    fn test_ddl_multi_thread_double_pop() {
        for _ in 0..10000 {
            let ddl = DoubleLinkList::new();

            ddl.push_tail(1);

            thread::scope(|s| {
                s.spawn(|| {
                    ddl.pop_head();
                });

                s.spawn(|| {
                    ddl.pop_tail();
                });
            });

            assert!(ddl.is_empty());
        }
    }

    #[test]
    fn test_ddl_multi_thread_double_push() {
        for _ in 0..10000 {
            let ddl = DoubleLinkList::new();

            thread::scope(|s| {
                s.spawn(|| {
                    ddl.push_head(1);
                });

                s.spawn(|| {
                    ddl.push_head(2);
                });
            });

            assert!(ddl.contains(&1));
            assert!(ddl.contains(&2));
        }
    }

    #[test]
    fn test_ddl_multi_thread_push_and_pop() {
        for _ in 0..10000 {
            let ddl = DoubleLinkList::new();
            let mut popped = false;

            thread::scope(|s| {
                s.spawn(|| {
                    popped = ddl.pop_head().is_some();
                });

                s.spawn(|| {
                    ddl.push_head(1);
                });
            });

            assert!((popped && !ddl.contains(&1)) || (!popped && ddl.contains(&1)));
        }
    }
}
