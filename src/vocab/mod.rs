//! The wire vocabulary: one Rust type per XML enum, with the XML's codes.
//!
//! Pure (serde only), so the compositor's model, the shell's model, provenance and the agent
//! tier share the protocols' values without linking Wayland. With the `client` and `server`
//! features a test checks every enum here against wayland-scanner's.

mod actor;
mod agent;
mod app;
mod code;
mod held_mods;
mod output;
mod privilege;
mod sensitive;
mod shell;
mod space;
mod trial;
mod trusted;
mod window;

pub use actor::ActorKind;
pub use agent::{GlowCode, LeaseState, MaskReason, PressEdge, RevokeReason, RunMode, Suspend};
pub use app::{AppLifeKind, AppVisibility, Attention, MenuSourceKind, ProofKind};
pub use code::{CodeError, WireCode};
pub use held_mods::HeldMods;
pub use output::Transform;
pub use privilege::{Answer, AskId, Memory, Power, SubjectKind};
pub use sensitive::SensitivityLevel;
pub use shell::{
    Corner, DragOutcome, FullScreenBar, KeyGesture, Motion, OverviewMode, OverviewState,
    ScreenEdge, ShellRole, Shown, SplitSide, WindowEntries, WindowEntry,
};
pub use space::{SpaceActivity, SpaceKind};
pub use trial::{TrialAnswer, TrialId};
pub use trusted::{TrustedAnchor, TrustedKind, Withdrawn};
pub use window::{
    Assign, Edited, Emphasis, FrameMode, Modality, PlaceKind, PlaceTarget, PlacementMark,
    Responsiveness, TileZone, Toggle, WindowRole, WindowStateKind,
};
