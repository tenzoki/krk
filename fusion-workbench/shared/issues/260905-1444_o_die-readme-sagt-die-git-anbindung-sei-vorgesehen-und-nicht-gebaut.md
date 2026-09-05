Die README sagt, die Git-Anbindung sei vorgesehen und nicht gebaut

---
`README.md:6` beschreibt KRK und schließt mit „Eine Git-Anbindung ist vorgesehen und noch nicht gebaut."

Sie ist gebaut. Die Runde 23 (`circles/260830-1045-git-bereich-liest-status-branch-verlauf`, beschränkt geschlossen am 260831-2024) hat den lesenden Git-Bereich gebracht: Branch, Statuszusammenfassung, Verlaufsliste mit Commit-Einzelheiten, dazu die Markenspalte in beiden Dateifenstern. Ausgeliefert in `v1.5.0`, seither in drei weiteren Fassungen fortgeschrieben.

**Der Satz ist nicht bloß veraltet, sondern für den einzigen Leser irreführend, den die README hat:** wer KRK herunterlädt, liest ihn als Auskunft über die Fassung, die er gerade bekommt.

**Die Abgrenzung, die er meint, gilt weiter und gehört in den Nachfolgesatz:** gebaut ist die **lesende** Stufe. Hinzufügen, Committen, Änderungen verwerfen und der Versions-Schieberegler aus der Directive der Runde 1 sind nicht gebaut, und die offene Entscheidung `shared/decisions/260802-0842_*_git-verwerfen-bedeutung.md` bindet sie.

**Gefunden wurde die Stelle beim Schreiben von `HowTo.md`**, deren Autor drei überholte Stellen meldete. Die zwei anderen — die Rundentabelle in `CLAUDE.md` und der Absatz über die gewachsenen Aufzählungen — waren bereits nachgezogen; er hat einen älteren Stand gelesen. Diese hier ist die einzige der drei, die zutrifft.

**Abnahmetest:** `README.md` sagt, was von der Git-Anbindung gebaut ist und was nicht, ohne eine Zahl zu nennen, die mit der nächsten Runde falsch wird.

---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Domain:** code
