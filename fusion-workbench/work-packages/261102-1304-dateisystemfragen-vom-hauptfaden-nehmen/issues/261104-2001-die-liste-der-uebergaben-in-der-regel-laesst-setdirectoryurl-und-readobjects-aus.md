Die Liste der Übergaben an das System in der Regeldatei lässt `setDirectoryURL:` und das Lesen der Ablage aus
---
Schwere: Niedrig. Dokumentation des benannten Rests aus `4af68a8b`; kein Fehlverhalten belegt.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `.claude/rules/coding-was-man-nicht-sieht.md:68` führt als Rest `openURL:`, `URLsForApplicationsToOpenURL:`, `openURLs:withApplicationAtURL:…`, `activateFileViewerSelectingURLs:`, `writeObjects:` mit Dateiadressen und `NSSharingServicePicker`. Dieselbe Liste steht in der Zeile `Resolved:` von `261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md`.

Nicht darin:

1. `crates/krk-ui/src/appkit/blaetter/ortwahl.rs:118` `dialog.setDirectoryURL(Some(&adresse))`, danach `beginSheetModalForWindow:completionHandler:` (`:138`), auf dem Hauptfaden und mit dem geltenden Notizordner als Beginn. Der liegt womöglich auf einer Freigabe. Der Plan hat in `## Current State` die Adresse dieser Stelle dem Weg „Ort wählen…“ zugerechnet, und Schritt 4 hat die Adresse berichtigt. Was der Dialog mit ihr tut, sieht KRK nicht.
2. `crates/krk-ui/src/appkit/zwischenablage.rs:626` `readObjectsForClasses:options:` in `dateiverweise`, gerufen aus `abwurfquellen` und `abwurf_annehmen` auf dem Hauptfaden. Der Kommentar an `der_abwurf_stellt_auf_dem_hauptfaden_keinen_systemaufruf` (`appkit/tabelle.rs`) sagt, `abwurfquellen` lese „die Ablage des Zuges und das Modell und keine Platte“. Das folgt nicht aus dem Quelltext: was das System beim Lesen tut, sieht KRK ebenso wenig wie beim Schreiben.

Ob einer der zwei Aufrufe an einer hängenden Freigabe die Schleife anhält, ist nicht gemessen.

**Abnahme.** Die Regeldatei führt beide als Teil des Rests oder sagt mit Beleg, warum nicht. Der Satz an der Probe sagt nichts über die Platte, was am Quelltext nicht zu sehen ist. Der Lauf, den `261102-1312-was-heisst-fragt-das-dateisystem-auf-dem-hauptfaden-bei-einer-uebergabe-an-das-system.md` dem Nutzer zuweist, schließt die zwei mit ein.

---
Resolved: `.claude/rules/coding-was-man-nicht-sieht.md` führt `readObjectsForClasses:options:` (`zwischenablage::dateiverweise`) und `setDirectoryURL:` mit dem Blatt dahinter (`blaetter/ortwahl.rs`) im benannten Rest der Übergaben, und der Lauf des Nutzers, von dem derselbe Absatz spricht, gilt damit auch für diese zwei. Der Kommentar an `der_abwurf_stellt_auf_dem_hauptfaden_keinen_systemaufruf` (`appkit/tabelle.rs`) sagt nicht mehr „keine Platte“, sondern dass KRK nicht sieht, was das System beim Lesen der Ablage fragt. Die Zeile `Resolved:` in `261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md` bleibt unverändert. Der Abschnitt `## Question` in `261102-1312-was-heisst-fragt-das-dateisystem-auf-dem-hauptfaden-bei-einer-uebergabe-an-das-system.md` zählt die zwei weiterhin nicht auf; dieser Datensatz ist hier nicht geändert. Gemessen ist weiterhin keine Übergabe.
