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
        Text::VorgangKeineRechte => "droits insuffisants",
        Text::VorgangGibtEsNichtMehr => "n’existe plus",
        Text::VorgangAmZielStehtEintrag => "une entrée existe déjà à la destination",
        Text::VorgangKeinPlatzAufDemDatentraeger => "plus d’espace libre sur le volume",
        Text::VorgangKeinArbeitsfaden => "aucun fil de travail libre\u{a0}: {grund}",
        Text::VorgangNeuerNameFehlt => "le nouveau nom manque",
        Text::VorgangZielordnerFehlt => "le dossier de destination manque",
        Text::VorgangPackenNichtQuelleFuerQuelle => {
            "la compression ne se fait pas source par source"
        }
        Text::VorgangPfadBenenntKeinenEintrag => "le chemin ne désigne aucune entrée",
        Text::VorgangQuelleUndZielDerselbeEintrag => {
            "la source et la destination sont la même entrée"
        }
        Text::VorgangZielLiegtInDerQuelle => "la destination se trouve dans la source",
        Text::VorgangZielNichtErsetzt => "la destination n’a pas pu être remplacée\u{a0}: {grund}",
        Text::VorgangNachAbbruchNichtWeggeraeumt => {
            "non supprimé après l’annulation\u{a0}: {grund}"
        }
        Text::VorgangOrdnerangabenNichtKopiert => {
            "contenu copié, mais pas les droits ni la date du dossier\u{a0}: {grund}"
        }
        Text::VorgangOrdnerSelbstBlieb => {
            "contenu déplacé, mais le dossier lui-même est resté\u{a0}: {grund}"
        }
        Text::VorgangNichtVollstaendigKopiert => "copie incomplète, resté dans la source",
        Text::VorgangKopiertAberInQuelleGeblieben => {
            "copié, mais resté dans la source\u{a0}: {grund}"
        }
        Text::VorgangKeinPapierkorb => {
            "aucune Corbeille n’est branchée\u{202f}; rien n’a été supprimé"
        }
        Text::VorgangKeineGewoehnlicheDatei => "pas un fichier ordinaire",
        Text::VorgangZielNichtInPapierkorb => {
            "la destination n’a pas pu être placée dans la Corbeille\u{a0}: {grund}"
        }
        Text::EntpackenEintragFuehrtHeraus => {
            "«\u{a0}{name}\u{a0}» sort du dossier de destination et est ignoré"
        }
        Text::EntpackenEintragMitGrund => "«\u{a0}{name}\u{a0}»\u{a0}: {grund}",
        Text::EntpackenAmZielStehtVerknuepfung => {
            "«\u{a0}{name}\u{a0}»\u{a0}: un lien symbolique existe déjà à la destination"
        }
        Text::EntpackenWegMitUnzulaessigemBestandteil => {
            "le chemin vers l’entrée contient un élément interdit"
        }
        Text::EntpackenWegDurchVerknuepfung => {
            "le chemin vers l’entrée traverse un lien symbolique qui sort du dossier de destination"
        }
        Text::EntpackenDateiStattOrdnerAufDemWeg => {
            "sur le chemin vers l’entrée se trouve un fichier là où il faudrait un dossier"
        }
        Text::EntpackenVerweiszielKeinText => "la cible du lien n’est pas un texte valide",
        Text::PackenArchivUnfertig => "l’archive est restée inachevée\u{a0}: {fehler}",
        Text::PackenKeinPlatzImArchiv => "pas de place dans l’archive\u{a0}: {fehler}",
        Text::PackenHalberEintragImArchiv => {
            "l’entrée à moitié écrite est restée dans l’archive\u{a0}: {fehler}"
        }
        Text::PackenNichtInsArchivGeschrieben => "non écrit dans l’archive\u{a0}: {fehler}",
        Text::StapelKeinStartwert => {
            "«\u{a0}{text}\u{a0}» n’est pas une valeur de départ pour la numérotation"
        }
        Text::StapelKeineStellenzahl => {
            "«\u{a0}{text}\u{a0}» n’est pas un nombre de chiffres entre 1 et {hoechste}"
        }
        Text::EditorKeinFreierDateizugriff => {
            "{pfad} ne peut pas être ouvert pour le moment\u{a0}: KRK n’a plus de descripteur de fichier libre ({grund})\u{202f}; réessayer après la fin de la recherche en cours"
        }
        Text::EditorNichtZuOeffnen => {
            "{pfad} ne peut pas être ouvert dans l’Éditeur\u{a0}: {grund}"
        }
        Text::EditorZuGross => {
            "{pfad} pèse {groesse} octets, trop pour l’Éditeur\u{202f}; la limite est de {grenze} octets"
        }
        Text::EditorKeineTextdatei => "{pfad} n’est pas un fichier texte et ne sera pas ouvert",
        Text::EditorOrdnerHatKeinenText => {
            "un dossier n’a pas de texte que l’Éditeur pourrait afficher"
        }
        Text::EditorKeineGewoehnlicheDatei => "ce n’est pas un fichier ordinaire",
        Text::LesenDatenschutzsperre => {
            "macOS bloque l’accès à «\u{a0}{name}\u{a0}». Autorisation\u{a0}: Réglages Système › Confidentialité et sécurité › Accès complet au disque › KRK, puis relancer KRK."
        }
        Text::LesenKeinVerzeichnis => "{pfad} n’est pas un dossier",
        Text::LesenPfadMitNullbyte => "{pfad} contient un octet nul",
    }
}

/// Einzahl und Mehrzahl zu einem franzoesischen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} octet", "{n} octets"),
    }
}
