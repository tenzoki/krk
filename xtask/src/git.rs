//! Der eine Zugang zu `git`.
//!
//! **Hier steht der einzige Prozessaufruf von `git` im ganzen Baum.** Die Probe
//! `xtask_ruft_git_an_genau_einer_stelle` in `release` haelt die Zahl auf eins.
//! Bis zum 260813 stand der Aufruf in `release`, weil `release` der einzige
//! Abnehmer war; seit `version` einen Stand eintraegt und taggt, sind es zwei,
//! und der Aufruf ist an die Stelle gewandert, die beide gemeinsam haben. Eine
//! zweite waere die zweite Wahrheit darueber, wie tief das Bauwerkzeug in den
//! Zustand des Arbeitsbaums schaut — und seit das Werkzeug auch schreibt, die
//! zweite Wahrheit darueber, was es schreiben darf.
//!
//! **Seit dem 260821 kommt keine nackte Wortliste mehr hier an.** [`rufen`]
//! nimmt einen [`Auftrag`], und `Auftrag` ist die vollstaendige Aufzaehlung
//! jedes Kommandos, das dieses Werkzeug an `git` reicht. Wer ein neues braucht,
//! kann es nicht danebenbauen: er bekommt es nur durch diese Tuer, und die Tuer
//! ist eine Variante. [`Auftrag::wortplaetze`] und [`Auftrag::wirkung`] sind
//! beide vollstaendige Fallunterscheidungen ohne Auffangzweig, also haelt der
//! Uebersetzer eine neue Variante an, bis sie ihre Woerter genannt und sich als
//! lesend oder schreibend eingeordnet hat — dieselbe Bauart, die dieses Projekt
//! fuer `Wirkungsbereich`, `Bereich` und `Fokus` fuehrt. [`Auftrag::worte`]
//! leitet sich aus der ersten ab und zaehlt die Varianten nicht ein zweites
//! Mal.
//!
//! **Bis dahin stand die Aufsicht daneben statt auf dem Weg.** Sie zaehlte drei
//! Bauer namentlich auf, und ein vierter — `git tag --list` — stand schon
//! daneben, ohne dass sie ihn las. Was eine Aufzaehlung von Namen nicht kann,
//! ist die Zusage tragen, dass sie vollstaendig ist. Sie steht deshalb jetzt in
//! [`aufsichtsbefund`], und [`rufen`] ruft sie vor jedem Prozessaufruf: gelesen
//! wird die Liste, die wirklich hinausgeht, und nicht die, an die jemand
//! gedacht hat.
//!
//! **Noch am selben Tag ging sie einen Schritt weiter: sie liest jetzt Plaetze
//! und nicht mehr freie Woerter.** [`Auftrag::wortplaetze`] sagt je Variante,
//! welche Woerter fest dastehen und welche Plaetze der Aufrufer belegt; jeder
//! belegte Platz nennt die [`Gestalt`], die er tragen darf. Davor las die
//! Aufsicht ein flaches Wortfeld und musste raten, ob ein Wort ein Schalter,
//! ein Wert, ein Refspec oder ein Pfad ist. Zwei Loecher kamen daraus, beide an
//! einem Wegwerf-Verzeichnis nachgemessen: `:refs/heads/feature` loescht einen
//! Zweig auf der Gegenseite und traegt dabei keine einzige Marke, und `git`
//! nimmt `--del` als `--delete` an, was ein Vergleich auf Gleichheit nicht
//! faengt. Beide landeten an demselben Platz — dem Verweis eines `push` —, und
//! dort steht jetzt eine Gestalt, die genau eine Form zulaesst. Niemand muss
//! eine Marke verbieten, an die er nicht gedacht hat.
//!
//! **Wie stark die Zusage danach ist.** *Der Uebersetzer haelt*, dass jedes
//! Kommando eine Variante ist, dass jede Variante ihre Woerter und ihre Wirkung
//! nennt und dass jedes Wort entweder fest dasteht oder ein Platz mit einer
//! Gestalt ist. *Die Aufsicht auf dem Weg haelt* dreierlei, und die drei sind
//! verschieden stark:
//!
//! 1. An einem Platz, dessen Wert `git` als Option oder als Verweis liest —
//!    einem Tagnamen, einem Tagverweis —, haelt sie eine Gestalt, die genau
//!    eine Form zulaesst. Das ist eine Erlaubnisliste, und sie ist
//!    vollstaendig: weder ein Doppelpunkt noch eine Marke, abgekuerzt oder
//!    nicht, hat in `refs/tags/v<zahl>` Platz.
//! 2. An einem Platz, dessen Wert `git` gar nicht als eigenes Wort liest —
//!    einer Meldung hinter `-m`, einem Pfad hinter `--` —, haelt sie, dass er
//!    wirklich dort steht, und darueber hinaus nur das Grobe: nicht leer, keine
//!    Steuerzeichen, kein Ausstieg aus dem Arbeitsbaum. **Hier traegt die
//!    Stellung und nicht die Gestalt**, und deshalb darf eine Meldung wie eine
//!    Marke aussehen.
//! 3. An einem festen Wort haelt sie nur, dass es keine bekannte Marke ist.
//!    Das ist eine Verbotsliste und wird nie beweisbar vollstaendig.
//!
//! *Allein eine Probe haelt*, dass kein zweiter Prozessaufruf an [`rufen`]
//! vorbeigeht: `xtask_ruft_git_an_genau_einer_stelle` zaehlt im ganzen Baum den
//! Aufruf von `Command` mit dem festen Pfad zu `git` und laesst ihn genau
//! einmal zu — der Uebersetzer haelt hier gar nichts, und einen Aufruf, dessen
//! Programmname aus einer Variablen kaeme, saehe auch die Probe nicht.
//!
//! **Dass an einem festen Wort eine Verbotsliste steht, ist kein Rest, sondern
//! die Einteilung selbst.** Ein belegter Platz nimmt einen Wert von aussen
//! entgegen, und was von aussen kommt, ist die Gefahr. Ein festes Wort schreibt
//! dagegen der hin, der eine Variante hinzufuegt, und er sieht es beim
//! Hinschreiben; [`MARKEN`] ist dort eine zweite Gelegenheit hinzusehen und
//! keine Zusage. Wer sie fuer eine haelt, liest sie staerker, als sie ist.
//!
//! **Die Gestalt eines Tagnamens steht nicht hier, sondern bei dem, der sie
//! festlegt:** [`Gestalt`] ruft dafuer `version::versionszahl_pruefen`. Bis zum
//! 260821 war jene Pruefung die Sicherung, die in Wahrheit trug — sie war der
//! Grund, aus dem kein Doppelpunkt und keine Marke je in einen Refspec dieses
//! Werkzeugs geriet —, waehrend die Aufsicht nichts von ihr wusste. Jetzt ist
//! sie die Aufsicht. Eine zweite Vorschrift darueber, wie eine Versionszahl
//! dieses Projekts aussieht, waere genau der Fehler, den der Doc-Kommentar
//! jener Funktion benennt.
//!
//! **Lesen und Schreiben stehen hier verschieden da, und das ist Absicht.** Die
//! Unterscheidung traegt [`Wirkung`], und sie ist nicht bloss Beschriftung: die
//! Aufsicht laesst zu einer lesenden Frage nur die lesenden Unterbefehle durch
//! und verlangt bei `tag` die Marke, die aus dem Anlegen eine Frage macht.
//! `git tag v0.2.0` als Frage getarnt kommt damit nicht hinaus.
//!
//! Die zwei kleinen Lesehilfen [`geaenderte_dateien`] und [`tag_steht`] stehen
//! ebenfalls hier und nicht bei ihren Abnehmern: `release` und `version`
//! stellen dieselben zwei Fragen, und zwei Auslegungen derselben Ausgabe waeren
//! zwei Antworten darauf, was ein sauberer Arbeitsbaum ist.

use std::path::Path;
use std::process::Command;

use crate::Abbruch;
use crate::version::versionszahl_pruefen;

/// Was ein Auftrag am Zustand aendert.
///
/// Jede Variante von [`Auftrag`] ordnet sich hier ein, und die Einordnung ist
/// nicht Beschriftung, sondern wird gelesen: [`aufsichtsbefund`] laesst zu
/// [`Wirkung::Liest`] nur [`LESENDE`] durch und zu [`Wirkung::Schreibt`] nur
/// [`SCHREIBENDE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Wirkung {
    /// Der Auftrag fragt und laesst den Baum, wie er ist.
    Liest,
    /// Der Auftrag aendert etwas — am Verzeichnis oder auf der Gegenseite.
    Schreibt,
}

