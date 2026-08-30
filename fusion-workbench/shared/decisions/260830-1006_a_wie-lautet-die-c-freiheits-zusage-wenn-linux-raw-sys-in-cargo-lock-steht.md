# Wie lautet die C-Freiheits-Zusage, wenn `linux-raw-sys` in `Cargo.lock` steht?

---
**Domain:** code
**Filed by:** analyst, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `shared/analyses/260830-1006-gix-als-git-anbindung-stufe-a.md` (Frage 2); `CLAUDE.md`, Abschnitt `## Projektstand`, Absatz zu `syntect`, `two-face` und `zip`; die Begründungen zu `zip`, `syntect` und `objc2-pdf-kit` in der Wurzel-`Cargo.toml`

---

## Question

Die Zusage aus der Technologiewahl steht heute an fünf Stellen in derselben Form: „`Cargo.lock` führt kein `cc` und außer `windows-sys` kein `-sys`-Paket." Sie ist der Prüfsatz, an dem dieses Projekt bisher jede fremde Kiste gemessen hat, und sie hat bei `syntect`, `two-face`, `zip` und `objc2-pdf-kit` gehalten.

`gix` bricht ihren Wortlaut, ohne ihre Sache zu brechen. Über `rustix` kommt `linux-raw-sys` in `Cargo.lock`. Das Paket hängt am Linux-Ziel, kommt auf keinem der beiden Mac-Ziele im Abhängigkeitsbaum an und übersetzt dort nie; es enthält keinen C-Code, sondern erzeugte Rust-Definitionen. Gemessen am 260830 mit `cargo tree -e normal,build` gegen `x86_64-apple-darwin` und `aarch64-apple-darwin`: in beiden Bäumen weder `cc` noch ein Name auf `-sys`.

Die Frage ist vor der Aufnahme von `gix` zu entscheiden, weil sonst fünf Prosastellen mit dem ersten Commit falsch werden und die nächste Durchsicht sie als Befund einsammelt.

## Options

1. **Die Zusage auf das Bauziel beziehen** — „auf den beiden Mac-Zielen kommt weder `cc` noch ein `-sys`-Paket im Baum an; `Cargo.lock` führt daneben `windows-sys` und `linux-raw-sys`, beide an fremden Zielen."
   - Pros: sagt, was die Zusage immer gemeint hat, nämlich dass sich die **Bauvoraussetzungen** nicht ändern; das Prüfkommando wird `cargo tree --target <ziel> -e normal,build` statt `grep` in `Cargo.lock` und misst damit die Sache statt ihres Schattens; `windows-sys` verliert seinen Ausnahmestatus und wird zum ersten Fall einer Regel statt zur Ausnahme von einer.
   - Cons: das Prüfkommando ist teurer als ein `grep` und muss zweimal laufen, einmal je Architektur; fünf Prosastellen sind nachzuziehen.
2. **Die Ausnahmeliste um `linux-raw-sys` erweitern** — „außer `windows-sys` und `linux-raw-sys` kein `-sys`-Paket".
   - Pros: kleinster Eingriff, `grep` bleibt das Prüfmittel.
   - Cons: die Liste wächst mit jeder fremden Kiste, die ein zielgebundenes `-sys`-Paket mitbringt, und jede Erweiterung ist eine eigene Entscheidung; die Regel misst weiterhin `Cargo.lock` und nicht das Bauziel, sagt also weiterhin nicht, was sie meint.
3. **Die Zusage unverändert lassen und `gix` an ihr scheitern lassen** — `rustix` wäre auszuschließen, was ohne Gabelung von `gix` nicht geht.
   - Pros: keine Änderung an einer bindenden Formulierung.
   - Cons: schließt `gix` aus, obwohl auf dem Bauziel nichts in C übersetzt; der Wortlaut entschiede gegen die Sache.

## Constraints

- Was die Zusage schützt, ist nachzulesen bei `syntect`: der Vorgabesatz `default-onig` hätte Oniguruma hereingezogen und „damit die Bauvoraussetzungen des Projekts geändert". Der Schutzgegenstand ist der Bau auf diesem Gerät und nicht der Inhalt von `Cargo.lock`.
- Die Antwort gilt für alle Stellen gleich; zwei Formulierungen nebeneinander wären zwei Wahrheiten.
- `linux-raw-sys` ist nicht abwählbar: es hängt an `rustix`, und `rustix` hängt an `gix-fs` und `gix-tempfile` als gewöhnliche Abhängigkeit ohne Merkmalsschalter.

## Recommendation

Wir empfehlen Möglichkeit 1. Die Zusage misst heute einen Stellvertreter für das, was sie sichern will, und der Stellvertreter versagt beim ersten zielgebundenen Paket. Möglichkeit 2 verlängert eine Liste, deren Wachstum keine Grenze hat, und lässt den Messfehler stehen.

---
Answered: shared/history/260830-0950-orchestrator-session.md:91 — Möglichkeit 1: die Zusage bezieht sich auf das Bauziel; Prüfmittel ist `cargo tree --target <ziel> -e normal,build`, `windows-sys` wird vom Ausnahmefall zum ersten Fall der Regel, fünf Prosastellen sind im Plan nachzuziehen.
