Ein Markensprung kann zwei Meldungen zugleich haben, und die Zeile trägt eine

---

`krk_core::text::Markensprung` trägt zwei **verschiedene** Auskünfte, und der Modulkopf
von `crates/krk-core/src/text/marke.rs:93-98` sagt ausdrücklich, dass der Aufrufer
beides zu melden hat:

- `Markensprung::fund` — ob der gemerkte Zeileninhalt wiedergefunden wurde.
  `Fund::NichtGefunden` verlangt nach C6, achtes Abnahmekriterium, die Meldung „die
  Stelle hat sich geändert".
- `Markensprung::sprung.lage` — ob die angesteuerte Zeilennummer im Text überhaupt
  vorkommt. `Zeilenlage::HinterDerLetzten` verlangt nach C5 die Meldung, dass die
  Nummer über der Zeilenzahl liegt.

Das Beispiel steht in `marke.rs:96-98`: eine Marke auf Zeile 500 einer inzwischen auf
100 Zeilen gekürzten Datei trägt beide.

**Die Statuszeile trägt einen Text.** Rang 1 hält eine Zeichenkette, und S21 hat die
Meldungen des Editors dort eingereiht, statt eine zweite Fläche daneben zu bauen. Zwei
Meldungen zugleich passen deshalb nicht hinein: eine der beiden fällt weg, und welche,
ist heute nirgends festgelegt.

**Was S21 gebaut hat und was nicht.** `Editormeldung::markenstelle` in
`crates/krk-ui/src/appkit/editor.rs` beantwortet die **erste** Hälfte, also den Fund,
mit einer vollständigen Fallunterscheidung über `Fund`. Die zweite Hälfte hat heute
keinen Auslöser: die Meldung der Zeilenlage gehört zum Zeilensprung aus C5 und kommt
mit S35. Solange nur eine Hälfte gebaut ist, kollidiert nichts, und der Doc-Kommentar
an `markenstelle` benennt die offene Hälfte samt Verweis auf diesen Datensatz.

**Wo es auffällt.** S39 baut den Sprung auf eine Textmarke und ist der erste und
einzige Aufrufer, bei dem beide Auskünfte an einem Wert hängen. Spätestens dort ist zu
entscheiden.

**Vorschlag.** Ein Satz für den zusammengesetzten Fall, kein Vorrang zwischen zwei
Sätzen. Zwei Meldungen, von denen eine gewinnt, wären eine Vorrangregel neben der
bestehenden aus `statuszeile::zeile`, und die trägt fünf Ränge nach dem Alter der
Aussage und nicht nach dem Gewicht eines Grundes. Der zusammengesetzte Fall ist
stattdessen ein eigener Zustand des Sprungs: der gemerkte Inhalt ist fort **und** die
Datei ist kürzer als die gemerkte Nummer. Er ist als Variante von `Editormeldung`
darstellbar und bekommt dann einen Satz, der beides sagt.

Die Alternative wäre, S35 und S39 je eine Meldung bauen zu lassen und die zweite
kommentarlos fallen zu lassen. Das widerspricht dem achten Abnahmekriterium von C6 und
dem von C5, die beide „kommentarlos nichts zu tun" ausschließen.

Gemeldet von: `coder`, bei der Umsetzung von S21.

---

Resolved am 260810-0204 bei der Umsetzung von S39, dem im Befund benannten
ersten und einzigen Aufrufer.

**Der Vorschlag des Befundes hat gehalten, seine Begründung ist stärker
geworden.** Der Befund schlug einen Satz für den zusammengesetzten Fall vor,
statt einer zweiten Vorrangregel, und begründete das damit, dass die bestehende
Vorrangregel aus `statuszeile::zeile` nach dem Alter einer Aussage ordnet und
nicht nach dem Gewicht eines Grundes. Das trägt. Beim Bauen kam ein zweiter,
schärferer Grund dazu: **die beiden Auskünfte sind nicht unabhängig.**

`krk_core::text::marke::wiederfinden` liefert `Fund::Getroffen` und
`Fund::Verschoben` allein für eine Nummer, deren Zeile es im heutigen Text gibt
— `Zeilenindex::inhalt_der_zeile` beantwortet jede andere mit `None`, und der
gelieferte Sprung entsteht aus derselben Nummer. Daraus folgt: eine von
`Zeilenlage::Getroffen` verschiedene Lage kommt **nur** zusammen mit
`Fund::NichtGefunden` vor. Von den neun Paarungen der beiden Aufzählungen sind
fünf erreichbar, und die zweite Auskunft kann nie für sich stehen.

Eine Vorrangregel wäre damit nicht nur unnötig, sondern falsch: sie täte so, als
könnten beide Meldungen einzeln auftreten und als müsste man wählen. Ein dritter
Wert für den zusammengesetzten Fall wäre der zweite Weg zu demselben Sachverhalt.

**Gebaut ist ein Wert mit zwei Feldern.** `Editormeldung::MarkenstelleGeaendert`
trägt neben der Zeilennummer die `Zeilenlage`; `Editormeldung::markenstelle`
setzt beide, und `Editormeldung::text` baut daraus einen Satz mit einem
gemeinsamen Anfang und drei Enden:

| Lage | Satz |
|---|---|
| `Getroffen` | „die gemerkte Stelle hat sich geändert; die Marke führt auf Zeile 118“ |
| `HinterDerLetzten` | „die gemerkte Stelle hat sich geändert; die Datei hat keine Zeile 500 mehr; die Schreibmarke steht am Dateiende“ |
| `VorDerErsten` | „die gemerkte Stelle hat sich geändert; Zeilen zählen ab 1; die Schreibmarke steht am Dateianfang“ |

Der Fund entscheidet, **ob** gemeldet wird, die Lage, **wohin** die Schreibmarke
gekommen ist. Die Fallunterscheidung über die Lage ist vollständig und hat keinen
Auffangzweig.

**Die dritte Zeile ist keine Vorsorge.** `Zeilenlage::VorDerErsten` entsteht aus
einer gemerkten Nummer 0, und die kommt aus keinem Anlegen — wohl aber aus einer
von Hand geänderten `bookmarks.toml`, wie die Probe
`eine_gemerkte_nummer_null_fuehrt_an_den_textanfang_und_sucht_trotzdem` in
`krk-core` es schon festhält.

Drei neue Proben in `crates/krk-ui/src/appkit/editor.rs` halten es:
`eine_marke_auf_eine_gekuerzte_datei_meldet_beide_auskuenfte_in_einem_satz`
(genau das Beispiel aus `marke.rs:96-98`),
`eine_gemerkte_nummer_null_meldet_den_dateianfang` und
`die_drei_lagen_des_markensprungs_tragen_drei_verschiedene_saetze`.

**Die zweite Hälfte des Befundes bleibt, wo sie war.** Die Meldung der
Zeilenlage beim Zeilensprung aus C5 (`Editormeldung::ZeileVorDerErsten` und
`ZeileHinterDerLetzten`, seit S35) steht unverändert daneben und teilt sich mit
dieser keinen Rang: sie ist die Antwort auf `cmd+j` und kommt nie im selben
Tastendruck wie ein Markensprung.
