use std::fmt::Display;
use std::io::Write;

// All the possible inputs to the FSM
enum Input {
    Digit(u8),
    DecimalPoint,
    Invalid,
}

impl Input {
    fn from_char(c: char) -> Self {
        match c {
            '0'..='9' => Self::Digit(c as u8 - '0' as u8),
            '.' => Self::DecimalPoint,
            _ => Self::Invalid,
        }
    }
}

// The FSM state
#[derive(PartialEq, Debug)]
enum NumberState {
    Start,
    Zero,
    Fail,
    DecimalPoint,
    DigitBeforeDecimalPoint,
    DigitAfterDecimalPoint,
}

impl NumberState {
    fn new() -> Self {
        Self::Start
    }

    fn next(self, i: Input) -> Self {
        match self {
            Self::Start => match i {
                Input::Digit(0) => Self::Zero,
                Input::Digit(_) => Self::DigitBeforeDecimalPoint,
                Input::DecimalPoint => Self::Fail,
                Input::Invalid => Self::Fail,
            },
            Self::Zero => match i {
                Input::DecimalPoint => Self::DecimalPoint,
                Input::Digit(_) => Self::Fail,
                Input::Invalid => Self::Fail,
            },
            Self::Fail => Self::Fail,
            Self::DecimalPoint => match i {
                Input::Digit(_) => Self::DigitAfterDecimalPoint,
                Input::DecimalPoint => Self::Fail,
                Input::Invalid => Self::Fail,
            },
            Self::DigitBeforeDecimalPoint => match i {
                Input::Digit(_) => Self::DigitBeforeDecimalPoint,
                Input::DecimalPoint => Self::DecimalPoint,
                Input::Invalid => Self::Fail,
            },
            Self::DigitAfterDecimalPoint => match i {
                Input::Digit(_) => Self::DigitAfterDecimalPoint,
                Input::DecimalPoint => Self::Fail,
                Input::Invalid => Self::Fail,
            },
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Self::DigitAfterDecimalPoint => true,
            Self::DigitBeforeDecimalPoint => true,
            Self::Zero => true,
            _ => false,
        }
    }
}

// Allow the FSM to be printed
impl Display for NumberState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::Zero => write!(f, "zero"),
            Self::Fail => write!(f, "fail"),
            Self::DecimalPoint => write!(f, "decimal point"),
            Self::DigitBeforeDecimalPoint => write!(f, "digit before decimal point"),
            Self::DigitAfterDecimalPoint => write!(f, "digit after decimal point"),
        }
    }
}

// Convert a string into the final state of the FSM
fn get_final_state_for_input(input: String) -> NumberState {
    let mut current_state = NumberState::new();

    for c in input.trim().chars() {
        let input = Input::from_char(c);

        current_state = current_state.next(input);
    }

    current_state
}

fn main() {
    let mut input = String::new();
    print!("Enter a string to check: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let final_state = get_final_state_for_input(input);

    println!("Final state: {}", final_state);
    println!("Is valid: {}", if final_state.is_valid() { "yes" } else { "no" });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_integers() {
        let valid_integers = vec![
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
            "100", "3141592653"
        ];

        for integer in valid_integers {
            let final_state = super::get_final_state_for_input(integer.to_string());
            assert_eq!(final_state.is_valid(), true);
        }
    }

    #[test]
    fn test_invalid_integers() {
        let invalid_integers = vec!["00", "01"];

        for integer in invalid_integers {
            let final_state = super::get_final_state_for_input(integer.to_string());
            assert_eq!(final_state.is_valid(), false);
        }
    }

    #[test]
    fn test_valid_decimals() {
        let valid_decimals = vec!["1.0", "0.0", "0.1", "10.0", "100.0", "3.14", "45.6"];

        for decimal in valid_decimals {
            let final_state = super::get_final_state_for_input(decimal.to_string());
            assert_eq!(final_state.is_valid(), true);
        }
    }

    #[test]
    fn test_invalid_decimals() {
        let invalid_decimals = vec![".", "0.", "1.", ".1", ".0", "0.."];

        for decimal in invalid_decimals {
            let final_state = super::get_final_state_for_input(decimal.to_string());
            assert_eq!(final_state.is_valid(), false);
        }
    }

    #[test]
    fn test_invalid_misc() {
        let invalid_misc = vec!["", "abc", "3.ad"];

        for misc in invalid_misc {
            let final_state = super::get_final_state_for_input(misc.to_string());
            assert_eq!(final_state.is_valid(), false);
        }
    }
}