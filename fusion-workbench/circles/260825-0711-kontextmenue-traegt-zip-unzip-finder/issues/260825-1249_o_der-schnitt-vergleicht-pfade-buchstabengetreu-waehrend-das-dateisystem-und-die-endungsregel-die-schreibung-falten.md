Der Schnitt vergleicht Pfade buchstabengetreu, waehrend das Dateisystem und die Endungsregel die Schreibung falten

---

`ist_ziel_des_laufs` (`crates/krk-ui/src/kommandos/kontextmenue.rs:598`) haelt Pfad gegen Pfad mit
`==`, also byteweise. Das Bauziel ist macOS, dessen APFS-Vorgabe die Gross- und Kleinschreibung
**faltet**, und dieselbe Datei erkennt ein Archiv ausdruecklich ohne Ruecksicht auf die Schreibung.
Ein Eintrag `PROJEKTE.ZIP` und das gerechnete Ziel `Projekte.zip` sind damit derselbe Eintrag auf der
Platte und zwei verschiedene fuer den Schnitt: die Zusage faellt in genau dem Fall wieder aus, den
sie schliessen soll.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code

**Gemessen am Baumstand `ddd41ff` am 260825-1249, in der dritten Durchsicht der Runde 17
(`6faaa91..ddd41ff`).**

## Die Rechnung, Zeile fuer Zeile

1. `crates/krk-ui/src/kommandos/kontextmenue.rs:598-600` — der Vergleich:

   ```rust
   fn ist_ziel_des_laufs(pfad: &Path, ziele: &[PathBuf]) -> bool {
       ziele.iter().any(|ziel| ziel.as_path() == pfad)
   }
   ```

   `PartialEq` auf `Path` vergleicht Bestandteile buchstabengetreu. `a/b` und `a//b` sind damit
   gleich, `Projekte.zip` und `PROJEKTE.ZIP` nicht.

2. `crates/krk-ui/src/kommandos/kontextmenue.rs:416-426` — `archivname` haengt die Konstante
   `ENDUNG` an, und die ist **kleingeschrieben** (`const ENDUNG: &str = ".zip";`, Zeile 134). Das
   Ziel eines Packlaufs traegt deshalb immer `.zip` und nie `.ZIP`.

3. `crates/krk-ui/src/kommandos/kontextmenue.rs:304-307` — die Gegenrichtung faltet:

   ```rust
   pub fn ist_zipname(name: &str) -> bool {
       let (_, endung) = namen_teilen(name);
       endung.eq_ignore_ascii_case(ENDUNG)
   }
   ```

   Das ist die zweite Nutzerentscheidung dieser Runde, ausgeschrieben im Modulkopf: „Ein Archiv wird
   an der Endung erkannt, ohne Ruecksicht auf Gross- und Kleinschreibung."

4. Die Datei weiss um die Lage und nennt sie an anderer Stelle selbst
   (`kontextmenue.rs:353-356`): „`a.zip` neben `a.ZIP` tut es seit dem ersten Tag dieser Runde, weil
   die Endung ohne Ruecksicht auf die Schreibung erkannt wird."

5. `crates/krk-ui/src/kommandos/kontextmenue.rs:591-597` — der Doc-Kommentar begruendet den
   buchstabengetreuen Vergleich so:

   ```
   /// Verglichen werden **Pfade, wie sie dastehen**, ohne `canonicalize` und ohne
   /// Ruecksicht auf Verknuepfungen. Mehr ist hier nicht noetig: beide Listen
   /// entstehen als `ordner.join(name)` ueber demselben angezeigten Ordner
   /// ([`super::operationen::betroffene`] auf der einen, [`archivname`] und
   /// [`paar`] auf der anderen Seite), und derselbe Eintrag ergibt damit
   /// buchstaeblich denselben Pfad.
   ```

   **Das ist die Stelle, an der die Begruendung bricht.** Sie stimmt fuer `paar`, das den Namen aus
   der Liste **nimmt**. Sie stimmt nicht fuer `archivname`, das den Namen **bildet**: der Stamm kommt
   zwar aus der Liste, die Endung aber aus der kleingeschriebenen Konstante `ENDUNG`. Das Ziel ist
   deshalb gerade nicht „derselbe Eintrag" aus derselben Liste, sondern ein gerechneter Name, und die
   Schreibung seiner Endung ist die des Bauwerks und nicht die der Platte.

## Der Ablauf am Bildschirm

