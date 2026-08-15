Zwei Modulkoepfe nennen das Verweisziel „am Deskriptor", obwohl es seit dem 260815 am Pfad fragt

---

`verweisziel::bestimmen` fragt seit der Behebung des Befunds `260815-1713` mit
`std::fs::metadata` am Pfad und nicht mehr mit `sys::ohne_warten_oeffnen` am Deskriptor.
Drei Stellen in zwei Dateien beschreiben noch den alten Mechanismus. Sie lagen ausserhalb
der Grenzen jenes Auftrags und sind deshalb stehen geblieben.

---

**Schwere:** mittel. Kein Fehlverhalten am Code; falsch ist die Beschreibung, und sie steht
an den zwei Stellen, an denen ein Leser den Mechanismus nachschlaegt. Genau eine solche
Beschreibung hat den behobenen Befund erst hervorgebracht.
**Gefunden von:** coder, beim Wechsel selbst
**Betroffen:** `crates/krk-core/src/verzeichnis/mod.rs:5-14`, `:16-22`, `:47-53`;
`crates/krk-ui/src/appkit/tabelle.rs:1403-1404`
**Domain:** code

## Die Stellen im Einzelnen

1. **`crates/krk-core/src/verzeichnis/mod.rs:5-14`** — die Modulskizze zieht einen Pfeil
   `sys ──> verweisziel`. Den gibt es nicht mehr: `verweisziel` bindet `sys` nicht.

2. **`crates/krk-core/src/verzeichnis/mod.rs:16-22`** — die Aufzaehlung der Aufrufer von
   `fcntl(2)`/`ohne_warten_oeffnen` fuehrt „seit dem Defekt `260814-1612` von
   [`verweisziel`]" auf. Die Huelle hat wieder genau zwei Aufrufer, den Editor und den
   Leseweg der Vorschau; `crates/krk-core/src/verzeichnis/sys.rs:15-16` fuehrt diese zwei
   bereits richtig, die Zeile in `mod.rs` widerspricht ihr jetzt.

3. **`crates/krk-core/src/verzeichnis/mod.rs:47-53`** — „[`verweisziel`] … haengt als
   einziges Modul unmittelbar an [`sys`]" und „Gefragt wird sie am Deskriptor". Beides ist
   falsch. Der Satz danach, dass der Lesevorgang keinen zusaetzlichen Systemaufruf bekommt,
   bleibt richtig.

4. **`crates/krk-ui/src/appkit/tabelle.rs:1403-1404`** — der Doc-Kommentar von
   `in_zeile_einsteigen` sagt „ueber [`verweisziel::bestimmen`] am Deskriptor". Die
   Verzweigung darunter ist richtig und bleibt; allein diese Angabe ist es nicht. Der
   uebrige Absatz, dass ein `stat` je Verknuepfung bei jeder Anzeige die Rechnung von L3 und
   L10 aenderte, bleibt ebenfalls richtig — er ist der Grund dafuer, dass ueberhaupt nur im
   Einstiegsweg gefragt wird, und nicht dafuer, wie gefragt wird.

## Warum das nicht im selben Zug behoben ist

Der Auftrag vom 260815-1749 zog seine Grenze ausdruecklich um
`crates/krk-core/src/verzeichnis/verweisziel.rs` und die zugehoerigen Proben und hielt fest,
dass die Verzweigung in `tabelle.rs` bleibt, wie sie ist. Die vier Stellen hier sind reine
Beschreibung; der Modulkopf von `verweisziel.rs` traegt die richtige Fassung samt Begruendung
und ist die Quelle, aus der sie nachgezogen werden koennen.

## Ablage

Gemeinsamer Speicher. Betrifft den Kern und die Oberflaeche und die Directive keiner Runde.

---
Abgleich 260815-1812 (reconciler): **Der Befund besteht an allen vier genannten Stellen,
und die Zahl im Rumpf dieses Datensatzes stimmt nicht mit seiner eigenen Liste überein.**

Am Baum gegengelesen, Stand `7fae5ba`:

| Stelle | Steht dort noch | Nachgelesen |
|---|---|---|
| `crates/krk-core/src/verzeichnis/mod.rs:13` | Pfeil `└──> verweisziel` unter `sys` | ja |
| `crates/krk-core/src/verzeichnis/mod.rs:20-22` | „und seit dem Defekt `260814-1612` von [`verweisziel`]" in der Aufzählung der `fcntl(2)`-Rufer | ja |
| `crates/krk-core/src/verzeichnis/mod.rs:47-49` | „haengt als einziges Modul unmittelbar an [`sys`]", „Gefragt wird sie am Deskriptor" | ja |
| `crates/krk-ui/src/appkit/tabelle.rs:1403-1404` | „ueber [`verweisziel::bestimmen`] am Deskriptor" | ja |

Der Widerspruch zu `crates/krk-core/src/verzeichnis/sys.rs:15-16` und `:787-792` besteht
ebenfalls unverändert: die Hülle führt dort richtig zwei Rufer, `mod.rs` führt drei.

**Zwei Zählangaben dieses Datensatzes tragen nicht.** Der Rumpf sagt „Drei Stellen in zwei
Dateien" und listet darunter vier, drei in `mod.rs` und eine in `tabelle.rs`. Der Titel sagt
„Zwei Modulkoepfe"; ein Modulkopf ist darunter, der von `verzeichnis/mod.rs`, die vierte
Stelle ist der Doc-Kommentar der Methode `in_zeile_einsteigen` und kein Modulkopf. Richtig
ist: **vier Beschreibungsstellen in zwei Dateien, davon drei in einem Modulkopf.** Titel und
Zahl sind hier nicht angefasst — der Titel benennt den Fehler und nicht den Tagesstand, und
die Liste im Rumpf ist vollständig; wer den Datensatz abarbeitet, geht nach der Liste und
nicht nach der Zahl davor.
