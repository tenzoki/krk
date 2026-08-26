# Vollbaum-Durchsicht R14: `xtask/` und die drei Hüllen darum

**Reviewed-range:** `004ff72..c13bf1c`
**Not-opened:** none
**Sender:** coderev
**Gelesen:** die neun Dateien unter `xtask/src/` (7.203 Zeilen per `wc -l`), dazu `xtask/Cargo.toml`, `Makefile`, `release.sh`, `certify-only.sh`; zum Abgleich `README.md` (Zeilen 201–383) und `crates/krk-core/src/ablage/pfade.rs:226-234`.
**Baumstand:** `c13bf1c`; der Quelltext ist seit `004ff72` unverändert, alle Commits dazwischen tragen Werkbankdateien.

## Summary

Die Kette hält ihre Kernzusagen: acht Stationen und keine neunte (`release.rs:202-245`), Station 1 fragt `gh` vor dem ersten Übersetzungslauf (`:210`), Station 8 schiebt HEAD und Tag in einem `git push` (`git.rs:396-401`) und legt die Seite erst nach der Existenzfrage an (`veroeffentlichung.rs:608`). Die Tag-Frage ist beantwortet: ein Tag bleibt bei jedem Scheitern lokal stehen, blockiert den Wiederholungslauf mit derselben Zahl aber nicht (`version.rs:148-154`). Was nicht hält, ist die Prosa darüber: fünf Stellen sagen, ein zweites `./release.sh` bräche an Station 1 ab, zwei andere sagen, es fahre gleich weiter, und der Code gibt den zweiten recht. Daneben fehlen den Abbruchmeldungen der Stationen 7 und 8 genau die Wiederaufnahmebefehle, für die es die zwei Nebenwege gibt, und der feste Releasetext nennt vier von sieben Ablagedateien.

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 4 |
| Low | 9 |
| Also seen (kein neuer Datensatz) | 3 |

## Die Tag-Frage, beantwortet

`cargo xtask version` setzt den Tag **vor** dem Bau, als letzten Schritt des Halbschritts (`version.rs:224`, `:231-257`); `cargo xtask release` liest ihn danach nur (`release.rs:279-281`). Scheitert eine der acht Stationen, bleiben Eintrag und Tag lokal stehen — so gewollt und im Modulkopf begründet (`version.rs:53-63`). Ein zweiter Lauf mit **derselben** Zahl fällt in `Vorhaben::NichtsZuTun` (`version.rs:368-370`, `:148-154`) und Station 1 lässt ihn durch, solange keine verfolgte Datei geändert ist. Ein lokaler Tag blockiert den nächsten Lauf also nicht. Er bleibt aber **liegen**, wenn der Nutzer nach dem Scheitern eine **andere** Zahl wählt: `Tagliste(v<neu>)` sieht den alten nicht (`version.rs:134`), und Station 8 schiebt je Lauf genau einen Tag (`veroeffentlichung.rs:517-522`). Der verwaiste Tag steht dann lokal, benennt einen nie ausgelieferten Stand, und `README.md:380` reicht ihn mit `git push origin --tags` von Hand öffentlich nach — die eine Marke, die die Aufsicht des Werkzeugs selbst verbietet (`git.rs:466-477`). Befund M4.

## Was ich geprüft und nicht beanstandet habe

