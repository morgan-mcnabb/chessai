use iced::Color;

#[derive(Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Copy, Clone)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

pub struct ChessBoard {
    pub squares: [[Square; 8]; 8],
}

#[derive(Clone, Copy)]
pub struct Square {
    pub color: Color,
    pub piece: Option<Piece>,
}

impl Piece {
    pub fn to_unicode(&self) -> &'static str {
        match (self.color, self.piece_type) {
            (PieceColor::White, PieceType::King) => "♔",
            (PieceColor::White, PieceType::Queen) => "♕",
            (PieceColor::White, PieceType::Rook) => "♖",
            (PieceColor::White, PieceType::Bishop) => "♗",
            (PieceColor::White, PieceType::Knight) => "♘",
            (PieceColor::White, PieceType::Pawn) => "♙",
            (PieceColor::Black, PieceType::King) => "♚",
            (PieceColor::Black, PieceType::Queen) => "♛",
            (PieceColor::Black, PieceType::Rook) => "♜",
            (PieceColor::Black, PieceType::Bishop) => "♝",
            (PieceColor::Black, PieceType::Knight) => "♞",
            (PieceColor::Black, PieceType::Pawn) => "♟︎",
        }
    }
}
impl ChessBoard {
    pub fn new() -> Self {
        let squares = Self::initialize_squares();
        ChessBoard { squares }
    }

    fn initialize_squares() -> [[Square; 8]; 8] {
        let mut squares = [[Square {
            color: Color::WHITE,
            piece: None,
        }; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                let is_light_square = (row + col) % 2 == 0;
                squares[row][col].color = if is_light_square {
                    Color::from_rgb(0.9, 0.9, 0.9)
                } else {
                    Color::from_rgb(0.2, 0.2, 0.2)
                };

                squares[row][col].piece = Self::initialize_piece(row, col);
            }
        }

        squares
    }

    fn initialize_piece(row: usize, col: usize) -> Option<Piece> {
        match row {
            0 => {
                // Black major pieces
                Some(Piece {
                    piece_type: match col {
                        0 | 7 => PieceType::Rook,
                        1 | 6 => PieceType::Knight,
                        2 | 5 => PieceType::Bishop,
                        3 => PieceType::Queen,
                        4 => PieceType::King,
                        _ => unreachable!(),
                    },
                    color: PieceColor::Black,
                })
            }
            1 => {
                // Black pawns
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: PieceColor::Black,
                })
            }
            6 => {
                // White pawns
                Some(Piece {
                    piece_type: PieceType::Pawn,
                    color: PieceColor::White,
                })
            }
            7 => {
                // White major pieces
                Some(Piece {
                    piece_type: match col {
                        0 | 7 => PieceType::Rook,
                        1 | 6 => PieceType::Knight,
                        2 | 5 => PieceType::Bishop,
                        3 => PieceType::Queen,
                        4 => PieceType::King,
                        _ => unreachable!(),
                    },
                    color: PieceColor::White,
                })
            }
            _ => None, // Empty squares
        }
    }
}
