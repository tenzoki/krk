//! Die deutsche Tabelle: die Quelle.
//!
//! Jeder Eintrag ist der Wortlaut, den die Oberflaeche vor dieser Arbeit an
//! der genannten Stelle trug, Zeichen fuer Zeichen; die Wortlautproben des
//! Baums halten ihn weiter gegen ihr eigenes Literal, und ein Vergleich mit
//! diesem Eintrag waere dort eine Tautologie. Mit Umlauten, wie es die
//! Umlautregel vom 260907 fuer nutzersichtbaren Text verlangt.

use super::super::{Text, Zahlwort};
use crate::tasten::belegung::{Kommando, Zugestellt};

/// Der deutsche Eintrag zu einem Schluessel.
pub(in super::super) const fn text(schluessel: Text) -> &'static str {
    match schluessel {
        Text::WirkungsbereichDateifenster => "Dateifenster",
        Text::WirkungsbereichLeiste => "Lesezeichen- und Geräteleiste",
        Text::WirkungsbereichDateibereiche => "Dateifenster, Vorschau und Editor",
        Text::WirkungsbereichEditor => "Editor",
        Text::WirkungsbereichEditortext => "Text im Editor",
        Text::WirkungsbereichEintraege => "Einträge im Editor",
        Text::WirkungsbereichReihenfolge => "Einträge in Dateireihenfolge im Editor",
        Text::WirkungsbereichAufgaben => "Aufgaben im Editor",
        Text::WirkungsbereichTermine => "Termine im Editor",
        Text::WirkungsbereichGeheimnisse => "Geheimnisse im Editor",
        Text::WirkungsbereichQuicknote => "Quicknote im Editor",
        Text::WirkungsbereichTabbereich => "Dateifenster und Vorschau",
        Text::WirkungsbereichNavigator => "Dateifenster, Leiste, Vorschau und Git-Bereich",
        Text::WirkungsbereichVorschau => "Vorschau",
        Text::WirkungsbereichBildfolge => "Dateifenster, solange die Vorschau eine Bildfolge zeigt",
        Text::WirkungsbereichUeberall => "überall",
        Text::OrtsmangelAbsolut => "ist ein absoluter Pfad",
        Text::OrtsmangelLeeresStueck => "trägt ein leeres Stück",
        Text::OrtsmangelPunktstueck => "trägt ein Stück . oder ..",
        Text::OrtsmangelMehrerePlatzhalter => {
            "trägt mehr als einen Platzhalter * und damit Kosten, die erst am Bestand feststünden"
        }
        Text::NameLeer => "der Name ist leer",
        Text::NameMitSchraegstrich => "ein Name darf keinen Schrägstrich enthalten",
        Text::NameMitNullbyte => "ein Name darf kein Nullbyte enthalten",
        Text::NamePunktname => "'.' und '..' sind keine Namen",
        Text::KollisionBestehender => "der Name ist schon vergeben",
        Text::KollisionDoppelt => "zweimal derselbe neue Name",
        Text::AblagegrundNichtLesbar => "ist nicht lesbar",
        Text::AblagegrundBeschaedigt => "ist beschädigt",
        Text::AblagegrundNichtAnlegbar => "ließ sich nicht anlegen",
        Text::ErsatzAuslieferungszustand => "und wird durch den Auslieferungszustand ersetzt",
        Text::ErsatzNichts => "und nichts tritt an ihre Stelle",
        Text::PinKeineVierZiffern => "Die PIN besteht aus genau vier Ziffern.",
        Text::AbweisungUmbruchImAufgabentext => {
            "Eine Aufgabe ist eine Zeile und trägt keinen Zeilenumbruch."
        }
        Text::AbweisungUmbruchImThema => "Ein Thema ist eine Zeile und trägt keinen Zeilenumbruch.",
        Text::AbweisungThemenzeileImNotiztext => {
            "Eine Zeile im Notiztext darf nicht mit „## “ beginnen, denn so beginnt die nächste Notiz."
        }
        Text::AbweisungUngueltigesDatum => {
            "Ein Datum steht als YYMMDD oder YYMMDD HH:MM, etwa 261002 oder 261002 09:30."
        }
        Text::AbweisungKopfzeileImTermintext => {
            "Eine Zeile im Termintext darf nicht mit „## “ beginnen, denn so beginnt der nächste Termin."
        }
        Text::ZaehlzeileDateien => "Dateien",
        Text::ZaehlzeileOrdner => "Ordner",
        Text::ZaehlzeileVerknuepfungen => "Verknüpfungen",
        Text::EinheitKilobyte => "kB",
        Text::EinheitMegabyte => "MB",
        Text::EinheitGigabyte => "GB",
        Text::EinheitTerabyte => "TB",
        Text::VorgangKeineRechte => "keine Rechte",
        Text::VorgangGibtEsNichtMehr => "gibt es nicht mehr",
        Text::VorgangAmZielStehtEintrag => "am Ziel steht schon ein Eintrag",
        Text::VorgangKeinPlatzAufDemDatentraeger => "kein Platz mehr auf dem Datenträger",
        Text::VorgangKeinArbeitsfaden => "kein Arbeitsfaden frei: {grund}",
        Text::VorgangNeuerNameFehlt => "es fehlt der neue Name",
        Text::VorgangZielordnerFehlt => "es fehlt der Zielordner",
        Text::VorgangPackenNichtQuelleFuerQuelle => "das Packen läuft nicht Quelle für Quelle",
        Text::VorgangPfadBenenntKeinenEintrag => "der Pfad benennt keinen Eintrag",
        Text::VorgangQuelleUndZielDerselbeEintrag => "Quelle und Ziel sind derselbe Eintrag",
        Text::VorgangZielLiegtInDerQuelle => "das Ziel liegt in der Quelle",
        Text::VorgangZielNichtErsetzt => "das Ziel ließ sich nicht ersetzen: {grund}",
        Text::VorgangNachAbbruchNichtWeggeraeumt => "nach dem Abbruch nicht weggeräumt: {grund}",
        Text::VorgangOrdnerangabenNichtKopiert => {
            "Inhalt kopiert, Rechte und Datum des Ordners nicht: {grund}"
        }
        Text::VorgangOrdnerSelbstBlieb => "Inhalt verschoben, der Ordner selbst blieb: {grund}",
        Text::VorgangNichtVollstaendigKopiert => {
            "nicht vollständig kopiert, in der Quelle geblieben"
        }
        Text::VorgangKopiertAberInQuelleGeblieben => {
            "kopiert, aber in der Quelle geblieben: {grund}"
        }
        Text::VorgangKeinPapierkorb => "kein Papierkorb eingehängt; es wurde nichts gelöscht",
        Text::VorgangKeineGewoehnlicheDatei => "keine gewöhnliche Datei",
        Text::VorgangZielNichtInPapierkorb => {
            "das Ziel ließ sich nicht in den Papierkorb räumen: {grund}"
        }
        Text::EntpackenEintragFuehrtHeraus => {
            "„{name}“ führt aus dem Zielordner heraus und ist ausgelassen"
        }
        Text::EntpackenEintragMitGrund => "„{name}“: {grund}",
        Text::EntpackenAmZielStehtVerknuepfung => "„{name}“: am Ziel steht schon eine Verknüpfung",
        Text::EntpackenWegMitUnzulaessigemBestandteil => {
            "der Weg zum Eintrag trägt einen unzulässigen Bestandteil"
        }
        Text::EntpackenWegDurchVerknuepfung => {
            "der Weg zum Eintrag führt durch eine Verknüpfung aus dem Zielordner heraus"
        }
        Text::EntpackenDateiStattOrdnerAufDemWeg => {
            "auf dem Weg zum Eintrag steht eine Datei, wo ein Ordner stehen müsste"
        }
        Text::EntpackenVerweiszielKeinText => "das Verweisziel ist kein gültiger Text",
        Text::PackenArchivUnfertig => "das Archiv blieb unfertig: {fehler}",
        Text::PackenKeinPlatzImArchiv => "kein Platz im Archiv: {fehler}",
        Text::PackenHalberEintragImArchiv => "der halbe Eintrag blieb im Archiv: {fehler}",
        Text::PackenNichtInsArchivGeschrieben => "nicht ins Archiv geschrieben: {fehler}",
        Text::StapelKeinStartwert => "„{text}“ ist kein Startwert für die Nummerierung",
        Text::StapelKeineStellenzahl => "„{text}“ ist keine Stellenzahl zwischen 1 und {hoechste}",
        Text::EditorKeinFreierDateizugriff => {
            "{pfad} lässt sich gerade nicht öffnen: KRK hat keinen freien Dateizugriff mehr ({grund}); nach dem Ende der laufenden Suche noch einmal versuchen"
        }
        Text::EditorNichtZuOeffnen => "{pfad} lässt sich nicht im Editor öffnen: {grund}",
        Text::EditorZuGross => {
            "{pfad} ist mit {groesse} Bytes zu groß für den Editor; die Grenze liegt bei {grenze} Bytes"
        }
        Text::EditorKeineTextdatei => "{pfad} ist keine Textdatei und wird nicht geöffnet",
        Text::EditorOrdnerHatKeinenText => {
            "ein Ordner hat keinen Text, den der Editor zeigen könnte"
        }
        Text::EditorKeineGewoehnlicheDatei => "das ist keine gewöhnliche Datei",
        Text::LesenDatenschutzsperre => {
            "macOS sperrt den Zugriff auf „{name}“. Freigabe: Systemeinstellungen › Datenschutz & Sicherheit › Festplattenvollzugriff › KRK, danach KRK neu starten."
        }
        Text::LesenKeinVerzeichnis => "{pfad} ist kein Verzeichnis",
        Text::LesenPfadMitNullbyte => "{pfad} enthält ein Nullbyte",
        Text::AblageKeinGueltigesUtf8 => "keine gültige UTF-8-Folge",
        Text::AblageOhneOberstenSchluessel => {
            "die Datei trägt keinen einzigen obersten Schlüssel, und KRK schreibt sie nie so"
        }
        Text::AblageKeinBenutzerverzeichnis => "das System nennt kein Benutzerverzeichnis",
        Text::AtomarOhneDateinamen => "{pfad} trägt keinen Dateinamen",
        Text::AtomarRechteNichtUebertragen => {
            "die Rechte {soll} von {pfad} lassen sich nicht übertragen; die Nachbardatei steht auf {gesetzt}"
        }
        Text::ErsetzungOhneSicherung => "{datei} {beschreibung} {ersatz}: {einzelheit}",
        Text::ErsetzungGesichert => {
            "Die bisherige Fassung liegt unter {sicherung}; {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::ErsetzungGekuerzt => {
            "Die bisherige Fassung liegt gekürzt unter {sicherung}, gesichert sind allein ihre ersten {grenze} Bytes; {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::ErsetzungSchonVorhanden => {
            "Die bisherige Fassung liegt seit einem früheren Start unter {sicherung} und bleibt dort; {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::ErsetzungSicherungGescheitert => {
            "Der Inhalt ließ sich nicht zur Seite legen ({fehler}); {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::EinstellungenVerweis => {
            "settings.toml ist ein symbolischer Verweis, und KRK ersetzt ihn nicht durch eine Datei; der Ort bleibt, wie er ist. Von Hand in die Zieldatei eintragen: {zeile}"
        }
        Text::EinstellungenBeschaedigt => {
            "settings.toml ist zuerst von Hand zu berichtigen, KRK schreibt sie so nicht: {befund}"
        }
        Text::EinstellungenNichtLesbar => {
            "settings.toml ist nicht lesbar, KRK schreibt sie nicht: {befund}"
        }
        Text::EinstellungenIntern => {
            "settings.toml bleibt, wie sie ist: das Ergebnis hätte mehr geändert als den Notizordner ({befund})"
        }
        Text::EinstellungenNichtGeschrieben => {
            "settings.toml ließ sich nicht schreiben und bleibt, wie sie war: {befund}"
        }
        Text::EinstellungenKeineGewoehnlicheDatei => {
            "an ihrer Stelle steht keine gewöhnliche Datei"
        }
        Text::EinstellungenNotizordnerKeinEinzelwert => {
            "notizordner steht nicht als einzelner Wert in einer Zeile „notizordner = …“ da"
        }
        Text::EinstellungenNeuerWertFehlt => "der neue Wert steht nicht als notizordner da",
        Text::EinstellungenAndererWertGeaendert => "ein anderer Wert der Datei hätte sich geändert",
        Text::WerksVerweis => {
            "{datei} ist ein symbolischer Verweis, und KRK ersetzt ihn nicht durch eine Datei; nichts ist zurückgesetzt."
        }
        Text::WerksKeineDatei => {
            "An der Stelle von {datei} steht keine gewöhnliche Datei; nichts ist zurückgesetzt."
        }
        Text::WerksNichtLesbar => {
            "{datei} lässt sich nicht befragen ({befund}); nichts ist zurückgesetzt."
        }
        Text::WerksKeineOrtszeit => {
            "Die Uhr dieses Geräts ergibt keinen Zeitstempel für die Sicherungen; nichts ist zurückgesetzt."
        }
        Text::WerksKeinFreierName => {
            "Für {datei} ist in dieser Minute kein freier Sicherungsname mehr übrig; nichts ist zurückgesetzt."
        }
        Text::WerksNichtBeiseitegelegt => {
            "{datei} ließ sich nicht beiseitelegen ({befund}); nichts ist zurückgesetzt."
        }
        Text::WerksNichtVorbereitet => {
            "Die neue Fassung von {datei} ließ sich nicht schreiben ({befund}); nichts ist zurückgesetzt."
        }
        Text::WerksEinstellungen => "{befund}. Nichts ist zurückgesetzt.",
        Text::WerksNichtZurueckgebaut => {
            "{hindernis} Liegen geblieben ist eine zweite Kopie des unveränderten Inhalts unter {pfade}."
        }
        Text::WerksZurueckgesetzt => "Auf Werkseinstellungen zurückgesetzt.",
        Text::WerksTeilweiseZurueckgesetzt => "Nur teilweise auf Werkseinstellungen zurückgesetzt.",
        Text::WerksBeiseitegelegt => "Beiseitegelegt: {pfade}.",
        Text::WerksStandNichtDa => "{datei} stand nicht da, für sie ist nichts beiseitegelegt.",
        Text::WerksDateiNichtZurueckgesetzt => "{datei} ist nicht zurückgesetzt: {fehler}.",
        Text::NeuerungenStartzeile => {
            "Neu in dieser Fassung: {teile}. Ihre Dateien liegen unter {ordner}."
        }
        Text::NeuerungenDateiFehlt => {
            "Diese Datei liegt nicht in Ihrer Ablage; verglichen wird nur, was dasteht."
        }
        Text::NeuerungenDateiBeschaedigt => {
            "Diese Datei ist beschädigt und wird deshalb nicht verglichen."
        }
        Text::NeuerungenNeuInDieserFassung => "Neu in dieser Fassung",
        Text::NeuerungenNurInIhrerDatei => "Nur in Ihrer Datei",
        Text::NeuerungenZeileLeer => "{ueberschrift}: —",
        Text::NeuerungenZeile => "{ueberschrift}: {namen}",
        Text::NeuerungenKeineEigenenEintraege => {
            "{zeile} (diese Datei kann keine eigenen Einträge führen; einen unbekannten Eintrag weist KRK als beschädigt ab)"
        }
        Text::NeuerungenPreisLeser => {
            "Ein Profil, das Ihre Datei nicht führt, kostet die Zusammenfassung für diesen Ort: die Vorschau zeigt dort die Metadaten."
        }
        Text::NeuerungenPreisEinstellungen => {
            "Ein Schlüssel, den Ihre Datei nicht führt, kostet allein den erklärenden Kommentarblock; den Wert selbst nimmt KRK aus der Auslieferungsfassung."
        }
        Text::NeuerungenPreisBelegung => {
            "Eine Funktion, die Ihre Datei nicht führt, kostet ihre ausgelieferten Tastenkombinationen; über das Hauptmenü bleibt sie erreichbar."
        }
        Text::NeuerungenSchlusssatz => {
            "Gezeigt ist der Stand, den KRK zuletzt gelesen hat. Womit KRK arbeitet, steht seit dem Start fest: eine geänderte Datei wirkt erst beim nächsten Start."
        }
        Text::OrtKeinBenutzerverzeichnis => {
            "Das System nennt kein Benutzerverzeichnis, also gibt es keinen Notizordner"
        }
        Text::OrtLeer => {
            "Der Notizordner in settings.toml ist leer; gültig ist ein Ort, der mit „~/“ oder „/“ beginnt, ab Werk „~/{ordnername}“"
        }
        Text::OrtNichtAbsolut => {
            "Der Notizordner „{wert}“ in settings.toml beginnt weder mit „~/“ noch mit „/“"
        }
        Text::OrtFremdesBenutzerverzeichnis => {
            "Der Notizordner „{wert}“ in settings.toml nennt ein fremdes Benutzerverzeichnis; gültig ist „~/“ für das eigene oder ein Pfad ab „/“"
        }
        Text::OrtKeinText => {
            "Der Notizordner in settings.toml ist kein Text, sondern {wert}; gültig ist ein Ort in Anführungszeichen, etwa {beispiel}"
        }
        Text::OrtImAblageordner => {
            "Der Notizordner „{wert}“ liegt im Ablageordner von KRK; ein Werkzeug, das KRK entfernt, nähme ihn mit, also gilt er nicht"
        }
        Text::OrtEinstellungenBeschaedigt => {
            "settings.toml {satzteil}, also gilt kein Notizordner, und F2 legt nichts an: settings.toml berichtigen und KRK neu starten, oder nach dem Berichtigen den Ort über „Home“ → „Ort wählen…“ setzen"
        }
        Text::OrtEinstellungenUngelesen => {
            "KRK konnte settings.toml beim Start nicht lesen ({ursache}), also gilt kein Notizordner, und F2 legt nichts an: KRK neu starten"
        }
        Text::OrtWechsel => {
            "Der Notizordner ist jetzt „{neu}“; am alten Ort „{alt}“ bleibt alles liegen, und F2 führt zum neuen"
        }
        Text::OrtAbweisung => {
            "Zuerst {datei} im Editor schließen; solange der Editor eine Datei des Notizordners hält, wählt KRK keinen anderen Ort"
        }
        Text::OrtGewaehlt => "Der Notizordner ist jetzt „{neu}“, und F2 führt dorthin",
        Text::OrtSchonDerOrt => {
            "„{neu}“ ist schon der Notizordner; settings.toml bleibt, wie sie ist"
        }
        Text::HeimKeinOrdner => {
            "{ordner} ist kein Ordner; KRK legt dort nichts an und öffnet keinen Tab"
        }
        Text::HeimNichtErreichbar => "{ordner} ist nicht erreichbar: {grund}",
        Text::HeimNichtAnlegbar => "{name} lässt sich nicht anlegen: {grund}",
        Text::HeimObererOrdnerFehlt => {
            "{ordner} lässt sich nicht anlegen, weil der Ordner darüber fehlt, etwa ein nicht eingehängtes Laufwerk; KRK legt nichts an"
        }
        Text::HeimMerkerNichtVermerkt => {
            "KRK kann sich nicht merken, dass die alten Zettel übernommen sind ({grund}); das nächste F2 versucht es wieder, und wird {ort} vorher gelöscht, übernimmt es sie noch einmal"
        }
        Text::HeimMerkerOhneAblage => {
            "KRK kann sich ohne seinen Ablageordner nicht merken, dass die alten Zettel übernommen sind; ein späteres F2 holt das nach, und wird {ort} vorher gelöscht, übernimmt es sie noch einmal"
        }
        Text::HeimGeheimnisseUmbenannt => "{alt} heißt jetzt {neu}",
        Text::HeimGeheimnisseBeideStehen => {
            "{neu} und {alt} stehen beide in {ort}; KRK benennt keine um, und es gilt {neu}"
        }
        Text::HeimGeheimnisseAlterNameBleibt => {
            "{alt} heißt jetzt auch {neu}, der alte Name lässt sich nicht entfernen: {grund}"
        }
        Text::HeimGeheimnisseNichtUmbenannt => {
            "{alt} lässt sich nicht in {neu} umbenennen ({grund}); sie bleibt unverändert, und {neu} ist nicht angelegt"
        }
        Text::HeimZettelNotizenStandenSchon => {
            "Die alten Zettel sind nicht übernommen, weil {notizen} schon stand; {dateien} liegen unverändert im Ablageordner"
        }
        Text::HeimZettelGescheitert => {
            "Die alten Zettel sind nicht übernommen ({grund}); {dateien} liegen unverändert im Ablageordner"
        }
        Text::HeimZettelThemenzeile => {
            "{thema} ist nicht übernommen, weil er eine Zeile mit „## “ trägt; {datei} liegt unverändert im Ablageordner"
        }
        Text::HeimZettelUnlesbar => {
            "{thema} ist nicht übernommen ({grund}); {datei} liegt unverändert im Ablageordner"
        }
        Text::HeimZettelZuGross => "mit {groesse} Bytes zu groß",
        Text::HeimZettelKeinText => "kein lesbarer Text",
        Text::HeimAngefangeneDateiBleibt => {
            "{fehler}; die angefangene Datei lässt sich nicht entfernen: {entfernen}"
        }
        Text::Und => "und",
        Text::TresorKeinZufall => "Das System liefert keinen Zufallswert: {grund}",
        Text::TresorAbleitung => "Der Schlüssel lässt sich nicht ableiten: {grund}",
        Text::TresorVerschluesselung => "Der Inhalt lässt sich nicht verschlüsseln",
        Text::TresorPinFalschOderVeraendert => "PIN falsch oder Datei verändert",
        Text::TresorKopfBeschaedigt => "Der Kopf der Datei ist beschädigt: {grund}",
        Text::TresorKopfKennungFehlt => "die Kennung am Anfang fehlt",
        Text::TresorKopfAbgeschnitten => "die Datei ist abgeschnitten",
        Text::TresorKopfUnbekannteVersion => "unbekannte Formatversion {version}",
        Text::TresorKopfUnbekannteAbleitung => "unbekannte Ableitung {ableitung}",
        Text::TresorKopfUngueltigeParameter => "die Parameter der Ableitung sind ungültig",
        Text::ZusammenfassungKopf => "Name: {name}\nPfad: {pfad}",
        Text::ZusammenfassungBlockzeile => "{beschriftung}:",
        Text::ZusammenfassungZeile => "{beschriftung}: {wert}",
        Text::ZusammenfassungPlatzhalter => "--",
        Text::WertUeberGrenze => {
            "mindestens {gezaehlt} (Lesung bei {grenze} Einträgen abgebrochen)"
        }
        Text::Ja => "ja",
        Text::Nein => "nein",
        Text::ProfilMeldung => "Profil „{profil}“: {grund}",
        Text::ProfilZeilenmeldung => "Profil „{profil}“, Zeile „{beschriftung}“: {grund}",
        Text::ProfilMehrereBausteine => {
            "sie nennt {anzahl} Bausteine ({namen}) und nicht genau einen"
        }
        Text::ProfilKeinBaustein => "sie nennt keinen der vier Bausteine ({namen})",
        Text::ProfilOhneErkennung => {
            "es nennt weder ein Pfadmuster noch eine Kennzeichendatei und könnte damit nie treffen"
        }
        Text::ProfilBildfolge => "die Bildfolge: {grund}",
        Text::ProfilPfadmuster => "das Pfadmuster",
        Text::ProfilKennzeichendatei => "die Kennzeichendatei",
        Text::ProfilErkennungsmusterNichtUebersetzt => {
            "{was} {muster} lässt sich nicht übersetzen: {grund}"
        }
        Text::ProfilMusterNichtUebersetzt => {
            "das Muster {muster} lässt sich nicht übersetzen: {grund}"
        }
        Text::ProfilFeldmusterFanggruppen => {
            "das Feldmuster {muster} trägt {gruppen} Fanggruppen und nicht genau eine"
        }
        Text::ProfilOrtsangabe => "die Ortsangabe {angabe} {mangel}",
        Text::ProfilOrtsangabeMitPlatzhalter => {
            "die Ortsangabe {angabe} trägt einen Platzhalter, und der Baustein „{baustein}“ nimmt keinen an: er liest Dateien und braucht dafür ihren Pfad, den ein zusammengelegter Lesestand nicht trägt"
        }
        Text::ProfilJuengsteNull => "juengste mit anzahl = 0 kann nie einen Eintrag zeigen",
        Text::GitKeinRepository => "Dieser Ordner liegt in keinem Git-Repository.",
        Text::GitOhneCommit => "noch kein Commit",
        Text::GitUnveraendert => "unverändert",
        Text::GitImOrdner => "{marken} in diesem Ordner",
        Text::GitKopfAbgeloest => "{kurzhash} (abgelöst)",
        Text::TasteNameFehlt => "es fehlt der Tastenname",
        Text::TasteKeineZusatztaste => "„{text}“ ist keine Zusatztaste; erlaubt sind {erlaubt}",
        Text::TasteFnKeineZusatztaste => {
            "fn ist keine Zusatztaste einer Belegung; Funktionstasten schlägt KRK über den Tastencode nach, und F3 mit gehaltener fn erzeugt denselben Tastencode wie ein nacktes F3"
        }
        Text::TasteZusatztasteDoppelt => "die Zusatztaste „{text}“ steht zweimal",
        Text::TasteReihenfolgeVerletzt => {
            "„{zusatztaste}“ steht hinter „{hinter}“; die Reihenfolge ist {reihenfolge}"
        }
        Text::TasteUnbekannterName => "„{text}“ ist kein Tastenname dieser Schreibweise",
        Text::FunktionsnameMitKennung => "„{name}“ ({kennung})",
        Text::BelegungKonflikt => {
            "die Kombination {kombination} gehört schon der Funktion {andere} und lässt sich nicht zusätzlich der Funktion {bewerber} zuweisen"
        }
        Text::BelegungSchreibweise => {
            "die Funktion {kennung} trägt die Kombination „{text}“: {fehler}"
        }
        Text::BelegungUnbekannteFunktion => "KRK kennt keine Funktion namens {kennung}",
        Text::BelegungFunktionDoppelt => "die Funktion {kennung} steht zweimal",
        Text::MenueUeberKrk => "Über KRK",
        Text::MenueTastenbelegungAlsMarkdown => "Tastenbelegung als Markdown sichern",
        Text::FunktionsbereichAnwendung => "Anwendung",
        Text::FunktionsbereichHome => "Home",
        Text::FunktionsbereichDateilisting => "Dateilisting",
        Text::FunktionsbereichDateioperationen => "Dateioperationen",
        Text::FunktionsbereichTabs => "Tabs",
        Text::FunktionsbereichVorschau => "Vorschau",
        Text::FunktionsbereichLeisteUndFokus => "Leiste und Fokus",
        Text::FunktionsbereichEditor => "Editor",
        Text::FunktionsbereichGit => "Git",
        Text::FunktionsbereichTextbefehle => "Bearbeiten",
        Text::FunktionsbereichFenster => "Fenster",
        Text::KontextOeffnenMit => "Öffnen mit",
        Text::KontextZippen => "Zip",
        Text::KontextEntpacken => "Unzip",
        Text::KontextDuplizieren => "Duplizieren…",
        Text::KontextImFinderOeffnen => "Im Finder öffnen",
        Text::KontextImFinderAnzeigen => "Im Finder anzeigen",
        Text::BereichLesezeichen => "Lesezeichen",
        Text::BereichLinks => "Links",
        Text::BereichRechts => "Rechts",
        Text::BereichVorschau => "Vorschau",
        Text::BereichEditor => "Editor",
        Text::BereichGit => "Git",
        Text::BereichLesezeichenLang => "Lesezeichen- und Geräteleiste",
        Text::BereichLinksLang => "Linkes Dateifenster",
        Text::BereichRechtsLang => "Rechtes Dateifenster",
        Text::BereichVorschauLang => "Vorschaufenster",
        Text::BereichEditorLang => "Eingebauter Editor",
        Text::BereichGitLang => "Git-Bereich",
        Text::SpalteName => "Name",
        Text::SpalteGroesse => "Größe",
        Text::SpalteDatum => "Datum",
        Text::SpalteTyp => "Typ",
        Text::SpalteMarke => "Marke",
        Text::SpalteAenderungsdatum => "Änderungsdatum",
        Text::LeisteBereichUmschalten => "{bereich} ein- und ausblenden",
        Text::LeisteSpalteUmschalten => {
            "Spalte „{spalte}“ in beiden Dateilisten ein- und ausblenden"
        }
        Text::LeisteTiefeHinweis => "Den stehenden Filter auf den Unterbaum ausdehnen",
        Text::LeisteInhaltHinweis => {
            "Den stehenden Filter auch auf den Inhalt der Dateien anwenden"
        }
        Text::LeisteUeberschriftLesezeichen => "Lesezeichen",
        Text::LeisteUeberschriftGeraete => "Geräte und Orte",
        Text::LeisteLesezeichenFehlt => "{name} (fehlt)",
        Text::LeisteSinnbildTextstelle => "Textstelle",
        Text::TypOrdner => "Ordner",
        Text::TypDatei => "Datei",
        Text::TypVerknuepfung => "Verknüpfung",
        Text::EintragsspalteAufgabe => "Aufgabe",
        Text::EintragsspalteThema => "Thema",
        Text::EintragsspalteNotiz => "Notiz",
        Text::EintragsspalteDatum => "Datum",
        Text::EintragsspalteTermin => "Termin",
        Text::BelegungReserviertEditor => "den Editor",
        Text::BelegungZusatzReserviert => "(reserviert für {wofuer})",
        Text::BelegungZustellerMenue => "Kürzel des Menüs",
        Text::BelegungZusatzZusteller => "({zusteller})",
        Text::BelegungKeineFunktionGewaehlt => "es ist keine Funktion ausgewählt",
        Text::BelegungSucheLeer => "Der Suchtext ist leer; jedes getippte Zeichen sucht.",
        Text::BelegungSucheTreffer => "Suche „{text}“: Treffer {stelle} von {anzahl}.",
        Text::BelegungSucheKeinTreffer => "Suche „{text}“: kein Treffer.",
        Text::BelegungsansichtTitel => "Tastaturbelegung",
        Text::BelegungsansichtZuweisen => "Zuweisen",
        Text::BelegungsansichtAuslieferungszustand => "Auslieferungszustand",
        Text::BelegungsansichtFertig => "Fertig",
        Text::BelegungsansichtTasteEingabe => "Eingabe",
        Text::BelegungsansichtErlaeuterung => {
            "Jedes getippte Zeichen sucht in beiden Spalten und springt auf den ersten Treffer; die Eingabetaste geht zum nächsten, die Rücktaste kürzt den Suchtext. Pfeiltasten wählen die Funktion. {zuweisen} ({zuweisen_taste}) nimmt die nächste gedrückte Kombination auf; esc bricht die Aufnahme ab. {zuruecksetzen} ({zuruecksetzen_taste}) setzt alles zurück. {fertig} ({fertig_taste}) oder esc verlässt die Ansicht und sichert die Änderungen."
        }
        Text::BelegungsansichtErstWaehlen => "Erst eine Funktion auswählen, dann Zuweisen drücken.",
        Text::BelegungsansichtAufnahme => {
            "Jetzt die gewünschte Kombination für „{name}“ drücken; esc bricht ab."
        }
        Text::BelegungsansichtZurueckgesetzt => {
            "Die Belegung ist auf den Auslieferungszustand zurückgesetzt."
        }
        Text::BelegungsansichtAufnahmeAbgebrochen => {
            "Die Aufnahme ist abgebrochen; die Belegung ist unverändert."
        }
        Text::BelegungsansichtKeineFunktion => {
            "Es ist keine Funktion ausgewählt; die Belegung ist unverändert."
        }
        Text::BelegungsansichtZugewiesen => "„{funktion}“ liegt jetzt auf {kombination}.",
        Text::BelegungsansichtTasteOhneNamen => {
            "Diese Taste hat in der Kombinationsschreibweise keinen Namen und lässt sich nicht ablegen; die Belegung ist unverändert."
        }
        Text::BelegungsansichtSpalteFunktion => "Funktion",
        Text::BelegungsansichtSpalteBelegung => "Belegung",
        Text::MarkdownUeberschrift => "# Tastenbelegung von KRK",
        Text::MarkdownTabellenkopf => "| Funktion | Kombinationen | Wirkt in |",
        Text::MarkdownNichtEingeordnet => "(von KRK nicht eingeordnet)",
        Text::MarkdownWirktTextfelderUndEditor => "Textfelder und Editor",
        Text::MarkdownGeschrieben => "Tastenbelegung geschrieben: {pfad}",
        Text::MarkdownKeinBenutzerverzeichnis => {
            "die Tastenbelegung ließ sich nicht schreiben: das System nennt kein Benutzerverzeichnis"
        }
        Text::MarkdownOrdnerFehlt => {
            "die Tastenbelegung ließ sich nicht schreiben: der Ordner zu {pfad} fehlt"
        }
        Text::MarkdownZugriffAbgelehnt => {
            "die Tastenbelegung ließ sich nicht schreiben: der Zugriff auf {pfad} ist abgelehnt"
        }
        Text::MarkdownFehlgeschlagen => {
            "die Tastenbelegung ließ sich nicht nach {pfad} schreiben: {grund}"
        }
        Text::TabelleKeineDateiAufDatentraeger => {
            "die Quelle liefert keine Datei auf dem Datenträger"
        }
        Text::TabelleNichtZuOeffnen => "{pfad} lässt sich nicht öffnen: {grund}",
        Text::TabelleZwischenablageLeer => "die Zwischenablage ist leer",
        Text::TabelleNichtAnBrowser => "{adresse} ließ sich nicht an den Systembrowser übergeben",
        Text::TabelleZwischenablageKeinZiel => {
            "die Zwischenablage trägt weder einen absoluten Pfad noch eine Web-Adresse"
        }
        Text::TabelleNichtInDerListe => "{name} steht nicht in der Liste",
        Text::FenstertitelQuicknote => "Quicknote",
    }
}

