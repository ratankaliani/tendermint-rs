use core::{ops::Sub, time::Duration};
use serde::Deserialize;
use std::str::FromStr;
use tendermint::{validator::Info, Time, Vote};
use tendermint_light_client_verifier::{
    errors::VerificationErrorDetail,
    options::Options,
    types::{Commit, Header, LightBlock, PeerId, SignedHeader, Validator, ValidatorSet},
    ProdVerifier, Verdict, Verifier,
};

// fn new_default_header() {
//     let validators = [Validator::new("1", 50), Validator::new("2", 50)];
//     let header = Header::new(&validators)
//         .height(height)
//         .chain_id(&chain_id)
//         .next_validators(&validators)
//         .time(time);
// }

// pub fn new_default_with_header(header: Header) -> LightBlock {
//     let commit = Commit::new(header.clone(), 1);
//     let signed_header = SignedHeader::new(header, commit);
//     LightBlock {
//         signed_header,
//         validators: header.validators.clone(),
//         next_validators: header.validators,
//         provider: Some(default_peer_id()),
//     }
// }

// /// Generate commit votes from all validators in the header.
// /// This function will panic if the header is not present
// pub fn generate_default_votes(mut commit: Commit) -> Commit {
//     let header = self.header.as_ref().unwrap();
//     let val_to_vote = |(i, v): (usize, &Validator)| -> Vote {
//         Vote::new(v.clone(), header.clone())
//             .index(i as u16)
//             .round(self.round.unwrap_or(1))
//     };
//     let votes = header
//         .validators
//         .as_ref()
//         .unwrap()
//         .iter()
//         .enumerate()
//         .map(val_to_vote)
//         .collect();
//     commit.votes = Some(votes);
//     commit
// }

#[derive(Debug, Deserialize)]
pub struct CommitResponse {
    pub result: SignedHeaderWrapper,
}

#[derive(Debug, Deserialize)]
pub struct SignedHeaderWrapper {
    pub signed_header: SignedHeader,
}

#[derive(Debug, Deserialize)]
pub struct ValidatorSetResponse {
    pub result: BlockValidatorSet,
}

#[derive(Debug, Deserialize)]
pub struct BlockValidatorSet {
    pub block_height: String,
    pub validators: Vec<Info>,
    pub count: String,
    pub total: String,
}

fn main() {
    // Generate the Light Block's without testgen
    let file_content = include_bytes!("../fixtures/1/signed_header.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");

    let commit_response: CommitResponse =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");
    let signed_header = commit_response.result.signed_header;

    let file_content = include_bytes!("../fixtures/1/validators.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");
    let validators_response: ValidatorSetResponse =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");
    let validators = validators_response.result;
    let validators = ValidatorSet::new(validators.validators, None);

    let file_content = include_bytes!("../fixtures/1/next_validators.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");
    let next_validators_response: ValidatorSetResponse =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");
    let next_validators = next_validators_response.result;
    let next_validators = ValidatorSet::new(next_validators.validators, None);

    // Create a default light block with a valid chain-id for height `1` with a timestamp 20
    // secs before now (to be treated as trusted state)
    let light_block_1: LightBlock = LightBlock::new(
        signed_header,
        validators,
        next_validators,
        PeerId::from_str("726bc8d260387cf56ecfad3a6bf6fecd903e18a2").unwrap(),
    );

    // // Generate the Light Block's without testgen
    let file_content = include_bytes!("../fixtures/2/signed_header.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");

    let commit_response: CommitResponse =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");
    let signed_header = commit_response.result.signed_header;

    let file_content = include_bytes!("../fixtures/2/validators.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");
    let validators_response: ValidatorSetResponse =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");
    let validators = validators_response.result;
    let validators = ValidatorSet::new(validators.validators, None);

    let file_content = include_bytes!("../fixtures/2/next_validators.json");
    let file_content_str =
        core::str::from_utf8(file_content).expect("Failed to convert file content to string");
    let next_validators_response: ValidatorSetResponse =
        serde_json::from_str(file_content_str).expect("Failed to parse JSON");
    let next_validators = next_validators_response.result;
    let next_validators = ValidatorSet::new(next_validators.validators, None);

    // Create a default light block with a valid chain-id for height `1` with a timestamp 20
    // secs before now (to be treated as trusted state)
    let light_block_2: LightBlock = LightBlock::new(
        signed_header,
        validators,
        next_validators,
        PeerId::from_str("726bc8d260387cf56ecfad3a6bf6fecd903e18a2").unwrap(),
    );

    let vp = ProdVerifier::default();
    let opt = Options {
        trust_threshold: Default::default(),
        trusting_period: Duration::from_secs(500),
        clock_drift: Default::default(),
    };

    let verify_time = light_block_2.time() + Duration::from_secs(20);

    let verdict = vp.verify_update_header(
        light_block_2.as_untrusted_state(),
        light_block_1.as_trusted_state(),
        &opt,
        verify_time.unwrap(),
    );

    match verdict {
        Verdict::Success => {},
        v => panic!("expected success, got: {:?}", v),
    }
}
