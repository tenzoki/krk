//! Die Schluessel der Sprachtabelle: [`Text`] fuer feste Texte und Saetze mit
//! Platzhaltern, [`Zahlwort`] fuer Mengenangaben mit Einzahl und Mehrzahl.
//!
//! **Ein Schluessel heisst nach der Flaeche und der Sache**, ASCII und ohne
//! Umschrift-Zweideutigkeit: `WirkungsbereichDateifenster`,
//! `AbweisungUmbruchImThema`, `EinheitKilobyte`. Ein Text, der heute an zwei
//! Stellen gleich steht, bekommt einen Schluessel und nicht zwei; so traegt
//! `NameLeer` den Grund fuer einen leeren Dateinamen und fuer einen leeren
//! Lesezeichennamen.
//!
//! Jede Aufzaehlung fuehrt daneben `ALLE`, und die Probe
//! `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` in
//! `crates/krk-core/tests/baum.rs` haelt die Liste gegen die Aufzaehlung; die
//! Proben in `crates/krk-core/tests/sprache.rs` laufen ueber `ALLE` und
//! halten je Sprache und Schluessel, was der Uebersetzer nicht haelt.
//!
//! Die Eintraege stehen in `tabelle/de.rs`, `tabelle/fr.rs` und
//! `tabelle/en.rs`; der Modulkopf von [`super`] traegt die Regel.

