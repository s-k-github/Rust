mod enum_generic_type;
mod generic_types;
mod struct_generic_type;
mod struct_generic_type_impl;
fn main() {
    generic_types::largest_of_vector();
    struct_generic_type::struct_generic();
    enum_generic_type::enum_generic();
    struct_generic_type_impl::struct_generic_type_with_impl();
    struct_generic_type_impl::struct_with_different_types();
}
