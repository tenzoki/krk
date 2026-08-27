# Planner: Die Vorschau zählt den Ordnerinhalt im eingebauten Default-Profil

**Datum:** 2026-08-27, 13:22
**Agent:** planner
**Circle:** 260827-0310-vorschau-zaehlt-ordnerinhalt-im-default-profil
**Auftrag:** Implementierungsplan gegen den freigegebenen Spec `planning/260827-0646_*_spec-vorschau-zaehlt-ordnerinhalt-im-default-profil.md`
**Executor-Menge im Dispatch:** coder, ontocoder, analyst

## Was entstanden ist

- `planning/260827-1322_o_plan-vorschau-zaehlt-ordnerinhalt-im-default-profil.md` — acht Schritte, davon fünf `coder`, einer `ontocoder`, einer `analyst` und einer Nutzerarbeit.
- `decisions/260827-1322_o_faellt-das-default-profil-auch-im-messmodus-an-und-was-misst-l7-danach.md` — beim Lesen der Messstrecke aufgefallen, drei Möglichkeiten mit ihren Downstream-Folgen.

## Gelesene Grundlage

Spec und Circle-Datensatz dieser Runde, die zwei beantworteten Entscheidungsdatensätze vom 260827-0629, `CLAUDE.md`, und im Baum: das ganze Modul `crates/krk-core/src/leseprofil/` (`mod.rs`, `bausteine.rs`, `datei.rs`, `erkennung.rs`), `crates/krk-core/src/verzeichnis/{eintrag.rs, leser.rs, modell.rs}`, `crates/krk-core/src/ablage/leseprofile.rs`, `crates/krk-ui/src/vorschaumodell.rs`, `crates/krk-ui/src/appkit/vorschau.rs`, die einschlägigen Stellen in `appkit/anwendung.rs` und `appkit/tabelle.rs`, `crates/krk-ui/src/messmodus.rs`, `crates/krk-bench/src/fixture.rs`, `resources/default-readers.toml` und die Umrisse von `crates/krk-core/tests/{leseprofil.rs, baum.rs, gemeinsam/mod.rs}`. Daneben die offenen und beantworteten Entscheidungsdatensätze beider Speicher, insbesondere `shared/decisions/260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`.

## Die sechs Entscheidungen aus `## Open for Planner`

1. **Ort und Gestalt des Default-Profils:** fester `Profil`-Wert in einer `LazyLock` im neuen Modul `crates/krk-core/src/leseprofil/defaultprofil.rs`, gefragt in `bausteine::zusammenfassen_gezaehlt`, nachdem `erkennen` leer zurückgekommen ist. Ein Zweig im Vorschaumodell fällt weg, weil die Zählmaschine im Kern wohnt und C4.5 Proben ohne Fenster verlangt.
2. **Wo die drei Zeilen an die sechs treten:** in der Ansicht, in `Vorschau::metadaten_text`. Sie ist die einzige Stelle, an der beide Hälften zugleich vorliegen — die sechs brauchen AppKit-Formatierer, die drei brauchen den Lesestand des Kerns. Aus `Zusammenfassung::als_text` wird `zeilen_als_text` herausgezogen, damit es bei einer Formatierungsstelle bleibt.
3. **Gestalt des Wertes mit Klammer:** siebter Wert `Wert::ZahlMitVersteckten { zahl, versteckt }`, `als_text` bleibt vollständig ohne Auffangzweig. `Wert::UeberGrenze` bleibt unverändert und trägt den Fall über der Schranke allein; damit entfällt die Klammer dort ohne zweite Regel.
4. **Wie die drei Zeilen sich den Leselauf teilen:** gar nicht neu. Alle drei tragen `Ortsangabe::wurzel`, lösen auf denselben `Ort::Einer` auf, und `Lauf::stand_am` merkt die Lesung. C4.2 fällt aus derselben Merkstelle, weil der Erkennungsdurchgang durch sie geht.
5. **Schreibweise der TOML-Werte:** `typ` mit `datei`, `ordner`, `verknuepfung` in Umschrift, `versteckt` als Wahrheitswert. Die offene Frage 260826-1225 hält das nicht auf: ihr Gegenstand ist nutzersichtbare Prosa, und ihre eigene Naht ordnet ein Schlüsselwort dem Übersetzer zu. Die drei Beschriftungen der Zeilen sind Prosa und tragen Umlaute.
6. **Buchung der Berührung von C2.5 der Runde 16:** Defektdatensatz in `issues/` dieses Circles (Herkunftsregel), Schritt 7, Executor `analyst`. Der fremde Spec wird zitiert und nicht angefasst.

## Befunde neben dem Auftrag

- **Der Messmodus umgeht das Default-Profil nicht.** Der Doc-Kommentar von `Anwendungsdelegierter::sitzung_laden` behauptet, ein leerer Profilsatz schalte die Auswertung ab und deshalb messe keine der zehn Zusagen an einer Zusammenfassung. Nach dieser Runde trägt der Schluss nicht mehr, weil das Default-Profil aus keiner Ablagedatei kommt. Die L7-Reihe wählt im Prüfordner auch Unterordner aus und misst danach einen Verzeichnisleselauf mit. Schritt 4 berichtigt den Kommentar, der Datensatz oben legt die Frage vor.
- **Zehn Stellen halten die Vollständigkeit der Aufzählung `Baustein`**, am 260827 einzeln nachgezählt und im Plan unter `## Testing Strategy` aufgeführt. Die Prosazahl „sieben" aus dem Constraints-Abschnitt des Entscheidungsdatensatzes vom 260827-0311 stimmt nicht; der Spec hat genau davor gewarnt und die Nachzählung dem Plan aufgetragen. Kein eigener Defektdatensatz: die Zahl steht in einem Datensatz, der seinen damaligen Stand aufbewahrt, und die Ortsregel lässt ihn stehen.
- **C3.7 lässt sich mit einer einfachen Nadel nicht halten.** `.versteckt` steht in Code-Zeilen von sechs Dateien unter `crates/*/src`. Die Zählprobe im Plan sucht deshalb das Paar aus `.versteckt` und einer Typfrage in derselben Datei und nennt die drei erwarteten Dateien beim Namen; ihr Doc-Kommentar sagt, was sie nicht sieht.

## Nicht getan

Kein Code, keine Daten, keine Ausführung. Der Plan wartet auf das Nutzertor.
