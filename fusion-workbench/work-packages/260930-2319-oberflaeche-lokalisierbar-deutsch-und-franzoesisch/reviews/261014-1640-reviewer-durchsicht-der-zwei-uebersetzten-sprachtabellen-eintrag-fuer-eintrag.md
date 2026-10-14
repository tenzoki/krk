# Durchsicht der zwei übersetzten Sprachtabellen fr.rs und en.rs, Eintrag für Eintrag

**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Reviewed-range:** `e46a678..5681f7a`
**Not-opened:** `CLAUDE.md`, `Cargo.lock`, `Cargo.toml`, `HowTo.md`, `README.md`, `crates/krk-core/src/ablage/atomar.rs`, `crates/krk-core/src/lib.rs`, `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/src/verzeichnis/mod.rs`, `crates/krk-core/src/verzeichnis/sys.rs`, `crates/krk-core/tests/baum.rs`, `crates/krk-core/tests/belegung.rs`, `crates/krk-core/tests/sprache.rs`, `crates/krk-ui/src/appkit/anwendung.rs`, `crates/krk-ui/src/appkit/bereichsleiste.rs`, `crates/krk-ui/src/appkit/blaetter/fremdaenderung.rs`, `crates/krk-ui/src/appkit/blaetter/mod.rs`, `crates/krk-ui/src/appkit/editor.rs`, `crates/krk-ui/src/appkit/fenster.rs`, `crates/krk-ui/src/appkit/statuszeile.rs`, `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/editormodell.rs`, `crates/krk-ui/src/filteranzeige.rs`, `crates/krk-ui/src/kommandos/fokus.rs`, `crates/krk-ui/src/kommandos/mod.rs`, `crates/krk-ui/src/kommandos/zellentaste.rs`, `crates/krk-ui/src/kommandos/zulaessigkeit.rs`, `crates/krk-ui/src/main.rs`, `crates/krk-ui/src/menuemodell.rs`, `crates/krk-ui/src/tabs.rs`, `resources/Info.plist`, `resources/de.lproj/InfoPlist.strings`, `resources/default-keymap.toml`, `resources/en.lproj/InfoPlist.strings`, `resources/fr.lproj/InfoPlist.strings`, `xtask/src/bundle.rs`
**Domain:** code
**Work-item:** 260930-2319-oberflaeche-lokalisierbar-deutsch-und-franzoesisch

Zum Range und zur Liste: der Auftrag war die Lesung der drei Tabellen als ganze Dateien am Stand `5681f7a`, nicht der Diff des Ranges. Vollständig gelesen sind `crates/krk-core/src/sprache/tabelle/de.rs`, `fr.rs`, `en.rs` und `mod.rs` (das Glossar), `crates/krk-core/src/sprache/mod.rs` und `crates/krk-core/src/sprache/schluessel.rs`, dazu der Diff von `5681f7a` an `fr.rs`. Kein Commit nach `5681f7a` fasst `crates/`, `xtask/` oder `resources/` an (`git log 5681f7a..HEAD -- crates/ xtask/ resources/` ist leer), der Arbeitsbaum ist also der gelesene Stand. Von `tests/sprache.rs` sind die Typografieprobe (Zeilen 160–216) und die Namen der Proben gelesen, der Diff nicht; die Datei steht deshalb in der Liste. Die 854 Pfade unter `fusion-workbench/` aus den Migrationscommits `89df15d`, `1fe34c4` und `7d20272` liegen nach dem Prompt außerhalb der Durchsicht und sind nicht einzeln aufgeführt.

## Zusammenfassung

Beide übersetzten Tabellen sagen an jedem Eintrag dasselbe wie die deutsche Quelle: kein Sinnfehler, keine vertauschte Richtung, kein falsches Objekt, kein fehlender oder hinzuerfundener Platzhalter, kein Zahlwort mit falschem Numerus. Was bleibt, sind Konsistenzbrüche innerhalb je einer Tabelle (dieselbe Sache mit zwei Wörtern, zweimal gegen das Glossar), eine Typografiestelle, zwei französische Wortwahlen und drei englische Sätze mit deutscher Satzstellung, dazu Lücken im Glossar. Alles Low; vier gebündelte Datensätze. Die drei Hinweise der Abschlussdurchsicht vom 261001 sind mit `5681f7a` richtig umgesetzt.

