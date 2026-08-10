//! Was `text::datei::versatz_nach_der_wandlung` an einem 16-MB-Text anlegt
//! (Defekt `260810-0424`).
//!
//! Die Probe zaehlt mit einem eigenen Allokator die Anlagen ab 1 MB, also die
//! Kopien in der Groessenordnung der ganzen Datei, und laesst die kleinen
//! Anlagen des Pruefcodes selbst ungezaehlt. Gemessen wird gegen die Fassung,
//! die bis zum 260810 im Baum stand — `in_gehaltene_form(rest.to_owned())` —,
//! damit die Zahl ein Vorher **und** ein Nachher hat und nicht nur eine
//! Behauptung ueber das Nachher ist. Dass beide Fassungen denselben Versatz
//! liefern, prueft dieselbe Probe mit: eine Einsparung an einer Rechnung mit
//! anderem Ergebnis waere keine.
//!
//! ```text
//!  Regelfall (vorn ein \r\n eingefuegt, Schreibmarke dahinter,
//!             der ganze Rest schon in gehaltener Form):
//!    Fassung bis zum 260810   1 Anlage,  16 MB
//!    Fassung ueber Cow<str>   0 Anlagen,  0 MB
//!
//!  Gegenfall (der Rest traegt selbst ein \r\n):
//!    Fassung bis zum 260810   2 Anlagen, 32 MB
//!    Fassung ueber Cow<str>   1 Anlage,  16 MB
//! ```
//!
//! **Die Zaehlung laeuft allein in diesem Binaerziel**, weil ein
//! `#[global_allocator]` fuer das ganze Ziel gilt und sie sonst die Anlagen
//! fremder Proben mitnaehme, die nebenher auf anderen Faeden laufen. Aus
//! demselben Grund steht die Messung in **einer** Probe: die drei Zaehler sind
//! global, und zwei gleichzeitig zaehlende Proben zaehlten einander mit. Die
//! zweite Probe hier zaehlt nicht: sie setzt [`ZAEHLT`] nie, und ihre
//! Zeichenketten sind ein Dutzend Bytes lang und damit weit unter
//! [`ZAEHLGRENZE`], koennen also auch nebenher nicht in die Rechnung geraten.
//!
//! Die vier gemessenen Zahlen stehen mit `cargo test -p krk-core --test
//! textkopien -- --nocapture` auf der Fehlerausgabe.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use krk_core::text::datei;

/// Ab welcher Groesse eine Anlage als Kopie der ganzen Datei zaehlt. Dieselbe
/// Grenze, die der Datensatz `260810-0424` genannt hat: die Zeichenketten des
/// Pruefcodes selbst liegen weit darunter, die gesuchten Kopien weit darueber.
const ZAEHLGRENZE: usize = 1024 * 1024;

static ZAEHLT: AtomicBool = AtomicBool::new(false);
static ANLAGEN: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

/// Ein Allokator, der die grossen Anlagen zaehlt und alles an `System` weitergibt.
///
/// Gezaehlt wird nur, solange [`ZAEHLT`] gesetzt ist, damit das Anlegen des
/// Prueftextes selbst nicht in der Rechnung landet. `realloc` bleibt bei der
/// Vorgabe des Merkmals: sie legt neu an und kopiert, laeuft also durch `alloc`
/// und wird damit gezaehlt — ein Wachsen einer Zeichenkette ist eine Kopie.
struct Zaehlend;

// SICHERHEIT: beide Methoden geben Anfrage und Zeiger unveraendert an `System`
// weiter; die Zaehlung fasst keinen Speicher an. Die Bedingungen des Merkmals
// sind damit dieselben, die `System` schon erfuellt.
unsafe impl GlobalAlloc for Zaehlend {
    unsafe fn alloc(&self, anfrage: Layout) -> *mut u8 {
        if anfrage.size() >= ZAEHLGRENZE && ZAEHLT.load(Ordering::Relaxed) {
            ANLAGEN.fetch_add(1, Ordering::Relaxed);
            BYTES.fetch_add(anfrage.size(), Ordering::Relaxed);
        }
        unsafe { System.alloc(anfrage) }
    }

