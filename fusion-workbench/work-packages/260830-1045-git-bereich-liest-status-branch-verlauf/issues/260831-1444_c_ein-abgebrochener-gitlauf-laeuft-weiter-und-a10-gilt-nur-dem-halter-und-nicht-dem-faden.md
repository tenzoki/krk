Ein abgebrochener Gitlauf läuft weiter, und A10 gilt nur dem Halter und nicht dem Faden

---
A10 und C7.11 der Runde 23 sagen zu: „Zwei Statusläufe für dasselbe Dateifenster laufen nie nebeneinander." Gehalten wird davon die schwächere Aussage, dass ein Tab höchstens **einen `Gitlauf`-Halter** führt.

- `Gitlauf::drop` (`crates/krk-core/src/git/lauf.rs:203-213`) setzt das Abbruchkennzeichen und wartet ausdrücklich nicht auf den Faden.
- Das Kennzeichen wird an vier Stellen gelesen (`lauf.rs:238,262,273,284`), und keine davon liegt **in** `Gitleser::marken` (`leser.rs:258-295`): die Funktion nimmt kein Abbruchkennzeichen entgegen und läuft nach dem Eintritt in jedem Fall zu Ende. Der Modulkopf beziffert sie mit 12 bis 164 ms; in einem größeren Baum ist es mehr.
- `Platform::index_worktree_options_mut().thread_limit` bleibt bewusst ungesetzt (`leser.rs:38-52`), also nimmt **jeder** laufende Status so viele Fäden, wie das Gerät Kerne hat.

Wer schnell durch Ordner navigiert, stapelt damit beliebig viele gleichzeitig laufende Statusläufe je Dateifenster, deren Ergebnis niemand mehr annimmt. Das berührt zwei Zusagen zugleich: A10 wörtlich, und C7.9 („Der Gitleser ist ein zweiter Leser und leert den Vorrat nicht"), denn die Kindprobe `kind_liest_unter_abgesenkter_deskriptorgrenze` (`crates/krk-core/tests/git.rs:489`) misst den Bedarf **eines** Lesers mit 30 freien Deskriptoren, nicht den von n gleichzeitig laufenden.

Die Probe `zwei_schnelle_ordnerwechsel_lassen_nie_zwei_gitlaeufe_stehen` (`crates/krk-ui/src/tabs.rs:2951-2971`) zählt über `stehende_gitlaeufe`, also über `tab.gitlauf.is_some()`, und kann den Unterschied nicht sehen.

**Abnahmetest:** entweder trägt `Gitleser::marken` das Abbruchkennzeichen hinein und bricht den Statusstrom ab (die Schleife `for posten in strom` in `leser.rs:283` ist die Stelle), oder A10, C7.11 und der Doc-Kommentar an `Tabinhalt::gitlauf` (`tabs.rs:100-111`) sagen, was sie wirklich zusagen: höchstens ein Halter je Tab, und ein abgebrochener Faden läuft aus. Welche der beiden Antworten gilt, ist eine Nutzerfrage, sobald die Deskriptorfrage daran hängt.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23, beim Prüfen der Nebenläufigkeitszusagen aus C7. Der Befund ist gelesen und nicht gemessen: wie viele Läufe sich bei schneller Navigation tatsächlich stapeln, ist am laufenden Bündel zu sehen und hier nicht.

---
Resolved: 260831, auf dem ersten der beiden Wege, die der Abnahmetest nennt. `Gitleser::marken` nimmt ein `&AtomicBool` entgegen und liest es vor jedem Posten des Statusstroms — genau an der Schleife, die der Datensatz benennt —; ein gesetztes Kennzeichen bricht ab und liefert `None`, also „unentschieden" und keine halbe Liste. `gitlauffaden` reicht sein Kennzeichen dorthin durch.

Die Form ist die des Durchlaufs und keine zweite daneben: `&AtomicBool`, `Ordering::Relaxed`, geprüft vor jeder Einheit, die dauern kann, der Eintritt selbst beim Rufer geprüft (`crates/krk-core/src/verzeichnis/durchlauf.rs`). Der Modulkopf von `crates/krk-core/src/git/mod.rs` sagt jetzt, dass genau eine der drei Auskünfte ein Kennzeichen kennt, und warum diese.

Die Probe ist `ein_gesetztes_abbruchkennzeichen_bricht_den_markenlauf_ab` (`crates/krk-core/tests/git.rs`); ihre Gegenprobe mit einem Kennzeichen auf `false` liefert am selben Ordner eine nicht leere Liste. Sie sieht ihren Fall: mit einer Fassung, die das Kennzeichen entgegennimmt und nicht liest, meldet sie `Some([("unverfolgt.txt", Neu)])` statt `None`.

**Was das hält und was nicht — die zweite Hälfte des Datensatzes bleibt wahr.** Ein aufgegebener Lauf bricht beim nächsten Posten ab, statt die vollen 12 bis 164 ms zu Ende zu lesen. Nebeneinander *laufen* können zwei Fäden trotzdem, so lange wie der ältere für einen Posten und den Aufbau seines Stroms braucht: `Gitlauf::drop` wartet ausdrücklich nicht auf den Faden, und ein Warten wäre genau die Bildzeit, die der Lauf vermeidet. Die wörtliche Fassung von A10 und C7.11 („laufen nie nebeneinander") ist damit weiter stärker als das Gebaute; der Modulkopf von `crates/krk-core/src/git/lauf.rs` und der Doc-Kommentar an `Tabinhalt::gitlauf` schreiben beides aus, den Halter und den Faden. Die zwei Spec-Zeilen sind unverändert und gehören zur Prosadurchsicht.

Die Deskriptorfrage aus C7.9 ist damit nicht beantwortet, nur kleiner: wie viele Läufe sich bei schneller Navigation tatsächlich überschneiden, ist am laufenden Bündel zu sehen und hier nicht gemessen. Die Nutzerfrage nach dem Fadendeckel steht weiter offen (`260830-1317_*_wird-die-fadenzahl-von-gix-gedeckelt-und-woran-waere-die-zahl-zu-messen.md`).