/// Ein fester Text oder ein Satz mit benannten Platzhaltern.
///
/// Die Doc-Zeile je Wert nennt die Stelle, die ihn zeigt, und nicht den
/// Wortlaut: der steht in der Tabelle, einmal je Sprache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Text {
    /// `Wirkungsbereich::Dateifenster` in der Markdown-Ausgabe der Belegung.
    WirkungsbereichDateifenster,
    /// `Wirkungsbereich::Leiste`.
    WirkungsbereichLeiste,
    /// `Wirkungsbereich::Dateibereiche`.
    WirkungsbereichDateibereiche,
    /// `Wirkungsbereich::Editor`.
    WirkungsbereichEditor,
    /// `Wirkungsbereich::Editortext`.
    WirkungsbereichEditortext,
    /// `Wirkungsbereich::Eintraege`.
    WirkungsbereichEintraege,
    /// `Wirkungsbereich::Reihenfolge`.
    WirkungsbereichReihenfolge,
    /// `Wirkungsbereich::Aufgaben`.
    WirkungsbereichAufgaben,
    /// `Wirkungsbereich::Termine`.
    WirkungsbereichTermine,
    /// `Wirkungsbereich::Geheimnisse`.
    WirkungsbereichGeheimnisse,
    /// `Wirkungsbereich::Quicknote`.
    WirkungsbereichQuicknote,
    /// `Wirkungsbereich::Tabbereich`.
    WirkungsbereichTabbereich,
    /// `Wirkungsbereich::Navigator`.
    WirkungsbereichNavigator,
    /// `Wirkungsbereich::Vorschau`.
    WirkungsbereichVorschau,
    /// `Wirkungsbereich::Bildfolge`.
    WirkungsbereichBildfolge,
    /// `Wirkungsbereich::Ueberall`.
    WirkungsbereichUeberall,
    /// `Ortsmangel::Absolut`: der Satzteil in der Statuszeile zu einer
    /// Ortsangabe in `readers.toml`.
    OrtsmangelAbsolut,
    /// `Ortsmangel::LeeresStueck`.
    OrtsmangelLeeresStueck,
    /// `Ortsmangel::Punktstueck`.
    OrtsmangelPunktstueck,
    /// `Ortsmangel::MehrerePlatzhalter`.
    OrtsmangelMehrerePlatzhalter,
    /// `Namensfehler::Leer` und `Namenshinweis::Leer`: der Grund, aus dem ein
    /// Name nicht vergeben wird.
    NameLeer,
    /// `Namensfehler::Schraegstrich`.
    NameMitSchraegstrich,
    /// `Namensfehler::Nullbyte`.
    NameMitNullbyte,
    /// `Namensfehler::Punktname`.
    NamePunktname,
    /// `Kollision::Bestehender` in der Spalte der Vorschau beim
    /// Stapelumbenennen.
    KollisionBestehender,
    /// `Kollision::Doppelt`.
    KollisionDoppelt,
    /// `ablage::Grund::NichtLesbar`: der Satzteil in der Meldung zu einer
    /// ersetzten Ablagedatei.
    AblagegrundNichtLesbar,
    /// `ablage::Grund::Beschaedigt`.
    AblagegrundBeschaedigt,
    /// `ablage::Grund::NichtAnlegbar`.
    AblagegrundNichtAnlegbar,
    /// `Ersatz::Auslieferungszustand`: was an die Stelle der ersetzten
    /// Ablagedatei tritt.
    ErsatzAuslieferungszustand,
    /// `Ersatz::Nichts`.
    ErsatzNichts,
    /// `Pinfehler::KeineVierZiffern` im Blatt der PIN-Abfrage.
    PinKeineVierZiffern,
    /// `Abweisung::UmbruchImAufgabentext` in der Statuszeile des Editors.
    AbweisungUmbruchImAufgabentext,
    /// `Abweisung::UmbruchImThema`.
    AbweisungUmbruchImThema,
    /// `Abweisung::ThemenzeileImNotiztext`.
    AbweisungThemenzeileImNotiztext,
    /// `Abweisung::UngueltigesDatum`.
    AbweisungUngueltigesDatum,
    /// `Abweisung::KopfzeileImTermintext`.
    AbweisungKopfzeileImTermintext,
    /// Die erste Zaehlzeile des Default-Profils in der Vorschau.
    ZaehlzeileDateien,
    /// Die zweite Zaehlzeile des Default-Profils.
    ZaehlzeileOrdner,
    /// Die dritte Zaehlzeile des Default-Profils.
    ZaehlzeileVerknuepfungen,
    /// Die Einheit hinter einer Datenmenge ab 1.000 Bytes.
    EinheitKilobyte,
    /// Die Einheit ab 1.000.000 Bytes.
    EinheitMegabyte,
    /// Die Einheit ab 1.000.000.000 Bytes.
    EinheitGigabyte,
    /// Die Einheit ab 1.000.000.000.000 Bytes.
    EinheitTerabyte,
    /// `operation::grund` bei `PermissionDenied`: der Grund in der
    /// Abschlussliste eines Vorgangs.
    VorgangKeineRechte,
    /// `operation::grund` bei `NotFound`.
    VorgangGibtEsNichtMehr,
    /// `operation::grund` bei `AlreadyExists`, und die Antwort
    /// „ueberspringen“ auf eine Konfliktfrage in jedem der drei Zielklaerer.
    VorgangAmZielStehtEintrag,
    /// `operation::grund` bei `StorageFull`.
    VorgangKeinPlatzAufDemDatentraeger,
    /// `operation::starten`, wenn kein Arbeitsfaden zu bekommen ist; `{grund}`
    /// ist der Systemtext.
    VorgangKeinArbeitsfaden,
    /// `einen_abarbeiten`: ein Eintrag des Stapel-Umbenennens oder des
    /// Duplizierens ohne neuen Namen.
    VorgangNeuerNameFehlt,
    /// `einen_abarbeiten`: ein Archiv ohne Zielordner.
    VorgangZielordnerFehlt,
    /// `einen_abarbeiten`: der Zweig fuer das Packen, den die Schleife nicht
    /// erreicht.
    VorgangPackenNichtQuelleFuerQuelle,
    /// `zielpfad` und `quellen_packen`: ein Pfad ohne letzten Namensteil.
    VorgangPfadBenenntKeinenEintrag,
    /// `zielpfad`: die erste der zwei Naemlichkeitsfragen.
    VorgangQuelleUndZielDerselbeEintrag,
    /// `zielpfad`: die zweite der zwei Naemlichkeitsfragen.
    VorgangZielLiegtInDerQuelle,
    /// `ziel_klaeren`: das Wegraeumen vor dem Ueberschreiben ist gescheitert;
    /// `{grund}`.
    VorgangZielNichtErsetzt,
    /// Die halbe Datei oder das halbe Archiv nach einem Abbruch; `{grund}`.
    VorgangNachAbbruchNichtWeggeraeumt,
    /// `kopieren::ordner`: Rechte und Datum des Ordners; `{grund}`.
    VorgangOrdnerangabenNichtKopiert,
    /// `verschieben::verschmelzen`: der leere Quellordner blieb; `{grund}`.
    VorgangOrdnerSelbstBlieb,
    /// `verschieben::ueber_datentraeger`: ein Kind ist nicht angekommen.
    VorgangNichtVollstaendigKopiert,
    /// `verschieben::ueber_datentraeger`: die Quelle liess sich nach der Kopie
    /// nicht entfernen; `{grund}`.
    VorgangKopiertAberInQuelleGeblieben,
    /// `OhnePapierkorb`: der Papierkorb, den es nicht gibt.
    VorgangKeinPapierkorb,
    /// Duplizieren und Packen: die Quelle ist Ordner, Verknuepfung, Roehre,
    /// Socket oder Geraetedatei.
    VorgangKeineGewoehnlicheDatei,
    /// Packen und Entpacken: das Ziel liess sich vor dem Ueberschreiben nicht
    /// in den Papierkorb raeumen; `{grund}`.
    VorgangZielNichtInPapierkorb,
    /// `entpacken`: ein Archiveintrag `{name}`, dessen Name ueber `..`
    /// hinausfuehrt.
    EntpackenEintragFuehrtHeraus,
    /// `entpacken`: ein Archiveintrag `{name}` mit dem Grund `{grund}` aus
    /// `kette_anlegen`.
    EntpackenEintragMitGrund,
    /// `entpacken`: am Ziel von `{name}` steht eine Verknuepfung.
    EntpackenAmZielStehtVerknuepfung,
    /// `entpacken::kette_anlegen`: eine Komponente, die kein blosser Name ist.
    EntpackenWegMitUnzulaessigemBestandteil,
    /// `entpacken::kette_anlegen`: eine Verknuepfung auf dem Weg.
    EntpackenWegDurchVerknuepfung,
    /// `entpacken::kette_anlegen`: eine Datei, wo ein Ordner stehen muesste.
    EntpackenDateiStattOrdnerAufDemWeg,
    /// `entpacken::verknuepfung_ablegen`: der Inhalt ist kein UTF-8.
    EntpackenVerweiszielKeinText,
    /// `zippen::lauf`: `finish` ist gescheitert; `{fehler}` ist der Text der
    /// Kiste.
    PackenArchivUnfertig,
    /// `zippen`: `start_file`, `add_directory` oder `add_symlink` ist
    /// gescheitert; `{fehler}`.
    PackenKeinPlatzImArchiv,
    /// `zippen::datei_packen`: `abort_file` ist gescheitert; `{fehler}`.
    PackenHalberEintragImArchiv,
    /// `zippen::datei_packen`: `write_all` ist gescheitert; `{fehler}`.
    PackenNichtInsArchivGeschrieben,
    /// `Regelfehler::Startwert` in der Hinweiszeile des Stapelumbenennens;
    /// `{text}` ist die Eingabe.
    StapelKeinStartwert,
    /// `Regelfehler::Stellenzahl`; `{text}` ist die Eingabe, `{hoechste}` die
    /// Obergrenze.
    StapelKeineStellenzahl,
    /// `Abweisung::KeinGueltigesZiel` mit Deskriptormangel in der Statuszeile
    /// des Editors; `{pfad}`, `{grund}`.
    EditorKeinFreierDateizugriff,
    /// `Abweisung::KeinGueltigesZiel` ohne Mangel; `{pfad}`, `{grund}`.
    EditorNichtZuOeffnen,
    /// `Abweisung::ZuGross`; `{pfad}`, `{groesse}` und `{grenze}` in Bytes.
    EditorZuGross,
    /// `Abweisung::NichtAlsTextLesbar`; `{pfad}`.
    EditorKeineTextdatei,
    /// `text::datei::lesen`: der Satzteil `{grund}` fuer einen Ordner.
    EditorOrdnerHatKeinenText,
    /// `text::datei::lesen`: der Satzteil `{grund}` fuer alles, was weder
    /// Ordner noch gewoehnliche Datei ist.
    EditorKeineGewoehnlicheDatei,
    /// `verzeichnis::datenschutzsperre` in der Statuszeile; `{name}` ist der
    /// Ordnername.
    LesenDatenschutzsperre,
    /// `Schwungleser::oeffnen`: der Pfad `{pfad}` benennt kein Verzeichnis.
    LesenKeinVerzeichnis,
    /// `sys::als_c_pfad`: der Pfad `{pfad}` traegt ein Nullbyte.
    LesenPfadMitNullbyte,
    /// `Zugang::laden` und `einstellungen::als_text`: der Befund zu einer
    /// Ablagedatei, die kein UTF-8 ist.
    AblageKeinGueltigesUtf8,
    /// `Zugang::laden`: der Befund zu einer Ablagedatei ohne obersten
    /// Schluessel.
    AblageOhneOberstenSchluessel,
    /// `Ablageort::im_benutzerverzeichnis`: der Fehlertext, wenn das System
    /// kein Benutzerverzeichnis nennt.
    AblageKeinBenutzerverzeichnis,
    /// `atomar::mit_endung`: der Pfad `{pfad}` traegt keinen Dateinamen.
    AtomarOhneDateinamen,
    /// `atomar::rechte_uebernehmen`: die Rechte `{soll}` von `{pfad}` sind
    /// nicht uebertragen, die Nachbardatei steht auf `{gesetzt}`.
    AtomarRechteNichtUebertragen,
    /// `Ersetzung` ohne Sicherung in der Statuszeile beim Start: `{datei}`,
    /// `{beschreibung}` (ein `Ablagegrund…`), `{ersatz}` (ein `Ersatz…`),
    /// `{einzelheit}`.
    ErsetzungOhneSicherung,
    /// `Ersetzung` mit Sicherung unter `{sicherung}`.
    ErsetzungGesichert,
    /// `Ersetzung` mit gekuerzter Sicherung; `{grenze}` in Bytes.
    ErsetzungGekuerzt,
    /// `Ersetzung` mit einer Sicherung aus einem frueheren Start.
    ErsetzungSchonVorhanden,
    /// `Ersetzung`, deren Sicherung gescheitert ist; `{fehler}`.
    ErsetzungSicherungGescheitert,
    /// `Schreibhindernis::Verweis` in der Statuszeile; `{zeile}` ist die
    /// Zeile zum Eintragen von Hand.
    EinstellungenVerweis,
    /// `Schreibhindernis::Beschaedigt`; `{befund}`.
    EinstellungenBeschaedigt,
    /// `Schreibhindernis::NichtLesbar`; `{befund}`.
    EinstellungenNichtLesbar,
    /// `Schreibhindernis::Intern`; `{befund}`.
    EinstellungenIntern,
    /// `Schreibhindernis::NichtGeschrieben`; `{befund}`.
    EinstellungenNichtGeschrieben,
    /// `notizordner_schreiben`: der Befund, wenn an der Stelle der Datei
    /// etwas anderes als eine gewoehnliche Datei steht.
    EinstellungenKeineGewoehnlicheDatei,
    /// `einstellungen::ortsstelle`: der Befund, wenn `notizordner` nicht als
    /// einzelner Wert dasteht.
    EinstellungenNotizordnerKeinEinzelwert,
    /// `einstellungen::pruefen`: der neue Wert steht nach der zweiten Lesung
    /// nicht da.
    EinstellungenNeuerWertFehlt,
    /// `einstellungen::pruefen`: ein anderer Wert haette sich geaendert.
    EinstellungenAndererWertGeaendert,
    /// `Werkshindernis::Verweis` in der Statuszeile; `{datei}` ist der
    /// Dateiname.
    WerksVerweis,
    /// `Werkshindernis::KeineDatei`; `{datei}`.
    WerksKeineDatei,
    /// `Werkshindernis::NichtLesbar`; `{datei}`, `{befund}`.
    WerksNichtLesbar,
    /// `Werkshindernis::KeineOrtszeit`.
    WerksKeineOrtszeit,
    /// `Werkshindernis::KeinFreierName`; `{datei}`.
    WerksKeinFreierName,
    /// `Werkshindernis::NichtBeiseitegelegt`; `{datei}`, `{befund}`.
    WerksNichtBeiseitegelegt,
    /// `Werkshindernis::NichtVorbereitet`; `{datei}`, `{befund}`.
    WerksNichtVorbereitet,
    /// `Werkshindernis::Einstellungen`; `{befund}` ist die Meldung des
    /// `Schreibhindernis`.
    WerksEinstellungen,
    /// `Werkshindernis::NichtZurueckgebaut`; `{hindernis}` ist die Meldung
    /// des ausloesenden Hindernisses, `{pfade}` die Liste der Sicherungen.
    WerksNichtZurueckgebaut,
    /// `Zurueckgesetzt::meldung`: der erste Satz, wenn jede Datei
    /// zurueckgesetzt ist.
    WerksZurueckgesetzt,
    /// `Zurueckgesetzt::meldung`: der erste Satz, wenn nicht jede Datei
    /// zurueckgesetzt ist.
    WerksTeilweiseZurueckgesetzt,
    /// `Zurueckgesetzt::meldung`: die Sicherungen `{pfade}`.
    WerksBeiseitegelegt,
    /// `Zurueckgesetzt::meldung`: `{datei}` stand vorher nicht da.
    WerksStandNichtDa,
    /// `Zurueckgesetzt::meldung`: `{datei}` ist nicht zurueckgesetzt;
    /// `{fehler}`.
    WerksDateiNichtZurueckgesetzt,
    /// `neuerungen::startzeile`: die eine Zeile beim ersten Start einer
    /// neuen Fassung; `{teile}` sind die Zahlen je Datei, `{ordner}` der
    /// Ablageordner.
    NeuerungenStartzeile,
    /// `neuerungen::blatttext`: der Absatz zu einer Datei, die nicht dasteht.
    NeuerungenDateiFehlt,
    /// `neuerungen::blatttext`: der Absatz zu einer beschaedigten Datei.
    NeuerungenDateiBeschaedigt,
    /// `neuerungen::blatttext`: die Ueberschrift der Hinrichtung.
    NeuerungenNeuInDieserFassung,
    /// `neuerungen::blatttext`: die Ueberschrift der Gegenrichtung.
    NeuerungenNurInIhrerDatei,
    /// `neuerungen::namenszeile`: eine Zeile ohne Namen; `{ueberschrift}`.
    NeuerungenZeileLeer,
    /// `neuerungen::namenszeile`: eine Zeile mit Namen; `{ueberschrift}`,
    /// `{namen}`.
    NeuerungenZeile,
    /// `neuerungen::gegenrichtung`: der Halbsatz hinter der leeren Zeile
    /// `{zeile}` einer Datei, die keine eigenen Eintraege fuehren kann.
    NeuerungenKeineEigenenEintraege,
    /// `neuerungen::preis` fuer `readers.toml`.
    NeuerungenPreisLeser,
    /// `neuerungen::preis` fuer `settings.toml`.
    NeuerungenPreisEinstellungen,
    /// `neuerungen::preis` fuer `keymap.toml`.
    NeuerungenPreisBelegung,
    /// `neuerungen::blatttext`: der Schlusssatz des Blattes.
    NeuerungenSchlusssatz,
    /// `Ortsfehler::KeinBenutzerverzeichnis` in der Statuszeile.
    OrtKeinBenutzerverzeichnis,
    /// `Ortsfehler::Leer`; `{ordnername}` ist der Name des Heimordners ab
    /// Werk.
    OrtLeer,
    /// `Ortsfehler::NichtAbsolut`; `{wert}`.
    OrtNichtAbsolut,
    /// `Ortsfehler::FremdesBenutzerverzeichnis`; `{wert}`.
    OrtFremdesBenutzerverzeichnis,
    /// `Ortsfehler::KeinText`; `{wert}` ist die TOML-Schreibweise des
    /// Werts, `{beispiel}` die eines gueltigen Orts.
    OrtKeinText,
    /// `Ortsfehler::ImAblageordner`; `{wert}`.
    OrtImAblageordner,
    /// `Ortsfehler::EinstellungenBeschaedigt`; `{satzteil}` ist ein
    /// `Ablagegrund…`.
    OrtEinstellungenBeschaedigt,
    /// `Ortsfehler::EinstellungenUngelesen`; `{ursache}`.
    OrtEinstellungenUngelesen,
    /// `ort::wechselsatz`: der Satz ueber einen Ortswechsel; `{neu}`,
    /// `{alt}`.
    OrtWechsel,
    /// `ort::abweisungssatz`: „Ort waehlen…“ verweigert sich, solange der
    /// Editor `{datei}` haelt.
    OrtAbweisung,
    /// `ort::wahlsatz` ohne alten Ort; `{neu}`.
    OrtGewaehlt,
    /// `ort::schon_der_ort`; `{neu}`.
    OrtSchonDerOrt,
    /// `Hindernis::KeinOrdner` in der Statuszeile nach F2; `{ordner}`.
    HeimKeinOrdner,
    /// `Hindernis::Unerreichbar`; `{ordner}`, `{grund}`.
    HeimNichtErreichbar,
    /// `Hindernis::NichtAnlegbar` und eine Eintragsdatei, die sich nicht
    /// anlegen liess; `{name}`, `{grund}`.
    HeimNichtAnlegbar,
    /// `Hindernis::ObererOrdnerFehlt`; `{ordner}`.
    HeimObererOrdnerFehlt,
    /// `Zettelmerker::NichtVermerkt`; `{grund}`, `{ort}`.
    HeimMerkerNichtVermerkt,
    /// `Zettelmerker::OhneAblage` nach einer Uebernahme; `{ort}`.
    HeimMerkerOhneAblage,
    /// `AlteGeheimnisse::Umbenannt`; `{alt}`, `{neu}`.
    HeimGeheimnisseUmbenannt,
    /// `AlteGeheimnisse::BeideStehen`; `{neu}`, `{alt}`, `{ort}`.
    HeimGeheimnisseBeideStehen,
    /// `AlteGeheimnisse::AlterNameBleibt`; `{alt}`, `{neu}`, `{grund}`.
    HeimGeheimnisseAlterNameBleibt,
    /// `AlteGeheimnisse::Gescheitert`; `{alt}`, `{neu}`, `{grund}`.
    HeimGeheimnisseNichtUmbenannt,
    /// `Uebernahmeausgang::NotizenStandenSchon`; `{notizen}`, `{dateien}`.
    HeimZettelNotizenStandenSchon,
    /// `Uebernahmeausgang::Gescheitert`; `{grund}`, `{dateien}`.
    HeimZettelGescheitert,
    /// `Zettelbefund::Themenzeile`; `{thema}`, `{datei}`.
    HeimZettelThemenzeile,
    /// `Zettelbefund::Unlesbar`; `{thema}`, `{grund}`, `{datei}`.
    HeimZettelUnlesbar,
    /// `zettel_lesen`: der Grund fuer einen zu grossen Zettel; `{groesse}` in
    /// Bytes.
    HeimZettelZuGross,
    /// `zettel_lesen`: der Grund fuer einen Zettel, der kein Text ist.
    HeimZettelKeinText,
    /// `exklusiv_anlegen`: die angefangene Datei blieb liegen; `{fehler}`,
    /// `{entfernen}`.
    HeimAngefangeneDateiBleibt,
    /// Das Bindewort zwischen zwei Gliedern einer Aufzaehlung („a und b“).
    Und,
    /// `Tresorfehler::KeinZufall` in der Statuszeile; `{grund}`.
    TresorKeinZufall,
    /// `Tresorfehler::Ableitung`; `{grund}`.
    TresorAbleitung,
    /// `Tresorfehler::Verschluesselung`.
    TresorVerschluesselung,
    /// `Oeffnungsfehler::PinFalschOderVeraendert`.
    TresorPinFalschOderVeraendert,
    /// `Oeffnungsfehler::KopfBeschaedigt`; `{grund}` ist einer der
    /// `TresorKopf…`-Eintraege.
    TresorKopfBeschaedigt,
    /// `Kopfschaden::FalscheKennung`.
    TresorKopfKennungFehlt,
    /// `Kopfschaden::Abgeschnitten`.
    TresorKopfAbgeschnitten,
    /// `Kopfschaden::UnbekannteVersion`; `{version}`.
    TresorKopfUnbekannteVersion,
    /// `Kopfschaden::UnbekannteAbleitung`; `{ableitung}`.
    TresorKopfUnbekannteAbleitung,
    /// `Kopfschaden::UngueltigeParameter`.
    TresorKopfUngueltigeParameter,
    /// `Zusammenfassung::als_text`: die zwei Kopfzeilen; `{name}`, `{pfad}`.
    ZusammenfassungKopf,
    /// `leseprofil::zeilen_als_text`: die Beschriftung einer Zeile, deren
    /// Wert darunter rutscht; `{beschriftung}`.
    ZusammenfassungBlockzeile,
    /// `leseprofil::zeilen_als_text`: eine Zeile aus Beschriftung und Wert;
    /// `{beschriftung}`, `{wert}`.
    ZusammenfassungZeile,
    /// `Wert::Nicht`: was an der Stelle eines Werts steht, ueber den nichts
    /// zu sagen ist (C3.12 der Runde 16).
    ZusammenfassungPlatzhalter,
    /// `Wert::UeberGrenze`; `{gezaehlt}` sind die Treffer, `{grenze}` die
    /// Lesegrenze.
    WertUeberGrenze,
    /// `Wert::Vorhanden(true)`.
    Ja,
    /// `Wert::Vorhanden(false)`.
    Nein,
    /// `leseprofil::datei::profilmeldung`: eine Meldung ueber ein Profil
    /// beim Start; `{profil}`, `{grund}`.
    ProfilMeldung,
    /// `leseprofil::datei::zeilenmeldung`; `{profil}`, `{beschriftung}`,
    /// `{grund}`.
    ProfilZeilenmeldung,
    /// `Zeilendatei::zerlegen`: eine Zeile mit mehr als einem Baustein;
    /// `{anzahl}`, `{namen}`.
    ProfilMehrereBausteine,
    /// `Zeilendatei::zerlegen`: eine Zeile ohne Baustein; `{namen}` sind die
    /// vier Tischnamen.
    ProfilKeinBaustein,
    /// `leseprofil::datei::pruefen`: ein Profil ohne Pfadmuster und ohne
    /// Kennzeichendatei.
    ProfilOhneErkennung,
    /// `leseprofil::datei::pruefen`: der Grund einer abgewiesenen Bildfolge;
    /// `{grund}`.
    ProfilBildfolge,
    /// `erkennungsmuster`: das erste der zwei Erkennungsmuster, als `{was}`
    /// in `ProfilErkennungsmusterNichtUebersetzt`.
    ProfilPfadmuster,
    /// `erkennungsmuster`: das zweite der zwei Erkennungsmuster.
    ProfilKennzeichendatei,
    /// `erkennungsmuster`: `{was}` `{muster}` uebersetzt nicht; `{grund}`.
    ProfilErkennungsmusterNichtUebersetzt,
    /// `leseprofil::datei::muster`: ein Bausteinmuster uebersetzt nicht;
    /// `{muster}`, `{grund}`.
    ProfilMusterNichtUebersetzt,
    /// `leseprofil::datei::feldmuster`; `{muster}`, `{gruppen}`.
    ProfilFeldmusterFanggruppen,
    /// `leseprofil::datei::ortsangabe`; `{angabe}`, `{mangel}` (ein
    /// `Ortsmangel…`).
    ProfilOrtsangabe,
    /// `ortsangabe_ohne_platzhalter`; `{angabe}`, `{baustein}`.
    ProfilOrtsangabeMitPlatzhalter,
    /// `gekappte_anzahl`: `juengste` mit `anzahl = 0`.
    ProfilJuengsteNull,
    /// `git::texte::kein_repository`: die Kopfzeile des Git-Bereichs in
    /// einem Ordner ohne Repository (A14 der Runde 23).
    GitKeinRepository,
    /// `git::texte::ohne_commit`: die Zusammenfassung, solange kein Commit
    /// da ist.
    GitOhneCommit,
    /// `git::texte::unveraendert`: die Zusammenfassung ohne Marke.
    GitUnveraendert,
    /// `git::texte::zusammenfassung`: die Zahlen `{marken}` mit dem Zusatz,
    /// dass der Satz den Ordner meint.
    GitImOrdner,
    /// `git::texte::kopfzeile` bei abgeloestem HEAD; `{kurzhash}`.
    GitKopfAbgeloest,
    /// `Schreibfehler::LeereTaste` in der Meldung zu einer Belegung.
    TasteNameFehlt,
    /// `Schreibfehler::UnbekannteZusatztaste`; `{text}`, `{erlaubt}`.
    TasteKeineZusatztaste,
    /// `Schreibfehler::FnAlsZusatztaste`.
    TasteFnKeineZusatztaste,
    /// `Schreibfehler::ZusatztasteDoppelt`; `{text}`.
    TasteZusatztasteDoppelt,
    /// `Schreibfehler::ReihenfolgeVerletzt`; `{zusatztaste}`, `{hinter}`,
    /// `{reihenfolge}`.
    TasteReihenfolgeVerletzt,
    /// `Schreibfehler::UnbekannterTastenname`; `{text}`.
    TasteUnbekannterName,
    /// `Funktionsname` in jeder Konfliktmeldung: der Name der Funktion mit
    /// ihrer Kennung; `{name}`, `{kennung}`.
    FunktionsnameMitKennung,
    /// `Konflikt`: eine Kombination, die zwei Funktionen beanspruchen;
    /// `{kombination}`, `{andere}`, `{bewerber}`.
    BelegungKonflikt,
    /// `Belegungsfehler::Schreibweise`; `{kennung}`, `{text}`, `{fehler}`.
    BelegungSchreibweise,
    /// `Belegungsfehler::UnbekannteFunktion` und
    /// `Zuweisungsfehler::UnbekannteFunktion`; `{kennung}`.
    BelegungUnbekannteFunktion,
    /// `Belegungsfehler::FunktionDoppelt`; `{kennung}`.
    BelegungFunktionDoppelt,
    /// Der Ueber-Sonderposten ganz oben im Anwendungsmenue.
    MenueUeberKrk,
    /// Der Markdown-Sonderposten ueber dem Beenden im Anwendungsmenue.
    MenueTastenbelegungAlsMarkdown,
    /// `Funktionsbereich::Anwendung`: der Titel des Obermenues, der
    /// Gruppenueberschrift in der F1-Ansicht und des Abschnitts der
    /// Markdown-Ausgabe.
    FunktionsbereichAnwendung,
    /// `Funktionsbereich::Home`.
    FunktionsbereichHome,
    /// `Funktionsbereich::Dateilisting`.
    FunktionsbereichDateilisting,
    /// `Funktionsbereich::Dateioperationen`.
    FunktionsbereichDateioperationen,
    /// `Funktionsbereich::Tabs`.
    FunktionsbereichTabs,
    /// `Funktionsbereich::Vorschau`.
    FunktionsbereichVorschau,
    /// `Funktionsbereich::LeisteUndFokus`.
    FunktionsbereichLeisteUndFokus,
    /// `Funktionsbereich::Editor`.
    FunktionsbereichEditor,
    /// `Funktionsbereich::Git`.
    FunktionsbereichGit,
    /// `Funktionsbereich::Textbefehle`: das Menue heisst nach der
    /// Mac-Gewohnheit „Bearbeiten“ und nicht nach der Variante.
    FunktionsbereichTextbefehle,
    /// `Funktionsbereich::Fenster`.
    FunktionsbereichFenster,
    /// `Kontextbefehl::OeffnenMit` im Kontextmenue der Dateiliste.
    KontextOeffnenMit,
    /// `Kontextbefehl::Zippen`.
    KontextZippen,
    /// `Kontextbefehl::Entpacken`.
    KontextEntpacken,
    /// `Kontextbefehl::Duplizieren`.
    KontextDuplizieren,
    /// `Kontextbefehl::ImFinderOeffnen`.
    KontextImFinderOeffnen,
    /// `Kontextbefehl::ImFinderAnzeigen`.
    KontextImFinderAnzeigen,
    /// `Bereich::Lesezeichen`: die Aufschrift des Schalters in der
    /// Bereichsleiste.
    BereichLesezeichen,
    /// `Bereich::Links`.
    BereichLinks,
    /// `Bereich::Rechts`.
    BereichRechts,
    /// `Bereich::Vorschau`.
    BereichVorschau,
    /// `Bereich::Editor`.
    BereichEditor,
    /// `Bereich::Git`.
    BereichGit,
    /// `Bereich::Lesezeichen`: der ausgeschriebene Name im Hinweistext des
    /// Schalters, als `{bereich}` in `LeisteBereichUmschalten`; Franzoesisch
    /// und Englisch tragen den Artikel mit, weil der Satz ihn braucht.
    BereichLesezeichenLang,
    /// `Bereich::Links`, ausgeschrieben.
    BereichLinksLang,
    /// `Bereich::Rechts`, ausgeschrieben.
    BereichRechtsLang,
    /// `Bereich::Vorschau`, ausgeschrieben.
    BereichVorschauLang,
    /// `Bereich::Editor`, ausgeschrieben.
    BereichEditorLang,
    /// `Bereich::Git`, ausgeschrieben.
    BereichGitLang,
    /// `Spalte::Name`: die Ueberschrift der Spalte und die Aufschrift ihres
    /// Schalters.
    SpalteName,
    /// `Spalte::Groesse`.
    SpalteGroesse,
    /// `Spalte::Geaendert`: der kurze Name auf dem Schalter der Bereichsleiste.
    SpalteDatum,
    /// `Spalte::Typ`.
    SpalteTyp,
    /// `Spalte::Marke`.
    SpalteMarke,
    /// `Spalte::Geaendert`: die Ueberschrift ueber der Spalte, die anders
    /// heisst als der Schalter.
    SpalteAenderungsdatum,
    /// Der Hinweistext eines Bereichsschalters; `{bereich}` ist ein
    /// `Bereich…Lang`.
    LeisteBereichUmschalten,
    /// Der Hinweistext eines Spaltenschalters; `{spalte}` ist eine
    /// `Spalte…`-Aufschrift.
    LeisteSpalteUmschalten,
    /// Der Hinweistext des Schalters „Deep“.
    LeisteTiefeHinweis,
    /// Der Hinweistext des Schalters „Content“.
    LeisteInhaltHinweis,
    /// `Teil::Lesezeichen`: die Ueberschrift des oberen Teils der Lesezeichen-
    /// und Geraeteleiste.
    LeisteUeberschriftLesezeichen,
    /// `Teil::Geraete`: die Ueberschrift des unteren Teils.
    LeisteUeberschriftGeraete,
    /// Ein Lesezeichen `{name}`, dessen Ziel fehlt, in der Leiste.
    LeisteLesezeichenFehlt,
    /// `Sinnbild::Textstelle`: die Beschreibung des Sinnbilds fuer VoiceOver;
    /// die des Ordners ist `TypOrdner`.
    LeisteSinnbildTextstelle,
    /// `Typ::Ordner` in der Metadatenanzeige der Vorschau und als
    /// Beschreibung des Ordner-Sinnbilds der Leiste.
    TypOrdner,
    /// `Typ::Datei`.
    TypDatei,
    /// `Typ::Verknuepfung`.
    TypVerknuepfung,
    /// Die Spalte der Aufgabentabelle im Editor.
    EintragsspalteAufgabe,
    /// Die erste Spalte der Notiztabelle.
    EintragsspalteThema,
    /// Die zweite Spalte der Notiztabelle.
    EintragsspalteNotiz,
    /// Die erste Spalte der Termintabelle.
    EintragsspalteDatum,
    /// Die zweite Spalte der Termintabelle.
    EintragsspalteTermin,
    /// `Belegungsmodell::funktionstext`: wofuer `reserviert_fuer = "editor"`
    /// steht, als `{wofuer}` in `BelegungZusatzReserviert`.
    BelegungReserviertEditor,
    /// `Belegungsmodell::funktionstext`: der Zusatz hinter einer reservierten
    /// Funktion; `{wofuer}`.
    BelegungZusatzReserviert,
    /// `Belegungsmodell::funktionstext`: wofuer `gehalten_von = "menue"`
    /// steht, als `{zusteller}` in `BelegungZusatzZusteller`.
    BelegungZustellerMenue,
    /// `Belegungsmodell::funktionstext`: der Zusatz hinter einer zugestellten
    /// Funktion; `{zusteller}`.
    BelegungZusatzZusteller,
    /// `Belegungsmodell::zuweisen`: der Grund, wenn die Zeile keine Funktion
    /// traegt.
    BelegungKeineFunktionGewaehlt,
    /// `Suchlage::meldung` ohne Suchtext.
    BelegungSucheLeer,
    /// `Suchlage::meldung` mit Treffer; `{text}`, `{stelle}`, `{anzahl}`.
    BelegungSucheTreffer,
    /// `Suchlage::meldung` ohne Treffer; `{text}`.
    BelegungSucheKeinTreffer,
    /// Der Titel des Blattes der F1-Ansicht.
    BelegungsansichtTitel,
    /// Die Schaltflaeche „Zuweisen“ der F1-Ansicht.
    BelegungsansichtZuweisen,
    /// Die Schaltflaeche „Auslieferungszustand“.
    BelegungsansichtAuslieferungszustand,
    /// Die Schaltflaeche „Fertig“.
    BelegungsansichtFertig,
    /// Der Name der Eingabetaste, wie die Erlaeuterungszeile ihn hinter
    /// `Cmd+` nennt.
    BelegungsansichtTasteEingabe,
    /// Der Satz unter der Ueberschrift des Blattes; `{zuweisen}`,
    /// `{zuweisen_taste}`, `{zuruecksetzen}`, `{zuruecksetzen_taste}`,
    /// `{fertig}`, `{fertig_taste}`.
    BelegungsansichtErlaeuterung,
    /// Die Meldung, wenn Zuweisen ohne gewaehlte Funktion gedrueckt wird.
    BelegungsansichtErstWaehlen,
    /// Die Meldung waehrend der Aufnahme; `{name}`.
    BelegungsansichtAufnahme,
    /// Die Meldung nach dem Zuruecksetzen.
    BelegungsansichtZurueckgesetzt,
    /// Die Meldung nach `esc` waehrend der Aufnahme.
    BelegungsansichtAufnahmeAbgebrochen,
    /// Die Meldung, wenn die Aufnahme ohne gewaehlte Funktion endet.
    BelegungsansichtKeineFunktion,
    /// Die Meldung nach einer Zuweisung; `{funktion}`, `{kombination}`.
    BelegungsansichtZugewiesen,
    /// Die Meldung zu einer Taste ohne Namen in der Kombinationsschreibweise.
    BelegungsansichtTasteOhneNamen,
    /// Die erste Spalte der F1-Ansicht.
    BelegungsansichtSpalteFunktion,
    /// Die zweite Spalte der F1-Ansicht.
    BelegungsansichtSpalteBelegung,
    /// Die Ueberschrift der Markdown-Ausgabe der Tastenbelegung.
    MarkdownUeberschrift,
    /// Die Kopfzeile jeder Tabelle der Markdown-Ausgabe.
    MarkdownTabellenkopf,
    /// Die dritte Zelle einer Funktion, die KRK nicht einordnen kann.
    MarkdownNichtEingeordnet,
    /// Die dritte Zelle der drei Zwischenablage-Befehle.
    MarkdownWirktTextfelderUndEditor,
    /// `Ausgang::Geschrieben`; `{pfad}`.
    MarkdownGeschrieben,
    /// `Ausgang::KeinBenutzerverzeichnis`.
    MarkdownKeinBenutzerverzeichnis,
    /// `Ausgang::OrdnerFehlt`; `{pfad}`.
    MarkdownOrdnerFehlt,
    /// `Ausgang::ZugriffAbgelehnt`; `{pfad}`.
    MarkdownZugriffAbgelehnt,
    /// `Ausgang::Fehlgeschlagen`; `{pfad}`, `{grund}`.
    MarkdownFehlgeschlagen,
    /// `Abwurfgrund::KeineDatei`: die eine Meldung des Abwurfs in der
    /// Statuszeile.
    TabelleKeineDateiAufDatentraeger,
    /// `in_zeile_einsteigen`: eine Verknuepfung mit unerreichbarem Ziel;
    /// `{pfad}`, `{grund}`.
    TabelleNichtZuOeffnen,
    /// `zwischenablage_springen` ohne Inhalt in der Zwischenablage.
    TabelleZwischenablageLeer,
    /// `zwischenablage_springen`: der Systembrowser nimmt `{adresse}` nicht.
    TabelleNichtAnBrowser,
    /// `zwischenablage_springen`: die Zwischenablage traegt kein Ziel.
    TabelleZwischenablageKeinZiel,
    /// `eintrag_anspringen`: `{name}` steht nicht in der gelesenen Liste.
    TabelleNichtInDerListe,
    /// Der Fenstertitel, solange der Editor die Quicknote zeigt.
    FenstertitelQuicknote,
    /// `statuszeile::filterstand_text`: der Satz des Filterstands;
    /// `{filtertext}`, `{gezeigt}`, `{vorhanden}` und die drei Satzteile
    /// `{liest}`, `{zu_gross}`, `{ausgeblendet}`, die leer bleiben duerfen.
    StatuszeileFilterstand,
    /// `filterstand_text`: der Satzteil `{liest}`, solange der Inhalt gelesen
    /// wird; er beginnt mit dem Komma, das ihn anhaengt.
    StatuszeileInhaltWirdGelesen,
    /// `statuszeile::seitenzaehler_text`; `{aktuell}`, `{gesamt}`.
    StatuszeileSeiteVon,
    /// `statuszeile::bildzaehler_text`; `{aktuell}`, `{gesamt}`.
    StatuszeileBildVon,
    /// `bildzaehler_text` bei gekuerzter Folge; `{grundsatz}` ist der
    /// `StatuszeileBildVon`, `{gruende}` die Grenzen, mit `Und` verbunden.
    StatuszeileFolgeGekuerzt,
    /// `statuszeile::seitenname` fuer `Fensterseite::Links`.
    StatuszeileLinkesDateifenster,
    /// `Fensterseite::Rechts`.
    StatuszeileRechtesDateifenster,
    /// `statuszeile::zeilentext`: die Meldung einer inaktiven Seite mit ihrem
    /// Namen davor; `{seite}`, `{text}`.
    StatuszeileMeldungMitSeite,
    /// `operationen::ueberschrift` fuer `Art::Kopieren`: womit die Statuszeile
    /// einen laufenden Vorgang benennt.
    VorgangsartKopieren,
    /// `Art::Verschieben`.
    VorgangsartVerschieben,
    /// `Art::InDenPapierkorb`.
    VorgangsartInDenPapierkorb,
    /// `Art::UmbenennenImStapel`.
    VorgangsartUmbenennen,
    /// `Art::Zippen`.
    VorgangsartPacken,
    /// `Art::Entpacken`.
    VorgangsartEntpacken,
    /// `Art::Duplizieren`.
    VorgangsartDuplizieren,
    /// `operationen::vorgangszeile`: der Hinweis auf `esc` am Ende der
    /// Zeile, als `{abbruch}`; der Tastenname bleibt in jeder Sprache.
    VorgangAbbruchhinweis,
    /// `vorgangszeile` ohne Fortschritt; `{was}`, `{positionen}`,
    /// `{abbruch}`. Der Mittelpunkt trennt die Angaben, weil die Statuszeile
    /// einzeilig ist und ein Umbruch dort abgeschnitten wuerde.
    VorgangWirdVorbereitet,
    /// `vorgangszeile` mit Fortschritt; `{was}`, `{eintraege}`, `{menge}`,
    /// `{positionen}`, `{name}`, `{abbruch}`.
    VorgangZeile,
    /// `operationen::abbruchzeile`; `{was}`.
    VorgangWirdAbgebrochen,
    /// `operationen::schon_ein_vorgang`; `{was}`.
    VorgangSchonEiner,
    /// `operationen::abschlusstext`: die Angabe des Uebertragenen;
    /// `{eintraege}`, `{menge}`, `{positionen}`.
    VorgangUebertragen,
    /// `abschlusstext` nach `Abschluss::Abgebrochen`; `{was}`,
    /// `{uebertragen}`.
    VorgangAbgebrochen,
    /// `abschlusstext` nach `Abschluss::Fertig`; `{was}`, `{uebertragen}`.
    VorgangFertig,
    /// `operationen::uebersprungenliste`: eine Zeile der Liste; `{name}`,
    /// `{grund}`.
    UebersprungenZeile,
    /// `Anlegeart::frage` fuer `Ordner`: die Kopfzeile des Eingabeblattes.
    AnlegenFrageOrdner,
    /// `Anlegeart::frage` fuer `Datei`.
    AnlegenFrageDatei,
    /// `Anlegeart::bestaetigen`: die bestaetigende Schaltflaeche.
    AnlegenBestaetigen,
    /// `operationen::angelegt_text` fuer einen Ordner; `{name}`.
    AngelegtOrdner,
    /// `angelegt_text` fuer eine Datei; `{name}`.
    AngelegtDatei,
    /// `operationen::anlegefehler` bei `PermissionDenied` fuer einen Ordner;
    /// `{name}`.
    AnlegenKeineRechteOrdner,
    /// `anlegefehler` bei `PermissionDenied` fuer eine Datei; `{name}`.
    AnlegenKeineRechteDatei,
    /// `anlegefehler` fuer jeden anderen Fehler; `{name}`, `{fehler}`.
    AnlegenGescheitert,
    /// `operationen::schon_vergeben`: derselbe Satz beim Anlegen, Umbenennen
    /// und Duplizieren; `{name}`.
    NameSchonVergeben,
    /// `operationen::duplikatfrage`: die Kopfzeile des Namensblatts.
    DuplikatFrage,
    /// `operationen::duplikat_bestaetigen`: die bestaetigende Schaltflaeche.
    DuplikatBestaetigen,
    /// `operationen::mehrere_zu_duplizieren`.
    DuplikatMehrere,
    /// `operationen::nicht_zu_duplizieren`; `{name}`, `{typ}` (ein
    /// `DuplikatTyp…`).
    DuplikatNichtGewoehnlich,
    /// `ordner_nicht_zu_duplizieren`: der Typ als `{typ}`.
    DuplikatTypOrdner,
    /// `verknuepfung_nicht_zu_duplizieren`: der Typ als `{typ}`.
    DuplikatTypVerknuepfung,
    /// `operationen::umbenennungsfehler` bei `PermissionDenied`; `{name}`.
    UmbenennenKeineRechte,
    /// `umbenennungsfehler` fuer jeden anderen Fehler; `{name}`, `{fehler}`.
    UmbenennenGescheitert,
    /// `operationen::ordner_fehlt`: an der Stelle steht kein Ordner mehr;
    /// `{pfad}`.
    OrdnerKeinOrdnerMehr,
    /// `ordner_fehlt`: der Ordner ist unerreichbar; `{pfad}`, `{fehler}`.
    OrdnerNichtMehrErreichbar,
    /// `operationen::kein_terminal`; `{kennung}`.
    KeinTerminal,
    /// `operationen::kopiermeldung` fuer einen Pfad; `{pfad}`.
    PfadKopiert,
    /// `kopiermeldung` fuer mehrere Pfade; `{n}`.
    PfadeKopiert,
    /// `operationen::nichts_betroffen`; `{nennform}` ist eine der
    /// `Nennform…`.
    NichtsBetroffen,
    /// `nichts_zu_kopieren`: die Nennform in `NichtsBetroffen`.
    NennformZuKopieren,
    /// `nichts_zu_oeffnen`.
    NennformZuOeffnen,
    /// `nichts_zu_packen`.
    NennformZuPacken,
    /// `nichts_anzuzeigen`.
    NennformAnzuzeigen,
    /// `nichts_zu_duplizieren`.
    NennformZuDuplizieren,
    /// `operationen::nichts_zu_teilen`.
    NichtsZuTeilen,
    /// `operationen::kein_archiv`.
    KeinArchiv,
    /// `operationen::mehrere_archive`.
    MehrereArchive,
    /// `operationen::kein_finder`.
    KeinFinder,
    /// `operationen::ablage_weist_ab`.
    AblageWeistTextAb,
    /// `operationen::ablagemeldung` fuer einen Eintrag; `{name}`.
    AbgelegtEiner,
    /// `ablagemeldung` fuer mehrere Eintraege; `{n}`.
    AbgelegtMehrere,
    /// `ablagemeldung` nach `Dateiablage::Ausschneiden`; `{kopiert}` ist
    /// einer der zwei Saetze davor.
    AbgelegtAusgeschnitten,
    /// `operationen::verweise_abgewiesen`.
    AblageWeistVerweiseAb,
    /// `Einfuegehindernis::KeinText` in `operationen::einfuegen_abgewiesen`.
    EinfuegenKeinText,
    /// `Einfuegehindernis::Mehrzeilig`.
    EinfuegenMehrzeilig,
    /// `Einfuegehindernis::NichtsTragbar`.
    EinfuegenNichtsTragbar,
    /// `operationen::oeffnungsmeldung`: ein Eintrag uebergeben; `{name}`.
    UebergebenEiner,
    /// `oeffnungsmeldung`: mehrere uebergeben; `{n}`.
    UebergebenMehrere,
    /// `oeffnungsmeldung`: ein Eintrag abgewiesen; `{name}`.
    NichtAngenommenEiner,
    /// `oeffnungsmeldung`: mehrere abgewiesen; `{n}`, `{gesamt}`.
    NichtAngenommenMehrere,
    /// `oeffnungsmeldung`: beide Haelften; `{genommen}`, `{abgelehnt}`.
    UebergebenUndAbgelehnt,
    /// `operationen::keine_anwendung`.
    KeineAnwendung,
    /// `operationen::oeffnungsmeldung_an`: ein Eintrag; `{anwendung}`,
    /// `{name}`.
    UebergebenAnEiner,
    /// `oeffnungsmeldung_an`: mehrere; `{n}`, `{anwendung}`.
    UebergebenAnMehrere,
    /// `operationen::nicht_uebergeben`; `{anwendung}`.
    NichtUebergebenAn,
    /// `operationen::belegungsdatei_hat_zwei_schreiber`; `{datei}`.
    BelegungsdateiZweiSchreiber,
    /// `operationen::keine_belegungsdatei`; `{datei}`.
    BelegungsdateiFehltNoch,
    /// `operationen::belegungsdatei_ohne_ablageordner`; `{datei}`.
    BelegungsdateiOhneAblageordner,
    /// `auswahl::markierungsstand_text`; `{n}`, `{ordner}` (ein
    /// `Zahlwort::Ordner`), `{groesse}`.
    Markierungsstand,
    /// `loeschwarnung::ohne_papierkorb`.
    LoeschenOhnePapierkorb,
    /// `Warngrund::Unentscheidbar`: die Fuegung in der Loeschfrage und in der
    /// Erlaeuterung.
    WarngrundUnentscheidbar,
    /// `Warngrund::Netzlaufwerk`.
    WarngrundNetzlaufwerk,
    /// `Warngrund::Cloudort`.
    WarngrundCloudort,
    /// `Warngrund::AusserhalbBenutzerordner`.
    WarngrundAusserhalbBenutzerordner,
    /// `Warngrund::ImBenutzerordner`.
    WarngrundImBenutzerordner,
    /// `Warngrund::Arbeitsbaum`.
    WarngrundArbeitsbaum,
    /// `Warngrund::Umfang(GenauDieSchwelle)`; nennt die Zahl aus
    /// `loeschwarnung::SCHWELLE`, und eine Zusicherung beim Uebersetzen haelt
    /// beide in jeder Sprache aneinander.
    WarngrundGenauDieSchwelle,
    /// `Warngrund::Umfang(MehrAlsDieSchwelle)`; ebenso.
    WarngrundMehrAlsDieSchwelle,
    /// `loeschwarnung::frage_und_erlaeuterung`: der erste Satz der
    /// Erlaeuterung; `{ordner}`.
    LoeschenGeraeumtAus,
    /// `frage_und_erlaeuterung`: der Absatz mit den uebrigen Gruenden;
    /// `{gruende}`.
    LoeschenAusserdem,
    /// `frage_und_erlaeuterung`: der Absatz zu den Ordnern; `{ordner}` (ein
    /// `Zahlwort::Ordner`).
    LoeschenDarunterOrdner,
    /// `blattmeldung::satz`: die abgewiesene Taste waehrend eines Blattes.
    BlattSteht,
    /// `pfadeingabe::pruefen`; `{pfad}`.
    PfadNichtAbsolut,
    /// `pfadeingabe::pruefen`; `{pfad}`, `{fehler}`.
    PfadGibtEsNicht,
    /// `pfadeingabe::pruefen`; `{pfad}`, `{fehler}`.
    PfadNichtLesbar,
    /// `pfadeingabe::pruefen`; `{pfad}`.
    PfadInKeinemOrdner,
    /// `werkseinstellungen::schaltflaeche`: die ausloesende Schaltflaeche.
    WerksSchaltflaeche,
    /// `werkseinstellungen::rueckfrage`: der Satz, dass der Notizordner
    /// bleibt, als `{notizordner}` in `WerksErlaeuterung`.
    WerksNotizordnerBleibt,
    /// `werkseinstellungen::rueckfrage`: die Frage.
    WerksFrage,
    /// `werkseinstellungen::rueckfrage`: die Erlaeuterung; `{notizordner}`.
    WerksErlaeuterung,
    /// `werkseinstellungen::rueckfrage`: der Absatz bei eigener
    /// `keymap.toml`.
    WerksEigeneZuweisungen,
    /// `tabs::wunschmeldung` fuer `Wunschausgang::Ausgefiltert`; `{name}`.
    TabAusgefiltert,
    /// `tabs::wunschmeldung` fuer `Wunschausgang::Fehlt`; `{name}`.
    TabNichtMehrDa,
    /// `tabs::lesemeldungen_einziehen`: ein Lesevorgang mit Fehler;
    /// `{ordner}`, `{fehler}`.
    TabNichtVollstaendigGelesen,
    /// `Systempapierkorb::in_den_papierkorb`; `{pfad}`.
    PapierkorbKeinUtf8Pfad,
    /// Die Schaltflaeche, die ein meldendes Blatt schliesst: die
    /// Startmeldungen und die Abschlussliste.
    BlattSchliessen,
}

