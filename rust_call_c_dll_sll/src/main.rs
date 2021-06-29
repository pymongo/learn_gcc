#[link(name="sll", kind="static")] // optional
extern "C" {
    fn print_from_static_linking_library();
}

/// src/dll.c
#[link(name="dll", kind="dylib")] // optional
extern "C" {
    // non-const int from c
    // [NOTE!] Rust用static mut绑定C不可变的int，运行时修改这个const int会segfault
    static mut const_int_one: i32;
    fn print_from_dynamic_linking_library();
}

fn main() {
    unsafe {
        assert_eq!(const_int_one, 1);
        const_int_one += 1;
        assert_eq!(const_int_one, 2);        
        print_from_dynamic_linking_library();
        print_from_static_linking_library();
    }
}
