# Abschlussdurchsicht: Dateisystemfragen vom Hauptfaden

**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Reviewed-range:** `998c6f37..d9a74ee1`
**Not-opened:** `fusion-workbench/orchestrator-events.jsonl`
**Review domain:** code
**Verdict:** revise

Der Bereich beginnt mit `fe48deea`, und sein Elternstand `998c6f37` ist die linke Grenze. Das Ereignisprotokoll habe ich nicht gelesen, sondern seine Zeilen allein nach Schlüsselwörtern für Geheimnisse durchsucht: kein Treffer. In den Schritten 1 bis 4 (`125e7945`, `db5be19c`, `70e843e8`, `0e6c832f`) habe ich die Betriebszeilen des Diffs gelesen und die Kommentarzeilen ausgeblendet; die Kommentare habe ich dort nur an den zitierten Stellen im Baum gelesen. Die `.record.json`-Dateien des Bereichs habe ich allein über ihren Zustand gelesen.

**Nicht im Programm gefahren:** weder der Abwurf noch der Löschweg noch eine hängende Freigabe. Jedes Fehlerbild unten ist aus dem Quelltext abgeleitet, wo es nicht ausdrücklich gemessen heißt.

## Summary

Das Paket hält, was es am Quelltext zusagt: keine der benannten Stellen fragt die Platte auf dem Hauptfaden, und die Proben sind grün. Der ausgelieferte Abwurf aus `63046f0d` ist korrekt gebaut und hat keinen Weg, auf dem eine Antwort den falschen Ordner trifft. Der größte Befund liegt daneben: die Rechnernamensprüfung aus `7ab5e891`, ebenfalls in 3.14.2, weist kodierte Bonjour-Namen mit Leerzeichen ab. Die übrigen Befunde sind Folgen davon, dass eine Antwort jetzt spät ankommen kann.

## Totals

| Schwere | Zahl (aus den Einträgen unten) |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 5 |
| Low | 2 |

## Findings by theme

### Eine Prüfung, die weiter greift als verlangt

**Hoch, `7ab5e891`, `261104-1955-rechnername-taugt-weist-kodierte-bonjour-namen-mit-leerzeichen-ab.md`.** `crates/krk-ui/src/appkit/verbinden.rs:776` prüft das dekodierte `host`. Gemessen auf 27.2: `smb://Mein%20NAS._smb._tcp.local/Daten` hat das `host` `Mein NAS._smb._tcp.local`, also weist KRK die Adresse ab, beim Anlegen, beim Einhängen und beim Übergeben. Dass Nutzer solche Namen haben, ist `inference:`, an dieser Stelle war keine Freigabe eingehängt. `crates/krk-core/src/heimordner/verbindung.rs:56` bis `:58` ist nicht nachgezogen. Antwort auf Frage 3: es bricht einen Fall, und die Kommentare stimmen nicht überein.

### Späte Antworten

Der Plan regelt veraltete Antworten als eine Regel: gegen den gereisten Wert anwenden, das vom Stand Abhängige neu prüfen. Drei Einzugsmethoden lassen dabei etwas aus.

**Mittel, `5cdf39e3`, `261104-1956-das-loeschblatt-oeffnet-nach-der-antwort-ohne-zu-fragen-ob-schon-ein-blatt-steht.md`.** `loeschbefund_einziehen` (`anwendung.rs:8365`) fragt `vorgang_laeuft`, nicht `blatt_steht`. Ein Blatt, das der Nutzer in der Lücke öffnet, verliert den Griff an das eingereihte Löschblatt. Antwort auf Frage 2, erster Teil: die Stufentafel wird neu gefragt, die Lage am Fenster nicht. Ein versehentliches Bestätigen scheidet aus, die Eingabetaste liegt auf „Abbrechen“.

**Mittel, `5cdf39e3`, `261104-1957-die-sperre-des-loeschwegs-gilt-programmweit-und-faellt-ohne-antwort-nie.md`.** Antwort auf Frage 2, zweiter Teil: bleibt die Antwort aus, fällt die Sperre nicht. Sie ist ein `Cell<bool>` im Delegierten und steht vor der Unterscheidung des Ortes, also löscht KRK dann nirgends mehr bis zum Neustart. Die Zusage „`fertig` genau einmal“ in `nebenfaden.rs:29` stimmt bei einer Panik nicht.

