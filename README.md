# INA4230 Rust Device Driver

A `#[no_std]` platform-agnostic driver for the [INA4230](https://www.ti.com/lit/ds/symlink/ina4230.pdf)
quad-channel power and energy sense monitor, based on the [`embedded-hal`](https://docs.rs/embedded-hal) traits.

Higher-level measurement APIs are built on the [embedded-sensors](https://github.com/OpenDevicePartnership/embedded-sensors)
async sensor traits (`VoltageSensor`, `CurrentSensor`, `PowerSensor`, `EnergySensor`).

## Features

- Full register coverage via pre-generated `src/device.rs` (generated from `INA4230.toml` using `device-driver-cli`)
- Async-first I²C interface (`embedded-hal-async`)
- Four independent measurement channels (bus voltage, shunt voltage, current, power, energy)
- Calibration helpers with correct SHUNT_CAL formula
- ADC range selection (±81.92 mV or ±20.48 mV full scale)
- Optional `defmt-03` logging support
- `no_std` compatible

## Usage

```toml
[dependencies]
ina4230 = "0.1.0"
embedded-hal-async = "1"
```

```rust,no_run
use ina4230::{AdcRange, Channel, CurrentSensor, Ina4230, INA4230_ADDR, PowerSensor, VoltageSensor};

// i2c implements embedded_hal_async::i2c::I2c
let mut sensor = Ina4230::new(i2c, INA4230_ADDR);

// Reset, then calibrate before taking measurements
sensor.reset().await?;
sensor.calibrate(Channel::Ch1, CURRENT_LSB_CH1, SHUNT_OHMS_CH1, AdcRange::Range0).await?;

// Wait for conversion
while !sensor.conversion_ready().await? {}

// Read channel 1
let vbus_mv = sensor.bus_voltage(Channel::Ch1).await?;
let ima      = sensor.current(Channel::Ch1).await?;
let pmw      = sensor.power(Channel::Ch1).await?;
```

## Configuration

Before taking measurements, two hardware-specific parameters must be set per channel.

### Shunt Resistor Value

Set `SHUNT_OHMS_CHx` to the resistance of the shunt resistor fitted on that channel, in ohms:

```rust,no_run
const SHUNT_OHMS_CH1: f32 = 0.010;  // 10 mΩ shunt on channel 1
```

Use a precision resistor (0.1% tolerance or better) for accurate results.
For very low value shunts, use Kelvin (4-wire) connections to eliminate
lead resistance errors.

### Current Resolution

`CURRENT_LSB` determines the resolution of the current measurement. A smaller
value gives finer resolution but a lower full-scale range; a larger value gives
a wider range but coarser resolution. The current register is 16-bit signed,
giving 32767 steps above zero.

Set `MAX_CURRENT_CHx` to the maximum current you expect on that channel — the
driver calculates the optimal `CURRENT_LSB` automatically:

```rust,no_run
// Example: expecting up to 100 mA on channel 1
const MAX_CURRENT_CH1: f32 = 0.1;            // amps
const CURRENT_LSB_CH1: f32 = MAX_CURRENT_CH1 / 32767.0;  // ~3.05 µA/LSB
```

The driver uses `CURRENT_LSB` to compute the `SHUNT_CAL` register value
written during `calibrate()`:

```
SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)
```

And to convert the raw current register reading back to milliamperes:

```
Current [mA] = raw × CURRENT_LSB × 1000
```

If `MAX_CURRENT_CHx` is set too low, the current register will saturate and
readings will be clamped. If set too high, resolution will be unnecessarily
coarse.

### ADC Range

The INA4230 supports two shunt input voltage ranges, configured via `AdcRange`:

| `AdcRange`         | Full-scale range | LSB    |
|--------------------|-----------------|--------|
| `Range0` (default) | ±81.92 mV       | 2.5 µV |
| `Range1`           | ±20.48 mV       | 625 nV |

`Range0` is the default and suits most applications. Use `Range1` for higher
resolution when measuring small currents through a large shunt resistor.

When using `Range1`, the `SHUNT_CAL` register value is automatically divided
by 4 and the shunt voltage LSB is adjusted to 625 nV:

```
Range0: SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT)
Range1: SHUNT_CAL = 0.00512 / (CURRENT_LSB × R_SHUNT) / 4
```

### Calibration

The calibration register serves two purposes: it provides the device with
the shunt resistor value used to calculate current from the measured
differential voltage, and it sets the resolution of the current and power
registers through the `CURRENT_LSB` and `Power_LSB` values.

Call `calibrate()` before taking current, power, or energy measurements:

```rust,no_run
sensor.calibrate(Channel::Ch1, CURRENT_LSB_CH1, SHUNT_OHMS_CH1, AdcRange::Range0).await?;
```

The calibration register must be programmed after initial power up, power
cycle events, or device enable to receive valid current, power, and energy
results. Bus voltage and shunt voltage readings do not require calibration.

## I²C Addresses

| A1  | A0  | Address |
|-----|-----|---------|
| GND | GND | `0x40`  |
| GND | VS  | `0x41`  |
| VS  | GND | `0x44`  |
| VS  | VS  | `0x45`  |

## Regenerating `src/device.rs`

```sh
cargo install device-driver-cli
device-driver-cli -m INA4230.toml -o generated.rs -d Device
rustfmt --edition 2024 generated.rs
cp generated.rs src/device.rs
```

## MSRV

Rust `1.88` and up.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT
