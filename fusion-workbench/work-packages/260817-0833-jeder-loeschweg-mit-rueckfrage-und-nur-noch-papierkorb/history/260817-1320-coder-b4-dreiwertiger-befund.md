# B4 — der dreiwertige Befund, mit ausgeschriebener Tafel über neun Kombinationen

**Datum:** 260817-1320
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, Schritt 4
(Bündel B, erster Schritt)
**Spec:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, die Zusage
„Unentschieden gilt als laut" und die Abnahmekriterien von C3 und C4
**Datensatz:** `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`
(bleibt `_a_`; er wandert nach Plan in Schritt 16)
**Baumstand vorher:** `8c18887`

---

## Was umgesetzt ist

`crates/krk-core/src/verzeichnis/befund.rs` (neu, 356 Zeilen)

- `pub enum Befund { Ja, Nein, Unentschieden }`, abgeleitet `Debug, Clone, Copy, PartialEq,
  Eq`. Kein `Ord`; warum nicht, steht unten unter „Zwei Entwurfsentscheidungen".
- `ist_warnwuerdig(self) -> bool` — „nicht `Nein`", als Or-Muster
  `Self::Ja | Self::Unentschieden => true` und `Self::Nein => false`. Kein Auffangzweig.
- `oder(self, andere) -> Befund` mit **neun ausgeschriebenen Zweigen**, einem je Feld der
  Tafel, und ohne jedes `_`-Muster.
- Beide tragen `#[must_use = "…"]` mit ausgeschriebenem Grund, in der Form der fünf
  bestehenden Stellen in `krk-core` (`sys.rs:894`, `ablage/atomar.rs:113`,
  `ablage/sperre.rs:111` und `:161`, `ablage/lesezeichen.rs:317`).
- Fünf Proben in `#[cfg(test)]`, alle ohne Prüfordner: es ist eine reine Aufzählung, und
  keine Probe fasst eine Datei an.

`crates/krk-core/src/verzeichnis/mod.rs`

- `pub mod befund;` und `pub use befund::Befund;`, damit `krk_core::verzeichnis::Befund`
  trägt.
- Der Modulkopf sagt jetzt „Elf Module" statt „Zehn", das Bild führt `befund` neben
  `verweisziel` als eines, das an keinem anderen hängt, und ein neuer Absatz nennt seinen
  Gegenstand: das einzige Modul hier, das **nichts liest**, und die Verallgemeinerung
  dessen, was `sys::ist_deskriptormangel` seit der Runde 10 am `durchlauf` leistet.
- **Eine Zeile mitgezogen, weil sie sonst falsch geworden wäre:** der Absatz über
  `verweisziel` sagte „haengt als einziges Modul an gar keinem anderen" und sagt jetzt
  „als einziges lesendes Modul". `befund` hängt ebenfalls an keinem, und die Aussage stand
  als Alleinstellung da.

## Woraus die Tafel abgeleitet ist

Der Auftrag verlangt die Ableitung an der Funktion und nicht die Behauptung. Sie steht dort
in drei Stufen und ergibt genau diese Tafel:

| `self` \ `andere` | `Ja` | `Nein` | `Unentschieden` |
|---|---|---|---|
| **`Ja`** | `Ja` | `Ja` | `Ja` |
| **`Nein`** | `Ja` | `Nein` | `Unentschieden` |
| **`Unentschieden`** | `Ja` | `Unentschieden` | `Unentschieden` |

1. Ein `Ja` ist eine gewusste Tatsache, und keine zweite Antwort nimmt sie zurück — `Ja` ist
   aufsaugend, das sind fünf der neun Felder.
2. Ruhig wird es nur mit Wissen: die ruhige Rückfrage sagt dem Nutzer, dass an diesem Ziel
   nichts Ungewöhnliches ist, und darf nur dastehen, wenn **beide** Seiten entschieden haben.
   `Nein.oder(Nein)` ist das einzige ruhige Feld.
3. Die drei übrigen Felder behalten den Zweifel. Nicht `Ja`, weil C3 den Grund nennen lässt
   und „ließ sich nicht einordnen" ein anderer Grund ist als der Wortlaut eines Auslösers —
   ein `Ja` hier machte den Grund falsch. Nicht `Nein`, weil das über genau den Fall
   schwiege, für den die Zusage da ist.

## „Unentschieden gilt als laut" und die dreiwertige Logik fallen nicht auseinander

Der Auftrag hält die Möglichkeit offen, dass sie es tun. Sie tun es für `oder` nicht, und
das ist nachgerechnet und nicht angenommen:

```text
a.oder(b).ist_warnwuerdig() == a.ist_warnwuerdig() || b.ist_warnwuerdig()
```

Die Gleichung hält in allen neun Feldern, und die Probe
`die_lautheit_ueberlebt_die_verknuepfung` fährt sie einzeln durch. Der Grund ist
strukturell: `ist_warnwuerdig` bildet `{Unentschieden, Ja}` auf `true` ab, und diese Menge
ist in der Ordnung `Nein < Unentschieden < Ja` nach oben abgeschlossen; damit verträgt sich
die Abbildung mit dem Maximum. Die abgeleitete Tafel ist deshalb dieselbe, die eine
Kleene-Logik liefert — sie ist hier aber aus den zwei Sätzen des Specs gewonnen und nicht
von dort übernommen.

