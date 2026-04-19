use std::path::PathBuf;

use clap::{Parser, Subcommand};
use merkle_tree_collection_reader::meta_merkle_tree::generated_merkle_tree::GeneratedMerkleTreeCollection;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long)]
    save_path: PathBuf,

    /// Number of times to greet
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
}

pub fn merkle_tree_collection_file_name(epoch: u64) -> String {
    format!("{}_merkle_tree_collection.json", epoch)
}

pub fn merkle_tree_collection_bincode_file_name(epoch: u64) -> String {
    format!("{}_merkle_tree_collection.bin", epoch)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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
    }

    Ok(())
}
