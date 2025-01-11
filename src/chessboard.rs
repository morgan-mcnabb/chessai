use iced::Color;

pub struct ChessBoard {
    pub squares: [[Square; 8]; 8],
}

#[derive(Clone, Copy)]
pub struct Square {
    pub color: Color,
}

impl ChessBoard {
    pub fn new() -> Self {
        let squares = Self::initialize_squares();
        ChessBoard { squares }
    }

    fn initialize_squares() -> [[Square; 8]; 8] {
        let mut squares = [[Square { color: Color::WHITE }; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                let is_light_square = (row + col) % 2 == 0;
                squares[row][col].color = if is_light_square {
                    Color::from_rgb(0.9, 0.9, 0.9) 
                } else {
                    Color::from_rgb(0.2, 0.2, 0.2) 
                };
            }
        }

        squares
    }
}