- **Vier Schichten, jede eine Sache.** `release.sh:31-35` prüft allein die Argumentzahl und `exec`t. `Makefile:150-154` setzt `NOTARPROFIL` und den Pfad zu cargo, fährt `version` und `release` als zwei Prozesse (begründet `:139-146`). `version.rs` schreibt die Zahl an eine Stelle (`:196-206`). `release.rs:195-246` trägt die Logik. Keine Logik ist in eine Hülle gerutscht.
- **Acht Stationen gegen `README.md:254-267`:** 1 `:202`+`:210`, 2 `:213`, 3 `:224-226`, 4 `:227`, 5 `:229`, 6 `:230`, 7 `:236`, 8 `:242-245`. Vollständig, keine neunte; die drei Vorläufe a/b/c (`:212`, `:214`, `:222`) stehen wie im Modulkopf beschrieben.
- **Station 8 nach Schub-Erfolg und `gh`-Scheitern:** der Wiederholungslauf über `cargo xtask veroeffentlichen` schiebt HEAD und Tag erneut, was für unveränderte Referenzen ein Leerlauf ist, und fragt vor dem Anlegen `gh release view` (`veroeffentlichung.rs:608`, `:654-663`). Kein doppeltes Release, kein gepushter Tag ohne reparablen Weg. Der eigenständige Rufer stellt gegenüber der achten Station **eine** Frage mehr, die Tagfrage (`:126-129`), und keine weniger als die Station selbst; den Arbeitsbaum prüft er nicht, und das steht dreifach dokumentiert (`:29-35`, `main.rs:158`, `README.md:365-366`).
- **Die vier Bytes:** `TICKETKENNUNG = b"s8ch"` an `Contents/CodeResources` (`veroeffentlichung.rs:65-68`, `:346-348`), nicht an `_CodeSignature/CodeResources`. Ein Bündel ohne Ticket kann die Prüfung nur bestehen, wenn eine fremde Datei dieses Namens mit dieser Kennung dort liegt; `Vorlage::zusammensetzen` räumt vor jeder Montage das ganze Bündel ab (`bundle.rs:219-222`), also bleibt kein Ticket eines früheren Laufs an einem neuen Bau hängen. Die Fehlrichtung bei einer geänderten Kennung ist die sichere (`:341-344`). Inferenz, nicht gemessen.
- **`beglaubigen` prüft am Bündel:** `Contents/Info.plist` **im Bündel** (`beglaubigung.rs:108-116`), Schlüssel `CFBundleShortVersionString` (`:70`), ausdrücklich nicht gegen `Cargo.toml` (`:159-162`). Weder `git::rufen` noch `auslieferungsstand_pruefen` stehen in der Datei; die Probe `:637-645` hält es.
- **Drei Stufen der Identitätssuche** (`sign.rs:48-69`, `:81-127`): Umgebungsvariable, Name/Präfix, einzige gültige. Keine Stufe fällt still auf Ad-hoc; am Ende steht `anleitung` (`:413-457`). Was durchkommt, ist ein ausdrücklich gesetztes `KRK_SIGN_IDENTITY=-`, siehe L2.
- **`iconutil`:** alle zehn Quellen werden vor dem ersten Übersetzungslauf geprüft (`bundle.rs:199`, `:383-396`); ein fehlendes PNG bricht benennend ab, bevor ein Verzeichnis entsteht. Die zehn Namen sind eindeutig (Probe `:580-589`) und jedes PNG unter `iconset/` ist zugeordnet (Probe `:598-614`).
- **`RELEASETEXT`** steht an genau einer Stelle (`veroeffentlichung.rs:551-575`); die Betriebsregel steht darin (`:563`, `:565`, `:567`, `:573-574`) und die Probe `:955-990` hält die Aussagen je einzeln mit eigenem Namen, nicht als einen Teilstring. Zwei Anmerkungen: die Regelaussage selbst („Ein Überkopieren ist gefahrlos, ein Löschen ist es nicht", `:567`) hält keine Nadel, nur ihre Überschrift; und die Aufzählung des Ordnerinhalts ist unvollständig (M2).
- **Vierte Prüfordner-Fassung** (`release.rs:905-932`): Prozesskennung plus Laufnummer, Abräumen in `Drop`, kein Anlegen im Konstruktor. Nicht auseinandergelaufen, räumt auf. Der offene Datensatz `260826-1302` deckt es; Also-seen-Zeile angehängt.
- **Fremde Werkzeuge:** mit festem Pfad `git` (`git.rs:745`), `codesign` (`sign.rs:234`, `beglaubigung.rs:192`), `security` (`sign.rs:343`), `lipo` (`release.rs:648`, `:669`), `ditto` (`beglaubigung.rs:345`, `veroeffentlichung.rs:388`), `xcrun` (`beglaubigung.rs:363`, `:379`, `:399`). Über den Suchpfad: `gh` (`veroeffentlichung.rs:59`, begründet `:37-45`), `iconutil` (`bundle.rs:427`), `rustup` (`release.rs:604`), `cargo` über `CARGO` (`bundle.rs:279`, `messen.rs:70`). Die Entscheidung `260821-1221` bleibt offen; hier nur der Bestand.
- **`unwrap`/`expect` außerhalb der Proben:** eine Stelle, `bundle.rs:302`, am Elternordner von `CARGO_MANIFEST_DIR` — der existiert beim Übersetzen zwangsläufig. Kein echter Fehlerfall.
- **Stille Fehlschläge:** `beglaubigung.rs:369` (`let _ = fs::remove_file`) folgt der Projektkonvention; die zwei in `release.rs:918`, `:928` liegen im Prüfmodul. Kein `catch`-Äquivalent, kein verschluckter Prozessstatus außer dem begründeten von `gh --version` (`veroeffentlichung.rs:163-166`).
- **Eine Station läuft weiter, obwohl die vorige scheiterte:** nirgends. Jede Station endet auf `?`; `release.rs:195-246` hat keinen Zweig, der einen Fehler abfängt und weiterfährt.

## Findings by theme

### A. Prosa gegen Code — der Wiederholungslauf

**M1 — Zwei Dokumentationsstände zum zweiten `./release.sh` nach einem Scheitern widersprechen sich, und der Code gibt dem selteneren recht (Medium).**
`shared/issues/260826-1441_o_*`

`version.rs:60-63` und `README.md:243-252` sagen: nach einem Abbruch der Stationen fällt ein zweites `./release.sh <zahl>` durch den Halbschritt und „fährt gleich weiter". `certify-only.sh:22-24`, `Makefile:157-160`, `main.rs:130-132`, `beglaubigung.rs:14-19`, `README.md:304-310` und `CLAUDE.md` sagen: es „bräche an Station 1 ab, weil der Tag nach dem Lauf nicht mehr allein auf HEAD steht". `stand_pruefen` (`release.rs:307-314`) fragt nur zweierlei — passt ein Tag auf HEAD, ist der Baum sauber. Ein gescheiterter Lauf bewegt HEAD nicht und ändert keine verfolgte Datei. Der Abbruch tritt nur ein, wenn zwischen den Läufen etwas eingetragen oder geändert wurde — am 260820 die Werkbankdateien aus `260813-1515`. Die Daseinsberechtigung von `certify-only.sh` ist damit richtig (kein zweiter Bau, keine zweite Signierung), aber falsch begründet; `beglaubigung.rs:17` nennt beide Gründe und ist die einzige Stelle, die stimmt. Betroffen: `release.sh`-Weg und `certify-only.sh`-Weg. Vorschlag: die Begründung überall auf „übersetzt beide Ziele neu und reicht neu ein" kürzen und den Station-1-Satz als Bedingung („wenn seither etwas eingetragen ist") formulieren.

