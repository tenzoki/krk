# Schritt 11: die Anwendung lädt die Profile im selben Durchgang und übergibt sie

**Datum:** 260824-1755
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Plan:** `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel D, Schritt 11
**Baumstand vorher:** Schritt 8 (`4516f4e`) und Schritt 10 (`a77bb77`) eingegangen, Arbeitsbaum mit den nicht eingegangenen Änderungen aus Schritt 8 und der Auslieferungsfassung (`8433935`)

---

## Auftrag

Die geprüften Leseprofile im selben Ablagedurchgang lesen, in dem heute Sitzung und
Einstellungen gelesen werden, sie in einem Merkfeld des Anwendungsdelegierten halten und
einmal an das Vorschaufenster übergeben. Dazu die zwei Hinterlassenschaften aus
Schritt 10 abräumen.

## Was entstanden ist

### `crates/krk-ui/src/appkit/anwendung.rs`

| Stelle | Zeile | Was |
|---|---|---|
| Importe | 218–223 | `leseprofile` in die `krk_core::ablage`-Liste, dazu `krk_core::leseprofil::Profile` |
| `AnwendungsIvars::profile` | 545 | `RefCell<Arc<Profile>>`, mit dem Grund am Feld: einmal beim Start geladen (C4.5), ein `Arc` statt einer Kopie, weil der Satz bis auf den Arbeitsfaden des Vorschaumodells mitreist |
| `Anwendungsdelegierter::neu` | 1074 | `profile: RefCell::new(Arc::default())`, also der leere Satz bis `sitzung_laden` gelaufen ist |
| `sitzung_laden`, Doc-Kommentar | 1449–1456 | der Satz über den Messmodus: alle vier Aufgaben kehren vor dem Durchgang zurück, also bleibt das Feld leer und die Vorschau bekommt einen leeren Profilsatz |
| `sitzung_laden`, der eine Durchgang | 1544–1580 | `leseprofile::laden(zugang)` als dritte Lesung unter derselben Schreibsperre; die `Ersetzung` (C1.6, C1.7) und die Liste der Prüfmeldungen (C2.7, C3.10) gehen beide in dasselbe `meldungen` |
| `oberflaeche_aufbauen` | 1215–1222 | `vorschau.profile_setzen(Arc::clone(&ivars.profile.borrow()))`, unmittelbar vor `ivars.vorschau.set(vorschau)` |
| `leseprofilproben::die_leseprofile_werden_im_baum_genau_einmal_geladen` | 8718 | die Zählprobe über `crate::quellbaum` |

**Ein zweiter Durchgang ist nicht entstanden.** Der Kommentar, der oben an jener Stelle
schon stand, gilt unverändert: `Ablage::oeffnen` legt den Ordner an, und zweimal
anzulegen hieße, dieselbe Frage zweimal an das Dateisystem zu stellen. Die Zeile heißt
jetzt „ein Durchgang für alle drei Dateien" statt „für beide".

**Zwei Sorten Meldung, eine Statuszeile.** `leseprofile::laden` gibt ein Paar zurück, und
beide Hälften sind Meldungsquellen mit verschiedenem Gegenstand: die `Ersetzung` sagt,
dass die Datei beiseitegelegt oder nicht anlegbar war, die Liste daneben nennt jedes
abgewiesene Profil und jede Zeile ohne Baustein. Sie stehen deshalb als zwei
`meldungen.extend(…)` nebeneinander und werden nicht vorher zusammengelegt; welche Tür
welche nimmt, steht im Modulkopf von `krk_core::ablage::leseprofile`.

### Die Zählprobe

`die_leseprofile_werden_im_baum_genau_einmal_geladen` zählt die Aufrufform
`leseprofile::laden(` über `quelldateien()` und verlangt das Ergebnis
`[("krk-ui/src/appkit/anwendung.rs", 1)]`. Die Nadel steht wie bei der Probe aus
Schritt 10 als `concat!("leseprofile::", "laden")` da, weil die Probe in der Datei liegt,
die sie zählt, und sich sonst selbst fände.

Was sie nicht sieht, steht in ihrem Doc-Kommentar: die Weitergabe der Funktion als Wert,
wie `krk-core/tests/ablage.rs` sie mit `.durchgang(leseprofile::laden)` schreibt, trägt
keine Klammer und wird nicht gezählt; dazu die Grenze aus dem Kopf von
`crate::quellbaum`, ein Aufruf unter anderem Namen über `use … as anders;`.

Sie ist die Gegenprobe zu der aus Schritt 10: jene hält fest, dass die Profile genau
einmal an die Vorschau übergeben werden, sieht aber nicht, ob überhaupt welche gelesen
wurden. Zusammen schließen die beiden die Kette von `readers.toml` bis zum Merkfeld der
Vorschau.

### `crates/krk-ui/src/appkit/vorschau.rs` — die zwei Hinterlassenschaften

Beide sind abgeräumt, und mehr ist an der Datei nicht geschehen.

1. **Die `dead_code`-Ausnahme ist heraus** (vormals Zeile 914). Ihr Kommentar nannte
   Schritt 11 als Ablaufdatum; der Rufer steht jetzt, und `make check` fährt clippy mit
   `-D warnings` weiter grün.
2. **Die untere Schranke der Zählprobe ist nachgezogen.** Aus `rufstellen.len() <= 1`
   samt der Schleife darüber ist ein `assert_eq!` auf
   `vec![("krk-ui/src/appkit/anwendung.rs", 1)]` geworden. Der Doc-Kommentar spricht
   nicht mehr von einer fehlenden Hälfte, sondern schreibt beide als „genau einmal" aus
   und hält fest, dass die zweite bis Schritt 11 eine obere Schranke war. Der Abschnitt
   „Was diese Probe nicht sieht" nennt jetzt die richtige verbleibende Blindheit: ein
   Rufer, der einen leeren Satz übergibt, besteht sie mühelos.

**Die Probe hat dabei ihren Namen gewechselt**, von
`die_profile_haben_einen_schreiber_und_hoechstens_einen_rufer` auf
`die_profile_haben_genau_einen_schreiber_und_einen_rufer`. Ein Name, der „höchstens"
sagt, während die Zusicherung darunter Gleichheit verlangt, wäre die Art von falscher
Auskunft, die dieser Baum an anderen Stellen ausdrücklich vermeidet. Der Verweis auf den
Namen im Doc-Kommentar von `profile_setzen` ist mitgezogen.

## Abnahme

```
make check   → exit 0
```

Beide Zählproben laufen grün:

- `appkit::anwendung::leseprofilproben::die_leseprofile_werden_im_baum_genau_einmal_geladen`
- `appkit::vorschau::tests::die_profile_haben_genau_einen_schreiber_und_einen_rufer`

Nicht geprüft und nicht prüfbar an diesem Platz: die sichtbaren Hälften von C4.1 bis
C4.5. Sie verlangen KRK im Vordergrund und stehen im Plan unter `## Nutzerarbeit`.

## Was offen bleibt

Schritt 12 (die Zählproben zu C6) hängt an Schritt 11 und ist damit frei. Der Marker des
Plans steht auf Schritt 11 `[DONE]`; die Statuszeile des Plans ist nachgezogen.
Committet wurde nichts.