impl Text {
    /// Alle Schluessel, in der Reihenfolge der Aufzaehlung.
    pub const ALLE: [Text; 404] = [
        Text::WirkungsbereichDateifenster,
        Text::WirkungsbereichLeiste,
        Text::WirkungsbereichDateibereiche,
        Text::WirkungsbereichEditor,
        Text::WirkungsbereichEditortext,
        Text::WirkungsbereichEintraege,
        Text::WirkungsbereichReihenfolge,
        Text::WirkungsbereichAufgaben,
        Text::WirkungsbereichTermine,
        Text::WirkungsbereichGeheimnisse,
        Text::WirkungsbereichQuicknote,
        Text::WirkungsbereichTabbereich,
        Text::WirkungsbereichNavigator,
        Text::WirkungsbereichVorschau,
        Text::WirkungsbereichBildfolge,
        Text::WirkungsbereichUeberall,
        Text::OrtsmangelAbsolut,
        Text::OrtsmangelLeeresStueck,
        Text::OrtsmangelPunktstueck,
        Text::OrtsmangelMehrerePlatzhalter,
        Text::NameLeer,
        Text::NameMitSchraegstrich,
        Text::NameMitNullbyte,
        Text::NamePunktname,
        Text::KollisionBestehender,
        Text::KollisionDoppelt,
        Text::AblagegrundNichtLesbar,
        Text::AblagegrundBeschaedigt,
        Text::AblagegrundNichtAnlegbar,
        Text::ErsatzAuslieferungszustand,
        Text::ErsatzNichts,
        Text::PinKeineVierZiffern,
        Text::AbweisungUmbruchImAufgabentext,
        Text::AbweisungUmbruchImThema,
        Text::AbweisungThemenzeileImNotiztext,
        Text::AbweisungUngueltigesDatum,
        Text::AbweisungKopfzeileImTermintext,
        Text::ZaehlzeileDateien,
        Text::ZaehlzeileOrdner,
        Text::ZaehlzeileVerknuepfungen,
        Text::EinheitKilobyte,
        Text::EinheitMegabyte,
        Text::EinheitGigabyte,
        Text::EinheitTerabyte,
        Text::VorgangKeineRechte,
        Text::VorgangGibtEsNichtMehr,
        Text::VorgangAmZielStehtEintrag,
        Text::VorgangKeinPlatzAufDemDatentraeger,
        Text::VorgangKeinArbeitsfaden,
        Text::VorgangNeuerNameFehlt,
        Text::VorgangZielordnerFehlt,
        Text::VorgangPackenNichtQuelleFuerQuelle,
        Text::VorgangPfadBenenntKeinenEintrag,
        Text::VorgangQuelleUndZielDerselbeEintrag,
        Text::VorgangZielLiegtInDerQuelle,
        Text::VorgangZielNichtErsetzt,
        Text::VorgangNachAbbruchNichtWeggeraeumt,
        Text::VorgangOrdnerangabenNichtKopiert,
        Text::VorgangOrdnerSelbstBlieb,
        Text::VorgangNichtVollstaendigKopiert,
        Text::VorgangKopiertAberInQuelleGeblieben,
        Text::VorgangKeinPapierkorb,
        Text::VorgangKeineGewoehnlicheDatei,
        Text::VorgangZielNichtInPapierkorb,
        Text::EntpackenEintragFuehrtHeraus,
        Text::EntpackenEintragMitGrund,
        Text::EntpackenAmZielStehtVerknuepfung,
        Text::EntpackenWegMitUnzulaessigemBestandteil,
        Text::EntpackenWegDurchVerknuepfung,
        Text::EntpackenDateiStattOrdnerAufDemWeg,
        Text::EntpackenVerweiszielKeinText,
        Text::PackenArchivUnfertig,
        Text::PackenKeinPlatzImArchiv,
        Text::PackenHalberEintragImArchiv,
        Text::PackenNichtInsArchivGeschrieben,
        Text::StapelKeinStartwert,
        Text::StapelKeineStellenzahl,
        Text::EditorKeinFreierDateizugriff,
        Text::EditorNichtZuOeffnen,
        Text::EditorZuGross,
        Text::EditorKeineTextdatei,
        Text::EditorOrdnerHatKeinenText,
        Text::EditorKeineGewoehnlicheDatei,
        Text::LesenDatenschutzsperre,
        Text::LesenKeinVerzeichnis,
        Text::LesenPfadMitNullbyte,
        Text::AblageKeinGueltigesUtf8,
        Text::AblageOhneOberstenSchluessel,
        Text::AblageKeinBenutzerverzeichnis,
        Text::AtomarOhneDateinamen,
        Text::AtomarRechteNichtUebertragen,
        Text::ErsetzungOhneSicherung,
        Text::ErsetzungGesichert,
        Text::ErsetzungGekuerzt,
        Text::ErsetzungSchonVorhanden,
        Text::ErsetzungSicherungGescheitert,
        Text::EinstellungenVerweis,
        Text::EinstellungenBeschaedigt,
        Text::EinstellungenNichtLesbar,
        Text::EinstellungenIntern,
        Text::EinstellungenNichtGeschrieben,
        Text::EinstellungenKeineGewoehnlicheDatei,
        Text::EinstellungenNotizordnerKeinEinzelwert,
        Text::EinstellungenNeuerWertFehlt,
        Text::EinstellungenAndererWertGeaendert,
        Text::WerksVerweis,
        Text::WerksKeineDatei,
        Text::WerksNichtLesbar,
        Text::WerksKeineOrtszeit,
        Text::WerksKeinFreierName,
        Text::WerksNichtBeiseitegelegt,
        Text::WerksNichtVorbereitet,
        Text::WerksEinstellungen,
        Text::WerksNichtZurueckgebaut,
        Text::WerksZurueckgesetzt,
        Text::WerksTeilweiseZurueckgesetzt,
        Text::WerksBeiseitegelegt,
        Text::WerksStandNichtDa,
        Text::WerksDateiNichtZurueckgesetzt,
        Text::NeuerungenStartzeile,
        Text::NeuerungenDateiFehlt,
        Text::NeuerungenDateiBeschaedigt,
        Text::NeuerungenNeuInDieserFassung,
        Text::NeuerungenNurInIhrerDatei,
        Text::NeuerungenZeileLeer,
        Text::NeuerungenZeile,
        Text::NeuerungenKeineEigenenEintraege,
        Text::NeuerungenPreisLeser,
        Text::NeuerungenPreisEinstellungen,
        Text::NeuerungenPreisBelegung,
        Text::NeuerungenSchlusssatz,
        Text::OrtKeinBenutzerverzeichnis,
        Text::OrtLeer,
        Text::OrtNichtAbsolut,
        Text::OrtFremdesBenutzerverzeichnis,
        Text::OrtKeinText,
        Text::OrtImAblageordner,
        Text::OrtEinstellungenBeschaedigt,
        Text::OrtEinstellungenUngelesen,
        Text::OrtWechsel,
        Text::OrtAbweisung,
        Text::OrtGewaehlt,
        Text::OrtSchonDerOrt,
        Text::HeimKeinOrdner,
        Text::HeimNichtErreichbar,
        Text::HeimNichtAnlegbar,
        Text::HeimObererOrdnerFehlt,
        Text::HeimMerkerNichtVermerkt,
        Text::HeimMerkerOhneAblage,
        Text::HeimGeheimnisseUmbenannt,
        Text::HeimGeheimnisseBeideStehen,
        Text::HeimGeheimnisseAlterNameBleibt,
        Text::HeimGeheimnisseNichtUmbenannt,
        Text::HeimZettelNotizenStandenSchon,
        Text::HeimZettelGescheitert,
        Text::HeimZettelThemenzeile,
        Text::HeimZettelUnlesbar,
        Text::HeimZettelZuGross,
        Text::HeimZettelKeinText,
        Text::HeimAngefangeneDateiBleibt,
        Text::Und,
        Text::TresorKeinZufall,
        Text::TresorAbleitung,
        Text::TresorVerschluesselung,
        Text::TresorPinFalschOderVeraendert,
        Text::TresorKopfBeschaedigt,
        Text::TresorKopfKennungFehlt,
        Text::TresorKopfAbgeschnitten,
        Text::TresorKopfUnbekannteVersion,
        Text::TresorKopfUnbekannteAbleitung,
        Text::TresorKopfUngueltigeParameter,
        Text::ZusammenfassungKopf,
        Text::ZusammenfassungBlockzeile,
        Text::ZusammenfassungZeile,
        Text::ZusammenfassungPlatzhalter,
        Text::WertUeberGrenze,
        Text::Ja,
        Text::Nein,
        Text::ProfilMeldung,
        Text::ProfilZeilenmeldung,
        Text::ProfilMehrereBausteine,
        Text::ProfilKeinBaustein,
        Text::ProfilOhneErkennung,
        Text::ProfilBildfolge,
        Text::ProfilPfadmuster,
        Text::ProfilKennzeichendatei,
        Text::ProfilErkennungsmusterNichtUebersetzt,
        Text::ProfilMusterNichtUebersetzt,
        Text::ProfilFeldmusterFanggruppen,
        Text::ProfilOrtsangabe,
        Text::ProfilOrtsangabeMitPlatzhalter,
        Text::ProfilJuengsteNull,
        Text::GitKeinRepository,
        Text::GitOhneCommit,
        Text::GitUnveraendert,
        Text::GitImOrdner,
        Text::GitKopfAbgeloest,
        Text::TasteNameFehlt,
        Text::TasteKeineZusatztaste,
        Text::TasteFnKeineZusatztaste,
        Text::TasteZusatztasteDoppelt,
        Text::TasteReihenfolgeVerletzt,
        Text::TasteUnbekannterName,
        Text::FunktionsnameMitKennung,
        Text::BelegungKonflikt,
        Text::BelegungSchreibweise,
        Text::BelegungUnbekannteFunktion,
        Text::BelegungFunktionDoppelt,
        Text::MenueUeberKrk,
        Text::MenueTastenbelegungAlsMarkdown,
        Text::FunktionsbereichAnwendung,
        Text::FunktionsbereichHome,
        Text::FunktionsbereichDateilisting,
        Text::FunktionsbereichDateioperationen,
        Text::FunktionsbereichTabs,
        Text::FunktionsbereichVorschau,
        Text::FunktionsbereichLeisteUndFokus,
        Text::FunktionsbereichEditor,
        Text::FunktionsbereichGit,
        Text::FunktionsbereichTextbefehle,
        Text::FunktionsbereichFenster,
        Text::KontextOeffnenMit,
        Text::KontextZippen,
        Text::KontextEntpacken,
        Text::KontextDuplizieren,
        Text::KontextImFinderOeffnen,
        Text::KontextImFinderAnzeigen,
        Text::BereichLesezeichen,
        Text::BereichLinks,
        Text::BereichRechts,
        Text::BereichVorschau,
        Text::BereichEditor,
        Text::BereichGit,
        Text::BereichLesezeichenLang,
        Text::BereichLinksLang,
        Text::BereichRechtsLang,
        Text::BereichVorschauLang,
        Text::BereichEditorLang,
        Text::BereichGitLang,
        Text::SpalteName,
        Text::SpalteGroesse,
        Text::SpalteDatum,
        Text::SpalteTyp,
        Text::SpalteMarke,
        Text::SpalteAenderungsdatum,
        Text::LeisteBereichUmschalten,
        Text::LeisteSpalteUmschalten,
        Text::LeisteTiefeHinweis,
        Text::LeisteInhaltHinweis,
        Text::LeisteUeberschriftLesezeichen,
        Text::LeisteUeberschriftGeraete,
        Text::LeisteLesezeichenFehlt,
        Text::LeisteSinnbildTextstelle,
        Text::TypOrdner,
        Text::TypDatei,
        Text::TypVerknuepfung,
        Text::EintragsspalteAufgabe,
        Text::EintragsspalteThema,
        Text::EintragsspalteNotiz,
        Text::EintragsspalteDatum,
        Text::EintragsspalteTermin,
        Text::BelegungReserviertEditor,
        Text::BelegungZusatzReserviert,
        Text::BelegungZustellerMenue,
        Text::BelegungZusatzZusteller,
        Text::BelegungKeineFunktionGewaehlt,
        Text::BelegungSucheLeer,
        Text::BelegungSucheTreffer,
        Text::BelegungSucheKeinTreffer,
        Text::BelegungsansichtTitel,
        Text::BelegungsansichtZuweisen,
        Text::BelegungsansichtAuslieferungszustand,
        Text::BelegungsansichtFertig,
        Text::BelegungsansichtTasteEingabe,
        Text::BelegungsansichtErlaeuterung,
        Text::BelegungsansichtErstWaehlen,
        Text::BelegungsansichtAufnahme,
        Text::BelegungsansichtZurueckgesetzt,
        Text::BelegungsansichtAufnahmeAbgebrochen,
        Text::BelegungsansichtKeineFunktion,
        Text::BelegungsansichtZugewiesen,
        Text::BelegungsansichtTasteOhneNamen,
        Text::BelegungsansichtSpalteFunktion,
        Text::BelegungsansichtSpalteBelegung,
        Text::MarkdownUeberschrift,
        Text::MarkdownTabellenkopf,
        Text::MarkdownNichtEingeordnet,
        Text::MarkdownWirktTextfelderUndEditor,
        Text::MarkdownGeschrieben,
        Text::MarkdownKeinBenutzerverzeichnis,
        Text::MarkdownOrdnerFehlt,
        Text::MarkdownZugriffAbgelehnt,
        Text::MarkdownFehlgeschlagen,
        Text::TabelleKeineDateiAufDatentraeger,
        Text::TabelleNichtZuOeffnen,
        Text::TabelleZwischenablageLeer,
        Text::TabelleNichtAnBrowser,
        Text::TabelleZwischenablageKeinZiel,
        Text::TabelleNichtInDerListe,
        Text::FenstertitelQuicknote,
        Text::StatuszeileFilterstand,
        Text::StatuszeileInhaltWirdGelesen,
        Text::StatuszeileSeiteVon,
        Text::StatuszeileBildVon,
        Text::StatuszeileFolgeGekuerzt,
        Text::StatuszeileLinkesDateifenster,
        Text::StatuszeileRechtesDateifenster,
        Text::StatuszeileMeldungMitSeite,
        Text::VorgangsartKopieren,
        Text::VorgangsartVerschieben,
        Text::VorgangsartInDenPapierkorb,
        Text::VorgangsartUmbenennen,
        Text::VorgangsartPacken,
        Text::VorgangsartEntpacken,
        Text::VorgangsartDuplizieren,
        Text::VorgangAbbruchhinweis,
        Text::VorgangWirdVorbereitet,
        Text::VorgangZeile,
        Text::VorgangWirdAbgebrochen,
        Text::VorgangSchonEiner,
        Text::VorgangUebertragen,
        Text::VorgangAbgebrochen,
        Text::VorgangFertig,
        Text::UebersprungenZeile,
        Text::AnlegenFrageOrdner,
        Text::AnlegenFrageDatei,
        Text::AnlegenBestaetigen,
        Text::AngelegtOrdner,
        Text::AngelegtDatei,
        Text::AnlegenKeineRechteOrdner,
        Text::AnlegenKeineRechteDatei,
        Text::AnlegenGescheitert,
        Text::NameSchonVergeben,
        Text::DuplikatFrage,
        Text::DuplikatBestaetigen,
        Text::DuplikatMehrere,
        Text::DuplikatNichtGewoehnlich,
        Text::DuplikatTypOrdner,
        Text::DuplikatTypVerknuepfung,
        Text::UmbenennenKeineRechte,
        Text::UmbenennenGescheitert,
        Text::OrdnerKeinOrdnerMehr,
        Text::OrdnerNichtMehrErreichbar,
        Text::KeinTerminal,
        Text::PfadKopiert,
        Text::PfadeKopiert,
        Text::NichtsBetroffen,
        Text::NennformZuKopieren,
        Text::NennformZuOeffnen,
        Text::NennformZuPacken,
        Text::NennformAnzuzeigen,
        Text::NennformZuDuplizieren,
        Text::NichtsZuTeilen,
        Text::KeinArchiv,
        Text::MehrereArchive,
        Text::KeinFinder,
        Text::AblageWeistTextAb,
        Text::AbgelegtEiner,
        Text::AbgelegtMehrere,
        Text::AbgelegtAusgeschnitten,
        Text::AblageWeistVerweiseAb,
        Text::EinfuegenKeinText,
        Text::EinfuegenMehrzeilig,
        Text::EinfuegenNichtsTragbar,
        Text::UebergebenEiner,
        Text::UebergebenMehrere,
        Text::NichtAngenommenEiner,
        Text::NichtAngenommenMehrere,
        Text::UebergebenUndAbgelehnt,
        Text::KeineAnwendung,
        Text::UebergebenAnEiner,
        Text::UebergebenAnMehrere,
        Text::NichtUebergebenAn,
        Text::BelegungsdateiZweiSchreiber,
        Text::BelegungsdateiFehltNoch,
        Text::BelegungsdateiOhneAblageordner,
        Text::Markierungsstand,
        Text::LoeschenOhnePapierkorb,
        Text::WarngrundUnentscheidbar,
        Text::WarngrundNetzlaufwerk,
        Text::WarngrundCloudort,
        Text::WarngrundAusserhalbBenutzerordner,
        Text::WarngrundImBenutzerordner,
        Text::WarngrundArbeitsbaum,
        Text::WarngrundGenauDieSchwelle,
        Text::WarngrundMehrAlsDieSchwelle,
        Text::LoeschenGeraeumtAus,
        Text::LoeschenAusserdem,
        Text::LoeschenDarunterOrdner,
        Text::BlattSteht,
        Text::PfadNichtAbsolut,
        Text::PfadGibtEsNicht,
        Text::PfadNichtLesbar,
        Text::PfadInKeinemOrdner,
        Text::WerksSchaltflaeche,
        Text::WerksNotizordnerBleibt,
        Text::WerksFrage,
        Text::WerksErlaeuterung,
        Text::WerksEigeneZuweisungen,
        Text::TabAusgefiltert,
        Text::TabNichtMehrDa,
        Text::TabNichtVollstaendigGelesen,
        Text::PapierkorbKeinUtf8Pfad,
        Text::BlattSchliessen,
    ];
}