/// Die Gestalt, die ein belegter Wortplatz tragen darf.
///
/// **Eine Erlaubnisliste und der Kern der Aufsicht.** Ein belegter Platz nimmt
/// seinen Wert von aussen entgegen; geprueft wird deshalb nicht, was der Wert
/// nicht sein darf, sondern ob er ist, was an seiner Stelle vorgesehen ist. Ein
/// Doppelpunkt, eine Marke und jede abgekuerzte Marke scheitern daran
/// gleichermassen, ohne dass eine von ihnen irgendwo als verboten dastuende.
///
/// Vier Gestalten, weil dieses Werkzeug vier Arten von Werten hinausreicht.
/// Eine fuenfte hinzuzufuegen heisst, sie hier zu beschreiben — und wer es tut,
/// steht vor derselben Frage wie der, der [`SCHREIBENDE`] erweitert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Gestalt {
    /// Ein Tagname dieses Projekts: `v` und danach eine Versionszahl.
    ///
    /// Er steht an einer Stelle, an der `git` Optionen liest — `git tag -d x`
    /// loescht —, also darf er gar nicht erst wie eine Option aussehen koennen.
    Tagname,
    /// Der vollstaendige Verweis auf einen Tag: [`TAGRAUM`] und ein Tagname.
    ///
    /// Er steht als Refspec eines `push`, und ein Refspec ist die eine Stelle,
    /// an der ein Wort ganz ohne Marke auf der Gegenseite schreibt oder
    /// loescht. Der Doppelpunkt, mit dem das geschieht, kommt in dieser Gestalt
    /// nicht vor.
    Tagverweis,
    /// Die Meldung eines Eintrags.
    ///
    /// Sie darf jedes druckbare Zeichen tragen, auch einen fuehrenden Strich:
    /// sie steht hinter `-m`, das sie als Wert aufnimmt, und das prueft
    /// [`stellungsbefund`]. Die alte Aufsicht las sie als haette sie eine Marke
    /// sein koennen und hielt `-m "-a"` an; eine Aufsicht, die Plaetze kennt,
    /// tut das nicht.
    Meldung,
    /// Ein Pfad im Arbeitsbaum, hinter dem Trenner `--`.
    Pfad,
}

/// Der Namensraum, in dem die Tags dieses Projekts stehen.
const TAGRAUM: &str = "refs/tags/";

/// Der Trenner, hinter dem `git` keine Option mehr liest.
const TRENNER: &str = "--";

impl Gestalt {
    /// Warum dieser Wert an einem Platz dieser Gestalt nicht stehen darf, oder
    /// `None`.
    ///
    /// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine fuenfte Gestalt
    /// haelt hier den Bau an.
    fn befund(self, wert: &str) -> Option<String> {
        match self {
            Gestalt::Tagname => tagnamenbefund(wert),
            Gestalt::Tagverweis => match wert.strip_prefix(TAGRAUM) {
                Some(name) => tagnamenbefund(name),
                None => Some(format!(
                    "der Verweis {wert} steht nicht unter {TAGRAUM} und benennt damit keinen Tag"
                )),
            },
            Gestalt::Meldung => {
                if wert.is_empty() {
                    return Some("die Eintragsmeldung ist leer".to_owned());
                }
                steuerzeichenbefund("die Eintragsmeldung", wert)
            }
            Gestalt::Pfad => {
                if wert.is_empty() {
                    return Some("ein Pfad des Eintrags ist leer".to_owned());
                }
                if wert.starts_with('/') {
                    return Some(format!(
                        "der Pfad {wert} ist absolut und zeigt damit aus dem Arbeitsbaum hinaus"
                    ));
                }
                if wert.split('/').any(|teil| teil == "..") {
                    return Some(format!("der Pfad {wert} steigt mit .. aus dem Arbeitsbaum"));
                }
                steuerzeichenbefund("der Pfad", wert)
            }
        }
    }
}

/// Warum dieser Name kein Tagname dieses Projekts ist, oder `None`.
///
/// **Die Zahl prueft `version::versionszahl_pruefen` und nicht diese Datei.**
/// Wie eine Versionszahl dieses Projekts aussieht, ist dort festgelegt und
/// dorthin gehoert es; eine zweite Vorschrift daneben waere eine zweite
/// Antwort auf dieselbe Frage. Was dieses Werkzeug taggt, ist ausschliesslich
/// ein Stand mit einer Versionszahl — wer es anders braucht, aendert die
/// Gestalt hier und weiss dann, was er tut.
fn tagnamenbefund(name: &str) -> Option<String> {
    let Some(zahl) = name.strip_prefix('v') else {
        return Some(format!(
            "der Tagname {name} faengt nicht mit v an und ist damit keiner dieses Projekts"
        ));
    };
    versionszahl_pruefen(zahl)
        .err()
        .map(|grund| format!("der Tagname {name} traegt keine Versionszahl: {grund}"))
}

/// Warum dieser Wert ein Steuerzeichen traegt, oder `None`.
fn steuerzeichenbefund(was: &str, wert: &str) -> Option<String> {
    let zeichen = wert.chars().find(|zeichen| zeichen.is_control())?;
    Some(format!(
        "{was} {wert:?} traegt das Steuerzeichen {zeichen:?}"
    ))
}

/// Ein Wort hinter `git`, mit seinem Platz.
///
/// **Der Unterschied zwischen aussen und innen.** Ein festes Wort schreibt der
/// hin, der eine Variante von [`Auftrag`] hinzufuegt; es steht im Quelltext und
/// ist beim Hinschreiben zu sehen. Ein Platz nimmt seinen Wert vom Aufrufer
/// entgegen. Die Aufsicht behandelt beide verschieden, und der Modulkopf sagt,
/// wie stark sie dabei je ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Wort<'a> {
    /// Ein Wort, das die Variante selbst mitbringt.
    Fest(&'a str),
    /// Ein Platz, den der Aufrufer belegt, samt der Gestalt, die er tragen darf.
    Platz(Gestalt, &'a str),
}

impl<'a> Wort<'a> {
    /// Das Wort, wie es hinausgeht.
    #[must_use]
    fn wert(&self) -> &'a str {
        match self {
            Wort::Fest(text) | Wort::Platz(_, text) => text,
        }
    }
}

