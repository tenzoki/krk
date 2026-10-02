# Was die zwölf Leseprofile an der wirklichen Werkbank kosten

**Agent:** analyst
**Datum:** 2026-08-25, 20:55 bis 21:13
**Aufgabe:** T-10, Schritt 10 des Plans
`fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`,
Strang 3 — „Was die acht Profile an der wirklichen Werkbank kosten", nach dem
Nachtrag zu Schritt 8 auf zwölf Profile gezogen.
**Status:** Complete

## Was entstanden ist

Ein Bericht im Analysespeicher,
`shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md`,
und zwei Defektdatensätze. Der Bericht führt für jedes der zwölf ausgelieferten Profile
vier Zahlen: Leseläufe, geöffnete Verzeichnisse, gelesene Einträge und Dateiöffnungen.

## Wie gemessen wurde

Zwei der vier Zahlen liest die Anwendung selbst aus. `zusammenfassen_gezaehlt` liefert
neben der Zusammenfassung den verbrauchten `Haushalt`, und der bucht Leseläufe und
Dateiöffnungen.

Die zwei anderen bucht der `Haushalt` nicht. Für sie zählt eine kleine Bibliothek in C
die Systemaufrufe mit: sie fängt über `DYLD_INSERT_LIBRARIES` `open(2)`,
`getattrlistbulk(2)` und `realpath(3)` ab, unterscheidet Verzeichnis von Datei über
`fstat(2)` am zurückgegebenen Deskriptor und grenzt den Messabschnitt über zwei
Sentinel-Pfade ab, so dass der eigene Start des Messprogramms in keine Zahl eingeht.

**Der KRK-Baum trägt keine Zeile Prüfcode dieser Messung.** Das Messprogramm ist ein
eigenes Cargo-Paket im Wegwerfverzeichnis der Sitzung mit einer Pfadabhängigkeit auf
`krk-core`; es baut in sein eigenes `target/` und ruft ausschließlich öffentliche
Schnittstellen. Die drei künstlichen Verzeichnisbäume der Schrankenmessung lagen
ebenfalls dort und sind nach der Messung entfernt. Kein `cargo`-Aufruf im Projektbaum,
kein Commit, kein Git-Kommando über den ganzen Baum.

## Die Befunde in Kürze

**Die Abweichung, um die es dem Schritt ging, ist bestätigt.** Das Profil `circles/`
kostet drei Leseläufe und neununddreißig Verzeichnisöffnungen, also genau die Rechnung
1 + 19 + 19 aus dem Plan. Bei den elf übrigen Profilen sind beide Zahlen gleich.

**Die gerechneten Zahlen des Plans halten bis auf zwei Stellen.** `circles/` und
`archive/` treffen vollständig, `shared/` in Läufen und Öffnungen (die Einträge sind 286
und nicht ~264), die Projektwurzel liegt in allen drei Spalten um eins zu hoch: gemessen
4 / 4 / 4 statt 5 / 5 / 5. Der Grund ist, dass `.active-circle` an dieser Werkbank nicht
steht und die Feldzeile „Aktive Runde" deshalb keine Datei öffnet.

**Die sechs Zahlenpaare aus Schritt 8 halten alle.** Was der Ontocoder nicht ausgewiesen
hat, ist die Spalte der geöffneten Verzeichnisse, und genau dort liegt der Preis.

**Die Rechnung des Datensatzes `260825-1953` ist bestätigt und nicht widerlegt.** An
künstlichen Bäumen gemessen: 22.001 Verzeichnisöffnungen bei elf Sammlungen gegen zwölf
Leseläufe, 4.001 beim ausgelieferten Profil über 2.500 Runden mit leerem `issues`, und
2.000 gescheiterte Auflösungsversuche, wo `issues` überall fehlt.

**Die Eintragsschranke fällt zwischen 67 und 93 Runden**, je nachdem ob man den Schnitt
über alle 19 Runden (29,9 Defektdatensätze je Runde) oder über die letzten zehn (19,4)
fortschreibt. Der Ort, an dem sie zuerst greift, ist die Sammlung `circles/*/issues` mit
heute 568 von 2.000 Einträgen.

**Die 64-KB-Grenze des Feldbausteins ist an dieser Werkbank schon überschritten**, von
einem Circle-Datensatz mit 119.614 Byte. Die Zeile „Directive" trägt trotzdem, weil der
Abschnitt in Zeile 12 steht.

## Was abgelegt ist

- `shared/issues/260825-2107_o_der-l7-entscheid-nennt-fuer-das-groesste-mitgelieferte-profil-fuenf-leselaeufe-gemessen-sind-vier.md`
- `shared/issues/260825-2107_o_ein-circle-datensatz-liegt-beim-1-8-fachen-der-64-kb-grenze-des-feldbausteins.md`

## Was ausdrücklich nicht behauptet ist

Keine Zeitmessung. Keine der zehn Zusagen aus C8 spricht über die Profil-Zusammenfassung,
die Messstrecke sieht sie nicht, und der Abnahmelauf verlangt KRK im Vordergrund und ist
Nutzerarbeit. Gezählt sind Aufrufe. Was eine Zusammenfassung an Zeit kostet, ist an
diesem Baum ungemessen.

Keiner der drei zitierten offenen Datensätze ist beantwortet. Der Bericht sagt bei jedem
aus, was die Erhebung beiträgt und was offen bleibt.

## Verification

`ls fusion-workbench/shared/analyses/260825-2107-was-die-zwoelf-leseprofile-an-der-wirklichen-werkbank-kosten.md fusion-workbench/shared/issues/260825-2107_o_*.md && grep -c '^10\. \[DONE\]' fusion-workbench/shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`
