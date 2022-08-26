use std::io;

fn main() {
    println!("Type anything and press enter to get a Wingdings translation. Type \"exit\" to close wdrs.");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            return;
        }
        println!("{}", to_wingdings(input));
    }
}

fn to_wingdings(text: &str) -> String {
    let mut wingdings = String::new();
    for c in text.split("") {
        match c {
            "a" => wingdings += "♋︎",
            "b" => wingdings += "♌︎",
            "c" => wingdings += "♍︎",
            "d" => wingdings += "♎︎",
            "e" => wingdings += "♏︎",
            "f" => wingdings += "♐︎",
            "g" => wingdings += "♑︎",
            "h" => wingdings += "♒︎",
            "i" => wingdings += "♓︎",
            "j" => wingdings += "🙰",
            "k" => wingdings += "🙵",
            "l" => wingdings += "●︎",
            "m" => wingdings += "❍︎",
            "n" => wingdings += "■︎",
            "o" => wingdings += "□︎",
            "p" => wingdings += "◻︎",
            "q" => wingdings += "❑︎",
            "r" => wingdings += "❒︎",
            "s" => wingdings += "⬧︎",
            "t" => wingdings += "⧫︎",
            "u" => wingdings += "◆︎",
            "v" => wingdings += "❖︎",
            "w" => wingdings += "⬥︎",
            "x" => wingdings += "⌧︎",
            "y" => wingdings += "⍓︎",
            "z" => wingdings += "⌘︎",
            "A" => wingdings += "✌︎",
            "B" => wingdings += "👌︎",
            "C" => wingdings += "👍︎",
            "D" => wingdings += "👎︎",
            "E" => wingdings += "☜︎",
            "F" => wingdings += "☞︎",
            "G" => wingdings += "☝︎",
            "H" => wingdings += "☟︎",
            "I" => wingdings += "✋︎",
            "J" => wingdings += "☺︎",
            "K" => wingdings += "😐︎",
            "L" => wingdings += "☹︎",
            "M" => wingdings += "💣︎",
            "N" => wingdings += "☠︎",
            "O" => wingdings += "⚐︎",
            "P" => wingdings += "🏱︎",
            "Q" => wingdings += "✈︎",
            "R" => wingdings += "☼︎",
            "S" => wingdings += "💧︎",
            "T" => wingdings += "❄︎",
            "U" => wingdings += "🕆︎",
            "V" => wingdings += "✞︎",
            "W" => wingdings += "🕈︎",
            "X" => wingdings += "✠︎",
            "Y" => wingdings += "✡︎",
            "Z" => wingdings += "☪︎",
            " " => wingdings += " ",
            "1" => wingdings += "📂︎",
            "2" => wingdings += "📄︎",
            "3" => wingdings += "🗏︎",
            "4" => wingdings += "🗐︎",
            "5" => wingdings += "🗄︎",
            "6" => wingdings += "⌛︎",
            "7" => wingdings += "🖮︎",
            "8" => wingdings += "🖰︎",
            "9" => wingdings += "🖲︎",
            "0" => wingdings += "📁︎",
            _ => wingdings += c,
        }
    }
    wingdings
}
