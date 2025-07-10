mod samples;
fn main() {
    samples::immutable_struct();
    samples::mutable_struct();
    samples::create_new_customer(123, String::from("Duck12"), false);
    samples::use_struct_as_param();
}
