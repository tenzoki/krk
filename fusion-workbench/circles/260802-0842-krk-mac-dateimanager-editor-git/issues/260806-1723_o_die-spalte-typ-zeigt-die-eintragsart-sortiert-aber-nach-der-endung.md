Die Spalte "Typ" zeigt die Eintragsart, sortiert aber seit der Umstellung nach der Endung

---

Der Nutzerentscheid vom 260806 zu
`decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md` legt fest,
dass die Sortierung nach Typ nach der **Dateiendung** ordnet. Das ist umgesetzt.
Die Spalte, ueber die der Nutzer diese Sortierung ausloest, zeigt aber weiterhin
etwas anderes an.

## Was auseinanderlaeuft

Nachgesehen am Code nach der Umstellung:

- `crates/krk-ui/src/appkit/tabelle.rs:170` — die Spalte traegt die Ueberschrift
  "Typ".
- `crates/krk-ui/src/appkit/tabelle.rs:1804` und `:1987` — ihre Zellen zeigen
  `typ_beschriften(eintrag.typ)`, also "Ordner", "Datei" oder "Verknüpfung".
- `crates/krk-core/src/verzeichnis/sortierung.rs:135` — ein Klick auf ihren Kopf
  ordnet jetzt nach `endungsschluessel`, also nach der Dateiendung.

Der Nutzer klickt damit auf eine Spalte, in der drei Werte stehen, und bekommt
eine Ordnung nach einem vierten, den die Spalte nicht zeigt. Zwei Dateien
untereinander tragen beide "Datei" und stehen dennoch weit auseinander, weil die
eine `.md` und die andere `.zip` heisst. Vor der Umstellung stimmten Anzeige und
Ordnung ueberein; jetzt nicht mehr.

## Warum es hier steht und nicht im Code

Der `coder` hat die Anzeige **nicht** angefasst. Was eine Spalte zeigt, ist eine
sichtbare Eigenschaft der Anwendung, und der Entscheid vom 260806 trifft sie
nicht: er spricht ueber den Schluessel der Sortierung, nicht ueber die Spalte.
Sie eigenmaechtig umzustellen hiesse, eine zweite Nutzerentscheidung im
Vorbeigehen zu treffen.

## Die Wege

1. **Die Spalte zeigt die Endung.** Ueberschrift "Endung", Zelleninhalt
   `eintrag.endung()` — das Feld liegt bereits vor. Anzeige und Ordnung stimmen
   wieder ueberein. Kosten: die Eintragsart ist dann in der Tabelle nicht mehr
   abzulesen; sie steht weiterhin in der Metadatenanzeige der Vorschau (C6, ueber
   dieselbe Funktion `typ_beschriften`).
2. **Die Spalte zeigt beides.** "Datei (zip)", "Ordner". Kosten: eine breitere
   Spalte und eine zweite Wortbildungsregel.
3. **Es bleibt, wie es ist.** Kosten: der beschriebene Bruch zwischen dem, was
   die Spalte zeigt, und dem, wonach sie ordnet.

Der Finder geht Weg 1 in der Sache und nennt seine Spalte "Art": er zeigt dort
den Dokumenttyp ("PNG-Bild", "Ordner"), nicht die Endung, und ordnet auch danach.
Das waere ein vierter Weg, verlangte aber eine Zuordnung von Endung zu
Dokumenttyp, die KRK nicht hat.

**Zustaendig:** ein Nutzerentscheid, danach `coder`.

**Aufgefallen bei:** der Umsetzung des Entscheids vom 260806
(`history/260806-1723-sprachsensitive-kollation-und-endung.md`).
