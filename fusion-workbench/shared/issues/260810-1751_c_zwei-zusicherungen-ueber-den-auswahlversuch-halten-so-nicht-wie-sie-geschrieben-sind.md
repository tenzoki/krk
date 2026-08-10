Zwei Zusicherungen über den `Auswahlversuch` halten so nicht, wie sie geschrieben sind

---

Commit `6964dde` hat drei Doc-Kommentare in `crates/krk-ui/src/appkit/anwendung.rs` um je
eine Zusicherung über `Auswahlversuch` ergänzt. Zwei davon sind stärker formuliert, als der
Programmtext sie trägt. Am Verhalten ändert das nichts — der Rückgabewert wird an allen drei
Stellen verworfen —, aber ein späterer Leser, der die Zusicherung glaubt, baut auf ihr
(`debug_assert!`, `expect`, ein Zweig weniger).

---

**Schwere:** Niedrig
**Gefunden:** coderev, Durchsicht des Codeanteils von Turn 1
(`shared/history/260810-1647-orchestrator-session.md`)
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs`
**Domain:** code

## Teil 1: `anlegen_ausfuehren` sagt „nie `Unbekannt`", und ein Weg dorthin steht offen

`anwendung.rs:2661-2662`:

> Ihre Antwort ist damit `Vorgemerkt` und nie `Unbekannt`; der Rueckgabewert traegt hier
> keine Auskunft.

Die Begründung trägt für `umbenennen_ausfuehren` (`anwendung.rs:2704-2712`): dort wird
`ordner` unmittelbar vor `ordner_neu_lesen` aus derselben Seite geholt, alles läuft in einem
synchronen Durchgang auf dem Hauptfaden, und `gleicher_ordner` vergleicht denselben Wert mit
sich selbst. Dort ist die Zusicherung haltbar.

Für `anlegen_ausfuehren` trägt sie nicht. `ordner` und `seite` werden in `anlegen`
(`anwendung.rs:2635-2636`) **vor** dem Blatt festgehalten, und das Blatt steht über
`beginSheetModalForWindow:completionHandler:` (`appkit/blaetter/mod.rs:508`), also ohne
eigene Ereignisschleife: die Laufschleife läuft weiter und stellt Meldungen zu. Der
Fokusvorbehalt und `blatt_steht` halten in dieser Spanne jeden **Befehl** an — aber nicht die
Datenträgerwache, die kein Befehl ist.

Der erreichbare Weg, Schritt für Schritt:

1. Das aktive Dateifenster zeigt einen Ordner auf einem externen Datenträger, das Blatt für
   `f7`/`shift+cmd+n` steht.
2. Der Nutzer wirft den Datenträger anderswo aus (Finder, `diskutil`; die Geräteleiste selbst
   ist vom Blatt gesperrt). `NSWorkspaceWillUnmountNotification` kommt an.
3. `datentraeger_gewechselt` (`anwendung.rs:1836-1842`) ruft `datentraeger_verloren`, und das
   schiebt **jeden** getroffenen Tab beider Seiten auf das Benutzerverzeichnis
   (`auffrischung.rs:368-370`, `sicht.tab_wechseln`).
4. Der Datenträger ist dabei noch eingehängt — das steht ausdrücklich im Kommentar bei
   `anwendung.rs:1821-1823`: „`willUnmount` zaehlt dabei noch mit: der Datentraeger ist bis
   zum Auswurf eingehaengt."
5. Der Nutzer bestätigt das Blatt. `operation::ordner_anlegen(ordner, &name)` gelingt, weil
   der Datenträger noch steht; die frühe Rückkehr bei `Err` greift also nicht.
6. `ordner_neu_lesen(self, ordner)` (`anwendung.rs:2686`) findet keine Seite mehr, die
   `ordner` zeigt: `gleicher_ordner` vergleicht `~` gegen den Pfad auf dem Datenträger.
   Null Auffrischungen, kein Lesevorgang.
7. `eintrag_waehlen(&name)` befragt das Modell des Benutzerverzeichnisses. `liest()` ist
   falsch, der Name steht dort nicht: `Unbekannt`.

Das ist genau der Weg, den der neue Kommentar im Zweig `Art::UmbenennenImStapel`
(`anwendung.rs:3186-3196`) für sich selbst beschreibt — „wechselt der Nutzer waehrenddessen
den Ordner dieser Seite" —, nur dass ihn hier nicht der Nutzer über einen Befehl auslöst,
sondern die Datenträgerwache.

## Teil 2: „die drei Aufrufer" sind fünf, und zwei von ihnen behandeln `Unbekannt`

`anwendung.rs:3188-3190`:

> `Auswahlversuch::Unbekannt` ist von den drei Aufrufern allein hier erreichbar.

`eintrag_waehlen` hat fünf Aufrufstellen:

| Stelle | Umgang mit `Unbekannt` |
|---|---|
| `appkit/tabelle.rs:1057` (`eintrag_anspringen`, C10) | meldet „{name} steht nicht in der Liste" — das ist der Zweck der Funktion |
| `appkit/anwendung.rs:2687` (`anlegen_ausfuehren`) | verworfen |
| `appkit/anwendung.rs:2711` (`umbenennen_ausfuehren`) | verworfen |
| `appkit/anwendung.rs:3204` (`vorgang_beenden`) | verworfen, mit dem neuen Kommentar |
| `appkit/anwendung.rs:4245` (`messhandlung`, `Handlung::Auswaehlen`) | gibt `Err` an den Messlauf zurück (`anwendung.rs:4256-4262`) |

Gemeint sind ersichtlich die drei Aufrufer aus dem zitierten Datensatz
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260807-0219_c_drei-aufrufer-von-eintrag-waehlen-werfen-den-auswahlversuch-weg.md`,
also die drei, die den Wert wegwerfen. Im Programmtext steht die Zahl aber ohne diesen
Bezug und neben dem Namen `eintrag_waehlen`. Wer den Verweis nicht aufmacht, liest
„`Unbekannt` kommt praktisch nur hier vor" — und das ist falsch, denn die auffälligste
Behandlung dieses Wertes steht in `eintrag_anspringen`, wo genau der Satz gemeldet wird, den
der neue Kommentar zwei Zeilen weiter als „eher Rauschen als Auskunft" verwirft.

