# C9 — Is the volume local

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 9 (third step of bundle C)
**Tree state before the task:** `3fcd375`
**Verification:** `make check` — exit 0

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`.
> The code stays German, identifiers and prose alike; existing artifacts are not translated.

## What the task asked for

One function in one file: `#[must_use] pub fn ist_lokal(pfad: &Path) -> Loeschzielbefund` in
`crates/krk-ui/src/appkit/volumes.rs`, over `resourceValuesForKeys_error` with
`NSURLVolumeIsLocalKey`. It answers trigger 3 of C3, "the folder's volume is not a local one". A
missing or unreadable value must be `Unentschieden` and never `Ja`. The module header takes on the
third question the module now answers, and its availability section takes on the new key, with the
lower bound read at the SDK rather than copied from the plan.

No caller — that is step 11. Nothing else in the tree was touched.

## Where the version numbers were read

All of them in
`$(xcrun --show-sdk-path)/System/Library/Frameworks/Foundation.framework/Headers/`, on 260817,
with the SDK path resolving to
`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk`.

| Touch | Header line | What the header says |
|---|---|---|
| `NSURLVolumeIsLocalKey` | `NSURL.h:338` | `API_AVAILABLE(macos(10.7), ios(5.0), watchos(2.0), tvos(9.0))` |
| `resourceValuesForKeys:error:` | `NSURL.h:183` | `API_AVAILABLE(macos(10.6), …)` |
| `fileURLWithPath:` (bare form) | `NSURL.h:52` | no `API_AVAILABLE`, therefore since 10.0 |
| `NSURLResourceKey` | `NSURL.h:17` | no `API_AVAILABLE`; a `typedef` on `NSString`, not a class |
| `NSNumber` | `NSValue.h:42` | no `API_AVAILABLE`, therefore since 10.0 |
| `boolValue` | `NSValue.h:73` | no `API_AVAILABLE`, therefore since 10.0 |

The plan's figure for `NSURLVolumeIsLocalKey` — 10.7, `NSURL.h:338` — holds exactly, line number
included. Following the precedent set at step 5, the touches that carry **no** availability note
are named as such rather than left out: a reader who finds `NSURLResourceKey` absent from the list
cannot tell "checked, since 10.0" from "never checked". Nothing in this file is younger than 10.7,
and the bundle targets 15.0.

`NSURLVolumeIsLocalKey` exists in `objc2-foundation 0.3.2` (`src/generated/NSURL.rs:639`), unlike
`NSURLVolumeSupportsTrashKey`, which step 5 found missing from both the SDK and the binding.

## The finding this step produced: the return value runs the wrong way

**The name the plan prescribes is truthful about what the function computes, and its return value
is the inverse of the field that will consume it.** That is filed as
`issues/260817-1623_o_ist-lokal-returns-the-inverse-of-the-field-it-fills.md` and is the one thing
from this step that needs a decision before step 11.

```text
  ist_lokal        Ja = local        harmless      polarity 2
  netzlaufwerk     Ja = network      warns         polarity 1
                   └── one type, two directions, nothing the compiler sees
```

`Loeschzielbefund` carries two polarities, written out in its module header. This function lands on
the **second** one, the same as `papierkorb::fuehrt_einen_papierkorb`: `Ja` is the harmless answer
and `Unentschieden` belongs with `Nein`. The field `Loeschziel.netzlaufwerk` from step 10 lands on
the first. A caller writing `netzlaufwerk: volumes::ist_lokal(&ordner)` compiles and inverts the
trigger, and the promise "undecided counts as loud" *keeps holding* while it does, because
`Unentschieden` is a fixed point of the inversion. The wrong reading is therefore not loud; only
the two decided cases swap, and they swap silently.

The name was not reversed on my own authority. The plan prescribes `ist_lokal`, and a silent flip
would have removed the question instead of raising it. What step 9 could do, it did: the module
header states the polarity and the inversion the caller owes, the doc comment names each outcome
and which of them warn, and a counting probe pins the one habit that would produce the error inside
this file. None of the three reaches `appkit/anwendung.rs`, and none makes the swap uncompilable —
the record names the three ways to do that and recommends the cheapest.

The same record notes that `Loeschzielbefund` has no three-valued inversion, so step 11 currently
has to write the `match` by hand at the call site.

## The counting probe, and the older record it partly serves

`hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt` counts over this file alone and asserts zero
call sites of `ist_warnwuerdig`, with the needle composed via `concat!` because the probe lies in
the tree it reads. It also fails loudly if `krk-ui/src/appkit/volumes.rs` is not among the files
`quellbaum::quelldateien` returned — otherwise a rename would make the count vacuously green.

That is the first of the two directions in
`issues/260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`,
which named steps 9 and 10 as the place where the cut would fall. **The record stays open** and
gained a progress note: `appkit/papierkorb.rs` and `kommandos/loeschwarnung.rs` carry no such
count, and its second, stronger direction is untouched.

## The negative outcome is measured, not assumed

**The task asked whether a place exists that is not local on every macOS, and one does:
`/System/Volumes/Data/home`.** macOS mounts the `auto_home` automount there; `/sbin/mount` lists it
without the `local` flag, and `NSURLVolumeIsLocalKey` answers `false`. Without that probe
`ist_lokal` would be green with a hardcoded `Ja`, which is the failure mode step 5 ran into and
resolved with `/dev`.

