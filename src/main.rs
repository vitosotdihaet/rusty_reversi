mod board;
use board::*;


fn input_coordinate() -> Result<usize, &'static str> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut num = buf.trim().parse::<usize>();

    while let Err(err) = num {
        println!("{}", err);
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        num = buf.trim().parse::<usize>();
    }
    let num = num.unwrap() - 1;

    if let 0..=7 = num {
        Ok(num)
    } else {
        Err("Input out of bounds.")
    }
}

fn main() {
    let mut board = Board::new();
    let mut game_over = false;

    loop {
        print!("{}[2J", 27 as char);

        if game_over && !board.has_moves() {
                println!("End of a game!");
                break
        }
        game_over = board.has_moves();

        println!("It is {}'s turn!", board.current_color);
        let (x, y);

        board.draw();

        println!("Input X value: ");
        let result = input_coordinate();
        if let Ok(num) = result {
            x = num
        } else {
            println!("{}", result.unwrap_err());
            continue;
        }

        println!("Input Y value: ");
        let result = input_coordinate();
        if let Ok(num) = result {
            y = num
        } else {
            println!("{}", result.unwrap_err());
            continue;
        }

        if board.turn(x, y).is_err() {
            println!("You can't make a move here!");
            continue;
        }
    }
}
