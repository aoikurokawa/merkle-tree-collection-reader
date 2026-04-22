use std::{path::PathBuf, str::FromStr, time::Instant};

use anchor_lang::AccountDeserialize;
use clap::{Parser, Subcommand};
use jito_tip_distribution::state::TipDistributionAccount;
use merkle_tree_collection_reader::meta_merkle_tree::generated_merkle_tree::GeneratedMerkleTreeCollection;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    save_path: PathBuf,

    #[arg(long)]
    epoch: u64,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SerdeJson,
    SerdeJsonSlice,
    /// Load the JSON and write a bincode sibling file (one-time conversion).
    Convert,
    Bincode,
    /// Load the JSON and write a wincode sibling file (one-time conversion).
    WincodeConvert,
    /// List validators using old merkle-root-upload-authority config.
    Wincode {
        /// validators.app API token
        #[arg(long, env = "VALIDATORS_APP_TOKEN")]
        api_token: String,
    },
}

pub fn merkle_tree_collection_file_name(epoch: u64) -> String {
    format!("{}_merkle_tree_collection.json", epoch)
}

pub fn merkle_tree_collection_bincode_file_name(epoch: u64) -> String {
    format!("{}_merkle_tree_collection.bin", epoch)
}

pub fn merkle_tree_collection_wincode_file_name(epoch: u64) -> String {
    format!("{}_merkle_tree_collection.wincode", epoch)
}

fn fetch_vote_to_name(
    http: &reqwest::blocking::Client,
    api_token: &str,
) -> std::collections::HashMap<String, String> {
    let resp = http
        .get("https://www.validators.app/api/v1/validators/mainnet.json")
        .header("Token", api_token)
        .send()
        .and_then(|r| r.json::<Vec<serde_json::Value>>())
        .ok()
        .unwrap_or_default();

    resp.into_iter()
        .filter_map(|v| {
            let vote = v["vote_account"].as_str()?.to_string();
            let name = v["name"].as_str().unwrap_or("Unknown").to_string();
            Some((vote, name))
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let rpc_client = RpcClient::new("".to_string());

    let merkle_tree_path = args
        .save_path
        .join(merkle_tree_collection_file_name(args.epoch));

    match args.command {
        Commands::SerdeJson => {
            let _merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_serde_json(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
        }
        Commands::SerdeJsonSlice => {
            let _merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_serde_json_slice(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
        }
        Commands::Convert => {
            let merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_serde_json_slice(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
            let bincode_path = args
                .save_path
                .join(merkle_tree_collection_bincode_file_name(args.epoch));
            merkle_trees
                .write_bincode_to_file(&bincode_path)
                .map_err(|e| anyhow::anyhow!("Failed to write bincode: {e}"))?;
            println!("Wrote bincode to {}", bincode_path.display());
        }
        Commands::Bincode => {
            let bincode_path = args
                .save_path
                .join(merkle_tree_collection_bincode_file_name(args.epoch));
            let _merkle_trees = GeneratedMerkleTreeCollection::new_from_file_bincode(&bincode_path)
                .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
        }
        Commands::WincodeConvert => {
            let merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_serde_json_slice(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
            let wincode_path = args
                .save_path
                .join(merkle_tree_collection_wincode_file_name(args.epoch));
            merkle_trees
                .write_wincode_to_file(&wincode_path)
                .map_err(|e| anyhow::anyhow!("Failed to write wincode: {e}"))?;
            println!("Wrote wincode to {}", wincode_path.display());
        }
        Commands::Wincode { api_token } => {
            let wincode_path = args
                .save_path
                .join(merkle_tree_collection_wincode_file_name(args.epoch));
            let start = Instant::now();
            let merkle_trees = GeneratedMerkleTreeCollection::new_from_file_wincode(&wincode_path)
                .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
            eprintln!("wincode load: {:?}", start.elapsed());

            let http = reqwest::blocking::Client::new();

            eprintln!("Fetching validators from validators.app...");
            let vote_to_name = fetch_vote_to_name(&http, &api_token);
            eprintln!("Loaded {} validators", vote_to_name.len());

            println!("name\tvote_account\tlink");
            let mut count = 0;
            for merkle_tree in merkle_trees.generated_merkle_trees {
                if merkle_tree.merkle_root_upload_authority
                    == Pubkey::from_str("GZctHpWXmsZC1YHACTGGcHhYxjdRqQvTpYkb9LMvxDib").unwrap()
                {
                    let acc_data = rpc_client
                        .get_account(&merkle_tree.distribution_account)
                        .unwrap();
                    let tda =
                        TipDistributionAccount::try_deserialize(&mut acc_data.data.as_slice())
                            .unwrap();

                    let vote_account = tda.validator_vote_account;
                    let name = vote_to_name
                        .get(&vote_account.to_string())
                        .cloned()
                        .unwrap_or_else(|| "Unknown".to_string());
                    let link = format!("https://www.jito.network/validator/{}/", vote_account);
                    println!("{}\t{}\t{}", name, vote_account, link);

                    count += 1;
                }
            }

            eprintln!("Old Merkle Tree Config: {count}");
        }
    }

    Ok(())
}
