//! Die eine Antwort darauf, welche Automatiken an einer bearbeitbaren
//! Textflaeche abgeschaltet gehoeren.
//!
//! ```text
//!   editor::textflaeche_bauen ──┐
//!                               ├──> automatiken_abschalten(&NSTextView)
//!   blaetter::zettel::zeigen ───┘          │
//!                                          └─> setzen_falls_vorhanden
//! ```
//!
//! **Ein Modul, weil es zwei Flaechen gibt.** Bis zur Runde 8 stand die Frage
//! mitten in [`super::editor::textflaeche_bauen`], und das war richtig, solange
//! es genau eine bearbeitbare `NSTextView` gab. Mit dem Notizzettel der Runde 9
//! gibt es zwei, und zwei Aufzaehlungen derselben Einstellungen waeren zwei
//! Wahrheiten darueber, was „abgeschaltet" heisst. Der Zettel haelt Pfade und
//! Ausschnitte aus Code; typografische Anfuehrungszeichen darin sind derselbe
//! Schaden wie in einer Datei des Editors.
//!
//! **Die Vorschau ist keine dieser Flaechen und darf es nicht werden.** Sie
//! setzt `setEditable(false)` und `setSelectable(false)`, damit sie den Fokus
//! nicht als Textsystem nimmt; was der Nutzer dort nicht tippen kann, kann auch
//! keine Automatik veraendern.
//!
//! **Was hier steht und was im Editor bleibt.** Hier steht allein, was jede
//! bearbeitbare Flaeche gleich haben muss. Alles Flaechenspezifische bleibt bei
//! seiner Flaeche: die Bildlaufansicht, der Rueckgaengigverlauf, der Zugriff auf
//! `layoutManager`, die Schrift und die Nummernspalte stehen weiterhin in
//! [`super::editor`], und der Zettel setzt seine eigenen.
//!
//! **Die Aufstellung `EINSTELLUNGEN` bleibt, wo sie ist.** Sie steht unter
//! `mod tests` in [`super::editor`] und ordnet jeden der sechsunddreissig
//! Selektoren der Vererbungskette einzeln ein; eine zweite Aufstellung neben ihr
//! waere die zweite Wahrheit, die dieses Modul gerade beseitigt. Gemessen wird
//! ebenfalls dort, an einer gebauten Flaeche und nicht an dieser Datei.
//!
//! **Der Bau haelt den Aufruf nicht, eine Zaehlprobe haelt ihn.** Wer eine
//! dritte bearbeitbare Flaeche baut und [`automatiken_abschalten`] vergisst,
//! bekommt vom Uebersetzer kein Wort; die Probe unter `mod tests` faellt
//! stattdessen beim ersten `make check`. Ihre Grenze steht in ihrem
//! Doc-Kommentar. Entschieden am 260814 vom Nutzer, Moeglichkeit 2 aus
//! `decisions/260814-0656_*_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md`;
//! Moeglichkeit 3, ein eigener Typ, an dem der Uebersetzer haelt, ist dort als
//! die einzige benannt, die wirklich traegt, und als heute zu teuer verworfen.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTextView`, `NSString`, `NSNumber` und `NSObject` stehen seit macOS 10.0
//! zur Verfuegung, ebenso `setRichText:`, `setSmartInsertDeleteEnabled:`,
//! `respondsToSelector:` und `setValue:forKey:`. Die vier tippenden Setzer
//! `setAutomaticQuoteSubstitutionEnabled:`,
//! `setAutomaticDashSubstitutionEnabled:`,
//! `setAutomaticTextReplacementEnabled:` und
//! `setAutomaticSpellingCorrectionEnabled:` stehen seit 10.5 (`NSTextView.h`).
//!
//! **Drei Beruehrungen sind juenger**, und alle drei liegen auf oder unter dem
//! Zielsystem: `setInlinePredictionType:` seit 14.0 und
//! `setMathExpressionCompletionType:` seit 15.0 (`NSTextInputTraits.h`),
//! `setWritingToolsBehavior:` seit 15.0 (`NSTextView.h`).
//!
//! **Eine liegt darueber und geht deshalb nicht ueber einen Aufruf**:
//! `setAllowsWritingToolsAffordance:` fuehrt das SDK allein an `NSTextField` und
//! erst ab 15.4. Sie laeuft ueber [`setzen_falls_vorhanden`], das
//! `respondsToSelector:` vorher fragt; der Grund im Einzelnen steht dort.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine der gerufenen
//! Methoden ist nach macOS 15 hinzugekommen, und keine Beruehrung in dieser
//! Datei ausser der einen genannten braucht deshalb eine Verfuegbarkeitspruefung
//! zur Laufzeit. `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der
//! Uebersetzer haelt die Untergrenze nicht; die Nennung hier ist die
//! Gegenmassnahme.