## Gelesene Einträge

| Tabelle | `Text` | `Zahlwort` (je zwei Formen) | `kommandoname` | `zugestellt_name` | Summe |
|---|---|---|---|---|---|
| `de.rs` (Quelle, Vergleichsmaßstab) | 601 | 33 | 103 | 7 | 744 |
| `fr.rs` | 601 | 33 | 103 | 7 | 744 |
| `en.rs` | 601 | 33 | 103 | 7 | 744 |

Die Zahlen sind die Längen von `Text::ALLE` (`schluessel.rs:1440`) und `Zahlwort::ALLE` (`:2151`) und die Zählung `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs | grep -cE '^    [A-Z]'` beziehungsweise dieselbe über `Zugestellt`; dass jede Tabelle genau diese Werte trägt, hält der Übersetzer (jede Fallunterscheidung ohne Auffangzweig). Jeder Eintrag von `fr.rs` und `en.rs` ist gegen seinen deutschen gelesen; die Platzhaltermengen hält daneben `die_platzhalter_sind_in_allen_sprachen_gleich` (`tests/sprache.rs:90`), die Guillemets und die geschützten Leerzeichen vor `:` hält `franzoesische_eintraege_tragen_die_franzoesische_typografie` (`:168`).

## Totals

| Schwere | Zahl |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 0 |
| Low | 4 Datensätze (23 Zeilen) |

Je Klasse:

| Klasse | Datensatz | Zeilen |
|---|---|---|
| Sinn | keiner | 0 |
| Sprache und Typografie, `fr.rs` | `261014-1640-drei-sprach-und-typografiebefunde-in-fr-rs-u00a0-vor-semikolon-branchee-und-aller-a-la-ligne.md` | 3 |
| Konsistenz und Glossarabweichung, `fr.rs` | `261014-1640-fuenf-begriffe-stehen-in-fr-rs-in-zwei-schreibungen-papierkorb-verb-zuruecksetzen-haelt-eine-datei-zu-gross-editeur-apercu.md` | 9 |
| Konsistenz und Sprache, `en.rs` | `261014-1640-vier-konsistenz-und-sprachbefunde-in-en-rs-leistenname-regular-ordinary-file-valid-is-preview-und-editor-klein.md` | 9 |
| Glossarlücken, `tabelle/mod.rs` | `261014-1640-das-glossar-in-tabelle-mod-rs-laesst-wiederkehrende-begriffe-aus-und-markierung-und-marke-fallen-in-fr-und-en-auf-ein-wort.md` | 9 Begriffe, 1 Zusammenfall |
| Platzhalter und Zahlwort | keiner | 0 |
| Länge | keiner | 0 |

Alle vier gehen an `code-implementer`: die Tabellen und das Glossar sind `.rs`-Dateien.

## Thema 1: Sinn

