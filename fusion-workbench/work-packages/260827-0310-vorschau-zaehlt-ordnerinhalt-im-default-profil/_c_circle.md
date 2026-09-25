# Die Vorschau zählt den Inhalt eines Ordners in einem eingebauten Default-Profil

---
**Domain:** code
**Filed by:** shaper (anticipated-circle mode), Kai Stalmann <kai@stalmann.org>
**Claim:** Unclaimed
**Active spec/plan:** circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/planning/260827-1322_*_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md
**Active session history:** circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/history/260827-1635-orchestrator-session.md

---

## Directive

See `**Active spec/plan:**` above. The cited spec or plan states the Directive in force.

## Grounding snapshot

**Der Rückfallweg, an dem diese Runde ansetzt, ist gebaut und ausdrücklich als Nutzerwille festgehalten.** `leseprofil::erkennung::erkennen` (`crates/krk-core/src/leseprofil/erkennung.rs:99`) beantwortet in zwei Durchgängen, welches Profil ein ausgewählter Ordner bekommt: erst alle Pfadmuster, dann alle Kennzeichendateien. Greift keines, bleibt es bei der Metadatenanzeige, und der Modulkopf hält fest, dass das kein Fehlerfall ist, sondern die Wahl des Nutzers vom 260823 (`erkennung.rs:6-8`). Genau an diese Stelle tritt das Default-Profil, und es ersetzt dort nichts, sondern hängt seine Zeilen an.

**Ein Default-Profil hat im heutigen Bau keinen Ort, und der Nutzerentscheid vom 260827 wählt unter den zwei möglichen.** Das Werk kennt allein „ein Profil greift" und „keines greift". Ein eingebautes Profil könnte als dreizehnter Block in die Auslieferungsfassung `resources/default-readers.toml` treten, erreichte damit aber keinen Nutzer, der `readers.toml` schon angelegt hat, denn die Fassung wird beim ersten Start wörtlich kopiert und danach nie wieder angefasst (`crates/krk-core/src/ablage/leseprofile.rs:1-32`; dazu `archive/260826-1637-safe-cleanup-tier-1/shared/decisions/260825-1725_*_wie-erreichen-neue-auslieferungsprofile-einen-nutzer-der-krk-schon-gestartet-hat.md`). Der Nutzer hat das Profil ausdrücklich als in KRK eingebaut und nicht als Block in der Profildatei bestimmt, womit der zweite Weg bleibt: ein Zweig neben `erkennen`. Die Zusage C2.5 der Runde 16, dass ohne Profiltreffer alle sechs Metadatenangaben stehen bleiben, ist davon berührt und der Sache nach gewahrt, denn die Zählzeilen treten unter die sechs und nicht an ihre Stelle.

**Die sechs Metadatenangaben stehen an einer Stelle, und sie steht in der Ansicht und nicht im Kern.** `metadaten_text` (`crates/krk-ui/src/appkit/vorschau.rs:1369-1399`) baut die Zeilen Name, Pfad, Größe, Geändert, Rechte und Typ aus `Inhalt::Metadaten` (`crates/krk-ui/src/vorschaumodell.rs:213-226`); für einen Ordner steht bei der Größe `--`, dieselbe Antwort wie in der Größenspalte aus C1. Die Zusammenfassung eines erkannten Ordners nimmt dagegen den Weg über `krk_core::leseprofil::zusammenfassen` und kommt als `Inhalt::Zusammenfassung` an dieselbe Fläche; verzweigt wird in `laden` (`vorschaumodell.rs:707-720`). Wo die drei Zählzeilen entstehen, entscheidet damit der Plan, und die zwei Wege stehen als Vorlage nebeneinander.

