/// Total XP required to reach the given level (level 1 = 0 XP).
///
/// Uses the classic RuneScape/OSRS curve:
/// `floor(sum(lvl=1..level-1, floor(lvl + 300 * 2^(lvl/7))) / 4)`
pub fn xp_for_level(level: u32) -> u64 {
    if level <= 1 {
        return 0;
    }

    let mut points = 0u64;
    for lvl in 1..level {
        let term = lvl as f64 + 300.0 * 2f64.powf(lvl as f64 / 7.0);
        points += term.floor() as u64;
    }

    points / 4
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

/// XP earned within the current level, XP required to reach the next level, and ratio (0.0–1.0).
pub fn progress_within_level(level: u32, total_xp: u64) -> (u64, u64, f64) {
    let floor = xp_for_level(level);
    let ceiling = xp_for_level(level.saturating_add(1));
    let span = ceiling.saturating_sub(floor).max(1);
    let into = total_xp.saturating_sub(floor);
    let ratio = (into as f64 / span as f64).clamp(0.0, 1.0);
    (into, span, ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_one_requires_zero_xp() {
        assert_eq!(xp_for_level(1), 0);
    }

    #[test]
    fn runescape_curve_matches_known_level_thresholds() {
        assert_eq!(xp_for_level(2), 83);
        assert_eq!(xp_for_level(3), 174);
        assert_eq!(xp_for_level(10), 1_154);
        assert_eq!(xp_for_level(50), 101_333);
        assert_eq!(xp_for_level(99), 13_034_431);
    }

    #[test]
    fn apply_xp_levels_up_at_runescape_threshold() {
        let (level, xp) = apply_xp(1, 0, 83);
        assert_eq!(level, 2);
        assert_eq!(xp, 83);
    }

    #[test]
    fn progress_at_level_one_uses_runescape_span_to_level_two() {
        let (into, span, ratio) = progress_within_level(1, 41);
        assert_eq!(into, 41);
        assert_eq!(span, 83);
        assert!((ratio - (41.0 / 83.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn progress_at_level_two_uses_runescape_span_to_level_three() {
        let level_two_total = xp_for_level(2);
        let (into, span, ratio) = progress_within_level(2, level_two_total + 46);
        assert_eq!(into, 46);
        assert_eq!(span, 91);
        assert!((ratio - (46.0 / 91.0)).abs() < f64::EPSILON);
    }
}
