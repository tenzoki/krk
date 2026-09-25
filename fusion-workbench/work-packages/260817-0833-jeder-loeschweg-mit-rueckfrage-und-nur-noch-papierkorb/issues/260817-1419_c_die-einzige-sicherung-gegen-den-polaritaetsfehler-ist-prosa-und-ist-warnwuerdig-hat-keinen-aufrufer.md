Die einzige Sicherung gegen den Polaritätsfehler ist Prosa, und `ist_warnwuerdig` hat heute keinen Aufrufer

---

`Befund` trägt zwei Polaritäten: bei der Papierkorbfrage ist `Ja` die Erlaubnis, bei den
Zieltests des Bündels C ist `Ja` der Warngrund. Drei Modulköpfe warnen davor,
`ist_warnwuerdig` an die erste zu halten. Keine Probe und kein Typ hält es davon ab, und
`ist_warnwuerdig` hat im ganzen Baum noch keinen Aufrufer.

---

**Schwere:** Mittel. Am heutigen Baum liegt kein Verwendungsfehler vor — es gibt nur einen
Verwender, und der liegt richtig. Der Befund ist die Sicherung: Bündel C bringt die erste
Aufrufstelle von `ist_warnwuerdig`, und wer sie schreibt, hat drei Prosaabsätze und keine rote
Probe als Widerstand. Der Fehler, den sie verhindern sollen, macht aus „wir wissen nichts über
das Ziel" die Erlaubnis zu löschen und nimmt C4 seine Zusage.
**Gefunden von:** coderev, Durchsicht `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`
**Betroffen:** `crates/krk-core/src/verzeichnis/befund.rs:50-68`, `:125`,
`crates/krk-ui/src/kommandos/loeschwarnung.rs:61-69`,
`crates/krk-ui/src/appkit/papierkorb.rs:49-59`
**Baumstand:** `ee85950`
**Domain:** code

## Was am Baum steht

`ist_warnwuerdig` hat keine Aufrufstelle. Nachgezählt mit
`grep -rn "ist_warnwuerdig" crates/`: sechs Treffer, drei davon in `befund.rs` selbst (die
Erklärung `:125` und zwei Proben), drei außerhalb — und alle drei stehen in einem
Doc-Kommentar:

| Stelle | Was dort steht |
|---|---|
| `loeschwarnung.rs:66` | „`Befund::ist_warnwuerdig` kommt in dieser Datei nicht vor, und das ist Absicht" |
| `loeschwarnung.rs:458` | „wer aus Gewohnheit `Befund::ist_warnwuerdig` nimmt, macht aus ‚wir wissen nichts' die Erlaubnis zu loeschen" |
| `papierkorb.rs:53` | „`Befund::ist_warnwuerdig` ist hier folglich das falsche Werkzeug" |

Die erste dieser drei ist eine **Aussage über den Baum** und keine Begründung: „kommt in
dieser Datei nicht vor". Genau diese Sorte Aussage hält dieses Projekt mit einer Zählprobe,
und `loeschwarnung.rs` trägt zweihundert Zeilen darunter eine
(`die_stufenregel_hat_genau_einen_aufrufer`, `:362`), die `crate::quellbaum::aufrufstellen`
schon benutzt. Der Modulkopf von `quellbaum` schreibt die Bauform aus.

Die fünf Proben in `befund.rs` prüfen die Tafel und die Lautheit. Keine prüft, **wo**
`ist_warnwuerdig` gerufen werden darf, und das ist die Frage, an der der Fehler hängt.

## Was daran heute schon zutrifft und was nicht

Geprüft und in Ordnung: die eine Verwendung liegt auf der richtigen Polarität.
`vor_der_rueckfrage` (`loeschwarnung.rs:242-246`) prüft auf `Befund::Ja` selbst und führt
`Nein` und `Unentschieden` ausgeschrieben in denselben Zweig; die Probe
`ohne_papierkorb_erscheint_kein_blatt` (`:462`) hält beide Zeilen einzeln fest und benennt in
ihrem Doc-Kommentar, dass ihre zweite Zusicherung die eigentliche ist.

