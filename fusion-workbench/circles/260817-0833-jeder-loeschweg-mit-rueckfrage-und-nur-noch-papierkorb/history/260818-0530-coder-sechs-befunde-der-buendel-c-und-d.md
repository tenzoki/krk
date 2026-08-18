# Sechs Befunde aus den Durchsichten der Bündel C und D

**Status:** Complete
**Agent:** coder
**Datum:** 260818-0530
**Baumstand vor der Arbeit:** `cdde9da`
**Abnahme:** `make check` — Exit 0

## Was der Auftrag war

Sechs Befundsdatensätze aus den Durchsichten der Bündel C und D beheben, in drei Gruppen:
die verkürzte Blattsperre (zwei), der Eingabewächter des Konfliktblattes (zwei) und zwei
einzelne. Verlangt waren eine eigene Erhebung über den ganzen Baum statt einer übernommenen
Zahl, und für jede neue Probe der Nachweis, dass sie den Fehler fängt, gegen den sie gerichtet
ist.

## Gruppe 1 — die verkürzte Blattsperre

### Die Erhebung

Gesucht wurde nicht der Wortlaut, sondern die **Aussage**: eine Stelle, die sagt, was die
Sperre als Ganzes durchlässt, und dabei weniger als vier Kommandos nennt. Zwei Nadeln über
`crates/`, `xtask/`, `resources/`, `CLAUDE.md`, `README.md`, `Makefile`, `idea.txt`,
`Cargo.toml` und `.claude/`; ausgenommen `fusion-workbench/`, `messungen/` und `spikes/` nach
der Ortsregel aus `CLAUDE.md`.

**Sechs Träger.** Vier waren in Datensätzen genannt, zwei in keinem — und einer der zwei ist
die Wurzel, aus der die übrigen vier ihre Formulierung haben: der Abschnittskopf und der
Doc-Kommentar von `waehrend_blatt_erlaubt` selbst hießen „Was durchkommt, solange ein Blatt
steht" und antworteten „Genau der Abbruchbefehl". Der zweite ungenannte sind die beiden Proben
derselben Regel, die die verkürzte Aussage im Namen und in der Fehlschlagsmeldung trugen,
während die Nachbarprobe in `zulaessigkeit.rs` die vollständige Aussage im Namen führt.

Behoben sind die vier in `crates/`. Zwei stehen weiter und sind vom Auftrag ausgenommen:
`resources/default-keymap.toml:710` und `CLAUDE.md:124`.

**Der blinde Fleck war zweimal ein anderer.** Der erste war die Ordnergrenze `crates/`, wie
`260817-1419` sagt. Der zweite war die Nadel selbst: `anwendung.rs` trug die Aussage in
anderen Worten, `operationen.rs` unter der Überschrift statt im Satz, und beide entgingen der
wörtlichen Suche. Der Modulkopf von `crate::quellbaum` schreibt genau das für jede Zählprobe
dieses Baums schon vor: nach dem Gegenstand suchen, nicht nach seinem Namen.

### Die Messung

`zulaessigkeit::tests::waehrend_eines_blattes_kommen_genau_diese_vier_durch` zählt, welche
Kommandos `zulaessig` bei stehendem Blatt durchlässt, prüft die Länge gegen 4 und schreibt die
vier Namen aus. Sie ist nicht dasselbe wie die Nachbarprobe
`waehrend_eines_blattes_kommt_allein_der_abbruch_und_die_ausnahmeliste_durch`: die prüft
`zulaessig` gegen `waehrend_blatt_erlaubt || immer_erreichbar`, hält also die
Zusammenrechnung und sagt über die **Zahl** nichts.

Nachweis, dass sie den Fehler fängt: ein vierter Eintrag auf `immer_erreichbar` (probeweise
`Kommando::Notizzettel`) lässt sie rot werden mit
`[FensterEinblenden, FensterSchliessen, Abbrechen, Beenden, Notizzettel]`. Zurückgenommen.

## Gruppe 2 — der Eingabewächter des Konfliktblattes

**Der zweite Datensatz war eine harte Vorbedingung des ersten und keine Zugabe.** Hätte der
Wächter am Namensfeld gehangen, solange seine bestätigende Seite fest auf der ersten
Schaltfläche lag, hätte ein Return im Feld „Überschreiben" ausgelöst und den Eintrag am Ziel
gelöscht — genau die Bewegung, die der Modulkopf des Konfliktblattes für die
Vorgabeschaltfläche ausdrücklich ausschließt. Der Wächter allein wäre kein halber Fix gewesen,
sondern ein neuer Defekt auf dem zerstörenden Ausgang.

