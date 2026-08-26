Der Kopf von Editormeldung sagt, zwei von sechs Ausloesern seien gebaut; die Tafel darunter fuehrt sieben als gebaut

---

Der Doc-Kommentar der Aufzaehlung `Editormeldung` beschreibt den Stand aus S22: "Sie ist heute kurz,
weil erst zwei der sechs Ausloeser gebaut sind; die vier uebrigen kommen mit ihren Schritten". Die Tafel
unmittelbar darunter fuehrt sieben Zeilen, jede mit "gebaut", und die Aufzaehlung traegt zehn Varianten.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-ui/src/appkit/editor.rs:480-483`: "der Wortlaut steht deshalb hier an einer Stelle und nicht
bei den **sechs** Befehlen".
`:487-490`: "Sie ist heute kurz, weil erst zwei der sechs Ausloeser gebaut sind; die vier uebrigen kommen
mit ihren Schritten und tragen ihre Variante bei".
`:492-500`: die Tafel, sieben Zeilen, alle "gebaut".
`:521-617`: zehn Varianten.

Der Absatz `:513-519` ("Seit S39 hat jeder Wert seinen Ausloeser") widerspricht dem Satz zwanzig Zeilen
darueber in derselben Doc.

## Was zu tun waere

Den Absatz `:487-490` streichen oder auf "vollstaendig seit S39" stellen; "sechs Befehlen" gegen die
Tafel halten.

## Umfang

`krk-ui`, `appkit/editor.rs`.
