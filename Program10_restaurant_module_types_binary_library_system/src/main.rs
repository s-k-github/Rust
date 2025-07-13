use Program10_restaurant_module_types_binary_library_system::back_of_house_with_enum::prepare_appetizer;
use Program10_restaurant_module_types_binary_library_system::front_of_house_with_struct::hosting;
pub fn eat_at_restaurant() {
    //absolute path
    hosting::add_to_wishlist();
    //relative path
    hosting::add_to_wishlist();
    hosting::Breakfast::add_item("french toast");

    let order1 = prepare_appetizer::Appetizer::soup;
    let order2 = prepare_appetizer::Appetizer::salad;
    println!("{:#?}", order1);
    println!("{:#?}", order2);
}
fn main() {
    eat_at_restaurant();
}
