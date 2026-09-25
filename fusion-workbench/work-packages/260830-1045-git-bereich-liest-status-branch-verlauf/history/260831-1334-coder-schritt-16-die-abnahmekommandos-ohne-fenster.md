# Schritt 16: Die Abnahmekommandos ohne Fenster

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Circle:** `260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 16
**Kriterien:** C3.8 (Prüfhälfte), C5.8 (Prüfhälfte), C6.4 (Prüfhälfte), C6.6 (Prüfhälfte), C8.4, C8.7, C8.8, C8.9, C10.3 (Prüfhälfte)
**Stand vor der Runde:** `d1fbaac`; verglichen wurde mit `git show d1fbaac:<pfad>` und mit einem
über `git archive d1fbaac` in einen Wegwerfordner ausgepackten Baum. Kein Kommando dieses
Schritts hat den Arbeitsbaum angefasst.

---

## Verification

```
make check — exit 0
```

---

## Der Bau

| Kommando | Exit | Befund |
|---|---|---|
| `make check` | 0 | „alle vier gruen" |
| `cargo xtask bundle` | 0 | Bündel gebaut, signiert mit „KRK Entwicklung" aus dem Schlüsselbund |

`make check` fährt `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` unter `-D warnings` und `cargo fmt --all --check`
(C8.7). Kein fehlgeschlagener Test im ganzen Arbeitsbereich; `tests/git.rs` meldet
15 bestanden und 1 ausgelassen, und das eine ist die Kindprobe
`kind_liest_unter_abgesenkter_deskriptorgrenze`, die der Elternteil über
`KRK_KINDPROBE_AUFTRAG` startet. `tests/baum.rs` ist grün und damit auch
`genau_drei_pruefordner_fassungen_stehen_im_baum` (C8.6).

Das Bündel ist gebaut und signiert (C8.8); eine Signaturidentität war da, der Abbruchweg
kam nicht zum Tragen. Es ist ein Entwicklungsbündel: keine Developer-ID, keine Beglaubigung,
allein `x86_64`. Genau das sagt `bundle` selbst am Ende seines Laufs, und die
Auslieferungskette ist von dieser Runde unberührt.

## Die Zusage, an der die Bibliothekswahl hing (C8.4)

**Beide Läufe namentlich, beide sauber:**

| Kommando | Exit | `cc` | `-sys` | eindeutige Pakete |
|---|---|---|---|---|
| `cargo tree --target x86_64-apple-darwin -e normal,build` | 0 | keiner | keiner | 197 |
| `cargo tree --target aarch64-apple-darwin -e normal,build` | 0 | keiner | keiner | 197 |

**Die Gegenprobe hält das Muster für `-sys` offen.** `cargo tree --target all -e normal,build`
(exit 0) führt `linux-raw-sys v0.12.1` und `windows-sys v0.61.2`, und dasselbe Muster findet
sie dort. Die Null der zwei Mac-Ziele ist also nicht die Null eines zu engen Musters, sondern
die Aussage, dass die zwei einzigen `-sys`-Pakete des Baums keines der beiden Bauziele
erreichen.

**Für `cc` liefert auch die Gegenprobe nichts**, und das ist keine Lücke im Muster:
`Cargo.lock` führt kein Paket `cc` (`grep -n '^name = "cc"' Cargo.lock`, exit 1), es kann auf
keinem Ziel auftauchen. Das Muster selbst ist an drei gestellten Zeilen geprüft
(`│   ├── cc v1.2.3`, `├── cc v1.0.0`, `cc v9.9.9`) und findet alle drei.

**Die erste Bedingung aus `## Stops when` greift nicht.** Sie greift, wenn einer der zwei
Läufe `cc` oder ein `-sys`-Paket führt; keiner tut es.

## Die sieben Belege

