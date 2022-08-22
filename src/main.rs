mod llist;

use llist::LList;

fn main() {
    let mut list = LList::new();
    list.push(0, "Hello!");
    list.push(1, "World!");
    list.push(2, "a");
    list.push(3, "b");
    list.push(4, "c");

    list.insert(69, "test", 2);

    list.remove(3);
    list.pop();

    list.push_front(32, "im at the front!");

    let len = list.len();
    println!("len: {len}\n{list}");

    for x in list.by_ref() {
        println!("{x:?}");
    }
}
