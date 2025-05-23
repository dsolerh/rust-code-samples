use rand::Rng;

pub(crate) type Die = u8;

#[derive(Debug)]
pub enum DiceRoll {
    Single(Die, Die),
    Double(Die),
}

impl DiceRoll {
    pub(crate) fn get_dice_roll() -> DiceRoll {
        let mut rng = rand::thread_rng();
        let die1: Die = rng.gen_range(1..=6);
        let die2: Die = rng.gen_range(1..=6);

        if die1 == die2 {
            Self::Double(die1)
        } else {
            Self::Single(die1, die2)
        }
    }

    pub(crate) fn get_high_low_die(&self) -> (Die, Die) {
        match self {
            DiceRoll::Single(die1, die2) => {
                if die1 > die2 {
                    (*die1, *die2)
                } else {
                    (*die2, *die1)
                }
            }
            DiceRoll::Double(_) => panic!("the roll must have exactly to dices"),
        }
    }
}

pub(crate) fn repeat_die(die: Die, times: usize) -> Vec<Die> {
    [die].into_iter().cycle().take(times).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_die() {
        assert_eq!(repeat_die(1, 2), vec![1, 1])
    }
}
