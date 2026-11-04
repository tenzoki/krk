Die Geräteaufzählung, F2 und das Öffnen von `secrets.txt` stellen eigene Fragen auf dem Hauptfaden, und die Regel führt sie nicht als Rest
---
Schwere: Mittel. Die Stellen liegen außerhalb des Sammeldefekts. Sie sind die Frageart, für die `.claude/rules/coding-was-man-nicht-sieht.md` seit `4af68a8b` den Nebenfaden verlangt, und an einem hängenden Netzlaufwerk können sie die Ereignisschleife anhalten.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg, gelesen am Quelltext, an keiner hängenden Freigabe gemessen.**

1. `crates/krk-ui/src/appkit/volumes.rs:196` bis `:213` `eingehaengte`: `mountedVolumeURLsIncludingResourceValuesForKeys:options:` und je Datenträger `resourceValuesForKeys:error:`, beide für `NSURLVolumeLocalizedNameKey`. Rufer `anwendung.rs:3904` `geraete_uebernehmen`, beim Start (`:2943`) und auf jede Ein- und Aushängemeldung (`:3885`). Die Aufzählung läuft über alle Datenträger, also auch über den, der gerade nicht antwortet. **Fehlerbild:** eine Freigabe hängt, der Nutzer steckt einen USB-Stick ein oder hängt etwas anderes ein, und KRK fragt auf dem Hauptfaden auch die hängende Freigabe. Dasselbe Paar steht in `verbinden.rs:646` `eingehaengte_freigabe`, dort mit dem Hinweis, es laufe auf dem Faden `krk-verbinden`. **Ob die zwei Aufrufe mit diesem Schlüssel die Platte erreichen, ist nicht gemessen.** `inference:` der Name des Datenträgers steht oft im Zwischenspeicher, das ist aber keine Zusage.
2. `anwendung.rs:6446` und `:6464` `notizordner_bereitstellen` (F2): `heimordner::bereitstellen` und `aufgeloest_erneuern` (`canonicalize`) auf dem Hauptfaden. Seit „Ort wählen…“ kann der Notizordner auf einer Freigabe liegen. Schritt 6 des Plans hat allein `ort_uebernehmen` umgestellt.
3. `crates/krk-ui/src/appkit/editor.rs:4927`: `std::fs::metadata(pfad)` auf `secrets.txt` im Zweig `Oeffnungsweg::PinErfragen`, auf dem Hauptfaden, mit demselben möglichen Ort.

**Die Regeldatei.** `.claude/rules/coding-was-man-nicht-sieht.md:68` sagt, eine Frage an die Platte gehe auf einem Hauptfadenweg über `appkit/nebenfaden.rs`, nennt `resourceValuesForKeys:` und führt als weiter bestehend allein `gleicher_ordner` und das Schreiben in `settings.toml`. Sie sagt zwar „keine Auskunft über den ganzen Baum“, aber wer die zwei Beispiele liest, hält den Rest für klein. Allein der Kommentar an `kein_betriebscode_der_oberflaeche_baut_eine_dateiadresse_ohne_isdirectory` (`crates/krk-core/tests/baum.rs`) nennt `eingehaengte`. Der Sammeldefekt `261030-1259-weitere-menue-und-tastenwege-fragen-das-dateisystem-auf-dem-hauptfaden.md` nennt keine der drei Stellen, und sein Schließen ist deshalb nicht falsch.

**Richtung.** Der Nutzer entscheidet über den Umfang. Für Punkt 1 liegt der Mechanismus bereit, und die Liste kommt dann über die Hauptschlange an `geraete_uebernehmen`. Die Punkte 2 und 3 hängen an der Frage, ob ein Notizordner auf einer Freigabe zu den zugesagten Fällen gehört.

**Abnahme.** Jede der drei Stellen läuft auf einem Nebenfaden mit einer Rumpfprobe oder steht mit Namen als Rest in der Regeldatei. Der Satz zu `eingehaengte` in `baum.rs` und die Regel sagen dasselbe.
