Eine späte Antwort der Pfadeingabe stellt den gerade sichtbaren Tab um und nicht den, aus dem gefragt wurde
---
Schwere: Mittel. Seit `d86ad752`. Mit der Frage reisen Quelle und Ort, kein Tab und keine Nummer. `Wechseln` gilt dem Tab, der bei der Ankunft sichtbar ist, und von zwei Fragen gewinnt die zuletzt angekommene.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/tabelle.rs` `pfad_pruefen_lassen`: `rueckweg` trägt `(Weak<Quelle>, Ort)`. `pfadpruefung_einziehen` vergleicht den angezeigten Ort, und `gegen_den_ort_der_frage` behandelt allein `NurAuswahl`. `ergebnis_anwenden` ruft für `Wechseln` `self.ordner_lesen`, und `ordner_lesen` (`:2042`) ruft `tabs.ordner_setzen`, das dem sichtbaren Tab gilt. Der Doc-Kommentar von `gegen_den_ort_der_frage` begründet: „`Wechseln` trägt einen absoluten Ordner und `Meldung` einen Satz, beide gelten unabhängig vom Stand.“ Das stimmt für das Ziel und nicht für den Tab, in dem es wirkt.

**Fehlerbild 1, aus dem Quelltext abgeleitet.** Der Nutzer tippt in Tab 1 einen Pfad auf einer langsamen Freigabe und wechselt nach Tab 2, weil nichts geschieht. Die Antwort kommt, und Tab 2 steht auf dem Ordner der Frage. Tab 1 steht unverändert. Der nächste Tastendruck des Nutzers gilt einem Ordner, den er nicht gewählt hat.

**Fehlerbild 2.** Der Nutzer springt zu `/Volumes/haengt/x`, dann zu `~/y`. Die erste Antwort kommt zuletzt und überschreibt den neueren Sprung. Vor dem Faden war die Reihenfolge durch den Hauptfaden festgelegt.

**Nicht gefahren, nicht gemessen.** Das Paket hat seine Rumpfproben, und die sehen die Reihenfolge nicht. Die Probe `nur_auswahl_wird_nach_einem_ortswechsel_zum_wechsel_in_den_ordner_der_frage` deckt allein die drei Ergebnisarten ab.

**Richtung.** Eine Sprungnummer je Quelle: die Antwort gilt nur, wenn seit der Frage kein neuer Sprung gefragt und im Tab kein anderer Ort gewählt wurde; sonst wird sie verworfen, eine `Meldung` eingeschlossen, oder bei `Meldung` als Hinweis gezeigt. `gegen_den_ort_der_frage` bleibt die eine reine Stelle dafür. Hat der Nutzer nur den Tab gewechselt, wäre Verwerfen die einfachere Regel; ein Anwenden auf den Tab der Frage verlangt eine Tabkennung, die heute nicht reist.

**Abnahme.** Eine reine Probe: Eine Antwort auf eine überholte Frage ändert keinen Tab. Eine Antwort bei gewechseltem Tab stellt den sichtbaren Tab nicht um.

---
Resolved: Eine Pfadprüfung gilt nur noch, wenn seit der Frage in diesem Dateifenster kein neuer Sprung gefragt wurde (`QuelleIvars::pfadsprung`) und der sichtbare Tab noch derselbe auf demselben Ort ist (`Tabstand`, neu in `crates/krk-ui/src/tabs.rs`, eine Nummer je gebautem `Tabinhalt`, keine Stelle, die ein Ortswechsel neu vergibt und eine Auffrischung stehen lässt); sonst fällt sie in jeder der drei Arten, entschieden allein in `gegen_den_ort_der_frage`. `NurAuswahl` wird nicht mehr zum Wechsel in den Ordner der Frage, und die Probe dazu heißt seitdem `eine_antwort_gilt_bei_unveraendertem_stand`, neben `eine_antwort_auf_eine_ueberholte_frage_faellt` und `eine_antwort_bei_gewechseltem_tab_stellt_den_sichtbaren_nicht_um`. Eine `Meldung` fällt mit, weil `befehlsantwort` ein Feld je Dateifenster ist und eine späte Meldung die Antwort auf den inzwischen gelaufenen Befehl zuschriebe; wer den Tab gewechselt hat, erfährt deshalb nicht, dass sein Pfad scheiterte, solange er nicht in den Tab der Frage zurückgekehrt ist, bevor die Antwort ankommt. Ungefahren: kein Lauf gegen eine hängende Freigabe, kein Lauf im Programm; die Proben sind rein und sehen das Eintreffen über die Hauptschlange nicht.