**Mittel, `d86ad752`, `261104-1959-eine-spaete-antwort-der-pfadeingabe-stellt-den-gerade-sichtbaren-tab-um.md`.** Mit der Frage reist kein Tab und keine Nummer. `Wechseln` trifft den sichtbaren Tab, und die zuletzt angekommene von zwei Fragen gewinnt.

**Mittel, `63046f0d`, `261104-1958-der-abwurf-startet-je-ordnerwechsel-einen-faden-ohne-obergrenze.md`.** Ein Faden je Eintritt in einen Ordner, ohne Grenze und auf einer hängenden Freigabe ohne Ende. Das ist kein Fehlurteil, aber eine unbegrenzte Fadenzahl, die mit der Zeigerbewegung wächst.

### Der Abwurf, Frage 1 im Einzelnen

Kein Befund außer dem eben genannten. Gelesen und gemessen:

- **Alte Antwort, falscher Ordner oder Zug: kein Weg.** `schreibrecht_einziehen` (`crates/krk-ui/src/appkit/tabelle.rs`) trägt nur ein, wenn `gilt_fuer(folge, ordner)` stimmt, und `merkerstand` antwortet für jeden anderen Zug und Ordner `Fehlt`. Der eine Grenzfall: verlässt der Zeiger die Tabelle und kehrt zum selben Ordner zurück, kann eine ältere Antwort die neue Frage vorweg beantworten. Sie gilt demselben Ordner im selben Zug, die spätere überschreibt sie, und ich halte das für harmlos. Der Satz „Beim nächsten Eintritt wird frisch gemessen“ stimmt in diesem Fall nur fast.
- **`RefCell`: kein Konflikt.** Die Ausleihe in `schreibrecht_gemerkt` endet mit der `let`-Anweisung, die in `schreibrecht_einziehen` berührt nichts außerhalb, und der Rückweg läuft über `exec_async`, auch bei gescheitertem Fadenstart. Es gibt keine Verschachtelung.
- **Signaturen: stimmen.** `NSDragging.h:121` `draggingExited:(nullable id <NSDraggingInfo>)` und `:126` `draggingEnded:(id<NSDraggingInfo>)`; die Überschreibungen nehmen `Option<&…>` und `&…`. Die Zeilenangaben im Modulkopf stimmen am SDK dieses Geräts.
- **`responds_to`: richtig gefragt.** `objc2` 0.6.4 ruft `class_respondsToSelector`, das die Instanzen der Klasse meint und nicht `+respondsToSelector:`. Gemessen auf 27.2 an einem Probeprogramm: `draggingExited:` 1, `draggingEnded:` **0**. Die Frage ist also keine Vorsicht ohne Anlass: ohne sie stürzte `draggingEnded:` mit einem unbekannten Selektor an `super` ab. Gefragt wird `NSTableView` und nicht die eigene Klasse, und das ist die Oberklasse, die `super` meint. Die Klassenkopf-Formulierung „jede ihrer drei Überschreibungen ruft die Fassung der Oberklasse“ ist durch den Nachsatz gedeckt.

**Bleibt offen, nicht geprüft:** ob die Hauptschlange während eines Zuges aus KRK selbst läuft, und damit ob die Antwort vor dem Loslassen ankommt. `inference:` sie läuft in den gemeinsamen Modi der Ereignisschleife und damit auch beim Zeigerverfolgen. Liefe sie nicht, urteilte der Abwurf immer mit `Unbekannt`, und ein schreibgeschützter Ordner würde nie mehr am Zeiger abgewiesen. Das sieht keine Probe; der Lauf an einem schreibgeschützten Ordner gehört zu dem, was der Nutzer nachholen muss: aus dem Finder und aus dem anderen Dateifenster ziehen, und den Zeiger über dem Ordner ruhen lassen und bewegen.

### Der Rest außerhalb der Zusage

