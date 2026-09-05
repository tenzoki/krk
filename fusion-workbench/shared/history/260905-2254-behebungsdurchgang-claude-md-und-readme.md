# Behebungsdurchgang an `CLAUDE.md` und `README.md`

**Status:** Complete
**Agent:** coder
**Person:** Kai Stalmann <kai@stalmann.org>
**Baumstand bei Beginn:** `8779a25`
**Grenze des Durchgangs:** `CLAUDE.md`, `README.md`, `HowTo.md` im Projektwurzelverzeichnis und die Datensätze unter `fusion-workbench/shared/issues/`. Keine Datei unter `crates/`, `xtask/`, `resources/`, `iconset/`, kein `Makefile`, kein `certify-only.sh`.

## Verification

- `cargo test -p xtask` — exit 0, 161 Proben. Die Kiste liest `README.md` und das `Makefile` in `der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr`; eine Änderung an der README kann sie rot machen.
- `grep -rn '^#!\[deny(unsafe_code)\]' crates/*/src/*.rs xtask/src/*.rs` — vier Zeilen, eine je Eintrag unter `members` der Wurzel-`Cargo.toml`.
- ``grep -rnE --exclude-dir=fusion-workbench --exclude-dir=target '[Dd]ie alte.{0,24}löschen' .`` — `HowTo.md`, `README.md`, `xtask/src/veroeffentlichung.rs` (`RELEASETEXT`), dazu `CLAUDE.md` selbst.
- `grep -c '^\[\[' resources/default-readers.toml` — zwölf.
- `grep -rn "sieben Stationen" --exclude-dir=fusion-workbench --exclude-dir=target --exclude-dir=.git .` — leer.
- Doc-Kommentare von `crates/krk-ui/src/appkit/menue.rs` und `crates/krk-ui/src/belegungsausgabe.rs` auf zweistellige Zahlen abgesucht: keine Funktionszahl mehr, allein Runden- und Datumsangaben.

## Was in `CLAUDE.md` steht

