//! Der Durchlauf: liegt unter diesem Ordner ein Treffer, traegt diese Datei ihn?
//!
//! Das Ordnermodell entscheidet die Sichtbarkeit einer Zeile aus sechs
//! Eingaben, und fuenf davon hat es selbst. Die sechste ist der Befund ueber
//! einen Eintrag, den sein eigener Name nicht entscheidet, und den ermittelt
//! dieses Modul: je Auftrag genau einen von zwei Befunden — oder gar keinen.
//! **Was einen Auftrag bekommt, traegt den Filtertext im Namen nicht**; wessen
//! Name ihn traegt, ist ohne dieses Modul entschieden. Wer die Auftraege
//! zusammenstellt, ist nicht Sache dieser Datei; sie bekommt die Liste beim
//! Start vollstaendig uebergeben, zusammen mit dem Bestand, in dem die
//! Auftraege ihre Namen nachschlagen.
//!
//! **Zwei Auftragsarten, eine Maschine.** [`Auftragsart::Unterbaum`] fragt nach
//! dem Unterbaum eines Ordners, [`Auftragsart::Inhalt`] nach dem Text einer
//! gewoehnlichen Datei. Beide stellen dieselbe Art Frage an dieselbe Art
//! Gegenstand — eine Auskunft von der Platte, die nebenlaeufig entsteht, einen
//! Eintragsindex traegt und die Sicht neu aufbauen laesst —, und deshalb
//! bekommt die zweite keine zweite Maschine daneben. Ueber den Kanal geht
//! weiterhin **genau eine [`Befundmeldung`] je Auftrag**, und ihre Bedeutung
//! aendert sich nicht.
//!
//! ```text
//! Auftragsart Inhalt — eine gewoehnliche Datei:
//!
//! abgebrochen?              ─ ja ──> kein Befund
//!            │ nein
//! traegt der Inhalt?        ─ Traegt ───────> Treffer
//!                           ├ TraegtNicht ──> kein Treffer
//!                           ├ ZuGross ──────> kein Treffer, der Zaehler steigt
//!                           └ Unentschieden > kein Befund
//!
//! Auftragsart Unterbaum — ein Ordner oder eine Verknuepfung:
//!
//! ist er eine Verknuepfung? ─ ja ──> kein Treffer darunter
//!            │ nein
//! laesst er sich oeffnen?   ─ nein ─> fehlt ein Deskriptor? ─ ja ─> kein Befund
//!            │ ja                              │ nein
//!            │                                 └───────> kein Treffer darunter
//! naechsten Stapel holen    ─ leer ─> naechster vorgemerkter Ordner
//!            │                        (oder: kein Treffer darunter)
//! Name traegt die Folge?    ─ ja ──> Treffer, der Rest bleibt ungelesen
//!            │ nein
//! ist es ein Ordner?        ─ ja ──> vormerken
//!            │ nein
//! eine Datei, und eine Grenze steht? ─ ja ──> traegt der Inhalt? (wie oben)
//!            └ nein ─────────────────> naechster Eintrag im Stapel
//! ```
//!
//! # Die Grenze reist als Argument, und `None` heisst „zaehlt nicht"
//!
//! `krk-core` kennt die 1 MB der Vorschau nicht und soll sie nicht kennen; sie
//! kommt aus `krk-ui` herein. [`Durchlauf::starten`] nimmt sie als
//! `Option<u64>`, und der Wert traegt zwei Aussagen in einem: `None` heisst
//! „der Inhalt zaehlt bei diesem Lauf nicht", `Some(n)` heisst „er zaehlt, und
//! `n` ist die groesste Zahl Bytes, die je Datei gelesen werden darf". **Ein
//! Lauf mit `None` verhaelt sich in jeder Hinsicht wie der Durchlauf vor der
//! Runde 11**: es wird keine Datei geoeffnet, weder flach noch im Unterbaum.
//! Zwei getrennte Argumente waeren zwei Gelegenheiten, sie widerspruechlich zu
//! setzen.
//!
//! # Die zu grossen Dateien sind ein Zustand des Laufs und kein zweiter Kanal
//!
//! Eine Datei ueber der Grenze wird gar nicht erst gelesen. Sie ist damit kein
//! Nichttreffer, sondern **ungelesen**, und die Statuszeile sagt, wie viele es
//! waren. Der Durchlauf zaehlt sie in einem zweiten geteilten Kennzeichen neben
//! dem Abbruch, einem `Arc<AtomicU64>`, abzulesen ueber [`Durchlauf::zu_gross`].
//! Es steht dort, wo der Lauf seinen anderen Zustand schon haelt, und es ist
//! ausdruecklich kein zweiter Kanal: der Kanal traegt Befunde ueber Eintraege,
//! die Zahl aber gehoert zum Lauf und zu keinem einzelnen Eintrag.
//!
//! # Die Bauart ist die des Lesevorgangs
//!
//! Ein Arbeitsfaden je Durchlauf, ein Kanal mit der Kapazitaet eines Stapels,
//! ein Abbruchkennzeichen als `Arc<AtomicBool>`, und `Drop` setzt es. Gelesen
//! wird ueber [`Schwungleser`], dieselbe Huelle, die auch
//! [`super::leser::lesen`] und der gestueckelte Lesevorgang benutzen; eine
//! zweite Lesemechanik entsteht nicht.
//!
//! **Der Kanal ist so tief wie ein Stapel, und die Einheit ist eine andere.**
//! Beim Lesevorgang haelt der Kanal Stapel von Eintraegen, und seine Tiefe 1
//! ist eine Aussage ueber den Speicher. Hier haelt er Befunde zu je einem
//! Eintragsindex und einem Wahrheitswert; dieselbe Zahl 1.024 kostet damit acht
//! Kilobyte statt eines zweiten Ordnerbestands. Mit der Tiefe 1 blockierte der
//! Arbeitsfaden nach jedem einzelnen Befund bis zum naechsten Einzugstakt, also
//! bis zu 16 ms je entschiedenem Ordner.
//!
//! **Die Abbruchzusage haengt nicht an der Kanaltiefe, sondern an den Einheiten
//! des Laufs.** Die Regel lautet: geprueft wird vor jeder Einheit, die dauern
//! kann. Das sind seit der Runde 11 **zwei**, der naechste Stapel eines Ordners
//! und die naechste gelesene Datei. Die zweite kam mit dem Inhaltsfilter dazu,
//! und ohne sie laese ein Ordner mit tausend Dateien in einem Stapel tausend
//! Dateien durch, bevor der Abbruch greift.
//!
//! Ausdruecklich **nicht** geprueft wird beim Absteigen: ein Ordner mit
//! fuenfzigtausend gewoehnlichen Eintraegen und ohne einen einzigen Unterordner
//! steigt kein einziges Mal ab, passiert die Stapelgrenze aber neunundvierzig
//! Mal. Haenge man die Pruefung ans Absteigen, waere genau dieser Ordner von
//! der Zusage ausgenommen.
//!
//! # Ein offener Deskriptor, gleich wie tief der Baum ist
//!
//! **Der Abstieg merkt Ordner als Pfad vor und nicht als offenen Leser.** Ein
//! Ordner wird ganz gelesen, seine Unterordner wandern dabei als Pfad auf den
//! Stapel `offen`, und erst wenn er zu Ende ist, faellt sein [`Schwungleser`]
//! und der naechste wird geoeffnet. Zu jedem Zeitpunkt haelt der Durchlauf
//! damit **genau einen** Verzeichnisdeskriptor, ob der Baum drei Ebenen tief
//! ist oder vierhundert.
//!
//! **Der Inhaltsfilter legt genau einen Deskriptor dazu, und nur waehrend eines
//! Lesens.** [`inhalt::traegt_der_inhalt`](super::inhalt::traegt_der_inhalt)
//! oeffnet die Datei, liest sie bis zur Grenze und gibt sie wieder frei, bevor
//! der naechste Kandidat drankommt; gehalten wird waehrenddessen ein
//! Verzeichnisdeskriptor und ein Dateideskriptor, gleich wie tief der Baum ist
//! und gleich wie viele Dateien im Ordner stehen. Wer daraus eine Liste offener
//! Dateien macht, holt sich den Defekt `260815-0211` in seiner zweiten Gestalt.
//!
//! Bis zum 260815 hielt er stattdessen einen Leser je Ebene, weil der
//! uebergeordnete Ordner nach der Rueckkehr aus dem Abstieg weitergelesen
//! wurde. Das war kein blosser Preis, sondern eine Fehlerquelle: der Durchlauf
//! erzeugte damit seinen eigenen `EMFILE`, und der wurde unten zu einem
//! stillen „kein Treffer darunter" (Defekt `260815-0211`). Die Kante „zurueck
//! zum uebergeordneten Ordner" gibt es deshalb nicht mehr — der Rueckweg
//! laeuft ueber den vorgemerkten Pfad.
//!
//! **Getauscht ist damit ein knapper, prozessweit geteilter Vorrat gegen einen
//! reichlichen, der diesem Durchlauf allein gehoert.** Die Deskriptortabelle
//! teilt sich der Durchlauf mit dem Editor, der Vorschau, den
//! Kopiervorgaengen und dem Lesevorgang der zweiten Dateiliste, und ein
//! aus dem Finder gestartetes Buendel bekommt sie klein. Der Stapel `offen`
//! haelt dagegen je vorgemerktem Ordner einen Pfad, und das ist weniger, als
//! das Ordnermodell fuer denselben Ordner ohnehin haelt: dort steht je Eintrag
//! ein [`Eintrag`](super::eintrag::Eintrag) mit zwei Sortierschluesseln.
//!
//! # Was dieses Modul nicht hat, und warum
//!
//! **Keine Tiefengrenze und keine Zaehlung gegen eine Grenze.** Der Abstieg
//! laeuft ueber einen eigenen Stapel von Pfaden und nicht ueber die Rekursion
//! des Arbeitsfadens; ein tiefer Baum sprengt damit keinen Fadenstapel, und es
//! gibt nichts, wogegen zu zaehlen waere.
//!
//! **Keinen mitgefuehrten Zustand ueber besuchte Ordner.** In eine symbolische
//! Verknuepfung wird nicht abgestiegen, und damit ist kein Ordner zweimal zu
//! erreichen; eine Besuchtliste haette nichts zu verhindern.
//!
//! **Keinen Deckel auf die Trefferzahl.** Es wird nicht gezaehlt: der erste
//! Treffer entscheidet den Auftrag, in welcher Tiefe er auch liegt, und der
//! Rest unter ihm bleibt ungelesen.
//!
//! **Kein `warten`.** Der [`Lesevorgang`](super::leser::Lesevorgang) haelt sein
//! Fadenstueck fuer Aufrufer, die auf den Abschluss warten; hier gibt es
//! keinen, der wartet. Dass der Faden geendet hat, sagt der geschlossene Kanal.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;

