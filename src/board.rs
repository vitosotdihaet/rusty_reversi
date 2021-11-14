use std::fmt;

#[derive(Clone, Copy)]
pub struct Board {
    grid: [[Option<Color>; 8]; 8],
    pub current_color: Color,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self {
            grid: [[None; 8]; 8], 
            current_color: Color::Black,
        };

        board.grid[3][3] = Some(Color::White);
        board.grid[3][4] = Some(Color::Black);
        board.grid[4][3] = Some(Color::Black);
        board.grid[4][4] = Some(Color::White);

        board
    }

    pub fn insert(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        if self.grid[x][y].is_some() {
            return Err("Cell already occupied.")
        }
        self.grid[x][y] = Some(self.current_color);
        self.current_color = !self.current_color;
        Ok(())
    }

    pub fn turn(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        let can_eat = self.cell_can_eat(x, y);
        if can_eat.is_empty() {
            return Err("No cells to be eaten.");
        }
        self.insert(x, y)?;
        for elem in can_eat {
            self.grid[elem.0][elem.1] = Some(!self.current_color);
        }
        Ok(())
    }

    pub fn cell_can_eat(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut can_eat = Vec::new();

        let x = x as i8;
        let y = y as i8;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                };

                match (x + dx, y + dy) {
                    (0..=7, 0..=7) => {}
                    _ => continue,
                }

                let mut x_cur = (x + dx) as usize;
                let mut y_cur = (y + dy) as usize;

                let mut can_eat_in_direction = Vec::new();
                while self.grid[x_cur][y_cur] == Some(!self.current_color) {
                    can_eat_in_direction.push((x_cur, y_cur));

                    match (x_cur as i8 + dx, y_cur as i8 + dy) {
                        (0..=7, 0..=7) => {}
                        _ => break,
                    }

                    x_cur = (x_cur as i8 + dx) as usize;
                    y_cur = (y_cur as i8 + dy) as usize;
                }

                if self.grid[x_cur][y_cur] == Some(self.current_color) {
                    can_eat.extend(can_eat_in_direction)
                }
            }
        }
        can_eat
    }

    pub fn has_moves(&self) -> bool {
        for x in 0..=7 {
            for y in 0..=7 {
                if self.grid[x][y].is_some() {
                    continue;
                }

                if !self.cell_can_eat(x, y).is_empty() {
                    return true;
                }
            }
        }

        false
    }

    pub fn draw(&self) {
        println!("  1 2 3 4 5 6 7 8");
        for y in 0..8 {
            print!("{} ", y + 1);
            for x in 0..8 {
                print!(
                    "{}",
                    if let Some(i) = &self.grid[x][y] {
                        format!("{} ", i)
                    } else {
                        String::from("▢ ")
                    }
                )
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        if let Color::White = self {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::White => '⦾',
                Color::Black => '⦿',
            }
        )
    }
}
