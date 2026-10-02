# Durchsicht: Artefakt und Release

**Datum:** 2026-08-21
**Agent:** coderev
**Reviewed-range:** `7db749e..465330b`
**Not-opened:** none
**Spec:** `shared/planning/260821-1115_o_spec-artefakt-und-release.md`, 40 Abnahmekriterien
**Plan:** `shared/planning/260821-1221_c_plan-artefakt-und-release.md`, elf Schritte

## Zusammenfassung

Der Weg ist sauber gebaut: die reinen Hälften tragen die Last, die Fallunterscheidung
`Tagfrage` ist vollständig und ohne Auffangzweig, die Ticketprüfung ist eng und weit genug,
und die Existenzfrage steht getrennt vom Anlegen, wie der Plan es verlangt. Ich habe
`cargo test -p xtask` selbst gefahren: 134 Proben, alle grün.

Neun Befunde, keiner ein Auslieferungshindernis. Der schwerste ist nicht der neue Code,
sondern die Aufsicht darüber: sie zählt drei Bauer namentlich auf, statt sie strukturell zu
binden, und ein vierter käme unbemerkt hinzu. Daneben stehen zwei Zahlenkorrekturen an der
Zuordnung des Plans, drei Stellen, an denen Prosa und Rumpf auseinanderliegen, und eine
Entwurfsfrage zur Reihenfolge der `gh`-Prüfung innerhalb von `release`.

## Zählung

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 1 |
| Mittel | 5 |
| Niedrig | 3 |

## Zu den 27 und den 13

**Die 13 stimmt als Zeilenzahl der Tabelle „Abnahme durch den Nutzer", aber nicht als Zahl
der Kriterien, die den Nutzer verlangen.** Die Zuordnungstabelle des Plans nennt zwei
weitere, die in jener Tabelle fehlen.

- **C1.4** trägt in der Zuordnung (`plan:485`) den Vermerk „Quelltextprobe, **dazu Nutzer**",
  steht aber nicht in der Tabelle der dreizehn. Zu Recht braucht es den Nutzer: die
  Quelltextprobe `die_achte_station_steht_hinter_der_beglaubigung` (`release.rs:1138`) liest
  die Reihenfolge des **Textes** und nicht den Ablauf, und ihr eigener Prüfkommentar sagt das.
  Dass `release` die achte Station wirklich fährt, ist damit nicht abgenommen.
- **C1.5** ist der zweite Fall der Defektklasse, die die Runde schon einmal getroffen hat
  (C1.6 in Schritt 11). Das Kriterium nennt sein Mittel selbst: „Prüfbar an den Änderungszeiten
  des Bündelinhalts vor und nach dem Lauf" (`spec:C1.5`). Abgenommen ist es mit
  `dieser_weg_baut_nichts` (`veroeffentlichung.rs:974`), einer Quelltextprobe über fünf Nadeln.
  Die beiden beantworten verschiedene Fragen: die Nadelprobe sagt, dass das Modul die
  Bauaufrufe nicht **nennt**, nicht, dass nach einem Lauf nichts neu entstanden ist. Sie sieht
  auch nicht, dass `zip_packen` sehr wohl nach `target/` schreibt.

**Verbindlicher Vorschlag:** C1.5 bekommt in der Zuordnung „Quelltextprobe, dazu Nutzer", und
C1.4 und C1.5 kommen in die Tabelle der Nutzerabnahme. Die Zahlen lauten dann **25 an Proben
und Lesen abgenommen, 15 warten auf den Nutzer**.

**Und die 27 verdeckt eine dritte Gruppe.** Vier der 27 (C5.4, C5.5, C6.4, C6.5) tragen in der
Zuordnung nicht „Probe", sondern „Lesen". Ich habe alle vier am Baum nachgelesen und alle vier
halten: die Voraussetzungstabelle führt `gh` (`README.md:18`), der einmalige Handgriff steht
mit seinem Zählkommando (`README.md:480-497`), die achte Station steht in derselben Form wie
die sieben davor (`README.md:342-359`), und der eigenständige Weg steht mit vollem Aufruf
(`README.md:430-478`). Es ist trotzdem eine eigene Art von Abnahme und keine Probe. Der ehrliche
Schnitt ist vierteilig: 21 an Proben, 4 am Lesen, 9 allein beim Nutzer, 6 halb an einer Probe
und halb beim Nutzer.

## Befunde nach Thema

### A. Die Aufsicht über die schreibenden Git-Kommandos

#### A1 — Die Aufsicht zählt auf, statt zu binden: ein vierter Bauer käme unbemerkt hinzu (Hoch)

`die_schreibenden_kommandos_tragen_keine_gewalt` (`version.rs:809-848`) nennt seine drei
Gegenstände namentlich: `tag_argumente`, `eintrag_argumente`,
`veroeffentlichung::schiebe_argumente`. Nichts im Baum verbindet diese Liste mit der Menge der
Listen, die tatsächlich bei `git::rufen` ankommen.

