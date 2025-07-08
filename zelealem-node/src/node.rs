use crate::chain::Chain;
use crate::ledger::Block;
use crate::state_db::StateDB;
use crate::validator::{TransactionValidator, ValidationError};
use thiserror::Error;

use crate::p2p::ZelealemBehaviour;
use libp2p::{
    gossipsub, identity, mdns, noise, tcp, yamux, PeerId, Swarm, SwarmBuilder,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tokio::runtime::Runtime; // CORRECTED: Import was missing.
use crate::consensus::ValidatorSet; // CORRECTED: Removed unused `Validator` import.

#[derive(Error, Debug)]
pub enum ProcessBlockError {
    #[error("Block's previous_hash does not match the latest block in the chain")]
    MismatchedPreviousHash,
    #[error("Transaction validation failed: {0}")]
    TransactionError(#[from] ValidationError),
}

pub struct Node {
    pub chain: Chain,
    pub state_db: StateDB,
    pub swarm: Swarm<ZelealemBehaviour>,
    pub runtime: Runtime,
    pub validator_set: ValidatorSet,
}

impl Node {
    pub async fn new() -> Self {
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        println!("Local peer ID: {}", peer_id);

        let behaviour = {
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .message_id_fn(message_id_fn)
                .build()
                .expect("Valid gossipsub config");
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(id_keys.clone()),
                gossipsub_config,
            )
            .expect("Correct gossipsub");

            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id).unwrap();

            ZelealemBehaviour { gossipsub, mdns }
        };
        
        // CORRECTED: The original `runtime` variable was out of scope.
        // We create it here and use it directly.
        let runtime = Runtime::new().unwrap();
        let swarm = {
            runtime.block_on(async {
                SwarmBuilder::with_existing_identity(id_keys)
                    .with_tokio()
                    .with_tcp(
                        tcp::Config::default(),
                        noise::Config::new,
                        yamux::Config::default,
                    )
                    .unwrap()
                    .with_behaviour(|_| behaviour)
                    .unwrap()
                    .with_swarm_config(|c| c.with_idle_connection_timeout(std::time::Duration::from_secs(60)))
                    .build()
            })
        };

        Self {
            chain: Chain::new(),
            state_db: StateDB::new(),
            swarm,
            runtime,
            validator_set: ValidatorSet::new(),
        }
    }
    
    pub fn process_block(&mut self, block: Block) -> Result<(), ProcessBlockError> {
        // ... function body is unchanged ...
        let latest_hash = self.chain.get_latest_hash();
        if block.previous_hash != latest_hash {
            return Err(ProcessBlockError::MismatchedPreviousHash);
        }

        let validator = TransactionValidator::new(&self.state_db);
        for tx in &block.transactions {
            validator.validate_transaction(tx)?;
        }

        for tx in &block.transactions {
            for input_id in &tx.inputs {
                self.state_db.remove_so(input_id).unwrap();
            }
            for output_so in &tx.outputs {
                self.state_db.add_so(output_so.clone()).unwrap();
            }
        }

        self.chain.add_block(block);
        Ok(())
    }
}