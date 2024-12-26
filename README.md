Embedded Rust Project for ATmega328P and Arduino Uno
=====

This project demonstrates the use of Rust to program an ATmega328P microcontroller for analog data acquisition and UART communication. It includes both simulation in Proteus and hardware implementation on the Arduino Uno.

## Features
 - Analog data acquisition from a potentiometer using ADC.
 - Data conversion and serial communication via UART.
 - Interrupt management with Timer1_OVF and semaphores.
 - Deployment and testing on Proteus simulation and hardware using Hercules Virtual Terminal.
 
## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Clone this repository:
```bash
git clone https://github.com/achreftel1/embedded_rust.git
cd embedded_rust
```

3. Build the firmware:
```bash
cargo build
```

4. Run the bash script to convert the ELF file to a HEX file, which will be used for the ATmega328P simulation in Proteus.
```bash
cd embedded_rust/target/avr-atmega328p/release
./elf2hex.sh
```

5. Open the simulation of proteus [Rust.pdsprj].

6. Hardware implementation using the Arduino Uno and flashing the firmware: 
```bash
cd embedded_rust/target/avr-atmega328p/debug
wc -c first.elf
avrdude -p m328p -c arduino -P /dev/ttyACM0 -b 115200 -U flash:w:first.elf
```

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Contributions are welcome and will be dual-licensed under the above terms unless explicitly stated otherwise.