Jeder Eintrag von `fr.rs` und `en.rs` sagt, was sein deutscher sagt. Geprüft sind besonders die Stellen, an denen eine Übersetzung leicht kippt: die Richtungen (`OrdnerAufwaerts`, `EintragHoch`/`EintragRunter`, `LesezeichenHoch`/`LesezeichenRunter`, `TabNaechster`/`TabVoriger`, `BildVor`/`BildZurueck`), die Objekte (Datei, Ordner, Eintrag, Tab, Lesezeichen, Notiz, Aufgabe, Termin), die Negationen (`VorgangNichtVollstaendigKopiert`, `VorgangKopiertAberInQuelleGeblieben`, `EditorFremdGeaendertNichtUeberschrieben`, die `Pin…`-Abweisungen), die Bedingungssätze der Hinweise (`HeimMerkerNichtVermerkt`, `HeimMerkerOhneAblage`, `WerksErlaeuterung`, `NeuerungenPreis…`) und die Tastenhinweise der Blätter. Die Satzteile, die ein Rufer zusammensetzt (`{satzteil}` aus `Ablagegrund…`, `{ersatz}` aus `Ersatz…`, `{grund}` aus `Editor…`, `{typ}` aus `DuplikatTyp…`, `{nennform}`, `{bereich}` aus `Bereich…Lang`, `{wofuer}`, `{zusteller}`), ergeben in fr und en an jeder Fügestelle einen Satz: etwa `settings.toml est endommagé, aucun dossier de notes ne s’applique donc` und `{pfad} ne peut pas être ouvert dans l’Éditeur : il est chiffré et ne s’ouvre qu’avec le code PIN`, wo das deutsche „sie“ und das französische „il“ je auf das Wort für die Datei zeigen.

Kein Befund.

## Thema 2: Sprache und Typografie

**`fr.rs`.** Die Guillemets tragen innen überall U+00A0, vor jedem `:` steht U+00A0, der Apostroph ist überall U+2019, kein Eintrag trägt ein ASCII-Anführungszeichen, die Elisionen stimmen (`l’Éditeur`, `d’une`, `qu’un`, `n’existe`; `Ce avec quoi` in `NeuerungenSchlusssatz` ist richtig, `ce` elidiert vor `avec` nicht). Die Akzente auf Großbuchstaben stehen (`À propos`, `Éditeur`, `Écraser`, `Étendre`, `Élargir`). Die Zahlwörter sind in Numerus und Genus richtig (`{n} sera renommée` / `seront renommées` zu `entrée`, `{n} nouveau` / `nouveaux`, `un dossier` / `{n} dossiers`), und die Einzahl bei 0 ist seit `261001-1929_*_franzoesisch-nimmt-bei-null-die-einzahl-und-eine-einzahl-ohne-n-sagt-dann-eins.md` Sache von `Sprache::form` und nicht der Tabelle.

Drei Befunde, Low, im ersten Datensatz:

- `fr.rs:453` (`LeisteFilterHinweis`): vor dem `;` steht U+00A0 statt U+202F; es ist das einzige Semikolon der Datei mit dem falschen Zeichen. Die Probe `franzoesische_eintraege_tragen_die_franzoesische_typografie` lässt vor `;` beide zu (`tests/sprache.rs:205`); das ist Mechanik und nicht Gegenstand dieser Durchsicht, aber der Grund, warum die Stelle stehen konnte.
- `fr.rs:104` (`VorgangKeinPapierkorb`): „aucune Corbeille n’est branchée“ sagt „angeschlossen“; für den Papierkorb eines Datenträgers passt „disponible“.
- `fr.rs:1173` (`Kommando::EditorZeileSpringen`): „Aller à la ligne“ ist die feste Wendung für den Zeilenumbruch; „Atteindre la ligne…“ sagt „Zu Zeile springen“.

**`en.rs`.** Die Befehlsnamen stehen in Title Case mit kleinen Präpositionen und Konjunktionen, wie macOS seine Menüs setzt (`Open with Default Application`, `Go to Top of List`, `Show or Hide Preview`); die Schaltflächen ebenso (`Don’t Save`, `Move to Trash and Replace`, `Apply to All Remaining`); Statuszeile und Erläuterungen in Satzform. Anführungszeichen “ ”, Apostroph U+2019, keine Guillemets.

Drei Sätze, Low, im dritten Datensatz: `OrtLeer`, `OrtFremdesBenutzerverzeichnis` und `OrtKeinText` (`en.rs:235`, `:241`, `:244`) beginnen ihren zweiten Teil mit „valid is a location …“, der deutschen Stellung von „gültig ist ein Ort“; im Englischen steht das Subjekt vor dem Verb.

