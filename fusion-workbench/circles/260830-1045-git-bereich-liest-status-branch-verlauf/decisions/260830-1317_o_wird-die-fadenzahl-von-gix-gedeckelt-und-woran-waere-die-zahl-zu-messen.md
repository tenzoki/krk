# Wird die Fadenzahl von `gix` gedeckelt, und woran wäre die Zahl zu messen?

---
**Domain:** code
**Filed by:** planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md` (C7.1, C7.2, `## Open for Planner`, siebter Punkt); `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md` (Entscheidung 7); `260830-1006-gix-als-git-anbindung-stufe-a.md` (Frage 6); `crates/krk-bench/src/messen.rs` (L1, L10)

---

## Question

`gix` verteilt den Statuslauf über die Merkmale `parallel` und `max-performance-safe` auf mehrere
Fäden und nimmt ohne Angabe so viele, wie das Gerät Kerne hat.
`Platform::index_worktree_options_mut` bietet ein `thread_limit`, mit dem sich die Zahl deckeln
lässt. Der Spec überlässt die Frage dem Planner; der Plan setzt keinen Deckel und legt sie hier ab,
weil sie die Stufe B mitbindet und weil ihre Antwort an einer Messung hängt, die kein Agent fahren
kann.

Die Lage: auf dem Referenzgerät mit acht Kernen können bis zu **zwei** Statusläufe nebeneinander
stehen, einer je Dateifenster, dazu bis zu zwei Durchläufe des Filters. Der Hauptfaden muss
währenddessen Bilder von sechzehn Millisekunden halten (L1), und die erste Bildschirmseite eines
Ordners mit hunderttausend Einträgen hat hundert Millisekunden (L10). Der Statuslauf liegt nicht
auf dem Hauptfaden, aber er teilt sich mit ihm den Planer des Betriebssystems.

**Was gemessen ist, ist der Deskriptorstand und nicht die Fadenzahl.** Der Lauf kommt unter
`ulimit -n 32` in derselben Zeit durch, und sein Höchststand liegt im niedrigen zweistelligen
Bereich (Analyse, Frage 6). Über die Wirkung auf den Zeichendurchgang sagt das nichts.

## Options

1. **Kein Deckel; die Stelle steht namentlich im Modulkopf als erster Hebel.**
   - Pros: Es wird keine Zahl gebaut, die niemand gemessen hat — dieselbe Reihenfolge, die der
     Datensatz zum Rückschreiben des Index für den anderen ungemessenen Posten schon gewählt hat.
     Der Statuslauf bleibt so schnell, wie er gemessen ist (12 bis 164 ms). Fällt C7.2 am
     Abnahmelauf negativ aus, ist der Deckel eine Zeile und kein Umbau.
   - Cons: Der Abnahmelauf ist Nutzerarbeit, und bis er gefahren ist, steht die Wirkung auf L1 offen.
     Ein Nutzer mit einem Repository von hunderttausend Einträgen in beiden Dateifenstern sieht sie
     als erster.
2. **Ein fester Deckel, etwa die Hälfte der Kerne.**
   - Pros: Der Hauptfaden behält Luft, auch wenn zwei Läufe nebeneinander stehen.
   - Cons: Die Zahl ist geraten. Sie kostet Durchsatz in jedem Fall und nützt nur in dem, der
     ungemessen ist; auf einem Gerät mit vier Kernen ist die Hälfte zwei, auf einem mit sechzehn
     acht, und keine der beiden Lagen ist geprüft. Eine Formel über die Kernzahl ist eine zweite
     ungemessene Annahme neben der ersten.
3. **Ein Deckel, der von der Zahl der laufenden Läufe abhängt.**
   - Pros: Träfe die Lage, die wirklich eng ist, nämlich zwei Läufe nebeneinander.
   - Cons: Die Zahl der laufenden Läufe steht in `krk-ui` und der Deckel in `krk-core`; der Wert
     müsste je Lauf hereingereicht werden, und ein Lauf, der startet, während ein anderer schon
     läuft, ändert dessen Deckel nicht mehr. Das ist Mechanik für eine Größe, deren Wirkung
     niemand gemessen hat.

## Constraints

- Die Frage „wie viele Fäden sind richtig" ist aus den Eingaben, die der Mechanismus hat, nicht
  entscheidbar: sie hängt an der Kernzahl, an der Zahl gleichzeitiger Läufe und an der übrigen Last.
  Die Frage dahinter — nimmt der Lauf dem Zeichendurchgang Bilder weg — ist entscheidbar, aber nur
  am laufenden Bündel, und der Abnahmelauf verlangt KRK im Vordergrund.
- Die Stufe A schreibt nicht; keine Möglichkeit oben ändert daran etwas.
- Keine der zehn Zeitzusagen wird von dieser Runde angefasst, und die Antwort ändert daran nichts:
  der teuerste gemessene Statuslauf liegt bei 164 ms je Ordner, das Budget von L10 für das
  vollständige Lesen bei 4 000 ms.
- Ein Deckel verlangsamt den Lauf messbar; die 155 ms für hunderttausend Einträge sind mit voller
  Parallelität gemessen.

## Recommendation

Wir empfehlen Möglichkeit 1 für diese Runde, mit ausdrücklicher Wiedervorlage nach dem ersten
Abnahmelauf. Die Reihenfolge ist dieselbe wie beim Rückschreiben des Index: erst messen, dann bauen.
Der Gitleser nennt `thread_limit` deshalb namentlich in seinem Modulkopf, mit dem Satz, dass es der
erste Hebel ist und heute keinen Wert setzt — damit findet die Wiedervorlage die Stelle, ohne sie
zu suchen.

Was die Wiedervorlage braucht, ist eine Beobachtung und keine Ableitung: ein Ordner mit
hunderttausend Einträgen in einem Repository, in beiden Dateifenstern, mit eingeschalteter
Markenspalte, und die Frage, ob die erste Bildschirmseite später dasteht als vor der Runde (C7.2).
