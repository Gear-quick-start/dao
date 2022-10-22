#![no_std]

use codec::{Decode, Encode};
use gstd::{ActorId, String};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
pub enum DaoAction {
    /// Adds members to whitelist.
    ///
    /// Requirements:
    /// * Only admin can add actors to whitelist;
    /// * Member ID cant be zero;
    /// * Member can not be added to whitelist more than once;
    ///
    /// Arguments:
    /// * `member`: valid actor ID
    ///
    /// On success replies with [`DaoEvent::MemberAddedToWhitelist`]
    AddToWhiteList(ActorId),

    /// The proposal of joining the DAO.
    ///
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses;
    /// * The applicant account must be either a DAO member or is in the whitelist.
    ///
    /// Arguments:
    /// * `applicant`: an actor who wishes to become a DAO member;
    /// * `token_tribute`: the number of tokens the applicant offered for shares in DAO;
    /// * `shares_requested`: the amount of shares the applicant is requesting for his token tribute;
    /// * `quorum`: a certain threshold of YES votes in order for the proposal to pass;
    /// * `details`: the proposal description.
    ///
    /// On success replies with [`DaoEvent::SubmitMembershipProposal`]
    SubmitMembershipProposal {
        applicant: ActorId,
        token_tribute: u128,
        shares_requested: u128,
        quorum: u128,
        details: String,
    },

    /// The proposal of funding.
    ///
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses;
    /// * The receiver ID can't be the zero;
    /// * The DAO must have enough funds to finance the proposal;
    ///
    /// Arguments:
    /// * `receiver`: an actor that will be funded;
    /// * `amount`: the number of fungible tokens that will be sent to the receiver;
    /// * `quorum`: a certain threshold of YES votes in order for the proposal to pass;
    /// * `details`: the proposal description;
    ///
    /// On success replies with [`DaoEvent::SubmitFundingProposal`]
    SubmitFundingProposal {
        applicant: ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    },

    /// The proposal processing after the proposal completes during the grace period.
    /// If the membership proposal is accepted, the tribute tokens are deposited into the contract
    /// and new shares are minted and issued to the applicant.
    /// If the membership proposal is rejected, the tribute tokens are returned to the applicant.
    /// If the funding proposal is accepted, the indicated amount of tokens is transfered to the applicant;
    /// If the funging proposal is rejected, the indicated amount of tokens remains in the contract.
    ///
    /// Requirements:
    /// * The previous proposal must be processed;
    /// * The proposal must exist and be ready for processing;
    /// * The proposal must not be aborted or already be processed.
    ///
    /// Arguments:
    /// * `proposal_id`: the proposal ID
    ///
    /// On success replies with [`DaoEvent::ProcessProposal`]
    ProcessProposal(u128),

    /// The member (or the delegate address of the member) submits his vote (YES or NO) on the proposal.
    ///
    /// Requirements:
    /// * The proposal can be submitted only by the existing members or their delegate addresses;
    /// * The member can vote on the proposal only once;
    /// * Proposal must exist, the voting period must has started and not expired;
    /// * Proposal must not be aborted.
    ///
    /// Arguments:
    /// * `proposal_id`: the proposal ID
    /// * `vote`: the member  a member vote (YES or NO)
    ///
    /// On success replies with [`DaoEvent::SubmitVote`]
    SubmitVote { proposal_id: u128, vote: Vote },

    /// Withdraws the capital of the member.
    ///
    /// Requirements:
    /// * `msg::source()` must be DAO member;
    /// * The member must have sufficient amount;
    /// * The latest proposal the member voted YES must be processed;
    /// * Admin can ragequit only after transferring his role to another actor.
    ///
    /// Arguments:
    /// * `amount`: The amount of shares the member would like to withdraw (the shares are converted to fungible tokens)
    ///
    /// On success replies with [`DaoEvent::RageQuit`]
    RageQuit(u128),

    /// Aborts the membership proposal.
    /// It can be used in case when applicant is disagree with the requested shares
    /// or the details the proposer indicated by the proposer.
    ///
    /// Requirements:
    /// * `msg::source()` must be the applicant;
    /// * The proposal must be membership proposal;
    /// * The proposal can be aborted during only the abort window
    /// * The proposal has not be already aborted.
    ///
    /// Arguments:
    /// * `proposal_id`: the proposal ID
    ///
    /// On success replies with [`DaoEvent::Abort`]
    Abort(u128),

    /// Sets the delegate key that is responsible for submitting proposals and voting;
    /// The deleagate key defaults to member address unless updated.
    ///
    /// Requirements:
    /// * `msg::source()` must be DAO member;
    /// * The delegate key must not be zero address;
    /// * A delegate key can be assigned only to one member.
    ///
    /// Arguments:
    /// * `new_delegate_key`: the valid actor ID.
    ///
    /// On success replies with [`DaoEvent::DelegateKeyUpdated`]
    UpdateDelegateKey(ActorId),

    /// Assigns the admin position to new actor.
    ///
    /// Requirements:
    /// * Only admin can assign new admin.
    ///
    /// Arguments:
    /// * `new_admin`: valid actor ID.
    ///
    /// On success replies with [`DaoEvent::AdminUpdated`]
    SetAdmin(ActorId),

    /// Continues the transaction if it fails due to lack of gas
    /// or due to an error in the token contract.
    ///
    /// Requirements:
    /// * Transaction must exist.
    ///
    /// Arguments:
    /// * `transaction_id`: valid actor ID.
    ///
    /// On success replies with the payload of continued transaction.
    Continue(u64),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum DaoEvent {
    MemberAddedToWhitelist(ActorId),
    SubmitMembershipProposal {
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        token_tribute: u128,
    },
    SubmitFundingProposal {
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    SubmitVote {
        account: ActorId,
        proposal_id: u128,
        vote: Vote,
    },
    ProcessProposal {
        proposal_id: u128,
        passed: bool,
    },
    RageQuit {
        member: ActorId,
        amount: u128,
    },
    Abort(u128),
    AdminUpdated(ActorId),
    DelegateKeyUpdated {
        member: ActorId,
        delegate: ActorId,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitDao {
    pub admin: ActorId,
    pub approved_token_program_id: ActorId,
    pub period_duration: u64,
    pub voting_period_length: u64,
    pub grace_period_length: u64,
    pub dilution_bound: u8,
    pub abort_window: u64,
}

#[derive(Debug, Encode, Decode, Clone, TypeInfo)]
pub enum Vote {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Copy, Clone)]
pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}
