mod hashmaps;
mod storing_enum_inside_vector;
mod strings;
fn main() {
    let array = [12, 3, 4];
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    println!("{:?}", vector);

    let vector_define_with_declare = vec![1, 2, 3];
    println!("{:?}", vector_define_with_declare);
    {
        let access_vector_outside_scope = vec![1, 2, 3];
    }
    // println!("{:?}", access_vector_outside_scope);

    //iterate over vector
    //access 3rd indec
    let access_by_index = vec![5, 6, 7, 8, 9];
    println!("{}", &access_by_index[3]);

    //above we can enter invalid index such as 20 and get index_out_of_bound
    //hence get method
    //handle index out of bound errors
    match access_by_index.get(2) {
        Some(n) => println!("3rd element is {}", n),
        None => println!("No such index present"),
    }

    //ownership issues
    let mut ownership_issue = vec![23, 34, 56, 78];
    let third = &ownership_issue[2];
    // ownership_issue.push(12); //error coz you cant borrown same variable as mutable and immutable
    println!("{}", third);

    //iterate over vectore
    for i in &mut ownership_issue {
        *i += 1;
        println!("{}", i);
    }

    storing_enum_inside_vector::fn_storing_enum_inside_vec();
    strings::sample_strings();
    strings::append_to_string();
    strings::append_with_without_ownership();
    strings::indexing_in_string();

    hashmaps::hashmaps();
    hashmaps::update_hashmap();
    hashmaps::count_words();
}
