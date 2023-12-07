use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<&Vec<Card>> for HandType {
    type Error = ();
    fn try_from(hand: &Vec<Card>) -> Result<Self, Self::Error> {
        if hand.len() != 5 {
            return Err(());
        }
        //Check 5 of a kind
        if hand[0] == hand[1] && hand[0] == hand[2] && hand[0] == hand[3] && hand[0] == hand[4] {
            return Ok(HandType::FiveOfAKind);
        }
        //Check of each type
        let mut count_array: [u8; 13] = [0; 13];
        for card in hand {
            count_array[(card.clone() as usize) - 2] += 1;
        }
        let mut pair_present = false;
        let mut triple_present = false;
        for count in count_array {
            if count == 4 {
                return Ok(HandType::FourOfAKind);
            }
            if count == 3 {
                triple_present = true;
            }
            if count == 2 {
                if pair_present {
                    return Ok(HandType::TwoPair);
                } else {
                    pair_present = true;
                }
            }
        }
        if triple_present && pair_present {
            return Ok(HandType::FullHouse);
        } else if triple_present {
            return Ok(HandType::ThreeOfAKind);
        } else if pair_present {
            return Ok(HandType::OnePair);
        } else {
            return Ok(HandType::HighCard);
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
enum Card {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        match item {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct HandData {
    hand: Vec<Card>,
    bid: u64,
    hand_type: HandType,
}

impl Ord for HandData {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (card1, card2) in std::iter::zip(&self.hand, &other.hand) {
                if card1 == card2 {
                    continue;
                } else {
                    return card1.cmp(card2);
                }
            }
            Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for HandData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandData {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            for (card1, card2) in std::iter::zip(&self.hand, &other.hand) {
                if card1 != card2 {
                    return false;
                } else {
                    continue;
                }
            }
            true
        } else {
            false
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
enum JokerCard {
    Joker = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for JokerCard {
    type Error = ();

    fn try_from(item: char) -> Result<Self, Self::Error> {
        match item {
            'A' => Ok(JokerCard::Ace),
            'K' => Ok(JokerCard::King),
            'Q' => Ok(JokerCard::Queen),
            'T' => Ok(JokerCard::Ten),
            '9' => Ok(JokerCard::Nine),
            '8' => Ok(JokerCard::Eight),
            '7' => Ok(JokerCard::Seven),
            '6' => Ok(JokerCard::Six),
            '5' => Ok(JokerCard::Five),
            '4' => Ok(JokerCard::Four),
            '3' => Ok(JokerCard::Three),
            '2' => Ok(JokerCard::Two),
            'J' => Ok(JokerCard::Joker),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Debug)]
enum JokerHandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<&Vec<JokerCard>> for JokerHandType {
    type Error = ();
    fn try_from(hand: &Vec<JokerCard>) -> Result<Self, Self::Error> {
        if hand.len() != 5 {
            return Err(());
        }
        //Check of each type
        let mut count_array: [u8; 13] = [0; 13];
        for card in hand {
            count_array[(card.clone() as usize) - 1] += 1;
        }
        let mut pair_count = 0;
        let mut true_pair_count = 0;
        let mut true_triple_present = false;
        let mut triple_present = false;
        for (i, count_ptr) in count_array.iter().enumerate() {
            let count = *count_ptr;
            if i == 0 {
                if count == 5 || count == 4 {
                    return Ok(JokerHandType::FiveOfAKind);
                } else {
                    continue;
                }
            } else {
                if count + count_array[0] == 5 {
                    return Ok(JokerHandType::FiveOfAKind);
                }
                if count + count_array[0] == 4 {
                    return Ok(JokerHandType::FourOfAKind);
                }
                if count == 3 {
                    true_triple_present = true;
                }
                if count + count_array[0] == 3 {
                    triple_present = true;
                }
                if count == 2 {
                    true_pair_count += 1;
                }
                if count + count_array[0] == 2 {
                    pair_count += 1;
                }
            }
        }
        if (true_triple_present && true_pair_count > 0)
            || (count_array[0] == 1 && true_pair_count >= 2)
        {
            return Ok(JokerHandType::FullHouse);
        } else if triple_present {
            return Ok(JokerHandType::ThreeOfAKind);
        } else if true_pair_count == 2 {
            return Ok(JokerHandType::TwoPair);
        } else if pair_count >= 2 {
            if count_array[0] >= 2 {
                return Ok(JokerHandType::TwoPair);
            } else {
                return Ok(JokerHandType::OnePair);
            }
        } else if pair_count == 1 {
            return Ok(JokerHandType::OnePair);
        } else {
            return Ok(JokerHandType::HighCard);
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct JokerHandData {
    hand: Vec<JokerCard>,
    bid: u64,
    hand_type: JokerHandType,
}

impl Ord for JokerHandData {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (card1, card2) in std::iter::zip(&self.hand, &other.hand) {
                if card1 == card2 {
                    continue;
                } else {
                    return card1.cmp(card2);
                }
            }
            Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for JokerHandData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for JokerHandData {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            for (card1, card2) in std::iter::zip(&self.hand, &other.hand) {
                if card1 != card2 {
                    return false;
                } else {
                    continue;
                }
            }
            true
        } else {
            false
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let mut all_hands: Vec<HandData> = input
        .lines()
        .map(|input_line| {
            let raw_hand = input_line
                .split_ascii_whitespace()
                .nth(0)
                .expect("should be a hand");
            let bid: u64 = input_line
                .split_ascii_whitespace()
                .nth(1)
                .expect("should be text for a bid")
                .parse()
                .expect("bid should be a number");
            let hand: Vec<Card> = raw_hand
                .chars()
                .map(|character| {
                    character
                        .try_into()
                        .expect(format!("{character} should be a valid card").as_str())
                })
                .collect();
            HandData {
                hand_type: (&hand).try_into().expect("hand length was incorrect"),
                hand,
                bid,
            }
        })
        .collect();
    all_hands.sort();
    all_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i as u64) + 1) * hand.bid)
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut all_hands: Vec<JokerHandData> = input
        .lines()
        .map(|input_line| {
            let raw_hand = input_line
                .split_ascii_whitespace()
                .nth(0)
                .expect("should be a hand");
            let bid: u64 = input_line
                .split_ascii_whitespace()
                .nth(1)
                .expect("should be text for a bid")
                .parse()
                .expect("bid should be a number");
            let hand: Vec<JokerCard> = raw_hand
                .chars()
                .map(|character| {
                    character
                        .try_into()
                        .expect(format!("{character} should be a valid card").as_str())
                })
                .collect();
            JokerHandData {
                hand_type: (&hand).try_into().expect("hand length was incorrect"),
                hand,
                bid,
            }
        })
        .collect();
    all_hands.sort();
    all_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| ((i as u64) + 1) * hand.bid)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "6440");
        assert_eq!(process_part2(file.as_str()), "5905");
    }
    #[test]
    fn test_big_input() {
        let file = fs::read_to_string("./input.txt").unwrap();
        assert_eq!(process_part2(file.as_str()), "idk lmao");
    }
}
