//! Das Fenstermodell: welches Dateifenster das aktive ist, welche Bereiche
//! sichtbar sind, wie breit sie stehen, welche Spalten die beiden Dateilisten
//! zeigen und in welcher Reihenfolge beim Start gelesen wird.
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile. Die
//! Ansicht dazu ist [`crate::appkit::aufteilung`], die aus den Zahlen hier
//! Rahmen fuer die fuenf Bereiche einer `NSSplitView` macht.
//!
//! # Die fuenf Bereiche
//!
//! ```text
//! ┌───────────┬──────────────────┬──────────────────┬───────────┬───────────┐
//! │ Lesezei-  │ linkes           │ rechtes          │ Vorschau  │ Editor    │
//! │ chen (C5) │ Dateifenster     │ Dateifenster     │ (C6)      │ (C1)      │
//! └───────────┴──────────────────┴──────────────────┴───────────┴───────────┘
//!  min 120         min 240            min 240          min 160     min 320
//! ```
//!
//! Vorschau und Editor teilen sich dieselbe Stelle am rechten Rand und sind
//! nie zugleich sichtbar. Der gegenseitige Ausschluss steht in
//! [`Bereich::teilt_flaeche_mit`] und wirkt ueber die eine Schreibstelle
//! [`Fenstermodell::sichtbar_setzen`]: zur Laufzeit ueber
//! [`Fenstermodell::umschalten`], durch das auch [`Fenstermodell::einblenden`]
//! geht, und beim Start ueber [`Fenstermodell::aus_sitzung`] fuer eine von Hand
//! geschriebene `session.toml`. Damit ist das erste Abnahmekriterium von C1 der
//! Editor-Runde eingeloest, einschliesslich seines dritten Satzes: beide
//! zugleich sichtbar zu haben ist ueber keinen Weg erreichbar.
//!
//! **Alle fuenf teilen sich die Zeile im Verhaeltnis ihrer gespeicherten
//! Breiten**; die Regel steht in [`bereichsbreiten`], und die Zahl unter der
//! Skizze ist das Mindestmass, das gegen den Anteil gewinnt. Eine gespeicherte
//! Breite ist damit ein Wunsch und keine Zusage in Punkten: was ein Bereich
//! bekommt, haengt daran, wer sonst noch steht und wie breit das Fenster ist.
//! Das ist die Aufteilung, die C7 der Runde 1 verlangt ("Die verbleibenden
//! Bereiche nutzen den frei gewordenen Platz"), und sie macht zugleich die
//! Zusage darunter einfach: eine gespeicherte Breite gilt auch fuer einen
//! ausgeblendeten Bereich, also steht sein Anteil beim Wiedereinblenden wieder
//! da.
//!
//! Bis zur Bereichsleisten-Runde waren die drei Randbereiche "fest" und die
//! beiden Dateifenster "beweglich": jene behielten ihre Punktzahl, diese
//! teilten sich, was uebrig blieb. Die Zweiteilung ist weg, und mit ihr die
//! Festlegung vom 260808, nach der die Lesezeichenleiste dem Editor nicht
//! weicht — sie fiel allein daraus an, in welcher Reihenfolge die festen
//! Bereiche bedient wurden.
//!
//! # Eines bleibt: die Regel ueber die beiden Dateifenster
//!
//! C7 sichert zu, dass mindestens ein Dateifenster sichtbar bleibt. **Welches
//! von beiden, sagt die Zusage nicht**, und seit der Bereichsleisten-Runde tut
//! es auch dieses Modell nicht mehr: beide gehen durch denselben Zweig von
//! [`Fenstermodell::umschalten`], und abgewiesen wird der Befehl, der das
//! **letzte sichtbare** ausblenden wuerde. Welcher Bereich ein Dateifenster
//! ist, beantwortet [`Bereich::seite`] und keine zweite Aufzaehlung.
//!
//! Bis dahin war das linke besonders: es liess sich gar nicht ausblenden,
//! [`Sichtbarkeit`] trug kein Feld dafuer, und `umschalten` wies jeden Befehl
//! auf [`Bereich::Links`] ab. Der Nutzerentscheid vom 260812-0306
//! (`decisions/260811-1305_*_traegt-das-linke-dateifenster-einen-schalter.md`)
//! hat das aufgehoben, damit alle fuenf Schalter der Bereichsleiste dieselbe
//! Bedeutung tragen; das fuenfte Feld ist der Preis.
//!
//! Die Regel steht an zwei Stellen, weil es zwei Wege zu ihr gibt: zur Laufzeit
//! in [`Fenstermodell::umschalten`], und beim Start in
//! [`Fenstermodell::aus_sitzung`] fuer eine von Hand geschriebene
//! `session.toml`, die beide ausblendet. Die Abweisung gehoert dabei in dieses
//! Modell und nicht in die Belegungsdatei: ein Klick in der Bereichsleiste ist
//! ein Weg wie ein Tastenbefehl, und beide gehen hier hindurch.
//!
//! # Die Spalten der beiden Dateilisten
//!
//! Neben den fuenf Bereichen haelt dieses Modell seit der Bereichsleisten-Runde
//! auch, welche Spalten die Dateilisten zeigen: [`Spaltensichtbarkeit`] mit drei
//! Feldern, geschaltet ueber [`Fenstermodell::spalte_umschalten`], gelesen ueber
//! [`spalte_sichtbar_in`]. **Eine Angabe fuer beide Listen** (Nutzerentscheid
//! vom 260812-0306), und **die Sortierung bleibt davon unberuehrt**: ein
//! Spaltenschalter verbirgt eine Spalte und tut sonst nichts.
//!
//! Sie stehen hier und nicht in [`crate::tabs`], weil sie keinem Tab und keiner
//! Seite gehoeren. Was in der Anzeige daraus wird, setzt der
//! Anwendungsdelegierte ueber `NSTableColumn::setHidden`; dieses Modell nennt
//! auch dafuer keine Zeile AppKit.

use std::path::PathBuf;

use krk_core::ablage::{
    Breiten, Dateifenster as Fensterzustand, Fensterseite, Sichtbarkeit, Sitzung,
    Spaltensichtbarkeit, Zettel,
};

use crate::spalten::Spalte;
use crate::tabs::Tabuebersicht;

/// Um wie viele Punkte ein Tastenbefehl einen Bereich breiter oder schmaler
/// macht.
///
/// Zwei Zeilenhoehen der Dateiliste. Ein kleinerer Schritt braeuchte zu viele
/// Anschlaege, ein groesserer spraenge ueber die gesuchte Breite hinweg.
pub const BREITENSCHRITT: f64 = 40.0;

/// Einer der fuenf Bereiche der Fensterzeile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bereich {
    /// Die Lesezeichen- und Geraeteleiste ganz links (C5).
    Lesezeichen,
    /// Das linke Dateifenster.
    Links,
    /// Das rechte Dateifenster.
    Rechts,
    /// Das Vorschaufenster ganz rechts (C6).
    Vorschau,
    /// Der eingebaute Editor (C1 der Editor-Runde).
    ///
    /// Er steht **hinter** der Vorschau, weil er ihre Stelle in der
    /// Fensterzeile einnimmt: beide sitzen am rechten Rand und sind nie
    /// zugleich sichtbar.
    Editor,
}

impl Bereich {
    /// Alle fuenf, von links nach rechts.
    pub const ALLE: [Bereich; 5] = [
        Bereich::Lesezeichen,
        Bereich::Links,
        Bereich::Rechts,
        Bereich::Vorschau,
        Bereich::Editor,
    ];

    /// Die Stelle des Bereichs in der Fensterzeile.
    pub const fn index(self) -> usize {
        match self {
            Bereich::Lesezeichen => 0,
            Bereich::Links => 1,
            Bereich::Rechts => 2,
            Bereich::Vorschau => 3,
            Bereich::Editor => 4,
        }
    }

    /// Der Bereich eines Dateifensters.
    pub const fn von_seite(seite: Fensterseite) -> Self {
        match seite {
            Fensterseite::Links => Bereich::Links,
            Fensterseite::Rechts => Bereich::Rechts,
        }
    }

    /// Das Dateifenster dieses Bereichs, falls er eines ist.
    ///
    /// Die Umkehrung von [`Bereich::von_seite`] und **die eine Stelle, die
    /// aufzaehlt, welche Bereiche Dateifenster sind**. Wer wissen will, welcher
    /// Bereich das Gegenueber eines Dateifensters ist, fragt hier und fuehrt
    /// die Liste nicht ein zweites Mal; [`Fenstermodell::breite_aendern`] tut
    /// genau das.
    ///
    /// **Eine vollstaendige Fallunterscheidung und kein `matches!`.** Ein
    /// sechster Bereich haelt hier den Bau an und erzwingt die Einordnung, wie
    /// es die uebrigen vollstaendigen Fallunterscheidungen dieses Projekts auch
    /// tun.
    pub const fn seite(self) -> Option<Fensterseite> {
        match self {
            Bereich::Links => Some(Fensterseite::Links),
            Bereich::Rechts => Some(Fensterseite::Rechts),
            Bereich::Lesezeichen | Bereich::Vorschau | Bereich::Editor => None,
        }
    }

    /// Der Bereich, der sich mit diesem dieselbe Flaeche teilt.
    ///
    /// **Die eine Stelle des gegenseitigen Ausschlusses aus C1 der
    /// Editor-Runde.** Vorschau und Editor sitzen beide am rechten Rand und
    /// teilen sich die Flaeche zeitlich statt raeumlich: wird einer von beiden
    /// sichtbar, geht der andere. Die Directive nennt ausdruecklich nur die
    /// eine Richtung, dass der Editor die Vorschau schliesst; die andere folgt
    /// daraus, dass sonst ein Weg bliebe, auf dem beide dieselbe Flaeche
    /// beanspruchen. Der Spec fuehrt sie unter `## Was die Abnahme
    /// mitentscheidet` als Ableitung, die der Nutzer am Gate umstossen kann.
    ///
    /// Der Editor bekommt einen fuenften Platz in den Feldern und keinen
    /// zweiten Rechenweg daneben: [`bereichsbreiten`] behandelt ihn wie jeden
    /// anderen sichtbaren Bereich.
    ///
    /// Die Beziehung ist symmetrisch; die Probe `der_ausschluss_ist_gegenseitig`
    /// haelt es fest, damit ein einseitiger Eintrag nicht eine Richtung stumm
    /// verliert.
    ///
    /// **Vollstaendig und ohne Auffangzweig**, wie die uebrigen
    /// Fallunterscheidungen ueber [`Bereich`]: ein sechster Bereich haelt den
    /// Bau an und erzwingt die Antwort darauf, ob er sich eine Flaeche teilt.
    pub const fn teilt_flaeche_mit(self) -> Option<Bereich> {
        match self {
            Bereich::Vorschau => Some(Bereich::Editor),
            Bereich::Editor => Some(Bereich::Vorschau),
            Bereich::Lesezeichen | Bereich::Links | Bereich::Rechts => None,
        }
    }

    /// Die Breite, unter die sich der Bereich nicht ziehen laesst.
    ///
    /// Ein Dateifenster braucht mehr, weil vier Spalten hineinpassen muessen;
    /// die beiden Randbereiche tragen je eine Liste mit einer Spalte.
    ///
    /// **Der Editor steht mit 320 ueber der Vorschau mit ihren 160**, und der
    /// Grund ist das vierte Abnahmekriterium von C1: "nicht schmaler, als eine
    /// Zeile Text noch lesbar ist". Bei der festen Schrift der Rohansicht in
    /// Systemgroesse traegt diese Breite rund 40 Zeichen. Die Vorschau kommt
    /// mit weniger aus, weil sie Metadaten zeigt und keine Zeilen.
    pub const fn mindestbreite(self) -> f64 {
        match self {
            Bereich::Lesezeichen => 120.0,
            Bereich::Links | Bereich::Rechts => 240.0,
            Bereich::Vorschau => 160.0,
            Bereich::Editor => 320.0,
        }
    }

    /// Die Breite, mit der der Bereich beim allerersten Start aufgeht.
    ///
    /// **Als Punktzahl gesetzt und nicht als Anteil geschrieben.** Sie ist der
    /// Wunsch, mit dem ein Bereich in [`bereichsbreiten`] eingeht, solange
    /// niemand eine Breite fuer ihn gespeichert hat; als Anteil geschrieben
    /// waere sie eine zweite Waehrung neben den Punktzahlen in `session.toml`.
    ///
    /// Die 460 des Editors folgen aus den bestehenden vier: sie summieren sich
    /// zu 1280, ein Drittel davon sind rund 427. C1 der Editor-Runde verlangt
    /// "rund ein Drittel der Fensterbreite", und die Anteilsregel haelt das
    /// ueber jede Fensterbreite hinweg: mit ausgeblendeter Vorschau wuenschen
    /// die vier sichtbaren Bereiche zusammen 1480, der Editor bekommt also
    /// 460/1480 der Zeile, knapp 31 Prozent. Die Zahl gilt nur beim
    /// allerersten Start; danach gilt die Breite des Nutzers.
    pub const fn anfangsbreite(self) -> f64 {
        match self {
            Bereich::Lesezeichen => 180.0,
            Bereich::Links | Bereich::Rechts => 420.0,
            Bereich::Vorschau => 260.0,
            Bereich::Editor => 460.0,
        }
    }

    /// Der kurze Name des Bereichs, wie ihn ein Schalter der Bereichsleiste
    /// traegt.
    ///
    /// **Kurz, weil die Leiste 18 Punkte hoch ist** und acht Schalter
    /// nebeneinander traegt; ausgeschriebene Namen passten bei der
    /// Mindestbreite des Fensters nicht mehr in eine Zeile. Was der Schalter
    /// meint, sagt der Hinweistext aus [`Bereich::langname`], der beim
    /// Verweilen erscheint.
    ///
    /// **Vollstaendig und ohne Auffangzweig**, wie die uebrigen
    /// Fallunterscheidungen ueber [`Bereich`]: ein sechster Bereich haelt den
    /// Bau an und erzwingt einen Namen fuer seinen Schalter, statt ihn still
    /// namenlos zu lassen.
    pub const fn beschriftung(self) -> &'static str {
        match self {
            Bereich::Lesezeichen => "Lesezeichen",
            Bereich::Links => "Links",
            Bereich::Rechts => "Rechts",
            Bereich::Vorschau => "Vorschau",
            Bereich::Editor => "Editor",
        }
    }

    /// Der ausgeschriebene Name des Bereichs, fuer den Hinweistext am
    /// Schalter.
    ///
    /// Die Gegenstuecke zu [`Bereich::beschriftung`]: dort steht, was auf dem
    /// Schalter Platz hat, hier, was er bedeutet. Beide nennen denselben
    /// Bereich, und keiner der beiden Texte laesst sich aus dem anderen
    /// ableiten — "Links" ist nicht die Abkuerzung von "Linkes Dateifenster",
    /// sondern ein anderer Name fuer dieselbe Sache.
    ///
    /// **Vollstaendig und ohne Auffangzweig**, aus demselben Grund wie
    /// [`Bereich::beschriftung`].
    pub const fn langname(self) -> &'static str {
        match self {
            Bereich::Lesezeichen => "Lesezeichen- und Geräteleiste",
            Bereich::Links => "Linkes Dateifenster",
            Bereich::Rechts => "Rechtes Dateifenster",
            Bereich::Vorschau => "Vorschaufenster",
            Bereich::Editor => "Eingebauter Editor",
        }
    }
}

/// Ob der Bereich in dieser Sichtbarkeit steht.
///
/// **Die eine Zuordnung von einem [`Bereich`] auf sein Feld in
/// [`Sichtbarkeit`]**, und die Leseseite zu
/// [`Fenstermodell::sichtbar_setzen`]. [`Fenstermodell::sichtbar`] fragt hier
/// nach, statt die Zuordnung ein zweites Mal aufzuschreiben.
///
/// Frei und nicht an [`Fenstermodell`] gebunden, weil ein Aufrufer sie fuer
/// einen Stand braucht, der nicht der gehaltene ist: der Anwendungsdelegierte
/// vergleicht die Sichtbarkeit vor und nach einem Aufruf, um zu erfahren,
/// **welche** Bereiche er bewegt hat. Seit dem gegenseitigen Ausschluss aus C1
/// koennen es zwei sein.
///
/// **"Die eine" gilt seit dem 260812 wieder wortwoertlich.**
/// [`crate::appkit::aufteilung`] fuehrte bis dahin eine zweite Fassung
/// (`sichtbar_im`), die sich anfangs in einem Zweig unterschied; als Schritt 3
/// der Bereichsleisten-Runde diesen Unterschied beseitigte, standen beide Zeile
/// fuer Zeile gleich da. Die Aufteilung ruft jetzt diese hier
/// (`issues/260812-0539_*_die-zuordnung-von-bereich-auf-sichtbarkeit-steht-seit-schritt-3-zweimal-gleich-da.md`).
pub fn sichtbar_in(sichtbar: &Sichtbarkeit, bereich: Bereich) -> bool {
    match bereich {
        Bereich::Lesezeichen => sichtbar.lesezeichen,
        Bereich::Links => sichtbar.erstes_dateifenster,
        Bereich::Rechts => sichtbar.zweites_dateifenster,
        Bereich::Vorschau => sichtbar.vorschau,
        Bereich::Editor => sichtbar.editor,
    }
}