**`grep -rn 'write_changes' crates/` — exit 0, zwei Treffer.** Beide sind Modulköpfe, die
ausschreiben, dass sie den Weg **nicht** rufen: `crates/krk-core/src/git/mod.rs:17` und
`crates/krk-core/src/git/leser.rs:50`. Der Aufruf-Grep `grep -rn 'write_changes(' crates/`
ist leer (exit 1), und er ist der, der trägt. Der Widerspruch zwischen C3.8 („keine
Fundstelle") und C10.3 („Treffer, die die Lesestelle nennen") ist bekannt und gefilt
(`issues/260830-1614_o_…`); der Baum gibt C10.3 recht, und dieser Schritt entscheidet die
Wortlautfrage nicht.

**`grep -rn 'NeedsUpdate' crates/` — exit 0.** Auf `EntryStatus::NeedsUpdate` eingegrenzt
bleiben vier Treffer: zwei Modulköpfe (`git/mod.rs:18`, `git/leser.rs:51`), ein
Doc-Kommentar (`git/leser.rs:370`) und **eine** Codestelle, `git/leser.rs:398`,
`EntryStatus::NeedsUpdate(_) => return None`. Sie liest und verwirft; eine Schreibstelle
steht nicht da. Die übrigen Treffer des rohen Musters sind `menuNeedsUpdate:`, der
AppKit-Selektor, und haben mit der Sache nichts zu tun. Der gefilte Nebenbefund
(`issues/260831-0855_o_…`) steht: der Zweig ist unerreichbar, weil `gix` den Posten in
seinem eigenen Statusiterator abfängt. Stufe A ist schreibfrei — der Zweig belegt es nicht,
der leere Aufruf-Grep belegt es.

**`grep -rn 'eprintln!' crates/krk-ui/src` — exit 0, 18 Zeilen, und dieselben 18 wie vor der
Runde.** Nach Datei und Wortlaut verglichen (Zeilennummern abgestreift), der Diff ist leer.
Keine neue Fundstelle, also keine Meldung auf die Standardfehlerausgabe aus dem Gitweg
(C6.6, Prüfhälfte).

**`grep -rn 'sichtbar_setzen' crates/krk-ui/src` — exit 0, und die Ruferliste trägt keinen
Gitweg** (C6.4, Prüfhälfte). Alle vier Aufrufe stehen in `fenstermodell.rs` selbst
(`:496` in einer Probe, `:626` in `mitbewerber_raeumen`, `:764`, `:774`); die Funktion ist
privat und hat außerhalb ihrer Datei keinen Rufer. Die einzige Nennung im Git-Bereich ist
`appkit/git.rs:41`, ein Modulkopfsatz, der ausdrücklich sagt, dass es **keinen** Aufruf gibt.

**`awk '/pub enum Schluessel/,/^}/'` — vier Werte, wie vor der Runde** (C5.8, Prüfhälfte).
`Name`, `Groesse`, `Geaendert`, `Typ` in `crates/krk-core/src/verzeichnis/sortierung.rs`,
wörtlich gleich zu `d1fbaac`. Kein fünfter Sortierbefehl.

**`grep -oE '"L[0-9]+"' crates/krk-bench/src/messen.rs | sort -u` — exit 0, zehn Kennungen,
dieselbe Menge wie vor der Runde.** `L1` bis `L10`, Diff leer. Keine elfte Zeitzusage, keine
der zehn angefasst.

**`grep -rn '^\s*#\[must_use' crates/*/src` — 169 Stellen, vor der Runde 139** (C8.9). Die
30 neuen verteilen sich so: `krk-core/src/git/lauf.rs` 1, `git/leser.rs` 5, `git/mod.rs` 2,
`git/texte.rs` 4, `verzeichnis/modell.rs` +2, `krk-ui/src/appkit/git.rs` 4,
`fenstermodell.rs` +1, `gitmodell.rs` 9, `tabs.rs` +2.

