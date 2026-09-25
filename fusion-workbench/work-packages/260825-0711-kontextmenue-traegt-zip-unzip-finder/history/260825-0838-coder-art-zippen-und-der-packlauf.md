# Coder: `Art::Zippen` und der Packlauf, und zwei Vorgaben, die die Kiste anders hält

**Datum:** 2026-08-25 08:38
**Status:** Complete
**Agent:** coder
**Baumstand:** `9af13ba` plus die Änderungen dieses Schritts

## Auftrag

Schritt 2 des Plans `planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md`:
`Art` bekommt den Wert `Zippen { ziel: PathBuf }` mit dem **vollen Pfad des Archivs**,
`Auftrag::zippen` als fünften Erzeuger, `zielordner()` für diesen Wert `None`, eine neue
Verzweigung in `ausfuehren` über die Art — vollständig und ohne Auffangzweig —, das neue
Modul `operation/zippen.rs` mit dem Lauf, und die vier vom Übersetzer genannten Stellen in
`krk-ui`.

## Was entstanden ist

**`crates/krk-core/src/operation/zippen.rs`, neu, 428 Zeilen.** Der Lauf in der Reihenfolge,
die der Plan vorgibt: Zielarchiv einmal klären, `ZipWriter` über `File::create`, je Quelle
absteigen, `finish()`, und nach einem Abbruch oder einem Schreibfehler das halbe Archiv
wegräumen.

`zielarchiv_klaeren` steht vor jedem `File::create` und behandelt alle vier
`Konfliktantwort`-Werte. Es gibt einen `Zielentscheid` zurück, also denselben Wert, den
`ziel_klaeren` für die vier übrigen Arten liefert; eine zweite Aufzählung für dieselbe Frage
entsteht nicht. `Ueberspringen` heißt hier „der Lauf hat sein einziges Ziel verloren" und endet
mit `Abschluss::Fertig` und dem Grund in der Abschlussliste.

`Packschritt` trägt **drei** Werte und nicht zwei. Der dritte, `ArchivHin`, trennt die
gescheiterte Einzelposition, die den Stapel nach C4 nicht aufhält, von einem Schreibfehler am
Archiv selbst, nach dem jeder weitere Eintrag verlorene Arbeit wäre. Ein Lesefehler mitten in
einer Datei nimmt den halben Eintrag über `ZipWriter::abort_file` wieder heraus und lässt die
übrigen Quellen weiterlaufen.

`datei_packen` prüft den Abbruch **innerhalb** der Stückschleife und nicht nur zwischen zwei
Einträgen; die Stückgröße ist `STUECK = 64 KiB`. `ordner_packen` schreibt den Ordnereintrag
**vor** seinem Inhalt, damit ein leerer Ordner nicht verlorengeht, und steigt über
`verzeichnis::lesen` ab, wie `kopieren::ordner` es tut. Gelesen wird über
`verzeichnis::sys::ohne_warten_oeffnen`; eine Probe belegt, dass eine benannte Röhre im Ordner
den Lauf nicht anhält.

**`auftrag.rs`:** der Wert `Zippen { ziel }`, der Erzeuger `Auftrag::zippen`, und `zielordner()`
liefert für ihn `None`. Der Doc-Kommentar von `zielordner` schreibt jetzt für alle drei
`None`-Fälle den Grund aus, damit keiner wie ein vergessener aussieht.

**`mod.rs`:** `ausfuehren` verzweigt über die Art und gibt `Zippen` an `zippen::lauf`, die vier
übrigen an die bestehende Schleife, die als `quelle_fuer_quelle` in eine eigene Funktion
gewandert ist. Die Verzweigung ist vollständig und hat keinen Auffangzweig. `einen_abarbeiten`
bekommt trotzdem einen `Zippen`-Zweig, weil auch dessen Fallunterscheidung vollständig ist; sein
Rumpf meldet über `ueberspringen`, statt stillzuschweigen — er ist heute unerreichbar, und ein
späterer Umbau der Verzweigung soll nicht unbemerkt dort landen.

**Die vier Stellen in `krk-ui`:** Überschrift „Packen" (`kommandos/operationen.rs`), kein
Auffrischungsaufschub (`auffrischung.rs`), kein zusätzlicher Ordner (`Vorgang::ordner` in
`appkit/anwendung.rs` — das Archiv entsteht im angezeigten Ordner, der schon als `quellordner`
dasteht), und keine Auswahl nach dem Abschluss (`vorgang_beenden`, mit dem Grund im Kommentar).

