# Welche vorhandene Größengrenze gilt für den Inhaltsfilter: die 1 MB der Vorschau oder die 16 MB des Editors?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `crates/krk-core/src/text/datei.rs:164` (`EDITORGRENZE`, 16 MB, samt der Begründung darüber); `crates/krk-ui/src/vorschaumodell.rs:121` (`TEXTGRENZE`, 1 MB) und `:678-690` (`bis_zur_grenze_lesen`); `crates/krk-core/src/text/datei.rs:411-465` (`lesen`, der Weg in `krk-core`); `shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md` (C1 und C2)

---

## Question

Der Nutzer hat am 260816 festgelegt, dass der Inhaltsfilter nur bis zu einer **vorhandenen** Größengrenze liest und keine neue Zahl einführt. Der Baum trägt zwei solche Grenzen, und aus dem Wort „vorhandene" folgt nicht, welche gemeint ist.

`EDITORGRENZE` steht bei 16 MB und begründet sich damit, dass sie „weit über den Dateien liegt, die man von Hand bearbeitet". Das ist eine Aussage über **eine** Datei, die der Nutzer absichtlich öffnet. `TEXTGRENZE` steht bei 1 MB und begründet sich aus dem Leseweg der Vorschau, der beim Bewegen der Auswahl läuft und die Zusage L7 mit 100 ms trägt. Das ist eine Aussage über einen Leseweg, der **ungefragt** und **oft** läuft.

Der Inhaltsfilter ist der zweite Fall und nicht der erste: er liest viele Dateien, ungefragt, auf dem Weg des Tippens. Damit spricht die Herleitung für die 1 MB. Dagegen steht, was der Nutzer verliert: zwischen 1 MB und 16 MB liegen die Dateien, die man am ehesten nach ihrem Inhalt durchsucht, nämlich Protokolle, Tabellen und große Datenablagen. Die Frage entscheidet also, welche Dateien überhaupt auffindbar sind, und sie gehört deshalb dem Nutzer.

**Sie entscheidet daneben die Abbruchspanne.** Der Durchlauf prüft sein Abbruchkennzeichen heute an der Stapelgrenze (`durchlauf.rs:287-291`). Der Inhaltsfilter fügt eine Einheit ein, die nicht unterbrochen wird: das Lesen einer einzelnen Datei. Die Größengrenze ist damit zugleich die obere Schranke dafür, wie lange ein Tastendruck auf den Abbruch des vorigen Durchlaufs wartet.

## Options

1. **Die 1 MB der Vorschau (`TEXTGRENZE`).**
   - Pro: dieselbe Herleitung wie die Zahl selbst, nämlich ein ungefragter Leseweg auf der Bedienstrecke. Die schlimmste ununterbrechbare Leseeinheit bleibt klein, und der Abbruch nach einem Tastendruck greift entsprechend schnell. Über einen Unterbaum gerechnet ist der Höchstaufwand je Datei um den Faktor 16 kleiner.
   - Kontra: eine Protokolldatei von 3 MB ist über ihren Inhalt nicht auffindbar, und der Nutzer sieht ihr das nicht an. Genau diese Dateien sind der häufigste Anlass für eine Inhaltssuche.
   - Folge für die Bauart: `krk_core::text::datei::lesen` erzwingt heute `EDITORGRENZE` fest. Für 1 MB bekommt es die Grenze als Argument, oder der Inhaltsfilter bekommt einen eigenen Einstieg neben ihm. Das ist eine Änderung an einer Stelle, die zwei Rufer hat, und keine dritte Lesemechanik.

2. **Die 16 MB des Editors (`EDITORGRENZE`).**
   - Pro: die Dateien, die man durchsucht, sind erfasst. `krk_core::text::datei::lesen` ist ohne Änderung der eine Weg, und `krk-core` kennt die Zahl der Vorschau ohnehin nicht als Bezug, sondern nur als Vergleichswert in einer Zusicherung.
   - Kontra: eine einzelne Datei kann 16 MB kosten, und über einen Unterbaum ist das der beherrschende Aufwand. Der Abbruch nach einem Tastendruck wartet im schlimmsten Fall auf ein vollständiges 16-MB-Lesen. Die Zahl ist ausdrücklich `speculation:` und nie gemessen worden; sie für einen zweiten, ganz anders gearteten Zweck zu übernehmen, überträgt eine ungeprüfte Annahme.

3. **Die 1 MB, und Dateien darüber werden ausdrücklich als ungelesen ausgewiesen.** Wie Möglichkeit 1, aber der Nutzer erfährt, dass etwas nicht durchsucht wurde.
   - Pro: der Verlust aus Möglichkeit 1 wird sichtbar statt still. Wer eine große Protokolldatei sucht, weiß, dass er sie anders suchen muss.
   - Kontra: eine Zahl mehr in der einen Statuszeile, und sie hängt an dem Datensatz, der die Rückmeldung während des Lesens klärt. Beide Antworten müssen zusammenpassen.

