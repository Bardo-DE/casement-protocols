//! The Quire transition.

use super::app::AppEvent;
use super::lifecycle::reduce_entity;
use super::set::{Active, Quire};
use super::shell::{ShellEvent, reduce_shell};
use super::space::SpaceEvent;
use super::window::{SpacePlace, WindowEvent};

/// `quire` after an app event. Pure and total: a change to something unknown is ignored, and a
/// `Done` marks its object announced.
pub(crate) fn reduce_app(mut quire: Quire, event: AppEvent) -> Quire {
    if let Some(key) = reduce_entity(&mut quire.apps, event) {
        if quire.active.app == Some(key) {
            quire.active.app = None;
        }
        for w in quire.windows.values_mut() {
            if w.app == Some(key) {
                w.app = None;
            }
        }
    }
    quire
}

/// `quire` after a window event (a window is created by its `Opened`).
pub(crate) fn reduce_window(mut quire: Quire, event: WindowEvent) -> Quire {
    if let Some(key) = reduce_entity(&mut quire.windows, event) {
        quire.forget_window(key);
    }
    quire
}

/// `quire` after a Space event.
pub(crate) fn reduce_space(mut quire: Quire, event: SpaceEvent) -> Quire {
    if let Some(key) = reduce_entity(&mut quire.spaces, event) {
        for w in quire.windows.values_mut() {
            if w.place == SpacePlace::On(key) {
                w.place = SpacePlace::Unknown;
            }
        }
    }
    quire
}

/// `quire` with its active app and key window replaced.
pub(crate) fn set_active(mut quire: Quire, active: Active) -> Quire {
    quire.active = active;
    quire
}

/// `quire` after a note from the shell.
pub(crate) fn reduce_note(mut quire: Quire, event: ShellEvent) -> Quire {
    quire.shell = reduce_shell(quire.shell, event);
    quire
}
