CLAUDE.md says the bundle ships as v0.4.1, and four tags have been set since

---
`CLAUDE.md:39` opens the section `## Projektstand` with "Geprüft am 260815-0600" and states that
`target/KRK.app` "liegt als `v0.4.1` aus". `Cargo.toml:13` carries `version = "0.5.1"`, and
`git tag -l` shows `v0.4.3`, `v0.4.4`, `v0.5.0` and `v0.5.1` after `v0.4.1`.

---

**Severity:** Low. Nothing reads the number, and the whole `## Projektstand` section is dated, so
a reader who takes the date seriously knows the paragraph is a snapshot. It is filed because
CLAUDE.md is the file every session opens first, and because the paragraph does not say "as of
260815 it was 0.4.1" — it says the bundle ships as 0.4.1, in the present tense, three releases
later.

**Found by:** coderev, review
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/reviews/260818-0024-coderev-bundle-e-the-prose-and-the-records.md`
**Cross-references:** `shared/issues/260816-2138_o_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md`
(the same paragraph's round count, already open and already stale a second time)
**Tree state:** `da716c1`
**Domain:** code

## Measured

```
$ grep -n "liegt als" CLAUDE.md
39: … ist signiert und liegt als `v0.4.1` aus. …

$ grep -n '^version' Cargo.toml
13:version = "0.5.1"

$ git tag -l | tail -5
v0.4.3
v0.4.4
v0.5.0
v0.5.1
```

## Origin

Found beside the Directive of
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`, not out of it: the
review of its bundle E had to check whether the executor's claim "die übrige Datei ist gegen den
Baum nachgezählt und stimmt" holds. It does for the four enumerations that claim names, and does
not for this line. The false claim itself is filed inside that Circle
(`issues/260818-0029_o_the-record-claims-the-rest-of-claude-md-was-counted-against-the-tree.md`);
this record is the defect in CLAUDE.md, which belongs to no Directive.

## Direction

Either drop the version from the sentence — `Cargo.toml` and `git tag -l` are the single source,
and this file already applies that reasoning to `Kommando` ("Für `Kommando` … steht hier keine
Zahl") and to the round count — or make the tense match the date the paragraph already carries.
The first is the pattern this file has chosen every other time a number aged, and it is the one
that cannot age again.

---
Abgleich 260819-1440 (reconciler, Baumstand `77dcd48`): **offen, und der Abstand ist von vier auf sieben Tags gewachsen.** `CLAUDE.md:39` sagt unverändert, das Bündel liege als `v0.4.1` aus. `Cargo.toml:13` trägt heute `version = "0.5.4"`, und der jüngste Tag ist `v0.5.4`. Nach `v0.4.1` stehen **sieben** Tags: `v0.4.3`, `v0.4.4`, `v0.5.0`, `v0.5.1`, `v0.5.2`, `v0.5.3`, `v0.5.4`. Der Datensatz misst vier; drei sind seither dazugekommen. Der Marker bleibt `_o_` für den Durchgang des Kurators; dieser Abgleich fasst `CLAUDE.md` nicht an.
