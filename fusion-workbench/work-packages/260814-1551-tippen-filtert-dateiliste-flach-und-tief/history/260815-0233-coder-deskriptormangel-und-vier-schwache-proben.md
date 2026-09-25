# Der Deskriptormangel des Durchlaufs, und vier Proben halten jetzt, was sie zusagen

**Status:** Complete
**Agent:** coder
**Datum:** 260815-0233
**Circle:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`
**Auftrag:** Turn 2, Reparaturrunde. Der hohe Befund der Durchsicht vom 260815-0211 und die vier niedrigen.

---

## Was gefahren ist

**Verification:** `make check` — exit 0.

### Der hohe Befund: zwei Fehler, zwei Behebungen

Der Befund hatte zwei Hälften, die unabhängig voneinander falsch waren. Behoben sind beide, und die zweite zuerst, weil sie die erste selten macht.

**Der Durchlauf hält keinen Deskriptor je Ebene mehr.** `unterbaum_entscheiden` liest einen Ordner ganz, merkt seine Unterordner dabei als Pfad auf einem Stapel `offen` vor und öffnet den nächsten erst, wenn der laufende zu Ende ist. Zu jedem Zeitpunkt hält der Durchlauf damit genau einen Verzeichnisdeskriptor, gleich wie tief der Baum ist. Die Kante „zurück zum übergeordneten Ordner", die den alten Bau teuer machte, gibt es nicht mehr: der Rückweg läuft über den vorgemerkten Pfad, genau wie der Auftrag es als Möglichkeit genannt hat.

Der Tausch ist ein knapper, prozessweit geteilter Vorrat gegen einen reichlichen, der dem Durchlauf allein gehört. Die Deskriptortabelle teilt sich der Durchlauf mit dem Editor, der Vorschau, den Kopiervorgängen und dem Lesevorgang der zweiten Dateiliste; ein aus dem Finder gestartetes Bündel bekommt sie klein. `offen` hält dagegen je vorgemerktem Ordner einen Pfad, und das ist weniger, als das Ordnermodell für denselben Ordner ohnehin hält — dort steht je Eintrag ein `Eintrag` mit zwei Sortierschlüsseln. Der alte Bau kostete daneben nicht nur einen Deskriptor je Ebene, sondern auch einen Antwortpuffer von 256 KB je Ebene.

**`EMFILE` und `ENFILE` sind von den übrigen Öffnungsfehlern getrennt.** `verzeichnis::sys::ist_deskriptormangel` beantwortet die Frage an einer Stelle; die beiden Konstanten stehen dort neben `EXDEV`, `ECANCELED` und `EWOULDBLOCK`, und `durchlauf.rs` bleibt damit frei von Konstanten, wie seine eigene Probe es verlangt. Trifft einer der beiden, liefert `unterbaum_entscheiden` `None`: der Auftrag bleibt unentschieden, wie beim Abbruch, und der Durchlauf endet.

Gewählt ist damit Stufe 1 der drei im Datensatz genannten. Der Grund gegen ein Warten mit erneutem Versuch: es stünde für eine Frage, die dieses Modul nicht beantworten kann — ob und wann ein anderer Teil von KRK einen Deskriptor freigibt —, und hielte den Arbeitsfaden dabei an. Der Grund für „unentschieden" statt eines neuen dritten Ausgangs: `None` ist die Antwort, die dieses Modul für „nicht entschieden" schon hat, und C3.13 unterscheidet „kein Treffer darunter" von „noch nicht entschieden" bereits als Kriterium. Der Unterschied für den Nutzer ist nicht die Zeile — ein unentschiedener Ordner steht ebenso wenig in der Liste wie ein negativ entschiedener —, sondern die Dauer: der falsche Befund hielt für den ganzen Filtertext, der ausbleibende wird von der nächsten Frage neu gestellt.

Stufe 3, das Anheben von `RLIMIT_NOFILE` beim Start, ist nicht gefahren. Sie berührt den ganzen Prozess und gehört nicht in dieses Modul.

### Die Probe misst den Fall, und die Gegenprobe ist gefahren

`die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden` legt eine 200 Ebenen tiefe Kette mit dem Treffer ganz unten an und lässt sie von einer Kindprobe entscheiden, die über `/bin/sh` mit `ulimit -n 64` startet. Die Bauform ist die, die `tests/ablage.rs` für ihre Zweiprozessproben schon führt: ein `#[ignore]`-Test, den der Elternteil über eine Umgebungsvariable beauftragt.

Drei Dinge daran sind Absicht:

