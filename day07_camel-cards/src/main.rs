use std::env;

use input::iterate_input;

mod input;

// lower is better everywhere (rank-based / desc)
// NOTE: this can't exceed index 99 due to limitations in scoring calculations
const CAMEL_CARDS_LABELS: &[char] = &['A','K','Q','J','T','9','8','7','6','5','4','3','2'];
const CAMEL_CARDS_JOKER: usize = CAMEL_CARDS_LABELS.len();

enum CamelCardsHandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}
impl CamelCardsHandKind {
    pub fn value(&self) -> usize {
        return match self {
            CamelCardsHandKind::FiveOfAKind => 0,
            CamelCardsHandKind::FourOfAKind => 1,
            CamelCardsHandKind::FullHouse => 2,
            CamelCardsHandKind::ThreeOfAKind => 3,
            CamelCardsHandKind::TwoPair => 4,
            CamelCardsHandKind::OnePair => 5,
            CamelCardsHandKind::HighCard => 6            
        };
    }

    pub fn to_string(&self) -> &str {
        return match self {
            CamelCardsHandKind::FiveOfAKind => "Five Of A Kind",
            CamelCardsHandKind::FourOfAKind => "Four Of A Kind",
            CamelCardsHandKind::FullHouse => "Full House",
            CamelCardsHandKind::ThreeOfAKind => "Three Of A Kind",
            CamelCardsHandKind::TwoPair => "Two Pair",
            CamelCardsHandKind::OnePair => "One Pair",
            CamelCardsHandKind::HighCard => "High Card"    
        };
    }

}

struct CamelCardsHand {

    labels: String,
    value: Option<usize>,
    bid: usize
}

impl PartialEq for CamelCardsHand {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialOrd for CamelCardsHand {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.value.partial_cmp(&other.value);
    }

}

impl CamelCardsHand {



    pub fn calc_hand_value(&mut self) {
        self.calc_hand_value_internal(None);
    }

    pub fn calc_hand_value_with_joker(&mut self, joker: char) {
        self.calc_hand_value_internal(Some(joker));
    }

    fn calc_hand_value_internal(&mut self, joker: Option<char>) {

        let mut label_values = Vec::new();

        for c in self.labels.chars() {

            if joker.is_some_and(|j| j == c) {
                // joker automatically has lowest value (== highest ix)
                label_values.push(CAMEL_CARDS_LABELS.len()); 
                continue;
            }

            if let Some(label_value) = Self::parse_label_value(&c) {

                label_values.push(label_value);
            }
        }

        let hand_kind = Self::calc_hand_kind(&label_values, joker.is_some());
        let face_value = Self::calc_face_value(&label_values);

        let score = (hand_kind.value() * 10_usize.pow(12)) + face_value;

        self.value = Some(score);
        //*
        debug!("Hand '{}'", self.labels);
        debug!(" - kind: {}", hand_kind.to_string());
        debug!(" - face: {}", face_value);
        debug!(" - score: {}", score);
        // */
    }


    // determines a score for the hand, based on its card values
    // used for tie-breaking
    // again, lower is better
    fn calc_face_value(label_values: &Vec<usize>) -> usize {

        let mut score = 1;

        // iterate in reverse; big endian
        for (ix, value) in label_values.iter().rev().enumerate() {
            score += value * 10_usize.pow((ix*2) as u32);
        }
        return score;
    }

    // determines the kind of hand based on the provided card values
    fn calc_hand_kind(label_values: &Vec<usize>, has_joker: bool) -> CamelCardsHandKind {

        let mut label_values_sorted = label_values.clone();
        label_values_sorted.sort();

        let mut streak_counter = 1;
        let mut streak_value = 0;

        let mut result = CamelCardsHandKind::HighCard;


        for i in 0..label_values_sorted.len() {

            let prev_value = label_values_sorted[i];
            let cur_value = label_values_sorted.get(i+1);


            /*
               debug!("\t prev: {}", CAMEL_CARDS_LABELS[prev_value]);
               debug!("\t cur: {}", CAMEL_CARDS_LABELS[*cur_value.unwrap_or(&0)]);
               debug!("\t streak: {}", streak_counter);
               debug!("---");
               */

            let is_end_of_streak: bool = match cur_value {
                Some(cv) => { 
                    let mut adjusted_prev_value = prev_value;
                    let mut adjusted_cur_value = *cv;

                    if has_joker {
                        if adjusted_cur_value == CAMEL_CARDS_JOKER {
                            adjusted_cur_value = streak_value;
                        }
                        if adjusted_prev_value == CAMEL_CARDS_JOKER {
                            adjusted_prev_value = streak_value;
                        }
                    }
                    
                    adjusted_cur_value != adjusted_prev_value
                },
                None => true
            };



            if is_end_of_streak  {
                // end of streak

                debug!("eos: {}", cur_value.unwrap_or(&0));

                if streak_counter == 5 {
                    return CamelCardsHandKind::FiveOfAKind;

                } else if streak_counter == 4 {
                    return CamelCardsHandKind::FourOfAKind;

                } else if streak_counter == 3 && 
                    matches!(result, CamelCardsHandKind::HighCard) {
                    result = CamelCardsHandKind::ThreeOfAKind;

                } else if streak_counter == 2 && 
                    matches!(result, CamelCardsHandKind::HighCard) {
                    result = CamelCardsHandKind::OnePair;

                } else if (streak_counter == 3 && 
                           matches!(result, CamelCardsHandKind::OnePair)) || 
                          (streak_counter == 2 && 
                           matches!(result, CamelCardsHandKind::ThreeOfAKind)) {
                    return CamelCardsHandKind::FullHouse;
                } else if streak_counter == 2 && 
                    matches!(result, CamelCardsHandKind::OnePair) {
                    return CamelCardsHandKind::TwoPair;
                }
                streak_counter = 1;

                streak_value = match cur_value {
                    Some(v) => *v,
                    None => 0
                };

            } else {
                streak_counter += 1;
            }
        }

        return result;
        }

        fn parse_label_value(label: &char) -> Option<usize> {

            for (ix, entry) in CAMEL_CARDS_LABELS.iter().enumerate() {
                if label == entry {
                    return Some(ix);
                }
            }

            return None;
        }
    }


    fn main() {

        env::set_var("PRINT_DEBUG", "true");


        let mut hands: Vec<CamelCardsHand> = Vec::new();


        for line in iterate_input() {

            let mut hand = parse_card_hand(line);
            //hand.calc_hand_value();
            hand.calc_hand_value_with_joker('J');
            hands.push(hand);
        }

        hands.sort_by(|a,b| a.partial_cmp(b).unwrap());

        let mut solution = 0;

        // lower is better, so iterate in reverse; ranks are bid multipliers
        for (ix, hand) in hands.iter().rev().enumerate() {

            let winnings = (ix + 1) * hand.bid;
            solution += winnings;
            debug!(format!("{}: {} ({}); bid: {}; won: {}", 
                           ix+1, 
                           hand.labels, 
                           hand.value.unwrap_or(0),
                           hand.bid,
                           winnings
                          ));

        }

        println!("Solution: {}", solution);
    }


    fn parse_card_hand(text: String) -> CamelCardsHand {

        let values: Vec<&str> = text.split(" ").collect();

        if values.len() != 2 {
            panic!("Invalid card hand input '{}'", text );
        }


        let bid_value = match values[1].parse::<usize>() {
            Ok(v) => v,
            Err(e) => { panic!("Error parsing bid value '{}'", e)} 
        };


        let hand = CamelCardsHand {
            labels:  values[0].to_string(),
            value: None,
            bid: bid_value
        };

        return hand;
    }
