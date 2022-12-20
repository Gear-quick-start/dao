mod utils;
mod utils_gclient;

use dao::io::*;
use gstd::prelude::*;
use utils_gclient::*;

#[tokio::test]
async fn membership_proposals_gclient() {
    let (api, mut listener, ft_token_id, dao_id) =
        setup_gclient().await.expect("Unable to setup gclient.");

    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let mut proposal_id: u128 = 0;

    // Add members to DAO
    for applicant in APPLICANTS {
        let api = api
            .clone()
            .with(applicant)
            .expect("Unable to change signer.");
        let applicant_id = api.get_actor_id();

        token_mint(
            &api,
            &mut listener,
            ft_token_id,
            0,
            applicant_id,
            token_tribute,
        )
        .await
        .expect("Unable to mint token.");
        /* token_approve(&api, ft_token_id, 1, dao_id, token_tribute)
            .await
            .expect("Unable to approve token.");

        let api = api.with(ADMIN).expect("Unable to change signer."); */
        /* dao_add_member(
            &api,
            &mut listener,
            dao_id,
            proposal_id,
            applicant_id,
            token_tribute,
            shares_requested,
        )
        .await
        .expect("Unable to add dao member"); */

        proposal_id += 1;

        /* ftoken.mint(0, *applicant, *applicant, token_tribute);
        ftoken.approve(1, *applicant, DAO_ID, token_tribute);
        dao.add_member(
            &system,
            proposal_id,
            *applicant,
            token_tribute,
            shares_requested,
        );
        proposal_id += 1; */
    }

    /* let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let applicant: u64 = 200;
    let quorum: u128 = 50;
    let mut proposal_id: u128 = 0;

    // add members to DAO
    for applicant in APPLICANTS {
        ftoken.mint(0, *applicant, *applicant, token_tribute);
        ftoken.approve(1, *applicant, DAO_ID, token_tribute);
        dao.add_member(
            &system,
            proposal_id,
            *applicant,
            token_tribute,
            shares_requested,
        );
        proposal_id += 1;
    }

    //membership proposal
    ftoken.mint(0, applicant, applicant, token_tribute);
    ftoken.approve(1, applicant, DAO_ID, token_tribute);

    dao.add_to_whitelist(ADMIN, applicant, false);
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // members of DAO vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::Yes } else { Vote::No };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal passed
    dao.process_proposal(proposal_id, true, false);

    // check balance of applicant
    ftoken.check_balance(applicant, 0);

    // new proposal
    ftoken.mint(2, applicant, applicant, token_tribute);
    ftoken.approve(3, applicant, DAO_ID, token_tribute);
    proposal_id += 1;
    dao.submit_membership_proposal(
        ADMIN,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        quorum,
        false,
    );

    // DAO members vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::No } else { Vote::Yes };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal didn't pass
    dao.process_proposal(proposal_id, false, false);

    // check balance of applicant (it must be equal to token tribute since proposal did not pass)
    ftoken.check_balance(applicant, token_tribute);
    // check balance of DAO
    ftoken.check_balance(DAO_ID, 11 * token_tribute); */
}

#[tokio::test]
async fn funding_proposals_gclient() {
    /* let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let amount = 30_000;
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let receiver: u64 = 100;
    let quorum: u128 = 50;
    let mut proposal_id: u128 = 0;

    // add members to DAO
    for applicant in APPLICANTS {
        ftoken.mint(0, *applicant, *applicant, token_tribute);
        ftoken.approve(1, *applicant, DAO_ID, token_tribute);
        dao.add_member(
            &system,
            proposal_id,
            *applicant,
            token_tribute,
            shares_requested,
        );
        proposal_id += 1;
    }

    //funding proposal
    dao.submit_funding_proposal(ADMIN, proposal_id, receiver, amount, quorum, false);

    // members of DAO vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::Yes } else { Vote::No };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal passed
    dao.process_proposal(proposal_id, true, false);

    // check balance of receiver
    ftoken.check_balance(receiver, amount);
    // check balance of DAO
    ftoken.check_balance(DAO_ID, 10 * token_tribute - amount);

    // new proposal
    proposal_id += 1;
    dao.submit_funding_proposal(ADMIN, proposal_id, receiver, amount, quorum, false);

    // DAO members vote
    for applicant in APPLICANTS {
        let vote: Vote = if applicant < &16 { Vote::No } else { Vote::Yes };
        dao.submit_vote(*applicant, proposal_id, vote, false);
    }

    system.spend_blocks(VOTING_PERIOD_LENGTH as u32 + 1);

    // proposal didn't pass
    dao.process_proposal(proposal_id, false, false);

    // check balance of applicant
    ftoken.check_balance(receiver, amount);
    // check balance of DAO
    ftoken.check_balance(DAO_ID, 10 * token_tribute - amount); */
}