## Zwei Vorgaben des Plans halten in `zip 8.6.0` nicht wie geschrieben

**Erstens: `unix_permissions(0o120777)` legt keine Verknüpfung ab.** Der Plan schreibt diesen
Aufruf wörtlich vor. Der Rumpf der Kiste lautet `self.permissions = Some(mode & 0o777)`
(`zip-8.6.0/src/write.rs:573`), wirft die oberen Modusbits also fort; die Kiste hat dafür sogar
eine eigene Probe, `unix_permissions_bitmask`, die genau mit `0o120777` prüft, dass `0o777`
herauskommt. Ein so abgelegter Eintrag wäre eine **gewöhnliche Datei**, deren Inhalt zufällig wie
ein Pfad aussieht, und jedes Entpackwerkzeug legte sie als solche an. Gesetzt wird `S_IFLNK`
allein von `ZipWriter::add_symlink`, das die Rechte um das Kennzeichen ergänzt.

Genommen ist deshalb `add_symlink`, und die Wahl trägt weiter `unix_permissions(0o120777)`:
`add_symlink` ergänzt eine Vorgabe nur, wenn keine Rechte dastehen, und die 0o777 der Maskierung
sind genau die, die es selbst setzen würde. Damit steht der Aufruf des Plans im Code und tut,
was der Plan mit ihm meinte. Der Modulkopf der Funktion schreibt den Mechanismus aus, damit
niemand ihn beim Aufräumen für überflüssig hält. Die Probe
`eine_verknuepfung_wird_als_verknuepfung_gepackt_und_nicht_ihr_ziel` prüft die oberen Modusbits
und nicht die Rechte.

**Zweitens: jeder Eintrag trägt den 1. Januar 1980.** Ohne das Merkmal `time` liefert
`DateTime::default_for_write()` einen festen Wert (`zip-8.6.0/src/datetime.rs:195`), und die
Merkmalswahl aus Schritt 1 schaltet es nicht ein. Das Änderungsdatum der Quelle kommt damit nicht
mit. Behoben ist es in diesem Schritt **nicht**: die MS-DOS-Zeitform des Formats ist bürgerliche
Ortszeit, die Umrechnung aus `SystemTime` braucht die Zeitzone des Geräts, und keine der
eingebundenen Kisten liefert sie. Abgelegt als Defekt mit drei Vorschlägen:
`issues/260825-0838_o_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md`.
Das Entpacken aus Schritt 3 spiegelt die Frage und gehört in denselben Zug.

## Was der Plan nicht nennt und trotzdem dasteht

**Die Rechte der Quelle wandern ins Archiv** (`rechte_uebernehmen`). Ohne sie wäre ein
ausführbares Skript nach dem Rundweg keines mehr. Lässt sich die Quelle gerade nicht befragen,
steht die übliche Vorgabe da (0o644 für Dateien, 0o755 für Ordner); ein Eintrag ohne Rechte wäre
schlechter als einer mit den üblichen. Die Probe `die_rechte_der_quelle_stehen_im_archiv` hält es.

**`set_auto_large_file()` am Schreiber.** Ohne sie bricht ein Eintrag über 4 GiB mit einem Fehler
ab, statt sich packen zu lassen; mit ihr schaltet die Kiste die ZIP64-Form für genau diesen
Eintrag ein und verschenkt bei den übrigen keine 20 Byte.

## Prüfung

`make check` läuft grün über den ganzen Workspace, Exit-Code 0: `cargo build --workspace`,
`cargo test --workspace`, `cargo fmt --all --check` und
`cargo clippy --workspace --all-targets -- -D warnings`.

Dreizehn neue Proben, zwölf davon in `crates/krk-core/tests/operation.rs` über den
selbstabräumenden Prüfordner aus `tests/gemeinsam/mod.rs` und den öffentlichen Weg
`operation::starten`, eine in `auftrag.rs`:

