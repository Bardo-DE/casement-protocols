# Conventions

casement-protocols follows the program's shared conventions, whose text lives in quire's
`CONVENTIONS.md` (identical in quire, sill, shell-host, palmrest and detent): types, traits,
effects, errors, tests, comments, change discipline, borrowing. Read it first (its type rules are
replaced by rule 4 below). `ARCHITECTURE.md`
here adds the protocol map, the one-home table, the trait, the recipes and the repo rules; where
the two disagree, `ARCHITECTURE.md` wins for this repo.

What this repo adds or decides differently, each with its reason:

1. **Closed sets are `wire_enum!`s implementing `WireCode`, not quire's `Word`.** This crate is
   a leaf below quire (quire's `ds-blitz` depends on it), so it cannot depend on `ds-core`. A
   value's stable slug is its serde `snake_case` form, which is also its XML entry name;
   labels a person reads belong to the UI that draws them.
2. **The XML is the interface.** The vocab and the bindings follow it; a test fails when either
   drifts from it. A wire change starts in the XML.
3. **Generated bindings are exempt from the style rules** (names, docs, `#[non_exhaustive]`):
   they are wayland-scanner's output, kept behind `client` and `server`, and nothing hand-written
   lives in those modules.
4. **Types: plain Rust first** (owner, 2026-10-28; replaces "no bool in a field or parameter" and
   "never `dyn`").
   - `bool` is fine for fields, return values, locals and private functions. Use an enum only when
     a public function would read as `f(true, false)`, a third state is realistic, or the value is
     written to disk or the bus where a named word reads better.
   - Newtypes only when they hold a rule: parsed or validated, a unit (`ScaleMilli`,
     `RefreshMilliHz`), or an id that must not be mixed with another.
   - Enums for closed sets the code branches on; a set that grows is data.
   - Behaviour that differs by case is a trait with default methods. Generics by default;
     `Box<dyn Trait>` is fine when the implementation is chosen from configuration at run time.
   - One fact, one type: no mirror enums across crates; a field-less view of an enum is a method
     (`PlaceTarget::zone`, `WireCode::to_wire`), not a second declaration. The lower crate owns
     it: `vocab::Transform` is `wl_output.transform` for the compositor, the client and the shell.
   - Two-way outcomes are `Result` or `Option`, not a two-arm enum (`supports` is a `bool`).
   - If the type can say it, no runtime check says it.
