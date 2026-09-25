# Coder: zwei mittlere Defekte — die unbegrenzte Kopie und acht falsche Verweise

**Date:** 2026-08-14
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260813-2332-notizzettel-als-blatt-mit-zwei-zetteln/`
**Vorgabe:** Spec `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md`, C5; Nutzerantwort vom 260814-1010
**Behobene Datensätze:** `issues/260814-0910_c_eine-zetteldatei-ueber-editorgrenze-wird-unbegrenzt-auf-dem-hauptfaden-kopiert.md` (mittel), `issues/260814-0911_c_acht-verweise-in-editor-rs-schicken-zu-textflaeche-bauen-wo-die-automatikzeilen-nicht-mehr-stehen.md` (mittel)
**Verification:** `make check` am 260814, Rückgabewert 0, „alle vier gruen"

---

## Defekt 1: die Kopie hält jetzt bei der Grenze an

**Der Spec ist zuerst nachgezogen worden, und das war die eigentliche Behebung.** C5
verlangte, eine Zetteldatei über `EDITORGRENZE` gehe „denselben Weg beiseite", und sagte
über die Größe der Kopie nichts. Der Bau tat genau das Verlangte: die Schranke stand über
dem Laden, `Zugang::beiseite_legen` reichte den offenen Deskriptor unbegrenzt an
`atomar::schreiben` und von dort an `io::copy`. Eine Datei von 40 GB unter dem Namen
`note-1.txt` wurde bei jedem `f2` vollständig kopiert, synchron auf dem Hauptfaden und
unter dem gehaltenen Schreibgriff.

Nachgezogen sind fünf Stellen im Spec: drei Kriterien und drei Festlegungen in C5, das
zweite Kriterium unter „Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1", und der
Abschnitt „Was der Nachtrag vom 260814-1010 an C5 geändert hat" am Ende. Der Kopf des Spec
trägt die dritte Nachtragszeile.

**Im Baum sind es vier Änderungen.**

1. **Die Kopie ist begrenzt.** `beiseite_legen` (`crates/krk-core/src/ablage/mod.rs`)
   schreibt aus `quelle.by_ref().take(EDITORGRENZE)`. Es ist die Zahl aus
   `krk_core::text::datei::EDITORGRENZE` und keine zweite daneben; der Baum führt sie
   unverändert an genau einer Stelle. Der Preis steht im Doc-Kommentar, wie der Nutzer es
   verlangt hat: von einer sehr großen Fremddatei werden nur die ersten 16 MB gesichert.
2. **`Beiseite` hat einen fünften Wert, `Gekuerzt(PathBuf)`.** Ohne ihn wäre die Kürzung
   für den Nutzer unsichtbar: auf der Platte sieht eine gekürzte Sicherung aus wie eine
   vollständige, und beim nächsten Start bleibt sie als die ältere Fassung stehen. Die
   Meldung nennt die Grenze. Die Fallunterscheidung bleibt vollständig und ohne
   Auffangzweig — der Übersetzer hat die nachzuziehenden Stellen selbst genannt.
3. **Gekürzt und vollständig werden an einem Byte hinter dem Budget unterschieden**, nicht
   am erschöpften Budget: eine Datei von genau `EDITORGRENZE` Bytes schöpft es restlos aus
   und ist trotzdem ganz gesichert. `steht_noch_etwas_an` liest dafür ein Byte und nicht
   mehr; ein Lesefehler wird zur vorsichtigen Seite hin als „ja" beantwortet, mit dem Grund
   am Doc-Kommentar.
4. **`atomar` hat keine Obergrenze bekommen.** Das Modul schreibt, was ihm gereicht wird;
   die Schranke sitzt beim Aufrufer, der weiß, wofür er kopiert. Modulkopf und
   `vorbereiten` sagen das jetzt aus, damit die Grenze beim nächsten Umbau nicht dort
   gesucht wird. Ein zweiter Schreibweg ist nicht entstanden.

**Proben.** `eine_zu_grosse_zetteldatei_wird_nicht_geladen_und_geht_gekuerzt_beiseite`
(`crates/krk-core/tests/ablage.rs`) hält fest, dass die Sicherung bei der Grenze anhält,
das Original unangetastet in voller Länge dasteht und die Meldung die Kürzung nennt; ihre
Längenzusicherung ist gegenüber Turn 1 umgekehrt und ihr Name mitgezogen. Neu daneben steht
`eine_zetteldatei_genau_auf_der_grenze_geht_ganz_beiseite` als Grenzfall. Die Anzeigeprobe
heißt jetzt `die_meldung_unterscheidet_die_fuenf_lagen_und_bleibt_einzeilig` und prüft alle
fünf Werte, darunter, dass die drei Sätze mit Pfad paarweise verschieden sind.

## Defekt 2: acht Verweise zeigen wieder auf die richtige Stelle

Die acht Stellen in `crates/krk-ui/src/appkit/editor.rs` nennen jetzt
`textautomatik::automatiken_abschalten`, wohin Strang C die neun Zeilen gezogen hat. Die
sieben Stellen, an denen `textflaeche_bauen` als **Erzeuger der Fläche** gemeint ist, sind
unangetastet: dort stimmt der Verweis weiter.

**Die teure Stelle war die Handlungsanweisung bei 4854.** Sie trägt jetzt neben dem
richtigen Namen auch die Begründung: eine Zeile in `textflaeche_bauen` gehörte allein dem
Editor, und der Zettel stünde ohne sie da — die zwei Wahrheiten darüber, was „abgeschaltet"
heißt. Der bloße Austausch des Namens hätte den nächsten Bauer richtig geschickt und ihm
nicht gesagt, warum.

Verhalten ändert sich dabei nicht; die Änderung ist Prosa. Dass die Verweise auflösen, ist
mit `cargo doc -p krk-ui --document-private-items` nachgesehen: keine der acht steht in der
Liste der unaufgelösten Verweise.

## Was ausdrücklich unangetastet geblieben ist

`immer_erreichbar`, `waehrend_blatt_erlaubt`, `zulaessigkeit::zulaessig`, die eine Erklärung
mit ihren vier Aufrufern und der eine `durchgang` beim Beenden. Die vierzehn übrigen offenen
Defekte des Circles sind nicht angefasst, darunter der zur Zeilenangabe `datei.rs:153` in
C5: die zitierte Zeilennummer steht unverändert in dem Kriterium, das dieser Auftrag
ergänzt hat.

## Verifikation

`make check` gefahren, Rückgabewert 0, Ausgabe „alle vier gruen" — `cargo build`,
`cargo test`, `cargo fmt --check` und `cargo clippy --workspace --all-targets -- -D warnings`
in einem Zug. Kein Bündelbau, wie im Auftrag verlangt.
