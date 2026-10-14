# Ein Circle-Datensatz liegt beim 1,8-fachen der 64-KB-Grenze des Feldbausteins

---
**Domain:** code
**Filed by:** analyst
**Cross-references:** `shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md` (Abschnitt „Die 64-KB-Grenze ist an dieser Werkbank schon überschritten"); `crates/krk-core/src/leseprofil/mod.rs` (`HOECHSTENS_BYTES`); `crates/krk-core/src/text/datei.rs` (`anlesen`); `resources/default-readers.toml` (Profil „fusion-Werkbank: eine Runde", Zeile „Directive")

---

## Was ist

Der Feldbaustein liest seine Datei über `text::datei::anlesen` bis `HOECHSTENS_BYTES`, also bis
64 KB, und sucht sein Muster in dem, was er bekommen hat. Findet er es dort nicht, liefert er den
Platzhalter, und die Zeile sieht aus wie eine Zeile über ein Feld, das es nicht gibt.

Gemessen am 260825-2107 an der Werkbank dieses Projekts: der Circle-Datensatz
`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_d_circle.md` ist **119.614
Byte** groß, also das 1,8-fache der Grenze. Die Zeile „Directive" jenes Profils antwortet
trotzdem richtig, und der Grund ist der Ort des Abschnitts und nicht die Größe der Datei:
`## Directive` steht in Zeile 12.

Die zweite Hälfte der Messung fällt entlastend aus. Keine der Dateien, die der Baustein „die
jüngsten zehn" in seiner Titelform öffnet, liegt heute über 64 KB: geprüft über
`find shared/history circles/*/history shared/issues circles/*/issues -name '*.md' -size +64k`,
Ergebnis null Treffer.

## Warum das zählt

Der Circle-Datensatz ist die eine Datei dieses Profils, deren Größe nicht durch eine Konvention
begrenzt ist: er wächst mit den Anhängen, die eine Runde über ihre Laufzeit sammelt, und der
größte liegt schon beim Doppelten. Solange `## Directive` weit oben steht, trägt die Zeile.
Wächst über ihm ein Abschnitt, den ein späterer Agent dort einfügt, fällt die Zeile still auf
den Platzhalter zurück, und die Anzeige unterscheidet nicht zwischen „diese Runde hat keine
Directive" und „die Directive liegt jenseits von 64 KB".

Der Fall ist heute keiner. Er ist eine Lage, die sich durch Bestand herstellt, und dieser
Datensatz hält sie fest, damit sie nicht als Rätsel wiederkehrt.

## Was zu tun wäre

Nichts am Code, ohne eine Entscheidung. Drei Wege liegen erkennbar da, und keiner ist offenkundig
der richtige:

1. **Es bleibt, wie es ist.** 64 KB sind für ein Feld nahe am Kopf einer Datei reichlich, und die
   Grenze steht in `readers.toml` erklärt.
2. **Der Feldbaustein sagt, dass er abgeschnitten hat.** Ein Nichtfund in einem abgeschnittenen
   Anlesen ist kein Nichtvorhandensein, und dieselbe Unterscheidung trägt der Zählbaustein schon
   über `Wert::UeberGrenze`. Das wäre die Anwendung einer Regel, die im Modulkopf von
   `leseprofil::bausteine` bereits dreimal dasteht, auf den vierten Baustein.
3. **Die Grenze steigt.** Sie verschöbe das Problem und nähme dem Haushalt seine Ablesbarkeit.

Möglichkeit 2 ist die einzige, die die Aussage der Zeile berichtigt statt sie zu verschieben; sie
kostet aber einen weiteren Wert oder eine weitere Bedeutung im Ergebnistyp und ist damit keine
Kleinigkeit.

**Schwere:** niedrig, latent.

**Gefunden:** analyst, bei der Kostenmessung zu Schritt 10 des Plans der Runde 18.

**Betroffen:** `crates/krk-core/src/leseprofil/bausteine.rs` (der Feldbaustein), `crates/krk-core/src/leseprofil/mod.rs` (`HOECHSTENS_BYTES`)