**Mittel, `261104-2000-geraeteaufzaehlung-f2-und-secrets-fragen-die-platte-weiter-auf-dem-hauptfaden.md`.** Frage 4, erster Teil: ein Defekt, und er ist breiter als gefragt. Neben `volumes.rs:208` und `:212` (Rufer `anwendung.rs:3904`) stellen F2 (`anwendung.rs:6446`, `:6464`) und das Öffnen von `secrets.txt` (`editor.rs:4927`) eigene Fragen auf dem Hauptfaden. Der Sammeldefekt nennt sie nicht, und sein Schließen ist deshalb nicht falsch. Ob `eingehaengte` die Platte erreicht, bleibt ungemessen.

**Niedrig, `261104-2001-die-liste-der-uebergaben-in-der-regel-laesst-setdirectoryurl-und-readobjects-aus.md`.** Frage 4, zweiter Teil: `setDirectoryURL` (`blaetter/ortwahl.rs:118`) fehlt in der Liste. Daneben fehlt `readObjectsForClasses:options:` (`zwischenablage.rs:626`), und der Kommentar an der Abwurfprobe sagt darüber mehr, als der Quelltext hergibt.

**Niedrig, `261104-2002-der-kommentar-an-plattenfrage-nennt-zwei-von-vier-wegen.md`.** `nebenfaden.rs:52` nennt die Schritte 5 und 6 und lässt den Löschweg und den Abwurf aus.

### Bauprofil, Frage 5

Kein Befund. Der Kommentar in `Cargo.toml:830` bis `:858` trennt „Gemessen“ und „Nicht gemessen“ sauber, und er nennt die Vermutung zu Xcode 27.2 als Vermutung. Nachgefahren am 261104 mit eigenem `CARGO_BUILD_BUILD_DIR` und `CARGO_TARGET_DIR`: mit `CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_STRIP=debuginfo` bricht `cargo build --profile release --package arc-swap --target x86_64-apple-darwin` mit E0463 und E0252 ab, und das `-v` zeigt `strip=debuginfo` an `rustversion`. Mit `none` und mit der Vorgabe des Baums läuft derselbe Bau durch. **Nicht nachgefahren:** der Bau ohne die Zeile in `Cargo.toml`; ich habe den Fehler über die Umgebungsvariable hervorgerufen und nichts am Baum geändert. Die Eingangssätze über die Vorgabe von Cargo stehen über der Überschrift „Gemessen“ und sind damit nicht als gemessen gekennzeichnet; das ist kein Mangel, sie geben Cargos Verhalten wieder.

Die Änderung trägt ein Rest: `make check` sieht den Defekt nicht, der Kommentar sagt das, und einen Bau im Profil `release` mit `--target` außerhalb der Auslieferung gibt es weiterhin nicht. Der Lauf dauert ohne Bündel Sekunden. Ob er zu `make check` gehört, ist eine Frage an den Nutzer und kein Defekt.

### Zusagen des Plans, Frage 6

`## Where this work stops`, Zeile für Zeile, gegen den Baum an `d9a74ee1`:

| Zeile | Stand | Beleg |
|---|---|---|
| Zehn Schritte erledigt, `make check` grün | Schritte erledigt; `make check` nicht selbst gefahren | `control.steps` im Plan: 10 von 10 `done`; gefahren habe ich die fünf Rumpfproben der Wege und `kein_betriebscode_der_oberflaeche_baut_eine_dateiadresse_ohne_isdirectory`, grün |
| Keine Frage in den Rümpfen der Wege 4, 5, 6, 3g, je eine Probe | gilt für die Rümpfe | `die_pfadeingabe_…`, `ort_waehlen_…`, `der_loeschweg_…`, `der_abwurf_…`; was sie nicht sehen, sagen ihre Kommentare |
| `fileURLWithPath(` ohne Art allein in `in_den_papierkorb` | gilt | `grep` über `crates`: allein `appkit/papierkorb.rs:155` |
| `Resolved:` an beiden Defekten | gilt | beide Zeilen stehen, beide Datensätze `closed` |
| `canonicalize` in `auffrischung.rs` bleibt offen | gilt | `260826-1442_*_gleicher-ordner-ruft-canonicalize-je-gemeldetem-pfad-und-dateifenster-und-die-kosten-sind-ungemessen.md` steht auf `open` |
| Übergaben bleiben auf dem Hauptfaden | gilt | `261102-1312-was-heisst-fragt-das-dateisystem-auf-dem-hauptfaden-bei-einer-uebergabe-an-das-system.md` beantwortet, Möglichkeit 1; zur Liste selbst `261104-2001-…` |
| Keine Messung gegen die zehn Zusagen | gilt | `261102-1313-schuldet-das-hauptfadenpaket-einen-abnahmelauf-gegen-die-zehn-zusagen.md`, Möglichkeit 2; kein Commit des Bereichs berührt `messungen/` |
| Lauf an hängender Freigabe ist Nutzerarbeit | gilt | keiner gefahren |
| Paket liefert nicht aus | nach Vorgabe kein Befund | 3.14.2 hat der Nutzer angeordnet |

