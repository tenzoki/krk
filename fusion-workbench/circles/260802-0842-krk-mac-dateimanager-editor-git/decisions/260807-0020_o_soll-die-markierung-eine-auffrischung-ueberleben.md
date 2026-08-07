# Soll die Markierung eine Auffrischung überleben?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:**
`crates/krk-core/src/verzeichnis/modell.rs` (`ersatz_einloesen`, `markiert`),
`crates/krk-ui/src/tabs.rs` (`Tabinhalt::wunschauswahl`, `aktiven_neu_lesen`),
`issues/260806-1445_c_ein-schnelles-verschieben-koennte-dieselbe-meldelawine-ausloesen-wie-das-stapel-umbenennen.md`

---

## Question

C9 sagt für eine Auffrischung zu: "Auswahl und Bildlaufposition überstehen den
Vorgang, soweit die Einträge noch existieren." Die **Markierung** aus C2 nennt
die Zusage nicht, und umgesetzt ist sie auch nicht: sie fällt mit jedem
Lesevorgang.

Der Grund ist mechanisch. Die Auswahl wird über den **Namen** des Eintrags
getragen (`Tabinhalt::wunschauswahl`), und der überlebt einen Lesevorgang.
Die Markierung ist eine Menge von Eintragsindizes; ein Index über einen
Lesevorgang hinweg zeigte danach auf einen beliebigen anderen Eintrag, also
fällt sie.

Aufgefallen bei der Umstellung der Lesestelle am 260807. Die Frage ist dadurch
nicht neu und nicht dringlicher geworden — sie ist nur sichtbar geworden, weil
dieselbe Zeile jetzt Auswahl und Markierung zusammen wegwirft.

## Was der Nutzer erlebt

Er markiert acht Dateien, eine fremde Änderung im angezeigten Ordner löst eine
Auffrischung aus, und die Markierung ist weg. Beobachtet ist das nicht; der
Eintrag hält den Befund, nicht eine Beschwerde.

## Options

1. **So lassen.** Die Markierung ist kurzlebig und gehört dem Augenblick vor
   einer Dateioperation.
   - Pro: Nichts zu bauen. Eine Markierung, die einen geänderten Ordnerinhalt
     überlebt, könnte auf Einträge zeigen, die der Nutzer so nicht mehr meint.
   - Contra: Ein Ordner, in dem sich etwas rührt, lässt sich nicht in Ruhe
     markieren.
2. **Über die Namen tragen**, wie die Auswahl. Beim Beginn eines Lesevorgangs
   die Namen der markierten Einträge merken, beim Abschluss wieder auflösen.
   - Pro: Derselbe Mechanismus wie bei der Auswahl, kein zweiter Weg.
   - Contra: Bei 100.000 markierten Einträgen sind das 100.000 Zeichenketten,
     die kopiert und danach nachgeschlagen werden. Das fällt in die Spanne, die
     L3 und L10 messen.
3. **Nur bei einer Auffrischung tragen, nicht bei einer Navigation.** Eine
   Navigation wechselt den Ordner, dort ist die Markierung ohnehin sinnlos.
   - Pro: Der teure Fall aus Option 2 tritt seltener ein.
   - Contra: Zwei Wege statt einem, und die Kosten bleiben dieselbe Größe, wenn
     der Fall eintritt.

## Constraints

- L3 (400 ms für 10.000 Einträge) und L10 (4000 ms für 100.000) decken den
  vollständigen Lesevorgang. Jede Lösung muss innerhalb dieser Zusagen bleiben,
  und der schlechteste Fall ist "alles markiert".
- Der Spec stellt für die Markierung keine Zusage über eine Auffrischung
  hinweg. Ein Nachzug im Spec gehört zu jeder Antwort außer Option 1.

## Recommendation

Keine. Der Befund ist unbeobachtet, und ohne eine Beschwerde des Nutzers ist
nicht zu entscheiden, ob der Fall überhaupt vorkommt. Wer ihn kennt,
entscheidet ihn in einem Satz; wer ihn nicht kennt, baut sonst Option 2 für
niemanden.

---
Answered:
Implemented:
Deferred:
Superseded by:
