mod custom_iterator;
mod filter_example;
mod fold_example;
mod iter_example;
mod iter_into_example;
mod iter_mut_example;
mod map_example;
mod test;
mod zip_example;
fn main() {
    iter_example::main();
    iter_mut_example::main();
    iter_into_example::main();
    map_example::main();
    filter_example::main();
    fold_example::main();
    zip_example::main();
}
