C3.14 nennt `bis_zur_grenze_lesen` als den Leseweg, und Schritt 4 hat `anlesen` gebaut

---

Das Abnahmekriterium C3.14 des freigegebenen Specs lautet: „Gelesen wird über
`krk_core::text::datei::bis_zur_grenze_lesen` und über die vorhandene Verzeichnismaschinerie.
Ein zweiter Leseweg entsteht nicht." Schritt 4 des Plans hat mit `datei::anlesen` eine dritte
Hülle gebaut, und die Bausteine aus Schritt 6 werden sie rufen und nicht
`bis_zur_grenze_lesen`. Der erste Satz von C3.14 ist damit als Abnahmekriterium nicht mehr
einlösbar. Kein Datensatz hält das fest, und der Plan nennt C3.14 an keiner Stelle.

---

**Gemessen am Baumstand `b76800b`.**

## Die drei Stellen, die auseinanderlaufen

- **Spec, C3.14** (`planning/260824-0613_o_spec-…:208`): nennt `bis_zur_grenze_lesen`
  namentlich als den Leseweg.
- **Entscheidungsdatensatz** `decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`,
  Abschnitt `## Constraints`: „Gelesen wird über den vorhandenen Weg
  `krk_core::text::datei::bis_zur_grenze_lesen`, der die Datei am offenen Deskriptor prüft und
  ihn vor dem nächsten Kandidaten freigibt; ein zweiter Leseweg entsteht nicht." Der Datensatz
  steht auf `_a_` und bindet weiter.
- **Plan, Schritt 4** (`planning/260824-0640_o_plan-…`): baut `anlesen` und begründet es aus
  C6.6. Der Abschnitt `## Current State` schreibt die Notwendigkeit aus („Der Weg vom Pfad zu
  den Bytes hat heute zwei Fragen und braucht eine dritte"), nennt aber weder C3.14 noch den
  Constraints-Abschnitt des Datensatzes, den er damit überholt.

## Warum der Widerspruch echt ist und nicht bloß eine Formulierung

C6.6 verlangt: „Eine Datei wird höchstens bis 64 KB gelesen. Der Titel und das Feld entstehen
aus diesen Bytes." `bis_zur_grenze_lesen` **weist** eine Datei über der Grenze ab
(`crates/krk-core/src/text/datei.rs:633-635`, Zweig `angaben.len() > grenze`), liefert also gar
keine Bytes. Am Bestand nachgemessen und im Plan wie im Doc-Kommentar zitiert: der größte
Circle-Datensatz dieser Werkbank ist 119.614 Bytes groß, seine Zeile `## Directive` steht bei
Byte 222. Mit `bis_zur_grenze_lesen` und 64 KB zeigte C5.6 für genau diesen Circle keine
Directive. C3.14 und C6.6 sind in ihrem Wortlaut nicht zugleich erfüllbar.

**Die zweite Hälfte von C3.14 hält dagegen unverändert**, und sie ist die prüfbare: „nachzuweisen
daran, dass keine neue Stelle im Baum eine Datei über ihren Pfad statt über den Deskriptor
öffnet." `anlesen` geht durch dieselbe eine Tür `verzeichnis::sys::ohne_warten_oeffnen`, prüft
den Typ am `fstat` des offenen Deskriptors und liest danach über `take`
(`crates/krk-core/src/text/datei.rs:683-708`). Eine zweite Tür entsteht nicht. Auch die
Constraints des Specs („Gelesen wird über den Deskriptor und nicht über den Pfad … Ein zweiter
Öffnungsweg entsteht nicht") halten.

## Was zu entscheiden ist

Die Sache ist eine Buchführung und keine Bauarbeit: der Bau ist begründet und geprüft, die
Aussage darüber steht an drei Stellen falsch.

1. C3.14 im Spec so fassen, dass sie die Zusage nennt statt der Funktion: gelesen wird über
   die Hüllen in `krk_core::text::datei`, die sämtlich durch `sys::ohne_warten_oeffnen` gehen
   und am Deskriptor prüfen; eine zweite Tür entsteht nicht. Der Nachweis bleibt derselbe.
2. In den Entscheidungsdatensatz `260824-0541_a_wie-zieht-der-baustein-…` einen Nachtrag
   setzen, der den Constraints-Abschnitt berichtigt — in derselben Form wie die dort schon
   stehende `**Berichtigung 260824-0910**`, also ohne den ursprünglichen Wortlaut zu
   überschreiben.
3. Prüfen, ob `bis_zur_grenze_lesen` in der Zusammenfassung überhaupt noch einen Rufer hat.
   Steht am Ende der Runde keiner mehr, nennt C3.14 eine Funktion, die die Runde nicht benutzt.

**Schwere:** mittel. Kein Fehlverhalten. Aber C3.14 ist eines der sechsundfünfzig
Abnahmekriterien, an denen `## Where this Circle stops` die Runde misst, und es steht in einer
Fassung, die am Ende der Runde als „nicht eingelöst" abzuhaken wäre, obwohl der Bau richtig ist.

**Gefunden:** coderev, Durchsicht des Bereichs `278a008..b76800b` am 260824-1014.

**Betroffen:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md:208`,
`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`,
`crates/krk-core/src/text/datei.rs`

**Domain:** code

---
Also seen: 260824-1130 by coder — Schritt 6 ist gebaut und ruft `text::datei::anlesen` in
`leseprofil::bausteine::angelesener_text`, dem einen Leseweg der Bausteine. Punkt 3 der Liste
oben ist damit entschieden: `bis_zur_grenze_lesen` hat in der Zusammenfassung keinen Rufer, und
C3.14 nennt in ihrer heutigen Fassung eine Funktion, die die Runde nicht benutzt.

---
Resolved: Alle drei Punkte der Liste sind erledigt. C3.14 des Specs nennt seit dem 260824-1224 die Zusage statt der Funktion: gelesen wird ueber die Huellen in `krk_core::text::datei`, die saemtlich durch `verzeichnis::sys::ohne_warten_oeffnen` gehen und den Typ am offenen Deskriptor pruefen; die zweite Haelfte und der Nachweis sind unveraendert. Der Constraints-Abschnitt des Specs, der dieselbe Aussage trug, ist mit derselben Notiz nachgezogen. Der Entscheidungsdatensatz `decisions/260824-0541_a_wie-zieht-der-baustein-…` hat eine `**Berichtigung 260824-1224**` in der Form der dort schon stehenden vom 260824-0910 bekommen, ohne den urspruenglichen Wortlaut zu ueberschreiben und ohne den Marker zu bewegen. **Ein zweiter Datensatz trug denselben ueberholten Constraint und ist mitberichtigt**, den dieser Befund nicht nannte: `decisions/260824-0600_a_der-titel-aus-der-ueberschriftenzeile-…`, Abschnitt `## Constraints`. Punkt 3 ist mit der Notiz vom 260824-1130 bereits entschieden: `bis_zur_grenze_lesen` hat in der Zusammenfassung keinen Rufer. Die Berichtigung aendert ein freigegebenes Abnahmekriterium inhaltlich und ist dem Nutzer vorzulegen.