Offen ist die Sicherung nach vorn. Die zweite Polarität existiert im Baum noch nicht, also
gibt es heute keine Stelle, an der eine Probe einen Fehler fangen könnte — und genau deshalb
ist jetzt der Zeitpunkt, an dem die Sicherung geschrieben wird und nicht nach dem ersten
Verwender.

## Richtung

Zwei Wege, und der zweite ist der stärkere.

1. **Eine Zählprobe.** In `loeschwarnung.rs` und `papierkorb.rs` je eine Zusicherung, dass
   `aufrufstellen(inhalt, "ist_warnwuerdig") == 0` gilt, mit zusammengesetzter Nadel wie bei
   der Vorlage. Billig, hält gegen den Gewohnheitsfehler und macht die drei Prosaaussagen
   prüfbar.
2. **Zwei Typen für zwei Fragen.** Die Polarität gehört zur Frage und nicht zum Wert, und
   `befund.rs:67-68` sagt das selbst: „welche das ist, haengt an der Frage und nicht am Typ".
   Ein `Warnbefund` mit `ist_warnwuerdig` und ein `Erlaubnisbefund` mit `erlaubt` machen den
   Fehler unübersetzbar; der gemeinsame Rumpf bleibt eine Aufzählung mit drei Werten. Kosten:
   ein Typ mehr und eine Umrechnung an vier Prüfstellen des Bündels C.

Bündel C berührt beide Dateien ohnehin (Schritte 9 und 10), also fällt der Schnitt dort ohne
zweite Änderung an derselben Stelle.

Hinweis (260817, Aufgabe T5b): Der Typ heißt seit `260817-1419_*_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund.md`
nicht mehr `Befund`, sondern `Loeschzielbefund`, und die Datei heißt
`crates/krk-core/src/verzeichnis/loeschzielbefund.rs`. Der Modulkopf ist dabei um den Abschnitt
`# Warum der Typ nicht Befund heisst` gewachsen, die Zeilenangaben unter **Betroffen** sind
damit verschoben. Der Befund selbst besteht unverändert: die Sicherung gegen den
Polaritätsfehler ist weiterhin Prosa in drei Modulköpfen.

Progress (260817-1623, coder, step 9): the first way now stands in **one** of the three files.
`crates/krk-ui/src/appkit/volumes.rs` carries `hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt`,
a count over its own file asserting `aufrufstellen(inhalt, "ist_warnwuerdig") == 0`, with the
composed needle. The record stays open: `appkit/papierkorb.rs` and `kommandos/loeschwarnung.rs`
carry no such count, and the second, stronger way — two types for two questions — is untouched.
The new check `volumes::ist_lokal` also adds a case this record does not describe, filed as
`260817-1623_o_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`: its return value runs
counter to the `Loeschziel` field that consumes it, so the hazard there is not the habitual
`ist_warnwuerdig` but a missing inversion.

Progress (260817-1722, coder, step 10 — and one claim of this record no longer holds).

**The expectation stated above is now falsified: bundle C does not bring the first call site of
`ist_warnwuerdig`.** This record says "Bündel C bringt die erste Aufrufstelle von
`ist_warnwuerdig`, und wer sie schreibt, hat drei Prosaabsätze und keine rote Probe als
Widerstand". Step 10 wrote the one place that would have made that call — `warngruende` in
`kommandos/loeschwarnung.rs`, the judge over all six triggers of C3 — and it does not call it,
for a reason that is design and not oversight:

