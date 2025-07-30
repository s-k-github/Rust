mod all_in_one_code;
mod dangling_reference;
mod generic_lifetime;
mod passing_invalid_reference;
mod static_lifetime;
mod struct_with_lifetime_annotations;
fn main() {
    dangling_reference::main();
    generic_lifetime::main();
    generic_lifetime::smallest_lifetime_example();
    passing_invalid_reference::passing_invalid_reference_from_function();
    struct_with_lifetime_annotations::struct_with_lifetime_annotations();
    static_lifetime::main();
    all_in_one_code::all_in_one_code("HEllo", "World", 12);
}
