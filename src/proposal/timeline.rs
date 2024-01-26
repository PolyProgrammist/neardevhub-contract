use near_sdk::borsh::{BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub enum TimelineStatus {
    Draft,
    Review { sponsor_requested_review: bool, reviewer_completed_attestation: bool },
    Approved,
    Rejected,
    ApprovedConditionally,
    PaymentProcessing { kyc_verified: bool, test_transaction_sent: bool, request_for_trustees_created: bool },
    Funded
}