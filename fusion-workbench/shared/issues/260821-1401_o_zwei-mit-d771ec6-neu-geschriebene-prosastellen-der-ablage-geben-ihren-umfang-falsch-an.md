Zwei mit `d771ec6` neu geschriebene Prosastellen der Ablage geben ihren Umfang falsch an

---

`d771ec6` hat fünf Prosastellen in `crates/krk-core/src/ablage/mod.rs` neu geschrieben. Zwei
davon geben den Umfang dessen, was sie beschreiben, falsch an: die erste der vier Regeln im
Modulkopf stellt die neue Ausnahme unbedingt dar, obwohl `Datei::leerbefund` sie auf
`bookmarks.toml` beschränkt, und der Doc-Kommentar an `Beiseite::Nicht` zählt „drei Fälle",
von denen einer keiner ist, während ein vierter Erzeuger im Baum ungezählt bleibt.

---

**Gemessen am Baumstand `d771ec6`.**

## Regel 1 im Modulkopf stellt die Ausnahme unbedingt dar

`mod.rs:104-107`, der von `d771ec6` angefügte Absatz:

```
//!   **Seit dem 260821 faellt umgekehrt ein Fall wieder heraus**: eine Datei
//!   ohne einen einzigen obersten Schluessel gilt als beschaedigt und wird
//!   trotzdem nicht gesichert, denn sie kann keinen Bestand tragen und
//!   sperrte den einen Platz gegen die Sicherung, die ihn traegt.
```

„Eine Datei ohne einen einzigen obersten Schlüssel gilt als beschädigt" steht ohne Vorbehalt.
Wahr ist es allein für `bookmarks.toml`: `Datei::leerbefund` (`pfade.rs:234-241`) gibt
`Leerbefund::Beschaedigt` nur für `Datei::Lesezeichen` zurück, und der Zweig in `Zugang::laden`
(`mod.rs:609`) prüft genau das, bevor er `ohne_obersten_schluessel` fragt. Für `keymap.toml`,
`session.toml` und `settings.toml` ist eine schlüssellose Datei **nicht** beschädigt, erzeugt
keine `Ersetzung` und erreicht den Zweig nie; die Probe
`eine_leere_datei_meldet_bei_den_drei_uebrigen_toml_dateien_nichts`
(`crates/krk-core/tests/ablage.rs:3043`) hält es fest.

**Der Satz sieben Zeilen weiter unten verschärft das.** `mod.rs:117-119`: „Alle vier
TOML-Dateien gehen durch `Zugang::laden`, und die vier Regeln gelten dort für alle vier gleich:
das Sichern selbst kennt keine Datei." Der zweite Halbsatz stimmt weiter — `beiseite_legen`
kennt keine Datei —, aber Regel 1 trägt seit `d771ec6` eine Ausnahme, deren Auslöser eine
Eigenschaft **der Datei** ist. Ein Leser, der die beiden Sätze zusammennimmt, schließt, eine
leere `session.toml` sei beschädigt und werde nicht gesichert. Beides ist falsch.

**Der Modulkopf sagt es weiter unten selbst richtig.** `mod.rs:145-152` (Abschnitt „Beschädigt
heißt nicht ‚ungültiges TOML'"): „**Kein einziger oberster Schlüssel** heißt je nach Datei
etwas anderes, und deshalb steht die Antwort in `pfade::Datei::leerbefund`". Der Kopf
widerspricht sich damit seit `d771ec6` selbst.

**Vor `d771ec6` bestand der Widerspruch nicht.** Regel 1 lautete „Nur eine beschädigte Datei
wird gesichert" und traf keine Aussage über schlüssellose Dateien; die Einordnung stand allein
im Abschnitt darunter.

## Der Doc-Kommentar an `Beiseite::Nicht` zählt drei Fälle

`mod.rs:294-300`, ebenfalls von `d771ec6` neu geschrieben:

```
    /// Drei Faelle, und der dritte ist seit dem 260821 dabei: von einer Datei,
    /// die sich nicht lesen liess, gibt es keinen Inhalt zu sichern; eine
    /// fehlende Datei ist der erste Start; und aus einer Datei ohne einen
    /// einzigen obersten Schluessel gibt es keinen **Bestand** zu sichern.
```

**Der zweite genannte Fall ist keiner.** Eine fehlende Datei erzeugt in `Zugang::laden`
(`mod.rs:592-596`) `ersetzung: None` und damit überhaupt keinen `Beiseite`-Wert. Der Satz „eine
fehlende Datei ist der erste Start" steht an drei weiteren Stellen des Moduls (`mod.rs:71`,
`:553`, `:685`) und benennt dort jedes Mal genau diesen wertlosen Ausgang. Als einer von drei
Fällen von `Beiseite::Nicht` aufgeführt, sagt er das Gegenteil.

**Ein Erzeuger fehlt in der Zählung.** `Beiseite::Nicht` entsteht an vier Stellen im Baum:

| Stelle | Grund |
|---|---|
| `mod.rs:604` (`laden`) | `Grund::NichtLesbar` — die Datei steht da und ließ sich nicht lesen |
| `mod.rs:622` (`laden`) | `Grund::Beschaedigt` — kein oberster Schlüssel; der neue Fall |
| `mod.rs:725` (`text_laden`) | `Grund::NichtLesbar` — `KeinGueltigesZiel`, Datei vorhanden |
| `einstellungen.rs:169` | `Grund::NichtAnlegbar` — `settings.toml` fehlt und ließ sich nicht anlegen |

Die vierte fällt unter keinen der drei genannten Fälle. Ihr eigener Kommentar
(`einstellungen.rs:168`) sagt „Eine Datei, die es nicht gibt, hat keinen Inhalt zu sichern" —
das ist ein eigener Grund und nicht „der erste Start", denn dieser Ausgang **erzeugt** eine
`Ersetzung`.