**Die drei Fixes aus `5681f7a`.** Geprüft gegen den Diff:

- „Retour“ für ↩ an neun Stellen (`fr.rs:483`, `:485`, `:711`, `:714`, `:722`, `:745`, `:757`, `:764`, `:770`) und im Glossar (`mod.rs:168–169`); `BelegungsansichtTasteEingabe` ergibt über `Schaltflaechenzeichen::anzeige` (`krk-ui/src/appkit/belegungsansicht.rs:244–256`) „Cmd+Retour“, und die Erläuterung nennt dieselbe Taste als „la touche Retour“. Die zwei verbliebenen „Entrée“ (`fr.rs:1184–1185`) meinen den Eintrag. Richtig.
- Die vier Auswahlbefehle (`fr.rs:1104–1107`) sind Sätze: „Monter / Descendre la sélection d’une entrée / d’une page d’écran“. Richtig, und je Sprache eindeutig.
- `VorgangAbgebrochen` (`fr.rs:572`): „{was} : annulé, transféré : {uebertragen}“ hängt kein Mitwort mehr an die Klammer von `VorgangUebertragen`. Richtig im Sinn. speculation: ob ein französischer Leser „Copie : annulé“ als unveränderliches Etikett liest (so der Kommentar `fr.rs:567–571` und die Probe `die_franzoesische_abbruchmeldung_braucht_keine_angleichung`) oder als fehlende Angleichung an das weibliche `Copie`, kann ich nicht belegen; fünf der sieben Vorgangsarten sind weiblich (`Copie`, `Mise à la Corbeille`, `Compression`, `Décompression`, `Duplication`). Eine Form, die beide Lesarten vermeidet, wäre mit Nomen: „{was} : annulation, transfert : {uebertragen}“ und für `VorgangFertig` (`fr.rs:573`) „{was} : terminé“ ebenso. Kein Datensatz, weil es keine Regel verletzt.

Weitere Stellen, an denen ich die Konvention von macOS nicht belegen kann, ohne Datensatz:

- speculation: macOS nennt die Taste ⌫ auf Französisch „Supprimer“ und nicht „retour arrière“ (`fr.rs:485`, `la touche de retour arrière`); auf Englisch „Delete“ und nicht „Backspace“ (`en.rs:439`). Beide Fassungen sind verständlich.
- speculation: der Finder nennt den Befehl auf Französisch „Placer dans la corbeille“; das Glossar sagt „mettre à la Corbeille“, und die Tabelle führt beides. Der Konsistenzbefund steht im zweiten Datensatz; welche Form gewinnt, entscheidet der Nutzer.

## Thema 3: Konsistenz und Glossar

**`fr.rs`**, neun Zeilen im zweiten Datensatz, Low:

- Das Papierkorb-Verb: `Kommando::InPapierkorb` (`fr.rs:1147`, „Placer dans la Corbeille“) und `VorgangZielNichtInPapierkorb` (`:108`, „placée“) gegen Glossar (`mod.rs:150`, „mettre“), Schaltfläche (`:985`), Konfliktblatt (`:708`), Löschfrage (`:1064–1065`) und Vorgangsüberschrift (`:552`). Der Menüeintrag und die Schaltfläche der Rückfrage, die auf ihn folgt, nennen denselben Vorgang verschieden; das ist die eine Stelle dieser Durchsicht, die ein Nutzer im selben Vorgang sieht.
- zurücksetzen: `Kommando::Werkseinstellungen` (`:1203`, „Rétablir“) und `WerksZurueckgesetzt`/`WerksTeilweiseZurueckgesetzt` (`:228–229`, „rétablis“) gegen Glossar (`mod.rs:84`, „réinitialiser“), die Frage (`:688`), die Schaltfläche (`:683`) und jede Hindernismeldung (`:204–224`).
- „hält eine Datei“: `EditorHaeltKeineDatei` (`:938`, „contient“) gegen `PinKeineDatei` (`:802`) und `OrtAbweisung` (`:287`, „tient“).
- „zu groß“: `HeimZettelZuGross` (`:329`, „trop gros“) gegen „trop volumineux“ (`:791`, `:1035`).
- Großschreibung: `BereichVorschauLang` (`:436`, „la fenêtre d’aperçu“) und `BereichEditorLang` (`:437`, „l’éditeur intégré“), während die Tabelle Aperçu und Éditeur sonst als Eigennamen groß schreibt.

