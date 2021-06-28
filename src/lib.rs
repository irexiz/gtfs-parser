use error::Error;
use serde::{de::DeserializeOwned, Deserialize};

use crate::structures::{
    agency::Agency,
    attributions::Attribution,
    calendar::Calendar,
    calendar_dates::CalendarDate,
    fare_attributes::FareAttribute,
    fare_rules::FareRule,
    feed_info::FeedInfo,
    frequencies::Frequency,
    levels::Level,
    pathways::Pathway,
    routes::Route,
    shapes::Shape,
    stop_times::{RawStopTime, StopTime},
    stops::Stop,
    transfers::Transfer,
    trips::{RawTrip, Trip},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Cursor, Read, Seek},
    path::Path,
    sync::Arc,
};

pub mod error;
pub mod gtfs_serde;
pub mod structures;

/// https://en.wikipedia.org/wiki/Byte_order_mark
const BYTE_ORDER_MARK: [u8; 3] = [0xEF, 0xBB, 0xBF];

const DATASET_FILES: [&str; 17] = [
    "agency.txt",
    "attributions.txt",
    "calendar.txt",
    "calendar_dates.txt",
    "fare_attributes.txt",
    "fare_rules.txt",
    "feed_info.txt",
    "frequencies.txt",
    "levels.txt",
    "pathways.txt",
    "routes.txt",
    "shapes.txt",
    "stop_times.txt",
    "stops.txt",
    "transfers.txt",
    "translations.txt",
    "trips.txt",
];

pub(crate) fn to_map<O: Id>(elements: impl IntoIterator<Item = O>) -> HashMap<String, O> {
    elements
        .into_iter()
        .map(|e| (e.id().to_owned(), e))
        .collect()
}

pub trait Id {
    fn id(&self) -> &str;
}

pub type FromUrl = Cursor<Vec<u8>>;
pub type FromPath = BufReader<File>;

impl ReadSeek for FromUrl {}
impl ReadSeek for FromPath {}

trait ReadSeek: Read + Seek {}

pub struct GtfsReader {
    archive: zip::ZipArchive<Box<dyn ReadSeek>>,
    /// File mapping (filename, archive_index)
    file_mappings: HashMap<String, usize>,
}

