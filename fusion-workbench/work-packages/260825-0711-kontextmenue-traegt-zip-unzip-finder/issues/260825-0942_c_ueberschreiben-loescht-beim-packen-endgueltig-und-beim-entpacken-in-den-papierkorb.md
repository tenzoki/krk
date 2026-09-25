"Ueberschreiben" loescht beim Packen endgueltig und beim Entpacken in den Papierkorb

---

Dieselbe Schaltflaeche desselben Blattes bedeutet in dieser Runde zweierlei. Beim Entpacken geht der vorhandene Zielordner ueber die `Papierkorb`-Schnittstelle, ausdruecklich unter Berufung auf die Bindung der Runde 12. Beim Packen ruft derselbe Zweig `loeschen::baum_entfernen`, also ein rekursives, endgueltiges Loeschen. Steht am Archivnamen ein **Ordner**, ist der ganze Baum darunter unwiederbringlich weg.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-core/src/operation/zippen.rs:155-165` — `Konfliktantwort::Ueberschreiben` → `loeschen::baum_entfernen(ziel)`.
- `crates/krk-core/src/operation/loeschen.rs:101-110` — `baum_entfernen` steigt in Ordner ab und ruft `fs::remove_file` und `fs::remove_dir`. Kein Papierkorb.
- `crates/krk-core/src/operation/entpacken.rs:37-45` und `:170-182` — die Gegenseite: `papierkorb.in_den_papierkorb(ziel)`, mit der Runde-12-Bindung im Modulkopf ausgeschrieben.
- `crates/krk-core/src/operation/mod.rs:177` — `zippen::lauf(auftrag, ziel, steuerung)` bekommt den `Papierkorb` gar nicht gereicht; die Leitung fehlt.

## Warum der Fall nicht theoretisch ist

Das Ziel des Packens ist ein Pfad, den die Oberflaeche aus einem Namen bildet (`kontextmenue::archivname`). Ein Ordner namens `Projekte.zip` neben einem Ordner namens `Projekte` ist ein gewoehnlicher Bestand; `zielarchiv_klaeren` fragt allein `fs::symlink_metadata(ziel).is_err()` und unterscheidet Datei und Ordner nicht. Die vierte Nutzerentscheidung dieser Runde legt "Ueberschreiben" im gekuerzten Blatt zudem auf `cmd+Eingabe`, also einen Anschlag.

## Was daran alt ist und was neu

Alt ist der Mechanismus: `ziel_klaeren` (`crates/krk-core/src/operation/mod.rs:423`) nimmt fuer das Kopieren und Verschieben seit jeher denselben Weg. Dort trifft er aber selten einen Ordnerbaum, weil ein Ordner auf einem gleichnamigen Ordner vorher verschmilzt (`mod.rs:418-420`) und der endgueltige Baumloescher nur bei ungleichen Typen ueberhaupt drankommt. Neu ist, dass der Packlauf sein Ziel immer als Datei anlegt und deshalb jeden gleichnamigen Ordner in die Loeschung schickt — und dass dieselbe Runde nebenan den entgegengesetzten Weg gewaehlt und begruendet hat.

## Zwei mogliche Antworten

1. `zippen::lauf` bekommt den `Papierkorb` gereicht und nimmt ihn wie `entpacken`. Dann bedeutet "Ueberschreiben" im ganzen Kontextmenue dasselbe.
2. Der Zustand bleibt, und die Ungleichheit wird als bewusste Wahl festgehalten. Dann gehoert in den Modulkopf von `zippen.rs`, warum die Runde-12-Bindung hier nicht gilt — heute steht dort dazu nichts, waehrend `entpacken.rs` sie ausschreibt.

Die erste ist die kleinere Aenderung und die einzige, die ohne einen zweiten Begriff von "ueberschreiben" auskommt. Ob die Bindung der Runde 12 ueber den Konflikt-Zweig des Kopierens mitentschieden ist, ist eine Nutzerfrage; sie ist mit dieser Runde zum ersten Mal an einer Stelle sichtbar geworden, an der beide Antworten nebeneinander im Baum stehen.

## Nebenbei, im selben Zweig

Auf `Konfliktantwort::UmbenennenIn` wird nicht ein zweites Mal gefragt (`zippen.rs:132-137` schreibt es aus, `entpacken.rs` tut es stillschweigend genauso). Bei `Konfliktregel::AutomatischUmbenennen` liefert `freier_name` einen freien Namen, und die Zusage haelt. Tippt der Nutzer den Namen im Blatt selbst, kann er einen belegten treffen: das Packen schneidet die getroffene Datei mit `File::create` ab, das Entpacken schreibt in den getroffenen Ordner hinein. Beides ohne Papierkorb und ohne Rueckfrage. Dieser Teil ist alt und gilt fuer das Kopieren ebenso; er steht hier, weil er dieselbe Wurzel hat.

## Umfang

`krk-core`, `operation/zippen.rs` und `operation/mod.rs` (die Uebergabe des `Papierkorb`).

---

## Antwort des Nutzers, 260825

**Moeglichkeit 1.** Der Packlauf bekommt den `Papierkorb` gereicht und nimmt ihn wie der
Entpacklauf; `loeschen::baum_entfernen` faellt aus diesem Zweig. "Ueberschreiben" bedeutet
danach im ganzen Kontextmenue dasselbe, und nichts ist unwiederbringlich weg.

Der Nutzer hat die Antwort um eine Zusage ergaenzt, die den Umfang der Loeschung festlegt und
mitzupruefen ist: **angetastet wird allein der Eintrag, dessen Name dem Archivnamen genau
gleicht.** Ein vorhandenes `Projekte.zip` geht in den Papierkorb, ein daneben liegender Ordner
`Projekte` **nicht** — und die Quellen des Laufs ohnehin nie. Der Baumstand haelt diese Zusage
schon (beide Loeschstellen in `zippen.rs` liegen auf `ziel`, keine auf `auftrag.quellen`); sie
ist bisher nur nirgends als Zusage aufgeschrieben und durch keine Probe gehalten. Wer diesen
Defekt behebt, legt beides dazu.

Zitiert in `shared/history/260824-2120-orchestrator-session.md`.

---
Resolved: Moeglichkeit 1 umgesetzt. `zippen::lauf` und `zielarchiv_klaeren` nehmen den
`Papierkorb` entgegen (`crates/krk-core/src/operation/zippen.rs`), der Zweig
`Konfliktantwort::Ueberschreiben` ruft `papierkorb.in_den_papierkorb(ziel)` statt
`loeschen::baum_entfernen`; die Leitung legt `operation/mod.rs` in `ausfuehren`. Die Zusage
"angetastet wird allein der Eintrag, dessen Name dem Archivnamen genau gleicht" steht als
eigener Abschnitt im Modulkopf von `zippen.rs` und an `zielarchiv_klaeren`; gehalten wird sie
von der neuen Probe `ueberschreiben_raeumt_allein_den_gleichnamigen_eintrag_in_den_papierkorb`
in `crates/krk-core/tests/operation.rs` (Ordner am Archivnamen geht in den Papierkorb und ist
dort vollstaendig, gleichnamiger Nachbar ohne `.zip` — zugleich die Quelle des Laufs — bleibt
unangetastet). `die_regel_ueberschreiben_ersetzt_ein_vorhandenes_archiv` belegt denselben Weg
fuer die Datei am Archivnamen. Der ueberholte Querverweis im Modulkopf von `entpacken.rs`
("das der Zip-Lauf fuer seine einzelne Zieldatei nimmt") ist nachgezogen. Der Nebenbefund zu
`UmbenennenIn` und zum selbst getippten Namen ist **nicht** angefasst: er ist alt, gilt fuer
das Kopieren ebenso und traegt keine Antwort des Nutzers.
