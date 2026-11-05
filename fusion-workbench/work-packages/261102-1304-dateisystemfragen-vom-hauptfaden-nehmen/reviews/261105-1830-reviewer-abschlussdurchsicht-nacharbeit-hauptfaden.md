# Abschlussdurchsicht: Nacharbeit der Dateisystemfragen vom Hauptfaden

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `85034f51..7f448770`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Verdict:** revise

Inhaltlich gelesen: `aae3c564`, `4b69b2a9`, `20a5d3a0`, `2c0ec6ea`, `886472ef`, `31b2a7c5`, `73b64984`, `541fcd4b`, `05b82470`. `f1bb0d6c` trägt allein das Ereignisprotokoll, das ich nicht geöffnet habe. `7f448770` trägt die Versionszahl; davon habe ich in `Cargo.lock` nur die Versionszeilen gelesen.

**Nicht im Programm gefahren.** `make check` habe ich nicht selbst gefahren, um den parallelen Lauf des Nutzers nicht zu stören. Jedes Fehlerbild unten ist aus dem Quelltext abgeleitet.

## Summary

Einen Datenverlust, einen falschen Faden, einen Doppelruf und einen `RefCell`-Konflikt habe ich nicht gefunden. Auf gesunder Platte verschluckt kein Stand einen Klick oder Tastendruck, soweit der Quelltext das zeigt. Die zwei mittleren Befunde liegen am Rand: eine verworfene Leistenauswahl lässt sich nicht wiederholen, und die Entscheidungen der Einzugsmethoden sind nur am Wortlaut bewacht. Die Fassung braucht deshalb Nacharbeit, aber keinen Rückruf.

## Totals

| Schwere | Anzahl | Datensätze |
|---|---|---|
| Kritisch | 0 | |
| Hoch | 0 | |
| Mittel | 2 | 261105-1823, 261105-1824 |
| Niedrig | 3 | 261105-1825, 261105-1826, 261105-1827 |

## Findings by theme

### Späte Antworten

**Mittel.** 261105-1823-eine-verworfene-leistenauswahl-laesst-die-zeile-gewaehlt-und-ein-zweiter-klick-loest-nichts-aus. `anwendung.rs:3204` verwirft still, `leistenmodell.rs:714` und `leiste.rs:381` lassen den zweiten Klick ins Leere laufen. Betrifft nur die Leiste.

**Niedrig.** 261105-1826-keine-laufende-plattenfrage-meldet-sich-und-jeder-weitere-befehl-startet-einen-faden. Betrifft Löschen, F2, F4 auf `secrets.txt`, Pfadsprung und Lesezeichen.

### Bewachung

**Mittel.** 261105-1824-die-standvergleiche-der-einzugsmethoden-sind-nur-als-zeichenketten-bewacht. Betrifft `leistenauswahl_einziehen`, `notizordner_einziehen`, `geheimnisgroesse_einziehen`, `loeschbefund_einziehen`.

### Löschweg

**Niedrig.** 261105-1825-eine-panik-auf-dem-loeschweg-fuehrt-ohne-hinweis-in-die-endgueltige-rueckfrage.

### Dokumentation

**Niedrig.** 261105-1827-howto-und-der-kommentar-an-blattsteht-folgen-den-neuen-faeden-nicht.

## Antworten auf die Fragen des Auftrags

**1. Verschluckt ein Stand auf gesunder Platte einen Klick oder Tastendruck?** Kein Weg belegt. Keiner der Stände (`Leistenauswahlstand`, `Notizordnerstand`, `Oeffnungsstand`, `Tabstand`, `Pfadfrage`) vergleicht etwas, das sich im Normalfall ändert. Eine Auffrischung baut nichts neu (`tabs.rs:1492`, `aktiven_neu_lesen`). Neue Nummern entstehen aus Befehlen des Nutzers oder aus `ausweichen`, `rueckfall` und `verdeckten_tab_setzen` (`tabs.rs:1122`). Einen Tausch des Orts an Ort und Stelle gibt es nicht.

Rest, abgeleitet und nicht gemessen: `marken_zeichnen` zieht `reloadData` und `auswahl_stellen` nach. Trifft eine geänderte Marke genau die etwa 100 ms eines laufenden Mausklicks, könnte das die Tabellenauswahl auf die alte Modellauswahl setzen und `clickedRow` berühren. Dafür lege ich keinen Datensatz an, solange kein Lauf es zeigt.

**2. Falscher Faden, doppelt, gar nicht, hängender Takt oder Merker?** Kein falscher Faden und kein Doppelruf. `nehmen` in `nebenfaden.rs` gibt den Halter genau einmal her. Panik und Startfehler stellen den Takt weiter, die leere Gruppe gibt ihn frei. Takt und Merker hängen nur, wenn die Frage nie zurückkommt, und das steht benannt in der Regeldatei.

Randfall ohne Datensatz: Trifft die jüngste F2-Antwort auf ein Hindernis, bleiben zurückgestellte Sätze bis zum nächsten F2 liegen, weil der `Err`-Zweig (`anwendung.rs:6740`) vor `saetze_mit` zurückkehrt. Die Sätze gehen nicht verloren, sie kommen verspätet. Der Kommentar `anwendung.rs:883` nennt nur den Fall, dass die jüngste Antwort nie ankommt.