/// Die Breite, die diese [`Breiten`] fuer den Bereich tragen.
///
/// **Die eine Zuordnung von einem [`Bereich`] auf sein Feld in [`Breiten`]**,
/// die Leseseite zu [`Fenstermodell::breite_setzen`] und die Schwester von
/// [`sichtbar_in`] daneben. [`Fenstermodell::breite`] fragt hier nach, statt
/// die Zuordnung ein zweites Mal aufzuschreiben.
///
/// `None` heisst "noch nie gesetzt"; wer stattdessen die Anfangsbreite will,
/// nimmt [`Fenstermodell::breite_oder_anfang`].
fn breite_in(breiten: &Breiten, bereich: Bereich) -> Option<f64> {
    match bereich {
        Bereich::Lesezeichen => breiten.lesezeichen,
        Bereich::Links => breiten.links,
        Bereich::Rechts => breiten.rechts,
        Bereich::Vorschau => breiten.vorschau,
        Bereich::Editor => breiten.editor,
    }
}

/// Ob die Spalte in dieser Spaltensichtbarkeit steht.
///
/// **Die eine Zuordnung von einer [`Spalte`] auf ihr Feld in
/// [`Spaltensichtbarkeit`]**, die Schwester von [`sichtbar_in`] daneben und die
/// Leseseite zu [`Fenstermodell::spalte_umschalten`].
///
/// **Vollstaendig und mit [`Spalte::Name`] darin, obwohl jene kein Feld hat.**
/// Die Aufzaehlung traegt vier Werte, die Ablage drei; ohne diesen Zweig
/// braeuchte jeder Aufrufer einen eigenen, oder die Fallunterscheidung bekaeme
/// einen Auffangzweig und eine fuenfte Spalte fiele still unter den Tisch. Die
/// Namensspalte steht immer, und der Grund steht an [`Spaltensichtbarkeit`]:
/// eine Dateiliste ohne sie zeigt nichts, was den Eintrag benennt.
///
/// Frei und nicht an [`Fenstermodell`] gebunden, aus demselben Grund wie
/// [`sichtbar_in`]: ein Aufrufer braucht sie fuer einen Stand, der nicht der
/// gehaltene ist.
pub fn spalte_sichtbar_in(spalten: &Spaltensichtbarkeit, spalte: Spalte) -> bool {
    match spalte {
        Spalte::Name => true,
        Spalte::Groesse => spalten.groesse,
        Spalte::Geaendert => spalten.geaendert,
        Spalte::Typ => spalten.typ,
    }
}

/// Das gehaltene Fenstermodell.
///
/// Es traegt, was nicht zu den Tabs gehoert: das aktive Dateifenster, die
/// Sichtbarkeit der fuenf Bereiche, ihre Breiten und die Sichtbarkeit der
/// Spalten beider Dateilisten. Die Tabs selbst haelt [`Tabliste`], je eine
/// Liste je Dateifenster.
///
/// **Die Spalten stehen hier und nicht bei den Tabs**, weil ein Spaltenschalter
/// nach dem Nutzerentscheid vom 260812-0306 beide Dateilisten zugleich trifft;
/// je Tab gefuehrt waeren sie zwei mal n Wahrheiten ueber eine Angabe.
#[derive(Debug)]
pub struct Fenstermodell {
    aktiv: Fensterseite,
    breiten: Breiten,
    sichtbar: Sichtbarkeit,
    spalten: Spaltensichtbarkeit,
}

impl Fenstermodell {
    /// Das Modell aus einer geladenen Sitzung.
    ///
    /// **Drei Zusicherungen werden hier hergestellt und nicht nur
    /// unterstellt**, weil `session.toml` nach C7 zum Lesen und Aendern von
    /// Hand gedacht ist und `serde` jede Feldkombination anstandslos einliest.
    /// Zur Laufzeit haelt [`Fenstermodell::umschalten`] alle drei; die Datei
    /// kommt nicht von dort.
    ///
    /// **Ein Dateifenster bleibt sichtbar.** Sind beide ausgeblendet, kommt das
    /// linke hervor. Es ist die Regel "eines bleibt" aus dem Modulkopf, an dem
    /// einen Weg, der nicht durch `umschalten` fuehrt; welches der beiden
    /// hervorkommt, sagt die Zusage nicht, und die Wahl faellt auf das linke,
    /// weil `Fensterseite::default` es ebenfalls waehlt.
    ///
    /// **Ein ausgeblendetes Dateifenster kann nicht das aktive sein.**
    /// `aktiv = "rechts"` neben `zweites_dateifenster = false` faende der
    /// Nutzer sonst nach dem Start als Auswahl, Eingabefokus und Ziel jeder
    /// Dateioperation in einem Dateifenster, das er nicht sieht.
    ///
    /// **Vorschau und Editor stehen nie zugleich.** `vorschau = true` neben
    /// `editor = true` waere sonst der eine Weg, den das erste Abnahmekriterium
    /// von C1 der Editor-Runde ausschliesst. Weichen muss der Editor: er haelt
    /// beim Start keine Datei, und ein sichtbarer leerer Editor naehme den
    /// Dateifenstern Platz fuer nichts — dieselbe Wahl und dieselbe
    /// Begruendung, die `Sichtbarkeit::default` fuer den Auslieferungszustand
    /// trifft.
    ///
    /// **Die Reihenfolge der ersten beiden zaehlt.** Erst stehen die
    /// Dateifenster fest, dann das aktive: die zweite Zusicherung schickt die
    /// Aktivitaet auf das andere Dateifenster und braucht dafuer eines, das
    /// sichtbar ist. Umgekehrt gerechnet koennte sie auf ein Fenster zeigen,
    /// das die erste erst danach hervorholt — oder auf keines.
    pub fn aus_sitzung(sitzung: &Sitzung) -> Self {
        let mut modell = Self {
            aktiv: sitzung.aktiv,
            breiten: sitzung.breiten,
            sichtbar: sitzung.sichtbar,
            // Ohne Zusicherung uebernommen: jede der acht Kombinationen der
            // drei Felder ist eine Lage, die der Nutzer auch ueber die
            // Schalter herstellen kann, und die Namensspalte steht ohnehin.
            spalten: sitzung.spalten,
        };
        if !modell.sichtbar(Bereich::Links) && !modell.sichtbar(Bereich::Rechts) {
            modell.sichtbar_setzen(Bereich::Links, true);
        }
        if !modell.sichtbar(Bereich::von_seite(modell.aktiv)) {
            // Das andere steht, weil die Zusicherung darueber eines
            // hergestellt hat.
            modell.aktiv = modell.aktiv.andere();
        }
        if modell.sichtbar(Bereich::Vorschau) {
            modell.gegenueber_raeumen(Bereich::Vorschau);
        }
        modell
    }

    /// Die Sitzung, wie sie in `session.toml` gehoert.
    ///
    /// Die Tabs kommen von aussen dazu, weil sie in
    /// [`Tabliste`](crate::tabs::Tabliste) wohnen und nicht hier. Damit gibt es
    /// genau einen Ort je Angabe und keine zweite Wahrheit ueber die Tabs.
    ///
    /// **Die Datei des Editors kommt aus demselben Grund von aussen.** Sie
    /// wohnt in [`Editormodell`](crate::editormodell::Editormodell), und dieses
    /// Modell haelt vom Editor allein Breite und Sichtbarkeit. Sie hier aus
    /// einer zweiten Quelle zu erfragen hiesse, zwei Orte darueber zu haben,
    /// welche Datei offen ist.
    ///
    /// **Der offene Notizzettel kommt aus demselben Grund von aussen.** Er
    /// wohnt in [`Zettelmodell`](crate::zettelmodell::Zettelmodell); dieses
    /// Modell kennt vom Zettel nichts, denn er ist ein Blatt und kein Bereich
    /// der Fensterzeile. Was mitgeht, ist allein die Merkung und nie der Text
    /// (C4 der Runde 9).
    pub fn sitzung(
        &self,
        fenster: [Fensterzustand; 2],
        editor: Option<PathBuf>,
        zettel: Zettel,
    ) -> Sitzung {
        Sitzung {
            aktiv: self.aktiv,
            editor,
            zettel,
            breiten: self.breiten,
            sichtbar: self.sichtbar,
            spalten: self.spalten,
            fenster,
        }
    }

    /// Welches Dateifenster gerade das aktive ist.
    ///
    /// Bei Dateioperationen ist es die Quelle und das andere das Ziel (C1).
    pub fn aktiv(&self) -> Fensterseite {
        self.aktiv
    }

    /// Macht das genannte Dateifenster zum aktiven, falls es sichtbar ist.
    ///
    /// Liefert, ob sich dadurch etwas geaendert hat.
    pub fn aktiv_setzen(&mut self, seite: Fensterseite) -> bool {
        if seite == self.aktiv || !self.sichtbar(Bereich::von_seite(seite)) {
            return false;
        }
        self.aktiv = seite;
        true
    }

    /// Wechselt das aktive Dateifenster (C1).
    ///
    /// Ist das andere ausgeblendet, geschieht nichts: ein aktives Dateifenster,
    /// das niemand sieht, waere ein Fenster ohne sichtbare Auswahl.
    pub fn fenster_wechseln(&mut self) -> bool {
        self.aktiv_setzen(self.aktiv.andere())
    }

    /// Ob der Bereich sichtbar ist.
    pub fn sichtbar(&self, bereich: Bereich) -> bool {
        sichtbar_in(&self.sichtbar, bereich)
    }

    /// Die sichtbaren Bereiche, von links nach rechts.
    ///
    /// **Die eine Stelle, die diese Liste bildet.** Drei Rechnungen brauchen
    /// sie und meinen dieselbe Menge: die Anteilsregel in [`bereichsbreiten`],
    /// die Rueckrechnung in [`Self::breiten_uebernehmen`] und der Massstab in
    /// [`Self::massstab`]. Dreimal ausgeschrieben waere sie dreimal
    /// nachzuziehen, sobald ein sechster Bereich dazukommt.
    fn sichtbare(&self) -> Vec<Bereich> {
        Bereich::ALLE
            .into_iter()
            .filter(|bereich| self.sichtbar(*bereich))
            .collect()
    }

    /// Setzt die Sichtbarkeit eines Bereichs.
    ///
    /// **Die eine Stelle, die ein Feld von [`Sichtbarkeit`] schreibt.** Der
    /// gegenseitige Ausschluss aus [`Bereich::teilt_flaeche_mit`] wirkt ueber
    /// sie und nicht neben ihr; wer sie umgeht, hat eine zweite Wahrheit
    /// darueber, welche Bereiche stehen.
    ///
    /// **Sie schreibt, was ihr gesagt wird, und prueft nichts.** Die Regel
    /// "eines bleibt" haelt [`Self::umschalten`] vor dem Aufruf; hier stuende
    /// sie ein zweites Mal, und [`Self::aus_sitzung`] koennte das letzte
    /// Dateifenster dann nicht mehr hervorholen.
    fn sichtbar_setzen(&mut self, bereich: Bereich, sichtbar: bool) {
        match bereich {
            Bereich::Lesezeichen => self.sichtbar.lesezeichen = sichtbar,
            Bereich::Links => self.sichtbar.erstes_dateifenster = sichtbar,
            Bereich::Rechts => self.sichtbar.zweites_dateifenster = sichtbar,
            Bereich::Vorschau => self.sichtbar.vorschau = sichtbar,
            Bereich::Editor => self.sichtbar.editor = sichtbar,
        }
    }

    /// Blendet den Bereich aus, der sich mit dem genannten die Flaeche teilt.
    ///
    /// Zu rufen, nachdem der genannte Bereich sichtbar geworden ist. Teilt er
    /// sich seine Flaeche mit keinem, geschieht nichts.
    fn gegenueber_raeumen(&mut self, bereich: Bereich) {
        if let Some(gegenueber) = bereich.teilt_flaeche_mit() {
            self.sichtbar_setzen(gegenueber, false);
        }
    }

    /// Die Sichtbarkeit aller fuenf Bereiche, von links nach rechts.
    pub fn sichtbarkeit(&self) -> Sichtbarkeit {
        self.sichtbar
    }

    /// Die Sichtbarkeit der drei schaltbaren Spalten.
    ///
    /// Sie gilt fuer **beide** Dateilisten; die Begruendung steht an
    /// [`Spaltensichtbarkeit`]. Wer eine einzelne Spalte fragt, nimmt
    /// [`spalte_sichtbar_in`] und bekommt auch fuer [`Spalte::Name`] eine
    /// Antwort.
    pub fn spaltensichtbarkeit(&self) -> Spaltensichtbarkeit {
        self.spalten
    }

    /// Blendet eine Spalte beider Dateilisten aus oder wieder ein (C3 der
    /// Bereichsleisten-Runde).
    ///
    /// Liefert, ob sich dadurch etwas geaendert hat. **Fuer [`Spalte::Name`]
    /// ist die Antwort `false`**, und sie bleibt stumm wie jede Abweisung in
    /// diesem Modell: die Namensspalte traegt keinen Schalter, weil eine
    /// Dateiliste ohne sie nichts zeigt, was den Eintrag benennt. Sie hier
    /// abzuweisen statt sie gar nicht erst anzubieten ist der Preis dafuer,
    /// dass die Aufzaehlung vier Werte hat und die Ablage drei; die Alternative
    /// waere eine zweite Aufzaehlung der schaltbaren Spalten neben
    /// [`Spalte::ALLE`].
    ///
    /// **Die Sortierung wird nicht angefasst.** Das ist Kriterium C3.3 und der
    /// Nutzerentscheid vom 260812-0306
    /// (`decisions/260812-0306_*_was-geschieht-mit-der-sortierung-wenn-die-sortierspalte-weggeschaltet-wird.md`):
    /// ein Spaltenschalter verbirgt eine Spalte und tut sonst nichts. Wer nach
    /// Groesse sortiert und die Spalte Groesse wegschaltet, sieht dieselbe
    /// Reihenfolge wie zuvor. Der Sortierschluessel wohnt ohnehin in
    /// [`Tabliste`](crate::tabs::Tabliste) und nicht hier; diese Funktion
    /// **kann** ihn nicht anfassen, und das ist die billigste Form, die Zusage
    /// zu halten.
    ///
    /// **Kein [`Zeilenmass`]**, anders als bei [`Self::umschalten`]: eine
    /// Spalte liegt in der Dateiliste und nicht in der Fensterzeile. Die
    /// Breiten der fuenf Bereiche stehen vorher und nachher gleich (C3.4), und
    /// deshalb gibt es hier auch keine Abweisung an den Mindestbreiten.
    #[must_use = "die Abweisung an der Namensspalte bleibt stumm; wer sie nicht liest, haelt eine Spalte fuer geschaltet, die das Modell nicht angefasst hat"]
    pub fn spalte_umschalten(&mut self, spalte: Spalte) -> bool {
        match spalte {
            Spalte::Name => false,
            Spalte::Groesse => {
                self.spalten.groesse = !self.spalten.groesse;
                true
            }
            Spalte::Geaendert => {
                self.spalten.geaendert = !self.spalten.geaendert;
                true
            }
            Spalte::Typ => {
                self.spalten.typ = !self.spalten.typ;
                true
            }
        }
    }

    /// Blendet einen Bereich aus oder wieder ein (C7).
    ///
    /// **Zwei Abweisungen liefern `false`, und beide bleiben stumm.** C7
    /// verlangt das fuer die erste ausdruecklich ("wird ohne Fehlermeldung
    /// ignoriert"), und zwei verschiedene Antworten auf zwei unmoegliche
    /// Sichtbarkeitsanforderungen waeren eine Fallunterscheidung ohne Grund:
    ///
    /// 1. **Das letzte sichtbare Dateifenster bleibt stehen**, gleich welches
    ///    der beiden es ist. Die Frage lautet nicht "ist es das linke", sondern
    ///    "ist es ein Dateifenster, und steht das andere"; das erste
    ///    beantwortet [`Bereich::seite`], das zweite die Sichtbarkeit des
    ///    Gegenuebers. Seit der Bereichsleisten-Runde laesst sich auch das
    ///    linke ausblenden, siehe den Modulkopf.
    /// 2. **Die Mindestbreiten muessen hineinpassen.** Wuerde die Summe der
    ///    Mindestbreiten der nach dem Umschalten sichtbaren Bereiche die dann
    ///    verfuegbare Breite uebersteigen, geschieht nichts; die Rechnung steht
    ///    in [`Self::mindestbreiten_passen`].
    ///
    /// **Die zweite greift nur beim Einschalten.** Ein Ausschaltbefehl kann die
    /// Summe der Mindestbreiten nicht vergroessern, und ihn an derselben
    /// Bedingung scheitern zu lassen hielte ein zu schmal gezogenes Fenster in
    /// seiner Enge fest, statt sie aufloesen zu lassen.
    ///
    /// **Wer nach dem Umschalten sichtbar ist, weiss allein dieses Modell**,
    /// und deshalb reist die Geometrie der Zeile als [`Zeilenmass`] herein,
    /// statt dass der Aufrufer die fertige Antwort mitbraechte: der
    /// gegenseitige Ausschluss kann die Anzahl der sichtbaren Bereiche gleich
    /// lassen — der eingeschaltete Editor verdraengt die Vorschau — oder um
    /// eins erhoehen, und davon haengt ab, wie viele Trennlinien abgehen.
    ///
    /// War der ausgeblendete Bereich das aktive Dateifenster, wandert die
    /// Aktivitaet auf das andere.
    ///
    /// **Wird der Bereich sichtbar, weicht sein Gegenueber.** Das ist der
    /// gegenseitige Ausschluss aus C1 der Editor-Runde, und er steht hier fuer
    /// beide Richtungen in einer Zeile: Vorschau und Editor sind dasselbe Paar,
    /// gleich von welcher Seite man kommt. [`Self::einblenden`] geht durch
    /// diese Funktion und erbt ihn damit, statt ihn ein zweites Mal
    /// aufzuschreiben. Welches Paar es ist, sagt [`Bereich::teilt_flaeche_mit`].
    ///
    /// **Ein verdraengter Editor verliert nichts.** Der Wechsel der
    /// Sichtbarkeit setzt `hidden` an den Ansichten und fasst das
    /// [`crate::editormodell::Editormodell`] nicht an; der gehaltene Stand steht
    /// hinterher unveraendert da. Deshalb geht dem Einblenden der Vorschau seit
    /// dem Nutzerentscheid vom 260810-0250 keine Nachfrage aus C4 mehr voraus
    /// (`decisions/260810-0021_*_was-verwirft-verwerfen-wenn-die-vorschau-den-editor-nur-verdraengt.md`).
    #[must_use = "eine Abweisung bleibt stumm; wer sie nicht liest, haelt eine Sichtbarkeit fuer hergestellt, die das Modell nicht angenommen hat"]
    pub fn umschalten(&mut self, bereich: Bereich, mass: Zeilenmass) -> bool {
        let jetzt_sichtbar = !self.sichtbar(bereich);
        if jetzt_sichtbar && !self.mindestbreiten_passen(bereich, mass) {
            return false;
        }
        match bereich.seite() {
            // Ein Dateifenster geht nur aus, solange das andere steht, und gibt
            // die Aktivitaet an dieses ab. Beide gehen durch diesen einen
            // Zweig; welcher Bereich ein Dateifenster ist, sagt
            // `Bereich::seite`.
            Some(seite) if !jetzt_sichtbar => {
                if !self.sichtbar(Bereich::von_seite(seite.andere())) {
                    return false;
                }
                self.sichtbar_setzen(bereich, false);
                if self.aktiv == seite {
                    self.aktiv = seite.andere();
                }
            }
            // Alles uebrige ist ein blosser Wechsel der Sichtbarkeit: ein
            // Dateifenster, das eingeblendet wird, und jeder Bereich, der
            // keines ist. Der Auffangzweig steht ueber `Option` und nicht ueber
            // `Bereich`; die vollstaendige Fallunterscheidung darueber ist
            // `Bereich::seite` und bleibt es.
            _ => self.sichtbar_setzen(bereich, jetzt_sichtbar),
        }
        if jetzt_sichtbar {
            self.gegenueber_raeumen(bereich);
        }
        true
    }

