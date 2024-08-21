# star-catalog

This crate provides binaries and a library.

The star-catalog a library manages a star catalog, which contains stars with
locations (from right ascension and declination, distance, magnitude,
color, and id).

## star_catalog binary

The star_catalog binary reads a portion of a star catalog file (generally from
JSON, such as the included hipparcos.json file) and allows for the
interrogationn of the contents, or plotting of portions of a star map
(if the 'image' feature is enabled).

The binary uses 'clap', and expects a subcommand for the operation it
should perform.

### Selection of stars to load

The star_catalog command expects either a builtin catalog, or a catalog filename, from
which stars are loaded. This is normally a JSON file or Postcard file
(.pst extension).

If the `hipp_bright` feature is enabled then the Hipparcos catalog of
stars with magnitude of 8 or brighter are loaded, from within the
star_catalog binary iself.

Postcard support is only enabled for the 'postcard' feature.

If the 'csv' feature is enabled when building the binary then a
hipparcos CSV file can be loaded; this can, for example, be used to
read 'hippparcos-voidmain.csv'. This is usually used to read a CSV
file and write out a JSON file for future use.

The command has a number of options that manage selection of stars
from within the catalog that are to be loaded:

 * magnitude: the '-m <N>' option selects stars of a magnitude of -N or brighter

 * location: the '-r <RA>', '-d <DE>', and '-a <ANGLE>' option select
   a centre and angle of a cone, outside of which stars from the
   catalog are ignored. RA, DE and ANGLE (right ascension and
   declination and cone angle) are specified in degrees.

### Naming of stars

A name mapping file can be specified, which must be JSON file
containing a list of pairs of (id, name), which is loaded to allow
stars to be identified by name.

### Subcommands

The subcommands supported are:

 * list: List the data for the stars in the catalog

 * find: Find data on a single star by name or id

 * write: Write out a star catalog as a JSON file or Postcard file

If no subcommand is provided then the command just prints out the
number of stars in the catalog as read.

### Subcommand `list`

The 'list' subcommand prints the star data, sorted by id.

The format is:

    <id> : <ra>, <de> : <distance> : <mag>

The id is specific to the catalog, and is a 'usize'. The right
ascension and declination are printed in degrees, the distance in
light years. The magnitude is that in the catalog, so normally visual
magnitude.

### Subcommand `find`

The 'find' subcommand prints the star data for a number of stars,
using either id or name (names only make sense if the catalog was
loaded with a names file too; see above).

  star_catalog <catalog> find <id> [<id>*]

  star_catalog <catalog> --names <names> find <id|name> [<id|name>*]

The output format is as shown in the 'list' command

### Subcommand `angle_between`

The 'angle_between' subcommand takes a number of ids or names (if a
named catalog) and prints out the angle between each pair, in degrees.

### Subcommand `write`

The 'write' subcommand writes out the subset of the catalog that was
loaded to a new json file or poscard file, specified with the '--output <filename>'
option.

This can be used to shrink a catalog file - for example:

  star_catalog hipparcos.json -m 2.5 write --output bright.json

The output catalog will only contains stars with a visual magnitude of
-2.5 or brighter.

If the `postcard` feature is enabled then a filename with '.pst'
extension will output the file as a Postcaard (binary) file (which are
about 40% of the size of JSON files)

### Subcommand `image`

This is only supported if the binary is compiled with the 'image' feature.

The 'image' subcommand generates an image of a region of the star map;
currently it is fairly primitive. Stars are colored according to B-V
(temperature) and sized according to magnitude.

THe region to display is specified by options to the 'image'
subcommand of -r, -d for right ascension, declination to provide a
direction; -a rotates the 'up' vector from north; -W and -H provide
the width and height of the image; -f provides a field of view of the
*width* of the image. The image produced is written to an output
filename provided by -o.

Support is provided for jpeg and png images

## Library

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

The 'image' feature allows writing of images of the sky map in the
star_catalog binary

The 'postcard' feature allows reading and writing of star catalogs in the
star_catalog binary

The 'hipp_bright' feature includes the Hipparcos catalog of stars with
magnitude 8 or brighter as `hipparcos::HIPP_BRIGHT_PST`; in the
star_catalog binary is provides this as a builtin catalog

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
