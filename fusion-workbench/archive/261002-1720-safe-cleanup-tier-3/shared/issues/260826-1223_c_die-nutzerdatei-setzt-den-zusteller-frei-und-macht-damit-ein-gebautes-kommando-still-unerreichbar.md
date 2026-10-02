Die Nutzerdatei setzt den Zusteller frei und macht damit ein gebautes Kommando still unerreichbar

---

`Belegung::bauen` prüft von einem Eintrag der `keymap.toml` allein die **Kennung** gegen den Wortschatz und übernimmt `name`, `reserviert_fuer` und `gehalten_von` unverändert aus der Nutzerdatei. `gehalten_von` ist die tragende Hälfte der Zustellerregel: ein von Hand gesetztes `gehalten_von = "menue"` an einem gebauten Befehl nimmt ihn aus dem Ereignisabgriff, gibt ihm kein `Kommando` mehr, erzeugt keinen Konflikt, keine Meldung und keinen Rückfall — der Befehl steht in der Belegungsansicht und tut nichts.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Am Baum

`crates/krk-core/src/tasten/belegung.rs:1419-1451`. Die einzige Prüfung gegen den Wortschatz steht in 1420-1424 und fragt `wortschatz.funktion(&eintrag.id)`. Danach:

```
funktionen.push(Funktion {
    kennung: eintrag.id.clone(),
    name: eintrag.name.clone(),
    tasten,
    reserviert_fuer: eintrag.reserviert_fuer.clone(),
    gehalten_von: eintrag.gehalten_von.clone(),
});
```

Drei der fünf Felder kommen ungeprüft aus der Nutzerdatei. Der Modulkopf (`belegung.rs:30-34`) beschreibt die Prüfung als „sie darf jede Kombination frei verteilen, aber nur auf Funktionen, die KRK kennt" — das nennt die Kennung und schweigt zu den drei anderen Feldern.

## Warum `gehalten_von` das schwere der drei ist

Der Modulkopf schreibt die Zustellerregel über 29 Zeilen aus (`belegung.rs:78-106`) und nennt vier Stellen, an denen sie greift, „und keine davon ist entbehrlich". Zwei davon lesen das Feld unmittelbar:

- `Belegung::nachschlag` (`belegung.rs:1290-1293`) überspringt jede Funktion mit gesetztem `gehalten_von`. Der Ereignisabgriff findet den Befehl damit nicht mehr.
- `Funktion::kommando` (`belegung.rs:1169-1174`) liefert `None`, sobald das Feld gesetzt ist. Auch der Weg über das Hauptmenü liefert kein Kommando, denn der Menüeintrag entsteht aus der festen Menüstruktur und nicht aus diesem Feld.

`Belegung::konflikte` (`belegung.rs:1379-1397`) vergleicht nur **innerhalb** desselben Zustellers und sieht deshalb nichts. `laden` (`belegung.rs:1493-1515`) fällt nur bei einem `Belegungsfehler` auf die Auslieferung zurück, und es entsteht keiner. Die Datei wird angenommen.

## Der Weg ist gemessen, die Folge ist nicht erfasst

Der Mechanismus ist im Baum schon einmal gemessen worden, am 260811-0955, und steht als Kommentar in `crates/krk-ui/src/belegungsausgabe.rs:326-339`:

> eine von Hand geschriebene `keymap.toml` gibt einer Kennung **mit** Kommando einen Zusteller, etwa `kopieren` ein `gehalten_von = "menue"`. `Belegung::vom_nutzer` nimmt sie an […] `kommando()=None gehalten_von=Some("menue") aus_kennung=Some(Kopieren)`

Dort ist daraus die richtige Folgerung für **jene** Stelle gezogen worden: der Auffangzweig ist erreichbar und darf kein `panic!` sein. Die Folge für den Nutzer — `kopieren` ist danach still tot — steht an keiner Stelle als Defekt, als Entscheidung oder als Meldung. Der Kommentar hält den Absturz auf, nicht die Wirkung.

## Wie schwer

`keymap.toml` ist ausdrücklich von Hand änderbar (`belegung.rs:1491`), und die Belegungsansicht schreibt sie mit `gehalten_von` zurück (`belegung.rs:1654-1662`), also sieht der Nutzer das Feld in seiner Datei stehen. Ein Vertipper daran ist damit nicht abwegig. Kein Weg über die Oberfläche setzt es; erreichbar ist der Fall allein von Hand.

## Vorschlag

Der billigste Schnitt hält die drei Felder aus dem Wortschatz statt aus der Nutzerdatei: `bauen` hat die Wortschatz-Funktion an Ort und Stelle (`belegung.rs:1420-1421`) und kann `name`, `reserviert_fuer` und `gehalten_von` von dort nehmen. Aus der Nutzerdatei bliebe dann `id` und `tasten` — genau das, was der Modulkopf als ihren Gegenstand nennt. Kostet nichts an Rückwärtsverträglichkeit, weil die geschriebene Datei dieselben Werte trägt, und macht `Eintrag`s drei Felder beim Lesen zu einer bloßen Duldung.

Verwandt und **nicht** dasselbe: `shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md` behandelt den **fehlenden** Eintrag, dieser Befund den vorhandenen mit abweichendem Zusteller.

Gefunden bei der Vollbaum-Durchsicht R4 an HEAD `004ff72`.

Also seen: 260826-1442 by coderev — die drei Oberflächen zeigen den freigesetzten Befehl verschieden: Belegungsansicht „(Kürzel des Menüs)“ (`belegungsmodell.rs:530-536`, sachlich falsch, das Menü stellt ihn nicht zu), Markdown „(von KRK nicht eingeordnet)“ (`belegungsausgabe.rs:237,357`), Hauptmenü grau ohne Kommando (`menuemodell.rs:295-300`); keine sagt, dass der Befehl unerreichbar ist.

