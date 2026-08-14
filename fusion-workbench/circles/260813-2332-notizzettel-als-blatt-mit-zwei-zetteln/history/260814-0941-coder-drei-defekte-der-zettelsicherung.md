# Coder: die drei zusammenhängenden Defekte der Zettelsicherung

**Date:** 2026-08-14
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Vorgabe:** Spec `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`, C4 in der Fassung vom 260814-0925
**Behobene Datensätze:** `issues/260814-0908_c_ein-neuoeffnen-nach-gescheiterter-sicherung-wirft-den-ungesicherten-zettelstand-weg.md` (hoch), `issues/260814-0909_c_je-sicherungsmoment-wird-hoechstens-ein-zettel-geschrieben-und-beim-beenden-gibt-es-kein-naechstes-mal.md` (mittel, zwei Wege)
**Verification:** `make check` am 260814-0947, Rückgabewert 0, „alle vier gruen"

---

## Was gebaut ist

**Drei Wege, eine Regel und eine Schleife.** Alle drei Defekte werden von derselben Lage
ausgelöst, einer Sicherung, die nicht geschrieben hat, und sie sind in einem Zug behoben:

1. **Der getippte Stand gewinnt.** `Zettelmodell::oeffnen`
   (`crates/krk-ui/src/zettelmodell.rs`) behält den gehaltenen Text eines abweichenden
   Zettels und verwirft das frisch Gelesene; nur wo nichts abweicht, wird das Gelesene
   beides. Damit sind das Neuöffnen und der Tabwechsel, die zwei Wege des ersten
   Datensatzes, mit einer Regel geschlossen und nicht mit zweien.
2. **Der Rückgabewert trägt die Regel zu den Aufrufern.** `oeffnen` liefert den Text, der
   in die Textfläche gehört, mit `#[must_use]` und der geschriebenen Begründung: wer ihn
   fallenließe, setzte das Gelesene in die Fläche und hätte denselben Verlust wieder. Beide
   Aufrufer nehmen ihn, `notizzettel_zeigen` und `zettel_wechseln`
   (`crates/krk-ui/src/appkit/anwendung.rs`).
3. **Jeder Sicherungsmoment schreibt jeden abweichenden Zettel.** `zu_sichern` liefert eine
   Aufzählung statt des ersten Treffers, `etwas_zu_sichern` beantwortet daneben die Frage
   „gibt es überhaupt etwas" und ist aus derselben Aufzählung abgeleitet.
   `zettel_sichern` läuft über die Liste. Ein Fehlschlag bricht nicht ab; in die
   Statuszeile geht der erste Grund.

**Was unangetastet geblieben ist**, wie der Auftrag es verlangt: `immer_erreichbar`,
`waehrend_blatt_erlaubt`, `zulaessigkeit::zulaessig`, die eine Erklärung des Sicherns mit
ihren vier Aufrufern und der eine `durchgang` in `applicationWillTerminate:`. Die vier
Zählproben aus Schritt 14 sind ohne Anpassung grün geblieben, und das ist der Beleg dafür.

**Die fehlende Meldung beim Beenden ist nicht gebaut, und der Grund steht jetzt im Code.**
Der Kommentar am `let _ =` in `applicationWillTerminate:` nennt beide Hälften: dass es dort
keine Statuszeile mehr gibt, und dass der Nutzer von einem Fehlschlag deshalb nichts
erfährt — der benannte und angenommene Preis aus C4, mit der Alternative unter
„Ausdrücklich außerhalb dieser Runde".

## Drei neue Proben, alle am Modell

Beide Abnahmekriterien des Nachtrags sagen „ohne Fenster am Modell prüfbar", und dort
stehen sie (`crates/krk-ui/src/zettelmodell.rs`, Prüfmodul):

- `das_oeffnen_setzt_den_abweichenden_stand_nicht_zurueck`
- `ein_sauberer_zettel_bekommt_den_neuen_dateiinhalt` — die Gegenprobe, damit die
  Einschränkung des Neulesens nicht als Streichung gelesen wird
- `jeder_abweichende_zettel_steht_zur_sicherung_an`

Die gescheiterte Sicherung bilden alle drei dadurch ab, dass `gesichert` gerade **nicht**
gerufen wird.

## Der Plan ist an sechs Stellen nachgezogen

`planning/260814-0656_o_plan-notizzettel-als-blatt-mit-zwei-zetteln.md`, Kopfnotiz vom
260814-0941: die Schritte 10 bis 14, der Kasten `zettel_sichern` im Bild der
Sicherungsmomente und die Risikozeile zu zwei Instanzen. Die Schritte bleiben auf `[DONE]`;
geändert sind ihre Beschreibungen und ihre Abnahmekriterien, nicht ihr Stand.

Daneben eine mechanische Berichtigung im Spec: vier Verweise auf die beiden Defektdatensätze
standen mit dem Marker `_o_` und zeigten nach der Schließung auf einen Dateinamen, den es
nicht mehr gibt. Sie tragen jetzt die Sternform, wie die Markerregel es für Dateien
außerhalb von `issues/`, `history/`, `reviews/` und ihren Nachbarn verlangt. Am Wortlaut des
Spec ist nichts geändert.

## Was offen bleibt

- Die fünf niedrigen Befunde der Durchsicht (`issues/260814-0912_o_` bis `260814-0916_o_`)
  und die zwei mittleren, die nicht zu dieser Sicherungslage gehören
  (`260814-0910_o_`, `260814-0911_o_`).
- Die zweiten Kriterienlisten aller fünf Fähigkeiten. Sie verlangen KRK im Vordergrund und
  sind Nutzerarbeit; kein Agent kann sie fahren. Der Weg der gescheiterten Sicherung ist
  darin mit drei Kriterien vertreten, und sie sind mit diesem Bau erst baubar geworden.
- Kein `make bundle` und kein `cargo xtask` gefahren, wie der Auftrag es verlangt.
