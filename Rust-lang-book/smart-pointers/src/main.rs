use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 1. Drop Trait Example
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // 2. Box<T> Example: heap allocation
    let b = Box::new(10);
    println!("Boxed value: {}", b);

    // 3. Rc<T> Example: multiple ownership
    let rc_a = Rc::new(RefCell::new(5));
    println!("Initial Rc count: {}", Rc::strong_count(&rc_a));

    // Clone increases reference count
    let rc_b = Rc::clone(&rc_a);
    println!("Rc count after clone: {}", Rc::strong_count(&rc_a));

    // 4. RefCell<T> Example: interior mutability
    *rc_a.borrow_mut() += 10;
    println!("Value after mutation via RefCell: {}", rc_b.borrow());

    // 5. Drop Trait Example
    let dsp = CustomSmartPointer { data: String::from("Hello") };
    println!("CustomSmartPointer created");

    // 6. Reference Cycle Example
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: RefCell<Option<Rc<Node>>>,
    }

    let first = Rc::new(Node { value: 1, next: RefCell::new(None) });
    let second = Rc::new(Node { value: 2, next: RefCell::new(None) });

    *first.next.borrow_mut() = Some(Rc::clone(&second));
    // Creating a cycle
    //*second.next.borrow_mut() = Some(Rc::clone(&first)); // Uncommenting will create a cycle

    println!("First node: {:?}", first);
    println!("Second node: {:?}", second);

    // Weak<T> can be used to break cycles (not shown here)
}
