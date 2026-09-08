`README.md` führt Station 1 mit drei Bedingungen und die Beglaubigung mit zwei Prüfungen; der Code stellt seit dem 260906 je eine Frage mehr

---

Station 1 von `cargo xtask release` fragt seit `260826-1443` zusätzlich, ob auf der Gegenseite
schon ein Release `v<version>` steht. `cargo xtask beglaubigen` fragt seit `260826-1447`
zusätzlich, ob das Bündel beide Architekturen trägt. Beide Behebungen lagen in `xtask/`; die
`README.md` beschreibt beide Wege weiter mit dem Stand davor.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig — Dokumentation, aber die des Wegs, der über das Gerät hinaus wirkt.
**Baumstand:** `ba0c6bd` plus die Änderungen dieses Durchgangs
**Betroffen:** `README.md`, `CLAUDE.md`

## Befund

**Station 1.** `README.md:272`, Zeile der Stationstabelle:

> | 1 | Stand prüfen | HEAD trägt `v<version>` passend zu `[workspace.package]`, keine verfolgte Datei ist geändert, `gh` ist vorhanden und angemeldet |

Der Code stellt jetzt vier Fragen. `release::ausfuehren` ruft hinter
`veroeffentlichung::gh_pruefen` neu `veroeffentlichung::release_frei_pruefen(&bundle::wurzel(),
bundle::VERSION)`. Nachgezogen sind der Modulkopf von `xtask/src/release.rs` und der Hilfetext in
`xtask/src/main.rs`; die `README.md` lag ausserhalb der Grenze jenes Durchgangs.

