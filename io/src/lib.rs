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
    SubmitFundingProposal {
        applicant: ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    },
    ProcessProposal(u128),
    SubmitVote {
        proposal_id: u128,
        vote: Vote,
    },
    RageQuit(u128),
    Abort(u128),
    UpdateDelegateKey(ActorId),
    SetAdmin(ActorId),
    Continue(u64),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum DaoEvent {
    MemberAddedToWhitelist(ActorId),
    TransactionProcessed,
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
        applicant: ActorId,
        proposal_id: u128,
        passed: bool,
    },
    RageQuit {
        member: ActorId,
        amount: u128,
    },
    Abort {
        member: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    Cancel {
        member: ActorId,
        proposal_id: u128,
    },
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
    pub dilution_bound: u128,
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