`ist_warnwuerdig` merges `Ja` and `Unentschieden`. `warngruende` has to keep them apart, because
they produce **different** entries in its list. A network volume answered `Ja` yields
`Warngrund::Netzlaufwerk`; answered `Unentschieden` it yields `Warngrund::Unentscheidbar`, and
*not* `Netzlaufwerk` as well — KRK does not know whether the volume is one, and naming it in the
explanation would be a claim with no measurement behind it. That is spec C3's own acceptance
criterion ("nennt als Grund, dass das Ziel sich nicht einordnen ließ"). The promise
"Unentschieden gilt als laut" is kept by `Unentscheidbar` sitting at rank 1, not by merging the
two answers. So every check in that function writes all three answers out, and the merged
question has no place to be asked.

Together with the rename recorded in
`260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`, this changes the shape of
the record rather than closing it:

| what this record asked for | state after step 10 |
|---|---|
| counting probe in `appkit/volumes.rs` | stands (step 9), **subject changed** by the rename — see below |
| counting probe in `appkit/papierkorb.rs` | **not done.** File outside this task's bounds |
| counting probe in `kommandos/loeschwarnung.rs` | **not done, and it would now over-promise** — see below |
| two types for two questions (second, stronger way) | **untouched** |

**Why the count in `volumes.rs` no longer measures what this record wanted.** After the rename
its value sits on polarity 1, so `ist_warnwuerdig` is the *correct* question for it. The probe
stays, with a rewritten rationale: the module answers the trigger and does not judge it. It is a
module boundary now, not a polarity guard.

**Why a file-level count in `loeschwarnung.rs` would over-promise.** After step 10 that one file
carries **both** polarities: `vor_der_rueckfrage` consumes the trash answer (polarity 2, `Ja` is
permission, `ist_warnwuerdig` forbidden) and `warngruende` consumes the trigger answers
(polarity 1, where it would be allowed and is merely useless). A count of zero over the whole
file would hold today and would state a ban that applies to one function and not the other; the
next person to add a polarity-1 consumer would have to break a green probe to write correct
code. What holds instead is written out at `warngruende`'s doc comment and in the module header:
all three answers are matched explicitly, and the reason is that `Ja` and `Unentschieden` lead
to different reasons.

**What is left of this record, restated.** One producer file (`appkit/papierkorb.rs`) still has
prose only. And the substantive question is unchanged and is the second way: the polarity belongs
to the question, not to the value, and one three-valued type for both cannot be made
uncompilable-when-swapped. Bundle C is now past both of its files, so the "der Schnitt fällt dort
ohne zweite Änderung an derselben Stelle" argument in **Richtung** has expired — the second way
would now cost its own change.

Tree state: `3fcd375` plus the uncommitted steps 9 and 10. Verification: `make check` — exit 0.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, und der Titel trifft nicht mehr
ganz.**

`ist_warnwuerdig` hat seit Schritt 8 (`5a0f041`) drei Aufrufstellen, alle im Probencode:
`crates/krk-core/tests/arbeitsbaum.rs:100` und `:305` sowie
`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:573` im `#[cfg(test)]`-Modul. Einen Aufrufer im
Nicht-Probencode gibt es weiterhin nicht, und alle drei liegen auf der ersten Polarität, wo die
Frage die richtige ist. Der Befund selbst ist davon unberührt: die Sicherung gegen die
Verwechslung der beiden Polaritäten ist nach wie vor Prosa in drei Modulköpfen plus eine
Zählprobe in `appkit/volumes.rs`, und `appkit/papierkorb.rs` trägt nur Prosa. Die Zählung von
„sechs Treffer" im Abschnitt `## Was am Baum steht` ist ein aufgezeichneter Stand und bleibt im
Wortlaut; am 260817-1833 liefert `grep -rn "ist_warnwuerdig" crates/ --include='*.rs'` 14
Zeilen.

