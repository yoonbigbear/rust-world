use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Sha256, Digest};

/// 서버 측: 시드 생성
/// 실제 서버에서는 thread_rng().next_u64() 등을 이용해 랜덤 시드를 만들 수 있다.
pub fn server_generate_seed() -> u64 {
    use rand::Rng;
    rand::thread_rng().gen()
}

/// 서버 측: 시드에 대한 커밋(commit) 생성
/// 커밋은 시드의 해시 값이다. 클라이언트에게 이 해시만 보내면 클라이언트는 시드를 알 수 없다.
pub fn commit_seed(seed: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(seed.to_le_bytes());
    let result = hasher.finalize();

    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&result);
    hash_bytes
}

/// 클라이언트 측: 서버로부터 커밋 해시를 받는다고 가정
/// 여기서는 함수 인자로 받지만 실제로는 네트워크 통해 수신
pub fn client_receive_commit(commit: [u8; 32]) -> [u8; 32] {
    commit
}

/// 서버 측: 나중에 시드 공개 (reveal)
/// 클라이언트는 이 시드로 해시를 다시 계산하여 commit과 동일한지 검증 가능
pub fn server_reveal_seed(seed: u64) -> u64 {
    seed
}

/// 클라이언트 측: 서버가 공개한 시드를 검증
pub fn client_verify_and_set_seed(commit: [u8; 32], revealed_seed: u64) -> Result<ChaCha20Rng, &'static str> {
    let verify_commit = commit_seed(revealed_seed);
    if verify_commit == commit {
        // 검증 성공 시 PRNG 초기화
        let rng = ChaCha20Rng::seed_from_u64(revealed_seed);
        Ok(rng)
    } else {
        Err("Commit hash does not match the revealed seed!")
    }
}

/// 난수열 생성 함수
pub fn generate_random_sequence(rng: &mut ChaCha20Rng, count: usize) -> Vec<u64> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(rng.next_u64());
    }
    result
}


/// 서버에서 난수열을 생성하는 로직(클라이언트 검증 전이라면 서버만 알고 있음)
pub fn server_generate_sequence_with_seed(seed: u64, count: usize) -> Vec<u64> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    generate_random_sequence(&mut rng, count)
}

/// 클라이언트 측: 시드 검증 후 동일한 난수열 재생성
pub fn client_generate_sequence_with_seed(rng: &mut ChaCha20Rng, count: usize) -> Vec<u64> {
    generate_random_sequence(rng, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_reveal() {
        // 서버: 시드 생성
        let server_seed = server_generate_seed();

        // 서버: 시드 커밋(해시) 생성 -> 클라이언트에게 전송했다고 가정
        let commit = commit_seed(server_seed);

        // 클라이언트: 커밋 수신 (아직 시드 모름)
        let client_commit = client_receive_commit(commit);

        // 여기서 서버와 클라이언트는 일부 게임 플레이나 사용자의 액션을 진행
        // 클라이언트는 시드를 모르는 상태이므로 난수 예측 어려움.

        // 나중에 서버가 결과를 검증해야 할 때 시드 공개
        let revealed_seed = server_reveal_seed(server_seed);

        // 클라이언트: 시드 검증
        let mut client_rng = client_verify_and_set_seed(client_commit, revealed_seed)
            .expect("Failed to verify seed");

        // 이제 클라이언트와 서버 모두 동일한 시드로 동일한 난수열 생성 가능
        let server_seq = server_generate_sequence_with_seed(server_seed, 5);
        let client_seq = client_generate_sequence_with_seed(&mut client_rng, 5);

        assert_eq!(server_seq, client_seq, "난수열이 일치하지 않습니다.");
    }

    #[test]
    fn test_wrong_reveal() {
        // 서버: 시드 생성
        let server_seed = server_generate_seed();
        let wrong_seed = server_seed.wrapping_add(1);

        // 서버: commit
        let commit = commit_seed(server_seed);

        // 클라이언트: commit 수신
        let client_commit = client_receive_commit(commit);

        // 서버가 엉뚱한 시드(cheating) 공개
        let revealed_seed = server_reveal_seed(wrong_seed);

        // 클라이언트: 검증 실패해야 함
        let result = client_verify_and_set_seed(client_commit, revealed_seed);
        assert!(result.is_err(), "잘못된 시드로 검증에 성공하면 안 됩니다.");
    }
}