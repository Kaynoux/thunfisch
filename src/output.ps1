# Definiert den Namen der Ausgabedatei
$outputFile = "alle_rs_dateien.txt"

# Definiert den Dateityp, nach dem gesucht werden soll
$fileType = "*.rs"

# Nachricht an den Benutzer
Write-Host "Suche nach '$fileType' Dateien im aktuellen Verzeichnis und Unterverzeichnissen..."

# Hole alle .rs Dateien rekursiv (nur Dateien, keine Verzeichnisse)
# -Path .        -> Startet im aktuellen Verzeichnis
# -Filter        -> Filtert nach dem Dateityp (effizienter als -Include für einfache Muster)
# -Recurse       -> Durchsucht auch Unterverzeichnisse
# -File          -> Stellt sicher, dass nur Dateien und keine Ordner zurückgegeben werden
$rsFiles = Get-ChildItem -Path . -Filter $fileType -Recurse -File

# Prüfe, ob Dateien gefunden wurden
if ($rsFiles) {
    Write-Host ("{0} '{1}' Dateien gefunden. Füge Inhalte in '{2}' zusammen..." -f $rsFiles.Count, $fileType, $outputFile)

    # Hole den Inhalt aller gefundenen Dateien und schreibe ihn in die Ausgabedatei
    # Get-Content liest den Inhalt der Dateien (kann eine Liste von Datei-Objekten verarbeiten)
    # Out-File schreibt die Ausgabe in eine Datei
    # -Encoding UTF8 -> Stellt sicher, dass die Datei mit UTF-8 Kodierung gespeichert wird (gut für Quellcode)
    # -Force         -> Überschreibt die Ausgabedatei, falls sie bereits existiert
    Get-Content -Path $rsFiles.FullName | Out-File -FilePath $outputFile -Encoding UTF8 -Force

    Write-Host "Fertig! Der Inhalt wurde in '$outputFile' gespeichert."
} else {
    Write-Host "Keine '$fileType' Dateien im aktuellen Verzeichnis oder Unterverzeichnissen gefunden."
}

# Optional: Kurze Pause am Ende, wenn das Skript direkt in einer Konsole ausgeführt wird, die sich sonst sofort schließt.
# Read-Host "Drücke Enter zum Beenden..."
