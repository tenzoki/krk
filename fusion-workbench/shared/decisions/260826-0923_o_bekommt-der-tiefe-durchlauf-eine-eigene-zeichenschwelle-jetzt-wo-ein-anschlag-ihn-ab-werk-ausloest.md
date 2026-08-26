# Bekommt der tiefe Durchlauf eine eigene Zeichenschwelle, jetzt wo ein einziger Anschlag ihn ab Werk auslöst?

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-ui/src/tabs.rs:893-919` (`durchlauf_nachziehen_an`, die Bedingung, die den Lauf anstößt); `crates/krk-core/src/verzeichnis/modell.rs:935-937` (`filter_steht`, ein Zeichen genügt); `crates/krk-core/src/verzeichnis/durchlauf.rs:134-148` („Was dieses Modul nicht hat, und warum": keine Tiefengrenze, kein Deckel auf die Trefferzahl); `crates/krk-core/src/verzeichnis/filter.rs:157-159` (`inhaltsschwelle`, die Staffelung, die es für den **Inhalt** schon gibt); `shared/decisions/260826-0859_*_die-vorgabe-der-tiefen-suche-hebt-die-schwelle-des-inhaltsfilters-von-drei-auf-fuenf.md` (dieselbe Verschiebung, aber allein für den Inhaltsfilter); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1830_*_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md` (offen)

---

## Question

Seit `20c9833` steht `tief` in `Ordnermodell::neu` auf `true`. Der Nutzer hat das verlangt, und
die Vorgabe selbst ist nicht Gegenstand dieser Frage.

Was sie auslöst, ist es. `Tabliste::durchlauf_nachziehen_an` stößt den Durchlauf über den
Unterbaum an, sobald `filter_steht()` **und** `tief()` gelten (`tabs.rs:897`), und
`filter_steht()` ist `!self.filtertext.is_empty()` (`modell.rs:936`) — **ein** Zeichen. Der
erste Anschlag im Dateifenster startet damit ab Werk einen Arbeitsfaden, der den ganzen
Unterbaum des angezeigten Ordners abläuft. Bis gestern war das die Folge eines Klicks auf
„Deep"; heute ist es die Vorgabe.

Der Durchlauf trägt dagegen keine Schranke, und das ist ausgeschrieben und gewollt
(`durchlauf.rs:136-148`): keine Tiefengrenze, kein Deckel auf die Trefferzahl, keine Besuchtliste.
Er hält einen Verzeichnisdeskriptor, entscheidet je Ordner beim ersten Treffer und lässt den Rest
darunter liegen — die Ersparnis greift also nur, wo es einen Treffer gibt. Ein Filtertext ohne
Treffer läuft den Unterbaum vollständig ab. In `~` sind das mit einem `Projects/` voller
`.git`- und `node_modules`-Bäume Zehntausende Verzeichnisse, und jeder weitere Anschlag bricht
den Lauf ab und beginnt einen neuen.

**Ohne „Content" liest der Lauf keine Datei**; das ist geprüft: `zeilengrund_von` erteilt einen
`Auftragsart::Inhalt` nur bei `inhalt_wirkt()`, und „Content" bleibt ab Werk aus. Es geht allein
um den Verzeichnisdurchlauf.

Die zehn Zeitzusagen aus C8 sagen dazu nichts — keine misst das Tippen, beide Messstrecken
setzen nie einen Filtertext (am Baum geprüft: weder `krk-bench/src/messen.rs` noch
`krk-ui/src/messmodus.rs` ruft `filtertext_setzen`, und `Anweisung::Taste` ist `pfeil_ab_senden`,
also ein Zeichen, das `traegt_ein_dateiname` abweist). Genau deshalb fängt die Zusagenprüfung
diese Verschiebung nicht ab.

Zu entscheiden ist jetzt, weil die Vorgabe steht und mit der nächsten Auslieferung bei jedem
Nutzer ankommt. Der Datensatz `260826-0859` stellt dieselbe Frage für die **Schwelle des
Inhaltsfilters**, die sich als Nebenwirkung mitverschoben hat; er stellt sie nicht für den
Durchlauf selbst, der ohne „Content" läuft und die eigentliche Arbeit macht.

## Options

1. **So lassen: ein Zeichen genügt weiter.** Die Vorgabe wirkt wie ein gesetzter Haken.
   - Pro: eine Regel, keine zweite Zahl. Der Durchlauf ist abbrechbar, läuft auf einem eigenen
     Faden und hält die Oberfläche nicht an; er ist genau dafür gebaut. Nichts ist zu bauen.
   - Kontra: die teuerste Wirkung der Vorgabe steht am billigsten Anlass. Bei einem Zeichen ist
     die Trefferwahrscheinlichkeit im Namen ohnehin hoch, der Erkenntnisgewinn aus dem Unterbaum
     also klein und die Arbeit am größten. Und sie ist auf dem Referenzgerät ungemessen.
