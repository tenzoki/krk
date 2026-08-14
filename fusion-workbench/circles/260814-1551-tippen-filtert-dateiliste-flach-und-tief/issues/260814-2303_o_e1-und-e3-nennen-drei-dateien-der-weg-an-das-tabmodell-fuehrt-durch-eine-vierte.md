# E1 und E3 nennen ihre Dateien vollständig, der Weg an das Tabmodell führt durch eine vierte

**Status:** Open
**Domain:** Plan der Filter-Runde, Strang E
**Filed by:** coder, beim Umsetzen von E1
**Related:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritte E1 und E3

## Befund

Schritt E1 nennt drei Dateien und schreibt vor, der Ausführungszweig in
`Anwendungsdelegierter::kommando_ausfuehren` kippe "das Kennzeichen am Modell des
sichtbaren Tabs des aktiven Dateifensters". Aus den drei genannten Dateien ist dieses
Modell nicht erreichbar.

Der Weg dorthin läuft über `self.dateifenster(seite).quelle()`, also über
`DateifensterQuelle` aus `crates/krk-ui/src/appkit/tabelle.rs`. Deren Tabliste steht im
Ivar `QuelleIvars::tabs` (`crates/krk-ui/src/appkit/tabelle.rs:348`), und das Feld ist
modulprivat; `QuelleIvars` selbst ist `pub`, seine Felder sind es nicht. Ein `impl`-Block
in `anwendung.rs` erreicht es deshalb nicht, auch nicht als inhärente Erweiterung
derselben Kiste. Am 260814-2303 trägt `DateifensterQuelle` keine öffentliche Methode, die
das Kennzeichen setzt oder liest: die Nachbarin `verstecke_umschalten`
(`tabelle.rs:1626`) ist privat und hat ihren einen Aufrufer in
`DateifensterQuelle::kommando_ausfuehren` derselben Datei.

**Schritt E3 trägt dasselbe von der Leseseite her.** Er nennt
`appkit/bereichsleiste.rs` und `appkit/anwendung.rs` und schreibt vor,
`bereichsleiste_nachziehen` hole den Wert "aus dem Modell des sichtbaren Tabs des aktiven
Dateifensters". Auch dafür gibt es heute keinen öffentlichen Leser, und `tabelle.rs` steht
auch dort nicht in der Dateiliste.

## Was beim Umsetzen von E1 daraus geworden ist

Der Umsetzer hat `DateifensterQuelle::tiefe_suche_umschalten` in
`crates/krk-ui/src/appkit/tabelle.rs` angelegt, unmittelbar neben `verstecke_umschalten`
und in derselben Bauart: Ausleihe, Kippen über `Ordnermodell::tief_setzen`, danach
`umsortiert`. Der Zweig in `anwendung.rs` ruft sie. Damit weicht der Schritt von seiner
Dateiliste ab, und die Abweichung steht hier, statt still zu bleiben.

## Warum das kein Streit mit Strang B ist

Schritt B1 baut `sprungmarke_tippen` derselben Datei zu `filterzeichen_tippen` um und
entfernt den Ivar `sprungmarke`. Die neue Methode berührt weder das eine noch das andere;
sie ruft `umsortiert`, das B1 ohnehin anfasst. Ein Zusammenstoß entsteht dort nicht.

## Vorschlag

Die Dateilisten von E1 und E3 um `crates/krk-ui/src/appkit/tabelle.rs` ergänzen, oder den
Zugriffsweg im Plan benennen, falls ein anderer gemeint war.
