Strip kürzt nachlaufenden Leerraum auch im Rumpf eines Heredocs, und `<<` in einer Arithmetik öffnet für Strip einen Heredoc
---
Zwei Stellen in `glaetten::befehl`, an denen der Shell-Automat seine Zusage aus E11 nicht hält. Das Heredoc-Problem ändert Inhalt, der für die Shell Bedeutung hat. Das Arithmetik-Problem liegt auf der sicheren Seite: es verbindet zu wenig.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md` (E11), `261031-1156-reviewer-abschlussdurchsicht-quicknote-scratch-strip-format.md`

**1. Nachlaufender Leerraum im Rumpf.** `zerlegen` (`crates/krk-core/src/text/glaetten.rs`) kürzt mit `trim_end()` jede Zeile, bevor der Automat die Lage kennt. Im Rumpf eines Heredocs und in einer offenen Anführung ist dieser Leerraum Inhalt. Für die zwei Leerzeichen eines harten Umbruchs in Markdown gilt das ebenso wie für einen Wert, der mit Leerzeichen endet. Gemessen am Stand `be1a725d`:

```
Eingabe:  cat <<EOF > a.md\n  zeile mit hartem Umbruch  \n  weiter\n  EOF
Ausgabe:  cat <<EOF > a.md\nzeile mit hartem Umbruch\nweiter\nEOF
```

Der Modulkopf sagt „was fuer die Shell Inhalt ist, bleibt“. E11 begründet das Kürzen allein als Kopierartefakt. Ob der Chat nachlaufende Leerzeichen eines Codeblocks mitliefert, ist nicht entscheidbar. Kürzen außerhalb von Rumpf und Anführung genügt aber für den Zweck, den E11 nennt, das Leerzeichen hinter `\`.

**2. `<<` in `$(( … ))`.** `zeile_lesen` öffnet bei jedem `<<` außerhalb von Anführungen einen Heredoc. In `echo $((1 << 4))` liest `heredoc_lesen` das Wort `4` als Endmarke, und jede weitere Zeile gilt als Rumpf: kein Terminalumbruch wird mehr verbunden, keine `\`-Fortsetzung entfernt. Gemessen: aus `echo $((1 << 4))\n  cat a.txt | … | awk '{print $2}'\n  -n` bleibt `-n` auf eigener Zeile.

**Abnahme:**
- Eine Probe in `crates/krk-core/tests/glaetten.rs` hält, dass nachlaufender Leerraum in einem Heredoc-Rumpf und in einer offenen Anführung erhalten bleibt und außerhalb weiter fällt; `strip_entfernt_backslash_fortsetzungen_ohne_die_bedeutung_zu_aendern` bleibt grün.
- Eine Probe hält, dass `$((a << b))` keinen Heredoc öffnet und ein Terminalumbruch danach verbunden wird.
- `strip_ist_idempotent` bleibt grün.
