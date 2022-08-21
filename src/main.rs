mod llist;

use llist::LList;

fn main() {
    let mut list = LList::new();
    list.push(1, "Hello!");
    list.push(2, "World!");
    list.push(3, "a");
    list.push(4, "b");
    list.push(5, "c");

    let len = list.len();
    println!("len: {len}\n{list}");

    for x in list {
        println!("{x:?}");
    }
}
