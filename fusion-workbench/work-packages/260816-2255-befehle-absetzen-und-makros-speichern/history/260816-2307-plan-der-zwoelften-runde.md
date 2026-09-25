# Planung der zwölften Runde: Befehle absetzen und Makros speichern

**Datum:** 2026-08-16 23:07
**Status:** Complete
**Agent:** planner
**Circle:** `circles/260816-2255-befehle-absetzen-und-makros-speichern`

## Auftrag

Den Implementierungsplan zum abgenommenen Spec `shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md` schreiben und die zehn Punkte unter dessen Abschnitt `## Offen für den Planner` entscheiden.

## Was gelesen wurde

Der Spec vollständig, der Circle-Datensatz `_t_circle.md`, die Beratung `shared/consult/260815-1354-befehlslauf-und-makros-in-krk.md` samt Quellenliste, `CLAUDE.md`. Im Baum am Stand `627b5f4`: `kommandos/operationen.rs` (Modulkopf, `Buendelung`, `Vorgangszustand`), `appkit/anwendung.rs` (`Vorgang`, `auftrag_starten`, `vorgang_zeichnen`, `vorgang_beenden`, `abbrechen`, `vermitteln`), `vorschaumodell.rs` (Modulkopf, `Vorschautab`, `zwischenablage_anzeigen`, `Ladevorgang`, `TEXTGRENZE`), `verzeichnis/sys.rs` (Modulkopf mit den fünf Schnittstellen), `ablage/pfade.rs`, `ablage/mod.rs`, `ablage/einstellungen.rs`, `ablage/atomar.rs`, `tasten/belegung.rs` (`Kommando`, `KENNUNGEN`, `Wirkungsbereich`), `belegungsmodell.rs` (`Funktionsbereich`, `bereich_des_kommandos`), `kommandos/zulaessigkeit.rs` samt Tafel, `appkit/statuszeile.rs` (die sechs Ränge), `menuemodell.rs`, `appkit/belegungsansicht.rs`, `appkit/blaetter/`, `appkit/terminal.rs`, `resources/default-keymap.toml`, die Manifeste des Arbeitsbereichs.

## Nachgezählt statt übernommen

`Kommando` trägt 79 Varianten, `Funktionsbereich::ALLE` neun, `Datei::ALLE` sechs. Die Zahlen aus C4.7 (79 auf 92) und C4.11 (85 auf 98 Funktionen) gehen damit auf.

## Was entschieden wurde

Die zehn offenen Punkte sind im Plan einzeln beantwortet. Vier Antworten weichen von dem ab, was der Spec oder die Beratung nahelegt oder offen lässt:

1. **Eine Röhre für beide Ausgabeströme** statt zweier. C1.5 verlangt die Reihenfolge des Eintreffens, und mit zwei Röhren und zwei Lesern ist sie nicht herstellbar.
2. **Der Lauf endet mit der Shell, nicht mit dem Dateiende der Röhre**, und dasselbe `killpg`, das der Abbruch schickt, schließt danach die übrigen Schreibenden. Ohne das hielte ein abgehängter Enkelprozess den Vorgang beliebig lange offen und C1.15 sperrte jeden weiteren Befehl. Datensatz `decisions/260816-2307_o_stirbt-die-prozessgruppe-auch-am-normalen-ende-des-laufs.md`.
3. **Das Setzen der Prozessgruppe braucht keine siebte Schnittstelle.** `std::os::unix::process::CommandExt::process_group` leistet es ohne `unsafe`; `killpg(2)` bleibt die sechste und einzige neue.
4. **Die Anführungsregel aus C2.6 wird nicht dem Wortlaut nach umgesetzt.** Die Verdopplung des Anführungszeichens verliert es in der Shell still, an diesem Gerät nachgemessen. Umgesetzt wird die Regel, die den Nachweis desselben Kriteriums besteht. Datensatz `issues/260816-2307_o_c2-6-beschreibt-das-verdoppeln-des-anfuehrungszeichens-die-shell-verliert-es-dabei.md`.

Daneben zwei Entwurfsentscheidungen, die den Zuschnitt tragen: ein Fach für einen Vorgang mit zwei Füllungen statt zweier Fächer, und ein `zieltab` im Vorschaumodell statt einer zweiten Tab-Sorte.

## Erzeugte Dateien

- `planning/260816-2307_o_plan-befehle-absetzen-und-makros-speichern.md` — 22 Schritte in fünf Bündeln, drei Mermaid-Diagramme
- `decisions/260816-2307_o_welche-shell-faehrt-den-lauf-und-woher-kommt-ihr-pfad.md`
- `decisions/260816-2307_o_stirbt-die-prozessgruppe-auch-am-normalen-ende-des-laufs.md`
- `issues/260816-2307_o_c2-6-beschreibt-das-verdoppeln-des-anfuehrungszeichens-die-shell-verliert-es-dabei.md`
- `shared/issues/260816-2307_o_der-doc-kommentar-von-ablage-pfad-nennt-vier-dateien-die-aufzaehlung-fuehrt-sechs.md`

## Nicht getan

Kein Code, keine Daten, kein Agent gestartet. Die Ausführung entscheidet der Nutzer am Gate.
