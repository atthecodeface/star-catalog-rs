# Release 0.0.8 in progress (2024-08-26)

- Added StarFilter, and StarFilterSelect, to provide simple filtering
  of results (and to allow selection of a subset of results)

- Added `find_stars_around` to Catalog

- Added to and from usize methods for CatalogIndex

# Release 0.0.7 (2024-08-24)

- Moved `find_id_or_name` to Catalog from the binary

- Improve cubemap layout

- Add constellation drawing to cubemap

- Added in more northern hemisphere constellations

- Moved image_view to be a module of the library accessible to all
  rather than part of the binary

- Add ability to select star draw style to ImageView

- Added `draw_line`,`draw_circle`, and `draw_line_between_stars` to ImageView

# Release 0.0.6 (2024-08-22)

- Add 'collated' names as a built-in

- Add 'cubemap' to generate a full cubemap from a star position

- Fix image generation (field of view etc)

- Add longhelp

# Release 0.0.5 (2024-08-21)

- Add 'triangle' to star_catalog binary to allow searching for a triangle of stars

# Release 0.0.4 (2024-08-21)

- Added internal catalog `hipp_bright` if hipp_bright feature is used

- Add search for a star triangle

- Add postcard feature, for reading and writing catalogs

- Made catalog use CatalogIndex for contents of its subcube Vecs

# Release 0.0.3 (2024-08-19)

- Added binary 'star_catalog' to display information and create images

- Added color temperature

- Removed SubcubeMask

# Release 0.0.2 (2024-08-17)

- Removed SubcubeNeighborIter

- Added documentation

- Moved to f64 for RA, DE and vector

- Added CatalogIndex

# Release 0.0.1 (2024-08-16)

- Publishing on crates.io for the first time

**Contributors**: @atthecodeface
