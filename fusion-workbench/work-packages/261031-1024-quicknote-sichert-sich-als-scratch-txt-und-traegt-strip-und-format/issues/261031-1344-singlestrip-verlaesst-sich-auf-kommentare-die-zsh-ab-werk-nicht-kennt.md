Singlestrip verlässt sich darauf, dass `#` einen Kommentar beginnt, und die zsh von macOS kennt ab Werk keine Kommentare in der interaktiven Eingabe
---
Seit `f39e14fb` hängt Singlestrip alles nach einem Kommentar an dessen Zeile. Modulkopf und `HowTo.md` sagen, das laufe nicht. In einer zsh ohne `setopt interactivecomments` stimmt das nicht. Das ist die Standardshell von macOS. Dort wird jedes Wort hinter `#` ein Argument des Befehls davor, auch jedes Wort aus den angehängten Folgezeilen.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Domain:** code
**Schwere:** Medium
**Cross-references:** `261031-1344-reviewer-durchsicht-singlestrip-und-format-korrektur.md`, `261031-1226-in-welche-richtung-irrt-strip-wenn-ein-umbruch-nicht-entscheidbar-ist.md`

**Wo:**
- `crates/krk-core/src/text/glaetten.rs`, Modulkopf, Abschnitt `# Singlestrip`, Absatz „Ein Kommentar haelt die Zeile bis zu ihrem Ende“: „Verbunden laeuft schlimmstenfalls nichts“.
- `HowTo.md`, Abschnitt `### Singlestrip und Format`, Absatz „Ein Kommentar (`# …`) reicht nach Singlestrip bis zum Ende der Zeile“: „Alles, was danach steht, steht im Kommentar und läuft nicht.“
- `.claude/rules/coding-was-man-nicht-sieht.md`, Absatz „Singlestrip und Format der Quicknote …“: „auch nach einem Kommentar, der damit bis zum Zeilenende reicht“.

**Gemessen** am 261031 auf diesem Gerät, eingegeben über die Standardeingabe einer interaktiven Shell:

```
zsh -f -i:   ls -la # zeigt alle dateien an
             -> ls: #: No such file or directory, ls: zeigt: …, ls: dateien: …, ls: an: …,
                und die Datei `alle` wird gelistet
zsh -f -i:   # Erst bauen cargo build
             -> zsh: command not found: #
bash --norc -i:  ls -la # zeigt alle dateien an  -> nur `ls -la`
```

`zsh -f -i -c '[[ -o interactivecomments ]]'` meldet `off`. Die eigene Konfiguration des Nutzers schaltet die Option ein (`on`), deshalb fällt das hier nicht auf. KRK wird öffentlich ausgeliefert, und `HowTo.md` setzt keine Shell voraus.

Singlestrip macht den Fall breiter, als er im eingefügten Rohtext wäre. Aus `ls -la # zeigt alle` + `  dateien an` wird eine Zeile, und in der zsh ab Werk bekommt `ls` alle fünf Wörter als Argumente. Bei einem löschenden Befehl löscht er jede gleichnamige Datei mit. Der Fall der Kommentarzeile über dem Befehl (`# … cargo build`) ist dort harmlos, aber aus einem anderen Grund als dem genannten: `#` ist kein Befehl.

**Richtung (zu entscheiden vom Planer oder Nutzer, nicht hier):** Entweder sagen `HowTo.md` und der Modulkopf, dass die Zusage eine Shell mit Kommentaren in der interaktiven Eingabe voraussetzt (bash, zsh mit `setopt interactivecomments`). Oder Singlestrip nimmt den Kommentar selbst heraus, vom `#` bis zum Ende der Ausgabezeile. Unter der Annahme „ein Befehl“ ist das entscheidbar und hängt an keiner Shell. Eine Kommentarzeile über dem Befehl ergäbe dann eine leere Ausgabe oder nur das, was vor dem `#` steht.

**Abnahme:**
- Kein Satz in `HowTo.md`, im Modulkopf von `glaetten.rs` oder in `.claude/rules/coding-was-man-nicht-sieht.md` sagt ohne Bedingung, dass Text hinter `#` nach Singlestrip nicht läuft. Entweder steht die Bedingung (Shell mit Kommentaren in der interaktiven Eingabe) daneben, oder die Ausgabe enthält den Kommentar nicht mehr.
- Fällt die zweite Richtung: Eine Probe in `crates/krk-core/tests/glaetten.rs` hält `befehl("ls -la # zeigt alle\n  dateien an")` fest, und `singlestrip_haelt_einen_kommentar_bis_zum_ende_der_zeile` wird an die neue Erwartung angepasst.
- `das_beispiel_des_nutzers_wird_zeichen_fuer_zeichen_zur_erwarteten_ausgabe` und `die_beispiele_der_anleitung_stimmen` bleiben grün.

---
Resolved: 261031, fixed in `29592073`: Singlestrip nimmt den Kommentar samt seinen Folgezeilen heraus, ausser einen Heredoc-Rumpf.
