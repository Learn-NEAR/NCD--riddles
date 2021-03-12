/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::wee_alloc;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId, Balance,
};
use near_sdk::{env, near_bindgen};
use near_sdk::{
    serde::{Deserialize, Serialize},
    Promise,
};

use std::{collections::HashMap, fmt::Display};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ----------------------------- category -----------------------------
#[derive(BorshSerialize, BorshDeserialize, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum RiddleKind {
    History,
    Culture,
    Science,
    Math,
    Other,
}

impl Display for RiddleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &RiddleKind::History => write!(f, "history"),
            &RiddleKind::Culture => write!(f, "Culture"),
            &RiddleKind::Science => write!(f, "Science"),
            &RiddleKind::Math => write!(f, "Math"),
            &RiddleKind::Other => write!(f, "Other"),
        }
    }
}

// ----------------------------- grade -----------------------------
#[derive(BorshSerialize, BorshDeserialize, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum RiddleGrade {
    Easy,
    Medium,
    Hard,
}

impl Display for RiddleGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &RiddleGrade::Easy => write!(f, "easy"),
            &RiddleGrade::Medium => write!(f, "medium"),
            &RiddleGrade::Hard => write!(f, "hard"),
        }
    }
}

// ----------------------------- add_riddle info -----------------------------
#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RiddleInput {
    riddle: RiddleInfo,
    grade: RiddleGrade,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RiddleInfo {
    question: String,
    sha256_answer: String,
    kind: RiddleKind,
}

// ----------------------------- answer_riddle info -----------------------------

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AnswerInfo {
    id: u128,
    sha256_answer: String,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Riddle {
    id: u128,
    grade: RiddleGrade,
    creator: AccountId,
    bonus: Balance,
    riddle_info: RiddleInfo,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RiddleGame {
    riddles: HashMap<u128, Riddle>,
    last_index: u128,
}

#[near_bindgen]
impl RiddleGame {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            riddles: HashMap::new(),
            last_index: 1_u128,
        }
    }

    #[payable]
    pub fn add_riddle(&mut self, input: RiddleInput) {
        let creator = env::signer_account_id();
        assert_eq!(
            creator,
            env::predecessor_account_id(),
            "predecessor account id must be the signer"
        );

        let bonus = env::attached_deposit();
        env::log(
            format!(
                "{} just created a new riddle in the domain {} with a bonus at {}",
                creator, input.riddle.kind, bonus
            )
            .as_bytes(),
        );

        self.last_index += 1;
        let riddle = Riddle {
            id: self.last_index,
            grade: input.grade,
            creator,
            bonus: bonus,
            riddle_info: input.riddle,
        };

        self.riddles.insert(self.last_index, riddle);
        // this._contract.buy_add_riddle({}, new BN("30000000000000"), requiredBalance);
        // TODO: check signer's banlance
        Promise::new(env::current_account_id()).transfer(bonus);
    }

    pub fn answer_riddle(&mut self, answer: AnswerInfo) -> Option<Riddle> {
        let answerer = env::signer_account_id();
        assert_eq!(
            answerer,
            env::predecessor_account_id(),
            "predecessor account id must be the answerer"
        );
        match self.riddles.get_mut(&answer.id) {
            Some(riddle) if riddle.riddle_info.sha256_answer == answer.sha256_answer => {
                let bonus = riddle.bonus;

                env::log(
                    format!(
                        "{} just answered riddle#{} correctly, he won the bonus at {}",
                        answerer, answer.id, bonus
                    )
                    .as_bytes(),
                );

                self.riddles.remove(&answer.id);
                Promise::new(answerer).transfer(1000);
            }
            _ => {}
        }
        self.riddles.get(&answer.id).cloned()
    }

    pub fn get_riddles(&self) -> Vec<Riddle> {
        self.riddles.values().cloned().collect()
    }

    pub fn get_riddles_of_kind(&self, kind: &RiddleKind) -> Vec<Riddle> {
        self.riddles
            .values()
            .filter(|r| &r.riddle_info.kind == kind)
            .cloned()
            .collect()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests;
