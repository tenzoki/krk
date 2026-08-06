# Codeprüfung Turn 23: S6b, Vorschau- und Oberflächenbefunde, Messstrecke und Bauwerkzeug

**Absender:** coderev
**Datum:** 2026-08-06, 13:35
**Umfang:** `git diff 8fd50a6..HEAD`, 75 Dateien, fünf Arbeitspakete
(`194ea16`, `fd5e3c5`, `4195aa3`, `bfaa9c4`, `1b0f3b0`)
**Abnahmelauf:** `make check` läuft grün durch (Bau, Prüfungen, Clippy, Format),
Exit-Code 0.

---

## Zusammenfassung

Der Turn hält, was er ankündigt. Alle vier vorrangig geprüften Änderungen sind
sachlich richtig gebaut: die Bildgrenze greift vor dem Lesen, die
Sitzungssicherung deckt den `?`-Abbruch und die Panik ab, der
Auffrischungsaufschub hängt am laufenden Vorgang und nicht an einem Zeitgeber,
und die `Arc`-Umstellung lässt Halteverhalten und Arbeitsfaden unangetastet.
Fünf Befunde bleiben, davon einer schwer: die Sitzungssicherung überlebt kein
Strg+C, obwohl ihr eigener Kommentar das zusagt.

Die Modulgrenzen halten ausnahmslos. Die 62 Kommentaränderungen sind
stichprobenfest.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch (Auslieferungssperre, Sicherheit, Datenverlust) | 0 |
| Hoch (Korrektheitsfehler, gebrochener Ablauf) | 1 |
| Mittel (Korrektheitsrisiko, Wartbarkeit) | 3 |
| Niedrig (Kosmetik, Aufräumen) | 1 |

