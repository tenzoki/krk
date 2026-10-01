//! Uebersetzt von einem Agenten am 261001; vom Nutzer noch nicht Flaeche
//! fuer Flaeche durchgesehen.
//!
//! Die franzoesische Tabelle. Das Glossar und die Typografie stehen im Kopf
//! von [`super`]; die geschuetzten Leerzeichen stehen als `\u{a0}` (vor `:`
//! und innen an « ») und `\u{202f}` (vor `;`, `!`, `?`) ausgeschrieben,
//! damit sie im Quelltext zu sehen sind.

use super::super::{Text, Zahlwort};
use crate::tasten::belegung::{Kommando, Zugestellt};

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
        Text::AblageKeinGueltigesUtf8 => "séquence UTF-8 invalide",
        Text::AblageOhneOberstenSchluessel => {
            "le fichier ne contient aucune clé de premier niveau, et KRK ne l’écrit jamais ainsi"
        }
        Text::AblageKeinBenutzerverzeichnis => "le système n’indique aucun dossier de départ",
        Text::AtomarOhneDateinamen => "{pfad} ne porte aucun nom de fichier",
        Text::AtomarRechteNichtUebertragen => {
            "les droits {soll} de {pfad} n’ont pas pu être transférés\u{202f}; le fichier voisin est à {gesetzt}"
        }
        Text::ErsetzungOhneSicherung => "{datei} {beschreibung} {ersatz}\u{a0}: {einzelheit}",
        Text::ErsetzungGesichert => {
            "La version précédente se trouve sous {sicherung}\u{202f}; {datei} {beschreibung} {ersatz}\u{a0}: {einzelheit}"
        }
        Text::ErsetzungGekuerzt => {
            "La version précédente se trouve tronquée sous {sicherung}, seuls ses {grenze} premiers octets sont sauvegardés\u{202f}; {datei} {beschreibung} {ersatz}\u{a0}: {einzelheit}"
        }
        Text::ErsetzungSchonVorhanden => {
            "La version précédente se trouve depuis un démarrage antérieur sous {sicherung} et y reste\u{202f}; {datei} {beschreibung} {ersatz}\u{a0}: {einzelheit}"
        }
        Text::ErsetzungSicherungGescheitert => {
            "Le contenu n’a pas pu être mis de côté ({fehler})\u{202f}; {datei} {beschreibung} {ersatz}\u{a0}: {einzelheit}"
        }
        Text::EinstellungenVerweis => {
            "settings.toml est un lien symbolique, et KRK ne le remplace pas par un fichier\u{202f}; l’emplacement reste tel quel. À saisir à la main dans le fichier cible\u{a0}: {zeile}"
        }
        Text::EinstellungenBeschaedigt => {
            "settings.toml est d’abord à corriger à la main, KRK ne l’écrit pas ainsi\u{a0}: {befund}"
        }
        Text::EinstellungenNichtLesbar => {
            "settings.toml est illisible, KRK ne l’écrit pas\u{a0}: {befund}"
        }
        Text::EinstellungenIntern => {
            "settings.toml reste tel quel\u{a0}: le résultat aurait modifié plus que le dossier de notes ({befund})"
        }
        Text::EinstellungenNichtGeschrieben => {
            "settings.toml n’a pas pu être écrit et reste tel qu’il était\u{a0}: {befund}"
        }
        Text::EinstellungenKeineGewoehnlicheDatei => {
            "à sa place se trouve autre chose qu’un fichier ordinaire"
        }
        Text::EinstellungenNotizordnerKeinEinzelwert => {
            "notizordner n’apparaît pas comme valeur simple sur une ligne «\u{a0}notizordner = …\u{a0}»"
        }
        Text::EinstellungenNeuerWertFehlt => "la nouvelle valeur n’apparaît pas comme notizordner",
        Text::EinstellungenAndererWertGeaendert => "une autre valeur du fichier aurait changé",
        Text::WerksVerweis => {
            "{datei} est un lien symbolique, et KRK ne le remplace pas par un fichier\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksKeineDatei => {
            "À la place de {datei} se trouve autre chose qu’un fichier ordinaire\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksNichtLesbar => {
            "{datei} ne peut pas être interrogé ({befund})\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksKeineOrtszeit => {
            "L’horloge de cet appareil ne donne aucun horodatage pour les sauvegardes\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksKeinFreierName => {
            "Pour {datei}, il ne reste aucun nom de sauvegarde libre dans cette minute\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksNichtBeiseitegelegt => {
            "{datei} n’a pas pu être mis de côté ({befund})\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksNichtVorbereitet => {
            "La nouvelle version de {datei} n’a pas pu être écrite ({befund})\u{202f}; rien n’est réinitialisé."
        }
        Text::WerksEinstellungen => "{befund}. Rien n’est réinitialisé.",
        Text::WerksNichtZurueckgebaut => {
            "{hindernis} Une seconde copie du contenu inchangé est restée sous {pfade}."
        }
        Text::WerksZurueckgesetzt => "Réglages d’usine rétablis.",
        Text::WerksTeilweiseZurueckgesetzt => "Réglages d’usine rétablis seulement en partie.",
        Text::WerksBeiseitegelegt => "Mis de côté\u{a0}: {pfade}.",
        Text::WerksStandNichtDa => "{datei} n’existait pas, rien n’a été mis de côté pour lui.",
        Text::WerksDateiNichtZurueckgesetzt => "{datei} n’est pas réinitialisé\u{a0}: {fehler}.",
        Text::NeuerungenStartzeile => {
            "Nouveau dans cette version\u{a0}: {teile}. Vos fichiers se trouvent sous {ordner}."
        }
        Text::NeuerungenDateiFehlt => {
            "Ce fichier n’est pas dans votre dossier de données\u{202f}; seul ce qui existe est comparé."
        }
        Text::NeuerungenDateiBeschaedigt => "Ce fichier est endommagé et n’est donc pas comparé.",
        Text::NeuerungenNeuInDieserFassung => "Nouveau dans cette version",
        Text::NeuerungenNurInIhrerDatei => "Seulement dans votre fichier",
        Text::NeuerungenZeileLeer => "{ueberschrift}\u{a0}: —",
        Text::NeuerungenZeile => "{ueberschrift}\u{a0}: {namen}",
        Text::NeuerungenKeineEigenenEintraege => {
            "{zeile} (ce fichier ne peut pas contenir d’entrées propres\u{202f}; KRK rejette une entrée inconnue comme endommagée)"
        }
        Text::NeuerungenPreisLeser => {
            "Un profil absent de votre fichier coûte le résumé de cet emplacement\u{a0}: l’Aperçu y montre les métadonnées."
        }
        Text::NeuerungenPreisEinstellungen => {
            "Une clé absente de votre fichier ne coûte que le bloc de commentaires explicatif\u{202f}; la valeur elle-même, KRK la prend dans la version livrée."
        }
        Text::NeuerungenPreisBelegung => {
            "Une fonction absente de votre fichier coûte ses raccourcis clavier livrés\u{202f}; elle reste accessible par le menu principal."
        }
        Text::NeuerungenSchlusssatz => {
            "Ce qui est affiché est l’état que KRK a lu en dernier. Ce avec quoi KRK travaille est fixé depuis le démarrage\u{a0}: un fichier modifié n’agit qu’au prochain démarrage."
        }
        Text::OrtKeinBenutzerverzeichnis => {
            "Le système n’indique aucun dossier de départ, il n’y a donc pas de dossier de notes"
        }
        Text::OrtLeer => {
            "Le dossier de notes dans settings.toml est vide\u{202f}; un emplacement valide commence par «\u{a0}~/\u{a0}» ou «\u{a0}/\u{a0}», par défaut «\u{a0}~/{ordnername}\u{a0}»"
        }
        Text::OrtNichtAbsolut => {
            "Le dossier de notes «\u{a0}{wert}\u{a0}» dans settings.toml ne commence ni par «\u{a0}~/\u{a0}» ni par «\u{a0}/\u{a0}»"
        }
        Text::OrtFremdesBenutzerverzeichnis => {
            "Le dossier de notes «\u{a0}{wert}\u{a0}» dans settings.toml désigne le dossier de départ d’un autre utilisateur\u{202f}; «\u{a0}~/\u{a0}» vaut pour le vôtre, sinon un chemin à partir de «\u{a0}/\u{a0}»"
        }
        Text::OrtKeinText => {
            "Le dossier de notes dans settings.toml n’est pas un texte mais {wert}\u{202f}; un emplacement valide est entre guillemets, par exemple {beispiel}"
        }
        Text::OrtImAblageordner => {
            "Le dossier de notes «\u{a0}{wert}\u{a0}» se trouve dans le dossier de données de KRK\u{202f}; un outil qui supprime KRK l’emporterait, il n’est donc pas retenu"
        }
        Text::OrtEinstellungenBeschaedigt => {
            "settings.toml {satzteil}, aucun dossier de notes ne s’applique donc, et F2 ne crée rien\u{a0}: corriger settings.toml et relancer KRK, ou, après la correction, choisir l’emplacement par «\u{a0}Home\u{a0}» → «\u{a0}Choisir l’emplacement…\u{a0}»"
        }
        Text::OrtEinstellungenUngelesen => {
            "KRK n’a pas pu lire settings.toml au démarrage ({ursache}), aucun dossier de notes ne s’applique donc, et F2 ne crée rien\u{a0}: relancer KRK"
        }
        Text::OrtWechsel => {
            "Le dossier de notes est maintenant «\u{a0}{neu}\u{a0}»\u{202f}; à l’ancien emplacement «\u{a0}{alt}\u{a0}», tout reste en place, et F2 mène au nouveau"
        }
        Text::OrtAbweisung => {
            "Fermer d’abord {datei} dans l’Éditeur\u{202f}; tant que l’Éditeur tient un fichier du dossier de notes, KRK ne choisit pas d’autre emplacement"
        }
        Text::OrtGewaehlt => "Le dossier de notes est maintenant «\u{a0}{neu}\u{a0}», et F2 y mène",
        Text::OrtSchonDerOrt => {
            "«\u{a0}{neu}\u{a0}» est déjà le dossier de notes\u{202f}; settings.toml reste tel quel"
        }
        Text::HeimKeinOrdner => {
            "{ordner} n’est pas un dossier\u{202f}; KRK n’y crée rien et n’ouvre aucun onglet"
        }
        Text::HeimNichtErreichbar => "{ordner} est inaccessible\u{a0}: {grund}",
        Text::HeimNichtAnlegbar => "{name} ne peut pas être créé\u{a0}: {grund}",
        Text::HeimObererOrdnerFehlt => {
            "{ordner} ne peut pas être créé, car le dossier parent manque, par exemple un volume non monté\u{202f}; KRK ne crée rien"
        }
        Text::HeimMerkerNichtVermerkt => {
            "KRK ne peut pas retenir que les anciennes notes ont été reprises ({grund})\u{202f}; le prochain F2 réessaiera, et si {ort} est supprimé avant, il les reprendra une nouvelle fois"
        }
        Text::HeimMerkerOhneAblage => {
            "Sans son dossier de données, KRK ne peut pas retenir que les anciennes notes ont été reprises\u{202f}; un F2 ultérieur le rattrapera, et si {ort} est supprimé avant, il les reprendra une nouvelle fois"
        }
        Text::HeimGeheimnisseUmbenannt => "{alt} s’appelle maintenant {neu}",
        Text::HeimGeheimnisseBeideStehen => {
            "{neu} et {alt} existent tous deux dans {ort}\u{202f}; KRK n’en renomme aucun, et c’est {neu} qui compte"
        }
        Text::HeimGeheimnisseAlterNameBleibt => {
            "{alt} s’appelle maintenant aussi {neu}, l’ancien nom ne peut pas être supprimé\u{a0}: {grund}"
        }
        Text::HeimGeheimnisseNichtUmbenannt => {
            "{alt} ne peut pas être renommé en {neu} ({grund})\u{202f}; il reste inchangé, et {neu} n’est pas créé"
        }
        Text::HeimZettelNotizenStandenSchon => {
            "Les anciennes notes n’ont pas été reprises, car {notizen} existait déjà\u{202f}; {dateien} restent inchangés dans le dossier de données"
        }
        Text::HeimZettelGescheitert => {
            "Les anciennes notes n’ont pas été reprises ({grund})\u{202f}; {dateien} restent inchangés dans le dossier de données"
        }
        Text::HeimZettelThemenzeile => {
            "{thema} n’a pas été repris, car il contient une ligne commençant par «\u{a0}##\u{a0}»\u{202f}; {datei} reste inchangé dans le dossier de données"
        }
        Text::HeimZettelUnlesbar => {
            "{thema} n’a pas été repris ({grund})\u{202f}; {datei} reste inchangé dans le dossier de données"
        }
        Text::HeimZettelZuGross => "trop gros avec {groesse} octets",
        Text::HeimZettelKeinText => "pas de texte lisible",
        Text::HeimAngefangeneDateiBleibt => {
            "{fehler}\u{202f}; le fichier commencé ne peut pas être supprimé\u{a0}: {entfernen}"
        }
        Text::Und => "et",
        Text::TresorKeinZufall => "Le système ne fournit aucune valeur aléatoire\u{a0}: {grund}",
        Text::TresorAbleitung => "La clé ne peut pas être dérivée\u{a0}: {grund}",
        Text::TresorVerschluesselung => "Le contenu ne peut pas être chiffré",
        Text::TresorPinFalschOderVeraendert => "Code PIN erroné ou fichier modifié",
        Text::TresorKopfBeschaedigt => "L’en-tête du fichier est endommagé\u{a0}: {grund}",
        Text::TresorKopfKennungFehlt => "l’identifiant au début manque",
        Text::TresorKopfAbgeschnitten => "le fichier est tronqué",
        Text::TresorKopfUnbekannteVersion => "version de format inconnue {version}",
        Text::TresorKopfUnbekannteAbleitung => "dérivation inconnue {ableitung}",
        Text::TresorKopfUngueltigeParameter => "les paramètres de la dérivation sont invalides",
        Text::ZusammenfassungKopf => "Nom\u{a0}: {name}\nChemin\u{a0}: {pfad}",
        Text::ZusammenfassungBlockzeile => "{beschriftung}\u{a0}:",
        Text::ZusammenfassungZeile => "{beschriftung}\u{a0}: {wert}",
        Text::ZusammenfassungPlatzhalter => "--",
        Text::WertUeberGrenze => "au moins {gezaehlt} (lecture interrompue à {grenze} entrées)",
        Text::Ja => "oui",
        Text::Nein => "non",
        Text::ProfilMeldung => "Profil «\u{a0}{profil}\u{a0}»\u{a0}: {grund}",
        Text::ProfilZeilenmeldung => {
            "Profil «\u{a0}{profil}\u{a0}», ligne «\u{a0}{beschriftung}\u{a0}»\u{a0}: {grund}"
        }
        Text::ProfilMehrereBausteine => "elle nomme {anzahl} briques ({namen}) au lieu d’une seule",
        Text::ProfilKeinBaustein => "elle ne nomme aucune des quatre briques ({namen})",
        Text::ProfilOhneErkennung => {
            "il ne nomme ni motif de chemin ni fichier repère et ne pourrait donc jamais correspondre"
        }
        Text::ProfilBildfolge => "la série de photos\u{a0}: {grund}",
        Text::ProfilPfadmuster => "le motif de chemin",
        Text::ProfilKennzeichendatei => "le fichier repère",
        Text::ProfilErkennungsmusterNichtUebersetzt => {
            "{was} {muster} ne peut pas être compilé\u{a0}: {grund}"
        }
        Text::ProfilMusterNichtUebersetzt => {
            "le motif {muster} ne peut pas être compilé\u{a0}: {grund}"
        }
        Text::ProfilFeldmusterFanggruppen => {
            "le motif de champ {muster} contient {gruppen} groupes de capture au lieu d’un seul"
        }
        Text::ProfilOrtsangabe => "l’indication d’emplacement {angabe} {mangel}",
        Text::ProfilOrtsangabeMitPlatzhalter => {
            "l’indication d’emplacement {angabe} contient un joker, et la brique «\u{a0}{baustein}\u{a0}» n’en accepte pas\u{a0}: elle lit des fichiers et a besoin de leur chemin, qu’un état de lecture fusionné ne porte pas"
        }
        Text::ProfilJuengsteNull => "juengste avec anzahl = 0 ne peut jamais montrer d’entrée",
        Text::GitKeinRepository => "Ce dossier ne se trouve dans aucun dépôt Git.",
        Text::GitOhneCommit => "pas encore de commit",
        Text::GitUnveraendert => "inchangé",
        Text::GitImOrdner => "{marken} dans ce dossier",
        Text::GitKopfAbgeloest => "{kurzhash} (détaché)",
        Text::TasteNameFehlt => "le nom de la touche manque",
        Text::TasteKeineZusatztaste => {
            "«\u{a0}{text}\u{a0}» n’est pas une touche de modification\u{202f}; sont admises {erlaubt}"
        }
        Text::TasteFnKeineZusatztaste => {
            "fn n’est pas une touche de modification d’un raccourci\u{202f}; KRK consulte les touches de fonction par leur code, et F3 avec fn enfoncée produit le même code qu’un F3 seul"
        }
        Text::TasteZusatztasteDoppelt => {
            "la touche de modification «\u{a0}{text}\u{a0}» apparaît deux fois"
        }
        Text::TasteReihenfolgeVerletzt => {
            "«\u{a0}{zusatztaste}\u{a0}» vient après «\u{a0}{hinter}\u{a0}»\u{202f}; l’ordre est {reihenfolge}"
        }
        Text::TasteUnbekannterName => {
            "«\u{a0}{text}\u{a0}» n’est pas un nom de touche de cette notation"
        }
        Text::FunktionsnameMitKennung => "«\u{a0}{name}\u{a0}» ({kennung})",
        Text::BelegungKonflikt => {
            "la combinaison {kombination} appartient déjà à la fonction {andere} et ne peut pas être attribuée en plus à la fonction {bewerber}"
        }
        Text::BelegungSchreibweise => {
            "la fonction {kennung} porte la combinaison «\u{a0}{text}\u{a0}»\u{a0}: {fehler}"
        }
        Text::BelegungUnbekannteFunktion => "KRK ne connaît aucune fonction nommée {kennung}",
        Text::BelegungFunktionDoppelt => "la fonction {kennung} figure deux fois",
        Text::MenueUeberKrk => "À propos de KRK",
        Text::MenueTastenbelegungAlsMarkdown => "Enregistrer les raccourcis clavier en Markdown",
        Text::FunktionsbereichAnwendung => "Application",
        Text::FunktionsbereichHome => "Home",
        Text::FunktionsbereichDateilisting => "Liste de fichiers",
        Text::FunktionsbereichDateioperationen => "Opérations sur les fichiers",
        Text::FunktionsbereichTabs => "Onglets",
        Text::FunktionsbereichVorschau => "Aperçu",
        Text::FunktionsbereichLeisteUndFokus => "Barre et focus",
        Text::FunktionsbereichEditor => "Éditeur",
        Text::FunktionsbereichGit => "Git",
        Text::FunktionsbereichTextbefehle => "Édition",
        Text::FunktionsbereichFenster => "Fenêtre",
        Text::KontextOeffnenMit => "Ouvrir avec",
        Text::KontextZippen => "Zip",
        Text::KontextEntpacken => "Unzip",
        Text::KontextDuplizieren => "Dupliquer…",
        Text::KontextImFinderOeffnen => "Ouvrir dans le Finder",
        Text::KontextImFinderAnzeigen => "Afficher dans le Finder",
        Text::BereichLesezeichen => "Signets",
        Text::BereichLinks => "Gauche",
        Text::BereichRechts => "Droite",
        Text::BereichVorschau => "Aperçu",
        Text::BereichEditor => "Éditeur",
        Text::BereichGit => "Git",
        Text::BereichLesezeichenLang => "la barre des signets et des volumes",
        Text::BereichLinksLang => "le volet de fichiers gauche",
        Text::BereichRechtsLang => "le volet de fichiers droit",
        Text::BereichVorschauLang => "la fenêtre d’aperçu",
        Text::BereichEditorLang => "l’éditeur intégré",
        Text::BereichGitLang => "la zone Git",
        Text::SpalteName => "Nom",
        Text::SpalteGroesse => "Taille",
        Text::SpalteDatum => "Date",
        Text::SpalteTyp => "Type",
        Text::SpalteMarke => "Marque",
        Text::SpalteAenderungsdatum => "Date de modification",
        Text::LeisteBereichUmschalten => "Afficher ou masquer {bereich}",
        Text::LeisteSpalteUmschalten => {
            "Afficher ou masquer la colonne «\u{a0}{spalte}\u{a0}» dans les deux listes de fichiers"
        }
        Text::LeisteTiefeHinweis => "Étendre le filtre en cours à la sous-arborescence",
        Text::LeisteInhaltHinweis => "Appliquer le filtre en cours aussi au contenu des fichiers",
        Text::LeisteUeberschriftLesezeichen => "Signets",
        Text::LeisteUeberschriftGeraete => "Volumes et emplacements",
        Text::LeisteLesezeichenFehlt => "{name} (absent)",
        Text::LeisteSinnbildTextstelle => "passage de texte",
        Text::TypOrdner => "dossier",
        Text::TypDatei => "fichier",
        Text::TypVerknuepfung => "lien symbolique",
        Text::EintragsspalteAufgabe => "Tâche",
        Text::EintragsspalteThema => "Sujet",
        Text::EintragsspalteNotiz => "Note",
        Text::EintragsspalteDatum => "Date",
        Text::EintragsspalteTermin => "Rendez-vous",
        Text::BelegungReserviertEditor => "l’Éditeur",
        Text::BelegungZusatzReserviert => "(réservé pour {wofuer})",
        Text::BelegungZustellerMenue => "raccourci du menu",
        Text::BelegungZusatzZusteller => "({zusteller})",
        Text::BelegungKeineFunktionGewaehlt => "aucune fonction n’est sélectionnée",
        Text::BelegungSucheLeer => {
            "Le texte de recherche est vide\u{202f}; chaque caractère saisi lance la recherche."
        }
        Text::BelegungSucheTreffer => {
            "Recherche «\u{a0}{text}\u{a0}»\u{a0}: résultat {stelle} sur {anzahl}."
        }
        Text::BelegungSucheKeinTreffer => "Recherche «\u{a0}{text}\u{a0}»\u{a0}: aucun résultat.",
        Text::BelegungsansichtTitel => "Raccourcis clavier",
        Text::BelegungsansichtZuweisen => "Attribuer",
        Text::BelegungsansichtAuslieferungszustand => "Version livrée",
        Text::BelegungsansichtFertig => "Terminé",
        Text::BelegungsansichtTasteEingabe => "Entrée",
        Text::BelegungsansichtErlaeuterung => {
            "Chaque caractère saisi cherche dans les deux colonnes et va au premier résultat\u{202f}; la touche Entrée passe au suivant, la touche de retour arrière raccourcit le texte de recherche. Les flèches choisissent la fonction. {zuweisen} ({zuweisen_taste}) enregistre la prochaine combinaison pressée\u{202f}; esc annule la saisie. {zuruecksetzen} ({zuruecksetzen_taste}) réinitialise tout. {fertig} ({fertig_taste}) ou esc quitte la vue et enregistre les modifications."
        }
        Text::BelegungsansichtErstWaehlen => {
            "Sélectionner d’abord une fonction, puis appuyer sur Attribuer."
        }
        Text::BelegungsansichtAufnahme => {
            "Appuyer maintenant sur la combinaison voulue pour «\u{a0}{name}\u{a0}»\u{202f}; esc annule."
        }
        Text::BelegungsansichtZurueckgesetzt => {
            "Les raccourcis sont réinitialisés à la version livrée."
        }
        Text::BelegungsansichtAufnahmeAbgebrochen => {
            "La saisie est annulée\u{202f}; les raccourcis sont inchangés."
        }
        Text::BelegungsansichtKeineFunktion => {
            "Aucune fonction n’est sélectionnée\u{202f}; les raccourcis sont inchangés."
        }
        Text::BelegungsansichtZugewiesen => {
            "«\u{a0}{funktion}\u{a0}» est maintenant sur {kombination}."
        }
        Text::BelegungsansichtTasteOhneNamen => {
            "Cette touche n’a pas de nom dans la notation des combinaisons et ne peut pas être enregistrée\u{202f}; les raccourcis sont inchangés."
        }
        Text::BelegungsansichtSpalteFunktion => "Fonction",
        Text::BelegungsansichtSpalteBelegung => "Raccourci",
        Text::MarkdownUeberschrift => "# Raccourcis clavier de KRK",
        Text::MarkdownTabellenkopf => "| Fonction | Combinaisons | Agit dans |",
        Text::MarkdownNichtEingeordnet => "(non classé par KRK)",
        Text::MarkdownWirktTextfelderUndEditor => "champs de texte et Éditeur",
        Text::MarkdownGeschrieben => "Raccourcis clavier enregistrés\u{a0}: {pfad}",
        Text::MarkdownKeinBenutzerverzeichnis => {
            "les raccourcis clavier n’ont pas pu être enregistrés\u{a0}: le système ne nomme aucun dossier de départ"
        }
        Text::MarkdownOrdnerFehlt => {
            "les raccourcis clavier n’ont pas pu être enregistrés\u{a0}: le dossier de {pfad} est absent"
        }
        Text::MarkdownZugriffAbgelehnt => {
            "les raccourcis clavier n’ont pas pu être enregistrés\u{a0}: l’accès à {pfad} est refusé"
        }
        Text::MarkdownFehlgeschlagen => {
            "les raccourcis clavier n’ont pas pu être enregistrés dans {pfad}\u{a0}: {grund}"
        }
        Text::TabelleKeineDateiAufDatentraeger => {
            "la source ne fournit aucun fichier sur le volume"
        }
        Text::TabelleNichtZuOeffnen => "{pfad} ne peut pas être ouvert\u{a0}: {grund}",
        Text::TabelleZwischenablageLeer => "le presse-papiers est vide",
        Text::TabelleNichtAnBrowser => {
            "{adresse} n’a pas pu être transmis au navigateur du système"
        }
        Text::TabelleZwischenablageKeinZiel => {
            "le presse-papiers ne contient ni chemin absolu ni adresse web"
        }
        Text::TabelleNichtInDerListe => "{name} ne figure pas dans la liste",
        Text::FenstertitelQuicknote => "Quicknote",
        Text::StatuszeileFilterstand => {
            "Filtre «\u{a0}{filtertext}\u{a0}»\u{a0}: {gezeigt} sur {vorhanden} affichés{liest}{zu_gross}{ausgeblendet}"
        }
        Text::StatuszeileInhaltWirdGelesen => ", contenu en cours de lecture",
        Text::StatuszeileSeiteVon => "Page {aktuell} sur {gesamt}",
        Text::StatuszeileBildVon => "Image {aktuell} sur {gesamt}",
        Text::StatuszeileFolgeGekuerzt => "{grundsatz} (série tronquée {gruende})",
        Text::StatuszeileLinkesDateifenster => "volet de fichiers gauche",
        Text::StatuszeileRechtesDateifenster => "volet de fichiers droit",
        Text::StatuszeileMeldungMitSeite => "{seite}\u{a0}: {text}",
        Text::VorgangsartKopieren => "Copie",
        Text::VorgangsartVerschieben => "Déplacement",
        Text::VorgangsartInDenPapierkorb => "Mise à la Corbeille",
        Text::VorgangsartUmbenennen => "Renommage",
        Text::VorgangsartPacken => "Compression",
        Text::VorgangsartEntpacken => "Décompression",
        Text::VorgangsartDuplizieren => "Duplication",
        Text::VorgangAbbruchhinweis => "Esc annule",
        Text::VorgangWirdVorbereitet => "{was} en préparation\u{a0}: {positionen} · {abbruch}",
        Text::VorgangZeile => {
            "{was}\u{a0}: {eintraege}, {menge}, {positionen} · {name} · {abbruch}"
        }
        Text::VorgangWirdAbgebrochen => {
            "{was} en cours d’annulation, l’opération se termine dans un instant…"
        }
        Text::VorgangSchonEiner => "une opération est déjà en cours\u{a0}: {was}",
        Text::VorgangUebertragen => "{eintraege}, {menge} ({positionen})",
        Text::VorgangAbgebrochen => "{was}\u{a0}: annulé, {uebertragen} transférés",
        Text::VorgangFertig => "{was}\u{a0}: terminé, {uebertragen}",
        Text::UebersprungenZeile => "{name}\u{a0}: {grund}",
        Text::AnlegenFrageOrdner => "Quel nom donner au nouveau dossier\u{202f}?",
        Text::AnlegenFrageDatei => "Quel nom donner au nouveau fichier\u{202f}?",
        Text::AnlegenBestaetigen => "Créer",
        Text::AngelegtOrdner => "Dossier «\u{a0}{name}\u{a0}» créé",
        Text::AngelegtDatei => "Fichier «\u{a0}{name}\u{a0}» créé",
        Text::AnlegenKeineRechteOrdner => {
            "droits insuffisants pour créer ici le dossier «\u{a0}{name}\u{a0}»"
        }
        Text::AnlegenKeineRechteDatei => {
            "droits insuffisants pour créer ici le fichier «\u{a0}{name}\u{a0}»"
        }
        Text::AnlegenGescheitert => "«\u{a0}{name}\u{a0}» n’a pas pu être créé\u{a0}: {fehler}",
        Text::NameSchonVergeben => "une entrée nommée «\u{a0}{name}\u{a0}» existe déjà",
        Text::DuplikatFrage => "Quel nom donner au duplicata\u{202f}?",
        Text::DuplikatBestaetigen => "Dupliquer",
        Text::DuplikatMehrere => {
            "rien à dupliquer\u{a0}: plusieurs entrées sont marquées, et on ne duplique qu’un seul fichier"
        }
        Text::DuplikatNichtGewoehnlich => {
            "rien à dupliquer\u{a0}: «\u{a0}{name}\u{a0}» est {typ}, et on ne duplique qu’un fichier ordinaire"
        }
        Text::DuplikatTypOrdner => "un dossier",
        Text::DuplikatTypVerknuepfung => "un lien symbolique",
        Text::UmbenennenKeineRechte => {
            "droits insuffisants pour renommer ici en «\u{a0}{name}\u{a0}»"
        }
        Text::UmbenennenGescheitert => {
            "«\u{a0}{name}\u{a0}» n’a pas pu être attribué\u{a0}: {fehler}"
        }
        Text::OrdnerKeinOrdnerMehr => "{pfad} n’est plus un dossier",
        Text::OrdnerNichtMehrErreichbar => "{pfad} n’est plus accessible\u{a0}: {fehler}",
        Text::KeinTerminal => {
            "aucune application portant l’identifiant de bundle «\u{a0}{kennung}\u{a0}» n’est installée\u{202f}; settings.toml la nomme sous terminal, une modification ne prend effet qu’après un redémarrage"
        }
        Text::PfadKopiert => "Chemin copié\u{a0}: {pfad}",
        Text::PfadeKopiert => "{n} chemins copiés",
        Text::NichtsBetroffen => "rien {nennform}\u{a0}: rien de marqué ni de sélectionné",
        Text::NennformZuKopieren => "à copier",
        Text::NennformZuOeffnen => "à ouvrir",
        Text::NennformZuPacken => "à compresser",
        Text::NennformAnzuzeigen => "à afficher",
        Text::NennformZuDuplizieren => "à dupliquer",
        Text::NichtsZuTeilen => {
            "rien à partager\u{a0}: rien ici ne peut être transmis aux services de partage"
        }
        Text::KeinArchiv => "rien à décompresser\u{a0}: aucun fichier portant l’extension .zip ici",
        Text::MehrereArchive => {
            "rien à décompresser\u{a0}: plusieurs archives sont ici, et la sélection n’en désigne aucune"
        }
        Text::KeinFinder => {
            "le Finder n’est pas accessible\u{a0}: le système n’a nommé aucune application pour cela"
        }
        Text::AblageWeistTextAb => "le presse-papiers n’a pas accepté le texte",
        Text::AbgelegtEiner => "copié\u{a0}: {name}",
        Text::AbgelegtMehrere => "{n} entrées copiées",
        Text::AbgelegtAusgeschnitten => {
            "{kopiert} – c’est la destination qui déplace (Finder\u{a0}: opt+cmd+v)"
        }
        Text::AblageWeistVerweiseAb => "le presse-papiers n’a pas accepté les entrées",
        Text::EinfuegenKeinText => {
            "rien à coller\u{a0}: le presse-papiers ne contient pas de texte"
        }
        Text::EinfuegenMehrzeilig => "non collé\u{a0}: le texte comporte plusieurs lignes",
        Text::EinfuegenNichtsTragbar => {
            "rien à coller\u{a0}: le texte ne contient aucun caractère qu’un nom puisse porter"
        }
        Text::UebergebenEiner => "transmis au système\u{a0}: {name}",
        Text::UebergebenMehrere => "{n} entrées transmises au système",
        Text::NichtAngenommenEiner => "le système n’a pas accepté {name}",
        Text::NichtAngenommenMehrere => "le système n’a pas accepté {n} entrées sur {gesamt}",
        Text::UebergebenUndAbgelehnt => "{genommen}\u{202f}; {abgelehnt}",
        Text::KeineAnwendung => {
            "rien à ouvrir\u{a0}: le système ne nomme aucune application pour cette entrée"
        }
        Text::UebergebenAnEiner => "transmis à {anwendung}\u{a0}: {name}",
        Text::UebergebenAnMehrere => "{n} entrées transmises à {anwendung}",
        Text::NichtUebergebenAn => {
            "non transmis à {anwendung}\u{a0}: un chemin n’est pas en UTF-8 valide"
        }
        Text::BelegungsdateiZweiSchreiber => {
            "{datei} a deux rédacteurs\u{a0}: une modification à la main ne prend effet qu’au prochain démarrage, et la vue des raccourcis (F1) l’écrase en la quittant"
        }
        Text::BelegungsdateiFehltNoch => {
            "{datei} n’existe pas encore\u{a0}: il est créé dès que la vue des raccourcis (F1) est quittée avec une modification"
        }
        Text::BelegungsdateiOhneAblageordner => {
            "{datei} ne peut pas être affiché\u{a0}: KRK fonctionne sans dossier de données"
        }
        Text::Markierungsstand => "{n} marqués, dont {ordner}, {groesse}",
        Text::LoeschenOhnePapierkorb => {
            "la destination n’a pas de Corbeille, rien n’a été supprimé\u{202f}; supprimer dans le Finder"
        }
        Text::WarngrundUnentscheidbar => "depuis une destination de nature inconnue",
        Text::WarngrundNetzlaufwerk => "depuis un volume réseau",
        Text::WarngrundCloudort => "depuis un dossier cloud",
        Text::WarngrundAusserhalbBenutzerordner => "hors du dossier de départ",
        Text::WarngrundImBenutzerordner => "directement dans le dossier de départ",
        Text::WarngrundArbeitsbaum => "depuis une arborescence de travail Git",
        Text::WarngrundGenauDieSchwelle => "avec 25 entrées au total",
        Text::WarngrundMehrAlsDieSchwelle => "avec plus de 25 entrées au total",
        Text::LoeschenGeraeumtAus => "Suppression depuis {ordner}.",
        Text::LoeschenAusserdem => "De plus\u{a0}: {gruende}.",
        Text::LoeschenDarunterOrdner => "Dont {ordner}, chacun avec tout son contenu.",
        Text::BlattSteht => "non exécuté\u{a0}: une feuille est ouverte sur la fenêtre",
        Text::PfadNichtAbsolut => "{pfad} n’est pas un chemin absolu",
        Text::PfadGibtEsNicht => "{pfad} n’existe pas\u{a0}: {fehler}",
        Text::PfadNichtLesbar => "{pfad} ne peut pas être lu\u{a0}: {fehler}",
        Text::PfadInKeinemOrdner => "{pfad} ne se trouve dans aucun dossier",
        Text::WerksSchaltflaeche => "Réinitialiser",
        Text::WerksNotizordnerBleibt => {
            "Le dossier de notes et tous les fichiers qu’il contient restent intacts."
        }
        Text::WerksFrage => {
            "Réinitialiser readers.toml, settings.toml et keymap.toml aux réglages d’usine\u{202f}?"
        }
        Text::WerksErlaeuterung => {
            "Chacun des trois fichiers présents dans le dossier de données est mis de côté par KRK sous son nom suivi d’un horodatage, par exemple readers.toml.AAMMJJ-HHMM, et aucun n’est supprimé. Ensuite, readers.toml et settings.toml sont tels que cette version de KRK les livre, sauf que settings.toml garde le dossier de notes réglé\u{202f}; keymap.toml est absent, et les raccourcis clavier livrés s’appliquent. {notizordner} KRK relit le nouvel état immédiatement."
        }
        Text::WerksEigeneZuweisungen => {
            "Toutes les attributions de touches personnelles de keymap.toml cessent ainsi de s’appliquer\u{202f}; elles ne subsistent ensuite que dans la sauvegarde."
        }
        Text::TabAusgefiltert => "{name} est filtré.",
        Text::TabNichtMehrDa => "{name} n’est plus là.",
        Text::TabNichtVollstaendigGelesen => {
            "{ordner} n’a pas pu être lu entièrement\u{a0}: {fehler}"
        }
        Text::PapierkorbKeinUtf8Pfad => "{pfad} n’est pas un chemin UTF-8 valide",
        Text::BlattSchliessen => "Fermer",
        Text::BlattAbbrechen => "Annuler",
        Text::BlattUmbenennen => "Renommer",
        Text::BlattFeldSuchenNach => "Rechercher\u{a0}:",
        Text::BlattFeldErsetzenDurch => "Remplacer par\u{a0}:",
        Text::KonfliktUeberspringen => "Ignorer",
        Text::KonfliktInDenPapierkorbUndErsetzen => "Mettre à la Corbeille et remplacer",
        Text::KonfliktEndgueltigLoeschenUndErsetzen => "Supprimer définitivement et remplacer",
        Text::KonfliktTastenhinweisEinZiel => {
            "Entrée et Esc annulent, Cmd+Entrée remplace, Opt+Entrée renomme."
        }
        Text::KonfliktTastenhinweisMehrereZiele => {
            "Entrée ignore, Cmd+Entrée remplace, Opt+Entrée renomme, Esc annule."
        }
        Text::KonfliktFrage => "«\u{a0}{name}\u{a0}» existe déjà à la destination",
        Text::KonfliktErlaeuterung => {
            "Source\u{a0}: {quelle}\nDestination\u{a0}: {ziel}\n\n{hinweis}"
        }
        Text::KonfliktFuerAlleWeiteren => "Appliquer à toutes les suivantes",
        Text::LoeschblattErlaeuterung => {
            "{erlaeuterung}\n\nEntrée et Esc annulent. Cmd+Entrée pour confirmer."
        }
        Text::NeuerungenBlattFrage => "Vos fichiers de données et ce que cette version apporte",
        Text::OrtwahlWaehlen => "Choisir",
        Text::OrtwahlFrage => "Où placer le dossier de notes\u{202f}?",
        Text::PinHinweis => {
            "Le code PIN empêche les programmes et les agents de lire le contenu. Il ne protège pas contre quelqu’un qui copie le fichier et l’attaque délibérément. Un code PIN oublié verrouille le contenu définitivement."
        }
        Text::PinAbweichung => "Les deux saisies ne correspondent pas.",
        Text::PinFrageFestlegen => "Définir un nouveau code PIN pour les secrets",
        Text::PinFrageEingeben => "Saisir le code PIN des secrets",
        Text::PinFrageAendern => "Modifier le code PIN des secrets",
        Text::PinBestaetigenFestlegen => "Définir",
        Text::PinBestaetigenEingeben => "Ouvrir",
        Text::PinBestaetigenAendern => "Modifier",
        Text::PinFeldNeuePin => "Nouveau code PIN\u{a0}:",
        Text::PinFeldWiederholen => "Répéter\u{a0}:",
        Text::PinFeldPin => "Code PIN\u{a0}:",
        Text::PinFeldAltePin => "Ancien code PIN\u{a0}:",
        Text::StapelSpalteBisher => "Actuel",
        Text::StapelSpalteNeu => "Nouveau",
        Text::StapelSpalteHinweis => "Remarque",
        Text::StapelErlaeuterung => {
            "L’aperçu montre ce que ferait la commande. Le renommage n’a lieu qu’avec Entrée\u{202f}; Esc annule. Les entrées accompagnées d’une remarque restent inchangées."
        }
        Text::StapelFeldNummerAb => "Numéro à partir de\u{a0}:",
        Text::StapelFeldStellen => "Chiffres\u{a0}:",
        Text::StapelZusammenfassungOhneKollisionen => {
            "{eintraege}, dont {umzubenennen} avec un nouveau nom"
        }
        Text::StapelZusammenfassungMitKollisionen => "{eintraege}\u{a0}: {umbenannt}, {stehend}",
        Text::SucheWeitersuchen => "Rechercher le suivant",
        Text::SucheErsetzen => "Remplacer",
        Text::SucheAlleErsetzen => "Tout remplacer",
        Text::SucheErlaeuterung => {
            "Entrée cherche le suivant, Cmd+Entrée remplace le résultat, Opt+Entrée remplace tout, Esc annule."
        }
        Text::SucheFrage => "Que rechercher\u{202f}?",
        Text::UngesichertSichern => "Enregistrer",
        Text::UngesichertVerwerfen => "Ne pas enregistrer",
        Text::UngesichertFrage => "«\u{a0}{name}\u{a0}» a des modifications non enregistrées",
        Text::UngesichertErlaeuterung => {
            "{pfad}\n\nEntrée enregistre, Cmd+Entrée abandonne les modifications, Esc annule."
        }
        Text::ZeilennummerFrage => "À quelle ligne\u{202f}?",
        Text::ZeilennummerSpringe => "Aller",
        Text::PfadeingabeFrage => "Vers quel dossier\u{202f}?",
        Text::PfadeingabeGehe => "Aller",
        Text::HinweisOk => "OK",
        Text::EditorTrefferVon => "Résultat {nummer} sur {anzahl}",
        Text::EditorKeinTrefferFuer => "Aucun résultat pour «\u{a0}{text}\u{a0}»",
        Text::EditorKeinWeitererTrefferFuer => "Aucun autre résultat pour «\u{a0}{text}\u{a0}»",
        Text::PinBleibt => "le code PIN reste tel qu’il était",
        Text::PinNichtAbleitbarGrund => {
            "le nouveau code PIN ne peut pas être dérivé\u{a0}: {fehler}\u{202f}; {bleibt}"
        }
        Text::EditorKeineTextmarkeInGeheimnissen => {
            "pour secrets.txt, KRK ne crée aucun signet de texte\u{a0}: il écrirait une ligne des secrets en clair dans les signets"
        }
        Text::EditorOhnePin => "il est chiffré et ne s’ouvre qu’avec le code PIN",
        Text::EditorKeineVerschluesselteDatei => {
            "ce n’est pas un fichier chiffré du dossier de notes"
        }
        Text::EditorGeheimnisseZuGross => "il est trop volumineux pour l’Éditeur",
        Text::EditorGeheimnisseKeinDateizugriff => "KRK n’a plus de descripteur de fichier libre",
        Text::EditorGeheimnisseNichtLesbar => "il ne peut pas être lu",
        Text::EditorFremdGeaendertNichtUeberschrieben => {
            "{pfad} a été modifié en dehors de KRK et ne sera pas écrasé"
        }
        Text::EditorVerschluesseltKeinKlartext => {
            "{pfad} est chiffré et ne sera pas écrit en clair"
        }
        Text::EditorNichtGesichert => "{pfad} n’a pas pu être enregistré\u{a0}: {fehler}",
        Text::EditorFremdGeaendert => "{pfad} a été modifié en dehors de KRK",
        Text::PinKeineDatei => "l’Éditeur ne tient aucun fichier\u{202f}; {bleibt}",
        Text::PinNochNichtGesichert => {
            "{pfad} ne porte pas encore de code PIN enregistré\u{202f}; enregistrer d’abord, modifier ensuite"
        }
        Text::PinWirdSchonGeaendert => "le code PIN est déjà en cours de modification",
        Text::PinFremdGeaendert => "{pfad} a été modifié en dehors de KRK\u{202f}; {bleibt}",
        Text::PinNichtVerschluesselt => "{pfad} n’est pas chiffré\u{202f}; {bleibt}",
        Text::PinAlteStimmtNicht => "l’ancien code PIN est incorrect\u{202f}; {bleibt}",
        Text::PinNichtAbleitbar => "le nouveau code PIN n’a pas pu être dérivé\u{202f}; {bleibt}",
        Text::PinGrundBleibt => "{grund}\u{202f}; {bleibt}",
        Text::PinDateiGrundBleibt => "{pfad}\u{a0}: {grund}\u{202f}; {bleibt}",
        Text::PinNichtMehrOffen => "{pfad} n’est plus ouvert\u{202f}; {bleibt}",
        Text::PinNichtMehrEntsperrt => "{pfad} n’est plus déverrouillé\u{202f}; {bleibt}",
        Text::PinNichtLesbar => "{pfad} ne peut pas être lu\u{202f}; {bleibt}",
        Text::PinNichtAlsTextLesbar => "{pfad} n’est pas lisible comme texte\u{202f}; {bleibt}",
        Text::PinNichtGeschrieben => {
            "{pfad} n’a pas pu être écrit\u{a0}: {fehler}\u{202f}; {bleibt}"
        }
        Text::EditorMarkeFuehrtAufZeile => "le signet mène à la ligne {zeile}",
        Text::EditorZeilenZaehlenAbEins => {
            "les lignes se comptent à partir de 1\u{202f}; le curseur est au début du fichier"
        }
        Text::EditorKeineZeileMehr => {
            "le fichier n’a plus de ligne {zeile}\u{202f}; le curseur est à la fin du fichier"
        }
        Text::EditorMarkenstelleGeaendert => "le passage mémorisé a changé\u{202f}; {wohin}",
        Text::EditorGesichert => "{pfad} enregistré",
        Text::EditorKeineZeilennummer => "«\u{a0}{eingabe}\u{a0}» n’est pas un numéro de ligne",
        Text::EditorKeineSuche => "aucune recherche n’est en cours",
        Text::EditorKeinTrefferErsetzt => "aucun résultat remplacé",
        Text::EditorPinGeaendert => "le code PIN de {pfad} est modifié",
        Text::EditorTermineAufsteigend => "Rendez-vous triés par ordre croissant",
        Text::EditorTermineAbsteigend => "Rendez-vous triés par ordre décroissant",
        Text::QuicknoteLeer => {
            "La Quicknote est vide\u{202f}; le presse-papiers reste tel qu’il était."
        }
        Text::QuicknoteNichtKopiert => {
            "La Quicknote n’a pas pu être copiée dans le presse-papiers\u{202f}; son texte reste en place."
        }
        Text::QuicknoteZuGross => {
            "Le texte est trop long pour la Quicknote\u{202f}; rien n’a été collé."
        }
        Text::QuicknoteKeineTextmarken => "Dans la Quicknote, il n’y a pas de signets de texte.",
        Text::QuicknoteLeeren => "Vider",
        Text::QuicknoteKopieren => "Copier",
        Text::EintragKeineAufgabentabelle => "l’Éditeur n’affiche pas de tableau des tâches",
        Text::EintragKeineNotiztabelle => "l’Éditeur n’affiche pas de tableau des notes",
        Text::EintragKeineTermintabelle => "l’Éditeur n’affiche pas de tableau des rendez-vous",
        Text::EintragKeineAufgabeGewaehlt => "aucune tâche n’est sélectionnée",
        Text::EintragKeineNotizGewaehlt => "aucune note n’est sélectionnée",
        Text::EintragKeinTerminGewaehlt => "aucun rendez-vous n’est sélectionné",
        Text::EintragAufgabeHinzugefuegt => {
            "nouvelle tâche à la fin\u{202f}; return valide le texte"
        }
        Text::EintragNotizHinzugefuegt => {
            "nouvelle note à la fin\u{202f}; tab passe au texte, cmd+return valide"
        }
        Text::EintragTerminHinzugefuegt => {
            "rendez-vous ajouté, à la date du jour\u{202f}; tab passe au texte, cmd+return valide"
        }
        Text::EintragAufgabeBearbeitung => "return valide, esc abandonne",
        Text::EintragNotizBearbeitung => {
            "cmd+return valide, tab change de cellule, return insère un saut de ligne dans le texte"
        }
        Text::EintragTerminBearbeitung => {
            "cmd+return valide, tab change de cellule, return insère un saut de ligne dans le rendez-vous"
        }
        Text::EintragAufgabeUebernommen => "tâche validée",
        Text::EintragNotizUebernommen => "note validée",
        Text::EintragTerminUebernommen => "rendez-vous validé",
        Text::EintragAufgabeAbgehakt => "tâche cochée",
        Text::EintragNotizOhneKaestchen => "une note n’a pas de case à cocher",
        Text::EintragTerminOhneKaestchen => "un rendez-vous n’a pas de case à cocher",
        Text::EintragAufgabeWiederOffen => "tâche de nouveau ouverte",
        Text::EintragAufgabeVerschoben => "tâche déplacée",
        Text::EintragNotizVerschoben => "note déplacée",
        Text::EintragAufgabeSchonOben => "la tâche est déjà en haut",
        Text::EintragNotizSchonOben => "la note est déjà en haut",
        Text::EintragAufgabeSchonUnten => "la tâche est déjà en bas",
        Text::EintragNotizSchonUnten => "la note est déjà en bas",
        Text::EintragTermineNachDatum => {
            "Les rendez-vous sont classés par date\u{202f}; aucun ne peut être déplacé."
        }
        Text::EintragAufgabeGeloescht => "tâche supprimée\u{202f}; cmd+z la rétablit",
        Text::EintragNotizGeloescht => "note supprimée\u{202f}; cmd+z la rétablit",
        Text::EintragTerminGeloescht => "rendez-vous supprimé\u{202f}; cmd+z le rétablit",
        Text::EintragZelleBleibt => "la cellule reste en cours de modification",
        Text::EintragAufgabeMitEscUebernommen => "tâche validée\u{202f}; cmd+z annule",
        Text::EintragNotizMitEscUebernommen => "note validée\u{202f}; cmd+z annule",
        Text::EintragTerminMitEscUebernommen => "rendez-vous validé\u{202f}; cmd+z annule",
        Text::EintragHeuteUnbestimmt => "La date du jour n’a pas pu être déterminée.",
        Text::VorschauGeheimnishinweis => {
            "Ce fichier est chiffré et s’ouvre avec F4 et le code PIN dans l’Éditeur."
        }
        Text::VorschautabLeer => "Vide",
        Text::VorschautabZwischenablage => "Presse-papiers",
        Text::VorschauZwischenablageLeer => "Le presse-papiers est vide.",
        Text::VorschauBildZuGross => {
            "L’image du presse-papiers pèse {groesse} Mo. L’Aperçu affiche les images jusqu’à {grenze} Mo."
        }
        Text::VorschauNichtLesbar => "{pfad} n’a pas pu être lu\u{a0}: {fehler}",
        Text::VorschauLeertext => {
            "Aucun contenu. La sélection dans le volet de fichiers remplit cet onglet."
        }
        Text::VorschauBildNichtDarstellbar => "L’image du presse-papiers n’a pas pu être affichée.",
        Text::MetadatenName => "Nom\u{a0}: {name}",
        Text::MetadatenPfad => "Chemin\u{a0}: {pfad}",
        Text::MetadatenGroesse => "Taille\u{a0}: {groesse}",
        Text::MetadatenGeaendert => "Modifié\u{a0}: {datum}",
        Text::MetadatenRechte => "Droits\u{a0}: {rechte}",
        Text::MetadatenTyp => "Type\u{a0}: {typ}",
    }
}

