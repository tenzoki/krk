# Codedurchsicht: Runde 17, vierte Durchsicht — der Behebungscommit der dritten

**Reviewed-range:** `f464bc5..95e55da`
**Not-opened:** none

**Geoeffnet:** `95e55da` als Unterschied in voller Laenge, dazu die fuenf beruehrten Codedateien am
Baumstand `95e55da` an allen beruehrten Stellen und ihren Nachbarn —
`crates/krk-ui/src/kommandos/kontextmenue.rs`, `crates/krk-ui/src/appkit/anwendung.rs`,
`crates/krk-ui/src/kommandos/operationen.rs`, `crates/krk-core/src/operation/zippen.rs`,
`crates/krk-core/src/verzeichnis/sys.rs` — und die gerufenen Nachbarn
`crates/krk-core/src/operation/umbenennen.rs` (`namen_teilen`),
`crates/krk-ui/src/quellbaum.rs` (`aufrufstellen`, `quelldateien`) und
`crates/krk-ui/src/appkit/statuszeile.rs` (Rangfolge). Gelesen sind ausserdem die sechs eigenen
Defektdatensaetze vom `260825-1249` samt ihrer `Resolved:`-Notizen, `shared/issues/260825-0727_*`
mit seiner Schlussnotiz und der Verlaufseintrag
`history/260825-1330-coder-sechs-befunde-der-dritten-durchsicht.md`.

**Hier gefahren am 260825:** `cargo test --workspace` (Exit 0, kein Fehlschlag),
`cargo clippy --workspace --all-targets` (Exit 0, keine Warnung), `cargo fmt --all --check`
(Exit 0). Dazu zwei Nachstellungen ausserhalb des Baums: der Rumpf von `gleicher_eintrag` gegen
sechs Namenspaare, und der Rumpf von `ohne_die_eigenen_ziele` ueber **alle** Eingabereihenfolgen von
drei, vier und fuenf gestaffelten Archiven, mit einer Festpunktzusicherung nach jedem Lauf.

Diese Durchsicht schliesst an `260825-1249-coderev-…` (`6faaa91..ddd41ff`),
`260825-1144-coderev-…` (`6ad9198..6faaa91`) und `260825-0942-coderev-…` (`428fbc4..423d5f2`) an.
Die vier tilen zusammen mit dem `CLAUDE.md`-Commit `8c111ea` und dem Werkbank-Commit `f464bc5` den
ganzen Sitzungsbereich `428fbc4..95e55da`.

---

## Zusammenfassung

Fuenf der sechs Befunde sind behoben, und keiner davon ist zugedeckt: jede Behebung ist am Code
nachgerechnet und nicht bloss an ihrer Notiz nachgelesen. Der sechste, B10, ist **verengt und nicht
geschlossen** — die Faltung greift auf ASCII, das Bauziel faltet Unicode, und der Doc-Kommentar
schreibt die eine Ungenauigkeit aus und die andere nicht. Das ist der einzige Befund dieser
Durchsicht.

Zwei Zusagen des Commits sind streng geprueft und halten: die Reihenfolge in `ohne_die_eigenen_ziele`
folgt tatsaechlich aus `paar` und ist keine Annahme, und die Diagnose, der Vorschlag des B14-Datensatzes
haette eine blinde Probe ergeben, stimmt.

## Summen

| Schwere | Zahl |
|---|---|
| Kritisch | 0 |
| Hoch | 0 |
| Mittel | 1 |
| Gering | 0 |

## Die fuenf Stellen, einzeln beantwortet

### 1. Die Schreibung (B10) — verengt, nicht geschlossen. Der einzige Befund.

**Was gebaut ist, ist die Nutzerwahl.** `gleicher_eintrag` (`kontextmenue.rs:689-699`) vergleicht
den letzten Bestandteil ueber `eq_ignore_ascii_case` auf `as_encoded_bytes()` und den Elternteil
buchstabengetreu, ohne Dateizugriff. Die Regel steht weiterhin genau einmal da, beide Rufer erben
sie (`ist_ziel_des_laufs`, `:676-678`), und L9 bleibt unberuehrt: keine Zeile fasst die Platte an.

