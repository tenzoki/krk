Ein leeres `fusion-workbench` ist nicht der Zustand vor `/fusion:setup`

---

`resources/default-readers.toml:629-631` sagt: „Ein leeres `fusion-workbench`, also der
Zustand vor `/fusion:setup`, oder eine Datei dieses Namens genügt, und die Vorschau zeigt
statt der Metadaten sieben Zeilen Platzhalter." Der Preis ist richtig und gemessen. Das
Beispiel dafür ist keines: vor `/fusion:setup` gibt es das Verzeichnis nicht, und Setup legt
es mit `mkdir -p` samt Unterordnern an und schreibt `.fusion-setup` im selben Lauf. Ein
leeres `fusion-workbench` entsteht durch keinen Weg, den fusion oder git gehen.

---

**Filed by:** ontorev, Kai Stalmann <kai@qantr.com>
**Cross-references:** `resources/default-readers.toml:627-635`;
`$FUSION_PLUGIN_ROOT/skills/setup/SKILL.md:80` (das `mkdir -p`) und `:68-69` (der Halt bei
`OLD=1`, der das `mkdir` gar nicht erst ausführt);
`shared/issues/260825-2126_c_die-zwei-projektwurzelprofile-erkennen-an-einem-namen-den-die-datei-zwoelf-zeilen-frueher-verwirft.md`

## Was gemessen ist

Der Preis selbst hält, nachgemessen am 260825-2233 über `leseprofil::zusammenfassen_gezaehlt`
an drei künstlichen Projektwurzeln: leeres Verzeichnis `fusion-workbench` → sieben `--`,
Datei `fusion-workbench` → sieben `--`, leeres `flight-workbench` → sieben `--`. Zwei
Leseläufe, null Öffnungen je Fall.

Gelesen in `skills/setup/SKILL.md` (fusion 10.7.0): Setup prüft zuerst auf eine
pre-v4-Werkbank und **hält an, ohne das `mkdir` zu fahren**, wenn es eine findet; sonst
fährt es `mkdir -p ./fusion-workbench/circles ./fusion-workbench/shared/planning …` und
schreibt danach den Marker. Ein Verzeichnis `fusion-workbench` ohne `.fusion-setup` hat
damit Unterordner, ist also nicht leer; ein leeres hat Setup nie gesehen. git legt kein
leeres Verzeichnis an, weil es keines führt.

## Warum das zählt

Der Absatz ist die Antwort auf M4 der ersten Durchsicht und soll den Preis der Erkennung am
Namen so nennen, dass der Nutzer entscheiden kann, ob er ihn zahlt. Das Beispiel soll den
Preis wahrscheinlich machen. Es nennt einen Zustand, den der Nutzer bei fusion nicht antrifft,
und lässt den aus, den er antrifft: eine Datei dieses Namens steht dahinter als zweites
Beispiel richtig, und ein von Hand angelegtes oder ausgeräumtes Verzeichnis wäre das erste.
Die Dispatch-Frage an diese Durchsicht war, ob jeder neue Satz sagt, was ist, und nicht, was
plausibel ist; dieser Halbsatz sagt, was plausibel ist.

Schwere **niedrig**: der Satz ändert an keinem Wert etwas, nur an der Begründung.

## Möglichkeiten

1. Den Halbsatz „also der Zustand vor `/fusion:setup`" streichen. Der Satz trägt danach zwei
   Beispiele, die beide vorkommen: ein leeres Verzeichnis dieses Namens und eine Datei.
2. Ihn durch den Zustand ersetzen, der wirklich vorkommt: „etwa von Hand angelegt oder nach
   dem Verlust seiner Stützdateien" — Letzteres ist der Fall der Betriebsregel aus
   `shared/analyses/260820-2242-…`, in dem ein Löschwerkzeug den Bestand mitnimmt, allerdings
   unter `~/Library/Application Support/KRK/` und nicht in der Werkbank; das Beispiel wäre
   also ein anderes Verzeichnis und nur der Mechanismus derselbe.

Die erste Möglichkeit kostet ein Wort und behauptet nichts.