    /// Ob die Mindestbreiten noch in die Zeile passen, wenn der Bereich
    /// dazukommt.
    ///
    /// Gefragt wird nach der Lage **nach** dem Einschalten: der genannte
    /// Bereich steht dann, sein Gegenueber aus [`Bereich::teilt_flaeche_mit`]
    /// steht dann nicht, und alle uebrigen stehen wie bisher. Aus dieser einen
    /// Menge folgen beide Groessen der Frage — die Summe der Mindestbreiten und
    /// die Anzahl der Trennlinien, die [`Zeilenmass::verfuegbar`] abzieht.
    ///
    /// **Die Anzahl kann dabei gleich bleiben.** Verdraengt der Editor die
    /// Vorschau, stehen vorher und nachher gleich viele Bereiche, und es
    /// entscheidet allein, dass der Editor 320 Punkte verlangt und die Vorschau
    /// 160. Genau deshalb kann der Aufrufer die Frage nicht stellen: er kennt
    /// die Menge nicht, die nach seinem Befehl steht.
    fn mindestbreiten_passen(&self, bereich: Bereich, mass: Zeilenmass) -> bool {
        let weicht = bereich.teilt_flaeche_mit();
        let danach: Vec<Bereich> = Bereich::ALLE
            .into_iter()
            .filter(|kandidat| {
                *kandidat == bereich || (Some(*kandidat) != weicht && self.sichtbar(*kandidat))
            })
            .collect();
        let mindestsumme: f64 = danach.iter().map(|bereich| bereich.mindestbreite()).sum();
        mindestsumme <= mass.verfuegbar(danach.len())
    }

    /// Holt einen ausgeblendeten Bereich hervor und blendet nie einen aus.
    ///
    /// Liefert, ob sich dadurch etwas geaendert hat; ein schon sichtbarer
    /// Bereich liefert `false` und bleibt unangetastet.
    ///
    /// **Die eine Stelle der Asymmetrie**, und sie steht neben
    /// [`Fenstermodell::umschalten`] statt bei den Befehlen, die sie brauchen.
    /// Drei tun das heute: `shift+f3` aus C10 zeigt den Inhalt der
    /// Zwischenablage in der Vorschau, und seit dem Nutzerentscheid vom 260807
    /// holen auch die Fokusbefehle ihren Bereich hervor
    /// (`decisions/260805-1730_*_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md`).
    /// Der gemeinsame Grund: wer einen Bereich verlangt, verlangt damit, ihn
    /// zu sehen; ausblenden tut keiner von ihnen, dafuer bleiben die Befehle
    /// aus C7. Es entsteht deshalb keine zweite Wahrheit ueber die
    /// Sichtbarkeit, sondern dieselbe Asymmetrie an einer Stelle.
    ///
    /// **Den gegenseitigen Ausschluss erbt diese Funktion von
    /// [`Self::umschalten`]**, durch das sie geht: `einblenden(Editor)` blendet
    /// damit die Vorschau aus, ohne dass der Ausschluss hier ein zweites Mal
    /// stuende. "Blendet nie einen aus" gilt weiterhin fuer den **genannten**
    /// Bereich; das Gegenueber weicht, weil beide dieselbe Flaeche haben.
    ///
    /// **Auch die Abweisung an den Mindestbreiten erbt es von dort**, und mit
    /// ihr das [`Zeilenmass`], das es allein deshalb annimmt und unveraendert
    /// weiterreicht.
    #[must_use = "eine Abweisung bleibt stumm; wer sie nicht liest, haelt einen Bereich fuer hervorgeholt, den das Modell nicht eingeblendet hat"]
    pub fn einblenden(&mut self, bereich: Bereich, mass: Zeilenmass) -> bool {
        if self.sichtbar(bereich) {
            return false;
        }
        self.umschalten(bereich, mass)
    }

    /// Die gespeicherten Breiten.
    pub fn breiten(&self) -> Breiten {
        self.breiten
    }

    /// Aendert die Breite eines Bereichs um den genannten Betrag.
    ///
    /// Der Weg der beiden Tastenbefehle aus C7. Ein Randbereich waechst
    /// unmittelbar; bei einem Dateifenster **verschiebt der Befehl die
    /// Trennlinie**, das heisst das andere Dateifenster gibt genau so viel ab,
    /// wie dieses bekommt. Der Unterschied ist nicht kosmetisch: die beiden
    /// Dateifenster teilen sich ihren Platz nach dem Verhaeltnis ihrer Breiten
    /// (siehe [`bereichsbreiten`]), und eine Zahl, die nur den Zaehler erhoeht,
    /// verschoebe die Linie um einen Bruchteil des Schritts. Gemessen am
    /// 260804 im laufenden Buendel: 40 Punkte Zuwachs bewegten die Linie um 13.
    ///
    /// Am Mindestmass hoert der Schritt auf, statt es zu unterschreiten.
    /// **Passen die beiden Mindestmasse gar nicht mehr nebeneinander, bleibt
    /// der Befehl ohne Wirkung** — es gibt dann keine Lage der Trennlinie, die
    /// beide haelt, und dieselbe Antwort gibt der Schirm ohnehin. Das ist
    /// dieselbe Antwort wie bei einem einzigen sichtbaren Dateifenster.
    ///
    /// Bis zum 260812 fehlte diese Feststellung, und die Deckelungskette
    /// unterstellte sie stillschweigend: liegt die untere Schranke ueber der
    /// oberen, gewinnt `.max()` ueber `.min()`, und das Vorzeichen des Betrags
    /// spielt keine Rolle mehr. `opt+cmd+links` und `opt+cmd+rechts` taten dann
    /// dasselbe und beide das Gegenteil ihres Namens; bei 780 Punkten
    /// Fensterbreite mit sichtbarem Editor waren es 71,05 Punkte statt der 40,
    /// die C4.9 zusagt. Der Datensatz ist
    /// `issues/260812-0539_*_die-breitenbefehle-aus-c7-wirken-unter-der-mindestsumme-in-die-falsche-richtung.md`.
    ///
    /// **Der Schritt gilt in Punkten auf dem Schirm und wird hier in
    /// gespeicherte Punkte umgerechnet**, ueber [`Self::massstab`] und mit ihm
    /// die Mindestbreiten, gegen die er deckelt. Beide Massstaebe sind dieselbe
    /// Zahl, solange die Summe der gespeicherten Breiten der sichtbaren
    /// Bereiche die verfuegbare Breite trifft; sonst gehen sie auseinander, und
    /// seit die Breiten Anteile sind, faellt der Faktor nicht mehr von selbst
    /// auf 1 zurueck, weil
    /// [`Fenstermodell::breiten_uebernehmen`](Self::breiten_uebernehmen) die
    /// gespeicherte Summe festhaelt. Ohne die Umrechnung sprang die Trennlinie
    /// bei 1920 Punkten Fensterbreite um 60 Punkte statt um die 40, die C7
    /// zusagt; der Datensatz dazu ist
    /// `issues/260812-0439_*_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`
    /// (behoben).
    ///
    /// **Das [`Zeilenmass`] kommt aus derselben Durchreichung wie das von
    /// [`Self::umschalten`].** Einen zweiten Weg an die Fensterbreite gibt es
    /// nicht: dieses Modell kennt AppKit nicht und kann die Zahl nicht selbst
    /// erfragen.
    ///
    /// **Das Gegenueber kommt aus [`Fensterseite::andere`]** und nicht aus einer
    /// eigenen Fallunterscheidung ueber [`Bereich`]. Bis zum 260809 stand hier
    /// ein `match` mit dem Auffangzweig `_ => Bereich::Links`; er gab die
    /// richtige Antwort, weil nur die beiden Dateifenster hierher kommen, und
    /// hat den fuenften Bereich der Editor-Runde stumm aufgenommen, ohne dass
    /// der Uebersetzer eine Einordnung verlangt haette. Die Frage "welcher
    /// Bereich ist das Gegenueber" wird jetzt nicht mehr richtig beantwortet,
    /// sondern gar nicht mehr gestellt.
    pub fn breite_aendern(&mut self, bereich: Bereich, betrag: f64, mass: Zeilenmass) {
        let massstab = self.massstab(mass);
        let betrag = betrag * massstab;
        // Auch die Grenzen stehen auf dem Schirm und nicht in der Ablage: ein
        // Bereich, der gedeckelt werden soll, sobald **auf dem Schirm** sein
        // Mindestmass erreicht ist, wird gegen dessen gespeicherte Entsprechung
        // gedeckelt.
        let mindestmass = |bereich: Bereich| bereich.mindestbreite() * massstab;
        if let Some(seite) = bereich.seite() {
            let anderer = Bereich::von_seite(seite.andere());
            if !self.sichtbar(anderer) {
                // Ein einziges sichtbares Dateifenster nimmt ohnehin die ganze
                // Breite; es gibt keine Trennlinie, die sich verschieben liesse.
                return;
            }
            let hier = self.breite_oder_anfang(bereich);
            let dort = self.breite_oder_anfang(anderer);
            // So viel kann das andere Dateifenster hoechstens abgeben, und so
            // viel muss dieses mindestens bekommen. Liegt die untere Schranke
            // ueber der oberen, passen die beiden Mindestmasse nicht
            // nebeneinander: keine Lage der Trennlinie haelt beide, und der
            // Befehl bleibt ohne Wirkung. Ohne diese Feststellung gewaenne
            // `.max()` ueber `.min()`, und beide Richtungen taeten dasselbe.
            let obergrenze = dort - mindestmass(anderer);
            let untergrenze = mindestmass(bereich) - hier;
            if untergrenze > obergrenze {
                return;
            }
            let betrag = betrag.min(obergrenze).max(untergrenze);
            self.breite_setzen(bereich, hier + betrag);
            self.breite_setzen(anderer, dort - betrag);
            return;
        }
        let jetzt = self.breite_oder_anfang(bereich);
        self.breite_setzen(bereich, (jetzt + betrag).max(mindestmass(bereich)));
    }

    /// Wie viele gespeicherte Punkte ein Punkt auf dem Schirm wert ist.
    ///
    /// **Der Faktor zwischen den beiden Massstaeben dieses Modells.** Eine
    /// gespeicherte Breite ist ein Wunsch unter mehreren (siehe
    /// [`bereichsbreiten`]); auf dem Schirm steht davon der Anteil an der
    /// verfuegbaren Breite, also das Gespeicherte mal `verfuegbar /
    /// gespeicherte Summe`. Wer eine Zahl vom Schirm in die Ablage traegt, geht
    /// durch den Kehrwert, und das ist dieser Faktor.
    ///
    /// Er gilt genau, solange kein sichtbarer Bereich an seinem Mindestmass
    /// haengt; dort ist die Abbildung nicht mehr linear, weil ein gedeckelter
    /// Bereich seinen Anteil nicht mehr mitbewegt. **Der Schritt wird dadurch
    /// ungenau, und das bleibt so**: eine Sonderregel dafuer waere ein zweiter
    /// Rechenweg neben [`bereichsbreiten`], und die eine Regel ist mehr wert
    /// als ein genauer Sonderfall.
    ///
    /// **Ungenau ist nicht dasselbe wie falsch herum**, und der zweite Fall ist
    /// seit dem 260812 behandelt: dass die Deckelung in
    /// [`Self::breite_aendern`] das Vorzeichen des Betrags verschluckt, faengt
    /// dort die Feststellung ab, ob die beiden Mindestmasse ueberhaupt
    /// nebeneinander passen. Dieser Faktor bleibt davon unberuehrt.
    ///
    /// Ohne verfuegbare Breite oder ohne gespeicherte Summe ist er 1. Es gibt
    /// dann keinen Schirm, auf dem ein Unterschied zu sehen waere, und ein
    /// Faktor von 0 fraesse jeden Befehl.
    fn massstab(&self, mass: Zeilenmass) -> f64 {
        let sichtbare = self.sichtbare();
        let verfuegbar = mass.verfuegbar(sichtbare.len());
        let gespeichert: f64 = sichtbare
            .iter()
            .map(|bereich| self.breite_oder_anfang(*bereich))
            .sum();
        if verfuegbar <= 0.0 || gespeichert <= 0.0 {
            return 1.0;
        }
        gespeichert / verfuegbar
    }

    /// Uebernimmt die Breiten, die gerade wirklich auf dem Schirm stehen.
    ///
    /// Gerufen, bevor die Sitzung geschrieben wird und bevor ein Tastenbefehl
    /// eine Breite aendert: eine mit der Maus verschobene Trennlinie steht in
    /// den Rahmen der Ansichten und nirgends sonst. Ein ausgeblendeter Bereich
    /// bleibt unangetastet, weil C7 verlangt, dass das Wiedereinblenden seinen
    /// Anteil herstellt.
    ///
    /// **Die gemessenen Punktzahlen werden auf die gespeicherte Summe
    /// zurueckgerechnet und nicht roh uebernommen.** Eine gespeicherte Breite
    /// ist ein Anteil (siehe [`bereichsbreiten`]), und ein Anteil gilt nur im
    /// Verhaeltnis zu den uebrigen — auch zu denen, die gerade nicht stehen und
    /// deren Zahl niemand nachfuehrt. Roh uebernommen wuechse die Summe der
    /// sichtbaren mit jedem Ausblenden und mit jedem Groesserziehen des
    /// Fensters, und die Zahl eines ausgeblendeten Bereichs schrumpfte gegen
    /// sie zusammen. Der Faktor haelt beides: das Vergroessern des Fensters
    /// aendert keine gespeicherte Breite, und ein wiedereingeblendeter Bereich
    /// kommt auf seinem Anteil zurueck.
    ///
    /// **Der am 260804 im laufenden Buendel gemessene Fehler faellt damit aus
    /// der Regel an und ist keine Sonderregel mehr.** Bis zur
    /// Bereichsleisten-Runde stand hier eine Ausnahme fuer die beiden
    /// Dateifenster: solange nur eines von ihnen sichtbar war, blieben beide
    /// unangetastet, weil das sichtbare den Platz des anderen mittrug und
    /// dieses sonst nach dem Wiedereinblenden auf 269 Punkten statt auf seinen
    /// 406 zurueckkam. Unter der Anteilsregel gilt derselbe Fehler fuer jedes
    /// Paar von Bereichen, weil jedes Ausblenden alle uebrigen aufblaeht; die
    /// Ausnahme waere damit zu wenig, und der Faktor ist die allgemeine
    /// Antwort.
    ///
    /// **Ein sichtbarer Bereich ohne gemessene Breite bleibt aussen vor**, in
    /// beiden Summen und beim Setzen. Er steht im Modell schon und auf dem
    /// Schirm noch nicht; seine 0 als Wunsch zu uebernehmen liesse ihn beim
    /// naechsten Auslegen zusammenfallen.
    ///
    /// **Uebernommen wird nur, was die Regel nicht selbst ausgelegt hat.**
    /// Steht auf dem Schirm genau das, was [`bereichsbreiten`] aus den
    /// gespeicherten Breiten rechnet, dann traegt keine gemessene Zahl einen
    /// Wunsch, den das Modell nicht schon haette, und dieser Aufruf bleibt
    /// ohne Wirkung; die Frage stellt [`traegt_eine_ziehbewegung`]. Der
    /// Unterschied faellt genau dort an, wo ein Bereich an seinem Mindestmass
    /// haengt: dann ist die Abbildung vom Wunsch auf den Schirm kein Faktor
    /// mehr, und die Rueckrechnung machte die gedeckelte Breite zum neuen
    /// Wunsch. Das ist der Datensatz
    /// `issues/260812-0539_*_ein-zusammengezogenes-fenster-ersetzt-die-aufteilung-des-nutzers-dauerhaft.md`.
    pub fn breiten_uebernehmen(&mut self, gemessen: [f64; 5], mass: Zeilenmass) {
        if !traegt_eine_ziehbewegung(mass, &self.breiten, &self.sichtbar, &gemessen) {
            return;
        }
        let nachzufuehren: Vec<Bereich> = self
            .sichtbare()
            .into_iter()
            .filter(|bereich| gemessen[bereich.index()] > 0.0)
            .collect();
        let gespeichert: f64 = nachzufuehren
            .iter()
            .map(|bereich| self.breite_oder_anfang(*bereich))
            .sum();
        let gemessene_summe: f64 = nachzufuehren
            .iter()
            .map(|bereich| gemessen[bereich.index()])
            .sum();
        if gespeichert <= 0.0 || gemessene_summe <= 0.0 {
            return;
        }
        let faktor = gespeichert / gemessene_summe;
        for bereich in nachzufuehren {
            self.breite_setzen(bereich, gemessen[bereich.index()] * faktor);
        }
    }

