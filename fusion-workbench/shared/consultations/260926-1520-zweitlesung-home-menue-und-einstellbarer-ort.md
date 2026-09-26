# Consultation: Zweitlesung von Spec und Plan „Menü Home und einstellbarer Ort“ vor dem Bau

**Date:** 2026-09-26 15:20
**Status:** Complete
**Requested by:** Kai Stalmann über den Orchestrator (Nutzer: „baue selbständig“); Arbeitspaket `260925-2356-f2-oeffnet-krkhome-statt-notizfenster`

## Question

Hält der Plan `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md` zusammen mit dem Spec `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md` einer Prüfung am Code stand, bevor gebaut wird? Sieben Einzelfragen: (1) die vom Planer benannten Widersprüche zwischen Spec und Plan, (2) das Schreiben von `settings.toml` über die Byte-Bereiche aus `toml::Spanned`, (3) der Griff als `Result<Heimordner, Ortsfehler>` und jede alte Abschrift nach einem Wechsel im Betrieb, (4) `NSOpenPanel` als Blatt, (5) der neue Funktionsbereich „Home“, (6) die Zeitzusagen L1 bis L10, (7) der offene Entscheid zur beschädigten `settings.toml`.

## Kurzurteil

**Baubar nach fünf Korrekturen, davon zwei im Code-Entwurf und drei im Text.** Die Bauform trägt: ein Ort, eine Prüfung, ein Wert im Umlauf, und der Schreibweg ersetzt genau einen Byte-Bereich. Keine der Fragen hat einen Weg gezeigt, auf dem `secrets.txt` durch einen Ortswechsel im Klartext auf die Platte käme. Gefunden haben wir zwei Fehler im Plan, die ohne Korrektur gebaut würden:

- **M1: `esc` schließt den Ordnerdialog nicht, sobald ein alter Blattgriff im Schlitz liegt.** Die Annahme des Plans über `abbrechen` gilt nur für einen leeren Schlitz.
- **M2: Scheitert beim Start das Öffnen der Ablage oder die Sperre, gilt still `~/krkhome`.** Genau diesen Rückfall schließt der Spec aus.

Dazu kommen drei Textkorrekturen: die Bedingung „beschädigt“ im Entscheid (M3), eine Anweisung an den Nutzer, die seine Belegung löscht (M4), und die zwei Spec-Stellen, die der Planer schon benannt hat (M5).

| # | Frage | Urteil |
|---|---|---|
| 1 | Widersprüche Spec ↔ Plan | Der Planer hat in allen vier Punkten recht. Der Spec zieht nach (Wortlaut unten). Die Abweisung bei offener Datei bleibt, mit richtiger Begründung und auf den neuen Ort erweitert. |
| 2 | Schreiben über `Spanned` | Robust, an fünf Randfällen nachgemessen. Die Regel „jede andere Zeile Byte für Byte“ ist eingehalten. Offen bleiben die verknüpfte Datei (Empfehlung: nicht schreiben) und der Vergleich „derselbe Ort“. |
| 3 | Alte Abschriften nach dem Wechsel | Kein Klartextleck und keine doppelte Übernahme. Zwei Lücken: M2 und der Editor mit einer Datei des **neuen** Orts (S1). |
| 4 | `NSOpenPanel` als Blatt | Richtig gewählt, mit M1. |
| 5 | Menü „Home“ | Die Bauform trägt. Die Anweisung „F1, `cmd+r`“ setzt die ganze Belegung zurück (M4). |
| 6 | L1 bis L10 | Keine neue Berührung. L4 steht ohnehin schon daneben. Die drei Proben des Plans genügen. |
| 7 | Beschädigte `settings.toml` | Möglichkeit 1, mit zwei Schärfungen (M2, M3). |

## Context

- HEAD ist `3091666`. Der Arbeitsbaum trägt Spec, Plan und die zwei neuen Entscheide als unverfolgte Dateien.
- Nachgelesen haben wir `crates/krk-core/src/heimordner/mod.rs`, `crates/krk-core/src/ablage/{einstellungen,mod,atomar,sperre,sitzung}.rs`, `crates/krk-ui/src/heimgriff.rs`, `crates/krk-ui/src/editormodell.rs`, `crates/krk-ui/src/tabs.rs`, `crates/krk-ui/src/vorschaumodell.rs`, `crates/krk-ui/src/belegungsmodell.rs`, `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-ui/src/appkit/{anwendung,ereignisse}.rs` und `crates/krk-ui/src/appkit/blaetter/mod.rs`, jeweils an den unten zitierten Stellen.
- Die Byte-Bereiche aus `toml::Spanned` haben wir selbst gemessen: ein Wegwerfprojekt im Arbeitsverzeichnis dieser Sitzung, offline gegen `toml` 1.1.4 aus der `Cargo.lock` des Projekts übersetzt. Ergebnisse unter Frage 2.
- Die eigene Belegung dieses Geräts (`~/Library/Application Support/KRK/keymap.toml`) führt `notizzettel` an Zeile 459 und keinen der sieben Eintragsbefehle. Nur so viel ist gelesen; die Datei selbst ist nicht verändert.

