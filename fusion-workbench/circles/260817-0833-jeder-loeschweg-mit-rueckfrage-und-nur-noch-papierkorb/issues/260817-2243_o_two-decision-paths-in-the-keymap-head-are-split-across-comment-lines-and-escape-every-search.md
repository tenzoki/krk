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
