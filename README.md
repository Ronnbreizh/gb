# GB emulator
This is a yet an other GB (GameBoy) emulator side project.
The goal here is to have some fun coding in Rust, experimenting with CI and to keep my skills sharp.
I'm planning to do a clean implementation of the device by abstracting components, registers, etc... in a way that make sens.
Ideally, this project, in the future, could be used to taught CPU/ALU behavior to students, showcase palette / texture for openGL or as reference for future GB emulators.

## How to use it ?
You can run the GB from the command line using `cargo run`.
I'm planning to make it able to load ROMs from the command line in a near™ future.

## For the future !
I have a few expensions of this project planned :
* supporting the GameBoy Color games; which is a superset of the GameBoy capabilities
* adding a nice GUI to make using this software easier.
* abstracting the backend, especially the graphic one, potentially to target Wasm.

# Some development notes :

## ROMS for testing
Graphic:
acid2

Jeux à tester :
Tetris
Kirby


## TODO

### UI
* interface en CLI
* interface GUI

### CPU
* aligner fonctions
* templating des fonctions ?
* tests unitaires

### GPU
* lecture buffer -> format projetable

### Audio
* tout

### Inputs
* tout