## Analysis

### 1. Die Widersprüche zwischen Spec und Plan

**H2.5, die aufgelöste Form beim Start: der Plan hat recht, und der Spec muss nachziehen.** Heute entsteht die leichte Form über `symlink_metadata` und `read_link` am Eintrag im Benutzerverzeichnis (`crates/krk-core/src/heimordner/mod.rs:162-177`). Das trifft allein das Benutzerverzeichnis, weil `krkhome` dort unmittelbar liegt. Für `/Volumes/X/notizen` trifft derselbe Aufruf das Laufwerk, und ein hängendes Laufwerk hielte den Start an. Der Plan schränkt die leichte Form deshalb auf einen Ort ein, dessen übergeordneter Ordner das Benutzerverzeichnis ist (Plan, Zeile 174). Für jeden anderen Ort gilt bis zum ersten F2 oder „Ort wählen…“ allein die geschriebene Form. Der Preis bleibt klein. Ein Tab auf einer anderen Schreibweise zeigt `notes.txt` bis dahin als Text. `secrets.txt` schützt `sonderdatei_genau` über Gerät und Inode unabhängig davon (`mod.rs:262-276`).

Vorschlag für den Spec, H2 Kriterium 5: „Die aufgelöste Form entsteht beim Start leicht, aber nur für einen Ort unmittelbar im Benutzerverzeichnis. Für jeden anderen Ort entsteht sie erst bei F2 und bei ‚Ort wählen…‘ über `canonicalize`. Bis dahin erkennt KRK den Ordner allein an der geschriebenen Form.“

**H3, die Begründung der Abweisung bei offener Datei: sie ist falsch, und die Abweisung bleibt trotzdem.** Das Sichern verzweigt über den gehaltenen `Schutz` (`crates/krk-ui/src/editormodell.rs:1649-1660`). Eine entsperrte `secrets.txt` wird nach einem Wechsel weiter verschlüsselt gesichert. Die Erkennung fragt allein der Zweig `Klartext`, und zwar live über den Griff (`editormodell.rs:1337-1341`). Auch die übrigen Sperren für Geheimes hängen am `Schutz` oder an der live gefragten Erkennung: `haelt_geheimnisse` (`editormodell.rs:1164-1170`), die verweigerte Textmarke laut Schließungsvermerk von `260926-1004_*_…` und die Übernahme nach dem Lesen (`editormodell.rs:1379`). Der Satz „der Klartextweg stünde offen“ trifft den Baum also nicht.

Die Abweisung ist dennoch vertretbar, weil der Editor sonst bis zum Schließen eine Eintragstabelle und „PIN ändern“ für eine Datei zeigte, die nicht mehr im Notizordner liegt. Wir empfehlen, sie so zu bauen, wie der Plan es vorsieht, und die Begründung im Spec zu ersetzen: „Hält der Editor eine Datei des geltenden Orts, zeigte er sie bis zum Schließen weiter als Eintragsdatei, obwohl sie danach eine gewöhnliche Datei ist. Das Sichern bliebe sicher, weil es über den gehaltenen Schutz verzweigt; die Abweisung verhindert eine Anzeige, die nicht mehr stimmt.“ Die Alternative im Spec, den Editor die Datei in ihrer Form halten zu lassen, ist keine neue Regel am Sicherungsweg, sondern sein heutiges Verhalten. Der Satz unter „Decisions made“ (Spec, Zeile 158) ist entsprechend zu berichtigen.

**Der Fall, den weder Spec noch Plan abdecken, ist die Datei des neuen Orts (S1).** Angenommen, der Editor hält eine leere `secrets.txt` aus dem Ordner, den der Nutzer gleich wählen wird, und hält sie als Klartext, weil der Ordner noch nicht der Notizordner ist. Nach dem Wechsel weist das Sichern ab (`editormodell.rs:1651-1656`). Das ist kein Leck, aber der Nutzer kann seinen Text dann nicht mehr sichern. Der Plan fragt `gehaltene_notizdatei` in `ort_uebernehmen` ein zweites Mal (Plan, Zeile 250), doch nur gegen den geltenden Ort. Dieselbe Funktion mit dem **neuen** `Heimordner` gefragt schließt den Fall. Das ist ein weiterer Aufruf und kein neuer Mechanismus.