Gebaut in dieser Reihenfolge:

1. `blaetter::bestaetigungsstelle` — eine reine Funktion neben `abbruchstelle`, die die erste
   Schaltfläche mit `Taste::Eingabe` liefert und ohne eine solche auf `abbruchstelle`
   zurückfällt. Der Rückfall ist ausdrücklich nicht die erste Stelle: in einem Blatt mit
   ausführender erster Schaltfläche legte das die Eingabetaste auf den zerstörenden Ausgang.
2. `konflikt.rs` ruft `waechter_anhaengen` — die dritte der drei Handlungen von
   `textfeld_setzen` und nicht dieses selbst, denn das Feld soll weiterhin nicht Ersthelfer
   werden.

Die Bedeutung des Wächters in diesem Blatt ist damit abgeleitet und nicht gewählt: die
Eingabetaste geht an „Überspringen", die Escape-Taste über `abbruchstelle` an „Abbrechen" —
also genau das, was die Erläuterung des Blattes dem Nutzer ansagt.

Zwei Proben, beide ohne AppKit. Nachweis: `bestaetigungsstelle` probeweise wieder auf `0`
festgelegt, beide werden rot, und die zweite meldet
`die Eingabetaste faellt auf "Überschreiben", und die traegt sie nicht`. Zurückgenommen.

Ungemessen bleibt, ob der Feldeditor `Cmd+Return` und `Opt+Return` durchlässt; das verlangt
KRK im Vordergrund und steht als offen im Modulkopf.

## Gruppe 3

**`loeschen_nach_rueckfrage`:** der Parameter `art: Art` ist gefallen, der Rumpf nennt
`Art::InDenPapierkorb` selbst. Der Typ kann die Einschränkung nicht tragen — `Art` gehört
`krk-core` und führt die vier Arten aller Dateioperationen —, aber der kleinste Typ, der nur
die zulässigen Werte kennt, ist hier kein Parameter. Das hält in jedem Profil, anders als das
`debug_assert!`, das der Datensatz als zweite Möglichkeit nennt und das in dieser Runde schon
einmal an genau dieser Eigenschaft gescheitert ist. `schaltflaeche: &str` bleibt und ist der
Grund, aus dem der Schnitt zu `in_den_papierkorb` noch einen Gegenstand hat.

**Der doppelte Wortlaut:** als Entscheidungsfrage weitergereicht, nicht gebaut. Die beiden
Wortlaute stehen wörtlich in der C3-Tafel des angenommenen Specs, und die zwei Möglichkeiten,
die die Doppelung wirklich auflösen, ändern beide diesen Text. Der Datensatz ist
`decisions/260818-0512_o_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`
mit vier Möglichkeiten samt Folgewirkungen; empfohlen ist Möglichkeit 2.

## Geänderte Dateien

- `crates/krk-ui/src/appkit/blaetter/mod.rs` — `bestaetigungsstelle`, Feld im `Blatt`, der
  Wächterzweig in `zeigen_mit_wahl`, zwei Proben, Modulkopf und `Blatt::neu`
- `crates/krk-ui/src/appkit/blaetter/konflikt.rs` — `waechter_anhaengen`, Modulkopf
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs` — die Vierer-Probe
- `crates/krk-ui/src/kommandos/operationen.rs` — Abschnittskopf und Doc von
  `waehrend_blatt_erlaubt`, zwei Probennamen, eine Fehlschlagsmeldung
- `crates/krk-ui/src/appkit/anwendung.rs` — Kopf von `kommando_ausfuehren`, Signatur und Doc
  von `loeschen_nach_rueckfrage`, `in_den_papierkorb`
- `crates/krk-ui/src/appkit/editor.rs` — die Begründung zur zurückgehaltenen Datei

## Datensätze

Geschlossen (`_o_` → `_c_`): `260817-1241`, `260817-1242`, `260817-1302`, `260817-1720`
(mit Verweis auf die Entscheidungsfrage), `260817-2243` (der `Art`-Parameter).

Offen geblieben: `260817-1419_o_ein-vierter-traeger-…`. Seine zweite Hälfte — dass eine
Erhebung über eine Kiste dieselbe Stelle wieder nicht sieht — ist erledigt und als Tabelle
angehängt; seine eigene Zeile in `resources/default-keymap.toml` ist vom Auftrag ausgenommen
und gehört daneben nicht dem `coder`.

Neu: die Entscheidungsfrage `260818-0512_o_wie-lautet-die-frage-…`.
