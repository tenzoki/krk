# Durchsicht der Runde 1 der Behebungssitzung: Schritte 1 und 2 des Plans `260826-1811`

**Reviewed-range:** `26e8039..9c02863`
**Not-opened:** none
**Sender:** coderev
**Massstab:** `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`, Schritte 1 und 2; `shared/issues/260826-1221_c_ein-gescheitertes-kopieren-…md`; `shared/issues/260826-1221_c_der-schwungleser-…md`
**Uebernommene Not-opened-Liste der vorigen Durchsicht:** none
**Gelesen:** alle Dateien der zwei Commits ueber `git show <commit>:<pfad>`, nicht aus dem Arbeitsbaum (Schritt 3 lief parallel in `tests/gemeinsam/mod.rs`, `tests/verzeichnis.rs`, `tests/umfang.rs`, `tests/leseprofil.rs`). Kein Bau, kein Probelauf: die Aussagen „rot vor gruen“ sind gegen den Code gelesen, nicht gemessen.

## Summary

Beide Behebungen halten. Der Zaehlstand der uebersprungenen Eintraege ist am Baum `9c02863` ein vollstaendiger Zeuge: jeder der sechs Wege in `kopieren.rs` und `ziel_klaeren`, auf dem ein Eintrag nicht ankommt, ruft `Steuerung::ueberspringen`, und `uebersprungen.push` hat genau diesen einen Rufer. Der Schwungleser geht ueber die Huelle, die Typpruefung bleibt am Deskriptor. Drei Befunde, alle Low, keiner betrifft die Behebung selbst: zwei Prosastellen, die der Commit `9c02863` falsch oder unvollstaendig hinterlaesst, eine Doppelung, die er stehen laesst, und die Form der zwei `Resolved:`-Zeilen.

## Totals

Critical 0 / High 0 / Medium 0 / Low 3.

## Pruefpunkt 1: behebt jeder Commit, was sein Datensatz beschreibt, und nur das

**`36e54b4` (Schritt 1).** Geaendert sind `fortschritt.rs` (zwei reine Antworten, `+18`) und `verschieben.rs` (`ueber_datentraeger` und ein Pruefmodul). Nichts daneben. `Ablauf` (`operation/mod.rs:124-129`) hat weiter zwei Werte.

Der Zaehlstand als Zeuge, Weg fuer Weg am Baum `9c02863` (`kopieren.rs` ist zwischen `26e8039` und `9c02863` unveraendert):

| Weg ohne Ankunft | Stelle | ruft `ueberspringen` | liefert |
|---|---|---|---|
| Datei: `sys_datei_kopieren` scheitert | `kopieren.rs:115-118` | ja, mit `quelle.pfad` | `Weiter` |
| Datei: Abbruch mitten in der Datei | `kopieren.rs:89-103` | nur wenn das Wegraeumen des Rests scheitert | `Abgebrochen`, vor dem Zeugen abgefangen (`verschieben.rs:128-130`) |
| Ordner: `create_dir` scheitert | `kopieren.rs:129-134` | ja, mit `quelle.pfad` | `Weiter` |
| Ordner: `lesen` scheitert | `kopieren.rs:136-142` | ja, mit `quelle.pfad` | `Weiter` |
| Ordner: Kind ueber `eintrag_kopieren` → `ziel_klaeren` | `mod.rs:432-452` | ja, an allen drei `Ueberspringen`-Zweigen (`:435`, `:443`, `:449`), mit dem Kindpfad | `Weiter` |
| Ordner: Rechte und Datum | `kopieren.rs:160-168` | ja, mit **`ziel`** | `Weiter` |
| Verknuepfung: `read_link`/`symlink` scheitert | `kopieren.rs:182-185` | ja, mit `quelle.pfad` | `Weiter` |

Der Plan zitiert `kopieren.rs:181-183` fuer die Verknuepfung; am Baum ist es `:182-184`, eine Zeile Drift, kein Befund. Der Weg „Rechte und Datum“ traegt den Zielpfad, also ist die Quelle dort nie `selbst_genannt` und bekommt die zweite Zeile aus `verschieben.rs:139-144`; das ist der Fall, den der Plan unter „Risks“ als gewollt konservativ fuehrt, und der Kommentar an `verschieben.rs:134-137` sagt ihn aus. Kein Weg scheitert ohne zu ueberspringen.

`uebersprungen_seit` schneidet mit `stand.min(len)` (`fortschritt.rs:364`); ein falscher Stand groesser als die Liste liefert still leer statt einer Panik. Bei einem Rufer und einem Stand aus derselben Liste ist das kein Defekt, ich vermerke es nur.

