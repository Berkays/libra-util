# Libra Util

Libra Utils is an interface to the Libra CLI commands to simplify certain parsing actions. This tool currently supports generating new accounts only and it is intended for testing purposes.

## Build

* Minimal image contains only the needed packages for the interface. Libra CLI is not included.
* Full image fetches the Official Libra repository and adds interface package. Libra CLI is included.

1. Fork this repository: `git clone https://github.com/Berkays/libra-util.git`
2. Run docker build in root directory (This might take few minutes):
    * Minimal image: `docker build -t libra-util -f docker\minimal\dockerfile .`
    * Full image: `docker build -t libra-util -f docker\full\dockerfile .`
3. Execute interface:
    * Create new account: 
    `$ docker run --rm libra-util:latest ./target/debug/interface create`
        
        Sample Output:

            $ 9620fb0dbfcdb22ec554b9405246973b106eab6d3c6946c35fd270246a4345c9:kite release view hurt release autumn north essay rocket swap enroll crash mechanic mystery guitar excuse crumble giggle toilet oval slab toe negative canoe
## Contribute

Feel free to contribute or ask about anything.