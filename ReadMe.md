Im Verzeichnis src finden sich zwei Testprogramme. Das eine ist eine eigene Implementierung von einer Liste. 
In dieser sind einige offensichtliche Bugs in der remove Methode eingebaut, um die quickcheck Funktionalität zu demonstrieren.
Im zweiten Programm Addition ist die funktionalität zum Addieren zweier zahlen implementiert. 
Dies dient als zweites Beispiel und um zu zeigen, dass auch in triviale Funktion wie der addition Bugs versteckt sein können.

Im Verzeichnis tests, sind die Quickcheck test implementiert. Aufgeteilt in test für die addition und für die Liste.
Verwendet wurde hier die propTest Library für RUST. 
In der Datei quickcheck_test im list verzeichnis, sind quickcheck test mit der Rust Library quickcheck implementiert.

In der mod.rs im list verzeichnis, sind die beiden funktionen own_list und own_list_large Generatoren.
Die anderen mit ```proptest!``` beginnenden funktionen, sind die Testfunktionen, welche mit ```prop_assert!``` die Eigenschaft des Testes überprüfen.

## Additions Tests

Da Rust ein extrem sicheres Speichermanagement besitzt, führen die property Tests für die additions Funktionalität zu etwas anderen Ergebnissen, wie in anderen Programmiersprachen. Dies liegt daran, dass Rust einen Overflow beim Addieren zweier Zahlen erkennt und einen Fehler erzeugt. In anderen Programmiersprachen, würde kein Fehler erzuegt werden, sondern eine unerwartete Zahl als Ergebnis berechnet werden.

In den Additions-test in der Datei [addition_test.rs](tests/addition/addition_test.rs), prüft der Test ```prop_neutrales_element```, dass die Addition mit der Null (dem neutralen Element), wieder dieselbe Zahl ergibt.

Der Test ```prop_kommutativität``` überprüft, die Eigenschaft der Kommutativität für die Addition. Also, dass die Reihenfolge der beiden Zahlen in der Addition egal ist.

Da die Addition zweier Zahlen vom Typ unsigned integer (unsigned sorgt für >= 0) immer eine positive Zahl ergeben muss, wird das vom Test ```prop_greater_zero``` überprüft.

Die beiden Test ```prop_kommutativität``` und ```prop_greater_zero``` schlagen beide Fehl, da beim addieren von großen Zahlen ein Overflow entsteht.

## Listen test

Die Tests für die Remove funktion der eigenen Implementierung der Liste, finden sich in der Datei [prop_test.rs](tests/list/prop_test.rs). Die Funktionen ```own_list_large```, ```own_list``` und ```own_list_small``` sind die Generatoren, welche die Testdaten für die eigene Listenimplementation erzeugen.
Der Generator ```own_list_large``` erzeugt dabei eine Liste mit zufälligen Daten aus einem sehr großen Raum. Das führt dazu, dass im Test ```prop_removed_item_not_found_large``` keine Fehler gefunden werden und dieser (fast) immer erfolgreich durchgeführt wird. Das zeigt, dass die Auswahl der Testdaten einen Einfluss auf die Qualität der Test hat und beeinflusst, wie wahrscheinlich Bugs entdeckt werden.

Die Generator-funktion ```own_list_small``` dient dazu, um die Ausgaben eines Testes zu demonstrieren, damit die Ausgaben noch für den Menschen sinnvoll lesbar sind, existiert hier ein eigener Generator, welcher nur kleine Testdaten generiert. Dieser kommt im Property Test ```show_prop_test_cases``` zum Einsatz.

Der Generator ```own_list``` erzeugt die Testdaten für alle anderen Testfälle. Der Property Test ```prop_removed_item_not_found``` überprüft, dass nach dem Löschen eines Elementes, dieses nicht mehr in der Liste gefunden wird. Es wird also überprüft, dass alle vorkommen und nicht nur das erste gelöscht werden.

Im Property Test ```prop_size_reduced``` wird geprüft, dass die Liste nach dem Löschen kleiner wird. Damit die Liste auch wirklich kleiner wird, muss das zu löschende Item in der Liste vorhanden sein. Um das vorhanden sein des Elementes in der Liste sicherzustellen, existiert die Funktionalität ```prop_assume!```. ```prop_assume!``` stellt sicher, dass die übergebene Bedingung erfüllt ist, bevor der Test ausgeführt wird. In diesem Fall ```prop_assume!(list.find(item) != None);```.

Der Property Test ```prop_size_reduced_by_num_occurences``` prüft, dass die Liste beim Löschen eines Elementes um die Anzahl des Elementes kleiner wird. Dadurch kann zum Beispiel geprüft werden, dass nicht einfach immer die leere Liste zurückgegeben wird.