Fünf Defektdateien angelegt, alle im Circle-Speicher
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/`.

---

## Die vier vorrangigen Prüfungen

### 1. Die Bildgrenze von 64 MB — greift, aber steht nirgends geschrieben

**Der Mechanismus stimmt.** `laden` (`crates/krk-ui/src/vorschaumodell.rs:466-473`)
prüft `metadaten.groesse > BILDGRENZE` **vor** `std::fs::read`. Die Größe kommt
aus dem `symlink_metadata`, das die Funktion für Name, Rechte und Typ ohnehin
schon erhoben hat; es entsteht kein zweiter Systemaufruf und kein Byte der
Datei wandert in den Speicher.

**Der Rückfall ist derselbe wie beim Text.** Beide Zweige liefern
`Inhalt::Metadaten(metadaten)` mit demselben Wert. Der Bildzweig führt die
Metadaten seit S19 ohnehin mit (`Inhalt::Bild { metadaten: Option<Metadaten> }`),
damit die Ansicht bei einer nicht dekodierbaren Datei darauf zurückfallen kann;
über der Grenze fällt sie eine Stufe früher darauf zurück. Kein zweiter
Rückfallweg.

**Der Befund liegt woanders.** Die Zahl 64 MB steht in keinem Spec, in keinem
Plan und in keinem Entscheidungsdatensatz. C6 zählt als fünftes
Abnahmekriterium auf (`planning/260802-1036_o_spec-navigator-geruest.md:288`):
"Textdateien, Markdown-Dateien und die gängigen Bildformate erscheinen mit
ihrem Inhalt" — ohne Vorbehalt. Für Text trägt L7 den Vorbehalt im Spec
(Zeile 337, "Vorschau einer Textdatei bis 1 MB sichtbar, sonst die
Metadaten"); für Bilder gibt es keine solche Zeile. Der Modulkopf nennt beide
Grenzen "dieselbe Regel mit zwei Zahlen", im Spec ist aber nur die eine belegt.

Geprüft mit `grep -rn "64 MB\|BILDGRENZE\|Bildgrenze"` über den ganzen
Circle-Ordner: einziger Treffer ist der Historieneintrag des Coders.

→ `issues/260806-1329_o_die-bildgrenze-von-64-mb-steht-in-keinem-spec-und-in-keinem-datensatz.md`
(Mittel)

### 2. Die Sitzungssicherung — drei von vier Wegen gedeckt

**Der `?`-Abbruch ist gedeckt.** `Sitzungssicherung::anlegen()` steht in
`Gesamtlauf::fahren` (`crates/krk-bench/src/messen.rs:932`) vor der Rundenschleife,
und der Wert lebt bis zum Ende der Funktion. Jede Runde, die mit `?` abbricht,
lässt den Wert fallen, und `Drop` spielt zurück.

**Die Panik ist gedeckt.** Der Workspace setzt kein `panic = "abort"`; geprüft
mit `grep -n "panic" Cargo.toml crates/*/Cargo.toml` — kein Treffer. Also
wickelt eine Panik ab, und `Drop` läuft.

**Der Fall ohne vorherige `session.toml` ist richtig behandelt.** `anlegen`
setzt `vorher = None` bei `NotFound`, und `Drop` löscht dann die Prüfsitzung
statt sie liegen zu lassen; `NotFound` beim Löschen wird toleriert. Geprüft
durch `ohne_vorigen_stand_bleibt_keine_pruefsitzung_liegen`.

**Die Reihenfolge stimmt ebenfalls.** `plan_schreiben` (Zeile 928) schreibt den
Messplan in den Prüfordner, nicht die `session.toml`; die Prüfsitzung stellt
erst der Kindprozess `krk --messmodus sitzung` über `Messplan::herstellen`
(`crates/krk-ui/src/messmodus.rs:294-308`) her. Die Sicherung liest also
tatsächlich den Stand des Nutzers und nicht bereits die Prüfsitzung. Beide
Seiten treffen denselben Pfad: `Ablage::pfad` leitet auf `Ablageort::datei`
weiter (`crates/krk-core/src/ablage/mod.rs:212-214`), und die Sicherung ruft
`Ablageort::datei` unmittelbar.

**Der vierte Weg fehlt.** Der Kommentar an `messen.rs:1129` sagt zu: "Ein
SIGKILL von aussen ueberlebt auch das nicht; alles darunter schon." Das stimmt
nicht. SIGINT (Strg+C) und SIGTERM beenden den Prozess ohne Abwicklung, also
ohne `Drop`. `krk-bench` hängt keinen Signalgriff ein; geprüft mit
`grep -rn "signal\|SIGINT\|ctrlc\|SIGTERM" crates/krk-bench/src/` — kein
Treffer. Ein Gesamtlauf fährt mehrere Runden mit `FRIST_SPANNEN = 300 s` je
Strecke, läuft also Minuten bis Viertelstunden; Strg+C ist der übliche Weg, ihn
abzubrechen, nicht der Ausnahmefall. Genau dann bleibt die Prüfsitzung als
`session.toml` des Nutzers liegen — die eine Wirkung, gegen die die Sicherung
gebaut wurde.

→ `issues/260806-1328_o_die-sitzungssicherung-ueberlebt-kein-strg-c-obwohl-ihr-kommentar-es-zusagt.md`
(Hoch)

### 3. Die Dateisystemwache — zwei Befunde, einer davon eine Entwurfsfrage

**Der Aufschub hängt richtig.** `vorgangsordner()`
(`crates/krk-ui/src/appkit/anwendung.rs:1249-1257`) liest `ivars.vorgang`; ohne
laufenden Vorgang ist die Liste leer und `gehoert_zu_vorgang` liefert für jeden
Pfad `false`. Ein Zeitgeber, der vergessen werden könnte, entsteht nicht.
`Vorgang::ordner` ist für den Aufschub und für die Abschlussauffrischung
dieselbe Aufzählung (`anwendung.rs:236-252` gegen `anwendung.rs:2290-2292`),
also gibt es keine zweite Wahrheit darüber, was eine Operation anfasst.

**Der erste Befund: ein Vorgang ohne Fertig-Meldung.** `vermitteln`
(`anwendung.rs:2634-2665`) verlässt seine Schleife auch dann, wenn der Kanal
schließt, ohne dass `Meldung::Fertig` kam — bei einer Panik im Arbeitsfaden aus
`krk_core::operation::starten` (`crates/krk-core/src/operation/mod.rs:120-127`).
Dann wird `stand.bericht` nie gesetzt, `vorgang_beenden` nie erreicht, und
`ivars.vorgang` bleibt für immer `Some`. Der Ordner ist damit für die ganze
Laufzeit von KRK von jeder Auffrischung ausgeschlossen, auch von fremden
Änderungen, die C9 zusagt.

`inference:` Einen konkreten Panikpfad in `ausfuehren` habe ich nicht gefunden;
die Produktivpfade unter `crates/krk-core/src/operation/` tragen kein `expect`
und kein `unwrap`. Der Fall ist unwahrscheinlich. Er wiegt aber jetzt schwerer
als vorher: bis zu dieser Runde kostete ein hängengebliebener Vorgang die
stehengebliebene Fortschrittszeile, jetzt hängt die Richtigkeit der angezeigten
Dateiliste daran.

Zwei Wege, an denen ich das Gegenteil geprüft und **ausgeschlossen** habe: die
unbeantwortete Konfliktfrage kommt nicht in Betracht, weil macOS ein Fenster mit
angehängtem Blatt nicht schließen lässt; die Bündelung des Weckrufs
(`Vorgangszustand::buendelung`) verliert den Bericht nicht, weil `melden()` nach
einem `gezeichnet()` einen neuen Weckruf anfordert.

→ `issues/260806-1330_o_ein-vorgang-ohne-fertig-meldung-friert-die-dateiliste-dauerhaft-ein.md`
(Mittel)

**Der zweite Befund: der Aufschub gilt für alle fünf Operationsarten.** Der
behobene Defekt betraf allein das Stapel-Umbenennen, wo FSEvents schneller
meldet, als ein Lesevorgang fertig wird. Für eine lange Kopie in einen
angezeigten Zielordner wurde ein Lesevorgang zwischen zwei Meldungen fertig,
und der Nutzer sah die Dateien nacheinander erscheinen; jetzt steht der
Zielordner bis zum Abschluss unverändert da. Die Ursache liegt eine Schicht
tiefer: ein neu angestoßener Lesevorgang leert sein Ordnermodell, bevor er
liefert. Der Aufschub umgeht das an der Meldestelle statt es an der Lesestelle
zu beheben.

→ `issues/260806-1331_o_der-auffrischungsaufschub-gilt-fuer-alle-fuenf-operationsarten-statt-nur-fuer-die-schnelle.md`
(Mittel)

### 4. Die `Arc`-Umstellung und die ausgeblendete Vorschau — beide sauber

**Das Halteverhalten je Tab aus C6 bleibt unverändert.** `vorschau_nachtrag`
(`anwendung.rs:308-320`) ist ein einziger Platz auf Anwendungsebene, kein Platz
je Tab. Das genügt, und zwar nachweisbar: der Fokus erreicht
`Fokus::Vorschau` nur per Mausklick in die Inhaltsfläche
(`crates/krk-ui/src/kommandos/fokus.rs:53-58`), und das Ausblenden setzt den
Fokus zwangsweise ins Dateifenster (`anwendung.rs:1600-1605`). Bei
ausgeblendeter Vorschau kann der Nutzer den aktiven Vorschau-Tab also nicht
wechseln; der vermerkte Pfad landet beim Einblenden zwingend in demselben Tab,
in dem er auch ohne den Aufschub gelandet wäre.

**Beide Wege zurück auf den Schirm sind bedient.** `bereich_umschalten`
(`anwendung.rs:1609-1614`) holt nach; `zwischenablage_ansehen`
(`anwendung.rs:798`) löscht den Vermerk, weil die Zwischenablage die neuere
Quelle für denselben Tab ist. Geprüft mit
`grep -rn "bereich_umschalten\|umschalten(Bereich" crates/krk-ui/src/` — es gibt
keinen dritten Weg.

**L7 läuft weiter auf einem Arbeitsfaden.** `datei_anzeigen`
(`crates/krk-ui/src/vorschaumodell.rs:339-343`) startet unverändert einen
`Ladevorgang`, der einen Faden aufmacht; die Änderung setzt das Laden bei
ausgeblendeter Vorschau nur ganz aus, sie zieht es nicht auf den Hauptfaden.
L1 ist damit nicht betroffen. L7 selbst misst weiter, weil die Prüfsitzung aus
C8 die Vorschau ausdrücklich eingeblendet hat (Spec, Messbedingungen,
Zeile 324).

**Die `Arc`-Umstellung wirkt, aber halb.** `Vorschaufenster::anzeigen`
(`crates/krk-ui/src/appkit/vorschau.rs:359-367`) klont den `Inhalt`, und für die
Bytes ist das seit `fd5e3c5` ein Zählerschritt. Eine Zeile weiter kopiert
`bild_zeigen` (`vorschau.rs:409`) den ganzen Puffer aber weiterhin in ein
`NSData`. Von zwei vollständigen Kopien je Anzeigedurchgang fällt also eine
weg, nicht beide. Der Kommentar am Feld sagt das richtig ("Der Klon kopiert
keine Bilddatei"), die Commit-Botschaft "Speicher deutlich gesenkt" ist die
optimistischere Lesart. Kein Defekt, nur eine Einordnung.

---

## Befunde nach Thema

### Größengrenzen: ein dritter Weg trägt keine

Die Zwischenablage geht nicht durch `laden` und umgeht damit beide Grenzen.
`inhalt_lesen` (`crates/krk-ui/src/appkit/zwischenablage.rs:87-98`) liest die
PNG- oder TIFF-Daten über `daten.to_vec()` vollständig ein, und
`zwischenablage_anzeigen` (`vorschaumodell.rs:350-365`) übernimmt sie unbesehen.
Ein kopiertes TIFF über 100 MB landet damit als Ganzes im Arbeitsspeicher —
dieselbe Wirkung, gegen die die Bildgrenze gebaut wurde. Leichter als der
behobene Defekt, weil der Nutzer das Bild selbst kopiert und selbst `shift+f3`
drückt, und weil die Daten ohnehin schon im Pasteboard-Server liegen.

→ `issues/260806-1332_o_das-bild-aus-der-zwischenablage-umgeht-beide-groessengrenzen.md`
(Niedrig)

### Bauwerkzeug: die Grenzprüfung ist gründlicher geworden, aber nicht vollständig

`ist_objc2_use` (`xtask/src/release.rs:169-200`) fängt jetzt `pub use` und das
führende `::`, und `sichtbarkeit_abstreifen` behandelt `pub(crate)`,
`pub(super)` und `pub(in …)` richtig. Ich habe die Fallunterscheidung
durchgespielt: `useobjc2::x;` fällt korrekt durch (weder Zwischenraum noch
`::`), `public_use objc2::x;` ebenfalls.

Zwei Lücken bleiben: ein voll ausgeschriebener `objc2::`-Pfad ohne `use`-Zeile
fällt durch (die Form steht heute mehrfach in `appkit/anwendung.rs`), und
`dateien_pruefen` begeht allein `crates/krk-ui/src` (`release.rs:103`), nicht
`krk-bench`. Für `krk-core` trägt das Abnahmekriterium von S15 über die
Abhängigkeiten der Kiste, für `krk-bench` gibt es keine entsprechende Zusage.

→ `issues/260806-1333_o_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten.md`
(Niedrig)

### Abbruch beim fehlenden Tastenabgriff (S6b): richtig gebaut

`hinweis::zeigen` (`crates/krk-ui/src/appkit/hinweis.rs:54-77`) ist eine schmale
Hülle: `NSAlert`, `Critical`, eine ausdrücklich beschriftete Schaltfläche,
`activate()` vor `runModal()`. Die Abgrenzung zu `blaetter` ist im Modulkopf
sauber begründet und trifft zu: ein Blatt hängt an einem Fenster, kehrt sofort
zurück und liefert seine Antwort über einen Rückruf — nichts davon passt auf
die letzte Ausgabe vor dem Beenden.

Beide `None`-Zweige gehen jetzt durch dieselbe Funktion
(`anwendung.rs:1094` und `anwendung.rs:1175` auf `ohne_tastenabgriff_beenden`,
`anwendung.rs:1125`). Der Weg über `terminate:` statt `exit` stimmt: bei
`tastenabgriff_nachziehen` hat der Nutzer gearbeitet, und
`applicationWillTerminate:` (`anwendung.rs:474-484`) schreibt seinen letzten
Stand noch.

Ein Restrisiko, das ich nicht als Defekt gemeldet habe: `runModal()` öffnet
eine geschachtelte Laufschleife, und `tastenabgriff_nachziehen` wird aus
`belegungsansicht_verlassen` (`anwendung.rs:1578`) gerufen, also während das
Belegungsblatt abgeräumt wird. Die Lage tritt nur ein, wenn sich ein Abgriff,
der eben noch stand, nicht neu aufsetzen lässt; sie ist theoretisch und die
Alternative (Abbruch verzögern) wäre die schlechtere Wahl. Ich nenne sie hier,
damit sie festgehalten ist, nicht als Aufforderung.

### Messstrecke: der Vorbehalt steht jetzt vor der Aufzählung

`messung_unmoeglich` (`crates/krk-ui/src/messmodus.rs:702-712`) prüft
`im_vordergrund` vor der Fallunterscheidung über die Messgröße. Das ist die
richtige Stelle: der Vorbehalt hängt an der Strecke und nicht an einer
einzelnen Zusage, und die Prüfung
`im_hintergrund_beginnt_keine_messung` geht ausdrücklich über **jede** Größe
statt nur über L5-Tab, die am 260806 als erste darauf traf. Die Erhebung fragt
`NSApplication::isActive()` und nicht das Schlüsselfenster
(`anwendung.rs:2544-2551`), mit der richtigen Begründung: während eines Blattes
ist dessen Panel das Schlüsselfenster, und KRK steht trotzdem vorn.

### Belegungsansicht: die Lücke war echt

`selectRowIndexes:byExtendingSelection:` fragt `tableView:shouldSelectRow:`
nicht — das ist zutreffend, und `waehlbare_zeile`
(`crates/krk-ui/src/belegungsmodell.rs:301-306`) schließt die Lücke im Modell
statt in der Ansicht. Die Prüfung geht über jede Zeile der Auslieferung, nicht
über eine Stichprobe. Kein Befund.

### Die 62 Zitatänderungen: stichprobenfest

`git show bfaa9c4 -U0` nach Abzug aller Zeilen, die mit `//`, `#` oder `*`
beginnen, lässt genau **eine** Programmzeile übrig: das entfernte
`"CFRunLoop",` aus `Cargo.toml`. Alles andere sind Kommentare, Markdown und
Zeichenketten in Modulköpfen. `make check` bestätigt, dass der Wegfall des
Merkmals die Übersetzung nicht antastet.

---

## Übergreifende Beobachtungen

**Zwei der fünf Befunde sind Zusagen, die weiter reichen als der Code.** Der
Kommentar der Sitzungssicherung sagt Signalfestigkeit zu, die es nicht gibt;
der Modulkopf der Vorschau sagt "beide Grenzen" zu, während ein dritter Weg
keine trägt. Beide Male ist der Code richtig und der Text zu großzügig. Das ist
in diesem Projekt ein wiederkehrendes Muster, weil die Modulköpfe ungewöhnlich
viel Begründung tragen — was ein Vorzug ist, aber die Textpflege zu einem
eigenen Prüfpunkt macht.

**Der Auffrischungsaufschub ist der einzige Eingriff des Turns, der eine Ursache
umgeht statt sie zu beheben.** Alle übrigen fünfzehn Befunde sind an der Wurzel
behoben. Das fällt auf, weil das Projekt sonst konsequent die andere Richtung
geht — `Vorgang::ordner` als die eine Aufzählung, `waehlbare_zeile` im Modell
statt in der Ansicht, `binaer_im_buendel` als die eine Stelle, die den Pfad
bildet.

**Die Modulgrenzen halten ausnahmslos.** Geprüft:

- Keine `objc2`-Erwähnung außerhalb von `krk-ui/src/appkit/` außer in zehn
  Modulkommentaren, die ihre eigene Abwesenheit feststellen.
- `krk-core` nennt `objc2` in zwei Kommentaren und sonst nirgends.
- `#![deny(unsafe_code)]` steht in `krk-core/src/lib.rs:1`,
  `krk-ui/src/main.rs:1` und `krk-bench/src/main.rs:1`. Die einzigen beiden
  `#![allow(unsafe_code)]` stehen in `krk-core/src/verzeichnis/sys.rs:50` und
  `krk-ui/src/appkit/mod.rs:1`, also genau dort, wo CLAUDE.md sie nennt.
- `vorschaumodell.rs`, `belegungsmodell.rs` und `messmodus.rs` sind objc2-frei.

**Die zehn Zeitzusagen aus C8 sind nicht angetastet.** Kein Befund berührt eine
Zahl, ein Abnahmemaß oder eine Messvorschrift. Die einzige Zusage, an der eine
der Änderungen entlangführt, ist L7, und dort bleibt das Lesen auf dem
Arbeitsfaden.

---

## Empfohlene Reihenfolge

**Vor der nächsten Messreihe:** der Strg+C-Befund
(`260806-1328`). Die nächste unterbrochene Messreihe kostet den Nutzer seine
Sitzung, und die Wahrscheinlichkeit dafür ist nicht klein.

**Vor der Abnahme von C6:** die Bildgrenze
(`260806-1329`). Solange die 64 MB nirgends stehen, ist das fünfte
Abnahmekriterium von C6 so formuliert, dass es verfehlt ist. Die Zahl selbst
braucht eine Bestätigung des Nutzers, kein Coder-Urteil. Der
Zwischenablage-Befund (`260806-1332`) hängt daran und wird mit derselben Zahl
erledigt.

**Vor dem Abschluss von Runde 1:** die Entwurfsfrage zum Auffrischungsaufschub
(`260806-1331`). Sie berührt C4 und C9 und sollte nicht als Modulkommentar
stehenbleiben, gleich welcher der drei Wege gewählt wird.

**Aufräumen, ohne Frist:** der Vorgang ohne Fertig-Meldung (`260806-1330`) und
die Lücken der Grenzprüfung (`260806-1333`).

---

## Angelegte Defektdateien

Alle in
`fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/issues/`:

| Datei | Schwere |
|---|---|
| `260806-1328_o_die-sitzungssicherung-ueberlebt-kein-strg-c-obwohl-ihr-kommentar-es-zusagt.md` | Hoch |
| `260806-1329_o_die-bildgrenze-von-64-mb-steht-in-keinem-spec-und-in-keinem-datensatz.md` | Mittel |
| `260806-1330_o_ein-vorgang-ohne-fertig-meldung-friert-die-dateiliste-dauerhaft-ein.md` | Mittel |
| `260806-1331_o_der-auffrischungsaufschub-gilt-fuer-alle-fuenf-operationsarten-statt-nur-fuer-die-schnelle.md` | Mittel |
| `260806-1332_o_das-bild-aus-der-zwischenablage-umgeht-beide-groessengrenzen.md` | Niedrig |
| `260806-1333_o_die-appkit-grenzpruefung-sieht-nur-use-zeilen-und-nur-eine-von-drei-kisten.md` | Niedrig |

## Gesamturteil

**Annehmbar.** Kein Befund sperrt die Auslieferung, und keiner der fünf Commits
muss zurückgenommen werden. Der Turn behebt neunzehn gemeldete Defekte und
schafft dabei einen neuen echten (`260806-1328`), eine Entwurfsfrage
(`260806-1331`) und drei kleinere Lücken. Das Verhältnis ist gut. Die Zusagen in
den Modulköpfen brauchen einen Nachzug, sonst laufen Text und Code auseinander.
