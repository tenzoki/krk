# Bugfix: Sichern auf einem Netzlaufwerk schlägt still fehl

**Datum:** 2026-09-04 19:05
**Status:** Complete (Teil 2 behoben; Teil 1 gemessen und nicht reproduzierbar)
**Auslöser:** Nutzerbefund, `shared/issues/260904-1827_o_sichern-auf-einem-netzlaufwerk-schlaegt-still-fehl-die-datei-bleibt-ungesichert.md`
**Kein Circle aktiv.**

## Der Fehler

Eine Datei auf Google Drive im eingebauten Editor geändert und gesichert: keine
Fehlermeldung, der Editor führt die Datei weiter als ungesichert, der Stand steht
nicht auf der Platte.

## Was gemessen wurde

Alle Läufe am 260904 auf dem Referenzgerät, gegen
`~/Library/CloudStorage/GoogleDrive-kai@qantr.com` (FileProvider-Einhängung,
`mount` führt sie nicht, `stat -f %T` liefert `/`).

**1. Der ganze Schreibweg, dreimal, mit eigenen Wegwerfdateien.** Ein
Wegwerf-Beispielprogramm in `krk-core` hat `nachbarpfad`, `File::create`,
`rename(2)` und `krk_core::text::datei::sichern` einzeln gefahren:

| Ort | `File::create` Nachbar | `rename(2)` | `datei::sichern` | Inhalt danach |
|---|---|---|---|---|
| `Meine Ablage/krk-wegwerf-260904/probe.txt` (neu) | ok | ok | ok | neu, richtig |
| dieselbe Datei nach Drives Upload | ok | ok | ok | neu, richtig |
| `Meine Ablage/qa/Buchhaltung/Fakura/Invoices/sheet/` (der Ordner des Nutzerfalls) | ok | ok | ok | neu, richtig |

**Kein `EXDEV`, kein `EACCES`, kein `errno` überhaupt.** Der Umweg über die
Nachbardatei ist auf dieser Einhängung unteilbar durchführbar; die Datei stand nach
15 Minuten unverändert mit dem neuen Inhalt da, Drive hat nichts zurückgerollt.

**2. Der Stempelvergleich gegen die Materialisierung.** Jede Textdatei im Ordner des
Nutzerfalls trägt `compressed,dataless`, ist also nicht lokal materialisiert.
`Ladevorgang::starten` erhebt den `Stempel` **vor** dem Lesen, und das Lesen
materialisiert. Gemessen an einer 127-Byte-Vorlage, nur lesend:

```
vor  = Some((SystemTime { tv_sec: 1781114281, tv_nsec: 26999950 }, 127))
gelesen: 127 Bytes
nach = Some((SystemTime { tv_sec: 1781114281, tv_nsec: 26999950 }, 127))
gleich = true
```

Die Materialisierung rührt weder `mtime` noch Größe an; `fremd_geaendert()` schlägt
dadurch nicht falsch an. Über 24 s Leerlauf blieb `mtime` einer eben geschriebenen
Drive-Datei ebenfalls unverändert.

**3. Die Bindung und die Signatur.** `cmd+s` steht in der Nutzerbelegung
(`~/Library/Application Support/KRK/keymap.toml:374`) auf `editor_sichern`. Das
Bündel ist ohne Sandbox und ohne Entitlements signiert (`codesign -d`), das
Systemprotokoll zeigt für den laufenden `krk`-Prozess keine TCC- oder
Zugriffsverweigerung.

## Was der Wortlaut daraus folgen lässt

`Editormodell::sichern` hat drei Ausgänge, alle drei enden in der Statuszeile:
`Gesichert` und `Gescheitert` über `editormeldung_zeigen`
(`crates/krk-ui/src/appkit/anwendung.rs:7628-7645`), `NichtsGehalten` über
`antwort_zeigen`. Die Befehlsantwort trägt Rang 1 von sieben
(`crates/krk-ui/src/appkit/statuszeile.rs:280`), wird erst vom **nächsten**
Tastenbefehl gelöscht, und das aktive Dateifenster ist immer sichtbar. **Ein
gescheitertes Sichern kann in diesem Baum nicht stumm bleiben.**

Der beobachtete Ausgang — kein Wort, kein Schreiben — ist deshalb mit keinem der drei
Ausgänge vereinbar. Er ist nur damit vereinbar, dass `sichern` gar nicht gelaufen ist.
Die eine Stelle, an der ein Sicherungsversuch ohne ein Wort endet, ist der
Zulässigkeitsvorbehalt am Kopf von `Anwendungsdelegierter::kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:3398-3401`): `return false`, keine Meldung.
Welcher seiner vier Bestandteile abgewiesen hat, ist aus dem Befund nicht zu
entscheiden und braucht einen Lauf mit Spur.

## Die Behebung

Teil 1 (das Schreiben scheitert) ist **nicht reproduzierbar** und deshalb nicht
behoben; ein Rückfall auf direktes Schreiben käme ohnehin nicht in Frage, weil die
Messung gerade zeigt, dass der atomare Weg hier trägt.

Teil 2 ist als **Vorsorge gegen das stille Fallenlassen** behoben. Die Zusage
„Der Grund gehört in die Statuszeile" hing bis heute allein an der Prosa eines
Doc-Kommentars; ein nackter Ruf an `sichern()` übersetzte grün.

| Datei | Änderung |
|---|---|
| `crates/krk-ui/src/editormodell.rs:551` | `#[must_use]` an `Sicherungsausgang`, mit dem Grund im Doc-Kommentar |
| `crates/krk-ui/src/appkit/anwendung.rs:7624` | `#[must_use]` an `editor_stand_sichern` |
| `crates/krk-ui/src/appkit/anwendung.rs:7607` | `let _ =` am einen Rufer, der den Ausgang bewusst nicht braucht |

## Verification

- [x] `make check` grün, Exit 0
- [x] Kein Rufer musste umgebaut werden; die zwei Attribute halten den Bau nur für
      künftige Rufer an
- [ ] Der ursprüngliche Fehler ist **nicht** behoben, weil er nicht reproduziert
      werden konnte

## Nebenbefunde

- `shared/issues/260904-1902_o_das-atomare-schreiben-weitet-die-rechte-einer-600-datei-auf-644.md`
  — gemessen, nicht behoben, verlangt eine Entscheidung.