use super::eintrag::{Eintrag, Typ};
use super::filter::traegt_die_folge;
use super::inhalt::{Inhaltsbefund, traegt_der_inhalt};
use super::leser::STAPELGROESSE;
use super::sys::{Schwungleser, ist_deskriptormangel};

/// Wonach ein Auftrag fragt.
///
/// **Zwei Werte, ueberschneidungsfrei und vollstaendig, ohne Auffangzweig.** Sie
/// bilden den Schnitt ab, den auch der Pruefschritt des Ordnermodells zieht: ein
/// Ordner oder eine Verknuepfung auf der einen Seite, eine gewoehnliche Datei
/// auf der anderen. Es ist seit dem 260816 buchstaeblich derselbe Schnitt und
/// nicht mehr nur derselbe der Absicht nach: die Auftragsliste entsteht in
/// [`Ordnermodell::auftraege`] aus dem Ergebnis des Pruefschritts. Dieses Modul
/// verzweigt danach und raet nicht am Typ herum.
///
/// [`Ordnermodell::auftraege`]: super::modell::Ordnermodell::auftraege
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auftragsart {
    /// Liegt unter diesem Ordner ein Eintrag, dessen Name die Folge traegt?
    Unterbaum,
    /// Traegt der Text dieser gewoehnlichen Datei die Folge?
    Inhalt,
}

