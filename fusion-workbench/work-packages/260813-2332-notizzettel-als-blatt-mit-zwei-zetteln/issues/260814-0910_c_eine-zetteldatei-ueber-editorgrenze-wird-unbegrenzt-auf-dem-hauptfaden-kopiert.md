Eine Zetteldatei über EDITORGRENZE wird unbegrenzt auf dem Hauptfaden kopiert

---

Der Spec sagt im Abschnitt zum Verhältnis zu den zehn Zeitzusagen zu: „Der Zettel liest und
schreibt seine Datei auf dem Hauptfaden, und die obere Schranke dafür ist `EDITORGRENZE`
mit 16 MB. Eine Datei darüber wird nicht geladen, sondern beiseitegelegt."

Die Schranke gilt für das **Laden**. Für das Beiseitelegen gilt sie nicht:
`Zugang::text_laden` (`crates/krk-core/src/ablage/mod.rs:564`, Zweig `:595`) reicht im Zweig
`Textstand::Unlesbar` den offenen Deskriptor an `beiseite_legen` weiter, das über
`atomar::schreiben` in `io::copy(quelle, &mut datei)` mündet
(`crates/krk-core/src/ablage/atomar.rs:156`) — ohne `take`, ohne Obergrenze. Eine Datei von
40 GB unter dem Namen `note-1.txt` wird bei jedem `f2` vollständig kopiert, synchron auf
dem Hauptfaden und unter dem gehaltenen Schreibgriff.

---

**Schwere:** mittel. Die Oberfläche steht für die Dauer der Kopie, eine zweite Instanz von
KRK wartet an der Sperre, und der Ablageordner wächst um die Größe der Fremddatei.

**Erreichbar ist der Fall genau so, wie C5 ihn beschreibt.** Der Spec lädt den Nutzer
ausdrücklich ein, die Zetteldateien mit fremden Programmen zu öffnen und zu ändern; die
Grenze steht dafür da, „den Fall abzufangen, in dem eine fremde Datei unter dem Namen eines
Zettels liegt". Genau dieser Fall löst die Kopie aus.

**Der Editor tut das nicht.** `text::datei::oeffnen` lässt den Deskriptor im Fall `ZuGross`
fallen (`crates/krk-core/src/text/datei.rs:498`) und kopiert nichts. Das Verhalten ist mit
dieser Runde neu.

**Es ist zuerst eine Lücke im Spec und dann eine im Bau.** C5 verlangt „Eine Zetteldatei
über `EDITORGRENZE` wird nicht geladen und geht denselben Weg beiseite", und der Bau tut
genau das. Was weder Spec noch Plan festlegen, ist, wie groß „beiseite" werden darf. Die
Frage gehört vor eine Behebung: eine Obergrenze auch für die Kopie, ein Verzicht auf die
Kopie oberhalb einer zweiten Zahl, oder ein Umbenennen statt eines Kopierens — Letzteres
verstieße gegen die erste der drei Regeln von `beiseite_legen`.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Die Probe `eine_zu_grosse_zetteldatei_wird_nicht_geladen_und_geht_beiseite`
  (`crates/krk-core/tests/ablage.rs`) misst den Fall an `EDITORGRENZE + 1` und hält die
  Vollständigkeit der Kopie ausdrücklich fest; sie sagt über die obere Grenze nichts.
- Der Modulkopf von `ablage/mod.rs` schreibt die Eigenschaft mit aus: „Sie wird dabei aus
  ihrem offenen Deskriptor kopiert und steht zu keinem Zeitpunkt vollständig im
  Arbeitsspeicher." Das trifft zu und beantwortet die Frage nach der Zeit nicht.

---
Resolved: Dieselbe Zahl gilt jetzt für die Kopie. `Zugang::beiseite_legen`
(`crates/krk-core/src/ablage/mod.rs`) reicht die Quelle nicht mehr unbegrenzt an
`atomar::schreiben`, sondern als `take(EDITORGRENZE)`. Kopiert werden damit höchstens
16 MB, gleich wie groß die Fremddatei ist, und der Ablageordner wächst höchstens um diese
16 MB je Zettel. Es ist die Zahl aus `krk_core::text::datei::EDITORGRENZE` und keine zweite
daneben; der Baum führt sie unverändert an genau einer Stelle.

