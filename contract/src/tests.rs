use super::*;
use near_sdk::MockedBlockchain;
use near_sdk::{testing_env, VMContext};

fn alice() -> AccountId {
    "alice.near".to_string()
}

fn carol() -> AccountId {
    "carol.near".to_string()
}

fn get_context(signer_account_id: AccountId, predecessor_account_id: AccountId) -> VMContext {
    VMContext {
        current_account_id: alice(),
        signer_account_id, // predecessor_account_id
        signer_account_pk: vec![0, 1, 2],
        predecessor_account_id,
        input: vec![],
        block_index: 0,
        block_timestamp: 0,
        account_balance: 0,
        account_locked_balance: 0,
        storage_usage: 10u64.pow(6),
        attached_deposit: 0,
        prepaid_gas: 10u64.pow(18),
        random_seed: vec![0, 1, 2],
        is_view: false,
        output_data_receivers: vec![],
        epoch_height: 0,
    }
}

#[test]
fn test_new() {
    let context = get_context(carol(), carol());
    testing_env!(context);
    let contract = RiddleGame::default();
    assert_eq!(
        contract
            .get_riddles_of_kind(&RiddleKind::Culture)
            .first()
            .is_some(),
        false
    );
    assert_eq!(
        contract
            .get_riddles_of_kind(&RiddleKind::Culture)
            .first()
            .is_none(),
        true
    );
}

#[test]
fn add_riddle() {
    // get our VMContext from earlier
    let context = get_context(carol(), carol());
    // set the testing environment
    testing_env!(context);
    let mut contract = RiddleGame::default();

    // input
    let input = RiddleInput {
        riddle: RiddleInfo {
            question: "I am an odd number. Take away a letter and I become even. What number am I?"
                .to_string(),
            sha256_answer: "3ba8d02b16fd2a01c1a8ba1a1f036d7ce386ed953696fa57331c2ac48a80b255"
                .to_string(),
            kind: RiddleKind::Culture,
        },
        grade: RiddleGrade::Easy,
    };
    contract.add_riddle(input);
    assert_eq!(
        contract
            .get_riddles_of_kind(&RiddleKind::Culture)
            .first()
            .is_some(),
        true
    );
}

#[test]
fn answer_riddle() {
    let context = get_context(carol(), carol());
    // set the testing environment
    testing_env!(context);
    let mut contract = RiddleGame::default();
    // id is error
    let ans1 = AnswerInfo {
        id: 100,
        sha256_answer: "3ba8d02b16fd2a01c1a8ba1a1f036d7ce386ed953696fa57331c2ac48a80b255"
            .to_string(),
    };
    assert_eq!(contract.answer_riddle(ans1).is_none(), true);
    // input
    let input = RiddleInput {
        riddle: RiddleInfo {
            question: "I am an odd number. Take away a letter and I become even. What number am I?"
                .to_string(),
            sha256_answer: "3ba8d02b16fd2a01c1a8ba1a1f036d7ce386ed953696fa57331c2ac48a80b255"
                .to_string(),
            kind: RiddleKind::Culture,
        },
        grade: RiddleGrade::Easy,
    };
    contract.add_riddle(input);
    // TODO: id is right
    // let riddle = contract.get_riddles_of_kind(&RiddleKind::Kid).first().unwrap();
    // assert_eq!(contract.answer_riddle(
    //     AnswerInfo { id: riddle.id, sha256_answer: "3ba8d02b16fd2a01c1a8ba1a1f036d7ce386ed953696fa57331c2ac48a80b255".to_string() }).is_none(), true);
}

#[test]
fn get_riddles() {
    let context = get_context(carol(), carol());
    // set the testing environment
    testing_env!(context);
    let mut contract = RiddleGame::default();

    // input
    let input = RiddleInput {
        riddle: RiddleInfo {
            question: "I am an odd number. Take away a letter and I become even. What number am I?"
                .to_string(),
            sha256_answer: "3ba8d02b16fd2a01c1a8ba1a1f036d7ce386ed953696fa57331c2ac48a80b255"
                .to_string(),
            kind: RiddleKind::Culture,
        },
        grade: RiddleGrade::Easy,
    };
    contract.add_riddle(input);
    assert_eq!(contract.get_riddles().len(), 1);
}

#[test]
fn get_riddles_of_kind() {
    // get our VMContext from earlier
    let context = get_context(carol(), carol());
    // set the testing environment
    testing_env!(context);
    let mut contract = RiddleGame::default();

    assert_eq!(
        contract
            .get_riddles_of_kind(&RiddleKind::Culture)
            .first()
            .is_some(),
        false
    );

    // input
    let input = RiddleInput {
        riddle: RiddleInfo {
            question: "I am an odd number. Take away a letter and I become even. What number am I?"
                .to_string(),
            sha256_answer: "3ba8d02b16fd2a01c1a8ba1a1f036d7ce386ed953696fa57331c2ac48a80b255"
                .to_string(),
            kind: RiddleKind::Culture,
        },
        grade: RiddleGrade::Easy,
    };
    contract.add_riddle(input);
    assert_eq!(
        contract
            .get_riddles_of_kind(&RiddleKind::Culture)
            .first()
            .is_some(),
        true
    );
}
