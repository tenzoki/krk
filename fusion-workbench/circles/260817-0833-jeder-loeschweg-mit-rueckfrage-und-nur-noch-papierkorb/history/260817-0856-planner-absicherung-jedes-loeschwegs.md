# Planner: der Implementierungsplan zur Absicherung jedes Löschwegs

**Datum:** 2026-08-17 08:56
**Agent:** planner
**Status:** Complete
**Circle:** `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`
**Ergebnis:** `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`

## Auftrag

Den Implementierungsplan für die zwölfte gefahrene Runde schreiben, gegen den abgenommenen Spec `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` und den aktiven Circle-Datensatz. Executor-Menge: `coder, ontocoder, analyst`. Ausdrücklich verlangt: die acht Punkte aus `## Offen für den Planner` beantworten, die Aufzählung der überholten Stellen aus C6 gegen den Baum prüfen statt sie zu übernehmen, den Umfang des Wegfalls selbst nachzählen, und die Bündel so schneiden, dass die Schwelle sichtbar ist, ab der der Nutzer gegen einen zweiten Schadensfall geschützt ist.

## Gelesen

Spec und Circle-Datensatz vollständig, die vier bindenden Entscheidungsdatensätze `260817-0536_a_*`, der Defektdatensatz `260816-2144_o_*`, die Forensik `260817-0419`, dazu `CLAUDE.md`. Am Baum: `appkit/anwendung.rs` (Löschwege, `auftrag_stellen`, Dispatch), `appkit/blaetter/loeschbestaetigung.rs` und `blaetter/mod.rs`, `appkit/papierkorb.rs`, `appkit/volumes.rs`, `kommandos/operationen.rs`, `kommandos/rueckschritt.rs`, `kommandos/zulaessigkeit.rs`, `krk-core/src/operation/{loeschen,auftrag,mod}.rs`, `krk-core/src/tasten/belegung.rs`, `krk-core/src/verzeichnis/{sys,durchlauf,leser,eintrag,mod}.rs`, `krk-core/src/ablage/pfade.rs`, `resources/default-keymap.toml`, `krk-ui/src/quellbaum.rs`, `krk-ui/src/auffrischung.rs`, `krk-ui/src/belegungsmodell.rs`. Von außen: die Bindungen in `objc2-foundation 0.3.2` und die Kopfdateien des lokalen macOS-SDK.

## Was gemessen wurde

- **Der Umfang des Wegfalls stimmt mit dem Spec.** `grep -rn "EndgueltigLoeschen" --include="*.rs" crates` liefert 20 Zeilen in 11 Dateien; drei stehen in Doc-Kommentaren. Es bleiben 17 Nennungen in 9 Dateien, genau die Tabelle unter C5.
- **Fünf Stellen fehlen in dieser Tabelle**, weil der Übersetzer sie nicht einfordert: zwei Zusicherungen in `krk-ui/src/belegungsmodell.rs` (Zeile 953 und die Probe ab 1186), der Kopf von `resources/default-keymap.toml` mit „85 Funktionen mit zusammen 90 Kombinationen" und seiner Nennung des überholten Datensatzes, und acht ausgeschriebene Zahlen in `belegungsausgabe.rs` und `appkit/menue.rs`.
- **Die C6-Aufzählung ist unvollständig.** Der Circle-Datensatz der Runde 1 trägt die überholte Festlegung an drei Stellen, nicht an einer: `## Directive`, die Liste „Beantwortet am 260802-1105" und der Absatz „*Später überholt, Stand 260802-1735:*". Der Spec der Runde 1 trägt neben den neun genannten Zeilen drei weitere, weil die Kürzel-Tabelle eine Zeile verliert und drei Stellen von „sechs" sprechen (Zeilen 205, 220, 222). Der Spec nennt den Pfad jenes Specs mit dem Marker `_o_`; die Datei trägt `_c_`.
- **Keine Messstrecke in `krk-bench` nennt einen Löschbefehl.** Die Aussage des Specs, dass keine der zehn Zeitzusagen berührt ist, hält auch von dieser Seite.
- **Die Verfügbarkeiten sind gegen die SDK-Kopfdateien gelesen, nicht erschlossen.** `URLForDirectory:inDomain:appropriateForURL:create:error:` seit macOS 10.6 (`NSFileManager.h:127`), `NSTrashDirectory` seit 10.8 (`NSPathUtilities.h:88`), `NSURLVolumeIsLocalKey` seit 10.7 (`NSURL.h:338`), `NSUserDomainMask` ohne Angabe. `NSURLVolumeSupportsTrashKey` existiert in diesem SDK nicht und ist in `objc2-foundation 0.3.2` nicht gebunden.

## Was der Plan entscheidet

Siebzehn Schritte in fünf Bündeln. Die Rückfrage geht dem Wegfall voran; nach Schritt 3 ist der Nutzer geschützt und `make check` grün. Der Papierkorbtest läuft über `URLForDirectory:` mit `create: NO`, also ohne Probelauf. Die gedeckelte Zählung zieht als eigenes Modul `verzeichnis/umfang.rs` neben `durchlauf` ein, liest über `Schwungleser` statt über `verzeichnis::lesen` und läuft auf dem Hauptfaden mit beweisbarer Schranke. Der Aufwärtsgang der Git-Prüfung bekommt `verzeichnis/arbeitsbaum.rs` und prüft die ausgewählten Einträge nur, wenn der Aufwärtsgang verneint. Die Tafel der Auslöser wohnt als reine Funktion in `kommandos/loeschwarnung.rs`, zusammen mit den drei Texten des einen Löschwegs; die Rangfolge ist die Deklarationsreihenfolge der Aufzählung und keine zweite Liste.

Ein bestehender Defekt wird mitbehoben: der bestätigte Auftrag trägt künftig die **gezeigte** Auswahl, statt `betroffene_eintraege()` nach dem Blatt ein zweites Mal zu lesen.

## Was nicht getan wurde

Kein Code geschrieben, kein Executor gestartet. Kein neuer Entscheidungsdatensatz und kein neuer Defektdatensatz: der Spec ist ohne ausstehende Nutzerentscheidungen, und die Lücken, die die Prüfung gegen den Baum gefunden hat, sind Umsetzungsstellen und keine Wahlpunkte. Sie stehen in den Schritten 12, 14 und 17 namentlich.