2. **Der Durchlauf bekommt eine eigene Schwelle**, in derselben Bauart wie `inhaltsschwelle`:
   erst ab n Zeichen wird abgestiegen, darunter filtert der Name flach.
   - Pro: die Staffelung, die es für den Inhalt schon gibt, gilt dann für beide Kosten desselben
     Schalters, und ihre Begründung ist wörtlich dieselbe („der Lauf ist tief und damit teuer").
     Eine Stelle mehr, kein zweiter Mechanismus.
   - Kontra: eine zweite unbegründete Zahl neben der Drei und der Fünf, die beide ungemessen
     sind. Und ein sichtbarer Bruch: der Nutzer tippt zwei Zeichen, sieht nichts aus dem
     Unterbaum, tippt ein drittes und bekommt eine andere Liste, ohne dass ein Haken gewechselt
     hätte.
3. **Erst messen, dann entscheiden.** Die Kosten eines Durchlaufs ohne Treffer über einen
   echten Unterbaum auf dem Referenzgerät festhalten, wie die Runde 17 es für das Packen getan
   hat.
   - Pro: dieselbe Zahl fehlt auch `260826-0859`; eine Messung entscheidet beide Fragen und
     ersetzt in beiden eine Schätzung durch einen Wert.
   - Kontra: die Vorgabe wird bis dahin ausgeliefert, also wirkt Möglichkeit 1 in der
     Zwischenzeit ohnehin.

## Constraints

- Die Bedingung steht an genau einer Stelle (`durchlauf_nachziehen_an`) und die Schwelle des
  Inhaltsfilters an genau einer (`filter::inhaltsschwelle`). Jede Antwort hält beides; eine
  zweite Schwelle neben `inhaltsschwelle` wäre ein zweiter Ort für dieselbe Art von Aussage.
- Ohne stehenden Filtertext ändert die Vorgabe nichts: `zeilengrund_von` verlässt den
  Prüfschritt vor der Frage nach der Tiefe, und `durchlauf_nachziehen_an` stößt keinen Lauf an.
  Der Programmstart und das bloße Blättern sind nicht betroffen.
- Keine der zehn Zeitzusagen aus C8 misst diese Strecke, und keine Messung auf dem
  Referenzgerät liegt vor. Eine Antwort, die sich auf Kosten beruft, beruft sich auf eine
  Schätzung.
- Die Vorgabe „Deep = ein" steht und ist nicht Gegenstand dieser Frage.

## Die Nachbarschaft, in der diese Frage steht

Der Baum trägt einen zweiten Mechanismus, der Verzeichnisse liest, ohne dass der Nutzer ihn
ausdrücklich anstößt: die Profil-Zusammenfassung der Vorschau. Sie trägt **vier** ausgeschriebene
Schranken, jede an ein Abnahmekriterium gebunden (`crates/krk-core/src/leseprofil/mod.rs:111`,
`:122`, `:138`, `:141`):

| | Zusammenfassung der Vorschau | tiefer Durchlauf des Filters |
|---|---|---|
| Leseläufe | `HOECHSTENS_LESELAEUFE = 12` (C6.4) | keine Schranke |
| Dateiöffnungen | `HOECHSTENS_OEFFNUNGEN = 24` (C6.4) | ohne „Content" keine Öffnung |
| Einträge je Lauf | `HOECHSTENS_EINTRAEGE = 2_000` (C6.5) | keine Schranke |
| Bytes je Datei | `HOECHSTENS_BYTES = 64 KiB` (C6.6) | `inhaltsgrenze`, nur mit „Content" |
| Auslöser | eine ausgewählte Zeile | seit dem 260826 der erste Anschlag |

Die Runde 18 hat an diesen Schranken gerade gearbeitet und `HOECHSTENS_EINTRAEGE` ausdrücklich
umgedeutet, weil `HOECHSTENS_LESELAEUFE` „es nicht mehr kann: die Zahl der Unterordner eines
Ordners wächst mit dem Bestand". Genau diese Größe ist es, die der Durchlauf ungedeckelt abläuft.

Das ist kein Widerspruch: der Durchlauf begründet seine Schrankenlosigkeit ausgeschrieben
(`durchlauf.rs:136-148`), er läuft auf einem eigenen Faden, ist abbrechbar und entscheidet je
Ordner beim ersten Treffer. Es ist der Vergleich, der die Frage schärft: dasselbe Projekt hat für
dieselbe Art Arbeit einmal vier Zahlen und einmal keine, und der Unterschied war bis gestern, dass
das eine ohne Zutun des Nutzers lief und das andere erst nach einem Klick.

## Recommendation

Möglichkeit 3, und bis die Zahl da ist gilt Möglichkeit 1. Die Empfehlung von `260826-0859`
läuft auf dieselbe fehlende Messung hinaus („wie lange ein Inhaltsfilter über einen echten
Unterbaum bei drei Zeichen tatsächlich braucht"), und eine Messung, die beide Fragen bedient,
ist billiger als zwei Zahlen, die keine bedient.

Gegen die Möglichkeit 2 in der jetzigen Lage spricht, dass sie eine dritte ungemessene Zahl
setzte, und der Datensatz `260826-0859` weist genau dieses Argument für seine eigene
Möglichkeit 3 zurück. Dieselbe Zurückweisung gilt hier.