`git::rufen` (`git.rs:79`) hat acht Aufrufstellen: `release.rs:236,252,253`,
`version.rs:102,116,122,123,202,217`, `veroeffentlichung.rs:400,463`. Die Probe, die die Enge
halten soll, ist `xtask_ruft_git_an_genau_einer_stelle` (`release.rs:1052`) — sie zählt die
Vorkommen von `Command::new("/usr/bin/git")` und bleibt bei eins, gleich wie viele Rufer
`git::rufen` bekommt. Eine neue reine Funktion, die einen `Vec<&str>` baut und ihn an
`git::rufen` reicht, übersetzt, besteht alle 134 Proben und wird von niemandem nachgesehen.

**Das ist keine Spekulation, der Fall liegt schon vor.** `version::tagliste_argumente`
(`version.rs:525`) ist heute ein vierter Bauer, der eine gebaute Liste an `git::rufen` reicht
(`version.rs:123`), und die Aufsicht liest ihn nicht. Er ist gedeckt, aber von einer **zweiten,
eigenen** Probe (`die_tagliste_fragt_nur`, `version.rs:774`) — also davon, dass jemand daran
gedacht hat, und nicht von einem Mechanismus.

Der Modulkopf von `git.rs:23-26` sagt es zudem stärker, als es zutrifft: „sie liest die Listen,
die hier ankommen, und keine anderen". Sie liest drei der vier, die hier ankommen.

CLAUDE.md führt genau diese Unterscheidung unter „Was man nicht sieht": es gibt Stellen, die
der Übersetzer hält, Stellen, die eine Probe hält, und Stellen, die nichts hält. Diese hier ist
die dritte Art, und sie ist die sicherheitsrelevanteste des Baums, weil `push` seit dieser
Runde dabei ist.

**Zwei Abhilfen, die eine ist die bessere.**

1. *Der Übersetzer hält es.* `git::rufen` bekommt einen zweiten Zugang `git::schreiben(wurzel,
   Schreibkommando)`, dessen Argument eine vollständige Aufzählung ist. Eine vierte Variante
   hält den Bau an, bis sie in der Aufsicht steht — dieselbe Bauart, die dieses Projekt für
   `Wirkungsbereich`, `Bereich` und `Fokus` schon führt.
2. *Eine Probe hält es.* Eine Quelltextprobe zählt die Aufrufstellen von `git::rufen(` über
   `xtask/src/*.rs` und hält die Zahl fest, so wie `xtask_ruft_git_an_genau_einer_stelle` es
   für den Prozessaufruf tut. Billiger, aber sie fängt den neunten Rufer erst, wenn jemand die
   Zahl anhebt, statt ihn einzuordnen.

#### A2 — Drei Marken fehlen in der Liste, und eine davon ist kurz (Mittel)

`MARKEN` (`version.rs:833-843`) trägt neun Wörter. Die Prüfung ist Element**gleichheit**
(`kommando[1..].contains(&marke)`), kein Präfixvergleich. Daraus folgen drei Lücken:

- **`-d`.** Es ist die kurze Form von `--delete` bei `git push` **und** bei `git tag`. Die
  lange Form steht in der Liste, die kurze nicht. `git push -d origin refs/tags/v0.5.6` löscht
  den Tag auf der Gegenseite und käme durch. Dieselbe Lücke gilt für `-f`/`--force`, dort ist
  sie geschlossen — die kurze Form steht da. Für `-d` fehlt sie.
- **`--force-with-lease` und `--force-if-includes`.** Weil verglichen und nicht auf Präfix
  geprüft wird, deckt `--force` sie nicht ab. Beide sind Formen von Gewalt.
- **`--prune`.** `git push --prune` entfernt auf der Gegenseite jede Referenz, die die
  Argumentliste nicht nennt. Das ist die weitreichendste der fehlenden Marken.

Dazu eine Lücke, die keine Marke ist: ein Refspec mit `+` davor (`+HEAD`, `+refs/tags/…`)
erzwingt, ohne dass eine Marke dastünde. Eine Prüfung, die jedes Wort ab Index 1 gegen ein
führendes `+` hält, schlösse sie.

**Was heute nicht bedroht ist.** Die Bauer sind `vec![…]`-Literale mit genau einem
eingesetzten Wort, und dieses Wort geht durch `versionszahl_pruefen` (`version.rs:418-440`),
das nur drei Gruppen von ASCII-Ziffern durchlässt. Es entsteht daraus immer `refs/tags/v<zahl>`
und nie eine Marke. Die Lücke liegt also nicht in der Gegenwart, sondern in dem, wofür die
Probe gebaut ist: die Änderung von morgen zu fangen.