/// Ein Eintrag des angezeigten Ordners, ueber den noch nichts bekannt ist.
///
/// Der Eintragsindex und nicht die Zeile: die Sichtreihenfolge wird bei jedem
/// Sortierwechsel neu gebaut, der Bestand nicht.
///
/// **Der Index und kein Name.** Bis zum 260816 trug der Auftrag eine Kopie des
/// Namens, und die Auftragsliste entstand bei jedem getippten Zeichen neu: bei
/// 100.000 Eintraegen also bis zu 100.000 Zeichenketten je Tastendruck, auf dem
/// Hauptfaden (`issues/260816-1933_*_die-auftragsliste-legt-je-tastendruck-einen-namen-je-datei-an-auf-dem-hauptfaden.md`).
/// Den Namen haelt das Ordnermodell ohnehin schon; [`Durchlauf::starten`]
/// bekommt seinen Bestand deshalb mitgereicht und schlaegt ihn dort nach. Ein
/// Auftrag ist damit acht Bytes und eine Sicht auf den Bestand statt einer
/// zweiten Fassung davon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Auftrag {
    /// Der Index des Eintrags im Bestand des Ordnermodells.
    pub index: u32,
    /// Wonach gefragt wird.
    pub art: Auftragsart,
}

/// Was der Arbeitsfaden ueber einen Auftrag meldet.
///
/// Genau eine Meldung je Auftrag, und sie beendet ihn. **`treffer: false` ist
/// nicht dasselbe wie „noch nichts gemeldet"**: der erste heisst „gelesen, es
/// liegt nichts darunter", das Ausbleiben heisst „noch nicht entschieden". Der
/// Unterschied ist die Zusage C3.13, und er entsteht dadurch, dass ein
/// abgebrochener Durchlauf gar nichts meldet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Befundmeldung {
    /// Der Eintragsindex aus dem [`Auftrag`].
    pub index: u32,
    /// Wahr, wenn der Eintrag die Frage seines Auftrags mit ja beantwortet:
    /// unter diesem Ordner liegt ein Name mit der Folge, oder der Text dieser
    /// Datei traegt sie.
    pub treffer: bool,
}