**Auseinander fallen die beiden erst, wenn jemand `Unentschieden` vor der Verknüpfung in ein
`Ja` umdeutet.** Dann stimmt die Lautheit weiter und der Grund nicht mehr, und das ist genau
das Abnahmekriterium von C3, das den Grund „das Ziel ließ sich nicht einordnen" verlangt.
Der Satz steht als Warnung am Doc-Kommentar von `oder`.

## Die zwei Polaritäten, und was daran für die Schritte 5 und 6 hängt

**Beim Schreiben aufgefallen und im Modulkopf ausgeschrieben, weil es keine Formalie ist:**
die Fragen dieser Runde haben zwei verschiedene Polaritäten, und `ist_warnwuerdig` trifft nur
die eine.

- Netzlaufwerk und Git-Arbeitsbaum: ein `Ja` ist ein **Warngrund**, `Unentschieden` gehört zu
  `Ja`, und `ist_warnwuerdig` ist die richtige Frage.
- Papierkorb (C4): ein `Ja` ist die **Erlaubnis**, `Unentschieden` gehört zu `Nein`, und der
  Aufrufer muss auf `Befund::Ja` selbst prüfen.

Wer in Schritt 6 aus Gewohnheit nach der Warnwürdigkeit fragt, macht aus „wir wissen nichts"
die Erlaubnis zu löschen — der Fall, gegen den C4 gebaut ist. Kein Defekt und keine offene
Frage: beide Polaritäten folgen derselben Haltung, im Zweifel die vorsichtigere Antwort zu
nehmen, und welche das ist, hängt an der Frage und nicht am Typ. Deshalb steht es als eigener
Abschnitt im Modulkopf und nicht als Datensatz.

## Zwei Entwurfsentscheidungen, beide begründet im Code

**Neun Zweige und kein `_`-Muster, obwohl fünf davon zusammenfielen.** Ein
`(Self::Ja, _) => Self::Ja` wäre kürzer und nähme dem Übersetzer die Vollständigkeitsprüfung
gegen eine **vierte** Variante ab: die fiele still in den bestehenden Zweig. `CLAUDE.md`
nennt genau das als Absicht des Baums — eine neue Variante hält den Bau an.

**Kein abgeleitetes `Ord`, obwohl `oder` dann `max` wäre.** Die Bedeutung läge dann in der
Reihenfolge der Aufzählung, wo niemand sie liest, und ein späteres Umsortieren der Varianten
änderte die Verknüpfung still mit. Der Plan leitet `Ord` für `Warngrund` in Schritt 10
ausdrücklich ab, dort **ist** die Reihenfolge die Aussage; hier ist sie es nicht.

## Kein `expect(dead_code)`, und kein Schritt muss eine Zeile zurücknehmen

Der Auftrag hält die Bauform aus `rueckschritt.rs` für den Fall bereit, dass `-D warnings`
wegen ungenutzten Codes anhält. Sie ist **nicht** nötig: `krk-core` ist eine Bibliothek, und
`Befund` ist von ihrer Wurzel aus erreichbar, also greift `dead_code` gar nicht. Nachgeprüft
am grünen `clippy --workspace --all-targets -- -D warnings`, nicht geschlossen. Damit hat
kein späterer Schritt hier eine Zeile zu entfernen; der Absatz im Modulkopf sagt es, damit
der nächste Leser nicht nach einer sucht.

## Abnahme

`make check` — **exit 0**, alle vier Kommandos grün: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`.

Die fünf neuen Proben laufen und stehen namentlich im Lauf:

```
verzeichnis::befund::tests::die_lautheit_traegt_zwei_der_drei_werte ... ok
verzeichnis::befund::tests::die_tafel_aus_neun_kombinationen_geht_auf ... ok
verzeichnis::befund::tests::nur_zwei_mal_nein_bleibt_ruhig ... ok
verzeichnis::befund::tests::die_lautheit_ueberlebt_die_verknuepfung ... ok
verzeichnis::befund::tests::die_verknuepfung_ist_symmetrisch_und_nein_ist_neutral ... ok
```

Die Kiste `krk-core` zählt damit 161 Proben in ihren `#[cfg(test)]`-Modulen (Lauf über
`src/lib.rs`), vor diesem Schritt 156. Die Tafel steht Zeile für Zeile in
`die_tafel_aus_neun_kombinationen_geht_auf` als `const TAFEL: [(Befund, Befund, Befund); 9]`,
in der Bauform von `die_tafel_aus_acht_faellen_geht_auf` in
`krk-ui/src/kommandos/rueckschritt.rs`. Neun Kombinationen, neun geprüfte Zeilen, keine
gerechnete Erwartung.

Zusätzlich `cargo doc -p krk-core --no-deps`: 31 Warnungen, alle vorbestehend, keine in einer
der beiden angefassten Dateien. Die Doc-Verweise auf `super::sys::ist_deskriptormangel` und
`super::durchlauf` lösen auf.

## Grenzen eingehalten

Angefasst sind zwei Dateien, `verzeichnis/befund.rs` (neu) und `verzeichnis/mod.rs`. **Kein
Aufrufer eingeführt** — die kommen in den Schritten 5, 7, 8 und 9. Kein Commit; das macht der
Orchestrator. Der Planschritt bleibt unverändert; das `[DONE]` setzt der Orchestrator.