#### A3 — `add` ist zu Recht herausgefallen (kein Befund)

Die Prüfung des ersten Worts auf Gleichheit macht `add` an Position 0 unerreichbar. An jeder
späteren Position wäre `add` ein Pfad- oder Namensargument und keine Marke; es dort zu
verbieten, verböte einen Dateinamen. Die Begründung im Prüfkommentar (`version.rs:803-806`)
trägt.

**Die Teilung selbst ist trennscharf und vollständig**, und das ist die richtige Antwort auf
die Lage, dass ein Kommando `push` **ist**, während die zwei anderen es nicht tragen dürfen.
Eine Ausnahmeliste wäre die schlechtere Fassung gewesen.

### B. Was ein abgebrochener Lauf hinterlässt

#### B1 — Die Reihenfolgeprobe hält vier der sechs Schritte (Mittel)

`die_voraussetzungspruefung_steht_vor_dem_ersten_wirken` (`veroeffentlichung.rs:739-765`)
vergleicht vier Stellen: `gh_pruefen` < `zip_packen` < `schieben` < `releaseseite_anlegen`.

Der Rumpf hat sechs Schritte, und die zwei, die die Probe nicht liest, liegen beide in der
prüfenden Hälfte: die Tagfrage (`:126-129`) und die Ticketprüfung (`:136`). Wer
`ticket_pruefen` hinter `zip_packen` zöge, ließe jede der 134 Proben grün und bräche genau die
Zusage, die der Doc-Kommentar bei `:103-105` ausschreibt: „die drei Prüfungen stehen vorn, weil
ein Abbruch an ihnen nichts hinterlässt."

**Abhilfe:** zwei weitere `find` in dieselbe Probe, `tagstand_fragen` und `ticket_pruefen`,
und die Kette auf sechs Glieder verlängern. Drei Zeilen.

#### B2 — Die Zwischenzustände, geprüft (kein Befund, mit einer Einschränkung)

Ich bin die vier Abbruchstellen der wirkenden Hälfte durchgegangen:

```
gh fehlt      →  nichts gepackt, nichts geschoben, kein Release      sauber
Tag fehlt     →  dito                                               sauber
Ticket fehlt  →  dito                                               sauber
ditto scheitert → evtl. ein halbes Zip in target/, nichts geschoben  unkritisch (Bauergebnis)
push scheitert  → Zip liegt, Gegenseite unverändert                  sauber
create scheitert→ Zip liegt, Gegenseite trägt HEAD und Tag,          benannt (siehe unten)
                  keine Releaseseite
```

Der letzte ist der einzige, der etwas Bleibendes hinterlässt, und die Meldung
(`veroeffentlichung.rs:592-601`) benennt ihn genau: geschoben ist geschoben, das Zip liegt,
derselbe Aufruf noch einmal holt die Seite nach. **Ich habe nachgerechnet, dass die Zusage
trägt:** ein zweiter Lauf findet den Tag weiter auf HEAD, packt neu, `git push` derselben zwei
Referenzen endet mit „Everything up-to-date" und Rückgabewert 0, `release_steht` verneint noch,
und das Anlegen läuft. Die Wiederaufnahme ist also wirklich wiederholbar und nicht bloß
behauptet.

Einschränkung: der Fall selbst ist ungemessen, weil `gh` auf diesem Gerät fehlt. Ich lese
Quelltext, nicht Läufe.

#### B3 — Zwei Meldungen behaupten einen Zustand, der dort nicht mehr gilt (Mittel)

`gh_fehlt_meldung` endet mit dem Satz „Es ist nichts gepackt und nichts veroeffentlicht."
(`veroeffentlichung.rs:197`). Sie wird an drei Stellen verwendet:

- `gh_pruefen` (`:167`, `:172`) — dort stimmt der Satz.
- `releaseseite_anlegen` (`:590`) — dort ist das Zip gepackt und geschoben ist geschoben.
- `release_steht` (`:628`) — dasselbe.

An den letzten zwei ist der Satz falsch. Der Fall ist selten (`gh` verschwindet zwischen der
Vorprüfung und dem Anlegen), aber die Meldung sagt dann genau das Gegenteil dessen, was der
Nutzer aufräumen muss.

**Abhilfe:** den Schlusssatz aus `gh_fehlt_meldung` herausnehmen und ihn an der einen Stelle
anhängen, an der er gilt, in `gh_pruefen`. Dann trägt die geteilte Meldung nur, was überall
stimmt.

#### B4 — `gh` wird auf dem `release`-Weg erst nach der Beglaubigung geprüft (Mittel, Entwurfsfrage)

`gh_pruefen` steht am Kopf der achten Station und damit auf dem Weg über
`cargo xtask release` hinter Station 1 bis 7: hinter dem Übersetzen beider Ziele, `lipo`, der
Montage, der Signierung mit gehärteter Laufzeitumgebung und einem abgeschlossenen Netzlauf zu
Apple.

