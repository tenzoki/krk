# Coder: Drei Befunde der zweiten Durchsicht

**Datum:** 2026-08-25 12:19
**Status:** Complete
**Agent:** coder
**Baumstand:** `e3478e6` plus die Änderungen dieses Schritts

## Auftrag

Die drei offenen Befunde der zweiten Durchsicht dieser Runde
(`reviews/260825-1144-coderev-runde-17-zweite-durchsicht-die-kette-vom-klick-bis-zur-wirkung.md`),
je ein Datensatz unter `issues/` mit dem Stempel `260825-1144`. Alle drei sind behoben und ihre
Datensätze auf `_c_` gesetzt.

## Befund 1: das Ziel eines Laufs kann eine seiner Quellen treffen

Der Nutzer hat den zweiten und kleineren der zwei Vorschläge gewählt: **die Oberfläche legt das
Ziel nicht auf eine Quelle**, der Kern bekommt ausdrücklich keinen Pfadvergleich.

Die Regel steht als `ist_ziel_des_laufs` **einmal** in
`crates/krk-ui/src/kommandos/kontextmenue.rs` und hat zwei Rufer, einen je Gestalt:

- **Packen.** Neu ist `packziel(betroffen, ordner) -> (Vec<PathBuf>, PathBuf)`: es rechnet über
  `archivname` das Ziel und gibt die Quellen ohne den Eintrag heraus, dessen Pfad ihm gleicht. Der
  Name wird aus der **ungefilterten** Markierung gerechnet und nach dem Schnitt nicht neu: sonst
  hieße das Archiv beim zweiten Lauf `a.txt.zip` statt wieder `Projekte.zip`, und die Zusage von
  `archivname` hätte zwei Fassungen.
- **Entpacken.** `entpackziel` führt seine Paare durch `ohne_die_eigenen_ziele`. Damit fällt ein
  Archiv heraus, das derselbe Lauf schon als Zielordner beansprucht — die Lage, die die
  anhängende Endungsregel selbst herstellt, indem sie `a.zip.zip` neben `a.zip` legt.

`zipauftrag_stellen` (`crates/krk-ui/src/appkit/anwendung.rs`) nimmt Quellen und Ziel in einem Zug
von `packziel`, zählt die Positionen aus den verbliebenen Quellen und stellt die Frage „gibt es
etwas zu packen" seither **hinter** der Zielklärung statt davor. Sie steht damit einmal statt
zweimal und deckt beide Fälle: die leere Markierung und die leer geschnittene. Leer geschnitten
werden kann sie nicht — bei einem einzelnen Eintrag ist das Ziel durch die angehängte Endung ein
anderer Name, bei mehreren trägt es den Ordnernamen, den höchstens einer der Einträge trägt —, und
die Frage steht trotzdem: eine Zusage, die nur ein Beweis in Prosa hält, gehört nicht zwischen den
Nutzer und einen leeren Auftrag.

**Drei Proben**, alle in `kontextmenue.rs`:

| Probe | hält |
|---|---|
| `das_archiv_des_vorigen_laufs_faellt_aus_den_quellen` | der zweite Zip-Lauf über denselben Ordner |
| `ein_archiv_das_zielordner_eines_anderen_ist_faellt_aus_den_quellen` | die Entpack-Gestalt |
| `ein_einzelnes_archiv_bleibt_seine_eigene_quelle` | dass der Schnitt nicht zu weit greift |

Gegenprobe gefahren: mit ausgeschaltetem `ist_ziel_des_laufs` werden die ersten zwei rot, die
dritte bleibt grün.

Der Modulkopf von `crates/krk-core/src/operation/zippen.rs` ist nachgezogen. Das Argument „keine
Löschstelle nennt `auftrag.quellen`" steht dort jetzt als das, was es ist: eine Aussage über den
Quelltext und keine über Pfadwerte. Die Zusage hängt ausgeschrieben am Rufer, mit dem Namen der
Funktion und dem Namen ihrer Probe, und der Kopf sagt daneben, was geschieht, wenn der Zielpfad
doch einmal auf der Quellenliste steht. `crates/krk-core/tests/operation.rs` ist unverändert: unter
dem gewählten Weg gibt es im Kern nichts Neues zu prüfen.

## Befund 2: die Probe prüfte Vorhandensein statt Paarung

`jeder_kontextbefehl_erreicht_seine_wirkung` (`appkit/anwendung.rs`, Prüfmodul `kontextproben`)
zählt jetzt zeilenweise: die Zeilen des Rumpfes von `kontextbefehl_ausfuehren`, die Befehlsnamen
und Zweignamen zugleich tragen, müssen genau eine sein. Der Doc-Kommentar sagt aus, dass die
Paarung gehalten wird, und benennt weiterhin, was die Zählung nicht sieht.

Gegenprobe gefahren: mit vertauschten Zweigen (Zippen auf `entpackauftrag_stellen`, Entpacken auf
`zipauftrag_stellen`) fällt die Zahl auf 0 und die Probe wird rot. Die Vertauschung ist
zurückgenommen.

## Befund 3: doppeltes Kommentarzeichen

Das zweite `//!` in Zeile 199 von `crates/krk-ui/src/appkit/tabelle.rs` ist gestrichen.
`grep -rn '^\s*//!\s*//!' crates/` gibt danach nichts mehr aus.

## Abnahme

`make check` — Exit 0 (Bau, `cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`). Die drei neuen Proben laufen grün, die
zwei Gegenproben rot und nach der Rücknahme wieder grün.

**Nicht gefahren:** der Abnahmelauf am gebauten Bündel. Er verlangt KRK im Vordergrund und ist
Nutzerarbeit.

## Angefasste Dateien

- `crates/krk-ui/src/kommandos/kontextmenue.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-core/src/operation/zippen.rs` (nur Modulkopf)
- die drei Datensätze unter `issues/`, je mit `Resolved:` und auf `_c_` gesetzt
