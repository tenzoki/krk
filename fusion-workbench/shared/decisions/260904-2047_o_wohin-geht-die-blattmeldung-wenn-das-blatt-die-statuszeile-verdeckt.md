# Wohin geht die Blattmeldung, wenn das Blatt die Statuszeile verdeckt?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-ui/src/kommandos/blattmeldung.rs`, `crates/krk-ui/src/appkit/statuszeile.rs` (`HOEHE`), `crates/krk-ui/src/appkit/fenster.rs` (`ANFANGSGROESSE`, `MINDESTGROESSE`, `fensterinhalt`), `crates/krk-ui/src/appkit/blaetter/zettel.rs` (`TEXTHOEHE`), `crates/krk-ui/src/appkit/blaetter/stapelumbenennen.rs` (`VORSCHAUHOEHE`), `260904-1827_*_sichern-auf-einem-netzlaufwerk-schlaegt-still-fehl-die-datei-bleibt-ungesichert.md`, `260904-2047-blattsperre-meldet-ihre-abweisung.md`, `260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`

---

## Question

Seit dem 260904 meldet die Blattsperre ihre Abweisung, und sie meldet sie in die
Statuszeile — den einen Weg, auf dem KRK dem Nutzer eine laufende Meldung zeigt
(Nutzerentscheid vom 260804-0830, Möglichkeit 1 aus
`260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md`). Die Meldung
entsteht, während das Blatt steht. Sieht der Nutzer sie dann?

**Gemessen und nicht angenommen.** Ein Nachbau der Fenstermaße dieses Baums
(1280 × 720 aus `ANFANGSGROESSE`, Statuszeile 18 pt hoch auf 18 pt über der
Unterkante) hat am 260904 sieben Blattmaße durchgefahren und Blattrahmen gegen
Zeilenrahmen gehalten. macOS 15 setzt ein Blatt **senkrecht mittig** in sein
Elternfenster und nicht an dessen Oberkante:

| Fensterhöhe | Beigabe des Blattes | Blatt hoch | verdeckt die Zeile |
|---|---|---|---|
| 720 | 0 (Löschbestätigung) | 234 | nein |
| 720 | 317 (Stapelumbenennen) | 537 | nein |
| 720 | 332 (Notizzettel) | 552 | nein |
| 720 | 440 | 660 | nein |
| 720 | 460 | 680 | **ja** |
| 500 | 332 (Notizzettel) | 552 | **ja** |
| 336 (Mindestmaß) | 317 (Stapelumbenennen) | 537 | **ja** |

Beim Anfangsmaß ist die Zeile also frei, und zwar mit Luft: das höchste Blatt
dieses Baums ist der Notizzettel mit 332 pt Beigabe, und die Schwelle liegt
zwischen 440 und 460. **Bei einem kleiner gezogenen Fenster ist sie es nicht.**
`MINDESTGROESSE` erlaubt 336 pt Höhe, und dort verdeckt schon das
Stapelumbenennen die Zeile ganz.

Was die Meldung trotzdem trägt: sie ist eine Befehlsantwort auf Rang 1 und fällt
erst mit dem **nächsten** Tastenbefehl. Sie steht also noch da, wenn das Blatt
weg ist — sofern der Weg, den das Blatt auslöst, nicht selbst in die Zeile
schreibt. Genau das tut er in manchen Fällen: die bestätigte Löschung schreibt
ihren Fortschritt dorthin.

Die Frage ist damit: bleibt es bei der einen Zeile, oder bekommt die Blattmeldung
für den Fall des kleinen Fensters einen zweiten Weg?

## Options

1. **Es bleibt bei der Statuszeile** — der gebaute Stand.
   - Pro: keine zweite Wahrheit darüber, wo KRK etwas sagt. Der Nutzerentscheid
     vom 260804-0830 gilt unverändert. Beim Anfangsmaß, und das ist das Maß,
     unter dem KRK ausgeliefert wird, ist die Zeile frei.
   - Contra: wer sein Fenster klein zieht und dabei den Notizzettel oder das
     Stapelumbenennen offen hat, sieht den Satz nicht — und wenn der Weg des
     Blattes danach selbst in die Zeile schreibt, sieht er ihn nie.
2. **Das Blatt selbst nimmt die Meldung auf**, etwa als Zeile unter seinem
   Text.
   - Pro: sie steht dort, wo der Nutzer ohnehin hinsieht, und in jeder
     Fenstergröße.
   - Contra: jedes der elf Blätter unter `appkit/blaetter/` müsste eine Stelle
     dafür haben, und `NSAlert` gibt seinen Text nach dem Aufgehen nicht mehr
     ohne weiteres her. Das ist ein zweiter Meldungsweg neben der Statuszeile,
     also genau das, was der Entscheid vom 260804-0830 vermieden hat.
3. **Ein Ton statt eines Satzes**, `NSBeep` bei jeder gemeldeten Abweisung,
   zusätzlich zur Zeile.
   - Pro: hängt an keiner Fläche und damit an keiner Fenstergröße. Ein einziger
     Aufruf, kein Blattumbau.
   - Contra: ein Ton sagt nicht, **was** nicht ausgeführt wurde, und er sagt es
     auch dem nicht, der den Ton abgeschaltet hat. Er ersetzt den Satz nicht,
     er zeigt nur, dass es einen gibt.
4. **Das Fenstermindestmaß steigt**, bis auch das höchste Blatt die Zeile frei
   lässt.
   - Pro: die eine Zeile bleibt die eine Zeile, und die Lücke schließt sich
     ohne neuen Meldungsweg.
   - Contra: die Zahl hinge an der Höhe des höchsten Blattes und veraltete mit
     dem nächsten. Und sie nähme dem Nutzer eine Fenstergröße weg, um einer
     Meldung willen, die er selten sieht.

## Constraints

- Die Statuszeile ist seit dem 260804-0830 der eine Weg für eine laufende
  Meldung. Jede Antwort außer Möglichkeit 1 legt einen zweiten daneben und
  ändert damit einen Nutzerentscheid.
- Der Satz selbst steht an genau einer Stelle, `blattmeldung::satz`. Keine
  Antwort hier darf ihn kopieren.
- Die Meldung entsteht, **während** ein Blatt steht. Ein Weg, der selbst ein
  Blatt bräuchte, fällt damit aus: an einem Fenster hängt genau ein Blatt.
- Gemessen ist die Verdeckung, nicht die Häufigkeit. Wie oft ein Nutzer sein
  Fenster unter 500 pt zieht, weiß dieser Baum nicht.

## Recommendation

Möglichkeit 1, vorerst. Der gebaute Stand löst den gemessenen Fall vom 260904
(1280 × 720, die Zeile frei), und die Lücke betrifft eine Fenstergröße, für die
es keinen Befund gibt. Möglichkeit 3 ist der billigste Zusatz, wenn sich
herausstellt, dass die Lücke jemanden trifft; sie ersetzt den Satz nicht,
sondern zeigt auf ihn. Der Auslöser, der diese Frage wieder aufmacht, ist ein
zweiter Nutzerbefund über eine Abweisung, die niemand gesehen hat.