    unsafe fn dealloc(&self, zeiger: *mut u8, anfrage: Layout) {
        unsafe { System.dealloc(zeiger, anfrage) }
    }
}

#[global_allocator]
static ALLOKATOR: Zaehlend = Zaehlend;

/// Faehrt `rechnung` mit gesetzter Zaehlung und liefert Anlagen und Bytes.
fn gezaehlt<T>(rechnung: impl FnOnce() -> T) -> (T, usize, usize) {
    ANLAGEN.store(0, Ordering::Relaxed);
    BYTES.store(0, Ordering::Relaxed);
    ZAEHLT.store(true, Ordering::Relaxed);
    let ergebnis = rechnung();
    ZAEHLT.store(false, Ordering::Relaxed);
    (
        ergebnis,
        ANLAGEN.load(Ordering::Relaxed),
        BYTES.load(Ordering::Relaxed),
    )
}

/// Die Fassung, die bis zum 260810 im Baum stand, Zeile fuer Zeile.
///
/// Sie steht hier und nicht mehr in `datei.rs`, damit die Probe ein Vorher hat.
/// Wandelt sich die Regel der Wandlung, wandelt sie sich in beiden, denn beide
/// rufen dieselbe eine Stelle.
fn versatz_mit_kopie(vorher: &str, versatz: usize, nachher: &str) -> usize {
    let Some(rest) = vorher.get(versatz..) else {
        return nachher.len();
    };
    nachher
        .len()
        .saturating_sub(datei::in_gehaltene_form(rest.to_owned()).len())
}

/// Ein Text knapp unter [`datei::EDITORGRENZE`], aus Zeilen in gehaltener Form.
fn grosser_text() -> String {
    let zeile = "Die Kollation traegt den Sortierschluessel als Bytefolge.\n";
    let wiederholungen = 16 * 1024 * 1024 / zeile.len();
    zeile.repeat(wiederholungen)
}

