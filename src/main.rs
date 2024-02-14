use rand::Rng;
use std::io;

#[derive(Debug, Clone)]
struct Row {
    peg_one: Color,
    peg_two: Color,
    peg_three: Color,
    peg_four: Color,
}

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    White,
    Blank,
}

impl Color {
    fn random_color() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            3 => Color::White,
            _ => Color::Blank,
        }
    }
    fn as_str(&self) -> &'static str {
        match *self {
            Color::Red => " Red ", // white space for formatting
            Color::Green => "Green",
            Color::Blue => "Blue ",  // also here
            Color::Blank => "     ", // 5 spaces to represent "blank"
            Color::White => "White",
        }
    }
}

fn main() {
    let secret_row: Row = Row {
        peg_one: crate::Color::random_color(),
        peg_two: crate::Color::random_color(),
        peg_three: crate::Color::random_color(),
        peg_four: crate::Color::random_color(),
    };

    let mut game_board: Vec<Row> = vec![
        Row {
            peg_one: Color::Blank,
            peg_two: Color::Blank,
            peg_three: Color::Blank,
            peg_four: Color::Blank,
        };
        10
    ];

    for i in 0..10 {
        print_board(&secret_row, &false); // remove after testing, prints the secret key
        for i in 0..10 {
            let end = i == 9;
            print_board(&game_board[i], &end);
        }

        println!("Please input all 4 guess seaperated by Enter:");
        game_board[i].peg_one = get_user_input();
        game_board[i].peg_two = get_user_input();
        game_board[i].peg_three = get_user_input();
        game_board[i].peg_four = get_user_input();
        if check_win(&game_board[i], &secret_row) {
            println!("You Win");
            break;
        } else if true {
            println!("Change me to a funtion calling to give the correct color/correct space")
        } else if i == 9 {
            println!("Loose");
            break;
        }
    }
}

fn print_board(row: &Row, end: &bool) {
    let length: usize = [&row.peg_one, &row.peg_two, &row.peg_three, &row.peg_four]
        .iter()
        .map(|peg| peg.as_str().len())
        .sum::<usize>()
        + 11;

    print!("+"); // prints top right corner
    for _i in 0..length {
        // prints - for the length of the strings
        print!("-");
    }
    println!("+"); // prints top left corner

    println!(
        // prints the peg colors
        "| {} | {} | {} | {} |",
        row.peg_one.as_str(),
        row.peg_two.as_str(),
        row.peg_three.as_str(),
        row.peg_four.as_str()
    );

    if *end == true {
        print!("+"); // prints bottom row, change later to only print if it is the bottom of the board
        for _i in 0..length {
            print!("-");
        }
        println!("+");
    }
}

fn get_user_input() -> Color {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let color: Color = match guess.to_lowercase().trim() {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "white" => Color::White,
            _ => {
                println!("Invalid color. Please enter, Red, Blue, Green, or White");
                continue;
            }
        };

        return color;
    }
}

fn check_win(latest_row: &Row, secret_key: &Row) -> bool {
    if latest_row.peg_one == secret_key.peg_one
        && latest_row.peg_two == secret_key.peg_two
        && latest_row.peg_three == secret_key.peg_three
        && latest_row.peg_four == secret_key.peg_four
    {
        return true;
    }
    false
}
