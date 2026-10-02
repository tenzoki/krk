# Der Zeichendurchgang während einer offenen Umbenennung: ein Fehlbefund

**Agent:** coder
**Status:** Complete
**Auftrag:** T2, Behebung des Defekts
`shared/issues/260815-2203_*_ein-zeichendurchgang-waehrend-einer-offenen-umbenennung-schreibt-name-schraegstrich-in-den-feldeditor.md`
**Dateigrenze:** voraussichtlich `crates/krk-ui/src/appkit/tabelle.rs`

---

## Ergebnis in einem Satz

Der Defekt besteht nicht. Der Zeichendurchgang, den der Datensatz voraussetzt, findet
nicht statt: AppKit reicht dem Delegierten nie eine Zelle mit offenem Feldeditor. Am Code
ist deshalb nichts geändert außer der Prosa, die diesen Fehlschluss künftig verhindert.

## Die Messung

Am 260816 auf macOS 15.7.7 mit einem weggeworfenen Programm auf dem wirklichen Hauptfaden,
in einer laufenden Ereignisschleife (`NSApplication::run`, Zeitgeber, Schlüsselfenster
nachgewiesen über `isKeyWindow`). Nachgebaut ist die Verdrahtung der Datei:
`NSTableView` in einer `NSScrollView`, 60 Zeilen, Zellenwiederverwendung über
`makeViewWithIdentifier:owner:`, eine Unterklasse von `NSTextField` mit derselben
`becomeFirstResponder`-Überschreibung wie `Namensfeld`, Ziel und Aktion am Feld.

| Anlass | Bearbeitung danach | Durchgang der bearbeiteten Zeile | `currentEditor` darin |
|---|---|---|---|
| `reloadData` | beendet | ja, danach | `None` |
| `reloadDataForRowIndexes:columnIndexes:` | beendet | ja, danach | `None` |
| `noteNumberOfRowsChanged` | steht weiter | keiner | — |
| `selectRowIndexes:byExtendingSelection:` | beendet | keiner | — |
| Bildlauf aus dem Bild und zurück | steht weiter | keiner, die Zeile wird übersprungen | — |
| erstmaliger Aufbau einer Zeile | — | ja | `None` |

**In keinem Durchgang, in keinem Lauf, stand ein Feldeditor.** Beide Wege, die der
Datensatz nennt, laufen über `reloadData`, und `reloadData` beendet die Bearbeitung, bevor
der erste Durchgang läuft. Der Bildlauf — den der Datensatz nicht nennt und der als
einziger die Bearbeitung überleben lässt — hält die bearbeitete Zeile aus dem Durchgang
heraus.

Zwei Gegenproben im selben Lauf, damit die Messung nicht bloß eine kaputte Verdrahtung
misst: `insertNewline:` am Feldeditor schickt die Aktion mit dem getippten Text
(`"Fotos"`), und ein `setStringValue:` unmittelbar während der Bearbeitung schreibt wie
gemeldet in den Feldeditor zurück (Feld und Feldeditor standen danach auf `"Bilder/"`).
Die Beobachtung des Datensatzes war also für sich richtig; falsch war die Folgerung, dass
`zellenansicht` diesen Zustand je erreicht.

**Zwei Kandidaten sind mitgemessen und gefallen.** `NSTableView::editedRow` und
`editedColumn` stehen während einer offenen Bearbeitung dieser Tabelle auf `-1` — sie
gehören der zellenbasierten Tabelle, die hier ist ansichtsbasiert. Der Ersthelfer des
Fensters ist während der Bearbeitung die `NSTextView` des Feldeditors, taugt aber als
Erkennung der *Zelle* nicht, weil er nicht sagt, zu welcher Zeile sie gehört.

## Was geändert wurde

Prosa in `crates/krk-ui/src/appkit/tabelle.rs`, an zwei Stellen, keine Zeile Code:

