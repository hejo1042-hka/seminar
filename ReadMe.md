Im Verzeichnis src finden sich zwei Testprogramme. Das eine ist eine eigene Implementierung von einer Liste. 
In dieser sind einige offensichtliche Bugs in der remove Methode eingebaut, um die quickcheck Funktionalität zu demonstrieren.
Im zweiten Programm Addition ist die funktionalität zum Addieren zweier zahlen implementiert. 
Dies dient als zweites Beispiel und um zu zeigen, dass auch in triviale Funktion wie der addition Bugs versteckt sein können.

Im Verzeichnis tests, sind die Quickcheck test implementiert. Aufgeteilt in test für die addition und für die Liste.
Verwendet wurde hier die propTest Library für RUST. 
In der Datei quickcheck_test im list verzeichnis, sind quickcheck test mit der Rust Library quickcheck implementiert.

In der mod.rs im list verzeichnis, sind die beiden funktionen own_list und own_list_large Generatoren.
Die anderen mit ```proptest!``` beginnenden funktionen, sind die Testfunktionen, welche mit ```prop_assert!``` die Eigenschaft des Testes überprüfen.
