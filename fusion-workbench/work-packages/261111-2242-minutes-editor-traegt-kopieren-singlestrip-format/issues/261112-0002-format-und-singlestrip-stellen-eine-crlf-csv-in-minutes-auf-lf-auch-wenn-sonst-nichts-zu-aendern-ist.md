Format und Singlestrip stellen eine CRLF-CSV in minutes/ auf LF, auch wenn sonst nichts zu ändern ist
---
Vor der Rohansicht einer `.csv` oder `.tsv` unmittelbar in `minutes/` stehen die drei Schaltflächen. Der Stand einer solchen Datei trägt ihre Zeilenenden selbst. `glaetten` wandelt jedes `\r\n` und `\r` in `\n`, also weicht das Ergebnis vom Ausschnitt ab, auch wenn die Umformung am Text nichts ändert: der Editor meldet nicht „nichts zu ändern“, sondern schreibt den Ausschnitt mit `\n` in den Stand. Mit Auswahl trägt die Datei danach gemischte Zeilenenden, ohne Auswahl durchgehend `\n`.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261111-2248-plan-protokolldatei-traegt-kopieren-singlestrip-format.md, 261019-1122-gilt-der-stand-mit-den-zeilenenden-der-datei-als-antwort-auf-die-sicherungsform-einer-csv.md, 261112-0002-reviewer-protokollknoepfe-vor-der-auslieferung.md

**Schwere:** Low. Betroffen ist allein eine CSV oder TSV mit `\r\n` in `minutes/`, in der Rohansicht, nach einem ausdrücklichen Klick; `cmd+z` nimmt es zurück, und der Kopf zeigt den ungesicherten Stand.

Beleg, am Quelltext gelesen und nicht ausgeführt:

- `crates/krk-core/src/text/glaetten.rs`, `zerlegen`: `eingabe.replace("\r\n", "\n").replace('\r', "\n")`; `fliesstext` und `befehl` rufen beide `zerlegen` und fügen mit `\n` zusammen.
- `crates/krk-ui/src/appkit/editor.rs`, `umgeformt`: `(nachher != vorher).then_some(nachher)`. Für `a,b\r\n1,2\r\n` unter Format ist `nachher` `a,b\n1,2\n` und damit verschieden.
- `crates/krk-core/src/text/datei.rs`, `in_stand_form`: bei `form.traegt_zeilenenden()` bleibt der Text unverändert, `Editormodell::bearbeiten` wandelt einen solchen Stand also nicht zurück, und `sichern` schreibt ihn Byte für Byte.
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs`, `protokolltext`: `Editorform::Text` mit `protokolldatei` genügt; die Rohansicht einer CSV in `minutes/` ist ausdrücklich eingeschlossen, und die Hauptfadenprobe `die_reihe_des_protokolls_folgt_datei_und_form` prüft die Reihe dort.

Die Zusage, dass eine CSV oder TSV ihre Zeilenenden behält, steht im zitierten Datensatz vom 261019. Für jede andere Datei trägt der Stand allein `\n`, und der Befund greift nicht.

Richtung: `protokoll_umgeformt` vergleicht und schreibt in der Zeilenendeform des Standes. Trägt der Ausschnitt `\r\n`, bekommt das Ergebnis vor dem Vergleich `\r\n` an jedes `\n`; ein Ausschnitt, an dem sich sonst nichts ändert, ergibt dann `None`.

Abnahme: eine Probe an `protokoll_umgeformt` hält, dass ein Stand mit `\r\n`, an dem Format nichts zu ändern hat, `None` ergibt, und dass ein umgeformter Ausschnitt eines solchen Standes kein einzelnes `\n` trägt.
