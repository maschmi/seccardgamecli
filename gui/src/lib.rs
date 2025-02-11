use game_lib::world::game::Game;
use crate::actions::command::Command;

mod app;
mod card_window;
pub mod start;
mod actions;
mod components;
mod side_panel;

pub(crate) type CommandToExecute = Option<Command>;

pub(crate) struct SecCardGameApp {
    game: Game,
    input: Input,
    command: CommandToExecute,
}

enum Message {
    Success(String),
    Failure(String),
    Warning(String),
    None,
}

struct Input {
    next_res: String,
    pay_res: String,
    inc_reputation: String,
    dec_reputation: String,
    message: Message,
    multiplier: String,
}
