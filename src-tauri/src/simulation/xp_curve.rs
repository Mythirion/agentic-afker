/// Total XP required to reach the given level (level 1 = 0 XP).
pub fn xp_for_level(level: u32) -> u64 {
    if level <= 1 {
        return 0;
    }

    let n = u64::from(level - 1);
    n * n * 100
}

/// Applies gained XP and returns the resulting level.
pub fn apply_xp(current_level: u32, current_xp: u64, gained_xp: u64) -> (u32, u64) {
    let mut level = current_level;
    let xp = current_xp.saturating_add(gained_xp);

    loop {
        let next_level = level.saturating_add(1);
        let threshold = xp_for_level(next_level);
        if xp < threshold {
            break;
        }
        level = next_level;
    }

    (level, xp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_one_requires_zero_xp() {
        assert_eq!(xp_for_level(1), 0);
    }

    #[test]
    fn level_two_requires_one_hundred_xp() {
        assert_eq!(xp_for_level(2), 100);
    }
}
