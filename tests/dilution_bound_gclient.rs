mod utils_gclient;

use utils_gclient::*;

#[tokio::test]
async fn dilution_bound_gclient() {
    let (api, mut listener, ft_token_id, dao_id) =
        setup_gclient().await.expect("Unable to setup gclient.");

    let applicant: &str = RANDOM_APPLICANT;
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let mut total_shares = 10 * shares_requested + 1;
    let mut balance = 10 * token_tribute;
    let ragequit_amount: u128 = 9_000;
    let quorum: u128 = 10;
    let mut proposal_id: u128 = 0;
    let last_start_proposal_timestamp;

    // Add members to DAO
    println!("-> Add members to DAO!");

    for applicant in APPLICANTS {
        println!("Applicant: {applicant}");

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
        token_approve(&api, &mut listener, ft_token_id, 1, dao_id, token_tribute)
            .await
            .expect("Unable to approve token.");

        let api = api.with(ADMIN).expect("Unable to change signer.");
        dao_add_member(
            &api,
            &mut listener,
            dao_id,
            proposal_id,
            applicant_id,
            token_tribute,
            shares_requested,
        )
        .await
        .expect("Unable to add dao member");

        proposal_id += 1;
    }

    // Membership proposal
    {
        println!("-> Membership proposal!");

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
        token_approve(&api, &mut listener, ft_token_id, 1, dao_id, token_tribute)
            .await
            .expect("Unable to approve token.");

        let api = api.with(ADMIN).expect("Unable to change signer.");
        dao_add_to_whitelist(&api, &mut listener, dao_id, applicant_id, false)
            .await
            .expect("Unable to add applicant to dao whitelist");

        dao_submit_membership_proposal(
            &api,
            &mut listener,
            dao_id,
            proposal_id,
            applicant_id,
            token_tribute,
            shares_requested,
            quorum,
            false,
        )
        .await
        .expect("Unable to submit dao membership proposal.");

        last_start_proposal_timestamp = api
            .last_block_timestamp()
            .await
            .expect("Unable obtain block timestamp.");
    }

    // Voting
    {
        println!("-> Voting!");

        let api = api
            .clone()
            .with(APPLICANTS[0])
            .expect("Unable to change signer.");
        dao_submit_vote(
            &api,
            &mut listener,
            dao_id,
            proposal_id,
            dao::io::Vote::Yes,
            false,
        )
        .await
        .expect("Unable to submit dao vote.");

        let api = api
            .clone()
            .with(APPLICANTS[1])
            .expect("Unable to change signer.");
        dao_submit_vote(
            &api,
            &mut listener,
            dao_id,
            proposal_id,
            dao::io::Vote::Yes,
            false,
        )
        .await
        .expect("Unable to submit dao vote.");
    }

    // Ragequit
    {
        println!("-> Ragequit!");

        for applicant in APPLICANTS.iter().skip(2) {
            let api = api
                .clone()
                .with(applicant)
                .expect("Unable to change signer.");

            let funds = (balance * ragequit_amount) / (total_shares);
            dao_ragequit(&api, &mut listener, dao_id, ragequit_amount, funds, false)
                .await
                .expect("Unable to dao ragequit.");
            total_shares -= ragequit_amount;
            balance -= funds;
        }
    }

    // Process proposal
    {
        println!("-> Process proposal!");

        let api = api.clone().with(ADMIN).expect("Unable to change signer.");

        wait_for_voting_finish(&api, last_start_proposal_timestamp)
            .await
            .expect("Unable to wait for voting finish.");
        dao_process_proposal(&api, &mut listener, dao_id, proposal_id, false, false)
            .await
            .expect("Unable to process dao proposal.");
    }

    /* let system = System::new();
    system.init_logger();
    let ftoken = Program::ftoken(&system);
    let dao = Program::dao(&system);
    let applicant: u64 = 200;
    let token_tribute: u128 = 10_000;
    let shares_requested: u128 = 10_000;
    let mut total_shares = 10 * shares_requested + 1;
    let mut balance = 10 * token_tribute;
    let ragequit_amount: u128 = 9_000;
    let quorum: u128 = 10;
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

    // APPLICANT[0] votes YES
    dao.submit_vote(APPLICANTS[0], proposal_id, Vote::Yes, false);
    // APPLICANT[1] votes YES
    dao.submit_vote(APPLICANTS[1], proposal_id, Vote::Yes, false);

    // APPLICANT[2]-APPLICANT[9] ragequit
    for applicant in APPLICANTS.iter().take(10).skip(2) {
        let funds = (balance * ragequit_amount) / (total_shares);
        dao.ragequit(*applicant, ragequit_amount, funds, false);
        total_shares -= ragequit_amount;
        balance -= funds;
    }

    // quorum is achieved and number of YES votes > NO votes
    // but max_total_shares_at_yes_vote > total_shares * dilution_bound
    // proposal is not passed
    system.spend_blocks(((VOTING_PERIOD_LENGTH + GRACE_PERIOD_LENGTH) / 1000) as u32);
    dao.process_proposal(proposal_id, false, false); */
}
