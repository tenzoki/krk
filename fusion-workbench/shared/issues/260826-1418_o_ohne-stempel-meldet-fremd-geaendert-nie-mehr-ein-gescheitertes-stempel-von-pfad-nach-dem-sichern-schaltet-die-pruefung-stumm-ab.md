Ohne Stempel meldet `fremd_geaendert` nie mehr: ein gescheitertes `Stempel::von_pfad` nach dem Sichern schaltet die Prüfung stumm ab

---

`Editormodell::fremd_geaendert` (`crates/krk-ui/src/editormodell.rs:1036-1041`) antwortet
`false`, sobald `self.stempel` `None` ist, auch wenn `self.pfad` eine Datei nennt. Der Doc-Kommentar
begründet das allein mit „Hält der Editor keine Datei". Es gibt aber einen zweiten Weg zu
`None` bei gehaltener Datei: `sichern` (`:996-1001`) setzt nach dem gelungenen Schreiben
`self.stempel = Stempel::von_pfad(pfad)`, und `Stempel::von_pfad` (`:357-363`) liefert `None`,
wenn `metadata` oder `modified` scheitert. Ab dann gilt für diese Datei bis zum nächsten Öffnen:
`sichern` überschreibt jede fremde Änderung ohne Rückhalt, und `fremdaenderung_melden` (`:1076`)
schweigt. Die Zusage aus C4, fremde Änderungen nicht ohne Zutun zu überschreiben, ist damit an
genau der Stelle aus, an der sie am teuersten fehlt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/editormodell.rs` (`fremd_geaendert`, `sichern`, `uebernehmen`)
**Baumstand:** `ca8072d`

## Wie der Fall entsteht

Zwischen `datei::sichern` (schreibt Nachbardatei, `rename`) und dem `stat(2)` darunter liegt
eine Spanne, in der ein fremdes Werkzeug die Datei wegräumen oder ihren Ordner unlesbar machen
kann. Dasselbe gilt für `uebernehmen` (`:778`): `Geladen::stempel` wird **vor** dem Lesen erhoben
(`:463`), und ein `metadata`-Fehler dort mit einem danach gelungenen `oeffnen` ist nicht
ausgeschlossen. Beides ist selten; der Punkt ist, dass der Zustand danach stumm ist und das
Modell ihn nicht von „keine Datei" unterscheidet.

## Warum das eine Invariante ohne Halter ist

Der Typ trägt `pfad: Option<PathBuf>` und `stempel: Option<Stempel>` getrennt; die Regel
„beides gesetzt oder beides leer" steht nirgends, weder als Kommentar noch als Probe. Die Probe
`der_stempel_kennt_eine_aenderung_von_aussen` (`:1872`) merkt in ihrem ersten Satz selbst an,
dass ohne gemerkten Stempel die Änderung von aussen keine Meldung fände — und misst dann nur den
Fall, in dem er da ist.

## Denkbarer Weg

`fremd_geaendert` behandelt `pfad = Some` mit `stempel = None` als „geändert" (dieselbe
vorsichtige Wahl, die der Doc-Kommentar an `Ladevorgang::starten` für die Reihenfolge des Stempels
trifft: lieber eine Meldung zu viel), oder `sichern` liefert bei gescheitertem Stempel
`Gescheitert`, obwohl geschrieben wurde. Der erste Weg ist eine Zeile und hält die Zusage.
