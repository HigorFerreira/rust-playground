pub mod aggregator {
    use std::fmt::Display;

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String
    }
    
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
    
    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }
    
    impl Summary for  SocialPost {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    /**
     * Using impl Trait is appropriate if we want this function
     * to allow item1 and item2 to have different types
     * (as long as both types implement Summary).
     * If we want to force both parameters to have the same type,
     * however, we must use a trait bound, like this:
     */
    pub fn notify<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
}