**H2.3, der Werttyp: wir empfehlen die einfachere Form statt `Ortswert`.** Der Plan führt `Option<toml::Spanned<toml::Value>>` und ein `Ortswert { Text, KeinText }` ein, damit `notizordner = 5` die Datei nicht beschädigt (Plan, Zeile 187). Für `terminal` gilt dieselbe Lage heute anders: ein Wert falschen Typs macht die ganze Datei beschädigt (`crates/krk-core/src/ablage/einstellungen.rs:127-132`). Mit Möglichkeit 1 des offenen Entscheids ergibt eine beschädigte Datei ohnehin „kein Ort“ samt Meldung. Der Nutzer sähe also in beiden Fällen ein F2, das mit Grund nichts tut. Die Sonderform bringt eine zweite Schadensklasse neben der vorhandenen, eine Variante `Ortsfehler::KeinText` und eine Ersetzung über einem Bereich, der gar keine Zeichenkette trägt. Unsere Messung zeigt, dass `Option<toml::Spanned<String>>` in der Struktur mit `deny_unknown_fields` den Bereich genauso liefert und `notizordner = 5` als Lesefehler abweist. Der Spec sollte H2.3 deshalb so fassen: „Ein Wert, der kein Text ist, macht die Datei beschädigt wie bei `terminal`.“ Diese Empfehlung ist nicht zwingend. Die Form des Plans ist ebenfalls korrekt, nur größer.

**Beschädigte `settings.toml`:** siehe Frage 7.

### 2. Das Schreiben über `toml::Spanned`

**Der Weg ist robust, und er hält die Regel ein, die er ablöst.** Die Messung gegen `toml` 1.1.4 mit `Option<toml::Spanned<String>>` und `deny_unknown_fields` ergab:

| Eingabe | Ergebnis |
|---|---|
| `notizordner = "~/a" # k` hinter `terminal` | Bereich 29..34, genau `"~/a"`, der Kommentar liegt außerhalb |
| UTF-8-BOM vor dem Schlüssel | Bereich 17..22. Der Versatz zählt die drei Bytes des BOM mit; ein Ersatz im selben Text trifft also richtig |
| CRLF und `'~/a'` | Bereich 30..35, genau `'~/a'` |
| Schlüssel zweimal | Lesefehler; nach dem Plan also `Beschaedigt` und kein Schreiben |
| `notizordner.x = "…"` | Lesefehler |
| `notizordner = 5` | Lesefehler (mit `Spanned<String>`) |
| `"notizordner" = """~/a"""` | Bereich deckt die dreifachen Anführungszeichen |
| `"~/aä"` | Bereich deckt die Schreibweise mit Escape |

Die einzelnen Robustheitsfragen:

