# Welche Form hat das Pfadmuster, und welche die Kennzeichendatei?

---
**Domain:** code
**Filed by:** shaper
**Cross-references:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/_*_circle.md`, `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_a_wie-zieht-der-baustein-ein-feld-aus-einer-datei-und-traegt-er-auch-einen-abschnitt.md`, `shared/backlog/260823-2136_*_readerconventions-profile-fuer-dateizugriff.md`

---

## Question

Der Nutzer hat am 260823 festgelegt, dass ein Profil seinen Ort über ein Pfadmuster **oder**
über eine Kennzeichendatei darin erkennt und dass das Pfadmuster vorgeht. Offen ist, welche
Form beide haben: was der Nutzer in seine `readers.toml` schreibt, wenn er einen Ort meint.
Die Antwort entscheidet, ob die sechs skizzierten Zusammenfassungen überhaupt ausdrückbar
sind und wie viele Profile die mitgelieferte `default-readers.toml` trägt, und sie
entscheidet, was der Nutzer lernen muss, bevor er ein eigenes Profil schreibt. Sie gehört
vor die Abnahmekriterien, weil jedes Kriterium über die Erkennung ihren Wortlaut trägt, und
sie war bis jetzt nicht gestellt.

Die sechs Orte des Beispielfalls, an denen sich jede Form messen lässt, mit dem Bestand vom
260824-0600:

| Ort | Beispiel | Zahl |
|---|---|---|
| Wurzel der Werkbank | `…/krk/fusion-workbench` | eine |
| ein gemeinsamer Speicher | `shared/analyses/`, `backlog/`, `consult/`, `history/`, `planning/`, `reviews/` | sechs Ordner |
| derselbe Speicher im Circle | `circles/<name>/analyses/` und die übrigen | sechs je Circle |
| Defektspeicher | `shared/issues/`, `circles/<name>/issues/` | zwei Formen |
| Circle-Verzeichnis der Werkbank | `circles/` | eines |
| ein einzelner Circle | `circles/260823-2208-vorschau-…/` | achtzehn, jeder anders benannt |

## Options

1. **Regulärer Ausdruck auf dem vollen Pfad** — Das Muster ist ein Ausdruck derselben Kiste,
   die der Baustein „Feld aus einer Datei" seit dem 260824-0555 ohnehin mitbringt. Die
   Kennzeichendatei ist ebenfalls ein Ausdruck auf dem Dateinamen.
   - Pros: Trägt alle sechs Zeilen der Tabelle, den einzelnen Circle eingeschlossen
     (`fusion-workbench/circles/[^/]+$`), und fasst die sechs Speicher zu **einem** Profil
     zusammen. Die mitgelieferte Datei bliebe bei etwa fünf Profilen. Eine Form für beide
     Muster und für den Baustein, also genau eine Sprache in der ganzen Datei. Die
     Kennzeichendatei erreicht damit auch die sechs Schreibweisen von `_?_circle.md`.
   - Cons: Der Nutzer schreibt Ausdrücke, wo er einen Ordner meint, und muss Punkte und
     Schrägstriche im Kopf behalten. Ein Ausdruck aus der `readers.toml` läuft im Arbeitsfaden
     der Vorschau, also innerhalb der Endbedingung von L7; er darf sie nicht anhalten können.
2. **Glob mit `*` und `**`** — `*` steht für ein Pfadstück, `**` für beliebige Tiefe:
   `**/fusion-workbench/circles/*`. Die Kennzeichendatei ist ein Glob auf dem Dateinamen.
   - Pros: Die geläufige Form für Pfade, ohne Maskierung und ohne Ausdruckswissen. Trägt alle
     sechs Zeilen der Tabelle. Die sechs Speicher fasst sie nur über eine Klammerform
     zusammen, sonst braucht jeder sein Profil.
   - Cons: Der Baum führt keinen Globvergleich, also entsteht er neu oder kostet eine zweite
     fremde Kiste. Zwei Mustersprachen in einer Datei: Glob für den Ort, Ausdruck für das
     Feld.
3. **Endstücke des Pfades, ohne jedes Sonderzeichen** — Das Muster ist eine Folge von
   Pfadstücken, und es trifft, wenn der Pfad auf sie endet: `shared/analyses` trifft
   `…/fusion-workbench/shared/analyses`, `issues` trifft beide Defektspeicher.
   - Pros: Nichts zu lernen und nichts zu maskieren. Trifft fünf der sechs Zeilen, und den
     Defektspeicher in beiden Formen mit einem einzigen Muster.
   - Cons: Den **einzelnen Circle** trifft sie nicht, denn seine achtzehn Namen haben kein
     gemeinsames Endstück; dieser Fall hinge allein an der Kennzeichendatei, und die müsste
     dann ihrerseits ein Muster tragen, um die sechs Schreibweisen von `_?_circle.md` zu
     erreichen. Die zwölf Speicherordner brauchen zwölf beinahe gleiche Profile.

## Constraints

Die Vorrangregel des Nutzers vom 260823 bleibt: erst die Pfadmuster in der Reihenfolge der
Datei, danach die Kennzeichendateien, und ohne Treffer bleibt die heutige Metadatenanzeige
stehen. Jede Antwort muss die sechs Zeilen der Tabelle ausdrücken können, sonst fällt ein
skizzierter Fall aus der Runde. Ein Muster, das der Nutzer schreibt, läuft im Arbeitsfaden
der Vorschau und darf ihn nicht anhalten; das ist dieselbe Bedingung, an der die Vorschau
seit dem 260810 ihre Dateien über einen Deskriptor mit `O_NONBLOCK` öffnet.

## Recommendation

Möglichkeit 1. Die Runde nimmt die Ausdruckskiste für den Feldbaustein ohnehin auf, und der
Datensatz `260824-0600_o_…-er-fuehrt-eine.md` zeigt, dass die Maschinerie über `syntect`
schon im Bündel steht. Eine zweite Mustersprache daneben wäre die Sorte Sonderfall, die
dieses Projekt sonst vermeidet, und Möglichkeit 3 verlöre den einzelnen Circle, also gerade
den Fall, für den die Frage nach dem Feldbaustein gestellt wurde. Der Preis ist benannt: der
Nutzer schreibt Ausdrücke, und die Runde schuldet eine Zusage, dass ein Ausdruck aus der
`readers.toml` die Vorschau nicht anhalten kann.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:

---
Answered: circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/history/260824-0530-orchestrator-session.md:84 — Regulärer Ausdruck auf dem vollen Pfad (Möglichkeit 1); eine Mustersprache statt zweier.
Implemented: 260824-1849, Commits `f013227` (Schritt 3) und `a327d08` (Schritt 5). `Profil::pfad` und `Profil::kennzeichen` sind beide `Option<Regex>`; das Pfadmuster läuft im ersten Durchgang gegen den vollen Pfad, die Kennzeichendatei im zweiten gegen die Namen der Einträge. Die Datei trägt damit eine Mustersprache und nicht zwei. Belegt durch die sieben Erkennungsproben in `crates/krk-core/tests/leseprofil.rs`, darunter `::ein_pfadmuster_trifft_seinen_ordner_und_den_daneben_nicht`.
