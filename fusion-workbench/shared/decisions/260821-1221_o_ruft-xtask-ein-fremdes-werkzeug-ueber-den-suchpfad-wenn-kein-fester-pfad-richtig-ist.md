# Ruft `xtask` ein fremdes Werkzeug über den Suchpfad, wenn kein fester Pfad richtig sein kann?

---
**Domain:** code
**Filed by:** planner
**Cross-references:** `shared/planning/260821-1221_o_plan-artefakt-und-release.md` (Schritt 2), `shared/planning/260821-1115_o_spec-artefakt-und-release.md` (C5), `xtask/src/git.rs`, `xtask/src/sign.rs`, `xtask/src/beglaubigung.rs`

---

## Frage

Das Bauwerkzeug ruft heute jedes fremde Werkzeug mit vollem Pfad: `/usr/bin/git`,
`/usr/bin/codesign`, `/usr/bin/ditto`, `/usr/bin/xcrun`, `/usr/bin/security`. Die Gewohnheit ist
in `git.rs` ausdrücklich benannt, „absoluter Pfad, weil der Baum jedes Systemwerkzeug so ruft",
und eine Probe zählt die eine git-Aufrufstelle sogar an der ausgeschriebenen Zeichenfolge
`Command::new("/usr/bin/git")`.

Mit der achten Station der Auslieferungskette kommt ein Werkzeug hinzu, für das kein fester Pfad
richtig sein kann. `gh` gehört nicht zu macOS, wird nachinstalliert und liegt bei einer
Homebrew-Installation auf Apple Silicon unter `/opt/homebrew/bin/gh`, auf Intel unter
`/usr/local/bin/gh`. Das Projekt baut für beide Architekturen. Ein fester Pfad wäre auf einer der
beiden falsch, und ein Werkzeug, das über eine andere Quelle installiert wurde, träfe er
ohnehin nicht.

Die Frage muss jetzt beantwortet werden, weil sie über die achte Station hinausreicht. Jedes
weitere fremde Werkzeug, das dieses Projekt später ruft und das nicht mit macOS kommt, steht vor
derselben Wahl, und ohne eine Regel entscheidet sie jedes Mal der Zufall der Sitzung.

## Optionen

1. **Der Suchpfad, mit einer Regel im Modulkopf.** `Command::new("gh")` sucht auf `PATH`. Die
   Regel lautet: mit macOS ausgelieferte Werkzeuge werden mit vollem Pfad gerufen,
   nachinstallierte über den Suchpfad, und die Ausnahme wird an ihrem Aufrufort begründet.
   - Pro: die einzige Fassung, die auf beiden Mac-Architekturen und unabhängig von der
     Installationsquelle trifft. Der Fehlerfall ist sauber: findet die Suche nichts, scheitert
     der Start, und genau daran erkennt die Prüfung aus C5.1 das fehlende Werkzeug.
   - Contra: `PATH` ist Umgebung und damit von außen steuerbar. Wer den Namen `gh` auf etwas
     anderes zeigen lässt, wird gerufen. Die Gewohnheit des Baums bekommt eine Ausnahme, und
     eine Ausnahme, die nur an einer Stelle begründet steht, wird beim zweiten Mal übersehen.
2. **Eine Stufensuche wie bei der Signaturidentität.** Zwei feste Pfade der Reihe nach probieren,
   `/opt/homebrew/bin/gh` und `/usr/local/bin/gh`, und erst danach aufgeben.
   - Pro: kein `PATH` im Spiel, und das Muster steht in diesem Baum schon: die Identitätssuche in
     `sign.rs` fährt drei Stufen und bricht ohne Treffer ab.
   - Contra: eine Liste, die jemand pflegen muss, und genau diese Bauart hat das Projekt bei
     `git::STAND` schon einmal ausdrücklich verworfen, „eine Liste der bauwirksamen Ordner müsste
     jemand pflegen, und sie zu ergänzen zu vergessen ist die zweite Art, eine Prüfung im
     Vorbeigehen zu verlieren". Eine Installation über MacPorts, über Nix oder von Hand nach
     `~/bin` trifft sie nicht.
3. **Der Suchpfad, aber der gefundene Pfad wird im Lauf genannt.** Wie Option 1, zusätzlich meldet
   der Lauf einmal, welches `gh` er gefunden hat.
   - Pro: nimmt Option 1 ihre einzige echte Schwäche, dass niemand sieht, welches Werkzeug
     gerufen wurde. Kostet einen Aufruf und eine Zeile Ausgabe.
   - Contra: eine vierte Prozessausführung an einer Stelle, die schon drei hat, und die Auskunft
     nützt allein im Störungsfall.

## Randbedingungen

- `xtask` führt keine fremde Kiste und soll keine bekommen. Jede Antwort kommt mit
  `std::process::Command` aus.
- Die Probe `xtask_ruft_git_an_genau_einer_stelle` zählt an den Zeichenfolgen
  `Command::new("/usr/bin/git")` und `Command::new("git")`. Keine Antwort darf sie brechen.
- Die Prüfung aus C5.1 muss ein fehlendes `gh` erkennen, bevor gepackt oder geschoben wird.
- Was immer entschieden wird, gilt für das nächste nachinstallierte Werkzeug mit.

## Empfehlung

Option 1, mit der Regel ausgeschrieben. Sie ist die einzige, die auf beiden Architekturen und
über alle Installationswege trifft, und sie erkauft das mit einer Abhängigkeit von der Umgebung,
die dieses Werkzeug ohnehin schon hat: `make` setzt `PATH`, um `cargo` überhaupt zu finden. Der
Sicherheitseinwand gegen `PATH` trägt hier wenig, weil `xtask` von demselben Nutzer in demselben
Terminal läuft, der auch `cargo` von dort holt.

Option 2 sieht nach der Gewohnheit des Baums aus und ist es nicht: sie ist eine Pflegeliste, und
dieses Projekt hat gegen Pflegelisten an anderer Stelle schon entschieden. Option 3 ist Option 1
plus eine Auskunft; sie wäre nachzuziehen, sobald der erste Störungsfall zeigt, dass die Auskunft
fehlt.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:
