# Das zweite Bild des Spec zeigt den Abstieg mit Rückkehr, der Baum merkt Pfade vor

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, zweites Bild und der Absatz „Die Rückkehr aus dem Abstieg ist die Kante, die das Bild teuer macht" (Zeile 210); C3.8, C3.10; `crates/krk-core/src/verzeichnis/durchlauf.rs`, Abschnitt `# Ein offener Deskriptor, gleich wie tief der Baum ist`; `issues/260815-0211_c_ein-deskriptormangel-des-prozesses-wird-zu-einem-dauerhaften-kein-treffer-darunter.md`

---

## Befund

Die Behebung des Defekts `260815-0211` hat den Abstieg des Durchlaufs umgebaut. Er hält keinen Leser je Ebene mehr, sondern merkt Unterordner als Pfad vor und öffnet den nächsten erst, wenn der laufende Ordner zu Ende gelesen ist. Damit stimmen drei Stellen des zweiten Bildes nicht mehr mit dem Baum überein:

1. Der Knoten `in ihn absteigen` (`ABST`) heißt jetzt „vormerken"; er öffnet nichts.
2. Die Kante `war es der Ordner des angezeigten Ordners? — nein: weiter im übergeordneten Ordner` (`ZURUECK --> NOCH`) gibt es nicht mehr. Der Weg zum nächsten Ordner läuft über den vorgemerkten Pfad und nicht über einen offen gehaltenen Leser.
3. Der Zweig `lässt er sich öffnen? — nein` hat einen zweiten Ausgang bekommen: fehlt ein Deskriptor (`EMFILE`, `ENFILE`), gibt es **keinen** Befund statt „kein Treffer darunter".

Die Kreiszählung im Absatz darunter hängt an genau der Kante aus Punkt 2. Sie nennt sieben einfache Kreise, davon vier über die Rückkehr, nachgerechnet am 260814-1950 über neunzehn Knoten und siebenundzwanzig Kanten. Nach dem Umbau sind es weniger, und die Zahlen im Prüfvorbehalt gelten nicht mehr.

## Was daran nicht zu berichtigen ist

**Kein Abnahmekriterium ist verletzt.** C3.8 (keine Tiefengrenze) hält weiterhin und ist jetzt zusätzlich unter einer abgesenkten Deskriptorgrenze gemessen. C3.10 (ein Ordner, der sich nicht öffnen lässt, gilt als „kein Treffer darunter") hält für jeden Grund, der am Pfad liegt; der Deskriptormangel liegt nicht am Pfad und war von C3.10 nie gemeint. C3.1, C3.3, C3.4, C3.9 und C3.13 sind unberührt.

Der Spec sagt unter „Was dieser Spec nicht festlegt" ausdrücklich, wie der Durchlauf gebaut ist, sei nicht seine Sache. Das Bild beschreibt trotzdem einen bestimmten Bau, und wer es liest, liest den falschen.

## Was zu tun wäre

Das zweite Bild neu zeichnen und den Absatz zur Kreiszählung mitziehen, danach eine Diagrammprüfung. Ob C3.10 daneben einen Satz über den Deskriptormangel bekommt oder ob dieser Fall in C3.13 als vierter Weg zu „nicht entschieden" gehört, ist die inhaltliche Frage dabei; der Baum entscheidet ihn heute als „nicht entschieden", zusammen mit dem Abbruch.

---
Resolved: 260815-0246, shaper. Das zweite Bild ist neu gezeichnet und trägt jetzt zweiundzwanzig Knoten, einunddreißig Kanten und fünf einfache Kreise statt neunzehn, siebenundzwanzig und sieben; die Zahlen sind am geschriebenen Bild nachgezählt und nicht übernommen. Der Knoten "in ihn absteigen" heißt "seinen Pfad vormerken", die Kante "war es der Ordner des angezeigten Ordners?" ist weg, die Frage nach dem Abbruch steht als eigener Knoten da, und der Deskriptormangel hat mit "nicht entschieden" einen vierten Ausgang. Der Absatz zur Kreiszählung ist ersetzt und führt die fünf Kreise einzeln auf. Inhaltlich entschieden ist dabei, dass der Deskriptormangel nach C3.13 zu "nicht entschieden" gehört und nicht nach C3.10: C3.10 gilt jetzt ausdrücklich für Gründe am Pfad, C3.13 nennt die zwei Ursachen von "nicht entschieden", und das neue C3.15 trägt die Regel. C3.8 ist um den einen gehaltenen Deskriptor erweitert und zieht die Kindprobe unter 64 Deskriptoren als Nachweis mit. Kein Abnahmekriterium war verletzt; die Zahl steigt von 75 auf 77.
