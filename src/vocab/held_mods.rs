//! The modifier keys the person holds, as pressed (`quire_shell_v1.mac_mods`, a bitfield; the XML name is the wire name and stays).

use serde::{Deserialize, Serialize};

/// A set of modifier keys, as pressed and before any per-app remapping: Command is the
/// Command key whatever the focused app sees.
///
/// The bits are `quire_shell_v1.mac_mods`'s values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeldMods(pub u32);

impl HeldMods {
    /// No modifier.
    pub const NONE: HeldMods = HeldMods(0);
    /// Shift.
    pub const SHIFT: HeldMods = HeldMods(1);
    /// Control.
    pub const CONTROL: HeldMods = HeldMods(2);
    /// Option.
    pub const OPTION: HeldMods = HeldMods(4);
    /// Command.
    pub const COMMAND: HeldMods = HeldMods(8);
    /// Caps Lock is on.
    pub const CAPS_LOCK: HeldMods = HeldMods(16);
    /// Fn.
    pub const FUNCTION: HeldMods = HeldMods(32);

    /// Whether every key of `required` is held (any others may be held too).
    pub fn holds(self, required: HeldMods) -> bool {
        self.0 & required.0 == required.0
    }

    /// This set with `other`'s keys added.
    pub fn with(self, other: HeldMods) -> HeldMods {
        HeldMods(self.0 | other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::HeldMods;

    /// (case, held, required, holds)
    const HOLDS: &[(&str, HeldMods, HeldMods, bool)] = &[
        (
            "nothing required is always held",
            HeldMods::NONE,
            HeldMods::NONE,
            true,
        ),
        ("the exact key", HeldMods::COMMAND, HeldMods::COMMAND, true),
        ("a missing key", HeldMods::SHIFT, HeldMods::COMMAND, false),
        (
            "extra keys do not matter",
            HeldMods(8 | 1),
            HeldMods::COMMAND,
            true,
        ),
        (
            "every key of a chord",
            HeldMods(8 | 1),
            HeldMods(8 | 1),
            true,
        ),
        (
            "half a chord is not enough",
            HeldMods::COMMAND,
            HeldMods(8 | 1),
            false,
        ),
    ];

    #[test]
    fn holds_needs_every_required_key() {
        for (case, held, required, want) in HOLDS {
            assert_eq!(held.holds(*required), *want, "{case}");
        }
    }

    #[test]
    fn with_adds_keys_and_keeps_the_ones_held() {
        let chord = HeldMods::COMMAND.with(HeldMods::SHIFT);
        assert_eq!(chord, HeldMods(8 | 1));
        assert_eq!(chord.with(HeldMods::SHIFT), chord);
        assert_eq!(HeldMods::NONE.with(HeldMods::OPTION), HeldMods::OPTION);
    }

    #[test]
    fn every_key_has_its_own_bit() {
        let keys = [
            HeldMods::SHIFT,
            HeldMods::CONTROL,
            HeldMods::OPTION,
            HeldMods::COMMAND,
            HeldMods::CAPS_LOCK,
            HeldMods::FUNCTION,
        ];
        for key in keys {
            assert_eq!(key.0.count_ones(), 1, "{key:?}");
        }
        let all = keys.iter().fold(HeldMods::NONE, |set, key| set.with(*key));
        assert_eq!(all.0.count_ones(), 6);
    }
}