## Constraints

- Keine neue, dritte Zahl. Das hat der Nutzer am 260816 festgelegt.
- Keine dritte Lesemechanik. Die beiden vorhandenen Wege (`text::datei::lesen` und `vorschaumodell::bis_zur_grenze_lesen`) gehen beide über `krk_core::verzeichnis::sys::ohne_warten_oeffnen`; der Inhaltsfilter geht denselben Weg.
- Die Typprüfung steht am Deskriptor und nicht am Pfad, und die Grenze wird gehalten und nicht vorhergesagt: gelesen werden höchstens `grenze + 1` Bytes, damit eine zwischen `fstat` und `read` wachsende Datei die Zusage nicht bricht.
- „Ist das Text?" beantwortet dieser Baum an genau einer Stelle, nämlich `String::from_utf8` über die gelesenen Bytes. Ein Inhaltsfilter, der nach Endungen entschiede, wäre die zweite Antwort auf dieselbe Frage.
- `Eintrag::groesse` steht im Bestand schon da. Die Grenze kostet vor dem Lesen keinen zusätzlichen Systemaufruf, gleich welche Zahl gewählt wird.

## Recommendation

Möglichkeit 3. Die Herleitung der 1 MB trifft den Inhaltsfilter genauer als die der 16 MB: beide Zahlen sind für ihren Zweck gesetzt worden, und der Zweck des Inhaltsfilters ist der der Vorschau, nämlich ungefragt und oft auf der Bedienstrecke zu lesen. Der Preis von Möglichkeit 1 ist ein stiller Verlust, und still ist er das eigentliche Problem: der Nutzer hält eine nicht gefundene Datei für nicht vorhanden. Möglichkeit 3 zahlt denselben Preis sichtbar. Möglichkeit 2 überträgt eine ungemessene Zahl auf einen Zweck, für den sie nicht aufgestellt wurde, und kauft die auffindbaren Protokolldateien mit einer Abbruchspanne, die der Nutzer beim Tippen spürt.

---
Answered: `shared/planning/260816-1310_*_spec-inhaltsfilter-der-dateiliste.md`, Abschnitt `## Was der Nutzer am 260816 entschieden hat` und C1.6, C1.7, C4.7 — Möglichkeit 1, die 1 MB der Vorschau (`TEXTGRENZE`). Der Nutzer hat sie am 260816 ausdrücklich gegen die 16 MB des Editors gewählt: der Inhaltsfilter gehört zur Klasse „KRK sieht sich eine Datei im Vorbeigehen an", und das ist die Klasse der Vorschau. Die Folge ist benannt und angenommen: eine Protokolldatei von 3 MB ist für die Suche unsichtbar. Die Zahl ist damit zugleich die obere Schranke der Abbruchspanne, weil eine gelesene Datei die kleinste nicht unterbrochene Einheit ist.

**Was die Antwort nicht entscheidet.** Möglichkeit 3 dieses Datensatzes hätte den Verlust sichtbar gemacht, indem ungelesene Dateien in der Statuszeile ausgewiesen werden. Die Antwort nennt die Zahl und nicht diesen Zusatz. Der Rest der Frage wandert deshalb in `260816-1310_*_was-zeigt-die-eine-statuszeile-waehrend-der-inhalt-gelesen-wird.md` und wird dort mitbeantwortet, statt hier als stillschweigendes Nein zu gelten.
Implemented:
Deferred:
Superseded by:

---
Implemented: `09baffd` — die 1 MB der Vorschau reisen an genau einer Stelle in den Kern:
`crates/krk-ui/src/tabs.rs:929` reicht `tab.modell.inhalt_wirkt().then_some(crate::vorschaumodell::TEXTGRENZE)`
an den Durchlauf weiter, und der Kommentar darüber schreibt die Bauform aus („**Die eine Stelle, an
der die 1 MB in den Kern reisen**"; `None` heißt „bei diesem Lauf wird keine Datei geöffnet",
`Some(n)` heißt „es wird gelesen, und n ist die Grenze"). `crates/krk-core/src/verzeichnis/inhalt.rs`
nimmt die Grenze als Argument entgegen und führt keine eigene Zahl (`7283d55`); `TEXTGRENZE` steht
unverändert bei `crates/krk-ui/src/vorschaumodell.rs:131` und ist nicht angehoben worden. Damit ist
die Antwort in ihrer tragenden Form eingelöst: keine dritte Zahl, keine dritte Lesemechanik, und
`krk-core` bekommt keinen Bezug auf `krk-ui`. Abgeglichen am 260820-2056 gegen `f5300f4`.
