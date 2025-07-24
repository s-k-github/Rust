use std::collections::HashMap;
pub fn hashmaps() {
    //team blue and red scores

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 23);
    scores.insert(String::from("Red"), 46);
    get_team_details(&scores);
    get_particular_team_score(String::from("Blue"), &scores);
}
fn get_team_details(score: &HashMap<String, i32>) {
    for (key, value) in score {
        println!("{} {}", key, value)
    }
}
fn get_particular_team_score(team_name: String, score: &HashMap<String, i32>) {
    println!("{:?}", score.get(&team_name));
    match score.get(&team_name) {
        Some(s) => println!("Score for {} is {}", team_name, s),
        None => println!("Team {} not found", team_name),
    }
}

pub fn update_hashmap() {
    let mut data: HashMap<String, i32> = HashMap::new();
    data.insert(String::from("Blue"), 10);
    data.insert(String::from("Blue"), 20); //this line will update blue's value directly

    data.entry(String::from("Red")).or_insert(30); //create entry
    data.entry(String::from("Red")).or_insert(40); //since already exist dont do anything
    get_particular_team_score(String::from("Red"), &data)
}

pub fn count_words() {
    let text = "Hello world , How's World";
    let mut map = HashMap::new();
    for i in text.split_whitespace() {
        let count = map.entry(i).or_insert(0);
        *count += 1; //deferenece
    }
    println!("{:?}", map)
}
