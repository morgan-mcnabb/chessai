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
    pub svg_path: &'static str,
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
    pub fn new(piece_type: PieceType, color: PieceColor) -> Self {
        let svg_path = match (color, piece_type) {
            (PieceColor::White, PieceType::King) => r"assets/white-king.svg",
            (PieceColor::White, PieceType::Queen) => r"assets/white-queen.svg",
            (PieceColor::White, PieceType::Rook) => r"assets/white-rook.svg",
            (PieceColor::White, PieceType::Bishop) => r"assets/white-bishop.svg",
            (PieceColor::White, PieceType::Knight) => r"assets/white-knight.svg",
            (PieceColor::White, PieceType::Pawn) => r"assets/white-pawn.svg",
            (PieceColor::Black, PieceType::King) => r"assets/black-king.svg",
            (PieceColor::Black, PieceType::Queen) => r"assets/black-queen.svg",
            (PieceColor::Black, PieceType::Rook) => r"assets/black-rook.svg",
            (PieceColor::Black, PieceType::Bishop) => r"assets/black-bishop.svg",
            (PieceColor::Black, PieceType::Knight) => r"assets/black-knight.svg",
            (PieceColor::Black, PieceType::Pawn) => r"assets/black-pawn.svg",
        };

        Piece {
            piece_type,
            color,
            svg_path,
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
                Some(Piece::new(
                    match col {
                        0 | 7 => PieceType::Rook,
                        1 | 6 => PieceType::Knight,
                        2 | 5 => PieceType::Bishop,
                        3 => PieceType::Queen,
                        4 => PieceType::King,
                        _ => unreachable!(),
                    },
                    PieceColor::Black,
                ))
            }
            1 => {
                // Black pawns
                Some(Piece::new(PieceType::Pawn, PieceColor::Black))
            }
            6 => {
                // White pawns
                Some(Piece::new(PieceType::Pawn, PieceColor::White))
            }
            7 => {
                // White major pieces
                Some(Piece::new(
                    match col {
                        0 | 7 => PieceType::Rook,
                        1 | 6 => PieceType::Knight,
                        2 | 5 => PieceType::Bishop,
                        3 => PieceType::Queen,
                        4 => PieceType::King,
                        _ => unreachable!(),
                    },
                    PieceColor::White,
                ))
            }
            _ => None, // Empty squares
        }
    }
}