- **Mehrfaches Vorkommen** gibt es in gültigem TOML nicht; der Leser weist ab, und der Plan schreibt dann nicht.
- **Fehlt der Schlüssel, hängt der Plan ihn ans Ende.** Das ist heute richtig, weil `deny_unknown_fields` außer den zwei skalaren Schlüsseln nichts zulässt. Eine Tabellenüberschrift kann also nicht dastehen. Käme später ein Schlüssel mit eigener Tabelle hinzu, landete ein angehängter `notizordner` darin. Die zweite Lesung des Ergebnisses im Plan (Zeile 234) fängt das ab und schreibt nicht. Wir empfehlen, die Regel im Modulkopf zu nennen (O1).
- **CRLF** bleibt in jeder unberührten Zeile erhalten. Eine angehängte Zeile endet auf `\n`, die Datei trägt danach gemischte Zeilenenden. Das ist harmlos und optional zu beheben (O2).
- **BOM:** erhalten, siehe Tabelle.
- **Verknüpfte Datei:** `atomar::schreiben` ersetzt das Ziel über `rename` (`crates/krk-core/src/ablage/atomar.rs:175-176`, `:292-294`). Ist `settings.toml` ein symbolischer Verweis, etwa in ein Dotfiles-Verzeichnis, steht danach eine gewöhnliche Datei an seiner Stelle. Das Ziel behält den alten Wert, und spätere Änderungen dort wirken nicht mehr. Der Plan nimmt das hin, weil `keymap.toml` dasselbe erlebt (Plan, Zeile 404). Wir empfehlen die andere Antwort (S3): Ist die Datei ein Verweis, schreibt „Ort wählen…“ nicht und sagt, dass der Wert von Hand zu setzen ist. Das kostet ein `symlink_metadata` unter der Sperre. Es hält die Regel, dass KRK eine vom Nutzer gepflegte Datei nicht still in etwas anderes verwandelt. `keymap.toml` bleibt davon unberührt.
- **Zweite Instanz:** Das Lesen, Ersetzen und Schreiben läuft unter der Schreibsperre (`crates/krk-core/src/ablage/mod.rs:628-631`), also verliert keine Instanz die Änderung der anderen auf der Platte. Die andere Instanz behält ihren Ort im Speicher bis zum Neustart. Ihr F2 legt deshalb am **alten** Ort an, was fehlt. Der Spec sagt, dass sie den neuen Ort erst nach dem Neustart kennt (H3, Kriterium 14), nennt aber nicht die Folge, dass am alten Ort dann doch etwas entstehen kann (O8).
- **Die Sperre:** Ein Durchgang darf nicht geschachtelt werden (`crates/krk-core/src/ablage/sperre.rs`, Abschnitt „Was der Übersetzer nicht hält“). Der Abschlussblock des Dialogs läuft auf dem Hauptfaden außerhalb jedes Durchgangs, und ein zweiter Faden fährt in diesem Prozess keinen. Die Bedingung ist erfüllt.
- **Ein Textprogramm mit offenem Puffer** kann den geschriebenen Wert beim nächsten eigenen Sichern überschreiben. Das ist jeder Datei eigen, die zwei Programme schreiben, und verdient einen Satz in `HowTo.md`, mehr nicht.

**Eine Lücke steckt im Vergleich „derselbe Ort“ (S4).** Der Plan vergleicht den gewählten Ort mit dem geltenden im Speicher und schreibt bei Gleichheit nichts (Plan, Zeile 250). Hat der Nutzer den Schlüssel seit dem Start von Hand geändert, gilt dieser Wert erst beim nächsten Start. Wählt er nun ausdrücklich den geltenden Ort, schreibt KRK nichts, und beim nächsten Start gilt doch der Wert von Hand. Der Vergleich gehört deshalb unter die Sperre gegen den Wert **in der Datei**. Nur wenn beide gleich sind, entfällt das Schreiben.

**Zur Projektregel:** Der Kopf von `einstellungen.rs` begründet das Schreibverbot mit dem Verlust der Kommentare (`einstellungen.rs:25-38`). Der neue Weg erhält jedes Byte außer dem Wertbereich, prüft das Ergebnis vor dem Schreiben durch ein zweites Lesen und schreibt eine beschädigte Datei nie. Verloren gehen die erweiterten Attribute, Zugriffslisten und harten Verweise der ersetzten Datei. `atomar.rs:56-66` nennt diesen Preis für jede Ablagedatei. Mit S3 halten wir den Weg für regelgerecht in der neu gefassten Form, die H3 verlangt.

### 3. Der Griff als `Result<Heimordner, Ortsfehler>` und die alten Abschriften

**Kein Weg führt durch einen Ortswechsel zu Klartext auf der Platte.** Klartext entsteht nur im Zweig `Schutz::Klartext` des Sicherns. Dieser Zweig fragt die Erkennung zum Zeitpunkt des Sicherns über den Griff (`editormodell.rs:1337-1341`, `:1651`), nicht eine beim Öffnen gemachte Abschrift. Jede andere Abschrift, die einen Aufruf überlebt, steuert allein die Anzeige oder das Lesen:

| Halter | Wirkung einer alten Abschrift | Befund |
|---|---|---|
| Ladeauftrag der Vorschau (`crates/krk-ui/src/vorschaumodell.rs:837`) | eine `secrets.txt` des neuen Orts wird mit altem Wert nicht erkannt; gelesen wird dann das Chiffrat oder null Bytes | kein Leck; 3.2b gibt den Tabs ohnehin einen neuen Auftrag |
| Eigenschaft „ohne Inhaltsauftrag“ (`crates/krk-ui/src/tabs.rs:1399-1419`) | der Inhaltsfilter liest `secrets.txt` am neuen Ort bis zum nächsten Lesevorgang | kein Leck, er liest Chiffrat; 3.2b liest die Tabs neu. Die tiefe Suche aus einem übergeordneten Ordner liest Chiffrat ohnehin, so vom Nutzer entschieden (`260926-0050_*_…`, `Answered:`) |
| Dateityp im Editormodell (`editormodell.rs:1385-1389`) | Tabelle statt Text oder umgekehrt | nur Anzeige; die Abweisung verhindert den Fall für den geltenden Ort |
| Einfärbelauf der Vorschau | Hervorhebung | nur Anzeige |
| `Schutz` im Editor | bleibt `Verschluesselt` für eine entsperrte `secrets.txt` des alten Orts | sichert weiter verschlüsselt; die Sperren für Zwischenablage, Tastenprotokoll und Textmarke greifen weiter (`editormodell.rs:1164-1170`) |

