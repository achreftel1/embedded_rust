#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // Enable experimental features

use panic_halt as _;
use avr_device::interrupt;

// Import the external atmega_328p_port module

mod atmega_328p_ports;
use crate::atmega_328p_ports::*;

// Pin constants
const PB7: u8 = 7;
const PB6: u8 = 6;
const ADSC: u8 = 6;
static mut SEM: bool = false; // Semaphore to signal that new ADC data is ready

#[avr_device::entry]
fn main() -> ! {
    config_timer();  // Configure the timer for 500 ms
    config_uart0();  // Configure UART for serial communication
    config_adc();    // Configure the ADC for analog-to-digital conversions
    
    unsafe {
        DDRB.write(0xFF); // Set PORTB as output
        DDRD.write(0);    // Set PORTD as input
        EICRA.write(2);   // Configure external interrupt on falling edge
        EIMSK.write(1);   // Enable external interrupt 0 (INT0)
        PORTD.write(4);   // Enable pull-up resistor on PD2 (INT0 pin)
        avr_device::interrupt::enable(); // Enable all interrupts
    }

    loop {
        unsafe {
            let mut adc7: u16 = ((ADCH.read() as u16) << 8) | (ADCL.read() as u16); // Read the 10-bit ADC result

            let mut buffer = [0u8; 5]; // Buffer to store digits (u16 has max 5 digits)
            let length = int_to_ascii(adc7, &mut buffer); // Convert the number to ASCII

            // Send each character of the string via UART
            for i in 0..length {
                UDR0.write(buffer[i]); // Send character through UART
                for _ in 1..100000 { avr_device::asm::nop(); } // Delay
            }
            UDR0.write(b'\r'); // Send carriage return
            for _ in 1..100000 { avr_device::asm::nop(); } // Delay
        }
        // Place any continuously running logic here
    }
}

// Function to convert an integer to an ASCII string
fn int_to_ascii(mut number: u16, buffer: &mut [u8]) -> usize {
    let mut index = 0;

    // Extract digits in reverse order
    while number > 0 || index == 0 {
        buffer[index] = b'0' + (number % 10) as u8;
        number /= 10;
        index += 1;
    }

    // Reverse the order of digits in the buffer
    buffer[..index].reverse();

    index // Return the length of the converted string
}

// Configuration for the timer (500 ms)
fn config_timer() {
    unsafe {
        DDRB.write(0xFF);    // Set PORTB as output
        TCCR1A.write(0);     // Timer mode (Normal)
        TCCR1B.write(2);     // Prescaler set to 8
        TIMSK1.write(1);     // Enable timer overflow interrupt
        TCNT1.write(55535);  // Initialize the counter
        PORTD.write(4);      // Enable PD2 (INT0)
        interrupt::enable(); // Enable global interrupts (equivalent to SREG |= 0x80 in C)
    }
}

// UART configuration for 19.2 kbps
fn config_uart0() {
    unsafe {
        UCSR0C.write(0b00000110); // Frame format: 8 data bits, 1 stop bit
        UBRR0.write(51);          // Set baud rate to 19.2 kbps
        UCSR0B.write(0x18);       // Enable TX and RX
    }
}

// Send a string via UART
fn send_string_uart(string: &str) {
    for byte in string.as_bytes() {
        unsafe {
            // Write the byte to the UART register
            UDR0.write(*byte);

            // Wait until the transmission is complete
            while (UCSR0A.read() & 0b01000000) == 0 {}
        }
    }
}

// ADC configuration
fn config_adc() {
    unsafe {
        ADMUX.write(0x45);  // Select ADC5 as the source and set Vref
        ADCSRA.write(0x8E); // Enable ADC, set prescaler to 64, and enable ADC interrupts
    }
}

// Enable a specific ADC pin
fn adc_pin_enable(pin: u8) {
    unsafe {
        DIDR0.write(DIDR0.read() | (1 << pin)); // Disable digital input on the specified ADC pin
    }
}

// Disable a specific ADC pin
fn adc_pin_disable(pin: u8) {
    unsafe {
        DIDR0.write(DIDR0.read() & !(1 << pin)); // Enable digital input on the specified ADC pin
    }
}

// Select the ADC channel
fn adc_pin_select(source: u8) {
    unsafe {
        ADMUX.write(ADMUX.read() & 0xF0);       // Clear MUX bits (channel selection)
        ADMUX.write(ADMUX.read() | source);    // Select the desired channel
    }
}

// Perform an ADC conversion
fn adc_convert() -> u16 {
    unsafe {
        let mut adcl: u8 = 0;
        let mut adch: u8 = 0;

        let mut adc_convert_done = false; // Flag to simulate polling

        // Start an ADC conversion
        ADCSRA.write(ADCSRA.read() | (1 << ADSC));

        // Wait until the conversion is complete
        while !adc_convert_done {
            if (ADCSRA.read() & (1 << ADSC)) == 0 {
                adc_convert_done = true;
            }
        }

        adcl = ADCL.read(); // Read low byte
        adch = ADCH.read(); // Read high byte

        ((adch as u16) << 8) | (adcl as u16) // Combine ADCL and ADCH to form a 16-bit result
    }
}

// Timer overflow interrupt handler (TIMER1)
#[interrupt(atmega328p)]
fn TIMER1_OVF() {
    unsafe {
        let pb: u8 = PORTB.read();  // Read current value of PORTB
        PORTB.write(pb ^ (1 << PB6)); // Toggle the LED state on PB6
        TCNT1.write(55535);         // Reset the counter
        ADCSRA.write(ADCSRA.read() | (1 << ADSC)); // Start a new ADC conversion
    }
}

// ADC interrupt handler
#[interrupt(atmega328p)]
fn ADC() {
    unsafe {
        SEM = true; // Indicate that new ADC data is ready
    }
}
