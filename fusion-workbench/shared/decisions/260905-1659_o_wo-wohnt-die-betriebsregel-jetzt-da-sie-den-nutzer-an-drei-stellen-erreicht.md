# Wo wohnt die Betriebsregel für den Austausch der App, jetzt da sie den Nutzer an drei Stellen erreicht?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260905-1658_*_claude-md-nennt-zwei-stellen-an-denen-der-nutzer-die-betriebsregel-liest-mit-der-anleitung-im-paket-sind-es-drei.md`, `260826-1444_*_der-releasetext-nennt-vier-von-sieben-ablagedateien-und-die-probe-haelt-die-unvollstaendige-liste.md`, `260820-2242-lesezeichenverlust-nach-installation.md`

---

## Question

Die Regel „die neue Fassung über die alte kopieren, die alte nicht vorher löschen" stammt aus der Untersuchung des Lesezeichenverlusts und steht ausformuliert an drei Stellen, die der Nutzer beim Installieren liest: in den ersten dreißig Zeilen der `README.md`, im `RELEASETEXT` von `xtask/src/veroeffentlichung.rs`, und seit dem 260905 in `HowTo.md`, die im Releasepaket neben der App mitreist. Die Frage stellt sich jetzt, weil die dritte Stelle erst mit dem Mitreisen der Anleitung eine Stelle im Sinne der Begründung geworden ist: vorher las sie nur, wer im Quellbaum liest. Nur eine der drei ist von einer Probe gehalten, und der Befund `260826-1444` zeigt an genau jenem `RELEASETEXT`, dass eine gehaltene Stelle trotzdem von der Wahrheit weglaufen kann.

## Options

1. **Drei Stellen bleiben, jede mit ihrem eigenen Lesemoment** — die Releaseseite liest, wer lädt; die `README.md`, wer im Vorhaben stöbert; `HowTo.md`, wer die App schon hat.
   - Pro: Jede erreicht ihren Leser ohne Umweg. Kein Verweis, der ins Leere zeigt, wenn eine der drei Dateien allein weitergegeben wird — und genau das geschieht mit `HowTo.md` im Paket.
   - Contra: Drei Wortlaute derselben Regel laufen auseinander. Nichts hält sie aneinander; die Zählprobe des `RELEASETEXT` hält nur ihn.
2. **`HowTo.md` wird die eine Quelle, `README.md` und `RELEASETEXT` verweisen darauf** — der Wortlaut steht einmal.
   - Pro: Eine Wahrheit, ein Ort. Die Datei reist ohnehin mit der App.
   - Contra: Der Verweis trägt nicht im Augenblick, in dem er zählt: wer auf der Releaseseite steht, hat die Anleitung noch nicht entpackt.
3. **Der `RELEASETEXT` bleibt der Wortlaut, und `HowTo.md` bekommt die Regel beim Packen eingesetzt** — dieselbe Fügestelle, die heute die Versionszahl einsetzt.
   - Pro: Ein Wortlaut im Code, zwei Ausgaben daraus, und die bestehende Probe hält beide.
   - Contra: Die ausgelieferte `HowTo.md` unterscheidet sich dann von der im Quellbaum; wer sie dort liest, liest eine Lücke. Und `xtask` schriebe in eine Datei, die dem Nutzer gehört.

## Constraints

Der Wortlaut auf der Releaseseite bleibt vollständig lesbar ohne jeden Download: das ist die Zusage aus der Untersuchung, und ein Verweis erfüllt sie nicht. Keine der drei Dateien darf die Regel verlieren, wenn sie allein weitergegeben wird.

## Recommendation

Keine. Der Weg 3 klingt sparsam und macht die ausgelieferte Anleitung zu einer anderen Datei als die eingecheckte; Weg 2 bricht die Zusage aus den Constraints an genau der Stelle, an der sie erkämpft wurde. Weg 1 ist der heutige Zustand und kostet die Pflege. Die Wahl hängt daran, was dem Nutzer mehr wert ist: ein Wortlaut oder drei erreichbare Leser.
