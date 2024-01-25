use std::collections::HashSet;

use crate::str_serializers::*;
use crate::{Balance, SponsorshipToken};

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Timestamp};

pub type ProposalId = u64;

type PostTag = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "proposal_version")]
#[borsh(crate = "near_sdk::borsh")]
pub enum VersionedProposal {
    V0(Proposal),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct Proposal {
    pub id: ProposalId,
    pub author_id: AccountId,
    pub snapshot: ProposalSnapshot,
    // // Excludes the current snapshot itself.
    pub snapshot_history: Vec<ProposalSnapshot>,
}

impl From<Proposal> for VersionedProposal {
    fn from(p: Proposal) -> Self {
        VersionedProposal::V0(p)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct ProposalSnapshot {
    pub editor_id: AccountId,
    #[serde(with = "u64_dec_format")]
    pub timestamp: Timestamp,
    pub labels: HashSet<PostTag>,
    #[serde(flatten)]
    pub body: VersionedProposalBody,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct ProposalBodyV0 {
    pub name: String,
    pub description: String,
    pub requested_sponsor: Option<AccountId>,
    #[serde(with = "u128_dec_format")]
    pub requested_sponsorship_amount: Balance,
    pub requested_sponsorship_token: Option<SponsorshipToken>,
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "proposal_body_version")]
#[borsh(crate = "near_sdk::borsh")]
pub enum VersionedProposalBody {
    V0(ProposalBodyV0),
}

impl From<VersionedProposalBody> for ProposalBodyV0 {
    fn from(solution: VersionedProposalBody) -> Self {
        match solution {
            VersionedProposalBody::V0(v0) => v0,
        }
    }
}

impl VersionedProposalBody {
    pub fn latest_version(self) -> ProposalBodyV0 {
        self.into()
    }
}

pub fn get_proposal_description(proposal: Proposal) -> String {
    return proposal.snapshot.body.clone().latest_version().description;
}