1. Der Doc-Kommentar von `DateifensterDelegierter::zellenansicht` trägt den Abschnitt
   `# Eine Zeile mit offener Namensbearbeitung kommt hier nicht an`: die Messtabelle, die
   beiden Weisen, auf die AppKit die Zeile heraushält, die Feststellung, dass eine
   `currentEditor`-Abfrage hier toter Code wäre, den gefallenen Kandidaten `editedRow`,
   und diesen Datensatz als den Fehlschluss, der aus dem bisherigen Schweigen entstand.
2. Der Kopf von `Namensfeld::wird_ersthelfer` trägt den Rückverweis. Dort steht die
   richtige Hälfte der Beobachtung (`setStringValue:` schreibt während der Bearbeitung in
   den Feldeditor zurück), und wer sie ohne die andere Hälfte liest, kommt zu demselben
   Fehlbefund.

**Das ist die Behebung im Sinne des Vorbilds `260810-1102`:** dort wie hier war der
eigentliche Defekt das Schweigen der Datei über die zweite Frage, nicht das Verhalten.

## Was ausdrücklich nicht geändert ist

- **Keine Fallunterscheidung in `zellenansicht`.** Ein Schutz, der messbar nie greift,
  wäre toter Code und trüge eine Behauptung über AppKit, die die Messung widerlegt.
- **Farbe und Schrift.** Der Auftrag fragte, ob sie dasselbe Problem haben. Sie haben
  keines: sie laufen in demselben Durchgang, den es während einer Bearbeitung nicht gibt.
  Eine zweite Fallunterscheidung daneben ist damit gegenstandslos.
- **Der Satz „genau diese Schleife messen L3 und L10"** an `namensform` und im Modulkopf,
  wie beauftragt unberührt; er gehört zu `260815-2202`.
- **Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen`** braucht keine
  Zeile: es ist keine AppKit-Methode dazugekommen. `currentEditor` und `editedRow` sind
  gemessen und wieder verworfen, nicht eingebaut.

## Keine Probe

Die Zusage ist eine über AppKit. Sie braucht ein Fenster und einen Feldeditor, `NSWindow`
wirft außerhalb des Hauptfadens, und `libtest` gibt ihn nicht her
(`issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`).
Eine reine Regel, die neu zu prüfen wäre, ist nicht entstanden; `namensform` und
`ohne_ordnerzeichen` stehen unverändert in ihren Proben.

## Datensätze

- `shared/issues/260815-2203` — `Resolved:` angehängt, Marker `_o_` → `_c_`.
- `shared/issues/260815-2125` — Nachtrag angehängt, bleibt offen. Die Messung hat dem
  dritten Ausgang zwei Anlässe hinzugefügt, die ohne Zutun des Nutzers auslösen:
  `reloadData` und `reloadDataForRowIndexes:columnIndexes:` beenden eine offene
  Bearbeitung, **ohne die Aktion zu schicken**. Der getippte Text ist damit fort, und
  umbenannt wird nichts. Rufer sind `nach_lesebeginn` (Navigation und Dateisystemwache)
  und `einziehen` (der Takt des Lesevorgangs). Die offene Nutzerfrage des Datensatzes —
  verwerfen oder übernehmen — trägt diese beiden Anlässe mit.

## Abnahme

```
make check
```

Exit 0, „alle vier gruen": `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --all --check`.
Keine neue Warnung.

## Offen

- **Kein Commit**, wie beauftragt.
- Ob `controlTextDidEndEditing:` bei den beiden Zeichendurchgängen kommt, ist **nicht**
  gemessen: das Messprogramm führte den Delegierten der Tabelle und keinen am Feld, und
  `NSTextField` schickt die Meldung an seinen eigenen Delegierten. Die Datei setzt ihn
  ebenso wenig. Wer die Meldung als Aufhänger einer Behebung von `260815-2125` nimmt,
  misst das zuerst.