**Der Fall, den der Datensatz vorfuehrt, ist geschlossen.** Nachgestellt und gefahren:
`PROJEKTE.ZIP` gegen das gerechnete `Projekte.zip` gibt `true`. Gehalten von
`das_archiv_des_vorigen_laufs_faellt_auch_in_abweichender_schreibung` und
`der_entpackschnitt_trifft_auch_in_abweichender_schreibung`, die Grenze von
`ein_aehnlich_benanntes_archiv_bleibt_quelle`.

**Der Fall daneben ist es nicht.** `eq_ignore_ascii_case` faltet ASCII-Buchstaben und vergleicht
jedes Byte darueber buchstabengetreu. Gemessen am nachgestellten Rumpf:

| Eintrag | gerechnetes Ziel | `gleicher_eintrag` | APFS in der Vorgabe |
|---|---|---|---|
| `PROJEKTE.ZIP` | `Projekte.zip` | true | ein Eintrag |
| `ÜBERSICHT.ZIP` | `Übersicht.zip` | true | ein Eintrag |
| `übersicht.zip` | `Übersicht.zip` | **false** | ein Eintrag |
| `äpfel.zip` | `Äpfel.zip` | **false** | ein Eintrag |

Die zweite Zeile faellt richtig aus, weil `Ü` in beiden Namen dasselbe Byte-Paar ist. Die dritte und
die vierte sind der Befund: fuer jeden Namen, dessen Schreibungsunterschied auf einem Buchstaben
ausserhalb ASCII liegt, laeuft der Konfliktweg genau wie vor `95e55da`, und „Ueberschreiben" raeumt
wieder eine Quelle desselben Laufs in den Papierkorb.

**Zur Frage, ob die Ungenauigkeit ehrlich dasteht: die eine ja, die andere nicht.** Der
Doc-Kommentar (`:669-675`) schreibt die Ungenauigkeit auf einem schreibungsempfindlich formatierten
Datentraeger vollstaendig aus, mit Folge und Kosten, so wie der Nutzer es verlangt hat. Von der
Verengung auf ASCII steht dort nichts, und die Ueberschrift daneben (`:656-657`) sagt ohne
Einschraenkung „ohne Ruecksicht auf Gross- und Kleinschreibung". Was der Nutzer in Kauf genommen
hat, ist die zu weite Faltung; die zu enge hat ihm niemand vorgelegt. In einem Vorhaben, dessen
Prosa deutsch ist und dessen Ordner Umlaute tragen, ist das kein Randfall.

Datensatz:
`issues/260825-1358_*_die-faltung-des-schnitts-gilt-nur-ascii-und-der-doc-kommentar-nennt-allein-die-andere-ungenauigkeit.md`,
Schwere mittel, drei Wege vorgelegt. Er ist der **Rest** des geschlossenen Datensatzes und nicht sein
Widerruf: die Nutzerwahl steht, sie ist nur nicht ganz gebaut.

### 2. Der Festpunkt (B11) — behoben, und die Reihenfolge folgt wirklich aus `paar`.

**Gepruefte Behauptung.** Der Bau sagt: ein Zielname ist der um `ENDUNG` gekuerzte Archivname und
damit vier Zeichen kuerzer, also kommt in absteigender Pfadlaenge jeder Beansprucher vor dem
Beanspruchten. Nachgerechnet in drei Schritten, alle drei tragen:

1. `ist_zipname` (`:344-347`) bejaht nur, wenn `namen_teilen` eine Endung liefert, die
   `eq_ignore_ascii_case(".zip")` erfuellt. `eq_ignore_ascii_case` verlangt gleiche Bytelaenge, also
   ist die Endung genau vier Bytes, und `namen_teilen`
   (`krk-core/src/operation/umbenennen.rs:177-182`) teilt am letzten Punkt. Der Stamm ist damit der
   Name minus vier Bytes, exakt.
