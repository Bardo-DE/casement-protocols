//! [`ShellVerb`], its command line and the way back.

/// Writes [`ShellVerb`], [`ALL`] and `argv` from one table, so a verb is stated once: its
/// meaning, its name and the words that spell it.
macro_rules! shell_verbs {
    ($( $(#[$doc:meta])* $variant:ident => [$($word:literal),+]; )+) => {
        /// One thing the compositor asks the shell to do.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ShellVerb {
            $( $(#[$doc])* $variant, )+
        }

        /// Every verb, in the order the table below names them.
        pub const ALL: &[ShellVerb] = &[$(ShellVerb::$variant),+];

        impl ShellVerb {
            /// The words after the shell's program name that spell this verb on its command line.
            pub fn argv(self) -> &'static [&'static str] {
                match self {
                    $( Self::$variant => &[$($word),+], )+
                }
            }
        }
    };
}

shell_verbs! {
    /// Lock the screen.
    Lock => ["lock"];
    /// Raise the front app's next window.
    NextWindow => ["next-window"];
    /// Flip the dock's auto-hide.
    DockToggleHide => ["dock", "toggle-hide"];
    /// Show the launcher, or hide it when it is up.
    LauncherToggle => ["launcher", "toggle"];
    /// Show the power menu. Open, not toggle: like the Mac it closes by Escape or a button.
    PowerMenuOpen => ["power-menu", "open"];
    /// Show the Force Quit panel, or hide it when it is up.
    ForceQuitToggle => ["force-quit", "toggle"];
    /// Capture the whole screen.
    ScreenshotScreen => ["screenshot", "screen"];
    /// Capture a chosen area.
    ScreenshotArea => ["screenshot", "area"];
    /// Capture a chosen window.
    ScreenshotWindow => ["screenshot", "window"];
    /// The app switcher steps to the next app.
    SwitcherNext => ["switcher", "next"];
    /// The app switcher steps to the previous app.
    SwitcherPrev => ["switcher", "prev"];
    /// The app switcher picks the app it is on.
    SwitcherCommit => ["switcher", "commit"];
    /// The app switcher closes without picking.
    SwitcherCancel => ["switcher", "cancel"];
    /// Show the control centre, or hide it when it is up.
    ControlCenterToggle => ["control-center", "toggle"];
    /// Show the notification centre, or hide it when it is up.
    NotificationCenterToggle => ["notifications", "toggle"];
    /// Open the launcher on emoji and symbols (the Mac's Character Viewer).
    EmojiAndSymbols => ["do", "emoji-and-symbols"];
    /// Switch to the next input source (keyboard layout or input method).
    NextInputSource => ["do", "next-input-source"];
}

impl ShellVerb {
    /// The verb `args` spell, the exact inverse of [`argv`](Self::argv): any other words, or
    /// extra words after a verb's own, are `None`.
    pub fn parse(args: &[&str]) -> Option<Self> {
        ALL.iter().copied().find(|verb| verb.argv() == args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_verb_round_trips_through_its_argv() {
        for verb in ALL {
            assert_eq!(ShellVerb::parse(verb.argv()), Some(*verb), "{verb:?}");
        }
    }

    #[test]
    fn no_two_verbs_share_an_argv() {
        let mut lines: Vec<&[&str]> = ALL.iter().map(|verb| verb.argv()).collect();
        lines.sort();
        let before = lines.len();
        lines.dedup();
        assert_eq!(lines.len(), before);
    }

    #[test]
    fn unknown_missing_or_extra_words_are_no_verb() {
        let cases: &[&[&str]] = &[
            &[],
            &["dock"],
            &["launcher", "open"],
            &["lock", "now"],
            &["screenshot", "screen", "extra"],
            &["Lock"],
        ];
        for args in cases {
            assert_eq!(ShellVerb::parse(args), None, "{args:?}");
        }
    }
}
