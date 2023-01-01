trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

	fn is_addable(&self) -> bool {
		false
	}

    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
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
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct PendingReview {
	approved_times: u8,
}

impl PendingReview {
    fn new() -> Self {
        Self {
			approved_times: 0,
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
		if self.approved_times == 0 {
        	Box::new(Self {
				approved_times: 1,
			})
		} else {
        	Box::new(Published::new())
		}
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft::new())
    }
}

struct Draft {
}

impl Draft {
	fn new() -> Self {
		Self {
		}
	}
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn is_addable(&self) -> bool {
		true
	}
}

pub struct Post {
    state: Option<Box<dyn State>>,
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
		if let Some(state) = &self.state {
			if state.is_addable() {
        		self.content.push_str(text);
			}
		}
    }

    pub fn content(&self) -> &str {
        if let Some(state) = self.state.as_ref() {
            state.content(self)
        } else {
			"None!!!"
		}
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

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

