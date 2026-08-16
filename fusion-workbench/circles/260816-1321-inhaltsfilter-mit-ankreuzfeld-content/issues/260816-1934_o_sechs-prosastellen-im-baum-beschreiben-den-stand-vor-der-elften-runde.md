Sechs Prosastellen im Baum beschreiben den Stand vor der elften Runde

---

Der Nachzug `b9ab8ae` hat fünf Stellen abgetragen, die die Schritte A1 und A2 falsch gemacht
hatten (`issues/260816-2015_c_…`). Sechs weitere stehen noch, und sie kommen aus den
Schritten B1, D1 und A1. Keine hält den Bau an, keine bricht ein Abnahmekriterium, und jede
ist an ihrer eigenen Datei nachzulesen.

**1. `crates/krk-core/src/verzeichnis/sys.rs:802` nennt einen Aufrufer der Lesehülle, es
sind zwei.** Der Satz lautet: „Aufrufer der Huelle ist heute die Vorschau, mit ihren zwei
Grenzen." Seit A2 ruft `verzeichnis::inhalt::traegt_der_inhalt`
(`verzeichnis/inhalt.rs:134`) dieselbe Hülle. Der Absatz darunter (`sys.rs:804-811`) zieht
die Folge weiter: „Verschieden sind die Antwort und die Grenze. Der Editor weist mit
[…] ab […]; die Vorschau faellt auf ihre Metadatenanzeige zurueck" — der dritte Ausgang,
`Inhaltsbefund`, kommt nicht vor, und die Aufzählung der Grenzen („`EDITORGRENZE`,
`TEXTGRENZE` oder `BILDGRENZE`") ebenso wenig, obwohl der Inhaltsfilter `TEXTGRENZE` als
vierten Weg mitbringt. Der Abschnitt heißt „# Zwei Aufrufer, und die Zielpruefung bleibt
bei beiden"; die Überschrift stimmt, ihr Text nicht mehr.

**2. `crates/krk-core/src/verzeichnis/verweisziel.rs:42-45` nennt die Vorschau in
`krk-ui` als Leseweg.** Der Satz lautet: „Der Editor (`text::datei::oeffnen`) und der
Leseweg der Vorschau in `krk-ui` lesen aus genau dem Deskriptor, den sie geprueft haben."
Seit A1 (`5c7f5b9`) liegt dieser Leseweg in `krk-core`, und es sind drei Leser und nicht
zwei. Die Datei ist von der Runde nicht angefasst worden; genau deshalb steht der Satz noch.

**3. `crates/krk-core/src/text/datei.rs:598` begründet die Schranke mit `/dev/zero`.**
„Zwischen `fstat` und `read` kann eine Datei wachsen, und `/dev/zero` liefert ohne Ende,
ohne je eine Groesse zu melden." Der zweite Halbsatz trifft diese Hülle nicht: `/dev/zero`
ist ein Zeichengerät, fällt zwei Zeilen früher am `!angaben.is_file()` heraus
(`datei.rs:616-618`) und erreicht das `take(grenze + 1)` nie. Der Satz ist mit dem Rumpf aus
`vorschaumodell.rs` mitgezogen, wo er dieselbe Schwäche hatte. Die Abnahmeliste hat das
gesehen und schreibt es aus (`messungen/260816-abnahme-inhaltsfilter.md:210-212`); an der
Quelle steht es weiter. Die Schranke selbst ist richtig und gehört gehalten — allein ihre
Begründung braucht einen Fall, der sie wirklich auslöst, nämlich eine gewöhnliche Datei, die
zwischen `fstat` und `read` wächst.

**4. `crates/krk-core/src/text/datei.rs:646-649` nennt eine zweite Stelle für
`String::from_utf8`, es sind drei.** „Gewandelt wird ueber [`String::from_utf8`], denselben
Weg, ueber den die Vorschau entscheidet, ob eine Datei Text ist
(`krk-ui/src/vorschaumodell.rs`)." Der Inhaltsfilter entscheidet dieselbe Frage über
denselben Weg (`verzeichnis/inhalt.rs:135`). Die Aussage des Absatzes — eine Antwort auf
„ist das Text" — bleibt richtig, ihre Aufzählung ist unvollständig.

**5. `crates/krk-ui/src/tabs.rs:661` zählt die Übertragung beim Ordnerwechsel falsch.**
„Die vierte Uebertragung, in derselben Bauart wie die drei darueber" steht am Filtertext.
Über ihm stehen seit D1 vier und nicht drei: Sortierung, Verstecke, `tief` und `inhalt`
(`tabs.rs:657-660`). Der Plan hat es richtig, er nennt sie „als fünfte Übertragung"
(`planning/260816-1359_c_plan-…:302`); der Doc-Kommentar darüber ist nachgezogen worden
(`tabs.rs:650-654`), der Zeilenkommentar darunter nicht.

**6. Zwei Stellen nennen „Deep" als einzigen Anlass eines Durchlaufs.**
`crates/krk-ui/src/tabs.rs:824-826` zählt die Rufer von `durchlauf_nachziehen` auf: „von
jeder Aenderung des Filtertexts, vom Umschalten des Filters der Tiefe und vom Einzugstakt".
Seit E3 ruft auch `DateifensterQuelle::inhaltssuche_umschalten`
(`crates/krk-ui/src/appkit/tabelle.rs:2091`), und dieser Rufer fehlt in der Liste.
`crates/krk-ui/src/appkit/tabelle.rs:1324-1326` sagt dasselbe von der anderen Seite: „Jede
Aenderung des Filtertexts bricht den laufenden ab und stoesst, **wenn „Deep" steht**, einen
neuen an." Die Bedingung im Rumpf lautet seit C1 `!filter_steht() || (!tief() &&
!inhalt_wirkt())` (`tabs.rs:897`), also stößt auch ein wirkender Inhaltsfilter ohne „Deep"
einen an.

---

Eine siebte Stelle steht in einem eigenen Datensatz, weil sie nicht nur Prosa ist:
`Ordnermodell::tief_setzen` begründet das ausbleibende Rücksetzen mit „weil ihn dann
niemand liest" (`verzeichnis/modell.rs:787`), und seit dieser Runde liest ihn der
Dateizweig. Siehe
`issues/260816-1930_o_content-ausschalten-laesst-ordnerzeilen-auf-einem-veralteten-inhaltsbefund-stehen.md`.

Gefunden bei der Durchsicht der elften Runde, Bereich `9f5ced5..b9ab8ae`.
