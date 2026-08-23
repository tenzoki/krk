Ein Klick in das andere Dateifenster nimmt eine Ziehbewegung zurück

---

`aktives_setzen` (`anwendung.rs:4320-4325`) ruft `aufteilung_nachziehen`, ohne vorher
`bildschirmbreiten_uebernehmen` zu rufen. Wer eine Trennlinie mit der Maus verschiebt und
danach ohne Tastendruck in das **andere** Dateifenster klickt, bekommt die Trennlinie an ihre
alte Lage zurückgeschoben: der Nachzug legt aus dem Fenstermodell aus, und dort steht die
Ziehbewegung noch nicht.

---

**Gemessen am Baumstand `df8163d`. Nicht am laufenden Bündel bestätigt** — die Abnahme verlangt
KRK im Vordergrund und ist Nutzerarbeit. Der Befund steht auf Codelektüre.

## Der Weg

```rust
// crates/krk-ui/src/appkit/anwendung.rs:4320
fn aktives_setzen(&self, seite: Fensterseite) {
    if self.ivars().modell.borrow_mut().aktiv_setzen(seite) {
        self.aufteilung_nachziehen();
        self.sitzung_vormerken();
    }
}
```

`aufteilung_nachziehen` (`:4530`) liest `modell.breiten()` und schreibt sie über
`Aufteilung::anwenden` (`aufteilung.rs:322-330`) auf den Schirm: erst
`delegierter.wuensche_merken(breiten)`, dann `auslegen(&teiler, breiten)`, und `auslegen`
setzt die Rahmen aller fünf Bereiche neu (`aufteilung.rs:586-605`).

**Eine Ziehbewegung steht zu diesem Zeitpunkt weder im Modell noch bei den Wünschen des
Delegierten.** Das Ziehen einer Trennlinie schreibt die neuen Rahmen unmittelbar in die
Unteransichten; `splitView:resizeSubviewsWithOldSize:` (`aufteilung.rs:198-209`) — die einzige
Stelle, an der der Delegierte seine Wünsche nachführt — feuert dabei nicht, sie feuert beim
Größerziehen des Fensters. Der einzige Weg vom Schirm zurück in das Modell ist
`bildschirmbreiten_uebernehmen` (`:4507`), und es hat genau zwei Rufer: den Kopf von
`kommando_ausfuehren` (`:2991`) und `sitzung_bauen` (`:7061`). Beide liegen **nicht** vor
diesem Nachzug: `sitzung_vormerken` steht in `aktives_setzen` eine Zeile **danach**, also misst
es die schon zurückgeschobene Lage und schreibt sie in die Sitzung.

## Die Folge steht am Nachbarn ausdrücklich da

Der Doc-Kommentar von `aufteilung_nachziehen` (`:4523-4529`) sagt die Bedingung selbst:

```
    /// **Das Modell ist hier die Quelle, und deshalb muss es aktuell sein.**
    /// Wer diese Funktion ruft, hat vorher entweder das Modell geaendert oder
    /// [`Self::bildschirmbreiten_uebernehmen`] gerufen; sonst schreibt sie eine
    /// ueberholte Breite auf den Schirm und nimmt dem Nutzer seine
    /// Ziehbewegung.
```

`aktives_setzen` ändert das Modell — aber nur `aktiv`, nicht die Breiten. Die Bedingung ist im
Buchstaben erfüllt und in der Sache nicht.

## Wann es eintritt

Der Nachzug läuft nur, wenn `aktiv_setzen` **true** liefert, also wenn der Klick das aktive
Dateifenster wechselt. Die Folge ist damit:

1. Trennlinie mit der Maus verschieben.
2. Ohne Tastendruck dazwischen in das andere Dateifenster klicken.
3. Die Trennlinie springt zurück.

Ein Tastendruck dazwischen fängt die Bewegung ab (`kommando_ausfuehren` misst am Kopf), ein
Klick in dasselbe Dateifenster ebenfalls (`aktiv_setzen` liefert false). Beides macht den Fall
schmal, aber nicht selten: Trennlinie ziehen und dann in die andere Liste greifen ist eine
gewöhnliche Folge.

Über `aktives_dem_ersthelfer_nachziehen` (`:4369-4374`) gilt dasselbe für jeden Klick in eine
Fläche des anderen Dateifensters, die keine Zeile ist — seit dem Nutzerentscheid vom 260819
holt jede Fläche den Fokus.

## Alter

Nicht durch `df8163d` entstanden. `git blame` weist `self.aufteilung_nachziehen();` in
`aktives_setzen` als `537fda53` vom 260804 aus; der zweite Rufer,
`aktives_dem_ersthelfer_nachziehen`, kam mit `76ceb683` vom 260819 dazu und hat die Reichweite
verbreitert. Gefunden bei der Durchsicht von `df8163d`, weil dieser Commit die Aufrufliste von
`aufteilung_nachziehen` erweitert und dabei die Prosastelle stehen ließ, die genau diesen Fall
als geprüft ausgibt (`shared/issues/260823-0730_*`, Punkt 1).

## Vorschlag

Der naheliegende Griff — `bildschirmbreiten_uebernehmen()` an den Kopf von `aktives_setzen` —
löst den Fall, setzt aber die Aufrufliste fort, deren Unvollständigkeit `df8163d` gerade erst
als Fehlerquelle erwiesen hat. Die Frage, die vorher zu beantworten ist: soll die Messung an
die Rufer oder in `aufteilung_nachziehen` selbst? Letzteres wäre die Entsprechung zu dem, was
`df8163d` für die Sichtbarkeit getan hat — die Zusage an die Quelle statt an die
Vollständigkeit einer Liste —, hieße aber, dass der Nachzug am Kopf von `kommando_ausfuehren`
und in `sitzung_bauen` doppelt misst und dass die Reihenfolgezusage von
`bildschirmbreiten_uebernehmen` („gemessen wird, solange Modell und Schirm dieselbe
Sichtbarkeit meinen", `:4489-4498`) neu zu prüfen ist. Sie ist nicht offensichtlich haltbar,
wenn die Messung hinter eine Modelländerung rutscht. Das gehört entschieden und nicht
nebenbei gegriffen.

**Schwere:** mittel. Sichtbares Fehlverhalten, kein Datenverlust; die zurückgeschobene Lage
geht anschließend über `sitzung_vormerken` in die `session.toml` und überlebt damit den Neustart.

**Prüfen im Abnahmelauf:** Schritte 1 bis 3 oben, mit jeder der vier Trennlinien.

**Gefunden:** coderev, Durchsicht des Commits `df8163d` am 260823-0731, Bereich
`ab11eb8..df8163d`

**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:4320-4325`, `:4369-4374`

**Domain:** code

**Verwandt:**
`shared/issues/260823-0730_o_drei-prosastellen-um-den-neuen-nachzug-sind-mit-df8163d-falsch-geworden.md`
— Punkt 1: die Prosastelle, die diesen Aufrufer als unbedenklich mitzählt, ohne ihn zu nennen.
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0539_*_ein-zusammengezogenes-fenster-ersetzt-die-aufteilung-des-nutzers-dauerhaft.md`
— derselbe Gegenstand von der anderen Seite: dort übernahm das Modell eine Zahl, die es nicht
übernehmen durfte, hier übernimmt es eine nicht, die es übernehmen müsste.
