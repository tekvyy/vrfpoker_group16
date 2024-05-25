use rand::Rng;
use schnorrkel::{signing_context, Keypair};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

fn main() {
    // Generate keypairs for each player
    let keypair1 = Keypair::generate();
    let keypair2 = Keypair::generate();

    let mut rng = rand::thread_rng();

    // Players generate random inputs
    let input1: [u8; 32] = rng.gen();
    let input2: [u8; 32] = rng.gen();

    // Players hash their inputs
    let _commit1 = Sha256::digest(&input1);
    let _commit2 = Sha256::digest(&input2);

    // Players reveal their inputs and derive a common input
    let combined_input = [input1.as_ref(), input2.as_ref()].concat();
    let common_input = Sha256::digest(&combined_input);

    // Create signing context
    let context = signing_context(b"example");

    // Players generate VRF outputs
    let vrf_out1 = keypair1.vrf_sign(context.bytes(&common_input));
    let vrf_out2 = keypair2.vrf_sign(context.bytes(&common_input));

    // Extract the VRF outputs
    let card1 = vrf_out1.0.to_output();
    let card2 = vrf_out2.0.to_output();

    // Example betting round and determining the winner
    let round_result = betting_round(Action::Bet(10), Action::Bet(10));
    let winner = match round_result {
        Some(player) => player,
        None => determine_winner(card1.to_bytes(), card2.to_bytes()),
    };

    println!("Winner is player {}", winner);
}

// Simplified betting logic
enum Action {
    Bet(u32),
    Fold,
}

fn betting_round(player1_action: Action, player2_action: Action) -> Option<usize> {
    match (player1_action, player2_action) {
        (Action::Fold, _) => Some(2),
        (_, Action::Fold) => Some(1),
        (Action::Bet(bet1), Action::Bet(bet2)) if bet1 == bet2 => None, // Continue to next round or reveal cards
        _ => None, // Handle other cases as necessary
    }
}

// Determine the winner by comparing cards
fn determine_winner(card1: [u8; 32], card2: [u8; 32]) -> usize {
    match card1.cmp(&card2) {
        Ordering::Greater => 1,
        Ordering::Less => 2,
        Ordering::Equal => 0, // In case of a tie
    }
}
