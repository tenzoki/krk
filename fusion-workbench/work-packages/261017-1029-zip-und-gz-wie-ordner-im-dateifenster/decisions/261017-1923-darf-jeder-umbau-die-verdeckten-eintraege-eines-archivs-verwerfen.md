# Darf jeder Umbau die verdeckten Einträge eines Archivs verwerfen, die der Nutzer nie gesehen und nie angefasst hat?

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261017-1923-reviewer-archivpaket-f52b94d0-bis-8c8579af.md`, `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`, `261017-1036-wird-ein-archiv-ein-dritter-fall-von-ort-und-schreibt-krk-hinein-ueber-einen-umbau-des-ganzen-archivs.md`

---

## Question

Das Inhaltsverzeichnis zeigt einen Teil der Einträge eines Archivs nicht:

- frühere Einträge mit doppeltem Namen;
- eine Datei, die einem gleichnamigen Ordner weichen muss;
- jeden Eintrag, dessen Weg durch eine Verknüpfung im selben Archiv führt.

Der Bau verwirft alle diese „verdeckten“ Einträge bei **jedem** Umbau, auch beim Anlegen einer leeren Datei an ganz anderer Stelle. Der Bericht nennt danach eine Zahl (`Zahlwort::ArchivVerdeckteWeggefallen`). Begründet ist das im Modulkopf von `crates/krk-core/src/archiv/umbau.rs` unter „Was im Fenster nicht steht“: behielte der Umbau einen verdeckten Eintrag, käme er nach dem Entfernen des verdeckenden wieder hervor, und „Löschen“ hieße dann „Aufdecken“. Der Plan entscheidet die Frage nicht. E3 und E4 regeln nur, was gezeigt wird, E13 nur, was ein Umbau mit sichtbaren Einträgen tut. Die Directive verlangt, dass ein Fehlschlag das Archiv nicht beschädigt. Hier ist es kein Fehlschlag, sondern ein gelungener Umbau, der Daten entfernt, nach denen der Nutzer nicht gefragt wurde. Die Frage muss jetzt fallen, weil der Nutzertest am Bündel bevorsteht und die Antwort in `HowTo.md` gehört.

## Options

1. **Verwerfen wie gebaut, und `HowTo.md` sagt es.**
   - Pros: kein weiterer Code; „Löschen“ deckt nie etwas auf.
   - Cons: eine Änderung an einer Stelle entfernt Daten an einer anderen. Ein Archiv mit Duplikaten, etwa nach einem Anhängen mit Pythons `zipfile` im Modus `a` (`speculation:` zur Häufigkeit), verliert die älteren Fassungen still bis auf eine Zeile im Bericht.
2. **Behalten, und nur dann verwerfen, wenn der Schritt den verdeckenden Eintrag selbst entfernt oder umhängt.**
   - Pros: ein Umbau ändert nur, was der Nutzer angefasst hat; das Aufdecken kann nicht geschehen.
   - Cons: mehr Logik in der Aufstellung, nämlich die Zuordnung verdeckt zu verdeckend.
3. **Ein Archiv mit verdeckten Einträgen ist allein lesbar**, wie eines mit verschlüsseltem Eintrag.
   - Pros: einfachste sichere Regel; das Archiv wird nie über das Gewollte hinaus verändert.
   - Cons: manche Archive aus fremden Werkzeugen lassen sich in KRK nicht mehr beschreiben.
4. **Vor dem ersten Byte fragen**, mit der Zahl der verdeckten Einträge, und erst auf ein Ja verwerfen.
   - Pros: der Nutzer entscheidet bewusst.
   - Cons: ein weiteres Blatt; eine Frage vor jedem Umbau eines solchen Archivs.

## Constraints

- Die Zusage der Directive („ohne dass ein Fehlschlag das Archiv beschädigt“) bleibt.
- Kein Eintrag mit unsicherem Namen erscheint je im Fenster oder entsteht beim Herauskopieren (E4).
- „Löschen“ eines sichtbaren Eintrags darf keinen anderen mit demselben Namen sichtbar machen.

## Recommendation

Möglichkeit 3 als Stand für den Nutzertest, Möglichkeit 2 als Folgeschritt, falls der Test solche Archive häufig trifft. Möglichkeit 3 ist die eine Regel, die alle drei Constraints ohne neue Fallunterscheidung hält. Sie folgt dem Muster, das der Bau für verschlüsselte Einträge schon hat (`Umbaufehler::Verschluesselt`).

---
Implemented: `crates/krk-core/src/archiv/verzeichnis.rs` (`Sammler::abschliessen` zaehlt verdeckte Eintraege, `Inhaltsverzeichnis::verdeckt`, Modulkopf), `crates/krk-core/src/archiv/umbau.rs` (`Umbaufehler::Verdeckt`, Pruefung in `umbauen_mit` vor der Aufstellung und beim Schreiben in `zip_schreiben` und `Tarstrom::schreiben`, Modulkopf „Was im Fenster nicht steht“; `Umgebaut::verworfen` entfaellt), `crates/krk-core/src/operation/archiv.rs` (`verworfene_melden` entfaellt), Sprachtabelle (`Text::ArchivVerdecktNurLesbar` neu, `Zahlwort::ArchivVerdeckteWeggefallen` entfaellt), `HowTo.md` und `CLAUDE.md` — Moeglichkeit 3 umgesetzt: ein Archiv mit einem verdeckten Eintrag (frueherer von doppelten Namen, Datei, die einem Ordner weicht, Eintrag hinter einer Verknuepfung) gilt als allein lesbar wie eines mit verschluesseltem Eintrag; jede Aenderung laesst es Byte fuer Byte stehen und meldet den Grund. Ein Eintrag mit unsicherem Namen ist nicht verdeckt und haelt den Umbau nicht auf. Proben `ein_archiv_mit_verdecktem_eintrag_wird_nicht_umgeschrieben` und `ein_unsicherer_name_bleibt_byte_fuer_byte_und_haelt_den_umbau_nicht_auf` in `crates/krk-core/tests/archiv_umbau.rs`.
