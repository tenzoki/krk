# Wie erreicht ein Baustein die Einträge mehrerer gleichartiger Unterordner?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Answered:** 260825-1740, Kai Stalmann — Moeglichkeit 1: die Ortsangabe traegt einen Platzhalter, kein fuenfter Baustein. Empfehlung des Planers ohne Aenderung uebernommen.
**Cross-references:** `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`; `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_*_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`; `crates/krk-core/src/leseprofil/mod.rs` (`Ortsangabe`, `Baustein`); `resources/default-readers.toml`

---

## Question

Zwei Auskünfte, die der Nutzer am 260825 für das Verzeichnis `fusion-workbench/circles`
verlangt, liegen nicht in diesem Ordner, sondern je eine Ebene tiefer in **jedem** seiner
Unterordner: der Zustandsmarker einer Runde steht im Dateinamen `_X_circle.md` innerhalb des
Rundenordners, und die offenen Defekte einer Runde stehen in `<runde>/issues/*_o_*.md`.

Die vier Bausteine der Runde 16 erreichen beides nicht. `Baustein::Zaehlung` läuft flach über
eine Ebene (Festlegung A2, C3.2), und `Ortsangabe` nimmt allein gewöhnliche
Namensbestandteile entgegen: `Ortsangabe::aus_angabe` weist einen absoluten Pfad, ein leeres
Stück und `.` wie `..` ab, kennt aber keine Stelle, an der ein Name offen bleibt. Ein Ordner
mit gleichartigen Unterordnern ist damit als Ort nicht benennbar.

Die Frage muss jetzt beantwortet werden, weil beide Auskünfte in derselben Runde entstehen und
weil die Antwort das Zusagegefüge aus C6 berührt: eine Zusammenfassung darf höchstens zwölf
Verzeichnisleseläufe kosten (`HOECHSTENS_LESELAEUFE`), und die Werkbank führt heute 19 Runden.
Eine Antwort, die je Runde einen Leselauf bucht, kostet für die zwei Zeilen zusammen 39 Läufe
und sprengt die Zusage schon am heutigen Bestand.

## Options

1. **Ein Platzhalter in der Ortsangabe, und ein Wildcard-Lauf zählt als ein Leselauf.**
   `ordner = "*"` und `ordner = "*/issues"`. Der Lauf liest den übergeordneten Ordner, steigt in
   jeden Unterordner vom Typ Ordner ab und legt deren Einträge zu **einem** `Lesestand`
   zusammen. Gegen den Haushalt bucht er einen Leselauf wie jede andere Ortsangabe; begrenzt
   wird er durch `HOECHSTENS_EINTRAEGE`, also 2.000 Einträge je Lauf, wie jede andere Lesung
   auch.
   - Pros: Kein fünfter Baustein, also keine Erweiterung der vollständigen Fallunterscheidung
     und kein Nachziehen an den sieben Stellen, die sie halten. `muster` behält in allen vier
     Bausteinen dieselbe Bedeutung, nämlich ein Muster auf dem Eintragsnamen. Die
     Abbruchvokabeln der Runde 16 tragen unverändert: eine abgeschnittene Lesung liefert
     `Wert::UeberGrenze` bei der Zählung und den Platzhalter bei den jüngsten N. Die Zusage
     `HOECHSTENS_LESELAEUFE = 12` bleibt stehen, und die zwei verlangten Zeilen kosten
     zusammen zwei Läufe statt 39. Die Eintragsschranke begrenzt die Arbeit weiter genau so,
     wie sie es bisher tat: zwölf Läufe zu je 2.000 Einträgen.
   - Cons: Ein Leselauf öffnet nicht mehr genau ein Verzeichnis, sondern eines je Treffer. Die
     Zahl der Systemaufrufe ist damit nicht mehr aus der Zahl der Läufe abzulesen, sondern erst
     aus Lauf und Bestand. Ein Ordner, der `*` heißt, ist als Ort nicht mehr benennbar. Die
     zwei Bausteine, die Dateien lesen — `Juengste` und `Feld` —, können den Platzhalter nicht
     annehmen, weil die zusammengelegten Einträge aus verschiedenen Ordnern stammen und ein
     einzelner Ordnerpfad sie nicht mehr auffindet.

2. **`zaehlung` bekommt eine Tiefenangabe.** `zaehlung = { tiefe = 2, muster = … }`, und
   `muster` läuft ab einer Tiefe über 1 gegen den Pfad relativ zum Ort statt gegen den Namen.
   - Pros: Beide verlangten Fälle sind ausdrückbar, ohne dass die Ortsangabe eine neue Form
     bekommt.
   - Cons: `muster` bedeutet dann in `zaehlung` etwas anderes als in den drei übrigen
     Bausteinen, und der Unterschied hängt am Wert eines zweiten Schlüssels. Ein Abstieg über
     eine Tiefe besucht **jeden** Unterordner, also für `circles` auch `planning`, `history`
     und `reviews` jeder Runde: 1.375 Einträge statt der 568, die `*/issues` allein trifft, und
     der Weg dorthin ist nicht zu beschneiden, weil aus dem Text eines Ausdrucks nicht
     hervorgeht, welcher Zweig noch treffen kann. Die Antwort auf die zweite Frage des Nutzers
     wäre damit doppelt so teuer wie nötig und rückte an die Eintragsschranke heran.