**Die Lücke im Spec ist zuerst geschlossen worden, so wie der Datensatz es verlangt.** C5
trägt jetzt drei Kriterien mehr (Grenze für die Kopie, unterscheidbare Meldung, Grenzfall)
und drei Festlegungen mehr, das zweite Kriterium unter „Verhältnis zu den zehn Zeitzusagen
aus C8 der Runde 1" nennt die Schranke für beide Richtungen, und der Anlass steht am Ende
des Spec unter „Was der Nachtrag vom 260814-1010 an C5 geändert hat". Grundlage ist die
Nutzerantwort vom 260814-1010: eine Zahl, zwei Verwendungen; von einer sehr großen
Fremddatei werden nur die ersten 16 MB gesichert. Der Preis steht im Spec und im
Doc-Kommentar von `beiseite_legen`. Verworfen sind eine zweite, größere Zahl und ein
Umbenennen statt eines Kopierens.

**`Beiseite` hat einen fünften Wert bekommen, `Gekuerzt(PathBuf)`.** Er ist nötig, weil
eine gekürzte Sicherung auf der Platte aussieht wie eine vollständige und beim nächsten
Start als die ältere Fassung stehen bleibt; die Meldung ist die einzige Stelle, an der der
Nutzer den Unterschied erfährt, und sie nennt die Grenze. Die Fallunterscheidung bleibt
vollständig und ohne Auffangzweig — der Übersetzer hat die Stellen selbst genannt, die
nachzuziehen waren.

**Gekürzt und vollständig werden an einem einzelnen Byte hinter dem Budget unterschieden
und nicht am erschöpften Budget selbst.** Eine Datei von genau `EDITORGRENZE` Bytes schöpft
es restlos aus und ist trotzdem ganz gesichert; `steht_noch_etwas_an` stellt die Frage und
liest dafür ein Byte und nicht mehr. Ein Lesefehler dort wird als „ja" beantwortet, also
zur vorsichtigen Seite hin, und der Grund steht am Doc-Kommentar.

**`atomar` hat keine Obergrenze bekommen und soll keine haben.** Das Modul schreibt, was
ihm gereicht wird; die Schranke sitzt beim Aufrufer, der weiß, wofür er kopiert. Der
Modulkopf und `vorbereiten` sagen das jetzt aus, damit die Grenze beim nächsten Umbau nicht
dort gesucht wird. Ein zweiter Schreibweg entsteht nicht.

Zwei Proben in `crates/krk-core/tests/ablage.rs` halten es fest:
`eine_zu_grosse_zetteldatei_wird_nicht_geladen_und_geht_gekuerzt_beiseite` (Sicherung hält
bei der Grenze an, Original unangetastet in voller Länge, Meldung nennt die Kürzung) und
`eine_zetteldatei_genau_auf_der_grenze_geht_ganz_beiseite` als Grenzfall. Die Anzeigeprobe
heißt jetzt `die_meldung_unterscheidet_die_fuenf_lagen_und_bleibt_einzeilig` und prüft alle
fünf Werte.

`make check` am 260814 gefahren, Rückgabewert 0, „alle vier gruen".

---

**Abgleich 260814-1247: die Behebung ist gegen den Baum gelesen und hält.**
`quelle.by_ref().take(EDITORGRENZE)` steht an `crates/krk-core/src/ablage/mod.rs:720`,
`Beiseite::Gekuerzt` an `:261` mit eigener Meldung an `:320`, und die Unterscheidung läuft über
`begrenzt.limit() > 0` und `steht_noch_etwas_an` und nicht über das erschöpfte Budget. Die zwei
Proben stehen an `crates/krk-core/tests/ablage.rs:1587` und `:1644`. `EDITORGRENZE` steht
weiterhin an genau einer Stelle im Quelltext (`crates/krk-core/src/text/datei.rs:164`); jede
weitere Fundstelle liest sie. `make check` beim Abgleich wiederholt: Rückgabewert 0, „alle vier
gruen". Die drei Kriterien, die der Spec dafür am 260814-1010 nachgetragen hat, halten alle
drei.
