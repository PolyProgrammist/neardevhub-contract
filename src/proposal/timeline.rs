use near_sdk::borsh::{BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde", tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
#[borsh(crate = "near_sdk::borsh")]
pub enum TimelineStatus {
    Draft,
    Review(ReviewStatus),
    Approved(ReviewStatus),
    Rejected(ReviewStatus),
    ApprovedConditionally(ReviewStatus),
    PaymentProcessing(PaymentProcessingStatus),
    Funded(FundedStatus),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct ReviewStatus {
    sponsor_requested_review: bool,
    reviewer_completed_attestation: bool
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct PaymentProcessingStatus {
    #[serde(flatten)]
    review_status: ReviewStatus,
    kyc_verified: bool,
    test_transaction_sent: bool,
    request_for_trustees_created: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct FundedStatus {
    #[serde(flatten)]
    payment_processing_status: PaymentProcessingStatus,
    trustees_released_payment: bool,
}


pub fn is_draft(ts: &TimelineStatus) -> bool {
    match ts {
        TimelineStatus::Draft => true,
        _ => false,
    }
}

pub fn is_empty_review(ts: &TimelineStatus) -> bool {
    match ts {
        TimelineStatus::Review(review_status) => {
            return !review_status.sponsor_requested_review && !review_status.reviewer_completed_attestation;
        },
        _ => false,
    }
}