| Probe | Was sie hält |
|---|---|
| `ein_ordnerbaum_wird_gepackt_und_jeder_eintrag_steht_im_archiv` | Namen, Inhalte, der leere Ordner, die gezählten Bytes |
| `mehrere_quellen_kommen_nebeneinander_in_ein_einziges_archiv` | ein Ziel für den ganzen Lauf |
| `die_rechte_der_quelle_stehen_im_archiv` | 0o755 überlebt |
| `eine_verknuepfung_wird_als_verknuepfung_gepackt_und_nicht_ihr_ziel` | `S_IFLNK`, Inhalt ist das Verweisziel |
| `eine_verknuepfung_auf_den_eigenen_ordner_laesst_den_lauf_enden` | der Abstieg folgt keinem Verweis |
| `ein_belegter_archivname_wird_einmal_und_vor_dem_ersten_byte_erfragt` | genau eine Frage, und das alte Archiv steht danach unverändert da |
| `die_regel_ueberschreiben_ersetzt_ein_vorhandenes_archiv` | Antwort 1 von 4 |
| `die_regel_ueberspringen_laesst_das_vorhandene_archiv_stehen` | Antwort 2 von 4 |
| `die_regel_umbenennen_legt_das_archiv_daneben` | Antwort 3 von 4 |
| (`ein_belegter_archivname_…`, drei Zeilen höher, antwortet „abbrechen") | Antwort 4 von 4 |
| `ein_abbruch_waehrend_des_packens_hinterlaesst_kein_halbes_archiv` | Abbruch innerhalb einer Datei, und die Datei ist danach weg |
| `eine_fehlende_quelle_wird_gemeldet_und_die_uebrigen_werden_gepackt` | C4: eine gescheiterte Position hält den Stapel nicht auf |
| `eine_benannte_roehre_im_ordner_haelt_das_packen_nicht_an` | `ohne_warten_oeffnen` statt `File::open` |
| `ein_packauftrag_hat_keinen_zielordner_sondern_eine_zieldatei` | `zielordner()` liefert `None` |

Die Abbruchprobe braucht kaum verdichtbare Bytes: eine Datei aus lauter gleichen Bytes wäre
gepackt, bevor der Abbruch sie erreicht. Der Hilfsbau `rauschdatei` schreibt sie aus einem
linearen Kongruenzgenerator — die Folge muss unvorhersehbar **aussehen**, nicht es sein —, und
`volle_datei` daneben bleibt unangetastet, weil sie für die Kopierprobe genau das Gegenteil
braucht.

**Ein fremdes Werkzeug öffnet das Archiv.** Über eine wegwerfbare Probe außerhalb des Baums
gepackt und danach wieder entfernt:

```
$ /usr/bin/unzip -t quelle.zip
    testing: quelle/                  OK
    testing: quelle/oben.txt          OK
    testing: quelle/unten/            OK
    testing: quelle/unten/tief.txt    OK
    testing: quelle/verweis.txt       OK
No errors detected in compressed data of quelle.zip.

$ /usr/bin/unzip -q quelle.zip -d aus && ls -l aus/quelle/
lrwxrwxrwx  verweis.txt -> oben.txt
```

Damit ist die erste Hälfte der Endbedingung „ein Archiv, das ein anderes Zip-Werkzeug öffnen
kann" belegt, und zugleich, dass die Verknüpfung als Verknüpfung ankommt. Die zweite Hälfte —
das Archiv im Finder — gehört in den Abnahmelauf des Nutzers.

## Was nicht Gegenstand war

Die Namensbildung des Archivs (`archivname`, Schritt 4) und die Kürzung des Konfliktblatts auf
drei Antworten (Schritt 8). Der Kern behandelt weiter alle vier `Konfliktantwort`-Werte, weil er
sie kennt und die Konfliktregel des Auftrags jede davon liefern kann, ohne dass ein Blatt im
Spiel wäre. `entpacken.rs` und `Art::Entpacken` entstehen in Schritt 3; die Verzweigung in
`ausfuehren` steht dafür bereit, `einen_abarbeiten` bekommt dort seinen fünften Zweig.

Kein Commit. `Cargo.lock` und `Cargo.toml` sind unberührt geblieben.

## Berührte Dateien

- `crates/krk-core/src/operation/zippen.rs` (neu)
- `crates/krk-core/src/operation/auftrag.rs`
- `crates/krk-core/src/operation/mod.rs`
- `crates/krk-core/tests/operation.rs`
- `crates/krk-ui/src/kommandos/operationen.rs`
- `crates/krk-ui/src/auffrischung.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/issues/260825-0838_o_jeder-gepackte-eintrag-traegt-den-1-januar-1980-statt-des-aenderungsdatums-der-quelle.md` (neu)
- `fusion-workbench/circles/260825-0711-kontextmenue-traegt-zip-unzip-finder/planning/260825-0727_p_plan-kontextmenue-traegt-zip-unzip-finder.md` (Schritt 2 auf `[DONE]`, Nachtrag)
