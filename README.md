Based heavily off of https://github.com/rust-transit/gtfs-structure

Handles nearly all datasets from the GTFS reference (https://developers.google.com/transit/gtfs/reference)

- TODO: https://developers.google.com/transit/gtfs/reference#translationstxt

Additionally, allows for "pulling out" fields not defined by the GTFS standard (a lot of GTFS providers add additional meta-data that can sometimes be useful):

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use gtfs_parser::GtfsReader;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct TripBrigade {
    trip_id: String,
    brigade_id: String,
}

let mut gtfs = GtfsReader::from_path(PathBuf::from("./resources/zips/gtfs.zip")).unwrap();
let trip_brigade: Vec<TripBrigade> = gtfs.custom("trips.txt").unwrap();

assert_eq!(trip_brigade[0].brigade_id, "010/51");
assert_eq!(trip_brigade[0].trip_id, "trip1");
```


