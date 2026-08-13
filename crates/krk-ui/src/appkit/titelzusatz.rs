//! Namen und Version links in der Titelleiste (C1 der Titelleisten-Runde).
//!
//! ```text
//!  ┌──────────────────────────────────────────────────────────┐
//!  │ ● ● ●   Name Version     /Users/k1/Projekte/krk          │
//!  └──────────────────────────────────────────────────────────┘
//!            ▲                 ▲
//!            dieses Modul      crate::fenstertitel::titel
//! ```
//!
//! **Die Zahl steht in dieser Skizze mit Absicht nicht ausgeschrieben.** Sie
//! darf in keiner `.rs`-Datei dieses Baums als Zeichenkette vorkommen, und
//! eine Probe zaehlt es nach; eine Skizze waere dafuer kein Sonderfall.
//!
//! Die eine Frage dieses Moduls: **wie kommen Name und Version in die
//! Titelleiste, ohne im Titel zu stehen.** Die Antwort ist ein eigener
//! Bereich der Leiste, ein `NSTitlebarAccessoryViewController` mit
//! `layoutAttribute == Left`, und was daraus herauskommt, ist genau er.
//!
//! Eingehaengt wird er an einer einzigen Stelle,
//! [`super::fenster::hauptfenster`], gleich neben `setContentMinSize:`. Ein
//! zweiter Einhaengepunkt entstuende sonst neben dem einen Aufbau des
//! Fensters.
//!
//! # Ein eigener Bereich und kein Namenszusatz im Titel
//!
//! Der Nutzer hat am 260813-0939 den eigenen Bereich gewaehlt. Der Grund ist
//! die Breite: Name und Zahl vor den Pfad gesetzt fraessen sie aus derselben
//! Zeile, und macOS kuerzte bei schmalem Fenster den Pfad, den KRK
//! ausdruecklich nicht kuerzt. Was `setTitle:` bekommt, ist deshalb
//! unveraendert allein das Ergebnis von [`crate::fenstertitel::titel`], und
//! dieses Modul ruft `setTitle:` nicht.
//!
//! Der Bereich ist **kein sechster Bereich der Fensterzeile**. Er liegt in der
//! Titelleiste und nicht in der `NSSplitView`, [`super::aufteilung`] kennt ihn
//! nicht, und `Bereich` wie `Fokus` bleiben bei fuenf Werten.
//!
//! # Der Text steht einmal da und wird nie nachgeschrieben
//!
//! [`beschriftung`] setzt Name und Version zusammen, [`bauen`] schreibt das
//! Ergebnis ein einziges Mal in das Feld, und danach gibt es keinen Weg mehr
//! an dieses Feld heran: es verlaesst diese Datei nicht, und aus [`bauen`]
//! kommt allein die Steuerung heraus. Weder Fokus noch Ordner, Tab, Datei oder
//! Blatt koennen den Text also aendern.
//!
//! Die Zahl kommt aus `[workspace.package] version` der Wurzel-`Cargo.toml`,
//! geerbt ueber `version.workspace = true` und gelesen ueber
//! `env!("CARGO_PKG_VERSION")`. Sie steht nirgends im Programmtext als
//! Zeichenkette, und es gibt keinen Bauschritt, der sie erzeugt oder einsetzt:
//! ein Bau aus einem geaenderten Baum zeigt deshalb dieselbe Zahl wie ein
//! ausgeliefertes Buendel. Was diese Zahl **deckt**, entscheidet nicht der Bau,
//! sondern der Tag auf HEAD, den `cargo xtask release` prueft.
//!
//! # `Left` und nicht `Leading`
//!
//! **Ein falscher Wert an `layoutAttribute` bricht zur Laufzeit ab, und der
//! Uebersetzer sagt dazu nichts**, weil die Eigenschaft den ganzen
//! Aufzaehlungstyp `NSLayoutAttribute` mit seinen zwoelf Werten nimmt. Der Kopf
//! des Systems (`NSTitlebarAccessoryViewController.h:23-30`) laesst davon
//! allein `Bottom` (die Vorgabe), `Right` und `Left` zu, dazu `Leading` und
//! `Trailing` fuer Anwendungen ab 10.12 und `Top` ab 10.13, letzteres nur
//! zusammen mit `NSWindowStyleMaskFullSizeContentView`. Woertlich: "All other
//! values are currently invalid and will assert."
//!
//! Gewaehlt ist `Left`. Es setzt den Bereich unmittelbar rechts neben die drei
//! Fensterknoepfe und behaelt diese Seite; `Leading` wechselte sie mit der
//! Schreibrichtung, und eine Lokalisierung, die davon Gebrauch machte, fuehrt
//! KRK nicht.
//!
//! # Den Ersthelferrang nimmt der Bereich nicht an
//!
//! Getragen wird das allein davon, was `labelWithString:` baut: der Kopf des
//! Systems nennt es "a non-wrapping, non-editable, non-selectable text field"
//! (`NSTextField.h:87-93`). `setEditable:` und `setSelectable:` stehen deshalb
//! nicht daneben — sie machten aus der Beschriftung genau das Feld, das sie
//! nicht sein soll. Die Traegerflaeche darum ist eine blanke `NSView` ohne
//! Aktion; ein Klick darauf loest nichts aus. Wie bei der Statuszeile gilt:
//! **ob das bei eingeschalteter vollstaendiger Tastaturbedienung haelt, ist
//! nicht gemessen**, und die Beobachtung dazu gehoert an das laufende Buendel.
//!
//! # Gekuerzt wird hier nichts
//!
//! Weder Name noch Version werden gekuerzt oder abgeschnitten. Wird die
//! Titelleiste zu schmal fuer alles, entscheidet macOS, was es zeigt — dieselbe
//! Zusage, die [`crate::fenstertitel`] fuer den Pfad traegt.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSTitlebarAccessoryViewController` steht seit macOS 10.10
//! (`NSTitlebarAccessoryViewController.h:20`), ebenso die Methode, mit der
//! [`super::fenster::hauptfenster`] den Bereich einhaengt,
//! `addTitlebarAccessoryViewController:` (`NSWindow.h:323`). Die Eigenschaft
//! `layoutAttribute` traegt im Kopf keine eigene Angabe und steht damit seit
//! 10.10 wie ihre Klasse. `NSViewController` steht seit 10.5
//! (`NSViewController.h:49`) und seine Eigenschaft `view` mit `setView:` ohne
//! eigene Angabe ebenso (`NSViewController.h:78`).
//! `NSTextField.labelWithString:` steht seit 10.12 (`NSTextField.h:93`),
//! `secondaryLabelColor` seit 10.10 (`NSColor.h:202`).
//!
//! `NSView`, `NSFont`, `NSColor`, `NSTextField` und `NSString` selbst stehen
//! seit 10.0, und ebenso die uebrigen Beruehrungen, die im Kopf des Systems
//! keine eigene Angabe tragen: `initWithFrame:`, `addSubview:`, `frame`,
//! `setFrame:`, `setAutoresizingMask:`, `setFont:`, `systemFontOfSize:`,
//! `smallSystemFontSize`, `setTextColor:`, `setStringValue:`, `sizeToFit` und
//! `new`.
//!
//! **Hoechste Untergrenze dieser Datei: 10.12.** Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine der genannten Klassen und Methoden ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use objc2::MainThreadOnly;
use objc2::rc::Retained;
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSFont, NSLayoutAttribute, NSTextField,
    NSTitlebarAccessoryViewController, NSView,
};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString, ns_string};

