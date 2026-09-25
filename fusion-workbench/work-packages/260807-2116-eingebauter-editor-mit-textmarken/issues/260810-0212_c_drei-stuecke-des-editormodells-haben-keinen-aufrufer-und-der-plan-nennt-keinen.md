# Drei Stücke des Editormodells haben keinen Aufrufer, und der Plan nennt keinen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Umsetzung von S35, S36 und S37
**Betroffen:** `crates/krk-ui/src/editormodell.rs` (`Suchlauf::treffer`, `Editormodell::haelt_zurueck`, `Editormodell::suche_beenden`)
**Cross-references:** `planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` Abschnitte `#### 15.`, `#### 31.`, S37

---

## Der Befund

`editormodell.rs` trug seit S16 ein `#![allow(dead_code)]` am Dateikopf, mit der
ausgeschriebenen Ankündigung, die Zeile falle mit S37:

> Der letzte davon ist S37; **dann** ist die Zeile wegzunehmen.

Mit S37 ist sie weggenommen. Von den vierzehn Fundstellen, die sie abdeckte,
haben zehn ihren Aufrufer bekommen. Vier nicht:

| Stück | Aufrufer |
|---|---|
| `Editormodell::stempel` | kommt mit S31, dem Melden einer Änderung von außen |
| `Suchlauf::treffer` | **kein Schritt des Plans nennt einen** |
| `Editormodell::haelt_zurueck` | **kein Schritt des Plans nennt einen** |
| `Editormodell::suche_beenden` | **kein Schritt des Plans nennt einen** |

Alle vier tragen jetzt ein eigenes `#[allow(dead_code)]` mit dem Grund daran,
statt sich hinter einer Zeile am Dateikopf zu verstecken. Für `stempel` ist das
in Ordnung: der Schritt steht im Plan und ist datiert. Für die drei übrigen ist
es eine Ausnahme ohne Ablaufdatum, und genau das führt dieser Eintrag.

## Warum jedes der drei heute ohne Aufrufer ist

- **`Suchlauf::treffer`** gibt die ganze Trefferliste heraus. Die Oberfläche
  kommt mit `zahl`, `nummer`, `angesteuert` und `meldung` aus; die ganze Liste
  bräuchte, wer alle Treffer zugleich zeichnete, und das sagt C5 nicht zu.
- **`Editormodell::haelt_zurueck`** fragt, ob eine gelesene Datei auf die
  Nachfrage aus C4 wartet. Die Oberfläche erfährt das als `Ladeausgang` und
  beantwortet es im Rückruf des Blattes, ohne zwischendurch nachzusehen.
- **`Editormodell::suche_beenden`** beendet den Suchlauf. Der Spec sagt keinen
  Befehl zu, der das tut; die Suche endet von selbst beim Tippen
  (`bearbeiten`), beim Dateiwechsel (`uebernehmen`) und beim Schließen
  (`schliessen`), und jede dieser drei Stellen setzt das Feld unmittelbar.

## Was zu entscheiden ist

Zwei Wege, und die Wahl gehört nicht in einen Schritt, der Suchen und Ersetzen
baut:

1. **Streichen.** Drei Zugriffsfunktionen ohne Aufrufer sind drei Zusagen, die
   niemand einlöst; der Plan verlangt „nur bauen, was gebraucht wird".
2. **Stehen lassen und die Ausnahme führen.** `haelt_zurueck` ist eine
   naheliegende Frage, die ein späterer Zustandsbericht stellen könnte, und
   `suche_beenden` ist die Gegenseite von `suche_starten`.

Bis zur Entscheidung bleiben sie mit ihrer einzelnen Ausnahme stehen. Der
Arbeitsbereich ist grün, und die Proben am Dateiende fassen jedes der drei an —
tot ist keines, unaufgerufen sind alle drei.

---

## Nachtrag vom 260810: es sind vier, nicht drei

S31 ist gebaut, und `Editormodell::stempel` hat den angekündigten Aufrufer
**nicht** bekommen. Der Grund ist kein Versäumnis, sondern dieselbe Regel, die
S25 schon gezogen hat: das Melden einer fremden Änderung fragt über
`Editormodell::fremd_geaendert` und nicht mit einer zweiten, enger geschnittenen
Frage daneben. Damit steht der Vergleich an einer Stelle statt an zweien, und
der Stempel selbst muss das Modell nicht verlassen. Der neue Aufrufer heißt
`Editormodell::fremdaenderung_melden` und ruft `fremd_geaendert`.

