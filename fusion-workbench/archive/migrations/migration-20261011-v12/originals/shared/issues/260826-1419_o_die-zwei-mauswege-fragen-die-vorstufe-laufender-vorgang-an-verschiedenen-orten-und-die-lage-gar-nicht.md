Die zwei Mauswege fragen die Vorstufe „läuft schon ein Vorgang" an verschiedenen Orten und die Lage gar nicht

---

Der Modulkopf von `kommandos/mod.rs:44-53` grenzt `abwurfregel` und `kontextmenue` als die zwei Module ohne Tastenbefehl ab und verspricht für beide: die Rechnung steht ohne Fenster da und ist ohne Fenster prüfbar. Für den Abwurf gilt das — `vorgang_laeuft` ist Eingabe der reinen Regel `urteil` (`abwurfregel.rs:324-337`). Für das Kontextmenü gilt es nicht: `kontextmenue.rs` kennt keinen laufenden Vorgang, die Frage steht allein als `vorgang_laeuft_schon` im Anwendungsdelegierten (`anwendung.rs:6243`, `:6293`), und keine Probe hält, dass Zip und Unzip bei laufendem Vorgang nichts starten. Keiner der beiden Wege fragt `blatt_steht` oder das Schlüsselfenster.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Befund 1: dieselbe Vorstufe in zwei Gestalten

| Weg | Wo „läuft schon ein Vorgang" entschieden wird | Probe |
|---|---|---|
| Löschweg (Taste) | in der reinen Regel `loeschwarnung::vor_der_rueckfrage`, Eingabe `vorgang_laeuft` (`loeschwarnung.rs:360-386`) | `die_tafel_aus_zwoelf_faellen_geht_auf` |
| Abwurf (Maus) | in der reinen Regel `abwurfregel::urteil`, Eingabe `vorgang_laeuft` (`abwurfregel.rs:324-337`) | `die_tafel_der_abweisungen_geht_auf` |
| Zip, Unzip (Maus) | im Anwendungsdelegierten vor der Regel, `vorgang_laeuft_schon` (`anwendung.rs:6243`, `:6293`) | keine |
| Im Finder öffnen (Maus) | nicht gefragt (`anwendung.rs:6342-6351`); richtig, es startet keinen Vorgang | — |

Die Runden 12 und 13 haben die Vorstufe in die Regel gezogen, damit eine Tafel sie hält; die Runde 17 hat sie davor stehen lassen. Wer `zipauftrag_stellen` umbaut und die zwei Zeilen verliert, bekommt zwei Vorgänge auf derselben Maschine, und nichts wird rot.

## Befund 2: kein Mausweg fragt die Lage

`zulaessig` fragt vier Bestandteile (`zulaessigkeit.rs:29-42`). `urteil` und `kontextbefehl_ausfuehren` (`anwendung.rs:6205-6210`) fragen keinen davon: weder `blatt_steht` noch `schluesselfenster_gehoert_krk`. Die stillschweigende Annahme ist, dass AppKit einen Rechtsklick und einen Abwurf auf ein Fenster mit angehängtem Blatt gar nicht zustellt. Für den Rechtsklick ist das die Fenstermodalität des Blattes. **Für den Abwurf habe ich es nicht verifiziert:** `NSDraggingDestination` wird von der Ziehsitzung zugestellt und nicht über die Ereignisschlange des Fensters, und ob ein angehängtes Blatt sie sperrt, sagt keine Stelle im Baum. Träfe es nicht zu, startete ein Abwurf während der Löschrückfrage einen Kopiervorgang, und die Bestätigung der Rückfrage liefe danach in „es läuft bereits eine Operation".

## Vorschlag

1. `Kontextbefehl` bekommt eine reine Vorstufe in `kontextmenue.rs` nach dem Muster von `loeschwarnung::vor_der_rueckfrage`: `vorstufe(befehl, vorgang_laeuft) -> Kontextvorstufe` mit Tafel, und `kontextbefehl_ausfuehren` ruft sie einmal statt je Zweig `vorgang_laeuft_schon`.
2. Die Annahme über das Blatt wird an `abwurfregel.rs` und `kontextmenue.rs` ausgeschrieben; für den Abwurf wird sie am Bündel gemessen (Rückfrage stehen lassen, aus dem Finder eine Datei hineinziehen). Fällt die Messung gegen die Annahme aus, bekommt `Abwurflage` das Feld `blatt_steht` und `urteil` eine Zeile.

Schwere: niedrig — kein gemessener Fehlfall; Befund 2 trägt eine offene Frage.