**Die alten Zettel werden nicht doppelt übernommen**, solange die Übernahme an `ordner_angelegt && heim.ist_vorgabeort()` hängt (Plan, Zeile 176). Eine Restlücke besteht schon heute, unabhängig von dieser Arbeit. `note-1.txt` und `note-2.txt` bleiben nach der Übernahme im Ablageordner liegen; auf diesem Gerät stehen sie dort. Jedes F2, das `~/krkhome` neu anlegt, übernimmt sie erneut. Mit dem einstellbaren Ort wird dieser Fall wahrscheinlicher: Der Nutzer zieht nach Dropbox um, löscht `~/krkhome` und kehrt später zum Vorgabeort zurück. Das ist optional (O4); ein Merker in `session.toml` („Zettel übernommen“) schlösse es.

**Wo der alte Ort noch gilt, obwohl er es nicht soll: M2.** `Anwendungsdelegierter::neu` baut den Griff mit `Heimordner::des_benutzers()`, also mit `~/krkhome` (`crates/krk-ui/src/appkit/anwendung.rs:1410`). Der Plan behält das „für den Messmodus“ bei und setzt den eingestellten Ort erst „in `sitzung_laden` nach dem Durchgang“ (Plan, Zeile 211). `sitzung_laden` hat aber zwei Ausgänge vor dem Durchgang: das Öffnen der Ablage scheitert (`anwendung.rs:2013`), oder die Schreibsperre lässt sich nicht nehmen (`anwendung.rs:2114`). In beiden Fällen bleibt der Griff auf `~/krkhome`, und F2 legt dort an, obwohl der Nutzer einen anderen Ort eingestellt hat. Genau diesen stillen Rückfall schließt der Spec aus (H2, Kriterium 3, und „Decisions made“, Zeile 117). Die Korrektur ist eine Bauform und keine Sonderregel: `sitzung_laden` liefert den `Notizort` als dritten Rückgabewert auf **jedem** Weg. Auf den zwei frühen Ausgängen ist er `Err`, etwa `Ortsfehler::EinstellungenUngelesen`. Allein die Messaufgaben behalten den Vorgabeort. Eine Quelltextprobe hält fest, dass jeder `return` in `sitzung_laden` einen Notizort trägt.

**Wo der neue Ort noch nicht gilt:** Tabs auf einer dritten Schreibweise und, für Orte außerhalb des Benutzerverzeichnisses, die aufgelöste Form bis zum ersten F2. Beides ist benannt und hingenommen (Plan, Zeilen 393 und 399), und `secrets.txt` ist über die Inode geschützt. Dazu kommt S1 aus Frage 1.

### 4. `NSOpenPanel` als Blatt

**Blatt statt `runModal` ist richtig.** Die Begründung des Plans stimmt: `runModal` hielte die Zeitgeber an, das Blatt kehrt sofort zurück. Die Blattsperre greift über `attachedSheet`. Ein abgewiesener Befehl geht unverändert an AppKit weiter (`crates/krk-ui/src/appkit/ereignisse.rs:672-693` und Modulkopf), so dass Pfeiltasten und Tippen den Dialog erreichen. Das Schlüsselfenster zählt als `BlattAmHauptfenster` (`anwendung.rs:3717-3733`). KRK läuft außerhalb der Sandbox, also braucht es keine sicherheitsbezogenen Lesezeichen.

**M1: `esc` wird geschluckt, wenn ein alter Blattgriff im Schlitz liegt.** Der Plan stützt sich darauf, dass `abbrechen` die Taste an AppKit zurückgibt, wenn der Griff des stehenden Blattes nicht im Schlitz liegt (Plan, Zeilen 28 und 38). Der Code tut das nur bei **leerem** Schlitz. Steht ein Blatt, nimmt `abbrechen` jeden Griff aus dem Schlitz, ruft `blatt.abbrechen()` und meldet `true` (`anwendung.rs:6666-6673`). Die fünf Eingabeblätter melden ihr Schließen nicht, und ihr Griff bleibt bis zum nächsten Blatt liegen (`anwendung.rs:3691-3697`). `Blattgriff::abbrechen` beendet das Blatt **seiner eigenen** Warnung (`crates/krk-ui/src/appkit/blaetter/mod.rs:674-677`), und dieses Blatt steht nicht mehr. Nach einer Umbenennung oder einer Pfadeingabe, gefolgt von „Ort wählen…“, verbraucht `esc` deshalb die Taste und tut nichts, während der Dialog stehen bleibt. Das ist dieselbe Defektklasse, die der Kommentar in `abbrechen` für den 260907 beschreibt.

