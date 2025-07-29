pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
    fn summarize_author(&self) -> String;
}
pub trait Display {
    fn summarize_display(&self) -> String {
        String::from("Read more...")
    }
    fn summarize_author_display(&self) -> String;
}
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!(
            "{}, by {} with {}",
            self.headline, self.author, self.content
        )
    }
}
impl Display for NewsArticle {
    fn summarize_author_display(&self) -> String {
        format!(
            "{}, by {} with {}",
            self.headline, self.author, self.content
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}, by {} . Reply:", self.username, self.content)
    }
}
pub fn trait_as_parameter(item: &impl Summary) {
    println!("Breaking news : {}", item.summarize_author())
}
//below is called trait bounds
pub fn trait_as_parameter_longer_version<T: Summary>(item: &T) {
    println!("Breaking news : {}", item.summarize_author())
}

pub fn trait_as_parameter_2(item1: &(impl Summary + Display), item2: &impl Summary) {
    println!("Breaking news : {}", item1.summarize_author_display());
    println!("Breaking news : {}", item2.summarize_author());
}
pub fn trait_as_parameter_trait_bounds<T: Summary + Display, U: Summary>(item1: &T, item2: &U) {
    println!("Breaking news : {}", item1.summarize_author_display());
    println!("Breaking news : {}", item2.summarize_author());
}
pub fn where_clause_to_fix_trait_bounds<T, U>(t: &T, u: &U)
where
    T: Summary + Display,
    U: Summary,
{
    println!("Breaking news : {}", t.summarize_author_display());
    println!("Breaking news : {}", u.summarize_author());
}
pub fn use_trait() {
    println!("trait----------------------------------------------------------------------->");
    let article = NewsArticle {
        author: String::from("supriya"),
        headline: String::from("learning rust"),
        content: String::from("its feels great learning new language"),
    };
    println!("Article {}", article.summarize());
    let tweet = Tweet {
        username: String::from("supriya"),
        content: String::from("This is first tweet"),
        reply: false,
        retweet: true,
    };
    println!("Tweet {}", tweet.summarize_author());
    println!(
        "trait_as_parameter----------------------------------------------------------------------->"
    );
    trait_as_parameter(&article);
    println!(
        "trait_as_parameter_longer_version----------------------------------------------------------------------->"
    );
    trait_as_parameter_longer_version(&article);
    println!(
        "trait_as_parameter_2----------------------------------------------------------------------->"
    );
    trait_as_parameter_2(&article, &tweet);
    println!(
        "trait_as_parameter_trait_bounds----------------------------------------------------------------------->"
    );
    trait_as_parameter_trait_bounds(&article, &tweet);
    println!(
        "where_clause_to_fix_trait_bounds----------------------------------------------------------------------->"
    );
    where_clause_to_fix_trait_bounds(&article, &tweet);
}
