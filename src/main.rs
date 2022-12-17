use iced::{Sandbox, Settings};
use reversi_iced::*;
mod test;
fn main() -> iced::Result {
    Game::run(Settings::default())
}
