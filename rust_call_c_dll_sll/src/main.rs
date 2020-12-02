extern "C" {
    /// function prototype defined in src/dll.c
    fn print_from_dynamic_linking_library();
    /// function prototype defined in src/sll.c
    fn print_from_static_linking_library();
}

fn main() {
    unsafe {
        print_from_dynamic_linking_library();
        print_from_static_linking_library();
    }
}