/// Eine Mengenangabe mit Einzahl und Mehrzahl.
///
/// Welche Form welche Zahl bekommt, sagt `Sprache::mehrzahl`; `{n}` steht
/// in der Form fuer die gruppierte Zahl, und die Einzahl darf es auslassen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zahlwort {
    /// Eine Datenmenge unter 1.000 Bytes, hinter `Sprache::menge`.
    Byte,
    /// `neuerungen::startzeile`: die Zahl der neuen Eintraege je Datei;
    /// `{datei}` ist der Dateiname.
    NeuerungenEintraegeIn,
    /// `Uebernahme::meldungen`: die alten Zettel sind als Notiz oder als
    /// Notizen uebernommen; gezaehlt werden die Themen `{themen}`, und beide
    /// Formen lassen `{n}` aus; `{notizen}` ist der Dateiname.
    HeimZettelUebernommen,
    /// `Marke::Geaendert` in der Zusammenfassung des Git-Bereichs, hinter
    /// seiner Zahl.
    MarkeGeaendert,
    /// `Marke::Vorgemerkt`.
    MarkeVorgemerkt,
    /// `Marke::Neu`.
    MarkeNeu,
    /// `Marke::Konflikt`.
    MarkeKonflikt,
    /// `Marke::Umbenannt`.
    MarkeUmbenannt,
    /// `statuszeile::filterstand_text`: der Satzteil `{zu_gross}` zu den
    /// Dateien ueber der Lesegrenze; beide Formen beginnen mit dem Komma, das
    /// sie anhaengt, und die Einzahl laesst `{n}` aus.
    StatuszeileDateienZuGross,
    /// `filterstand_text`: der Satzteil `{ausgeblendet}` zu den Markierungen,
    /// die der Filter ausblendet; ebenso gebaut.
    StatuszeileMarkierungenAusgeblendet,
    /// `statuszeile::bildzaehler_text`: die Fotogrenze einer gekuerzten
    /// Folge, als Glied von `{gruende}`.
    BildfolgeGrenzeFotos,
    /// `bildzaehler_text`: die Ordnergrenze.
    BildfolgeGrenzeOrdner,
    /// `bildzaehler_text`: die Grenze je Ordner.
    BildfolgeGrenzeEintraege,
    /// `operationen::eintraege_text`: die Zahl der Eintraege eines Vorgangs,
    /// als `{eintraege}` in den Vorgangszeilen; die Einzahl laesst `{n}` aus.
    Eintraege,
    /// `operationen::positionen_text`: die ausgewaehlten Positionen eines
    /// Vorgangs, als `{positionen}`; ebenso gebaut.
    AusgewaehltePositionen,
    /// `operationen::ordner_text`: die Ordner im Markierungsstand und in
    /// der Loeschfrage, als `{ordner}`; ebenso gebaut.
    Ordner,
    /// `operationen::abschlusstext`: der Zusatz zu den uebersprungenen
    /// Eintraegen; beide Formen beginnen mit dem Komma, das sie anhaengt.
    VorgangUebersprungen,
    /// `abschlusstext`: der Zusatz zu den Eintraegen, die derselbe Lauf als
    /// Ziel ausgelassen hat; ebenso gebaut.
    VorgangAusgelassen,
    /// `operationen::uebersprungenliste`: die Kopfzeile des Blattes.
    UebersprungenFrage,
    /// `Einfuegehindernis::MehrereVerweise` in
    /// `operationen::einfuegen_abgewiesen`: die Zahl der Dateiverweise.
    EinfuegenDateiverweise,
    /// `loeschwarnung::frage_und_erlaeuterung`: die Loeschfrage; `{grund}`
    /// ist der genannte Warngrund mit seinem Abstand oder leer, und die
    /// Einzahl laesst `{n}` aus.
    LoeschfrageEintraege,
    /// `startmeldungen::auskunft`: die Kopfzeile des Blattes; gerufen wird
    /// sie erst ab zwei Meldungen, die Einzahl steht der Vollstaendigkeit
    /// halber.
    StartMeldungen,
}

