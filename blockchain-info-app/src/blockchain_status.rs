// struct fopr marshaling the json response
struct fror marshaling json
pub struct BlockchainStatus {
	pub blockbook: Blockbook,
	pub backend: Backend,
}

pub struct Backend {
    pub chain: String,
     blocks: i64,
     headers: i64,
     best_block_hash: String,
     difficulty: String,
     size_on_disk: i64,
     version: String,
     subversion: String,
     protocol_version: String
}