/// Jedes Kommando, das dieses Werkzeug an `git` reicht.
///
/// **Die Aufzaehlung ist die Aufsicht.** [`rufen`] nimmt nichts anderes
/// entgegen, also gibt es keine Liste, die an dieser Aufzaehlung vorbei bei
/// `git` ankommt. Ein achter Auftrag ist eine achte Variante, und beide
/// Fallunterscheidungen darunter haben keinen Auffangzweig: der Bau steht, bis
/// die neue Variante ihre Woerter genannt und sich eingeordnet hat.
///
/// **Die Bauer standen bis zum 260821 bei ihren Abnehmern**, `git tag` und
/// `git commit` in `version`, `git push` in `veroeffentlichung`. Sie stehen
/// jetzt hier, weil die Aufsicht hier steht; was bei den Abnehmern bleibt, ist
/// die Entscheidung, den Auftrag zu erteilen, und deren Begruendung. Die
/// Begruendung der einzelnen Woerter steht an der Variante, die sie traegt.
pub(crate) enum Auftrag<'a> {
    /// `git rev-parse --git-dir`: liegt ueberhaupt ein Git-Verzeichnis vor?
    ///
    /// Steht getrennt, damit die Antwort darauf nicht am Wortlaut einer
    /// Fehlermeldung der anderen Fragen haengt.
    Verzeichnis,
    /// `git tag --points-at HEAD`: welche Tags stehen auf HEAD?
    ///
    /// `--points-at` fragt allein; `git tag` legt erst dann einen Tag an, wenn
    /// ein Name dabeisteht. Annotierte und leichte Tags stehen in dieser
    /// Ausgabe gleich, und das ist die Zusage aus C3.3: gefragt ist, welcher
    /// Name auf HEAD steht, und nicht, wie er entstanden ist.
    TagsAufHead,
    /// `git status --porcelain --untracked-files=no`: welche verfolgten Dateien
    /// sind geaendert?
    ///
    /// `--porcelain` meldet vorgemerkte und nicht vorgemerkte Aenderungen in
    /// derselben Form und fuehrt geloeschte verfolgte Dateien mit;
    /// `--untracked-files=no` haelt unbeachtete Dateien draussen, wie es der
    /// Entscheid vom 260813-1010 verlangt.
    ///
    /// **Ohne Pfadfilter, und das ist eine Festlegung.** Gezaehlt wird das
    /// ganze Verzeichnis. Eine Liste der bauwirksamen Ordner muesste jemand
    /// pflegen, und sie zu ergaenzen zu vergessen ist die zweite Art, eine
    /// Pruefung im Vorbeigehen zu verlieren — dieselbe Erwaegung, die schon bei
    /// `release::GRENZWURZEL` steht. Was der Verzicht kostet, steht in
    /// `shared/issues/260813-1515_*_die-auslieferungspruefung-schlaegt-nach-jeder-agentensitzung-an-weil-vier-werkbankdateien-verfolgt-sind.md`;
    /// beide Abnehmer zaehlen die betroffenen Dateien deshalb nicht nur, sie
    /// nennen sie beim Namen.
    Stand,
    /// `git tag --list <name>`: steht dieser Tag irgendwo im Verzeichnis?
    ///
    /// `--list` ist das, was den Aufruf zu einer Frage macht. Ohne die Marke
    /// legte derselbe Aufruf den Tag an, und die Aufsicht laesst ihn deshalb
    /// als [`Wirkung::Liest`] nur mit ihr durch.
    Tagliste(&'a str),
    /// `git tag <name>`: der leichte Tag auf HEAD.
    ///
    /// Leicht und nicht annotiert, wie `v0.1.0` vom 260813, den der Nutzer von
    /// Hand gesetzt hat. Die Frage nach den Tags auf HEAD unterscheidet die
    /// beiden Arten nicht; zwei Arten nebeneinander waeren trotzdem zwei
    /// Schreibweisen fuer dieselbe Sache.
    ///
    /// **Ohne `-f`.** Ein bestehender Tag laesst diesen Aufruf scheitern, und
    /// das ist die Absicht: `version::vorhaben_bestimmen` hat den Fall vorher
    /// entschieden.
    TagSetzen(&'a str),
    /// `git commit --only -m <meldung> -- <dateien>`: der Eintrag einer
    /// benannten Aenderung.
    ///
    /// **`--only` mit Pfaden und kein `git add`.** Der Eintrag entsteht aus dem
    /// Stand der genannten Dateien im Arbeitsbaum, ohne die Vormerkung
    /// anzufassen. Das hat zwei Wirkungen: ein gescheiterter Eintrag laesst
    /// nichts Vorgemerktes zurueck, das jemand wegraeumen muesste, und der Lauf
    /// greift nicht auf die gemeinsame Vormerkung zu, an der in diesem Projekt
    /// auch Agenten arbeiten.
    ///
    /// **Die Dateien kommen herein und stehen nicht hier.** Welche zwei der
    /// Versionsschritt eintraegt, ist seine Sache und steht als
    /// `version::EINGETRAGENE` dort; hierher gehoert allein die Form des
    /// Kommandos.
    Eintrag {
        /// Die Eintragsmeldung, ein Wort hinter `-m`.
        meldung: &'a str,
        /// Die Pfade hinter dem Trenner `--`.
        dateien: &'a [&'a str],
    },
    /// `git push origin HEAD <verweis>`: das Schieben zur Gegenseite.
    ///
    /// Genau vier Woerter. **Geschoben wird `HEAD` und nicht der Zweigname**,
    /// damit keine vierte lesende Frage nach `git` noetig wird; `HEAD` als
    /// Quellreferenz schreibt auf der Gegenseite in den Zweig gleichen Namens.
    ///
    /// **Es ist ein Auftrag und nicht zwei.** Zwei haetten einen
    /// Zwischenzustand, in dem der Zweig oben steht und der Tag nicht; und eine
    /// Liste, die beide Referenzen traegt, ist an einer Stelle nachzusehen
    /// statt an zweien.
    ///
    /// Der Verweis kommt fertig herein und wird nicht hier gefuegt: gefuegt
    /// wird in `veroeffentlichung::tagverweis`, bei dem, der schiebt.
    Schub {
        /// Der vollstaendige Verweis auf den Tag, `refs/tags/<name>`.
        verweis: &'a str,
    },
}

impl<'a> Auftrag<'a> {
    /// Die Woerter hinter `git`, jedes mit seinem Platz.
    ///
    /// **Die eine Stelle, an der die sieben Varianten ihre Woerter nennen.**
    /// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine achte Variante
    /// haelt hier den Bau an, und sie kommt nicht davon, ohne je Wort gesagt zu
    /// haben, ob es fest dasteht oder ein Platz ist. [`Auftrag::worte`] leitet
    /// sich hieraus ab und zaehlt die Varianten nicht ein zweites Mal.
    #[must_use]
    pub(crate) fn wortplaetze(&self) -> Vec<Wort<'a>> {
        match self {
            Auftrag::Verzeichnis => vec![Wort::Fest("rev-parse"), Wort::Fest("--git-dir")],
            Auftrag::TagsAufHead => vec![
                Wort::Fest("tag"),
                Wort::Fest("--points-at"),
                Wort::Fest("HEAD"),
            ],
            Auftrag::Stand => vec![
                Wort::Fest("status"),
                Wort::Fest("--porcelain"),
                Wort::Fest("--untracked-files=no"),
            ],
            Auftrag::Tagliste(name) => vec![
                Wort::Fest("tag"),
                Wort::Fest("--list"),
                Wort::Platz(Gestalt::Tagname, name),
            ],
            Auftrag::TagSetzen(name) => {
                vec![Wort::Fest("tag"), Wort::Platz(Gestalt::Tagname, name)]
            }
            Auftrag::Eintrag { meldung, dateien } => {
                let mut worte = vec![
                    Wort::Fest("commit"),
                    Wort::Fest("--only"),
                    Wort::Fest("-m"),
                    Wort::Platz(Gestalt::Meldung, meldung),
                    Wort::Fest(TRENNER),
                ];
                worte.extend(
                    dateien
                        .iter()
                        .map(|datei| Wort::Platz(Gestalt::Pfad, datei)),
                );
                worte
            }
            Auftrag::Schub { verweis } => vec![
                Wort::Fest("push"),
                Wort::Fest("origin"),
                Wort::Fest("HEAD"),
                Wort::Platz(Gestalt::Tagverweis, verweis),
            ],
        }
    }

    /// Die Woerter hinter `git`, in ihrer Reihenfolge und ohne ihre Plaetze.
    ///
    /// Fuer den Prozessaufruf und fuer jede Meldung, die das Kommando
    /// ausschreibt. Abgeleitet aus [`Auftrag::wortplaetze`]: eine zweite
    /// Aufzaehlung der Varianten waere eine zweite Wahrheit darueber, was
    /// hinausgeht.
    #[must_use]
    pub(crate) fn worte(&self) -> Vec<&'a str> {
        self.wortplaetze().iter().map(Wort::wert).collect()
    }

    /// Ob dieser Auftrag liest oder schreibt.
    ///
    /// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine neue Variante
    /// haelt auch hier den Bau an und muss sich einordnen. Die Einordnung wird
    /// gelesen, siehe [`aufsichtsbefund`].
    #[must_use]
    pub(crate) fn wirkung(&self) -> Wirkung {
        match self {
            Auftrag::Verzeichnis | Auftrag::TagsAufHead | Auftrag::Stand | Auftrag::Tagliste(_) => {
                Wirkung::Liest
            }
            Auftrag::TagSetzen(_) | Auftrag::Eintrag { .. } | Auftrag::Schub { .. } => {
                Wirkung::Schreibt
            }
        }
    }
}

/// Die Unterbefehle, die eine lesende Frage tragen darf.
///
/// Eine Erlaubnisliste und keine Verbotsliste: sie faellt zur sicheren Seite.
/// Ein Unterbefehl, an den niemand gedacht hat, steht nicht darin und kommt
/// deshalb nicht durch, statt durchzukommen, weil niemand ihn verboten hat.
const LESENDE: [&str; 3] = ["rev-parse", "tag", "status"];

/// Die Unterbefehle, die ein schreibender Auftrag tragen darf.
///
/// Ebenfalls eine Erlaubnisliste. `reset`, `clean`, `checkout`, `restore` und
/// `stash` stehen nicht darin und kommen deshalb nicht vor, ohne dass sie
/// jemand einzeln verbieten muesste.
const SCHREIBENDE: [&str; 3] = ["tag", "commit", "push"];

