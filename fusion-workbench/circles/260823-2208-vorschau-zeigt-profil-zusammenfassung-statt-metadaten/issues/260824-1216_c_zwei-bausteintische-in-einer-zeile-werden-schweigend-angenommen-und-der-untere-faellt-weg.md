Zwei Bausteintische in einer Zeile werden schweigend angenommen, und der untere fällt weg

---

C3 sagt: „Eine Profilzeile besteht aus einer Beschriftung und **genau einem** Baustein." Schreibt
der Nutzer zwei Tische in eine `[[profil.zeile]]`, nimmt `serde` sie an, wählt den in der
Aufzählung oberen und lässt den zweiten fallen. `pruefen` sieht davon nichts, es entsteht keine
Meldung, und der Nutzer sucht die Zeile, die er geschrieben hat und die nie erscheint.

---

**Gemessen am 260824-1216 an diesem Baum**, Stand `abe1a31`, in einer Wegwerfprobe unter
`crates/krk-core/tests/`, die danach wieder entfernt wurde.

## Der Lauf

```toml
  [[profil.zeile]]
  beschriftung = "Beides"
  zaehlung = { }
  vorhandensein = { muster = 'y' }
```

```text
ANGENOMMEN: profile=1  meldungen=[]
   zeile "Beides" -> Some(Zaehlung { ort: [], muster: None })
```

Der Grund steht in `crates/krk-core/src/leseprofil/datei.rs:121-144`: `Bausteindatei` ist eine
unmarkierte Auswahl (`#[serde(untagged)]`), und eine Strukturvariante darin lässt unbekannte
Felder liegen. `deny_unknown_fields` an den vier Tischen greift **innerhalb** eines Tisches und
nicht neben ihm.

## Die Umsetzung weiß davon und nennt es keinen Fehler

Der Doc-Kommentar an `Bausteindatei` sagt es ausdrücklich: „stuenden in einer Zeile zwei Tische,
gewaenne der obere" (`datei.rs:119-120`). Das ist eine zutreffende Beschreibung und keine
Begründung: sie sagt, was geschieht, und nicht, warum ein schweigender Verlust hier richtig
wäre. Kein Abnahmekriterium deckt den Fall, und keine Probe hält ihn.

## Wo der Widerspruch liegt

Der Plan hat den Fall gesehen und für den **Ausweichweg** vorgesehen, nicht für den gebauten:
im Risikoabschnitt steht, falls die Rundreise über `flatten` und `untagged` fiele, bekäme die
Zeile „ein Feld `baustein = "zaehlung"` als ausgeschriebene Sortenkennung und einen von Hand
geschriebenen Prüfschritt, **der genau eine Bausteinangabe fordert**". Die Rundreise ist
gelaufen und grün, also ist der Ausweichweg nicht gezogen worden — und mit ihm ist die Prüfung
auf „genau eine" entfallen, die im Ausweichweg noch ausdrücklich dastand.

## Was zu tun ist

Der Prüfschritt kann die Frage nicht beantworten: bei ihm kommt schon die gewählte Variante an,
der zweite Tisch ist zu diesem Zeitpunkt verloren. Wer sie beantworten will, braucht die
Zwischenstufe, also entweder

1. eine `Zeilendatei`, deren Bausteinhälfte als `toml::Table` oder als vier `Option<…>`-Felder
   hereinkommt und deren Prüfschritt zählt, wie viele davon dastehen, oder
2. die ausgeschriebene Sortenkennung aus dem Risikoabschnitt des Plans.

Beides kostet mehr als der heutige Stand. **Die dritte Möglichkeit ist, den Fall bewusst
hinzunehmen** und ihn dort auszuschreiben, wo der Nutzer ihn liest: in den Kommentarzeilen von
`resources/default-readers.toml`, die nach C5.10 ohnehin alle vier Bausteine erklären. Dann ist
es eine Eigenschaft der Datei und kein stiller Verlust.

**Schwere:** niedrig bis mittel. Fehlbedienung des Nutzers, nicht Fehlverhalten des Codes, aber
sie geht ohne jede Meldung ab, und das Projekt hält es sonst anders.

**Gefunden:** coderev, bei der Durchsicht von Bündel B am 260824-1216.

**Betroffen:** `crates/krk-core/src/leseprofil/datei.rs` (`Bausteindatei`, `Zeilendatei`,
`pruefen`), `resources/default-readers.toml` (steht noch aus, Schritt 7)

**Domain:** code

---
Resolved: Weg 1 des Datensatzes, und er hat den Befund `260824-1217` mit erledigt — eine
Änderung für beide, weil beide dieselbe Wurzel hatten: die unmarkierte Auswahl hinter
`#[serde(flatten)]`.

`Zeilendatei` trägt jetzt vier benannte Felder `zaehlung`, `juengste`, `feld` und
`vorhandensein`, je `Option<…>`, und `#[serde(deny_unknown_fields)]` dazu.
`Zeilendatei::zerlegen` zählt, wie viele davon dastehen, und beantwortet die Frage aus C3 selbst:
genau einer ergibt den Baustein, keiner und zwei sind je ein Grund mit Meldung. `Bausteindatei`
bleibt als Aufzählung stehen, trägt aber kein `Deserialize` mehr; sie entsteht in `zerlegen` und
nicht beim Lesen, und `baustein_pruefen` behält seine vollständige Fallunterscheidung.

**Die Gestalt der Datei ist unverändert.** Der Nutzer schreibt weiter `zaehlung = { … }` neben
seine Beschriftung, ohne Sortenkennung; die dritte Möglichkeit des Datensatzes, den Fall
hinzunehmen und in `default-readers.toml` auszuschreiben, war damit nicht nötig. Weg 2, die
ausgeschriebene Sortenkennung aus dem Risikoabschnitt des Plans, kostet eine Zeile je Profilzeile
in der Datei und ist nicht gezogen.

**Drei Dinge fallen mit ab**, die vorher keine Meldung hatten: ein zusätzlicher Schlüssel neben
der Beschriftung, eine Zeile ganz ohne Bausteintisch, und der Vorbehalt über die Verbindung aus
`flatten` und `untagged` — beide Sonderwege sind weg, die Rundreise nimmt jetzt vier gewöhnliche
`Option`-Felder ab.

Belegt von `eine_zeile_mit_zwei_bausteinen_oder_ohne_einen_verliert_ihren_baustein`
(`crates/krk-core/tests/leseprofil.rs`): die Zeile behält ihre Beschriftung, verliert ihren
Baustein, und die Meldung nennt Profilnamen, Beschriftung und bei zwei Tischen deren Namen.
Modulkopf von `leseprofil::datei` und der Abschnitt des Elternmoduls sind nachgezogen.