/// Der Abstand links und rechts vom Text, in Punkten.
///
/// Links haelt er die Beschriftung von den drei Fensterknoepfen weg, rechts
/// vom Titel daneben. Ohne ihn klebte der Name am Schliessknopf.
const RAND: f64 = 8.0;

/// Was in der Titelleiste steht: der Name, ein Leerzeichen, die Versionszahl.
///
/// **Die einzige Stelle im Baum, die Name und Version zusammensetzt.** Der
/// Ueber-Dialog von AppKit setzt seine eigene Zeile aus der `Info.plist`
/// zusammen und bekommt sie nicht von hier; dass es dabei bei einer Stelle
/// bleibt, haelt eine Zaehlprobe fest.
///
/// Keine spitzen Klammern, keine runden Klammern, kein Zusatz davor oder
/// dahinter: die spitzen Klammern im Entwurf des Nutzers waren
/// Platzhalter-Notation und keine Schreibweise (Antwort 4 der Klaerungsrunde
/// vom 260813-0939).
///
/// Eine reine Funktion ohne AppKit. Sie ist damit ohne Fenster pruefbar, und
/// das ist die halbe Abnahme von C1.
pub fn beschriftung() -> String {
    concat!("KRK ", env!("CARGO_PKG_VERSION")).to_owned()
}

