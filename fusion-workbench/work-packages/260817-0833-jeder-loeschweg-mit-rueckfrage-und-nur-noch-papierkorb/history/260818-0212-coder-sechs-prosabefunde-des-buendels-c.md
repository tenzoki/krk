# Sechs Prosabefunde des Bündels C

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Baum bei Beginn:** `ae665e5`
**Abnahme:** `make check` — exit 0

## Auftrag

Sechs Datensätze aus der Durchsicht des Bündels C, alle derselben Klasse: eine Aussage in
einem Doc-Kommentar oder Modulkopf, die gegen den Code nicht stimmt. Nur Prosa unter
`crates/` — keine Logik, keine Signatur, keine Probe. Ausgenommen waren
`resources/default-keymap.toml`, `CLAUDE.md`, der Spec und die Prüfkörper, um die es in
`260817-1759`, `260817-2354` und `260817-2355` geht.

## Die sechs, und was jetzt dasteht

**`260817-1800` — zwei Modulköpfe schicken den Aufrufer zu `ist_warnwuerdig`.**
`arbeitsbaum.rs:91-113` behauptete als Tatsache „Der Aufrufer fragt `ist_warnwuerdig`".
Jetzt steht dort, dass die Frage für einen Wert dieser Polarität **zulässig wäre**, dass
sie kein Aufrufer stellt und dass ihre Einführung ein Fehler wäre: `Ja` führt auf „aus
einem Git-Arbeitsbaum", `Unentschieden` auf „von einem Ziel unbekannter Einordnung", und
eine zusammenfassende Frage kann die beiden nicht mehr trennen. `volumes.rs:70-78` heißt
jetzt „zulässige" statt „richtige" Frage und trägt dieselbe Regel; `volumes.rs:251-259`
verliert die falsche zweite Hälfte („gestellt dort, wo die Rangfolge steht") und nennt
stattdessen die zwei Wortlaute, die sie einebnen würde.

**`260817-1801` — zwei Modulköpfe nennen einen Aufrufer, der den Aufruf nicht enthält.**
`umfang.rs:136-149` und `arbeitsbaum.rs:163-176` nennen jetzt
`Anwendungsdelegierter::loeschtexte` und sagen, dass `loeschen_nach_rueckfrage` es im
**vierten** Zweig seiner Stufenregel erreicht. Der Halbsatz über die beiden billigen
Stufen bleibt stehen und ist damit erstmals in der Funktion nachlesbar, die den Aufruf
hält.

**`260817-1802` — zwei weitere „noch kein Aufrufer"-Aussagen.**
`loeschzielbefund.rs:121-144` verliert die „steht noch nicht alles da"-Rahmung und nennt
als Tafel die vier Prüfungen, die den Typ beantworten, mit ihrem Ort.
`loeschwarnung.rs:1253` verliert den Nachsatz „— heute keine".

**`260817-1803` — eine SDK-Zeilennummer für zwei Symbole.** `volumes.rs:133-136` gibt
jedem Namen seine eigene: `NSURLVolumeLocalizedNameKey` `NSURL.h:344`,
`NSURLVolumeIsLocalKey` `NSURL.h:338`.

**`260817-1805` — der dritte Umgang mit einem fehlenden Benutzerverzeichnis.**
`ablage/pfade.rs:187-209` zählt keine Aufrufer mehr, sondern führt die drei Umgangsformen
auf: scheitern, auf `/` ausweichen, unentschieden bleiben — mit je einem Aufrufer und mit
der Begründung, warum die dritte die ist, die ein neuer Aufrufer am ehesten verfehlt.

**`260817-1806` — „die Folgen" als dritter Unterschied.**
`blaetter/loeschbestaetigung.rs:10-23` nennt den dritten Unterschied so, wie der Code ihn
erzeugt: der **erste** Warngrund in der Frage, die **übrigen** als eigener Absatz der
Erläuterung. Ein Satz sagt ausdrücklich, dass kein Satz über Folgen hinzukommt.

## Zwei Stellen derselben Klasse mitgezogen

Beide in `crates/krk-ui/src/kommandos/loeschwarnung.rs` und beide dieselbe Verwechslung
wie `260817-1801`:

- `:208-215` sagte, `loeschen_nach_rueckfrage` rufe im vierten Zweig `warngruende` und
  `frage_und_erlaeuterung`. Beide Aufrufe stehen in `loeschtexte`; der Satz führt jetzt
  darüber.
- `:1262-1266`, der Rumpf von `die_ausloesertafel_hat_genau_einen_aufrufer`, nannte
  ebenfalls `loeschen_nach_rueckfrage` als den einen Aufrufer. Berichtigt auf
  `loeschtexte`.

Gefunden wurden beide mit `grep -rn "loeschen_nach_rueckfrage" crates/`, also mit der
Suche über den ganzen Baum, die `260817-1802` als die eigentliche Lehre benennt.

## Zwei Zahlen durch Regeln ersetzt

Beide Ersetzungen verlieren keine Auskunft und veralten nicht mehr mit dem nächsten
Aufrufer:

- `loeschzielbefund.rs`: „Solange keine davon dasteht, hat der Typ in dieser Kiste keinen
  Aufrufer, und `dead_code` trifft ihn trotzdem nicht" → „Ob `dead_code` ihn trifft, hängt
  nicht daran, wer ihn ruft." `krk-core` ist eine Bibliothek, der Typ von ihrer Wurzel aus
  erreichbar; die Ausnahme nach dem Vorbild von `kommandos/rueckschritt.rs` bräuchte er
  auch ohne Aufrufer nicht.
- `ablage/pfade.rs`: „Zwei Aufrufer hängen daran" → die drei Umgangsformen, jede mit einem
  Aufrufer, dazu der ausdrückliche Satz, dass hier keine Zahl steht und warum.

Daneben trägt `volumes.rs:135-136` jetzt die Regel, aus der der Befund `260817-1803`
entstanden ist: eine Zeilenangabe für ein Paar stimmt beim Nachlesen für höchstens einen
der beiden.

## Wie jede neue Aussage geprüft wurde

| Aussage | geprüft mit |
|---|---|
| `ist_warnwuerdig` hat keinen Aufrufer im Programm | `grep -rn "ist_warnwuerdig" crates/` — jeder Aufruf steht in einer Probe |
| die zwei Wortlaute, die eine Zusammenfassung einebnen würde | `Warngrund::wortlaut`, `loeschwarnung.rs:528-533` |
| `warngruende` schreibt alle drei Antworten aus | Rumpf `loeschwarnung.rs:670-681` |
| `loeschtexte` hält die beiden Aufrufe | `grep -rn "umfang::zaehlen\|beruehrt_einen_arbeitsbaum" crates/` → `anwendung.rs:4799`, `:4810`, beide im Rumpf ab `:4784` |
| vierter Zweig der Stufenregel | `match` in `loeschen_nach_rueckfrage`, `anwendung.rs:4647-4692`; `Vorstufe::Rueckfrage` ist der vierte Arm |
| die vier Prüfungen, die `Loeschzielbefund` beantworten | `grep -rn -- "-> Loeschzielbefund" crates/`, dazu `umfang.rs:183-196` für den Weg über `Umfang` |
| beide SDK-Zeilennummern, und die vier daneben | `grep -n` bzw. `sed -n` im SDK unter `$(xcrun --show-sdk-path)/…/Foundation.framework/Headers/` |
| die drei Umgangsformen sind vollständig | `grep -rn "benutzerverzeichnis()" crates/` — sechs Aufrufstellen im Programm, zwei in `#[test]`-Rümpfen; jede der sechs fällt in eine der drei |
| die laute Erläuterung gewinnt Gründe und keine Folge | Rumpf `frage_und_erlaeuterung`, `loeschwarnung.rs:756-788` |
| keine „Folgen"-Aussage bleibt stehen | `grep -rn "Folgen in der Erl\|die Folgen" crates/` |
| keine „noch kein Aufrufer"-Aussage bleibt stehen | `grep -rn "keinen Aufrufer\|noch keinen\|Zum Zeitpunkt dieses Schrittes\|heute keine\|noch keine" crates/` — neun Treffer, keiner davon eine Aufrufer-Aussage |

## Abnahme

`make check` — exit 0, „alle vier gruen".

Zusätzlich `cargo doc --no-deps --workspace` vor und nach dem Durchgang: 107 Warnungen in
beiden Läufen, also keine neue unaufgelöste Doc-Verweisung durch die neuen Verweise
(`super::umfang::Umfang` in `loeschzielbefund.rs` und der Verweis auf
`frage_und_erlaeuterung` in `loeschbestaetigung.rs`).

## Offen geblieben

Aus `260817-1806` bleibt die eine Zeile im Spec stehen:
`shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C3, letztes
Abnahmekriterium, sagt weiterhin „den Folgen in der Erläuterung". Der Spec ist vom Nutzer
abgenommen und liegt außerhalb dieses Auftrags. Das wirksame Kriterium zwei Punkte darüber
(„Treffen mehrere Auslöser zugleich zu …") erfüllt der Baum; der zusammenfassende Punkt
nennt dieselbe Sache unter einem irreführenden Wort. Eine Zeile für den Durchgang des
Bündels E über überholte Formulierungen (C6).

## Was der nächste Durchgang wissen sollte

Die Verwechslung von `loeschen_nach_rueckfrage` und `loeschtexte` steckte an vier Stellen
und wurde in zwei Datensätzen erfasst. Der Grund ist derselbe wie in `260817-1802`: der
Schritt, der `loeschtexte` anlegte, hat seine Sätze über die Dateien gesucht, die er
angefasst hatte, und nicht über den Namen, den er verschoben hat. Eine Suche nach dem
alten Namen hätte alle vier auf einmal geliefert.