**`git -C ~/Library/Caches/krk-messplatz rev-parse --show-toplevel` — exit 128, keine
Ausgabe:** „Kein Git-Repository (oder irgendeines der Elternverzeichnisse)". Das Verzeichnis
steht (`ls -d` findet es), und bis zur Wurzel liegt kein `.git` darüber. Die Messstrecke
läuft in keinem Repository, `gix::discover` findet dort nichts, und die zehn Zusagen sehen
von dieser Runde nichts (Entscheidung 8 des Plans).

## Die Durchsicht der neuen Rückgabewerte (C8.9)

Durchgesehen sind alle neuen Funktionen mit Rückgabewert aus dem Diff `d1fbaac..HEAD` über
`crates/`. **Jeder neue Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, trägt die
Marke**, und die Begründungen an den Marken sind ausgeschrieben statt nackt, wo die Antwort
eine Bedingung trägt — `Gitlauf::starten` („ein sofort fallengelassener Lauf bricht sich
selbst ab und meldet nichts"), `Ordnermodell::gitmarken_setzen` („die Antwort sagt, ob der
Befund noch zu diesem Bestand gehoert hat"), `Tabliste::git_gefragt_setzen` und
`verlauf_nachladen` („laeuft jetzt ein Gitlauf, ist der Einzugstakt anzuwerfen"),
`Gitfenster::kommando_ausfuehren` („ein nicht ausgefuehrtes Kommando laeuft weiter").

Ohne Marke bleiben allein die Fälle, in denen der Baum es an der Schwesterstelle ebenso
hält, und jeder ist einzeln nachgesehen:

- `Gitlauf::meldungen(&self) -> &Receiver<Gitmeldung>` — die zwei vorhandenen Fassungen
  desselben Namens (`operation/fortschritt.rs:222`, `verzeichnis/leser.rs:126`) tragen sie
  auch nicht; ein geliehener Kanal ohne Nebenwirkung.
- `Gitfenster::bauen`, `sicht`, `fokusansicht` — `Vorschaufenster` und `Leiste` halten es
  gleich.
- `Tabliste::gitlauf_nachziehen_an` (privat) — `durchlauf_nachziehen_an` daneben ebenso;
  die öffentliche Hülle `git_gefragt_setzen` trägt die Marke.
- `Bereich::flaeche` — die Fassung, die sie ersetzt (`teilt_flaeche_mit`), trug sie in
  `d1fbaac` auch nicht; ebenso `bereichsbreiten` und `Aufteilung::gemessene_breiten`, die
  beide schon vor der Runde ohne Marke standen.

## `make menue`: der Diff trägt genau die erwarteten Abweichungen

Vorher gefahren im ausgepackten Baum von `d1fbaac`, nachher im Arbeitsbaum; beide Läufe
exit 0, verglichen wurden die Zeilen ab `menue=` (92 vorher, 95 jetzt).

```
32a33
> menue="Dateilisting" eintrag="Spalte Marke ein- und ausblenden" kombination=(keines) …
79a81,82
> menue="Git" eintrag="Fokus in den Git-Bereich" kombination=shift+cmd+b …
> menue="Git" eintrag="Git-Bereich ein- und ausblenden" kombination=opt+cmd+r …
```

Drei neue Einträge, keine geänderte und keine gefallene Zeile. Die Obermenüs steigen von
neun auf zehn, und das zehnte ist „Git". Es steht in der Ausgabe an achter Stelle,
unmittelbar hinter „Editor" und vor „Bearbeiten" und „Fenster" — genau die Stelle, die
Entscheidung 8 des Plans für den neuen `Funktionsbereich` verlangt: die drei Bereiche am
rechten Rand des Fensters stehen im Menü in derselben Folge wie in der Fensterzeile.
**Keine weitere Abweichung.**

## `make tasten`: das Prüfmittel gibt es so nicht, und der Befund ist gefilt

`make tasten` ist **nicht** die Tabelle, die Schritt 16 erwartet. Das Ziel baut das Bündel
und startet es mit `--tasten-protokoll`; die Marke schaltet den Protokollmodus des
Ereignisabgriffs ein, und `ereignisse::protokollieren`
(`crates/krk-ui/src/appkit/ereignisse.rs:817`) schreibt je **empfangenem Tastendruck** eine
Zeile `tastencode=… zeichen=… maske=… kombination=… funktion=…`. Keine Spalten, kein
`Wirkungsbereich`, und der Lauf endet erst mit `Cmd+Q` — er verlangt KRK im Vordergrund und
ist damit Nutzerarbeit wie Schritt 17. `docs/tastenbelegung.md`, das Entscheidung 2 des
Plans daneben nennt, gibt es in diesem Baum nicht.

Die Fläche mit der dritten Spalte ist `belegungsausgabe::markdown`, deren `wirkung(funktion)`
(`crates/krk-ui/src/belegungsausgabe.rs:263`) in der ersten Begründungslage
`kommando.wirkungsbereich().beschriftung()` abliest; geschrieben wird sie über den
Menüeintrag „Tastenbelegung als Markdown sichern" in den Ordner „Downloads", also ebenfalls
aus der laufenden Anwendung. Gefilt als
`issues/260831-1334_o_make-tasten-ist-der-interaktive-tastenlogger-und-traegt-keine-dritte-spalte.md`.

**Die erwartete Änderung ist stattdessen an ihren zwei Eingaben belegt**, und die beiden
zusammen legen fest, welche Zeilen der Tabelle sich ändern:

- `Wirkungsbereich::beschriftung` hat genau eine ihrer acht Zeilen geändert:
  `Navigator` von `"Dateifenster, Leiste und Vorschau"` auf
  `"Dateifenster, Leiste, Vorschau und Git-Bereich"`. Die anderen sieben stehen wörtlich wie
  in `d1fbaac`. `Wirkungsbereich` trägt vor und nach der Runde acht Werte.
- Genau drei Kommandos tragen `Wirkungsbereich::Navigator` — `FensterWechseln`,
  `AuswahlHoch`, `AuswahlRunter` —, mit den Kennungen `fenster_wechseln`, `auswahl_hoch`,
  `auswahl_runter`.
- `resources/default-keymap.toml` wächst von 88 auf 91 Funktionen; die drei neuen sind
  `spalte_marke_umschalten` (ohne Kombination), `fokus_git` (`shift+cmd+b`) und
  `git_bereich_umschalten` (`opt+cmd+r`). Keine vorhandene `id`- und keine vorhandene
  `tasten`-Zeile ist angefasst.

Damit ändern sich in der Tabelle genau die drei Navigator-Zeilen in ihrer dritten Spalte,
und es kommen genau drei Zeilen dazu — die erwartete Abweichung, nur an anderer Stelle
nachgewiesen als der Plan es vorsieht.

## Befunde

1. **`make tasten` ist nicht das Prüfmittel, das Schritt 16 beschreibt**, und kein Agent kann
   es fahren. Gefilt: `issues/260831-1334_o_make-tasten-ist-der-interaktive-tastenlogger-…`.
2. **Vier Schwesterfassungen von `kommando_ausfuehren` tragen kein `#[must_use]`**, während
   die neue des Git-Bereichs sie trägt, und drei Aufrufstellen lassen die Antwort heute nackt
   fallen. Pre-existierend, nicht von dieser Runde verursacht, aber von ihr sichtbar gemacht.
   Gefilt: `issues/260831-1334_o_vier-schwesterfassungen-von-kommando-ausfuehren-…`.

Beide Befunde halten Schritt 16 nicht an: der erste betrifft die Beschreibung eines
Prüfmittels und nicht den Baum, der zweite eine Lage, die vor der Runde schon bestand.

## Was dieser Schritt nicht sagt

Er sagt nichts über C7.2 und C7.3, über die Lesbarkeit der Verlaufszeile bei 340 Punkten und
über die Fadenzahl von `gix`. Das ist Schritt 17, und der verlangt KRK im Vordergrund.