impl GtfsReader {
    #[cfg(feature = "read-url")]
    pub fn from_url<U: reqwest::IntoUrl>(url: U) -> Result<GtfsReader, Error> {
        let mut res = reqwest::blocking::get(url)?;
        let mut body = Vec::new();
        res.read_to_end(&mut body)?;
        let cursor = Box::new(Cursor::new(body));
        Self::from_reader(cursor)
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<GtfsReader, Error> {
        let reader = File::open(&path)?;
        let buf_reader = Box::new(BufReader::new(reader));

        Self::from_reader(buf_reader)
    }

    fn from_reader(reader: Box<dyn ReadSeek>) -> Result<GtfsReader, Error> {
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut file_mappings = HashMap::new();

        // This is a bit roundabout, but we do this in case provided GTFS zip has its files nested
        // inside another subdirectory
        for index in 0..archive.len() {
            let archive_file = archive.by_index(index)?;

            let path = std::path::Path::new(archive_file.name());
            for dataset_file in DATASET_FILES.iter() {
                if path.file_name() == Some(std::ffi::OsStr::new(dataset_file)) {
                    file_mappings.insert(dataset_file.to_string(), index);
                }
            }
        }

        Ok(Self {
            archive,
            file_mappings,
        })
    }

    fn read_gtfs<T: DeserializeOwned>(&mut self, filename: &str) -> Result<Vec<T>, Error> {
        let (filename, index) = self
            .file_mappings
            .get_key_value(filename)
            .map(|(k, v)| (k.clone(), *v))
            .unwrap();

        self.read_objects(filename, index)
    }

    /// Some GTFS providers add additional data along the GTFS standard,
    /// such as the Polish Wroclaw GTFS with their trip.brigade_id in trips.txt
    ///
    /// The custom() method provides an interface to deserialize such GTFS file into
    /// a user-defined struct.
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use std::path::PathBuf;
    /// use gtfs_parser::GtfsReader;
    /// use std::fs::File;
    /// use std::io::BufReader;
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct TripBrigade {
    ///     trip_id: String,
    ///     brigade_id: String,
    /// }
    ///
    /// let mut gtfs = GtfsReader::from_path(PathBuf::from("./resources/zips/gtfs.zip")).unwrap();
    /// let trip_brigade: Vec<TripBrigade> = gtfs.custom("trips.txt").unwrap();
    ///
    /// assert_eq!(trip_brigade[0].brigade_id, "010/51");
    /// assert_eq!(trip_brigade[0].trip_id, "trip1");
    /// ```
    pub fn custom<T: DeserializeOwned>(&mut self, filename: &str) -> Result<Vec<T>, Error> {
        if let Some(index) = self.file_mappings.get(filename) {
            let idx = *index;
            self.read_objects(filename.to_string(), idx)
        } else {
            Err(Error::FileNotFound(filename.to_string()))
        }
    }

    pub fn agencies(&mut self) -> Result<Vec<Agency>, Error> {
        self.read_gtfs("agency.txt")
    }

    pub fn attributions(&mut self) -> Result<Vec<Attribution>, Error> {
        self.read_gtfs("attributions.txt")
    }

    pub fn calendar(&mut self) -> Result<Vec<Calendar>, Error> {
        self.read_gtfs("calendar.txt")
    }

    pub fn calendar_map(&mut self) -> Result<HashMap<String, Calendar>, Error> {
        Ok(to_map(self.read_gtfs("calendar.txt")?))
    }

    pub fn calendar_dates(&mut self) -> Result<Vec<CalendarDate>, Error> {
        self.read_gtfs("calendar_dates.txt")
    }

    pub fn calendar_dates_map(&mut self) -> Result<HashMap<String, Vec<CalendarDate>>, Error> {
        let mut dates = HashMap::new();
        let calendar_dates: Vec<CalendarDate> = self.read_gtfs("calendar_dates.txt")?;

        for calendar_date in calendar_dates {
            let date = dates
                .entry(calendar_date.service_id.clone())
                .or_insert_with(Vec::new);
            date.push(calendar_date)
        }

        Ok(dates)
    }

    pub fn fare_attributes(&mut self) -> Result<Vec<FareAttribute>, Error> {
        self.read_gtfs("fare_attributes.txt")
    }

    pub fn fare_rules(&mut self) -> Result<Vec<FareRule>, Error> {
        self.read_gtfs("fare_rules.txt")
    }

    pub fn feed_info(&mut self) -> Result<Vec<FeedInfo>, Error> {
        self.read_gtfs("feed_info.txt")
    }

    pub fn frequencies(&mut self) -> Result<Vec<Frequency>, Error> {
        self.read_gtfs("frequencies.txt")
    }

    pub fn levels(&mut self) -> Result<Vec<Level>, Error> {
        self.read_gtfs("levels.txt")
    }

    pub fn pathways(&mut self) -> Result<Vec<Pathway>, Error> {
        self.read_gtfs("pathways.txt")
    }

    pub fn routes(&mut self) -> Result<Vec<Route>, Error> {
        self.read_gtfs("routes.txt")
    }

    pub fn shapes(&mut self) -> Result<Vec<Shape>, Error> {
        self.read_gtfs("shapes.txt")
    }

    pub fn raw_stop_times(&mut self) -> Result<Vec<RawStopTime>, Error> {
        self.read_gtfs("stop_times.txt")
    }

    pub fn stops(&mut self) -> Result<Vec<Stop>, Error> {
        self.read_gtfs("stops.txt")
    }

    pub fn transfers(&mut self) -> Result<Vec<Transfer>, Error> {
        self.read_gtfs("transfers.txt")
    }

    pub fn raw_trips(&mut self) -> Result<Vec<RawTrip>, Error> {
        self.read_gtfs("trips.txt")
    }

    pub fn trips(&mut self) -> Result<HashMap<String, Trip>, Error> {
        let raw_trips = self.raw_trips()?;
        let raw_stop_times = self.raw_stop_times()?;

        let stops: HashMap<String, Arc<Stop>> = self
            .stops()?
            .into_iter()
            .map(|s| (s.id.clone(), Arc::new(s)))
            .collect();

        let mut trips = to_map(raw_trips.into_iter().map(Trip::from));

        for raw in raw_stop_times {
            let trip = &mut trips
                .get_mut(&raw.trip_id)
                .ok_or_else(|| Error::ReferenceError(raw.trip_id.to_string()))?;

            let stop = stops
                .get(&raw.stop_id)
                .ok_or_else(|| Error::ReferenceError(raw.stop_id.to_string()))?;
            trip.stop_times
                .push(StopTime::from(&raw, Arc::clone(&stop)));
        }

        for trip in &mut trips.values_mut() {
            trip.stop_times
                .sort_by(|a, b| a.stop_sequence.cmp(&b.stop_sequence));
        }

        Ok(trips)
    }

    fn read_objects<D>(&mut self, filename: String, index: usize) -> Result<Vec<D>, Error>
    where
        for<'de> D: Deserialize<'de>,
    {
        let mut zipfile = self
            .archive
            .by_index(index)
            .map_err(|_| Error::FileNotFound(format!("Missing file: {}", filename)))?;

        let mut bom = [0; 3];

        zipfile
            .read_exact(&mut bom)
            .map_err(|err| Error::FileReadError {
                filename: filename.clone(),
                source: err,
            })?;

        let chained = if bom != BYTE_ORDER_MARK {
            bom.chain(zipfile)
        } else {
            [].chain(zipfile)
        };

        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_reader(chained);

        // Store the headers to be able to return them in case of errors
        let headers = reader
            .headers()
            .map_err(|e| Error::CSVError {
                filename: filename.clone(),
                source: e,
                line_in_error: None,
            })?
            .clone();

        let mut objects = Vec::new();
        for record in reader.records() {
            let string_record = record.map_err(|err| Error::CSVError {
                filename: filename.clone(),
                source: err,
                line_in_error: Some(error::LineError {
                    headers: headers
                        .into_iter()
                        .map(|header| header.to_owned())
                        .collect(),
                    values: vec![],
                }),
            })?;

            let obj = string_record
                .deserialize(Some(&headers))
                .map_err(|err| Error::CSVError {
                    filename: filename.clone(),
                    source: err,
                    line_in_error: Some(error::LineError {
                        headers: headers
                            .into_iter()
                            .map(|header| header.to_owned())
                            .collect(),
                        values: string_record.into_iter().map(ToOwned::to_owned).collect(),
                    }),
                })?;

            objects.push(obj);
        }

        Ok(objects)
    }
}

#[cfg(test)]
mod test {

    use chrono::NaiveDate;
    use rgb::RGB8;

    use crate::{
        structures::{
            calendar_dates::Exception,
            fare_attributes::{PaymentMethod, Transfers},
            frequencies::ServiceType,
            pathways::PathwayMode,
            routes::{ContinuousPickupDropOff, RouteType},
            stop_times::PickupDropOffType,
            stops::{StopLocationType, WheelchairBoardingAvailable},
            transfers::TransferType,
            trips::{BikesAllowed, Direction},
        },
        Id,
    };

    use super::*;
    use std::path::PathBuf;

    fn parse_time_over_midnight(time: u64) -> String {
        let hours = time / 3600;
        let minutes = (time / 60) % 60;
        let seconds = time % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    #[test]
    fn create_trips_test() {
        let mut gtfs = GtfsReader::from_path(PathBuf::from("./resources/zips/gtfs.zip")).unwrap();
        gtfs.trips().expect("trips");
    }

    #[ignore]
    #[test]
    fn test_url() {
        let mut gtfs =
            GtfsReader::from_url("https://www.wroclaw.pl/open-data/87b09b32-f076-4475-8ec9-6020ed1f9ac0/OtwartyWroclaw_rozklad_jazdy_GTFS.zip")
                .unwrap();
        gtfs.trips().expect("trips");
    }

    #[test]
    fn custom_test() {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize, Debug)]
        struct TripBrigade {
            trip_id: String,
            brigade_id: String,
        }

        let mut gtfs = GtfsReader::from_path(PathBuf::from("./resources/zips/gtfs.zip")).unwrap();
        let trip_brigade: Vec<TripBrigade> = gtfs.custom("trips.txt").unwrap();

        assert_eq!(trip_brigade[0].brigade_id, "010/51");
        assert_eq!(trip_brigade[0].trip_id, "trip1");
    }

    macro_rules! test_gtfs {
        ($function:ident, $method:ident, $zip:literal) => {
            #[test]
            fn $function() {
                let mut gtfs =
                    GtfsReader::from_path(PathBuf::from(concat!("./resources/zips/", $zip)))
                        .unwrap();
                let target = gtfs.$method().unwrap();
                $method(target);
            }
        };
    }

    test_gtfs! { agencies_from_zip, agencies, "gtfs.zip" }
    test_gtfs! { agencies_from_zip_with_bom, agencies, "gtfs_with_bom.zip" }
    test_gtfs! { agencies_from_zip_subdirectory, agencies, "subdirectory.zip" }

    test_gtfs! { attributions_from_zip, attributions, "gtfs.zip" }
    test_gtfs! { attributions_from_zip_with_bom, attributions, "gtfs_with_bom.zip" }
    test_gtfs! { attributions_from_zip_subdirectory, attributions, "subdirectory.zip" }

    test_gtfs! { calendar_from_zip, calendar, "gtfs.zip" }
    test_gtfs! { calendar_from_zip_with_bom, calendar, "gtfs_with_bom.zip" }
    test_gtfs! { calendar_from_zip_subdirectory, calendar, "subdirectory.zip" }

    test_gtfs! { calendar_dates_from_zip, calendar_dates, "gtfs.zip" }
    test_gtfs! { calendar_dates_from_zip_with_bom, calendar_dates, "gtfs_with_bom.zip" }
    test_gtfs! { calendar_dates_from_zip_subdirectory, calendar_dates, "subdirectory.zip" }

    test_gtfs! { fare_attributes_from_zip, fare_attributes, "gtfs.zip" }
    test_gtfs! { fare_attributes_from_zip_with_bom, fare_attributes, "gtfs_with_bom.zip" }
    test_gtfs! { fare_attributes_from_zip_subdirectory, fare_attributes, "subdirectory.zip" }

    test_gtfs! { fare_rules_from_zip, fare_rules, "gtfs.zip" }
    test_gtfs! { fare_rules_from_zip_with_bom, fare_rules, "gtfs_with_bom.zip" }
    test_gtfs! { fare_rules_from_zip_subdirectory, fare_rules, "subdirectory.zip" }

    test_gtfs! { frequencies_from_zip, frequencies, "gtfs.zip" }
    test_gtfs! { frequencies_from_zip_with_bom, frequencies, "gtfs_with_bom.zip" }
    test_gtfs! { frequencies_from_zip_subdirectory, frequencies, "subdirectory.zip" }

    test_gtfs! { levels_from_zip, levels, "gtfs.zip" }
    test_gtfs! { levels_from_zip_with_bom, levels, "gtfs_with_bom.zip" }
    test_gtfs! { levels_from_zip_subdirectory, levels, "subdirectory.zip" }

    test_gtfs! { pathways_from_zip, pathways, "gtfs.zip" }
    test_gtfs! { pathways_from_zip_with_bom, pathways, "gtfs_with_bom.zip" }
    test_gtfs! { pathways_from_zip_subdirectory, pathways, "subdirectory.zip" }

    test_gtfs! { routes_from_zip, routes, "gtfs.zip" }
    test_gtfs! { routes_from_zip_with_bom, routes, "gtfs_with_bom.zip" }
    test_gtfs! { routes_from_zip_subdirectory, routes, "subdirectory.zip" }

    test_gtfs! { shapes_from_zip, shapes, "gtfs.zip" }
    test_gtfs! { shapes_from_zip_with_bom, shapes, "gtfs_with_bom.zip" }
    test_gtfs! { shapes_from_zip_subdirectory, shapes, "subdirectory.zip" }

    test_gtfs! { raw_stop_times_from_zip, raw_stop_times, "gtfs.zip" }
    test_gtfs! { raw_stop_times_from_zip_with_bom, raw_stop_times, "gtfs_with_bom.zip" }
    test_gtfs! { raw_stop_times_from_zip_subdirectory, raw_stop_times, "subdirectory.zip" }

    test_gtfs! { stops_from_zip, stops, "gtfs.zip" }
    test_gtfs! { stops_from_zip_with_bom, stops, "gtfs_with_bom.zip" }
    test_gtfs! { stops_from_zip_subdirectory, stops, "subdirectory.zip" }

    test_gtfs! { transfers_from_zip, transfers, "gtfs.zip" }
    test_gtfs! { transfers_from_zip_with_bom, transfers, "gtfs_with_bom.zip" }
    test_gtfs! { transfers_from_zip_subdirectory, transfers, "subdirectory.zip" }

    test_gtfs! { raw_trips_from_zip, raw_trips, "gtfs.zip" }
    test_gtfs! { raw_trips_from_zip_with_bom, raw_trips, "gtfs_with_bom.zip" }
    test_gtfs! { raw_trips_from_zip_subdirectory, raw_trips, "subdirectory.zip" }

    fn agencies(target: Vec<Agency>) {
        let target = &target[0];
        assert_eq!(target.id(), "agency001");
        assert_eq!(target.name, "Transit Agency");
        assert_eq!(target.url, "http://www.transitcommuterbus.com/");
        assert_eq!(target.timezone, "PST");
        assert_eq!(target.lang, Some("en".to_string()));
        assert_eq!(target.phone, Some("123-456-TEST".to_string()));
        assert_eq!(target.fare_url, Some("http://test.com".to_string()));
        assert_eq!(target.email, Some("email@test.com".to_string()));
    }

    fn attributions(target: Vec<Attribution>) {
        let target = &target[0];
        assert_eq!(target.id(), "attribution001");
        assert_eq!(target.agency_id, Some("agency001".to_string()));
        assert_eq!(target.is_producer, true);
        assert_eq!(target.is_operator, false);
        assert_eq!(target.is_authority, false);
        assert_eq!(target.organization_name, "Transit Feed Solutions USA");
    }

    fn calendar(target: Vec<Calendar>) {
        let target = &target[0];
        assert_eq!(target.id(), "WE");
        assert_eq!(target.monday, false);
        assert_eq!(target.tuesday, false);
        assert_eq!(target.wednesday, false);
        assert_eq!(target.thursday, false);
        assert_eq!(target.friday, false);
        assert_eq!(target.saturday, true);
        assert_eq!(target.sunday, true);
        assert_eq!(target.start_date, NaiveDate::from_ymd(2006, 7, 1));
        assert_eq!(target.end_date, NaiveDate::from_ymd(2006, 7, 31));
    }

    fn calendar_dates(target: Vec<CalendarDate>) {
        let target = &target[0];
        assert_eq!(target.service_id, "WD");
        assert_eq!(target.date, NaiveDate::from_ymd(2006, 7, 3));
        assert_eq!(target.exception_type, Exception::Deleted);
    }

    fn fare_attributes(target: Vec<FareAttribute>) {
        let target = &target[1];
        assert_eq!(target.id(), "2");
        assert!(target.price.eq(&0.5f64));
        assert_eq!(target.currency, "USD");
        assert_eq!(target.payment_method, PaymentMethod::Aboard);
        assert_eq!(target.transfers, Transfers::NoTransfer);
    }

    fn fare_rules(target: Vec<FareRule>) {
        let target = &target[0];
        assert_eq!(target.id(), "a");
        assert_eq!(target.route_id, Some("TSW".to_string()));
        assert_eq!(target.origin_id, Some("1".to_string()));
        assert_eq!(target.destination_id, Some("1".to_string()));
    }

    fn frequencies(target: Vec<Frequency>) {
        let target = &target[2];
        assert_eq!(target.trip_id, "AWE1");
        assert_eq!(parse_time_over_midnight(target.start_time), "20:30:00");
        assert_eq!(parse_time_over_midnight(target.end_time), "28:00:00");
        assert_eq!(target.headway_secs, 420);
        assert_eq!(target.exact_times, ServiceType::FrequencyBased);
    }

    fn levels(target: Vec<Level>) {
        let target = &target[1];
        assert_eq!(target.id(), "L1");
        assert_eq!(target.index, -1);
        assert_eq!(target.name, Some("Mezzanine".to_string()));
    }

    fn pathways(target: Vec<Pathway>) {
        let target = &target[1];
        assert_eq!(target.id(), "E2N1");
        assert_eq!(target.from_stop_id, "E2");
        assert_eq!(target.to_stop_id, "N1");
        assert_eq!(target.mode, PathwayMode::Stairs);
        assert_eq!(target.is_bidirectional, true);
    }

    fn routes(target: Vec<Route>) {
        let target_1 = &target[0];
        assert_eq!(target_1.id(), "A");
        assert_eq!(target_1.agency_id, Some("agency001".to_string()));
        assert_eq!(target_1.short_name, "17");
        assert_eq!(target_1.long_name, "Mission");
        assert_eq!(
            target_1.desc,
            Some("The \"A\" route travels from lower Mission to Downtown.".to_string())
        );
        assert_eq!(target_1.route_type, RouteType::Bus);
        assert_eq!(target_1.url, Some("http://route.url".to_string()));
        assert_eq!(target_1.route_color, Some(RGB8::from((255, 255, 255))));
        assert_eq!(target_1.route_text_color, Some(RGB8::from((0, 0, 0))));
        assert_eq!(
            target_1.continuous_pickup,
            ContinuousPickupDropOff::CoordinateWithDriver
        );
        assert_eq!(
            target_1.continuous_drop_off,
            ContinuousPickupDropOff::ArrangeByPhone
        );

        let target_2 = &target[1];
        assert_eq!(target_2.id(), "A");
        assert_eq!(target_2.agency_id, Some("agency001".to_string()));
        assert_eq!(target_2.short_name, "17");
        assert_eq!(target_2.long_name, "Mission");
        assert_eq!(
            target_2.desc,
            Some("The \"A\" route travels from lower Mission to Downtown.".to_string())
        );
        assert_eq!(target_2.route_type, RouteType::Bus);
        assert_eq!(target_2.url, Some("http://route.url".to_string()));
        assert_eq!(target_2.route_color, Some(RGB8::from((255, 255, 255))));
        assert_eq!(target_2.route_text_color, Some(RGB8::from((0, 0, 0))));
        assert_eq!(
            target_2.continuous_pickup,
            ContinuousPickupDropOff::NotAvailable
        );
        assert_eq!(
            target_2.continuous_drop_off,
            ContinuousPickupDropOff::NotAvailable
        );
    }

    fn shapes(target: Vec<Shape>) {
        let target = &target[1];
        assert_eq!(target.id(), "A_shp");
        assert!(target.latitude.eq(&37.64430));
        assert!(target.longitude.eq(&-122.41070));
        assert_eq!(target.sequence, 2);
        assert!(target.dist_traveled.eq(&Some(6.8310)));
    }

    fn raw_stop_times(target: Vec<RawStopTime>) {
        let target_1 = &target[0];
        assert_eq!(target_1.trip_id, "trip1");
        assert_eq!(target_1.arrival_time, Some(14 * 60 * 60));
        assert_eq!(target_1.departure_time, Some(14 * 60 * 60));
        assert_eq!(target_1.stop_id, "stop2");
        assert_eq!(target_1.stop_sequence, 0);
        assert_eq!(target_1.pickup_type, PickupDropOffType::Regular);
        assert_eq!(target_1.drop_off_type, PickupDropOffType::NotAvailable);
        assert_eq!(
            target_1.continuous_pickup,
            ContinuousPickupDropOff::ArrangeByPhone
        );
        assert_eq!(
            target_1.continuous_drop_off,
            ContinuousPickupDropOff::CoordinateWithDriver
        );
        assert_eq!(target_1.timepoint, true);

        let target_2 = &target[1];
        assert_eq!(target_2.trip_id, "trip1");
        assert_eq!(target_2.arrival_time, Some(15 * 60 * 60));
        assert_eq!(target_2.departure_time, Some(15 * 60 * 60));
        assert_eq!(target_2.stop_id, "stop3");
        assert_eq!(target_2.stop_sequence, 0);
        assert_eq!(target_2.pickup_type, PickupDropOffType::Regular);
        assert_eq!(target_2.drop_off_type, PickupDropOffType::Regular);
        assert_eq!(
            target_2.continuous_pickup,
            ContinuousPickupDropOff::NotAvailable
        );
        assert_eq!(
            target_2.continuous_drop_off,
            ContinuousPickupDropOff::NotAvailable
        );
        assert_eq!(target_2.timepoint, true);
    }

    fn stops(target: Vec<Stop>) {
        let target = &target[0];
        assert_eq!(target.id(), "stop1");
        assert_eq!(target.name, Some("Stop Area".to_string()));
        assert_eq!(target.description, None);
        assert!(target.latitude.eq(&Some(48.796058)));
        assert!(target.longitude.eq(&Some(2.449386)));
        assert_eq!(target.location_type, StopLocationType::StopArea);
        assert_eq!(
            target.wheelchair_boarding,
            WheelchairBoardingAvailable::InformationNotAvailable
        );
    }

    fn transfers(target: Vec<Transfer>) {
        let target_1 = &target[0];
        assert_eq!(target_1.from_stop_id, "S6");
        assert_eq!(target_1.to_stop_id, "S7");
        assert_eq!(target_1.transfer_type, TransferType::TimedMinimum);
        assert_eq!(target_1.min_transfer_time, Some(300));

        let target_2 = &target[1];
        assert_eq!(target_2.from_stop_id, "S7");
        assert_eq!(target_2.to_stop_id, "S6");
        assert_eq!(target_2.transfer_type, TransferType::NotPossible);
        assert_eq!(target_2.min_transfer_time, None);
    }

    fn raw_trips(target: Vec<RawTrip>) {
        let target = &target[0];
        assert_eq!(target.id(), "trip1");
        assert_eq!(target.route_id, "route1");
        assert_eq!(target.service_id, "service1");
        assert_eq!(target.headsign, Some("85088452".to_string()));
        assert_eq!(target.short_name, None);
        assert_eq!(target.direction_id, Some(Direction::Outbound));
        assert_eq!(target.block_id, None);
        assert_eq!(target.bikes_allowed, BikesAllowed::NoBikeInfo);
        assert_eq!(
            target.wheelchair_accessible,
            WheelchairBoardingAvailable::InformationNotAvailable
        );
    }
}
