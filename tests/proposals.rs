mod test_env;

use near_sdk::{borsh::to_vec, NearToken};
use serde_json::Value;
use {crate::test_env::*, serde_json::json};

#[tokio::test]
async fn test_proposal() -> anyhow::Result<()> {
    // Initialize the devhub and near social contract on chain,
    // contract is devhub contract instance.
    let (contract, _) = init_contracts_from_res().await?;

    let deposit_amount = NearToken::from_near(2);

    let add_proposal = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "Hello to @petersalomonsen.near and @psalomo.near. This is an idea with mentions.",
                "category": "cat",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "DRAFT"}
            },
            "labels": ["test1", "test2"],
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

    let edit_proposal = contract
        .call("edit_proposal")
        .args_json(json!({
            "id": 0,
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "Hello to @petersalomonsen.near and @psalomo.near. This is an idea with mentions.",
                "category": "another",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "DRAFT"}
            },
            "labels": ["test1", "test2"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    let get_proposal: serde_json::Value = contract
        .call("get_proposal")
        .args_json(json!({
            "proposal_id" : 0
        }))
        .view()
        .await?
        .json()?;

    assert_eq!(get_proposal["snapshot"]["category"], "another");

    let add_second_proposal = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "One more",
                "description": "Hello to @petersalomonsen.near and @psalomo.near. This is an idea with mentions.",
                "category": "cat",
                "summary": "sum",
                "linked_proposals": [],
                "requested_sponsorship_amount": "200",
                "requested_sponsorship_token": "NEAR",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "DRAFT"}
            },
            "labels": ["test3"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    let get_proposals = contract
        .call("get_proposals")
        .args_json(json!({}))
        .view()
        .await?
        .json::<Value>()?;

    let proposals_array = get_proposals.as_array().unwrap();

    assert_eq!(proposals_array.len(), 2);
    assert_eq!(proposals_array.get(1).unwrap()["snapshot"]["name"], "One more");

    let get_proposal_ids = contract
        .call("get_all_proposal_ids")
        .args_json(json!({}))
        .view()
        .await?
        .json::<Value>()?;

    let proposal_ids = get_proposal_ids
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.clone().as_u64().unwrap()).collect::<Vec<_>>();

    let expected_ids = [0u64, 1u64].to_vec();

    assert_eq!(proposal_ids, expected_ids);

    // let get_proposals_by_label = contract
    //     .call("get_proposals")
    //     .args_json(json!({}))
    //     .view()
    //     .await?
    //     .json::<Value>()?;

    

    Ok(())
}
