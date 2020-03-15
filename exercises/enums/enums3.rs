// enums3.rs
// Address all the TODOs to make the tests pass!

enum Message {
    // TODO: implement the message variant types based on their usage below
    Quit, // takes in nothing
    Move{x: u8, y : u8}, // anonymous struct
    Echo(String),
    ChangeColor(u8, u8, u8),
}

// above is similar to doing it all with a struct
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

// impl is essentally the fxn definitions of a class
// METHOD DEFINITON WORKS FOR BOTH struct and enum
impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        // essentially a switch case
        // but its pretty cool that you can do it on a type?
        match message {
            // ChangeColor => self.change_color(message.ChangeColor),
            Message::ChangeColor(r, g, b) => {
                self.change_color((r,g,b));
            }

            Message::Move{x, y} => {
                // create a new point
                let s = Point{
                    x : x,
                    y : y,
                };

                // pass into self call
                self.move_position(s);
            }

            Message::Quit => {
                self.quit();
            }

            Message::Echo(string) => {
                self.echo(string);
            }

            // this is default
            _ => {
                println!("Something else");
            }
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State{
            quit: false,
            position: Point{ x: 0, y: 0 },
            color: (0, 0, 0)
        };

        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move{ x: 10, y: 15 });
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }

}
