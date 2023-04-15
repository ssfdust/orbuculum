mod services;
mod utils;
mod views;
use std::sync::Arc;

use eyre::Result;
use requestty::{ListItem, Question};
use views::{greeters::greeter, nm::draw_nm_ui};
use terminal::{Clear, Action};

fn enter_to_continue() {
    let question = Question::input("enter").message("Please enter to continue").build();
    requestty::prompt_one(question).unwrap();
}

pub async fn mainloop(grpc_addr: Arc<&str>) -> Result<()> {
    loop {
        let terminal = terminal::stdout();
        terminal.act(Action::ClearTerminal(Clear::All))?;
        terminal.act(Action::MoveCursorTo(0, 0))?;
        let some_action = greeter()?;
        match some_action.as_list_item() {
            Some(ListItem { index: _, text }) if text == "Network" => {
                draw_nm_ui(grpc_addr.clone()).await?;
            }
            _ => break,
        }
        enter_to_continue();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
