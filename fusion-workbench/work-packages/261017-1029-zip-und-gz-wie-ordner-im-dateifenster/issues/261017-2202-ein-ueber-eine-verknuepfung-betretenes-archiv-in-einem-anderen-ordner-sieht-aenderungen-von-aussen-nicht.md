Ein über eine Verknüpfung betretenes Archiv in einem anderen Ordner sieht Änderungen von außen nicht
---
Ein Archivtab beobachtet den Ordner, in dem die Verknüpfung liegt, und nicht den Ordner der Archivdatei. Eine Änderung von außen an der Archivdatei erreicht den Tab deshalb nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Severity:** Low
**Cross-references:** `261017-2202-reviewer-nachdurchsicht-archivpaket-8c8579af-bis-c9651443.md`, `261017-1923-eine-verknuepfung-auf-ein-archiv-wird-nur-an-ihrem-eigenen-namen-erkannt.md`

Seit `c9651443` betritt KRK eine Verknüpfung auch dann als Archiv, wenn nur der Name ihres Ziels die Endung trägt (`Tabliste::verknuepfung_pruefen`, `crates/krk-ui/src/tabs.rs`). Der Zugang trägt dabei den Pfad der **Verknüpfung** (`archivregister::zugang(&verknuepfung, format)` im Einzug). Für eine Verknüpfung mit eigener Endung galt das schon vorher.

Die Beobachtung rechnet mit diesem Pfad:

- `auffrischung::sichtbare_ordner` nimmt für einen Archivtab `archiv.datei().parent()` auf, also den Ordner der Verknüpfung (`crates/krk-ui/src/auffrischung.rs`).
- `zeigt_gemeldeten_ort` vergleicht eine FSEvents-Meldung mit demselben Ordner.

Liegt das Ziel in einem anderen Ordner, etwa `~/dl/aktuell -> ~/archiv/paket-1.2.zip`, wird `~/archiv` nicht beobachtet. Schreibt ein anderes Programm das Archiv neu, zeigt der Tab weiter den alten Inhalt, bis der Nutzer ihn neu lädt oder neu betritt. HowTo verspricht dagegen: „Änderungen von außen erscheinen von selbst.“

Datenverlust entsteht dabei nicht. Jeder Umbau liest das Verzeichnis zur aktuellen Kennung neu (`umbau::verzeichnis_zur_kennung`), und `ersetzen` prüft die Kennung vor dem `rename`. Eigene Vorgänge von KRK frischen den Tab auf, weil `gemeldete_orte` auf denselben Verknüpfungsordner abbildet.

Belegt ist der Befund durch Lesen des Codes, nicht am laufenden Bündel.

Akzeptanz:

- Entweder beobachtet ein Archivtab auch den Ordner der aufgelösten Archivdatei. Aufgelöst wird auf dem Faden, der das Ziel schon fragt, und nicht auf dem Hauptfaden.
- Oder `HowTo.md` nennt die Ausnahme bei „Änderungen von außen erscheinen von selbst“.

---
Resolved: Ein über eine Verknüpfung betretenes Archiv beobachtet zusätzlich den Ordner ihres aufgelösten Ziels. Der Faden der Verweisfrage (`verweisziel_format`, `crates/krk-ui/src/tabs.rs`) löst das Ziel auf und liefert es mit dem Format, das jetzt aus dem eigenen Namen oder dem des Ziels kommt; `verknuepfung_betreten` (`appkit/tabelle.rs`) schickt deshalb jede Verknüpfung über diesen Faden und fällt nur ohne Faden auf den Namensweg zurück. Der Zugang trägt das Ziel (`Archivzugang::ueber_verknuepfung`, `Archivzugang::aufgeloeste_datei`, angelegt über `archivregister::zugang_ueber_verknuepfung`), und `auffrischung::archivordner` gibt `sichtbare_ordner`, `zeigt_gemeldeten_ort` und `gemeldete_orte` beide Ordner, am Wert und ohne Systemaufruf. Proben: `ein_ueber_eine_verknuepfung_betretenes_archiv_beobachtet_den_ordner_seines_ziels` (`auffrischung.rs`), `ueber_eine_verknuepfung_folgt_der_zugang_dem_ziel` (`archivregister.rs`), die erweiterte `eine_verknuepfung_wird_am_namen_ihres_ziels_als_archiv_erkannt` (`tabs.rs`) und die um das Register erweiterte Rumpfprobe `der_archiveinstieg_stellt_auf_dem_hauptfaden_keinen_systemaufruf`. Grenze: eine nach dem Einstieg umgehängte Verknüpfung beobachtet bis zum nächsten Betreten das alte Ziel; `HowTo.md` und `CLAUDE.md` nennen die Beobachtung des Zielordners.
