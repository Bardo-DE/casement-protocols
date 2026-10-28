//! The one transition.

use crate::capture::reduce_captures;
use crate::event::DesktopEvent;
use crate::output::reduce_outputs;
use crate::quire::{
    Active, Quire, project_toplevels, reduce_app, reduce_note, reduce_space, reduce_window,
    set_active,
};
use crate::state::DesktopState;
use crate::toplevel::{ToplevelEvent, reduce_toplevels};
use crate::workspace::reduce_workspaces;

/// `state` after `event`. Pure and total: an event for something unknown changes nothing.
pub fn reduce(state: DesktopState, event: DesktopEvent) -> DesktopState {
    match event {
        DesktopEvent::Globals(protocols) => DesktopState { protocols, ..state },
        DesktopEvent::Toplevel(e) => toplevel(state, e),
        DesktopEvent::Workspace(e) => DesktopState {
            workspaces: reduce_workspaces(state.workspaces, e),
            ..state
        },
        DesktopEvent::Output(e) => DesktopState {
            outputs: reduce_outputs(state.outputs, e),
            ..state
        },
        DesktopEvent::Capture { id, event } => DesktopState {
            captures: reduce_captures(state.captures, id, event),
            ..state
        },
        DesktopEvent::App(e) => quire(state, |q| reduce_app(q, e)),
        DesktopEvent::Window(e) => quire(state, |q| reduce_window(q, e)),
        DesktopEvent::Space(e) => quire(state, |q| reduce_space(q, e)),
        DesktopEvent::Active { app, key, actor } => {
            quire(state, |q| set_active(q, Active { app, key, actor }))
        }
        DesktopEvent::Shell(e) => quire(state, |q| reduce_note(q, e)),
    }
}

/// A toplevel event; a toplevel that closes takes its Quire window with it.
fn toplevel(state: DesktopState, event: ToplevelEvent) -> DesktopState {
    let mut quire = state.quire;
    if let ToplevelEvent::Closed(key) = &event {
        quire.forget_window(*key);
    }
    DesktopState {
        toplevels: reduce_toplevels(state.toplevels, event),
        quire,
        ..state
    }
}

/// A Quire event, applied by `step`; the windows' state is then projected onto the toplevels.
fn quire(state: DesktopState, step: impl FnOnce(Quire) -> Quire) -> DesktopState {
    let quire = step(state.quire);
    DesktopState {
        toplevels: project_toplevels(state.toplevels, &quire),
        quire,
        ..state
    }
}