    /// Die gespeicherte Breite eines Bereichs, sonst seine Anfangsbreite.
    fn breite_oder_anfang(&self, bereich: Bereich) -> f64 {
        self.breite(bereich)
            .unwrap_or_else(|| bereich.anfangsbreite())
    }

    /// In welcher Reihenfolge die Tabs beim Start gelesen werden.
    ///
    /// Zuerst der sichtbare Tab jedes sichtbaren Dateifensters, danach alles
    /// uebrige. Die Reihenfolge folgt aus C8: L4 endet, sobald die sichtbaren
    /// Tabs ihre erste Bildschirmseite zeigen.
    pub fn lesereihenfolge(&self, fenster: [Tabuebersicht; 2]) -> Vec<(Fensterseite, usize)> {
        let mut reihenfolge = Vec::new();
        for seite in Fensterseite::ALLE {
            if self.sichtbar(Bereich::von_seite(seite)) {
                reihenfolge.push((seite, fenster[seite.index()].sichtbar));
            }
        }
        for seite in Fensterseite::ALLE {
            for stelle in 0..fenster[seite.index()].zahl {
                if !reihenfolge.contains(&(seite, stelle)) {
                    reihenfolge.push((seite, stelle));
                }
            }
        }
        reihenfolge
    }

    /// Die gespeicherte Breite eines Bereichs, falls es eine gibt.
    fn breite(&self, bereich: Bereich) -> Option<f64> {
        breite_in(&self.breiten, bereich)
    }

    fn breite_setzen(&mut self, bereich: Bereich, breite: f64) {
        let feld = match bereich {
            Bereich::Lesezeichen => &mut self.breiten.lesezeichen,
            Bereich::Links => &mut self.breiten.links,
            Bereich::Rechts => &mut self.breiten.rechts,
            Bereich::Vorschau => &mut self.breiten.vorschau,
            Bereich::Editor => &mut self.breiten.editor,
        };
        *feld = Some(breite);
    }
}

/// Das Mass der Fensterzeile: ihre Breite und die einer Trennlinie.
///
/// **Die eine Stelle, an der "n sichtbare Bereiche brauchen n minus eine
/// Trennlinie" gerechnet wird.** Bis zur Bereichsleisten-Runde stand die
/// Rechnung in [`crate::appkit::aufteilung`], also dort, wo die beiden Zahlen
/// aus AppKit kommen, und [`bereichsbreiten`] bekam allein das Ergebnis. Damit
/// konnte das Modell nicht beantworten, was eine **andere** Anzahl sichtbarer
/// Bereiche kosten wuerde — genau die Frage, die eine Abweisung an den
/// Mindestbreiten stellen muss. Die Geometrie reist deshalb als Wert.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zeilenmass {
    /// Die volle Breite der Fensterzeile, die Trennlinien eingeschlossen.
    pub gesamt: f64,
    /// Die Breite einer Trennlinie zwischen zwei Bereichen.
    pub trennerbreite: f64,
}

impl Zeilenmass {
    /// Was den Bereichen bleibt, wenn `anzahl_sichtbar` von ihnen stehen.
    ///
    /// Zwischen n sichtbaren Bereichen liegen n minus eine Trennlinie; bei
    /// keinem und bei einem liegt keine. Nie weniger als nichts.
    pub fn verfuegbar(&self, anzahl_sichtbar: usize) -> f64 {
        let trenner = self.trennerbreite * anzahl_sichtbar.saturating_sub(1) as f64;
        (self.gesamt - trenner).max(0.0)
    }
}

/// Verteilt den Platz der Fensterzeile auf die fuenf Bereiche.
///
/// **Die eine Breitenregel des Programms.** Sie steht hier und nirgends sonst;
/// [`crate::appkit::aufteilung`] setzt nur um, was hier herauskommt.
///
/// Ein ausgeblendeter Bereich bekommt `0.0`; seine gespeicherte Breite bleibt
/// unangetastet und steht beim Wiedereinblenden wieder zur Verfuegung.
///
/// # Ein Anteil je sichtbarem Bereich
///
/// Jeder sichtbare Bereich geht mit einem **Wunsch** in die Rechnung: mit
/// seiner gespeicherten Breite, ersatzweise mit seiner
/// [`Bereich::anfangsbreite`]. Die verfuegbare Breite verteilt sich im
/// Verhaeltnis dieser Wuensche. Eine gespeicherte Breite ist damit keine Zusage
/// in Punkten, sondern ein Anteil: zwei Bereiche im Verhaeltnis 2:1 stehen auch
/// dann noch so zueinander, wenn ein dritter dazukommt, und zwar gleich welche
/// drei es sind.
///
/// Zwei Zweige, und sie schliessen einander aus:
///
/// 1. **Die Mindestbreiten passen in die Zeile.** Dann gilt das Verhaeltnis,
///    aber das Mindestmass gewinnt gegen den Anteil: wer unter sein Mindestmass
///    gedrueckt wuerde, bekommt es, scheidet aus der Verteilung aus, und die
///    uebrigen teilen den Rest weiter im Verhaeltnis ihrer Wuensche. Diese
///    Wasserstandsrechnung endet nach hoechstens so vielen Durchgaengen, wie es
///    sichtbare Bereiche gibt, weil jeder Durchgang mindestens einen
///    herausnimmt.
/// 2. **Sie passen nicht mehr hinein.** Dann bekommt jeder sichtbare Bereich
///    sein Mindestmass mal dem Verhaeltnis von verfuegbarer Breite zu
///    Mindestsumme, schrumpft also mit demselben Faktor wie alle anderen unter
///    sein Mindestmass. Dieser Fall entsteht nicht durch einen Umschaltbefehl,
///    sondern allein dadurch, dass der Nutzer das Fenster schmaler zieht, als
///    die Mindestbreiten erlauben.
///
/// Die Summe der fuenf Werte ist in beiden Zweigen genau die verfuegbare
/// Breite; dafuer bekommt der jeweils letzte Bereich den Rest und nicht seinen
/// gerundeten Anteil (siehe [`anteilig`]).
pub fn bereichsbreiten(mass: Zeilenmass, breiten: &Breiten, sichtbar: &Sichtbarkeit) -> [f64; 5] {
    // Ein Modell allein, um an `sichtbare()` heranzukommen. Die beiden Felder,
    // die die Breitenrechnung nicht liest, stehen auf ihrem Vorgabewert: die
    // Spalten liegen in der Dateiliste und nicht in der Fensterzeile, und
    // welches Dateifenster das aktive ist, aendert an keiner Breite etwas.
    let modell = Fenstermodell {
        aktiv: Fensterseite::Links,
        breiten: *breiten,
        sichtbar: *sichtbar,
        spalten: Spaltensichtbarkeit::default(),
    };
    let mut ergebnis = [0.0_f64; 5];

    // Welche Bereiche etwas bekommen, sagt allein die Sichtbarkeit. Bis zur
    // Editor-Runde stand hier die Literalliste der festen Bereiche als zweite
    // Aufzaehlung daneben; seit der Anteilsregel gibt es keine festen Bereiche
    // mehr, an denen eine solche Liste ansetzen koennte.
    let sichtbare = modell.sichtbare();
    if sichtbare.is_empty() {
        // Ohne sichtbaren Bereich gibt es nichts zu verteilen. Der Fall tritt
        // nicht ein: `Fenstermodell::umschalten` haelt ein Dateifenster.
        return ergebnis;
    }

    let verfuegbar = mass.verfuegbar(sichtbare.len());
    let mindestsumme: f64 = sichtbare
        .iter()
        .map(|bereich| bereich.mindestbreite())
        .sum();

    if verfuegbar < mindestsumme {
        // Zweiter Zweig: alle schrumpfen mit demselben Faktor. `mindestsumme`
        // ist dabei groesser als `verfuegbar` und damit groesser als 0.
        let anteile: Vec<(Bereich, f64)> = sichtbare
            .iter()
            .map(|bereich| (*bereich, bereich.mindestbreite()))
            .collect();
        anteilig(&mut ergebnis, &anteile, verfuegbar);
        return ergebnis;
    }

    // Der Wunsch eines Bereichs, nie kleiner als 1: ein Wunsch von 0 bekaeme
    // keinen Anteil und nur ueber sein Mindestmass wieder Breite.
    let wunsch = |bereich: Bereich| modell.breite_oder_anfang(bereich).max(1.0);

    let mut offen = sichtbare;
    let mut rest = verfuegbar;
    loop {
        let wunschsumme: f64 = offen.iter().map(|bereich| wunsch(*bereich)).sum();
        let zu_klein: Vec<Bereich> = offen
            .iter()
            .copied()
            .filter(|bereich| rest * wunsch(*bereich) / wunschsumme < bereich.mindestbreite())
            .collect();
        if zu_klein.is_empty() {
            let anteile: Vec<(Bereich, f64)> = offen
                .iter()
                .map(|bereich| (*bereich, wunsch(*bereich)))
                .collect();
            anteilig(&mut ergebnis, &anteile, rest);
            return ergebnis;
        }
        for bereich in &zu_klein {
            ergebnis[bereich.index()] = bereich.mindestbreite();
            rest -= bereich.mindestbreite();
        }
        // Jeder Durchgang nimmt mindestens einen Bereich heraus, also endet die
        // Schleife. Dass nie alle zugleich herausfallen, folgt aus dem Zweig
        // darueber: die Summe der Anteile ist der Rest, und der reicht fuer die
        // Mindestbreiten der offenen Bereiche. Faellt der letzte doch heraus,
        // trifft der naechste Durchgang auf eine leere Liste und bricht ab.
        offen.retain(|bereich| !zu_klein.contains(bereich));
    }
}

/// Verteilt `gesamt` auf die genannten Bereiche im Verhaeltnis ihrer Gewichte.
///
/// **Der letzte bekommt den Rest und nicht seinen gerundeten Anteil.** Damit
/// ist die Summe der ausgegebenen Breiten genau `gesamt` und nicht `gesamt`
/// plus n Rundungsfehler. Die Aufteilung rechnet aus diesen Breiten die Lage
/// jeder Trennlinie, und ein halber Punkt am rechten Rand waere dort zu sehen.
fn anteilig(ergebnis: &mut [f64; 5], anteile: &[(Bereich, f64)], gesamt: f64) {
    let Some(((letzter, _), vordere)) = anteile.split_last() else {
        return;
    };
    let summe: f64 = anteile.iter().map(|(_, gewicht)| gewicht).sum();
    let mut vergeben = 0.0;
    for (bereich, gewicht) in vordere {
        let breite = gesamt * gewicht / summe;
        ergebnis[bereich.index()] = breite;
        vergeben += breite;
    }
    ergebnis[letzter.index()] = gesamt - vergeben;
}

/// Wie weit eine gemessene Breite von der ausgelegten abweichen darf, ohne als
/// Ziehbewegung zu gelten.
///
/// Ein Viertelpunkt. Er liegt unter dem kleinsten Schritt, mit dem sich eine
/// Trennlinie ziehen laesst — ein halber Punkt auf einem Schirm mit doppelter
/// Aufloesung —, und ueber dem, was ein Runden der Rahmen auf ganze
/// Bildpunkte an Unterschied hinterliesse. Faellt die Wahl doch einmal falsch,
/// faellt sie auf die sichere Seite: eine Abweichung, die faelschlich als
/// Ziehbewegung gilt, wird uebernommen, und das ist das Verhalten von vor dem
/// 260812.
const ZIEHSPIELRAUM: f64 = 0.25;

/// Ob die gemessene Zeile etwas traegt, was die Regel nicht selbst ausgelegt
/// hat.
///
/// **Die eine Frage vor jeder Rueckrechnung vom Schirm in einen Wunsch**, und
/// zwei Stellen stellen sie: [`Fenstermodell::breiten_uebernehmen`], das den
/// Schirm in das Modell traegt, und [`wuensche_nachfuehren`], das ihn beim
/// naechsten Auslegen wieder einspeist.
///
/// Der Grund, aus dem es sie braucht: die Abbildung vom gespeicherten Wunsch
/// auf den Bildschirmpunkt ist **nur so lange umkehrbar, wie kein sichtbarer
/// Bereich an seinem Mindestmass haengt** (siehe [`Fenstermodell::massstab`]).
/// Wo sie es nicht ist, traegt die gemessene Zahl nicht mehr den Wunsch,
/// sondern die Deckelung, und wer sie zurueckrechnet, ersetzt den Wunsch des
/// Nutzers durch das Verhaeltnis der Mindestbreiten. Ein Hin und Her am
/// Fensterrand genuegte dafuer.
///
/// **Gefragt wird deshalb nicht, ob die Abbildung umkehrbar ist, sondern ob
/// ueberhaupt etwas zurueckzulesen ist.** Das ist die engere und die
/// entscheidbare Frage: was die Regel selbst ausgelegt hat, kann keinen neuen
/// Wunsch tragen, gleich ob gedeckelt oder nicht. Nur eine Trennlinie, die
/// jemand mit der Maus verschoben hat, steht anders im Rahmen, als die Regel
/// sie hingeschrieben hat — und genau dafuer gibt es die Rueckrechnung.
#[must_use]
fn traegt_eine_ziehbewegung(
    mass: Zeilenmass,
    breiten: &Breiten,
    sichtbar: &Sichtbarkeit,
    gemessen: &[f64; 5],
) -> bool {
    let ausgelegt = bereichsbreiten(mass, breiten, sichtbar);
    Bereich::ALLE.into_iter().any(|bereich| {
        (ausgelegt[bereich.index()] - gemessen[bereich.index()]).abs() > ZIEHSPIELRAUM
    })
}

/// Die Wuensche, aus denen die Fensterzeile nach einer Groessenaenderung
/// auszulegen ist.
///
/// **Die Entscheidung hinter `splitView:resizeSubviewsWithOldSize:`**, und sie
/// steht hier statt in [`crate::appkit::aufteilung`], weil sie zur Breitenregel
/// gehoert und ohne Fenster pruefbar sein soll.
///
/// `gehalten` sind die Wuensche, aus denen die Zeile zuletzt ausgelegt wurde,
/// `gemessen` die Breiten, die gerade in den Rahmen stehen, und `mass` die
/// Geometrie der Zeile **vor** der Aenderung, also die, unter der die
/// gemessenen Breiten entstanden sind. Hat jemand eine Trennlinie mit der Maus
/// verschoben, gelten die gemessenen Breiten als der neue Wunsch; sonst bleiben
/// die gehaltenen stehen.
///
/// **Ohne diese Unterscheidung ist das Wiedereinspeisen nicht neutral.** Bis
/// zum 260812 speiste die Aufteilung bei jedem Bild die gemessenen Breiten
/// wieder als Wuensche ein. Das ist unschaedlich, solange nichts gedeckelt ist,
/// weil die Abbildung dann ein einheitlicher Faktor ist; sobald gedeckelt wird,
/// ist es das nicht mehr. Ein Zug am Fensterrand von 1280 auf 780 Punkte und
/// zurueck ersetzte damit die Aufteilung des Nutzers durch das Verhaeltnis der
/// Mindestbreiten — die Dateifenster verloren 8,1 Prozent, der Editor gewann
/// 11,9 —, und kein Tastenbefehl war dafuer noetig.
#[must_use]
pub fn wuensche_nachfuehren(
    gehalten: Breiten,
    gemessen: Breiten,
    mass: Zeilenmass,
    sichtbar: &Sichtbarkeit,
) -> Breiten {
    let punkte = Bereich::ALLE.map(|bereich| breite_in(&gemessen, bereich).unwrap_or(0.0));
    if traegt_eine_ziehbewegung(mass, &gehalten, sichtbar, &punkte) {
        gemessen
    } else {
        gehalten
    }
}

#[cfg(test)]
mod tests {
    use krk_core::verzeichnis::{Richtung, Schluessel, Sortierung};

    use super::*;

    fn modell() -> Fenstermodell {
        Fenstermodell::aus_sitzung(&Sitzung::default())
    }

    /// Ein Zeilenmass ohne Trennlinien.
    ///
    /// Die verfuegbare Breite ist dann die Fensterbreite, und die Zahlen dieser
    /// Proben sind ohne Umrechnung zu lesen. Dass die Trennlinien abgezogen
    /// werden, prueft `das_zeilenmass_zieht_je_trennlinie_ab`, und dass die
    /// Summe auch mit ihnen aufgeht,
    /// `die_summe_ist_immer_die_verfuegbare_breite`.
    fn mass(gesamt: f64) -> Zeilenmass {
        Zeilenmass {
            gesamt,
            trennerbreite: 0.0,
        }
    }

