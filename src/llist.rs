use std::{
    borrow::Borrow,
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
    K: Eq + Debug,
    T: Clone + Debug,
{
    pub fn new() -> Self {
        LList {
            start: None,
            tip: None,
            iter_cursor: null(),
        }
    }

    #[cfg(debug_assertions)]
    fn check_if_key_exists(&self, key: &K) {
        for node in self.start.borrow() {
            if *key == node.as_ref().borrow().key {
                panic!("Key already exists in list!");
            }
        }
    }

    fn append_tip(&mut self, key: K, item: T) {
        #[cfg(debug_assertions)]
        self.check_if_key_exists(&key);
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

    pub fn insert(&mut self, key: K, item: T, index: usize) {
        todo!();

        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return,
        };
        for _ in 0..index {
            cursor = match cursor.as_ref().borrow().next.clone() {
                Some(x) => x,
                None => return,
            };
        }

        let cursor_old_next = cursor.as_ref().borrow().next.clone();
        let new_node = Node {
            key,
            item,
            next: cursor_old_next,
        };
    }

    pub fn get(&self, key: K) -> Option<T> {
        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return None,
        };
        loop {
            if cursor.as_ref().borrow().key != key {
                cursor = match cursor.clone().as_ref().borrow().next.clone() {
                    Some(x) => x,
                    None => return None,
                };
                continue;
            }
            return Some(cursor.as_ref().borrow().item.clone());
        }
    }

    pub fn len(&self) -> usize {
        println!("Nodes in len: {:?}", self.start);
        let mut i: usize = 0;
        let mut cursor = match self.start.clone() {
            Some(x) => x,
            None => return 0,
        };
        loop {
            i += 1;
            let cur_next = cursor.as_ref().borrow().next.clone();
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
            let key = cursor.as_ref().borrow().key.clone();
            let item = cursor.as_ref().borrow().item.clone();
            let fmt = format!("{key} -> {item}");
            str_buf.push_str(&fmt);
            str_buf.push('\n');
            let cur_next = cursor.as_ref().borrow().next.clone();
            cursor = match cur_next {
                Some(x) => x,
                None => break,
            };
        }
        f.write_str(&str_buf)
    }
}
