//Implementing an Object-Oriented Design Pattern

/*
We’ll implement a blog post workflow in an incremental way. The blog’s final functionality will look like this:

    1.A blog post starts as an empty draft.
    2.When the draft is done, a review of the post is requested.
    3.When the post is approved, it gets published.
    4.Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
*/

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(self.state.as_ref().unwrap().add_text(text));
    }
    pub fn content(&mut self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(x) = self.state.take() {
            self.state = Some(x.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(x) = self.state.take() {
            self.state = Some(x.approve());
        }
    }
    pub fn reject(&mut self) {
        if let Some(x) = self.state.take() {
            self.state = Some(x.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        text
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingFinalReview {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct PendingFinalReview {}

impl State for PendingFinalReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}