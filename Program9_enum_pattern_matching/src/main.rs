mod type1_basic_enums;
mod type2_enums_with_data;
mod type3_option_enum;
fn main() {
    type1_basic_enums::only_enum();
    type1_basic_enums::enum1();

    type2_enums_with_data::main();

    type3_option_enum::option_enum();
}