    /// Ein Zeilenmass, in das jede Menge sichtbarer Bereiche hineinpasst.
    ///
    /// Proben, die nur eine Sichtbarkeit **herstellen** wollen, nehmen es: die
    /// Abweisung an den Mindestbreiten greift dort nie, und die Probe misst,
    /// was ihr Name nennt. Die Abweisung selbst misst
    /// `am_engen_fenster_wird_das_einschalten_abgewiesen`.
    fn weit() -> Zeilenmass {
        mass(4000.0)
    }

    /// Ein Zeilenmass, in dem ein gespeicherter Punkt ein Punkt auf dem Schirm
    /// ist.
    ///
    /// Die verfuegbare Breite trifft dann die Summe der gespeicherten Breiten
    /// der sichtbaren Bereiche, und `Fenstermodell::massstab` liefert 1. Proben
    /// ueber `breite_aendern`, die in gespeicherten Punkten rechnen wollen,
    /// nehmen es; dass der Schritt auch bei jeder anderen Fensterbreite
    /// ankommt, misst `der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt`.
    fn passend(modell: &Fenstermodell) -> Zeilenmass {
        mass(
            modell
                .sichtbare()
                .iter()
                .map(|bereich| modell.breite_oder_anfang(*bereich))
                .sum(),
        )
    }

    /// Schaltet einen Bereich um und besteht darauf, dass es geschieht.
    ///
    /// Die Proben **stellen** damit eine Sichtbarkeit her, statt sie zu
    /// erbitten. Seit `umschalten` zwei Abweisungen traegt, liesse eine stumme
    /// die Probe auf einer anderen Lage messen als der, die ihr Name nennt;
    /// genau dagegen steht das `#[must_use]` an der Funktion.
    #[track_caller]
    fn schalten(modell: &mut Fenstermodell, bereich: Bereich) {
        assert!(
            modell.umschalten(bereich, weit()),
            "{bereich:?} liess sich nicht umschalten"
        );
    }

    /// Die ausgelegte Zeile so, wie die Aufteilung sie misst.
    ///
    /// Dieselbe Umsetzung wie `appkit::aufteilung::gemessene_breiten`: eine
    /// Breite von 0 heisst "steht nicht im Fenster" und liefert `None`. Proben
    /// ueber `wuensche_nachfuehren` brauchen sie, weil jene Funktion in der
    /// Anwendung eine gemessene Zeile bekommt und keine gerechnete.
    fn gemessen(breiten: [f64; 5]) -> Breiten {
        let feld = |bereich: Bereich| {
            let breite = breiten[bereich.index()];
            (breite > 0.0).then_some(breite)
        };
        Breiten {
            lesezeichen: feld(Bereich::Lesezeichen),
            links: feld(Bereich::Links),
            rechts: feld(Bereich::Rechts),
            vorschau: feld(Bereich::Vorschau),
            editor: feld(Bereich::Editor),
        }
    }

    /// Vergleicht fuenf Breiten mit den erwarteten, auf einen tausendstel
    /// Punkt genau.
    ///
    /// Die Anteilsregel liefert Bruchzahlen; ein `assert_eq!` auf die volle
    /// Genauigkeit haenge an der letzten Stelle der Gleitkommarechnung.
    #[track_caller]
    fn breiten_gleich(ist: [f64; 5], soll: [f64; 5]) {
        for bereich in Bereich::ALLE {
            let a = ist[bereich.index()];
            let b = soll[bereich.index()];
            assert!(
                (a - b).abs() < 0.001,
                "{bereich:?}: {a} statt {b} (ist {ist:?}, soll {soll:?})"
            );
        }
    }

    /// Ein paar Lagen der Sichtbarkeit, ueber die eine Probe laufen kann.
    fn bereichslagen() -> Vec<Fenstermodell> {
        let mut mit_editor = modell();
        schalten(&mut mit_editor, Bereich::Vorschau);
        schalten(&mut mit_editor, Bereich::Editor);

        let mut nur_dateifenster = modell();
        schalten(&mut nur_dateifenster, Bereich::Lesezeichen);
        schalten(&mut nur_dateifenster, Bereich::Vorschau);

        let mut ein_dateifenster = modell();
        schalten(&mut ein_dateifenster, Bereich::Rechts);
        schalten(&mut ein_dateifenster, Bereich::Lesezeichen);
        schalten(&mut ein_dateifenster, Bereich::Vorschau);

        vec![modell(), mit_editor, nur_dateifenster, ein_dateifenster]
    }

    /// Der Auslieferungszustand zeigt die vier Bereiche der Runde 1 und
    /// blendet den Editor aus.
    ///
    /// Der Editor ist der einzige Bereich, der beim allerersten Start nicht
    /// steht: er haelt keine Datei, und ein sichtbarer leerer Editor naehme den
    /// Dateifenstern Platz fuer nichts.
    #[test]
    fn der_auslieferungszustand_zeigt_alle_bereiche_ausser_dem_editor() {
        let modell = modell();
        for bereich in Bereich::ALLE {
            if bereich == Bereich::Editor {
                assert!(!modell.sichtbar(bereich), "der Editor steht schon");
                continue;
            }
            assert!(modell.sichtbar(bereich), "{bereich:?} ist ausgeblendet");
        }
        assert_eq!(modell.aktiv(), Fensterseite::Links);
    }

    /// Eine von Hand geaenderte `session.toml` macht kein ausgeblendetes
    /// Dateifenster zum aktiven.
    ///
    /// Gefunden am 260805 neben dem Fokusdefekt: `umschalten` haelt die
    /// Zusicherung, `aus_sitzung` hielt sie nicht.
    #[test]
    fn ein_ausgeblendetes_dateifenster_kommt_nicht_als_aktives_aus_der_sitzung() {
        let mut sitzung = Sitzung {
            aktiv: Fensterseite::Rechts,
            ..Sitzung::default()
        };
        sitzung.sichtbar.zweites_dateifenster = false;
        let modell = Fenstermodell::aus_sitzung(&sitzung);
        assert_eq!(
            modell.aktiv(),
            Fensterseite::Links,
            "das aktive Dateifenster waere unsichtbar"
        );

        // Dieselbe Zusicherung in der anderen Richtung, seit auch das linke
        // Dateifenster sich ausblenden laesst.
        let mut sitzung = Sitzung::default();
        sitzung.sichtbar.erstes_dateifenster = false;
        assert_eq!(
            Fenstermodell::aus_sitzung(&sitzung).aktiv(),
            Fensterseite::Rechts,
            "das aktive Dateifenster waere unsichtbar"
        );

        // Die Gegenprobe: ist es sichtbar, bleibt es das aktive.
        let sitzung = Sitzung {
            aktiv: Fensterseite::Rechts,
            ..Sitzung::default()
        };
        assert_eq!(
            Fenstermodell::aus_sitzung(&sitzung).aktiv(),
            Fensterseite::Rechts
        );
    }

    /// Eine von Hand geschriebene `session.toml`, die beide Dateifenster
    /// ausblendet, geht mit einem sichtbaren linken auf.
    ///
    /// Die dritte hergestellte Zusicherung aus [`Fenstermodell::aus_sitzung`].
    /// Sie ist der eine Weg zur Regel "eines bleibt", der nicht durch
    /// [`Fenstermodell::umschalten`] fuehrt: `serde` liest jede Feldkombination
    /// anstandslos ein, und ohne diese Zeile fuende der Nutzer ein Fenster ohne
    /// Dateiliste vor.
    ///
    /// **Die Probe laeuft ueber beide Werte von `aktiv`**, weil daran die
    /// Reihenfolge der Zusicherungen haengt: erst stehen die Dateifenster fest,
    /// dann das aktive. Umgekehrt gerechnet bliebe `aktiv = "rechts"` auf einem
    /// Dateifenster stehen, das niemand sieht.
    #[test]
    fn eine_sitzung_ohne_sichtbares_dateifenster_holt_das_linke_hervor() {
        for aktiv in Fensterseite::ALLE {
            let mut sitzung = Sitzung {
                aktiv,
                ..Sitzung::default()
            };
            sitzung.sichtbar.erstes_dateifenster = false;
            sitzung.sichtbar.zweites_dateifenster = false;

            let modell = Fenstermodell::aus_sitzung(&sitzung);

            assert!(
                modell.sichtbar(Bereich::Links),
                "bei aktivem {aktiv:?} steht kein Dateifenster"
            );
            assert!(
                !modell.sichtbar(Bereich::Rechts),
                "hervorgeholt wird eines und nicht beide"
            );
            assert_eq!(
                modell.aktiv(),
                Fensterseite::Links,
                "das aktive Dateifenster ist das hervorgeholte"
            );
        }
    }

    /// Das letzte sichtbare Dateifenster bleibt stehen, gleich welches der
    /// beiden es ist.
    ///
    /// **Beide Richtungen, weil die Regel seit der Bereichsleisten-Runde
    /// "eines bleibt" heisst und nicht mehr "das linke ist besonders."** Bis
    /// dahin wies `umschalten` jeden Befehl auf das linke ab; heute geht es
    /// aus, solange das rechte steht, und dann ist das rechte das
    /// unantastbare.
    #[test]
    fn das_letzte_dateifenster_laesst_sich_nicht_ausblenden() {
        for zuerst in [Bereich::Links, Bereich::Rechts] {
            let zuletzt = if zuerst == Bereich::Links {
                Bereich::Rechts
            } else {
                Bereich::Links
            };
            let mut modell = modell();
            schalten(&mut modell, zuerst);
            assert!(
                !modell.sichtbar(zuerst),
                "{zuerst:?} laesst sich ausblenden, solange {zuletzt:?} steht"
            );
            assert!(
                !modell.umschalten(zuletzt, weit()),
                "C7 verwirft den Befehl auf das letzte sichtbare Dateifenster"
            );
            assert!(modell.sichtbar(zuletzt), "{zuletzt:?} steht weiter");
        }
    }

    /// Keine Folge von Umschaltbefehlen blendet beide Dateifenster aus.
    ///
    /// Die Gegenprobe zur Regel "eines bleibt" ueber alle Folgen bis zur Laenge
    /// drei. Sie steht an der Stelle, an der bis zur Bereichsleisten-Runde
    /// `das_letzte_dateifenster_ist_immer_schon_eingeblendet` stand: die alte
    /// Zusage war "das linke steht immer", die neue ist diese.
    #[test]
    fn keine_folge_von_befehlen_blendet_beide_dateifenster_aus() {
        let dateifenster = [Bereich::Links, Bereich::Rechts];
        for erster in dateifenster {
            for zweiter in dateifenster {
                for dritter in dateifenster {
                    let mut modell = modell();
                    for bereich in [erster, zweiter, dritter] {
                        let _ = modell.umschalten(bereich, weit());
                        assert!(
                            modell.sichtbar(Bereich::Links) || modell.sichtbar(Bereich::Rechts),
                            "nach {erster:?}, {zweiter:?}, {dritter:?} steht kein Dateifenster mehr"
                        );
                        assert!(
                            modell.sichtbar(Bereich::von_seite(modell.aktiv())),
                            "das aktive Dateifenster ist ausgeblendet"
                        );
                    }
                }
            }
        }
    }

    /// Jedes der beiden Dateifenster geht aus und wieder ein, solange das
    /// andere steht.
    #[test]
    fn jedes_dateifenster_geht_aus_und_wieder_ein() {
        for bereich in [Bereich::Links, Bereich::Rechts] {
            let mut modell = modell();
            assert!(modell.umschalten(bereich, weit()));
            assert!(!modell.sichtbar(bereich));
            assert!(modell.umschalten(bereich, weit()));
            assert!(modell.sichtbar(bereich));
        }
    }

    /// Bei 780 Punkten Fensterbreite laesst sich der Editor nicht
    /// einschalten, bei 1280 schon.
    ///
    /// Die zweite Abweisung an ihrem Fall: Lesezeichenleiste, beide
    /// Dateifenster und der Editor verlangen zusammen 920 Punkte
    /// (120 + 240 + 240 + 320), und mehr als 780 gibt die Mindestgroesse des
    /// Fensters nicht her. **Die Zahl der sichtbaren Bereiche bleibt dabei
    /// gleich**, weil der Editor die Vorschau verdraengt; es entscheidet
    /// allein, dass er 320 verlangt und sie 160. Genau deshalb kann der
    /// Aufrufer die Frage nicht stellen.
    #[test]
    fn am_engen_fenster_wird_das_einschalten_abgewiesen() {
        let mut eng = modell();
        assert!(
            !eng.umschalten(Bereich::Editor, mass(780.0)),
            "920 Punkte Mindestbreite passen nicht in 780"
        );
        assert!(
            !eng.sichtbar(Bereich::Editor),
            "der abgewiesene Befehl hat den Editor trotzdem eingeschaltet"
        );
        assert!(
            eng.sichtbar(Bereich::Vorschau),
            "und die Vorschau steht unangetastet weiter"
        );

        let mut weiter = modell();
        assert!(weiter.umschalten(Bereich::Editor, mass(1280.0)));
        assert!(weiter.sichtbar(Bereich::Editor));
        assert!(!weiter.sichtbar(Bereich::Vorschau));
    }

    /// Ein Ausschaltbefehl scheitert nie an den Mindestbreiten.
    ///
    /// Er kann ihre Summe nicht vergroessern, und ein Fenster, das schon zu
    /// schmal steht, liesse sich sonst nicht mehr aufraeumen: jeder Befehl, der
    /// Platz schaffte, waere abgewiesen. Gemessen bei 200 Punkten, also weit
    /// unter jeder einzelnen Mindestbreite.
    #[test]
    fn ein_ausschaltbefehl_scheitert_nie_an_den_mindestbreiten() {
        let winzig = mass(200.0);
        for bereich in [Bereich::Vorschau, Bereich::Lesezeichen, Bereich::Rechts] {
            let mut modell = modell();
            assert!(modell.sichtbar(bereich), "die Probe beginnt sichtbar");
            assert!(
                modell.umschalten(bereich, winzig),
                "{bereich:?} liess sich bei 200 Punkten nicht ausblenden"
            );
            assert!(!modell.sichtbar(bereich));

            // Die Gegenprobe: hinein kommt er bei dieser Breite nicht wieder.
            assert!(
                !modell.umschalten(bereich, winzig),
                "{bereich:?} kam bei 200 Punkten zurueck"
            );
            assert!(!modell.sichtbar(bereich));
        }
    }

    /// Das Einblenden holt hervor, was ausgeblendet ist, und laesst alles
    /// andere stehen.
    ///
    /// Die Zusage hinter dem Nutzerentscheid vom 260807: `shift+cmd+l` auf
    /// eine ausgeblendete Leiste blendet sie ein, `shift+cmd+l` auf eine
    /// sichtbare laesst sie sichtbar. Ohne die zweite Haelfte waere aus dem
    /// Fokusbefehl ein zweites Umschalten geworden, und zwei Befehle sagten
    /// widersprechendes ueber denselben Bereich.
    #[test]
    fn das_einblenden_holt_hervor_und_blendet_nie_aus() {
        for bereich in [
            Bereich::Lesezeichen,
            Bereich::Vorschau,
            Bereich::Links,
            Bereich::Rechts,
        ] {
            let mut modell = modell();
            assert!(
                modell.sichtbar(bereich),
                "die Probe beginnt mit sichtbarem Bereich"
            );
            assert!(
                !modell.einblenden(bereich, weit()),
                "ein sichtbarer Bereich aendert sich nicht"
            );
            assert!(modell.sichtbar(bereich), "und bleibt sichtbar");

            schalten(&mut modell, bereich);
            assert!(!modell.sichtbar(bereich));
            assert!(
                modell.einblenden(bereich, weit()),
                "der ausgeblendete kommt hervor"
            );
            assert!(modell.sichtbar(bereich));
        }
    }

    /// Wird das aktive Dateifenster ausgeblendet, wandert die Aktivitaet auf
    /// das andere — in beide Richtungen.
    ///
    /// Bis zur Bereichsleisten-Runde stand hier nur die eine Richtung, weil
    /// das linke Dateifenster sich nicht ausblenden liess.
    #[test]
    fn das_ausblenden_gibt_die_aktivitaet_an_das_andere_dateifenster() {
        for seite in Fensterseite::ALLE {
            let mut modell = modell();
            if modell.aktiv() != seite {
                assert!(modell.fenster_wechseln());
            }
            assert_eq!(modell.aktiv(), seite);

            schalten(&mut modell, Bereich::von_seite(seite));
            assert_eq!(
                modell.aktiv(),
                seite.andere(),
                "ein ausgeblendetes Dateifenster kann nicht das aktive sein"
            );
            assert!(
                !modell.fenster_wechseln(),
                "und der Wechsel dorthin geschieht nicht"
            );
        }
    }

