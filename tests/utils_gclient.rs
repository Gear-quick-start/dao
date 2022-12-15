mod utils;

use blake2_rfc::blake2b;
use dao::io::{DaoAction, InitDao, Vote};
use ft_logic_io::Action;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken};
use gclient::{EventListener, EventProcessor, GearApi, Result};
use gstd::{prelude::*, ActorId, Encode};
use utils::{
    Hash, ABORT_WINDOW, ADMIN, DAO_WASM_PATH, DILUTION_BOUND, FT_LOGIC_WASM_PATH,
    FT_MAIN_WASM_PATH, FT_STORAGE_WASM_PATH, GRACE_PERIOD_LENGTH, HASH_LENGTH, PERIOD_DURATION,
    VOTING_PERIOD_LENGTH,
};

async fn dao_add_to_whitelist(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    account: u64,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::AddToWhiteList(account.into());

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_submit_membership_proposal(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    _proposal_id: u128,
    applicant: u64,
    token_tribute: u128,
    shares_requested: u128,
    quorum: u128,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::SubmitMembershipProposal {
        applicant: applicant.into(),
        token_tribute,
        shares_requested,
        quorum,
        details: String::from(""),
    };

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_submit_funding_proposal(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    _proposal_id: u128,
    applicant: u64,
    amount: u128,
    quorum: u128,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::SubmitFundingProposal {
        applicant: applicant.into(),
        amount,
        quorum,
        details: String::from(""),
    };

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_process_proposal(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    proposal_id: u128,
    _passed: bool,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::ProcessProposal(proposal_id);

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_submit_vote(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    proposal_id: u128,
    vote: Vote,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::SubmitVote { proposal_id, vote };

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_ragequit(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    amount: u128,
    _funds: u128,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::RageQuit(amount);

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_abort(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    proposal_id: u128,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::Abort(proposal_id);

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_update_delegate_key(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    account: u64,
    error: bool,
) -> Result<()> {
    let payload = DaoAction::UpdateDelegateKey(account.into());

    let program_id: Hash = dao_id
        .encode()
        .try_into()
        .expect("Unexpected invalid dao id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    assert_eq!(
        listener.message_processed(message_id).await?.succeed(),
        !error
    );

    Ok(())
}

async fn dao_add_member(
    api: &GearApi,
    listener: &mut EventListener,
    dao_id: ActorId,
    proposal_id: u128,
    applicant: u64,
    token_tribute: u128,
    shares_requested: u128,
) -> Result<()> {
    dao_add_to_whitelist(api, listener, dao_id, applicant, false).await?;
    dao_submit_membership_proposal(
        api,
        listener,
        dao_id,
        proposal_id,
        applicant,
        token_tribute,
        shares_requested,
        0,
        false,
    )
    .await?;
    dao_submit_vote(api, listener, dao_id, proposal_id, Vote::Yes, false).await?;
    // TODO: Spend blocks
    dao_process_proposal(api, listener, dao_id, proposal_id, true, false).await?;

    Ok(())
}

async fn token_mint(
    api: &GearApi,
    token_id: ActorId,
    transaction_id: u64,
    account: u64,
    amount: u128,
) -> Result<()> {
    let payload = FTokenAction::Message {
        transaction_id,
        payload: Action::Mint {
            recipient: account.into(),
            amount,
        }
        .encode(),
    };

    let program_id: Hash = token_id
        .encode()
        .try_into()
        .expect("Unexpected invalid token id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    api.send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    Ok(())
}

async fn token_approve(
    api: &GearApi,
    token_id: ActorId,
    transaction_id: u64,
    approved_account: u64,
    amount: u128,
) -> Result<()> {
    let payload = FTokenAction::Message {
        transaction_id,
        payload: Action::Approve {
            approved_account: approved_account.into(),
            amount,
        }
        .encode(),
    };

    let program_id: Hash = token_id
        .encode()
        .try_into()
        .expect("Unexpected invalid token id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    api.send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    Ok(())
}

async fn token_check_balance(
    api: &GearApi,
    token_id: ActorId,
    account: u64,
    expected_amount: u128,
) -> Result<()> {
    let account: Hash = account
        .to_le_bytes()
        .to_vec()
        .try_into()
        .expect("Unexpected invalid account.");
    let payload = FTokenAction::GetBalance(account.into());

    let program_id: Hash = token_id
        .encode()
        .try_into()
        .expect("Unexpected invalid token id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true, None)
        .await?;

    let (message_id, _hash) = api
        .send_message(program_id.into(), payload.encode(), gas_info.min_limit, 0)
        .await?;

    let (stored_message, _) = api
        .get_from_mailbox(&account.into(), message_id)
        .await?
        .expect("Unexpected empty mailbox.");
    assert_eq!(
        stored_message.payload(),
        &FTokenEvent::Balance(expected_amount).encode()
    );

    Ok(())
}

async fn upload_and_init_program(
    api: &GearApi,
    listener: &mut EventListener,
    wasm_path: impl AsRef<str>,
    init_payload: &Vec<u8>,
) -> Result<ActorId> {
    // 1. Calculate gas limit for program upload
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(wasm_path.as_ref())?,
            init_payload.clone(),
            0,
            true,
            None,
        )
        .await?;

    // 2. Upload program
    let (message_id, program_id, _hash) = api
        .upload_program_bytes_by_path(
            wasm_path.as_ref(),
            gclient::bytes_now(),
            init_payload,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(ActorId::new(
        program_id
            .encode()
            .try_into()
            .expect("Unexpected invalid `ProgramId` length."),
    ))
}

async fn upload_with_code_hash(api: &GearApi, wasm_path: impl AsRef<str>) -> Result<Hash> {
    let mut code_hash: Hash = Default::default();
    let wasm_code = gclient::code_from_os(wasm_path.as_ref())?;

    code_hash[..].copy_from_slice(blake2b::blake2b(HASH_LENGTH, &[], &wasm_code).as_bytes());

    api.upload_code(wasm_code).await?;

    Ok(code_hash)
}

#[tokio::test]
async fn setup_gclient() -> Result<()> {
    let api = GearApi::dev().await?.with("//Bob")?;
    let mut listener = api.subscribe().await?;

    assert!(listener.blocks_running().await?);

    // 1. Upload ft code hashes
    let ft_logic_code_hash = upload_with_code_hash(&api, FT_LOGIC_WASM_PATH).await?;
    let ft_storage_code_hash = upload_with_code_hash(&api, FT_STORAGE_WASM_PATH).await?;

    // 2. Upload main ft
    let ft_token_id = upload_and_init_program(
        &api,
        &mut listener,
        FT_MAIN_WASM_PATH,
        &InitFToken {
            storage_code_hash: ft_storage_code_hash.into(),
            ft_logic_code_hash: ft_logic_code_hash.into(),
        }
        .encode(),
    )
    .await?;

    // 3. Upload dao
    let _dao_id = upload_and_init_program(
        &api,
        &mut listener,
        DAO_WASM_PATH,
        &InitDao {
            admin: ADMIN.into(),
            approved_token_program_id: ft_token_id,
            period_duration: PERIOD_DURATION,
            voting_period_length: VOTING_PERIOD_LENGTH,
            grace_period_length: GRACE_PERIOD_LENGTH,
            dilution_bound: DILUTION_BOUND,
            abort_window: ABORT_WINDOW,
        }
        .encode(),
    )
    .await?;

    Ok(())
}
