
mod chessboard;
mod chessboard_view;
mod styles; // Ensure this correctly points to your styles module

use chessboard_view::ChessBoardView;
use iced::{
    executor, window, Application, Command, Element, Settings, Theme, Size,
};

pub fn main() -> iced::Result {
    ChessBot::run(Settings {
        window: window::Settings {
            size: Size::new(480.0, 480.0), // Use `Size::new` instead of a tuple
            resizable: false,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct ChessBot {
    chessboard_view: ChessBoardView,
}

type Message = (); // Define Message as `()` since there are no messages yet

impl Application for ChessBot {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme; // Default theme
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            ChessBot {
                chessboard_view: ChessBoardView::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Chess Bot")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        self.chessboard_view.view()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
