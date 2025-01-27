# Changelog

## 0.15.1 - 2024-01-23

### Bug fixes
- Fixed an import error in the Python type stub file

## 0.15.0 - 2024-01-16
### Enhancements
- Improved `Debug` implementation for all record types
  - Prices are formatted as decimals
  - Fixed-length strings are formatted as strings
  - Bit flag fields are formatted as binary
  - Several fields are formatted as enums instead of their raw representations
- Improved `Debug` implementation for `RecordRef` to show `RecordHeader`
- Added `--schema` option to `dbn` CLI tool to filter a DBN to a particular schema. This
  allows outputting saved live data to CSV
- Allowed passing `--limit` option to `dbn` CLI tool with `--metadata` flag
- Improved performance of decoding uncompressed DBN fragments with the `dbn` CLI tool
- Added builders to `CsvEncoder`, `DynEncoder`, and `JsonEncoder` to assist with the
  growing number of customizations
  - Added option to write CSV header as part of creating `CsvEncoder` to make it harder
    to forget
- Added `-s`/`--map-symbols` flag to CLI to create a `symbol` field in the output with
  the text symbol mapped from the instrument ID
- Added `version` param to Python `Metadata` contructor choose between DBNv1 and DBNv2
- Implemented `EncodeRecordTextExt` for `DynEncoder`
- Implemented `Deserialize` and `Serialize` for all records and enums (with `serde`
  feature enabled). This allows serializing records with additional encodings not
  supported by the DBN crate
- Implemented `Hash` for all record types
- Added new publisher value for OPRA MIAX Sapphire
- Added Python type definition for `Metadata.__init__`
- Added `metadata_mut` method to decoders to get a mutable reference to the decoded
  metadata
- Improved panic message on `RecordRef::get` when length doesn't match expected to be
  actionable
- Added `encode::ZSTD_COMPRESSION_LEVEL` constant

### Breaking changes
- Increased size of `SystemMsg` and `ErrorMsg` to provide better messages from Live
  gateway
  - Increased length of `err` and `msg` fields for more detailed messages
  - Added `is_last` field to `ErrorMsg` to indicate the last error in a chain
  - Added `code` field to `SystemMsg` and `ErrorMsg`, although currently unused
  - Added new `is_last` parameter to `ErrorMsg::new`
  - Decoding these is backwards-compatible and records with longer messages won't be
    sent during the DBN version 2 migration period
  - Renamed previous records to `compat::ErrorMsgV1` and `compat::SystemMsgV1`
- Split `DecodeDbn` trait into `DecodeRecord` and `DbnMetadata` traits for more
  flexibility. `DecodeDbn` continues to exist as a trait alias
- Moved `decode_stream` out of `DecodeDbn` to its own separate trait `DecodeStream`
- Changed trait bounds of `EncodeDbn::encode_decoded` and `encode_decoded_with_limit` to
  `DecodeRecordRef + DbnMetadata`

### Bug fixes
- Fixed panic in `TsSymbolMap` when `start_date` == `end_date`
- Added missing Python `__eq__` and `__ne__` implementations for `BidAskPair`
- Fixed Python `size_hint` return value for `InstrumentDefMsgV1` and
  `SymbolMappingMsgV1`
- Fixed cases where `dbn` CLI tool would write a broken pipe error to standard error
  such as when piping to `head`
- Fixed bug in sync and async `MetadataEncoder`s where `version` was used to determine
  the encoded length of fixed-length symbols instead of the `symbol_cstr_len` field

## 0.14.2 - 2023-11-17
### Enhancements
- Added `set_upgrade_policy` setters to `DbnDecoder`, `DbnRecordDecoder`,
  `AsyncDbnDecoder`, and `AsyncDbnRecordDecoder`
- Added `from_schema` classmethod for Python `RType` enum

### Breaking changes
- Renamed parameter for Python Enum classmethod constructors to `value` from `data`.

## 0.14.1 - 2023-11-17
### Enhancements
- Added new trait `compat::SymbolMappingRec` for code reuse when working with
  both versions of `SymbolMappingMsg`
- Changed `PitSymbolMap::on_symbol_mapping` to accept either version of
  `SymbolMappingMsg`

### Bug fixes
- Fixed missing DBNv1 compatibility in `PitSymbolMap::on_record`
- Fixed missing Python export for `VersionUpgradePolicy`
- Fixed missing Python export and methods for `InstrumentDefMsgV1` and
  `SymbolMappingMsgV1`
