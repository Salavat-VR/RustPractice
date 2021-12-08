use std::ops::Deref;

fn main() {
    //let my_str = 5;
    //let ref_my_str = &my_str;

    //println!("my str is {}", my_str);
    //assert_eq!(my_str, *ref_my_str);

    let csp = CustomSmartPointer {
        data: String::from("dmytro"),
    };
    println!("do some useful stuff and then out of scope");
}

struct CustomSmartPointer {
    data: String,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(item: T) -> MyBox<T> {
        MyBox(item)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("{}\t\t value out of scope!", self.data)
    }
}
