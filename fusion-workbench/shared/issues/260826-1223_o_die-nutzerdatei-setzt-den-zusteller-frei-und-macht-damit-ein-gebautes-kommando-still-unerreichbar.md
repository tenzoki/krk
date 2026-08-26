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