/// Baut den Bereich, den [`super::fenster::hauptfenster`] links in die
/// Titelleiste haengt.
///
/// Drei Ansichten, von innen nach aussen: die Beschriftung, die
/// Traegerflaeche, die den Rand haelt, und die Steuerung, die AppKit die Seite
/// nennt.
///
/// **Die Bauform der Beschriftung ist die der Statuszeile**
/// ([`super::statuszeile::Statuszeile::bauen`]): kleine Systemschrift und
/// [`NSColor::secondaryLabelColor`]. Eine eigene Farbe setzt dieses Modul
/// nicht, und genau daran haengt, dass der Text in jedem Erscheinungsbild
/// lesbar bleibt — hell wie dunkel entscheidet das System, nicht KRK.
///
/// **Der Text wird genau einmal geschrieben.** `sizeToFit` misst danach, wie
/// breit er ist, und die Traegerflaeche bekommt diese Breite plus zweimal
/// [`RAND`]. Die Beschriftung selbst haelt oben und unten einen mitwachsenden
/// Abstand, damit sie in der Mitte der Leiste steht, wenn AppKit die
/// Traegerflaeche auf die Hoehe der Titelleiste zieht.
pub fn bauen(mtm: MainThreadMarker) -> Retained<NSTitlebarAccessoryViewController> {
    let feld = NSTextField::labelWithString(ns_string!(""), mtm);
    feld.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    feld.setTextColor(Some(&NSColor::secondaryLabelColor()));
    feld.setStringValue(&NSString::from_str(&beschriftung()));
    feld.sizeToFit();

    let text = feld.frame().size;
    feld.setFrame(NSRect::new(NSPoint::new(RAND, 0.0), text));
    feld.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewMinYMargin | NSAutoresizingMaskOptions::ViewMaxYMargin,
    );

    let traeger = NSView::initWithFrame(
        NSView::alloc(mtm),
        NSRect::new(
            NSPoint::ZERO,
            NSSize::new(text.width + 2.0 * RAND, text.height),
        ),
    );
    traeger.addSubview(&feld);

    let steuerung = NSTitlebarAccessoryViewController::new(mtm);
    steuerung.setView(&traeger);
    // Nicht `Leading`. Die Begruendung steht im Modulkopf; der Uebersetzer
    // faengt einen falschen Wert hier nicht ab.
    steuerung.setLayoutAttribute(NSLayoutAttribute::Left);
    steuerung
}

#[cfg(test)]
mod tests {
    use super::beschriftung;
    use crate::quellbaum::quelldateien;

    /// Der Pfad dieser Datei in dem Baum, den [`quelldateien`] liest.
    const DIESE_DATEI: &str = "krk-ui/src/appkit/titelzusatz.rs";

    /// Der Inhalt dieser Datei, so wie die Zaehlproben ihn lesen.
    fn diese_datei() -> String {
        quelldateien()
            .into_iter()
            .find(|(name, _)| name == DIESE_DATEI)
            .map(|(_, inhalt)| inhalt)
            .unwrap_or_else(|| panic!("der Quellbaum fuehrt {DIESE_DATEI} nicht"))
    }

    /// Der Text ist der Name, ein Leerzeichen, die Versionszahl (C1.1).
    ///
    /// Drei Zusicherungen und nicht eine: die erste haelt die Zusammensetzung,
    /// die zweite schliesst die Klammern aus, die der Entwurf des Nutzers als
    /// Platzhalter trug, und die dritte haelt fest, dass genau **ein**
    /// Leerzeichen darin steht.
    ///
    /// **Die Versionszahl steht auch hier nicht als Zeichenkette da.**
    /// `concat!` setzt sie beim Uebersetzen aus `env!("CARGO_PKG_VERSION")`
    /// zusammen, also aus derselben Quelle wie [`beschriftung`] selbst. Die
    /// Probe darunter zaehlt genau das nach.
    #[test]
    fn die_beschriftung_ist_name_leerzeichen_version() {
        let text = beschriftung();
        assert_eq!(text, concat!("KRK ", env!("CARGO_PKG_VERSION")));
        assert!(
            !text.contains('(') && !text.contains(')') && !text.contains('<'),
            "die Beschriftung {text:?} traegt eine Klammer"
        );
        assert_eq!(
            text.matches(' ').count(),
            1,
            "die Beschriftung {text:?} traegt nicht genau ein Leerzeichen"
        );
    }