**Ablage:** dieser Datensatz führt zwei Sachen, und nur eine davon ist ein Defekt. Der Defekt
ist die fehlende Messung der drei Prosaaussagen. Der zweite Weg der `## Richtung` — „zwei Typen
für zwei Fragen", mit benannten Kosten von einem Typ plus einer Umrechnung — ist ein Wahlpunkt
und kein Defekt, und die Fortschrittsnotiz von Schritt 10 nennt ihn ausdrücklich als „die
substanzielle Frage". Als Befund fällt er aus jeder Suche nach aktiver Grundlage heraus. Er
gehört als Entscheidungsdatensatz in `shared/decisions/`; der Vorschlag steht im
Abgleichsprotokoll `history/260817-1833-reconciliation.md` unter „Misfiled — should be a
decision", und die Umlage ist Nutzerarbeit.

---
Resolved: Die drei Prosaaussagen sind gemessen, und der Wahlpunkt, den dieser
Datensatz daneben führte, steht jetzt als Entscheidungsdatensatz da.

**Der Defekt: der erste Weg steht an allen drei Dateien.** `appkit/volumes.rs`
trug die Zählung seit Schritt 9; dazugekommen sind
`appkit/papierkorb.rs` und `kommandos/loeschwarnung.rs`, beide mit der Bauform
der Vorlage — Nadel aus `concat!`, `aufrufstellen(inhalt, name) == 0` über die
eigene Datei, Blindheit am Doc-Kommentar benannt.

| Datei | Polarität | was die Zählung sagt |
|---|---|---|
| `appkit/volumes.rs` | 1 | Modulgrenze: beantwortet den Auslöser, beurteilt ihn nicht |
| `appkit/papierkorb.rs` | 2 | `Ja` ist die Erlaubnis; die zusammenfassende Frage machte daraus die Erlaubnis bei Unkenntnis |
| `kommandos/loeschwarnung.rs` | 1 **und** 2 | beide Gründe zugleich, siehe unten |

**Der Einwand aus der Fortschrittsnotiz vom 260817-1722 trifft nicht mehr.**
Dort stand, eine Zählung über die ganze Datei `loeschwarnung.rs` würde
überversprechen, weil ein künftiger Verbraucher der ersten Polarität eine grüne
Probe brechen müsste, um richtigen Code zu schreiben. Der Modulkopf derselben
Datei sagt inzwischen ausdrücklich das Gegenteil (Abschnitt „Ein unentschiedener
Eingang nennt seinen eigenen Auslöser nicht mit"): in **dieser** Datei ist die
Frage auch für die erste Polarität unbrauchbar, weil `Ja` und `Unentschieden`
dort zu verschiedenen Warngründen führen und auseinandergehalten werden müssen.
Die Zählung sagt damit genau das, was der Kopf sagt, und keinen Satz mehr. Der
Doc-Kommentar der Probe schreibt beide Gründe getrennt aus.

Nachweis, dass beide neuen Zählungen fangen: je ein Aufruf von
`ist_warnwuerdig` versuchsweise in den Rumpf gesetzt
(`fuehrt_einen_papierkorb` beziehungsweise `vor_der_rueckfrage`), und beide
Proben schlagen fehl; zurückgenommen, beide wieder grün. Die Zählung in
`volumes.rs` blieb dabei grün, wie sie soll — der Fehler saß nicht in ihrer
Datei.

**Der Wahlpunkt: der zweite Weg ist jetzt ein Entscheidungsdatensatz.** Der
Abgleich vom 260817-1833 hat richtig festgestellt, dass „zwei Typen für zwei
Fragen" kein Defekt ist und als Befund aus jeder Suche nach aktiver Grundlage
herausfällt. Er steht seit dem 260818 unter
`decisions/260818-0249_o_bekommen-die-zwei-polaritaeten-des-loeschzielbefunds-zwei-typen.md`,
mit dem Stand am Baum, drei Möglichkeiten und ihren Kosten. Der Datensatz liegt
im Circle und nicht in `shared/`, weil die Frage aus dieser Directive entstanden
ist (Herkunftsregel).

Was am Baum unverändert offen bleibt und dort steht: keine der vier Messungen
macht die Verdrehung unübersetzbar. Sie fangen sie, nachdem jemand sie
geschrieben hat.