/// Ein laufender Durchlauf auf einem eigenen Faden.
pub struct Durchlauf {
    abbruch: Arc<AtomicBool>,
    zu_gross: Arc<AtomicU64>,
    befunde: Receiver<Befundmeldung>,
}

impl Durchlauf {
    /// Startet den Durchlauf und kehrt sofort zurueck.
    ///
    /// `filter_klein` ist der bereits kleingeschriebene Filtertext: er wird
    /// einmal je Suche umgeschrieben und nicht einmal je gelesenem Namen.
    ///
    /// `inhaltsgrenze` ist die groesste Zahl Bytes, die je Datei gelesen werden
    /// darf; `None` heisst, dass bei diesem Lauf keine Datei geoeffnet wird.
    /// Der Modulkopf schreibt aus, warum die Zahl von aussen kommt.
    ///
    /// `generation` benennt allein den Arbeitsfaden (`krk-durchlauf-<n>`),
    /// damit ein Fadenprotokoll lesbar bleibt; den Befunden liegt sie nicht
    /// bei, weil jeder Tab seinen eigenen Durchlauf haelt und allein aus dessen
    /// Kanal liest.
    ///
    /// `bestand` ist der Bestand des Ordnermodells, aus dem die Auftraege
    /// stammen; jeder Auftrag liest seinen Namen dort nach. Er wird **geteilt
    /// und nicht kopiert** — was daran haengt, steht bei [`Auftrag`]. Ein Index
    /// ausserhalb dieses Bestands kann von einem Aufrufer, der beide aus
    /// demselben Modell nimmt, nicht kommen; er endet trotzdem in einem eigenen
    /// Zweig, weil ein stillschweigend uebergangener Auftrag ein Befund waere,
    /// den niemand je bekommt.
    pub fn starten(
        bestand: Arc<Vec<Eintrag>>,
        auftraege: Vec<Auftrag>,
        ordner: PathBuf,
        filter_klein: String,
        inhaltsgrenze: Option<u64>,
        generation: u64,
    ) -> Self {
        let abbruch = Arc::new(AtomicBool::new(false));
        let zu_gross = Arc::new(AtomicU64::new(0));
        let (sender, befunde) = sync_channel(STAPELGROESSE);
        let faden_abbruch = Arc::clone(&abbruch);
        let faden_zu_gross = Arc::clone(&zu_gross);
        thread::Builder::new()
            .name(format!("krk-durchlauf-{generation}"))
            .spawn(move || {
                let lage = Auftragslage {
                    bestand: &bestand,
                    auftraege: &auftraege,
                    ordner: &ordner,
                    filter_klein: filter_klein.as_str(),
                    inhaltsgrenze,
                };
                durchlauffaden(&lage, &faden_abbruch, &faden_zu_gross, &sender);
            })
            .expect("Arbeitsfaden fuer den Durchlauf laesst sich nicht starten");
        Self {
            abbruch,
            zu_gross,
            befunde,
        }
    }

    /// Der Kanal, aus dem der Hauptfaden die Befunde holt.
    ///
    /// Er schliesst, wenn der Arbeitsfaden geendet hat. Ein geschlossener Kanal
    /// ohne weitere Meldung heisst nicht, dass die restlichen Auftraege keinen
    /// Treffer tragen: er heisst, dass sie nicht entschieden sind.
    pub fn befunde(&self) -> &Receiver<Befundmeldung> {
        &self.befunde
    }

    /// Wie viele Dateien dieser Lauf bisher wegen ihrer Groesse **nicht**
    /// gelesen hat.
    ///
    /// Der Wert waechst waehrend des Laufs und faellt nie; wer ihn zweimal
    /// fragt, bekommt zweimal den Stand von genau diesem Augenblick. Er zaehlt
    /// Dateien und keine Auftraege: eine ungelesene Datei tief im Unterbaum
    /// zaehlt genauso mit wie eine, die selbst einen Auftrag hatte.
    ///
    /// **Er ist kein Befund ueber eine Zeile.** Eine zu grosse Datei steht
    /// nicht in der Liste, und sie steht auch nicht als Nichttreffer da — sie
    /// wurde nicht angesehen, und diese Zahl ist der Satzteil der Statuszeile,
    /// der das sagt.
    pub fn zu_gross(&self) -> u64 {
        self.zu_gross.load(Ordering::Relaxed)
    }