C5.1 ist damit wörtlich erfüllt („bevor er packt und bevor er schiebt"). Die Begründung, die
der Spec unter C5 gibt, ist es nicht: „Eine fehlende Voraussetzung soll auffallen, solange noch
nichts geschehen ist." Auf dem `release`-Weg ist zu diesem Zeitpunkt eine Einreichung bei Apple
geschehen.

Der Baum sagt es ehrlich (`release.rs:74-79`, `README.md:352-359`), und die Wiederaufnahme über
`cargo xtask veroeffentlichen <zahl>` existiert. Es ist deshalb kein gebrochenes Versprechen,
sondern eine Frage an den Entwurf: **`gh_pruefen` in Station 1 zu ziehen kostet nichts** und
verletzt keine der Randbedingungen des Specs — Station 1 fährt allein `release`, `bundle` bekäme
keine neue Vorbedingung, `make check` bekäme keine Abhängigkeit von `gh`, und die Probe
`allein_release_fragt_nach_tag_und_arbeitsbaum` bliebe unberührt. Die achte Station behielte
ihre eigene Prüfung für den eigenständigen Weg.

Ich lege das als Frage vor und nicht als Auflage: die Reihenfolge steht so im abgenommenen
Spec, und sie zu ändern ist eine Sache des Nutzers.

### C. Die Ticketprüfung

#### C1 — Eng und weit genug, an allen vier Rändern geprüft (kein Befund)

`traegt_angeheftetes_ticket` (`veroeffentlichung.rs:295`) ist `inhalt.starts_with(b"s8ch")`.
Ich bin die vier Fälle durchgegangen, nach denen gefragt war:

| Lage | Was geschieht | Wo |
|---|---|---|
| Datei fehlt | `fs::read` scheitert, Abbruch mit der Meldung, die den Pfad und `./certify-only.sh` nennt | `:250,257` |
| Datei unter vier Bytes | `starts_with` ist falsch, derselbe Abbruch | `:296`, Probe `:842` |
| Bündel steht gar nicht da | schon vorher abgefangen, mit eigener Meldung | `:132-134` |
| ungeheftet, aber vorhanden | die XML-Eigenschaftsliste beginnt mit `<?xml`, wird abgewiesen | Probe `:831` |

`starts_with` auf `&[u8]` ist bei einem zu kurzen Puffer falsch und nicht panisch, und die
Probe `ein_zu_kurzer_puffer_traegt_die_kennung_nicht` (`:842`) nimmt beide Richtungen ab: `s8c`
zu kurz, `s8cH` gleich lang und falsch. Groß- und Kleinschreibung fallen nicht zusammen.
Ein Vorkommen der Kennung an späterer Stelle reicht nicht (`:834`).

**Die Wahl des Pfades ist die richtige.** `TICKETDATEI` ist `Contents/CodeResources` und nicht
der bloße Dateiname; die gleichnamige Datei unter `_CodeSignature/` ist eine
Eigenschaftsliste, und ein Vergleich am Namen träfe die falsche. Das steht im Doc-Kommentar
(`:278-281`) und ist gemessen belegt.

Die Fehlrichtung ist die sichere und ist als solche benannt (`:290-293`): ändert Apple die
Kennung, bricht der Lauf ab, statt ein ungeheftetes Bündel zu veröffentlichen.

### D. Der eigenständige Weg gegen die achte Station

#### D1 — Die Fallunterscheidung stimmt, und der eigenständige Weg überspringt nichts (kein Befund)

`Tagfrage` (`:79-85`) ist eine vollständige Aufzählung ohne Auffangzweig, und die
Fallunterscheidung im Rumpf (`:126-129`) hat keinen `_`-Zweig. Ein dritter Rufer hielte damit
den Bau an, statt sich stillschweigend für eine Seite zu entscheiden. Das ist genau die Bauart,
die CLAUDE.md unter „Etliche Fallunterscheidungen sind vollständig" beschreibt, und sie ist
hier richtig angewandt.

Die vier Unterschiede zwischen den Wegen habe ich einzeln geprüft:

- **Argumentzahl und Versionszahl:** nur der eigenständige Weg hat sie, `ausfuehren`
  (`:89-99`) prüft beides. `release` nimmt kein Argument und reicht `env!("CARGO_PKG_VERSION")`
  durch (`release.rs:216-219`).
- **Tag auf HEAD:** der eigenständige Weg fragt selbst (`:127`), mit `git::TAGS_AUF_HEAD` und
  `git::tag_steht` und nicht mit `auslieferungsstand_pruefen`. Der Name jener Funktion steht in
  diesem Modul nirgends ausgeschrieben, also bleibt `allein_release_fragt_nach_tag_und_arbeitsbaum`
  grün — ich habe die Probe laufen sehen.
- **Arbeitsbaum:** prüft keiner der zwei Wege in Station 8. Beim `release`-Weg tut es Station 1,
  beim eigenständigen niemand, und der Modulkopf sagt das ausdrücklich (`:29-35`). Dieselbe
  Grenze wie beim Nur-Beglaubigungsweg, konsistent gezogen.
- **Bündel und Ticket:** fragen beide Wege, in derselben Reihenfolge. Richtig so: die achte
  Station innerhalb von `release` läuft zwar direkt hinter der Beglaubigung, aber die Prüfung
  ist zugleich die Milderung des offenen Defekts `260813-0026`, und ein Überspringen nähme sie
  weg.

**Eine Bemerkung ohne Befund:** auf dem `release`-Weg geht `env!("CARGO_PKG_VERSION")` nicht
durch `versionszahl_pruefen`. Der Wert kommt aus der `Cargo.toml`, die `cargo xtask version`
schreibt, und dort ist er geprüft. Wer die Datei von Hand verstellt, bekäme daraus höchstens
`refs/tags/v<unsinn>` — nie eine Marke, weil das Wort immer mit `refs/` beginnt. Kein Weg für
eine Einschleusung.

### E. Die Releaseseite

#### E1 — Die Existenzfrage steht getrennt und ist sauber von „`gh` antwortet nicht" geschieden (kein Befund)

`release_steht` (`:621-630`) ruft `gh release view <tag>` und misst allein den Rückgabewert,
nicht den Wortlaut. Der Plan hat das ausdrücklich verlangt, und so ist es gebaut. Die Probe
`die_existenzfrage_steht_vor_dem_anlegen` (`:947`) hält die Reihenfolge.

Die Trennung der zwei Fälle ist die Stelle, an der ich am genauesten hingesehen habe:

```
gh startet nicht        →  Err(Abbruch), der Lauf endet          (:628)
gh endet mit 0          →  true:  das Release steht              (:629)
gh endet ungleich 0     →  false: es steht nicht — ODER es ist
                           von hier aus gerade nicht zu erfragen
```

Der dritte Zweig fasst zwei Sachlagen zusammen, und der Doc-Kommentar (`:616-620`) sagt das
selbst. Die Zusammenfassung ist zulässig, weil das Anlegen unmittelbar danach die Frage
entscheidet: `gh release create` weist ein bestehendes Release ab, überschreibt es nicht, und
die Meldung des Anlegens fängt den Fall. Die Vorfrage ist also keine Entscheidung, sondern eine
Verkürzung des Wegs zur besseren Meldung. Das ist ein sauberer Schnitt und keine Näherung an
eine unentscheidbare Frage.

#### E2 — Der Text trägt alle sieben Aussagen des Specs (kein Befund)

Ich habe `RELEASETEXT` (`:518-542`) gegen C4.4 bis C4.9 einzeln gelesen: Versionszahl,
macOS 15, Beglaubigung samt Folge „ohne Rückfrage", die drei Installationszeilen, die benannte
Folge des Löschens mit dem Ordner und seinen vier Inhalten, und die Absicherung. Alle da. Die
Probe `der_releasetext_traegt_jede_seiner_aussagen` (`:880`) prüft dreizehn Nadeln einzeln mit
je eigener Behauptung, und sie prüft zusätzlich, dass keine Fügestelle ungefüllt blieb
(`:911-914`) — der Fall, den `str::replace` still durchließe.

Dass der Text Umlaute trägt und der Rest des Moduls nicht, ist begründet (`:509-513`) und
richtig entschieden: es ist der einzige Text dieses Moduls, den ein Fremder liest.

### F. Prosa gegen Code

#### F1 — `make release` beschreibt sich seit dieser Runde falsch (Mittel)

`Makefile:130` trägt die Zeile, die `make help` ausgibt:

```
release: ## Universelles Buendel bauen, mit Developer-ID signieren, beglaubigen
```

Der Befehl tut seit dieser Runde mehr: er packt, **schiebt HEAD und einen Tag zu `origin`** und
legt eine öffentliche Releaseseite an. Das Schieben ist die einzige Wirkung dieser Runde, die
über das Gerät hinausgeht und nicht zurückzunehmen ist, und die Zeile, die der Nutzer vor dem
Tippen liest, nennt sie nicht.

Der Hilfetext von `xtask` nennt sie auch nur mittelbar: `main.rs:82-84` sagt „veroeffentlicht
es als GitHub-Release", ohne das Schieben zu benennen. Der Abschnitt zu `veroeffentlichen`
(`main.rs:127-148`) sagt es, aber wer `cargo xtask release` sucht, liest den nicht.

Schritt 9 des Plans hatte `Makefile` gar nicht im Umfang, und die Zählprobe fängt die Stelle
nicht, weil dort nie „sieben Stationen" stand. Der Umfang war also zu eng geschnitten, nicht
der Schritt schlecht ausgeführt.

**Abhilfe:** die `##`-Zeile in `Makefile:130` um „veröffentlichen" ergänzen, und den
`release`-Absatz der `HILFE` (`main.rs:75-84`) um das Schieben. Zwei Zeilen.

#### F2 — Ein Doc-Kommentar steht seit dieser Runde bei der falschen Funktion (Niedrig)

`release.rs:1159-1160` trägt die zwei Zeilen

```
/// Alle `.rs`-Dateien des Baums, ohne `target/` und ohne das
/// Git-Verzeichnis.
```

Sie gehören zu `rust_dateien` (`release.rs:1201`). Die neue Probe
`der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr` (`:1182`) ist zwischen den Kommentar und
seine Funktion gesetzt worden. Der Kommentar hängt jetzt als erste Zeile am Doc-Block der
Probe, und `rust_dateien` hat keinen mehr. Es übersetzt, weil ein Doc-Kommentar an das nächste
Element bindet; `clippy` und `fmt` sagen nichts dazu.

**Abhilfe:** die zwei Zeilen vor `fn rust_dateien` zurückschieben.

#### F3 — Die Köpfe von `veroeffentlichung.rs`, `release.rs`, `git.rs` und `main.rs`, Satz für Satz (ein Befund, sonst deckungsgleich)

Ich habe die vier Modulköpfe gegen den Rumpf gelesen.

**`veroeffentlichung.rs:1-45` deckt sich.** Die vier Aussagen des Plans stehen da und stimmen:
wozu der Weg da ist; dass er nichts baut (Probe `dieser_weg_baut_nichts`); dass er nichts
einreicht (Probe `dieser_weg_reicht_nichts_ein`); dass er den Arbeitsbaum nicht prüft. Der
Absatz zum Suchpfad (`:37-45`) ist begründet und nennt den Datensatz. Der Satz „Beide Rufer
teilen einen Rumpf, und ihr einziger Unterschied steht als `Tagfrage` da" ist am Rumpf
nachgeprüft und stimmt.

**`release.rs:69-79` deckt sich**, mit einer Ungenauigkeit: „fehlt sie, bricht allein diese
Station ab, und das beglaubigte Bündel bleibt liegen" — richtig, aber der Kopf verschweigt, was
die Station im Erfolgsfall an `origin` schreibt. Siehe F1.

**`git.rs:21-26` sagt eine Aussage zu stark.** Siehe A1: „sie liest die Listen, die hier
ankommen, und keine anderen" trifft auf drei der vier zu.

**`main.rs`:** der Abschnitt zu `veroeffentlichen` (`:127-148`) deckt sich mit dem Rumpf; der
Abschnitt zu `bundle` (`:43-51`) schließt den Defekt `260815-1436` und ist an der Probe
`der_abschnitt_zu_bundle_nennt_die_weitergabe` gehalten. Die Hilfsfunktion `hilfeabschnitt`
(`:260-283`) schneidet richtig — ich habe sie gegen alle drei Kopfzeilen durchgerechnet, und
`HILFE.find` trifft in allen drei Fällen die Kopfzeile und nicht eine frühere Erwähnung. Sie
ist an dieser Eigenschaft aber nicht gehalten: nennt später einmal ein Abschnitt eine Kopfzeile
eines *späteren* Abschnitts in seiner Prosa, greift `find` die falsche Stelle. Niedrig,
Beobachtung, kein Befund.

### G. Die Zählprobe

#### G1 — Sie zählt sich nicht mit, liest was sie behauptet, und lässt die Werkbank draußen (kein Befund)

`der_quellbaum_nennt_die_alte_stationszahl_nicht_mehr` (`release.rs:1182-1199`), drei Fragen
einzeln nachgeprüft:

- **Selbstbezug:** die Nadel steht als `concat!("sieben ", "Stationen")` (`:1183`). Die
  Meldung lautet „die alte Zahl steht noch in {stellen:?}" und schreibt sie nicht aus. Der
  Prüfkommentar umschreibt sie („die Wendung aus der Zahl vor der achten Station und dem Wort
  `Stationen`", „an sieben **Stellen**"). Ich habe den ganzen Quellbaum unabhängig durchsucht:
  kein einziges Vorkommen. Die Probe ist grün gelaufen und nicht bloß grün behauptet.
- **Umfang:** sie liest `README.md`, `Makefile` und `rust_dateien(wurzel/xtask)`. Das sind die
  drei Orte, die der Plan nennt, und dort standen alle sieben Stellen (`README.md` drei,
  `version.rs` zwei, `main.rs` eine, `release.rs` eine). Alle sieben sind nachgezogen; „acht
  Stationen" steht heute an vier Stellen der `README.md` und an vier im Quellbaum.
- **Werkbank:** sie liest `fusion-workbench/` nie, weil sie nur diese drei Wege aufzählt.
  Die Begrenzung ist im Prüfkommentar begründet und mit dem gefilten Defekt
  `260821-1221_o_das-abnahmekriterium-c6-3-…` belegt. Das ist die richtige Behandlung eines
  Kriteriums, das die Zeichenfolge selbst trägt.

**Was sie nicht liest:** `release.sh`, `certify-only.sh`, `crates/`, `idea.txt`. Ich habe alle
vier von Hand durchsucht, keine trägt die Wendung. `certify-only.sh:22` und `Makefile:133,136`
sprechen von „Station 1" und „der siebten Station" — beides bleibt richtig, weil es die
einzelne Station meint und nicht ihre Zahl.

### H. Die `README.md`

#### H1 — Die fünf Stellen halten, mit zwei Anmerkungen (Niedrig)

Gegen den gebauten Stand gelesen:

- **Voraussetzungstabelle** (`:18`): `gh` steht als dritte äußere Voraussetzung, mit Zweck
  („nur für die Auslieferung, nicht für den Bau") und Herkunft (`brew install gh`, danach
  `gh auth login`). C5.4 hält. *Anmerkung:* die Zweckangabe sagt, wofür `gh` **nicht** gebraucht
  wird; „für die öffentliche Releaseseite" stünde näher an dem, was das Kriterium verlangt. Der
  Absatz darunter (`:23-30`) sagt es dann ausdrücklich, also ist die Auskunft vollständig, nur
  nicht in der Zeile selbst.
- **Achte Station** (`:342-359`): in derselben Form wie die sieben davor, mit den drei
  Vorprüfungen in der richtigen Reihenfolge und der Begründung, warum es ein Aufruf und nicht
  zwei sind. C6.4 hält. Der Absatz darunter („Sechs der acht Stationen…") ist gegen den Rumpf
  richtig.
- **„Nur veröffentlichen"** (`:430-478`): der vollständige Aufruf mit vollem cargo-Pfad steht
  da, die Tabelle der sechs Schritte stimmt Wort für Wort mit dem Rumpf überein (ich habe alle
  sechs Zeilen verglichen), und der Verzicht auf eine Hülle ist mit dem offenen Datensatz
  belegt. C6.4 hält auch hier.
- **Der einmalige Handgriff** (`:480-497`): `git push origin --tags` steht als Voraussetzung
  des ersten Laufs, und die Zahl der fehlenden Tags steht **nicht** als feste Zahl, sondern als
  das `comm`-Kommando, das sie zählt. C5.5 und C6.5 halten, genau in der Form, die C6.5
  verlangt.
- **„Installieren und aktualisieren"** (`:501-534`): der Anlass der Runde. Der Text ist ohne
  Kenntnis der Untersuchung verständlich — er sagt zuerst, was zu tun ist (drei nummerierte
  Schritte), dann warum nicht gelöscht wird, dann was ein Löschen kostet, dann die Absicherung.
  Die Betriebsregel steht vor ihrer Begründung, und das ist die richtige Reihenfolge für einen
  Text, den jemand im Augenblick des Installierens liest.

  *Anmerkung:* der Absatz verweist auf `fusion-workbench/shared/analyses/260820-2242-…`, und
  ein Leser des ausgelieferten Zips hat diese Datei nicht. Der Verweis schadet nicht, weil der
  Text ohne ihn vollständig ist, aber er ist an dieser Stelle Werkbank-Innenleben in einem
  Abschnitt, der sich ausdrücklich „an den Nutzer des ausgelieferten Bündels und nicht an den,
  der es baut" richtet.

## Querliegende Beobachtungen

**Die Runde hat ihre eigene Defektklasse zweimal getroffen und einmal gefangen.** Der Coder hat
C1.6 in Schritt 11 selbst gefunden und behoben. C1.4 und C1.5 stehen daneben und sind nicht
gefangen worden, und zwar beide aus demselben Grund: eine **Quelltextprobe** wurde für eine
Zusage genommen, die den **Ablauf** betrifft. Der Baum weiß das und schreibt es an beiden
Proben in den Prüfkommentar („sie liest die Reihenfolge des Textes und nicht den Ablauf"). Der
Schritt, der fehlt, ist, diese Einschränkung in die Zuordnungstabelle zu übernehmen: wo eine
Quelltextprobe steht, gehört „dazu Nutzer" daneben, außer die Zusage ist selbst eine über den
Text.

**Zwei Befunde teilen eine Wurzel: eine Zusage wird von einer Aufzählung getragen statt von
einer Struktur.** A1 (drei namentlich genannte Bauer) und B1 (vier von sechs Schritten
verglichen) sind dieselbe Bauart. In beiden Fällen ist die Zusage in Prosa vollständig und in
der Probe unvollständig, und in beiden Fällen fällt die Lücke erst auf, wenn jemand die Stelle
ändert, die sie betrifft. Das ist genau der Zustand, den CLAUDE.md unter „Was der Übersetzer
einfordert, und was er nicht einfordert" beschreibt.

**Der Umfang von Schritt 9 war um eine Datei zu eng.** Die Zählprobe konnte `Makefile:130`
nicht fangen, weil die Stelle nie die gesuchte Zeichenfolge trug. Eine Zählprobe fängt, was
falsch **dasteht**, nie, was fehlt. Für die nächste Runde, die eine Station hinzufügt: die
`##`-Zeilen des Makefiles gehören in den Umfang, weil `make help` sie ausgibt.

## Nachträge zu offenen Datensätzen

Diese Befunde gehen über die zwei offenen Fragen nicht hinaus und sind dort nachzutragen statt
neu zu filen:

- **`shared/decisions/260821-1221_o_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-…`** —
  Der Datensatz nennt unter Contra von Option 1 bereits, dass `PATH` von außen steuerbar ist.
  Nachzutragen ist die zweite Hälfte, die der Baum jetzt zeigt: der Aufruf über den Suchpfad
  steht an **drei** Stellen (`veroeffentlichung.rs:166`, `:579`, `:622`), alle über die
  Konstante `GH` (`:59`). Eine spätere Umstellung auf eine Stufensuche berührt damit genau eine
  Zeile. Das senkt die Kosten von Option 2 gegenüber dem, was der Datensatz annimmt.
- **`shared/decisions/260821-1115_o_bekommt-der-veroeffentlichungsbefehl-eine-eigene-huelle-…`** —
  Nachzutragen: die `README.md` (`:441-448`) trägt die Begründung für Option 1 inzwischen
  ausgeschrieben, samt dem Handgriff `export PATH=…`. Wer Option 2 oder 3 wählt, zieht diese
  vier Zeilen mit.

**Und einer, der gemildert und nicht behoben ist.**
`shared/issues/260813-0026_o_bundle-und-release-schreiben-an-denselben-ort-…` bleibt offen. Ich
habe geprüft, dass kein Schritt und keine Datei dieser Runde etwas anderes behauptet: der Plan
sagt es in der Risikotabelle ausdrücklich, der Spec sagt es unter „Randbedingungen", und der
Quelltext behauptet an keiner Stelle einen Abschluss. Die Ticketprüfung fängt den Fall, dass
ein Entwicklungsbau das beglaubigte Bündel überschrieben hat — sie fängt ihn aber erst am
Anfang der achten Station und nicht beim Überschreiben selbst, und ein Entwicklungsbau
zerstört das beglaubigte Bündel weiterhin unwiderruflich. Das ist eine Milderung. Nachzutragen
ist am Datensatz allein, dass es seit dem 260821 **zwei** Stellen sind, die den Fall abfangen,
`beglaubigen` und `veroeffentlichen`, und dass beide hinter dem Schaden stehen und nicht davor.

## Empfohlene Reihenfolge

**Vor der ersten Auslieferung** (weil beide den Nutzer über eine nicht zurücknehmbare Wirkung
im Unklaren lassen):

1. F1 — `Makefile:130` und der `release`-Absatz der `HILFE` nennen das Schieben. Zwei Zeilen.
2. B3 — der Schlusssatz von `gh_fehlt_meldung` wandert an die eine Stelle, an der er gilt.

**Danach, als Aufräumen:**

3. A1 — die Aufsicht strukturell binden. Der größte der Befunde und der einzige, der einen
   Entwurf berührt.
4. A2 — `-d`, `--force-with-lease`, `--force-if-includes`, `--prune` und das führende `+` in
   die Prüfung.
5. B1 — die Reihenfolgeprobe auf sechs Glieder.
6. Die Zuordnungstabelle des Plans: C1.4 und C1.5 in die Nutzerabnahme, die Zahlen auf 25/15.
7. F2 — den Doc-Kommentar zurückschieben.
8. G1/H1 — die zwei Anmerkungen zur `README.md`, wenn ohnehin jemand die Datei anfasst.

**Nicht als Auflage, sondern als Frage an den Nutzer:** B4, ob `gh_pruefen` in Station 1 zieht.

## Was ich nicht geprüft habe

Alles, was einen Lauf gegen GitHub oder einen zweiten Mac verlangt. `gh` ist auf diesem Gerät
nicht installiert, und ich habe nichts ausgeliefert. Die dreizehn Kriterien der Nutzertabelle
und die zwei, die ich ihr zurechne, sind von dieser Durchsicht unberührt. Ich habe Quelltext
gelesen und `cargo test -p xtask` gefahren, mehr nicht.
