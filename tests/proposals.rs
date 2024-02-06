mod test_env;

use near_sdk::{borsh::to_vec, NearToken};
use serde_json::Value;
use {crate::test_env::*, serde_json::json};

#[tokio::test]
async fn test_proposal() -> anyhow::Result<()> {
    // Initialize the devhub and near social contract on chain,
    // contract is devhub contract instance.
    let (contract, worker) = init_contracts_from_res().await?;

    let deposit_amount = NearToken::from_near(2);

    let _add_proposal = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "Marketing",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [ ],
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

    assert_eq!(get_proposal["snapshot"]["category"], "Marketing");

    let social_db_post_block_height: u64 = get_proposal["social_db_post_block_height"].as_str().unwrap().parse::<u64>()?;
    assert!(social_db_post_block_height > 0);

    let _edit_proposal = contract
        .call("edit_proposal")
        .args_json(json!({
            "id": 0,
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "Events",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "REVIEW", "sponsor_requested_review": true, "reviewer_completed_attestation": false }
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

    assert_eq!(get_proposal["snapshot"]["category"], "Events");

    let _add_second_proposal = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "One more",
                "description": "some description",
                "category": "Events",
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

    let expected_ids: Vec<u64> = [0u64, 1u64].to_vec();

    assert_eq!(proposal_ids, expected_ids);
    
    let second_account = worker.root_account()?
        .create_subaccount("second")
        .initial_balance(NearToken::from_near(20))
        .transact()
        .await?
        .into_result()?;

    let _second_author_add_proposal = second_account.call(contract.id(), "add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "another author",
                "description": "some description",
                "category": "Events",
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
            "labels": ["test2", "test3"],
        }))
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    let get_proposal: serde_json::Value = contract
        .call("get_proposal")
        .args_json(json!({
            "proposal_id" : 2
        }))
        .view()
        .await?
        .json()?;

    assert_eq!(get_proposal["author_id"], "second.test.near");

    let get_proposals_by_author = contract
        .call("get_proposals_by_author")
        .args_json(json!({
            "author": "devhub.near"
        }))
        .view()
        .await?
        .json::<Value>()?;

    let proposals_array = get_proposals_by_author.as_array().unwrap();

    assert_eq!(proposals_array.len(), 2);

    let get_proposals_by_label = contract
        .call("get_proposals_by_label")
        .args_json(json!({
            "label": "test2"
        }))
        .view()
        .await?
        .json::<Value>()?;

    let proposal_ids = get_proposals_by_label
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap()).collect::<Vec<_>>();

    let expected_ids: Vec<u64> = [0u64, 2u64].to_vec();
    assert_eq!(proposal_ids, expected_ids);

    let get_all_proposal_labels = contract
        .call("get_all_proposal_labels")
        .args_json(json!({}))
        .view()
        .await?
        .json::<Value>()?;

    let proposal_labels = get_all_proposal_labels
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap()).collect::<Vec<_>>();

    let expected_labels: Vec<&str> = ["test1", "test2", "test3"].to_vec();
    assert_eq!(proposal_labels, expected_labels);

    let get_all_proposal_authors = contract
        .call("get_all_proposal_authors")
        .args_json(json!({}))
        .view()
        .await?
        .json::<Value>()?;

    let proposal_authors = get_all_proposal_authors
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap()).collect::<Vec<_>>();

    let expected_authors: Vec<&str> = ["devhub.near", "second.test.near"].to_vec();
    assert_eq!(proposal_authors, expected_authors);

    let is_allowed_to_edit_proposal_false = contract
        .call("is_allowed_to_edit_proposal")
        .args_json(json!({
            "proposal_id": 0,
            "editor": "second.test.near"
        }))
        .view()
        .await?
        .json::<Value>()?;

    assert!(!is_allowed_to_edit_proposal_false.as_bool().unwrap());

    let is_allowed_to_edit_proposal_true = contract
        .call("is_allowed_to_edit_proposal")
        .args_json(json!({
            "proposal_id": 0,
            "editor": "devhub.near"
        }))
        .view()
        .await?
        .json::<Value>()?;

    assert!(is_allowed_to_edit_proposal_true.as_bool().unwrap());

    let get_all_allowed_proposal_labels = contract
        .call("get_all_allowed_proposal_labels")
        .args_json(json!({
            "editor": "devhub.near"
        }))
        .view()
        .await?
        .json::<Value>()?;

    let proposal_labels = get_all_allowed_proposal_labels
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap()).collect::<Vec<_>>();

    let expected_labels: Vec<&str> = ["test1", "test2", "test3"].to_vec();
    assert_eq!(proposal_labels, expected_labels);

    let add_proposal_incorrect = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "Events",
                "summary": "sum",
                "linked_proposals": [],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "REVIEW", "sponsor_requested_review": false, "reviewer_completed_attestation": false }
            },
            "labels": ["test1", "test2"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;
    assert!(!add_proposal_incorrect.is_success());

    let add_proposal_incorrect = contract
        .call("add_proposal")
        .args_json(json!({
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "Events",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [ "2cXzSP1Z9AM8A7mg18voh9c4sBmiUzxzyDXiYW5fiZd6" ],
                "timeline_status": {"timeline_status": "DRAFT"}
            },
            "labels": ["test1", "test2"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    assert!(!add_proposal_incorrect.is_success());


    let edit_proposal_incorrect = second_account.call(contract.id(), "edit_proposal")
        .args_json(json!({
            "id": 2,
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "Events",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "REVIEW", "sponsor_requested_review": true, "reviewer_completed_attestation": false }
            },
            "labels": ["test1", "test2"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    assert!(!edit_proposal_incorrect.is_success());

    let edit_proposal = second_account.call(contract.id(), "edit_proposal")
        .args_json(json!({
            "id": 2,
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "Events",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [ "5PHaiXRvtZTYVSEBN5prT6M1odceCPxKzgpTZDmqrZsC" ],
                "timeline_status": {"timeline_status": "REVIEW", "sponsor_requested_review": false, "reviewer_completed_attestation": false }
            },
            "labels": ["test1", "test2"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    assert!(edit_proposal.is_success());

    let set_categories = second_account
        .call(contract.id(), "set_allowed_categories")
        .args_json(json!({"new_categories": ["One", "Two"]}))
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    assert!(set_categories.is_failure());

    let _set_categories = contract
        .call("set_allowed_categories")
        .args_json(json!({"new_categories": ["Two", "Three"]}))
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    let get_categories: serde_json::Value = contract
        .call("get_allowed_categories")
        .args_json(json!({}))
        .view()
        .await?
        .json()?;

    let categories: Vec<String> = get_categories
        .as_array()
        .unwrap()
        .iter()
        .map(|x| String::from(x.clone().as_str().unwrap())).collect::<Vec<_>>();

    assert_eq!(categories, vec!["Two", "Three"]);

    let edit_proposal = contract
        .call("edit_proposal")
        .args_json(json!({
            "id": 0,
            "body": {
                "proposal_body_version": "V0",
                "name": "another post",
                "description": "some description",
                "category": "bad cat",
                "summary": "sum",
                "linked_proposals": [{"link_type": "PostId", "id": 1}, {"link_type": "PostId", "id": 3}],
                "requested_sponsorship_amount": "1000000000",
                "requested_sponsorship_token": "USD",
                "receiver_account": "polyprogrammist.near",
                "supervisor": "frol.near",
                "sponsor": "neardevdao.near",
                "payouts": [],
                "timeline_status": {"timeline_status": "REVIEW", "sponsor_requested_review": true, "reviewer_completed_attestation": false }
            },
            "labels": ["test1", "test2"],
        }))
        .max_gas()
        .deposit(deposit_amount)
        .transact()
        .await?;

    assert!(edit_proposal.is_failure());

    Ok(())
}
