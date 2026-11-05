# Schlussdurchsicht: Dateisystemfragen vom Hauptfaden, Nacharbeit nach 3.14.3

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `7f448770..7938a6b0`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Verdict:** revise

Inhaltlich gelesen: `32c10893` (die vorige Durchsicht samt ihren fünf Defektdatensätzen), `c145f081`, `0e9347c0`, `0053d623`, `672af5d6`, `ae6ff09f`, `7938a6b0`. Die Protokolldatei liegt nicht im Bereich (`git diff --name-only` nennt sie nicht); sie steht als Übertrag aus dem Auftrag hier und ist im Arbeitsbaum geändert.

**Gefahren.** Auf dem Stand `7938a6b0`, mit eigenem Zielverzeichnis, damit kein paralleler Lauf gestört wird: `cargo test --workspace`, `cargo clippy --workspace --all-targets`, `cargo fmt --all --check`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`. Alle grün, auch das Prüfziel `hauptfaden` mit dem echten Zeitgeber.

**Nicht gefahren.** Das Programm selbst. Jedes Fehlerbild unten ist aus dem Quelltext abgeleitet. Die neun Mutationen aus `0053d623` habe ich nicht wiederholt; die Tafeln habe ich gelesen, und sie decken die genannten Vergleiche ab (inference).

## Summary

Keinen Datenverlust, keinen Doppelruf und keinen verschluckten Klick auf gesunder Platte gefunden. `0053d623` ändert das Verhalten nicht, `672af5d6` lässt den Entscheid zu nicht auflösbaren Ordnern unberührt. Der eine mittlere Befund ist die Lebensdauer des neuen Wartesatzes: er hängt an eigenen Nummern und bleibt nach einem überholten Befehl stehen.

## Totals

| Schwere | Anzahl | Datensätze |
|---|---|---|
| Kritisch | 0 | |
| Hoch | 0 | |
| Mittel | 1 | 261105-2000 |
| Niedrig | 2 | 261105-2001, 261105-2002 |

## Findings by theme

### Lebensdauer des Wartesatzes

**Mittel.** 261105-2000-der-wartesatz-zaehlt-eigene-nummern-und-bleibt-nach-einem-ueberholten-befehl-stehen. `plattenwarte.rs:143` kennt allein die eigene Nummer; `anwendung.rs:3343` führt ein Gerät aus, ohne ihn zu berühren. Betrifft alle fünf Befehle, am sichtbarsten das Lesezeichen. In der Gegenrichtung fällt der Satz beim Pfadsprung zu früh, weil zwei Dateifenster sich einen Stand teilen.

### Dokumentation

**Niedrig.** 261105-2001-howto-sagt-nach-einem-anderen-tun-geschehe-nichts-mehr-das-loeschblatt-geht-trotzdem-auf. `HowTo.md:207` gegen `anwendung.rs:9059`.

**Niedrig.** 261105-2002-kommentare-der-statuszeile-zaehlen-die-raenge-noch-ohne-das-warten-auf-die-platte. `statuszeile.rs:432`, `:486`, `:498`, `:1436`, `anwendung.rs:8372`.

## Antworten auf die Fragen des Auftrags

**1. `klick_gemeldet` und `offene_auswahl`.** Kein Doppelnavigieren und kein verschluckter Klick auf gesunder Platte.

- Neue Zeile, wie angenommen: Auswahlmeldung, dann `action`. Die `action` verbraucht den Merker (`leistenmodell.rs:788`), kein zweites Fragen.
- Doppelklick: Ist die Antwort schon da, ist `offene_auswahl` leer und der zweite Klick tut nichts. Ist sie nicht da, startet er eine zweite Frage mit neuer Nummer, die erste Antwort verfällt (`anwendung.rs:3392`), und navigiert wird einmal. Die Hauptschlange läuft auf einem Faden, also gibt es keine Zwischenlage.
- **Umgekehrte Reihenfolge, nicht gemessen:** `action` zuerst findet die neue Zeile noch nicht gewählt und liefert `false`; danach setzt `waehlen` den Merker, und er bleibt liegen. Folge: keine Doppelfrage, aber der erste Wiederholungsklick auf eine verfallene Auswahl wirkt nicht, erst der zweite. Der Fehler fällt also auf die harmlose Seite.
- Dauerhaft falsch liegt kein Merker. `klick_gemeldet` lebt höchstens bis zur nächsten `action` oder Pfeiltaste (`:812`). Rest: Eine Auswahlmeldung ohne folgende `action` lässt ihn einen Klick lang liegen; einen Weg dafür sehe ich nur in der Tippsuche der Tabelle, die ich nicht geprüft habe, und im Loslassen außerhalb der Tabelle. Ziehen trägt die Leiste nicht (kein `registerForDraggedTypes` in `leiste.rs`). Der Schaden wäre ein wirkungsloser Wiederholungsklick.
- `offene_auswahl` bleibt nur nach einer verfallenen Antwort stehen und führt dann bei einem Klick auf genau dieses Ziel dorthin. Das ist der Zweck. Eine Verbindung und ein Gerät schließen sie noch im selben Aufruf (`anwendung.rs:3431`).
- Der Befund 261105-1823 ist beseitigt. Der Rest, den die Behebung ausdrücklich lässt: Die Tastatur hat keine Wiederholung, und der Satz über ein fehlendes Ziel zählt als vollzogen.

**2. Plattenwarte.** Auf gesunder Platte bleibt der Satz nicht hängen: jeder Zweig erreicht `beantwortet`.

- Startfehler und Panik in `arbeit`: `fertig` läuft (`nebenfaden.rs`, `fragen_lassen`), das Abmelden steht davor. Panik in `fertig` selbst: schon abgemeldet.
- Überholte Nummer: `frist_um` und `beantwortet` vergleichen mit der jüngsten, die Tafeln halten das.
- Fenster geschlossen: das Abmelden hängt nicht am Delegierten, der Melder liest die Zeile über `Weak`, `statuszeile_nachziehen` fragt `statuszeile.get()`.
- Ohne Antwort bleibt nur der Fall, den `nebenfaden.rs:35` schon nennt, darunter eine Panik in `startfehler`; alle fünf `startfehler` lese ich als einfach.
- `DispatchTime::try_from` rechnet relativ zu `NOW` (`dispatch2-0.3.1/src/time.rs:43`).

Die Zählung je Befehl stimmt in sich, trifft aber nicht, was gilt: 261105-2000. Der neue Rang blockiert auf gesunder Platte nichts, weil er ohne Satz nicht antritt. Auf einer hängenden Platte verdeckt er `Fenstermeldung` und `Tabmeldung`. Das ist der Kern von 261105-2000. **Nicht gemessen:** ob die Frist von 1 s auf gesunder Platte je auslöst, etwa beim Löschbefund auf einer anlaufenden Platte.

**3. `0053d623`.** Verhalten gleich, Rumpf gegen Rumpf gelesen.

- F2: `gilt`, Griff, Nummer, Sätze, Tab und Fokus in derselben Folge und gegen dasselbe `jetzt`. Das Hindernis wird wie vorher nur bei `gilt` gezeigt.
- Löschausgang: gleiche Zweige, gleiche Gründe. Einzige Änderung: `blatt_steht()` wird jetzt immer gefragt, nicht nur im Wächter. Es liest allein `attachedSheet` (`anwendung.rs:5083`), also ohne Wirkung.
- `pruefen_fuer_archivtab` als Funktionswert statt Abschluss: gleich.
- Weggefallene Prüfungen „Griff vor Tab“ und „Stand vor `geheimnisweg`“: Beide Reihenfolgen stehen im Code weiter richtig (`anwendung.rs:6951`, `editor.rs:5044`). Sie hält jetzt keine Probe mehr, und `contains("if !geheimnisgroesse_gilt(")` sagt nichts über die Stelle im Rumpf. Der Commit nennt das. Kein Datensatz.
- Der Befund 261105-1824 ist beseitigt, soweit eine Probe das kann: ob die Methode der Entscheidung folgt, bleibt Sache des Laufs.

**4. `672af5d6`.** Der Rang vor `BlattSteht` ändert nichts Falsches: beide Ausgänge öffnen nichts, nur der gezeigte Satz unterscheidet sich, und ein Abbruch ist die genauere Auskunft. Die Stufen `VorgangLaeuft` und `NichtsAusgewaehlt` antworten weiter zuerst. Eine überholte Nummer verwirft auch den Abbruch (`anwendung.rs:9059`). Der Entscheid vom 261019 bleibt unberührt: `lokaler_loeschbefund` ist nicht geändert, sein unentschiedener Befund kommt als `Ok(Some(_))` und führt weiter auf `PapierkorbUngewiss`, die Tafel hält das. `Loeschbefund::unentschieden` bleibt für den lokalen Gegenstand ohne Befund. Der Befund 261105-1825 ist beseitigt. **Nicht ausgelöst:** weder Panik noch Startfehler im Programm.

**5. `HowTo.md` und Modulköpfe.** `leistenmodell.rs`, `leiste.rs`, `nebenfaden.rs` und der Abschnitt über Geräte und Orte stimmen mit dem Code. Abweichungen: 261105-2001 und 261105-2002. Die Regeldatei `.claude/rules/coding-was-man-nicht-sieht.md` widerspricht dem Code an den gelesenen Stellen nicht. Der Befund 261105-1827 ist beseitigt.

## Cross-cutting observations

Der Wartesatz ist die zweite Buchführung darüber, welche Frage noch gilt, neben den Nummern und Ständen der fünf Befehle. Daher kommen beide Richtungen in 261105-2000. Die Modulköpfe nennen die Trennung einen Vorzug („`beantwortet` kennt die Staende der Befehle gar nicht“, `plattenwarte.rs:34`). Für das Ende des Wartens trägt sie nur, solange eine Antwort kommt.

`HowTo.md:207` behandelt alle fünf Befehle gleich, und das tun sie nicht: Das Löschen vergleicht als einziges nur die Nummer. Dasselbe Muster wie oben: ein Satz über alle fünf, wo sie sich unterscheiden.

## Recommended sequencing

Nichts davon blockiert aus Sicherheitsgründen.

1. 261105-2000: Du entscheidest zuerst, ob der Satz einen überholten Befehl überleben darf. Ohne diese Antwort lässt sich der Modulkopf nicht richtig schreiben.
2. 261105-2001 zusammen damit, weil derselbe Absatz betroffen ist.
3. 261105-2002 nebenbei.

## Nicht gemessen, für den Lauf des Nutzers

- Lesezeichen auf getrennter Freigabe klicken, dann das Benutzerverzeichnis: bleibt der Satz stehen (261105-2000)?
- Reihenfolge von Auswahlmeldung und `action`: Lesezeichen klicken, sofort Tab wechseln, Antwort verfallen lassen. Wirkt der erste Wiederholungsklick?
- Doppelklick auf ein Lesezeichen: navigiert es einmal?
- Erscheint der Satz im Fenster und fällt er mit der Antwort?
- Löst die Frist auf gesunder Platte aus, etwa beim Löschen eines großen Ordners?
- Weiterhin offen aus der vorigen Durchsicht: Netzlaufwerk trennen mit F2, Löschen und F4; NetFS mit kodiertem Namen; ändert eine Marke die Auswahl unter dem Klick?
