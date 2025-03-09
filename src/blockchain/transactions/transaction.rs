use std::ptr::read;
use chrono::Utc;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use hex;
use secp256k1::ecdsa::Signature;
use sha2::{Digest, Sha256};
use uuid::Uuid;

const BASE_FEE: f64 = 0.1;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoteTransaction {
    pub transaction_id: String,
    pub voter_public_key: String,
    pub vote_data: String,
    pub timestamp: String,
    pub election_id: String,
}
#[derive(Debug, Clone)]
pub struct SignedTransaction {
    pub(crate) transaction: VoteTransaction,
    pub(crate) signature: Vec<u8>
}

impl SignedTransaction {
    pub fn is_valid(&self, public_key: &PublicKey) -> bool {
        VoteTransaction::verify_transaction(public_key, &self.transaction, &self.signature)
    }
}

impl VoteTransaction {

    pub fn new(voter_public_key: String, vote_data: String, election_id: String) -> Self {
        VoteTransaction {
            transaction_id: Uuid::new_v4().to_string(),
            voter_public_key,
            vote_data,
            election_id,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn sing_transaction(transaction: &VoteTransaction, private_key: &str) -> Result<SignedTransaction, String> {
        let secp: Secp256k1<All> = Secp256k1::new();

        let private_key: SecretKey = VoteTransaction::get_private_key(&private_key)?;

        let transaction_hash = Self::hash_transaction(transaction);

        let message = Self::get_message(&transaction_hash).expect("Failed to get the message");

        let signature = secp.sign_ecdsa(&message, &private_key);

        Ok(SignedTransaction {
            transaction: transaction.clone(),
            signature: signature.serialize_compact().to_vec()
        })
    }

    pub fn verify_transaction(public_key: &PublicKey, transaction: &VoteTransaction, signature: &[u8]) -> bool {
        let secp: Secp256k1<All> = Secp256k1::new();
        let transaction_hash = Self::hash_transaction(transaction);
        let message_hash = Self::get_message(&transaction_hash).expect("Failed to get message");

        let parsed_signature = match Signature::from_compact(signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        secp.verify_ecdsa(&message_hash, &parsed_signature, public_key).is_ok()
    }

    fn parse_signature(signature: &[u8]) -> Result<Signature, String> {
        match Signature::from_compact(signature) {
            Ok(sig) => Ok(sig),
            Err(_) => Err("Failed to parse signature".to_string()),
        }
    }
    fn get_message(transaction_hash: &[u8]) -> Result<Message, String> {
        Message::from_slice(transaction_hash).map_err(|_| "Invalid message length".to_string())
    }

    // Decode hex string into bytes and convert them into a SecretKey
    fn get_private_key(private_key: &str) -> Result<SecretKey, String> {
        hex::decode(private_key)
            .map_err(|_| "Failed to decode private key".to_string())
            .and_then(|bytes| SecretKey::from_slice(&bytes).map_err(|_| "Invalid private key".to_string()))
    }

    fn hash_transaction(tx: &VoteTransaction) -> [u8; 32] {
        let serialized_tx: Vec<u8> = serde_json::to_vec(tx).expect("Failed to serialize transaction");
        let hash = Sha256::digest(&serialized_tx);
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash);
        result
    }
}

