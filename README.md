# py32_flasher
PY32 Flasher based on probe-rs. 

### Supported Chips
[PY32F0 Series](https://github.com/probe-rs/probe-rs/blob/master/probe-rs/targets/PY32F0_Series.yaml)

### Usage
```text
PY32 Flasher based on probe-rs

Usage: py32_flasher <COMMAND>

Commands:
  list             List all probes
  chips            List all supported chips
  flash            Flash .hex .bin .elf
  disable-rdp      Disable readout protection
  enable-rdp       Enable readout protection
  read-opt-bytes   Read option bytes
  write-opt-bytes  Write option bytes
  auto             Automatic flashing mode
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Installation
```bash
cargo install --git https://github.com/TinyHai/py32_flasher.git
```

# Credits
[probe-rs](https://github.com/probe-rs/probe-rs.git)

# License
[MIT license](https://github.com/TinyHai/py32_flasher/blob/main/LICENSE)
