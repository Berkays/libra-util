# Libra Util

Libra Utils is an interface to the Libra blockhain CLI commands to simplify certain parsing actions. This tool currently supports generating new accounts only and it is intended for testing purposes.

## Build

* Minimal image contains only the needed packages for the interface. Libra CLI is not included.
* Full image fetches the Official Libra repository and adds interface package. Libra CLI is included.

1. Fork this repository: `git clone https://`
2. Run docker build in root directory (This might take few minutes):
    * Minimal image: `docker build -t libra-util -f docker\minimal\dockerfile .`
    * Full image: `docker build -t libra-util -f docker\full\dockerfile .`
3. Execute interface:
    * Create new account: `docker run --rm libra-util:latest ./target/debug/interface create`

## Contribute

Feel free to contribute or ask about anything.