3. **Ein fünfter Baustein.** Ein `unterordner`-Baustein, der je Unterordner zählt.
   - Pros: Die vorhandenen vier bleiben unberührt.
   - Cons: Festlegung A7 der Runde 16 hält die Zahl vier ausdrücklich fest, und `Baustein` ist
     eine vollständige Fallunterscheidung ohne Auffangzweig: ein fünfter Wert hält den Bau an
     `Zeilendatei`, `Bausteindatei`, `Zeilendatei::zerlegen`, `BAUSTEINNAMEN`,
     `baustein_pruefen`, `Lauf::rechnen` und der Auslieferungsfassung an. Das ist der Preis
     dafür, dass jede Stelle bewusst nachgezogen wird — hier aber ohne Gegenwert, denn der neue
     Baustein täte dasselbe wie `zaehlung` an einem anderen Ort, und ein Ort ist genau das,
     was `Ortsangabe` beschreibt.

## Constraints

- Die Zusage aus C6.4 (`HOECHSTENS_LESELAEUFE = 12`) darf nicht steigen, ohne dass gesagt ist,
  wovor sie schützt. Jede feste Zahl, die mit der Zahl der Runden wachsen müsste, ist auf
  Sicht wieder falsch: heute reichten 39, bei hundert Runden 201.
- C3.13 bleibt: eine Zusammenfassung liest nie außerhalb des Ordners, über den sie spricht.
  Ein Platzhalter darf diese Schranke nicht öffnen.
- Ein Muster in `readers.toml` darf die Vorschau nicht anhalten können (C2.8).
- Die drei Regeln zur unvollständigen Lesung aus dem Modulkopf von `leseprofil::bausteine`
  gelten weiter: es wird nur gesagt, was die Teillesung entscheidet.

## Recommendation

**Möglichkeit 1.** Sie beantwortet beide Fälle mit **einem** Mechanismus, wie der Auftrag es
verlangt, und sie tut es an der Stelle, an der die Frage entsteht: nicht der Baustein wird
erweitert, sondern der Ort, an dem er arbeitet. Die Bedeutung von `muster` bleibt in allen vier
Bausteinen dieselbe, und der Bausteinsatz bleibt bei vier.

Drei Festlegungen gehören zur Empfehlung dazu, weil ohne sie die Kostenaussage nicht hält:

- **Höchstens ein Platzhalter je Ortsangabe.** Damit ist die Form der Kosten aus dem Profil
  ablesbar: ein Lauf über den übergeordneten Ordner, dann einer je Treffer. Ein zweiter
  Platzhalter vervielfachte sie. Zwei oder mehr werden beim Laden abgewiesen, und die Zeile
  behält ihre Beschriftung und verliert ihren Baustein — die dritte Reichweite der Prüfung.
- **Der Platzhalter greift allein Einträge vom Typ Ordner und folgt keiner Verknüpfung.**
  Damit hält C3.13 durch Bauart statt durch Prüfung: ein wirklicher Unterordner eines Ordners
  innerhalb der Schranke liegt innerhalb der Schranke. Der Rest der Ortsangabe hinter dem
  Platzhalter wird weiter aufgelöst und geprüft wie heute. Es ist derselbe Grund, aus dem der
  Durchlauf aus `verzeichnis/durchlauf.rs` nicht in eine Verknüpfung absteigt.
- **`juengste` und `feld` nehmen keine Ortsangabe mit Platzhalter an**, und die Grenze liegt
  auf einer Naht, die der Modulkopf von `leseprofil::bausteine` schon zieht: zwei Bausteine
  sehen auf Namen, zwei lesen Dateien. Wer eine Datei liest, braucht ihren Pfad, und den trägt
  ein zusammengelegter Lesestand nicht. Abgewiesen wird beim Laden mit Meldung, die Zeile
  behält ihre Beschriftung.

Gemessen am 260825 an der Werkbank dieses Vorhabens: `*` legt rund 135 Einträge aus 19 Ordnern
zusammen, `*/issues` 568 aus 19 Ordnern. Beides liegt weit unter der Eintragsschranke von
2.000. Bei hundert Runden erreicht `*/issues` sie, und dann greift `Wert::UeberGrenze` mit dem
Satz, den die Runde 16 dafür geschrieben hat: mindestens so viele, und die Lesung wurde
abgebrochen.

---
Implemented: 3cadb45 — `Ortsangabe::aus_angabe` nimmt genau ein Stück `*` an und weist zwei oder mehr ab (`crates/krk-core/src/leseprofil/mod.rs:456`, `:467`); `Ortsmangel` trägt dafür den vierten Wert `MehrerePlatzhalter` (`:507`) und bleibt vollständig ohne Auffangzweig. Der Platzhalter greift allein Einträge vom Typ Ordner und folgt keiner Verknüpfung (`:405`), `juengste` und `feld` nehmen ihn nicht an (`:412`). Nachgemessen am 260826-0149 gegen den Baum, `make check` grün.
