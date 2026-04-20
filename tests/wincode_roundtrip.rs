use std::{env, fs, path::PathBuf, process};

use merkle_tree_collection_reader::meta_merkle_tree::generated_merkle_tree::{
    CLAIM_STATUS_SEED, GeneratedMerkleTree, GeneratedMerkleTreeCollection, TreeNode,
};
use solana_program::{hash::Hash, pubkey::Pubkey};

fn temp_path(suffix: &str) -> PathBuf {
    env::temp_dir().join(format!(
        "wincode_roundtrip_{}_{}.wincode",
        process::id(),
        suffix
    ))
}

fn sample_collection() -> GeneratedMerkleTreeCollection {
    let tip_distribution_id =
        Pubkey::from_str_const("4R3gSG8BpU4t19KYj8CfnbtRpnT8gtk4dvTHxVRwc2r7");
    let tda = Pubkey::new_unique();
    let (acct_0, acct_1) = (Pubkey::new_unique(), Pubkey::new_unique());
    let claim_status_0 = Pubkey::find_program_address(
        &[CLAIM_STATUS_SEED, &acct_0.to_bytes(), &tda.to_bytes()],
        &tip_distribution_id,
    );
    let claim_status_1 = Pubkey::find_program_address(
        &[CLAIM_STATUS_SEED, &acct_1.to_bytes(), &tda.to_bytes()],
        &tip_distribution_id,
    );

    let tree_nodes = vec![
        TreeNode {
            claimant: acct_0,
            claim_status_pubkey: claim_status_0.0,
            claim_status_bump: claim_status_0.1,
            staker_pubkey: Pubkey::default(),
            withdrawer_pubkey: Pubkey::default(),
            amount: 151_507,
            proof: None,
        },
        TreeNode {
            claimant: acct_1,
            claim_status_pubkey: claim_status_1.0,
            claim_status_bump: claim_status_1.1,
            staker_pubkey: Pubkey::default(),
            withdrawer_pubkey: Pubkey::default(),
            amount: 176_624,
            proof: Some(vec![[7u8; 32], [9u8; 32]]),
        },
    ];

    GeneratedMerkleTreeCollection {
        generated_merkle_trees: vec![GeneratedMerkleTree {
            distribution_program: tip_distribution_id,
            distribution_account: tda,
            merkle_root_upload_authority: Pubkey::new_unique(),
            merkle_root: Hash::new_from_array([0xab; 32]),
            tree_nodes,
            max_total_claim: 151_507 + 176_624,
            max_num_nodes: 2,
        }],
        bank_hash: "test-bank-hash".to_string(),
        epoch: 42,
        slot: 1_234_567,
    }
}

#[test]
fn wincode_file_roundtrip_preserves_collection() {
    let collection = sample_collection();
    let path = temp_path("file_roundtrip");

    collection
        .write_wincode_to_file(&path)
        .expect("write wincode");
    let decoded =
        GeneratedMerkleTreeCollection::new_from_file_wincode(&path).expect("read wincode");

    let _ = fs::remove_file(&path);

    assert_eq!(collection, decoded);
}