/// Einzahl und Mehrzahl zu einem franzoesischen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} octet", "{n} octets"),
        Zahlwort::NeuerungenEintraegeIn => ("{n} entrée dans {datei}", "{n} entrées dans {datei}"),
        Zahlwort::HeimZettelUebernommen => (
            "{themen} repris comme note dans {notizen}",
            "{themen} repris comme notes dans {notizen}",
        ),
        Zahlwort::MarkeGeaendert => ("{n} modifié", "{n} modifiés"),
        Zahlwort::MarkeVorgemerkt => ("{n} indexé", "{n} indexés"),
        Zahlwort::MarkeNeu => ("{n} nouveau", "{n} nouveaux"),
        Zahlwort::MarkeKonflikt => ("{n} en conflit", "{n} en conflit"),
        Zahlwort::MarkeUmbenannt => ("{n} renommé", "{n} renommés"),
        Zahlwort::StatuszeileDateienZuGross => (
            ", un fichier trop volumineux",
            ", {n} fichiers trop volumineux",
        ),
        Zahlwort::StatuszeileMarkierungenAusgeblendet => {
            (", une marque masquée", ", {n} marques masquées")
        }
        Zahlwort::BildfolgeGrenzeFotos => ("après {n} photo", "après {n} photos"),
        Zahlwort::BildfolgeGrenzeOrdner => ("après {n} dossier", "après {n} dossiers"),
        Zahlwort::BildfolgeGrenzeEintraege => {
            ("à {n} entrée d’un dossier", "à {n} entrées d’un dossier")
        }
        Zahlwort::Eintraege => ("une entrée", "{n} entrées"),
        Zahlwort::AusgewaehltePositionen => {
            ("une position sélectionnée", "{n} positions sélectionnées")
        }
        Zahlwort::Ordner => ("un dossier", "{n} dossiers"),
        Zahlwort::VorgangUebersprungen => (", une entrée ignorée", ", {n} entrées ignorées"),
        Zahlwort::VorgangAusgelassen => (
            ", une entrée écartée comme destination de ce passage",
            ", {n} entrées écartées comme destination de ce passage",
        ),
        Zahlwort::UebersprungenFrage => {
            ("Une entrée a été ignorée", "{n} entrées ont été ignorées")
        }
        Zahlwort::EinfuegenDateiverweise => (
            "non collé\u{a0}: le presse-papiers contient {n} référence de fichier",
            "non collé\u{a0}: le presse-papiers contient {n} références de fichiers",
        ),
        Zahlwort::LoeschfrageEintraege => (
            "Mettre cette entrée {grund}à la Corbeille\u{202f}?",
            "Mettre ces {n} entrées {grund}à la Corbeille\u{202f}?",
        ),
        Zahlwort::StartMeldungen => (
            "Il y a eu un message au démarrage",
            "Il y a eu {n} messages au démarrage",
        ),
        Zahlwort::StapelFrage => ("Renommer une entrée", "Renommer {n} entrées par lot"),
        Zahlwort::StapelEintraege => ("{n} entrée", "{n} entrées"),
        Zahlwort::StapelWerdenUmbenannt => ("{n} sera renommée", "{n} seront renommées"),
        Zahlwort::StapelBleibenStehen => ("{n} reste inchangée", "{n} restent inchangées"),
        Zahlwort::EditorTrefferErsetzt => ("un résultat remplacé", "{n} résultats remplacés"),
        Zahlwort::EditorZeilenHinterDerLetzten => (
            "le fichier a {n} ligne\u{202f}; le curseur est à la fin du fichier",
            "le fichier a {n} lignes\u{202f}; le curseur est à la fin du fichier",
        ),
        Zahlwort::QuicknoteKopiert => (
            "La Quicknote est dans le presse-papiers\u{a0}: {n} caractère.",
            "La Quicknote est dans le presse-papiers\u{a0}: {n} caractères.",
        ),
        Zahlwort::BildfolgeVorbereitet => (
            "La série de photos se prépare\u{a0}: {n} photo.",
            "La série de photos se prépare\u{a0}: {n} photos.",
        ),
    }
}

