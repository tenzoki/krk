# Was teilen sich zwei Instanzen an der Ablage, und wer schreibt die Sitzung?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Nachgezogen:** 260813-0130, nach der Diagrammprüfung `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/reviews/260813-0109-conceptrev-…`. Möglichkeit 1 nannte zwei verschiedene Mechanismen unter einem Wort; sie heißen jetzt Schreibsperre und Sitzungsrecht. Die Frage selbst ist unverändert.
**Cross-references:** `shared/planning/260813-0053_o_spec-suche-in-der-belegung-vollstaendiges-menue-zweite-instanz.md` (C3), `crates/krk-core/src/ablage/mod.rs`, `crates/krk-core/src/ablage/atomar.rs:39`, `crates/krk-core/src/ablage/sitzung.rs:407-460`

---

## Frage

Sobald KRK ein zweites Mal läuft, greifen zwei Prozesse ohne jede Absprache auf dieselben vier Dateien unter `~/Library/Application Support/KRK/` zu. Am 260813 am Baum erhoben: es gibt keine Sperre, kein `flock`, kein `O_EXCL`; die Suche danach über `crates/` liefert keinen Treffer. Was zwei Instanzen anrichten, unterscheidet sich je Datei.

| Datei | Wer schreibt, wann | Was zwei Instanzen anrichten |
|---|---|---|
| `session.toml` | jede Änderung, gebündelt höchstens alle zwei Sekunden, dazu einmal beim Beenden | Beide schreiben dauernd ihren ganzen Zustand. Die zuletzt schreibende Instanz gewinnt; die Tabs, Ordner und Breiten der anderen sind fort. |
| `bookmarks.toml` | bei jedem Anlegen, Umbenennen, Löschen und Verschieben eines Lesezeichens | Verlorene Änderung. Instanz B kennt das Lesezeichen nicht, das A eben angelegt hat, und schreibt beim nächsten eigenen Lesezeichenbefehl seine ältere Liste darüber. |
| `keymap.toml` | beim Verlassen der Belegungsansicht, wenn etwas geändert wurde | Dasselbe Muster, seltener. |
| `settings.toml` | nur beim allerersten Start | Unbedenklich. |

**Ein Befund wiegt schwerer als die verlorene Änderung.** `atomar::nachbarpfad` leitet den Namen der Nachbardatei fest aus dem Ziel ab und trägt ausdrücklich keine Laufnummer, damit ein Absturz höchstens eine liegengebliebene Datei hinterlässt. Zwei Instanzen, die dieselbe Datei zugleich schreiben, benutzen damit **dieselbe** Nachbardatei: beide öffnen sie mit Abschneiden, beide schreiben an ihren eigenen Versätzen, und das `rename` veröffentlicht ein Gemisch. Die Zusage des Moduls, der Leser sehe entweder den alten Inhalt ganz oder den neuen ganz, gilt für einen Schreiber und nicht für zwei. Die Runde 6 hat gebaut, dass eine beschädigte Ablagedatei zur Seite gelegt statt überschrieben wird; der Bestand wäre also wiederherstellbar, aber der Nutzer verlöre ihn aus der laufenden Sitzung.

## Möglichkeiten

1. **Eine Schreibsperre über der Ablage, ein Sitzungsrecht daneben; wer schreibt, liest vorher neu.** Zwei Mechanismen mit zwei Lebensdauern, und sie sind auseinanderzuhalten:
   - Die **Schreibsperre** wird für einen vollständigen Durchgang aus Lesen, Ändern und Schreiben auf den Ablageordner genommen und gleich wieder abgegeben. Lesezeichen werden unter ihr frisch von der Platte gelesen und die eine Änderung darauf angewandt statt auf den Stand vom Programmstart. Läge das Lesen außerhalb, wäre die verlorene Änderung nur seltener und nicht fort.
   - Das **Sitzungsrecht** wird beim Start genommen und bis zum Ende des Prozesses gehalten. Die Sitzung schreibt genau seine Halterin; jede weitere Instanz startet aus derselben gespeicherten Sitzung und schreibt sie nicht zurück. Sie sagt das einmal beim Start in der Statuszeile.
   - Dafür: Kein Gemisch, keine verlorene Änderung an Lesezeichen und Belegung, und die Frage „welche Sitzung gehört welcher Instanz" wird nicht geschätzt, sondern durch eine entscheidbare Tatsache beantwortet, nämlich wer das Sitzungsrecht hält.
   - Dagegen: Der Kern bekommt zwei Absprachen, wo er bisher ohne auskam. Die zweite Instanz merkt sich ihre Fensteraufteilung nicht.
   - **Warum zwei und nicht eine:** ein einziger Mechanismus kann beides nicht leisten. Hielte Instanz 1 ihn vom Start bis zum Ende, käme keine zweite je zum Schreiben. Gäbe jeder Schreibvorgang ihn wieder ab, hielte ihn nach dem ersten Schreiben niemand, und „wer ihn hält" beantwortete die Frage nach der Sitzung nicht mehr. Bis zum 260813-0130 stand hier ein Wort für beides; die Diagrammprüfung hat es sichtbar gemacht.
