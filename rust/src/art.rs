/// Bar-chart art directly mapping SHA-256 bytes to bar heights

pub fn generate_omikuji_art(hash_bytes: &[u8; 32]) -> String {
    // Bar characters: 8 levels of height (▁▂▃▄▅▆▇█)
    const BARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let mut art = String::with_capacity(16);

    // Use first 16 bytes, map each byte (0-255) to bar height (0-7)
    for &byte in hash_bytes.iter().take(16) {
        let level = (byte / 32) as usize; // 256/8 = 32
        art.push(BARS[level]);
    }

    art
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    fn make_hash(seed: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(seed);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    #[test]
    fn test_art_length() {
        let hash = make_hash(b"test");
        let art = generate_omikuji_art(&hash);
        assert_eq!(art.chars().count(), 16);
    }

    #[test]
    fn test_art_deterministic() {
        let hash = make_hash(b"test-seed");
        let art1 = generate_omikuji_art(&hash);
        let art2 = generate_omikuji_art(&hash);
        assert_eq!(art1, art2);
    }

    #[test]
    fn test_different_hashes_different_art() {
        let hash1 = make_hash(b"alice");
        let hash2 = make_hash(b"bob");
        let art1 = generate_omikuji_art(&hash1);
        let art2 = generate_omikuji_art(&hash2);
        assert_ne!(art1, art2);
    }

    #[test]
    fn test_art_contains_only_bar_chars() {
        let hash = make_hash(b"random-test-seed");
        let art = generate_omikuji_art(&hash);
        let valid_chars: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        for ch in art.chars() {
            assert!(valid_chars.contains(&ch), "Invalid char: {}", ch);
        }
    }

    #[test]
    fn test_all_zeros_lowest_bars() {
        let hash = [0u8; 32];
        let art = generate_omikuji_art(&hash);
        assert!(art.chars().all(|c| c == '▁'), "All zeros should produce lowest bars");
    }

    #[test]
    fn test_all_max_highest_bars() {
        let hash = [255u8; 32];
        let art = generate_omikuji_art(&hash);
        assert!(art.chars().all(|c| c == '█'), "All 255s should produce highest bars");
    }
}
