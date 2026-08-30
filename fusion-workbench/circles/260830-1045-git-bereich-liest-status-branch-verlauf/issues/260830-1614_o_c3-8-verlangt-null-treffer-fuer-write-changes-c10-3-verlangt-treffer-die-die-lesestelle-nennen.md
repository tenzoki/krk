C3.8 verlangt null Treffer für `write_changes`, C10.3 verlangt Treffer, die die Lesestelle nennen
---
Zwei Abnahmekriterien des Specs der Runde 23 schreiben demselben `grep` zwei verschiedene Ergebnisse
vor.

- **C3.8:** „Zu prüfen mit `grep -rn 'write_changes' crates/`, das nach der Runde **keine
  Fundstelle** liefert".
- **C10.3:** „Zu prüfen mit `grep -rn 'NeedsUpdate\|write_changes' crates/`, dessen **Treffer die
  Lesestelle nennen** und keine Schreibstelle".

Der Plan entscheidet die Sache in Schritt 3 zugunsten von C10.3, und zwar ausdrücklich:
„`EntryStatus::NeedsUpdate` wird gelesen und verworfen; `Outcome::write_changes` wird nicht gerufen
(E8, C3.8, C10.3), und **der Modulkopf sagt es** mit dem Verweis auf den offenen Datensatz". Ein
Modulkopf, der es sagt, nennt beide Namen, und damit findet das `grep` sie.

**Stand nach Schritt 3:** `grep -rn 'NeedsUpdate\|write_changes' crates/` liefert Treffer in
`crates/krk-core/src/git/mod.rs` (Modulkopf), `crates/krk-core/src/git/leser.rs` (Modulkopf und der
Zweig, der `EntryStatus::NeedsUpdate` liest und verwirft). Keiner davon ist ein Aufruf; `write_changes`
kommt allein in Prosa vor.

Die beiden Kriterien sind nicht zugleich erfüllbar, solange der Plan den Modulkopf verlangt. C3.8 in
seinem Wortlaut zu erfüllen hieße, die Begründung zu streichen, die der Plan ausdrücklich anordnet
und die den offenen Datensatz
`shared/decisions/260830-1006_*_darf-stufe-a-den-aufgefrischten-index-zurueckschreiben-oder-zahlt-sie-die-wiederholung.md`
im Code auffindbar macht.

**Ein zweiter Riss in demselben `grep`, und er ist aelter als diese Runde:** das Muster
`NeedsUpdate` trifft in `crates/krk-ui/src/appkit/` neunmal `menuNeedsUpdate:`, den Selektor des
Kontextmenues, in `teilen.rs`, `vorschau.rs`, `tabelle.rs` und `anwendung.rs`. Diese Treffer standen
schon vor der Runde 23 da und haben mit `gix` nichts zu tun. Ein Kriterium, dessen Pruefmittel neun
Treffer liefert, die es nicht meint, prueft nichts; das Muster gehoert an eine Wortgrenze gebunden
(`grep -rnw` oder `EntryStatus::NeedsUpdate`).

**Abnahmetest:** C3.8 verlangt nicht mehr null Treffer, sondern dasselbe wie C10.3 — dass jeder
Treffer eine Lesestelle oder eine Prosastelle ist und keine Schreibstelle. Die Prüfung, die dabei
trägt, ist die auf den **Aufruf**: `grep -rn 'write_changes(' crates/` bleibt ohne Fundstelle.
