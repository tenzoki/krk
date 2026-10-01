//! Uebersetzt von einem Agenten am 261001; vom Nutzer noch nicht Flaeche
//! fuer Flaeche durchgesehen.
//!
//! Die franzoesische Tabelle. Das Glossar und die Typografie stehen im Kopf
//! von [`super`]; die geschuetzten Leerzeichen stehen als `\u{a0}` (vor `:`
//! und innen an « ») und `\u{202f}` (vor `;`, `!`, `?`) ausgeschrieben,
//! damit sie im Quelltext zu sehen sind.

use super::super::{Text, Zahlwort};

/// Der franzoesische Eintrag zu einem Schluessel.
pub(in super::super) const fn text(schluessel: Text) -> &'static str {
    match schluessel {
        Text::WirkungsbereichDateifenster => "volet de fichiers",
        Text::WirkungsbereichLeiste => "barre des signets et des volumes",
        Text::WirkungsbereichDateibereiche => "volet de fichiers, Aperçu et Éditeur",
        Text::WirkungsbereichEditor => "Éditeur",
        Text::WirkungsbereichEditortext => "texte dans l’Éditeur",
        Text::WirkungsbereichEintraege => "entrées dans l’Éditeur",
        Text::WirkungsbereichReihenfolge => "entrées dans l’ordre du fichier dans l’Éditeur",
        Text::WirkungsbereichAufgaben => "tâches dans l’Éditeur",
        Text::WirkungsbereichTermine => "rendez-vous dans l’Éditeur",
        Text::WirkungsbereichGeheimnisse => "secrets dans l’Éditeur",
        Text::WirkungsbereichQuicknote => "Quicknote dans l’Éditeur",
        Text::WirkungsbereichTabbereich => "volet de fichiers et Aperçu",
        Text::WirkungsbereichNavigator => "volet de fichiers, barre, Aperçu et zone Git",
        Text::WirkungsbereichVorschau => "Aperçu",
        Text::WirkungsbereichBildfolge => {
            "volet de fichiers, tant que l’Aperçu montre une série de photos"
        }
        Text::WirkungsbereichUeberall => "partout",
        Text::OrtsmangelAbsolut => "est un chemin absolu",
        Text::OrtsmangelLeeresStueck => "contient un segment vide",
        Text::OrtsmangelPunktstueck => "contient un segment . ou ..",
        Text::OrtsmangelMehrerePlatzhalter => {
            "contient plus d’un joker * et donc un coût qui ne serait connu qu’au vu du contenu"
        }
        Text::NameLeer => "le nom est vide",
        Text::NameMitSchraegstrich => "un nom ne peut pas contenir de barre oblique",
        Text::NameMitNullbyte => "un nom ne peut pas contenir d’octet nul",
        Text::NamePunktname => "«\u{a0}.\u{a0}» et «\u{a0}..\u{a0}» ne sont pas des noms",
        Text::KollisionBestehender => "le nom est déjà pris",
        Text::KollisionDoppelt => "deux fois le même nouveau nom",
        Text::AblagegrundNichtLesbar => "est illisible",
        Text::AblagegrundBeschaedigt => "est endommagé",
        Text::AblagegrundNichtAnlegbar => "n’a pas pu être créé",
        Text::ErsatzAuslieferungszustand => "et est remplacé par la version livrée",
        Text::ErsatzNichts => "et rien ne le remplace",
        Text::PinKeineVierZiffern => "Le code PIN se compose de quatre chiffres exactement.",
        Text::AbweisungUmbruchImAufgabentext => {
            "Une tâche tient sur une ligne et ne contient pas de saut de ligne."
        }
        Text::AbweisungUmbruchImThema => {
            "Un sujet tient sur une ligne et ne contient pas de saut de ligne."
        }
        Text::AbweisungThemenzeileImNotiztext => {
            "Une ligne du texte de la note ne peut pas commencer par «\u{a0}##\u{a0}», car c’est ainsi que commence la note suivante."
        }
        Text::AbweisungUngueltigesDatum => {
            "Une date s’écrit AAMMJJ ou AAMMJJ HH:MM, par exemple 261002 ou 261002 09:30."
        }
        Text::AbweisungKopfzeileImTermintext => {
            "Une ligne du texte du rendez-vous ne peut pas commencer par «\u{a0}##\u{a0}», car c’est ainsi que commence le rendez-vous suivant."
        }
        Text::MarkeGeaendert => "modifié",
        Text::MarkeVorgemerkt => "indexé",
        Text::MarkeNeu => "nouveau",
        Text::MarkeKonflikt => "en conflit",
        Text::MarkeUmbenannt => "renommé",
        Text::ZaehlzeileDateien => "Fichiers",
        Text::ZaehlzeileOrdner => "Dossiers",
        Text::ZaehlzeileVerknuepfungen => "Liens symboliques",
        Text::EinheitKilobyte => "ko",
        Text::EinheitMegabyte => "Mo",
        Text::EinheitGigabyte => "Go",
        Text::EinheitTerabyte => "To",
    }
}

/// Einzahl und Mehrzahl zu einem franzoesischen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} octet", "{n} octets"),
    }
}
