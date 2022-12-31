trait State {
    fn request_review(self: Box<Self>) -> Box<Self>;

    fn approve(self: Box<Self>) -> Box<Self>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        post.content
    }
}

struct Published {
}

impl Published {
    fn new() -> Self {
        Self {
        }
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<Self> {
        self
    }

    fn approve(self: Box<Self>) -> Box<Self> {
        self
    }
}

struct PendingReview {
}

impl PendingReview {
    fn new() -> Self {
        Self {
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<Self> {
        self
    }

    fn approve(self: Box<Self>) -> Box<Self> {
        Box::new(Published::new())
    }
}

struct Draft {
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<Self> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<Self> {
        self
    }
}

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        if let Some(state) = self.state.as_ref() {
            state.content(self)
        } else "None!!!"
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