**Der Baustein `zaehlung` gibt es schon, und er kann das Verlangte nicht.** `Baustein::Zaehlung` (`crates/krk-core/src/leseprofil/mod.rs:296-300`) zählt die Einträge, deren Name ein Muster erfüllt, ohne Muster alle, und sieht dabei auf Namen jeden Typs. Er trennt weder nach Typ noch nach versteckt, und keine der drei anderen Zählstellen im Baum tut es an einem Ordnerbestand: `verzeichnis::umfang::zaehlen` läuft über den Unterbaum und deckelt bei 26, `Ordnermodell::markierungsstand` (`crates/krk-core/src/verzeichnis/modell.rs:643`) zählt Markierungen und kennt zwei Klassen statt drei. Beide Trennungen liegen dagegen am Eintrag bereit: `Typ` (`crates/krk-core/src/verzeichnis/eintrag.rs:16-25`) trägt genau die drei Werte Ordner, Datei und Verknüpfung, die die drei Zählzeilen verlangen, wobei `Datei` ausdrücklich auch Gerätedatei, Fifo und Socket aufnimmt, und `Eintrag::versteckt` (`ebd.:60`) trägt das Kennzeichen, das ein führender Punkt **oder** das Systemflag `UF_HIDDEN` setzt (`ebd.:123`, `crates/krk-core/src/verzeichnis/sys.rs:164`). Ob diese Trennung eine allgemeine Fähigkeit der Profile wird oder dem eingebauten Default-Profil vorbehalten bleibt, ist als Datensatz dieser Runde abgelegt; die Runde 18 hat einen fünften Baustein aus einem verwandten Anlass ausdrücklich verworfen (`shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`, Abschnitt „Warum es kein fünfter Baustein wird, und kein größerer Deckel").

**Flach und nicht über den Unterbaum ist keine offene Frage, sondern Festlegung A2 der Runde 16.** Der Doc-Kommentar von `Baustein::Zaehlung` schreibt sie aus: „Sie läuft flach über eine Ebene und nicht über den Unterbaum (Festlegung A2, C3.2)". Die drei Zählzeilen erben diese Lesart, und eine tiefe Zählung stünde gegen sie.

**Der Weg, einen fremden Ordner einmal zu lesen, ohne das angezeigte Ordnermodell anzufassen, steht schon.** `verzeichnis::leser::lesen_hoechstens` (`crates/krk-core/src/verzeichnis/leser.rs:234`) öffnet und schließt innerhalb des Aufrufs, deckelt im Abschluss und liefert `Lesestand { eintraege, abgeschnitten }`; die Zahl kommt von außen und wohnt nicht dort (`ebd.:238-246`). Genau dieser Weg trägt heute jede Profil-Zusammenfassung (`crates/krk-core/src/leseprofil/bausteine.rs:422`), und er filtert Verstecke nicht: die Trennung nach versteckt bekommt eine Zählzeile umsonst, sofern sie am `Eintrag` rechnet und nicht an der Sicht. Ein neuer Rufer erbt dabei den offenen Defekt `shared/issues/260826-1223_*_lesen-trennt-den-deskriptormangel-nicht-obwohl-beide-nachbarlesewege-es-tun-und-die-trennung-tragend-heisst.md`.

**Der Haushalt einer Zusammenfassung ist gedeckelt, und der Deckel wird sichtbar.** `HOECHSTENS_LESELAEUFE` steht auf 12, `HOECHSTENS_OEFFNUNGEN` auf 24 und `HOECHSTENS_EINTRAEGE` auf 2.000 (`crates/krk-core/src/leseprofil/mod.rs:111-138`). Eine Lesung, die an der Eintragsschranke abbricht, liefert `Wert::UeberGrenze` statt `Wert::Zahl` und sagt dann „mindestens N (Lesung bei 2.000 Einträgen abgebrochen)" (`crates/krk-core/src/leseprofil/bausteine.rs:93-115`). Ein Ordner mit mehr als zweitausend Einträgen ist also der Fall, in dem die Zählzeilen etwas anderes sagen müssen als eine Zahl, und was sie dort sagen, ist als Datensatz dieser Runde abgelegt. Das Default-Profil braucht dabei einen einzigen Leselauf über einen einzigen Ordner; es verbreitert damit den offenen Defekt `shared/issues/260825-1953_*_ein-platzhalterlauf-oeffnet-bis-zu-zweitausend-verzeichnisse-und-die-eintragsschranke-faengt-das-nicht.md` nicht, der an der Ortsangabe mit Platzhalter hängt.

**Was die Zeilen für einen unlesbaren Ordner sagen, folgt aus zwei bestehenden Festlegungen und ist keine neue Frage.** Stehen die Einträge nicht zur Verfügung, ist das unentschieden und nicht negativ entschieden (`erkennung.rs`, Abschnitt zu `None`), und ein Baustein, der ins Leere greift, zeigt den Platzhalter `--` (`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-0541_*_was-zeigt-die-zusammenfassung-wenn-ein-baustein-ins-leere-greift.md`). Daneben steht die offene Frage, ob ein Ordner ohne Leserecht sich überhaupt meldet (`shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`).

**Ohne angewählte Zeile beschreibt die Vorschau den angezeigten Ordner, und daraus folgt eine Wirkung, die niemand eigens beschließen muss.** Die Regel steht als eine Funktion in `zu_beschreiben` (`crates/krk-ui/src/appkit/tabelle.rs:488`) und stammt aus dem Nutzerentscheid vom 260825-1740 (`archive/260826-1637-safe-cleanup-tier-1/shared/decisions/260825-1725_*_was-zeigt-die-vorschau-wenn-keine-zeile-ausgewaehlt-ist.md`, Möglichkeit 1, umgesetzt in `9322d5d`). Sobald das Default-Profil greift, tragen die Zählzeilen deshalb auch den Ordner, in den der Nutzer gerade eingetreten ist, ohne dass er eine Zeile anwählt. Die Zahlen kommen dabei aus einem eigenen Leselauf; die Vorschau bekommt nur den Pfad gereicht (`crates/krk-ui/src/appkit/anwendung.rs:1707`) und liest den angezeigten Ordner ein zweites Mal, obwohl das `Ordnermodell` seine Einträge hält.

**Gezählt wird unabhängig vom Schalter, und der Kern kann das schon.** Der Schalter sitzt am Ordnermodell und wirkt an genau einer Stelle, `zeilengrund_von` (`crates/krk-core/src/verzeichnis/modell.rs:722-730`); umgeschaltet wird er über das Kommando `VersteckteUmschalten` (`crates/krk-core/src/tasten/belegung.rs:404`), und sein Zustand reist je Tab (`crates/krk-ui/src/tabs.rs:102`). Der Leseweg der Profile kennt ihn gar nicht. Eine Zählung über alle Einträge ist damit der billigere und nicht der teurere Weg.

**Das Lesen läuft auf dem Arbeitsfaden der Vorschau, und dieser Faden kennt keinen Abbruch.** `Vorschaumodell::datei_anzeigen` kehrt sofort zurück und startet je Anfrage einen eigenen Faden (`crates/krk-ui/src/vorschaumodell.rs:331`); auf dem Hauptfaden gerechnet ginge L7 auf Kosten von L1. Fällt der `Ladevorgang`, fällt sein Empfänger, das `send` scheitert still, und der Faden liest zu Ende (`ebd.:307-311`). Schnelles Durchtippen der Zeilenmarke erzeugt damit je Ordner einen Faden, der ihn ganz liest, und die Deskriptortabelle teilen sich Editor, Vorschau, Vorgänge und beide Dateilisten (`crates/krk-core/src/verzeichnis/umfang.rs:48-56`). Diese Runde setzt keine elfte Zeitzusage und fasst keine der zehn an. Gemessen wird die Arbeit an der Vorschau bis heute nicht: `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md` ist offen und bindet auch diese Runde, und der offene Defekt `shared/issues/260825-1922_*_eine-auffrischung-stoesst-die-vorschau-mit-an-und-die-kosten-sind-ungemessen.md` liegt daneben.

**Zwei offene Defekte an der neuen Vorschauregel treffen das Default-Profil unmittelbar.** `shared/issues/260825-1922_*_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md` hält fest, dass die Regel „ohne Auswahl der angezeigte Ordner" beim Start und beim Tabwechsel nicht greift; solange er steht, zeigt die Vorschau dort auch keine Zählzeilen. `shared/issues/260826-0149_*_claude-md-sagt-nichts-ueber-die-fuenf-neuerungen-der-runde-18-an-der-vorschau.md` sagt, dass die Projektbeschreibung den Stand der Vorschau ohnehin nachzieht.

**Die Runde 18 hat keinen Circle-Datensatz, und ihre Entscheidungen liegen im Archiv.** Ihr Plan ist `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`, gebaut und nicht abgenommen, alle zehn Schritte auf `[DONE]`; einen Spec gibt es nicht, das Schärfen ist übersprungen worden. Ihre sieben beantworteten Datensätze stehen unter `archive/260826-1637-safe-cleanup-tier-1/shared/decisions/` und nicht im aktiven Speicher. Wer diese Runde plant, sucht ihren Vorläufer dort und findet ihn unter `shared/decisions/` nicht mehr.

## Dependencies

- `260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten` — das Leseprofil-Werk, die vier Bausteine, die drei Haushaltszahlen und die Festlegungen A2 und A7 stammen aus dieser Runde. Ihr offener Datensatz `decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md` bindet auch diese Runde.
- `260802-0842-krk-mac-dateimanager-editor-git` — die Dreiteilung der Vorschau, die sechs Metadatenangaben und der Arbeitsfaden stammen aus C6 jener Runde; die Zeitzusagen L1 und L7 aus C8 gelten unverändert.
- Ohne Circle-Datensatz, aber bindend: die Runde 18. Ihr Plan ist `shared/planning/260825-1725_*_plan-vorschau-vertieft-und-zwei-fehler.md`, ihre offenen Defekte stehen unter `shared/issues/`, und ihre sieben beantworteten Entscheidungsdatensätze liegen seit dem Archivlauf vom 260826-1637 unter `archive/260826-1637-safe-cleanup-tier-1/shared/decisions/`.

## Turn log

- **Turn 1 (260827, 16:35–19:10):** alle acht Planschritte gefahren und einzeln committet — S1 `3ee2638`, S3 `bf3a91d`, S2 `9f91f92`, S4 `5e506e6`, S7 `162058f`, S5 `891f313`, S6 `c072de7` (Ontocoder-Tor: ausführen), S8 `d444879` (Abnahmelauf vom Nutzer gefahren, alle Punkte halten). `make check` nach jedem Schritt grün, keine Fehler, kein Bugfixer. Kohärenz im Turn `ok`; Abgleich `coherent`, Empfehlung `none` (`history/260827-1907-reconciliation.md`). Ein Defektdatensatz planmäßig gefilet (Schritt 7).

## Activation proposal

**Vorgeschlagen am:** 260827-0403
**Playmaker-Lauf:** `shared/history/260827-0403-playmaker-direct-dispatch.md`
**Domain-Gewichtung:** code

Diese Runde ist der einzige vorgesehene Circle, und ihre Grundlage liegt
vollständig auf der Platte. Der Rückfallweg, an dem sie ansetzt, ist gebaut und
als Nutzerwille festgehalten (`crates/krk-core/src/leseprofil/erkennung.rs`);
die Trennungen, die die drei Zählzeilen verlangen, liegen am `Eintrag` bereit;
und der Leseweg für einen fremden Ordner steht mit `lesen_hoechstens`. Vier
offene Entscheidungen binden sie, und keine hält sie auf: zwei hat der Shaper
bei der Anlage gestellt und sind vom Nutzer in einem Zug zu beantworten
(`decisions/260827-0311_*_bekommen-die-profile-aus-readers-toml-die-zaehlung-nach-typ-und-versteckt.md`
und `decisions/260827-0311_*_was-sagen-die-zaehlzeilen-fuer-einen-ordner-ueber-der-eintragsschranke.md`),
die zwei anderen stammen von außerhalb
(`shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`
und `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/decisions/260824-1900_*_wie-wird-die-arbeit-dieser-runde-jemals-gegen-l7-gemessen-die-messstrecke-sieht-sie-nicht.md`).

**Die zwei genannten Vorläufer sind beschränkt und nicht kohärent geschlossen,
und das ist hier kein Mangel.** Beide haben ihre Planschritte vollständig belegt
und sind allein am nicht gefahrenen Abnahmelauf beschränkt geblieben, der KRK im
Vordergrund verlangt und damit Nutzerarbeit ist. Gelesen sind für diese Prüfung
die Schließungsnotizen und nicht die Marker.

Kein Abhängigkeitszyklus, keine veraltete Grundlage.

## Closure note

**Geschlossen kohärent (`_c_`) am 260827-1920.** Sitzung `circles/260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil/history/260827-1635-orchestrator-session.md`, ein Turn, neun Commits `a5c7a46..d444879`. Abgleich `history/260827-1907-reconciliation.md`: Urteil **coherent**, Empfehlung `none`, alle acht Planschritte gegen den Baum belegt. Der Nutzer hat den Abnahmelauf aus Schritt 8 am Bündel auf `c072de7` gefahren und alle Punkte bestätigt; die neun Stop-Bedingungen des Plans gelten nach seiner Lesart alle. Durchsicht `reviews/260827-1911-coderev-durchsicht-runde-19-default-profil-zaehlzeilen.md` deckt den ganzen Bereich (`not-opened: none`) und lässt zwei Low-Befunde als offene Datensätze für eine Folgerunde zurück: `issues/260827-1911_o_drei-saetze-im-kommentarteil-der-auslieferungsfassung-…` (Ontocoder) und `issues/260827-1911_o_erkennung-rs-sagt-none-heisse-die-heutige-metadatenanzeige-…` (Coder). Offen bleiben planmäßig `issues/260827-1710_o_c2-5-der-runde-16-…` (Schließung gehört dem Nutzer nach dem Abnahmelauf der Runde 16) und `decisions/260827-1322_o_faellt-das-default-profil-auch-im-messmodus-an-…` (keine Vorbedingung). Keine Auslieferung in dieser Runde.
