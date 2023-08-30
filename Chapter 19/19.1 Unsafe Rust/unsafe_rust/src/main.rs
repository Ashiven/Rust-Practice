use std::slice;

fn main() {
    let mut num = 5;

    // we can create raw pointers outside of unsafe blocks but we can't dereference them
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // if we tried to create an immutable reference and a mutable reference and tried
    // to read them we would have a problem with the ownership rules

    // creating a raw pointer to arbitrary memory
    let address = 0x012345usize;
    let r = address as *const i32;

    // now we can deref the raw pointer in an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // using unsafe to call unsafe methods
    unsafe fn dangerous() {}

    // dangerous() has been declared as unsafe function so we call it in an unsafe block
    unsafe {
        dangerous();
    }

    // implementing an unsafe function that is safe but the compiler doesn't know
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr(); // returns a raw pointer of type *mut i32

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // calling external code with extern
    // here we have to specify the name of the function we want to use and it's signature
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // creating an interface for other languages to use rust via the extern keyword

    // here we make our function accessible through C code after it's compiled
    // to a shared library and linked from C

    // important to note that we use the 'no_mangle' attribute to prevent
    // the compiler from renaming the function at compile time
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // accessing and modifying mutable static variables

    // this is a static variable, meaning it holds an immutable ref with a static lifetime
    static HELLO_WORLD: &str = "Hello, world!";

    // this is the same thing but mutable
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        // mutating the mutable static variable is an unsafe operation (global access, data races)
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(4);
    unsafe {
        // accessing it is also unsafe due to the previously mentioned reasons
        println!("COUNTER: {}", COUNTER);
    }

    // implementing unsafe traits (has at least one unsafe method)

    unsafe trait Foo {
        // unsafe method
    }

    unsafe impl Foo for i32 {
        // unsafe method implementation
    }
}