**`9c02863` (Schritt 2).** `sys.rs:240` ist die eine Codezeile der Behebung; die Typpruefung an `:241-246` ist unveraendert. `ohne_warten_oeffnen` (`sys.rs:906-913`) oeffnet mit `O_NONBLOCK` und nimmt es ueber `blockierend_stellen` sofort ab, ein Verzeichnisdeskriptor kommt also ohne das Kennzeichen beim Leser an. `mit_zeitschranke` ist ohne Aenderung am Rumpf von `text.rs` nach `gemeinsam` gezogen (Diff gelesen); `text.rs` braucht `mpsc` weiter (`:840`) und `Duration` (`:548`), kein toter Import. Die Aenderung an `CLAUDE.md` ist ein Absatz. Nichts daneben.

## Pruefpunkt 2: rot vor gruen, gegen den Code gelesen

**Fall a** (`eine_datei_die_nicht_ankommt_bleibt_in_der_quelle`): am Stand `26e8039` liefert `datei` fuer das Ziel unter einem fehlenden Ordner `Weiter` nach `ueberspringen` (`kopieren.rs:115-118`); `ueber_datentraeger` alt faellt unbedingt in `baum_entfernen(quelle)` (`loeschen.rs:101-104`, `remove_file`), die Quelle ist weg, `assert!(quelle.exists())` faellt. **Rot, bestaetigt durch Lesung.**

**Fall b** (`ein_ordner_mit_einem_uebersprungenen_kind_bleibt_in_der_quelle`): `create_dir(ziel)` trifft `AlreadyExists` und wird toleriert (`kopieren.rs:129-130`); `a.txt` kommt an; `b.txt` trifft in `ziel_klaeren` ein Verzeichnis gleichen Namens, die Quelle ist `Typ::Datei`, also kein Verschmelzen (`mod.rs:427`), `konflikt_loesen` liefert bei `Konfliktregel::Ueberspringen` ohne Nachfrage `Ueberspringen` (`fortschritt.rs:375`), `ueberspringen(quelle/b.txt)` (`mod.rs:443`). Alt: `baum_entfernen(quelle)` steigt ab (`loeschen.rs:106-109`) und nimmt `a.txt` mit; `assert!(quelle.join("a.txt").is_file())` faellt. **Rot, bestaetigt.** Neu: `seither` traegt `quelle/b.txt`, die Quelle ist nicht selbst genannt, bekommt ihre Zeile, `pfade` enthaelt Kind und Ordner. Gruen.

**Fall c**: an beiden Staenden gruen, wie der Sitzungseintrag sagt.

**Schritt 2**: `File::open` auf eine Roehre ohne Schreiber blockiert im `open(2)`; `mit_zeitschranke` bricht nach 5 s mit Panik ab. Die Probe haengt am alten Code. Neu: das nicht blockierende Oeffnen kehrt sofort zurueck, `metadata().is_dir()` ist falsch, der Fehler wird mit `io::Error::new` gebaut und traegt keine Betriebssystemnummer, genau was `:3553-3557` verlangt.

## Pruefpunkt 3: Regeln

- `#[must_use]` an `uebersprungen_stand` (`fortschritt.rs:354`) und `uebersprungen_seit` (`:362`). Beide `pub(crate)`, die Liste bleibt privat.
- Kein `libc`, kein neues `unsafe`: die Diffs der zwei Commits enthalten weder das Wort noch einen `extern`-Block.
- Das Pruefmodul in `verschieben.rs:157-306` traegt kein `impl Drop`; die Nadeln der Baumprobe `genau_drei_pruefordner_fassungen_stehen_im_baum` (`tests/baum.rs:136-138`) verlangen alle drei zugleich, also faellt die Datei aus dem Fund. Der Sitzungseintrag haelt fest, dass die erste Fassung mit `Drop` genau dort rot war.
- Panik mitten in einer Probe: `pruefpfad` baut ausschliesslich `std::env::temp_dir().join("krk-verschieben-<probe>-<pid>")` (`verschieben.rs:178-184`); jede Datei der drei Proben liegt darunter. Ein Rest nach Panik liegt unter `temp_dir()` und nirgends sonst. Das Vorab-Abraeumen in `pruefpfad` faengt ihn beim naechsten Lauf desselben Prozesses; ueber Prozessgrenzen bleibt er liegen, was der Auftrag als hinnehmbar nennt.

## Pruefpunkt 4: der Absatz in `CLAUDE.md`

