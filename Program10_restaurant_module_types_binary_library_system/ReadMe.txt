NOTE : This project created using cargo new --lib project_name

When we run cargo new project_new it creates a folder structure where 
    main.rs is a crate -> binary crate -> produces and executable. 
    if you create lib.rs->library crate->Produces a reusable library (.rlib)
    src/
    ├── main.rs   <-- calls into library
    ├── lib.rs    <-- library root
create contain modules, which help you organize a chunk of code and control the privacy rules.
for example a library crate needs an authentication module. we can then we can make all module private but one login method public to expose ouside the module.
Workspaces:
    A workspace is a set of multiple packages (crates) that share the same Cargo.lock and target/ directory.
 Why Use Workspaces?
    1. Share dependencies across crates
    2. Manage multiple binaries/libraries together
    3. Build/test all at once
    4. Ideal for microservices, CLI+lib combo, plugin systems
    my_workspace/             ← Workspace root
    │
    ├── Cargo.toml            ← Defines [workspace] and members
    │
    ├── crates/
    │   ├── lib_common/       ← Library crate
    │   │   └── src/
    │   │       └── lib.rs    ← Exposes pub functions
    │   │
    │   ├── app_cli/          ← Binary crate using lib_common
    │   │   └── src/
    │   │       └── main.rs   ← Calls lib_common::greet()
    │   │
    │   └── app_web/          ← Another binary crate (optional)
    │       └── src/
    │           └── main.rs

Rust convention for binary crate:
    if we have main.rs in defined source directory , then a binary crate with the same name as package will be created and main.rs will be crate rule.

Rust convention for library crate:
    if lib.rs is there in src then rust will take it as root with creating application named as package
rules:
    1. package must have at least one crate
    2. package could wither have 1 or no library crate
    3. package could have n number of binary crate
these binary crates go in bin folder
src->bin->binary crate

example created here
    create 
        front_of_house
            hosting
                add_to_wishlist
                seat_at_table
            serving
                take_order
                serve_order
                take_payment


using full path in mod is not productive hence we Use
    use keyword. we get the module in scope and use it directly
    use Program10_restaurant_module_types_binary_library_system::back_of_house_with_enum::prepare_appetizer;
    pub use Program10_restaurant_module_types_binary_library_system::back_of_house_with_enum::prepare_appetizer;
