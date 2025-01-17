use crate::chessboard::{ChessBoard, Square};
use crate::styles::square_container::SquareContainer;
use iced::{
    theme,
    widget::svg::{Handle, Svg},
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
        if let Some(piece) = square.piece {
            let svg_handle = Handle::from_path(piece.svg_path);
            let svg = Svg::new(svg_handle)
                .width(Length::Fill)
                .height(Length::Fill);

            Container::new(svg)
                .padding(5)
                .center_x()
                .center_y()
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0))
                .style(theme::Container::Custom(Box::new(SquareContainer::new(
                    square.color,
                ))))
                .into()
        } else {
            Container::new(Text::new(""))
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0))
                .style(theme::Container::Custom(Box::new(SquareContainer::new(
                    square.color,
                ))))
                .into()
        }
    }
}