2. Der Ausweg ueber `ERSATZSTAMM` bricht die Rechnung nicht, sondern faellt aus ihr heraus:
   `ERSATZSTAMM` ist `"Archiv"`, endet nicht auf `.zip` und kann deshalb kein Archiv der Liste
   treffen, gleich welcher Laenge. Eine Beanspruchung entsteht dort gar nicht erst.
3. Die Faltung aendert daran nichts: `eq_ignore_ascii_case` trifft nur gleich lange Namen, und die
   Elternteile werden buchstabengetreu verglichen. Auch mit einem Archiv in einem tieferen Ordner
   (Deep) bleibt der Beansprucher der laengere Pfad, denn sein Ziel liegt im angezeigten Ordner.

**Gemessen fuer vier und fuenf, wie verlangt.** Der Rumpf ist ausserhalb des Baums nachgestellt und
ueber **jede** Eingabereihenfolge gefahren, mit einer Festpunktzusicherung nach jedem Lauf:

| Kette | Ergebnismengen ueber alle Permutationen | bleibt |
|---|---|---|
| `a.zip … a.zip.zip.zip` (3) | genau eine | `a.zip`, `a.zip.zip.zip` |
| dieselbe mit vier Gliedern | genau eine | `a.zip.zip`, `a.zip.zip.zip.zip` |
| dieselbe mit fuenf Gliedern | genau eine | `a.zip`, `a.zip.zip.zip`, `a.zip.zip.zip.zip.zip` |
| `{a.zip, a.zip.zip, b.zip, b.zip.zip}` | genau eine | `a.zip.zip`, `b.zip.zip` |

Zwei aus vier und drei aus fuenf sind zugleich das Hoechste, was ein Festpunkt auf einer Kette
zulaesst. Das Ergebnis haengt nicht an der Eingabereihenfolge, obwohl `sort_by_key` nur nach Laenge
sortiert: gleich lange Archive koennen einander nicht beanspruchen. Und der Ausgang ist beweisbar ein
Festpunkt und nicht bloss eine zweite Runde — ein spaeter drankommendes Archiv ist kuerzer, sein Ziel
noch einmal vier Zeichen kuerzer, kann also kein bereits behaltenes treffen.

