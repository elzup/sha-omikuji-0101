/// Bar-chart style art using 256 bits of SHA-256 hash
/// Each bar height represents visit count from a random walk

pub fn generate_omikuji_art(hash_bytes: &[u8; 32]) -> String {
    const WIDTH: usize = 32;
    let mut grid = [0u8; WIDTH];

    // Use random walk to distribute visits
    let mut position: usize = WIDTH / 2;

    // Process all 256 bits as 2-bit pairs (128 steps)
    for byte in hash_bytes.iter() {
        for shift in (0..8).step_by(2) {
            let bits = (byte >> (6 - shift)) & 0b11;

            // Move left or right with wrap-around
            let movement: i32 = match bits {
                0b00 => -1, // left
                0b01 => 0,  // stay
                0b10 => 1,  // right
                0b11 => 2,  // right x2
                _ => unreachable!(),
            };

            position = ((position as i32 + movement).rem_euclid(WIDTH as i32)) as usize;
            grid[position] = grid[position].saturating_add(1);
        }
    }

    // Bar characters: 8 levels of height
    // ▁▂▃▄▅▆▇█
    const BARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    // Find max for normalization
    let max_count = *grid.iter().max().unwrap_or(&1).max(&1);

    let mut art = String::with_capacity(WIDTH);
    for &count in grid.iter() {
        // Normalize to 0-8 range
        let level = if count == 0 {
            0
        } else {
            ((count as usize * 8) / max_count as usize).clamp(1, 8)
        };
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
        assert_eq!(art.chars().count(), 32, "Should have 32 bar characters");
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
        let valid_chars: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        for ch in art.chars() {
            assert!(
                valid_chars.contains(&ch),
                "Invalid character in art: '{}' (code: {})",
                ch,
                ch as u32
            );
        }
    }

    #[test]
    fn test_art_many_seeds() {
        for i in 0..100 {
            let seed = format!("test-seed-{}", i);
            let hash = make_hash(seed.as_bytes());
            let art = generate_omikuji_art(&hash);
            assert_eq!(art.chars().count(), 32, "Seed {} should produce 32 chars", seed);
        }
    }

    #[test]
    fn test_art_variety() {
        let mut unique_arts = std::collections::HashSet::new();
        for i in 0..20 {
            let seed = format!("variety-test-{}", i);
            let hash = make_hash(seed.as_bytes());
            let art = generate_omikuji_art(&hash);
            unique_arts.insert(art);
        }
        assert!(
            unique_arts.len() >= 15,
            "Should have at least 15 unique patterns out of 20, got {}",
            unique_arts.len()
        );
    }

    #[test]
    fn test_all_zeros_concentrated() {
        // All zeros = all left moves, concentrated at one position
        let hash = [0u8; 32];
        let art = generate_omikuji_art(&hash);
        // Should have at least one max bar (█)
        assert!(art.contains('█'), "All zeros should produce concentrated visits");
    }
}
