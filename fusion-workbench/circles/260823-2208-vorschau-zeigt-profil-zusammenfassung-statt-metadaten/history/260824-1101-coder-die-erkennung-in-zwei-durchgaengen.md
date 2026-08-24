# Schritt 5: Die Erkennung, in zwei Durchgängen und ohne dritten

**Datum:** 260824-1101
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel B, Schritt 5
**Spec:** `…/planning/260824-0613_o_spec-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, C2

---

## Was gebaut ist

Das neue Modul `crates/krk-core/src/leseprofil/erkennung.rs` trägt eine einzige
öffentliche Funktion:

```rust
pub fn erkennen<'p, 'e>(
    profile: &'p Profile,
    voller_pfad: &Path,
    eintraege: &dyn Fn() -> Option<&'e [Eintrag]>,
) -> Option<&'p Profil>
```

Sie fährt die Regel aus C2 in ihrer ausgeschriebenen Form: erst alle Profile in
Dateireihenfolge gegen ihr Pfadmuster auf dem vollen Pfad, danach alle Profile
in derselben Reihenfolge gegen ihre Kennzeichendatei auf den Namen der
Einträge. Das erste Profil mit Treffer gewinnt im jeweiligen Durchgang; greift
keines, ist die Antwort `None`, und die Vorschau bleibt bei der heutigen
Metadatenanzeige.

Der Modulkopf schreibt die drei Dinge aus, die der Schritt verlangt: dass die
Reihenfolge der zwei Durchgänge und die Reihenfolge in der Datei zwei
verschiedene Regeln sind (Festlegung A1), dass C2.3 aus der Trennung folgt statt
danebenzustehen, und warum die Einträge als Abschluss hereinkommen und nicht als
Feld.

## Die Zahl der Dateiabfragen

`erkennen` fragt das Dateisystem **nicht selbst**. Der erste Durchgang sieht auf
einen Pfad, den der Aufrufer ohnehin hält, und kostet nichts. Der zweite ruft
den hereingereichten Abschluss **höchstens einmal**: beim ersten Profil mit
Kennzeichendatei, und die Antwort wird für die übrigen Profile desselben
Durchgangs gemerkt. Nennt kein Profil eine Kennzeichendatei, wird der Abschluss
gar nicht gerufen. Damit kostet die Erkennung eines Ordners null oder einen
Verzeichnisleselauf, und die Buchung gegen den `Haushalt` liegt beim Aufrufer,
der den Abschluss stellt — das ist Schritt 6.

Zwei der sieben Proben zählen die Rufe des Abschlusses mit und sind damit die
Belegstelle für diese Aussage.

## Die Proben

Sieben, alle in `crates/krk-core/tests/leseprofil.rs`, alle ohne Fenster und
ohne Dateisystem. Die Profile entstehen aus TOML-Text über den vorhandenen
Helfer `gepruefte`, die Einträge über den neuen Helfer `bestand`, der einen
`Eintrag` je Name baut.

| Probe | Kriterium |
|---|---|
| `ein_pfadmuster_trifft_seinen_ordner_und_den_daneben_nicht` | C2.1 |
| `von_zwei_passenden_pfadmustern_gewinnt_das_obere` | C2.2, beide Reihenfolgen |
| `ein_spaeteres_pfadmuster_schlaegt_ein_frueheres_kennzeichen` | C2.3 |
| `das_kennzeichen_eines_circles_trifft_bei_jedem_der_sechs_marker` | C2.4 |
| `der_erste_durchgang_ruft_den_abschluss_nicht` | die Bauart hinter C6.7 |
| `ein_profil_mit_beidem_nimmt_an_beiden_durchgaengen_teil` | C2, beide Hälften eines Profils |
| `ohne_eintraege_trifft_keine_kennzeichendatei` | unentschieden statt negativ |

C2.4 steht im Spec als Aussage über die 18 Circle-Verzeichnisse **dieser**
Werkbank; der Schritt übersetzt sie in die sechs Zustandsmarker des Vokabulars,
und die Probe geht diesen Weg. Das folgt der Regel aus `## Testing Strategy`,
dass die Zahlen dieser Werkbank in keiner Probe stehen.

## Zwei Beobachtungen, keine davon ein Defekt

**Die Signatur im Plan trägt eine unbenannte Lebenszeit und übersetzt so nicht.**
`&dyn Fn() -> Option<&[Eintrag]>` hat kein Eingabeargument, aus dem die
Lebenszeit des Ausschnitts abzuleiten wäre; sie ist deshalb als `'e` benannt und
neben `'p` gestellt. Das ist eine Schreibweise und keine Abweichung von der
Bauart, die der Schritt beschreibt: die Einträge kommen weiter als Abschluss
herein und der Rückgabewert bleibt ein Verweis in die Profilliste.

**Der Marker des Datensatzes `260824-0600_a_welche-form-hat-das-pfadmuster-und-
welche-die-kennzeichendatei.md` bleibt auf `_a_`.** Der Plan realisiert ihn in
den Schritten 3 und 5 und legt unter `## Welcher Schritt welchen Datensatz
realisiert` ausdrücklich fest, dass die acht Marker beim Rundenabschluss
wandern, nach dem Abgleich gegen den Baum, und nicht in einem einzelnen Schritt.
Dazu kommt, dass dieser Schritt keinen Commit erzeugt: den setzt der
Orchestrator, und die Zeile `Implemented:` verlangt den Hash.

## Prüfung

`make check` fährt die vier Abnahmekommandos in einem Zug und läuft grün. Ein
erster Lauf ist an `cargo fmt --all --check` gescheitert (die Umbrüche in drei
neu geschriebenen Ausdrücken); `cargo fmt --all` hat sie gesetzt, der zweite
Lauf ist durchgelaufen.

```
make check — exit 0
```

## Berührte Dateien

- `crates/krk-core/src/leseprofil/erkennung.rs` (neu)
- `crates/krk-core/src/leseprofil/mod.rs` (`pub mod erkennung;`, ein Satz im Modulkopf, das Ablaufbild nennt jetzt das Modul)
- `crates/krk-core/tests/leseprofil.rs` (sieben Proben, zwei Helfer, Modulkopf)
- `…/planning/260824-0640_o_plan-…md` (Schritt 5 auf `[DONE]`, `Files:` ergänzt)
