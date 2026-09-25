Die Sitzungsgeschichte sagt "Gekuerzt wird das Blatt allein beim Zip"; der Baum kuerzt auch beim Entpacken eines einzelnen Archivs

---

Die fuenfte Nutzerentscheidung dieser Runde ist in `shared/history/260824-2120-orchestrator-session.md`
mit dem Satz festgehalten: "Gekuerzt wird das Blatt allein beim Zip." Der Baum kuerzt auch beim
Entpacken, sobald der Vorgang genau **ein** Archiv traegt. Beide Lesarten sind vertretbar, aber es
sind zwei, und der Datensatz `260825-0711_*_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`
steht vor dem Hochstufen auf "umgesetzt".

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Die zwei Aussagen nebeneinander

- `fusion-workbench/shared/history/260824-2120-orchestrator-session.md`, Punkt 5 der Entscheidungen:
  "bei mehreren Archiven erzeugt der Vorgang mehrere Ziele, also greift `erzeugt_genau_ein_ziel`
  nicht, das volle Blatt erscheint samt Ankreuzfeld ... **Gekuerzt wird das Blatt allein beim Zip.**"
- `crates/krk-ui/src/kommandos/operationen.rs:482-491`:

  ```rust
  Art::Zippen { .. } => true,
  Art::Entpacken { ziele } => ziele.len() == 1,
  ```

Ein Unzip ueber genau ein Archiv liefert damit `true`, und das Blatt erscheint in der gekuerzten
Gestalt: Ueberschreiben, Umbenennen, Abbrechen, ohne Ankreuzfeld.

## Welcher Fall das ist

Der haeufigste. Die Directive beschreibt Unzip in der Einzahl, und die Ersatzregel liefert
ausdruecklich **ein** Archiv; erst die dritte Nutzerentscheidung hat mehrere ueberhaupt moeglich
gemacht. Wer ein Archiv zum zweiten Mal entpackt, sieht also die gekuerzte Gestalt, und die
Sitzungsgeschichte sagt, das solle nicht sein.

## Wofuer die Kuerzung an dieser Stelle spricht

Der Doc-Kommentar in `operationen.rs:467-471` fuehrt es aus, und die Begruendung traegt: bei genau
einem Ziel faellt "Ueberspringen" mit "Abbrechen" zusammen, und das Ankreuzfeld hat keinen weiteren
Fall, fuer den es gelten koennte. Das sind genau die zwei Gruende aus dem Datensatz, und sie haengen
an der **Zahl der Ziele** und nicht an der Vorgangsart. Die Codefassung ist damit die konsequentere
Lesart derselben Entscheidung.

## Was zu tun ist

Der Nutzer sagt, welche der zwei Lesarten bindet, und die unterlegene Stelle wird nachgezogen:

1. **Die Codefassung bindet.** Der Satz in der Sitzungsgeschichte ist zu berichtigen, und der
   Datensatz `260825-0711_*_welche-antworten-bietet-das-konfliktblatt-bei-genau-einer-zieldatei.md`
   bekommt eine Zeile, die die Ausweitung auf das Entpacken ausschreibt. Danach ist er umgesetzt.
2. **Der Satz der Sitzungsgeschichte bindet.** `erzeugt_genau_ein_ziel` liefert fuer
   `Art::Entpacken` immer `false`, und die Tafel im Doc-Kommentar wie die Probe
   `die_tafel_ueber_alle_sechs_werte` ziehen nach. Der Preis ist ein Blatt, dessen Ankreuzfeld und
   dessen "Ueberspringen" bei einem einzelnen Archiv beide keinen Gegenstand haben — genau die Lage,
   die der Datensatz fuer das Packen als vermeidenswert benennt.

Solange die Frage offen ist, traegt `260825-0711_*_welche-antworten-bietet-das-konfliktblatt-...`
kein `Implemented:`: die Gestalt ist gebaut, ihr Geltungsbereich ist es nicht.

## Umfang

`krk-ui`, `kommandos/operationen.rs` und die Sitzungsgeschichte. Das Blatt selbst
(`appkit/blaetter/konflikt.rs`) ist in beiden Faellen unveraendert: es kennt die Art nicht und
bekommt die Antwort gereicht.

---
Resolved: Die Sitzungsgeschichte war die falsche Seite, nicht der Baum. Der Nutzerentscheid lautet
"bei genau einer Zieldatei", und ein Entpacken eines einzelnen Archivs erzeugt genau ein Ziel;
`erzeugt_genau_ein_ziel` haengt beim Entpacken deshalb zu Recht an `ziele.len() == 1`. Der zu enge
Satz in `shared/history/260824-2120-orchestrator-session.md` ist berichtigt, und der Abschnitt
"Berichtigung, 260825: die Reichweite des gekuerzten Konfliktblatts" derselben Datei schreibt aus,
warum. Am Code ist nichts geaendert.