Wir empfehlen die Korrektur an der Wurzel und nicht im Befehl: `abbrechen` nimmt den Griff nur, wenn dessen Blatt das anhängende ist; sonst gibt es `false` zurück. Die Nämlichkeitsfrage steht schon im Baum, in `Blattgriff::verdeckt_und_steht` (`blaetter/mod.rs:660-666`). Sie gehört als `steht()` herausgelöst und in beiden Fällen gerufen. Damit hält jedes künftige Blatt ohne Griff dieselbe Regel. Das bloße Leeren des Schlitzes in `ort_waehlen` wäre die kleinere, aber punktuelle Lösung.

**Zwei Punkte gehören in die Nutzerprüfung und sind nicht belegt.** Erstens (**Spekulation:**) kann `NSOpenPanel` je nach macOS-Fassung außerhalb des Prozesses laufen. Dann meldet `keyWindow` womöglich nicht das Blatt, und die Tasten erreichen den lokalen Abgriff gar nicht. Beides wäre unschädlich, weil jeder Befehl dann an der Schlüsselfensterfrage scheitert. Zweitens (**Spekulation:**) wir haben nicht nachgelesen, ob `NSOpenPanel` einen gewählten Verweis auf einen Ordner selbst auflöst. Die Standardwerte von `resolvesAliases` sollten ausdrücklich gesetzt werden, und das Nutzerkriterium mit dem Verweis (Spec, Zeile 151) prüft das Ergebnis. Optional (O5): `directoryURL` auf einen Ort, dessen Laufwerk hängt, könnte das Öffnen des Dialogs verzögern.

### 5. Der Funktionsbereich „Home“

**Die Bauform trägt, und die Fallen, die `CLAUDE.md` nennt, sind im Plan abgedeckt.** `Funktionsbereich::ALLE` wird länger, die vollständigen Fallunterscheidungen halten den Bau an, und `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` hält die Liste. Die drei Pflichtstellen des neuen Kommandos samt Ausführungszweig stehen in 3.2a. Die Behauptung aus Schritt 1.1, dass das Verschieben des Eintragsblocks in keinem Bereich die Reihenfolge ändert, haben wir an den Kennungen der Zeilen 1104 bis 1389 von `resources/default-keymap.toml` nachgelesen; sie stimmt.

**M4: Die Anweisung „F1, `cmd+r`, Ansicht verlassen“ vor der Nutzerabnahme von Stufe 3 ist unnötig und schädlich.** `cmd+r` in der Belegungsansicht „setzt die ganze Belegung zurück“ (`HowTo.md:182`). Damit fielen die eigenen Tasten dieses Geräts weg. Der Handgriff ist auch nicht nötig. `Belegung::bauen` hängt jede Funktion, die die Nutzerdatei nicht nennt, unbelegt an (`crates/krk-core/src/tasten/belegung.rs:1846-1858`). Der Plan sagt das selbst (Zeile 272). „Ort wählen…“ erscheint also ohne jeden Handgriff im Menü „Home“. Der Satz gehört aus dem Spec (Zeilen 54 und 145) und aus dem Plan (Zeile 285) gestrichen.

**Die Reihenfolge im Menü folgt der Nutzerdatei.** `nach_bereichen` ordnet je Bereich in der Reihenfolge von `Belegung::funktionen` (`belegungsmodell.rs:933-980`), und die kommt bei einer eigenen Belegung aus deren Datei. Auf diesem Gerät steht `notizzettel` in der eigenen Datei, die übrigen acht Befehle fehlen dort und werden in der Folge der Auslieferung angehängt. Das Menü zeigt damit genau die Folge des Spec. Bei einer eigenen Datei, die die Eintragsbefehle vor `notizzettel` führt, sähe die Folge anders aus. Das ist richtig so, sollte aber im Kriterium H1.1 nicht als Zusage für jede Belegung stehen.