use std::ffi::CString;

use objc2::msg_send;
use objc2::runtime::Sel;
use objc2_app_kit::{NSTextInputTraitType, NSTextView, NSWritingToolsBehavior};
use objc2_foundation::{NSNumber, NSObjectProtocol, NSString};

/// Schaltet an einer bearbeitbaren Textflaeche alles ab, was den getippten Text
/// veraendern wuerde.
///
/// **Reiner Text und die sieben Automatiken aus: der gesicherte Stand ist der
/// getippte.** Eine typografische Ersetzung von Anfuehrungszeichen oder
/// Bindestrichen aendert Programmtext still, und beide Flaechen dieses
/// Programms schreiben ihren Inhalt Zeichen fuer Zeichen in eine Datei zurueck.
///
/// **Die sieben zerfallen in drei Gruppen, und jede folgende war ueber der
/// vorigen uebersehen.** Vier greifen beim **Tippen**: Anfuehrungszeichen,
/// Bindestriche, Textersetzung, Rechtschreibkorrektur. Die fuenfte,
/// `smartInsertDeleteEnabled`, greift beim **Einfuegen und Ausschneiden** — sie
/// setzt ein Leerzeichen vor oder hinter ein eingefuegtes Wort und nimmt beim
/// Ausschneiden ein ueberzaehliges fort; sie blieb an, weil die Aufzaehlung nach
/// den vier tippenden aufhoerte
/// (`issues/260809-1650_*_die-fuenfte-textveraendernde-automatik-smart-insert-delete-bleibt-an.md`).
/// Die sechste und die siebte, `inlinePredictionType` (macOS 14) und
/// `mathExpressionCompletionType` (macOS 15), blieben an, weil die Aufzaehlung
/// nach den fuenf mit `set…Enabled:` aufhoerte
/// (`issues/260810-0416_*_zwei-weitere-textveraendernde-automatiken-ohne-enabled-schalter-bleiben-an.md`).
///
/// **Die achte und die neunte sind die Schreibwerkzeuge**, und der Unterschied
/// ist, wer sie ausloest: sie greifen erst auf einen ausdruecklichen Aufruf des
/// Nutzers aus dem Kontextmenue. Dass C4 der Editor-Runde sie trotzdem
/// ausschliesst, ist eine Lesart und war eine Frage an den Nutzer; entschieden
/// am 260810 fuer den Ausschluss
/// (`decisions/260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md`).
///
/// **Was hier nicht steht**: `setEnabledTextCheckingTypes:`, die Sammeltuer ueber
/// mehrere Automatiken auf einmal. Sie waere eine zweite Stelle mit einer Meinung
/// darueber, was abgeschaltet ist, und die einzelnen Zeilen sind die erste. Ihre
/// Einordnung steht in `EINSTELLUNGEN` unter `mod tests` in [`super::editor`].
pub(crate) fn automatiken_abschalten(text: &NSTextView) {
    // Reiner Text, und die sieben Automatiken aus. Der Grund steht darueber.
    text.setRichText(false);
    // Die vier, die beim Tippen greifen.
    text.setAutomaticQuoteSubstitutionEnabled(false);
    text.setAutomaticDashSubstitutionEnabled(false);
    text.setAutomaticTextReplacementEnabled(false);
    text.setAutomaticSpellingCorrectionEnabled(false);
    // Die fuenfte greift beim Einfuegen und Ausschneiden statt beim Tippen und
    // steht deshalb fuer sich. Ab Werk ist sie **an**; ohne diese Zeile setzte
    // ein Einfuegen ein Leerzeichen dazu, das niemand getippt hat.
    text.setSmartInsertDeleteEnabled(false);
    // Die sechste und die siebte tragen keinen Schalter der Form `set…Enabled:`
    // und waeren deshalb beinahe stehen geblieben. Die Vorhersage im Textfluss
    // schlaegt die Fortsetzung eines Wortes grau vor und traegt sie ein, sobald
    // der Nutzer die Leer- oder die Tabulatortaste drueckt; die Auswertung von
    // Rechenausdruecken ersetzt beim Tippen von `=` den Ausdruck davor durch
    // sein Ergebnis, und in `wert=1+2` einer Konfigurationsdatei ist das eine
    // Aenderung, die niemand getippt hat. Beide stehen ab Werk auf `Default`,
    // ueberlassen die Wahl also dem System; `No` ist die Absage. Gemessen an
    // derselben Flaeche, nicht der Dokumentation entnommen.
    text.setInlinePredictionType(NSTextInputTraitType::No);
    text.setMathExpressionCompletionType(NSTextInputTraitType::No);
    // Die achte: die Schreibwerkzeuge aus macOS 15. Sie schreiben markierten Text
    // um und wirken beim Korrekturlesen ueber eine ganze Datei; danach steht in
    // `NSTextView::string` nicht mehr das Getippte, und beim Sichern geht es so
    // in die Datei. `None` ist die Absage, `Default` (der Werkswert) ueberliesse
    // dem System die Wahl. Die Faehigkeit ist damit nicht verloren: sie steht in
    // jedem anderen Textfeld des Systems, nur nicht an einer Flaeche, deren
    // Inhalt Zeichen fuer Zeichen in eine Datei zurueckgeschrieben wird.
    text.setWritingToolsBehavior(NSWritingToolsBehavior::None);
    // Und die neunte, die Angebotsflaeche derselben Sache: das Sinnbild, das die
    // Schreibwerkzeuge von sich aus anbietet. Sie steht ab Werk **an** — gemessen,
    // nicht der Dokumentation entnommen — und haengt nicht am Verhalten darueber:
    // `setWritingToolsBehavior(None)` laesst sie unberuehrt, sie ist also keine
    // zweite Tuer, sondern eine eigene Einstellung mit eigenem Aus-Wert.
    //
    // Sie geht ueber den gehueteten Weg und nicht ueber einen Aufruf, weil sie
    // erst ab macOS 15.4 zugesagt ist; der Grund steht bei
    // [`setzen_falls_vorhanden`].
    setzen_falls_vorhanden(text, "allowsWritingToolsAffordance", false);
    // Die beiden uebrigen Schreibwerkzeug-Einstellungen bekommen **keine** Zeile,
    // und das ist gemessen und nicht vergessen: `allowedWritingToolsResultOptions`
    // und `writingToolsAllowedInputOptions` sind Bitmasken, deren Null
    // `…ResultDefault` heisst — "das System waehlt" — und nicht "nichts". Einen
    // Wert, der nichts zulaesst, fuehrt die Aufzaehlung nicht, und beide stehen ab
    // Werk schon auf Null. Eine Zeile waere hier ein Aufruf ohne Wirkung; was die
    // Schreibwerkzeuge abschaltet, sind die beiden Zeilen darueber. Die Einordnung
    // und ihre Probe stehen unter `mod tests` in [`super::editor`] als
    // `Einordnung::Gegenstandslos`.
}