Die Zeile in der Tabelle oben ist damit überholt: `stempel` ist das vierte Stück
ohne Aufrufer, und für ihn nennt der Plan seit S31 auch keinen mehr. Die beiden
Wege unter „Was zu entscheiden ist" gelten für ihn wie für die drei anderen;
`stempel` liegt dabei näher am Streichen als `haelt_zurueck`, weil seine einzige
Verwendung eine Probe am Dateiende ist.

Der Titel dieses Eintrags nennt weiter drei. Er bleibt, wie er ist: der
Dateiname ist aus dem Programmtext heraus zitiert, und ein zweiter Eintrag für
dieselbe Frage wäre die schlechtere Antwort als eine Zeile, die man lesen muss.

---
Resolved: **Weg 1, Streichen** — und zwar für alle vier Stücke, die der Nachtrag
zählt, nicht nur für die drei des Titels. `Suchlauf::treffer`,
`Editormodell::stempel`, `Editormodell::haelt_zurueck` und
`Editormodell::suche_beenden` sind aus `crates/krk-ui/src/editormodell.rs`
entfernt; die Datei trägt danach **kein** `#[allow(dead_code)]` mehr, weder am
Kopf noch an einem einzelnen Stück.

Geprüft wurde für jedes einzeln, ob in der Oberfläche eine Stelle steht, die es
rufen müsste — also ob der Befund ein fehlender Aufruf ist. Bei keinem der vier
gibt es sie:

- `Suchlauf::treffer`: die Oberfläche kommt mit `zahl`, `nummer`, `angesteuert`
  und `meldung` aus, und C5 sagt kein Zeichnen aller Treffer zugleich zu.
- `Editormodell::stempel`: gefragt wird über `fremd_geaendert`, damit der
  Vergleich an einer Stelle steht statt an zweien (S25, S31). Der Stempel muss
  das Modell dafür nicht verlassen.
- `Editormodell::haelt_zurueck`: die Oberfläche erfährt das Zurückhalten als
  `Ladeausgang` und antwortet im Rückruf des Blattes
  (`anwendung.rs::editorausgang_behandeln`, `anlass_ausfuehren`,
  `anlass_unterbleibt`); dazwischen sieht niemand nach.
- `Editormodell::suche_beenden`: der Spec sagt keinen Befehl zu, der eine Suche
  beendet. Sie endet in `bearbeiten`, `uebernehmen` und `schliessen`, und jede
  der drei Stellen setzt das Feld unmittelbar; eine `pub fn` daneben behauptete
  einen Befehl, den es nicht gibt.

Belegt mit `grep -rn` über `crates/`: außer den Proben am Dateiende rief keines
der vier je etwas.

**Die zehn Zusicherungen, die an den vier hingen, sind nicht gefallen, sondern
umgestellt.** Sechs von ihnen standen in der Zeile daneben schon; die übrigen
vier fragen jetzt über das Verhalten statt über ein Feld:

- Sechs waren mit einer benachbarten Zusicherung deckungsgleich und sind mit
  einem Kommentar an ihrer Stelle ersetzt, der sagt, welche Zeile die Aussage
  trägt. Beispiele: `lauf.treffer()` nannte die Versätze `[0, 10, 20]`, die die
  drei `weitersuchen`-Zeilen darunter ohnehin einzeln nennen, dazu noch den
  Umlauf; `!haelt_zurueck()` nach `Ladeausgang::Geoeffnet` folgt daraus, dass
  die vier Werte von `Ladeausgang` überschneidungsfrei sind.
- Vier fragen jetzt über `zurueckgehaltenes_uebernehmen() == None`, also über den
  Weg, der eine wartende Datei aufnehmen würde: findet er nichts, wartet nichts.
  Das ist die stärkere Probe, weil sie das Verhalten misst und nicht ein Feld.
- Die Zusicherung `modell.suche_beenden(); …` in `die_suche_zaehlt_und_laeuft_um`
  ist ganz gefallen: `eine_bearbeitung_beendet_den_suchlauf` prüft dieselbe
  Aussage auf einem Weg, den das Programm wirklich geht.

Der Modulkopf von `editormodell.rs` führt die vier weiter namentlich, aber im
Perfekt: er sagt, was sie waren, warum keines gebraucht wurde und dass die Datei
jetzt ohne Ausnahme von der Totprüfung auskommt.

Abnahme am 260810-1030: `cargo build --workspace`, `cargo test --workspace`,
`cargo clippy --workspace --all-targets` und `cargo fmt --all --check` jeweils
`exit 0`.
