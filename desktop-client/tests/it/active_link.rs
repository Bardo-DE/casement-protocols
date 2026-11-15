//! The manager's `active` event through the linker: a key window that links after `active`
//! still becomes the key window.

use casement_protocols::vocab::ActorKind;
use desktop_client::quire::{Linker, WindowEvent, WindowObject};
use desktop_client::toplevel::{ToplevelChange, ToplevelEvent};
use desktop_client::{DesktopEvent, ToplevelIdentifier};

use crate::support::{ak, run, tk, top, top_change};

fn obj(n: u64) -> WindowObject {
    WindowObject::new(n)
}

fn id(text: &str) -> ToplevelIdentifier {
    ToplevelIdentifier::new(text)
}

fn active(app: u64, key: Option<u64>) -> DesktopEvent {
    DesktopEvent::Active {
        app: Some(ak(app)),
        key: key.map(tk),
        actor: ActorKind::User,
    }
}

/// The ext handle for `n`, announced and done.
fn handle(n: u64) -> Vec<DesktopEvent> {
    vec![
        top(ToplevelEvent::Opened(tk(n))),
        top_change(tk(n), ToplevelChange::Title("T".into())),
        top(ToplevelEvent::Done(tk(n))),
    ]
}

#[test]
fn a_linked_key_window_is_named_at_once() {
    let mut linker = Linker::default();
    linker.listed(tk(1), id("a"));
    linker.identified(obj(9), id("a"));
    assert_eq!(
        linker.active(Some(ak(1)), Some(obj(9)), ActorKind::User),
        vec![active(1, Some(1))]
    );
    assert!(linker.listed(tk(2), id("b")).is_empty(), "nothing waits");
}

#[test]
fn a_key_window_that_links_later_is_named_when_it_links() {
    let mut linker = Linker::default();
    linker.identified(obj(9), id("a"));
    assert_eq!(
        linker.active(Some(ak(1)), Some(obj(9)), ActorKind::User),
        vec![active(1, None)],
        "no key window yet"
    );
    assert_eq!(
        linker.listed(tk(1), id("a")),
        vec![
            DesktopEvent::Window(WindowEvent::Opened(tk(1))),
            active(1, Some(1)),
        ]
    );
}

#[test]
fn a_later_active_replaces_the_one_that_waits() {
    let mut linker = Linker::default();
    linker.identified(obj(9), id("a"));
    linker.active(Some(ak(1)), Some(obj(9)), ActorKind::User);
    linker.active(Some(ak(2)), None, ActorKind::User);
    assert_eq!(
        linker.listed(tk(1), id("a")),
        vec![DesktopEvent::Window(WindowEvent::Opened(tk(1)))]
    );
}

#[test]
fn a_waiting_key_window_that_closes_is_forgotten() {
    let mut linker = Linker::default();
    linker.identified(obj(9), id("a"));
    linker.active(Some(ak(1)), Some(obj(9)), ActorKind::User);
    linker.closed(obj(9));
    linker.identified(obj(9), id("a"));
    assert_eq!(
        linker.listed(tk(1), id("a")),
        vec![DesktopEvent::Window(WindowEvent::Opened(tk(1)))]
    );
}

#[test]
fn the_late_key_window_is_activated_on_its_toplevel() {
    let mut linker = Linker::default();
    let mut events = Vec::new();
    events.extend(linker.identified(obj(9), id("a")));
    events.extend(linker.done(obj(9)));
    events.extend(linker.active(Some(ak(1)), Some(obj(9)), ActorKind::User));
    events.extend(handle(1));
    events.extend(linker.listed(tk(1), id("a")));
    let state = run(events);
    assert_eq!(state.toplevels.activated().map(|t| t.key), Some(tk(1)));
}
