//! ### Tools module
use eyre::{Result, bail};
use requestty::{prompt_one, Question};
pub trait QuestionOnce<T> {
    fn execute(&self) -> Result<&T>;
}

pub struct QuestionText<'a, U> {
    name: &'a str,
    message: &'a str,
    choices: &'a Vec<String>,
    items: &'a Vec<U>,
}

impl<'a, U> QuestionText<'a, U> {
    pub fn new(
        name: &'a str,
        message: &'a str,
        choices: &'a Vec<String>,
        items: &'a Vec<U>,
    ) -> Self {
        Self {
            name,
            message,
            choices,
            items,
        }
    }
}

impl<'a, U> QuestionOnce<U> for QuestionText<'a, U> {
    fn execute(&self) -> Result<&U> {
        let page_size = get_left_linenum()?;
        let question = Question::select(self.name)
            .message(self.message)
            .page_size(page_size as usize)
            .choices(self.choices)
            .build();
        let answer = prompt_one(question)?;
        let index = answer
            .as_list_item()
            .and_then(|item| Some(item.index))
            .unwrap();
        let choice = self.items.get(index).unwrap();
        Ok(choice.clone())
    }
}

pub fn get_left_linenum() -> Result<i32> {
    let mut row = -1;
    let mut current_row = -1;
    let terminal = terminal::stdout();
    let term_size = terminal.get(terminal::Value::TerminalSize)?;
    match term_size {
        terminal::Retrieved::TerminalSize(_, trow) => row = trow as i32,
        _ => bail!("Failed to get terminal size")
    }
    let cursor_pos = terminal.get(terminal::Value::CursorPosition)?;
    match cursor_pos {
        terminal::Retrieved::CursorPosition(_, y) => current_row = y as i32,
        _ => bail!("Failed to get cursor position")
    }
    let left_rows = row - current_row - 2;
    let page_size = (left_rows / 11) * 11 + 2;
    println!("{}", page_size);
    Ok(page_size)
}
