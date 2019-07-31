fn main() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Pizza is the greatest!"),
        location: String::from("Beaconsfield"),
        author: String::from("Christian"),
        content: String::from("pizza is amazing")
    };

    println!("Article: {}", article.summarize());
    notify(article);
}