Plan und Paket stehen noch auf `in_progress` und `claimed`.

### Die übrigen Commits

Kein Befund. `7ab5e891` in `crates/krk-core/tests/archiv_sqlite.rs` und `appkit/editor.rs` lockert keine Zusicherung; die Sqlite-Probe bildet den Fall über `ATTACH` nach, ihr Kommentar nennt die gemessene Fassung. `e373b0d1` setzt `plugin_version` in `.fusion-setup` von 13.3.9 auf 13.3.7; das entspricht der hier installierten Fassung. `48ebcb8f` ändert allein die Versionszeilen in `Cargo.toml` und `Cargo.lock`. `d9a74ee1` führt allein den Übergang des Sammeldefekts. Die Schritte 1 bis 4 tun, was der Plan sagt, und ich habe in den Betriebszeilen keine vertauschte Art gefunden; die Verknüpfung kommt überall als `false`, und die Vorschau leitet die Art aus ihrem Inhalt ab (`vorschaumodell.rs` `aktive_adresse`).

`Auswahl::adressen` (`kommandos/operationen.rs`) hält die Längengleichheit allein mit `debug_assert_eq!`, und das `zip` kürzte im Profil `release` still. `ist_ordner` ist `pub`, und außer `betroffene` baut nur Prüfcode den Wert. Das ist heute kein Fehler und deshalb kein Datensatz.

## Cross-cutting observations

Die Regel „was vom heutigen Stand abhängt, prüft die Einzugsmethode neu“ ist in jedem Weg einzeln ausgelegt, und alle vier Wege haben sie für sich beantwortet: der Löschweg prüft den Vorgang, die Pfadeingabe den Ort, „Ort wählen…“ die gehaltene Datei, der Abwurf Zug und Ordner. Allein der Abwurf trägt eine Kennung seiner Frage. Die zwei Befunde an Löschweg und Pfadeingabe haben dieselbe Lösung, eine Nummer je Frage, und „Ort wählen…“ könnte sie ebenfalls tragen, wenn zwei Wahlen sich überholen. Das wäre eine Regel und keine dritte Sonderprüfung.

Die Rumpfproben halten alle fest, wo eine Frage nicht steht, und keine, was bei einer späten Antwort geschieht. Das hat der Plan so vorgesehen, und es erklärt, warum `make check` hier nichts sehen konnte.

## Recommended sequencing

Der Rechnername zuerst, er ist ausgeliefert und betrifft den Normalbetrieb: `261104-1955-…`. Dann die zwei Löschbefunde zusammen, `261104-1957-…` und `261104-1956-…`, mit der Nummer aus dem vorigen Abschnitt, und `261104-1959-…` mit derselben Bauform. Danach `261104-1958-…`. `261104-2000-…` braucht zuerst die Entscheidung des Nutzers über den Umfang. Die zwei niedrigen Datensätze sind Aufräumarbeit.

Ein Lauf des Nutzers ist unabhängig davon offen, denn `63046f0d` ist in 3.14.2 ungefahren: Abwurf aus dem Finder und aus dem anderen Dateifenster auf einen schreibgeschützten Ordner, und danach dieselben Wege an einer getrennten Freigabe.