- Fixed bug where Python `DbnDecoder` and `Transcoder` would throw exceptions
  when attempting to decode partial metadata

## 0.14.0 - 2023-11-15
### Enhancements
- This version begins the transition to DBN version 2 (DBNv2). In this version, the
  decoders support decoding both versions of DBN and the DBN encoders default to
  keeping version of the input. However, in a future version, decoders will by default
  convert DBNv1 to DBNv2 and support will be dropped for encoding DBNv1.
  - Affects `SymbolMappingMsg`, `InstrumentDefMsg`, and `Metadata`. All other record
    types and market data schemas are unchanged
  - Version 1 structs can be converted to version 2 structs with the `From` trait
- Added `symbol_cstr_len` field to `Metadata` to indicate the length of fixed symbol
  strings
- Added `stype_in` and `stype_out` fields to `SymbolMappingMsg` to provide more context
  with live symbology updates
- Added smart wrapping to `dbn` CLI help output
- Updated `rtype_dispatch` family of macros to check record length to handle both
  versions of records. This is temporary during the transition period
- Added `VersionUpgradePolicy` enum and associated methods to the decoders to
  allow specifying how to handle decoding records from prior DBN versions
- Added `Metadata::upgrade()` method to update `Metadata` from a prior DBN version to
  the latest version
- Added `-u`/`--upgrade` flags to `dbn` CLI that when passed upgrades DBN data from
  previous versions. By default data is decoded as-is
