# Bekommen die zwei Polaritäten des Löschzielbefunds zwei Typen?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:**
`issues/260817-1419_c_die-einzige-sicherung-gegen-den-polaritaetsfehler-ist-prosa-und-ist-warnwuerdig-hat-keinen-aufrufer.md`
(der Befund, dessen zweiter Weg diese Frage ist),
`issues/260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`
(der eingetretene Polaritätsfehler),
`issues/260817-1759_c_the-one-place-a-polarity-swap-still-compiles-carries-no-probe.md`
(die Verdrahtungsstelle, jetzt gemessen),
`history/260817-1833-reconciliation.md` („Misfiled — should be a decision")

---

## Frage

`Loeschzielbefund` beantwortet zwei Fragen entgegengesetzter Polarität mit
demselben Typ. Bei der Frage nach dem Papierkorb ist `Ja` die **Erlaubnis**,
bei den sechs Auslösern aus C3 ist `Ja` der **Warngrund**. Der Übersetzer sieht
eine Verwechslung nicht: beide Seiten tragen denselben Typ, und ein
`Unentschieden` ist ein Fixpunkt jeder Umkehrung, sodass die Zusage
„Unentschieden gilt als laut" auch bei vertauschten Polaritäten sichtbar
erfüllt bleibt. Genau dieser Fehler ist am 260817-1640 eingetreten (`ist_lokal`
lieferte die Umkehrung seines Feldes), und gefunden hat ihn der Nutzer und
keine Probe.

Der Befund `260817-1419` nennt zwei Wege dagegen. Der erste — Zählproben, die
die Prosaaussagen messen — steht seit dem 260818 an allen drei Dateien und hat
den Befund geschlossen. Der zweite ist diese Frage, und der Befund selbst nennt
ihn „die substanzielle". Er ist kein Defekt, sondern ein Entwurfsschnitt mit
Kosten, und deshalb steht er hier statt in `issues/`.

## Stand am Baum (260818)

Was heute gegen die Verwechslung hält, ist **gemessen und nicht mehr nur
Prosa**, aber es hält je Stelle und nicht am Typ:

| Stelle | was sie hält | Form |
|---|---|---|
| `appkit/volumes.rs` | fragt nicht nach der Warnwürdigkeit | Zählprobe über die Datei |
| `appkit/papierkorb.rs` | dasselbe, zweite Polarität | Zählprobe über die Datei |
| `kommandos/loeschwarnung.rs` | dasselbe, beide Polaritäten | Zählprobe über die Datei |
| `appkit/anwendung.rs` | die fünf Tatsachen erreichen ihre fünf Felder | zwei Proben über `loeschtexte` |

Keine davon macht die Verdrehung **unübersetzbar**. Sie fangen sie, nachdem
jemand sie geschrieben hat.

## Möglichkeiten

1. **Es bleibt beim einen Typ.** Kosten: keine. Preis: jede neue Frage mit
   Löschzielbefund muss ihre Polarität wieder von Hand richtig treffen, und die
   Sicherung dagegen ist eine Zählprobe je Datei, die ein Autor erst schreiben
   muss.
2. **Zwei Typen für zwei Fragen.** Ein `Warnbefund` mit `ist_warnwuerdig` und
   ein `Erlaubnisbefund` mit `erlaubt`; der gemeinsame Rumpf bleibt eine
   Aufzählung mit drei Werten. Die Verdrehung wird unübersetzbar. Kosten laut
   `260817-1419`: ein Typ mehr und eine Umrechnung an vier Prüfstellen. **Der
   Zeitvorteil ist verfallen**: als der Befund gestellt wurde, berührte Bündel C
   beide Dateien ohnehin; seit dem 260817-1722 ist es an beiden vorbei, der
   Schnitt kostet jetzt eine eigene Änderung.
3. **Ein `Loeschzielbefund::umgekehrt()`.** Am 260817-1640 vom Nutzer
   ausdrücklich verworfen, zusammen mit der Umkehrung von Hand im Aufrufer
   (`appkit/volumes.rs`, Modulkopf). Steht hier nur, damit die Aufzählung
   vollständig ist.

## Randbedingungen

- `Loeschzielbefund` leitet bewusst kein `Ord` ab: eine Ordnung wäre dort eine
  Behauptung ohne Gegenstand. Zwei Typen dürfen das nicht aufweichen.
- `krk-core` darf `objc2` nicht kennen; beide Typen müssten im Kern wohnen.
- Die drei Werte müssen erhalten bleiben. `Unentschieden` ist die eigentliche
  Zusage („Unentschieden gilt als laut"), und ein Wahrheitswert je Polarität
  wäre kein Ersatz.

## Empfehlung

Keine. Der Wahlpunkt hängt daran, wie viele Fragen mit Löschzielbefund noch
dazukommen, und das entscheidet die Planung der nächsten Runden und nicht der
Baum von heute.
