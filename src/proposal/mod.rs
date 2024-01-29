pub mod repost;
pub mod timeline;

use std::collections::HashSet;

use crate::post::PostId;
use crate::str_serializers::*;
use crate::{Balance, SponsorshipToken};

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, BlockHeight, CryptoHash, Timestamp};

use self::timeline::TimelineStatus;

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
    #[serde(with = "u64_dec_format")]
    pub block_height: BlockHeight,
    pub snapshot: ProposalSnapshot,
    // // Excludes the current snapshot itself.
    pub snapshot_history: Vec<ProposalSnapshot>,
}

impl From<VersionedProposal> for Proposal {
    fn from(vp: VersionedProposal) -> Self {
        match vp {
            VersionedProposal::V0(v0) => v0,
        }    
    }
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
#[serde(tag = "proposal_link_version")]
#[borsh(crate = "near_sdk::borsh")]
pub enum ProposalLink {
    ProposalId(ProposalId),
    PostId(PostId)
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct ProposalBodyV0 {
    pub name: String,
    pub category: String,
    pub summary: String,
    pub description: String,
    pub linked_proposals: Vec<ProposalLink>,
    #[serde(with = "u128_dec_format")]
    pub requested_sponsorship_amount: Balance,
    pub requested_sponsorship_token: Option<SponsorshipToken>,
    pub receiver_account: AccountId,
    pub requested_sponsor: Option<AccountId>,
    pub supervisor: AccountId,
    pub payouts: Vec<CryptoHash>,
    pub timeline_status: TimelineStatus,
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

pub fn get_proposal_description(proposal_body: VersionedProposalBody) -> String {
    return proposal_body.clone().latest_version().description;
}