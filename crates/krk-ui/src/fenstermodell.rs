//! Das Fenstermodell: welches Dateifenster das aktive ist, welche Bereiche
//! sichtbar sind, wie breit sie stehen und in welcher Reihenfolge beim Start
//! gelesen wird.
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
//! # Was das linke Dateifenster von den anderen unterscheidet
//!
//! Es laesst sich nicht ausblenden. C7 sichert zu, dass mindestens ein
//! Dateifenster sichtbar bleibt, und [`Sichtbarkeit`] traegt deshalb gar kein
//! Feld dafuer. [`Fenstermodell::umschalten`] weist einen Befehl auf
//! [`Bereich::Links`] trotzdem ausdruecklich ab, statt sich auf die fehlende
//! Belegung zu verlassen: die Zusage gehoert an die Stelle, die sie einloest.

use std::path::PathBuf;

use krk_core::ablage::{
    Breiten, Dateifenster as Fensterzustand, Fensterseite, Sichtbarkeit, Sitzung,
};

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
pub fn sichtbar_in(sichtbar: &Sichtbarkeit, bereich: Bereich) -> bool {
    match bereich {
        Bereich::Lesezeichen => sichtbar.lesezeichen,
        // C7 laesst das letzte Dateifenster nicht ausblenden, und
        // `Sichtbarkeit` traegt deshalb kein Feld dafuer; siehe den Modulkopf.
        Bereich::Links => true,
        Bereich::Rechts => sichtbar.zweites_dateifenster,
        Bereich::Vorschau => sichtbar.vorschau,
        Bereich::Editor => sichtbar.editor,
    }
}

/// Das gehaltene Fenstermodell.
///
/// Es traegt, was nicht zu den Tabs gehoert: das aktive Dateifenster, die
/// Sichtbarkeit der fuenf Bereiche und ihre Breiten. Die Tabs selbst haelt
/// [`Tabliste`], je eine Liste je Dateifenster.
#[derive(Debug)]
pub struct Fenstermodell {
    aktiv: Fensterseite,
    breiten: Breiten,
    sichtbar: Sichtbarkeit,
}

