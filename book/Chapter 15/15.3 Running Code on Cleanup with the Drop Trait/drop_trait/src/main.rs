struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let f = CustomSmartPointer {
        data: String::from("more stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(f); // dropping f before the end of it's scope via std::mem::drop
}
