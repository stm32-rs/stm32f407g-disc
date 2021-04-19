stm32f407g-disc
===============

_stm32f407g-disc_ contains a basic board support package for the
[STM32F407G-DISC][] microcontroller board (also known as STM32F4DISCOVERY, but
easy to confuse with other STM32F4 discovery boards which also exist) to write
firmwares using the Rust language. This experimentation board features multiple
user programmable LEDs an accelerometer, an audio DAC with amplified, a
microphone jack, a microphone and a user programmable USB connector. A shield
with breakout for Ethernet, RS232 serial port, SD-Card reader, and LCD
connector is also available.

It also contains a (non-removable) capable ST-Link V2 debugging interface.

[STM32F407G-DISC]: https://www.st.com/en/evaluation-tools/stm32f4discovery.html

Programming
-----------

Several methods for programming exist. If the ST-Link on your board has a recent firmware (can be updated e.g. via [ST-LINK utility](https://www.st.com/en/development-tools/stsw-link004.html)), we recomend the use of [cargo-embed](https://github.com/probe-rs/cargo-embed):

```sh
cargo embed --example=mems
```

(`cargo embed` doesn't support ITM at the moment.)

Otherwise this repo also contains config files for [OpenOCD](http://openocd.org/).


License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