Two things about it are worth carrying forward.

**It is the target path and not `/home`.** Measured on 260817, `/home` answers `true` while
`/System/Volumes/Data/home` answers `false`, although the first is a firmlink onto the second. Why
that is, I did not establish, and I did not guess in the source: the measurement stands, and the
doc comment warns that "simplifying" the path makes the probe silently green.

**The probe checks its own precondition.** A user can switch the `/home` automount off in
`/etc/auto_master`; an ordinary empty directory would then stand there, it would be local, and the
probe would go red without anything being wrong with `ist_lokal`. So it first asserts that a
distinct mount point is present at all, by comparing the device id from `stat(2)` against the
parent directory's — pure `std`, no AppKit, and not the function under test. Missing the mount
point **stops** the probe with a message naming the cause rather than skipping it: a silent skip
would delete the only negative measurement in this file without anyone noticing.

This is a weaker guarantee than step 5's `/dev`, and it is named as such rather than dressed up.
`/dev` cannot carry a trash directory by its nature; `/System/Volumes/Data/home` is not local by
system configuration, and configuration can be changed. On this reference device the measurement
holds today. What no probe here reaches is a Finder-mounted network share — acceptance criterion
"Dasselbe gilt auf einem vom Finder eingehängten Netzlaufwerk" under C3 remains the user's run.

## What the probes establish, one by one

| Outcome | Probe | What it rests on |
|---|---|---|
| `Ja` | `das_benutzerverzeichnis_liegt_auf_einem_lokalen_datentraeger` | the real home directory |
| `Nein` | `ein_nicht_lokaler_datentraeger_wird_erkannt` | the `auto_home` automount, with its precondition checked |
| `Unentschieden`, error branch | `ein_fehlender_pfad_bleibt_unentschieden` | a path asserted absent first |
| `Unentschieden`, translation branch | `ein_pfad_ohne_gueltiges_utf8_bleibt_unentschieden` | the byte `0xff`, invalid in any UTF-8 sequence |
| the polarity | `hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt` | a count over this file's own source |

The missing-path probe is the one that separates this function from its neighbour: at
`papierkorb::fuehrt_einen_papierkorb` an error **is** the answer and means `Nein`, here it says
nothing about the volume and a `Nein` would claim a network share nobody saw. In the running
program the case does not arrive, because the caller resolves the folder first and counts a failed
resolution as undecided itself; the probe pins the branch anyway, because it is the one place a
convenient default would silence the warning.

None of the five needs a window or the main thread, on the same ground the header of
`appkit/papierkorb.rs` writes out for its own probes.

## What was written

- `crates/krk-ui/src/appkit/volumes.rs`, `+330 −11`:
  - the opening of the module header, which now names three questions instead of one, with a sketch
    of which function asks which of AppKit and why the third belongs here rather than beside;
  - a new section `# Die dritte Frage, und auf welcher Polaritaet ihre Antwort liegt`;
  - the availability section, rewritten with the six touches above and the note that the line
    numbers were read at the SDK;
  - `ist_lokal` with `#[must_use]` carrying its reason, and `namensteil` switched from a
    fully-qualified `std::path::Path` to the now-imported `Path`;
  - a `#[cfg(test)]` constant `AUTOMATIK_HOME` and a probe module with the five probes.

`#[cfg_attr(not(test), expect(dead_code, reason = …))]` sits on `ist_lokal` in the form step 5 used,
and **step 11 has to remove it**: it is the first caller, and with a caller the expectation goes
unfulfilled and `-D warnings` stops the build until the lines are gone. That `expect` and not
`allow` is deliberate, per the header of `kommandos/rueckschritt.rs` — an exception with an expiry
date. That it is genuinely needed was measured and not assumed: with the attribute removed,
`cargo clippy --workspace --all-targets -- -D warnings` reports `error: function ist_lokal is never
used`.

## Verification

`make check` — exit 0. All four acceptance commands green: 1,304 probes passing across the
workspace, 10 ignored, among them the 5 new probes of this step. `cargo fmt --all --check` is
clean.

The load-dependent race probe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`
(`tests/text.rs`) was green in this run. It is described by two open records,
`shared/issues/260816-0055_o_…` and `shared/issues/260815-1019_o_…`, and is not a finding of this
step.

## What this step did not touch

`appkit/mod.rs` keeps its three mentions of `volumes`; none of them becomes wrong, since none
enumerates the module's questions. No `Warngrund`, no `Loeschziel`, no table of triggers — step 10.
No caller and no fact-gathering in `appkit/anwendung.rs` — step 11. `CLAUDE.md` untouched: it names
neither this module nor a count that changed.

One observation in passing, not filed: the closing section of the previous log,
`260817-1602-coder-c8-arbeitsbaum-aufwaerts-und-in-der-auswahl.md`, has the two step numbers the
other way round — it puts the `krk-ui` caller at step 10 and the trigger table at step 9, while the
plan has step 9 as this function and step 10 as the table. A record of a past state, left as it is.