    /// Eine gespeicherte Breite ueberlebt das Ausblenden, und das
    /// Wiedereinblenden stellt denselben Anteil wieder her.
    ///
    /// Die Zusage aus C7, in der Waehrung der Anteilsregel gelesen: dieselbe
    /// Punktzahl kommt zurueck, solange sich an der uebrigen Aufteilung nichts
    /// geaendert hat. Die gespeicherte 200 selbst steht dabei nie auf dem
    /// Schirm; sie ist ein Wunsch unter vieren.
    #[test]
    fn eine_ausgeblendete_breite_bleibt_erhalten() {
        let mut modell = modell();
        modell.breite_setzen(Bereich::Lesezeichen, 200.0);
        let vorher = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        assert!(vorher[Bereich::Lesezeichen.index()] > 0.0, "{vorher:?}");

        schalten(&mut modell, Bereich::Lesezeichen);
        let breiten = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Lesezeichen.index()], 0.0);
        assert_eq!(
            modell.breiten().lesezeichen,
            Some(200.0),
            "die gespeicherte Breite ueberlebt das Ausblenden"
        );

        schalten(&mut modell, Bereich::Lesezeichen);
        let nachher = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(nachher, vorher);
    }

    /// Der Platz eines ausgeblendeten Bereichs geht an alle uebrigen
    /// sichtbaren, im Verhaeltnis ihrer Wuensche.
    ///
    /// Bis zur Bereichsleisten-Runde bekamen ihn allein die beiden
    /// Dateifenster, weil die festen Bereiche ihre Punktzahl behielten. Unter
    /// der Anteilsregel waechst jeder sichtbare Bereich, und die Dateifenster
    /// sind darunter.
    #[test]
    fn der_frei_gewordene_platz_geht_an_die_uebrigen_bereiche() {
        let modell = modell();
        let voll = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        let summe_voll: f64 = voll.iter().sum();
        assert!((summe_voll - 1400.0).abs() < 0.001, "{voll:?}");

        let mut ohne_vorschau = modell;
        schalten(&mut ohne_vorschau, Bereich::Vorschau);
        let jetzt = bereichsbreiten(
            mass(1400.0),
            &ohne_vorschau.breiten(),
            &ohne_vorschau.sichtbarkeit(),
        );
        assert_eq!(jetzt[Bereich::Vorschau.index()], 0.0);
        for bereich in [Bereich::Lesezeichen, Bereich::Links, Bereich::Rechts] {
            assert!(
                jetzt[bereich.index()] > voll[bereich.index()],
                "{bereich:?} hat nichts vom frei gewordenen Platz bekommen: voll {voll:?}, jetzt {jetzt:?}"
            );
        }
        let summe: f64 = jetzt.iter().sum();
        assert!((summe - 1400.0).abs() < 0.001, "{jetzt:?}");
    }

    #[test]
    fn ein_einziges_dateifenster_nimmt_die_ganze_breite() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Rechts);
        schalten(&mut modell, Bereich::Lesezeichen);
        schalten(&mut modell, Bereich::Vorschau);
        let breiten = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten, [0.0, 1400.0, 0.0, 0.0, 0.0]);
    }

    /// Der eingeblendete Editor bekommt seinen Anteil, und die uebrigen
    /// bekommen ihren.
    ///
    /// Der Fall aus C1 der Editor-Runde: Editor sichtbar, Vorschau
    /// ausgeblendet. Die vier sichtbaren Bereiche wuenschen zusammen 1480
    /// Punkte (180, 420, 420, 460); bei 1280 Punkten Fensterbreite bekommt
    /// jeder 32/37 seines Wunsches, und keiner faellt dabei unter sein
    /// Mindestmass. Bis zur Bereichsleisten-Runde bekam der Editor seine
    /// vollen 460 und die Dateifenster teilten sich, was uebrig blieb.
    #[test]
    fn der_eingeblendete_editor_bekommt_seinen_anteil() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Vorschau);
        assert!(modell.umschalten(Bereich::Editor, weit()));
        assert!(modell.sichtbar(Bereich::Editor));

        let breiten = bereichsbreiten(mass(1280.0), &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(breiten, [155.676, 363.243, 363.243, 0.0, 397.838]);
        for bereich in Bereich::ALLE {
            if !modell.sichtbar(bereich) {
                continue;
            }
            assert!(
                breiten[bereich.index()] >= bereich.mindestbreite(),
                "{bereich:?} faellt unter sein Mindestmass: {breiten:?}"
            );
        }
        let summe: f64 = breiten.iter().sum();
        assert!((summe - 1280.0).abs() < 0.001, "{breiten:?}");
    }

    /// Am engen Fenster gewinnt das Mindestmass gegen den Anteil, und die
    /// uebrigen teilen den Rest weiter im Verhaeltnis ihrer Wuensche.
    ///
    /// Die Wasserstandsrechnung, an dem Bereich gemessen, den sie zuerst
    /// trifft: der Editor hat unter den fuenf das groesste Verhaeltnis von
    /// Mindestmass zu Wunsch (320 zu 460) und faellt deshalb als erster auf
    /// sein Mindestmass. Die restlichen 700 Punkte teilen Leiste und
    /// Dateifenster im Verhaeltnis 180 zu 420 zu 420.
    ///
    /// **Wer nachgibt, entscheidet nicht mehr die Reihenfolge von
    /// `Bereich::ALLE`.** Bis zur Bereichsleisten-Runde wurden die festen
    /// Bereiche in dieser Folge bedient, und der Editor stand hinten.
    #[test]
    fn am_engen_fenster_gewinnt_das_mindestmass_gegen_den_anteil() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Vorschau);
        schalten(&mut modell, Bereich::Editor);

        let breiten = bereichsbreiten(mass(1020.0), &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(breiten, [123.529, 288.235, 288.235, 0.0, 320.0]);
        assert_eq!(
            breiten[Bereich::Editor.index()],
            Bereich::Editor.mindestbreite(),
            "der Editor steht auf seinem Mindestmass: {breiten:?}"
        );
        for bereich in [Bereich::Lesezeichen, Bereich::Links, Bereich::Rechts] {
            assert!(
                breiten[bereich.index()] > bereich.mindestbreite(),
                "{bereich:?} sollte ueber seinem Mindestmass stehen: {breiten:?}"
            );
        }
        let summe: f64 = breiten.iter().sum();
        assert!((summe - 1020.0).abs() < 0.001, "{breiten:?}");
    }

    /// Der ausgeblendete Editor bekommt 0 und behaelt seine gespeicherte
    /// Breite.
    ///
    /// Dieselbe Zusage aus C7, die schon fuer Leiste und Vorschau gilt: das
    /// Wiedereinblenden stellt den vorherigen Anteil her. Die gespeicherten 500
    /// stehen dabei nie auf dem Schirm; sie sind der Wunsch, mit dem der Editor
    /// in die Verteilung geht.
    #[test]
    fn der_ausgeblendete_editor_behaelt_seine_gespeicherte_breite() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Vorschau);
        schalten(&mut modell, Bereich::Editor);
        modell.breite_setzen(Bereich::Editor, 500.0);
        let vorher = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());

        schalten(&mut modell, Bereich::Editor);
        assert!(!modell.sichtbar(Bereich::Editor));
        let breiten = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Editor.index()], 0.0, "{breiten:?}");
        assert_eq!(modell.breiten().editor, Some(500.0));

        schalten(&mut modell, Bereich::Editor);
        let nachher = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(nachher, vorher);
    }

    /// Die Lesezeichenleiste teilt sich die Zeile mit dem Editor, statt ihm
    /// vorzugehen.
    ///
    /// **Die Festlegung des Nutzers vom 260808 gilt nicht mehr.** Sie fiel
    /// allein daraus an, dass `Bereich::ALLE` die Leiste vor den Editor stellte
    /// und die festen Bereiche in dieser Folge bedient wurden. Unter der
    /// Anteilsregel gibt es keine Folge mehr: die offene Leiste kostet jeden
    /// anderen sichtbaren Bereich seinen Anteil daran, den Editor
    /// eingeschlossen.
    #[test]
    fn die_leiste_schrumpft_mit_dem_editor() {
        let mut mit_leiste = modell();
        schalten(&mut mit_leiste, Bereich::Vorschau);
        schalten(&mut mit_leiste, Bereich::Editor);
        let offen = bereichsbreiten(
            mass(1280.0),
            &mit_leiste.breiten(),
            &mit_leiste.sichtbarkeit(),
        );

        let mut ohne_leiste = modell();
        schalten(&mut ohne_leiste, Bereich::Vorschau);
        schalten(&mut ohne_leiste, Bereich::Editor);
        schalten(&mut ohne_leiste, Bereich::Lesezeichen);
        let zu = bereichsbreiten(
            mass(1280.0),
            &ohne_leiste.breiten(),
            &ohne_leiste.sichtbarkeit(),
        );

        // Die Zahlen ausgeschrieben, damit die Regel nachlesbar ist und nicht
        // nur als Ungleichung dasteht. Offen: 1280 Punkte auf die Wuensche
        // 180 + 420 + 420 + 460 = 1480. Zu: 1280 Punkte auf 420 + 420 + 460 =
        // 1300.
        breiten_gleich(offen, [155.676, 363.243, 363.243, 0.0, 397.838]);
        breiten_gleich(zu, [0.0, 413.538, 413.538, 0.0, 452.923]);

        assert!(
            zu[Bereich::Editor.index()] > offen[Bereich::Editor.index()],
            "der Editor hat vom Platz der Leiste nichts bekommen: offen {offen:?}, zu {zu:?}"
        );
        assert!(
            zu[Bereich::Links.index()] > offen[Bereich::Links.index()],
            "die Dateifenster haben den Platz der Leiste nicht bekommen"
        );
        assert!(zu[Bereich::Rechts.index()] > offen[Bereich::Rechts.index()]);
    }

    /// Welche Bereiche etwas bekommen, sagt die Sichtbarkeit und sonst
    /// niemand.
    ///
    /// Die Probe zu Befund 6 des Editor-Plans: bis dahin fuehrte
    /// `bereichsbreiten` die festen Bereiche als Literalliste ein zweites Mal,
    /// und ein weiterer fester Bereich haette dort still gefehlt und dauerhaft
    /// die Breite 0 bekommen. Seit der Anteilsregel gibt es keine festen
    /// Bereiche mehr, an denen eine solche Liste ansetzen koennte: jeder
    /// sichtbare geht mit seinem Wunsch in dieselbe Rechnung, jeder
    /// ausgeblendete bekommt 0.
    #[test]
    fn jeder_sichtbare_bereich_bekommt_seinen_anteil_ohne_zweite_aufzaehlung() {
        let modell = modell();
        let breiten = bereichsbreiten(mass(1600.0), &modell.breiten(), &modell.sichtbarkeit());
        for bereich in Bereich::ALLE {
            if modell.sichtbar(bereich) {
                assert!(
                    breiten[bereich.index()] >= bereich.mindestbreite(),
                    "{bereich:?} ist sichtbar, bekommt aber nur {}: {breiten:?}",
                    breiten[bereich.index()]
                );
            } else {
                assert_eq!(
                    breiten[bereich.index()],
                    0.0,
                    "{bereich:?} ist ausgeblendet: {breiten:?}"
                );
            }
        }
        // 1600 Punkte auf die Wuensche 180 + 420 + 420 + 260 = 1280, also das
        // 1,25-Fache jedes Wunsches.
        breiten_gleich(breiten, [225.0, 525.0, 525.0, 325.0, 0.0]);
    }

    /// Die Zuordnung von Bereich und Fensterseite laeuft in beide Richtungen,
    /// und es gibt genau so viele Dateifenster wie Seiten.
    ///
    /// Die Probe zum Befund vom 260808: `breite_aendern` fuehrte den Partner
    /// ueber einen `match` mit `_ => Bereich::Links` und hat den fuenften
    /// Bereich stumm aufgenommen. Seit dem 260809 gibt es die Zuordnung nur in
    /// `Bereich::seite`, und diese Probe haelt fest, dass sie mit
    /// `Bereich::von_seite` zusammenpasst.
    #[test]
    fn die_zuordnung_von_bereich_und_fensterseite_laeuft_in_beide_richtungen() {
        for bereich in Bereich::ALLE {
            if let Some(seite) = bereich.seite() {
                assert_eq!(Bereich::von_seite(seite), bereich, "{bereich:?}");
            }
        }
        for seite in Fensterseite::ALLE {
            assert_eq!(Bereich::von_seite(seite).seite(), Some(seite));
        }
        assert_eq!(
            Bereich::ALLE
                .iter()
                .filter(|bereich| bereich.seite().is_some())
                .count(),
            Fensterseite::ALLE.len(),
            "es gibt genau so viele Dateifenster wie Fensterseiten"
        );
    }

    /// Ein Bereich ohne Fensterseite waechst unmittelbar und zieht kein
    /// Dateifenster mit.
    ///
    /// Der Editor ist der Fall, den der Auffangzweig stumm aufgenommen hatte.
    ///
    /// **Die Unterscheidung heisst [`Bereich::seite`]** und nicht mehr
    /// `ist_beweglich`: die Zweiteilung in feste und bewegliche Bereiche ist
    /// mit Schritt 1 der Bereichsleisten-Runde weggefallen, der Probenname
    /// trug sie bis zum 260812 weiter
    /// (`issues/260812-0539_*_eine-probe-heisst-noch-nach-der-abgeschafften-zweiteilung-in-feste-und-bewegliche-bereiche.md`).
    #[test]
    fn ein_bereich_ohne_fensterseite_aendert_nur_seine_eigene_breite() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Vorschau);
        schalten(&mut modell, Bereich::Editor);
        let links_vorher = modell.breite_oder_anfang(Bereich::Links);
        let rechts_vorher = modell.breite_oder_anfang(Bereich::Rechts);

        modell.breite_aendern(Bereich::Editor, BREITENSCHRITT, passend(&modell));
        assert_eq!(
            modell.breiten().editor,
            Some(Bereich::Editor.anfangsbreite() + BREITENSCHRITT)
        );
        assert_eq!(modell.breite_oder_anfang(Bereich::Links), links_vorher);
        assert_eq!(modell.breite_oder_anfang(Bereich::Rechts), rechts_vorher);

        // Und er faellt nicht unter sein Mindestmass. Das Mass wird neu
        // genommen: der Schritt darueber hat die gespeicherte Summe um 40
        // Punkte erhoeht, und ein gespeicherter Punkt soll ein Punkt auf dem
        // Schirm bleiben.
        modell.breite_aendern(Bereich::Editor, -10_000.0, passend(&modell));
        assert_eq!(
            modell.breiten().editor,
            Some(Bereich::Editor.mindestbreite())
        );
    }

    /// Passen die Mindestbreiten der beiden Dateifenster nicht mehr
    /// nebeneinander, bleibt der Breitenbefehl ohne Wirkung — in beiden
    /// Richtungen.
    ///
    /// **Die Probe zum Befund 1 der Durchsicht vom 260812-0539.** Bis dahin
    /// taten `opt+cmd+links` und `opt+cmd+rechts` dort dasselbe, und beide das
    /// Gegenteil ihres Namens: die Deckelungskette setzte voraus, dass ihre
    /// untere Schranke nicht ueber der oberen liegt, und wo sie es tat, gewann
    /// `.max()` und das Vorzeichen des Betrags fiel weg.
    ///
    /// Die Zahlen sind gerechnet. Vier sichtbare Bereiche der Runde 1
    /// wuenschen zusammen 1280 Punkte und brauchen mindestens 760; bei 600
    /// Punkten greift der zweite Zweig. Der Massstab ist 1280/600 = 2,1333, das
    /// skalierte Mindestmass eines Dateifensters also 240 × 2,1333 = 512, und
    /// beide Dateifenster stehen mit ihren 420 gespeicherten Punkten darunter.
    /// Vor der Behebung kamen beide Richtungen auf 512 zu 328, also eine
    /// Verschiebung um 92 gespeicherte Punkte, wo der Schritt 40 × 2,1333 =
    /// 85,33 verlangt und die eine Richtung das Gegenteil.
    #[test]
    fn unter_der_mindestsumme_bleibt_der_breitenbefehl_ohne_wirkung() {
        let zeile = mass(600.0);
        let vergleich = modell();
        assert!(
            bereichsbreiten(zeile, &vergleich.breiten(), &vergleich.sichtbarkeit())
                [Bereich::Links.index()]
                < Bereich::Links.mindestbreite(),
            "die Lage der Probe ist nicht der zweite Zweig"
        );

        for betrag in [BREITENSCHRITT, -BREITENSCHRITT] {
            let mut modell = modell();
            modell.breite_aendern(Bereich::Links, betrag, zeile);
            assert_eq!(
                modell.breiten(),
                vergleich.breiten(),
                "der Befehl mit {betrag} hat gespeicherte Breiten gesetzt, wo keine Lage der \
                 Trennlinie beide Mindestmasse haelt"
            );
        }
    }

    /// Ein gedeckelter dritter Bereich sperrt den Breitenbefehl nicht.
    ///
    /// Die Gegenprobe zu `unter_der_mindestsumme_bleibt_der_breitenbefehl_ohne_wirkung`:
    /// die Feststellung gilt dem **Paar**, dessen Trennlinie der Befehl
    /// verschiebt, und nicht der ganzen Zeile. Haengt allein die Leiste an
    /// ihrem Mindestmass, wirkt der Befehl weiter.
    ///
    /// Die Zahlen sind gerechnet. Bei 800 Punkten steht auf dem Schirm
    /// [120; 259,64; 259,64; 160,73]: die Leiste ist gedeckelt, die beiden
    /// Dateifenster nicht. Der Massstab ist 1280/800 = 1,6, das skalierte
    /// Mindestmass eines Dateifensters 384, und der Schritt von 40 × 1,6 = 64
    /// gespeicherten Punkten wird auf 36 gekuerzt, weil das rechte
    /// Dateifenster nicht mehr abgeben kann: 456 zu 384. Auf dem Schirm sind
    /// das 280 zu 240. **Gekuerzt ist nicht dasselbe wie umgekehrt** — die
    /// Ungenauigkeit unter einer Deckelung ist an `Fenstermodell::massstab`
    /// benannt und bleibt.
    #[test]
    fn ein_gedeckelter_dritter_bereich_sperrt_den_breitenbefehl_nicht() {
        let zeile = mass(800.0);
        let mut modell = modell();
        let vorher = bereichsbreiten(zeile, &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(vorher, [120.0, 259.636, 259.636, 160.727, 0.0]);

        modell.breite_aendern(Bereich::Links, BREITENSCHRITT, zeile);
        assert_eq!(modell.breiten().links, Some(456.0));
        assert_eq!(modell.breiten().rechts, Some(384.0));
        breiten_gleich(
            bereichsbreiten(zeile, &modell.breiten(), &modell.sichtbarkeit()),
            [120.0, 280.0, 240.0, 160.0, 0.0],
        );
    }

    /// Ein zusammengezogenes Fenster laesst die gespeicherten Breiten stehen.
    ///
    /// **Die Probe zum Befund 2 der Durchsicht vom 260812-0539, Weg ueber das
    /// Modell.** Unter der Mindestsumme traegt keine gemessene Zahl mehr einen
    /// Wunsch: die Zeile steht dann im Verhaeltnis der Mindestbreiten, gleich
    /// was der Nutzer eingestellt hat. Sie zurueckzurechnen machte dieses
    /// Verhaeltnis zu seinem neuen Wunsch.
    ///
    /// Die Zahlen sind gerechnet. Bei 600 Punkten steht auf dem Schirm das
    /// 600/760-Fache der Mindestbreiten, also [94,74; 189,47; 189,47; 126,32].
    /// Vor der Behebung wurde daraus mit dem Faktor 1280/600 die gespeicherte
    /// Aufteilung [202,11; 404,21; 404,21; 269,47] — aus 180/420/420/260, die
    /// niemand angefasst hatte.
    #[test]
    fn ein_zusammengezogenes_fenster_laesst_die_gespeicherten_breiten_stehen() {
        let zeile = mass(600.0);
        let mut modell = modell();
        let vorher = modell.breiten();
        let gemessen = bereichsbreiten(zeile, &vorher, &modell.sichtbarkeit());
        breiten_gleich(gemessen, [94.737, 189.474, 189.474, 126.316, 0.0]);

        modell.breiten_uebernehmen(gemessen, zeile);
        assert_eq!(
            modell.breiten(),
            vorher,
            "die gedeckelte Zeile ist als Wunsch in das Modell gewandert"
        );
    }

    /// Ein Hin und Her am Fensterrand stellt die Aufteilung des Nutzers wieder
    /// her.
    ///
    /// **Die Probe zum Befund 2 der Durchsicht vom 260812-0539, Weg ueber den
    /// Schirm.** Er braucht keinen Tastenbefehl: `neu_auslegen` speiste bei
    /// jedem Bild die gemessenen Breiten wieder als Wuensche ein, und das ist
    /// nur ohne Deckelung neutral. Gemessen wird hier dieselbe Folge von
    /// Aufrufen, die die Aufteilung fuehrt — nachfuehren, dann auslegen —, nur
    /// ohne Fenster.
    ///
    /// Die Zahlen sind gerechnet. Bei 1280 Punkten treffen die Wuensche die
    /// Zeile genau: 180/420/420/260. Auf 600 gezogen steht das 600/760-Fache
    /// der Mindestbreiten. Vor der Behebung kam die Zeile mit
    /// [202,11; 404,21; 404,21; 269,47] zurueck, die Dateifenster also 3,8
    /// Prozent schmaler und die Vorschau 3,6 Prozent breiter als eingestellt.
    #[test]
    fn ein_hin_und_her_am_fensterrand_stellt_die_aufteilung_wieder_her() {
        let modell = modell();
        let sichtbar = modell.sichtbarkeit();
        let weit = mass(1280.0);
        let eng = mass(600.0);

        let gehalten = modell.breiten();
        let bei_1280 = bereichsbreiten(weit, &gehalten, &sichtbar);
        breiten_gleich(bei_1280, [180.0, 420.0, 420.0, 260.0, 0.0]);

        // Der Nutzer zieht das Fenster auf 600 Punkte zusammen.
        let gehalten = wuensche_nachfuehren(gehalten, gemessen(bei_1280), weit, &sichtbar);
        let bei_600 = bereichsbreiten(eng, &gehalten, &sichtbar);
        breiten_gleich(bei_600, [94.737, 189.474, 189.474, 126.316, 0.0]);

        // Und wieder auf 1280 auf.
        let gehalten = wuensche_nachfuehren(gehalten, gemessen(bei_600), eng, &sichtbar);
        breiten_gleich(
            bereichsbreiten(weit, &gehalten, &sichtbar),
            [180.0, 420.0, 420.0, 260.0, 0.0],
        );
    }

    /// Eine mit der Maus verschobene Trennlinie gilt als neuer Wunsch.
    ///
    /// Die Gegenprobe zu `ein_hin_und_her_am_fensterrand_stellt_die_aufteilung_wieder_her`:
    /// die gemessenen Breiten werden nicht verworfen, sondern nur dort, wo sie
    /// nichts tragen, was die Regel nicht selbst ausgelegt hat. Das ist die
    /// Zusage aus dem Modulkopf von `appkit::aufteilung`, dass eine
    /// Ziehbewegung die naechste Fenstergroessenaenderung uebersteht.
    ///
    /// Die Zahlen sind gerechnet. Bei 1280 Punkten steht 180/420/420/260; die
    /// Linie zwischen den Dateifenstern wandert um 60 Punkte nach rechts, also
    /// 480 zu 360. Bei 1600 Punkten ist jeder Wunsch das 1,25-Fache wert:
    /// 225/600/450/325.
    #[test]
    fn eine_mit_der_maus_verschobene_trennlinie_gilt_als_neuer_wunsch() {
        let modell = modell();
        let sichtbar = modell.sichtbarkeit();
        let weit = mass(1280.0);

        let mut gezogen = bereichsbreiten(weit, &modell.breiten(), &sichtbar);
        gezogen[Bereich::Links.index()] += 60.0;
        gezogen[Bereich::Rechts.index()] -= 60.0;

        let gehalten = wuensche_nachfuehren(modell.breiten(), gemessen(gezogen), weit, &sichtbar);
        breiten_gleich(
            bereichsbreiten(mass(1600.0), &gehalten, &sichtbar),
            [225.0, 600.0, 450.0, 325.0, 0.0],
        );
    }

    /// Die Zuordnung von einem Bereich auf sein Feld in der Sichtbarkeit steht
    /// einmal.
    ///
    /// **Die Probe zum Befund 3 der Durchsicht vom 260812-0539.**
    /// `appkit::aufteilung` fuehrte bis dahin eine zweite, gleichlautende
    /// Fassung; sie ist weg, und diese Probe haelt fest, dass die verbliebene
    /// jedes der fuenf Felder trifft und keine zwei Bereiche auf dasselbe zeigt.
    #[test]
    fn die_zuordnung_von_bereich_auf_sichtbarkeit_trifft_jedes_feld() {
        for bereich in Bereich::ALLE {
            let mut sichtbar = Sichtbarkeit {
                lesezeichen: false,
                erstes_dateifenster: false,
                zweites_dateifenster: false,
                vorschau: false,
                editor: false,
            };
            match bereich {
                Bereich::Lesezeichen => sichtbar.lesezeichen = true,
                Bereich::Links => sichtbar.erstes_dateifenster = true,
                Bereich::Rechts => sichtbar.zweites_dateifenster = true,
                Bereich::Vorschau => sichtbar.vorschau = true,
                Bereich::Editor => sichtbar.editor = true,
            }
            for anderer in Bereich::ALLE {
                assert_eq!(
                    sichtbar_in(&sichtbar, anderer),
                    anderer == bereich,
                    "{anderer:?} bei gesetztem Feld von {bereich:?}"
                );
            }
        }
    }

    /// Kein sichtbarer Bereich faellt unter sein Mindestmass, solange die
    /// Mindestbreiten zusammen noch hineinpassen.
    ///
    /// 800 Punkte sind eng, aber nicht zu eng: die vier Bereiche der Runde 1
    /// brauchen mindestens 760. Der Anteil der Leiste faellt unter ihre 120,
    /// sie bekommt ihr Mindestmass, und die uebrigen teilen die restlichen 680
    /// weiter. Wird es noch enger, greift der zweite Zweig; den prueft
    /// `unter_der_summe_der_mindestbreiten_schrumpfen_alle_mit_demselben_faktor`.
    #[test]
    fn kein_bereich_faellt_unter_sein_mindestmass() {
        let modell = modell();
        let breiten = bereichsbreiten(mass(800.0), &modell.breiten(), &modell.sichtbarkeit());
        for bereich in Bereich::ALLE {
            if !modell.sichtbar(bereich) {
                continue;
            }
            assert!(
                breiten[bereich.index()] >= bereich.mindestbreite(),
                "{bereich:?}: {breiten:?}"
            );
        }
        breiten_gleich(breiten, [120.0, 259.636, 259.636, 160.727, 0.0]);
        let summe: f64 = breiten.iter().sum();
        assert!((summe - 800.0).abs() < 0.001, "{breiten:?}");
    }

    /// Der Tastenbefehl bewegt die Trennlinie um genau einen Schritt.
    ///
    /// Die Zahl im Modell und die Zahl auf dem Schirm muessen dieselbe sein.
    /// Bevor `breite_aendern` das andere Dateifenster mitzog, waren es 13
    /// Punkte statt 40, gemessen am 260804 im laufenden Buendel.
    ///
    /// **Gemessen wird ueber drei Fensterbreiten, und das ist die Behebung des
    /// Defekts vom 260812-0439.** Nach Schritt 1 der Bereichsleisten-Runde hielt
    /// die Zusage allein bei 1280 Punkten: dort trifft die Summe der
    /// gespeicherten Breiten die verfuegbare Breite, und ein gespeicherter Punkt
    /// ist ein Punkt auf dem Schirm. Bei 1400 sprang die Trennlinie um 43,75
    /// Punkte, bei 1920 um 60. Seit `breite_aendern` denselben Massstab nimmt
    /// wie die Anteilsregel, sind es ueberall 40; der Datensatz ist
    /// `issues/260812-0439_*_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`.
    #[test]
    fn der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt() {
        for gesamt in [1280.0, 1400.0, 1920.0] {
            let zeile = mass(gesamt);
            let mut modell = modell();
            // Wie vor jedem Befehl: erst nachlesen, was auf dem Schirm steht.
            // Seit dem 260812 bleibt der Ruf hier ohne Wirkung, weil auf dem
            // Schirm genau das steht, was die Regel selbst ausgelegt hat; der
            // Grund steht an `traegt_eine_ziehbewegung`. Er steht trotzdem da,
            // weil er in der Anwendung an dieser Stelle faellt.
            modell.breiten_uebernehmen(
                bereichsbreiten(zeile, &modell.breiten(), &modell.sichtbarkeit()),
                zeile,
            );
            let vorher = bereichsbreiten(zeile, &modell.breiten(), &modell.sichtbarkeit());

            modell.breite_aendern(Bereich::Links, BREITENSCHRITT, zeile);
            let nachher = bereichsbreiten(zeile, &modell.breiten(), &modell.sichtbarkeit());
            assert!(
                (nachher[Bereich::Links.index()] - vorher[Bereich::Links.index()] - BREITENSCHRITT)
                    .abs()
                    < 0.001,
                "bei {gesamt} Punkten Fensterbreite: vorher {vorher:?}, nachher {nachher:?}"
            );
            assert!(
                (vorher[Bereich::Rechts.index()]
                    - nachher[Bereich::Rechts.index()]
                    - BREITENSCHRITT)
                    .abs()
                    < 0.001,
                "das andere Dateifenster gibt nicht ab, was dieses bekommt: bei {gesamt} Punkten {vorher:?} zu {nachher:?}"
            );

            modell.breite_aendern(Bereich::Links, -BREITENSCHRITT, zeile);
            let zurueck = bereichsbreiten(zeile, &modell.breiten(), &modell.sichtbarkeit());
            assert!(
                (zurueck[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001,
                "bei {gesamt} Punkten kommt der Schritt zurueck: {zurueck:?} statt {vorher:?}"
            );
        }
    }

    /// Das wiedereingeblendete Dateifenster kommt auf seiner alten Breite
    /// zurueck.
    ///
    /// Der Fall, der die Zusage aus C7 am 260804 im laufenden Buendel verfehlt
    /// hat: das sichtbare Dateifenster traegt den Platz des ausgeblendeten mit,
    /// und diese Zahl darf nicht als sein Wunsch in das Modell zurueckfliessen.
    /// Seit der Anteilsregel haelt das nicht mehr eine Ausnahme fuer die beiden
    /// Dateifenster, sondern die Rueckrechnung in `breiten_uebernehmen`.
    #[test]
    fn das_wiedereingeblendete_dateifenster_hat_wieder_seine_alte_breite() {
        let mut modell = modell();
        modell.breiten_uebernehmen(
            bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit()),
            mass(1400.0),
        );
        let vorher = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());

        schalten(&mut modell, Bereich::Rechts);
        // Der Bildaufbau schreibt die gemessenen Breiten zurueck, so wie es der
        // Sitzungsabgleich und jeder Breitenbefehl tun.
        let alleine = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        modell.breiten_uebernehmen(alleine, mass(1400.0));
        assert!(
            alleine[Bereich::Links.index()] > vorher[Bereich::Links.index()],
            "das linke Dateifenster hat den Platz nicht uebernommen"
        );

        schalten(&mut modell, Bereich::Rechts);
        let nachher = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            (nachher[Bereich::Rechts.index()] - vorher[Bereich::Rechts.index()]).abs() < 0.001,
            "vorher {vorher:?}, nachher {nachher:?}"
        );
        assert!((nachher[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001);
    }

    /// Zwischen n sichtbaren Bereichen liegen n minus eine Trennlinie.
    ///
    /// Die Rechnung stand bis zur Bereichsleisten-Runde in
    /// `appkit::aufteilung::auslegen` und damit an einer Stelle, die keine
    /// Probe ohne Fenster erreicht.
    #[test]
    fn das_zeilenmass_zieht_je_trennlinie_ab() {
        let mass = Zeilenmass {
            gesamt: 1000.0,
            trennerbreite: 8.0,
        };
        assert_eq!(mass.verfuegbar(0), 1000.0, "kein Bereich, keine Trennlinie");
        assert_eq!(mass.verfuegbar(1), 1000.0, "ein Bereich, keine Trennlinie");
        assert_eq!(mass.verfuegbar(2), 992.0);
        assert_eq!(mass.verfuegbar(5), 968.0);
        assert_eq!(
            Zeilenmass {
                gesamt: 10.0,
                trennerbreite: 8.0,
            }
            .verfuegbar(5),
            0.0,
            "nie weniger als nichts"
        );
    }

    /// Zwei Bereiche im Verhaeltnis 2:1 stehen nach dem Einblenden eines
    /// dritten weiterhin im Verhaeltnis 2:1.
    ///
    /// Die Zusage, um die es der Directive dieser Runde geht. Sie gilt fuer
    /// jedes Paar und nicht nur fuer die beiden Dateifenster: hier sind es die
    /// Lesezeichenleiste und das linke Dateifenster, die bis zur
    /// Bereichsleisten-Runde in verschiedenen Zweigen der Rechnung lagen.
    #[test]
    fn das_verhaeltnis_zweier_bereiche_ueberlebt_das_einblenden_eines_dritten() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Rechts);
        schalten(&mut modell, Bereich::Vorschau);
        modell.breite_setzen(Bereich::Lesezeichen, 400.0);
        modell.breite_setzen(Bereich::Links, 200.0);

        let verhaeltnis = |breiten: [f64; 5]| {
            breiten[Bereich::Lesezeichen.index()] / breiten[Bereich::Links.index()]
        };

        let zwei = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(zwei, [800.0, 400.0, 0.0, 0.0, 0.0]);
        assert!((verhaeltnis(zwei) - 2.0).abs() < 0.001, "{zwei:?}");

        schalten(&mut modell, Bereich::Vorschau);
        let drei = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        assert!(drei[Bereich::Vorschau.index()] > 0.0, "{drei:?}");
        assert!(
            (verhaeltnis(drei) - 2.0).abs() < 0.001,
            "das Verhaeltnis hat sich verschoben: zu zweit {zwei:?}, zu dritt {drei:?}"
        );
        assert!(
            drei[Bereich::Lesezeichen.index()] < zwei[Bereich::Lesezeichen.index()],
            "der dritte Bereich hat nichts gekostet: {drei:?}"
        );
    }

    /// Die Summe der fuenf Breiten ist immer genau die verfuegbare Breite.
    ///
    /// Ueber vier Lagen der Sichtbarkeit, fuenf Fensterbreiten und drei
    /// Trennlinienbreiten, also auch weit unter der Summe der Mindestbreiten.
    /// Die Aufteilung rechnet aus diesen Breiten die Lage jeder Trennlinie; ein
    /// halber Punkt zu viel oder zu wenig waere am rechten Rand zu sehen.
    #[test]
    fn die_summe_ist_immer_die_verfuegbare_breite() {
        for gesamt in [500.0, 780.0, 1000.0, 1280.0, 2400.0] {
            for trennerbreite in [0.0, 1.0, 8.0] {
                let mass = Zeilenmass {
                    gesamt,
                    trennerbreite,
                };
                for lage in bereichslagen() {
                    let breiten = bereichsbreiten(mass, &lage.breiten(), &lage.sichtbarkeit());
                    let sichtbare = Bereich::ALLE
                        .iter()
                        .filter(|bereich| lage.sichtbar(**bereich))
                        .count();
                    let summe: f64 = breiten.iter().sum();
                    let verfuegbar = mass.verfuegbar(sichtbare);
                    assert!(
                        (summe - verfuegbar).abs() < 0.001,
                        "{summe} statt {verfuegbar} bei {mass:?}, {sichtbare} sichtbaren: {breiten:?}"
                    );
                }
            }
        }
    }

    /// Passt die Summe der Mindestbreiten nicht mehr in die Zeile, schrumpfen
    /// alle sichtbaren Bereiche mit demselben Faktor unter ihr Mindestmass.
    ///
    /// Der zweite Zweig der Regel. Er entsteht nicht durch einen
    /// Umschaltbefehl, sondern allein dadurch, dass der Nutzer das Fenster
    /// schmaler zieht als die Mindestbreiten erlauben; bei den vier Bereichen
    /// der Runde 1 sind das 760 Punkte.
    #[test]
    fn unter_der_summe_der_mindestbreiten_schrumpfen_alle_mit_demselben_faktor() {
        let modell = modell();
        let breiten = bereichsbreiten(mass(600.0), &modell.breiten(), &modell.sichtbarkeit());
        let faktor = 600.0 / 760.0;
        breiten_gleich(
            breiten,
            [
                120.0 * faktor,
                240.0 * faktor,
                240.0 * faktor,
                160.0 * faktor,
                0.0,
            ],
        );
        for bereich in Bereich::ALLE {
            if !modell.sichtbar(bereich) {
                continue;
            }
            assert!(
                breiten[bereich.index()] < bereich.mindestbreite(),
                "{bereich:?} steht noch auf seinem Mindestmass: {breiten:?}"
            );
        }
        let summe: f64 = breiten.iter().sum();
        assert!((summe - 600.0).abs() < 0.001, "{breiten:?}");
    }

    /// Das Vergroessern des Fensters aendert keine gespeicherte Breite.
    ///
    /// Der Nachzug rechnet die gemessenen Breiten auf die gespeicherte Summe
    /// zurueck. Ohne diese Rueckrechnung wuechse die Zahl jedes sichtbaren
    /// Bereichs mit dem Fenster, und die des ausgeblendeten Editors schrumpfte
    /// gegen sie zusammen: sein Anteil beim naechsten Einblenden waere ein
    /// anderer, obwohl niemand ihn angefasst hat.
    #[test]
    fn das_vergroessern_des_fensters_laesst_die_gespeicherten_breiten_stehen() {
        let mut modell = modell();
        modell.breite_setzen(Bereich::Editor, 500.0);
        modell.breiten_uebernehmen(
            bereichsbreiten(mass(1280.0), &modell.breiten(), &modell.sichtbarkeit()),
            mass(1280.0),
        );
        let vorher: Vec<f64> = Bereich::ALLE
            .iter()
            .map(|bereich| modell.breite_oder_anfang(*bereich))
            .collect();

        // Der Nutzer zieht das Fenster von 1280 auf 2000 Punkte auf, und der
        // naechste Befehl misst nach, bevor er das Modell anfasst.
        let gemessen = bereichsbreiten(mass(2000.0), &modell.breiten(), &modell.sichtbarkeit());
        modell.breiten_uebernehmen(gemessen, mass(2000.0));

        for bereich in Bereich::ALLE {
            let jetzt = modell.breite_oder_anfang(bereich);
            let damals = vorher[bereich.index()];
            assert!(
                (jetzt - damals).abs() < 0.001,
                "{bereich:?}: {jetzt} statt {damals}"
            );
        }
        assert_eq!(
            modell.breiten().editor,
            Some(500.0),
            "der ausgeblendete Editor bleibt unangetastet"
        );
    }

    /// Der Ausschluss aus C1 ist gegenseitig, und die Zuordnung sagt es in
    /// beide Richtungen.
    ///
    /// Ein einseitiger Eintrag traege die Regel nur fuer den einen Weg, und die
    /// Probe darunter faende ihn erst an einer von zwei Aufrufreihenfolgen.
    #[test]
    fn der_ausschluss_ist_gegenseitig() {
        assert_eq!(Bereich::Vorschau.teilt_flaeche_mit(), Some(Bereich::Editor));
        assert_eq!(Bereich::Editor.teilt_flaeche_mit(), Some(Bereich::Vorschau));
        for bereich in Bereich::ALLE {
            let Some(gegenueber) = bereich.teilt_flaeche_mit() else {
                continue;
            };
            assert_eq!(
                gegenueber.teilt_flaeche_mit(),
                Some(bereich),
                "{bereich:?} teilt sich die Flaeche mit {gegenueber:?}, aber nicht umgekehrt"
            );
        }
    }

    /// Kriterium C2.1 der Bereichsleisten-Runde: fuenf Schalter, und jeder
    /// nennt seinen Bereich.
    ///
    /// Zwei Schalter mit derselben Aufschrift waeren fuer den Nutzer nicht zu
    /// unterscheiden, und ein leerer traege gar keine Auskunft.
    #[test]
    fn jeder_bereich_hat_eine_eigene_beschriftung() {
        for (stelle, bereich) in Bereich::ALLE.into_iter().enumerate() {
            assert!(
                !bereich.beschriftung().is_empty(),
                "{bereich:?} traegt keine Aufschrift"
            );
            assert!(
                !bereich.langname().is_empty(),
                "{bereich:?} traegt keinen Hinweistext"
            );
            for andere in Bereich::ALLE.into_iter().skip(stelle + 1) {
                assert_ne!(
                    bereich.beschriftung(),
                    andere.beschriftung(),
                    "{bereich:?} und {andere:?} tragen dieselbe Aufschrift"
                );
                assert_ne!(
                    bereich.langname(),
                    andere.langname(),
                    "{bereich:?} und {andere:?} tragen denselben Hinweistext"
                );
            }
        }
    }

    /// Das erste Abnahmekriterium von C1 der Editor-Runde, Satz eins und zwei:
    /// wer den einen einblendet, blendet den anderen aus.
    #[test]
    fn der_editor_schliesst_die_vorschau_und_die_vorschau_den_editor() {
        let mut modell = modell();
        assert!(modell.sichtbar(Bereich::Vorschau), "die Probe beginnt so");
        assert!(!modell.sichtbar(Bereich::Editor));

        assert!(modell.einblenden(Bereich::Editor, weit()));
        assert!(modell.sichtbar(Bereich::Editor));
        assert!(
            !modell.sichtbar(Bereich::Vorschau),
            "der geoeffnete Editor hat die Vorschau nicht geschlossen"
        );

        assert!(modell.einblenden(Bereich::Vorschau, weit()));
        assert!(modell.sichtbar(Bereich::Vorschau));
        assert!(
            !modell.sichtbar(Bereich::Editor),
            "die eingeblendete Vorschau hat den Editor nicht verdraengt"
        );
    }

    /// Das erste Abnahmekriterium von C1, Satz drei: "Beide zugleich sichtbar
    /// zu haben ist ueber keinen Weg erreichbar."
    ///
    /// Erreichbar sind zur Laufzeit genau zwei Aufrufe, und die Probe faehrt
    /// jedes Paar aus zweien ueber jeden Bereich, vom Auslieferungszustand aus.
    /// Geprueft wird nach **jedem** der beiden Aufrufe, damit auch ein
    /// Zwischenzustand nicht durchgeht.
    #[test]
    fn keine_folge_aus_zwei_aufrufen_zeigt_editor_und_vorschau_zugleich() {
        type Aufruf = fn(&mut Fenstermodell, Bereich, Zeilenmass) -> bool;
        const AUFRUFE: [(&str, Aufruf); 2] = [
            ("umschalten", Fenstermodell::umschalten),
            ("einblenden", Fenstermodell::einblenden),
        ];

        for (erster_name, erster) in AUFRUFE {
            for erster_bereich in Bereich::ALLE {
                for (zweiter_name, zweiter) in AUFRUFE {
                    for zweiter_bereich in Bereich::ALLE {
                        let spur = format!(
                            "{erster_name}({erster_bereich:?}), {zweiter_name}({zweiter_bereich:?})"
                        );
                        let mut modell = modell();
                        // Ob ein einzelner Aufruf abgewiesen wird, ist hier
                        // ohne Belang: gemessen wird der Zustand danach, und
                        // ein abgewiesener Aufruf laesst ihn stehen.
                        let _ = erster(&mut modell, erster_bereich, weit());
                        beide_nicht_zugleich(&modell, &spur);
                        let _ = zweiter(&mut modell, zweiter_bereich, weit());
                        beide_nicht_zugleich(&modell, &spur);
                    }
                }
            }
        }
    }

    fn beide_nicht_zugleich(modell: &Fenstermodell, spur: &str) {
        assert!(
            !(modell.sichtbar(Bereich::Vorschau) && modell.sichtbar(Bereich::Editor)),
            "Vorschau und Editor stehen zugleich nach: {spur}"
        );
    }

    /// Eine von Hand geschriebene `session.toml` bringt die beiden nicht
    /// zugleich auf den Schirm.
    ///
    /// Derselbe Fall und derselbe Grund wie beim ausgeblendeten aktiven
    /// Dateifenster darueber: `serde` liest jede Feldkombination ein, und die
    /// Zusicherung gehoert an die Stelle, die sie einloest.
    #[test]
    fn eine_von_hand_gesetzte_sitzung_zeigt_nicht_beide_zugleich() {
        let mut sitzung = Sitzung::default();
        sitzung.sichtbar.vorschau = true;
        sitzung.sichtbar.editor = true;
        let modell = Fenstermodell::aus_sitzung(&sitzung);
        assert!(modell.sichtbar(Bereich::Vorschau));
        assert!(
            !modell.sichtbar(Bereich::Editor),
            "der Editor haelt beim Start keine Datei und weicht der Vorschau"
        );

        // Die Gegenprobe: ohne Vorschau bleibt der Editor stehen.
        let mut sitzung = Sitzung::default();
        sitzung.sichtbar.vorschau = false;
        sitzung.sichtbar.editor = true;
        let modell = Fenstermodell::aus_sitzung(&sitzung);
        assert!(modell.sichtbar(Bereich::Editor));
        assert!(!modell.sichtbar(Bereich::Vorschau));
    }

    /// Eine verstellte Editorbreite steht in `session.toml` und kommt beim
    /// Einlesen wieder heraus (C1, C7 der Runde 1).
    ///
    /// Die Agentenseite des fuenften Abnahmekriteriums von C1. Dass sie
    /// Beenden und Neustart auch am laufenden Buendel uebersteht, prueft der
    /// Nutzer; hier laeuft dieselbe Zeichenkette durch, die auf die Platte
    /// geht, und der Weg ist deshalb derselbe.
    #[test]
    fn eine_verstellte_editorbreite_ueberlebt_die_sitzung() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Editor);
        modell.breite_aendern(Bereich::Editor, BREITENSCHRITT, passend(&modell));
        let gewuenscht = Bereich::Editor.anfangsbreite() + BREITENSCHRITT;
        assert_eq!(modell.breiten().editor, Some(gewuenscht));

        let sitzung = modell.sitzung(Sitzung::default().fenster, None, Zettel::Erster);
        let text = toml::to_string(&sitzung).expect("die Sitzung laesst sich schreiben");
        assert!(
            text.contains("editor"),
            "die Editorbreite steht nicht in session.toml: {text}"
        );

        let gelesen: Sitzung = toml::from_str(&text).expect("die Sitzung laesst sich lesen");
        let wieder = Fenstermodell::aus_sitzung(&gelesen);
        assert_eq!(wieder.breiten().editor, Some(gewuenscht));
        assert!(wieder.sichtbar(Bereich::Editor));
        assert!(
            !wieder.sichtbar(Bereich::Vorschau),
            "der Ausschluss aus C1 uebersteht die Sitzung ebenfalls"
        );
    }

    #[test]
    fn die_lesereihenfolge_nimmt_die_sichtbaren_tabs_zuerst() {
        let modell = modell();
        let links = Tabuebersicht {
            zahl: 2,
            sichtbar: 0,
        };
        let rechts = Tabuebersicht {
            zahl: 2,
            sichtbar: 1,
        };
        assert_eq!(
            modell.lesereihenfolge([links, rechts]),
            [
                (Fensterseite::Links, 0),
                (Fensterseite::Rechts, 1),
                (Fensterseite::Links, 1),
                (Fensterseite::Rechts, 0),
            ],
            "das ist die Pruefsitzung aus C8: je zwei Tabs, je einer sichtbar"
        );
    }

    #[test]
    fn ein_ausgeblendetes_dateifenster_liest_erst_in_der_zweiten_stufe() {
        let mut modell = modell();
        schalten(&mut modell, Bereich::Rechts);
        let eines = Tabuebersicht {
            zahl: 1,
            sichtbar: 0,
        };
        assert_eq!(
            modell.lesereihenfolge([eines, eines]),
            [(Fensterseite::Links, 0), (Fensterseite::Rechts, 0)]
        );
    }

    // -------------------------------------------------------------------
    // Die Spalten der beiden Dateilisten (C3 der Bereichsleisten-Runde)
    // -------------------------------------------------------------------

    /// Ab Werk stehen alle vier Spalten (Kriterium C7.2, zweiter Halbsatz).
    #[test]
    fn der_auslieferungszustand_zeigt_alle_vier_spalten() {
        let modell = modell();
        for spalte in Spalte::ALLE {
            assert!(
                spalte_sichtbar_in(&modell.spaltensichtbarkeit(), spalte),
                "{spalte:?} steht ab Werk nicht"
            );
        }
    }

    /// Jede der drei schaltbaren Spalten kippt und kommt zurueck, und keine
    /// nimmt eine andere mit.
    #[test]
    fn jede_schaltbare_spalte_kippt_fuer_sich() {
        for geschaltet in [Spalte::Groesse, Spalte::Geaendert, Spalte::Typ] {
            let mut modell = modell();
            assert!(
                modell.spalte_umschalten(geschaltet),
                "{geschaltet:?} liess sich nicht schalten"
            );

            for spalte in Spalte::ALLE {
                let steht = spalte_sichtbar_in(&modell.spaltensichtbarkeit(), spalte);
                assert_eq!(
                    steht,
                    spalte != geschaltet,
                    "nach dem Schalten von {geschaltet:?} steht {spalte:?} falsch"
                );
            }

            assert!(modell.spalte_umschalten(geschaltet));
            assert!(
                spalte_sichtbar_in(&modell.spaltensichtbarkeit(), geschaltet),
                "{geschaltet:?} kommt nicht zurueck"
            );
        }
    }

    /// Die Namensspalte traegt keinen Schalter: der Befehl wird abgewiesen und
    /// die Spalte bleibt stehen.
    ///
    /// Eine Dateiliste ohne sie zeigt nichts, was den Eintrag benennt; die
    /// Abweisung bleibt stumm wie jede in diesem Modell.
    #[test]
    fn die_namensspalte_laesst_sich_nicht_wegschalten() {
        let mut modell = modell();
        assert!(
            !modell.spalte_umschalten(Spalte::Name),
            "die Namensspalte hat sich schalten lassen"
        );
        assert!(spalte_sichtbar_in(
            &modell.spaltensichtbarkeit(),
            Spalte::Name
        ));
        assert_eq!(
            modell.spaltensichtbarkeit(),
            Spaltensichtbarkeit::default(),
            "die abgewiesene Namensspalte hat ein anderes Feld angefasst"
        );
    }

    /// Das Wegschalten der Sortierspalte laesst die Sortierung stehen
    /// (Kriterium C3.3).
    ///
    /// **Gemessen wird an dem, was in `session.toml` landet**, denn dort steht
    /// der Sortierschluessel: die Tabs reisen als [`Fensterzustand`] durch
    /// `sitzung()`, und wenn das Schalten einer Spalte an ihnen etwas aenderte,
    /// muesste es hier zu sehen sein. Dass es das nicht kann, ist die Zusage —
    /// `spalte_umschalten` kommt an die Tabs gar nicht heran, und diese Probe
    /// haelt es fest, bevor eine spaetere Runde die beiden zusammenlegt.
    #[test]
    fn das_wegschalten_der_sortierspalte_laesst_die_sortierung_stehen() {
        let nach_groesse = Sortierung::neu(Schluessel::Groesse, Richtung::Absteigend);
        let mut fenster = Sitzung::default().fenster;
        for seite in &mut fenster {
            seite.tabs[0].sortierung = nach_groesse;
        }

        let mut modell = modell();
        assert!(modell.spalte_umschalten(Spalte::Groesse));

        let sitzung = modell.sitzung(fenster, None, Zettel::Erster);
        assert!(
            !sitzung.spalten.groesse,
            "die Spalte Groesse ist nicht weggeschaltet"
        );
        for seite in Fensterseite::ALLE {
            assert_eq!(
                sitzung.fenster(seite).tabs[0].sortierung,
                nach_groesse,
                "{seite:?} hat seine Sortierung verloren"
            );
        }
    }

    /// Die Spaltensichtbarkeit uebersteht den Weg durch `session.toml`
    /// (Kriterium C7.2).
    #[test]
    fn die_spaltensichtbarkeit_uebersteht_die_sitzung() {
        let mut modell = modell();
        assert!(modell.spalte_umschalten(Spalte::Groesse));
        assert!(modell.spalte_umschalten(Spalte::Typ));

        let sitzung = modell.sitzung(Sitzung::default().fenster, None, Zettel::Erster);
        let text = toml::to_string(&sitzung).expect("die Sitzung laesst sich schreiben");
        let gelesen: Sitzung = toml::from_str(&text).expect("die Sitzung laesst sich lesen");
        let wieder = Fenstermodell::aus_sitzung(&gelesen);

        assert_eq!(wieder.spaltensichtbarkeit(), modell.spaltensichtbarkeit());
        assert!(!spalte_sichtbar_in(
            &wieder.spaltensichtbarkeit(),
            Spalte::Groesse
        ));
        assert!(spalte_sichtbar_in(
            &wieder.spaltensichtbarkeit(),
            Spalte::Geaendert
        ));
        assert!(!spalte_sichtbar_in(
            &wieder.spaltensichtbarkeit(),
            Spalte::Typ
        ));
    }
}
