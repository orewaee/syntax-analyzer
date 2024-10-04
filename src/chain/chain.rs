pub trait Chain {
    fn from(string: &str) -> Self;
    fn is_valid(&self) -> Result<(), usize>;
}

pub struct ForChain {
    pub string: String
}

impl Chain for ForChain {
    fn from(string: &str) -> Self {
        Chain { string: String::from(string) }
    }

    fn is_valid(&self) -> Result<(), (usize, &str)> {
        if self.string.eq("for") {
            return Ok(());
        }

        Err((0, "invalid keyword"))

        /*
        let mut state = State::S;
        let mut index = 0;
        let mut symbol: char;

        while state != State::F && state != State::E && index < self.string.len() {
            symbol = self.string.chars().nth(index).unwrap();
            match state {
                State::S => {
                    if symbol == 'f' || symbol == 'F' {
                        state = State::A;
                    } else {
                        state = State::E;
                        return Err((index, "maybe you wanted to start with f (for)"));
                    }
                }

                State::A => {
                    if symbol == 'o' || symbol == 'O' {
                        state = State::B;
                    } else {
                        state = State::E;
                        return Err((index, "maybe you wanted to use o (for)"));
                    }
                }

                State::B => {
                    if symbol == 'r' || symbol == 'R' {
                        state = State::C;
                    } else {
                        state = State::E;
                        return Err((index, "maybe you wanted to use r (for)"));
                    }
                }

                State::C => {
                    if symbol == ' ' {
                        state = State::F;
                    } else {
                        state = State::E;
                        return Err((index, "maybe you wanted to use space (end chain)"));
                    }
                }

                State::E | State::F => {
                    break;
                }
            }

            index += 1;
        }

        if state == State::F {
            return Ok(());
        }

        if state == State::E {
            return Err((index, "unexpected error"));
        }

        Err((index, "the chain does not contain a terminating char"))
        */
    }
}
