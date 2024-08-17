# star-catalog

A library providing a star catalog, which contains stars with
locations (from right ascension and declination, distance, magnitude,
color, and id).

The catalog can be searched by id, and the closest star to a
particular right ascension and declination can be found.

The catalog supports serde.

The library includes the Hipparcos star catalog, and the related
Hipparcos id values for the IAU named stars.

This is still an early release; the initial purpose of the library is
to permit characterization of camera lenses from photographs of stars
(as the stars have known fixed relative orientations, the angle
between stars on the sensor of a camera can be mapped to real world
angles).

## Usage

```
cargo add star-catalog
```

## Features

The 'csv' feature can be added to permit reading of (e.g.) Hipparcos
CSV catalog files. This is not required in normal use, as JSON is used
for the included Hipparcos catalog.

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
