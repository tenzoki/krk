# Was geschieht mit einem öffentlichen Namen ohne Rufer im Betriebscode?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1221_*_fuenf-oeffentliche-namen-der-zwei-module-haben-keinen-rufer-ausser-hoechstens-ihrer-eigenen-probe.md`, `260826-1225_*_zwei-oeffentliche-zugaenge-der-ablage-haben-im-ganzen-arbeitsbereich-keinen-rufer.md`, `260911-1838_*_vier-lesezugaenge-von-bestand-und-neuerungen-haben-keinen-rufer-im-betriebscode.md`

---

## Question

Drei Durchsichten haben dreimal denselben Befund abgelegt: ein Name ist `pub`, und im
Betriebscode ruft ihn niemand. Der Übersetzer sieht das nie, weil `pub` in einer
Bibliothekskiste als benutzt gilt. Jeder der drei Datensätze lässt die Antwort offen und
schreibt sinngemäß „entweder fällt, was keinen Rufer hat, oder es hat einen" — also dreimal
dieselbe Frage, dreimal ungelöst, und die nächste Durchsicht legt den vierten ab.

Zu entscheiden ist nicht der Einzelfall, sondern der Maßstab: **woran erkennt dieses Projekt
einen öffentlichen Namen, der weg soll, und woran einen, der bleiben darf?** Ohne den Maßstab
weiß eine Durchsicht nicht, was sie melden soll, und der Bestand wächst weiter.

## Was die Lage entscheidet und in keinem der drei Datensätze steht

**`krk-core`s Proben liegen außerhalb der Kiste.** 16 Dateien unter `crates/krk-core/tests/`
sind Integrationsproben und erreichen ausschließlich `pub`. „Nur von Proben gerufen" ist dort
also keine Nachlässigkeit, sondern die einzige Form, die eine Kernprobe haben kann. Das
unterscheidet `krk-core` von `krk-ui`, das kein Bibliotheksziel hat und seine Proben in
`#[cfg(test)]`-Modulen neben dem Code führt.

**Das Projekt hat am 260908 schon einmal so entschieden, und nicht durch Löschen.**
`MELDEABSTAND` und `HOECHSTE_STELLENZAHL` sind aus der Weiterreichung in der jeweiligen
Modulwurzel gestrichen worden, blieben `pub` in ihrem eigenen Modul, und an die Stelle kam der
Grund. Das ist ein dritter Zug neben „fällt" und „bekommt einen Rufer".

**Die zehn heute offenen Namen zerfallen in zwei Klassen, nicht in eine.** Gezählt am Stand
`6ac43e0`:

| Klasse | Namen |
|---|---|
| gar kein Rufer, auch keine Probe | `operation::Abschluss::ist_abgebrochen`, `Lesezeichenliste::eintrag`, `Nachbardatei::ziel` |
| nur von Proben gerufen | `Lauf::warten`, `Bestand::ordner`, `Bestand::dateien`, `Bestand::fuer`, `Bestand::traegt_unterschied`, `Regel::ist_wirkungslos` |
| hat inzwischen einen Betriebsrufer | `Neuerungen::traegt_unterschied` — seit `ad43d87` von `Bestand::traegt_unterschied` gerufen |

## Options

1. **Zwei Klassen, zwei Antworten.** Ohne jeden Rufer fällt; nur von Proben gerufen bleibt,
   mit dem Grund an der Stelle.
   - Pro: trifft die Unterscheidung, die der Baum wirklich trägt; die zweite Klasse hört auf,
     ein Befund zu sein, und eine Durchsicht weiß, was sie meldet.
   - Contra: drei Namen fallen, sechs bekommen je einen Satz Begründung, den jemand schreiben
     muss.
2. **Ein Maßstab: nur Betriebsrufer zählen.** Neun Namen fallen, die daran hängenden Proben
   wandern nach innen oder fallen mit.
   - Pro: ein Satz, kein Klassenbegriff.
   - Contra: echter Umbau in `krk-core`, und seine Proben verlieren die Außensicht, die sie
     heute prüfen.
3. **Sichtbarkeit statt Existenz.** Nichts fällt; jeder Name ohne Betriebsrufer verliert die
   Weiterreichung aus der Modulwurzel und behält `pub` nur in seinem Modul.
   - Pro: billigster Zug, und der vom 260908 schon gegangene.
   - Contra: die drei völlig ruferlosen Namen bleiben stehen, und mit ihnen die Frage, die
     jeder von ihnen dem nächsten Leser stellt.

## Constraints

- `krk-core` hat zwei Abnehmer im Arbeitsbereich, `krk-ui` und `krk-bench`, und keinen
  außerhalb. Eine Zusage an Fremde gibt es also nicht; `pub` heißt hier allein „der Übersetzer
  warnt nicht".
- Was ein Name behauptet, darf keine Probe halten, die es nicht gibt: `Lesezeichenliste::eintrag`
  trägt heute eine Zusage, die niemand prüft.

---
Answered: 260912-1149_*_was-geschieht-mit-einem-oeffentlichen-namen-ohne-rufer-im-betriebscode.md `## Options` — Möglichkeit 1, zwei Klassen und zwei Antworten: ein öffentlicher Name ohne jeden Rufer fällt, ein nur von Proben gerufener bleibt und bekommt den Grund an seine Stelle geschrieben. Eine Durchsicht meldet künftig allein die erste Klasse; ruled by user, Kai Stalmann <kai@stalmann.org>.