/// Einzahl und Mehrzahl zu einem deutschen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} Byte", "{n} Bytes"),
        Zahlwort::NeuerungenEintraegeIn => ("{n} Eintrag in {datei}", "{n} Einträge in {datei}"),
        Zahlwort::HeimZettelUebernommen => (
            "{themen} als Notiz in {notizen} übernommen",
            "{themen} als Notizen in {notizen} übernommen",
        ),
        Zahlwort::MarkeGeaendert => ("{n} geändert", "{n} geändert"),
        Zahlwort::MarkeVorgemerkt => ("{n} vorgemerkt", "{n} vorgemerkt"),
        Zahlwort::MarkeNeu => ("{n} neu", "{n} neu"),
        Zahlwort::MarkeKonflikt => ("{n} in Konflikt", "{n} in Konflikt"),
        Zahlwort::MarkeUmbenannt => ("{n} umbenannt", "{n} umbenannt"),
    }
}

/// Der deutsche Name eines Kommandos: der Eintrag des Hauptmenues, der
/// Belegungsansicht und jeder Konfliktmeldung.
pub(in super::super) const fn kommandoname(schluessel: Kommando) -> &'static str {
    match schluessel {
        Kommando::AuswahlHoch => "Auswahl einen Eintrag nach oben",
        Kommando::AuswahlRunter => "Auswahl einen Eintrag nach unten",
        Kommando::SeiteHoch => "Auswahl eine Bildschirmseite nach oben",
        Kommando::SeiteRunter => "Auswahl eine Bildschirmseite nach unten",
        Kommando::Listenanfang => "An den Anfang der Liste",
        Kommando::Listenende => "An das Ende der Liste",
        Kommando::Oeffnen => "In den ausgewählten Ordner einsteigen",
        Kommando::OrdnerAufwaerts => "In den übergeordneten Ordner",
        Kommando::OrdnerDerDatei => "Ordner der angezeigten Datei zeigen",
        Kommando::OrdnerAngleichen => "Anderes Dateifenster auf diesen Ordner stellen",
        Kommando::Pfadeingabe => "Pfad eingeben und dorthin springen",
        Kommando::MarkierungUmschalten => "Eintrag markieren und zum nächsten rücken",
        Kommando::AlleMarkieren => "Alle Einträge markieren",
        Kommando::MarkierungAufheben => "Jede Markierung aufheben",
        Kommando::MarkierungUmkehren => "Markierung umkehren",
        Kommando::SortierungName => "Nach Name sortieren",
        Kommando::SortierungGroesse => "Nach Größe sortieren",
        Kommando::SortierungDatum => "Nach Änderungsdatum sortieren",
        Kommando::SortierungTyp => "Nach Typ sortieren",
        Kommando::SortierrichtungUmkehren => "Sortierrichtung umkehren",
        Kommando::VersteckteUmschalten => "Versteckte Dateien ein- und ausblenden",
        Kommando::SpalteGroesseUmschalten => "Spalte Größe ein- und ausblenden",
        Kommando::SpalteDatumUmschalten => "Spalte Änderungsdatum ein- und ausblenden",
        Kommando::SpalteTypUmschalten => "Spalte Typ ein- und ausblenden",
        Kommando::TiefeSucheUmschalten => "Tiefe Suche ein- und ausschalten",
        Kommando::InhaltssucheUmschalten => "Inhaltssuche ein- und ausschalten",
        Kommando::ZwischenablageSpringen => "Zum Inhalt der Zwischenablage springen",
        Kommando::ZwischenablageAnsehen => "Zwischenablage ansehen",
        Kommando::TabNeu => "Neuen Tab öffnen",
        Kommando::TabSchliessen => "Aktiven Tab schließen",
        Kommando::TabNaechster => "Zum nächsten Tab",
        Kommando::TabVoriger => "Zum vorigen Tab",
        Kommando::FensterWechseln => "Aktives Dateifenster wechseln",
        Kommando::LeisteUmschalten => "Lesezeichen- und Geräteleiste ein- und ausblenden",
        Kommando::ErstesFensterUmschalten => "Linkes Dateifenster ein- und ausblenden",
        Kommando::ZweitesFensterUmschalten => "Zweites Dateifenster ein- und ausblenden",
        Kommando::VorschauUmschalten => "Vorschau anzeigen und ausblenden",
        Kommando::FensterEinblenden => "Fenster einblenden",
        Kommando::FensterSchliessen => "Fenster schließen",
        Kommando::BereichVerbreitern => "Aktiven Bereich verbreitern",
        Kommando::BereichVerschmaelern => "Aktiven Bereich verschmälern",
        Kommando::Kopieren => "In das andere Fenster kopieren",
        Kommando::Verschieben => "In das andere Fenster verschieben",
        Kommando::InPapierkorb => "In den Papierkorb räumen",
        Kommando::Abbrechen => "Laufende Operation abbrechen",
        Kommando::OrdnerAnlegen => "Ordner anlegen",
        Kommando::DateiAnlegen => "Leere Datei anlegen",
        Kommando::UmbenennenStapel => "Im Stapel umbenennen",
        Kommando::Umbenennen => "Umbenennen",
        Kommando::TerminalOeffnen => "Ordner im Terminal öffnen",
        Kommando::OrdnerpfadKopieren => "Pfad des angezeigten Ordners kopieren",
        Kommando::EintragspfadKopieren => "Pfad des Eintrags kopieren",
        Kommando::MitStandardprogrammOeffnen => "Mit dem Standardprogramm öffnen",
        Kommando::Teilen => "Teilen",
        Kommando::LesezeichenAnlegen => "Lesezeichen anlegen",
        Kommando::LesezeichenUmbenennen => "Lesezeichen umbenennen",
        Kommando::LesezeichenLoeschen => "Lesezeichen löschen",
        Kommando::LesezeichenHoch => "Lesezeichen nach oben verschieben",
        Kommando::LesezeichenRunter => "Lesezeichen nach unten verschieben",
        Kommando::FokusLeiste => "Fokus in die Lesezeichen- und Geräteleiste",
        Kommando::FokusDateifenster => "Fokus zurück in das Dateifenster",
        Kommando::FokusVorschau => "Fokus in das Vorschaufenster",
        Kommando::Bearbeiten => "Bearbeiten",
        Kommando::EditorRundweg => "In den Editor und zurück",
        Kommando::FokusEditor => "Fokus in den Editor",
        Kommando::EditorSchliessen => "Editor schließen",
        Kommando::EditorUmschalten => "Editor ein- und ausblenden",
        Kommando::EditorAnsichtUmschalten => "Zwischen Roh- und Formatansicht wechseln",
        Kommando::EditorSichern => "Sichern",
        Kommando::EditorZeileSpringen => "Zu Zeile springen",
        Kommando::EditorSuchen => "Im Text suchen",
        Kommando::EditorWeitersuchen => "Weitersuchen",
        Kommando::EditorRueckwaertsSuchen => "Rückwärts weitersuchen",
        Kommando::EditorErsetzen => "Ersetzen",
        Kommando::EditorAlleErsetzen => "Alle ersetzen",
        Kommando::QuicknoteUmschalten => "Quicknote öffnen und schließen",
        Kommando::QuicknoteKopieren => "Quicknote kopieren und schließen",
        Kommando::QuicknoteLeeren => "Quicknote leeren",
        Kommando::EintragHinzufuegen => "Eintrag hinzufügen",
        Kommando::EintragBearbeiten => "Eintrag bearbeiten",
        Kommando::EintragHoch => "Eintrag nach oben",
        Kommando::EintragRunter => "Eintrag nach unten",
        Kommando::EintragLoeschen => "Eintrag löschen",
        Kommando::AufgabeAbhaken => "Aufgabe abhaken oder öffnen",
        Kommando::PinAendern => "PIN ändern",
        Kommando::TermineRichtungUmkehren => "Termine: Sortierrichtung umkehren",
        Kommando::BelegungAnsehen => "Tastaturbelegung anzeigen",
        Kommando::BelegungsdateiAnsehen => "Tastaturdefinition öffnen",
        Kommando::Beenden => "KRK beenden",
        Kommando::WeitereInstanz => "Weitere Instanz starten",
        Kommando::Notizordner => "Notizordner öffnen",
        Kommando::OrtWaehlen => "Ort wählen…",
        Kommando::VorschauVergroessern => "Vorschau vergrößern",
        Kommando::VorschauVerkleinern => "Vorschau verkleinern",
        Kommando::VorschauAusgangsgroesse => "Vorschau in Ausgangsgröße",
        Kommando::GitBereichUmschalten => "Git-Bereich ein- und ausblenden",
        Kommando::FokusGit => "Fokus in den Git-Bereich",
        Kommando::SpalteMarkeUmschalten => "Spalte Marke ein- und ausblenden",
        Kommando::NeuerungenZeigen => "Neuerungen anzeigen",
        Kommando::Werkseinstellungen => "Auf Werkseinstellungen zurücksetzen…",
        Kommando::BildVor => "Nächstes Bild",
        Kommando::BildZurueck => "Voriges Bild",
        Kommando::ZumBild => "Zum angezeigten Bild springen",
    }
}

/// Der deutsche Name einer vom Hauptmenue zugestellten Funktion.
pub(in super::super) const fn zugestellt_name(schluessel: Zugestellt) -> &'static str {
    match schluessel {
        Zugestellt::FilterEinfuegen => "In den Filter einfügen",
        Zugestellt::TextAusschneiden => "Ausschneiden",
        Zugestellt::TextKopieren => "Kopieren",
        Zugestellt::TextEinfuegen => "Einfügen",
        Zugestellt::TextAllesAuswaehlen => "Alles auswählen",
        Zugestellt::TextRueckgaengig => "Rückgängig",
        Zugestellt::TextWiederholen => "Wiederholen",
    }
}