### B. Was ein Abbruch dem Nutzer sagt

**M2 — Die Abbruchmeldungen der Station 7 nennen `./certify-only.sh` nicht, obwohl der Weg für genau diesen Abbruch gebaut ist (Medium).**
`shared/issues/260826-1442_o_*`

`beglaubigung.rs:371-376` (Einreichung gescheitert, der Fall vom 260820) und `:385-389` (Heften gescheitert) nennen Bündelpfad und `notarytool log`, aber keinen Wiederaufnahmebefehl. Allein der Zweig „Profil fehlt" (`:333-336`) nennt `./certify-only.sh <zahl>`. Der Nutzer, der den Zeitüberlauf erlebt, liest an der Stelle des Scheiterns nicht den Weg, der dafür da ist. Betrifft beide Rufer. Vorschlag: dieselbe zweizeilige Abhilfe in beide späten Zweige.

**Also seen — Station 8: „Derselbe Aufruf noch einmal" nennt keinen Befehl.** `veroeffentlichung.rs:245-247`, `:626-630`, `:676-680`. Für den Rufer über `./release.sh` ist „derselbe Aufruf" ein vollständiger Neubau samt Neueinreichung; der billige Weg `cargo xtask veroeffentlichen <zahl>` steht in keiner der drei Meldungen. Der offene Datensatz `circles/260821-1644-veroeffentlichen-als-achte-station/issues/260821-2105_o_*` trägt die Meldung als unvollständige Fallunterscheidung; Also-seen-Zeile dort angehängt.

