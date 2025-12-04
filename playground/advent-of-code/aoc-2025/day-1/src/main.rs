struct RotaryDial {
    dial_position: i32,
    number_of_positions: i32,
    times_landed_on_zero: i32,
}

impl Default for RotaryDial {
    fn default() -> Self {
        Self::new(0)
    }
}

#[allow(clippy::needless_return)]
impl RotaryDial {
    fn new(start_position: i32) -> RotaryDial {
        let rotary_dial: RotaryDial = RotaryDial {
            dial_position: start_position,
            number_of_positions: 100, // 0-99 is a hundred available positions
            times_landed_on_zero: 0,
        };

        if start_position > rotary_dial.number_of_positions {
            panic!(
                "Attempted to create a rotary dial with starting position greater than the available number of positions ({})",
                rotary_dial.number_of_positions
            )
        }

        return rotary_dial;
    }

    fn rotate_dial(&mut self, direction: char, rotations: i32) {
        if !matches!(direction, 'L' | 'R') {
            panic!("{direction} is not a valid direction!")
        }

        println!(
            "Instructed to rotate {rotations} turns {direction} from position {}",
            self.dial_position
        );

        let r: i32 = rotations % 100;
        if r == 0 {
            return self.check_final_position();
        }

        // Move the rotary dial
        if direction == 'L' {
            if (self.dial_position - r) < 0 {
                self.dial_position = self.number_of_positions - (r - self.dial_position);
            } else {
                self.dial_position -= r;
            }
        }

        if direction == 'R' {
            if (self.dial_position + r) > self.number_of_positions {
                self.dial_position = self.dial_position + r - self.number_of_positions;
            } else {
                self.dial_position += r;
            }
        }

        return self.check_final_position();
    }

    fn check_final_position(&mut self) {
        if matches!(self.dial_position, 100 | 0) {
            self.dial_position = 0; // Account for off by one errors
            self.times_landed_on_zero += 1;
        }

        println!("Done rotating! New dial position is {}", self.dial_position);
    }
}

#[allow(dead_code, clippy::explicit_counter_loop)]
fn main() {
    let mut dial: RotaryDial = RotaryDial::new(50);
    for line in include_str!("./input.txt").split("\n") {
        // data conversion
        if line.is_empty() {
            break;
        }

        let l = String::from(line);
        let d: &str = &l[0..1]; // Direction
        let r: &str = &l[1..l.len()]; // Rotations
        let c: Vec<char> = d.chars().collect();

        // dial rotation
        let direction: char = c[0];
        let rotations: i32 = r.parse().unwrap();
        dial.rotate_dial(direction, rotations);
    }

    println!(
        "Done cracking password! The rotary dial landed on zero {} times.",
        dial.times_landed_on_zero
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotary_dial_default_start_is_zero() {
        let rotary_dial = RotaryDial::default();
        assert_eq!(rotary_dial.dial_position, 0);
    }

    #[test]
    fn can_set_custom_starting_position() {
        let rotary_dial = RotaryDial::new(50);
        assert_eq!(rotary_dial.dial_position, 50);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_custom_start_position_exceeds_max_positions() {
        let _ = RotaryDial::new(111);
    }

    #[test]
    #[should_panic]
    fn should_panic_if_rotating_in_invalid_direction() {
        let mut rotary_dial = RotaryDial::default();
        assert_eq!(rotary_dial.dial_position, 0);
        rotary_dial.rotate_dial('Z', 7);
    }

    #[test]
    fn should_be_able_to_rotate_past_available_positions() {
        let mut rotary_dial = RotaryDial::default();
        rotary_dial.rotate_dial('R', 501);
        assert_eq!(rotary_dial.dial_position, 1);
    }

    #[test]
    fn making_a_hundred_rotations_or_a_multitude_of_hundred_should_not_change_dial_position() {
        let mut rd: RotaryDial = RotaryDial::new(69);
        rd.rotate_dial('R', 200);
        assert_eq!(rd.dial_position, 69);

        rd.rotate_dial('L', 700);
        assert_eq!(rd.dial_position, 69);
    }

    #[test]
    fn counter_should_increment_on_landing_on_zero_regardless_of_moving_direction() {
        let mut rd_right = RotaryDial::new(52);
        assert_eq!(rd_right.times_landed_on_zero, 0);

        rd_right.rotate_dial('R', 48);
        assert_eq!(rd_right.dial_position, 0);
        assert_eq!(rd_right.times_landed_on_zero, 1);

        let mut rd_left = RotaryDial::new(52);
        assert_eq!(rd_left.times_landed_on_zero, 0);

        rd_left.rotate_dial('L', 52);
        assert_eq!(rd_left.dial_position, 0);
        assert_eq!(rd_left.times_landed_on_zero, 1);
    }

    #[test]
    fn should_be_able_to_extract_password_from_instruction_file() {
        let mut rd: RotaryDial = RotaryDial::new(50);
        for line in include_str!("./test.txt").split("\n") {
            if line.is_empty() {
                break;
            }

            let l = String::from(line);
            let d: &str = &l[0..1]; // Direction
            let r: &str = &l[1..l.len()]; // Rotations
            let c: Vec<char> = d.chars().collect();

            // dial rotation
            let direction: char = c[0];
            let rotations: i32 = r.parse().unwrap();
            rd.rotate_dial(direction, rotations);
        }

        assert_eq!(rd.times_landed_on_zero, 3);
    }
}