**Prosastellen mit Zahlen:** `crates/krk-ui/src/belegungsausgabe.rs:671` sagt „alle zehn“ Bereiche. Die Suche aus Schritt 1.2 (Plan, Zeile 152) findet nur Menünamen, keine Zahlwörter. Sie sollte um `zehn` erweitert werden (O7). Die Warnung des Plans zur Kamerakerbe bei elf Obermenüs halten wir für berechtigt.

### 6. Die Zeitzusagen L1 bis L10

**Keine Zusage wird neu berührt.** L1 (Tastendruck) läuft weiter über `Belegung::nachschlag`, und der Griff wird dort nicht gefragt. L3, L6 und L10 (Lesen) fragen je Lesevorgang einmal `ohne_inhaltsauftrag_fuer` (`tabs.rs:1399`); die Abschrift trägt künftig eine Zeichenkette mehr, und `ist` bleibt ein Textvergleich. L7 (Vorschau) behält die zwei `stat(2)` allein für `secrets.txt` (`vorschaumodell.rs:837`). L4 (Start) bekommt ein Obermenü mehr und ein Feld mehr in `session.toml`. Für einen Ort außerhalb des Benutzerverzeichnisses fallen die zwei Systemaufrufe am Start sogar weg. L4 steht laut `CLAUDE.md` seit dem 260910 ohnehin neben den Zusagen und wartet auf die spätere Messrunde. Die drei Proben aus dem Plan (Zeile 294) genügen als Nachweis. Die Korrektur M2 muss den Messmodus auf dem Vorgabeort lassen, sonst ändert sich, was L4 misst.

### 7. Der offene Entscheid zur beschädigten `settings.toml`

**Wir stimmen Möglichkeit 1 zu, mit zwei Schärfungen.** Die Begründung trägt: Ein Ersatzort ist nur harmlos, wenn der Nutzer nie einen anderen eingestellt hat, und das weiß KRK bei einer unlesbaren Datei nicht. Möglichkeit 3 führte eine zweite Quelle für den Ort ein, die der Nutzer der Datei nicht ansieht.

- **Schärfung zu M3: „beschädigt“ darf nicht „irgendeine Ersetzung“ heißen.** Der Plan übergibt `einstellungen_ersetzt` (Plan, Zeile 211; `anwendung.rs:2059`). Diese Angabe ist auch dann gesetzt, wenn `settings.toml` fehlte und sich nicht anlegen ließ (`Grund::NichtAnlegbar`, `crates/krk-core/src/ablage/mod.rs:289-305`; `einstellungen.rs:161-176`). Dann hat der Nutzer nachweislich keinen Ort eingestellt, und der Vorgabeort ist richtig. „Kein Ort“ gilt für `Grund::Beschaedigt` und `Grund::NichtLesbar` und, nach M2, für die zwei frühen Ausgänge des Starts.
- **Schärfung zum Wortlaut der Meldung:** Der Entscheid schlägt vor, der Notizordner gelte wieder, „wenn sie berichtigt ist“. Ein Berichtigen von Hand wirkt aber erst beim nächsten Start. Die Meldung sollte beide Wege nennen: `settings.toml` berichtigen und KRK neu starten, oder den Ort mit „Ort wählen…“ setzen. Der zweite Weg funktioniert im Betrieb, denn `notizordner_schreiben` liest die Datei unter der Sperre neu. Ist sie inzwischen berichtigt, schreibt der Befehl, und der Griff wird gültig.

Der Preis bleibt, wie der Entscheid ihn nennt: Ein Tippfehler an `terminal` legt F2 still, bis die Datei berichtigt ist. Wir halten ihn für tragbar, weil die Startmeldung den Schaden schon heute nennt und F2 ihn mit Grund wiederholt.

## Recommendations

**Vor dem Bau zu beheben** (Plan durch den `implementation-planner`, Spec durch den `requirements-designer`; beides Textarbeit, kein Datensatz nötig bis auf M3):