- Made `AsyncDbnDecoder::decode_record`, `AsyncDbnDecoder::decode_record_ref`,
  `dbn::AsyncRecordDecoder::decode`, and `dbn::AsyncRecordDecoder::decode_ref`
  cancellation safe. This makes them safe to use within a
  `tokio::select!`(https://docs.rs/tokio/latest/tokio/macro.select.html) statement
- Added documention around cancellation safety for async APIs
- Improved error messages for conversion errors
- Added `TOB` flag to denote top-of-book messages
- Added new publisher values in preparation for IFEU.IMPACT and NDEX.IMPACT datasets
- Added new publisher values for consolidated DBEQ.BASIC and DBEQ.PLUS
- Added `MAX_RECORD_LEN` constant for the length of the largest record type
- Exposed record flag constants in `databento_dbn` with `F_` prefix
- Added export to Python for `RType`

### Breaking changes
- The old `InstrumentDefMsg` is now `compat::InstrumentDefMsgV1`
- `compat::InstrumentDefMsgV2` is now an alias for `InstrumentDefMsg`
- The old `SymbolMappingMsg` is now `compat::SymbolMappingMsgV1`
- `compat::SymbolMappingMsgV2` is now an alias for `SymbolMappingMsg`
- Changed `SYMBOL_CSTR_LEN` constant to 71. Previous value is now in
  `compat::SYMBOL_CSTR_V1`
- Changed `DBN_VERSION` constant to 2
- `security_update_action` was converted to a raw `c_char` to safely support adding
  variants in the future
- Renamed `_dummy` in `InstrumentDefMsg` to `_reserved`
- Removed `_reserved2`, `_reserved3`, and `_reserved5` from `InstrumentDefMsg`
- Removed `_dummy` from `SymbolMappingMsg`
- Moved position of `strike_price` within `InstrumentDefMsg` but left text serialization
  order unchanged
- Made `Error` non-exhaustive, meaning it can no longer be exhaustively matched against.
  This allows adding additional error variants in the future without a breaking change
- Added `upgrade_policy` parameter to `RecordDecoder::with_version` constructor to
  control whether records of previous versions will be upgraded
- Added `upgrade_policy` parameter to `DynDecoder` constructors to control whether
  records of previous versions will be upgraded
- Renamed `symbol_map` parameter for Python Transcoder to `symbol_interval_map` to
  better reflect the date intervals it contains

### Deprecations
- Deprecated unused `write_dbn_file` function from Python interface. Please use
  `Transcoder` instead

### Bug fixes
- Fixed typo in Python type definition for `InstrumentDefMsg.pretty_high_limit_price`
- Fixed type signature for `Metadata.stype_in` and `Metadata.stype_out` Python methods
- Fixed incorrect version in `pyproject.toml`

## 0.13.0 - 2023-10-20
### Enhancements
- Added `SymbolMappingMsgV2::new` method
- Added `Record` trait for all types beginning with a `RecordHeader`
  - Added new `index_ts` and `raw_index_ts` methods to `Record` trait, which returns the
    primary timestamp for a record
- Added `RecordMut` trait for accessing a mutable reference to a `RecordHeader`
- Implemented `PartialOrd` for all record types, based on `raw_index_ts`
- Loosened `DbnEncodable` from requiring `HasRType` to only requiring `Record`. This means
  `RecordRef`s and concrete records can be encoded with the same methods

### Breaking changes
- Split part of `HasRType` into new `Record` and `RecordMut` traits, which are object-
  safe: they can be used in `Box<dyn>`. `RecordRef` also implements `Record`, so it's
  easier to write code that works for both concrete records as well as `RecordRef`
- Removed `RecordRef` methods made redundant by it implementing `Record`
- Removed `input_compression` parameter from Python `Transcoder`

### Deprecations
- Deprecated `SymbolIndex::get_for_rec_ref`, which was made redundant by loosening the
  trait bound on `SymbolIndex::get_for_rec` to accept `RecordRef`s

### Bug fixes
- Fixed `TsSymbolMap` not always using the correct timestamp for getting the mapped
  symbol

## 0.12.0 - 2023-10-16
### Enhancements
- Added `map_symbols` support to Python `Transcoder`
- Added new publisher variants in preparation for DBEQ.PLUS dataset
- Added `from_dataset_venue` function to `Publisher` to facilitate destructuring
- Implemented `Default` for most records to make testing easier
- Added `from_zstd` function to `AsyncDbnEncoder` to match synchronous encoder
- Added re-exports for `enums::flags`, `enums::rtype`, `record::BidAskPair`,
  `record::RecordHeader`, and `record::WithTsOut` to simplify imports
- Added `--fragment` CLI flag for writing DBN without the metadata header
- Added `--input-dbn-version` CLI option for specifying the DBN version of a DBN
  fragment
- Added `serde::Deserialize` implementations for `Dataset`, `Venue`, and `Publisher`
- Added support for Python 3.12 to `databento_dbn`
- Added `RecordDecoder::with_version` for future use when dealing with compatibility
  between different DBN versions
- Added new dispatch macros: `rtype_ts_out_method_dispatch`,
  `rtype_ts_out_async_method_dispatch`, `rtype_method_dispatch`, and
  `schema_ts_out_method_dispatch`
- Added `InstrumentDefMsgV2` and `SymbolMappingMsgV2` for forward compatibility with a
  version of DBN
- Added `TsSymbolMap` and `PitSymbolMap` to aid with both historical and live symbology
  - Added support for inverse symbology, i.e. with `stype_in=InstrumentId`

### Breaking changes
- Changed `Metadata::symbol_map` to return `TsSymbolMap`
- Changed `Metadata::symbol_map_for_date` to return `PitSymbolMap`
- Changed `Default` implementation for `BidAskPair` by setting prices to `UNDEF_PRICE`
- Added new publisher values in preparation for DBEQ.PLUS
- Added `ts_out` parameter to `encode_header_for_schema` in `CsvEncoder` and
  `DynEncoder` to allow controlling whether "ts_out" is in the header

## 0.11.1 - 2023-10-05
### Enhancements
- Upgraded `async-compression` to 0.4.3
- Upgraded `csv` to 1.3
- Upgraded `num_enum` to 0.7

### Bug fixes
- Changed DBN stream detection to ignore the DBN version

## 0.11.0 - 2023-09-21
### Enhancements
- Added new `EncodeRecordTextExt` trait which is implemented for the CSV and JSON
  encoders. It adds two methods for encoding a `symbol` field along side the rest of the
  record fields, matching the behavior of `map_symbols` in the historical API
- Added `encode_header` and `encode_header_for_schema` methods to `CsvEncoder` and
  `DynEncoder` to give more flexibility for encoding CSV headers
- Added `from_file` and `from_zstd_file` functions to `AsyncDbnDecoder` to match
  synchronous decoder
- Implemented `Copy` for `RecordRef` to make it behave more like a reference
- Added `AsyncDbnEncoder` for simpler DBN encoding and to match sync API
- Added `RecordEnum` and `RecordRefEnum` to more easily be able to pattern match on
  records of different types
- Added `ARCX.PILLAR.ARCX` publisher
- Added `From` DBN records for `RecordRef`
- Added re-exports to the top level of the crate for all enums and records for simpler
  imports
- Added `ClosePrice` and `NetChange` `StatType`s used in the `OPRA.PILLAR` dataset

### Breaking changes
- Split `encode_record_ref` into a safe method with no arguments and an unsafe method
  with a `ts_out` parameter to reduce `unsafe` usage when not working with live data
  that may contain `ts_out`

### Bug fixes
- Fixed `dbn` CLI not writing CSV header when using `--fragment` and `--zstd-fragment`
  flags
- Fixed lifetime on return value from `RecordRef::get_unchecked`
- Fixed missing check for `stype_out` before building `Metadata` symbology maps

## 0.10.2 - 2023-09-12
### Bug fixes
- Fixed query range checking in `Metadata::symbol_map_for_date`
- Added `debug_assert_eq!` check for alignment in `RecordRef::new`

## 0.10.1 - 2023-09-07
### Bug fixes
- Changed `Metadata::symbol_map` and `symbol_map_for_date` to return `String` values
  instead of `&str`, which made it difficult to use

## 0.10.0 - 2023-09-07
### Enhancements
- Added `start` and `end` getters to `Metadata` that return `time::OffsetDateTime`
- Added `symbol_map` and `symbol_map_for_date` methods to `Metadata` to aid historical
  symbology mapping from the instrument IDs in records
- Added `DynReader` struct for being agnostic about whether an input stream is
  zstd-compressed
- Improved safety of `RecordRef::get` by adding length check
- Added Python DBN `Transcoder` class for converting DBN to JSON and CSV with optional
  zstd compression
- Added optional `has_metadata` parameter to Python `DBNDecoder` to allow
  decoding plain records by passing `False`. By default `DBNDecoder` expects a complete
  DBN stream, which begins with metadata
- Added `get_ref` methods to `dbn::Decoder` and `dbn::RecordDecoder` which return a
  reference to the inner reader
- Added `UNDEF_PRICE`, `UNDEF_ORDER_SIZE`, `UNDEF_STAT_QUANTITY`, and `UNDEF_TIMESTAMP`
  constants to `databento_dbn` Python package to make it easier to filter null values
- Added `Metadata::builder()` function to create a new builder instance

### Breaking changes
- Split out `EncodeRecordRef` trait from `EncodeDbn` to have a boxable trait (i.e.
  `Box<dyn EncodeRecordRef>`) for dynamic encoding
- Split out `EncodeRecord` trait from `EncodeDbn`
- Split out `DecodeRecordRef` trait from `DecodeDbn` to have a boxable trait (i.e.
  `Box<dyn DecodeRecordRef>`) for dynamic decoding
- Changed `DynWriter` from an enum to a struct with only private fields

### Bug fixes
- Fixed typo in `BATY.PITCH.BATY` publisher
- Fixed typo in `README.md` (credit: @thomas-k-cameron)

## 0.9.0 - 2023-08-24
### Enhancements
- Added `publisher` method to `RecordHeader` and all record types for converting
  the `publisher_id` to an enum
- Added getters that return `time::OffsetDateTime` for the following fields:
  `ts_event`, `ts_recv`, `ts_ref`, `activation`, `expiration`, `start_ts`, `end_ts`,
  `ts_out`
- Added getters for `ts_in_delta` that return `time::Duration`

## 0.8.3 - 2023-08-15
### Bug fixes
- Fixed missing `raw_instrument_id` field in Python `InstrumentDefMsg`
- Fixed missing `OHLCV_EOD` variant in Python `Schema` type hint

## 0.8.2 - 2023-08-10
### Enhancements
- Added new `OhlcvEod` schema variant for future use with OHLCV bars based around the
  end of the trading session
- Implemented `std::fmt::Display` for publisher enums (`Publisher`, `Dataset`, and
  `Venue`)

### Bug fixes
- Fixed Python type hint for `Encoding.variants()`

## 0.8.1 - 2023-08-02
### Enhancements
- Added `raw_instrument_id` field to `InstrumentDefMsg` (definition schema) for use in
  future datasets consolidated from multiple publishers
- Added new `OHLCV_EOD` rtype for future daily OHLCV schema based on the trading
  session
- Added new `SType::Nasdaq` and `SType::Cms` to support querying US equities datasets
  using either convention, regardless of the original convention of the dataset.
- Relaxed `pyo3`, `tokio`, and `zstd` dependency version requirements
- Added `FIXED_PRICE_SCALE` constant to `databento_dbn` Python package
- Added generated field metadata for each record type to aid in pandas DataFrame
  creation

### Breaking changes
- Changed `size_hint` class method to class attribute for Python records

### Bug fixes
- Fixed multi-frame Zstd decoding for async decoders

## 0.8.0 - 2023-07-19
### Enhancements
- Switched from `anyhow::Error` to custom `dbn::Error` for all public fallible functions
  and methods. This should make it easier to disambiguate between error types.
- `EncodeDbn::encode_record` and `EncodeDbn::record_record_ref` no longer treat a
  `BrokenPipe` error differently
- Added `AsyncDbnDecoder`
- Added `pretty::Px` and `pretty::Ts` newtypes to expose price and timestamp formatting
  logic outside of CSV and JSON encoding
- Added interning for Python strings
- Added `rtype` to encoded JSON and CSV to aid differeniating between different record types.
  This is particularly important when working with live data.
- Added `pretty_` Python attributes for DBN price fields
- Added `pretty_` Python attributes for DBN UTC timestamp fields

### Breaking changes
- All fallible operations now return a `dbn::Error` instead of an `anyhow::Error`
- Updated serialization order to serialize `ts_recv` and `ts_event` first
- Moved header fields (`rtype`, `publisher_id`, `instrument_id`, and `ts_event`) to
  nested object under the key `hd` in JSON encoding to match structure definitions
- Changed JSON encoding of all 64-bit integers to strings to avoid loss of precision
- Updated `MboMsg` serialization order to serialize `action`, `side`, and `channel_id`
  earlier given their importance
- Updated `Mbp1Msg`, `Mbp10Msg`, and `TradeMsg` serialization order to serialize
  `action`, `side`, and `depth` earlier given their importance
- Updated `InstrumentDefMsg` serialization order to serialize `raw_symbol`,
  `security_update_action`, and `instrument_class` earlier given their importance
- Removed `bool` return value from `EncodeDbn::encode_record` and
  `EncodeDbn::record_record_ref`. These now return `dbn::Result<()>`.

### Bug fixes
- Fixed handling of NUL byte when encoding DBN to CSV and JSON
- Fixed handling of broken pipe in `dbn` CLI tool

## 0.7.1 - 2023-06-26
- Added Python `variants` method to return an iterator over the enum variants for
  `Compression`, `Encoding`, `Schema`, and `SType`
- Improved Python enum conversions for `Compression`, `Encoding`, `Schema`, and `SType`

## 0.7.0 - 2023-06-20
### Enhancements
- Added publishers enums
- Added export to Python for `Compression`, `Encoding`, `SType`, and `Schema`
  enums
- Improved Python string representation of `ErrorMsg` and `SystemMsg`
- Added async JSON encoder

### Breaking changes
- Dropped support for Python 3.7

### Bug fixes
- Fixed pretty timestamp formatting to match API

## 0.6.1 - 2023-06-02
- Added `--fragment` and `--zstd-fragment` CLI arguments to read DBN streams
  without metadata
- Added `csv::Decoder::get_ref` that returns reference to the underlying writer
- Added missing Python getter for `InstrumentDefMsg::group`
- Added dataset constants
- Changed `c_char` fields to be exposed to Python as `str`

## 0.6.0 - 2023-05-26
### Enhancements
- Added `--limit NUM` CLI argument to output only the first `NUM` records
- Added `AsRef<[u8]>` implementation for `RecordRef`
- Added Python `size_hint` classmethod for DBN records
- Improved DBN encoding performance of `RecordRef`s
- Added `use_pretty_px` for price formatting and `use_pretty_ts` for datetime formatting
  to CSV and JSON encoders
- Added `UNDEF_TIMESTAMP` constant for when timestamp fields are unset

### Breaking changes
- Renamed `booklevel` MBP field to `levels` for brevity and consistent naming
- Renamed `--pretty-json` CLI flag to `--pretty` and added support for CSV. Passing this
  flag now also enables `use_pretty_px` and `use_pretty_ts`
- Removed `open_interest_qty` and `cleared_volume` fields that were always unset from
  definition schema
- Changed Python `DBNDecoder.decode` to return records with a `ts_out` attribute, instead
  of a tuple
- Rename Python `DbnDecoder` to `DBNDecoder`

### Bug fixes
- Fixed `Action` conversion methods (credit: @thomas-k-cameron)

## 0.5.1 - 2023-05-05
- Added `F`ill action type for MBO messages
- Added Python type stub for `StatMsg`

## 0.5.0 - 2023-04-25
### Enhancements
- Added support for Statistics schema
- Added `RType` enum for exhaustive pattern matching
- Added `&str` getters for more `c_char` array record fields
- Changed `DbnDecoder.decode` to always return a list of tuples

### Breaking changes
- Changed `schema` and `stype_in` to optional in `Metadata` to support live data
- Renamed `SType::ProductId` to `SType::InstrumentId` and `SType::Native` to `SType::RawSymbol`
- Renamed `RecordHeader::product_id` to `instrument_id`
- Renamed `InstrumentDefMsg::symbol` to `raw_symbol`
- Renamed `SymbolMapping::native_symbol` to `raw_symbol`
- Deprecated `SType::Smart` to split into `SType::Parent` and `SType::Continuous`

### Bug fixes
- Fixed value associated with `Side::None`
- Fixed issue with decoding partial records in Python `DbnDecoder`
- Fixed missing type hint for Metadata bytes support
- Added support for equality comparisons in Python classes

## 0.4.3 - 2023-04-07
- Fixed typo in Python type stubs

## 0.4.2 - 2023-04-06
- Fixed support for `ErrorMsg`, `SystemMsg`, and `SymbolMappingMsg` in Python

## 0.4.1 - 2023-04-05
### Enhancements
- Added enums `MatchAlgorithm`, `UserDefinedInstrument`
- Added constants `UNDEF_PRICE` and `UNDEF_ORDER_SIZE`
- Added Python type stubs for `databento_dbn` package

### Bug fixes
- Fixed `Metadata.__bytes__` method to return valid DBN
- Fixed panics when decoding invalid records
- Fixed issue with attempting to decode partial records in Python `DbnDecoder`
- Fixed support for `ImbalanceMsg` in Python `DbnDecoder`

## 0.4.0 - 2023-03-24
### Enhancements
- Added support for Imbalance schema
- Updated `InstrumentDefMsg` to include options-related fields and `instrument_class`
- Added support for encoding and decoding `ts_out`
- Added `ts_out` to `Metadata`
- Improved enum API
- Relaxed requirement for slice passed to `RecordRef::new` to be mutable
- Added error forwarding from `DecodeDbn` methods
- Added `SystemMsg` record
- Exposed constructor and additional methods for DBN records and `Metadata` to Python
- Made `RecordRef` implement `Sync` and `Send`

### Breaking changes
- Introduced separate rtypes for each OHLCV schema
- Removed `record_count` from `Metadata`
- Changed serialization of `c_char` fields to strings instead of ints
- Renamed `dbn::RecordDecoder::decode_record` to `decode`
- Renamed `dbn::RecordDecoder::decode_record_ref` to `decode_ref`
- Renamed `HasRType::size` to `record_size` to avoid confusion with order size fields
- Stopped serializing `related` and `related_security_id` fields in `InstrumentDefMsg`

## 0.3.2 - 2023-03-01
### Enhancements
- Added records and `Metadata` as exports of `databento_dbn` Python package
- Improved how `Metadata` appears in Python and added `__repr__`

### Bug fixes
- Fixed bug where `dbn` CLI tool didn't truncate existing files

## 0.3.1 - 2023-02-27
### Enhancements
- Added improved Python bindings for decoding DBN
- Standardized documentation for `start`, `end`, and `limit`

### Bug fixes
- Fixed bug with `encode_metadata` Python function

## 0.3.0 - 2023-02-22
### Enhancements
- Added ability to migrate legacy DBZ to DBN through CLI
- Relaxed requirement that DBN be Zstandard-compressed
- Folded in `databento-defs`
- Added support for async encoding and decoding
- Added billable size calculation to `dbn` CLI
- Added `MetadataBuilder` to assist with defaults
- Refactored into encoder and decoder types

### Breaking changes
- Renamed DBZ to DBN
- Renamed python package to `databento-dbn`
- Moved metadata out of skippable frame

## 0.2.1 - 2022-12-02
- Added Python DBZ writing example
- Changed [databento-defs](https://crates.io/crates/databento-defs) dependency to
  crates.io version

## 0.2.0 - 2022-11-28
### Enhancements
- Added interface for writing DBZ files
- Enabled Zstd checksums
- Changed DBZ decoding to use [streaming-iterator](https://crates.io/crates/streaming-iterator)

### Breaking changes
- Changed JSON output to NDJSON

### Bug fixes
- Change nanosecond timestamps to strings in JSON to avoid loss of precision when parsing

## 0.1.5 - 2022-09-14
- Initial release