`CLAUDE.md:151` nennt den Verzeichnisleser als dritten Ort der Huellenrufer, ohne Zahl, in dem einen Absatz, der die Huelle behandelt; `grep -n Schwungleser CLAUDE.md` trifft nur diese Zeile. Was der Satz sagt, tut der Baum, mit einer Ausnahme: „als einziger Oeffner mit `File::open`“ ist am Baum falsch, `kopieren.rs:198` und `entpacken.rs:413` rufen `File::open` weiter (auf gerade selbst angelegte Ziele; haengen koennen sie nur im Wettlauf). Dieselbe Wendung steht in `sys.rs:857`. **Befund 1**, Low.

## Pruefpunkt 5: Doppelung

- Kein dritter `Ablauf`-Wert: `git grep 'enum Ablauf' 9c02863` trifft eine Stelle mit zwei Werten.
- Zeitschranke: `gemeinsam::mit_zeitschranke` ist die einzige mit diesem Namen. `tests/verzeichnis.rs` haelt aber zwei Fassungen derselben Bauart (`:1714-1729` und `:3518-3525`) in der Datei, die jetzt die gemeinsame importiert, und die zweite begruendet ihre Eigenform mit dem Fehlen einer gemeinsamen. Der Coder hat das gesehen und als ausserhalb des Planschritts gelassen. **Befund 2**, Low, Aufraeumarbeit, nicht Teil der Behebung.

## Befunde

1. **Low** — `sys.rs:900-902` zaehlt die Rufer bis fuenf und laesst den sechsten aus, den derselbe Commit an `:855-858` eintraegt; `sys.rs:857` und `CLAUDE.md:151` nennen den Schwungleser „einzigen Oeffner mit `File::open`“, der Baum traegt zwei weitere. `issues/260826-1933_o_zwei-prosastellen-an-ohne-warten-oeffnen-zaehlen-fuenf-rufer-und-nennen-den-schwungleser-als-einzigen-file-open-oeffner.md`
2. **Low** — `gemeinsam::mit_zeitschranke` nennt sich „die eine Fassung“, `tests/verzeichnis.rs` haelt zwei eigene daneben. `issues/260826-1933_o_mit-zeitschranke-nennt-sich-die-eine-fassung-und-tests-verzeichnis-rs-haelt-zwei-eigene-daneben.md`
3. **Low** — die zwei `Resolved:`-Zeilen tragen den Sitzungsstempel, der Plan verlangt den Commit. `issues/260826-1933_o_die-zwei-resolved-zeilen-der-schritte-1-und-2-tragen-den-sitzungsstempel-statt-des-commits.md`

Keiner der drei betrifft die Behebung selbst; keiner ist Arbeit fuer den naechsten Turn dieser Sitzung, sofern der Nutzer die Prosa nicht mit Schritt 5 (`CLAUDE.md`) zusammenlegen will, der denselben Abschnitt „Was man nicht sieht“ anfasst.

## Cross-cutting

Die Zusage „jeder Weg ohne Ankunft ueberspringt“ ist jetzt an zwei Stellen dokumentiert (`verschieben.rs:112-120`, `operation/mod.rs:50-54`) und von keiner Probe ueber `kopieren.rs` selbst gehalten; die zwei Proben in `verschieben.rs` halten sie fuer den Datei- und den Kindfall. Ein neuer Fehlerzweig in `kopieren.rs` ohne `ueberspringen` bricht sie still. Der Plan nennt das unter „Risks“ als den Preis gegenueber dem dritten `Ablauf`-Wert; ich stimme dem zu und lege keinen Befund darueber an.

## Recommended sequencing

Nichts blockiert. Befund 1 laesst sich mit Schritt 5 in einem Commit erledigen, weil beide `CLAUDE.md` anfassen; Befund 2 gehoert in den zweiten Plan neben die anderen Probenaufraeumungen; Befund 3 ist eine Entscheidung ueber die Form, vor Schritt 3.

## Verification

`git show 26e8039..9c02863` fuer beide Diffs; `git show 9c02863:<pfad>` fuer `operation/kopieren.rs`, `operation/mod.rs`, `operation/verschieben.rs`, `operation/loeschen.rs`, `operation/fortschritt.rs`, `verzeichnis/sys.rs`, `tests/gemeinsam/mod.rs`, `tests/text.rs`, `tests/verzeichnis.rs`, `tests/baum.rs`; `git grep` am Commit fuer `File::open`, `ohne_warten_oeffnen(`, `recv_timeout`, `enum Ablauf`, `uebersprungen.push`. Kein `cargo`-Lauf, kein Zugriff auf den Arbeitsbaum ausser `CLAUDE.md` (unveraendert seit `9c02863`, per `git status` sauber am Start).
