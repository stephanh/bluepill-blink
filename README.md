# Bluepill Blink

Very simple program for the stm32f103 bluepill that blinks the onboard LED every 1s.

```
openocd -f interface/stlink-v2.cfg -f target/stm32f1x_2.cfg
cargo run
```
