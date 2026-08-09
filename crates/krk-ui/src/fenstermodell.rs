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
//!    fest          beweglich          beweglich         fest        fest
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
//! "Fest" und "beweglich" beziehen sich auf das Verteilen des Platzes: die
//! beiden Randbereiche behalten ihre Breite, die beiden Dateifenster teilen
//! sich, was uebrig bleibt. Das ist die Aufteilung, die C7 verlangt ("Die
//! verbleibenden Bereiche nutzen den frei gewordenen Platz"), und sie macht
//! zugleich die Zusage darunter einfach: eine gespeicherte Breite gilt auch
//! fuer einen ausgeblendeten Bereich, also steht sie beim Wiedereinblenden
//! wieder da.
//!
//! # Was das linke Dateifenster von den anderen unterscheidet
//!
//! Es laesst sich nicht ausblenden. C7 sichert zu, dass mindestens ein
//! Dateifenster sichtbar bleibt, und [`Sichtbarkeit`] traegt deshalb gar kein
//! Feld dafuer. [`Fenstermodell::umschalten`] weist einen Befehl auf
//! [`Bereich::Links`] trotzdem ausdruecklich ab, statt sich auf die fehlende
//! Belegung zu verlassen: die Zusage gehoert an die Stelle, die sie einloest.

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
    /// aufzaehlt, welche Bereiche Dateifenster sind**. [`Bereich::ist_beweglich`]
    /// leitet sich daraus ab, statt die Liste ein zweites Mal zu fuehren: ein
    /// Bereich teilt sich den frei werdenden Platz genau dann, wenn er ein
    /// Dateifenster ist, und das ist der Grund und nicht nur die Beobachtung.
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
    /// Aus dem Ausschluss folgt, dass [`bereichsbreiten`] unveraendert bleibt:
    /// hoechstens zwei feste Bereiche sind zugleich zu bedienen, wie vor der
    /// Editor-Runde. Der Editor bekommt einen fuenften Platz in den Feldern und
    /// keinen zweiten Rechenweg daneben.
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
    /// **Als Punktzahl gesetzt und nicht als Anteil gerechnet.** C1 der
    /// Editor-Runde verlangt fuer den Editor "rund ein Drittel der
    /// Fensterbreite"; ein Anteil an dieser Stelle waere ein zweiter Rechenweg
    /// neben [`bereichsbreiten`], und die Runde 1 traegt fuer alle vier
    /// bestehenden Bereiche ebenfalls Zahlen.
    ///
    /// Die 460 des Editors folgen aus den bestehenden vier: sie summieren sich
    /// zu 1280, ein Drittel davon sind rund 427. Mit ausgeblendeter Vorschau
    /// bleiben fuer die beiden Dateifenster 1280 minus 180 minus 460 gleich
    /// 640, also 320 je Fenster gegen ihre Mindestbreite von 240. Die Zahl gilt
    /// nur beim allerersten Start; danach gilt die Breite des Nutzers.
    pub const fn anfangsbreite(self) -> f64 {
        match self {
            Bereich::Lesezeichen => 180.0,
            Bereich::Links | Bereich::Rechts => 420.0,
            Bereich::Vorschau => 260.0,
            Bereich::Editor => 460.0,
        }
    }

    /// Ob dieser Bereich sich den frei werdenden Platz teilt.
    ///
    /// Die beiden Dateifenster tun es, die festen Bereiche nicht.
    ///
    /// **Abgeleitet und nicht aufgezaehlt.** Bis zur Editor-Runde stand hier
    /// `matches!(self, Links | Rechts)`, und ein neuer Bereich waere still als
    /// unbeweglich durchgegangen — mit der richtigen Antwort, aber aus dem
    /// falschen Grund. Danach stand hier eine eigene vollstaendige
    /// Fallunterscheidung, die dieselbe Zweiteilung ein zweites Mal aufschrieb.
    /// Seit dem 260809 fragt sie [`Bereich::seite`], und die Aufzaehlung steht
    /// nur noch dort; die Antwort kommt weiterhin aus einer Zeile, die jemand
    /// geschrieben hat, und der Uebersetzer haelt einen sechsten Bereich
    /// weiterhin an.
    ///
    /// [`bereichsbreiten`] fragt hier nach, statt die Liste ein drittes Mal zu
    /// fuehren.
    const fn ist_beweglich(self) -> bool {
        self.seite().is_some()
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
    pub fn sitzung(&self, fenster: [Fensterzustand; 2]) -> Sitzung {
        Sitzung {
            aktiv: self.aktiv,
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
    /// Die Nachfrage vor dem Verdraengen eines Editors mit ungesichertem Stand
    /// gehoert dem Aufrufer und kommt mit ihrem eigenen Schritt; diese Funktion
    /// baut die Sichtbarkeit.
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
    /// behaelt seine gespeicherte Breite, weil C7 verlangt, dass das
    /// Wiedereinblenden sie herstellt.
    ///
    /// **Die beiden Dateifenster bleiben unangetastet, solange nur eines von
    /// ihnen sichtbar ist.** Das sichtbare traegt dann den Platz des anderen
    /// mit, und diese Zahl zu uebernehmen hiesse, das Verhaeltnis der beiden
    /// zueinander mit einem Wert zu ueberschreiben, in dem das andere gar nicht
    /// vorkommt. Am 260804 im laufenden Buendel gemessen: das zweite
    /// Dateifenster kam nach dem Wiedereinblenden auf 269 Punkten statt auf
    /// seinen 406 zurueck, und das verfehlte die Zusage aus C7.
    pub fn breiten_uebernehmen(&mut self, gemessen: [f64; 5]) {
        let beide_dateifenster = self.sichtbar(Bereich::Links) && self.sichtbar(Bereich::Rechts);
        for bereich in Bereich::ALLE {
            if bereich.ist_beweglich() && !beide_dateifenster {
                continue;
            }
            if self.sichtbar(bereich) && gemessen[bereich.index()] > 0.0 {
                self.breite_setzen(bereich, gemessen[bereich.index()]);
            }
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

/// Verteilt den verfuegbaren Platz auf die fuenf Bereiche.
///
/// **Die eine Breitenregel des Programms.** Sie steht hier und nirgends sonst;
/// [`crate::appkit::aufteilung`] setzt nur um, was hier herauskommt.
///
/// Ein ausgeblendeter Bereich bekommt `0.0`; seine gespeicherte Breite bleibt
/// unangetastet und steht beim Wiedereinblenden wieder zur Verfuegung.
///
/// Die Regel in drei Saetzen. Die festen Bereiche bekommen der Reihe nach ihre
/// gespeicherte Breite, hoechstens aber so viel, dass fuer die Dateifenster
/// deren Mindestbreite bleibt. Was danach uebrig ist, teilen die sichtbaren
/// Dateifenster im Verhaeltnis ihrer gespeicherten Breiten. Ist nur eines
/// sichtbar, bekommt es alles.
///
/// **Die Reihenfolge von [`Bereich::ALLE`] ist dabei eine Zusage und kein
/// Zufall.** Die festen Bereiche werden in dieser Folge bedient, und wer vorn
/// steht, behaelt seine Wunschbreite, wenn es eng wird. Daraus faellt die
/// Festlegung des Nutzers vom 260808 ohne eine zweite Regel an: die
/// Lesezeichenleiste steht vor dem Editor, also weicht sie nicht, wenn beide
/// zugleich stehen, und die beiden Dateifenster ruecken zusammen. Erst wenn ihr
/// Mindestmass erreicht ist, gibt der Editor nach.
///
/// Dass die Dateifenster ueber ein **Verhaeltnis** und nicht ueber ihre
/// absolute Zahl gehen, ist die Antwort auf zwei Fragen zugleich: eine
/// Fenstervergroesserung kommt dort an, wo der Nutzer sie braucht, und die
/// beiden Tastenbefehle aus C7 verschieben die Trennlinie, statt eine Breite zu
/// setzen, die der naechste Bildaufbau wieder einkassiert.
pub fn bereichsbreiten(verfuegbar: f64, breiten: &Breiten, sichtbar: &Sichtbarkeit) -> [f64; 5] {
    let modell = Fenstermodell {
        aktiv: Fensterseite::Links,
        breiten: *breiten,
        sichtbar: *sichtbar,
    };
    let mut ergebnis = [0.0_f64; 5];

    // Was die sichtbaren Dateifenster mindestens brauchen.
    let mindestens_dateifenster: f64 = Bereich::ALLE
        .iter()
        .filter(|bereich| bereich.ist_beweglich() && modell.sichtbar(**bereich))
        .map(|bereich| bereich.mindestbreite())
        .sum();

    // Welche Bereiche fest stehen, sagt `ist_beweglich` und sonst niemand. Bis
    // zur Editor-Runde stand hier die Literalliste
    // `[Bereich::Lesezeichen, Bereich::Vorschau]` als zweite Aufzaehlung
    // daneben: ein fuenfter fester Bereich, der dort fehlte, haette dauerhaft
    // die Breite 0 bekommen, ohne dass der Uebersetzer etwas gesagt haette.
    let mut rest = verfuegbar;
    for bereich in Bereich::ALLE
        .into_iter()
        .filter(|bereich| !bereich.ist_beweglich() && modell.sichtbar(*bereich))
    {
        let gewuenscht = modell
            .breite(bereich)
            .unwrap_or_else(|| bereich.anfangsbreite())
            .max(bereich.mindestbreite());
        // Nie so viel, dass die Dateifenster unter ihr Mindestmass fallen.
        let hoechstens = (rest - mindestens_dateifenster).max(0.0);
        let breite = gewuenscht.min(hoechstens);
        ergebnis[bereich.index()] = breite;
        rest -= breite;
    }

    let beweglich: Vec<Bereich> = Bereich::ALLE
        .into_iter()
        .filter(|bereich| bereich.ist_beweglich() && modell.sichtbar(*bereich))
        .collect();
    let rest = rest.max(0.0);
    match beweglich.as_slice() {
        [eines] => ergebnis[eines.index()] = rest,
        [links, rechts] => {
            let wunsch_links = modell
                .breite(*links)
                .unwrap_or_else(|| links.anfangsbreite())
                .max(1.0);
            let wunsch_rechts = modell
                .breite(*rechts)
                .unwrap_or_else(|| rechts.anfangsbreite())
                .max(1.0);
            let anteil = wunsch_links / (wunsch_links + wunsch_rechts);
            let mut breite_links = rest * anteil;
            // Das Mindestmass gewinnt gegen das Verhaeltnis. Beide zugleich
            // sind nur zu halten, solange das Fenster breit genug ist, und
            // dafuer sorgt seine eigene Mindestgroesse.
            breite_links = breite_links
                .max(links.mindestbreite())
                .min((rest - rechts.mindestbreite()).max(links.mindestbreite()));
            ergebnis[links.index()] = breite_links;
            ergebnis[rechts.index()] = (rest - breite_links).max(0.0);
        }
        // Ohne sichtbares Dateifenster gibt es nichts zu verteilen. Der Fall
        // tritt nicht ein: `Fenstermodell::umschalten` weist ihn ab.
        _ => {}
    }
    ergebnis
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modell() -> Fenstermodell {
        Fenstermodell::aus_sitzung(&Sitzung::default())
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

    #[test]
    fn eine_ausgeblendete_breite_bleibt_erhalten() {
        let mut modell = modell();
        modell.breite_setzen(Bereich::Lesezeichen, 200.0);
        modell.umschalten(Bereich::Lesezeichen);
        let breiten = bereichsbreiten(1200.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Lesezeichen.index()], 0.0);
        assert_eq!(
            modell.breiten().lesezeichen,
            Some(200.0),
            "die gespeicherte Breite ueberlebt das Ausblenden"
        );

        modell.umschalten(Bereich::Lesezeichen);
        let breiten = bereichsbreiten(1200.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Lesezeichen.index()], 200.0);
    }

    #[test]
    fn der_frei_gewordene_platz_geht_an_die_dateifenster() {
        let modell = modell();
        let voll = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        let summe_voll: f64 = voll.iter().sum();
        assert!((summe_voll - 1400.0).abs() < 0.001, "{voll:?}");

        let mut ohne_vorschau = modell;
        ohne_vorschau.umschalten(Bereich::Vorschau);
        let jetzt = bereichsbreiten(
            1400.0,
            &ohne_vorschau.breiten(),
            &ohne_vorschau.sichtbarkeit(),
        );
        assert_eq!(jetzt[Bereich::Vorschau.index()], 0.0);
        assert!(
            jetzt[Bereich::Links.index()] > voll[Bereich::Links.index()],
            "das linke Dateifenster hat nichts vom frei gewordenen Platz bekommen"
        );
        assert!(jetzt[Bereich::Rechts.index()] > voll[Bereich::Rechts.index()]);
        let summe: f64 = jetzt.iter().sum();
        assert!((summe - 1400.0).abs() < 0.001, "{jetzt:?}");
    }

    #[test]
    fn ein_einziges_dateifenster_nimmt_die_ganze_breite() {
        let mut modell = modell();
        modell.umschalten(Bereich::Rechts);
        modell.umschalten(Bereich::Lesezeichen);
        modell.umschalten(Bereich::Vorschau);
        let breiten = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten, [0.0, 1400.0, 0.0, 0.0, 0.0]);
    }

    /// Der eingeblendete Editor bekommt seine Breite, und die Dateifenster
    /// teilen sich den Rest.
    ///
    /// Der Fall aus C1 der Editor-Runde: Editor sichtbar, Vorschau
    /// ausgeblendet. Die Zahlen sind die des Auslieferungszustands.
    #[test]
    fn der_eingeblendete_editor_bekommt_seine_breite_und_die_dateifenster_den_rest() {
        let mut modell = modell();
        modell.umschalten(Bereich::Vorschau);
        assert!(modell.umschalten(Bereich::Editor));
        assert!(modell.sichtbar(Bereich::Editor));

        let breiten = bereichsbreiten(1280.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Vorschau.index()], 0.0);
        assert_eq!(
            breiten[Bereich::Editor.index()],
            Bereich::Editor.anfangsbreite(),
            "{breiten:?}"
        );
        assert_eq!(
            breiten[Bereich::Lesezeichen.index()],
            Bereich::Lesezeichen.anfangsbreite(),
            "die Leiste steht vor dem Editor und weicht ihm nicht"
        );
        // Was Leiste und Editor uebrig lassen, teilen die beiden Dateifenster
        // im Verhaeltnis ihrer gleichen Wunschbreiten, also je zur Haelfte.
        assert!(
            (breiten[Bereich::Links.index()] - 320.0).abs() < 0.001,
            "{breiten:?}"
        );
        assert!(
            (breiten[Bereich::Rechts.index()] - 320.0).abs() < 0.001,
            "{breiten:?}"
        );
        let summe: f64 = breiten.iter().sum();
        assert!((summe - 1280.0).abs() < 0.001, "{breiten:?}");
    }

    /// Am engen Fenster gewinnt das Mindestmass der Dateifenster gegen die
    /// Wunschbreite des Editors.
    ///
    /// Die Reihenfolge von `Bereich::ALLE` entscheidet, wer nachgibt: die
    /// Leiste steht vorn und behaelt ihre Breite, der Editor steht hinten und
    /// bekommt, was uebrig bleibt.
    #[test]
    fn am_engen_fenster_gewinnt_das_mindestmass_der_dateifenster() {
        let mut modell = modell();
        modell.umschalten(Bereich::Vorschau);
        modell.umschalten(Bereich::Editor);

        let breiten = bereichsbreiten(900.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(
            breiten,
            [180.0, 240.0, 240.0, 0.0, 240.0],
            "der Editor gibt nach, die Leiste und die Dateifenster nicht"
        );
        assert_eq!(
            breiten[Bereich::Lesezeichen.index()],
            Bereich::Lesezeichen.anfangsbreite(),
            "{breiten:?}"
        );
        assert!(
            breiten[Bereich::Links.index()] >= Bereich::Links.mindestbreite(),
            "{breiten:?}"
        );
        assert!(
            breiten[Bereich::Rechts.index()] >= Bereich::Rechts.mindestbreite(),
            "{breiten:?}"
        );
        assert!(
            breiten[Bereich::Editor.index()] < Bereich::Editor.anfangsbreite(),
            "der Editor haette seine Wunschbreite auf Kosten der Dateifenster behalten: {breiten:?}"
        );
        let summe: f64 = breiten.iter().sum();
        assert!((summe - 900.0).abs() < 0.001, "{breiten:?}");
    }

    /// Der ausgeblendete Editor bekommt 0 und behaelt seine gespeicherte
    /// Breite.
    ///
    /// Dieselbe Zusage aus C7, die schon fuer Leiste und Vorschau gilt: das
    /// Wiedereinblenden stellt die vorherige Breite her.
    #[test]
    fn der_ausgeblendete_editor_behaelt_seine_gespeicherte_breite() {
        let mut modell = modell();
        modell.umschalten(Bereich::Vorschau);
        modell.umschalten(Bereich::Editor);
        modell.breite_setzen(Bereich::Editor, 500.0);

        modell.umschalten(Bereich::Editor);
        assert!(!modell.sichtbar(Bereich::Editor));
        let breiten = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Editor.index()], 0.0, "{breiten:?}");
        assert_eq!(modell.breiten().editor, Some(500.0));

        modell.umschalten(Bereich::Editor);
        let breiten = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert_eq!(breiten[Bereich::Editor.index()], 500.0, "{breiten:?}");
    }

    /// Die Lesezeichenleiste weicht dem Editor nicht; die Dateifenster ruecken
    /// zusammen.
    ///
    /// Die Festlegung des Nutzers vom 260808. Sie faellt aus der bestehenden
    /// Regel an, weil `Bereich::ALLE` die Leiste vor den Editor stellt; eine
    /// zweite Breitenregel entsteht dafuer nicht.
    #[test]
    fn die_leiste_weicht_dem_editor_nicht() {
        let mut mit_leiste = modell();
        mit_leiste.umschalten(Bereich::Vorschau);
        mit_leiste.umschalten(Bereich::Editor);
        let offen = bereichsbreiten(1280.0, &mit_leiste.breiten(), &mit_leiste.sichtbarkeit());

        let mut ohne_leiste = modell();
        ohne_leiste.umschalten(Bereich::Vorschau);
        ohne_leiste.umschalten(Bereich::Editor);
        ohne_leiste.umschalten(Bereich::Lesezeichen);
        let zu = bereichsbreiten(1280.0, &ohne_leiste.breiten(), &ohne_leiste.sichtbarkeit());

        // Die Zahlen ausgeschrieben, damit die Zusage nachlesbar ist und nicht
        // nur als Ungleichung dasteht. 1280 ist die Summe der vier
        // Anfangsbreiten der Runde 1.
        assert_eq!(offen, [180.0, 320.0, 320.0, 0.0, 460.0], "Leiste offen");
        assert_eq!(zu, [0.0, 410.0, 410.0, 0.0, 460.0], "Leiste geschlossen");

        assert_eq!(
            offen[Bereich::Editor.index()],
            zu[Bereich::Editor.index()],
            "die Leiste ist zu Lasten des Editors gegangen: offen {offen:?}, zu {zu:?}"
        );
        assert!(
            zu[Bereich::Links.index()] > offen[Bereich::Links.index()],
            "die Dateifenster haben den Platz der Leiste nicht bekommen"
        );
        assert!(zu[Bereich::Rechts.index()] > offen[Bereich::Rechts.index()]);
    }

    /// Welche Bereiche fest stehen, sagt `ist_beweglich` und sonst niemand.
    ///
    /// Die Probe zu Befund 6 des Editor-Plans: bis dahin fuehrte
    /// `bereichsbreiten` die festen Bereiche als Literalliste ein zweites Mal,
    /// und ein weiterer fester Bereich haette dort still gefehlt und dauerhaft
    /// die Breite 0 bekommen. Solange das Fenster breit genug ist, bekommt
    /// jeder feste, sichtbare Bereich seine Breite — hergeleitet aus
    /// `ist_beweglich` und nicht aus einer Aufzaehlung im Rechenweg.
    #[test]
    fn jeder_feste_bereich_bekommt_seine_breite_ohne_zweite_aufzaehlung() {
        let modell = modell();
        let breiten = bereichsbreiten(1600.0, &modell.breiten(), &modell.sichtbarkeit());
        for bereich in Bereich::ALLE {
            if bereich.ist_beweglich() || !modell.sichtbar(bereich) {
                continue;
            }
            assert_eq!(
                breiten[bereich.index()],
                bereich.anfangsbreite(),
                "{bereich:?} ist fest und sichtbar, bekommt aber nicht seine Breite"
            );
        }
    }

    /// Beweglich ist ein Bereich genau dann, wenn er ein Dateifenster ist, und
    /// sein Gegenueber kommt aus [`Fensterseite::andere`].
    ///
    /// Die Probe zum Befund vom 260808: `breite_aendern` fuehrte den Partner
    /// ueber einen `match` mit `_ => Bereich::Links` und hat den fuenften
    /// Bereich stumm aufgenommen. Seit dem 260809 gibt es die Zuordnung nur in
    /// `Bereich::seite`, und diese Probe haelt fest, dass sie mit
    /// `Bereich::von_seite` zusammenpasst.
    #[test]
    fn beweglich_ist_genau_ein_dateifenster_und_die_zuordnung_laeuft_in_beide_richtungen() {
        for bereich in Bereich::ALLE {
            assert_eq!(
                bereich.ist_beweglich(),
                bereich.seite().is_some(),
                "{bereich:?}"
            );
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
                .filter(|bereich| bereich.ist_beweglich())
                .count(),
            Fensterseite::ALLE.len(),
            "es gibt genau so viele bewegliche Bereiche wie Dateifenster"
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

    #[test]
    fn kein_bereich_faellt_unter_sein_mindestmass() {
        let modell = modell();
        // Enger, als die vier Mindestbreiten zusammen erlauben.
        let breiten = bereichsbreiten(500.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            breiten[Bereich::Links.index()] >= Bereich::Links.mindestbreite(),
            "{breiten:?}"
        );
        assert!(
            breiten[Bereich::Rechts.index()] >= Bereich::Rechts.mindestbreite(),
            "{breiten:?}"
        );
    }

    /// Der Tastenbefehl bewegt die Trennlinie um genau einen Schritt.
    ///
    /// Die Zahl im Modell und die Zahl auf dem Schirm muessen dieselbe sein.
    /// Bevor `breite_aendern` das andere Dateifenster mitzog, waren es 13
    /// Punkte statt 40, gemessen am 260804 im laufenden Buendel.
    #[test]
    fn der_tastenbefehl_verschiebt_die_trennlinie_um_genau_einen_schritt() {
        let mut modell = modell();
        modell.breiten_uebernehmen(bereichsbreiten(
            1400.0,
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());

        modell.breite_aendern(Bereich::Links, BREITENSCHRITT);
        let nachher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
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
        let zurueck = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!((zurueck[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001);
    }

    /// Das wiedereingeblendete Dateifenster kommt auf seiner alten Breite
    /// zurueck.
    ///
    /// Der Fall, der die Zusage aus C7 am 260804 im laufenden Buendel verfehlt
    /// hat: das sichtbare Dateifenster traegt den Platz des ausgeblendeten mit,
    /// und diese Zahl darf nicht als sein Wunsch in das Modell zurueckfliessen.
    #[test]
    fn das_wiedereingeblendete_dateifenster_hat_wieder_seine_alte_breite() {
        let mut modell = modell();
        modell.breiten_uebernehmen(bereichsbreiten(
            1400.0,
            &modell.breiten(),
            &modell.sichtbarkeit(),
        ));
        let vorher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());

        modell.umschalten(Bereich::Rechts);
        // Der Bildaufbau schreibt die gemessenen Breiten zurueck, so wie es der
        // Sitzungsabgleich und jeder Breitenbefehl tun.
        let alleine = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        modell.breiten_uebernehmen(alleine);
        assert!(
            alleine[Bereich::Links.index()] > vorher[Bereich::Links.index()],
            "das linke Dateifenster hat den Platz nicht uebernommen"
        );

        modell.umschalten(Bereich::Rechts);
        let nachher = bereichsbreiten(1400.0, &modell.breiten(), &modell.sichtbarkeit());
        assert!(
            (nachher[Bereich::Rechts.index()] - vorher[Bereich::Rechts.index()]).abs() < 0.001,
            "vorher {vorher:?}, nachher {nachher:?}"
        );
        assert!((nachher[Bereich::Links.index()] - vorher[Bereich::Links.index()]).abs() < 0.001);
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

        let sitzung = modell.sitzung(Sitzung::default().fenster);
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
