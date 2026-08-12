use lazy_static::lazy_static;

use crate::bit_flag_line_transformer::BitFlagLineTransformer;
use crate::line_transformer::LineTransformer;

pub struct HitboxStateTransformer {
    transformer: BitFlagLineTransformer<u16>,
}

lazy_static! {
    static ref HITBOX_STATES: [(u16, &'static str); 4] = [
        (1 << 7, "HITBOX_INVULNERABLE"), // 0x80
        (1 << 2, "HITBOX_WEAPON_HIT"),   // 0x04
        (1 << 1, "HITBOX_SOLID"),         // 0x02
        (1 << 0, "HITBOX_ACTIVE"),        // 0x01
    ];
}

impl HitboxStateTransformer {
    pub fn new() -> Self {
        Self {
            transformer: BitFlagLineTransformer::<u16>::new(
                "hitboxState",
                "HITBOX_INACTIVE",
                &HITBOX_STATES.iter().collect(),
            ),
        }
    }
}

impl LineTransformer for HitboxStateTransformer {
    fn transform_line(&self, line: &str) -> String {
        self.transformer.transform_line(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static HT: Lazy<HitboxStateTransformer> = Lazy::new(|| HitboxStateTransformer::new());

    #[test]
    fn test_inactive_zero() {
        let input = "self->hitboxState = 0;";
        let expected = "self->hitboxState = HITBOX_INACTIVE;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_active() {
        let input = "self->hitboxState = 1;";
        let expected = "self->hitboxState = HITBOX_ACTIVE;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_solid() {
        let input = "self->hitboxState = 2;";
        let expected = "self->hitboxState = HITBOX_SOLID;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_weapon_hit() {
        let input = "self->hitboxState = 4;";
        let expected = "self->hitboxState = HITBOX_WEAPON_HIT;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_invulnerable_hex() {
        let input = "self->hitboxState = 0x80;";
        let expected = "self->hitboxState = HITBOX_INVULNERABLE;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_invulnerable_decimal() {
        let input = "self->hitboxState = 128;";
        let expected = "self->hitboxState = HITBOX_INVULNERABLE;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_combined_invulnerable_solid() {
        // 0x82 = HITBOX_INVULNERABLE | HITBOX_SOLID
        let input = "self->hitboxState = 0x82;";
        let expected = "self->hitboxState = HITBOX_INVULNERABLE | HITBOX_SOLID;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_already_named_unchanged() {
        let input = "self->hitboxState = HITBOX_WEAPON_HIT;";
        let expected = "self->hitboxState = HITBOX_WEAPON_HIT;";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_equality_check() {
        let input = "if (other->hitboxState == 0) {";
        let expected = "if (other->hitboxState == HITBOX_INACTIVE) {";
        assert_eq!(HT.transform_line(input), expected);
    }

    #[test]
    fn test_player_hitbox_state() {
        let input = "PLAYER.hitboxState = 0;";
        let expected = "PLAYER.hitboxState = HITBOX_INACTIVE;";
        assert_eq!(HT.transform_line(input), expected);
    }
}
