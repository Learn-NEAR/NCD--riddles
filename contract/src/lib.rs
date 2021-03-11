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
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::wee_alloc;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId, Balance,
};
use near_sdk::{env, near_bindgen};

use std::{collections::HashMap, fmt::Display};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshSerialize, BorshDeserialize, Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum RiddleKind {
    History,
}

impl Display for RiddleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &RiddleKind::History => write!(f, "history"),
            _ => unreachable!(),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RiddleInput {
    question: String,
    sha256_answer: String,
    kind: RiddleKind,
    bonus: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Riddle {
    id: u64,
    difficulty: u64,
    creator: AccountId,
    input: RiddleInput,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct RiddleGame {
    riddles: HashMap<u64, Riddle>,
}

#[near_bindgen]
impl RiddleGame {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            riddles: HashMap::new(),
        }
    }

    #[payable]
    pub fn add_riddle(&mut self, input: RiddleInput) {
        let creator = env::signer_account_id();

        env::log(
            format!(
                "{} just created a new riddle in the domain {} with a bonus at {}",
                creator, input.kind, input.bonus
            )
            .as_bytes(),
        );

        // TODO: check overflow
        let id = (self.riddles.len() + 1) as u64;
        let difficulty = 1_u64;
        let riddle = Riddle {
            id,
            difficulty,
            creator,
            input,
        };

        // TODO: transfer bonus from creator to contract
        self.riddles.insert(id, riddle);
    }

    pub fn answer_riddle(&mut self, id: u64, sha256_answer: String) -> bool {
        let (is_correct, bonus) = match self.riddles.get_mut(&id) {
            Some(riddle) if riddle.input.sha256_answer == sha256_answer => {
                (true, riddle.input.bonus)
            }
            Some(riddle) => {
                riddle.difficulty += 1;
                (false, Balance::from(0_u128))
            }
            None => (false, Balance::from(0_u128)),
        };

        if is_correct {
            let answerer = env::signer_account_id();
            env::log(
                format!(
                    "{} just answered riddle#{} correctly, he won the bonus at {}",
                    answerer, id, bonus
                )
                .as_bytes(),
            );

            self.riddles.remove(&id);
            // TODO: pay bonus to answerer
        }

        is_correct
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
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = RiddleGame::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = RiddleGame::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}