**3. `RefCell`-Ausleihen über Rückweg oder `reloadData`?** Kein Konflikt. Alle `borrow_mut` enden vor `reloadData` und Fadenstart, auch die in `if`-Bedingungen, deren Temporary mit der Bedingung endet. `auswahl_stellen` (`leiste.rs:735`) löst die frühere `match`-Ausleihe auf. `oeffnungsstand` hält ein `borrow()` über `quicknote_offen`, das nur eine `Cell` liest.

**4. `catch_unwind` auf dem Löschweg.** Es wirkt auch im Release, `panic` ist nirgends gesetzt. Eine Panik führt zu `Loeschbefund::unentschieden` und zur Rückfrage mit `PapierkorbUngewiss`. Sicher in dem Sinn, dass nichts ohne Klick geschieht, Vorgabe „Abbrechen“. Falsch begründet und vom Entscheid 261019 nicht gemeint: siehe 261105-1825.

**5. Rechnerteil in `verbinden.rs`.** Kein Befund. Gelesen: Port, Konto, IPv6, `?`, `#`, leer, ohne `://`. `%0A` und `%3C` schaden auf den Wegen, die KRK selbst bedient, nicht: sftp läuft nicht über `url_aus`, sondern über `fernziel` und `rechnername_gueltig` (`crates/krk-core/src/fern/adresse.rs:200`), das `%` nur als IPv6-Zone zulässt. `gleiche_freigabe` vergleicht nur dekodiert. KRK startet keine Shell, und die Eingabe stammt vom Nutzer selbst. **Nicht gemessen:** was NetFS und der ssh-Handler des Systems mit `%0A` oder `%00` tun.

**6. Regeldatei, Modulköpfe, `HowTo.md`.** Die Regeldatei stimmt mit dem Code überein: die drei benannten Reste, die Felder der Stände, die Probenamen. Veraltete Namen (`gueltigkeit_pruefen`, `loeschpruefung`, `merkerstand`) habe ich nicht gefunden. Eine Suche nach Plattenfragen in `anwendung.rs`, `tabelle.rs`, `editor.rs`, `leiste.rs` und `verbinden.rs` fand nur `lokaler_loeschbefund` (auf dem Faden), `notizordner_erheben` (benannt) und `anwendung.rs:7094` (`ort_aufloesen`). Den Rumpf um `:7094` habe ich nicht neu geöffnet; dass er auf dem Faden läuft, stützt sich auf Schritt 6 des Plans. Abweichungen: 261105-1827, und der fehlende Absatz in 261105-1826.

**7. Bewachen die Rumpfproben das Behauptete?** Geteilt. Verhalten prüfen `Fragetakt`, `Schreibrechtmerker`, `gegen_den_ort_der_frage`, die Proben des Leistenmodells, `rechnerteil` und die Hauptfadenproben von `nebenfaden`. Die Abwesenheit von Systemaufrufen halten die Rumpfproben mit Nadeln brauchbar. Die Entscheidungen der Einzugsmethoden halten sie nur als Zeichenketten: 261105-1824.

## Cross-cutting observations

Der frühere mittlere Befund zum Abwurf ist behoben. Die vier neuen Wege haben bewusst keine Obergrenze, je Befehl statt je Zeigerbewegung. Das halte ich für vertretbar, der Nutzer hat es aber nicht entschieden.

Das gemeinsame Muster aller Wege heißt: Frage auf dem Faden, Stand mitgeben, beim Einzug vergleichen, sonst still verwerfen. Die Form ist einheitlich und richtig. Die Stille beim Verwerfen und beim Warten ist der gemeinsame Preis, und daher kommen 261105-1823 und 261105-1826.

Löschen vergleicht als einziger Weg nur die Nummer und keinen Stand. Weil das Blatt seinen Gegenstand nennt, ist das kein Befund.

## Recommended sequencing

1. 261105-1823, weil der Nutzer es an einer aufwachenden Freigabe als toten Klick erlebt.
2. 261105-1826: zuerst die Entscheidung des Nutzers, denn eine Meldung nach Frist kann auch das Warten aus 261105-1823 sichtbar machen.
3. 261105-1824, weil die reinen Funktionen die Änderungen an diesen Methoden absichern. Danach sinkt das Risiko für alles Weitere.
4. 261105-1825 und 261105-1827 nebenbei.

## Nicht gemessen, für den Lauf des Nutzers

- Netzlaufwerk trennen, dann Klick auf ein Lesezeichen, F2, Löschen, F4 auf `secrets.txt`: bleibt die Oberfläche bedienbar, und was geschieht nach dem Wiederkommen?
- Lesezeichen auf aufwachender Freigabe, Tab wechseln, zweiter Klick auf dieselbe Zeile (261105-1823).
- NetFS mit `Mein%20NAS._smb._tcp.local`.
- Hauptschlange während eines Zuges in der Leiste: ändert eine Marke die Auswahl unter dem Klick?
