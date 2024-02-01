mod test_env;

use near_sdk::NearToken;
use {crate::test_env::*, serde_json::json};

#[tokio::test]
async fn test_proposal() -> anyhow::Result<()> {
    // Initialize the devhub and near social contract on chain,
    // contract is devhub contract instance.
    let (contract, _) = init_contracts_from_res().await?;

    let deposit_amount = NearToken::from_near(2);

    // Add a community
    let add_proposal = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "name": "another post",
                "description": "Hello to @petersalomonsen.near and @psalomo.near. This is an idea with mentions.",
                "post_type": "Idea",
                "idea_version": "V1",
                "category": "cat",
                "summary": "sum",
                "linked_proposals": [],
                "requested_sponsorship_amount": "500",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "DRAFT"},
                "proposal_body_version": "V0"
            },
            "labels": [],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    println!("{:?}", add_proposal);

    let get_proposal: serde_json::Value = contract
        .call("get_proposal")
        .args_json(json!({
            "proposal_id" : 0
        }))
        .view()
        .await?
        .json()?;

    assert_eq!(get_proposal["snapshot"]["category"], "cat");

    let social_db_post_block_height: u64 = get_proposal["social_db_post_block_height"].as_str().unwrap().parse::<u64>()?;
    assert!(social_db_post_block_height > 0);

    Ok(())
}
