# Merkt sich der Feldbaustein seine Dateiöffnungen, wie der Leselauf seine Lesungen?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260912-2154_*_zwei-proben-in-leseprofil-rs-schreiben-die-gestalt-der-profile-von-vor-fusion-11-fest.md`, `260911-1825-fusion-leseprofile-auf-fusion-11-nachziehen.md`

---

## Question

Das Nachziehen der fusion-Leseprofile auf Fassung 11 hat die vier markerlesenden Ja/Nein-Zeilen
des Rundenprofils durch eine Feldzeile „Zustand" ersetzt, die `**Status:**` aus dem Datensatz
liest. Neben ihr steht die Feldzeile „Directive", die **dieselbe Datei** nennt. Beide öffnen
sie, also zweimal.

Damit bricht die Auslieferungsfassung das Abnahmekriterium C6.7 der Runde 16 um genau eine
Öffnung. Wörtlich: „Das größte mitgelieferte Profil, das des einzelnen Circles, bleibt in der
Messung unter allen Grenzen aus C6.4: es löst höchstens 7 Verzeichnisleseläufe und höchstens
11 Dateiöffnungen aus." Gemessen sind 7 und **12**.

Die Frage ist nicht die Zahl, sondern ihre Ursache: **soll zwei Feldzeilen auf dieselbe Datei
eine Öffnung kosten statt zwei?**

## Der Vorgänger, der die Frage schon einmal beantwortet hat

Die Runde 18 hat genau diesen Zug für den **Leselauf** gefahren: ein Ort je Zusammenfassung
wird höchstens einmal gelesen, und `Lauf` merkt sich seine Lesungen unter einem Schlüssel
(`crates/krk-core/src/leseprofil/bausteine.rs:343`). Das kehrte eine begründete Festlegung der
Runde 16 um, die die Asymmetrie ausdrücklich gewollt hatte.

Für die **Dateiöffnung** ist derselbe Zug nicht gefahren worden, und bis heute fiel das nicht
auf: vor dieser Änderung nannte keine zwei Feldzeilen eines mitgelieferten Profils dieselbe
Datei.

## Options

1. **Der Feldbaustein merkt sich seine Öffnungen, je Zusammenfassung, wie der Leselauf seine
   Lesungen.** Zwei Feldzeilen auf dieselbe Datei kosten eine Öffnung.
   - Pro: behebt die Ursache statt der Zahl; C6.7 bleibt unberührt und gilt weiter als
     gemessene Zusage; der Zug ist der, den die Runde 18 an der Nachbarstelle schon gefahren
     hat, also keine neue Bauart.
   - Contra: Arbeit in `krk-core/src/leseprofil/`, nicht in einer Datendatei; der Merkschlüssel
     muss Datei **und** gelesenen Bereich treffen, sonst liefert die zweite Zeile den Wert der
     ersten.
2. **C6.7 auf zwölf nachziehen.**
   - Pro: heute fertig, eine Zeile im Spec und eine in der Probe.
   - Contra: verschiebt eine Zusage, damit die Umsetzung hineinpasst. Die nächste Feldzeile auf
     eine schon geöffnete Datei stellt dieselbe Frage wieder, und dann steht dreizehn zur
     Wahl.
3. **Eine der zwei Zeilen aufgeben.**
   - Pro: billigste Rechnung.
   - Contra: „Zustand" ist die Auskunft, für die die Umstellung überhaupt gefahren wurde.

## Constraints

- C6.7 ist eine gemessene Zusage und keine Schätzung; die Probe
  `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` zählt die Öffnungen
  und hält heute die exakte Zwölf, damit jede weitere rot wird.
- Der Feldbaustein liest bis 64 KB (C6.6). Zwei Feldzeilen auf derselben Datei lesen denselben
  Bereich, also ist der gemerkte Inhalt für beide derselbe — das ist die Voraussetzung, unter
  der Möglichkeit 1 überhaupt trägt, und sie ist zu prüfen und nicht anzunehmen.

---
Answered: `260913-0851_*_merkt-sich-der-feldbaustein-seine-dateioeffnungen-wie-der-leselauf-seine-lesungen.md` `## Options` — Möglichkeit 1: der Feldbaustein merkt sich seine Öffnungen je Zusammenfassung, wie der Leselauf seit der Runde 18 seine Lesungen. Die Ursache wird behoben und nicht die Zahl, C6.7 bleibt unberührt. Die Auslieferung wartet darauf; ruled by user, Kai Stalmann <kai@stalmann.org>.
