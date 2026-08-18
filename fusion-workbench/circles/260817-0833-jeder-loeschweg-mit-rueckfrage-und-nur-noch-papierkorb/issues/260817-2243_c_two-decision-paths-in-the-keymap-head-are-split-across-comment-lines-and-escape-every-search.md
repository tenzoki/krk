Two decision paths in the keymap head are split across comment lines and escape every search

---
`82707ef` added two references to decision records in the head of
`resources/default-keymap.toml` and broke each one after `shared/decisions/`, so the directory
and the filename stand on separate comment lines. A search for the path as a path finds neither.

---

**Severity:** Low. Both records exist and both are the right ones; the citation is only
unreachable by the search that would find it. The project has filed the same class of defect
once already — `shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`,
where eight short-form references escaped five surveys in a row.
**Found by:** coderev, review `reviews/260817-2243-coderev-bundle-d-the-removal.md`
**Affected:** `resources/default-keymap.toml:12-13`, `:66-67`
**Tree state:** `f7a85c1`
**Domain:** data

## Measured

```
$ sed -n '11,13p;66,67p' resources/default-keymap.toml
# `shared/decisions/260802-0842_*_f-tasten-unter-macos-systembelegung.md` und
# `shared/decisions/
# 260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.
# (`shared/decisions/
# 260817-0536_*_bekommt-f8-den-papierkorb-nachdem-das-endgueltige-loeschen-weggefallen-ist.md`).

$ grep -rn "shared/decisions/260817-0536" resources/
(no match)
```

The one line above them, `:11`, carries its whole path unbroken. So does every citation of these
same records elsewhere in the tree — seven of them, in `krk-core/tests/belegung.rs:1625`,
`krk-core/src/verzeichnis/arbeitsbaum.rs:32,162,164`, `loeschzielbefund.rs:134`, `umfang.rs:150`
and `krk-ui/src/appkit/blaetter/loeschbestaetigung.rs:73`.

## The break buys nothing

It is not a line-width rule. The two lines are 97 and 96 characters after the break, and the
file already carries 21 lines over 80 characters, the longest at 199:

```
$ awk 'length($0) > 80 { n++ } END { print n }' resources/default-keymap.toml
21
$ awk 'length($0) > 80 { if (length($0) > m) m = length($0) } END { print m }' resources/default-keymap.toml
199
```

## Direction

Put each path on one comment line, as `:11` and as the rest of the tree do. Length is not a
constraint in this file, and a citation that a path search cannot find is the one form that
survives a review while being useless.

---
Resolved 260818 (ontocoder, tree state `48bb57f`): **every path in
`resources/default-keymap.toml` now stands whole on one comment line, and every one of them
resolves to a file that exists.**

**The survey ran over the whole file, not over the two lines this record names.** It found
**22** broken paths and not two: `:12`, `:66`, `:85`, `:122`, `:277`, `:353`, `:359`, `:366`,
`:397`, `:421`, `:519`, `:540`, `:635`, `:649`, `:696`, `:732`, `:868`, `:922`, `:958`, `:996`,
`:1012` and `:1023` in the pre-fix numbering. Two of them broke across **three** comment lines,
not two. Each was joined and its paragraph re-wrapped to the file's prevailing width of 78
characters, with the path itself held on one line as an unbreakable token.

**A second defect surfaced under the same check and is fixed with it.** Ten citations stood in
the bare form `decisions/…` or `issues/…` without a Circle. `CLAUDE.md` resolves an unqualified
path of that form against **Runde 2**, `circles/260807-2116-eingebauter-editor-mit-textmarken`,
and none of the ten lives there: nine belong to Runde 1
(`circles/260802-0842-krk-mac-dateimanager-editor-git`) and one to Runde 4
(`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`). Read by the project's own
path rule they pointed at nothing. All ten now carry their Circle, the form the majority of this
file already used.

**Measured after the change**, 33 paths in backticked form, each one on a single line and each
one resolving through its marker glob:

```
$ grep -nE '`[^`]*/$' resources/default-keymap.toml
(no match)
$ python3 <glob-resolve every backticked path>
paths checked: 33  unresolved: 0
```

`make check` — exit 0.
