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

---

## Nutzerentscheid vom 260806-2300: ein fuenfter Weg

Der Nutzer hat einen Weg gewaehlt, den die Aufstellung oben nicht fuehrt:
**die Ueberschrift bleibt "Typ", und die Zelle zeigt die Endung.**

Vorgelegt wurde er, weil drei am Code geprueste Befunde die Wege 1 bis 3 anders
gewichten, als der Bericht oben es tat:

- **Ordner stehen in jeder der acht Sortierungen vorn** (`gruppe` in
  `crates/krk-core/src/verzeichnis/sortierung.rs:148`). Ob ein Eintrag Ordner
  oder Datei ist, bleibt damit an der Position ablesbar, auch wenn die Spalte es
  nicht mehr schreibt. Der Verlust der Anzeige trifft allein die
  **Verknuepfung**, die mit den Dateien gruppiert.
- **Die Tastenfunktion heisst "Nach Typ sortieren"** (`resources/default-keymap.toml:267`,
  Cmd+4). Eine Spalte namens "Endung" verschoebe den Namensbruch dorthin, statt
  ihn aufzuloesen, und der Nachzug waere eine Aenderung an einer `.toml`, also
  eine zweite Aufgabe fuer den `ontocoder`.
- **Der Spec schreibt die Ueberschrift nirgends fest.** C1 zaehlt die Spalten
  nicht auf, und C2 verlangt allein, dass es bei vier bleibt. Ein Wechsel des
  Zelleninhalts kostet damit keinen Spec-Nachzug, ein Wechsel der Ueberschrift
  schon.

Der gewaehlte Weg beruehrt eine einzige Codestelle, laesst `default-keymap.toml`
unangetastet, braucht keine breitere Spalte und haelt Spalte, Tastenfunktion und
Spec-Wortlaut auf demselben Wort. Zugesagt ist damit: "Typ" heisst in KRK die
Dateiendung, und die Spalte sagt das jetzt auch.

**Was der `coder` umsetzt:**

- `crates/krk-ui/src/appkit/tabelle.rs:1804` und `:1987` — die Zelle der Spalte
  `Typ` zeigt `eintrag.endung()` statt `typ_beschriften(eintrag.typ)`. Bei einem
  Eintrag ohne Endung bleibt die Zelle leer.
- Die Ueberschrift `:170` und die Breite `:180` bleiben unveraendert; eine
  Endung braucht weniger Platz als "Verknüpfung", nicht mehr.
- `typ_beschriften` bleibt bestehen: die Metadatenanzeige der Vorschau (C6)
  ruft sie ueber `crates/krk-ui/src/appkit/vorschau.rs:458` weiterhin, und dort
  ist die Eintragsart nach wie vor zugesagt.

---
Resolved: Die Zelle der Spalte `Typ` zeigt jetzt `eintrag.endung()` statt der
Eintragsart (`crates/krk-ui/src/appkit/tabelle.rs:1816`). Ueberschrift (`:178`)
und Breite (`:188`) bleiben unveraendert, `typ_beschriften` (`:2000`) bleibt
fuer die Metadatenanzeige der Vorschau (C6) bestehen und ist jetzt deren
einziger Aufrufer. `make check` gruen, 497 Pruefungen.
Bericht: `history/260806-2330-coder-die-spalte-typ-zeigt-die-endung.md`.
