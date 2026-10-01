//! Die drei Sprachtabellen, je eine Datei: [`de`], [`fr`], [`en`].
//!
//! Jede Datei traegt `text(Text) -> &'static str` und
//! `zahlwort(Zahlwort) -> (&'static str, &'static str)` als `match` ohne
//! Auffangzweig; ein neuer Schluessel haelt den Bau in allen drei Dateien an.
//! Die deutsche Tabelle ist die Quelle: ihre Eintraege sind der Wortlaut,
//! den die Oberflaeche vor dieser Arbeit trug, Zeichen fuer Zeichen, damit
//! jede Wortlautprobe weiter haelt. Die franzoesische und die englische hat
//! ein Agent uebersetzt; ihr Kopf sagt, solange der Nutzer sie nicht
//! Flaeche fuer Flaeche durchgesehen hat.
//!
//! **Dieses Verzeichnis ist die eine Eigenschaft, die die Umlautprobe
//! ausnimmt**: ein Stringliteral mit Umlaut im Betriebscode ausserhalb von
//! `sprache/tabelle/` ist ein Defekt, hier drin ist es die Regel.
//!
//! # Glossar
//!
//! Das Glossar bindet jeden Eintrag von `fr.rs` und `en.rs`; wer einen
//! Begriff braucht, der hier fehlt, traegt ihn ein. Die deutsche Spalte
//! steht in Umschrift, weil sie ein Kommentar ist; die Tabelle traegt die
//! Umlaute.
//!
//! | Deutsch | Franzoesisch | Englisch |
//! |---|---|---|
//! | Finder | Finder | Finder |
//! | Papierkorb | Corbeille | Trash |
//! | Schreibtisch | Bureau | Desktop |
//! | Zwischenablage | Presse-papiers | Clipboard |
//! | Ordner | dossier | folder |
//! | Datei | fichier | file |
//! | Dateifenster | volet de fichiers | file pane |
//! | Vorschau | Aperçu | Preview |
//! | Editor | Éditeur | Editor |
//! | Lesezeichen | signet | bookmark |
//! | Datentraeger | volume | volume |
//! | Verknuepfung | lien symbolique | symbolic link |
//! | Tastenbelegung | raccourcis clavier | key bindings |
//! | Belegungsansicht | vue des raccourcis | key-binding view |
//! | Blatt | feuille | sheet |
//! | Werkseinstellungen | réglages d’usine | factory settings |
//! | Notizordner | dossier de notes | notes folder |
//! | Geheimnisse | secrets | secrets |
//! | Termine | rendez-vous | appointments |
//! | Aufgaben | tâches | tasks |
//! | Bildfolge | série de photos | photo sequence |
//! | Quicknote | Quicknote | Quicknote |
//! | Rueckgaengig / Wiederholen | Annuler / Rétablir | Undo / Redo |
//! | Ausschneiden / Kopieren / Einfuegen | Couper / Copier / Coller | Cut / Copy / Paste |
//! | Alles auswaehlen | Tout sélectionner | Select All |
//! | Bearbeiten (Menue) | Édition | Edit |
//! | Fenster (Menue) | Fenêtre | Window |
//! | Ueber KRK | À propos de KRK | About KRK |
//! | Beenden | Quitter KRK | Quit KRK |
//! | Schliessen | Fermer | Close |
//! | Abbrechen | Annuler | Cancel |
//! | Sichern | Enregistrer | Save |
//! | Verwerfen | Ne pas enregistrer | Don’t Save |
//! | Deep, Content (Ankreuzfelder) | Deep, Content | Deep, Content |
//! | Tastennamen (`cmd`, `return`, `esc`) | unveraendert | unveraendert |
//! | Leiste (Lesezeichen- und Geraeteleiste) | barre | bar |
//! | Git-Bereich | zone Git | Git area |
//! | Auslieferungszustand | version livrée | shipped version |
//! | Byte, kB, MB, GB, TB | octet, ko, Mo, Go, To | byte, kB, MB, GB, TB |
//! | Thema (einer Notiz) | sujet | topic |
//! | Platzhalter `*` | joker | wildcard |
//! | vorgemerkt (Git) | indexé | staged |
//! | Eintrag (eines Ordners, eines Archivs) | entrée | entry |
//! | Quelle / Ziel (eines Vorgangs) | source / destination | source / destination |
//! | Zielordner | dossier de destination | destination folder |
//! | Archiv | archive | archive |
//! | Arbeitsfaden | fil de travail | worker thread |
//! | Dateizugriff (Deskriptor) | descripteur de fichier | file descriptor |
//! | Abbruch (eines Vorgangs) | annulation | cancelling |
//! | Nummerierung / Startwert / Stellenzahl | numérotation / valeur de départ / nombre de chiffres | numbering / starting value / number of digits |
//! | Systemeinstellungen | Réglages Système | System Settings |
//! | Datenschutz & Sicherheit | Confidentialité et sécurité | Privacy & Security |
//! | Festplattenvollzugriff | Accès complet au disque | Full Disk Access |
//! | Ablage, Ablageordner (`~/Library/Application Support/KRK`) | dossier de données | data folder |
//! | Benutzerverzeichnis | dossier de départ | home directory |
//! | Ort (des Notizordners) | emplacement | location |
//! | Ort waehlen… (Menue Home) | Choisir l’emplacement… | Choose Location… |
//! | Home (Menue) | Home | Home |
//! | beiseitelegen (eine Sicherung) | mettre de côté | set aside |
//! | zuruecksetzen (auf Werkseinstellungen) | réinitialiser | reset |
//! | Zettel (die alten Notizzettel) | note | note |
//! | uebernehmen (die alten Zettel) | reprendre | take over |
//! | Baustein (eines Leseprofils) | brique | building block |
//! | Pfadmuster / Kennzeichendatei | motif de chemin / fichier repère | path pattern / marker file |
//! | Zusatztaste | touche de modification | modifier key |
//! | Zusammenfassung (eines Profils) | résumé | summary |
//! | Tresor, Kopf der Datei (`secrets.txt`) | en-tête du fichier | header of the file |
//! | Repository, Commit, Branch (Git) | dépôt, commit, branche | repository, commit, branch |
//! | abgeloest (HEAD) | détaché | detached |
//! | Tab (eines Dateifensters) | onglet | tab |
//! | markieren (Eintraege fuer einen Vorgang) | marquer | mark |
//! | Marke (die Spalte des Git-Status) | marque | mark |
//! | Fokus | focus | focus |
//! | Standardprogramm | application par défaut | default application |
//! | Terminal | Terminal | Terminal |
//! | Roh- und Formatansicht | vue brute et vue formatée | raw and formatted view |
//! | Neuerungen | nouveautés | What’s New |
//! | Instanz | instance | instance |
//! | Bild (einer Bildfolge) | image | image |
//! | PIN | code PIN | PIN |
//! | laufende Operation | opération en cours | running operation |
//! | im Stapel (umbenennen) | par lot | batch |
//! | Sortierrichtung | ordre de tri | sort order |
//! | versteckte Dateien | fichiers cachés | hidden files |
//! | Pfad | chemin | path |
//! | Bildschirmseite | page d’écran | screen page |
//! | Bereich (der Fensterzeile) | zone | area |
//!
//! # Die Befehlsnamen
//!
//! Jede Datei traegt daneben `kommandoname(Kommando)` und
//! `zugestellt_name(Zugestellt)`, je ein `match` ohne Auffangzweig ueber die
//! Aufzaehlungen aus `crate::tasten::belegung`; ein neues Kommando haelt den
//! Bau in allen drei Dateien an, bis es seinen Namen hat. Die deutschen Namen
//! sind die, die `resources/default-keymap.toml` bis zum Schritt 5 des Plans
//! unter `name` trug, Zeichen fuer Zeichen. Die englischen Namen stehen in
//! Title Case, weil sie Eintraege des Hauptmenues sind; die franzoesischen in
//! Satzform, wie macOS seine franzoesischen Menues fuehrt. Je Sprache tragen
//! keine zwei Funktionen denselben Namen; das haelt
//! `keine_zwei_funktionen_tragen_in_einer_sprache_denselben_namen` in
//! `crates/krk-core/tests/belegung.rs`.
//!
//! # Typografie
//!
//! Franzoesisch: Anfuehrungszeichen « » mit U+00A0 innen, geschuetztes
//! Leerzeichen vor `:` (U+00A0) und vor `;`, `!`, `?` (U+202F), Akzente
//! auch auf Grossbuchstaben (`À propos`, `Éditeur`), Einzahl bei 0 und 1.
//! Ein `:` innerhalb einer Schreibweise wie `HH:MM` ist kein Satzzeichen
//! und bekommt kein geschuetztes Leerzeichen. Englisch: “ ” als
//! Anfuehrungszeichen, Title Case in Menueeintraegen und Schaltflaechen,
//! wie macOS sie fuehrt, Satzform in Statuszeile und Erlaeuterungen.
//! Deutsch: der heutige Wortlaut mit „ “. Der Apostroph ist in allen drei
//! Sprachen U+2019. **Kein Eintrag traegt ein ASCII-Anfuehrungszeichen**;
//! wo eine Meldung die TOML- oder die Debug-Schreibweise eines Werts zeigt
//! (`"~/krkhome"`, `"a(b"`), kommt sie als Platzhalterwert zur Laufzeit
//! herein (`{beispiel}` in `OrtKeinText`, `{muster}` in den
//! `Profil…`-Eintraegen), und der Rufer bildet sie mit `toml::Value` oder
//! `{:?}`.

pub(super) mod de;
pub(super) mod en;
pub(super) mod fr;
