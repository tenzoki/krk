# Hängt der Gitbefund zusätzlich an einem Beobachter auf `.git`?

---
**Domain:** code
**Filed by:** shaper, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260830-1251_*_spec-git-bereich-liest-status-branch-verlauf.md` (Festlegung A9, Kriterium C7.10); `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` (Open Questions, dritter Punkt); `crates/krk-ui/src/auffrischung.rs` (der eine Auffrischungspfad); `crates/krk-ui/src/appkit/fsevents.rs`

---

## Question

Der Spec der Runde 23 hängt den Gitbefund an `auffrischung::ordner_neu_lesen`, den einen Weg, auf dem ein Dateifenster seinen Ordner noch einmal liest. Damit ist die Anzeige genau dann aktuell, wenn die Dateiliste es ist, und ein zweiter Beobachter entsteht nicht. Der Preis fällt in einem Fall an, der nicht selten ist: wer in einem Terminal committet, während KRK einen **Unterordner** des Repositorys zeigt, ändert nichts in diesem Unterordner. Der FSEvents-Strom meldet dann nichts, `ordner_neu_lesen` läuft nicht, und der Git-Bereich zeigt den Stand von vorher, bis der Nutzer den Ordner wechselt.

Zu entscheiden ist die Frage nicht vor dem Plan, sondern nach der ersten Abnahme: erst am laufenden Bündel zeigt sich, wie oft der veraltete Stand auffällt. Der Spec fährt bis dahin auf Möglichkeit 1.

## Options

1. **Kein zweiter Beobachter; der Gitbefund hängt am einen Auffrischungspfad.**
   - Pros: ein Weg und kein zweiter daneben, wie es der Modulkopf von `auffrischung.rs` für die zwei vorhandenen Auslöser ausschreibt; keine neue Bindung an das Dateisystem und keine neue Fehlerquelle; die Runde bleibt klein.
   - Cons: ein Commit von außen wird erst beim nächsten Neulesen des angezeigten Ordners sichtbar. Wie oft das auffällt, ist ungemessen.
2. **Der bestehende `FSEventStream` beobachtet zusätzlich das `.git` des Repositorys, in dem der angezeigte Ordner liegt.** Der Strom beobachtet seit der Editor-Runde schon einen dritten Ordner, nämlich den der Datei im Editor; ein vierter wäre dieselbe Bauform.
   - Pros: der Git-Bereich ist so aktuell wie `git status` in einem Terminal daneben; die Bauform steht vor und braucht keinen zweiten Strom.
   - Cons: `sichtbare_ordner` bekommt eine vierte Quelle, und die Menge der beobachteten Ordner wechselt fortan bei jedem Ordnerwechsel zwischen zwei Repositorys; `.git` meldet bei jedem Git-Befehl viele Ereignisse, und der Statuslauf müsste gegen ein Aufschaukeln gedämpft werden.
3. **Ein Befehl, der den Gitbefund von Hand neu holt.** Eine Taste, die allein den Git-Bereich auffrischt.
   - Pros: der Nutzer entscheidet, wann es ihn kostet; keine Beobachtung, keine Dämpfung.
   - Cons: ein Befehl für etwas, das der Nutzer nicht sieht, solange er ihn nicht drückt; die Belegung bekäme eine dritte Kombination in dieser Runde, und die Maxime „supersimpel" spricht dagegen, ein Aktualitätsproblem an den Nutzer weiterzugeben.

## Constraints

- `auffrischung::ordner_neu_lesen` bleibt der eine Weg, auf dem ein Dateifenster seinen Ordner neu liest; jede Antwort, die einen zweiten einführt, bricht die Zusage im Modulkopf jener Datei.
- Es bleibt bei einem `FSEventStream`; ein zweiter daneben ist im Baum schon einmal ausgeschlossen worden.
- Die Antwort ändert keine der zehn Zeitzusagen: ein Statuslauf über den angezeigten Ordner kostet gemessen 12 bis 164 ms und läuft nebenläufig.
- Stufe A bleibt schreibfrei; keine Möglichkeit oben fasst ein Repository an.

## Recommendation

Wir empfehlen Möglichkeit 1 für diese Runde, mit Wiedervorlage nach der ersten Abnahme am Bündel. Ob der veraltete Stand stört, ist eine Beobachtung am laufenden Programm und keine Ableitung; einen zweiten Beobachter samt Dämpfung zu bauen, bevor jemand den Mangel gespürt hat, ist die Reihenfolge verkehrt herum. Möglichkeit 3 raten wir ab: sie macht die Aktualität zur Aufgabe des Nutzers.
