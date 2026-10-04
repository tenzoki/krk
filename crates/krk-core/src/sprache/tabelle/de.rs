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
        Text::StatuszeileFilterstand => {
            "Filter „{filtertext}“: {gezeigt} von {vorhanden} angezeigt{liest}{zu_gross}{ausgeblendet}"
        }
        Text::StatuszeileInhaltWirdGelesen => ", Inhalt wird gelesen",
        Text::StatuszeileSeiteVon => "Seite {aktuell} von {gesamt}",
        Text::StatuszeileBildVon => "Bild {aktuell} von {gesamt}",
        Text::StatuszeileFolgeGekuerzt => "{grundsatz} (Folge {gruende} gekürzt)",
        Text::StatuszeileLinkesDateifenster => "linkes Dateifenster",
        Text::StatuszeileRechtesDateifenster => "rechtes Dateifenster",
        Text::StatuszeileMeldungMitSeite => "{seite}: {text}",
        Text::VorgangsartKopieren => "Kopieren",
        Text::VorgangsartVerschieben => "Verschieben",
        Text::VorgangsartInDenPapierkorb => "In den Papierkorb räumen",
        Text::VorgangsartUmbenennen => "Umbenennen",
        Text::VorgangsartPacken => "Packen",
        Text::VorgangsartEntpacken => "Entpacken",
        Text::VorgangsartDuplizieren => "Duplizieren",
        Text::VorgangAbbruchhinweis => "Esc bricht ab",
        Text::VorgangWirdVorbereitet => "{was} wird vorbereitet: {positionen} · {abbruch}",
        Text::VorgangZeile => "{was}: {eintraege}, {menge}, {positionen} · {name} · {abbruch}",
        Text::VorgangWirdAbgebrochen => "{was} wird abgebrochen, der Vorgang endet gleich …",
        Text::VorgangSchonEiner => "es läuft bereits eine Operation: {was}",
        Text::VorgangUebertragen => "{eintraege}, {menge} ({positionen})",
        Text::VorgangAbgebrochen => "{was} abgebrochen: {uebertragen} übertragen",
        Text::VorgangFertig => "{was} fertig: {uebertragen}",
        Text::UebersprungenZeile => "{name}: {grund}",
        Text::AnlegenFrageOrdner => "Wie soll der neue Ordner heißen?",
        Text::AnlegenFrageDatei => "Wie soll die neue Datei heißen?",
        Text::AnlegenBestaetigen => "Anlegen",
        Text::AngelegtOrdner => "Ordner „{name}“ angelegt",
        Text::AngelegtDatei => "Datei „{name}“ angelegt",
        Text::AnlegenKeineRechteOrdner => "keine Rechte, hier den Ordner „{name}“ anzulegen",
        Text::AnlegenKeineRechteDatei => "keine Rechte, hier die Datei „{name}“ anzulegen",
        Text::AnlegenGescheitert => "„{name}“ ließ sich nicht anlegen: {fehler}",
        Text::NameSchonVergeben => "es gibt schon einen Eintrag namens „{name}“",
        Text::DuplikatFrage => "Wie soll das Duplikat heißen?",
        Text::DuplikatBestaetigen => "Duplizieren",
        Text::DuplikatMehrere => {
            "nichts zu duplizieren: es sind mehrere Einträge markiert, und dupliziert wird genau eine Datei"
        }
        Text::DuplikatNichtGewoehnlich => {
            "nichts zu duplizieren: „{name}“ ist {typ}, und dupliziert wird allein eine gewöhnliche Datei"
        }
        Text::DuplikatTypOrdner => "ein Ordner",
        Text::DuplikatTypVerknuepfung => "eine Verknüpfung",
        Text::UmbenennenKeineRechte => "keine Rechte, hier in „{name}“ umzubenennen",
        Text::UmbenennenGescheitert => "„{name}“ ließ sich nicht vergeben: {fehler}",
        Text::OrdnerKeinOrdnerMehr => "{pfad} ist kein Ordner mehr",
        Text::OrdnerNichtMehrErreichbar => "{pfad} ist nicht mehr erreichbar: {fehler}",
        Text::KeinTerminal => {
            "keine Anwendung mit der Bündelkennung „{kennung}“ installiert; settings.toml nennt sie unter terminal, eine Änderung wirkt erst nach einem Neustart"
        }
        Text::PfadKopiert => "Pfad kopiert: {pfad}",
        Text::PfadeKopiert => "{n} Pfade kopiert",
        Text::NichtsBetroffen => "nichts {nennform}: nichts markiert und nichts ausgewählt",
        Text::NennformZuKopieren => "zu kopieren",
        Text::NennformZuOeffnen => "zu öffnen",
        Text::NennformZuPacken => "zu packen",
        Text::NennformAnzuzeigen => "anzuzeigen",
        Text::NennformZuDuplizieren => "zu duplizieren",
        Text::NichtsZuTeilen => {
            "nichts zu teilen: hier steht nichts, was an die Freigabedienste ginge"
        }
        Text::KeinArchiv => "nichts zu entpacken: hier steht keine Datei mit der Endung .zip",
        Text::MehrereArchive => {
            "nichts zu entpacken: hier stehen mehrere Archive, und die Auswahl zeigt auf keines"
        }
        Text::KeinFinder => {
            "der Finder ist nicht erreichbar: das System hat keine Anwendung dafür genannt"
        }
        Text::AblageWeistTextAb => "die Zwischenablage hat den Text nicht angenommen",
        Text::AbgelegtEiner => "kopiert: {name}",
        Text::AbgelegtMehrere => "{n} Einträge kopiert",
        Text::AbgelegtAusgeschnitten => "{kopiert} – verschieben tut das Ziel (Finder: opt+cmd+v)",
        Text::AblageWeistVerweiseAb => "die Zwischenablage hat die Einträge nicht angenommen",
        Text::EinfuegenKeinText => "nichts einzufügen: die Zwischenablage trägt keinen Text",
        Text::EinfuegenMehrzeilig => "nicht eingefügt: der Text hat mehrere Zeilen",
        Text::EinfuegenNichtsTragbar => {
            "nichts einzufügen: der Text trägt kein Zeichen, das ein Name tragen kann"
        }
        Text::UebergebenEiner => "an das System übergeben: {name}",
        Text::UebergebenMehrere => "{n} Einträge an das System übergeben",
        Text::NichtAngenommenEiner => "das System hat {name} nicht angenommen",
        Text::NichtAngenommenMehrere => {
            "das System hat {n} von {gesamt} Einträgen nicht angenommen"
        }
        Text::UebergebenUndAbgelehnt => "{genommen}; {abgelehnt}",
        Text::KeineAnwendung => {
            "nichts zu öffnen: das System nennt für diesen Eintrag keine Anwendung"
        }
        Text::UebergebenAnEiner => "an {anwendung} übergeben: {name}",
        Text::UebergebenAnMehrere => "{n} Einträge an {anwendung} übergeben",
        Text::NichtUebergebenAn => {
            "an {anwendung} nicht übergeben: ein Pfad trägt kein gültiges UTF-8"
        }
        Text::BelegungsdateiZweiSchreiber => {
            "{datei} hat zwei Schreiber: eine Änderung von Hand wirkt erst beim nächsten Start, und die Belegungsansicht (F1) überschreibt sie beim Verlassen"
        }
        Text::BelegungsdateiFehltNoch => {
            "{datei} gibt es noch nicht: sie entsteht, sobald die Belegungsansicht (F1) mit einer Änderung verlassen wird"
        }
        Text::BelegungsdateiOhneAblageordner => {
            "{datei} ist nicht zu zeigen: KRK läuft ohne Ablageordner"
        }
        Text::Markierungsstand => "{n} markiert, davon {ordner}, {groesse}",
        Text::LoeschenOhnePapierkorb => {
            "das Ziel führt keinen Papierkorb, es wurde nichts gelöscht; im Finder löschen"
        }
        Text::WarngrundUnentscheidbar => "von einem Ziel unbekannter Einordnung",
        Text::WarngrundNetzlaufwerk => "von einem Netzlaufwerk",
        Text::WarngrundCloudort => "aus einem Cloud-Ordner",
        Text::WarngrundAusserhalbBenutzerordner => "außerhalb des Benutzerordners",
        Text::WarngrundImBenutzerordner => "unmittelbar im Benutzerordner",
        Text::WarngrundArbeitsbaum => "aus einem Git-Arbeitsbaum",
        Text::WarngrundGenauDieSchwelle => "mit 25 Einträgen insgesamt",
        Text::WarngrundMehrAlsDieSchwelle => "mit mehr als 25 Einträgen insgesamt",
        Text::LoeschenGeraeumtAus => "Geräumt wird aus {ordner}.",
        Text::LoeschenAusserdem => "Außerdem: {gruende}.",
        Text::LoeschenDarunterOrdner => "Darunter {ordner}, jeweils mit ihrem gesamten Inhalt.",
        Text::BlattSteht => "nicht ausgeführt: über dem Fenster steht ein Blatt",
        Text::PfadNichtAbsolut => "{pfad} ist kein absoluter Pfad",
        Text::PfadGibtEsNicht => "{pfad} gibt es nicht: {fehler}",
        Text::PfadNichtLesbar => "{pfad} lässt sich nicht lesen: {fehler}",
        Text::PfadInKeinemOrdner => "{pfad} liegt in keinem Ordner",
        Text::WerksSchaltflaeche => "Zurücksetzen",
        Text::WerksNotizordnerBleibt => "Der Notizordner und alle Dateien darin bleiben unberührt.",
        Text::WerksFrage => {
            "readers.toml, settings.toml und keymap.toml auf Werkseinstellungen zurücksetzen?"
        }
        Text::WerksErlaeuterung => {
            "Jede der drei Dateien, die im Ablageordner steht, legt KRK unter ihrem Namen mit angehängtem Zeitstempel beiseite, etwa readers.toml.JJMMTT-HHMM, und löscht keine davon. Danach stehen readers.toml und settings.toml so da, wie diese Fassung von KRK sie mitbringt, nur behält settings.toml den eingestellten Notizordner; keymap.toml fehlt, und es gilt die mitgelieferte Tastenbelegung. {notizordner} KRK liest den neuen Stand sofort ein."
        }
        Text::WerksEigeneZuweisungen => {
            "Alle eigenen Tastenzuweisungen aus keymap.toml gehen damit aus dem Betrieb; sie liegen danach allein in der Sicherung."
        }
        Text::TabAusgefiltert => "{name} ist ausgefiltert.",
        Text::TabNichtMehrDa => "{name} ist nicht mehr da.",
        Text::TabNichtVollstaendigGelesen => "{ordner} ließ sich nicht vollständig lesen: {fehler}",
        Text::PapierkorbKeinUtf8Pfad => "{pfad} ist kein gültiger UTF-8-Pfad",
        Text::BlattSchliessen => "Schließen",
        Text::BlattAbbrechen => "Abbrechen",
        Text::BlattUmbenennen => "Umbenennen",
        Text::BlattFeldSuchenNach => "Suchen nach:",
        Text::BlattFeldErsetzenDurch => "Ersetzen durch:",
        Text::KonfliktUeberspringen => "Überspringen",
        Text::KonfliktInDenPapierkorbUndErsetzen => "In den Papierkorb und ersetzen",
        Text::KonfliktEndgueltigLoeschenUndErsetzen => "Endgültig löschen und ersetzen",
        Text::KonfliktTastenhinweisEinZiel => {
            "Return und Esc brechen ab, Cmd+Return ersetzt, Opt+Return benennt um."
        }
        Text::KonfliktTastenhinweisMehrereZiele => {
            "Return überspringt, Cmd+Return ersetzt, Opt+Return benennt um, Esc bricht ab."
        }
        Text::KonfliktFrage => "„{name}“ gibt es am Ziel schon",
        Text::KonfliktErlaeuterung => "Quelle: {quelle}\nZiel: {ziel}\n\n{hinweis}",
        Text::KonfliktFuerAlleWeiteren => "Für alle weiteren übernehmen",
        Text::LoeschblattErlaeuterung => {
            "{erlaeuterung}\n\nReturn und Esc brechen ab. Zum Bestätigen Cmd+Return."
        }
        Text::NeuerungenBlattFrage => "Ihre Ablagedateien und was diese Fassung mitbringt",
        Text::OrtwahlWaehlen => "Wählen",
        Text::OrtwahlFrage => "Wo soll der Notizordner liegen?",
        Text::PinHinweis => {
            "Die PIN hält Programme und Agenten vom Mitlesen ab. Gegen jemanden, der die Datei kopiert und gezielt angreift, schützt sie nicht. Eine vergessene PIN verschließt den Inhalt endgültig."
        }
        Text::PinAbweichung => "Die beiden Eingaben stimmen nicht überein.",
        Text::PinFrageFestlegen => "Neue PIN für die Geheimnisse festlegen",
        Text::PinFrageEingeben => "PIN für die Geheimnisse eingeben",
        Text::PinFrageAendern => "PIN für die Geheimnisse ändern",
        Text::PinBestaetigenFestlegen => "Festlegen",
        Text::PinBestaetigenEingeben => "Öffnen",
        Text::PinBestaetigenAendern => "Ändern",
        Text::PinFeldNeuePin => "Neue PIN:",
        Text::PinFeldWiederholen => "Wiederholen:",
        Text::PinFeldPin => "PIN:",
        Text::PinFeldAltePin => "Alte PIN:",
        Text::StapelSpalteBisher => "Bisher",
        Text::StapelSpalteNeu => "Neu",
        Text::StapelSpalteHinweis => "Hinweis",
        Text::StapelErlaeuterung => {
            "Die Vorschau zeigt, was der Befehl täte. Umbenannt wird erst mit Return; Esc bricht ab. Einträge mit einem Hinweis bleiben stehen."
        }
        Text::StapelFeldNummerAb => "Nummer ab:",
        Text::StapelFeldStellen => "Stellen:",
        Text::StapelZusammenfassungOhneKollisionen => {
            "{eintraege}, davon {umzubenennen} mit neuem Namen"
        }
        Text::StapelZusammenfassungMitKollisionen => "{eintraege}: {umbenannt}, {stehend}",
        Text::SucheWeitersuchen => "Weitersuchen",
        Text::SucheErsetzen => "Ersetzen",
        Text::SucheAlleErsetzen => "Alle ersetzen",
        Text::SucheErlaeuterung => {
            "Return sucht weiter, Cmd+Return ersetzt den Treffer, Opt+Return ersetzt alle, Esc bricht ab."
        }
        Text::SucheFrage => "Wonach suchen?",
        Text::UngesichertSichern => "Sichern",
        Text::UngesichertVerwerfen => "Verwerfen",
        Text::UngesichertFrage => "„{name}“ hat ungesicherte Änderungen",
        Text::UngesichertErlaeuterung => {
            "{pfad}\n\nReturn sichert, Cmd+Return verwirft die Änderungen, Esc bricht ab."
        }
        Text::FremdaenderungNeuLaden => "Neu laden",
        Text::FremdaenderungUeberschreiben => "Trotzdem überschreiben",
        Text::FremdaenderungFrage => "„{name}“ hat sich außerhalb von KRK geändert",
        Text::FremdaenderungErlaeuterung => {
            "{pfad}\n\nNeu laden verwirft die Änderungen im Editor, Überschreiben ersetzt die Fassung auf der Platte. Return und Esc brechen ab, Opt+Return lädt neu, Cmd+Return überschreibt."
        }
        Text::ZeilennummerFrage => "Zu welcher Zeile?",
        Text::ZeilennummerSpringe => "Springe",
        Text::PfadeingabeFrage => "Zu welchem Ordner?",
        Text::PfadeingabeGehe => "Gehe",
        Text::HinweisOk => "OK",
        Text::EditorTrefferVon => "Treffer {nummer} von {anzahl}",
        Text::EditorKeinTrefferFuer => "Kein Treffer für „{text}“",
        Text::EditorKeinWeitererTrefferFuer => "Kein weiterer Treffer für „{text}“",
        Text::PinBleibt => "die PIN bleibt, wie sie war",
        Text::PinNichtAbleitbarGrund => {
            "die neue PIN lässt sich nicht ableiten: {fehler}; {bleibt}"
        }
        Text::EditorKeineTextmarkeInGeheimnissen => {
            "für secrets.txt legt KRK keine Textmarke an: sie schriebe eine Zeile der Geheimnisse im Klartext in die Lesezeichen"
        }
        Text::EditorOhnePin => "sie ist verschlüsselt und öffnet sich allein mit der PIN",
        Text::EditorKeineVerschluesselteDatei => {
            "sie ist keine verschlüsselte Datei im Notizordner"
        }
        Text::EditorGeheimnisseZuGross => "sie ist zu groß für den Editor",
        Text::EditorGeheimnisseKeinDateizugriff => "KRK hat keinen freien Dateizugriff mehr",
        Text::EditorGeheimnisseNichtLesbar => "sie lässt sich nicht lesen",
        Text::EditorFremdGeaendertNichtUeberschrieben => {
            "{pfad} hat sich außerhalb von KRK geändert und wird nicht überschrieben"
        }
        Text::EditorVerschluesseltKeinKlartext => {
            "{pfad} ist verschlüsselt und wird nicht im Klartext geschrieben"
        }
        Text::EditorNichtGesichert => "{pfad} ließ sich nicht sichern: {fehler}",
        Text::EditorFremdGeaendert => "{pfad} hat sich außerhalb von KRK geändert",
        Text::PinKeineDatei => "der Editor hält keine Datei; {bleibt}",
        Text::PinNochNichtGesichert => {
            "{pfad} trägt noch keine gesicherte PIN; erst sichern, dann ändern"
        }
        Text::PinWirdSchonGeaendert => "die PIN wird schon geändert",
        Text::PinFremdGeaendert => "{pfad} hat sich außerhalb von KRK geändert; {bleibt}",
        Text::PinNichtVerschluesselt => "{pfad} ist nicht verschlüsselt; {bleibt}",
        Text::PinAlteStimmtNicht => "die alte PIN stimmt nicht; {bleibt}",
        Text::PinNichtAbleitbar => "die neue PIN ließ sich nicht ableiten; {bleibt}",
        Text::PinGrundBleibt => "{grund}; {bleibt}",
        Text::PinDateiGrundBleibt => "{pfad}: {grund}; {bleibt}",
        Text::PinNichtMehrOffen => "{pfad} ist nicht mehr offen; {bleibt}",
        Text::PinNichtMehrEntsperrt => "{pfad} ist nicht mehr entsperrt; {bleibt}",
        Text::PinNichtLesbar => "{pfad} lässt sich nicht lesen; {bleibt}",
        Text::PinNichtAlsTextLesbar => "{pfad} ist nicht als Text lesbar; {bleibt}",
        Text::PinNichtGeschrieben => "{pfad} ließ sich nicht schreiben: {fehler}; {bleibt}",
        Text::EditorMarkeFuehrtAufZeile => "die Marke führt auf Zeile {zeile}",
        Text::EditorZeilenZaehlenAbEins => {
            "Zeilen zählen ab 1; die Schreibmarke steht am Dateianfang"
        }
        Text::EditorKeineZeileMehr => {
            "die Datei hat keine Zeile {zeile} mehr; die Schreibmarke steht am Dateiende"
        }
        Text::EditorMarkenstelleGeaendert => "die gemerkte Stelle hat sich geändert; {wohin}",
        Text::EditorGesichert => "{pfad} gesichert",
        Text::EditorKeineZeilennummer => "„{eingabe}“ ist keine Zeilennummer",
        Text::EditorKeineSuche => "es läuft keine Suche",
        Text::EditorKeinTrefferErsetzt => "kein Treffer ersetzt",
        Text::EditorPinGeaendert => "die PIN von {pfad} ist geändert",
        Text::EditorTermineAufsteigend => "Termine aufsteigend sortiert",
        Text::EditorTermineAbsteigend => "Termine absteigend sortiert",
        Text::QuicknoteLeer => "Die Quicknote ist leer; die Zwischenablage bleibt, wie sie war.",
        Text::QuicknoteNichtKopiert => {
            "Die Quicknote ließ sich nicht in die Zwischenablage kopieren; ihr Text bleibt stehen."
        }
        Text::QuicknoteZuGross => "Der Text ist zu groß für die Quicknote; eingefügt wurde nichts.",
        Text::QuicknoteKeineTextmarken => "In der Quicknote gibt es keine Textmarken.",
        Text::QuicknoteLeeren => "Leeren",
        Text::QuicknoteKopieren => "Kopieren",
        Text::EintragKeineAufgabentabelle => "der Editor zeigt keine Aufgabentabelle",
        Text::EintragKeineNotiztabelle => "der Editor zeigt keine Notiztabelle",
        Text::EintragKeineTermintabelle => "der Editor zeigt keine Termintabelle",
        Text::EintragKeineAufgabeGewaehlt => "es ist keine Aufgabe gewählt",
        Text::EintragKeineNotizGewaehlt => "es ist keine Notiz gewählt",
        Text::EintragKeinTerminGewaehlt => "es ist kein Termin gewählt",
        Text::EintragAufgabeHinzugefuegt => "neue Aufgabe am Ende; return übernimmt den Text",
        Text::EintragNotizHinzugefuegt => {
            "neue Notiz am Ende; tab wechselt zum Text, cmd+return übernimmt"
        }
        Text::EintragTerminHinzugefuegt => {
            "Termin hinzugefügt, mit dem heutigen Datum; tab wechselt zum Text, cmd+return übernimmt"
        }
        Text::EintragAufgabeBearbeitung => "return übernimmt, esc verwirft",
        Text::EintragNotizBearbeitung => {
            "cmd+return übernimmt, tab wechselt die Zelle, return schreibt im Text einen Zeilenumbruch"
        }
        Text::EintragTerminBearbeitung => {
            "cmd+return übernimmt, tab wechselt die Zelle, return schreibt im Termin einen Zeilenumbruch"
        }
        Text::EintragAufgabeUebernommen => "Aufgabe übernommen",
        Text::EintragNotizUebernommen => "Notiz übernommen",
        Text::EintragTerminUebernommen => "Termin übernommen",
        Text::EintragAufgabeAbgehakt => "Aufgabe abgehakt",
        Text::EintragNotizOhneKaestchen => "eine Notiz hat kein Kästchen",
        Text::EintragTerminOhneKaestchen => "ein Termin hat kein Kästchen",
        Text::EintragAufgabeWiederOffen => "Aufgabe wieder offen",
        Text::EintragAufgabeVerschoben => "Aufgabe verschoben",
        Text::EintragNotizVerschoben => "Notiz verschoben",
        Text::EintragAufgabeSchonOben => "die Aufgabe steht schon oben",
        Text::EintragNotizSchonOben => "die Notiz steht schon oben",
        Text::EintragAufgabeSchonUnten => "die Aufgabe steht schon unten",
        Text::EintragNotizSchonUnten => "die Notiz steht schon unten",
        Text::EintragTermineNachDatum => {
            "Termine stehen nach ihrem Datum; verschieben lässt sich keiner."
        }
        Text::EintragAufgabeGeloescht => "Aufgabe gelöscht; cmd+z holt sie zurück",
        Text::EintragNotizGeloescht => "Notiz gelöscht; cmd+z holt sie zurück",
        Text::EintragTerminGeloescht => "Termin gelöscht; cmd+z holt ihn zurück",
        Text::EintragZelleBleibt => "die Zelle bleibt in Bearbeitung",
        Text::EintragAufgabeMitEscUebernommen => "Aufgabe übernommen; cmd+z nimmt es zurück",
        Text::EintragNotizMitEscUebernommen => "Notiz übernommen; cmd+z nimmt es zurück",
        Text::EintragTerminMitEscUebernommen => "Termin übernommen; cmd+z nimmt es zurück",
        Text::EintragHeuteUnbestimmt => "Das heutige Datum ließ sich nicht bestimmen.",
        Text::VorschauGeheimnishinweis => {
            "Diese Datei ist verschlüsselt und öffnet sich mit F4 und der PIN im Editor."
        }
        Text::VorschautabLeer => "Leer",
        Text::VorschautabZwischenablage => "Zwischenablage",
        Text::VorschauZwischenablageLeer => "Die Zwischenablage ist leer.",
        Text::VorschauBildZuGross => {
            "Das Bild in der Zwischenablage ist {groesse} MB groß. Die Vorschau zeigt Bilder bis {grenze} MB."
        }
        Text::VorschauNichtLesbar => "{pfad} ließ sich nicht lesen: {fehler}",
        Text::VorschauLeertext => "Kein Inhalt. Die Auswahl im Dateifenster füllt diesen Tab.",
        Text::VorschauBildNichtDarstellbar => {
            "Das Bild aus der Zwischenablage ließ sich nicht darstellen."
        }
        Text::MetadatenName => "Name: {name}",
        Text::MetadatenPfad => "Pfad: {pfad}",
        Text::MetadatenGroesse => "Größe: {groesse}",
        Text::MetadatenGeaendert => "Geändert: {datum}",
        Text::MetadatenRechte => "Rechte: {rechte}",
        Text::MetadatenTyp => "Typ: {typ}",
        Text::StartAblageordnerNichtGeoeffnet => {
            "der Ablageordner ließ sich nicht öffnen, die Sitzung wird nicht gesichert: {fehler}"
        }
        Text::OrtUrsacheAblageordnerNichtGeoeffnet => {
            "der Ablageordner ließ sich nicht öffnen: {fehler}"
        }
        Text::StartSchreibsperreNichtGenommen => {
            "die Schreibsperre der Ablage lässt sich nicht nehmen, es wird nichts geladen und nichts gesichert: {fehler}"
        }
        Text::OrtUrsacheSchreibsperreNichtGenommen => {
            "die Schreibsperre der Ablage ließ sich nicht nehmen: {fehler}"
        }
        Text::OrtUrsacheStartNochNichtGelesen => "der Start hat sie noch nicht gelesen",
        Text::LesezeichenZielFehlt => "„{name}“ fehlt: {pfad} gibt es nicht mehr",
        Text::LesezeichenGesperrt => {
            "die Lesezeichen ließen sich nicht ändern, die Schreibsperre der Ablage ist nicht zu nehmen: {fehler}"
        }
        Text::LesezeichenVonAndererInstanzGeaendert => {
            "dieses Lesezeichen steht nicht mehr so in der Liste; eine andere Instanz von KRK hat es geändert oder gelöscht"
        }
        Text::LesezeichenNichtGesichert => "die Lesezeichen ließen sich nicht sichern: {fehler}",
        Text::LesezeichenNameFrage => "Wie soll das Lesezeichen heißen?",
        Text::LesezeichenAngelegt => "Lesezeichen „{name}“ angelegt",
        Text::EditorHaeltKeineDatei => "der Editor hält keine Datei",
        Text::HinweisTastenabgriffTitel => "KRK kann keine Tastendrücke lesen",
        Text::HinweisTastenabgriffText => {
            "Der Tastenabgriff ließ sich nicht einrichten. Ohne ihn bewegt keine Taste die Auswahl, und kein Tastenkürzel wirkt. KRK wird beendet, statt mit einem Fenster ohne Tastatursteuerung weiterzulaufen."
        }
        Text::OrdnerNichtBeobachtet => {
            "die Ordner lassen sich nicht beobachten; fremde Änderungen erscheinen erst nach einem Ordnerwechsel"
        }
        Text::KeineAngezeigteDatei => "keine angezeigte Datei, zu der gesprungen werden könnte",
        Text::BildfolgeNochInVorbereitung => "Die Bildfolge wird noch vorbereitet.",
        Text::AngleichenFensterZuSchmal => {
            "das Fenster ist zu schmal; es wurde nichts eingeblendet und nichts gestellt"
        }
        Text::AngleichenZeigtSchon => "das andere Dateifenster zeigt diesen Ordner bereits",
        Text::AngleichenEingeblendetZeigtSchon => {
            "das andere Dateifenster wurde eingeblendet und zeigt diesen Ordner bereits"
        }
        Text::NeuerungenOhneAblageordner => {
            "es gibt keinen Ablageordner, und damit keine Dateien, die hinter dieser Fassung zurückliegen könnten"
        }
        Text::NeuerungenGesperrt => {
            "die Neuerungen lassen sich nicht nachsehen: die Schreibsperre der Ablage lässt sich nicht nehmen ({fehler})"
        }
        Text::WerksOhneAblageordner => {
            "Es gibt keinen Ablageordner, und damit nichts zurückzusetzen."
        }
        Text::WerksGesperrt => {
            "Nichts ist zurückgesetzt: die Schreibsperre der Ablage lässt sich nicht nehmen ({fehler})."
        }
        Text::BelegungNichtGesichert => "die Belegung gilt, ließ sich aber nicht sichern: {fehler}",
        Text::BelegungOhneAblageordner => {
            "die Belegung gilt, ist aber ohne Ablageordner nicht gesichert und geht mit dem Beenden verloren"
        }
        Text::BelegungGesperrt => {
            "die Belegung gilt, ist aber nicht gesichert: die Schreibsperre der Ablage lässt sich nicht nehmen ({fehler})"
        }
        Text::OrtKeinUtf8 => {
            "Der gewählte Ort lässt sich nicht in settings.toml schreiben: sein Pfad ist kein gültiges UTF-8"
        }
        Text::OrtOhneAblageordner => {
            "Es gibt keinen Ablageordner und damit keine settings.toml, in die KRK den Ort schreiben könnte; der Notizordner bleibt, wo er ist"
        }
        Text::OrtGesperrt => {
            "Der Ort lässt sich nicht in settings.toml schreiben: die Schreibsperre der Ablage lässt sich nicht nehmen ({fehler}); der Notizordner bleibt, wo er ist"
        }
        Text::LoeschblattSchaltflaeche => "In den Papierkorb räumen",
        Text::NichtsAusgewaehlt => "es ist nichts ausgewählt",
        Text::StapelKeineRegel => {
            "nichts umzubenennen: aus den Feldern ließ sich keine Regel bauen"
        }
        Text::StapelJedeZeileMitHinweis => "nichts umzubenennen: jede Zeile trägt einen Hinweis",
        Text::QuelleUndZielDerselbeOrdner => "Quelle und Ziel sind derselbe Ordner",
        Text::VorgangNichtGestartet => "die Operation ließ sich nicht starten: {fehler}",
        Text::VorschauKeineDateiZumBearbeiten => "die Vorschau zeigt keine Datei zum Bearbeiten",
        Text::QuicknoteFensterZuSchmal => "Für die Quicknote ist das Fenster zu schmal.",
        Text::SitzungNichtGesichert => "die Sitzung ließ sich nicht sichern: {fehler}",
        Text::WeitereInstanzOhneBuendel => {
            "KRK läuft nicht aus einem Bündel; eine weitere Instanz startet nur das gebaute KRK.app"
        }
        Text::AuswurfDateifensterZeigt => {
            "{name} wurde ausgeworfen; das Dateifenster zeigt jetzt {ziel}"
        }
        Text::StartOhneSitzungsrecht => {
            "eine weitere Instanz von KRK läuft schon; Tabs und Aufteilung dieses Fensters werden nicht gesichert"
        }
        Text::StartNeuerungenNichtVermerkt => {
            "es ließ sich nicht vermerken, dass die Neuerungen dieser Fassung gemeldet sind; die Meldung kommt beim nächsten Start wieder: {fehler}"
        }
        Text::StartSitzungsrechtNichtAngefordert => {
            "das Sitzungsrecht lässt sich nicht anfordern, die Sitzung wird nicht gesichert: {fehler}"
        }
        Text::StartLesezeichenNichtGeladen => {
            "die Lesezeichen ließen sich nicht laden, die Schreibsperre der Ablage ist nicht zu nehmen: {fehler}"
        }
        Text::HeimOhneSperreAngelegt => {
            "die Schreibsperre der Ablage lässt sich nicht nehmen ({fehler}); F2 hat ohne sie angelegt"
        }
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
        Zahlwort::StatuszeileDateienZuGross => (", eine Datei zu groß", ", {n} Dateien zu groß"),
        Zahlwort::StatuszeileMarkierungenAusgeblendet => (
            ", eine Markierung ausgeblendet",
            ", {n} Markierungen ausgeblendet",
        ),
        Zahlwort::BildfolgeGrenzeFotos => ("nach {n} Foto", "nach {n} Fotos"),
        Zahlwort::BildfolgeGrenzeOrdner => ("nach {n} Ordner", "nach {n} Ordnern"),
        Zahlwort::BildfolgeGrenzeEintraege => (
            "an {n} Eintrag eines Ordners",
            "an {n} Einträgen eines Ordners",
        ),
        Zahlwort::Eintraege => ("ein Eintrag", "{n} Einträge"),
        Zahlwort::AusgewaehltePositionen => {
            ("eine ausgewählte Position", "{n} ausgewählte Positionen")
        }
        Zahlwort::Ordner => ("ein Ordner", "{n} Ordner"),
        Zahlwort::VorgangUebersprungen => {
            (", ein Eintrag übersprungen", ", {n} Einträge übersprungen")
        }
        Zahlwort::VorgangAusgelassen => (
            ", ein Eintrag als Ziel dieses Laufs ausgelassen",
            ", {n} Einträge als Ziel dieses Laufs ausgelassen",
        ),
        Zahlwort::UebersprungenFrage => (
            "Ein Eintrag wurde übersprungen",
            "{n} Einträge wurden übersprungen",
        ),
        Zahlwort::EinfuegenDateiverweise => (
            "nicht eingefügt: die Zwischenablage trägt {n} Dateiverweis",
            "nicht eingefügt: die Zwischenablage trägt {n} Dateiverweise",
        ),
        Zahlwort::LoeschfrageEintraege => (
            "Diesen Eintrag {grund}in den Papierkorb räumen?",
            "Diese {n} Einträge {grund}in den Papierkorb räumen?",
        ),
        Zahlwort::StartMeldungen => (
            "Beim Start gab es eine Meldung",
            "Beim Start gab es {n} Meldungen",
        ),
        Zahlwort::StapelFrage => (
            "Einen Eintrag umbenennen",
            "{n} Einträge im Stapel umbenennen",
        ),
        Zahlwort::StapelEintraege => ("{n} Eintrag", "{n} Einträge"),
        Zahlwort::StapelWerdenUmbenannt => ("{n} wird umbenannt", "{n} werden umbenannt"),
        Zahlwort::StapelBleibenStehen => ("{n} bleibt stehen", "{n} bleiben stehen"),
        Zahlwort::EditorTrefferErsetzt => ("ein Treffer ersetzt", "{n} Treffer ersetzt"),
        Zahlwort::EditorZeilenHinterDerLetzten => (
            "die Datei hat {n} Zeile; die Schreibmarke steht am Dateiende",
            "die Datei hat {n} Zeilen; die Schreibmarke steht am Dateiende",
        ),
        Zahlwort::QuicknoteKopiert => (
            "Die Quicknote ist in der Zwischenablage: {n} Zeichen.",
            "Die Quicknote ist in der Zwischenablage: {n} Zeichen.",
        ),
        Zahlwort::BildfolgeVorbereitet => (
            "Die Bildfolge wird vorbereitet: {n} Foto.",
            "Die Bildfolge wird vorbereitet: {n} Fotos.",
        ),
        Zahlwort::AuswurfDateifensterUndVerdeckteTabs => (
            "{name} wurde ausgeworfen; das Dateifenster und ein verdeckter Tab zeigen jetzt {ziel}",
            "{name} wurde ausgeworfen; das Dateifenster und {n} verdeckte Tabs zeigen jetzt {ziel}",
        ),
        Zahlwort::AuswurfVerdeckteTabs => (
            "{name} wurde ausgeworfen; ein verdeckter Tab zeigt jetzt {ziel}",
            "{name} wurde ausgeworfen; {n} verdeckte Tabs zeigen jetzt {ziel}",
        ),
        Zahlwort::GekuerztWeitere => ("… und {n} weitere", "… und {n} weitere"),
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
