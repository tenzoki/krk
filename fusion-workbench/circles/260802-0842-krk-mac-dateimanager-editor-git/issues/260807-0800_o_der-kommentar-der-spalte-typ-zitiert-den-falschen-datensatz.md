Der Kommentar der Spalte Typ zitiert den falschen Datensatz

---

Der Dokumentationskommentar der Aufzählungsvariante `Spalte::Typ` verweist für
den Entscheid, dass die Spalte die Dateiendung zeigt, auf den
Sortierungsdatensatz. Dort steht dieser Entscheid nicht; der zitierte Datensatz
sagt ausdrücklich das Gegenteil.

---

## Die Stelle

`crates/krk-ui/src/appkit/tabelle.rs:137-146`:

```rust
/// Die Dateiendung.
///
/// "Typ" heisst in KRK die Dateiendung: die Spalte zeigt sie, die
/// Sortierung nach Typ ordnet nach ihr ([`Schluessel::Typ`]), und die
/// Tastenfunktion "Nach Typ sortieren" loest dieselbe Ordnung aus. Die
/// Eintragsart selbst (Ordner, Datei, Verknuepfung) steht in der
/// Metadatenanzeige der Vorschau, nicht in der Tabelle; entschieden am
/// 260806, siehe den Datensatz zur Sortierung ohne sprachsensitive
/// Kollation.
Typ,
```

## Was auseinanderläuft

`decisions/260802-1810_i_sortierung-ohne-sprachsensitive-kollation.md`
entscheidet zwei Dinge: die sprachsensitive Kollation für die Namenssortierung
und die Dateiendung als **Schlüssel der Typsortierung**. Über den Inhalt der
Spalte sagt er nichts.

Der Entscheid über den Zelleninhalt steht in
`issues/260806-1723_c_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md`
unter der Überschrift "Nutzerentscheid vom 260806-2300: ein fünfter Weg". Genau
dieser Datensatz hält im Abschnitt "Warum es hier steht und nicht im Code"
fest: "der Entscheid vom 260806 trifft sie nicht: er spricht über den Schlüssel
der Sortierung, nicht über die Spalte."

Wer dem Kommentar folgt, landet also bei einem Datensatz, der die zitierte
Aussage ausdrücklich von sich weist, und findet den tragenden Entscheid nicht.

## Denkbarer Weg

Den Verweis auf den Defektdatensatz umstellen, ohne Zustandsmarker im Pfad, wie
`issues/260806-1320_c_die-belegungsdateien-zitieren-workbench-pfade-mit-zustandsmarker.md`
es für zitierte Workbench-Pfade festgelegt hat:

```
issues/260806-1723_*_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md
```

Der Verweis auf den Sortierungsdatensatz darf danebenstehen, dann aber für die
Aussage, für die er gilt: den Schlüssel der Sortierung.

## Dringlichkeit

Gering. Kein Nutzer sieht etwas davon. Der Wert liegt darin, dass die nächste
Änderung an der Spalte den Entscheid findet, der sie bindet.

**Betrifft:** `crates/krk-ui/src/appkit/tabelle.rs`.

**Aufgefallen bei:** der inkrementellen Durchsicht nach Turn 25 der Sitzung
260806-2257, Diff `f9a0462..HEAD`, Commit `3e9613a`.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1723_c_die-spalte-typ-zeigt-die-eintragsart-sortiert-aber-nach-der-endung.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260806-1123_c_modulkopf-zitiert-den-issue-pfad-mit-ueberholtem-marker.md`
