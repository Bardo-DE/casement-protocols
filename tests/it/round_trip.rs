//! The serde forms of the vocab: each value's JSON is its XML entry name, and every stored or
//! wire type round-trips.

use std::fmt::Debug;

use casement_protocols::vocab::{
    ActorKind, Answer, AppLifeKind, AppVisibility, Assign, Attention, CodeError, Corner,
    DragOutcome, Edited, Emphasis, FrameMode, GlowCode, HeldMods, KeyGesture, LeaseState,
    MaskReason, Memory, MenuSourceKind, Modality, OverviewMode, OverviewState, PlaceKind,
    PlaceTarget, PlacementMark, Power, PressEdge, ProofKind, Responsiveness, RevokeReason, RunMode,
    ScreenEdge, SensitivityLevel, ShellRole, Shown, SpaceActivity, SpaceKind, SplitSide,
    SubjectKind, Suspend, TileZone, Toggle, Transform, TrialAnswer, TrustedAnchor, TrustedKind,
    WindowEntry, WindowRole, WindowStateKind, WireCode, Withdrawn,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

fn round_trips<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) -> String {
    let json = serde_json::to_string(value).expect("serializes");
    let back: T = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(&back, value, "{json}");
    json
}

/// Every value of `T` round-trips through serde and through its code, and an unknown code
/// names `T`.
fn every_value_round_trips<T>(name: &'static str)
where
    T: WireCode + Serialize + DeserializeOwned + PartialEq + Debug,
{
    for value in T::ALL {
        round_trips(value);
        assert_eq!(T::from_code(value.code()), Ok(*value), "{name}");
    }
    let past = u32::try_from(T::ALL.len()).expect("small enum");
    assert_eq!(
        T::from_code(past),
        Err(CodeError::Unknown {
            what: name,
            code: past
        })
    );
}

#[test]
fn every_vocab_enum_round_trips() {
    every_value_round_trips::<WindowRole>("WindowRole");
    every_value_round_trips::<Modality>("Modality");
    every_value_round_trips::<WindowStateKind>("WindowStateKind");
    every_value_round_trips::<TileZone>("TileZone");
    every_value_round_trips::<PlaceTarget>("PlaceTarget");
    every_value_round_trips::<PlacementMark>("PlacementMark");
    every_value_round_trips::<Emphasis>("Emphasis");
    every_value_round_trips::<Responsiveness>("Responsiveness");
    every_value_round_trips::<FrameMode>("FrameMode");
    every_value_round_trips::<Edited>("Edited");
    every_value_round_trips::<Toggle>("Toggle");
    every_value_round_trips::<Assign>("Assign");
    every_value_round_trips::<PlaceKind>("PlaceKind");
    every_value_round_trips::<AppLifeKind>("AppLifeKind");
    every_value_round_trips::<AppVisibility>("AppVisibility");
    every_value_round_trips::<Attention>("Attention");
    every_value_round_trips::<ProofKind>("ProofKind");
    every_value_round_trips::<MenuSourceKind>("MenuSourceKind");
    every_value_round_trips::<SpaceKind>("SpaceKind");
    every_value_round_trips::<SpaceActivity>("SpaceActivity");
    every_value_round_trips::<ShellRole>("ShellRole");
    every_value_round_trips::<KeyGesture>("KeyGesture");
    every_value_round_trips::<ScreenEdge>("ScreenEdge");
    every_value_round_trips::<Corner>("Corner");
    every_value_round_trips::<Shown>("Shown");
    every_value_round_trips::<WindowEntry>("WindowEntry");
    every_value_round_trips::<SplitSide>("SplitSide");
    every_value_round_trips::<DragOutcome>("DragOutcome");
    every_value_round_trips::<Power>("Power");
    every_value_round_trips::<SubjectKind>("SubjectKind");
    every_value_round_trips::<Memory>("Memory");
    every_value_round_trips::<Answer>("Answer");
    every_value_round_trips::<TrialAnswer>("TrialAnswer");
    every_value_round_trips::<OverviewMode>("OverviewMode");
    every_value_round_trips::<OverviewState>("OverviewState");
    every_value_round_trips::<RunMode>("RunMode");
    every_value_round_trips::<RevokeReason>("RevokeReason");
    every_value_round_trips::<Suspend>("Suspend");
    every_value_round_trips::<GlowCode>("GlowCode");
    every_value_round_trips::<MaskReason>("MaskReason");
    every_value_round_trips::<LeaseState>("LeaseState");
    every_value_round_trips::<PressEdge>("PressEdge");
    every_value_round_trips::<SensitivityLevel>("SensitivityLevel");
    every_value_round_trips::<TrustedKind>("TrustedKind");
    every_value_round_trips::<TrustedAnchor>("TrustedAnchor");
    every_value_round_trips::<Withdrawn>("Withdrawn");
    every_value_round_trips::<ActorKind>("ActorKind");
    every_value_round_trips::<Transform>("Transform");
}

#[test]
fn consumers_stored_forms_are_unchanged() {
    // docket stores RunMode and prov stores ActorKind in these forms today.
    assert_eq!(
        round_trips(&RunMode::AgentWorkspace),
        r#""agent_workspace""#
    );
    assert_eq!(round_trips(&ActorKind::ThirdParty), r#""third_party""#);
    assert_eq!(round_trips(&MaskReason::A11ySecret), r#""a11y_secret""#);
    // casement's privacy book stores the power and the answer in these forms.
    assert_eq!(round_trips(&Power::TypingHelp), r#""typing_help""#);
    assert_eq!(round_trips(&Answer::Deny), r#""deny""#);
}

#[test]
fn held_mods_is_its_bits() {
    let chord = HeldMods::COMMAND.with(HeldMods::SHIFT);
    assert_eq!(round_trips(&chord), "9");
}

#[test]
fn a_tile_zone_names_the_same_place_in_every_enum() {
    for zone in TileZone::ALL {
        let target = PlaceTarget::from(*zone);
        assert_eq!(target.zone(), Some(*zone), "{zone:?}");
        assert_eq!(target.code(), zone.code(), "{zone:?} has one code in both");
        let mark = PlacementMark::from(*zone);
        assert_eq!(
            format!("{mark:?}"),
            format!("{zone:?}"),
            "{zone:?} keeps its name as a mark"
        );
    }
    for target in [PlaceTarget::Fill, PlaceTarget::Centre, PlaceTarget::Restore] {
        assert_eq!(target.zone(), None, "{target:?}");
    }
}
