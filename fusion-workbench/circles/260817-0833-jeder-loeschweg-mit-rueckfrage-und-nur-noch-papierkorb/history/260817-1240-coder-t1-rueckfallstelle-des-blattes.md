# T1 — eine unbekannte Blattantwort fällt auf die abbrechende Schaltfläche

**Datum:** 260817-1240
**Agent:** coder
**Status:** Complete
**Datensatz:** `issues/260817-1106_*_eine-unbekannte-blattantwort-faellt-im-loeschblatt-auf-die-zerstoerende-schaltflaeche.md`
**Durchsicht:** `reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`, Befund 1
**Baumstand vorher:** `472eb81`

---

## Was umgesetzt ist

Die Frage „welche Schaltfläche ist die ungefährliche" steht in `blaetter/mod.rs` jetzt
einmal, als reine Funktion über die angelegte Reihenfolge, und wird aus einer
ausdrücklichen Angabe des Blattes abgeleitet statt aus der Escape-Taste.

`crates/krk-ui/src/appkit/blaetter/mod.rs`

- Neue Aufzählung `Wirkung` mit zwei Werten, `Ausfuehren` und `Liegenlassen`. `Schaltflaeche`
  trägt sie als drittes Pflichtfeld, und `Schaltflaeche::neu` nimmt sie als drittes Argument.
  **Ohne Vorgabe**: ein künftiges Blatt kann die Angabe nicht stillschweigend auslassen.
- Neue reine Funktion `abbruchstelle(&[Schaltflaeche]) -> usize` mit ausgeschriebener Tafel
  (drei Zeilen: erste liegenlassende Stelle, keine liegenlassende, keine Schaltfläche) und
  `#[must_use]`. `Blatt::mit_schaltflaechen` ruft sie einmal je Blatt.
- Die Ableitung über `Taste::Escape` ist gefallen. Sie konnte für die Löschrückfrage nichts
  liefern, weil dieses Blatt keine Schaltfläche mit Escape trägt.
- `Blatt.abbruchstelle` ist `usize` statt `Option<usize>`; damit fällt die zweite,
  entgegengesetzte Vorbelegung (`map_or(NSAlertFirstButtonReturn, …)`) weg.
- Drei Stellen lesen die eine Antwort: der Auffangzweig im Abschlussblock (vorher
  `antworten.len().saturating_sub(1)`), der `abbruchcode` des `Blattgriff`, und der
  `Eingabewaechter` auf dem Escape-Weg (vorher fest `NSAlertSecondButtonReturn`). Damit ist
  die Einfuhr von `NSAlertSecondButtonReturn` weg.
- `debug_assert!` in `mit_schaltflaechen`: mindestens eine Schaltfläche trägt
  `Wirkung::Liegenlassen`. Ein Blatt ohne ungefährlichen Ausgang fliegt im Probenbau auf,
  statt still auf eine ausführende Schaltfläche zu fallen.