    /// Bricht den Durchlauf ab.
    ///
    /// Der Arbeitsfaden bemerkt es an der naechsten Stapelgrenze und meldet
    /// nichts mehr. Bereits gesendete Befunde bleiben gueltig.
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }
}

impl Drop for Durchlauf {
    /// Fordert den Abbruch an, wartet aber nicht auf den Faden.
    ///
    /// Warten hiesse, dass ein getipptes Zeichen auf den Durchlauf des vorigen
    /// Filtertexts wartet. Der Faden endet von selbst: entweder bemerkt er das
    /// Abbruchkennzeichen, oder sein naechstes Senden scheitert, weil der
    /// Empfaenger mit dem `Durchlauf` gefallen ist.
    fn drop(&mut self) {
        self.abbrechen();
    }
}

/// Was ein Lauf zu tun hat: die Frage und die Gegenstaende, an die sie geht.
///
/// **Eine Struktur und keine fuenf einzelnen Argumente**, und der Grund ist
/// nicht die Bequemlichkeit: die fuenf Werte stehen fuer die Dauer eines Laufs
/// fest und gehoeren zusammen, waehrend das Abbruchkennzeichen, der Zaehler und
/// der Kanal sich waehrenddessen aendern. Zusammengefasst sind genau die
/// unveraenderlichen.
struct Auftragslage<'a> {
    /// Der Bestand des Ordnermodells, aus dem die Auftraege stammen.
    bestand: &'a [Eintrag],
    /// Die Auftraege, in der Reihenfolge, in der sie abzuarbeiten sind.
    auftraege: &'a [Auftrag],
    /// Der angezeigte Ordner; alle Auftraege liegen unmittelbar in ihm.
    ordner: &'a Path,
    /// Der bereits kleingeschriebene Filtertext.
    filter_klein: &'a str,
    /// Die groesste Zahl Bytes je Datei, oder `None` fuer „es wird keine Datei
    /// geoeffnet".
    inhaltsgrenze: Option<u64>,
}

/// Arbeitet die Auftraege der Reihe nach ab und meldet je einen Befund.
///
/// Endet ohne weitere Meldung, sobald der Abbruch greift, ein Deskriptor fehlt
/// oder der Empfaenger verschwunden ist. Die Reihenfolge ist die der Liste;
/// keine Zusage haengt an ihr, und ein Ordner mit grossem Unterbaum ohne
/// Treffer verzoegert die nach ihm.
fn durchlauffaden(
    lage: &Auftragslage<'_>,
    abbruch: &AtomicBool,
    zu_gross: &AtomicU64,
    sender: &SyncSender<Befundmeldung>,
) {
    let filter_klein = lage.filter_klein;
    for auftrag in lage.auftraege {
        // Ein Auftrag ohne Eintrag im Bestand ist von diesem Lauf nicht zu
        // beantworten. Die Lage kann nicht auftreten — Auftragsliste und
        // Bestand stammen aus demselben Ordnermodell —, und sie endet aus
        // demselben Grund wie der Deskriptormangel: unentschieden.
        let Some(eintrag) = lage.bestand.get(auftrag.index as usize) else {
            return;
        };
        let pfad = lage.ordner.join(&eintrag.name);
        // Vollstaendig und ohne Auffangzweig: zwei Auftragsarten mal die zwei
        // Gestalten der Grenze.
        let entschieden = match (auftrag.art, lage.inhaltsgrenze) {
            (Auftragsart::Unterbaum, grenze) => {
                unterbaum_entscheiden(&pfad, filter_klein, grenze, abbruch, zu_gross)
            }
            (Auftragsart::Inhalt, Some(grenze)) => {
                // Die zweite Stelle der Abbruchgrenze, hier im flachen Zweig:
                // eine gelesene Datei ist die kleinere der beiden Einheiten,
                // die dauern koennen.
                if abbruch.load(Ordering::Relaxed) {
                    None
                } else {
                    datei_entscheiden(&pfad, filter_klein, grenze, zu_gross)
                }
            }
            // Ein Inhaltsauftrag ohne Grenze ist von diesem Lauf nicht zu
            // beantworten, und ungelesen heisst unentschieden und nicht „traegt
            // nicht". Die Paarung kann nicht auftreten: der Aufrufer leitet die
            // Auftragsart und die Grenze aus derselben Frage ab, naemlich ob
            // „Content" wirkt. Sie steht hier, weil ein Auffangzweig sie
            // stillschweigend negativ entschiede.
            (Auftragsart::Inhalt, None) => None,
        };
        let Some(treffer) = entschieden else {
            return;
        };
        let meldung = Befundmeldung {
            index: auftrag.index,
            treffer,
        };
        if sender.send(meldung).is_err() {
            return;
        }
    }
}

