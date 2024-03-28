
EXPORT PROJECT XML
==================
This is a post-processor for VeSys(Capital Essentials) XML files, which you get by exporting a whole project into XML.

![Project XML](screenshot2.JPG)

EXPORT LIBRARY XML
==================
The program also needs a `Library.xml` file in the same directory as `*.exe`. You can export Library XML from Component Manager.


![Library XML](screenshot3.JPG)

BUILD
=====

This application is written in Rust.

cargo build
cargo run


OPEN PROJECT
============

Go to File -> Open to load project XML, then you may right click on various harness items and export them into available file formats.

![Library XML](screenshot.JPG)




