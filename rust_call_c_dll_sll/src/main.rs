extern "C" {
    /// src/sll.c
    fn print_from_static_linking_library();
}

/// src/dll.c
#[link(name="dll", kind="dylib")]
extern "C" {
    pub static three: i32;
    fn print_from_dynamic_linking_library();
}

fn main() {
    unsafe {
        dbg!(three);
        assert_eq!(three, 3);
        print_from_dynamic_linking_library();
        print_from_static_linking_library();
    }
}
