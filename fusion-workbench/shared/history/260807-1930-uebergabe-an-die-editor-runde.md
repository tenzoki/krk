# Übergabe an die Editor-Runde

**Geschrieben:** 260807-1930 vom `orchestrator`, am Ende der Sitzung 260806-2257.
**Anlass:** Der Nutzer hat die Runde 1 abgeschlossen und als nächste Runde den **Editor** gewählt. Er startet die Sitzung neu; dieses Dokument ist, was die nächste nicht wieder herleiten muss.

---

## Der Stand in vier Sätzen

Die Runde 1 hat das Navigator-Gerüst gebaut und ist als beschränkter Abschluss geschlossen. Der Circle `260802-0842-krk-mac-dateimanager-editor-git` trägt `_b_` und ist damit endgültig; sein Plan und sein Spec stehen auf `_c_`. Der Editor gehört nach der ursprünglichen Directive in denselben Circle, aber ein terminaler Circle nimmt keine Arbeit mehr auf — die Editor-Runde braucht **einen neuen Circle**, der den geschlossenen über `## Dependencies` zitiert. Das Portfolio führt daneben einen vorgesehenen Circle, den Web-Betrachter; er ist **nicht** der gewählte nächste Schritt.

## Was der Editor laut Directive ist

Aus dem Circle-Datensatz der Runde 1, Abschnitt `## Directive`:

> Der eingebaute Editor öffnet Text, Code und Markdown in einer Rohansicht und einer Formatansicht, springt zu einer Zeilennummer, sucht und ersetzt innerhalb der geöffneten Datei und speichert Marken auf Textstellen und Textbereiche als Lesezeichen im Home-Verzeichnis des Nutzers.

Der Spec der Runde 1 grenzt ihn ausdrücklich aus (`## Ausdrücklich außerhalb dieser Runde`): „Der eingebaute Editor bleibt draußen. Dazu zählen die Rohansicht und die Formatansicht, der Sprung zu einer Zeilennummer, das Suchen und Ersetzen innerhalb der geöffneten Datei sowie das Speichern von Textmarken auf Stellen und Bereiche als Lesezeichen im Benutzerverzeichnis."

**Suchen und Ersetzen über mehrere Dateien gehört nicht dazu.** Der Shaper hat es am 260802 als eigenes Vorhaben abgegrenzt; innerhalb der geöffneten Datei bleibt es beim Editor.

## Was die Runde 1 dem Editor hinterlässt

**Eine Taste ist für ihn freigehalten.** F4 ist unbelegt und in der Belegungsansicht als für den Editor reserviert gekennzeichnet. Die Norton-Bedeutung von F4 ist „Bearbeiten". Ein Cmd-Kürzel trägt die Funktion in Runde 1 ebenfalls nicht. Der Spec begründet das: eine Belegung mit dem Systemeditor wäre ein Behelf, den die spätere Runde wieder entfernen müsste.

**Die Anzeige steht, die Bearbeitung fehlt.** Das Vorschaufenster zeigt Text und Markdown bis 1 MB als rohen Inhalt ohne Formatierung, Bilder bis 64 MB, alles andere als Metadaten. Es hat eigene Tabs, ist seit dem 260807 über `shift+cmd+y` mit der Tastatur erreichbar, und die vier Tabbefehle aus C1 bedienen es, sobald der Fokus dort steht. Der Editor wird sich fragen müssen, ob er in dieser Fläche wohnt oder daneben.

**Der Fokus kennt drei Bereiche.** Lesezeichenleiste, Dateifenster, Vorschau. Ein Fokusbefehl blendet seinen Bereich seit dem 260807 ein, wenn er ausgeblendet war, und blendet nie aus. Ein vierter Bereich fügt sich in dieses Muster ein, ohne es zu ändern.

**Die Statuszeile trägt fünf Ränge** nach dem Alter der Aussage. Ein Editor, der etwas zu melden hat, reiht sich dort ein und baut keine zweite Zeile daneben.

## Was den Editor bindet

**Eine offene Entscheidung gehört ihm direkt:**

- `shared/decisions/260802-0842_o_editor-formatansicht-je-dateityp.md` — was die Formatansicht bei Text, bei Code und bei Markdown zeigt. Der Spec der Runde 1 hält fest, dass sie diese Runde nicht bindet; die Editor-Runde bindet sie sehr wohl, und zwar vor dem ersten Schritt.

**Eine weitere könnte ihn treffen, je nach Zuschnitt:**

- `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` — wie KRK aus Rust eine Schnittstelle anspricht, die es erst ab macOS 26 gibt. Bindend für die Runde, die eine solche anspricht. Ein Texteditor über `NSTextView` kommt vermutlich ohne aus; das ist zu prüfen und nicht anzunehmen.

**Zwei Fragen aus der Runde 1 hängen an der Lesestelle und sollten vor größeren Eingriffen beantwortet sein:**

- `…/decisions/260807-0010_o_kann-der-auffrischungsaufschub-entfallen-nachdem-die-lesestelle-nicht-mehr-vorab-leert.md`
- `…/decisions/260807-0020_o_soll-die-markierung-eine-auffrischung-ueberleben.md`

## Was offen liegen bleibt und den Editor nicht aufhält

**Die Ursache der L9-Verschlechterung.** `shared/issues/260807-1748_o_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`. Der Nutzer hat die Zusage am 260807-1900 auf 65 Prozent gesenkt, statt die Ursache zu suchen, und die Kosten dieser Wahl stehen im Datensatz `shared/decisions/260807-1904_i_l9-verfehlt-auch-die-gesenkte-schwelle-wie-weiter.md`. **Der Defekt ist ausdrücklich nicht geschlossen.** Wer die Kollation, die Auffrischung während einer Kopie oder das Zeichnen markierter Zeilen anfasst, sollte ihn zuerst lesen: der Hauptverdächtige `16e4558` sitzt genau dort.

Der nächste Schritt an diesem Defekt ist keine Reparatur, sondern eine Messgröße, die die beiden Unterschiede zwischen L1 und L9 trennt — die laufende Kopie und die vollständige Markierung aller 10.000 Einträge. Das ändert die Messstrecke, und die ist der Maßstab; deshalb gehört es an den Anfang einer Sitzung.

**Vier weitere offene Defekte** stehen in beiden Speichern, alle mit geringer Dringlichkeit. Verbindlich ist der Dateibestand.

## Wie die nächste Sitzung anfängt

Der Circle für die Editor-Runde existiert noch nicht. Ihn anzulegen ist Sache des Nutzers über `/fusion:direct`; der `shaper` klärt daraus die Directive und schreibt einen vorgesehenen Circle. Ein Entwurf, der die Directive-Formulierung von oben aufnimmt und die drei Punkte trägt, die diese Übergabe als bindend nennt, spart die erste Klärungsrunde.

**Vor dem ersten Planschritt gehört die Formatansicht-Frage beantwortet.** Sie entscheidet den Zuschnitt: ob der Editor eine Formatansicht je Dateityp baut oder eine gemeinsame, und ob Markdown gerendert oder nur hervorgehoben wird. Ohne sie plant der Planner ins Ungefähre.
