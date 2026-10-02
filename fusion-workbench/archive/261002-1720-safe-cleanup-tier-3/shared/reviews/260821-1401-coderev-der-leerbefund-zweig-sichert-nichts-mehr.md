# Durchsicht: der Leerbefund-Zweig sichert nichts mehr und sperrt keinen Platz

**Reviewed-range:** `073448e..d771ec6`
**Not-opened:** none
**Sender:** coderev
**Datum:** 260821-1401

## Zusammenfassung

Der Commit `d771ec6` behebt, was er zu beheben angibt. Die Vierschrittfolge ist zu, und ich
habe das nicht der Notiz geglaubt, sondern gemessen: mit zurückgenommener Codeänderung
scheitert die neue Probe, und sie scheitert an zwei voneinander unabhängigen Stellen, also
misst sie den Verlust und nicht bloß den neuen Rückgabewert. Die Erkennung aus `073448e` ist
unversehrt, die Meldung stimmt an allen erreichbaren Zweigen, die Zählprobe zählt weiter fünf.
Drei Befunde bleiben: einer verhaltensbezogen und mittel, zwei an der Prosa und niedrig.

## Zahlen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 1 |
| Niedrig | 2 |

Dazu zwei Beobachtungen ohne eigenen Datensatz und ein Nachtrag an einem offenen.

## Was gemessen ist, und wie

Der Bereich sind zwei Codedateien, 197 Zeilen. Beide vollständig gelesen, dazu
`ablage/atomar.rs`, `ablage/pfade.rs`, `ablage/einstellungen.rs` und `tests/baum.rs` als
Umgebung. Vier Messungen an einem Arbeitsbaum auf `d771ec6`:

1. **Die Codeänderung zurückgenommen, `cargo test -p krk-core --test ablage`.** Drei Proben
   werden rot, darunter die neue:

   ```
   ---- nach_einem_leerbefund_bleibt_der_sicherungsplatz_fuer_den_echten_bestand_frei ----
   panicked at crates/krk-core/tests/ablage.rs:2916:5:
   der Leerbefund hat den einen Sicherungsplatz belegt
   ```

2. **Zusätzlich die Zusicherung aus Schritt 1 ausgehängt**, um zu sehen, ob die Schritte 2 bis
   4 selbst tragen oder nur im Schatten von Schritt 1 stehen. Sie tragen:

   ```
   panicked at crates/krk-core/tests/ablage.rs:2930:5:
   assertion `left == right` failed: der Bestand ist nicht gesichert worden
     left: SchonVorhanden(".../bookmarks.toml.beschaedigt")
    right: Gesichert(".../bookmarks.toml.beschaedigt")
   ```

3. **Sauberer Baum, `--test ablage --test baum`:** 71 + 4 grün, 5 ignoriert.

4. **Ein eigener Probelauf über `Ablage::oeffnen`, `durchgang`, `laden`** in der umgekehrten
   Reihenfolge — erst der echte Bestand, dann die schlüssellose Datei. Ausgabe unten beim
   ersten Befund.

## Die sechs Fragen der Beauftragung

### 1. Ist die Vierschrittfolge wirklich zu? Ja, und die Probe misst sie.

Der Coder sagt, die Probe habe vor der Codeänderung im Baum gestanden und sei scheitern
gesehen worden. Das ist nachgemessen (Messung 1) und stimmt. Die Frage der Beauftragung war
schärfer: misst sie die Folge, oder fragt sie bloß den neuen Rückgabewert ab? Sie misst sie.

Schritt 1 prüft `!sicherung.try_exists()` — eine Aussage über das Dateisystem, nicht über einen
`enum`-Wert. Und Messung 2 zeigt, dass Schritt 2 unabhängig davon greift: er stellt
`SchonVorhanden` gegen `Gesichert` fest, also genau den Zustand, aus dem der Verlust folgte.
Schritt 3 vergleicht den Inhalt der Sicherung mit dem Bestand, Schritt 4 fährt den gewöhnlichen
Schreibvorgang und prüft, dass er die Sicherung stehen lässt. Vier Zusicherungen, von denen
mindestens zwei einzeln rot würden. Ein späterer Umbau, der den Platz auf anderem Weg belegt,
fällt hier auf.

