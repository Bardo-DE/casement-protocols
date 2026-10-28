//! Everything the core reduces.

use crate::capture::CaptureEvent;
use crate::keys::{AppKey, CaptureId, ToplevelKey};
use crate::output::OutputEvent;
use crate::protocol::ProtocolSet;
use crate::quire::{AppEvent, ShellEvent, SpaceEvent, WindowEvent};
use crate::toplevel::ToplevelEvent;
use crate::workspace::WorkspaceEvent;
use casement_protocols::vocab::ActorKind;

/// One thing the compositor said, in the core's words.
///
/// The last five are the Quire arm (`quire_desktop_v1` and `quire_shell_v1`); `Window` is a
/// Quire window, keyed by its ext handle's key, where `Toplevel` is the ext or wlr handle.
///
/// ```
/// use desktop_client::quire::{AppChange, AppEvent};
/// use desktop_client::{AppKey, DesktopEvent, DesktopState, reduce};
/// use casement_protocols::vocab::AppLifeKind;
///
/// let app = AppKey::new(7);
/// let events = [
///     AppEvent::Opened(app),
///     AppEvent::Changed { key: app, change: AppChange::Life(AppLifeKind::Running) },
///     AppEvent::Done(app),
/// ];
/// let state = events
///     .into_iter()
///     .fold(DesktopState::default(), |s, e| reduce(s, DesktopEvent::App(e)));
/// assert_eq!(state.quire.apps().next().unwrap().life, AppLifeKind::Running);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DesktopEvent {
    /// The registry's offer is known (once, after the first round trip).
    Globals(ProtocolSet),
    /// A toplevel event.
    Toplevel(ToplevelEvent),
    /// A workspace event.
    Workspace(WorkspaceEvent),
    /// An output event.
    Output(OutputEvent),
    /// An event of one capture.
    Capture {
        /// Which capture.
        id: CaptureId,
        /// What happened.
        event: CaptureEvent,
    },
    /// A Quire app event.
    App(AppEvent),
    /// A Quire window event (the window is keyed by its ext handle's key).
    Window(WindowEvent),
    /// A Quire Space event.
    Space(SpaceEvent),
    /// The active app and key window changed (`quire_desktop_manager_v1.active`).
    Active {
        /// The active app; `None` when no app is active.
        app: Option<AppKey>,
        /// The key window; `None` when the active app has none.
        key: Option<ToplevelKey>,
        /// Who caused it.
        actor: ActorKind,
    },
    /// A note from `quire_shell_v1`.
    Shell(ShellEvent),
}