**L1 — Eine irrtümliche Wiederholung einer schon veröffentlichten Zahl wird erst nach Bau und Einreichung angehalten (Low).**
`shared/issues/260826-1443_o_*`

`v1.2.0` steht auf HEAD und ist veröffentlicht. `./release.sh 1.2.0` → `NichtsZuTun` (`version.rs:148`), Station 1 grün, drei Übersetzungsläufe, Signierung, Einreichung bei Apple — und dann `release_steht` (`veroeffentlichung.rs:608`): „steht bereits". Die Existenzfrage kostet einen `gh`-Aufruf und ließe sich neben `gh_pruefen` an Station 1 stellen (`release.rs:210`); die Begründung dafür steht schon da (`:204-209`). Vorschlag zur Entscheidung, nicht abgeleitet: die Frage an Station 1 spiegeln, allein für `Tagfrage::Erledigt`.

### C. Der Releasetext

**M3 — `RELEASETEXT` nennt vier von sieben Ablagedateien, und die Probe hält die unvollständige Liste (Medium).**
`shared/issues/260826-1444_o_*`

`veroeffentlichung.rs:569-571`: „die Lesezeichen, die gesicherte Sitzung, die abweichende Tastenbelegung und die zwei Notizzettel". `Datei::ALLE` (`crates/krk-core/src/ablage/pfade.rs:226-234`) führt sieben: dazu `Einstellungen` und `Leser` (die von Hand gepflegten Leseprofile, `readers.toml` seit Runde 16). `CLAUDE.md` stellt dieselbe Lücke an seiner eigenen Aufzählung fest („hat die Einstellungen und die Leseprofile übergangen, seit es sie gibt"). Der Releasetext ist die eine Stelle, die ein Fremder beim Installieren liest; wer die Betriebsregel wegen der vier genannten Dinge für sich als unerheblich hält, verliert seine Leseprofile. Die Probe `:973-978` hält genau die vier. Vorschlag: den Satz auf die Regel statt die Liste stellen („alles, was KRK sich merkt") oder die zwei nachtragen und die Probe mitziehen.

### D. Der Tag, der liegen bleibt

**M4 — `README.md:380` reicht verwaiste Tags mit `git push origin --tags` öffentlich nach, der Marke, die die Aufsicht des Werkzeugs verbietet (Medium).**
`shared/issues/260826-1445_o_*`

Mechanik oben unter „Die Tag-Frage". Der Handgriff ist für die Tags der Runden vor der Station 8 geschrieben (`README.md:369-381`), schiebt aber jeden lokalen Tag, auch einen, der nach einem gescheiterten Lauf mit anschließend neu gewählter Zahl zurückblieb. Die `comm`-Zeile davor zeigt ihn, sagt aber nicht, dass er zu prüfen ist. `git.rs:466-477` hält `--tags` in `MARKEN` mit dem Doc-Kommentar „erweitert die Reichweite"; die README empfiehlt es von Hand. Am 260826 stehen lokal und auf `origin` dieselben 18 Tags, ein verwaister fehlt heute. Vorschlag: den Handgriff auf `git push origin refs/tags/v<zahl>` je fehlendem Tag umschreiben, oder den verwaisten Tag als Fall benennen.

### E. Signierung

**L2 — `KRK_SIGN_IDENTITY=-` signiert ad hoc, und keine Stelle sagt es (Low).**
`shared/issues/260826-1446_o_*`

`aus_umgebung` (`sign.rs:254-261`) trimmt und prüft auf nichtleer; `-` kommt durch, und `codesign --sign -` ist die Ad-hoc-Signatur. Der Modulkopf `:6-10` und der Hilfetext `main.rs:40-41` formulieren die Regel absolut. Wer die Variable so setzt, tut es absichtlich; der Weitergabehinweis sagte dann „mit "-", und dieser Name ist nicht der einer Developer-ID" (`:194-198`). Vorschlag: `-` in `aus_umgebung` benennend abweisen oder den Satz „nicht ad hoc" auf die drei Suchstufen einschränken.

**L3 — `beglaubigen` prüft die Universalität nicht, obwohl die Anzeige sie mitliefert (Low).**
`shared/issues/260826-1447_o_*`

`README.md:302-303` und `beglaubigung.rs:12-14` sprechen vom „universellen" Bündel; geprüft werden Developer-ID und `runtime` (`:231-268`). Die Zeile `Format=app bundle with Mach-O universal (x86_64 arm64)` steht in derselben `codesign --display`-Ausgabe (`:431`), die `signaturstand_pruefen` schon liest. Ein über `KRK_SIGN_IDENTITY` mit Developer-ID und gehärteter Laufzeitumgebung nachsigniertes Einzelarchitektur-Bündel aus `bundle` ginge bei Apple ein. Konstruiert, aber billig zu schließen.

### F. Werkzeugwege und Dokumentation

**L4 — `iconutil` wird über den Suchpfad gerufen, während Doc-Kommentar und Abbruchmeldung `/usr/bin/iconutil` behaupten; `messen.rs` liest `CARGO` ein zweites Mal neben `bundle::cargo()` (Low).**
`shared/issues/260826-1448_o_*`

`bundle.rs:427` `Command::new("iconutil")`, `:85-86` „liegt unter `/usr/bin/iconutil`, wie `codesign`", `:434-435` dieselbe Behauptung in der Meldung — der Aufruf tut nicht, was die zwei Sätze sagen. `messen.rs:70` baut `std::env::var("CARGO").unwrap_or_else(…)` nach, obwohl `bundle.rs:275-277` sagt, „beide inneren Aufrufe lesen ihn hier"; es sind drei. Die offene Frage `260821-1221` (Suchpfad als Regel) wird davon nicht berührt und hier nicht entschieden.

**L5 — `README.md:256` beschreibt Station 2 als „keine `use objc2`-Zeile", der Code fängt seit dem 260806 auch den ausgeschriebenen Pfad (Low).**
`shared/issues/260826-1449_o_*`

`release.rs:39-41`, `:492-498`, `:510-528`. Die Tabelle nennt die Hälfte der Prüfung. `main.rs:77-78` hat dieselbe Halbaussage („keine `use objc2`-Zeile"). Kleiner Nachtrag an zwei Stellen.

### G. Konventionen der Kiste

**L6 — `xtask` trägt kein `#![deny(unsafe_code)]` (Low).**
`shared/issues/260826-1450_o_*`

`main.rs` führt kein Kistenattribut; `grep -rn 'unsafe' xtask/src` trifft nur den Doc-Kommentar `release.rs:43`. `CLAUDE.md` zählt die Grenze für die drei `krk-*`-Kisten und nennt `xtask` nicht. Die Kiste ist heute frei von `unsafe`, also kostet die Zeile nichts und hält den Zustand. Verwandt: `260826-1302_o_die-probenziele-des-kerns-tragen-kein-deny-unsafe-code-…` für die Probenziele.

**L7 — `#[must_use]` ist in `xtask` ungleich verteilt: `veroeffentlichung.rs` trägt es an allen vierzehn reinen Antworten, `version.rs`, `messen.rs` und `main.rs` an keiner (Low).**
`shared/issues/260826-1451_o_*`

Zählung per `grep -c '#\[must_use\]'`: `veroeffentlichung.rs` 14, `git.rs` 6, `release.rs` 2, `beglaubigung.rs` 2, `bundle.rs` 1, `sign.rs` 1, `version.rs` 0, `messen.rs` 0, `main.rs` 0. Ohne Attribut, mit reinem Rückgabewert: `git.rs` `tag_steht` (`:779`), `geaenderte_dateien` (`:767`), `aufsichtsbefund` (`:540`), `gewaltbefund` (`:621`), `stellungsbefund` (`:603`), `Gestalt::befund` (`:167`); `sign.rs` `enthaelt_identitaet` (`:363`), `gueltige_namen` (`:374`), `eintragsname` (`:392`), `abschnitt_der_treffer` (`:316`), `anleitung` (`:413`), `developer_id_namen` (`:130`); `bundle.rs` `plist_zeichenkette` (`:465`), `zielpfad` (`:522`), `wurzel` (`:299`), `cargo` (`:278`); `release.rs` `verletzt_grenze` (`:492`), `ist_objc2_use` (`:555`), `nennt_objc2_pfad` (`:510`); `version.rs` `wertspanne` (`:515`), `eintragsmeldung` (`:531`), `arbeitsbaum_meldung` (`:388`), `zuruecknehmen` (`:272`). Heute lässt kein Rufer eine davon fallen; der Befund ist die Regel aus `CLAUDE.md`, nicht ein Verhalten. Dieselbe Gestalt wie die acht `must-use`-Datensätze dieser Sitzung.

### H. Proben

**L8 — Der Sammler der Zählproben in `release.rs` verschluckt Lesefehler und liest die Werkbank mit (Low).**
`shared/issues/260826-1452_o_*`

`sammeln` (`release.rs:1304-1322`): `let Ok(eintraege) = fs::read_dir(ordner) else { return; }` und `eintraege.flatten()`. Ein unlesbares Verzeichnis lässt `xtask_ruft_git_an_genau_einer_stelle` (`:1086-1099`) und `allein_release_fragt_nach_tag_und_arbeitsbaum` (`:1108-1131`) mit weniger Dateien grün laufen, als der Baum trägt. Ausgeschlossen sind nur `target` und `.git`; `fusion-workbench/` und `spikes/` werden gelesen, obwohl `:1272-1279` für die Nachbarprobe begründet, warum die Werkbank draußen bleibt. Vorschlag: Lesefehler zum `panic!` machen, Ausschlussliste teilen.

**L9 — Zwei Prüfhelfer und eine Konstante tragen den absoluten Pfad `/Users/k1/Projects/productive/krk/…` (Low).**
`shared/issues/260826-1453_o_*`

`beglaubigung.rs:493-495`, `veroeffentlichung.rs:689-691` (`buendel()`), `beglaubigung.rs:429` (`AUSGELIEFERT`, dort als aufgezeichnete Ausgabe vertretbar). Die zwei Helfer liefern nur Meldungstext, die Proben laufen auf jedem Gerät; aber ein zweiter Mac ist ausdrücklich Zielgerät, und `bundle::wurzel()` liegt daneben. Kosmetik.

**Also seen — Zeilenzitate im Quelltext.** `veroeffentlichung.rs:374-375` zitiert `beglaubigung.rs:344`, `:369`, `:379`; alle drei stimmen am `c13bf1c` noch. Es ist dieselbe Gattung, die `shared/issues/260823-1439_o_*` als ins Leere zeigend führt; Also-seen-Zeile dort angehängt, weil die nächste Zeile in `beglaubigung.rs` sie kippt.

## Cross-cutting observations

- **Sätze über den Wiederholungslauf sind der wiederkehrende Fehlerort.** M1, M2, der Also-seen an `260821-2105` und L1 sind ein Muster: die Kette weiß an jeder Station, was schon geschehen ist, aber die Meldung und die Prosa sagen dem Nutzer nicht den billigsten nächsten Befehl. Drei Wege existieren (`release.sh`, `certify-only.sh`, `veroeffentlichen`), und die Meldungen zeigen von sieben Abbruchstellen nur an zweien auf den richtigen (`veroeffentlichung.rs:363`, `beglaubigung.rs:335-336`).
- **Die Aufzählung des Ordnerinhalts ist an drei Stellen unvollständig, und `CLAUDE.md` weiß es von sich selbst:** M3 (Releasetext), `CLAUDE.md` (eigene Zeile), und die Probe, die die Lücke festschreibt. Die Regel, die `CLAUDE.md` für Zahlen gezogen hat — streichen statt korrigieren —, gilt hier für eine Liste.
- **Das Werkzeug verbietet sich, was die README dem Nutzer empfiehlt** (M4, `--tags`). Die Aufsicht in `git.rs` ist die stärkste Sicherung dieser Kiste und hat keinen Arm in die Dokumentation.

## Recommended sequencing

1. **Vor der nächsten Auslieferung:** M3 (der Releasetext geht mit dem nächsten `gh release create` öffentlich) und M4 (bevor jemand den Handgriff aus `README.md:380` fährt).
2. **Vor dem nächsten Scheitern an Station 7 oder 8:** M2 und der Also-seen an `260821-2105`; M1 als Begleitung derselben Textstellen.
3. **Aufräumen:** L1 (Entscheidung), L2, L3, L4, L5, L6, L7, L8, L9.

## Berührte offene Datensätze, nicht angefasst außer Also-seen

- `shared/issues/260826-1302_o_eine-vierte-pruefordner-fassung-steht-in-xtask-…` — Also seen.
- `circles/260821-1644-veroeffentlichen-als-achte-station/issues/260821-2105_o_ein-angemeldetes-gh-…` — Also seen.
- `shared/issues/260823-1439_o_drei-zeilenzitate-im-quelltext-…` — Also seen.
- `shared/issues/260813-0026_o_bundle-und-release-schreiben-an-denselben-ort-…` — deckt `bundle.rs:219-222`; nicht wiederholt.
- `shared/decisions/260821-1221_o_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-…` und `260821-1115_o_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-…` — offen, hier nicht entschieden; der Bestand steht oben unter „Fremde Werkzeuge".

**Verification:** 13 Dateien vollständig gelesen (`xtask/src/*.rs` per `Read`, die vier Hüllen und `README.md:201-383` per `cat`/`sed`), 7.524 Zeilen per `wc -l`; jede Zeilenangabe am Baum `c13bf1c` abgelesen und gegen die nummerierte Ausgabe ein zweites Mal gelesen; die drei Zeilenzitate in `veroeffentlichung.rs:374-375` gegen `beglaubigung.rs` nachgeschlagen und als zutreffend befunden; `Command::new`-Stellen mit `grep -n` erhoben, `must_use` mit `grep -c` gezählt, `Datei::ALLE` in `pfade.rs:226-234` gelesen; die Tag-Bestände mit `git tag -l` und `git ls-remote --tags origin` verglichen (18 und 18). Nicht übersetzt, nicht getestet, kein `cargo xtask`, kein `git tag`, kein `git push`, keine Datei im Quellbaum geändert. Die Aussage zum stehengebliebenen Ticket ist Inferenz am Code und nicht gemessen.
