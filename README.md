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

TO/FROM:
Identifies label information that you are supposed match.
Does not neccesserily mean to assemble in place.
TO/FROM component is included on BOM if it has that harness attribute.