## Denkbarer Weg

Beides sind Textänderungen, keine Verhaltensänderungen.

Zu Teil 1: die Zusicherung auf das beschränken, was gilt. Etwa — solange kein Befehl den
Ordner der Seite wechseln kann, während das Blatt steht, ist die Antwort `Vorgemerkt`; die
eine Ausnahme ist der Auswurf des Datenträgers, der über die Datenträgerwache und nicht über
einen Befehl läuft, und für sie gilt dieselbe Abwägung wie im Zweig `UmbenennenImStapel`:
gemeldet wird nichts. Damit stehen beide Stellen auf **einer** Begründung statt auf zwei, von
denen die eine die andere ausschließt.

Zu Teil 2: „von den drei Aufrufern" durch die gemeinte Menge ersetzen — „von den drei
Aufrufern, die den Wert verwerfen" — und daneben nennen, dass `eintrag_anspringen` und
`messhandlung` ihn auswerten. Sonst steht im selben Absatz eine Meldung als Rauschen
abgetan, die eine andere Stelle bewusst zeigt.

## Dringlichkeit

Gering. Kein Abnahmekriterium, keine der zehn Zeitzusagen aus C8 und kein sichtbares
Verhalten sind berührt; der Weg in Teil 1 verlangt einen Auswurf bei stehendem Blatt. Der
Schaden ist der einer falschen Zusicherung im Programmtext, und der fällt erst bei dem an,
der als nächster auf sie baut.

---
Resolved: Der Weg der Durchsicht ist am Programmtext nachgeprueft und haelt. Belegkette:
`anlegen` haelt `ordner` und `seite` bei `anwendung.rs:2635-2636` fest, `datentraeger_verloren`
(`auffrischung.rs:370-372`) ruft `tab_wechseln` fuer jeden getroffenen Tab, und
`tab_ordner_setzen` (`tabelle.rs:494-497`) schickt den sichtbaren Tab ueber `ordner_lesen` auf
das Benutzerverzeichnis. Danach vergleicht `gleicher_ordner` das Benutzerverzeichnis gegen den
Pfad auf dem Datentraeger, also frischt `ordner_neu_lesen` nichts auf.

**Eine Verfeinerung gegenueber der Durchsicht:** die Antwort ist `Unbekannt` nur dann, wenn der
Lesevorgang des Benutzerverzeichnisses zum Zeitpunkt der Bestaetigung schon fertig ist. Laeuft er
noch, kommt `Vorgemerkt` zurueck, und der vorgemerkte Name wird im falschen Ordner nie gefunden.
Der gewoehnliche Fall ist der erste, weil der Nutzer den Namen erst tippen und bestaetigen muss.
Beide widerlegen die Zusicherung "nie `Unbekannt`", und behandelt werden beide gleich.

Geaendert sind drei Dinge in `crates/krk-ui/src/appkit/anwendung.rs`:
1. Der Doc-Einzeiler an `anlegen_ausfuehren` sichert nichts mehr zu, sondern beschreibt den
   erreichbaren Weg und verweist auf dieselbe Abwaegung wie der Zweig `UmbenennenImStapel`.
2. Der Aufruf ist ein begruendetes `let _ =` geworden. Damit traegt der Programmtext die
   Unterscheidung selbst: ein nacktes `eintrag_waehlen(...)` heisst "`Unbekannt` kann hier nicht
   eintreten" — so bleibt `umbenennen_ausfuehren`, dessen Zusicherung haelt —, ein `let _ =`
   heisst "kann eintreten und wird bewusst verworfen".
3. "von den drei Aufrufern" ist ersetzt. Der Satz nennt jetzt die fuenf Aufrufstellen, die zwei
   auswertenden namentlich und die drei verwerfenden, und stimmt ohne den Datensatz 260807-0219.
   Dazu ein Halbsatz, der die Meldung aus `eintrag_anspringen` gegen das Schweigen hier abgrenzt.
Abgenommen mit `make check`, exit 0.

Geschlossen in der Sitzung `shared/history/260810-1647-orchestrator-session.md`, Turn 2.