#[test]
fn versatz_nach_der_wandlung_kopiert_den_rest_nicht_mehr() {
    let inhalt = grosser_text();
    assert!(
        inhalt.len() >= 15 * 1024 * 1024,
        "der Prueftext soll in der Groessenordnung der Editorgrenze liegen, ist aber {} Bytes",
        inhalt.len()
    );

    // Der Regelfall des Defekts: vorn ein `\r\n` eingefuegt, die Schreibmarke
    // dahinter. Hinter ihr steht fast die ganze Datei, und sie ist in
    // gehaltener Form.
    let flaeche = format!("\r\n{inhalt}");
    let stand = datei::in_gehaltene_form(flaeche.clone());
    let hinter_der_marke = "\r\n".len();

    let (neu, anlagen_neu, bytes_neu) =
        gezaehlt(|| datei::versatz_nach_der_wandlung(&flaeche, hinter_der_marke, &stand));
    let (alt, anlagen_alt, bytes_alt) =
        gezaehlt(|| versatz_mit_kopie(&flaeche, hinter_der_marke, &stand));

    assert_eq!(
        neu, alt,
        "die beiden Fassungen sollen denselben Versatz liefern"
    );
    assert_eq!(
        neu,
        "\n".len(),
        "die Schreibmarke steht hinter dem einen \\n"
    );

    eprintln!(
        "Regelfall, Prueftext {} Bytes: Fassung bis zum 260810 {anlagen_alt} Anlagen \
         ueber {bytes_alt} Bytes, Fassung ueber Cow<str> {anlagen_neu} Anlagen ueber \
         {bytes_neu} Bytes",
        flaeche.len()
    );

    assert_eq!(
        anlagen_alt, 1,
        "die Fassung bis zum 260810 legt den Rest an; gezaehlt wurden {anlagen_alt} Anlagen \
         ueber {bytes_alt} Bytes"
    );
    assert!(
        bytes_alt >= 15 * 1024 * 1024,
        "die eine Anlage soll die Groessenordnung der ganzen Datei haben, sie hat {bytes_alt} Bytes"
    );
    assert_eq!(
        anlagen_neu, 0,
        "die Fassung ueber Cow<str> soll im Regelfall nichts anlegen; gezaehlt wurden \
         {anlagen_neu} Anlagen ueber {bytes_neu} Bytes"
    );

    // Der Gegenfall: der Rest hinter der Schreibmarke traegt selbst ein `\r\n`,
    // die Wandlung muss also eine Zeichenkette bauen. Eine Anlage bleibt
    // deshalb; die Eingangskopie faellt auch hier weg.
    let (mitte, _) = inhalt.split_at(inhalt.len() / 2);
    let flaeche = format!("\r\n{mitte}\r\n{}", &inhalt[inhalt.len() / 2..]);
    let stand = datei::in_gehaltene_form(flaeche.clone());

    let (neu, anlagen_neu, bytes_neu) =
        gezaehlt(|| datei::versatz_nach_der_wandlung(&flaeche, hinter_der_marke, &stand));
    let (alt, anlagen_alt, bytes_alt) =
        gezaehlt(|| versatz_mit_kopie(&flaeche, hinter_der_marke, &stand));

    assert_eq!(
        neu, alt,
        "auch im Gegenfall sollen beide Fassungen denselben Versatz liefern"
    );
    eprintln!(
        "Gegenfall, Prueftext {} Bytes: Fassung bis zum 260810 {anlagen_alt} Anlagen \
         ueber {bytes_alt} Bytes, Fassung ueber Cow<str> {anlagen_neu} Anlagen ueber \
         {bytes_neu} Bytes",
        flaeche.len()
    );

    assert_eq!(
        anlagen_alt, 2,
        "im Gegenfall legt die alte Fassung zweimal an; gezaehlt wurden {anlagen_alt} Anlagen \
         ueber {bytes_alt} Bytes"
    );
    assert_eq!(
        anlagen_neu, 1,
        "im Gegenfall bleibt die Anlage der Wandlung, und nur sie; gezaehlt wurden \
         {anlagen_neu} Anlagen ueber {bytes_neu} Bytes"
    );
    assert!(
        bytes_neu * 2 <= bytes_alt + ZAEHLGRENZE,
        "die Fassung ueber Cow<str> soll im Gegenfall etwa die Haelfte anlegen: {bytes_neu} \
         gegen {bytes_alt} Bytes"
    );
}

/// Die kurze Fassung derselben Zusage, ohne 16 MB: `gehaltene_form` gibt einen
/// Text in gehaltener Form geliehen zurueck und legt nichts an.
///
/// Sie prueft die Eigenschaft, aus der die Einsparung folgt, statt sie nur an
/// der Zahl abzulesen. Die Probe darueber laeuft ueber
/// `versatz_nach_der_wandlung` und sieht `gehaltene_form` nur mittelbar.
#[test]
fn gehaltene_form_leiht_was_die_form_schon_hat() {
    let gehalten = String::from("erste\nzweite\n");
    let geliehen = datei::gehaltene_form(&gehalten);
    assert!(
        matches!(geliehen, std::borrow::Cow::Borrowed(_)),
        "ein Text in gehaltener Form soll geliehen zurueckkommen"
    );
    assert_eq!(geliehen, gehalten);

    let mit_crlf = String::from("erste\r\nzweite\n");
    let gewandelt = datei::gehaltene_form(&mit_crlf);
    assert!(
        matches!(gewandelt, std::borrow::Cow::Owned(_)),
        "ein Text mit \\r\\n soll uebernommen zurueckkommen"
    );
    assert_eq!(gewandelt, "erste\nzweite\n");

    // Die beiden Formen sind eine Wandlung: dasselbe Ergebnis, egal welchen
    // Namen der Aufrufer nimmt.
    assert_eq!(
        datei::in_gehaltene_form(mit_crlf.clone()),
        datei::gehaltene_form(&mit_crlf).into_owned()
    );
}