/// Setzt einen booleschen Schalter der Textflaeche, **falls diese Laufzeit ihn
/// fuehrt**, und meldet, ob sie es tat.
///
/// # Warum gefragt und nicht gerufen wird
///
/// `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt
/// die Untergrenze des Buendels deshalb nicht (`CLAUDE.md`, Technologiewahl). Ein
/// Setzer, den das System nicht kennt, ist kein Fehler zur Uebersetzungszeit,
/// sondern ein `doesNotRecognizeSelector:` zur Laufzeit — also ein Absturz auf
/// dem Geraet des Nutzers.
///
/// Der eine Aufrufer ist [`automatiken_abschalten`] mit
/// `setAllowsWritingToolsAffordance:`, und der braucht die Frage: das SDK fuehrt
/// die Angebotsflaeche allein an `NSTextField` und erst ab **macOS 15.4**, das
/// Buendel zielt auf **15.0**. Die Laufzeit von 15.7.7 antwortet an `NSTextView`
/// darauf, aber undokumentiert; auf 15.0 bis 15.3 kann sie fehlen. `objc2` bildet
/// den Setzer an `NSTextView` folgerichtig nicht ab, weshalb der Weg ueber
/// `msg_send!` und nicht ueber eine erzeugte Methode laeuft.
///
/// **Fehlt der Setzer, ist nichts zu tun, und das ist keine Notluege.** Eine
/// Einstellung, die es nicht gibt, aendert kein Zeichen — dieselbe Begruendung,
/// mit der die Gegenrichtung des Stolperdrahts ein Hinweis und kein Fehlschlag
/// ist (Defekt 260810-0417). Der Rueckgabewert ist fuer die Proben da, die
/// zwischen "steht aus" und "gibt es nicht" unterscheiden muessen.
///
/// **Der gehuetete Weg ist einer und nicht zwei.** Die lesende Seite derselben
/// Frage ist `merkmal_falls_vorhanden` unter `mod tests` in [`super::editor`];
/// beide fragen `respondsToSelector:` nach demselben, von [`setzername`]
/// gebildeten Setzer und sonst nichts. Wer eine dritte Beruehrung einer Methode
/// ueber der Untergrenze braucht, nimmt diese Frage und baut keine
/// Versionsabfrage daneben.
///
/// # Warum ueber `setValue:forKey:` und nicht ueber den Setzer selbst
///
/// Weil `objc2` den Setzer an `NSTextView` nicht abbildet, muesste ein
/// unmittelbarer Ruf durch `msg_send!` — und dessen Selektor steht zur
/// Uebersetzungszeit fest, waehrend die Frage davor zur Laufzeit faellt. Ein
/// dynamischer Selektor mit einem `BOOL`-Argument ist in `objc2` nur ueber rohes
/// `objc_msgSend` samt Umdeutung des Funktionszeigers zu haben, und
/// `performSelector:withObject:` ist **keine** Ausweiche: es uebergibt einen
/// Objektzeiger, wo der Setzer ein `BOOL` erwartet, und ein Zeiger ist nie
/// `NO`. Die Schluesselwertkodierung nimmt dagegen die `NSNumber`, liest die
/// Typkodierung des Merkmals und packt den Wahrheitswert selbst aus — derselbe
/// Weg, den `merkmal_setzen` unter `mod tests` in [`super::editor`] schon geht.
pub(crate) fn setzen_falls_vorhanden(flaeche: &NSTextView, merkmal: &str, wert: bool) -> bool {
    let setzer = CString::new(setzername(merkmal)).expect("ein Setzername traegt kein Nullbyte");
    if !flaeche.respondsToSelector(Sel::register(&setzer)) {
        return false;
    }
    let schluessel = NSString::from_str(merkmal);
    let zahl = NSNumber::new_bool(wert);
    // SAFETY: Das Merkmal traegt an dieser Flaeche einen Setzer — eben gefragt —,
    // und damit ist es schluesselwertkodiert erreichbar; `setValue:forKey:` steht
    // an `NSObject` seit macOS 10.0.
    let _: () = unsafe { msg_send![flaeche, setValue: &*zahl, forKey: &*schluessel] };
    true
}