/// Liest eine einzelne Datei und uebersetzt ihren Inhaltsbefund in die Antwort
/// des Durchlaufs.
///
/// `Some(true)` ist der Treffer, `Some(false)` der negative Befund, `None`
/// heisst „nicht entschieden" und beendet den ganzen Durchlauf — dieselbe
/// Bedeutung wie bei [`unterbaum_entscheiden`], und aus demselben Grund: der
/// Deskriptormangel ist ein Zustand des Prozesses, und die naechste Datei
/// muesste aus demselben leeren Vorrat oeffnen.
///
/// **Eine zu grosse Datei ist ein `Some(false)` mit einem Nebeneffekt**, und
/// das ist keine Vermengung zweier Aussagen: die Zeile steht nicht, weil ueber
/// sie nichts bekannt ist, und dass sie nicht angesehen wurde, sagt der Zaehler
/// und nicht ihre Zeile. Einen dritten Trefferzustand gaebe es sonst, und
/// [`Befund`](super::modell::Befund) hat aus genau diesem Grund keinen.
///
/// **Die Abbruchgrenze steht nicht hier, sondern bei jedem Rufer davor.** Beide
/// Rufer pruefen unmittelbar vor dem Aufruf; sie hier hereinzuziehen naehme dem
/// Modulkopf seine Aussage, dass die Pruefung vor jeder Einheit steht, die
/// dauern kann, und machte aus zwei sichtbaren Stellen eine versteckte.
fn datei_entscheiden(
    pfad: &Path,
    filter_klein: &str,
    grenze: u64,
    zu_gross: &AtomicU64,
) -> Option<bool> {
    match traegt_der_inhalt(pfad, filter_klein, grenze) {
        Inhaltsbefund::Traegt => Some(true),
        Inhaltsbefund::TraegtNicht => Some(false),
        Inhaltsbefund::ZuGross => {
            zu_gross.fetch_add(1, Ordering::Relaxed);
            Some(false)
        }
        Inhaltsbefund::Unentschieden => None,
    }
}