1. **M1**: `abbrechen` nimmt einen Griff nur, wenn dessen Blatt das anhängende ist. Die Nämlichkeitsfrage wird dazu aus `verdeckt_und_steht` als `Blattgriff::steht` herausgelöst. Eine Probe am Quelltext hält fest, dass der erste Rang von `abbrechen` sie fragt. In Schritt 3.2a aufnehmen.
2. **M2**: `sitzung_laden` liefert den `Notizort` auf jedem Weg. Die zwei frühen Ausgänge ergeben `Err`, allein die Messaufgaben den Vorgabeort. In Schritt 2.4 aufnehmen, samt Quelltextprobe über die Ausgänge.
3. **M3**: Der Entscheid `260926-1506_*_…` wird mit der Schärfung beantwortet: „kein Ort“ bei `Beschaedigt` und `NichtLesbar` und bei ungelesener Ablage, Vorgabeort bei `NichtAnlegbar`. Die Meldung nennt Neustart und „Ort wählen…“. Die Antwort gibt der Nutzer.
4. **M4**: Den Handgriff „F1, `cmd+r`“ aus Spec (Zeilen 54, 145) und Plan (Zeile 285) streichen; er setzt die eigene Belegung zurück.
5. **M5**: Spec H2 Kriterium 5 und die Begründung der Abweisung in H3 samt Zeile 158 nach Frage 1 berichtigen. Der Plan verlangt das ohnehin vor der jeweiligen Auslieferung (Zeile 293). Wir raten, es vor dem Bau zu tun, damit Spec und Code nicht auseinander gebaut werden.

**Vor Stufe 3 empfohlen, klein:**

- **S1**: `ort_uebernehmen` fragt `gehaltene_notizdatei` auch gegen den neuen `Heimordner`.
- **S2**: H2.3 als Dateischaden wie bei `terminal` fassen und `Option<toml::Spanned<String>>` statt `Ortswert` führen. Das ist nicht zwingend, aber kleiner.
- **S3**: Eine verknüpfte `settings.toml` nicht ersetzen, sondern mit Meldung abweisen. Damit ist die offene Frage im Plan (Zeile 404) beantwortet.
- **S4**: Den Vergleich „derselbe Ort“ unter der Sperre gegen den Wert in der Datei stellen.

**Optional:**

- **O1**: Im Modulkopf von `einstellungen.rs` festhalten, dass das Anhängen am Ende nur gilt, solange die Datei keine Tabelle kennt.
- **O2**: Beim Anhängen das Zeilenende der Datei übernehmen.
- **O3**: Den Ausschluss des Ablageordners ohne Rücksicht auf Groß- und Kleinschreibung vergleichen, wie `sonderdatei_genau` es beim Namen tut. Das Volume unterscheidet sie in der Regel nicht.
- **O4**: Die Übernahme der alten Zettel an einen Merker hängen statt an jedes Anlegen von `~/krkhome`.
- **O5**: `directoryURL` auf einen Ort außerhalb des Benutzerverzeichnisses mit Vorsicht setzen.
- **O6**: `resolvesAliases` ausdrücklich setzen.
- **O7**: Die Prosasuche aus Schritt 1.2 um Zahlwörter erweitern (`belegungsausgabe.rs:671`).
- **O8**: In `HowTo.md` sagen, dass eine zweite laufende Instanz bis zu ihrem Neustart am alten Ort anlegen kann.

## Open Questions

- [ ] Die Antwort des Nutzers auf `260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`; wir empfehlen Möglichkeit 1 mit der Schärfung aus M3.
- [ ] Ob `NSOpenPanel` auf dem Referenzgerät im Prozess läuft und einen gewählten Verweis auflöst, zeigt erst die Nutzerprüfung am Bündel.
- [ ] Ob elf Obermenüs auf dem Bildschirm des Nutzers Platz finden, ebenfalls Nutzerprüfung (Plan, Zeile 397).

## Sources

- `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`
- `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md`
- `260926-1447_*_bekommt-krkhome-ein-eigenes-menue-und-einen-einstellbaren-ort.md`, `260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`, `260926-0050_*_wie-weit-reicht-der-inhaltsfilter-liest-secrets-txt-nicht-wenn-das-kennzeichen-versteckt-ihn-nicht-haelt.md`, `260926-0007_*_wie-lange-gilt-eine-eingegebene-pin.md`
- `260926-1004_*_eine-textmarke-in-secrets-txt-schreibt-eine-klartextzeile-in-die-lesezeichendatei.md` (Schließungsvermerk)
- `260926-1047-schlussdurchsicht-f2-krkhome.md` (diese Ablage)
- Code: `crates/krk-core/src/heimordner/mod.rs`, `crates/krk-core/src/ablage/{einstellungen,mod,atomar,sperre,sitzung}.rs`, `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-ui/src/{heimgriff,editormodell,tabs,vorschaumodell,belegungsmodell,belegungsausgabe}.rs`, `crates/krk-ui/src/appkit/{anwendung,ereignisse}.rs`, `crates/krk-ui/src/appkit/blaetter/mod.rs`, `resources/default-keymap.toml`, `HowTo.md`
- Messung: Wegwerfprojekt `spanprobe` im Arbeitsverzeichnis dieser Sitzung, `toml` 1.1.4 offline, acht Eingaben (Tabelle unter Frage 2)
