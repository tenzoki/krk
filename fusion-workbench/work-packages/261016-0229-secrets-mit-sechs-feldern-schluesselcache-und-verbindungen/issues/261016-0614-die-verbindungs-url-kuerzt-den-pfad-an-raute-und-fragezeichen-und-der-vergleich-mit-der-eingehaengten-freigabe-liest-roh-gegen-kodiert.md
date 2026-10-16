Die Verbindungs-URL kürzt den Pfad an `#` und `?`, und der Vergleich mit der eingehängten Freigabe hält die rohe Adresse gegen die kodierte
---
Seit `9d1b111` kodiert `appkit::verbinden::url_aus` Leerzeichen und Umlaute im Pfad, und das Blatt nimmt solche Adressen ausdrücklich an. Zwei Stellen halten damit nicht Schritt: `gleiche_freigabe` vergleicht die rohe Adresse mit dem prozentkodierten `absoluteString` des Systems und findet eine schon eingehängte Freigabe mit Leerzeichen oder Umlaut nie; und `#` oder `?` in der Adresse sind gültige URL-Zeichen, werden nicht kodiert und schneiden den Pfad still ab.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** 261016-0306-plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md (Entscheidung 10)

## Defekt

**(a) Vergleich roh gegen kodiert.** Bei `EEXIST` ruft `einhaengen` `eingehaengte_freigabe(url)` mit der **rohen** URL aus dem Plan (`crates/krk-ui/src/appkit/verbinden.rs:335-336`). Dort wird sie gegen `wieder.absoluteString()` gehalten (`:380-381`), und `gleiche_freigabe` (`:395-411`) vergleicht das erste Pfadstück als Text, nur in Kleinschreibung gefaltet. `absoluteString` ist kodiert: für `smb://nas.local/Daten Ordner` liefert `url_aus` `smb://nas.local/Daten%20Ordner`. Das hält die Probe `die_url_kodiert_was_sich_kodieren_laesst` in derselben Datei fest, und eine Gegenprobe mit `NSURL(string:encodingInvalidCharacters:)` in Swift auf diesem Gerät ergibt dasselbe. `daten ordner` ≠ `daten%20ordner`, also `SchonEingehaengt(None)`, und der zweite Klick meldet „schon verbunden“, statt den Einhängepunkt zu zeigen (`anwendung.rs:9017`). **inference:** `NSURLVolumeURLForRemountingKey` liefert die URL kodiert wie jedes `NSURL`; an einer eingehängten Freigabe nicht nachgesehen.

**(b) `#` und `?` im Pfad.** Gemessen mit derselben Swift-Gegenprobe: `smb://nas.local/Daten#1/x` ergibt `path = /Daten`, `smb://nas.local/a?b` ergibt `path = /a`. Beide sind gültige URL-Zeichen, `encodingInvalidCharacters:` lässt sie stehen, und `url_aus` gibt eine URL zurück. Das Blatt nimmt die Adresse an (`appkit/blaetter/verbindung.rs:155-165`), und der Klick hängt still einen anderen Pfad oder eine andere Freigabe ein. Der neue Doc-Kommentar von `url_aus` und der Modulkopf von `krk-core/src/heimordner/verbindung.rs` sagen, was kodiert wird. Dass `#` und `?` den Pfad beenden, sagt keiner von beiden, `HowTo.md` auch nicht („KRK hängt `<protokoll>://<adresse>` ein“).

Beide Teile liegen vor `9d1b111` schon im Code, denn `URLWithString:` kodiert unter dem SDK von macOS 14 und später genauso. Der Commit sagt die Adresse mit Leerzeichen aber jetzt ausdrücklich zu und prüft sie im Blatt.

## Richtung der Behebung

Eine Stelle, an der aus der Adresse eine URL wird, und ein Vergleich über diese Form:

- (a) `eingehaengte_freigabe` vergleicht gegen die kodierte Form (`url_aus(url)?.absoluteString()`) oder zerlegt beide über `NSURL` (`host`, `path`) statt über Text. Die reine Probe von `gleiche_freigabe` bekommt einen Fall mit Leerzeichen.
- (b) Entweder kodiert der Kern `#` und `?` im Pfadteil der Adresse, oder das Blatt weist sie mit Grund ab. Gleich welche Wahl: Doc-Kommentar von `url_aus`, Modulkopf von `verbindung.rs` und `HowTo.md` nennen die Regel.

## Akzeptanz

- Eine Probe hält, dass `gleiche_freigabe` (oder ihr Nachfolger) `smb://nas.local/Daten%20Ordner` und `smb://nas.local/Daten Ordner` als dieselbe Freigabe erkennt.
- Eine Probe hält für `nas.local/Daten#1`, dass entweder das Blatt abweist oder die URL den ganzen Pfad trägt.
- `make check` grün.

---
Resolved: (a) `gleiche_freigabe` (`crates/krk-ui/src/appkit/verbinden.rs`) zerlegt beide URLs und vergleicht Rechner und Freigabe dekodiert ueber die neue reine Funktion `prozent_dekodiert`; Probe `eine_freigabe_wird_kodiert_und_roh_als_dieselbe_erkannt` (`Daten%20Ordner` gegen `Daten Ordner`, Umlaut, kodierte Raute). (b) Gewaehlt ist Kodieren, passend zur Protokolltabelle: `verbindungsziel` (`crates/krk-core/src/heimordner/verbindung.rs`, Helfer `ohne_anfrage_und_fragment`) setzt `#` und `?` auf dem Einhaengeweg als `%23` und `%3F` in die URL, weil eine Freigabe weder Anfrage noch Fragment kennt; auf dem Uebergabeweg bleiben beide stehen, weil sie in `http`/`https` Anfrage und Fragment sind. Modulkopf samt Tabelle von `verbindung.rs` und Doc-Kommentar von `url_aus` nennen die Regel. Proben `raute_und_fragezeichen_beenden_den_pfad_einer_freigabe_nicht` (`crates/krk-core/tests/heimordner.rs`) und `raute_und_fragezeichen_bleiben_im_pfad_der_freigabe` (verbinden.rs, `nas.local/Daten#1/x` ergibt `path` `/Daten#1/x`). `HowTo.md`, Abschnitt „Eine Verbindung herstellen“, Absatz nach der Protokolltabelle.