    /// Die Versionszahl steht in keiner `.rs`-Datei des Baums als Zeichenkette
    /// (C1.2).
    ///
    /// Die Nadel wird zur Pruefzeit aus `env!("CARGO_PKG_VERSION")` genommen
    /// und nicht hingeschrieben; deshalb findet die Probe sich nicht selbst,
    /// und deshalb zieht sie mit jeder Versionsaenderung von allein mit.
    ///
    /// **Was sie nicht unterscheiden kann:** eine Zeichenkette mit der Version
    /// und eine beliebige andere Fundstelle derselben Ziffernfolge, etwa in
    /// einer Messtafel. Wird sie eines Tages daran rot, ist das ein Anlass
    /// hinzusehen und nicht einer, die Nadel zu verengen — der Baum fuehrt
    /// heute keine solche Stelle.
    #[test]
    fn die_versionszahl_steht_in_keiner_quelldatei() {
        let zahl = env!("CARGO_PKG_VERSION");
        let treffer: Vec<String> = quelldateien()
            .into_iter()
            .filter(|(_, inhalt)| inhalt.contains(zahl))
            .map(|(name, _)| name)
            .collect();
        assert!(
            treffer.is_empty(),
            "die Version {zahl} steht als Zeichenkette in {treffer:?}; \
             sie gehoert allein in die Cargo.toml"
        );
    }

    /// Der Text wird genau einmal geschrieben, und keine Funktion schreibt ihn
    /// nach (C1.3).
    ///
    /// Zwei Zaehlungen. Die erste haelt fest, dass es genau einen Schreibruf
    /// gibt; die zweite, dass diese Datei nach aussen genau zwei Funktionen
    /// fuehrt, [`beschriftung`] und [`super::bauen`], und damit keine, die den
    /// Text nachtruege.
    ///
    /// **Beide Nadeln stehen zusammengesetzt da**, wie bei jeder Zaehlprobe
    /// dieses Baums: als ein Stueck geschrieben faende jede sich in dieser
    /// Datei selbst und zaehlte eine Fundstelle zu viel.
    ///
    /// **Ihre Blindheit:** eine private Funktion mit einem zweiten Schreibruf
    /// faenge die erste Zaehlung, eine ohne Schreibruf ginge beiden durch. Was
    /// den Fall traegt, ist nicht die Zaehlung allein, sondern dass das Feld
    /// diese Datei nicht verlaesst — aus `bauen` kommt die Steuerung heraus und
    /// nicht die Beschriftung.
    #[test]
    fn der_text_wird_genau_einmal_geschrieben() {
        let schreiben = concat!("setString", "Value(");
        let nach_aussen = concat!("pub ", "fn ");
        let inhalt = diese_datei();
        assert_eq!(
            inhalt.matches(schreiben).count(),
            1,
            "der Text der Titelleiste wird nicht genau einmal geschrieben"
        );
        assert_eq!(
            inhalt.matches(nach_aussen).count(),
            2,
            "diese Datei fuehrt nach aussen nicht mehr genau die Beschriftung \
             und den Bau"
        );
    }

    /// Die Beschriftung nimmt den Ersthelferrang nicht an (C1.6).
    ///
    /// Gezaehlt wird, was das Feld baut und was daneben **nicht** steht:
    /// `labelWithString:` liefert nach dem Kopf des Systems ein nicht
    /// bearbeitbares und nicht auswaehlbares Feld, und die beiden Schalter, die
    /// das zuruecknehmen wuerden, kommen in dieser Datei nicht vor.
    ///
    /// **Ihre Blindheit steht im Namen der Sache:** die Probe liest den
    /// Quelltext und nicht das Verhalten einer Instanz. Ob ein Klick auf den
    /// Bereich wirklich nichts ausloest, sieht der Nutzer am laufenden Buendel;
    /// das Kriterium traegt dafuer seine eigene Beobachtung.
    #[test]
    fn die_beschriftung_nimmt_den_ersthelferrang_nicht_an() {
        let beschriftungsfeld = concat!("labelWith", "String(");
        let bearbeitbar = concat!("setEdit", "able(");
        let auswaehlbar = concat!("setSelect", "able(");
        let inhalt = diese_datei();
        assert_eq!(
            inhalt.matches(beschriftungsfeld).count(),
            1,
            "das Feld entsteht nicht genau einmal als Beschriftung"
        );
        assert!(
            !inhalt.contains(bearbeitbar) && !inhalt.contains(auswaehlbar),
            "die Beschriftung wird bearbeitbar oder auswaehlbar gestellt"
        );
    }
}
