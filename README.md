# dev22-Ownership_in_Rust
Quellcode zum Artikel von Stefan Baumgartner, iX-Developer-Sonderheft Moderne Programmiersprachen, 2022

# iX-tract
* Managed Memory über Garbage Collection (Java, C#, ...) und manuelle Speicherverwaltung (C, C++) bergen Fallstricke – Rust geht das Speichermanagement auf andere Weise an: mit dem Ownership-Modell.
* Rust prüft zur Übersetzungszeit, welche Speicherbewegungen vorliegen und ob immer eindeutig ist, welche Variable Besitzer des zu verwaltenden Speichers ist. Bei Übertretungen schlägt der Compiler Alarm.
* Das Ownership-Modell erfordert neue Strukturen und ein Umdenken bei Entwicklern, löst aber eine Vielzahl an Problemen.
