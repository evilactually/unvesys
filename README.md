cmd --logicdesign "Control Box" --harnessdesign P2 --label --cutlist --fuselist --format --bom 

Connector/Device, 22 GA, 20 GA, 18 GA, 16 GA, 14 GA, 12 GA


cmd --logicdesign "Tiger v1" --harness "P2 Harness" --labels
Open logic design "Tiger v1", filter design by harness attribute, reference harness design for wire length


unvesys --project T2.xml --info
unvesys --project T2.xml --design Tiger --harness N1
unvesys --project T2.xml --design Tiger --info
unvesys --project T2.xml --design Tiger --harness N1 --labels
unvesys --project T2.xml --harness N1 --labels n1.csv


[N1]
-outputs
-bom

THINGS I'M TRYING TO PROVIDE:
- Wire build information
- Label To/From Device/Pin

TERMINATION:
The thing you actually put on the wire.
RINGS are terminations, not connectors. Ring usually has a device where it is connected to.
Wires can share a termination.

TO/FROM:
Identifies label information that you are supposed match.
Does not neccesserily mean to assemble in place.
TO/FROM component is included on BOM if it has that harness attribute.

ROADMAP
=======

Cutlist
-------

1. TERMINAL property of a pin
2. SPLICE side property of a wire
3. Grouping, alignment


Index
-----
1. Device name, short description, location
2. Diagrams, location

BRADY
-----
1. To from device and pin
2. CAYMAN 

CAYMAN
---------------
1. Length, Wire Processing, Color, Stripping, Label



unvesys -d "FireFly 5000 Ver1" -a "P9 Harness" Standard.xml --cutlist P9.xlsx
unvesys -d "FireFly 5000 Ver1" -a "J9 Harness" Standard.xml --cutlist J9.xlsx
unvesys -d "FireFly 5000 Ver1" -a "M3 Harness" Standard.xml --cutlist M3.xlsx
unvesys -d "FireFly 5000 Ver1" -a "M8 Harness" Standard.xml --cutlist M8.xlsx
unvesys -d "FireFly 5000 Ver1" -a "P6 Harness" Standard.xml --cutlist P6.xlsx
unvesys -d "FireFly 5000 Ver1" -a "J10 Harness" Standard.xml --cutlist J10.xlsx
unvesys -d "FireFly 5000 Ver1" -a "P10 Harness" Standard.xml --cutlist P10.xlsx
unvesys -d "FireFly 5000 Ver1" -a "J6 Harness" Standard.xml --cutlist J6.xlsx
unvesys -d "FireFly 5000 Ver1" -a "P8 Harness" Standard.xml --cutlist P8.xlsx
unvesys -d "FireFly 5000 Ver1" -a "J1 Harness" Standard.xml --cutlist J1.xlsx
unvesys -d "FireFly 5000 Ver1" -a "P1 Harness" Standard.xml --cutlist P1.xlsx
unvesys -d "FireFly 5000 Ver1" -a "M5 Harness" Standard.xml --cutlist M5.xlsx
unvesys -d "FireFly 5000 Ver1" -a "Control Wires" Standard.xml --cutlist control.xlsx