/// Schreitet den Unterbaum ab, bis der erste Treffer faellt oder nichts mehr
/// offen ist.
///
/// `Some(true)` ist der Treffer, `Some(false)` der negative Befund; er entsteht
/// auf drei Wegen, und alle drei stehen unten im Rumpf: eine symbolische
/// Verknuepfung, ein Ordner, der sich nicht oeffnen laesst, und ein
/// abgeschrittener Unterbaum ohne Fund. Keiner der drei haelt den Durchlauf an,
/// und keiner erzeugt eine Meldung ueber den Befund hinaus.
///
/// **`inhaltsgrenze` macht den Abstieg inhaltsempfindlich.** Steht sie, wird
/// jede gewoehnliche Datei gelesen, deren Name die Folge nicht traegt, und ein
/// Treffer in ihrem Text entscheidet den Ordner genauso wie ein Treffer an
/// einem Namen. Steht sie nicht, wird im ganzen Unterbaum keine Datei
/// geoeffnet. Der Kurzschluss des Namens gilt dabei unveraendert: wessen Name
/// die Folge traegt, entscheidet den Ordner sofort und bleibt ungelesen.
///
/// **`None` heisst „nicht entschieden", und es hat zwei Ursachen.** Die erste
/// ist der Abbruch. Die zweite ist ein Mangel an Deskriptoren, und sie ist die
/// Antwort auf den Defekt `260815-0211`: `EMFILE` und `ENFILE` sind kein
/// Befund ueber den Ordner, sondern ein Zustand des Prozesses, und derselbe
/// Aufruf auf denselben Pfad kann gleich darauf gelingen. Ihn als „kein Treffer
/// darunter" zu melden, hiesse eine Zeile dauerhaft und ohne Meldung aus der
/// Liste zu nehmen; ihn unentschieden zu lassen, heisst, dass die naechste
/// Frage — ein weiteres Zeichen, ein Umschalten, ein Ordnerwechsel — ihn neu
/// stellt. Die Trennung selbst steht in
/// [`sys::ist_deskriptormangel`](super::sys::ist_deskriptormangel).
///
/// **Beide Ursachen beenden den ganzen Durchlauf und nicht nur diesen
/// Auftrag**, denn der Aufrufer wertet `None` als „hoer auf". Fuer den Abbruch
/// ist das die Zusage selbst; fuer den Deskriptormangel ist es die ehrliche
/// Antwort, weil der naechste Auftrag aus demselben leeren Vorrat oeffnen
/// muesste. Ein Warten mit erneutem Versuch stuende dagegen fuer eine Frage,
/// die dieses Modul nicht beantworten kann — ob und wann ein anderer Teil von
/// KRK einen Deskriptor freigibt —, und hielte den Arbeitsfaden dabei an. Seit
/// der Runde 11 hat `None` eine dritte Ursache, und sie ist dieselbe Sache in
/// anderer Gestalt: der Deskriptormangel beim Lesen einer Datei im Unterbaum.
fn unterbaum_entscheiden(
    wurzel: &Path,
    filter_klein: &str,
    inhaltsgrenze: Option<u64>,
    abbruch: &AtomicBool,
    zu_gross: &AtomicU64,
) -> Option<bool> {
    // Erster Zweig: eine symbolische Verknuepfung ist ohne Lesen entschieden.
    //
    // Gefragt wird mit `symlink_metadata`, also `lstat(2)`, und das ist hier
    // die richtige Frage und keine Rueckkehr zur Pfadpruefung, die der Defekt
    // `260809-1652` abgeschafft hat: jene fragte nach dem Typ, um danach zu
    // oeffnen, und blockierte an einer benannten Roehre. `lstat(2)` oeffnet
    // nichts und kann deshalb nicht blockieren, und die Frage „ist der Pfad
    // selbst eine Verknuepfung?" ist an einem Deskriptor gar nicht zu stellen:
    // wer oeffnet, ist ihr schon gefolgt.
    if ist_verknuepfung(wurzel) {
        return Some(false);
    }

    // Die vorgemerkten Ordner, als Pfad und nicht als offener Leser. Der
    // Auftrag selbst ist der erste, und tiefer liegende kommen beim Lesen
    // dazu.
    let mut offen = vec![wurzel.to_path_buf()];
    while let Some(pfad) = offen.pop() {
        // Zweiter Zweig, und hier faellt die Unterscheidung, um die es geht:
        // was sich aus einem Grund am Pfad nicht oeffnen laesst, ist
        // uebergangen (C3.10) — was sich mangels Deskriptor nicht oeffnen
        // laesst, ist ueberhaupt nicht entschieden.
        let leser = match Schwungleser::oeffnen(&pfad) {
            Ok(leser) => leser,
            Err(fehler) if ist_deskriptormangel(&fehler) => return None,
            Err(_) => continue,
        };

        let mut lesestand = Lesestand::neu(leser, pfad);
        loop {
            // Hier und nur hier steht die Abbruchgrenze, und sie gilt auch fuer
            // einen Ordner, der keinen einzigen Unterordner traegt: ein frisch
            // geoeffneter Ordner steht sofort an ihr, weil sein Stapel leer
            // beginnt.
            if abbruch.load(Ordering::Relaxed) {
                return None;
            }

            // Ein leerer Stapel und ein Lesefehler mitten im Ordner enden beide
            // gleich: dieser Ordner ist fertig. Ein Fehler beim Weiterlesen
            // sagt dasselbe wie ein Fehler beim Oeffnen, naemlich dass von hier
            // nichts mehr zu holen ist, und er haelt den Durchlauf ebenso wenig
            // an.
            match lesestand.stapel_holen() {
                Ok(true) => {}
                Ok(false) | Err(_) => break,
            }

            for kandidat in lesestand.stapel.by_ref() {
                if traegt_die_folge(&kandidat.name, filter_klein) {
                    // Der erste Treffer entscheidet den Auftrag, in welcher
                    // Tiefe er auch liegt. Der offene Leser faellt mit
                    // `lesestand`, die Vormerkungen mit `offen`, und der Rest
                    // darunter bleibt ungelesen. Der Kurzschluss spart hier
                    // seit der Runde 11 auch das Lesen: eine namentlich
                    // passende Datei wird nie geoeffnet.
                    return Some(true);
                }
                // Die Fallunterscheidung ueber den Typ ist vollstaendig und hat
                // keinen Auffangzweig. `Ordner` ist auch eine Verknuepfung auf
                // einen Ordner; es ist derselbe Schnitt, den die Sichtbarkeit
                // zieht. Erst der Zweig fuer `Verknuepfung` trennt die beiden,
                // und er steht am Kopf dieser Funktion fuer den Auftrag und
                // hier fuer den Abstieg: in eine Verknuepfung wird weder
                // abgestiegen noch hineingelesen, sie traegt damit nichts bei.
                match kandidat.typ {
                    Typ::Ordner => offen.push(lesestand.pfad.join(&kandidat.name)),
                    Typ::Datei => {
                        let Some(grenze) = inhaltsgrenze else {
                            continue;
                        };
                        // Die zweite Stelle der Abbruchgrenze, hier in der
                        // Kandidatenschleife. Ohne sie laese ein Ordner mit
                        // tausend Dateien den ganzen Stapel durch, bevor der
                        // Abbruch an der Stapelgrenze wieder drankaeme.
                        if abbruch.load(Ordering::Relaxed) {
                            return None;
                        }
                        let pfad = lesestand.pfad.join(&kandidat.name);
                        if datei_entscheiden(&pfad, filter_klein, grenze, zu_gross)? {
                            return Some(true);
                        }
                    }
                    Typ::Verknuepfung => {}
                }
            }
        }
        // `lesestand` faellt hier, und mit ihm der eine offene Deskriptor.
    }

    // Dritter Zweig: abgeschritten, kein Treffer gefunden.
    Some(false)
}

