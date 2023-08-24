#![allow(unused_variables)]
#![allow(dead_code)]
// note that the internal fields of the struct are private because we don't want
// programs to interact with them directly, and to use our API instead
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
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // as_ref allows us to get a reference to the inner Box inside of the 'state' field Option
        // we then call the content() method defined in the State trait with self (the post)
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        // take() extracts the value from the 'state' field Option and replaces it with None
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()); //this is the method defined in the State trait
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

// our posts hold an Option with a Box containing a Trait Object that implements this trait
// in other words, the 'state' field can contain any type that implements this trait
// furthermore, since it is a trait we will define behavior that is shared by all state types
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // default implementation for content will just return "" if
    // a any of the states did not implement this method
    // the lifetime specifier is needed because we pass two references and return a reference
    // obviously the returned reference is to the content in post, so it should live as long as post
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    // when we call the request_review method on a Draft state, we will
    // receive a Box containing a PendingReview state (which will be updated in the Post object)
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    // when we call the request_review method on the PendingReview state, we will
    // receive a a Box containing a PendingReview state because a post that is pending
    // review should stay in that state if it has a review requested
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // if the PendingReview state has approve() called on it, it will
    // return a published state so Post can change it's state to Published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    // we don't expect to change states from published when a review is requested
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // neither do we expect to change state when an approval comes in because we are already approved
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // we only want to stray from the default implementation and actually return post content on published posts
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