/// `smartQuotesType` wird zu `setSmartQuotesType:` — die eine kanonische Form, in
/// der dieses Projekt eine Einstellung benennt.
///
/// Die Regel ist die, die Objective-C selbst anwendet, wenn eine Eigenschaft
/// keinen eigenen Setzer nennt: `set`, erster Buchstabe gross, Doppelpunkt. Sie
/// steht neben [`setzen_falls_vorhanden`] und nicht unter `mod tests`, weil beide
/// Seiten der Frage — die setzende hier, die lesende im Pruefmodul von
/// [`super::editor`] — **denselben** Namen bilden muessen; sonst fragte die eine
/// nach einem anderen Selektor als die andere setzt.
pub(crate) fn setzername(merkmal: &str) -> String {
    let mut zeichen = merkmal.chars();
    let erstes = zeichen.next().expect("ein Merkmalsname ist nicht leer");
    format!("set{}{}:", erstes.to_uppercase(), zeichen.as_str())
}

#[cfg(test)]
mod tests {
    use crate::quellbaum::quelldateien;

    use super::setzername;

    /// Jede Datei, die eine `NSTextView` bearbeitbar macht, ruft auch die
    /// Abschaltung der Automatiken.
    ///
    /// Der Bau haelt das nicht: eine bearbeitbare Textflaeche ohne abgeschaltete
    /// Automatiken uebersetzt anstandslos, und der Schaden ist still — ein
    /// Zeichen in einer Datei des Nutzers, das er nicht getippt hat, und kein
    /// Fehlschlag irgendwo. Entschieden am 260814 vom Nutzer als Moeglichkeit 2
    /// aus
    /// `decisions/260814-0656_*_wird-die-abschaltung-der-textautomatiken-bauanhaltend.md`.
    ///
    /// # Was diese Nadel nicht sieht
    ///
    /// **Sie bindet an zwei Schreibweisen, und das ist ihre Grenze.** Gesucht
    /// wird eine Datei, die eine `NSTextView` **anlegt** und in einer Codezeile
    /// `setEditable(true)` schreibt. Eine Flaeche, die ihre Bearbeitbarkeit ueber
    /// `setValue:forKey:` setzt, die den Aufruf ueber zwei Zeilen umbricht oder
    /// die sie in einer anderen Datei einschaltet als der, die sie anlegt,
    /// entgeht ihr vollstaendig. Der Kopf von `crates/krk-core/tests/baum.rs`
    /// sagt, warum keine Nadel restlos dicht ist; hier steht der blinde Fleck,
    /// statt vom Namen der Probe ueberschrieben zu werden.
    ///
    /// **Und sie sieht nur `NSTextView`.** Ein bearbeitbares `NSTextField` —
    /// `appkit/tabelle.rs` traegt eines, das Umbenennen in der Liste — faellt aus
    /// der Frage heraus: [`super::automatiken_abschalten`] nimmt eine
    /// `NSTextView` entgegen, und der Feldeditor eines Textfeldes gehoert dem
    /// Fenster und nicht der Zelle. Ob die Automatiken **dort** abgeschaltet
    /// gehoeren, ist eine eigene Frage; diese Probe beantwortet sie nicht und
    /// behauptet auch nicht, sie sei gestellt.
    #[test]
    fn jede_bearbeitbare_textflaeche_schaltet_die_automatiken_ab() {
        let angelegt = concat!("NSTextView::", "alloc");
        let bearbeitbar = concat!("setEditable(", "true)");
        let abschaltung = concat!("automatiken_", "abschalten");

        let ohne: Vec<String> = quelldateien()
            .into_iter()
            .filter(|(_, inhalt)| inhalt.contains(angelegt))
            .filter(|(_, inhalt)| {
                inhalt.lines().any(|zeile| {
                    !zeile.trim_start().starts_with("//") && zeile.contains(bearbeitbar)
                })
            })
            .filter(|(_, inhalt)| !inhalt.contains(abschaltung))
            .map(|(name, _)| name)
            .collect();

        assert!(
            ohne.is_empty(),
            "diese Dateien legen eine bearbeitbare NSTextView an, ohne die Automatiken \
             abzuschalten: {ohne:?}"
        );
    }

    /// Die kanonische Namensform, an der die setzende und die lesende Seite
    /// haengen.
    ///
    /// Sie steht hier und nicht im Pruefmodul des Editors, weil die Funktion
    /// hier wohnt; gelesen wird sie von beiden Seiten.
    #[test]
    fn ein_merkmalsname_wird_zu_seinem_setzer() {
        assert_eq!(setzername("smartQuotesType"), "setSmartQuotesType:");
        assert_eq!(
            setzername("allowsWritingToolsAffordance"),
            "setAllowsWritingToolsAffordance:"
        );
    }
}
