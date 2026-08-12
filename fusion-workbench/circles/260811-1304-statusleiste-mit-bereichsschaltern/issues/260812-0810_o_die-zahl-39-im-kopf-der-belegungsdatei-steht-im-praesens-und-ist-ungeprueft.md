Die Zahl 39 im Kopf der Belegungsdatei steht im Präsens und ist ungeprüft

---

`resources/default-keymap.toml` sagt im Kopf, die Auslieferung führe „39 frei gewählte
Kombinationen". Der Satz steht im Präsens und zitiert damit einen Datensatz der Runde 1
(`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_*_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`)
als gegenwärtige Aussage über die Datei.

---

**Schwere:** niedrig (kein falsches Verhalten; eine Zahl in einem Kommentar, die falsch sein kann)
**Gefunden:** ontocoder, beim Berichtigen des Spaltenblock-Kommentars am 260812-0805
**Betroffen:** `resources/default-keymap.toml`, Dateikopf
**Domain:** code

## Warum sie nicht einfach nachgezählt wurde

„Frei gewählt" ist in der Datei nicht definiert. Die Abgrenzung liegt im Datensatz der Runde 1
und trennt die Kombinationen, die das Projekt selbst gewählt hat, von denen, die eine
Systemgewohnheit oder ein Norton-Erbe vorgibt. Wer die Zahl nachzählen will, muss diese
Abgrenzung zuerst aus dem Datensatz herausziehen und je Eintrag anwenden.

Diese Runde hat drei Kombinationen hinzugefügt (`opt+cmd+left`, `opt+cmd+b`, `opt+cmd+right`).
Ob alle drei „frei gewählt" sind, ist genau die Frage, die die fehlende Abgrenzung offenlässt.

## Der Unterschied zur Zählzeile daneben

Die Zeile darüber, „79 Funktionen mit zusammen 85 Kombinationen", trägt eine Probe
(`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`), die sie bei jedem Bau gegen
den Inhalt hält. Die 39 trägt keine. Sie ist damit die einzige Zahl im Kopf dieser Datei, die
still veralten kann — und in dieser Runde vermutlich veraltet ist.

## Zwei Wege

1. Die Abgrenzung in die Datei holen und eine zweite Probe schreiben, die auch die 39 hält.
2. Den Satz ins Perfekt setzen: die Runde 1 hat 39 frei gewählte Kombinationen festgelegt. Dann
   ist er eine Aussage über einen Zeitpunkt und veraltet nicht.

Der zweite Weg ist billiger und für den Leser genauso brauchbar; der erste ist der einzige, der
die Zahl auch künftig stimmen lässt.