Der Absatz `README.md:283-285` daneben („**Station 1 fragt schon nach `gh`, obwohl erst Station 8
es braucht.**") bleibt sachlich richtig, gilt aber jetzt für zwei Vorabfragen und nicht mehr für
eine; er nennt die zweite nicht.

**Die Beglaubigung.** `README.md:329`:

> Geprüft wird zweierlei, und beides am Bündel, das dort liegt:

Darunter eine Tabelle mit zwei Zeilen, „die Versionszahl" und „der Signaturstand". Der
Signaturstand zerfällt jetzt in drei Fragen: Developer-ID in der Signaturkette, gehärtete
Laufzeitumgebung, beide Architekturen. Die Abbruchbedingung der zweiten Tabellenzeile („keine
`Authority=`-Zeile mit `Developer ID Application` beginnt oder die Merkmalsliste `runtime` nicht
nennt") nennt die dritte nicht.

**`CLAUDE.md`** ist mitzuprüfen: der Absatz „Seit dem 260820 steht daneben ein zweiter Weg" und
der Absatz zur achten Station zählen die Bedingungen nicht auf und sind nach meiner Lesung nicht
falsch geworden; ich habe sie nicht Zeile für Zeile gegen den neuen Stand gehalten, weil die
Datei ausserhalb der Grenze lag. `inference:` — geprüft ist der Wortlaut, nicht jede Folgerung
daraus.

## Abhilfe

Die zwei Stellen in der `README.md` auf den neuen Stand bringen. Für die Beglaubigungstabelle
bietet sich an, sie nicht auf drei Zeilen zu erweitern, sondern auf die Frage zu stellen, die der
Code führt — `xtask/src/beglaubigung.rs`, `signaturstand_pruefen`, dessen Doc-Kommentar aus
demselben Grund keine Zahl mehr nennt: die Zahl war schon zweimal falsch.

**Warum es nicht in demselben Durchgang behoben ist:** die Grenze des Durchgangs vom 260905/0906
umfasste `xtask/`, `crates/krk-bench/`, `Makefile`, `release.sh` und `certify-only.sh`, und
`README.md` und `CLAUDE.md` ausdrücklich nicht; zwei weitere Agenten arbeiteten zeitgleich am
Baum.

**Cross-references:**
`shared/issues/260826-1443_c_eine-irrtuemliche-wiederholung-einer-schon-veroeffentlichten-zahl-wird-erst-nach-bau-und-einreichung-angehalten.md`,
`shared/issues/260826-1447_c_beglaubigen-prueft-die-universalitaet-nicht-obwohl-die-signaturanzeige-sie-mitliefert.md`

**Herkunft:** gemeinsamer Speicher. Kein Circle war aktiv, und der Befund betrifft den Bauweg des
ganzen Projekts.

---
Halb erledigt am 260908-0632, Marker bleibt `_o_`: die `CLAUDE.md`-Hälfte steht,
die `README.md`-Hälfte nicht.

**Erledigt.** Der Satz „deshalb fragt schon Station 1 nach `gh`, obwohl erst
Station 8 es braucht" nannte eine Vorabfrage, wo der Code zwei stellt. Er heißt
jetzt: Station 1 stellt zwei Vorabfragen, die erst Station 8 braucht —
`gh_pruefen`, ob `gh` vorhanden und angemeldet ist, und `release_frei_pruefen`,
ob auf der Gegenseite nicht schon ein Release `v<zahl>` steht; beide in
`veroeffentlichung`, gerufen aus `xtask/src/release.rs`. Belegt am Baum vor der
Änderung: `xtask/src/release.rs:220` und `:225`.

Die Lesung des Befunds, `CLAUDE.md` sei nicht falsch geworden, trifft damit für
den Absatz zur achten Station **nicht** zu; für den Absatz „Seit dem 260820 steht
daneben ein zweiter Weg" trifft sie zu, und er ist unverändert geblieben. Dieser
Durchgang hat den Beglaubigungsweg nur gelesen und nicht angefasst: `CLAUDE.md`
zählt dort keine Prüfungen auf.

**Offen bleibt die `README.md`-Hälfte**, und sie ist der Kern des Befunds: die
Stationstabelle bei `README.md:272`, der Absatz bei `:283-285` und die
Beglaubigungstabelle bei `:329`. Sie liegt außerhalb des Auftrags dieses
Durchgangs, der ausdrücklich allein `CLAUDE.md` umfasste. Die Abhilfe steht oben
unverändert, samt dem Vorschlag, die Beglaubigungstabelle nicht auf drei Zeilen
zu erweitern, sondern auf die Frage zu stellen, die `signaturstand_pruefen`
führt.

---
Resolved: Beide Stellen der `README.md` sind auf den Stand des Codes gebracht.

**Station 1.** Die Zeile der Stationstabelle nennt die vierte Frage („und die Gegenseite führt
noch kein Release `v<version>`"). Der Absatz darunter heißt jetzt „Station 1 stellt zwei
Fragen, die erst Station 8 braucht" statt „fragt schon nach `gh`", nennt beide und sagt dazu,
dass die zweite ins Netz geht und die erste nicht. Der Absatz unter `### Nur beglaubigen`, der
sagt, woran ein zweites `./release.sh` an Station 1 anhielte, sagt zusätzlich, warum die
vierte Frage im Fall eines an Station 7 gescheiterten Laufs durchkommt: Station 8 hat dann nie
eine Releaseseite angelegt.

**Die Beglaubigung.** Die zweite Tabellenzeile nennt keine Teilfragen mehr, sondern die Frage,
die der Code führt, und zeigt auf `signaturstand_pruefen` (`xtask/src/beglaubigung.rs`); ein
Absatz daneben sagt, dass hier aus demselben Grund keine Zahl steht wie im Doc-Kommentar
dieser Funktion — sie war schon zweimal falsch. Der Einleitungssatz „Geprüft wird zweierlei"
ist mit der Zahl weg.

**`CLAUDE.md` bleibt unverändert, und das ist geprüft und nicht angenommen.** Die eine Stelle,
die eine Bedingung von Station 1 nennt (`CLAUDE.md:125`), schreibt sie `stand_pruefen` zu, und
`stand_pruefen` (`xtask/src/release.rs`) fragt am Baum weiterhin allein nach einem passenden
Tag auf HEAD und einem sauberen Arbeitsbaum; die zwei Vorabfragen stehen daneben in
`release::ausfuehren` und nicht darin. Der Absatz zur achten Station zählt keine Bedingungen
auf. Damit ist der `inference:`-Vorbehalt des Datensatzes eingelöst.