Der Doc-Kommentar (`:701-728`) schreibt genau das aus, samt der `ERSATZSTAMM`-Ausnahme, und die
falsche Zeile von `dd74b0e` („aus der Kette bleibt das letzte") ist weg. Probe:
`aus_einer_kette_von_drei_archiven_bleiben_zwei`.

### 3. Die Meldung (B12) — erreicht den Nutzer, und die Zahl stimmt.

**Der Weg ist durchgegangen, Glied fuer Glied.** `zipauftrag_stellen` rechnet
`auswahl.pfade.len() - quellen.len()` (`anwendung.rs:6155`) und reicht es an `auftrag_starten`
weiter; `entpackauftrag_stellen` nimmt es aus `Entpackbefund::Archive { paare, ausgelassen }`;
`auftrag_starten` legt es in `Vorgang::ausgelassen` (`:6309`), und der Abschluss gibt es an
`operationen::abschlusstext` (`:6470-6475`), dessen Ergebnis `antwort_zeigen` auf den ersten Rang
der Statuszeile stellt. `Vorgang` wird im Baum an genau einer Stelle gebaut, also kann die Zahl auf
dem Weg durch das Konfliktblatt nicht verlorengehen; `abschlusstext` hat in `krk-ui` genau einen
Rufer.

**Kein Unterlauf.** `packziel` bekommt `&auswahl.pfade` und filtert daraus; `quellen` ist eine
Teilmenge, die Subtraktion kann nicht negativ werden. Auf der Entpackseite ist
`markiert - betroffene_archive.len()` aus demselben Grund sicher, und `ohne_die_eigenen_ziele` gibt
zu einer nichtleeren Eingabe nie eine leere Liste heraus, sodass der Zweig mit der gerechneten Zahl
auch wirklich genommen wird. Die Ersatzregel traegt fest `ausgelassen: 0` und schneidet nichts.

**Zur Frage nach den zwei Schneidern zugleich: das kann nicht eintreten.** `packziel` gehoert Zip,
`ohne_die_eigenen_ziele` gehoert Unzip; ein Vorgang traegt genau eine `Art`. Was zusammentreffen
kann, sind Ueberspringen und Auslassen, und das komponiert richtig: `abschlusstext`
(`operationen.rs:566-596`) haengt erst den Halbsatz zu den uebersprungenen und dann den zu den
ausgelassenen an, beide nur bei Zahl groesser null, beide ueber `eintraege_text` mit Singular und
Plural. Die Reihenfolge ist begruendet und die Begruendung steht dabei. Probe:
`der_abschlusstext_nennt_die_ausgelassenen_eintraege` prueft beide Richtungen.

**Der Ort der Meldung weicht vom Vorschlag ab, und die Abweichung ist die bessere Wahl.** Der
Datensatz schlug die Antwort beim Auftragstellen vor. Gebaut ist sie im Abschlusstext, mit dem
Grund, dass eine Befehlsantwort ueber der Vorgangsanzeige steht (`statuszeile::Rang`) und eine
Meldung vor dem Start damit genau den Fortschritt verdeckt haette, den sie ankuendigt. Nachgesehen
an der Rangfolge: stimmt. Der Fall, den der Datensatz vorfuehrt — Unzip auf `{a.zip, a.zip.zip}`,
der Nutzer waehlt „Ueberspringen" —, endet jetzt auf einem Abschlusstext, der das ausgelassene
Archiv nennt.

### 4. Das gehaltene Glied (B13) — die zwei Proben halten es zusammen.

**Die Paarungsprobe faengt die Umgehung, die der Datensatz beschreibt.**
`der_packauftrag_reicht_die_quellen_aus_packziel_weiter` verlangt genau eine Zeile im Rumpf von
`zipauftrag_stellen`, die `kontextmenue::packziel(` **und** `quellen` traegt, und genau eine, die
`Auftrag::zippen(` **und** `quellen` traegt. Wer `packziel` weiter ruft, davon nur das Ziel nimmt und
`auswahl.pfade` weiterreicht, faellt am ersten Glied auf null. Der `rumpf`-Helfer
(`anwendung.rs:8285-8299`) wirft Kommentarzeilen weg, sucht `fn {name}(` und bricht mit `panic!` ab,
wenn Kopf oder Ende fehlen — kein stiller Ausgang, und der Kommentar im Rumpf, der `quellen` nennt,
kann die Zaehlung nicht heben.

**Die Aufruferzaehlung faengt den zweiten Eingang.**
`ein_packauftrag_entsteht_in_der_oberflaeche_genau_einmal` verlangt, dass `Auftrag::zippen(` in
`krk-ui/` an genau einer Datei und dort genau einmal steht. `aufrufstellen`
(`quellbaum.rs`) laesst Kommentarzeilen aus und verlangt, dass dem Namen kein Bezeichnerzeichen und
kein `fn` vorausgeht; beide Proben nennen ihre Nadel ueber `concat!`, zaehlen sich also nicht selbst.
Die Beschraenkung auf `krk-ui` ist begruendet und die Begruendung steht dabei.

**Zusammen decken sie beide Wege zurueck in den Defekt**, und was sie nicht sehen, steht in ihren
Doc-Kommentaren: eine Zuweisung zwischen den zwei Zeilen und ein Aufruf ueber `type X = Auftrag;`.
Das ist die Grenze, die der Kopf von `crate::quellbaum` fuer jede Suche im Quelltext ohnehin
ausschreibt, und sie ist hier benannt statt verschwiegen. Der Modulkopf von `zippen.rs` (`:74-86`)
nennt beide Proben beim Namen; „sichert der Rufer" steht nicht mehr allein da.

**Kein Befund am fehlenden Zwilling fuer `Auftrag::entpacken`,** und der Grund ist die Bauform und
nicht die Nachlaessigkeit: `Auftrag::entpacken` nimmt **Paare** und keine zwei Listen, „damit die
beiden Listen gar nicht erst getrennt uebergeben werden koennen"
(`krk-core/src/operation/auftrag.rs:164-169`). Die Umgehung, die B13 beschreibt, hat auf jener Seite
keine Entsprechung; die Paare kommen als Ganzes aus `entpackziel`, und wer sie umginge, muesste
`paar` von Hand nachbauen.

### 5. Die Probe zu B14 — sie sieht jetzt, was sie sehen soll, und die Diagnose stimmt.

**Die Diagnose des Coders ist richtig.** Der Datensatz schlug vor: „ein Archiv markiert,
`entpackziel` gerufen, das Paar steht. Fuenf Zeilen." So gebaut waere die Probe blind, und der Weg
ist am Rumpf von `entpackziel` (`kontextmenue.rs:588-620`) nachvollzogen: bei einem zu weiten Schnitt
kaeme `ohne_die_eigenen_ziele` leer zurueck, der erste Zweig fiele aus, und die Ersatzregel lieferte
mit **einem** sichtbaren Archiv genau dasselbe Paar mit `ausgelassen: 0` — Wort fuer Wort das, was
die Probe erwartet. Sie waere gruen geblieben. Der Vorschlag war an dieser Stelle falsch, und der
Datensatz sagt es jetzt selbst.

**Der Zusatz schliesst die Luecke.** Mit `modell_mit(&["sicherung.zip", "anderes.zip"])` findet die
Ersatzregel zwei sichtbare Archive, `sichtbare.next().is_some()` trifft zu und sie antwortet
`Entpackbefund::Mehrere` — ein anderer Zweig der Aufzaehlung, an dem `assert_eq!` scheitert. Die
Probe sieht damit genau den zu weiten Schnitt, gegen den sie steht. Der Doc-Kommentar von
`ein_einzelnes_archiv_bleibt_seine_eigene_quelle` traegt jetzt die Einschraenkung, dass sie den
Packschneider prueft und nur ihn, und verweist auf die neue Probe.

## Die uebrigen zwei Behebungen

**B15 — fuenf Zitate, nicht drei.** Nachgezaehlt am Baum: alle fuenf umgebrochenen Verweise stehen
wieder auf einer Zeile, also auch die zwei aelteren aus fruehen Commits derselben Runde, denen der
Datensatz ausdruecklich mitgalt (`zippen.rs:40-41`, `93-94`, `65-66`, `kontextmenue.rs:87-88`,
`anwendung.rs:9054-9055`). `cargo fmt --all --check` gibt Exit 0.

**Die Codehaelfte von `shared/issues/260825-0727_*` — die Zahl stimmt jetzt an allen drei Stellen.**
Nachgezaehlt: `grep -rn 'ohne_warten_oeffnen(' crates/krk-core/src` gibt fuenf Aufruferzeilen
(`text/datei.rs:434,620,692`, `operation/zippen.rs:362`, `operation/entpacken.rs:118`), das enge
Muster `sys::ohne_warten_oeffnen(` findet vier davon. Die ASCII-Skizze fuehrt fuenf, die Aufzaehlung
nennt Klassen statt Namen und benennt die Falle des engen Musters, und der Schlusssatz sagt, dass die
Zahl keine Zusage ist. Der Zusatz, dass `anlesen` die Groesse gar nicht gegen eine Grenze haelt, ist
richtig und war vorher falsch beschrieben.

## Die Projektbindungen

Alle geprueft, alle halten.

- **`#[must_use]`**: `gleicher_eintrag` ist ein privater Helfer neben `paar` und `ist_archivpfad`,
  die es ebenso wenig tragen; sein Wert wird an genau einer Stelle unmittelbar verbraucht. Keine
  Stelle, an der ein stilles Fallenlassen unbemerkt bliebe.
- **`#![deny(unsafe_code)]`**: unberuehrt. `zippen.rs` und `sys.rs` haben allein Prosa bekommen; der
  Unterschied an `sys.rs` zeigt keine Codezeile.
- **Abschnitt „Ab welchem macOS die angesprochenen Klassen stehen"**: keine neue Datei unter
  `appkit/`, kein Abschnitt angetastet.
- **Vollstaendige Fallunterscheidungen ohne Auffangzweig**: der Umbau von `Entpackbefund::Archive`
  zur Strukturvariante laesst die drei Zweige in `entpackauftrag_stellen` vollstaendig und ohne
  `_ =>`; das Feld wird an jeder der acht Probenstellen ausgeschrieben, der Uebersetzer haette eine
  vergessene angehalten.
- **L9 (kein Dateisystemzugriff, der den Hauptfaden anhaelt)**: gehalten. `gleicher_eintrag`,
  `ist_ziel_des_laufs` und `ohne_die_eigenen_ziele` rechnen auf Pfadwerten. Der neue Sortierschritt
  ist O(n log n) ueber die Zahl der markierten Archive und fasst nichts an.
- **Prosa deutsch**: durchweg.

## Was quer liegt

**Der Behebungscommit hat zweimal mehr getan als sein Auftrag, und beide Male in dieselbe
Richtung.** Der Datensatz zu B14 schlug eine Probe vor, die blind gewesen waere; der Bau hat es beim
Gegenproben gemerkt und die Probe repariert statt sie abzuschreiben. Der Datensatz zu B15 nannte
drei Zitate; der Bau hat fuenf gerichtet, weil zwei aeltere dieselbe Form trugen. In beiden Faellen
ist die Abweichung vom Vorschlag im Datensatz vermerkt und nicht bloss gemacht worden. Das ist die
Gegenbewegung zu dem, was die drei vorigen Durchsichten gefunden haben: dort ruckte der Beleg jedes
Mal einen Schritt von der Stelle weg, an der die Zusage gilt.

**Der eine Befund ist derselbe Riss wie B10, an der Kante des Zeichensatzes.** Die
Fallunterscheidung ist wieder disjunkt zu einem Modell und nicht zur Platte — diesmal nicht, weil
gar nicht gefaltet wird, sondern weil die Faltung an der ASCII-Grenze endet, die APFS nicht kennt.
Der Unterschied zur dritten Durchsicht ist, dass die Grenze diesmal im Rumpf steht und nicht in der
Prosa: `eq_ignore_ascii_case` sagt beim Lesen, was es tut, und der Doc-Kommentar darueber sagt es
nicht.

**Was diese Durchsicht ausdruecklich nicht als Befund fuehrt**, weil die Beauftragung es ausnimmt:
dass die drei Befehle nur ueber die Maus erreichbar sind; dass der Abnahmelauf am gebauten Buendel
aussteht; der offene Defekt `260825-0838` zum Zeitstempel 1980.

## Reihenfolge

1. **Der eine Befund** ist kein Riegel vor dem Rundenabschluss. Weg 2 seines Datensatzes — zwei
   Saetze im Doc-Kommentar — kostet nichts und macht die Lage lesbar; Weg 1 gehoert in die naechste
   Runde oder an den naechsten Zug an dieser Datei.
2. Sonst nichts. Die uebrigen fuenf Befunde der dritten Durchsicht und die Codehaelfte des aelteren
   Datensatzes sind erledigt, jeder am Code nachgerechnet.