**Die alte Fassung zählte nicht.** Sie lautete „Der Wert jeder Ersetzung außer der
beschädigten" — eine Regel, die alle vier Erzeuger deckte, ohne eine Zahl zu behaupten.
`d771ec6` hat eine Regel durch eine Zählung ersetzt, und die Zählung war bei ihrem Entstehen
falsch. `CLAUDE.md` führt genau diesen Wechsel als wiederkehrende Fehlerquelle dieses Projekts
(Abschnitt „Projektstand", zu `Kommando`; `shared/issues/260812-2253_*` und
`shared/issues/260812-1438_*`).

## Vorschlag

Regel 1 den Vorbehalt geben, den der Zweig trägt — etwa „eine `bookmarks.toml` ohne einen
einzigen obersten Schlüssel", oder „eine Datei, für die `Datei::leerbefund`
`Leerbefund::Beschaedigt` sagt". Der Satz „die vier Regeln gelten dort für alle vier gleich"
kann dann stehen bleiben oder den Zusatz bekommen, dass Regel 1 ihren Auslöser aus
`Datei::leerbefund` bezieht.

Am Doc-Kommentar zu `Beiseite::Nicht` die Zählung wieder durch eine Regel ersetzen, statt sie
auf vier zu korrigieren: „Der Wert jeder `Ersetzung`, aus der es nichts zu sichern gibt" deckt
alle vier Erzeuger und altert nicht mit dem fünften. Der neue Fall braucht seine ausführliche
Begründung weiterhin — sie ist die eigentliche Leistung des Kommentars und nicht die Zahl.
Der Fall „eine fehlende Datei ist der erste Start" gehört aus der Aufzählung heraus; er
erzeugt keinen `Beiseite`-Wert.

**Schwere:** niedrig. Kein Fehlverhalten. Beide Stellen sind mit `d771ec6` neu entstanden, und
beide beschreiben genau die Fallunterscheidung, deren Umfang dieser Turn zweimal falsch
eingeschätzt hat.

**Gefunden:** coderev, Durchsicht des Commits `d771ec6` am 260821-1401, Bereich
`073448e..d771ec6`

**Betroffen:** `crates/krk-core/src/ablage/mod.rs:103-107`, `:117-119`, `:294-300`,
`crates/krk-core/src/ablage/einstellungen.rs:168-169`

**Domain:** code

**Verwandt:**
`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`
— dieselbe Klasse und dasselbe Modul, aber ein anderer Baumstand: jener Datensatz ist an
`e688238` gemessen und hält ausdrücklich fest, dass seine Stellen älter als der Turn sind.
Diese zwei sind mit `d771ec6` neu. Wer beide behebt, fasst sie zusammen.
`shared/issues/260821-1023_c_der-neue-leerbefund-zweig-belegt-den-einen-sicherungsplatz-mit-einer-datei-ohne-bestand.md`
— dessen `Resolved:`-Notiz nennt vier mitgezogene Prosastellen; es sind fünf, der Absatz
`mod.rs:162-167` kam ungezählt dazu.

---

## Nachtrag 260821-1532 (Abgleich): die Erzeugertabelle zählt selbst zu niedrig

**Offen, der Befund gilt, und er ist größer als hier gemessen.** Der Abschnitt „Ein Erzeuger
fehlt in der Zählung" nennt vier Stellen im Baum. Am Baumstand `4e810f9` nachgezählt über
`grep -rn 'Beiseite::Nicht' crates/krk-core/src` sind es **sieben**; drei liegen in einem
Modul, das der Bereich der Durchsicht (`073448e..d771ec6`) nicht berührt hat und das der
`grep` deshalb nicht gesehen hat.

| Stelle | Grund | in der Tabelle oben |
|---|---|---|
| `ablage/mod.rs:604` (`laden`) | `Grund::NichtLesbar` | ja |
| `ablage/mod.rs:622` (`laden`) | `Grund::Beschaedigt`, kein oberster Schlüssel | ja |
| `ablage/mod.rs:725` (`text_laden`) | `Grund::NichtLesbar`, `KeinGueltigesZiel` | ja |
| `ablage/einstellungen.rs:169` | `Grund::NichtAnlegbar` | ja |
| `tasten/belegung.rs:1464` | `Grund::Beschaedigt` — gültiges TOML, eine Ebene höher als widersprüchlich aufgefallen | **nein** |
| `tasten/belegung.rs:1498` | `Grund::NichtLesbar` — der Durchgang scheitert an der Schreibsperre | **nein** |
| `tasten/belegung.rs:1509` | `Grund::NichtLesbar` — kein Ablageordner | **nein** |

Alle sieben bauen eine `Ersetzung`; die drei aus `belegung.rs` sind nicht weniger Erzeuger als
die vier genannten. Der fünfte fällt dabei unter keinen der drei Fälle des Doc-Kommentars und
auch unter keinen der vier der Tabelle: die Datei war lesbar **und** gültiges TOML.

**Das stärkt den Vorschlag und ändert ihn nicht.** Die Empfehlung lautet, die Zählung wieder
durch eine Regel zu ersetzen — „Der Wert jeder `Ersetzung`, aus der es nichts zu sichern gibt".
Dass eine zweite Zählung, gefahren zwei Stunden nach der ersten, um drei danebenlag, ist der
Beleg dafür, dass eine Zahl an dieser Stelle nicht zu halten ist. Wer den Befund behebt, setzt
**keine** Sieben ein.

**Nachgetragen von:** reconciler, Abgleich 260821-1532, Baumstand `4e810f9`.