**`en.rs`**, sechs Zeilen im dritten Datensatz, Low:

- `WirkungsbereichLeiste` (`en.rs:14`, „bookmark and volume bar“) gegen „bookmarks and volumes bar“ (`:393`, `:1019`, `:1045`).
- „ordinary file“ (`:170`, `:181`) gegen „regular file“ (`:86`, `:126`, `:527`).
- `BereichVorschauLang`/`BereichEditorLang` (`:396–397`) klein, sonst Preview und Editor groß.
- `Zahlwort::StartMeldungen` (`:950–951`, „at start“) gegen „start-up“ (`:229`, `:253`).

**Glossar**, vierter Datensatz, Low: neun Begriffe, die in beiden Tabellen mehrfach vorkommen und keine Zeile haben (Auswahl, Markierung, Tabelle, Kombination, Funktion, Unterbaum, zu groß, hält, Rücktaste); bei zweien (zu groß, hält) ist `fr.rs` deshalb uneinheitlich. Dazu der Zusammenfall von „Markierung“ und „Marke“ auf „marque“ / „mark“ in beiden Sprachen (`fr.rs:1039`, `:1117–1118`, `:443`; `en.rs:925`, `:999–1000`, `:403`), die das Deutsche trennt.

Geprüft und ohne Befund: die Eindeutigkeit der Befehlsnamen je Sprache (hält die Probe `keine_zwei_funktionen_tragen_in_einer_sprache_denselben_namen`; nachgelesen, dass auch `Kommando::Bearbeiten` „Modifier“ / „Edit“ neben `EintragBearbeiten` „Modifier l’entrée“ / „Edit Entry“ und dem Menü „Édition“ / „Edit“ eindeutig bleibt); die Schaltflächentitel passen zu den Tastenhinweisen, die sie nennen (`Ignorer` zu „Retour ignore“, `Recharger`/`Écraser` zur Erläuterung, `Enregistrer`/`Ne pas enregistrer` zu „Retour enregistre, Cmd+Retour abandonne“, en ebenso); die Glossarbegriffe `Presse-papiers`, `volet de fichiers`, `dossier de départ`, `dossier de données`, `signet`, `onglet`, `série de photos`, `réglages d’usine`, `What’s New`, `file pane`, `data folder`, `home directory` stehen überall, wo der deutsche Begriff steht.

## Thema 4: Platzhalter und Zahlwörter

Jeder `{name}` des deutschen Eintrags steht im französischen und im englischen, keiner dazu; die Probe `die_platzhalter_sind_in_allen_sprachen_gleich` hält das, und die Lesung hat an keiner Stelle eine Umgehung gefunden (kein Platzhalter in einem Kommentar, keiner mit geänderter Schreibweise). Die Zahlwortpaare stimmen in beiden Sprachen in Numerus und Kongruenz, auch wo die Einzahl `{n}` auslässt (`une entrée`, `one entry`, `un dossier`, `one folder`, `, une marque masquée`, `, one mark hidden`), und die Sätze, die ein Zahlwort einsetzen (`Markierungsstand`, `VorgangZeile`, `VorgangUebertragen`, `StatuszeileFolgeGekuerzt`, `LoeschenDarunterOrdner`, `StapelZusammenfassung…`), ergeben mit beiden Formen einen Satz. Die französische 0 geht seit `Sprache::form` an die Mehrzahl, wo die Einzahl `{n}` auslässt („0 entrées“), und bleibt bei der Einzahl mit `{n}` („0 octet“), was französisch richtig ist.