`crates/krk-core/tests/ablage.rs:2904-2960`.

### 2. Ist die Erkennung unversehrt geblieben? Ja, vollständig.

Der Zweig steht unverändert an derselben Stelle und mit derselben Eintrittsbedingung
(`crates/krk-core/src/ablage/mod.rs:609`); allein der Rumpf gibt jetzt `Beiseite::Nicht` zurück
statt `beiseite_legen` zu rufen. Damit steht weiterhin:

- `wert: T::default()` — der Auslieferungszustand springt ein (`mod.rs:611`)
- `ersetzung: Some(...)` — es ist **kein** erster Start (`mod.rs:612`)
- `Grund::Beschaedigt("die Datei traegt keinen einzigen obersten Schluessel, und KRK schreibt
  sie nie so")` — der Nutzer bekommt eine Meldung (`mod.rs:614-618`)

Der gemeinsame Rumpf der drei Proben prüft alle drei weiter, und die Zusicherung „die Datei
wurde nicht überschrieben" steht jetzt **außerhalb** der neuen Fallunterscheidung
(`tests/ablage.rs:2836-2840`), gilt also für beide Spuren. `Datei::leerbefund` ist unangetastet
(`pfade.rs:234-241`), `#[serde(deny_unknown_fields)]` an `Lesezeichenliste` ebenfalls
(`lesezeichen.rs:350`). Zu viel zurückgenommen ist nichts.

### 3. Die Meldung. Stimmt an jedem erreichbaren Zweig.

Für den Leerbefund ist jetzt genau ein `Display`-Zweig erreichbar, der von `Beiseite::Nicht`
(`mod.rs:385-390`). Sein Satz nennt keinen zweiten Pfad und verspricht nichts:

```
.../bookmarks.toml ist beschaedigt und wird durch den Auslieferungszustand ersetzt:
die Datei traegt keinen einzigen obersten Schluessel, und KRK schreibt sie nie so
```

Der Coder hat recht: `Display` musste nicht angefasst werden. Die Probe hält die Zusage in der
richtigen Form fest — sie prüft, dass die Meldung den Beiseitepfad **nicht nennt**, statt einen
Wortlaut zu vergleichen (`tests/ablage.rs:2829-2833`); ein umformulierter Satz lässt sie grün,
eine wiedereingebaute Pfadnennung nicht.

Für die andere Hälfte des Zweiges — der Leser meldet einen Fehler — sind alle fünf
`Beiseite`-Zweige weiter erreichbar und unverändert.

### 4. Ein dritter Zustand? Die Fallunterscheidung ist vollständig, eine Reihenfolge verliert etwas.

`Beiseite` trägt unverändert fünf Werte und keinen Auffangzweig; `Display` und die Proben
nehmen sie vollständig auseinander. Der Übersetzer hält das.

Der Gegenstand des Zweiges ist dabei wasserdicht: `ohne_obersten_schluessel` fragt
`toml::from_str::<toml::Table>(text).is_ok_and(|d| d.is_empty())` (`mod.rs:880-882`). Eine
Datei, die diese Frage bejaht, ist gültiges TOML mit null Schlüsseln — also nur Kommentare und
Leerraum. Sie kann `eintraege` nie tragen. Die Begründung des Datensatzes hält also nicht bloß
für den 0-Byte-Fall, sondern für jede Datei, die der Zweig fängt.

**Eine Reihenfolge verhält sich trotzdem anders als vorher**, und sie ist in der
`Resolved:`-Notiz nicht benannt. Umgekehrt zur gemessenen Folge — erst der echte Bestand, dann
die schlüssellose Datei — fällt mit `beiseite_legen` auch dessen Frage weg, ob unter dem
Beiseitepfad schon etwas steht. Gemessen:

