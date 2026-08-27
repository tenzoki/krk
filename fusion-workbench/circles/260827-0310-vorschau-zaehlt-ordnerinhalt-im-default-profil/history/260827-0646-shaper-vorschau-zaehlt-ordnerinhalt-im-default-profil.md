# Shaper: Spec für die Runde „Die Vorschau zählt den Inhalt eines Ordners im Default-Profil"

**Datum:** 2026-08-27
**Agent:** shaper, user-direct mode
**Circle:** `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil`
**Status:** Complete

---

## Auftrag

Der Nutzer hat die Directive des aktiven Circles als festgelegt bezeichnet und um ihre Übersetzung in nummerierte Abnahmekriterien gebeten, dazu die Abgrenzung, das Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1, die Lage der Zusage C2.5 der Runde 16 und eine ausdrückliche Antwort auf die Frage, ob diese Runde ein neues Kommando braucht.

## Ergebnis

`circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/planning/260827-0646_o_spec-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`

Vier Fähigkeiten mit 38 Abnahmekriterien, dazu zwei Kriterien im Abschnitt zu den Zeitzusagen, zusammen 40. Die Datei trägt 43 Kästchen; die drei übrigen sind offene Nutzerfragen und keine Kriterien.

- **C1** Das eingebaute Default-Leseprofil, neun Kriterien.
- **C2** Die drei Zählzeilen unter den sechs Metadatenangaben, zwölf Kriterien.
- **C3** Der Baustein `zaehlung` bekommt zwei freiwillige Kriterien, zehn Kriterien.
- **C4** Abzählbare Grenzen statt einer Zeitmessung, sieben Kriterien.

## Keine Klärungsrunde gefahren, und warum

Die Directive und die vier Festlegungen vom 260827 sind vom Nutzer bestimmt, die zwei Entscheidungsdatensätze des Circles am 260827-0629 beantwortet. Was danach offen blieb, waren sieben Fragen unterhalb der Schwelle einer eigenen Runde am Nutzer: die Beschriftungen und ihre Reihenfolge, die Klammer bei null versteckten, die Zeile bei null Einträgen, die Behandlung einer Verknüpfung an zwei Stellen, die Gestalt der zwei neuen Kriterien, und der unlesbare Ordner.

Sie sind als abgeleitete Festlegungen A1 bis A7 in den Spec geschrieben und dort am Spec-Tor überstimmbar. Diese Form stammt aus der Runde 16, die für ihre eigenen sieben Festlegungen denselben Weg gegangen ist.

**Eine Frage ist geprüft und nicht vorgelegt worden.** Ob die Klammer „42 (3)" auch einem Profil aus `readers.toml` offensteht, sah zunächst nach einer Nutzerfrage aus. Die beiden Gegenmöglichkeiten fallen aber schon an bestehenden Festlegungen: eine Klammer allein im Default-Profil wäre der zweite Zählweg, den die Antwort vom 260827-0629 ausschließt, und eine Klammer an jeder Zählung änderte die Ausgabe der zwölf mitgelieferten Profile. Eine Frage, deren Möglichkeiten der Bestand bereits ausschließt, ist keine Frage mehr; sie steht als Festlegung A6 im Spec.

## Berührte Zusagen, im Spec ausgeschrieben

- **L7** aus C8 der Runde 1. Die drei Zählzeilen fallen in seine Endbedingung. Diese Runde setzt keine elfte Zahl, bringt keine Messstrecke mit und schuldet denselben späteren Abnahmelauf wie die Runden 14 und 16. An seine Stelle treten die abzählbaren Grenzen aus C4.
- **C2.5 der Runde 16.** Ihre tragende Hälfte, die sechs Metadatenangaben, bleibt gewahrt. Das Wort „unverändert" trifft nach dieser Runde für die Anzeige als Ganzes nicht mehr zu. Der Spec der Runde 16 wird dabei nicht angefasst; wie die Buchführung darüber aussieht, steht unter `## Open for Planner`.
- **C2.6 und C4.2 der Runde 16** bleiben unberührt und werden von C1.6 und C1.2 gehalten.

## Kein neues Kommando

Geprüft und im Spec als eigener Abschnitt ausgeschrieben. `Kommando::KENNUNGEN`, `Kommando::wirkungsbereich`, `Wirkungsbereich`, `bereich_des_kommandos`, `resources/default-keymap.toml` und die Aufzählung `Kontextbefehl` bleiben sämtlich unberührt. Die drei Zählzeilen haben keinen eigenen Auslöser: sie entstehen dort, wo heute die Metadatenanzeige entsteht.

Gelesen, nicht geschrieben, wird ein Kommando: `versteckte_umschalten` (`shift+cmd+h`). Die Festlegung 4 des Nutzers sagt, dass die Zahlen ihm nicht folgen; C2.7 hält das als Kriterium.

## Gelesen

Der Circle-Datensatz mit seiner Grundlagen-Aufnahme, die zwei Entscheidungsdatensätze des Circles, der Spec der Runde 2 (Abschnitt zu den Zeitzusagen als Vorbild), der Spec der Runde 16 (C2 bis C6), `crates/krk-core/src/leseprofil/mod.rs`, `crates/krk-core/src/leseprofil/erkennung.rs`, `crates/krk-core/src/verzeichnis/eintrag.rs`, `crates/krk-ui/src/vorschaumodell.rs`, `crates/krk-ui/src/appkit/vorschau.rs`, `resources/default-readers.toml`, `resources/default-keymap.toml`, dazu die offenen Datensätze aus `shared/decisions`, `shared/issues` und den Circle-Speichern.

## Kein Datensatz gefiltert oder angelegt

Kein Defekt und keine Entscheidungsfrage sind in dieser Sitzung neu entstanden. Die Berührung der Zusage C2.5 ist heute noch keine Unstimmigkeit, sondern eine Folge des geplanten Baus; ihre Buchführung steht als Auftrag im Abschnitt `## Open for Planner` des Specs.

## Nicht getan

Der Circle-Datensatz ist nicht angefasst worden. Die Zeile `**Active spec/plan:**` steht weiter auf `(none yet)`; sie nachzuziehen ist Sache des Orchestrators oder des Nutzers.