2. **Letzter Schreiber gewinnt, und KRK sagt es.** Keine Sperre; die zweite Instanz meldet beim Start, dass Lesezeichen und Aufteilung verlorengehen können.
   - Dafür: Kostet fast nichts.
   - Dagegen: Der Verlust von Lesezeichen ist der Verlust von Nutzerarbeit, und das Gemisch in der Nachbardatei bleibt möglich. Ein wissentlich ausgeliefertes Verlustrisiko steht gegen die Runde 6, die den Bestand des Nutzers gerade erst geschützt hat.
3. **Die zweite Instanz ist an der Ablage nur lesend.** Sie liest die vier Dateien beim Start und schreibt keine.
   - Dafür: Kein Konflikt, keine Sperre.
   - Dagegen: In der zweiten Instanz lässt sich kein Lesezeichen anlegen und keine Belegung ändern. Das ist ein Saum von Sonderfällen mitten in der Bedienung, und der Nutzer müsste bei jedem Befehl wissen, in welcher Instanz er steht.
4. **Jede Instanz bekommt ihren eigenen Ablageordner.** `Ablage::oeffnen` nimmt schon heute jeden Ordner entgegen.
   - Dafür: Am billigsten von allen, kein geteilter Zustand.
   - Dagegen: Getrennte Lesezeichen und getrennte Tastenbelegung. Das will niemand, der eine zweite Instanz startet, um dieselben Orte zu erreichen.

## Randbedingungen

- Die Sitzung ist ihrer Natur nach nicht teilbar: sie beschreibt, wie *ein* Fenster aussah. Welche gespeicherte Sitzung zu welcher Instanz gehört, ist aus den Eingaben, die ein Prozess hat, nicht zu beantworten — ein Prozess trägt über einen Neustart hinweg keine Nämlichkeit. Eine Näherung darüber wäre keine Lösung; entscheidbar ist stattdessen die andere Frage, wer die Sperre hält.
- Der Vorgang liegt auf dem Startpfad. Die Zusage L4 aus C8 der Runde 1 gibt dem Kaltstart bis zur bedienbaren Oberfläche 1000 ms; die Vergabe des Sitzungsrechts ist ein Systemaufruf und fällt darin nicht auf, gehört aber in den nächsten Abnahmelauf.
- **Beides muss ein Prozess auch beim Absturz freigeben.** Ein Mechanismus, der eine Marke im Dateisystem hinterlässt, die niemand aufhebt, sperrt nach einem Absturz jede weitere Instanz für immer aus dem Sitzungsschreiben aus. Wer einen solchen wählt, braucht eine Aufräumregel dazu. Das Kriterium steht als C3.13 im Spec.

## Empfehlung

Möglichkeit 1, mit den zwei getrennten Mechanismen. Sie ist die einzige, die weder Nutzerarbeit verliert noch die Bedienung je Instanz verschieden macht, und sie beantwortet die unentscheidbare Frage nicht, sondern ersetzt sie durch eine entscheidbare: nicht „welche Sitzung gehört diesem Prozess", sondern „hält dieser Prozess das Sitzungsrecht". Der Preis, den sie kostet, ist benannt und klein: die zweite Instanz merkt sich ihre Aufteilung nicht, und sie sagt es.

Die Runde fährt bis zu einer Antwort auf Möglichkeit 1.

---
Answered:
Implemented:
Deferred:
Superseded by:
