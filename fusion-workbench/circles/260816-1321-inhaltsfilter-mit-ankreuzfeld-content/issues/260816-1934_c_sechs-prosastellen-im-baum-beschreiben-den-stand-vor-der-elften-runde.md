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

---

## Abgleich 260908, und die Behebung

Jede der sechs Stellen einzeln gegen den heutigen Baum gelesen. **Zwei sind seit dem 260816
von fremder Hand abgetragen**, vier standen noch, dazu eine siebte derselben Bauart.

**Schon abgetragen (2).**

- **Stelle 1**, `verzeichnis/sys.rs`: der Abschnitt heisst heute „Mehrere Aufrufer, und die
  Zielpruefung bleibt bei jedem von ihnen", und der Modulkopf nennt die Rufer nach Klassen
  statt als Aufzaehlung — Textwege, Archivwege, Verzeichnisleser — samt Zaehlkommando.
- **Stelle 6a**, `tabs.rs`, die Rufer von `durchlauf_nachziehen`: die Aufzaehlung sagt heute
  „vom Umschalten **eines der beiden** Filter" und nennt daneben das Ein- und Ausblenden der
  versteckten Eintraege.

**Behoben (5).**

- **Stelle 2**, `crates/krk-core/src/verzeichnis/verweisziel.rs`: der Satz nennt die Leser
  nicht mehr namentlich, sondern zeigt auf den Modulkopf von `verzeichnis::sys`, der sie nach
  Klassen fuehrt. Dass die Zahl seit der Runde 11 dreimal gestiegen ist, steht dabei.
- **Stelle 3**, `crates/krk-core/src/text/datei.rs`, `bis_zur_grenze_lesen`: die Begruendung
  der Schranke ist jetzt eine gewoehnliche Datei, an die ein Schreiber waehrend des Lesens
  anhaengt. `/dev/zero` steht ausdruecklich als **untaugliches** Beispiel dabei, mit dem
  Grund: Zeichengeraet, faellt am `!angaben.is_file()` heraus.
- **Stelle 3b**, dieselbe Datei, `anlesen`: **dieselbe falsche Begruendung ein zweites Mal**,
  vom Datensatz nicht genannt und beim Nachlesen gefunden. `anlesen` traegt dasselbe
  `!angaben.is_file()`, also erreicht `/dev/zero` auch dort die Schranke nie. Mitbehoben, mit
  Verweis auf die Schwesterstelle.
- **Stelle 4**, dieselbe Datei, `einlesen`: der Inhaltsfilter
  (`verzeichnis::inhalt::traegt_der_inhalt`) steht als dritter Weg ueber `String::from_utf8`
  dabei. **Statt einer Zahl** steht das Erhebungskommando
  `grep -rn 'String::from_utf8' crates/*/src`.
- **Stelle 5**, `crates/krk-ui/src/tabs.rs`, `ordner_setzen`: „die vierte Uebertragung, in
  derselben Bauart wie die drei darueber" heisst jetzt „die fuenfte … wie die vier darueber".
  Ueber ihr stehen Sortierung, Verstecke, `tief` und `inhalt`.
- **Stelle 6b**, `crates/krk-ui/src/appkit/tabelle.rs`: „stoesst, wenn ‚Deep' steht, einen
  neuen an" heisst jetzt „wenn **einer der beiden Filterschalter** etwas zu tun gibt", mit
  dem wirkenden Inhaltsfilter ohne „Deep" als dem Fall, den die alte Fassung uebersah. Die
  Bedingung im Rumpf ist heute `!tief_wirkt() && !inhalt_wirkt()`
  (`Tabliste::durchlauf_nachziehen_an`) und deckt sich damit.

Resolved: 260908 — `crates/krk-core/src/verzeichnis/verweisziel.rs`,
`crates/krk-core/src/text/datei.rs` (drei Stellen), `crates/krk-ui/src/tabs.rs`,
`crates/krk-ui/src/appkit/tabelle.rs`. Zwei der sechs waren bereits abgetragen, eine siebte
derselben Bauart ist beim Nachlesen gefunden und mitbehoben.
