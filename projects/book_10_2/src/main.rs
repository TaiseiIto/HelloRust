trait Summary {
	fn summarize(&self) -> String;
}

#[derive(Debug)]
struct NewsArticle {
	headline: String,
	location: String,
	author: String,
	content: String,
}

impl NewsArticle {
	fn new(headline: &str, location: &str, author: &str, content: &str) -> NewsArticle {
		NewsArticle {
			headline: headline.to_string(),
			location: location.to_string(),
			author: author.to_string(),
			content: content.to_string(),
		}
	}
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, by {} ({}) : {}", self.headline, self.author, self.location, self.content)
	}
}

#[derive(Debug)]
struct Tweet {
	username: String,
	content: String,
	reply: bool,
	retweet: bool,
}

impl Tweet {
	fn new(username: &str, content: &str, reply: bool, retweet: bool) -> Tweet {
		Tweet {
			username: username.to_string(),
			content: content.to_string(),
			reply,
			retweet,
		}
	}
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!(
			"{}{}{} by {}",
			match self.reply {
				true => "reply ",
				false => "",
			},
			match self.retweet {
				true => "retweet ",
				false => "",
			},
			self.content,
			self.username,
		)
	}
}

fn main() {
	let news_article: NewsArticle = NewsArticle::new("Hello", "World", "Taisei", "lol");
	let summary: String = news_article.summarize();
	println!("{}", summary);
	let tweet = Tweet::new("Taisei", "lol", true, false);
	let summary: String = tweet.summarize();
	println!("{}", summary);
}

