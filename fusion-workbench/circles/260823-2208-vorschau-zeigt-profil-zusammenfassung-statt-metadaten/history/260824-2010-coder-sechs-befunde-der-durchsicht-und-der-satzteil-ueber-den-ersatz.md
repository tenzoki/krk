# Die sechs Code-Befunde der Durchsicht vom 260824-1700

**Datum:** 260824-2010
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Auftrag:** Dispatch, keine Planschritte — sieben Defektdatensätze aus `issues/`
**Baumstand vorher:** `8433935` auf HEAD, daneben die Arbeit des `ontocoder` an `resources/default-readers.toml` und die Werkbankdateien

---

## Auftrag

Sieben Defektdatensätze der Durchsicht räumen, alle Änderungen unterhalb `crates/`.
`resources/default-readers.toml` räumt parallel der `ontocoder`, Spec und Plan der
`analyst`; beides ist nicht angefasst worden.

## Der schwerste: die Naht des Deckels

`260824-1650_c_die-probe-zur-naht-des-deckels-…`

Die Probe `ein_abgeschnittenes_zeichen_am_ende_nimmt_der_datei_nicht_ihren_text`
(`crates/krk-core/src/leseprofil/bausteine.rs`) schnitt `"Überschrift"` bei
`len() - 1` und nahm damit das abschließende `t` weg statt des zweiten Bytes des `Ü`.
Der Schnitt lag hinter einem ganzen Zeichen, also lief die Probe durch den **ersten**
Zweig von `lesbarer_anfang` und nie durch den zweiten.

Die neue Fassung schneidet `"Titel Ü"` (acht Bytes) bei `&ganz[..7]`: das erste Byte
des `Ü` bleibt stehen, das zweite fehlt, `error_len()` ist `None`, `valid_up_to()` ist
sechs, und `lesbarer_anfang` liefert `Some("Titel ")`. Ohne den zweiten Zweig wäre die
Antwort `None`.

Daneben stehen jetzt vier weitere Zusicherungen:

| Eingabe | erwartet | Zweig |
|---|---|---|
| `"Titel Ü"` ganz | `Some("Titel Ü")` | erster |
| `&ganz[..6]` | `Some("Titel ")` | erster, Schnitt hinter ganzem Zeichen |
| `&ganz[..7]` | `Some("Titel ")` | **zweiter**, die Naht |
| `[b'a', 0xff, b'b']` | `None` | dritter |
| `[b'T', 0xc3, 0x9c, 0xff]` | `None` | dritter, ungültiges Byte am Ende |

Vorangestellt ist `assert!(std::str::from_utf8(naht).is_err())`. Sie ist die Stelle, an
der die Probe künftig umfällt, wenn jemand den Schnitt wieder auf ein ganzes Zeichen
zurückzieht: dann trüge schon der erste Zweig die Antwort, und der Fehlschlag benennt
genau das.

### Wie festgestellt ist, dass der Zweig jetzt läuft

Nicht durch Lesen, sondern durch zwei Läufe an ausgehöhltem Zweig. Der Rumpf
`std::str::from_utf8(&bytes[..fehler.valid_up_to()]).ok()` wurde vorübergehend durch
`None` ersetzt — inhaltlich genau der Umbau, den der Datensatz als „bliebe grün"
beschreibt.

- **Neue Fassung, ausgehöhlter Zweig:** `FAILED`, an
  `bausteine.rs:567`, `left: None, right: Some("Titel ")`.
- **Alte Fassung, derselbe ausgehöhlte Zweig:** `ok`. Sie hätte den Umbau also
  durchgelassen.

Danach ist der Zweig aus der Sicherungskopie zurückgestellt und die Probe grün gelaufen.
Der Fehlschlag der ersten Zeile ist der Beleg: eine Probe, die an der Aushöhlung eines
Zweiges umfällt, hat ihn ausgeführt.

## Die Entwurfsfrage: wer trägt den Satzteil über den Ersatz

`260824-1508_c_die-meldung-einer-ersetzung-verspricht-…`

Der Datensatz stellt die Frage, ob `Grund`, `Datei` oder der Formatierer die Auskunft
trägt, was an die Stelle einer beschädigten Ablagedatei tritt.

**Entschieden: `Datei`.** Die Begründung steht am neuen `enum Ersatz` in
`crates/krk-core/src/ablage/pfade.rs` und ist zweiteilig.

`Grund` kann sie nicht tragen: derselbe Grund trifft jede der sieben Dateien, und
beschädigt ist beschädigt, gleich ob danach etwas einspringt. Ein `Grund`, der die
Auskunft trüge, müsste sie an jedem seiner vier Werte doppelt führen.

`Datei` trägt sie, weil sie dort schon zwei Fragen derselben Bauart trägt:
`Datei::format` und `Datei::leerbefund` sind beide vollständige Fallunterscheidungen
ohne Auffangzweig, je Datei beantwortet, und beide halten den Bau an, wenn eine achte
Ablagedatei dazukommt. `Datei::ersatz` ist die dritte und steht neben ihnen.

```
Datei ──format()───────> Format      (Toml | Text)
      ──leerbefund()───> Leerbefund  (Vorgabe | Beschaedigt)
      ──ersatz()───────> Ersatz      (Auslieferungszustand | Nichts)   <- neu
```

