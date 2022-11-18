use oop4::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    
    let post = post.approve();

    assert_eq!(post.content(), "I ate a salad for lunch today");
}