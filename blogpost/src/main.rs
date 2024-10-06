fn main() {
    let mut post = Post::new();
    assert_eq!("", post.content());  

    post.add_text("I ate salad for lunch");
    // post message is only avaialbe if post is approved
    assert_eq!("", post.content());  

    post.request_review();
    // post message is only avaialbe if post is approved
    assert_eq!("", post.content());  

    post.approve();
    // post message is only avaialbe if post is approved
    assert_eq!("I ate salad for lunch", post.content());  
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        // "state" is an Option value. 
        // We call as_ref() to get a reference to the value inside instead of obtaining the ownership
        self.state.as_ref().unwrap().content(self)            
    }

    pub fn add_text(&mut self, text: &str) {
        let _ = &self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }

    }
}
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // we return a string slice from a String that is owned by the post so we have to add lifespans
    fn content<'a>(&self, _post:&'a Post) -> &'a str {
        ""
    }
}     

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self    
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
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

    fn content<'a>(&self, post:&'a Post) -> &'a str {
        &post.content   
    }
}