```
SCHRITT1 beiseite=Gesichert(".../bookmarks.toml.beschaedigt")
SCHRITT2 beiseite=Nicht
SCHRITT2 meldung=... ist beschaedigt und wird durch den Auslieferungszustand ersetzt:
        die Datei traegt keinen einzigen obersten Schluessel, ...
SCHRITT2 sicherung_da=Ok(true)
        inhalt=Some("[[lesezeichen]]\nname = \"P\"\nordner = \"/\"\n")
```

Die Sicherung mit dem echten Bestand liegt da und ist vollständig. Die Meldung nennt sie nicht
mehr; vor `d771ec6` tat sie es über `Beiseite::SchonVorhanden`. Kein Datenverlust, aber eine
fehlende Auskunft an genau den Nutzer, der seinen Bestand gerade sucht — die Reihenfolge
entsteht, wenn er `bookmarks.toml` von Hand leert, um den alten Bestand hineinzukopieren.

Datensatz:
`shared/issues/260821-1401_o_der-leerbefund-zweig-verschweigt-eine-dastehende-sicherung-die-den-bestand-traegt.md`
(Schwere mittel, drei Wege zur Wahl, darunter „stehen lassen und den Preis aufschreiben").

### 5. `nur_benannte_dateien_erreichen_das_atomare_schreiben`. Zählt fünf, und der Grund trägt.

Nachgezählt und grün. Wichtiger als das Ergebnis ist, warum es nicht wackeln kann: die Probe
zählt **Dateien**, nicht Rufstellen (`crates/krk-core/tests/baum.rs:178-206`). Sie filtert die
Quelldateien des Baums danach, ob `atomar::schreiben` darin vorkommt. `beiseite_legen` steht
weiter in `mod.rs`, also bleibt `mod.rs` in der Liste — ein Rufer weniger ändert daran nichts.

Auch der Kommentar in der Probe bleibt richtig: „Vier Schreiber hinter einem `Zugang`:
`Zugang::sichern`, `Zugang::text_sichern`, `Zugang::beiseite_legen` und die Anlage von
`settings.toml`" (`baum.rs:192-194`). `beiseite_legen` ist weiterhin ein Schreiber, es hat nur
zwei Rufer statt drei.

### 6. Vier Prosastellen, und der ganze Modulkopf. Zwei Befunde, eine Zählung daneben.

Der Modulkopf ist Zeile für Zeile gegen den Baum gelesen. Das meiste hält: „sechs Dateien in
zwei Formaten" stimmt, „Sechs Module" stimmt, „Vier Regeln tragen den Vorgang" stimmt weiter
(es sind vier Aufzählungspunkte), „drei der vier TOML-Dateien tragen `deny_unknown_fields`"
stimmt (`belegung.rs:1587`, `einstellungen.rs:125`, `lesezeichen.rs:350`; `Sitzung` nicht),
„eine sechste lässt sie rot werden" stimmt zur Fünferliste in `baum.rs`. Der neue Absatz zur
dritten Gestalt des Verlusts (`mod.rs:162-167`) ist sachlich richtig.

Zwei mit diesem Commit neu geschriebene Stellen geben ihren Umfang falsch an:

- **Regel 1 im Modulkopf** (`mod.rs:104-107`) sagt unbedingt, eine Datei ohne obersten
  Schlüssel gelte als beschädigt. Das gilt allein für `bookmarks.toml`; `Datei::leerbefund`
  entscheidet es je Datei. Vierzehn Zeilen weiter steht „die vier Regeln gelten dort für alle
  vier gleich" (`mod.rs:117-119`) — zusammengenommen führt das auf den Schluss, eine leere
  `session.toml` sei beschädigt. Der Kopf sagt es weiter unten selbst richtig
  (`mod.rs:145-152`) und widerspricht sich damit seit diesem Commit.
- **Der Doc-Kommentar an `Beiseite::Nicht`** (`mod.rs:294-300`) zählt „drei Fälle". Der zweite
  genannte — „eine fehlende Datei ist der erste Start" — erzeugt gar keinen `Beiseite`-Wert,
  sondern `ersetzung: None` (`mod.rs:592-596`). Und ein vierter Erzeuger bleibt ungezählt:
  `einstellungen.rs:169`, `Grund::NichtAnlegbar`. Die alte Fassung stand als **Regel** da („der
  Wert jeder Ersetzung außer der beschädigten") und deckte alle vier; `d771ec6` hat eine Regel
  durch eine Zählung ersetzt, und die Zählung war bei ihrem Entstehen falsch. Genau diesen
  Wechsel führt `CLAUDE.md` als wiederkehrende Fehlerquelle dieses Projekts.

Datensatz:
`shared/issues/260821-1401_o_zwei-mit-d771ec6-neu-geschriebene-prosastellen-der-ablage-geben-ihren-umfang-falsch-an.md`
(Schwere niedrig).

**Es sind fünf mitgezogene Prosastellen, nicht vier.** Commit-Nachricht und `Resolved:`-Notiz
nennen vier; der Diff ändert daneben den Absatz „Die Zusage deckt weiterhin nicht jede Gestalt
des Verlusts" um fünf Zeilen (`mod.rs:162-167`). Der angefügte Text ist richtig, allein die
Zahl daneben nicht. Kein eigener Datensatz — im Prosabefund vermerkt.

## Nachtrag statt eines zweiten Datensatzes

`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-nennen-die-zahl-der-dateien-und-den-umfang-von-leerbefund-falsch.md`
hat einen Nachtrag mit den geprüften neuen Zeilennummern bekommen. Beim Nachzählen kam heraus,
dass die Verschiebungstabelle im geschlossenen Datensatz **um genau eine Zeile zu niedrig
steht**, und zwar an allen fünf Einträgen: `142→150` statt `151`, `425→460` statt `461`,
`427→462` statt `463`, `467→502` statt `503`, `508→543` statt `544`. Wer die Zahlen von dort
übernimmt, landet auf einer Leerzeile, auf `///` oder auf `impl Zugang<'_> {`. Der offene
Datensatz trägt jetzt die geprüfte Tabelle.

## Eine Beobachtung außerhalb des Bereichs

`cargo doc -p krk-core --no-deps` meldet vier unauflösbare Doc-Verweise in der Ablage:
`[Ablage::laden]` (`mod.rs:70`, `:87` und `tasten/belegung.rs:27`) und `[Ablage::sichern]`
(`mod.rs:62`, `einstellungen.rs:25`). Beide Methoden hängen an `Zugang`, nicht an `Ablage`.
Sie sind älter als dieser Commit, und `d771ec6` hat keinen neuen dazugelegt. Unbemerkt bleiben
sie, weil `make check` die vier Abnahmekommandos fährt und `cargo doc` nicht darunter ist
(`Makefile:56`). Kein Datensatz von mir — das ist eine eigene Frage („soll `cargo doc` in
`make check`?"), und sie gehört dem Nutzer, nicht einem Defektdatensatz.

## Reihenfolge

- **Kein Auslieferungshindernis.** `d771ec6` ist eine Verbesserung gegenüber `073448e` und
  gegenüber `01d2365`; der gemessene Verlustweg ist zu.
- **Als Nächstes:** der mittlere Befund. Er ist eine Entscheidung und keine Reparatur — Weg 3
  („stehen lassen, den Preis aufschreiben") ist vertretbar und kostet nur zwei Sätze Prosa.
- **Beim nächsten Anfassen der Ablage:** die beiden Prosabefunde und die sieben Stellen des
  offenen Datensatzes von `260821-1023` in einem Zug. Sie liegen in derselben Datei und
  teilweise in denselben Absätzen.