/// Die langen Marken, die einem Kommando Reichweite oder Gewalt hinzufuegen.
///
/// **Eine Verbotsliste, und sie steht mit Vorbehalt da.** Sie wird allein auf
/// feste Woerter angewandt, also auf das, was jemand beim Hinzufuegen einer
/// Variante selbst hinschreibt; sie ist dort eine zweite Gelegenheit
/// hinzusehen und keine Zusage. An einem belegten Platz steht statt ihrer die
/// [`Gestalt`], und die ist eine Erlaubnisliste.
///
/// **Verglichen wird nicht auf Gleichheit.** `git` nimmt eine lange Marke auch
/// abgekuerzt entgegen, solange die Abkuerzung eindeutig ist: `--del` loescht
/// einen Tag, `--ame` aendert den letzten Eintrag, `--mirr` und `--al` kommen
/// durch den Optionszerleger. Ein Vergleich auf Gleichheit sah keines dieser
/// Woerter (Durchsicht 260821-1432, A2). [`verwandte_marke`] haelt deshalb
/// jedes Wort an, von dem ein Eintrag der Anfang ist oder das der Anfang eines
/// Eintrags ist, nachdem ein Anhang hinter `=` abgetrennt ist. Damit deckt
/// `--force` seine ganze Familie — `--force-with-lease`, `--force-if-includes`,
/// `--force-with-lease=<verweis>` —, und `--exec` und `--receive-pack` decken
/// ihre Formen mit Wert.
const MARKEN: [&str; 10] = [
    "--tags",
    "--follow-tags",
    "--all",
    "--mirror",
    "--delete",
    "--prune",
    "--amend",
    "--force",
    "--exec",
    "--receive-pack",
];

/// Weitere lange Marken, die eine Pruefung uebergehen.
///
/// Getrennt von [`MARKEN`] allein wegen der Meldung: wer sie liest, soll den
/// Unterschied zwischen mehr Reichweite und einer uebergangenen Pruefung
/// sehen. Verglichen wird nach derselben Regel.
const UEBERGEHENDE: [&str; 1] = ["--no-verify"];

/// Die Buchstaben, die in einer kurzen Marke Gewalt oder Reichweite bedeuten.
///
/// Gemeint sind die Gruppen mit einem Strich: `-f`, `-d`, `-a` — und `-fd`,
/// denn `git` nimmt kurze Marken auch zusammengezogen entgegen. Geprueft wird
/// deshalb Buchstabe fuer Buchstabe und nicht das Wort als Ganzes; ein
/// Vergleich auf Gleichheit liesse `-fd` durch. `-m` bleibt zulaessig, weil `m`
/// nicht darunter steht.
const GEWALTBUCHSTABEN: [char; 3] = ['f', 'd', 'a'];

