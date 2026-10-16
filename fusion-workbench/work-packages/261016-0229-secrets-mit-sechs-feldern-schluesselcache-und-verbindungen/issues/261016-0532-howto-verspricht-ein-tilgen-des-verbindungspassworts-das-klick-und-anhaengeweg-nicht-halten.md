`HowTo.md` verspricht ein Tilgen des Verbindungspassworts, das Klick- und Anhängeweg nicht halten: der ganze Klartext und Kopien aus dem Blatt fallen ungetilgt
---
Jeder Klick auf eine Verbindung und jedes „Verbinden…“ entschlüsselt die ganze `secrets.txt` auf einem Arbeitsfaden und lässt den Klartext mit **allen** Passwörtern ungetilgt fallen; das Blatt „Verbinden“ legt bei jedem Tastendruck ungetilgte `String`-Kopien des Passworts an. `HowTo.md` sagt dem Nutzer dagegen, das Passwort einer Verbindung werde getilgt und ungetilgt bleibe der Klartext nur „im Editor, in der Tabelle oder in den Feldern des Blattes“. Daneben leitet `tresorweg::Entsperrt` `Debug` ab und zeigt den ganzen Klartext.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Entscheidungen 11, 13, 19), 261016-0306-wie-lange-gilt-ein-eingegebener-schluessel-fuer-secrets-txt-und-was-wird-gemerkt.md

## Defekt

**(a) Der Klartext der Arbeitsfäden.** `tresorweg::verbindung_vorbereiten` (`crates/krk-ui/src/tresorweg.rs:477`) und `tresorweg::anhaengen` (`tresorweg.rs:400`) rufen `geheimnisse_lesen`; dort entsteht `Geoeffnet::klartext` (`Vec<u8>`) und daraus `Entsperrt::stand` (`String`, `tresorweg.rs:239`). Beide tragen jedes Passwort der Datei und fallen am Ende des Fadens als gewöhnlicher Speicher, ohne `zeroize`. `geheimnisse::finden` macht nur aus dem einen gesuchten Passwort ein `Passwort`. Der Modulkopf von `crates/krk-core/src/heimordner/tresor.rs` sagt das für `Geoeffnet::klartext` ehrlich; `HowTo.md` nicht.

**(b) Das Blatt.** `blaetter/verbindung.rs:231-235` liest bei jeder Textänderung (`textaenderung_melden`), bei der Prüfung der Eingabetaste und beim Bestätigen alle sechs Felder in ein `Vec<String>`; `entwurf_aus` (`blaetter/verbindung.rs:127`) klont das Passwort daraus in ein `Zeroizing`, das Original im `Vec` fällt ungetilgt. Je Tastendruck im Passwortfeld bleibt so eine weitere Kopie im freigegebenen Speicher.

**(c) `Debug`.** `#[derive(Debug)] pub struct Entsperrt` (`tresorweg.rs:130-131`) gibt mit `{:?}` den ganzen Klartext samt Passwörtern aus. Heute formatiert ihn keine Betriebsstelle; eine Probe mit `expect_err` auf `Result<Entsperrt, _>` gäbe ihn bei unerwartetem Erfolg aus. Das widerspricht der Linie, die das Paket sonst hält (`Passwort`, `Geheimnis`, `Geheimniszeile`, `Verbindungsentwurf` verbergen das Passwort in `Debug`).

**Die Zusage, gegen die das steht:** `HowTo.md:1621-1626` („Getilgt wird das Passwort einer Verbindung … der Wert aus dem Blatt „Verbinden“ und der Wert, den ein Klick aus `secrets.txt` liest“ / „**Nicht** getilgt werden … der entschlüsselte Text, solange er im Editor, in der Tabelle oder in den Feldern des Blattes „Verbinden“ steht“). Ein Leser schließt daraus, dass ein Klick bei geschlossenem Editor nur das eine Passwort kurz im Speicher hält.

Das Bedrohungsmodell der Datei bleibt das versehentliche Lesen durch Werkzeuge; die Schwere kommt aus der zu großen Zusage an den Nutzer, nicht aus einem Leck auf die Platte.

## Richtung der Behebung

Eine Entscheidung, zwei Wege: entweder die Fäden tilgen, was sie entschlüsseln (`Entsperrt::stand` als `Zeroizing<String>`, `klartext` vor dem Fallen tilgen, im Blatt die Feldwerte in `Zeroizing` lesen und verschieben statt klonen), oder `HowTo.md` und der Modulkopf von `tresorweg.rs` sagen genau, was bleibt. In beiden Fällen bekommt `Entsperrt` ein `Debug` ohne `stand`.

## Akzeptanz

- `HowTo.md`, Abschnitt „Was getilgt wird und was nicht“, stimmt mit dem Code überein, Satz für Satz.
- `format!("{:?}", entsperrt)` enthält einen Wert aus dem Klartext nicht (Probe in `tresorweg.rs`).
- `make check` grün.

---
Resolved: Code getilgt, wo vertretbar: `tresorweg::Entsperrt::stand` ist `Zeroizing<String>`, die entschluesselten Bytes stehen in `geheimnisse_lesen` in einem `Zeroizing` und werden geliehen in genau eine Zeichenkette gewandelt; `anhaengen` haelt den neuen Text, `Chiffrat::verschliessen` eine gewachsene Sicherungsform in `Zeroizing` (`crates/krk-ui/src/tresorweg.rs`); der Editor nimmt den Stand per `std::mem::take` heraus (`editormodell.rs`, `geheimnisse_laden`). `Entsperrt` hat ein eigenes `Debug` mit der Laenge statt des Textes, Probe `debug_einer_entsperrten_datei_zeigt_keinen_klartext`. Das Blatt nimmt die Felder heraus statt sie zu klonen (`entwurf_aus(Vec<String>)`, `appkit/blaetter/verbindung.rs`). Im Kern entstehen beim Anhaengen keine ungetilgten Zwischenkopien des neuen Passworts mehr (`geheimnisse::hinzufuegen` mit `Zeroizing` und fester Kapazitaet, `notizen::angehaengt` ohne `themenzeile`/`textzeilen`-Zwischenkopie). `HowTo.md` „Was getilgt wird und was nicht“, `README.md`, `CLAUDE.md` und die Modulkoepfe von `tresorweg.rs` und `krk-core/src/heimordner/tresor.rs` sagen jetzt genau, was getilgt wird und was nicht (macOS-Feldwerte des Blattes, uebrige Felder neben dem Text, Kopie an das System, Arbeitsdaten der Verfahren, Reste beim Verschieben oder Wachsen).