- **Das Kind misst seine Grenze selbst**, indem es Deskriptoren nimmt, bis keiner mehr kommt. Ohne diese Zusicherung bestünde die Probe auch dann, wenn `ulimit` nicht gegriffen hätte — sie wäre wieder eine Behauptung.
- **Der Elternteil legt den Baum an und räumt ihn ab.** `remove_dir_all` hält selbst einen Deskriptor je Ebene und käme unter der abgesenkten Grenze nicht durch.
- **Die Tiefe 200** liegt deutlich über den rund 55 freien Deskriptoren des Kindes und deutlich unter `PATH_MAX / 2`.

**Gegenprobe:** mit dem alten `durchlauf.rs` an derselben Stelle meldet die Probe `treffer: false` bei 61 freien Deskriptoren, also genau den Befund der Nachstellung aus der Durchsicht. Mit dem neuen meldet sie `treffer: true`.

`der_durchlauf_kennt_keine_tiefengrenze` bleibt daneben stehen. Sie prüft C3.8 unter der Grenze der Anmeldesitzung, die neue unter der, die ein Bündel bekommt.

### Die vier schwachen Proben

**Die Abbruchprobe** misst jetzt an zwei Läufen über denselben Ordner: ohne Abbruch muss `treffer: false` kommen, mit Abbruch nichts. Erst dadurch heißt das Schweigen „der Abbruch hat gegriffen" und nicht „der Durchlauf meldet hier ohnehin nichts" — eine zweite Schwäche, die der Befund nicht genannt hatte. Die Zahl **zwei** aus C3.4 misst sie weiter nicht, und der Doc-Kommentar sagt das jetzt statt es zu verdecken. Die vorgeschlagene Probe mit dem Treffer hinter der Zwei-Stapel-Grenze ist nicht gebaut: es gibt keinen Rendezvouspunkt zwischen Hauptfaden und Arbeitsfaden, und wer stattdessen auf die Laufzeit setzt, hat eine Probe über den Planer des Betriebssystems gebaut. Entscheidbar wird die Frage erst mit einer Größe am Durchlauf, an der die geleistete Arbeit abzulesen ist.

**`im_filter_steht_keine_zeitmessung`** liest fünf Dateien statt vier. `code_zeilen_vor_dem_pruefmodul` schneidet am ersten `#[cfg(test)]`, und damit ließ sich `krk-ui/src/tabs.rs` aufnehmen, deren Prüfmodul zwischen zwei Einzugstakten schläft. Die Zusicherung ist jetzt eine über den Filter und nicht über eine Dateiliste.

**`die_dateiliste_bleibt_flach_und_hat_vier_spalten`** zählt `NSOutlineView` über `quelldateien()` statt über `include_str!` einer Datei. `NSTableView` ist ausdrücklich keine Nadel: KRK hat mehrere Tabellen, und eine Zählung darüber sähe die Belegungsansicht und das Blatt zum Stapelumbenennen als Fundstellen.

**`die_angezeigte_datei_bleibt_bei_zwei_quellen`** liest keine Zeile Quelltext mehr. An die Stelle der Zählung von `return Some(` treten die Bindung an einen Funktionszeiger mit genau vier Eingaben — eine dritte Quelle, die eine fünfte braucht, hält den Bau an — und die Prüfung über alle sechzehn Kombinationen, dass die Antwort `None` oder genau einer der beiden übergebenen Pfade ist. Damit ist die Schreibweise gleichgültig geworden, und die Probe folgt der Bauanleitung aus `quellbaum.rs`: nach dem Gegenstand suchen, nicht nach dem Namen.

---

## Berührte Dateien

- `crates/krk-core/src/verzeichnis/sys.rs` — `ist_deskriptormangel`, `EMFILE`, `ENFILE`
- `crates/krk-core/src/verzeichnis/durchlauf.rs` — Modulkopf, Bild, `unterbaum_entscheiden`, `Ebene` → `Lesestand`
- `crates/krk-core/tests/verzeichnis.rs` — neue Deskriptorprobe samt Kindprobe, Abbruchprobe, Zeitmessungsprobe, `code_zeilen_vor_dem_pruefmodul`
- `crates/krk-ui/src/tabs.rs` — die beiden Zählproben

## Datensätze

Geschlossen: die fünf Datensätze mit Stempel `260815-0211`, die der Auftrag benannt hat.

Neu abgelegt: `issues/260815-0233_o_das-zweite-bild-des-spec-zeigt-den-abstieg-mit-rueckkehr-der-baum-merkt-pfade-vor.md`. Der Umbau des Abstiegs macht drei Stellen des zweiten Bildes und die Kreiszählung darunter falsch. Kein Abnahmekriterium ist verletzt; das Bild beschreibt aber einen bestimmten Bau, und wer es liest, liest jetzt den falschen.

Nicht angefasst: die beiden mittleren Befunde zu C1.11. Sie sind am Spec zu berichtigen und laufen getrennt.
