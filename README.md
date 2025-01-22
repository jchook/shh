*Shhh!*
=======

Get alerts when you are too loud.


About
-----

This app monitors the microphone input on your computer and plays a "SHH!" sound when you exceed a certain decibel threshold.


Download
--------

You can find binaries in the [releases](https://github.com/jchook/shhh/releases) section.

If you have `cargo` installed, you can install it via crates.io:

```sh
cargo install shhh
```

Run the program from the command line (e.g. Terminal on macOS).


Environment Variables
---------------------

| Variable | Default | Description |
|----------|---------|-------------|
| `SHH_ALERT_FREQUENCY` | 1 | Time between alerts (in seconds) |
| `SHH_DECIBEL_THRESHOLD` | -10.0 | dB threshold for an alert |
| `SHH_SENSITIVITY` | 0.8 | Between 0 and 1, sensitivity to volume spikes |


License
-------

MIT.

