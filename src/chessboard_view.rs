use crate::chessboard::{ChessBoard, Square};
use crate::styles::square_container::SquareContainer;
use iced::{
    theme,
    widget::{Column, Container, Row, Text},
    Alignment, Element, Length,
};

pub struct ChessBoardView {
    chess_board: ChessBoard,
}

impl ChessBoardView {
    pub fn new() -> Self {
        ChessBoardView {
            chess_board: ChessBoard::new(),
        }
    }

    pub fn view(&self) -> Element<()> {
        let mut rows = Column::new().spacing(0).align_items(Alignment::Center);

        for row in &self.chess_board.squares {
            let mut cols = Row::new().spacing(0).align_items(Alignment::Center);

            for square in row {
                cols = cols.push(self.square_view(square));
            }

            rows = rows.push(cols);
        }

        Container::new(rows)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn square_view(&self, square: &Square) -> Element<()> {
        let mut content = Container::new(Text::new(""));

        if let Some(piece) = square.piece {
            let piece_symbol = piece.to_unicode();
            let piece_color = match piece.color {
                crate::chessboard::PieceColor::White => iced::Color::WHITE,
                crate::chessboard::PieceColor::Black => iced::Color::BLACK,
            };
            content = Container::new(Text::new(piece_symbol).size(40).style(piece_color))
                .center_x()
                .center_y();
        }

        content
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(60.0))
            .style(theme::Container::Custom(Box::new(SquareContainer::new(
                square.color,
            ))))
            .into()
    }
}
