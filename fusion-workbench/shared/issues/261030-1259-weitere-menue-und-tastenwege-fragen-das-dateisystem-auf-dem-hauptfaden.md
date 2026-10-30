Weitere Menü- und Tastenwege fragen das Dateisystem auf dem Hauptfaden, und ein hängendes Netzlaufwerk hält dort die Ereignisschleife an
---
Nachdem `ctrl+o` (`838273ae`) und „Im Finder öffnen“ (`8278e685`) ihre Ordnerfrage auf den Faden `krk-terminal` gelegt haben, hat eine Suche über den Betriebscode von `crates/krk-ui/src/appkit/*.rs` und `crates/krk-ui/src/kommandos/*.rs` weitere Stellen gefunden, die auf dem Hauptfaden das Dateisystem fragen. Über NetFS eingehängte Ordner sind lokale Orte, also sind diese Wege dort zulässig; antwortet der Server nicht, wartet der Aufruf auf das Zeitlimit des Dateisystems, und die Ereignisschleife steht so lange. Festgestellt in `838273ae`: `NSURL fileURLWithPath:` ohne `isDirectory:` fragt das Dateisystem selbst, ob der Pfad ein Ordner ist.

1. `appkit/finder.rs:93` `aufdecken` („Im Finder anzeigen“): `fileURLWithPath:` ohne `isDirectory:` auf dem Hauptfaden; der Doc-Kommentar dort behauptet das Gegenteil.
2. `appkit/standardprogramm.rs:108` `oeffnen`, Rufer `tabelle.rs:2746` (Öffnen mit dem Standardprogramm): derselbe Fall, derselbe falsche Doc-Kommentar.
3. `fileURLWithPath:` ohne `isDirectory:` an Stellen, die Menü- oder Tastenwege bedienen; ob jede auf dem Hauptfaden läuft, ist nicht im Einzelnen nachgesehen: `oeffnenmit.rs:159`, `:219`, `:224` („Öffnen mit“), `teilen.rs:383` (Teilen), `zwischenablage.rs:503` (Dateiverweise bei `cmd+c`/`cmd+x`), `papierkorb.rs:143` und `:200`, `abwurf.rs:228` (Abwurf), `volumes.rs:287`.
4. `kommandos/pfadeingabe.rs:101` und `:125`: `fs::metadata` und `fs::read_dir` in `pruefen_gegen`, über `pfadeingabe::pruefen` aus `tabelle.rs:3566` `pfad_anspringen` (Pfadeingabe, Sprung zur Zwischenablage).
5. `appkit/anwendung.rs:6376` `ort_uebernehmen` („Ort wählen…“): `std::fs::canonicalize`.
6. `appkit/anwendung.rs:8030` `lokaler_papierkorbbefund` und `:8219` `loeschgruende`: `canonicalize` im Löschweg — bereits offen als `261016-1956-der-loeschweg-fragt-canonicalize-und-den-papierkorb-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md`.
7. `kommandos/pfadeingabe.rs:195` `gleicher_ordner`: `canonicalize`, gerufen aus `auffrischung.rs`; kein Menü- oder Tastenweg, aber wohl ebenfalls auf dem Hauptfaden.

Zeilennummern Stand `8278e685`.
---
**Filed by:** orchestrator, Kai Stalmann <ks@qantr.com>

Gefunden vom code-implementer beim Umbau von „Im Finder öffnen“, am Quelltext gelesen am 261030, nicht an einer hängenden Freigabe gemessen. Abgelegt auf Wunsch des Nutzers als ein Sammeldefekt.

Vorbild für die Behebung: `terminal::ordner_pruefen_lassen` (Faden `krk-terminal`, Rückweg über `DispatchQueue::main`) und die Quelltextproben `ctrl_o_stellt_auf_dem_hauptfaden_keinen_systemaufruf` und `im_finder_oeffnen_stellt_auf_dem_hauptfaden_keinen_systemaufruf`; für `fileURLWithPath:` genügt, wo die Art des Pfades schon bekannt ist, `fileURLWithPath:isDirectory:`. Der Umfang spricht für ein eigenes Arbeitspaket mit Plan.