/// Der franzoesische Name eines Kommandos: der Eintrag des Hauptmenues, der
/// Belegungsansicht und jeder Konfliktmeldung.
pub(in super::super) const fn kommandoname(schluessel: Kommando) -> &'static str {
    match schluessel {
        Kommando::AuswahlHoch => "Sélection une entrée vers le haut",
        Kommando::AuswahlRunter => "Sélection une entrée vers le bas",
        Kommando::SeiteHoch => "Sélection une page d’écran vers le haut",
        Kommando::SeiteRunter => "Sélection une page d’écran vers le bas",
        Kommando::Listenanfang => "Au début de la liste",
        Kommando::Listenende => "À la fin de la liste",
        Kommando::Oeffnen => "Entrer dans le dossier sélectionné",
        Kommando::OrdnerAufwaerts => "Vers le dossier parent",
        Kommando::OrdnerDerDatei => "Afficher le dossier du fichier affiché",
        Kommando::OrdnerAngleichen => "Placer l’autre volet de fichiers sur ce dossier",
        Kommando::Pfadeingabe => "Saisir un chemin et y aller",
        Kommando::MarkierungUmschalten => "Marquer l’entrée et passer à la suivante",
        Kommando::AlleMarkieren => "Marquer toutes les entrées",
        Kommando::MarkierungAufheben => "Supprimer toutes les marques",
        Kommando::MarkierungUmkehren => "Inverser les marques",
        Kommando::SortierungName => "Trier par nom",
        Kommando::SortierungGroesse => "Trier par taille",
        Kommando::SortierungDatum => "Trier par date de modification",
        Kommando::SortierungTyp => "Trier par type",
        Kommando::SortierrichtungUmkehren => "Inverser l’ordre de tri",
        Kommando::VersteckteUmschalten => "Afficher ou masquer les fichiers cachés",
        Kommando::SpalteGroesseUmschalten => "Afficher ou masquer la colonne Taille",
        Kommando::SpalteDatumUmschalten => "Afficher ou masquer la colonne Date de modification",
        Kommando::SpalteTypUmschalten => "Afficher ou masquer la colonne Type",
        Kommando::TiefeSucheUmschalten => "Activer ou désactiver la recherche en profondeur",
        Kommando::InhaltssucheUmschalten => "Activer ou désactiver la recherche dans le contenu",
        Kommando::ZwischenablageSpringen => "Aller au contenu du Presse-papiers",
        Kommando::ZwischenablageAnsehen => "Afficher le Presse-papiers",
        Kommando::TabNeu => "Ouvrir un nouvel onglet",
        Kommando::TabSchliessen => "Fermer l’onglet actif",
        Kommando::TabNaechster => "Onglet suivant",
        Kommando::TabVoriger => "Onglet précédent",
        Kommando::FensterWechseln => "Changer de volet de fichiers actif",
        Kommando::LeisteUmschalten => "Afficher ou masquer la barre des signets et des volumes",
        Kommando::ErstesFensterUmschalten => "Afficher ou masquer le volet de fichiers gauche",
        Kommando::ZweitesFensterUmschalten => "Afficher ou masquer le second volet de fichiers",
        Kommando::VorschauUmschalten => "Afficher ou masquer l’Aperçu",
        Kommando::FensterEinblenden => "Afficher la fenêtre",
        Kommando::FensterSchliessen => "Fermer la fenêtre",
        Kommando::BereichVerbreitern => "Élargir la zone active",
        Kommando::BereichVerschmaelern => "Rétrécir la zone active",
        Kommando::Kopieren => "Copier dans l’autre volet",
        Kommando::Verschieben => "Déplacer vers l’autre volet",
        Kommando::InPapierkorb => "Placer dans la Corbeille",
        Kommando::Abbrechen => "Annuler l’opération en cours",
        Kommando::OrdnerAnlegen => "Nouveau dossier",
        Kommando::DateiAnlegen => "Nouveau fichier vide",
        Kommando::UmbenennenStapel => "Renommer par lot",
        Kommando::Umbenennen => "Renommer",
        Kommando::TerminalOeffnen => "Ouvrir le dossier dans le Terminal",
        Kommando::OrdnerpfadKopieren => "Copier le chemin du dossier affiché",
        Kommando::EintragspfadKopieren => "Copier le chemin de l’entrée",
        Kommando::MitStandardprogrammOeffnen => "Ouvrir avec l’application par défaut",
        Kommando::Teilen => "Partager",
        Kommando::LesezeichenAnlegen => "Ajouter un signet",
        Kommando::LesezeichenUmbenennen => "Renommer le signet",
        Kommando::LesezeichenLoeschen => "Supprimer le signet",
        Kommando::LesezeichenHoch => "Déplacer le signet vers le haut",
        Kommando::LesezeichenRunter => "Déplacer le signet vers le bas",
        Kommando::FokusLeiste => "Focus sur la barre des signets et des volumes",
        Kommando::FokusDateifenster => "Focus de retour sur le volet de fichiers",
        Kommando::FokusVorschau => "Focus sur l’Aperçu",
        Kommando::Bearbeiten => "Modifier",
        Kommando::EditorRundweg => "Vers l’Éditeur et retour",
        Kommando::FokusEditor => "Focus sur l’Éditeur",
        Kommando::EditorSchliessen => "Fermer l’Éditeur",
        Kommando::EditorUmschalten => "Afficher ou masquer l’Éditeur",
        Kommando::EditorAnsichtUmschalten => "Basculer entre vue brute et vue formatée",
        Kommando::EditorSichern => "Enregistrer",
        Kommando::EditorZeileSpringen => "Aller à la ligne",
        Kommando::EditorSuchen => "Rechercher dans le texte",
        Kommando::EditorWeitersuchen => "Rechercher le suivant",
        Kommando::EditorRueckwaertsSuchen => "Rechercher le précédent",
        Kommando::EditorErsetzen => "Remplacer",
        Kommando::EditorAlleErsetzen => "Tout remplacer",
        Kommando::QuicknoteUmschalten => "Ouvrir ou fermer la Quicknote",
        Kommando::QuicknoteKopieren => "Copier la Quicknote et la fermer",
        Kommando::QuicknoteLeeren => "Vider la Quicknote",
        Kommando::EintragHinzufuegen => "Ajouter une entrée",
        Kommando::EintragBearbeiten => "Modifier l’entrée",
        Kommando::EintragHoch => "Entrée vers le haut",
        Kommando::EintragRunter => "Entrée vers le bas",
        Kommando::EintragLoeschen => "Supprimer l’entrée",
        Kommando::AufgabeAbhaken => "Cocher ou rouvrir la tâche",
        Kommando::PinAendern => "Modifier le code PIN",
        Kommando::TermineRichtungUmkehren => "Rendez-vous\u{a0}: inverser l’ordre de tri",
        Kommando::BelegungAnsehen => "Afficher les raccourcis clavier",
        Kommando::BelegungsdateiAnsehen => "Ouvrir le fichier des raccourcis clavier",
        Kommando::Beenden => "Quitter KRK",
        Kommando::WeitereInstanz => "Lancer une autre instance",
        Kommando::Notizordner => "Ouvrir le dossier de notes",
        Kommando::OrtWaehlen => "Choisir l’emplacement…",
        Kommando::VorschauVergroessern => "Agrandir l’Aperçu",
        Kommando::VorschauVerkleinern => "Réduire l’Aperçu",
        Kommando::VorschauAusgangsgroesse => "Aperçu à la taille d’origine",
        Kommando::GitBereichUmschalten => "Afficher ou masquer la zone Git",
        Kommando::FokusGit => "Focus sur la zone Git",
        Kommando::SpalteMarkeUmschalten => "Afficher ou masquer la colonne Marque",
        Kommando::NeuerungenZeigen => "Afficher les nouveautés",
        Kommando::Werkseinstellungen => "Rétablir les réglages d’usine…",
        Kommando::BildVor => "Image suivante",
        Kommando::BildZurueck => "Image précédente",
        Kommando::ZumBild => "Aller à l’image affichée",
    }
}

/// Der franzoesische Name einer vom Hauptmenue zugestellten Funktion.
pub(in super::super) const fn zugestellt_name(schluessel: Zugestellt) -> &'static str {
    match schluessel {
        Zugestellt::FilterEinfuegen => "Coller dans le filtre",
        Zugestellt::TextAusschneiden => "Couper",
        Zugestellt::TextKopieren => "Copier",
        Zugestellt::TextEinfuegen => "Coller",
        Zugestellt::TextAllesAuswaehlen => "Tout sélectionner",
        Zugestellt::TextRueckgaengig => "Annuler",
        Zugestellt::TextWiederholen => "Rétablir",
    }
}
