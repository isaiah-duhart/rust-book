use traits::{NewsArticle, SocialPost, Summary, notify};

fn main() {
    let article = NewsArticle {
        headline: String::from("This happened!"),
        location: String::from("Atlanta, GA"),
        author: String::from("Isaiah Duhart"),
        content: String::from("This crazy thing happened today...")

    };

    let post = SocialPost {
        username: String::from("IsaiahDuhart"),
        content: String::from("This is my post"),
        reply: false,
        repost: false
    };

    println!("{}", article.summarize());
    println!("{}", post.summarize());

    notify(&article);
    notify(&post);
}