---

## Abgleich 260908, und die Behebung

**Der Befund bestand unveraendert**, und eine Probe im Baum hat den Weg als richtiges
Verhalten gemessen: `der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege`
(`crates/krk-core/tests/belegung.rs`) gab `fenster_schliessen` in ihrer Nutzerdatei ein
`gehalten_von = "menue"` und hielt anschliessend fest, dass `kommando()` `None` liefert und
der Nachschlag `Unbelegt` sagt — genau die Wirkung, die dieser Datensatz als Defekt fuehrt.
Die Auslieferungsbelegung gibt `fenster_schliessen` ausdruecklich **keinen** Zusteller
(`resources/default-keymap.toml`, „Sie traegt deshalb kein `gehalten_von`").

**Gebaut ist der Schnitt des Datensatzes, aber enger als er ihn vorschlaegt: allein
`gehalten_von`.** `Belegung::bauen` holt die Wortschatz-Funktion einmal an Ort und Stelle und
nimmt den Zusteller von dort; `name` und `reserviert_fuer` kommen weiter aus der Nutzerdatei.

**Die Einengung ist am Baum begruendet und nicht Vorsicht.** Der Datensatz nennt alle drei
Felder als „billigsten Schnitt", weil `bauen` die Wortschatz-Funktion ohnehin zur Hand hat;
gemessen an der Wirkung sind sie aber nicht dieselbe Sache. Keines von beiden entscheidet, ob
ein Befehl ankommt: `name` ist die Beschriftung, `reserviert_fuer` haengt allein einen Zusatz
an den Text der Belegungsansicht (`krk-ui/src/belegungsmodell.rs`, `funktionstext`) — und
dessen Doc-Kommentar rechnet **ausdruecklich** mit einer `keymap.toml` aus einer aelteren
Fassung, die das Feld noch traegt. Beide aus dem Wortschatz zu nehmen naehme dem Nutzer eine
Umbenennung, die niemandem schadet, und verwuerfe still den Vorbehalt einer alten Datei. Der
erste Anlauf hat das getan und dabei zwei Proben rot gemacht, darunter die zur Maskierung des
senkrechten Strichs, die einen Namen aus der Nutzerdatei braucht, um ueberhaupt einen Strich
in die Tabelle zu bekommen.

**Zwei Proben halten es.** Neu ist
`die_nutzerdatei_setzt_weder_zusteller_noch_name_noch_vorbehalt`: eine Datei, die `kopieren`
einen fremden Namen, ein `reserviert_fuer` und ein `gehalten_von = "menue"` gibt, kommt mit
einem Kommando heraus, behaelt aber ihren Namen, ihren Vorbehalt und **ihre** Taste `ctrl+c`
— die letzte Zusicherung ist noetig, sonst waere mit dieser Aenderung die Zusage gefallen,
dass die Datei jede Kombination frei verteilen darf. Die vorhandene Reihenfolgeprobe misst die
vierte Stelle der Zustellerregel jetzt an `text_alles_auswaehlen`, das `gehalten_von = "menue"`
ab Werk traegt, und haelt daneben fest, dass `fenster_schliessen` trotz des Eintrags in der
Datei erreichbar bleibt.

**Eine Folge, die ueber die Kiste hinausgeht und mitgebaut ist.** Der Auffangzweig von
`belegungsausgabe::wirkung` (`krk-ui`) war ueber genau diesen Weg erreichbar und ist es nicht
mehr: kein Eintrag der Auslieferungsbelegung traegt einen Zusteller **und** eine Kennung aus
`Kommando::KENNUNGEN`. Der Zweig bleibt stehen — der `match` laeuft ueber `&str` und braucht
ohnehin einen Auffangzweig —, und was ihn unerreichbar haelt, ist jetzt gemessen statt
verabredet: die Probe `keine_ausgelieferte_funktion_traegt_zusteller_und_kommando` ersetzt
`eine_kennung_mit_kommando_und_zusteller_landet_im_auffangzweig` und wird rot, sobald eine
spaetere Runde einem gebauten Befehl ein `gehalten_von` gibt.

**Der Modulkopf sagt es jetzt auch.** Der Satz „sie darf jede Kombination frei verteilen, aber
nur auf Funktionen, die KRK kennt" hat einen Absatz bekommen, der den Zusteller als Duldung
beim Lesen einordnet und daneben ausschreibt, warum `name` und `reserviert_fuer` bleiben, wo
sie sind.

**Was der Datensatz daneben festhaelt, bleibt offen:** dass die drei Oberflaechen einen
freigesetzten Befehl verschieden und keine ihn als unerreichbar zeigt
(`Also seen: 260826-1442`). Nach dieser Behebung ist der Fall ueber die Nutzerdatei nicht mehr
erreichbar; die drei Anzeigen gelten weiter fuer die Befehle, die ab Werk einen Zusteller
tragen, und die Frage, wie sie einen solchen Befehl benennen, ist eine eigene.

Resolved: 260908, `crates/krk-core/src/tasten/belegung.rs` — `bauen` nimmt `gehalten_von` aus
dem Wortschatz, `name` und `reserviert_fuer` bleiben Sache der Nutzerdatei; Modulkopf
nachgezogen; zwei Proben in `crates/krk-core/tests/belegung.rs` und eine in
`crates/krk-ui/src/belegungsausgabe.rs`.
