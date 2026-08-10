# Der Plan führt außerhalb von S28 weiter „vier Anlässe", samt Sequenzdiagramm

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 1 dieser Sitzung (`9bc0d9d..HEAD`)
**Betroffen:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` Zeilen 315, 326, 693, 874, 883, 897, 900, 903
**Cross-references:** `decisions/260810-0021_i_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md`, Commit `c68f701`, Spec C4

---

## Der Befund

Der Nutzerentscheid vom 260810-0250 hat den vierten Anlass fallen lassen. Im
Code und im Spec ist das nachgezogen, im Plan nur an einer Stelle: dem Nachtrag
in Schritt 28 (Zeile 903). Sieben weitere Stellen stehen unverändert und
beschreiben einen Zustand, den der Code nicht mehr trägt.

| Zeile | Was dort steht | Steht in |
|---|---|---|
| 315 | Abschnittsüberschrift „Die Nachfrage vor den **vier** Anlässen" | Aufbauteil, außerhalb jedes Schritts |
| 326 | Sequenzdiagramm, Kante `N->>A: einer der vier Anlässe` | dasselbe |
| 693 | „bekommt an den **vier** Anlässen trotzdem die Nachfrage" | Schritt zum Editormodell |
| 874 | „Das Blatt … kennt keinen der **vier** Anlässe" | Schritt zum Blatt |
| 883 | Schrittüberschrift „Drei der vier Anlässe: schließen, andere Datei, **Vorschau einblenden**" | S28 |
| 897 | „Die **vier** Anlässe teilen sich mehr als das Blatt … ein **fünfter** Anlass hält an beiden Fallunterscheidungen den Bau an" | S28 |
| 900 | „**Der Anlass „Vorschau einblenden" trägt zwei Befehle**… Die Vorbedingung steht als eine Frage an die Sichtbarkeit (`vorschau_verdraengt_den_editor`)" | S28 |

Die Zeilen 883, 897 und 900 stehen im Wirkungsbereich des Nachtrags und sind
damit halbwegs eingefangen — er sagt ausdrücklich „Wer diesen Schritt später
liest, baut die Vorschau-Anlässe **nicht** nach". Die Zeilen 315, 326, 693 und
874 stehen außerhalb und tragen keine Korrektur. Das Sequenzdiagramm ist der
schwerste Fall: es wird gelesen statt durchgezählt.

Zum Vergleich: die verschobenen Kriteriennummern **sind** im Plan sauber
nachgezogen. Geprüft an den Zeilen 842 (achtes), 843 (neuntes), 919 (sechstes),
926 (siebtes) und 944 (achtes); jede löst gegen die neun Abnahmekriterien des
geänderten Specs richtig auf. Der Befund betrifft allein die Zahl der Anlässe.

## Ein zweiter, kleinerer Punkt in derselben Zeile

Zeile 903 endet mit „Der Datensatz trägt jetzt den Marker `_a_`." Der Datensatz
heißt inzwischen
`decisions/260810-0021_i_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md`
und steht damit auf `_i_` (umgesetzt). Auch der Verweis im Spec-Abschnitt C4
nennt ihn noch als `..._a_...`.

## Was zu tun ist

Die sieben Stellen auf drei Anlässe ziehen, das Sequenzdiagramm mit, und die
beiden Markerangaben auf `_i_`. Die Schrittüberschrift 883 gehört auf „Zwei der
drei Anlässe: schließen, andere Datei" — sie ist die Zeile, die eine spätere
Abgleichsrunde als erstes liest.
