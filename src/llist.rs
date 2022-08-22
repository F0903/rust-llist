use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ptr::null,
    rc::Rc,
};

type NodeElem<K, T> = Rc<RefCell<Node<K, T>>>;

#[derive(Clone, Debug)]
struct Node<K: Eq, T> {
    key: K,
    item: T,
    next: Option<NodeElem<K, T>>,
}

pub struct LList<K, T>
where
    K: Eq,
{
    start: Option<NodeElem<K, T>>,
    tip: Option<NodeElem<K, T>>,
    iter_cursor: *const Node<K, T>,
}

impl<K, T> LList<K, T>
where
    K: Clone + Eq + Debug,
    T: Clone + Debug,
{
    /// Create a new empty list.
    pub fn new() -> Self {
        LList {
            start: None,
            tip: None,
            iter_cursor: null(),
        }
    }

    fn reset_cursor(&mut self) {
        self.iter_cursor = match &self.start {
            Some(x) => x.as_ptr(),
            None => null(),
        };
    }

    fn check_if_key_exists(&mut self, key: &K) -> bool {
        let mut result = false;
        for (k, _) in self.by_ref() {
            if *key == k {
                result = true;
                break;
            }
        }
        self.reset_cursor();
        result
    }

    fn set_start(&mut self, new_start: NodeElem<K, T>) {
        self.start = Some(new_start.clone());
        self.iter_cursor = new_start.as_ptr();
    }

    fn append_tip(&mut self, key: K, item: T) {
        debug_assert!(!self.check_if_key_exists(&key));
        let next = Rc::new(RefCell::new(Node {
            key,
            item,
            next: None,
        }));
        if let Some(tip) = self.tip.clone() {
            // Set old tip to the new tip.
            tip.borrow_mut().next = Some(next.clone());
        }
        self.tip = Some(next);
        println!("Tip is now: {:?}", self.tip);
    }

    /// Push element to the back of the list.
    pub fn push(&mut self, key: K, item: T) {
        if let None = self.tip {
            let start = Rc::new(RefCell::new(Node {
                key,
                item,
                next: None,
            }));
            let start = start;
            self.start = Some(start.clone());
            self.tip = Some(start.clone());
            self.iter_cursor = start.as_ptr();
            return;
        }

        self.append_tip(key, item);
    }

    /// Push element to the start of the list.
    pub fn push_front(&mut self, key: K, item: T) {
        let cursor = match self.start.clone() {
            Some(x) => x,
            None => return,
        };
        let new_node = Rc::new(RefCell::new(Node {
            key,
            item,
            next: Some(cursor),
        }));
        self.set_start(new_node);
    }

    /// Insert element after the element at the specified index.
    pub fn insert(&mut self, key: K, item: T, index: usize) {
        debug_assert!(index < self.len());

        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return,
        };

        for _ in 0..index {
            let cur_next = cursor.borrow().next.clone();
            cursor = match cur_next {
                Some(x) => x,
                None => return,
            };
        }

        let mut cursor_node = cursor.borrow_mut();
        let cursor_old_next = cursor_node.next.clone();
        let new_node = Rc::new(RefCell::new(Node {
            key,
            item,
            next: cursor_old_next,
        }));
        cursor_node.next = Some(new_node);
    }

    // Removes the element with the specified key, and returns it's value or None if not found.
    pub fn remove(&mut self, key: K) -> Option<T> {
        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return None,
        };
        let mut back_cursor = cursor.clone();
        loop {
            if cursor.borrow().key != key {
                back_cursor = cursor.clone();
                cursor = match cursor.clone().borrow().next.clone() {
                    Some(x) => x,
                    None => return None,
                };
                continue;
            }
            let next = cursor.borrow().next.clone();
            back_cursor.borrow_mut().next = next;
            return Some(cursor.borrow().item.clone());
        }
    }

    /// Get element with the specified key, or None if none was found.
    pub fn get(&self, key: K) -> Option<T> {
        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return None,
        };
        loop {
            if cursor.borrow().key != key {
                cursor = match cursor.clone().borrow().next.clone() {
                    Some(x) => x,
                    None => return None,
                };
                continue;
            }
            return Some(cursor.borrow().item.clone());
        }
    }

    /// Get the length of the list. (requires iterating through the entire list)
    pub fn len(&self) -> usize {
        println!("Nodes in len: {:?}", self.start);
        let mut i: usize = 0;
        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return 0,
        };
        loop {
            i += 1;
            let cur_next = cursor.borrow().next.clone();
            cursor = match cur_next {
                Some(x) => x,
                None => break,
            };
        }
        i
    }
}

impl<K, T> Iterator for LList<K, T>
where
    K: Eq + Clone,
    T: Clone,
{
    type Item = (K, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_cursor.is_null() {
            return None;
        }
        loop {
            let cursor = unsafe { &*self.iter_cursor };

            let cur_key = cursor.key.clone();
            let cur_val = cursor.item.clone();

            self.iter_cursor = match cursor.next.clone() {
                Some(x) => x.as_ptr(),
                None => null(),
            };
            return Some((cur_key, cur_val));
        }
    }
}

impl<K, T> Display for LList<K, T>
where
    K: Eq + Clone + Display,
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return Ok(()),
        };
        let mut str_buf = String::new();
        loop {
            let key = cursor.borrow().key.clone();
            let item = cursor.borrow().item.clone();
            let fmt = format!("{key} -> {item}");
            str_buf.push_str(&fmt);
            str_buf.push('\n');
            let cur_next = cursor.borrow().next.clone();
            cursor = match cur_next {
                Some(x) => x,
                None => break,
            };
        }
        f.write_str(&str_buf)
    }
}
