`Lage::bildfolge` fragt allein den Inhalt der Vorschau und lenkt Cmd+Pfeil hoch und Return nach einem Fenster- oder Auswahlwechsel auf die alte Folge
---
`Anwendungsdelegierter::lage` setzt `bildfolge` auf „Vorschau sichtbar und ihr aktiver Tab zeigt `Inhalt::Bildfolge`“. Ob die Folge zu dem gehört, was das aktive Dateifenster gerade beschreibt, fragt die Lage nicht. In zwei Lagen zeigt die Vorschau eine Folge, die nicht mehr vor dem Nutzer liegt, und dann gewinnen `BildZurueck` und `ZumBild` die geteilten Kombinationen: nach einem Wechsel des aktiven Dateifensters dauerhaft, nach einem Auswahlwechsel bis zur Lieferung des neuen Ladevorgangs. Das verletzt C3.4 und C4.6 des Spec (`260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md`): mit einem Ordner ohne Bildfolge ausgewählt führt Cmd+Pfeil hoch nicht nach oben, und Return öffnet nicht mit dem Standardprogramm.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Cross-references:** `260929-1423_*_plan-vorschau-blaettert-fotos-nach-aufnahmedatum.md` (Entscheidung 8, Schritt 7); `260929-1423_*_wie-teilen-zwei-funktionen-eine-kombination-wenn-die-eine-nur-bei-stehender-bildfolge-wirkt.md`; `260825-1922_*_der-programmstart-und-der-tabwechsel-erreichen-die-neue-vorschauregel-nicht.md` (derselbe Mechanismus von der anderen Seite: die Vorschau folgt nicht jedem Weg, auf dem sich ändert, was vor dem Nutzer liegt)

## Befund

- `crates/krk-ui/src/appkit/anwendung.rs:4006`: `bildfolge: …sichtbar(Bereich::Vorschau) && …vorschau.zeigt_bildfolge()`.
- `crates/krk-ui/src/vorschaumodell.rs:1218`, `zeigt_bildfolge`: `matches!(self.aktiver_inhalt(), Inhalt::Bildfolge(_))`. Kein Blick auf `ladevorgang`, auf `folge` oder auf den Pfad.
- `crates/krk-ui/src/vorschaumodell.rs:1168`, `folgebild`: liest ebenso allein `aktiver_inhalt()`.

**Lage 1, Fensterwechsel (deterministisch).** `vorschau_fuellen` (`anwendung.rs:2374`) nimmt allein Meldungen des aktiven Dateifensters an, und nur, wenn dessen Auswahl sich ändert. `Kommando::FensterWechseln` (`anwendung.rs:4255`) und `aktives_setzen` (`anwendung.rs:6312`) rufen `fokus_setzen`, aber keine Meldung an die Vorschau. Links steht `Fotos` mit ausgewählter Zeile `2008`, die Vorschau zeigt die Jahresfolge. Nach Tab ins rechte Dateifenster, das irgendeinen Ordner ohne Bildfolge zeigt:
- Cmd+Pfeil hoch: `waehlen` nimmt `BildZurueck` (Fokus Dateifenster, `bildfolge` wahr). Die Folge links blättert, das rechte Fenster steigt nicht auf.
- Return: `ZumBild` → `zum_bild_springen` (`anwendung.rs:4775`) → `zum_eintrag_springen` im **aktiven, also rechten** Dateifenster: es springt in den Monatsordner des Fotos, das links angezeigt wird (etwa `Fotos/2008/01`), statt die Auswahl rechts zu öffnen.

**Lage 2, Auswahlwechsel im selben Fenster (Zeitfenster).** `datei_anzeigen` (`vorschaumodell.rs:1023`) setzt `tab.folge = None` (`:1036`) und startet einen Ladevorgang; `tab.inhalt` bleibt die alte `Inhalt::Bildfolge`, bis `einziehen` die neue Meldung abholt (Takt `LADETAKT` = 1/60 s, `appkit/vorschau.rs:353`, plus Ladezeit). In dieser Spanne:
- Return nach Pfeil runter von `2008` auf eine Datei oder auf `2009`: `folgebild` liefert das Foto der alten Folge, und die Liste springt dorthin.
- Cmd+Pfeil hoch: `BildZurueck` gewinnt, `Vorschautab::blaettern` findet `folge == None` und tut nichts; der Aufstieg bleibt aus.

**inference:** Die Spanne ist bei lokalen Ordnern kurz (kopflos gemessen 4 ms für die Erhebung, `messungen/260929-1432-bildfolge-kopflos.txt`, dazu ein Takt). Bei einem Fotoarchiv auf einem Netzlaufwerk oder einem Ordner, dessen Default-Zählung lange liest, reicht sie bis in den Bereich eines normalen Tastenabstands. Am Bündel ist keine der zwei Lagen gemessen.

Die Probe `die_bildfolge_geht_vor_und_wirkt_allein_in_der_dateiliste` (`kommandos/zulaessigkeit.rs`) prüft die Wahl an einer gebauten `Lage` und sieht keine der zwei Lagen, weil der Fehler in der Erhebung des Feldes steht und nicht in der Regel.

## Richtung

`Lage::bildfolge` nur dann wahr, wenn die Folge zu dem gehört, was das aktive Dateifenster jetzt beschreibt, und kein neuerer Auftrag im Tab wartet. Etwa: `Vorschaumodell::zeigt_bildfolge_fuer(pfad)` mit `ladevorgang.is_none()`, `folge.is_some()` und `tab.pfad == pfad`, gefragt mit `zu_beschreiben` des aktiven Dateifensters. Eine Frage an einer Stelle, keine zweite Regel neben `folge_passt`. Der Zusammenhang mit der offenen Frage `260929-1646_*_blaettert-cmd-up-im-angezeigten-jahres-oder-monatsordner-ohne-ausgewaehlte-zeile.md` gehört mit in den Entwurf.

## Abnahme

- Eine Probe im Modell: nach `datei_anzeigen` mit einem anderen Pfad antwortet die Frage nein, bevor die neue Meldung eingezogen ist.
- Eine Probe: die Frage antwortet nein, wenn der gefragte Pfad nicht der Pfad des Tabs ist.
- Am Bündel: links `Fotos` mit Zeile `2008`, Tab nach rechts in einen Ordner mit ausgewählter Datei; Cmd+Pfeil hoch steigt rechts auf, Return öffnet die Datei mit dem Standardprogramm.

---
Resolved: `Lage::bildfolge` kommt allein aus `Anwendungsdelegierter::bildfolge_steht`, das die Vorschau mit `DateifensterQuelle::beschrieben` des aktiven Dateifensters fragt (`Vorschaumodell::zeigt_bildfolge_von`: gleicher Pfad, kein wartender Auftrag); Proben `die_bildfolge_gilt_allein_fuer_ihren_pfad_und_nicht_vor_einer_neuen_lieferung` und `die_bildfolge_der_lage_fragt_den_pfad_des_aktiven_dateifensters`, die Abnahme am Buendel bleibt Nutzerarbeit.
