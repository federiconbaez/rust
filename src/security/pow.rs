use sha2::{Sha256, Digest};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;

// In-memory store for active challenges (simple for now, could be Redis)
// Map<ChallengeID, Difficulty>
static ACTIVE_CHALLENGES: Lazy<Arc<Mutex<HashMap<String, u32>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub struct PoWService;

impl PoWService {
    // Generate a new challenge
    pub fn generate_challenge(difficulty: u32) -> String {
        let challenge_id = Uuid::new_v4().to_string();
        
        let mut challenges = ACTIVE_CHALLENGES.lock().unwrap();
        challenges.insert(challenge_id.clone(), difficulty);
        
        // Clean up old challenges occasionally (naive implementation)
        if challenges.len() > 1000 {
            challenges.clear(); 
        }

        challenge_id
    }

    // Verify the solution
    // Client sends: challenge_id, nonce
    // We verify: hash(challenge_id + nonce) starts with '0' * difficulty
    pub fn verify_solution(challenge_id: &str, nonce: &str) -> bool {
        let difficulty = {
            let mut challenges = ACTIVE_CHALLENGES.lock().unwrap();
            match challenges.remove(challenge_id) {
                Some(d) => d,
                None => return false, // Challenge not found or expired
            }
        };

        let input = format!("{}{}", challenge_id, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let hex_hash = hex::encode(result);

        let prefix = "0".repeat(difficulty as usize);
        hex_hash.starts_with(&prefix)
    }
}
