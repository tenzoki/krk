Die Probe gegen den zu weiten Schnitt prueft nur den Packschneider; der Entpackschneider hat keine

---

`ein_einzelnes_archiv_bleibt_seine_eigene_quelle` (`crates/krk-ui/src/kommandos/kontextmenue.rs:1124`)
ist die Probe, die der Commit `dd74b0e` „gegen den zu weiten Schnitt" gestellt hat. Sie ruft
`packziel` und sonst nichts. Der zweite Schneider, `ohne_die_eigenen_ziele`, hat keine eigene Probe
gegen den zu weiten Schnitt; dass ein einzelnes Archiv beim Entpacken seine eigene Quelle bleibt —
der haeufigste Unzip-Fall ueberhaupt —, haelt allein aelterer Bestand, der vor dem Schnitt
geschrieben wurde und ihn im Namen nicht nennt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1249, in der dritten Durchsicht der Runde 17
(`6faaa91..ddd41ff`).**

## Was die Probe tut

```rust
#[test]
fn ein_einzelnes_archiv_bleibt_seine_eigene_quelle() {
    let ordner = ordner();
    let betroffen = vec![ordner.join("sicherung.zip")];

    let (quellen, ziel) = packziel(&betroffen, ordner);

    assert_eq!(ziel, ordner.join("sicherung.zip.zip"));
    assert_eq!(quellen, betroffen);
}
```

Ihr Doc-Kommentar sagt es selbst: „Die angehaengte Endung macht aus `sicherung.zip` das Archiv
`sicherung.zip.zip`, und damit ist der Eintrag nicht sein eigenes Ziel." Das ist die **Pack**-Regel.
`entpackziel` kommt in der Probe nicht vor.

## Was das fuer die Gegenprobe bedeutet

Die Commit-Nachricht von `dd74b0e` haelt fest: „Gegenprobe gefahren: mit ausgeschaltetem
`ist_ziel_des_laufs` werden die ersten zwei rot, die dritte bleibt gruen." Nachgerechnet stimmt das,
und der Grund ist ein anderer als der genannte: die dritte bleibt nicht gruen, weil der Schnitt
massvoll ist, sondern weil sie den Schnitt der Entpackseite gar nicht beruehrt und der Schnitt der
Packseite auf ihre Eingabe nicht wirkt. Eine Gegenprobe, die aus dem Gruenbleiben einer Probe auf die
Genauigkeit des Schnitts schliesst, misst hier nichts.

## Was den Entpackfall heute haelt

Nachgesehen: er ist gehalten, nur nicht von der genannten Probe.

- `ohne_betroffenes_archiv_gilt_das_eine_des_ordners` (`kontextmenue.rs:1021`) — die Ersatzregel mit
  einem Archiv.
- `drei_betroffene_archive_ergeben_drei_zielordner` (`kontextmenue.rs:980`) — drei Archive ohne
  Kettenbezug, alle drei bleiben stehen. Diese wuerde rot, waere der Entpackschnitt zu weit.
- `der_filtertext_engt_die_ersatzregel_ein` (`kontextmenue.rs:1066`) und weitere.

Alle drei stammen aus der Zeit vor `dd74b0e`. Wer den Schnitt spaeter anfasst, liest ihre Namen und
erkennt nicht, dass sie seine Grenze halten; wer sie umbaut, nimmt die Grenze mit, ohne es zu merken.
Genau diese Lage schreibt der Kopf von `crate::quellbaum` als die zu vermeidende aus.

## Vorschlag

Eine vierte Probe neben den dreien, mit sprechendem Namen, etwa
`ein_einzelnes_archiv_behaelt_seinen_zielordner`: ein Archiv markiert, `entpackziel` gerufen, das
Paar steht. Fuenf Zeilen. Und im Doc-Kommentar der bestehenden Probe die Einschraenkung, dass sie den
Packschneider prueft und nicht beide.

**Schwere:** gering. Kein Fehler am Baum, eine Luecke in der Abdeckung an der Stelle, an der die
Runde ihre eigene Zusage gegen das Uebermass sichert.

**Betroffen:** `crates/krk-ui/src/kommandos/kontextmenue.rs`, Probenmodul.

---
Resolved: Die vierte Probe steht da, unter dem vorgeschlagenen Namen
`ein_einzelnes_archiv_behaelt_seinen_zielordner`
(`crates/krk-ui/src/kommandos/kontextmenue.rs`, Pruefmodul). Sie ruft
`entpackziel` mit einem markierten Archiv und haelt das Paar.

**Ein zweites Archiv steht dabei im Ordnermodell, und das ist die eigentliche
Pruefung.** Ohne es bliebe sie auch bei einem zu weiten Schnitt gruen: die
Markierung fiele leer aus, und die Ersatzregel lieferte dasselbe eine Paar
wieder zurueck. Mit ihm antwortet die Ersatzregel `Entpackbefund::Mehrere`, und
der zu weite Schnitt faellt auf. Beim Bauen der Gegenprobe ist genau das
aufgefallen; die erste Fassung der Probe war blind.

Der Doc-Kommentar von `ein_einzelnes_archiv_bleibt_seine_eigene_quelle` sagt
jetzt, dass sie den Packschneider prueft und nur ihn, und verweist auf die neue
Probe daneben. Gegenprobe gefahren — laesst man ein Archiv als sein eigenes Ziel
gelten, wird die neue Probe rot.
