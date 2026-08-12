# Braucht die Vorschau mit gerendertem Markdown eine höhere Mindestbreite als 160 Punkte?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/fenstermodell.rs:213` (`Bereich::mindestbreite`); `crates/krk-ui/src/appkit/fenster.rs:134` (`MINDESTGROESSE`, 780 Punkte); `circles/260811-1304-statusleiste-mit-bereichsschaltern/_b_circle.md` (Runde 5, `## Closure note`); `circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_a_circle.md`, `## Parent grounding stale` vom 260812-0816

---

## Question

Die Vorschau hat eine Mindestbreite von 160 Punkten. Der Editor steht mit 320 darüber, und der Grund dafür steht im Kopf von `Bereich::mindestbreite`: das vierte Abnahmekriterium von C1 der Runde 2 verlangt „nicht schmaler, als eine Zeile Text noch lesbar ist", und bei fester Schrift in Systemgröße trägt diese Breite rund 40 Zeichen. Die Vorschau kommt mit 160 aus, „weil sie Metadaten zeigt und keine Zeilen".

Genau diese Begründung fällt mit dieser Runde weg. Die Vorschau zeigt nach ihr gerenderten Fließtext mit Überschriften und eingerückten Listen, also Zeilen, und für die gilt dasselbe Lesbarkeitsargument wie für den Editor.

**Die Zahl trägt seit der Runde 5 zwei Entscheidungen statt keiner**, und deshalb ist die Frage teurer, als sie aussieht. Bis dahin galt sie nur beim Ziehen der Trennlinie. Seither entscheidet sie erstens, ob die Vorschau überhaupt aufgeht, weil `Fenstermodell::umschalten` einen Einschaltbefehl stumm abweist, dessen Bereichssatz nicht mehr in die Fensterzeile passt; und zweitens, wer beim Schrumpfen weicht, weil `bereichsbreiten` einen Bereich unter seinem Mindestmaß aus der Verteilung nimmt und die übrigen den kleineren Rest teilen lässt. Gedeckelt ist beides an der Fensterbreite von 780 Punkten aus `MINDESTGROESSE`. Für die Vorschau bleibt dort eine Obergrenze von rund 177 Punkten, gerechnet und nicht gemessen; darüber geht sie am schmalsten zulässigen Fenster gar nicht mehr auf.

**Die 17 Punkte Luft gehören nicht dieser Runde allein.** Der vorgesehene Circle `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` braucht sie ebenfalls, und die Zahl gehört dem Bereich und nicht dem Tab: sie gilt für jeden Vorschau-Tab mit. Wer sie hier verbraucht, nimmt sie jenem Circle weg.

Die Frage hält keinen Planschritt auf und bindet zwei Circles.

## Options

1. **Die 160 bleiben.** Gerendertes Markdown wird in einer schmalen Vorschau eng, und das nimmt die Runde hin.
   - Folge: keine Berührung mit `Fenstermodell::umschalten`, `bereichsbreiten` und deren Proben, und die 17 Punkte Luft bleiben dem Web-Betrachter. Der Nutzer, dem es zu schmal ist, zieht die Trennlinie; das kann er heute schon.
   - Preis: eine Überschrift in vergrößerter Schrift und eine dreifach eingerückte Liste brauchen bei 160 Punkten mehr Umbrüche, als lesbar ist. Wie viel genau, ist an keinem laufenden Bündel gemessen.

2. **Die Mindestbreite steigt auf einen Wert unterhalb der Obergrenze, etwa 176.** Die Vorschau bleibt am schmalsten zulässigen Fenster einschaltbar.
   - Folge: gerenderter Text bekommt etwas Luft, und die beiden Entscheidungen der Runde 5 bleiben in ihrem Rahmen.
   - Preis: die 17 Punkte sind danach verbraucht. Der Web-Betrachter findet keine Luft mehr vor und muss die Obergrenze entweder selbst anheben oder mit 176 auskommen. Daneben ist die Obergrenze **gerechnet und nicht gemessen**; ist die Rechnung um wenige Punkte zu großzügig, weist `Fenstermodell::umschalten` den Einschaltbefehl am schmalen Fenster stumm ab, und das ist ein Fehler, den niemand sieht, bis er auftritt.