/// Die Aufsicht ueber ein Kommando: was daran nicht hinausgehen darf.
///
/// **Sie steht auf dem Weg und nicht daneben.** [`rufen`] ruft sie vor jedem
/// Prozessaufruf mit den Wortplaetzen, die wirklich hinausgehen. Bis zum 260821
/// stand an ihrer Stelle eine Probe, die drei Bauer namentlich aufzaehlte; eine
/// Aufzaehlung von Namen kann aber nicht zusagen, dass sie vollstaendig ist,
/// und sie war es auch nicht. Hier kommt nichts vorbei.
///
/// **Sie liest Plaetze und keine freien Woerter.** Bis zur Durchsicht vom
/// 260821-1432 las sie ein flaches Wortfeld und musste raten, ob ein Wort ein
/// Schalter, ein Wert, ein Refspec oder ein Pfad ist. Zwei Loecher kamen
/// daraus, und sie schliessen sich beide dadurch, dass die Auskunft, die
/// [`Auftrag`] ohnehin hat, bis hierher durchgereicht wird.
///
/// Fuenf Fragen, in dieser Reihenfolge:
///
/// 1. Es steht ueberhaupt ein Unterbefehl da, und er steht fest.
/// 2. Der Unterbefehl steht in der Erlaubnisliste, die zur [`Wirkung`] gehoert.
///    Damit ist `push` an einem lesenden Auftrag ausgeschlossen und `reset` an
///    jedem.
/// 3. Ein lesendes `tag` traegt die Marke, die aus dem Anlegen eine Frage
///    macht — und sie steht fest. `git tag v0.2.0` als Frage getarnt kommt so
///    nicht hinaus, und eine `--list` von aussen zaehlt dabei nicht.
/// 4. Jeder belegte Platz traegt die [`Gestalt`], die an seiner Stelle
///    zulaessig ist, und steht an der Stelle, an der `git` ihn als Wert liest.
/// 5. Kein festes Wort hinter dem Unterbefehl traegt Gewalt: keine Marke aus
///    [`MARKEN`] oder [`UEBERGEHENDE`], auch nicht abgekuerzt, keine kurze
///    Gruppe mit einem Buchstaben aus [`GEWALTBUCHSTABEN`], kein Verweis mit
///    fuehrendem `+` und keiner mit einem `:`, die beide ohne jede Marke auf
///    der Gegenseite wirken.
///
/// **Wie stark die fuenf sind, ist verschieden**, und der Modulkopf schreibt
/// es aus. Die Punkte 1 bis 3 sind Erlaubnislisten. Punkt 4 ist eine, wo der
/// Wert an einer Stelle steht, an der `git` eine Option oder einen Verweis
/// liest — ein Tagname, ein Tagverweis —, und traegt sonst die Stellung: eine
/// Meldung hinter `-m` und ein Pfad hinter `--` liest `git` nicht als eigenes
/// Wort, und was sie darueber hinaus abweist, ist grob. Punkt 5 ist eine
/// Verbotsliste und wird es bleiben — sie gilt aber nur fuer feste Woerter,
/// also fuer das, was der hinschreibt, der eine Variante hinzufuegt.
///
/// **Was sie nicht kann.** Ein Auftrag, der mit erlaubtem Unterbefehl, ohne
/// Marke und mit gestaltrichtigen Werten etwas Unerwuenschtes tut, kommt durch;
/// was ihn haelt, ist die Aufzaehlung [`Auftrag`] und der Blick dessen, der
/// eine Variante hinzufuegt. Und ein Prozessaufruf an [`rufen`] vorbei erreicht
/// sie gar nicht — dazu der Modulkopf.
fn aufsichtsbefund(wirkung: Wirkung, plaetze: &[Wort<'_>]) -> Option<String> {
    let Some(erstes) = plaetze.first() else {
        return Some("es steht gar kein Unterbefehl da".to_owned());
    };
    let Wort::Fest(unterbefehl) = erstes else {
        return Some(format!(
            "der Unterbefehl {} kommt von aussen, statt an seiner Variante festzustehen",
            erstes.wert()
        ));
    };
    let erlaubte: &[&str] = match wirkung {
        Wirkung::Liest => &LESENDE,
        Wirkung::Schreibt => &SCHREIBENDE,
    };
    if !erlaubte.contains(unterbefehl) {
        return Some(format!(
            "der Unterbefehl {unterbefehl} steht nicht in der Erlaubnisliste {erlaubte:?}"
        ));
    }
    let feste: Vec<&str> = plaetze[1..]
        .iter()
        .filter_map(|wort| match wort {
            Wort::Fest(text) => Some(*text),
            Wort::Platz(..) => None,
        })
        .collect();
    if wirkung == Wirkung::Liest
        && *unterbefehl == "tag"
        && !feste.contains(&"--points-at")
        && !feste.contains(&"--list")
    {
        return Some(
            "git tag ohne --points-at und ohne --list legt einen Tag an, statt zu fragen"
                .to_owned(),
        );
    }
    for (stelle, wort) in plaetze.iter().enumerate().skip(1) {
        let befund = match wort {
            Wort::Fest(text) => gewaltbefund(text),
            Wort::Platz(gestalt, wert) => {
                stellungsbefund(*gestalt, &plaetze[..stelle]).or_else(|| gestalt.befund(wert))
            }
        };
        if befund.is_some() {
            return befund;
        }
    }
    None
}

/// Ob ein Platz dieser Gestalt an dieser Stelle steht.
///
/// **Die Gestalt allein genuegt nicht; es zaehlt auch, wo sie steht.** Eine
/// Eintragsmeldung ist nur deshalb harmlos, weil `-m` unmittelbar davorsteht
/// und sie als Wert aufnimmt; dieselbe Zeichenfolge an einer anderen Stelle
/// laese `git` als Wort. Ein Pfad ist nur deshalb ein Pfad, weil der Trenner
/// `--` davorsteht; davor waere er ein Refspec.
///
/// Ein Tagname und ein Tagverweis stehen an Stellen, an denen `git` Optionen
/// liest, und tragen ihre Sicherheit deshalb ganz in ihrer Gestalt.
///
/// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine fuenfte Gestalt
/// haelt hier den Bau an und muss sagen, wo sie stehen darf.
fn stellungsbefund(gestalt: Gestalt, davor: &[Wort<'_>]) -> Option<String> {
    match gestalt {
        Gestalt::Meldung => (davor.last() != Some(&Wort::Fest("-m"))).then(|| {
            "die Eintragsmeldung steht nicht unmittelbar hinter -m und wird damit als Wort gelesen"
                .to_owned()
        }),
        Gestalt::Pfad => (!davor.contains(&Wort::Fest(TRENNER))).then(|| {
            format!("ein Pfad steht vor dem Trenner {TRENNER} und wird damit als Verweis gelesen")
        }),
        Gestalt::Tagname | Gestalt::Tagverweis => None,
    }
}

/// Ob ein festes Wort hinter dem Unterbefehl Gewalt oder Reichweite traegt.
///
/// Die innerste Haelfte der Aufsicht, getrennt, weil ihre fuenf Faelle sich
/// einzeln nachsehen lassen. **Sie gilt nur fuer feste Woerter** — der
/// Vorbehalt, unter dem sie steht, haengt am Doc-Kommentar von [`MARKEN`].
fn gewaltbefund(wort: &str) -> Option<String> {
    if wort == TRENNER {
        return None;
    }
    if let Some(eintrag) = verwandte_marke(&MARKEN, wort) {
        return Some(markenmeldung(wort, eintrag, "erweitert die Reichweite"));
    }
    if let Some(eintrag) = verwandte_marke(&UEBERGEHENDE, wort) {
        return Some(markenmeldung(wort, eintrag, "uebergeht eine Pruefung"));
    }
    if let Some(buchstaben) = kurze_marke(wort)
        && let Some(gewalt) = buchstaben
            .chars()
            .find(|zeichen| GEWALTBUCHSTABEN.contains(zeichen))
    {
        return Some(format!(
            "die kurze Marke {wort} traegt -{gewalt} und erzwingt oder erweitert"
        ));
    }
    if wort.starts_with('+') {
        return Some(format!(
            "der Verweis {wort} erzwingt mit seinem fuehrenden + und ohne jede Marke"
        ));
    }
    if wort.contains(':') {
        return Some(format!(
            "der Verweis {wort} benennt mit seinem : ein Ziel auf der Gegenseite und schreibt \
             oder loescht es, ohne jede Marke"
        ));
    }
    None
}

/// Die Meldung zu einer angehaltenen langen Marke.
///
/// Sie nennt den Eintrag dazu, wenn das Wort ihn abkuerzt oder erweitert: wer
/// `--del` liest, soll `--delete` daneben sehen und nicht raten muessen, warum
/// die Aufsicht anhaelt.
#[must_use]
fn markenmeldung(wort: &str, eintrag: &str, wirkt: &str) -> String {
    if wort == eintrag {
        format!("die Marke {wort} {wirkt}")
    } else {
        format!("die Marke {wort} steht fuer {eintrag} und {wirkt}")
    }
}

/// Der Eintrag der Liste, den dieses Wort meint, oder `None`.
///
/// **Weder Gleichheit noch blosser Wortanfang, sondern beides.** `git` nimmt
/// eine lange Marke abgekuerzt entgegen, also meint `--del` den Eintrag
/// `--delete`; und eine Marke traegt Formen mit Anhang, also meint
/// `--force-with-lease` den Eintrag `--force`. Angehalten wird deshalb, wenn
/// eines von beiden der Anfang des anderen ist.
///
/// Ein Anhang hinter `=` faellt vorher weg: `--exec=/bin/sh` meint `--exec`.
///
/// **Kurze Marken bleiben draussen**, und das ist nicht Nachlaessigkeit: `-m`
/// waere der Anfang von `--mirror`, wenn man den einen Strich nicht verlangte,
/// und die Eintragsmeldung braucht `-m`. Kurze Gruppen liest [`kurze_marke`],
/// Buchstabe fuer Buchstabe. Der Trenner `--` ist der Anfang jeder langen
/// Marke und faellt ebenfalls heraus; ihn haelt [`gewaltbefund`] vorweg ab.
fn verwandte_marke<'l>(liste: &'l [&'l str], wort: &str) -> Option<&'l str> {
    let kern = wort.split('=').next().unwrap_or(wort);
    if kern == TRENNER || !kern.starts_with(TRENNER) {
        return None;
    }
    liste
        .iter()
        .copied()
        .find(|eintrag| eintrag.starts_with(kern) || kern.starts_with(eintrag))
}

/// Die Buchstaben einer kurzen Markengruppe, oder `None`.
///
/// Eine kurze Gruppe ist ein Strich und danach nur Buchstaben: `-f`, `-fd`,
/// `-m`. Der Trenner `--`, jede lange Marke und jeder Pfad fallen heraus.
fn kurze_marke(wort: &str) -> Option<&str> {
    let ohne_strich = wort.strip_prefix('-')?;
    if ohne_strich.is_empty()
        || !ohne_strich
            .chars()
            .all(|zeichen| zeichen.is_ascii_alphabetic())
    {
        return None;
    }
    Some(ohne_strich)
}

/// Die Meldung, wenn die Aufsicht ein Kommando anhaelt.
///
/// Sie schreibt die ganze Liste aus, denn wer sie liest, sucht die Stelle, an
/// der sie gebaut wird, und nicht die Aufsicht.
#[must_use]
fn aufsichtsmeldung(worte: &[&str], befund: &str) -> String {
    format!(
        "Die Aufsicht haelt das Kommando `git {}` an: {befund}.\n\
         \n\
         Dieses Werkzeug reicht an git nur Kommandos, die als Variante von git::Auftrag \
         dastehen und die Aufsicht in git.rs bestehen. Wer ein weitergehendes braucht, \
         entscheidet es dort und nicht im Vorbeigehen.\n\
         \n\
         Es wird nichts ausgefuehrt.",
        worte.join(" ")
    )
}

/// Ruft `git` im Projektverzeichnis und liefert seine Standardausgabe.
///
/// Nach dem Muster von `security_fragen` in `sign`: absoluter Pfad, weil der
/// Baum jedes Systemwerkzeug so ruft, `.current_dir` auf die Projektwurzel,
/// weil die Antwort sonst am Arbeitsverzeichnis des Aufrufers haengt.
/// Startfehler und ein Rueckgabewert ungleich null werden beide zum
/// Laufabbruch.
///
/// **Vor dem Prozessaufruf steht die Aufsicht.** Sie liest die Wortplaetze des
/// Auftrags, nicht seinen Namen; ein Auftrag, den niemand nachgesehen hat,
/// kommt damit trotzdem nicht als Gewalt hinaus.
pub(crate) fn rufen(wurzel: &Path, auftrag: &Auftrag<'_>) -> Result<String, Abbruch> {
    let plaetze = auftrag.wortplaetze();
    let worte = auftrag.worte();
    if let Some(befund) = aufsichtsbefund(auftrag.wirkung(), &plaetze) {
        return Err(Abbruch::Lauf(aufsichtsmeldung(&worte, &befund)));
    }
    let ausgabe = Command::new("/usr/bin/git")
        .args(&worte)
        .current_dir(wurzel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("git laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "git {} ist gescheitert ({}): {}",
            worte.join(" "),
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&ausgabe.stdout).into_owned())
}

/// Die geaenderten verfolgten Dateien aus der Ausgabe von [`Auftrag::Stand`].
///
/// Eine Datei je Zeile, samt der zweistelligen Zustandsspalte, die
/// `--porcelain` voranstellt. Leerzeilen fallen weg; der Rest bleibt so
/// stehen, wie `git` ihn schreibt, damit die Meldung die Datei so nennt, wie
/// der Nutzer sie im naechsten `git status` wiederfindet.
pub(crate) fn geaenderte_dateien(ausgabe: &str) -> Vec<&str> {
    ausgabe
        .lines()
        .map(str::trim_end)
        .filter(|zeile| !zeile.trim().is_empty())
        .collect()
}

/// Steht `erwartet` in der Ausgabe von [`Auftrag::TagsAufHead`]?
///
/// Verglichen wird die ganze Zeile und nicht ihr Anfang: sonst deckte
/// `v0.1.0-rc1` die Auslieferung von `0.1.0`.
pub(crate) fn tag_steht(tags_auf_head: &str, erwartet: &str) -> bool {
    tags_auf_head.lines().any(|zeile| zeile.trim() == erwartet)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die zwei Dateien, die der Eintrag des Versionsschritts traegt.
    const BEISPIELDATEIEN: [&str; 2] = ["Cargo.toml", "Cargo.lock"];

    /// Jede lange Marke, die angehalten werden muss, ausgeschrieben.
    ///
    /// **Ausgeschrieben und nicht aus [`MARKEN`] gelesen** — der Grund steht
    /// bei [`jede_einzelne_marke_wird_angehalten`].
    const ANGEHALTENE_LANGE: [&str; 11] = [
        "--tags",
        "--follow-tags",
        "--all",
        "--mirror",
        "--delete",
        "--prune",
        "--amend",
        "--force",
        "--exec",
        "--receive-pack",
        "--no-verify",
    ];

    /// Jede kurze Gewaltmarke, ausgeschrieben.
    const ANGEHALTENE_KURZE: [&str; 3] = ["-f", "-d", "-a"];

    /// Die Beispielmeldung des Versionsschritts.
    const BEISPIELMELDUNG: &str = "chore(release): die Version steht auf 0.2.0";

    /// Der naechste Auftrag der Aufzaehlung, mit Beispielwerten, oder `None`.
    ///
    /// **Diese Kette ist es, die [`beispiele`] an die Aufzaehlung bindet.**
    /// Vollstaendige Fallunterscheidung ohne Auffangzweig: eine achte Variante
    /// haelt hier den Bau an, und wer sie einordnet, muss sagen, wer nach ihr
    /// kommt — davor war [`beispiele`] ein `vec!`-Literal, das eine achte
    /// Variante gruen liegengelassen haette (Durchsicht 260821-1432, B1).
    ///
    /// **Was die Kette nicht haelt:** wer den neuen Zweig auf `None` setzt und
    /// den bisherigen letzten stehenlaesst, haengt seine Variante nicht ein.
    /// Der Bau haelt dann nicht an, und die Aufsicht faengt den Auftrag
    /// trotzdem — nur eben erst beim Lauf. Mehr sagt diese Probe nicht zu.
    fn naechster(auftrag: &Auftrag<'_>) -> Option<Auftrag<'static>> {
        match auftrag {
            Auftrag::Verzeichnis => Some(Auftrag::TagsAufHead),
            Auftrag::TagsAufHead => Some(Auftrag::Stand),
            Auftrag::Stand => Some(Auftrag::Tagliste("v0.2.0")),
            Auftrag::Tagliste(_) => Some(Auftrag::TagSetzen("v0.2.0")),
            Auftrag::TagSetzen(_) => Some(Auftrag::Eintrag {
                meldung: BEISPIELMELDUNG,
                dateien: &BEISPIELDATEIEN,
            }),
            Auftrag::Eintrag { .. } => Some(Auftrag::Schub {
                verweis: "refs/tags/v0.2.0",
            }),
            Auftrag::Schub { .. } => None,
        }
    }

    /// Jeder Auftrag einmal, mit Beispielwerten.
    ///
    /// **Diese Liste ist nicht die Zusage, sondern ihre Vorwegnahme.** Was
    /// haelt, dass kein Auftrag Gewalt traegt, ist [`aufsichtsbefund`] auf dem
    /// Weg selbst; diese Liste laesst den Ausfall bei `cargo test` geschehen
    /// statt beim Auslieferungslauf. Sie entsteht aus [`naechster`], damit sie
    /// nicht neben der Aufzaehlung veraltet.
    fn beispiele() -> Vec<Auftrag<'static>> {
        let mut alle = vec![Auftrag::Verzeichnis];
        while let Some(folgender) =
            naechster(alle.last().expect("die Kette faengt bei Verzeichnis an"))
        {
            alle.push(folgender);
        }
        alle
    }

    /// Eine Liste aus lauter festen Woertern.
    ///
    /// So saehe die Liste einer Variante aus, die jemand hinzugefuegt hat, ohne
    /// dass jemand sie nachgesehen hat: alles steht im Quelltext, nichts kommt
    /// von aussen. Genau dafuer ist [`gewaltbefund`] da.
    fn fest(worte: &[&'static str]) -> Vec<Wort<'static>> {
        worte.iter().copied().map(Wort::Fest).collect()
    }

    /// Kein Auftrag dieses Werkzeugs traegt Gewalt.
    #[test]
    fn die_aufsicht_laesst_jeden_auftrag_durch() {
        for auftrag in beispiele() {
            let plaetze = auftrag.wortplaetze();
            assert_eq!(
                aufsichtsbefund(auftrag.wirkung(), &plaetze),
                None,
                "{:?} kommt nicht durch",
                auftrag.worte()
            );
        }
    }

    /// Die drei Fragen des Auslieferungswegs lesen, jede einzeln nachgesehen.
    ///
    /// **Der Name bleibt, und der Gegenstand auch.** Das Abnahmekriterium C3.7
    /// der Runde „Artefakt und Release" nennt diese Probe beim Namen und
    /// verlangt, dass sie unveraendert gruen laeuft; sie steht deshalb weiter
    /// da, obwohl [`aufsichtsbefund`] dieselbe Frage seit dem 260821 auf dem
    /// Weg selbst stellt. Was sich geaendert hat, ist die Form der drei Fragen:
    /// sie sind Varianten von [`Auftrag`] statt dreier Konstanten.
    ///
    /// `tag` steht in [`SCHREIBENDE`] und trotzdem in [`LESENDE`], weil es
    /// beides kann: mit `--points-at` fragt es, mit einem Namen legt es an.
    /// Genau diese Unterscheidung prueft der zweite Teil.
    #[test]
    fn keine_der_drei_fragen_schreibt() {
        for frage in [Auftrag::Verzeichnis, Auftrag::TagsAufHead, Auftrag::Stand] {
            assert_eq!(frage.wirkung(), Wirkung::Liest);
            let worte = frage.worte();
            assert_eq!(
                aufsichtsbefund(Wirkung::Liest, &frage.wortplaetze()),
                None,
                "{worte:?}"
            );
            assert!(LESENDE.contains(&worte[0]), "{worte:?}");
            if worte[0] == "tag" {
                assert!(
                    worte.contains(&"--points-at"),
                    "git tag ohne --points-at legt einen Tag an: {worte:?}"
                );
                assert!(
                    worte
                        .iter()
                        .all(|wort| wort.starts_with("--") || *wort == "tag" || *wort == "HEAD"),
                    "ein Name hinter git tag legt ihn an: {worte:?}"
                );
            }
        }
    }

    /// Jeder Auftrag, Wort fuer Wort.
    ///
    /// Bis zum 260821 stand diese Nachschau in `version::tests` und las drei
    /// Bauer namentlich; sie steht jetzt dort, wo die Bauer stehen, und liest
    /// alle.
    ///
    /// **Die Zahl steht nicht in der Prosa, sondern haengt an [`beispiele`].**
    /// Der Doc-Kommentar sprach bis zur Durchsicht 260821-1432 von „den sieben
    /// Auftraegen"; eine achte Variante haette den Satz falsch gemacht, und
    /// nichts haette es gemerkt. Jetzt zaehlt die Probe selbst nach.
    #[test]
    fn die_auftraege_stehen_wort_fuer_wort() {
        let nachgesehen: Vec<(Auftrag<'static>, Vec<&str>)> = vec![
            (Auftrag::Verzeichnis, vec!["rev-parse", "--git-dir"]),
            (Auftrag::TagsAufHead, vec!["tag", "--points-at", "HEAD"]),
            (
                Auftrag::Stand,
                vec!["status", "--porcelain", "--untracked-files=no"],
            ),
            (Auftrag::Tagliste("v0.2.0"), vec!["tag", "--list", "v0.2.0"]),
            (Auftrag::TagSetzen("v0.2.0"), vec!["tag", "v0.2.0"]),
            (
                Auftrag::Eintrag {
                    meldung: "eine Meldung",
                    dateien: &BEISPIELDATEIEN,
                },
                vec![
                    "commit",
                    "--only",
                    "-m",
                    "eine Meldung",
                    "--",
                    "Cargo.toml",
                    "Cargo.lock",
                ],
            ),
            (
                Auftrag::Schub {
                    verweis: "refs/tags/v0.2.0",
                },
                vec!["push", "origin", "HEAD", "refs/tags/v0.2.0"],
            ),
        ];
        assert_eq!(
            nachgesehen.len(),
            beispiele().len(),
            "hier steht nicht jede Variante von Auftrag Wort fuer Wort da"
        );
        for (auftrag, erwartet) in nachgesehen {
            assert_eq!(auftrag.worte(), erwartet);
        }
    }

    /// Genau drei Auftraege schreiben, und `push` ist einer davon.
    ///
    /// Die Teilung ist trennscharf und ohne Ausnahmeliste: `push` ist an der
    /// einen Stelle erlaubt, weil dort ein Auftrag `push` **ist**, und an jeder
    /// lesenden Frage ausgeschlossen, weil `push` nicht in [`LESENDE`] steht.
    #[test]
    fn lesen_und_schreiben_sind_getrennt() {
        let schreibende: Vec<Vec<&str>> = beispiele()
            .iter()
            .filter(|auftrag| auftrag.wirkung() == Wirkung::Schreibt)
            .map(Auftrag::worte)
            .collect();
        assert_eq!(schreibende.len(), 3, "{schreibende:?}");
        for worte in &schreibende {
            assert!(SCHREIBENDE.contains(&worte[0]), "{worte:?}");
        }
        for auftrag in beispiele() {
            if auftrag.wirkung() == Wirkung::Liest {
                assert!(!SCHREIBENDE.contains(&auftrag.worte()[0]) || auftrag.worte()[0] == "tag");
            }
        }
    }

    /// Die drei Lucken, die die alte Markenliste hatte.
    ///
    /// Keine dieser Listen baut heute jemand — genau darum geht es. Die
    /// Aufsicht ist fuer die Aenderung von morgen gebaut, und diese Probe
    /// stellt sie ihr.
    #[test]
    fn die_aufsicht_faengt_die_kurze_form_die_leihgabe_und_das_abraeumen() {
        for gewalt in [
            vec!["push", "origin", "-d", "refs/tags/v0.2.0"],
            vec!["tag", "-d", "v0.2.0"],
            vec!["push", "origin", "-fd", "HEAD"],
            vec!["push", "--force-with-lease", "origin", "HEAD"],
            vec![
                "push",
                "--force-with-lease=refs/heads/main",
                "origin",
                "HEAD",
            ],
            vec!["push", "--force-if-includes", "origin", "HEAD"],
            vec!["push", "--prune", "origin", "refs/tags/*"],
            vec!["push", "--follow-tags", "origin", "HEAD"],
        ] {
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &fest(&gewalt)).is_some(),
                "{gewalt:?} kommt durch"
            );
        }
    }

    /// Ein Verweis mit fuehrendem `+` erzwingt ohne jede Marke.
    #[test]
    fn die_aufsicht_faengt_den_erzwingenden_verweis() {
        assert!(aufsichtsbefund(Wirkung::Schreibt, &fest(&["push", "origin", "+HEAD"])).is_some());
        assert!(
            aufsichtsbefund(
                Wirkung::Schreibt,
                &fest(&["push", "origin", "+refs/tags/v0.2.0:refs/tags/v0.2.0"])
            )
            .is_some()
        );
    }

    /// Jede einzelne Marke der Listen wird angehalten.
    ///
    /// **Die Woerter stehen hier ausgeschrieben und werden nicht aus den Listen
    /// gelesen.** Eine Schleife ueber [`MARKEN`] pruefte, was dasteht, und
    /// bliebe gruen, wenn ein Eintrag verschwaende — genau das war der Befund
    /// B1 der Durchsicht vom 260821-1432: fuenf der sieben Marken, der
    /// Gewaltbuchstabe `a` und `--no-verify` massen keine Probe, und wer einen
    /// von ihnen loeschte, liess alles gruen. Ausgeschrieben wird die Probe
    /// rot.
    ///
    /// Damit ist der zweite Satz des Abnahmekriteriums C3.4 gemessen: die
    /// sechs, die es nennt — `--force`, `-f`, `--tags`, `--all`, `--mirror`,
    /// `--delete` — stehen alle hier.
    #[test]
    fn jede_einzelne_marke_wird_angehalten() {
        for marke in ANGEHALTENE_LANGE {
            assert!(gewaltbefund(marke).is_some(), "{marke} kommt durch");
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &fest(&["push", marke, "origin"])).is_some(),
                "{marke} kommt an einem push durch"
            );
        }
        for marke in ANGEHALTENE_KURZE {
            assert!(gewaltbefund(marke).is_some(), "{marke} kommt durch");
        }
    }

    /// Kein Eintrag der Listen steht ohne Anhalteprobe da.
    ///
    /// Die Gegenrichtung zu [`jede_einzelne_marke_wird_angehalten`]: die dortige
    /// Liste faengt einen geloeschten Eintrag, diese hier faengt einen
    /// hinzugefuegten, den niemand nachgesehen hat.
    #[test]
    fn jeder_eintrag_der_listen_steht_in_der_anhalteprobe() {
        for eintrag in MARKEN.iter().chain(UEBERGEHENDE.iter()) {
            assert!(
                ANGEHALTENE_LANGE.contains(eintrag),
                "{eintrag} steht in einer Liste, aber in keiner Anhalteprobe"
            );
        }
        for buchstabe in GEWALTBUCHSTABEN {
            let kurz = format!("-{buchstabe}");
            assert!(
                ANGEHALTENE_KURZE.contains(&kurz.as_str()),
                "{kurz} steht in GEWALTBUCHSTABEN, aber in keiner Anhalteprobe"
            );
        }
    }

    /// `git` nimmt eine lange Marke abgekuerzt an, und die Aufsicht auch.
    ///
    /// Nachgemessen an einem Wegwerf-Verzeichnis (Durchsicht 260821-1432, A2):
    /// `git tag --del t1` loescht, `git commit --ame` aendert den letzten
    /// Eintrag, `git push --mirr` und `--al` kommen durch den Optionszerleger.
    /// Ein Vergleich auf Gleichheit sah keines dieser Woerter.
    #[test]
    fn die_aufsicht_faengt_die_abgekuerzte_marke() {
        for kurz in [
            "--del",
            "--dele",
            "--ame",
            "--mirr",
            "--al",
            "--tag",
            "--pru",
            "--no-verif",
            "--force-with-lease",
            "--exec=/bin/sh",
            "--receive-pack=/bin/sh",
        ] {
            assert!(gewaltbefund(kurz).is_some(), "{kurz} kommt durch");
        }
        let meldung = gewaltbefund("--del").expect("--del wird angehalten");
        assert!(meldung.contains("--delete"), "{meldung}");
    }

    /// Ein Refspec mit `:` schreibt oder loescht ohne jede Marke.
    ///
    /// `git push origin HEAD :refs/heads/feature` sind genau die vier Woerter
    /// von [`Auftrag::Schub`] und loeschen einen Zweig auf der Gegenseite;
    /// nachgemessen an einem Wegwerf-Verzeichnis (Durchsicht 260821-1432, A1).
    /// Die Regel fuer `+` stand schon da — der Doppelpunkt ist dieselbe Wirkung
    /// in der aelteren Schreibweise.
    #[test]
    fn die_aufsicht_faengt_den_refspec_mit_doppelpunkt() {
        for verweis in [
            ":refs/heads/feature",
            "HEAD:refs/heads/main",
            "refs/tags/v0.2.0:refs/heads/main",
        ] {
            assert!(
                gewaltbefund(verweis).is_some(),
                "{verweis} kommt als festes Wort durch"
            );
            let auftrag = Auftrag::Schub { verweis };
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &auftrag.wortplaetze()).is_some(),
                "{verweis} kommt als belegter Platz durch"
            );
        }
    }

    /// Ein belegter Platz traegt seine Gestalt, oder er kommt nicht durch.
    ///
    /// **Hier steht die Erlaubnisliste, und sie ist der eigentliche Schnitt.**
    /// Kein Wort dieser Probe steht irgendwo als verboten da; sie kommen nicht
    /// durch, weil sie nicht aussehen wie das, was an ihrer Stelle vorgesehen
    /// ist. Damit muss niemand eine Marke verbieten, an die er nicht gedacht
    /// hat — der Vorbehalt, unter dem [`MARKEN`] steht, gilt fuer belegte
    /// Plaetze nicht.
    #[test]
    fn ein_belegter_platz_traegt_nur_seine_gestalt() {
        for verweis in [
            "--delete",
            "--del",
            "+refs/tags/v0.2.0",
            "refs/heads/main",
            "v0.2.0",
            "refs/tags/0.2.0",
            "refs/tags/v0.2.0-rc1",
            "",
        ] {
            let auftrag = Auftrag::Schub { verweis };
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &auftrag.wortplaetze()).is_some(),
                "der Verweis {verweis:?} kommt durch"
            );
        }
        for name in ["-d", "--delete", "0.2.0", "v0.2", "v0.2.0-rc1", "HEAD", ""] {
            for auftrag in [Auftrag::TagSetzen(name), Auftrag::Tagliste(name)] {
                assert!(
                    aufsichtsbefund(auftrag.wirkung(), &auftrag.wortplaetze()).is_some(),
                    "der Tagname {name:?} kommt durch"
                );
            }
        }
        for datei in ["/etc/passwd", "../fremd.toml", "", "Cargo\ntoml"] {
            let dateien = [datei];
            let auftrag = Auftrag::Eintrag {
                meldung: BEISPIELMELDUNG,
                dateien: &dateien,
            };
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &auftrag.wortplaetze()).is_some(),
                "der Pfad {datei:?} kommt durch"
            );
        }
    }

    /// Eine Eintragsmeldung darf wie eine Marke aussehen.
    ///
    /// Sie steht hinter `-m`, das sie als Wert aufnimmt; `git` liest sie nicht
    /// als Schalter. Die alte Aufsicht las jedes Wort als haette es eine Marke
    /// sein koennen und hielt diesen Auftrag an — ein Falschalarm, den eine
    /// Aufsicht, die Plaetze kennt, nicht mehr hat.
    #[test]
    fn die_eintragsmeldung_darf_wie_eine_marke_aussehen() {
        for meldung in ["-a", "--force", "-m", "chore(release): 0.2.0"] {
            let auftrag = Auftrag::Eintrag {
                meldung,
                dateien: &BEISPIELDATEIEN,
            };
            assert_eq!(
                aufsichtsbefund(Wirkung::Schreibt, &auftrag.wortplaetze()),
                None,
                "die Meldung {meldung:?} wird angehalten"
            );
        }
    }

    /// Ein Platz an der falschen Stelle kommt nicht durch.
    ///
    /// Die Gestalt allein genuegt nicht: eine Meldung ist nur hinter `-m`
    /// harmlos, ein Pfad nur hinter dem Trenner.
    #[test]
    fn ein_platz_an_der_falschen_stelle_kommt_nicht_durch() {
        assert!(stellungsbefund(Gestalt::Meldung, &fest(&["commit", "--only"])).is_some());
        assert!(stellungsbefund(Gestalt::Meldung, &fest(&["commit", "--only", "-m"])).is_none());
        assert!(stellungsbefund(Gestalt::Pfad, &fest(&["commit", "--only"])).is_some());
        assert!(stellungsbefund(Gestalt::Pfad, &fest(&["commit", "--"])).is_none());
        assert!(stellungsbefund(Gestalt::Tagname, &fest(&["tag"])).is_none());
        assert!(stellungsbefund(Gestalt::Tagverweis, &fest(&["push", "origin"])).is_none());
    }

    /// Der Unterbefehl steht fest und kommt nicht von aussen.
    #[test]
    fn ein_unterbefehl_von_aussen_kommt_nicht_durch() {
        let plaetze = [Wort::Platz(Gestalt::Meldung, "status")];
        assert!(aufsichtsbefund(Wirkung::Liest, &plaetze).is_some());
    }

    /// Die Marken der sieben Varianten bleiben zulaessig.
    ///
    /// Der Vergleich am Wortanfang faellt zur sicheren Seite, und diese Probe
    /// haelt fest, dass er dabei nichts mitnimmt, was dastehen muss.
    #[test]
    fn kein_festes_wort_der_sieben_varianten_wird_angehalten() {
        for zulaessig in [
            "--git-dir",
            "--points-at",
            "--porcelain",
            "--untracked-files=no",
            "--list",
            "--only",
            "--",
            "-m",
            "HEAD",
            "origin",
        ] {
            assert_eq!(
                gewaltbefund(zulaessig),
                None,
                "{zulaessig} wird angehalten, obwohl es dastehen muss"
            );
        }
    }

    /// Ein fremder Unterbefehl kommt nicht durch, ohne dass ihn jemand einzeln
    /// verboten haette.
    ///
    /// Das ist der Unterschied zwischen einer Erlaubnisliste und einer
    /// Verbotsliste: die vier hier stehen nirgends als verboten da.
    #[test]
    fn die_aufsicht_faengt_jeden_fremden_unterbefehl() {
        for fremd in [
            vec!["reset", "--hard"],
            vec!["clean", "-x"],
            vec!["checkout", "."],
            vec!["stash"],
            vec!["update-ref", "refs/heads/main", "HEAD"],
        ] {
            assert!(
                aufsichtsbefund(Wirkung::Schreibt, &fest(&fremd)).is_some(),
                "{fremd:?} kommt durch"
            );
            assert!(
                aufsichtsbefund(Wirkung::Liest, &fest(&fremd)).is_some(),
                "{fremd:?} kommt als Frage durch"
            );
        }
        assert!(aufsichtsbefund(Wirkung::Liest, &[]).is_some());
    }

    /// Eine lesende Frage darf nicht schreiben.
    ///
    /// `git tag <name>` legt an; als [`Wirkung::Liest`] eingeordnet kommt es
    /// nicht hinaus, und `push` an einer Frage ebenso wenig.
    #[test]
    fn eine_lesende_frage_legt_keinen_tag_an() {
        assert!(aufsichtsbefund(Wirkung::Liest, &fest(&["tag", "v0.2.0"])).is_some());
        assert!(
            aufsichtsbefund(Wirkung::Liest, &Auftrag::Tagliste("v0.2.0").wortplaetze()).is_none()
        );
        assert!(aufsichtsbefund(Wirkung::Liest, &Auftrag::TagsAufHead.wortplaetze()).is_none());
        assert!(aufsichtsbefund(Wirkung::Liest, &fest(&["push", "origin", "HEAD"])).is_some());
    }

    /// `-m` bleibt zulaessig: `m` ist kein Gewaltbuchstabe.
    #[test]
    fn die_kurze_marke_des_eintrags_bleibt_zulaessig() {
        assert_eq!(kurze_marke("-m"), Some("m"));
        assert_eq!(kurze_marke("--"), None);
        assert_eq!(kurze_marke("--only"), None);
        assert_eq!(kurze_marke("Cargo.toml"), None);
        assert_eq!(gewaltbefund("-m"), None);
        assert!(gewaltbefund("-f").is_some());
        assert!(gewaltbefund("-df").is_some());
    }

    /// Unbeachtete Dateien bleiben aussen vor, und das haengt an der Marke.
    #[test]
    fn die_standabfrage_laesst_unbeachtete_dateien_aussen_vor() {
        let stand = Auftrag::Stand.worte();
        assert!(stand.contains(&"--untracked-files=no"), "{stand:?}");
        // Kein Pfadfilter: die Abfrage traegt kein `--` und keinen Pfad.
        assert!(
            !stand.contains(&"--"),
            "ein Pfadfilter waere eine Liste, die jemand pflegen muss: {stand:?}"
        );
    }

    #[test]
    fn leerzeilen_zaehlen_nicht_als_geaenderte_datei() {
        assert!(geaenderte_dateien("").is_empty());
        assert!(geaenderte_dateien("\n\n").is_empty());
        assert_eq!(
            geaenderte_dateien("M  Cargo.toml\n M README.md\n"),
            vec!["M  Cargo.toml", " M README.md"]
        );
    }

    #[test]
    fn ein_tag_gilt_nur_bei_ganzer_uebereinstimmung() {
        assert!(tag_steht("v0.1.0\n", "v0.1.0"));
        assert!(tag_steht("anderer\nv0.1.0\nnoch-einer\n", "v0.1.0"));
        assert!(!tag_steht("v0.1.0-rc1\nv0.1.10\n", "v0.1.0"));
        assert!(!tag_steht("", "v0.1.0"));
    }

    /// Die Meldung nennt das Kommando und den Befund.
    #[test]
    fn die_aufsichtsmeldung_nennt_kommando_und_befund() {
        let meldung =
            aufsichtsmeldung(&["push", "origin", "--force"], "die Marke --force erzwingt");
        assert!(meldung.contains("git push origin --force"), "{meldung}");
        assert!(meldung.contains("--force erzwingt"), "{meldung}");
        assert!(meldung.contains("Es wird nichts ausgefuehrt."), "{meldung}");
    }
}
