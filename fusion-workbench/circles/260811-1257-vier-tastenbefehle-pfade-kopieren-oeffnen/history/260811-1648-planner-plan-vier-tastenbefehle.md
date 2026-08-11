# Planner: Umsetzungsplan zur Runde 4, vier Tastenbefehle für Pfade, das Öffnen und Cmd+W

**Datum:** 260811-1648
**Agent:** planner
**Circle:** `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`
**Status:** Complete

## Auftrag

Den Umsetzungsplan zum abgenommenen Spec `planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md` schreiben, mit Marker `_o_`, ohne Code anzufassen und ohne zu committen.

## Was entstanden ist

- `planning/260811-1648_o_plan-vier-tastenbefehle-pfade-kopieren-oeffnen.md` — fünf Schritte, elf Befunde aus der Erhebung am Baum, Antworten auf die acht Punkte, die der Spec dem Planner überlässt, zwei Mermaid-Bilder, eine Zuordnung aller 62 Abnahmekriterien.
- `decisions/260811-1648_o_fragt-krk-nach-bevor-return-viele-eintraege-oeffnet.md` — die Frage, die `260811-1612_*` ausdrücklich offen lässt, als eigener Datensatz.
- `issues/260811-1648_o_fuenf-entscheidungsdatensaetze-tragen-im-rumpf-noch-den-stand-offen.md`
- `fusion-workbench/shared/issues/260811-1648_o_die-untergrenzen-angabe-im-modulkopf-steht-in-sieben-von-32-appkit-modulen.md` — nebenbei gefunden, deshalb im gemeinsamen Speicher.

## Die tragenden Entwurfsentscheidungen

**Die drei neuen Befehle wohnen in `DateifensterQuelle::kommando_ausfuehren` und nicht im Anwendungsdelegierten.** Sie brauchen nichts, was der Delegierte hält; der angezeigte Ordner, die betroffenen Einträge und die Statuszeile hängen alle an der Quelle. Der Weg dorthin ist der bestehende `bereichskommando`, und damit ist das Kriterium "das aktive Dateifenster" ohne eine eigene Zeile erfüllt. Der Delegierte bekommt genau einen neuen Zweig, und der gehört Cmd+W.

**Das Öffnen bekommt ein eigenes Modul `appkit/standardprogramm.rs`**, nach der Hausregel "ein Modul je Frage", die `terminal.rs` in seinem Kopf ausschreibt. Die Schreibseite der Zwischenablage bleibt in der einen Hülle.

**Eine Umsetzung des Öffnens, zwei Zugänge.** `DateifensterQuelle::mit_standardprogramm_oeffnen(pfade)` ist die eine Stelle; die Taste gibt ihr `betroffene()`, der Doppelklick die eine angeklickte Zeile. Der Einstieg in einen Ordner wird aus `auswahl_oeffnen` als `in_zeile_einsteigen(zeile)` herausgezogen und von beiden Wegen geteilt.

**Der Schnitt der Schritte folgt einer Eigenschaft der Kiste**: `krk-ui` hat kein Bibliotheksziel, eine `pub`-Funktion ohne Aufrufer ist `dead_code`, und `make lint` fährt mit `-D warnings`. Hülle und erster Aufrufer stehen deshalb je in einem Schritt.

## Drei Befunde, die der Spec nicht führt

1. **Fünf schriftliche Zusicherungen brechen, nicht zwei.** Dazu kommen `resources/default-keymap.toml:52-56` (die Eingabetaste bleibt frei), `resources/default-keymap.toml:149-151` (die vier Tabbefehle wirken auf den Bereich mit dem Fokus) und der Doc-Kommentar von `befehlsantwort_loeschen` (`tabelle.rs:1496`), sobald der Doppelklick sein zweiter Aufrufer wird.
2. **Der angezeigte Ordner kann auf einem Schrägstrich enden.** `pfadeingabe::pruefen` (`pfadeingabe.rs:77-80`) und `zwischenablage::deuten` (`krk-core/src/zwischenablage.rs:68-69`) übernehmen den eingegebenen Text wörtlich. Ein Abnahmekriterium von C1 verlangt die Form; sie entsteht deshalb in einer reinen Funktion `pfadtext` und nicht an der Quelle.
3. **Eine Probe darf `text_schreiben` nicht rufen.** `generalPasteboard` ist die Zwischenablage des angemeldeten Nutzers, und ein `make check`, das sie überschreibt, wirft weg, was der Entwickler kopiert hatte. Die Schreibhülle bekommt keine Probe, und der Modulkopf nennt den Grund.

## Was offen bleibt

Zwei Fragen stehen im Plan unter `## Offene Fragen`, und keine hält einen Schritt auf: die Schwelle vor dem Öffnen vieler Einträge (eigener Datensatz) und die Frage, ob `NSTableView` die Eigenschaft `target` schwach führt (S4 beantwortet sie an der erzeugten Zeile der Bindung, beide Ausgänge sind vorgesehen).

Der Abnahmelauf ist in dieser Runde nicht vorgesehen. Sieben Abnahmekriterien sind allein am laufenden Bündel zu sehen; sie stehen im Plan in einem eigenen Abschnitt und sind **keinem Schritt zugeordnet**, damit der Abschluss der Runde nicht an einem Lauf hängt, den kein Agent fahren kann.

## Grenzen eingehalten

Kein Code angefasst, kein Commit, der Spec unverändert. Der Marker des Plans ist `_o_`.
