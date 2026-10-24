# SQLite-Datenbanken im Dateifenster und in der Vorschau durchsehen

## Directive

KRK soll eine SQLite-Datei wie `/Users/k1/Projects/productive/wordlist/wordstore/wordstore.db` durchsehen lassen. Die Idee des Nutzers: Pfeil nach rechts auf der Datei öffnet die Datenbank und listet ihre Tabellen im Dateifenster; steht die Auswahl auf einer Tabelle, zeigt die Vorschau deren Inhalt. Erster Schritt ist eine Analyse, wie sich das in KRK einfügt; erreicht ist das Paket, wenn ein Nutzer so durch die Tabellen einer Datenbank blättern kann, ohne KRK zu verlassen.

Wortlaut des Nutzers (261023):

> make a new package, analys how we could use the preview window to browser sqlite db's like : /Users/k1/Projects/productive/wordlist/wordstore/wordstore.db
>
> my idea would be:
> - when pressing right arrow on file we open the db and list the tables in the file window, when moving on a table the table is beeing displayed in the preview.

## Nutzerwort 261024, nach dem Abschlussreview

> dont think it make sense to export blobs.
> you can go on in automode from here and invlove the consultant as sparring partner.

Folge: BLOB-Inhalte werden weder in der Vorschau noch in der Ausfuhr gezeigt; der Platzhalter `[BLOB, n Bytes]` bleibt, eine Zellgrenze für BLOBs ist damit unnötig. Die Review-Issues werden ohne Rückfrage je Schritt abgearbeitet, Entscheidungen mit dem Consultant als Sparringspartner getroffen und im Bericht an den Nutzer begründet.