Die `Text`-Einträge mit `{n}`, die nur ab zwei gerufen werden (`PfadeKopiert`, `AbgelegtMehrere`, `UebergebenMehrere`, `NichtAngenommenMehrere`, `UebergebenAnMehrere`), stehen in fr und en in der Mehrzahl; ihre Rufer in `krk-ui/src/kommandos/operationen.rs` (`:1320`, `:1583`, `:1688`, `:1699`, `:1775`) erreichen sie allein im Zweig `mehrere`.

Kein Befund.

## Thema 5: Länge

Die Einträge an schmalen Stellen bleiben in der Größenordnung des Deutschen: die Schalter der Bereichsleiste (`Signets`, `Gauche`, `Droite`, `Aperçu`, `Éditeur`, `Git`; `Bookmarks`, `Left`, `Right`, `Preview`, `Editor`, `Git`), die Spaltenköpfe (`Date de modification` mit 20 Zeichen gegen `Änderungsdatum` mit 14 ist die längste Abweichung, `Date Modified` 13), die Spalten der Stapelvorschau (`Actuel`/`Nouveau`/`Remarque`), die Zählzeilen der Vorschau, die Vorgangszeile der Statuszeile (fr „Copie : 3 entrées, 1,2 Mo, 2 positions sélectionnées · name · Esc annule“ gegen de „Kopieren: 3 Einträge, 1,2 MB, 2 ausgewählte Positionen · name · Esc bricht ab“) und die Schaltflächen der Blätter (`Rechercher le suivant` für `Weitersuchen` ist die längste, ein Blatt trägt sie). Kein Eintrag ist um ein Vielfaches länger.

Kein Befund.

## Querschnitt

- **Die Tabelle hält die Form, nicht die Wortwahl.** Alles, was eine Probe prüfen kann (Platzhalter, Guillemets, Leerzeichen vor `:`, ASCII-Anführungszeichen, Eindeutigkeit der Befehlsnamen), stimmt; jeder Befund dieser Durchsicht liegt dort, wo keine Probe hinreicht: ein Wort an zwei Stellen verschieden, ein Wort mit Nebensinn, eine Satzstellung. Das ist die Grenze, die der Modulkopf von `sprache/mod.rs` unter „Die Regel“ selbst zieht.
- **Beide Konsistenzbrüche gegen das Glossar betreffen einen Menüeintrag gegenüber seinem Blatt** (Papierkorb, Werkseinstellungen): der Befehlsname ist in einer anderen Fallunterscheidung (`kommandoname`) übersetzt als die Texte des Blattes (`text`), und die zwei sind offenbar getrennt entstanden. Wer das Glossar nachzieht (vierter Datensatz), liest `kommandoname` gegen `text` je Begriff gegen.
- **Die Typografieprobe ist weiter als die Regel**: sie lässt U+00A0 und U+202F vor `;`, `!`, `?` beide zu, die Regel verlangt U+202F. Die eine durchgerutschte Stelle ist der Beleg. Ob die Probe enger wird, ist eine Frage an die Mechanik, die diese Durchsicht nicht hatte; sie steht hier als Beobachtung.

## Empfohlene Reihenfolge

1. Mit der nächsten Arbeit an den Tabellen, in einem Durchgang: die vier Datensätze, zusammen 23 Zeilen, keiner blockiert eine Auslieferung. Beim Papierkorb-Verb und beim Zeilensprung zuerst die Wahl des Nutzers einholen (Glossar oder Finder; „Atteindre la ligne…“), der Rest folgt dem Glossar und den Zeilen „soll“.
2. Nach Behebung gelten `fr.rs` und `en.rs` als Fläche für Fläche durchgesehen; die zwei Vermerkzeilen am Kopf beider Dateien können dann fallen, ohne eine zweite Lesung. Die Behebung selbst braucht keine neue Durchsicht, wenn sie allein die Zeilen „soll“ einsetzt und `cargo test -p krk-core` grün bleibt.