impl Zahlwort {
    /// Alle Zahlwoerter, in der Reihenfolge der Aufzaehlung.
    pub const ALLE: [Zahlwort; 22] = [
        Zahlwort::Byte,
        Zahlwort::NeuerungenEintraegeIn,
        Zahlwort::HeimZettelUebernommen,
        Zahlwort::MarkeGeaendert,
        Zahlwort::MarkeVorgemerkt,
        Zahlwort::MarkeNeu,
        Zahlwort::MarkeKonflikt,
        Zahlwort::MarkeUmbenannt,
        Zahlwort::StatuszeileDateienZuGross,
        Zahlwort::StatuszeileMarkierungenAusgeblendet,
        Zahlwort::BildfolgeGrenzeFotos,
        Zahlwort::BildfolgeGrenzeOrdner,
        Zahlwort::BildfolgeGrenzeEintraege,
        Zahlwort::Eintraege,
        Zahlwort::AusgewaehltePositionen,
        Zahlwort::Ordner,
        Zahlwort::VorgangUebersprungen,
        Zahlwort::VorgangAusgelassen,
        Zahlwort::UebersprungenFrage,
        Zahlwort::EinfuegenDateiverweise,
        Zahlwort::LoeschfrageEintraege,
        Zahlwort::StartMeldungen,
    ];
}
