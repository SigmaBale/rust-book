use oop3::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("Monke");

    post.approve();
    assert_eq!("", post.content());

    post.add_text("Monke");

    post.approve();
    post.add_text("Monke");
    assert_eq!("I ate a salad for lunch today", post.content());
}