`Ersetzung` bekommt dafür das Feld `welche: Datei` neben dem bestehenden `datei: PathBuf`.
Beide werden gebraucht: der Pfad ist nicht aus der Angabe ableitbar, weil
`tasten::belegung::fuer_den_betrieb` eine Meldung baut, wenn es gar keinen Ablageordner
gibt, und dort den nackten Dateinamen einträgt. Neun Bauplätze sind nachgezogen, jeder
hatte den passenden `Datei`-Wert bereits in der Zeile darüber stehen.

Der Wortlaut für `readers.toml` ist **„und nichts tritt an ihre Stelle"** und nicht
„kein Profil". Der Kern kennt Pfad, Format und Fehlerbehandlung und nicht den Inhalt;
„Profil" wäre Inhaltswissen an der falschen Stelle.

**Die Frage ist nicht größer als ein Defekt.** Sie sah so aus, weil `Ersetzung` ein
öffentliches Feld dazubekommt, aber der Umbau endet an den neun Bauplätzen und einer
Testhilfe. Kein Ladeweg, kein Meldeweg und keine Struktur der Oberfläche ändert sich.

### Was daran belegt ist

- `die_meldung_zu_readers_toml_verspricht_keinen_auslieferungszustand`
  (`crates/krk-core/tests/ablage.rs`): alle fünf `Beiseite`-Lagen, je mit der Gegenprobe
  an `settings.toml`. Sie hält fest, dass der Unterschied allein an `Datei::ersatz` hängt.
- `genau_readers_toml_bekommt_keinen_ersatz`: läuft über `Datei::ALLE` und kann keine
  Datei vergessen. Eine achte mit `Ersatz::Nichts` lässt sie rot werden.
- `pruefe_meldung`, die Hilfe hinter sechs Proben, prüft den Satzteil jetzt je Datei und
  schreibt die zwei erwarteten Wortlaute aus, statt sie aus `Ersatz` zu holen.

Zwei bestehende Proben — `eine_kaputte_datei_fuehrt_zum_auslieferungszustand_und_zu_einer_meldung`
und `eine_nicht_lesbare_readers_toml_ergibt_kein_profil_und_eine_meldung` — sind beim
ersten Lauf rot geworden. Das ist die Bestätigung des Befundes: sie prüften über
`pruefe_meldung` genau das falsche Versprechen an genau der Datei, für die es falsch ist.

### Drei Prosastellen mitgezogen

Sie hätten sonst die alte Aussage weitergetragen:

- der Doc-Kommentar an `Grund` (hieß „Warum eine Datei durch den Auslieferungszustand
  ersetzt wurde"),
- der an `Ersetzung` („Eine Datei wurde durch den Auslieferungszustand ersetzt"),
- der Modulkopf von `ablage/leseprofile.rs`, der die Trennung von `Ersetzung` und den
  Prüfmeldungen mit demselben Satz begründete.

## Die vier kleinen Befunde

| Datensatz | Änderung |
|---|---|
| `260824-1652` | Vierter Fall nachgetragen statt Zahl gesenkt: ein unbekannter Schlüssel auf der obersten Ebene, der an `Profildatei`s `deny_unknown_fields` fällt. Die Schleife nimmt vollständige Texte, weil der vierte Fall vor dem ersten Block stehen muss. Die zweiundzwanzig Leerzeichen im `vorspann` sind weg. |
| `260824-1653` | Zwei Stellen in `vorschaumodell.rs` sagen „im ausgelieferten Programm" statt „in diesem Baum"; der Doc-Kommentar an `laden` schreibt aus, warum. |
| `260824-1654` | Die Zahl ist aus dem Satz heraus, nicht berichtigt: „mehrere Vorhandensein-Zeilen", mit dem Hinweis, dass die Zahl in der Auslieferungsfassung steht. Die tragende Hälfte von A7 bleibt. |
| `260824-1655` | „vier der fünf" mit ausgeschriebener Aufzählung, `session.toml` als benannte Ausnahme. Die Nachbarzeile ist auf Anweisung des Datensatzes mitgezogen. |
| `260824-1656` | `_mit_ladeweg_` gestrichen, beide Verweise tragen jetzt denselben Namen. |

## Was offen bleibt

`shared/issues/260821-1023_o_sieben-prosastellen-der-ablage-…` führt in seiner Tabelle
die Zeile `:154`/`:158` von `ablage/mod.rs`. Sie ist mit dem Befund `260824-1655` in der
Sache erledigt — der Satz sagt jetzt „Die vier uebrigen TOML-Dateien und die zwei
Zettel" —, aber jener Datensatz liegt im gemeinsamen Speicher, war nicht Teil dieses
Auftrags und ist nicht angefasst worden. Sein Eintrag ist nachzuziehen.

Kein `#[must_use]` an `Datei::ersatz` und `Ersatz::satzteil`: `Datei::format` und
`Datei::leerbefund` daneben tragen keins, `pfade.rs` führt die Angabe nirgends, und ein
fallengelassener Aufruf einer wirkungsfreien `const fn` bleibt nicht unbemerkt, sondern
ist folgenlos. Die Angabe nur an der dritten der drei zu setzen, hätte die drei
auseinanderlaufen lassen.

## Abnahme

```
make check   ->  exit 0   (alle vier grün)
```

Vorher zwei gezielte Läufe für den Zweigbeleg, beide oben ausgeschrieben.
