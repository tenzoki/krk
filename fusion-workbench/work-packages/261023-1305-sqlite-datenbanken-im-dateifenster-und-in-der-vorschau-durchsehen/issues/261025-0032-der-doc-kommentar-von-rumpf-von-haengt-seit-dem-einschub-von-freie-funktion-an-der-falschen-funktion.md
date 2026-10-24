# Der Doc-Kommentar von `rumpf_von` hängt seit dem Einschub von `freie_funktion` an der falschen Funktion

---
`8a3b212a` hat die Hilfsfunktion `freie_funktion` mit eigenem Doc-Kommentar zwischen den Doc-Kommentar von `rumpf_von` und dessen `fn` gesetzt. Beide Kommentare verschmelzen jetzt zu einem an `freie_funktion`, und `rumpf_von` steht ohne Kommentar da.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Low
**Cross-references:** 261024-2114-die-ersatzangaben-der-vorschau-zeigen-die-zeilenzahl-einer-tabelle-als-bytes.md

## Befund

`crates/krk-ui/src/appkit/tabelle.rs`, Prüfmodul: Der Block „Der Rumpf einer Methode dieser Datei, vom Kopf bis zur schliessenden Klammer auf Methodenebene. … **Eine Naeherung und als solche benannt.** …“ endet mit „… die unangemessene.“. Unmittelbar darauf folgt `/// Der Rumpf der freien Funktion `name`, …` und dann `fn freie_funktion<'a>(…)`. `fn rumpf_von<'a>(…)` folgt erst danach und ohne `///`. Rustdoc und jede Leserin ordnen die Näherungsbegründung damit `freie_funktion` zu, die an `\n}\n` auf Dateiebene schneidet und nicht an vier Leerzeichen.

## Erwartet

`freie_funktion` steht samt Kommentar vor dem Doc-Block von `rumpf_von` oder hinter `rumpf_von`, sodass jeder Kommentar an seiner Funktion hängt.

## Abnahme

Im Quelltext folgt auf die letzte `///`-Zeile, die mit „unangemessene.“ endet, unmittelbar `fn rumpf_von`.