1. **`deny(unsafe_code)`** (Abschnitt „Projektstand", letzter Absatz). Die Kistenliste ist gefallen, an ihrer Stelle steht die Regel für jedes Mitglied des Workspace und das Zählkommando. Der Satz zu den zwei `#![allow(unsafe_code)]`-Ausnahmen nennt jetzt die Probe, die sie namentlich hält, und deren Reichweite: sie liest allein `crates/`.
2. **Die Betriebsregel gegen den Datenverlust** (Abschnitt „Was man nicht sieht", erster Absatz). „an zwei Stellen" ist gefallen; an seiner Stelle stehen das Erhebungskommando und der Satz, der es vollständig hält (jede Stelle führt die Wendung „die alte … löschen"). Die heute antwortenden Stellen sind daneben benannt, `HowTo.md` mit ihrem Weg ins Paket.
3. **Zwei neue Absätze zur Vorschau der Runde 18** (hinter dem ersten Absatz desselben Abschnitts): der Handgriff mit der `readers.toml` als zweite Regel derselben Bauart, und die vier Änderungen am Mechanismus samt der Abweichung zwischen Leseläufen und Verzeichnisöffnungen. Keine Profilzahl in Prosa.
4. **`ohne_warten_oeffnen`** (Absatz „Die Prüfung dessen, was da geöffnet wurde"). „einziger Öffner" ist auf „einziger Öffner **dieser Datei**" eingeengt, mit der Begründung: `File::open` steht daneben noch im Kopieren und im Entpacken, je auf einem selbst angelegten Pfad.

## Was in `README.md` steht

1. **Erster Absatz.** „Eine Git-Anbindung ist vorgesehen und noch nicht gebaut." ist ersetzt durch die Abgrenzung, die er meinte: die Anbindung liest und schreibt nicht, die vier ausstehenden Operationen sind namentlich genannt.
2. **Station 2 der Stationstabelle.** Nennt jetzt beide Hälften der Prüfung, `use`-Zeile und ausgeschriebener Pfad.
3. **„Nur beglaubigen".** Der Grund für den Weg steht auf dem Aufwand und nicht auf einem Abbruch; der Station-1-Satz ist als Bedingung formuliert.
4. **„Einmal vor dem ersten Lauf: die alten Tags nachschieben".** `git push origin --tags` ist gefallen; an seiner Stelle steht der Handgriff je Tag, der verwaiste Tag ist als Fall benannt, und der Grund gegen die Sammelmarke steht dabei: sie steht in `MARKEN`, die das Bauwerkzeug sich selbst untersagt.

`HowTo.md` blieb unangetastet: keiner der Datensätze trifft eine ihrer Aussagen.

## Geschlossene Datensätze

- `260905-2155_c_claude-md-nennt-drei-kisten-mit-deny-unsafe-code-seit-heute-tragen-es-vier.md`
- `260905-1658_c_claude-md-nennt-zwei-stellen-an-denen-der-nutzer-die-betriebsregel-liest-mit-der-anleitung-im-paket-sind-es-drei.md`
- `260826-0149_c_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-der-runde-18-an-der-vorschau.md`
- `260815-1448_c_die-neun-berichtigten-zahlen-stehen-weiter-unverankert-und-die-benannte-ursache-traegt-keinen-datensatz.md`
- `260826-1933_c_zwei-prosastellen-an-ohne-warten-oeffnen-zaehlen-fuenf-rufer-und-nennen-den-schwungleser-als-einzigen-file-open-oeffner.md`
- `260905-1444_c_die-readme-sagt-die-git-anbindung-sei-vorgesehen-und-nicht-gebaut.md`
- `260826-1445_c_readme-reicht-verwaiste-tags-mit-git-push-origin-tags-oeffentlich-nach-der-marke-die-die-aufsicht-des-werkzeugs-verbietet.md`

## Offen geblieben

- `260826-1441_o_…-und-der-code-gibt-dem-selteneren-recht.md` — die README-Hälfte ist getan, `certify-only.sh:22`, `Makefile:147` und `xtask/src/main.rs:139` tragen die falsche Begründung weiter. Abgleichnotiz im Datensatz.
- `260826-1449_o_readme-und-hilfetext-beschreiben-station-2-…` — die README-Hälfte ist getan, der Hilfetext in `xtask/src/main.rs:85` steht weiter auf der halben Aussage. Abgleichnotiz im Datensatz.
- `260821-1221_o_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md` — der Quellbaum ist sauber, die Zeichenfolge steht allein noch im Kriterium selbst, `shared/planning/260821-1115_*_spec-artefakt-und-release.md`. Kein Handgriff in den drei Dokumentationsdateien.
- `260821-1532_o_zwei-fremde-werkzeuge-werden-seit-langem-ueber-den-suchpfad-gerufen-…` — trifft keine der drei Dateien. `iconutil` (`xtask/src/bundle.rs:460`) und `rustup` (`xtask/src/release.rs:609`) gehen unverändert über den Suchpfad und tragen an ihrer Stelle keine Begründung; die zwei Prosastellen liegen unter `shared/decisions/` und `shared/planning/`.

## Neuer Befund

`260905-2254_o_das-zaehlkommando-fuer-ohne-warten-oeffnen-gibt-elf-zeilen-fuer-sechs-rufer-aus.md`. Beide Prosastellen, die eine Zahl durch ein Kommando ersetzt haben, nennen ein Kommando, das Doc-Zeilen, die Definition und eine Probe mitzählt: elf Zeilen für sechs Aufrufer, und vier der fünf falschen Zeilen sind Prosa über das Kommando selbst, wachsen also mit jeder weiteren Stelle, die es nennt.

## Zahlen, die stehen geblieben sind

Keine. Jede Zahl, die dieser Durchgang angefasst hat, ist durch ein Kommando oder durch eine Probe ersetzt; die zwei `#![allow(unsafe_code)]`-Ausnahmen bleiben als Namen stehen, und die Probe `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` hält sie.
