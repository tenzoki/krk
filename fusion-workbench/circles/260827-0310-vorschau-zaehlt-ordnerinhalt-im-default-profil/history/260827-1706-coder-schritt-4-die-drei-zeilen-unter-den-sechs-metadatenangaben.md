# Coder: Schritt 4 der Runde 19 — die drei Zeilen treten unter die sechs Metadatenangaben

**Datum:** 260827-1706
**Plan:** `planning/260827-1322_p_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`, Schritt 4
**Kriterien:** C1.6, C1.8, C2.1, C2.2, C2.12, C4.6
**Status:** Complete

## Was gebaut ist

- `crates/krk-ui/src/vorschaumodell.rs`: `Inhalt::Metadaten` ist der Strukturwert `{ metadaten: Metadaten, zaehlzeilen: Vec<Zusammenfassungszeile> }`; der Doc-Kommentar sagt, warum kein achter Wert von `Inhalt` entsteht. `laden` verzweigt im Zweig „kein Dateityp" vollständig über `Some(Erkannt)` → `Inhalt::Zusammenfassung`, `Some(Default(zeilen))` → `Metadaten` mit den Zeilen, `None` → `Metadaten` mit leerer Folge; der vorläufige Kommentar aus Schritt 3 ist weg. Die drei Erzeuger im Dateizweig (Bildfehler, Textgrenze/kein UTF-8) übergeben `Vec::new()` (C1.6). Alle Musterstellen (`zeigt_dateitext`, Proben) ziehen auf `Inhalt::Metadaten { metadaten, .. }` bzw. `{ .. }` mit. Modulkopf „Die Zusammenfassung ist der vierte Weg": neuer Absatz zu den zwei Antworten für einen Ordner ohne Profiltreffer — ersetzen gegen unter-treten; der Satz zu C2.5 ist aus dem alten Absatz herausgenommen und dort in der neuen Form wiederholt.
- `crates/krk-ui/src/appkit/vorschau.rs`: `metadaten_text(&self, &Metadaten, &[Zusammenfassungszeile])` hängt `zeilen_als_text` hinter „Typ" an (C2.1, C2.2); der Anzeigezweig reicht die Zeilen durch, der Bildrückfall (`bild_zeigen`) übergibt `&[]`. Doc-Kommentar sagt, warum die Zeilen in der Ansicht und nicht im Kern entstehen. Import `{Profile, Zusammenfassungszeile, zeilen_als_text}`; Musterstellen in `einzufaerben` und der Probe nachgezogen.
- `crates/krk-ui/src/appkit/anwendung.rs`: Doc-Kommentar von `sitzung_laden` berichtigt. Der Satz „keine der zehn Zeitzusagen misst an einer Zusammenfassung" ist gestrichen; der neue Absatz sagt, dass das Default-Profil nicht aus der Ablage kommt und im Messmodus für jeden Ordner des Messplatzes eintritt, und nennt den Datensatz `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/decisions/260827-1322_o_faellt-das-default-profil-auch-im-messmodus-an-und-was-misst-l7-danach.md`.

C1.8 (Zahlen wandern beim Betreten eines Unterordners ohne Auswahl mit), C2.12 (Zeilen bleiben über einen Tabwechsel stehen) und C4.6 (Auswählbarkeit) fallen aus dem Weg heraus, den die Zeilen nehmen: sie reisen im `Inhalt` des Tabs und werden über `text_zeigen` gestellt wie die sechs Angaben; kein eigener Mechanismus.

## Nicht gemacht, mit Grund

- Keine neuen Abnahmeproben (drei Zeilen für einen Ordner, keine für eine Verknüpfung): Schritt 5. Geändert sind nur die Proben, die die neue Gestalt zum Übersetzen brauchen.
- Nichts unter `crates/krk-core/` (Schritt 2 läuft parallel), nichts unter `resources/`; Planschritt nicht auf `[DONE]` gesetzt (Dispatch-Verbot für `fusion-workbench/` außer diesem Eintrag).

## Verifikation

`make check` — exit 0, mit dem parallel geänderten `crates/krk-core/src/leseprofil/datei.rs` im Baum. `rustfmt` auf den drei Dateien war nötig, `cargo fmt --check` war davor der einzige rote Schritt.