- Modulkopf: neuer Abschnitt „Welche Schaltfläche die ungefährliche ist, steht genau einmal"
  mit dem behobenen Befund und dem Grund, warum die Escape-Taste die Frage nicht beantworten
  kann. Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` ist nachgezogen:
  `NSAlertSecondButtonReturn` steht nicht mehr in der Aufzählung der vier
  Übersetzungszeitkonstanten, sondern als Satz darunter. **Keine neue Klasse und keine neue
  Methode**, also keine neue Untergrenze.

`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs`

- Die beiden Schaltflächen stehen als reine Funktion `schaltflaechen(vorgang)` da, damit die
  Reihenfolge ohne AppKit und ohne Hauptfaden prüfbar ist.
- `const AUSFUEHRENDE_STELLE: usize = 1` ersetzt die nackte `1` im Rückruf und ist die eine
  Zahl, gegen die die Probe liest.
- Modulkopf: ein Absatz zur unbekannten Antwort und zum behobenen Befund.

Nachgezogen, je um die dritte Angabe an `Schaltflaeche::neu`:
`blaetter/konflikt.rs` (Abbrechen an vierter Stelle), `blaetter/ungesichert.rs` (an dritter),
`blaetter/uebersprungen.rs` („Schließen", einzige), `blaetter/zettel.rs` („Fertig", einzige),
`appkit/belegungsansicht.rs` („Fertig", einzige). In `ungesichert.rs` sagt der Kommentar am
Auffangzweig jetzt, woher die Stelle kommt.

## Nachgezählt statt übernommen

Sechs Aufrufer von `mit_schaltflaechen` und fünf von `Blatt::neu`; `Blatt::neu` legt selbst
„Abbrechen" mit `Wirkung::Liegenlassen` an zweiter Stelle an.

| Blatt | Schaltflächen in Reihenfolge | Escape | liegenlassende Stelle | vorher: letzte |
|---|---|---|---|---|
| `loeschbestaetigung.rs` | Abbrechen, `<Vorgang>` | keine | **0** | 1 (die löschende) |
| `konflikt.rs` | Überschreiben, Überspringen, Umbenennen, Abbrechen | Stelle 3 | 3 | 3 |
| `ungesichert.rs` | Sichern, Verwerfen, Abbrechen | Stelle 2 | 2 | 2 |
| `uebersprungen.rs` | Schließen | keine | 0 | 0 |
| `zettel.rs` | Fertig | Stelle 0 | 0 | 0 |
| `belegungsansicht.rs` | Fertig | keine | 0 | 0 |
| `Blatt::neu` (fünf Blätter) | `<bestätigen>`, Abbrechen | Stelle 1 | 1 | 1 |

Der `abbruchcode` des `Blattgriff` ist damit an **keinem** Blatt ein anderer als vorher; die
Löschrückfrage ist das eine Blatt, an dem der Auffangzweig des Abschlussblocks sich ändert,
von der ausführenden Stelle 1 auf die abbrechende Stelle 0. Genau der Befund.

## Proben

Fünf neue, alle ohne AppKit und ohne Hauptfaden:

- `blaetter/mod.rs`: `die_tafel_der_liegenlassenden_stelle` (jede Zeile der Tafel, dazu zwei
  liegenlassende und die leere Liste), `ohne_escape_taste_faellt_die_antwort_trotzdem_auf_die_liegenlassende`,
  `jedes_blatt_nennt_seine_liegenlassende_schaltflaeche`.
- `blaetter/loeschbestaetigung.rs`: `eine_unbekannte_antwort_stellt_keinen_auftrag` (das
  Abnahmekriterium: die Rückfallstelle ist nicht `AUSFUEHRENDE_STELLE`),
  `die_ausfuehrende_stelle_zeigt_auf_die_ausfuehrende_schaltflaeche`.

Die dritte ist eine Zählprobe über den Quellbaum (`crate::quellbaum`), weil die Zusage eine
Aussage über den Baum ist: es gibt kein Blatt, dessen Schaltflächen alle etwas ausführen. Ihre
Blindheit steht an ihrem Doc-Kommentar — sie prüft je Datei und nicht je Blatt, und heute
trägt jede Datei genau ein Blatt.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün: Bau, Proben, Clippy unter `-D warnings`,
Formatprüfung.

## Was dieser Schritt nicht baut

Keinen erreichbaren Auslöser hat die Durchsicht für den alten Zustand gefunden, und dieser
Schritt sucht auch keinen: er nimmt der Vorbelegung die zerstörende Richtung. Der `Rumpf` der
Schutzschwelle bleibt ungeprüft (`issues/260817-1107_o_…`), und die bestätigende Seite des
`Eingabewaechter` bleibt fest auf der ersten Schaltfläche — jedes bewachte Blatt kommt aus
`Blatt::neu`, dessen Reihenfolge sie dort festlegt. Das Konfliktblatt trägt ein Textfeld
**ohne** Wächter; daran ist nichts angefasst.