3. **Die Mindestbreite steigt deutlich, und `MINDESTGROESSE` des Fensters steigt mit.** Etwa 240 für die Vorschau, dazu eine größere Mindestbreite des Fensters.
   - Folge: gerenderter Text ist in jeder zulässigen Lage lesbar, und der Web-Betrachter erbt eine Vorschau, die genug Platz hat.
   - Preis: `MINDESTGROESSE` ist die Zahl, gegen die die Runde 5 alle ihre Breitenrechnungen geprüft hat, und die Bereichsleiste mit ihren acht Schaltern ist bereits an ihr bemessen (überschlagen rund 540 Punkte, gerechnet und nicht gemessen). Sie anzuheben zieht die Prüfung der Runde 5 ein zweites Mal nach sich, und die dreizehn Kriterien jener Runde, die nur am laufenden Bündel abzunehmen sind, wären erneut zu sehen.

## Constraints

- `Bereich::mindestbreite` ist eine vollständige Fallunterscheidung über `Bereich`. Eine geänderte Zahl darf keinen zweiten Rechenweg daneben anlegen.
- Die Rechnung „rund 177 Punkte Obergrenze" stammt aus dem Playmaker-Lauf vom 260812-0816 und ist ausdrücklich gerechnet und nicht gemessen. Sie am laufenden Bündel zu prüfen ist Nutzerarbeit.
- Die Runde 5 hat die Regel des Nutzers vom 260808, die Lesezeichenleiste weiche dem Editor nicht, unter der Anteilsregel fallen lassen. Wer jetzt an den Mindestbreiten dreht, dreht an derselben Stelle, an der diese Regel gefallen ist.
- Der Web-Betrachter-Circle nennt diese Runde in seinem Abschnitt `## Dependencies` nicht, weil es sie beim Anlegen jenes Circles noch nicht gab. Die Kante läuft von hier dorthin.

## Recommendation

**Wir empfehlen Möglichkeit 1 für diese Runde.** Der Grund ist nicht, dass 160 Punkte genügen, sondern dass niemand weiß, ob sie es tun: die Obergrenze von 177 ist gerechnet, die Schalterbreite von 540 ist gerechnet, und die Enge des gerenderten Markdown ist ungemessen. Drei ungemessene Zahlen gegeneinander zu verrechnen und daraus eine neue Mindestbreite abzuleiten, ist der Weg, auf dem dieses Projekt bei L9 zweimal eine Zusage gesenkt und danach nicht mehr gewusst hat, warum.

Der Auslöser, der die Frage wieder aufmacht, ist derselbe wie bei L9: ein Lauf am laufenden Bündel. Wer die Vorschau nach dieser Runde mit gerendertem Markdown bei 160 Punkten ansieht, weiß in einer Minute, was drei Rechnungen nicht beantworten.

`inference:` Wir vermuten, dass 160 Punkte für gerenderten Fließtext zu schmal sind, weil der Editor mit derselben Begründung 320 trägt. Der Editor zeigt allerdings feste Schrift und die Vorschau proportionale, die mehr Zeichen in dieselbe Breite bringt. Geprüft ist keines von beidem.


## Antwort 260812-1105

**Moeglichkeit 1.**

Die Mindestbreite der Vorschau bleibt in dieser Runde bei 160 Punkten.

Nicht, weil 160 genuegen, sondern weil niemand weiss, ob sie es tun: die Obergrenze von 177 ist
gerechnet, die Schalterbreite von 540 ist gerechnet, und die Enge des gerenderten Markdown ist
ungemessen. Drei ungemessene Zahlen gegeneinander zu verrechnen und daraus eine Zusage abzuleiten,
ist der Weg, auf dem dieses Projekt bei L9 zweimal eine Zahl gesenkt und danach nicht mehr gewusst
hat, warum.

**Der Ausloeser, der die Frage wieder aufmacht, ist ein Lauf am laufenden Buendel.** Wer die
Vorschau nach dieser Runde mit gerendertem Markdown bei 160 Punkten ansieht, weiss in einer
Minute, was drei Rechnungen nicht beantworten. Der Spec nennt das als Beobachtungspunkt.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:
