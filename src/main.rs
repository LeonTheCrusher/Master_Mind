use rand::Rng;

#[derive(Debug, Clone)]
struct Row {
    peg_one: Color,
    peg_two: Color,
    peg_three: Color,
    peg_four: Color,
}

#[derive(Debug, Clone)]
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
}
impl Color {
    fn as_str(&self) -> &'static str {
        match *self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
            Color::Blank => "Blank",
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

    print_board(&secret_row, &false); // remove after testing, prints the secret key
    for i in 0..10 {
        let end = i == 9;
        print_board(&game_board[i], &end);
    }
}

fn print_board(row: &Row, end: &bool) {
    let length: usize = [&row.peg_one, &row.peg_two, &row.peg_three, &row.peg_four]
        .iter()
        .map(|peg| peg.as_str().trim().len())
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
        "| {:?} | {:?} | {:?} | {:?} |",
        row.peg_one, row.peg_two, row.peg_three, row.peg_four
    );

    if *end == true {
        print!("+"); // prints bottom row, change later to only print if it is the bottom of the board
        for _i in 0..length {
            print!("-");
        }
        println!("+");
    }
}
