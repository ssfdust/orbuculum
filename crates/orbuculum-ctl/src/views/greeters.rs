//! ## Greeter module
//!
use eyre::Result;
use requestty::{prompt_one, Answer, DefaultSeparator, Question};

pub fn greeter() -> Result<Answer> {
    let prompt = Question::select("theme")
        .message("Please select action:")
        .choices(vec!["Network".into(), DefaultSeparator, "Quit".into()])
        .build();
    let data = prompt_one(prompt)?;
    Ok(data)
}