impl Fenstermodell {
    /// Das Modell aus einer geladenen Sitzung.
    ///
    /// **Zwei Zusicherungen werden hier hergestellt und nicht nur
    /// unterstellt**, weil `session.toml` nach C7 zum Lesen und Aendern von
    /// Hand gedacht ist und `serde` jede Feldkombination anstandslos einliest.
    /// Zur Laufzeit haelt [`Fenstermodell::umschalten`] beide; die Datei kommt
    /// nicht von dort.
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
    pub fn aus_sitzung(sitzung: &Sitzung) -> Self {
        let mut modell = Self {
            aktiv: sitzung.aktiv,
            breiten: sitzung.breiten,
            sichtbar: sitzung.sichtbar,
        };
        if !modell.sichtbar(Bereich::von_seite(modell.aktiv)) {
            // Das linke ist immer sichtbar, siehe den Modulkopf.
            modell.aktiv = Fensterseite::Links;
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
    pub fn sitzung(&self, fenster: [Fensterzustand; 2], editor: Option<PathBuf>) -> Sitzung {
        Sitzung {
            aktiv: self.aktiv,
            editor,
            breiten: self.breiten,
            sichtbar: self.sichtbar,
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

    /// Setzt die Sichtbarkeit eines Bereichs.
    ///
    /// **Die eine Stelle, die ein Feld von [`Sichtbarkeit`] schreibt.** Der
    /// gegenseitige Ausschluss aus [`Bereich::teilt_flaeche_mit`] wirkt ueber
    /// sie und nicht neben ihr; wer sie umgeht, hat eine zweite Wahrheit
    /// darueber, welche Bereiche stehen.
    ///
    /// Fuer [`Bereich::Links`] geschieht nichts, und das ist kein
    /// Auffangzweig: [`Sichtbarkeit`] traegt gar kein Feld fuer das linke
    /// Dateifenster, weil C7 zusagt, dass mindestens eines sichtbar bleibt
    /// (siehe den Modulkopf). Der Weg hierher ist ohnehin versperrt, weil
    /// [`Self::umschalten`] den Bereich vorher abweist.
    fn sichtbar_setzen(&mut self, bereich: Bereich, sichtbar: bool) {
        match bereich {
            Bereich::Lesezeichen => self.sichtbar.lesezeichen = sichtbar,
            Bereich::Links => {}
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

    /// Blendet einen Bereich aus oder wieder ein (C7).
    ///
    /// Liefert `false` fuer den einen Befehl, den C7 ausdruecklich verwirft:
    /// das letzte sichtbare Dateifenster auszublenden. Der Aufrufer meldet
    /// nichts; die Zusage lautet "wird ohne Fehlermeldung ignoriert".
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
    pub fn umschalten(&mut self, bereich: Bereich) -> bool {
        let jetzt_sichtbar = !self.sichtbar(bereich);
        match bereich {
            Bereich::Lesezeichen | Bereich::Vorschau | Bereich::Editor => {
                self.sichtbar_setzen(bereich, jetzt_sichtbar);
            }
            Bereich::Rechts => {
                self.sichtbar_setzen(bereich, jetzt_sichtbar);
                if !jetzt_sichtbar && self.aktiv == Fensterseite::Rechts {
                    self.aktiv = Fensterseite::Links;
                }
            }
            // Das letzte sichtbare Dateifenster. Kein ausgeliefertes Kuerzel
            // fuehrt heute hierher; die Abweisung steht trotzdem hier und nicht
            // in der Belegungsdatei, weil eine spaetere Belegung sie sonst
            // umgehen koennte.
            Bereich::Links => return false,
        }
        if jetzt_sichtbar {
            self.gegenueber_raeumen(bereich);
        }
        true
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
    pub fn einblenden(&mut self, bereich: Bereich) -> bool {
        if self.sichtbar(bereich) {
            return false;
        }
        self.umschalten(bereich)
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
    ///
    /// **Der Schritt gilt in gespeicherten Punkten und nicht in Punkten auf dem
    /// Schirm.** Beide sind dieselbe Zahl, solange die Summe der gespeicherten
    /// Breiten der sichtbaren Bereiche die verfuegbare Breite trifft; steht das
    /// Fenster breiter, kommt der Schritt um denselben Faktor vergroessert an.
    /// Seit die Breiten Anteile sind, faellt dieser Faktor nicht mehr von
    /// selbst auf 1 zurueck, weil
    /// [`Fenstermodell::breiten_uebernehmen`](Self::breiten_uebernehmen) die
    /// gespeicherte Summe festhaelt. Der Datensatz dazu ist
    /// `issues/260812-0439_*_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`.
    ///
    /// **Das Gegenueber kommt aus [`Fensterseite::andere`]** und nicht aus einer
    /// eigenen Fallunterscheidung ueber [`Bereich`]. Bis zum 260809 stand hier
    /// ein `match` mit dem Auffangzweig `_ => Bereich::Links`; er gab die
    /// richtige Antwort, weil nur die beiden Dateifenster hierher kommen, und
    /// hat den fuenften Bereich der Editor-Runde stumm aufgenommen, ohne dass
    /// der Uebersetzer eine Einordnung verlangt haette. Die Frage "welcher
    /// Bereich ist das Gegenueber" wird jetzt nicht mehr richtig beantwortet,
    /// sondern gar nicht mehr gestellt.
    pub fn breite_aendern(&mut self, bereich: Bereich, betrag: f64) {
        if let Some(seite) = bereich.seite() {
            let anderer = Bereich::von_seite(seite.andere());
            if !self.sichtbar(anderer) {
                // Ein einziges sichtbares Dateifenster nimmt ohnehin die ganze
                // Breite; es gibt keine Trennlinie, die sich verschieben liesse.
                return;
            }
            let hier = self.breite_oder_anfang(bereich);
            let dort = self.breite_oder_anfang(anderer);
            let betrag = betrag
                .min(dort - anderer.mindestbreite())
                .max(bereich.mindestbreite() - hier);
            self.breite_setzen(bereich, hier + betrag);
            self.breite_setzen(anderer, dort - betrag);
            return;
        }
        let jetzt = self.breite_oder_anfang(bereich);
        self.breite_setzen(bereich, (jetzt + betrag).max(bereich.mindestbreite()));
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
    pub fn breiten_uebernehmen(&mut self, gemessen: [f64; 5]) {
        let nachzufuehren: Vec<Bereich> = Bereich::ALLE
            .into_iter()
            .filter(|bereich| self.sichtbar(*bereich) && gemessen[bereich.index()] > 0.0)
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
        match bereich {
            Bereich::Lesezeichen => self.breiten.lesezeichen,
            Bereich::Links => self.breiten.links,
            Bereich::Rechts => self.breiten.rechts,
            Bereich::Vorschau => self.breiten.vorschau,
            Bereich::Editor => self.breiten.editor,
        }
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
    let modell = Fenstermodell {
        aktiv: Fensterseite::Links,
        breiten: *breiten,
        sichtbar: *sichtbar,
    };
    let mut ergebnis = [0.0_f64; 5];

    // Welche Bereiche etwas bekommen, sagt allein die Sichtbarkeit. Bis zur
    // Editor-Runde stand hier die Literalliste der festen Bereiche als zweite
    // Aufzaehlung daneben; seit der Anteilsregel gibt es keine festen Bereiche
    // mehr, an denen eine solche Liste ansetzen koennte.
    let sichtbare: Vec<Bereich> = Bereich::ALLE
        .into_iter()
        .filter(|bereich| modell.sichtbar(*bereich))
        .collect();
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

#[cfg(test)]
mod tests {
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
        mit_editor.umschalten(Bereich::Vorschau);
        mit_editor.umschalten(Bereich::Editor);

        let mut nur_dateifenster = modell();
        nur_dateifenster.umschalten(Bereich::Lesezeichen);
        nur_dateifenster.umschalten(Bereich::Vorschau);

        let mut ein_dateifenster = modell();
        ein_dateifenster.umschalten(Bereich::Rechts);
        ein_dateifenster.umschalten(Bereich::Lesezeichen);
        ein_dateifenster.umschalten(Bereich::Vorschau);

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

    #[test]
    fn das_letzte_dateifenster_laesst_sich_nicht_ausblenden() {
        let mut modell = modell();
        assert!(
            !modell.umschalten(Bereich::Links),
            "C7 verwirft diesen Befehl"
        );
        assert!(modell.sichtbar(Bereich::Links));
    }

    #[test]
    fn das_zweite_dateifenster_geht_aus_und_wieder_ein() {
        let mut modell = modell();
        assert!(modell.umschalten(Bereich::Rechts));
        assert!(!modell.sichtbar(Bereich::Rechts));
        assert!(modell.umschalten(Bereich::Rechts));
        assert!(modell.sichtbar(Bereich::Rechts));
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
        for bereich in [Bereich::Lesezeichen, Bereich::Vorschau, Bereich::Rechts] {
            let mut modell = modell();
            assert!(
                modell.sichtbar(bereich),
                "die Probe beginnt mit sichtbarem Bereich"
            );
            assert!(
                !modell.einblenden(bereich),
                "ein sichtbarer Bereich aendert sich nicht"
            );
            assert!(modell.sichtbar(bereich), "und bleibt sichtbar");

            modell.umschalten(bereich);
            assert!(!modell.sichtbar(bereich));
            assert!(modell.einblenden(bereich), "der ausgeblendete kommt hervor");
            assert!(modell.sichtbar(bereich));
        }
    }

    /// Das linke Dateifenster ist nie ausgeblendet, und deshalb hat der
    /// Fokusbefehl dorthin nichts hervorzuholen.
    #[test]
    fn das_letzte_dateifenster_ist_immer_schon_eingeblendet() {
        let mut modell = modell();
        assert!(modell.sichtbar(Bereich::Links));
        assert!(!modell.einblenden(Bereich::Links));
        assert!(modell.sichtbar(Bereich::Links));
    }

    #[test]
    fn das_ausblenden_holt_die_aktivitaet_zurueck_nach_links() {
        let mut modell = modell();
        assert!(modell.fenster_wechseln());
        assert_eq!(modell.aktiv(), Fensterseite::Rechts);
        modell.umschalten(Bereich::Rechts);
        assert_eq!(
            modell.aktiv(),
            Fensterseite::Links,
            "ein ausgeblendetes Dateifenster kann nicht das aktive sein"
        );
        assert!(
            !modell.fenster_wechseln(),
            "und der Wechsel dorthin geschieht nicht"
        );
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

        modell.umschalten(Bereich::Lesezeichen);
        let breiten = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Lesezeichen.index()], 0.0);
        assert_eq!(
            modell.breiten().lesezeichen,
            Some(200.0),
            "die gespeicherte Breite ueberlebt das Ausblenden"
        );

        modell.umschalten(Bereich::Lesezeichen);
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
        ohne_vorschau.umschalten(Bereich::Vorschau);
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
        modell.umschalten(Bereich::Rechts);
        modell.umschalten(Bereich::Lesezeichen);
        modell.umschalten(Bereich::Vorschau);
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
        modell.umschalten(Bereich::Vorschau);
        assert!(modell.umschalten(Bereich::Editor));
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
        modell.umschalten(Bereich::Vorschau);
        modell.umschalten(Bereich::Editor);

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
        modell.umschalten(Bereich::Vorschau);
        modell.umschalten(Bereich::Editor);
        modell.breite_setzen(Bereich::Editor, 500.0);
        let vorher = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());

        modell.umschalten(Bereich::Editor);
        assert!(!modell.sichtbar(Bereich::Editor));
        let breiten = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Editor.index()], 0.0, "{breiten:?}");
        assert_eq!(modell.breiten().editor, Some(500.0));

        modell.umschalten(Bereich::Editor);
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
        mit_leiste.umschalten(Bereich::Vorschau);
        mit_leiste.umschalten(Bereich::Editor);
        let offen = bereichsbreiten(
            mass(1280.0),
            &mit_leiste.breiten(),
            &mit_leiste.sichtbarkeit(),
        );

        let mut ohne_leiste = modell();
        ohne_leiste.umschalten(Bereich::Vorschau);
        ohne_leiste.umschalten(Bereich::Editor);
        ohne_leiste.umschalten(Bereich::Lesezeichen);
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

    /// Ein fester Bereich waechst unmittelbar und zieht kein Dateifenster mit.
    ///
    /// Der Editor ist der Fall, den der Auffangzweig stumm aufgenommen hatte.
    #[test]
    fn ein_fester_bereich_aendert_nur_seine_eigene_breite() {
        let mut modell = modell();
        modell.umschalten(Bereich::Vorschau);
        modell.umschalten(Bereich::Editor);
        let links_vorher = modell.breite_oder_anfang(Bereich::Links);
        let rechts_vorher = modell.breite_oder_anfang(Bereich::Rechts);

        modell.breite_aendern(Bereich::Editor, BREITENSCHRITT);
        assert_eq!(
            modell.breiten().editor,
            Some(Bereich::Editor.anfangsbreite() + BREITENSCHRITT)
        );
        assert_eq!(modell.breite_oder_anfang(Bereich::Links), links_vorher);
        assert_eq!(modell.breite_oder_anfang(Bereich::Rechts), rechts_vorher);

        // Und er faellt nicht unter sein Mindestmass.
        modell.breite_aendern(Bereich::Editor, -10_000.0);
        assert_eq!(
            modell.breiten().editor,
            Some(Bereich::Editor.mindestbreite())
        );
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
    /// **Gemessen wird bei 1280 Punkten, und das ist keine beliebige Zahl:**
    /// dort trifft die Summe der gespeicherten Breiten die verfuegbare Breite,
    /// und ein gespeicherter Punkt ist ein Punkt auf dem Schirm. Steht das
    /// Fenster breiter, kommt der Schritt um den Faktor der Anteilsregel
    /// vergroessert an; der Datensatz dazu ist
    /// `issues/260812-0439_*_der-breitenschritt-aus-c7-kommt-unter-der-anteilsregel-skaliert-auf-dem-schirm-an.md`,
    /// und der Kommentar an `breite_aendern` fuehrt ihn.
    #[test]
    fn der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt() {
        let mut modell = modell();
        modell.breiten_uebernehmen(bereichsbreiten(
            mass(1280.0),
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher = bereichsbreiten(mass(1280.0), &modell.breiten(), &modell.sichtbarkeit());

        modell.breite_aendern(Bereich::Links, BREITENSCHRITT);
        let nachher = bereichsbreiten(mass(1280.0), &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            (nachher[Bereich::Links.index()] - vorher[Bereich::Links.index()] - BREITENSCHRITT)
                .abs()
                < 0.001,
            "vorher {vorher:?}, nachher {nachher:?}"
        );
        assert!(
            (vorher[Bereich::Rechts.index()] - nachher[Bereich::Rechts.index()] - BREITENSCHRITT)
                .abs()
                < 0.001,
            "das andere Dateifenster gibt nicht ab, was dieses bekommt"
        );

        modell.breite_aendern(Bereich::Links, -BREITENSCHRITT);
        let zurueck = bereichsbreiten(mass(1280.0), &modell.breiten(), &modell.sichtbarkeit());
        assert!((zurueck[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001);
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
        modell.breiten_uebernehmen(bereichsbreiten(
            mass(1400.0),
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());

        modell.umschalten(Bereich::Rechts);
        // Der Bildaufbau schreibt die gemessenen Breiten zurueck, so wie es der
        // Sitzungsabgleich und jeder Breitenbefehl tun.
        let alleine = bereichsbreiten(mass(1400.0), &modell.breiten(), &modell.sichtbarkeit());
        modell.breiten_uebernehmen(alleine);
        assert!(
            alleine[Bereich::Links.index()] > vorher[Bereich::Links.index()],
            "das linke Dateifenster hat den Platz nicht uebernommen"
        );

        modell.umschalten(Bereich::Rechts);
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
        modell.umschalten(Bereich::Rechts);
        modell.umschalten(Bereich::Vorschau);
        modell.breite_setzen(Bereich::Lesezeichen, 400.0);
        modell.breite_setzen(Bereich::Links, 200.0);

        let verhaeltnis = |breiten: [f64; 5]| {
            breiten[Bereich::Lesezeichen.index()] / breiten[Bereich::Links.index()]
        };

        let zwei = bereichsbreiten(mass(1200.0), &modell.breiten(), &modell.sichtbarkeit());
        breiten_gleich(zwei, [800.0, 400.0, 0.0, 0.0, 0.0]);
        assert!((verhaeltnis(zwei) - 2.0).abs() < 0.001, "{zwei:?}");

        modell.umschalten(Bereich::Vorschau);
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
        modell.breiten_uebernehmen(bereichsbreiten(
            mass(1280.0),
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher: Vec<f64> = Bereich::ALLE
            .iter()
            .map(|bereich| modell.breite_oder_anfang(*bereich))
            .collect();

        // Der Nutzer zieht das Fenster von 1280 auf 2000 Punkte auf, und der
        // naechste Befehl misst nach, bevor er das Modell anfasst.
        let gemessen = bereichsbreiten(mass(2000.0), &modell.breiten(), &modell.sichtbarkeit());
        modell.breiten_uebernehmen(gemessen);

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

    /// Das erste Abnahmekriterium von C1 der Editor-Runde, Satz eins und zwei:
    /// wer den einen einblendet, blendet den anderen aus.
    #[test]
    fn der_editor_schliesst_die_vorschau_und_die_vorschau_den_editor() {
        let mut modell = modell();
        assert!(modell.sichtbar(Bereich::Vorschau), "die Probe beginnt so");
        assert!(!modell.sichtbar(Bereich::Editor));

        assert!(modell.einblenden(Bereich::Editor));
        assert!(modell.sichtbar(Bereich::Editor));
        assert!(
            !modell.sichtbar(Bereich::Vorschau),
            "der geoeffnete Editor hat die Vorschau nicht geschlossen"
        );

        assert!(modell.einblenden(Bereich::Vorschau));
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
        type Aufruf = fn(&mut Fenstermodell, Bereich) -> bool;
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
                        erster(&mut modell, erster_bereich);
                        beide_nicht_zugleich(&modell, &spur);
                        zweiter(&mut modell, zweiter_bereich);
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
        modell.umschalten(Bereich::Editor);
        modell.breite_aendern(Bereich::Editor, BREITENSCHRITT);
        let gewuenscht = Bereich::Editor.anfangsbreite() + BREITENSCHRITT;
        assert_eq!(modell.breiten().editor, Some(gewuenscht));

        let sitzung = modell.sitzung(Sitzung::default().fenster, None);
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
        modell.umschalten(Bereich::Rechts);
        let eines = Tabuebersicht {
            zahl: 1,
            sichtbar: 0,
        };
        assert_eq!(
            modell.lesereihenfolge([eines, eines]),
            [(Fensterseite::Links, 0), (Fensterseite::Rechts, 0)]
        );
    }
}