**Packen.** Im Ordner `Projekte` liegt `PROJEKTE.ZIP` — von einem fremden Werkzeug angelegt, aus
einem Archiv entpackt oder von Hand umbenannt. Der Nutzer markiert mehrere Eintraege darunter dieses
und waehlt Zip. `archivname` rechnet `Projekte/Projekte.zip`; `ist_ziel_des_laufs` haelt
`Projekte/PROJEKTE.ZIP` dagegen und sagt „nein, ein anderer Pfad". Der Eintrag bleibt Quelle. Auf der
Platte trifft `fs::symlink_metadata(ziel)` in `zielarchiv_klaeren`
(`crates/krk-core/src/operation/zippen.rs`) dieselbe Datei, das Konfliktblatt geht auf, und
„Ueberschreiben" raeumt **eine Quelle des Laufs** in den Papierkorb. Das ist Wort fuer Wort der
Befund `260825-1144_*_ueberschreiben-raeumt-eine-quelle-des-laufs-in-den-papierkorb-wenn-der-archivname-ihrem-namen-gleicht.md`,
den dieser Commit schliessen sollte.

**Entpacken.** Neben `a.zip` steht `A.ZIP.zip`. Beide markiert: `paar` rechnet fuer das zweite den
Zielordner `<ordner>/A.ZIP`, `ohne_die_eigenen_ziele` haelt `<ordner>/a.zip` dagegen und laesst es
stehen. Auf der Platte ist es derselbe Eintrag.

## Was die Proben nicht sehen

Die drei neuen Proben (`kontextmenue.rs:1095`, `1124`, `1147`) arbeiten durchweg mit gleicher
Schreibung. `die_endung_entscheidet_ohne_ruecksicht_auf_die_schreibung` (`kontextmenue.rs:757`)
prueft die Faltung, aber an `ist_zipname` und nicht am Schnitt.

## Warum das keine Zeile nebenbei ist

Ein `eq_ignore_ascii_case` an dieser Stelle waere die falsche Antwort: APFS laesst sich
**gross-/kleinschreibungsempfindlich** formatieren, und auf einem solchen Datentraeger sind
`Projekte.zip` und `PROJEKTE.ZIP` zwei Dateien. Ein faltender Vergleich schnitte dort eine Quelle
heraus, die keine Kollision ist — der zu weite Schnitt, gegen den
`ein_einzelnes_archiv_bleibt_seine_eigene_quelle` gebaut ist, nur in der anderen Gestalt. Welche
Regel das Dateisystem fuehrt, ist ohne Zugriff auf die Platte nicht entscheidbar, und dieses Modul
fasst ausdruecklich kein Dateisystem an (`kontextmenue.rs:606-608`).

## Vorschlag

Drei Wege, und die Wahl ist die des Nutzers.

1. **Falten und die Ungenauigkeit ausschreiben.** `ist_ziel_des_laufs` vergleicht die letzten
   Bestandteile mit `eq_ignore_ascii_case` und den Rest wie bisher. Auf dem
   gross-/kleinschreibungsempfindlichen Datentraeger faellt dann gelegentlich eine Quelle heraus,
   die keine Kollision waere; der Nutzer verliert nichts, ihm fehlt ein Eintrag im Archiv. Der
   billigste Weg, und die Ungenauigkeit gehoert dann in den Doc-Kommentar.
2. **Die Frage an die Platte geben.** Der Schnitt zieht in die Oberflaechenschicht um, die einen
   Dateizugriff haben darf, und fragt `same_file`/`fs::canonicalize`. Das ist die genaue Antwort und
   bricht die Zusage dieses Moduls, ohne Dateisystem zu rechnen; ausserdem faellt sie unter L9,
   sobald sie auf dem Hauptfaden steht.
3. **Die Zusage einschraenken.** Der Modulkopf sagt, dass der Schnitt auf die Schreibung ankommt,
   und der Fall der abweichenden Schreibung bleibt offen. Ehrlich, und die Lage bleibt bestehen.

Dazu in jedem Fall die Probe, die heute fehlt: ein Lauf, dessen Ziel eine seiner Quellen in
abweichender Schreibung trifft.

**Schwere:** mittel. Kein Datenverlust — der Eintrag geht in den Papierkorb —, aber die Nutzerzusage
dieser Runde haelt in einem erreichbaren Fall nicht.

**Betroffen:** `crates/krk-ui/src/kommandos/kontextmenue.rs` (`ist_ziel_des_laufs`, beide Rufer);
mittelbar `crates/krk-core/src/operation/zippen.rs` und `entpacken.rs`, deren Konfliktblatt die Lage
sichtbar macht.
