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

---

## Nachtrag 260912-1330 — eine dritte Klasse, gefunden bei der ersten Anwendung

Die Umsetzung hat zwei Dinge zutage gefördert, die den Maßstab schärfen. Beide sind vom
Nutzer am 260912 entschieden.

**Der Maßstab hatte eine Lücke: die innere Probe.** `Regel::ist_wirkungslos`
(`stapelumbenennen/regel.rs:100`) wird allein von einer Probe gerufen, fiele also in die
zweite Klasse — aber diese Probe liegt **innen**, im `#[cfg(test)]`-Modul derselben Datei,
und käme über `pub(crate)` oder Modulsichtbarkeit genauso heran. Die Begründung, die die
zweite Klasse trägt, gilt hier gerade nicht: sie beruht darauf, dass `krk-core`s
Integrationsproben in `crates/krk-core/tests/` eine eigene Kiste sind und ausschließlich
`pub` erreichen.

**Die dritte Klasse und ihre Antwort:** ein Name, den allein eine Probe **innerhalb seiner
eigenen Kiste** ruft, bleibt und verliert `pub`. Der Maßstab lautet damit vollständig:

| Rufer | Antwort |
|---|---|
| gar keiner | fällt |
| allein eine Probe außerhalb der Kiste (`crates/krk-core/tests/`) | bleibt `pub`, mit dem Grund an der Stelle |
| allein eine Probe innerhalb der Kiste | bleibt, Sichtbarkeit auf `pub(crate)` zurück |
| mindestens einer im Betriebscode | bleibt, kein Befund |

Damit ist die Prüffrage nicht mehr „ruft ihn jemand", sondern „**braucht** der Rufer die
Sichtbarkeit, die der Name trägt". Das ist die Frage, die in allen vier Zeilen dieselbe ist.

**Die Doppelung an `Regel::ist_wirkungslos` bleibt davon unberührt.** Sie beantwortet für die
ganze Regel dieselbe Frage, die `Vorschauzeile::wird_umbenannt` (`stapelumbenennen/vorschau.rs:38-40`)
je Zeile beantwortet. Das ist kein Sichtbarkeitsproblem und von dieser Antwort nicht erledigt.

**Zwei der ursprünglichen Befunde waren falsch, in entgegengesetzte Richtungen**, und beide
Fehler haben dieselbe Ursache: eine Erhebung, die nicht je Name geprüft hat.

- `Lauf::warten` hat einen Betriebsrufer, `krk-ui/src/appkit/anwendung.rs:9391`, seit
  `343a7f3` vom 260804 — drei Wochen **vor** der Erhebung vom 260826, die ihn übersah, weil
  ihr `grep` auf `krk-core` beschränkt war. Der Name war nie ein Befund.
- `Bestand::ordner` hat **gar keinen** Rufer, auch keine Probe. Der Befund vom 260911 hat vier
  Zugänge gemeinsam geprüft und festgestellt, jeder Treffer liege in `tests/ablage.rs`; für
  `ordner` gab es überhaupt keinen Treffer. Der Name fällt.

**Daraus die Erhebungsregel für die nächste Durchsicht:** je Name einzeln zählen, über
`crates` **und** `xtask`, und den Fund nach der Kiste des Rufers unterscheiden. Eine
Sammelaussage über mehrere Namen ist in beide Richtungen falsch geworden; ruled by user,
Kai Stalmann <kai@stalmann.org>.

### Was die dritte Zeile in der Praxis bedeutet

„Bleibt, Sichtbarkeit auf `pub(crate)` zurück" ist schärfer, als es klingt, und das gehört in
den Maßstab statt nur an eine Fundstelle. **`pub(crate)` allein übersetzt nicht.** Ein `pub` in
einer Bibliothekskiste gilt dem Übersetzer als benutzt, ein `pub(crate)` nicht; ohne
Betriebsrufer schlägt `dead_code` unter `-D warnings` zu, und `make lint` bricht ab. Wer die
dritte Zeile fährt, setzt also `#[cfg(test)]` daneben — und damit gibt es den Namen im
ausgelieferten Bau nicht mehr.

Sachlich ändert das nichts, denn ein Name ohne Betriebsrufer war dort ohnehin toter Code. Aber
die dritte Zeile heißt praktisch **„wird Prüfcode"** und nicht „behält seinen Platz mit
engerer Sichtbarkeit". Wer das nicht will, hat nur die erste Zeile: fallen lassen.

`#[cfg(test)]` und nicht `#[allow(dead_code)]`, weil das zweite einen künftigen Rufer
ankündigt, den es nicht gibt; und nicht `#[expect(dead_code)]`, weil im Probenbau ein Rufer
existiert und die Erwartung dort unerfüllt bliebe. Dieselbe Wahl steht an `Fokus::ALLE`
(`krk-ui/src/kommandos/fokus.rs`) ausgeschrieben.

**Der Zug vom 260908 war deshalb billiger**, und das erklärt ihn nachträglich: bei
`MELDEABSTAND` und `HOECHSTE_STELLENZAHL` blieb `pub` stehen und fiel nur die Weiterreichung
aus der Modulwurzel. Wo ein Rufer im Betriebscode steht, ist das der mildeste Zug; wo keiner
steht, gibt es ihn nicht.
