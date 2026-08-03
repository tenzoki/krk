Der Hilfetext von `cargo xtask` beschreibt die Identitätssuche noch zweistufig

---

`xtask/src/main.rs` trägt in der Konstante `HILFE` (Zeile 22) die Beschreibung der Signaturidentitätssuche aus dem Stand vom 260802-1927:

> Die Signaturidentitaet kommt aus der Umgebungsvariablen KRK_SIGN_IDENTITY. Fehlt sie, wird im Schluesselbund die lokale Identitaet "KRK Entwicklung" gesucht. Fehlt auch die, bricht der Bau mit einer Anleitung ab und weicht nicht auf eine Ad-hoc-Signatur aus.

`xtask/src/sign.rs` sucht seit dem 260802-2253 in **drei** Stufen. Die dritte fehlt im Hilfetext: findet `security find-identity -v -p codesigning` genau eine gültige Identität, nimmt der Bau sie; bei null oder mehr als einer bricht er mit einer Anleitung ab.

Der Nutzer liest also bei `cargo xtask --hilfe` und in jeder Fehlermeldung (`main.rs:43` und `main.rs:82` geben `HILFE` aus) eine Beschreibung, die das Verhalten nicht mehr trifft. Wer eine Apple-Identität im Schlüsselbund hat, aber keine namens "KRK Entwicklung", erwartet nach diesem Text einen Abbruch und bekommt einen erfolgreichen Bau.

---

**Herkunft.** Der `coder` hat den Punkt bei der Umsetzung des Defekts `260802-2050_o_signaturidentitaet-wird-nur-unter-einem-festen-namen-gefunden.md` selbst festgehalten, in `history/260802-2253-signaturidentitaet-eindeutige-lage-und-zertifikatskette.md`, Abschnitt "Ein Punkt, der außerhalb der Grenzen liegt". `main.rs` lag außerhalb der für jene Aufgabe gesetzten Dateigrenzen (`sign.rs` und `README.md`), deshalb hat er ihn gemeldet statt ihn nebenbei mitzuändern. Das ist das richtige Vorgehen; hier ist die Meldung als eigener Defekt.

**Was zu tun ist.** Der `coder` zieht den Absatz in `HILFE` auf die drei Stufen nach. Der Wortlaut soll sagen, was der Bau tut, nicht wie `sign.rs` aufgebaut ist: ausdrückliche Angabe schlägt alles, sonst der Name "KRK Entwicklung", sonst die einzige gültige Identität, sonst Abbruch mit Anleitung. `README.md` beschreibt die drei Stufen bereits korrekt und dient als Vorlage.

**Grenzen.** Nur `xtask/src/main.rs`. Kein Eingriff in `sign.rs`, `bundle.rs` oder `crates/`.