/// Ob der Pfad selbst eine symbolische Verknuepfung ist.
///
/// Ein Pfad, den es nicht gibt, ist keine; er scheitert dann am Oeffnen und
/// braucht keinen eigenen Zweig.
fn ist_verknuepfung(pfad: &Path) -> bool {
    std::fs::symlink_metadata(pfad).is_ok_and(|angaben| angaben.file_type().is_symlink())
}

/// Ein Eintrag, wie der Durchlauf ihn braucht.
///
/// Name und Typ und sonst nichts: Groesse, Aenderungszeit und vor allem die
/// beiden Sortierschluessel braucht niemand hier, und der Schluessel kostet je
/// Eintrag einen Gang durch die Kollation. Deshalb entsteht kein
/// [`Eintrag`](super::eintrag::Eintrag).
#[derive(Debug)]
struct Kandidat {
    name: String,
    typ: Typ,
}

/// Der **eine** Ordner, der gerade gelesen wird.
///
/// Es gibt zu jedem Zeitpunkt genau einen davon, und daran haengt die Zusage
/// aus dem Modulkopf: ein offener Deskriptor, gleich wie tief der Baum ist. Wer
/// hier wieder einen Stapel daraus macht, holt sich den Defekt `260815-0211`
/// zurueck.
struct Lesestand {
    leser: Schwungleser,
    /// Der Ordner, um die Namen seiner Eintraege anzuhaengen.
    pfad: PathBuf,
    /// Der Stapel, der gerade abgearbeitet wird.
    stapel: std::vec::IntoIter<Kandidat>,
    /// Was der Leser darueber hinaus geliefert hat.
    ///
    /// Ein Schwung des Lesers ist nicht auf [`STAPELGROESSE`] begrenzt: ein
    /// einziger Systemaufruf liefert je nach Namenslaenge mehrere tausend
    /// Eintraege. Der Stapel ist es, denn an seiner Grenze haengt die
    /// Abbruchzusage; was darueber hinausgeht, wartet hier auf den naechsten
    /// Griff.
    vorrat: Vec<Kandidat>,
    /// Wahr, sobald der Leser das Ende des Ordners gemeldet hat.
    erschoepft: bool,
}

impl Lesestand {
    fn neu(leser: Schwungleser, pfad: PathBuf) -> Self {
        Self {
            leser,
            pfad,
            stapel: Vec::new().into_iter(),
            vorrat: Vec::with_capacity(STAPELGROESSE),
            erschoepft: false,
        }
    }

    /// Holt den naechsten Stapel von hoechstens [`STAPELGROESSE`] Eintraegen.
    ///
    /// `Ok(false)` heisst: der Ordner ist zu Ende, dieser Lesestand ist
    /// fertig.
    fn stapel_holen(&mut self) -> io::Result<bool> {
        let Self {
            leser,
            vorrat,
            erschoepft,
            ..
        } = self;
        while !*erschoepft && vorrat.len() < STAPELGROESSE {
            let geliefert = leser.naechster_schwung(|roh| {
                vorrat.push(Kandidat {
                    name: roh.name.into_owned(),
                    typ: roh.typ,
                });
            })?;
            if geliefert == 0 {
                *erschoepft = true;
            }
        }
        let anzahl = self.vorrat.len().min(STAPELGROESSE);
        let stapel: Vec<Kandidat> = self.vorrat.drain(..anzahl).collect();
        let traegt_eintraege = !stapel.is_empty();
        self.stapel = stapel.into_iter();
        Ok(traegt_eintraege)
    }
}
