STM32 home sensor STM32F103C8 (blue pill)
=========================================

> Example of home sensor using STM32F103C8 (blue pill) written in Rust. It
> displays temperature, humidity and co2 on a LCD1602 display. Data is pulled
> every 2 seconds from a SCD30 sensor which share the same i2c bus as the
> display.

## Connectivity

```mermaid
graph TB
    subgraph lcd1602[lcd1602 display - <i>i2c addr 0x27</i>]
        lcd1602_scl(SCL)
        lcd1602_sda(SDA)
        lcd1602_grd(GRD)
        lcd1602_vcc(VCC)
    end
    subgraph stm32f103c8
        stm32_b6(B6)
        stm32_b7(B7)
        stm32_grd(GRD)
        stm32_vcc(VCC 3.3v)
    end
    subgraph scd30[scd30 sensor - <i>i2c addr 0x61</i>]
        scd30_scl(SCL)
        scd30_sda(SDA)
        scd30_grd(GRD)
        scd30_vcc(VCC)
    end
    
    stm32_b6-->lcd1602_scl-->scd30_scl
    stm32_b7-->lcd1602_sda-->scd30_sda
    stm32_grd-->lcd1602_grd-->scd30_grd
    stm32_vcc-->lcd1602_vcc-->scd30_vcc
```

## Getting started

```bash
cargo make flash
```

### Debugging

Uncomment hprintln lines in the main.rs, then:
```bash
# build and listen
cargo make openocd

# then debug in another terminal
cargo make debug
(gdb) continue
```

## Datasheet

- [Qapass LCD1602](https://funduino.de/DL/1602LCD.pdf) with [PCF8574T](https://www.ti.com/lit/ds/symlink/pcf8574.pdf) i2c chip
- [SCD30](https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9.5_CO2/Sensirion_CO2_Sensors_SCD30_Interface_Description.pdf)
