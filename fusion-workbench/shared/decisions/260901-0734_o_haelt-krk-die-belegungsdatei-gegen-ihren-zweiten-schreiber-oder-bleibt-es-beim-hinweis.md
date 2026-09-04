# Hält KRK die Belegungsdatei gegen ihren zweiten Schreiber, oder bleibt es beim Hinweis?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/tasten/belegung.rs` (`fuer_den_betrieb`, `Belegung::sichern`), `crates/krk-ui/src/appkit/anwendung.rs` (`Anwendungsdelegierter::belegungsansicht_verlassen`, `Anwendungsdelegierter::belegungsdatei_ansehen`), `crates/krk-ui/src/kommandos/operationen.rs` (`belegungsdatei_hat_zwei_schreiber`), `shared/decisions/260821-0142_*_gilt-die-strenge-bestandsregel-auch-fuer-session-toml-und-keymap-toml.md`, `shared/decisions/260813-0053_*_was-teilen-sich-zwei-instanzen-an-der-ablage-und-wer-schreibt-die-sitzung.md`

---

## Question

`~/Library/Application Support/KRK/keymap.toml` hat zwei Schreiber, und keiner von beiden
weiß vom anderen.

Der eine ist der Nutzer mit einem Editor. KRK liest die Datei genau einmal, beim Start
(`fuer_den_betrieb`), und lädt sie im Betrieb nicht nach; eine Änderung von Hand wirkt
deshalb erst beim nächsten Start.

Der andere ist die Belegungsansicht auf `f1`. Sie arbeitet auf einer Arbeitskopie, die aus
dem Stand des **Starts** gebaut ist, und schreibt sie beim Verlassen als Ganzes zurück,
sobald sich etwas geändert hat (`belegungsansicht_verlassen` → `Belegung::sichern`). Jede
Handänderung seit dem Start ist danach fort, ohne Rückfrage und ohne Meldung.

Der Nutzerauftrag vom 260901 hat den Menübefehl „Tastaturdefinition öffnen" gebaut, der
die Datei in die Vorschau stellt und damit den Weg zum Ändern von Hand erst bequem macht.
Er hat dabei die Vorbelegung angenommen: **KRK tut nichts dagegen, aber sagt es.** Ein Satz
in der Statuszeile nennt beim Öffnen beide Schreiber und beide Folgen. Ob es dabei bleibt,
ist die Frage hier; sie muss beantwortet sein, bevor jemand die nächste Stufe baut, weil
jede der drei unten den Ladeweg oder den Schreibweg anfasst.

## Options

1. **Es bleibt beim Hinweis** — der gebaute Stand. Kein Neuladen, keine Sperre, keine
   Warnung beim Überschreiben; der Satz beim Öffnen ist die ganze Vorkehrung.
   - Pro: nichts zu bauen, nichts zu messen. Der Nutzer, der die Datei von Hand ändert,
     ist genau der, der den Hinweis gelesen hat, denn er hat sie über diesen Befehl geöffnet.
   - Contra: die Warnung erreicht nicht, wer die Datei außerhalb von KRK öffnet. Und sie
     steht auf Rang 1 der Statuszeile, fällt also mit dem nächsten Tastenbefehl.
2. **KRK lädt `keymap.toml` im Betrieb nach**, sobald sie sich auf der Platte ändert.
   - Pro: die Handänderung wirkt sofort, und die Arbeitskopie der Belegungsansicht baut
     auf dem neuen Stand auf; damit fällt der zweite Schreiber als Problem weg.
   - Contra: die Beobachtung des Ablageordners ist ein zweiter Bestand neben
     `Dateisystemwache`, die heute Ordner der Dateilisten beobachtet. Ein Nachladen
     mitten im Betrieb baut Hauptmenü und Ereignisabgriff neu auf — derselbe Weg, den
     `belegungsansicht_verlassen` schon geht, aber zu einem Zeitpunkt, den der Nutzer
     nicht ausgelöst hat.
3. **Die Belegungsansicht liest vor dem Sichern nach und fragt bei Abweichung** —
   der schmalste Eingriff: nicht der Ladeweg ändert sich, sondern der Schreibweg.
   - Pro: trifft genau den Verlustfall, und nur ihn. Kein Beobachter, kein Nachladen.
   - Contra: eine vierte Rückfrage am Hauptfenster, und sie kommt beim Verlassen eines
     Blattes, also an einer Stelle, an der der Nutzer schon fertig zu sein glaubt.
     Was „Abweichung" heißt, ist dabei zu entscheiden: der Dateiinhalt Byte für Byte
     oder die gelesene Belegung.

## Constraints

- Der Ladeweg beim Start ist die eine Stelle, an der `keymap.toml` gelesen wird
  (`fuer_den_betrieb`), und der Schreibweg beim Verlassen der Belegungsansicht die eine,
  an der sie geschrieben wird (`Belegung::sichern` unter der Schreibsperre). Jede Antwort
  hier fasst genau eine der beiden an; eine dritte Stelle entsteht nicht.
- Die Schreibsperre der Ablage schützt gegen eine **zweite Instanz von KRK**, nicht gegen
  einen fremden Editor. Sie ist für diese Frage kein Mittel.
- Eine fehlende `keymap.toml` bleibt der erste Start und liefert die Auslieferungsbelegung
  ohne Meldung; keine Antwort darf daraus einen Fehlerfall machen.

## Recommendation

Keine. Die Kosten der Möglichkeiten 2 und 3 sind zu verschieden, um sie ohne den Nutzer zu
wägen: die zweite kauft die Bequemlichkeit mit einem neuen Beobachter, die dritte den
Schutz mit einer weiteren Rückfrage. Möglichkeit 1 ist der gebaute Stand und braucht keine
Entscheidung, um zu gelten — sie braucht eine, um zu **bleiben**, und deshalb steht dieser
Datensatz hier.
