Der dritte der drei Sätze im Modulkopf von `git.rs` widerspricht sich selbst

---

Der mit `94855a7` geschriebene Absatz „Wie stark die Zusage danach ist, in drei Sätzen"
(`xtask/src/git.rs:30-37`) ist die einzige Stelle im Baum, die ausschreibt, wie weit die
Absicherung um `git push` trägt. Sein dritter Satz sagt in einem Atemzug, dass **nichts** einen
zweiten Prozessaufruf hält, und dass eine benannte Probe ihn hält.

---

**Gemessen am Baumstand `94855a7`.**

## Der Satz

`git.rs:33-37`:

```
//! erzwingenden Verweis mit `+` traegt. Was **nichts** haelt, ist ein zweiter
//! Prozessaufruf an [`rufen`] vorbei; das haelt weiterhin allein die Probe
//! `xtask_ruft_git_an_genau_einer_stelle`, und der Uebersetzer haelt es nicht.
```

Erst „was nichts hält", dann „das hält weiterhin allein die Probe". Gemeint ist ersichtlich:
der Übersetzer hält es nicht, eine Probe hält es. Dastehen tut beides zugleich.

Der Absatz ist nach der Bauart „was der Übersetzer hält / was die Aufsicht hält / was nichts
hält" geschnitten, und das ist die richtige Bauart. Die dritte Kategorie ist hier nur falsch
belegt: sie ist nicht leer, sondern trägt eine Probe — also eine schwächere Sicherung als die
zwei davor, aber keine Abwesenheit. Wer den Satz beim Überfliegen liest, nimmt eine Abwesenheit
mit, wo eine Probe steht; wer ihn genau liest, findet einen Widerspruch und muss die Probe
selbst nachschlagen, um zu wissen, welche Hälfte gilt.

## Was die Probe wirklich hält, und was nicht

Nachgesehen in `xtask/src/release.rs:1076-1092`. Sie zählt in jeder `.rs`-Datei des Baums die
beiden Zeichenfolgen `Command::new("/usr/bin/git")` und `Command::new("git")` und verlangt
genau ein Vorkommen, in `xtask/src/git.rs`. Sie ist nach `94855a7` unverändert und misst
weiter, was sie behauptet: die Bauform darunter hat sich geändert, der gesuchte Text nicht.
`rust_dateien` läuft über den ganzen Baum ohne `target/` und `.git` (`release.rs:1287-1305`),
`xtask/src` also eingeschlossen. Grün am Baumstand, zusammen mit den übrigen 145 Proben.

Was sie nicht sieht, sagt der Satz nicht: einen Aufruf, dessen Programmname aus einer Variablen
oder Konstanten kommt, und einen Aufruf über eine Zwischenschale. Genau das ist die Auskunft,
die an dieser Stelle stünde, wenn die dritte Kategorie richtig belegt wäre.

## Abhilfe

Die dritte Kategorie umbenennen und die Grenze der Probe dazuschreiben, etwa:

> **Was allein eine Probe hält**, ist ein zweiter Prozessaufruf an `rufen` vorbei. Der
> Übersetzer hält ihn nicht; `xtask_ruft_git_an_genau_einer_stelle` zählt die Zeichenfolge
> `Command::new("/usr/bin/git")` im ganzen Baum und lässt sie genau einmal zu — einen Aufruf,
> dessen Programmname aus einer Variablen kommt, sieht auch sie nicht.

**Schwere:** niedrig. Kein Fehlverhalten. Es ist die Stelle, an der ein Leser die Stärke der
Absicherung nachschlägt, und sie ist an ihrem schwächsten Punkt uneindeutig.

**Gefunden:** coderev, Durchsicht des Commits `94855a7` am 260821-1432, Bereich
`465330b..94855a7`

**Betroffen:** `xtask/src/git.rs:30-37`

**Domain:** code

**Verwandt:**
`shared/issues/260821-1432_o_git-nimmt-abkuerzungen-langer-marken-an-und-der-gleichheitsvergleich-in-marken-faengt-sie-nicht.md`
— derselbe Absatz, zweiter Satz: dort ist die Zusage nicht uneindeutig, sondern zu stark. Wer
den Absatz anfasst, fasst beides an.
