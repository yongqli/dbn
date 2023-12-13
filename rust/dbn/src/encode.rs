//! Encoding DBN and Zstd-compressed DBN files and streams. Encoders implement the
//! [`EncodeDbn`] trait.
pub mod csv;
pub mod dbn;
mod dyn_encoder;
mod dyn_writer;
pub mod json;

use std::{fmt, io, num::NonZeroU64};

use streaming_iterator::StreamingIterator;

// Re-exports
pub use self::{
    csv::Encoder as CsvEncoder,
    dbn::{
        Encoder as DbnEncoder, MetadataEncoder as DbnMetadataEncoder,
        RecordEncoder as DbnRecordEncoder,
    },
    dyn_encoder::{DynEncoder, DynEncoderBuilder},
    dyn_writer::DynWriter,
    json::Encoder as JsonEncoder,
};
#[cfg(feature = "async")]
pub use self::{
    dbn::{
        AsyncEncoder as AsyncDbnEncoder, AsyncMetadataEncoder as AsyncDbnMetadataEncoder,
        AsyncRecordEncoder as AsyncDbnRecordEncoder,
    },
    dyn_writer::DynAsyncWriter,
    json::AsyncEncoder as AsyncJsonEncoder,
};

use crate::{
    decode::{DbnMetadata, DecodeRecordRef},
    rtype_method_dispatch, rtype_ts_out_method_dispatch, Error, HasRType, Record, RecordRef,
    Result,
};

use self::{csv::serialize::CsvSerialize, json::serialize::JsonSerialize};

/// Trait alias for [`HasRType`], `AsRef<[u8]>`, `CsvSerialize`, [`fmt::Debug`], and `JsonSerialize`.
pub trait DbnEncodable: Record + AsRef<[u8]> + CsvSerialize + fmt::Debug + JsonSerialize {}
impl<T> DbnEncodable for T where
    T: HasRType + AsRef<[u8]> + CsvSerialize + fmt::Debug + JsonSerialize
{
}

/// Trait for types that encode a DBN record of a specific type.
pub trait EncodeRecord {
    /// Encodes a single DBN record of type `R`.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_record<R: DbnEncodable>(&mut self, record: &R) -> Result<()>;

    /// Flushes any buffered content to the true output.
    ///
    /// # Errors
    /// This function returns an error if it's unable to flush the underlying writer.
    fn flush(&mut self) -> Result<()>;
}

/// Trait for types that encode DBN records with mixed schemas.
pub trait EncodeRecordRef {
    /// Encodes a single DBN [`RecordRef`].
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_record_ref(&mut self, record: RecordRef) -> Result<()>;

    /// Encodes a single DBN [`RecordRef`] with an optional `ts_out` (see
    /// [`record::WithTsOut`](crate::record::WithTsOut)).
    ///
    /// # Safety
    /// `ts_out` must be `false` if `record` does not have an appended `ts_out`.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    unsafe fn encode_record_ref_ts_out(&mut self, record: RecordRef, ts_out: bool) -> Result<()>;
}

/// Trait for types that encode DBN records with a specific record type.
pub trait EncodeDbn: EncodeRecord + EncodeRecordRef {
    /// Encodes a slice of DBN records.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_records<R: DbnEncodable>(&mut self, records: &[R]) -> Result<()> {
        for record in records {
            self.encode_record(record)?;
        }
        self.flush()?;
        Ok(())
    }

    /// Encodes a stream of DBN records.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_stream<R: DbnEncodable>(
        &mut self,
        mut stream: impl StreamingIterator<Item = R>,
    ) -> Result<()> {
        while let Some(record) = stream.next() {
            self.encode_record(record)?;
        }
        self.flush()?;
        Ok(())
    }

    /// Encodes DBN records directly from a DBN decoder.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_decoded<D: DecodeRecordRef + DbnMetadata>(&mut self, mut decoder: D) -> Result<()> {
        let ts_out = decoder.metadata().ts_out;
        while let Some(record) = decoder.decode_record_ref()? {
            // Safety: It's safe to cast to `WithTsOut` because we're passing in the `ts_out`
            // from the metadata header.
            unsafe { self.encode_record_ref_ts_out(record, ts_out) }?;
        }
        self.flush()?;
        Ok(())
    }

    /// Encodes DBN records directly from a DBN decoder, outputting no more than
    /// `limit` records.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_decoded_with_limit<D: DecodeRecordRef + DbnMetadata>(
        &mut self,
        mut decoder: D,
        limit: NonZeroU64,
    ) -> Result<()> {
        let ts_out = decoder.metadata().ts_out;
        let mut i = 0;
        while let Some(record) = decoder.decode_record_ref()? {
            // Safety: It's safe to cast to `WithTsOut` because we're passing in the `ts_out`
            // from the metadata header.
            unsafe { self.encode_record_ref_ts_out(record, ts_out) }?;
            i += 1;
            if i == limit.get() {
                break;
            }
        }
        self.flush()?;
        Ok(())
    }
}

/// Extension trait for text encodings.
pub trait EncodeRecordTextExt: EncodeRecord + EncodeRecordRef {
    /// Encodes a single DBN record of type `R` along with the record's text symbol.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_record_with_sym<R: DbnEncodable>(
        &mut self,
        record: &R,
        symbol: Option<&str>,
    ) -> Result<()>;

    /// Encodes a single DBN [`RecordRef`] along with the record's text symbol.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    fn encode_ref_with_sym(&mut self, record: RecordRef, symbol: Option<&str>) -> Result<()> {
        rtype_method_dispatch!(record, self, encode_record_with_sym, symbol)?
    }

    /// Encodes a single DBN [`RecordRef`] with an optional `ts_out` (see
    /// [`record::WithTsOut`](crate::record::WithTsOut)) along with the record's text
    /// symbol.
    ///
    /// # Safety
    /// `ts_out` must be `false` if `record` does not have an appended `ts_out`.
    ///
    /// # Errors
    /// This function returns an error if it's unable to write to the underlying writer
    /// or there's a serialization error.
    unsafe fn encode_ref_ts_out_with_sym(
        &mut self,
        record: RecordRef,
        ts_out: bool,
        symbol: Option<&str>,
    ) -> Result<()> {
        rtype_ts_out_method_dispatch!(record, ts_out, self, encode_record_with_sym, symbol)?
    }
}

/// The default Zstandard compression level used.
pub const ZSTD_COMPRESSION_LEVEL: i32 = 0;

fn zstd_encoder<'a, W: io::Write>(writer: W) -> Result<zstd::stream::AutoFinishEncoder<'a, W>> {
    let mut zstd_encoder = zstd::Encoder::new(writer, ZSTD_COMPRESSION_LEVEL)
        .map_err(|e| Error::io(e, "creating zstd encoder"))?;
    zstd_encoder
        .include_checksum(true)
        .map_err(|e| Error::io(e, "setting zstd checksum"))?;
    Ok(zstd_encoder.auto_finish())
}

#[cfg(test)]
mod test_data {
    use streaming_iterator::StreamingIterator;

    use crate::record::{BidAskPair, RecordHeader};

    // Common data used in multiple tests
    pub const RECORD_HEADER: RecordHeader = RecordHeader {
        length: 30,
        rtype: 4,
        publisher_id: 1,
        instrument_id: 323,
        ts_event: 1658441851000000000,
    };

    pub const BID_ASK: BidAskPair = BidAskPair {
        bid_px: 372000000000000,
        ask_px: 372500000000000,
        bid_sz: 10,
        ask_sz: 5,
        bid_ct: 5,
        ask_ct: 2,
    };

    /// A testing shim to get a streaming iterator from a [`Vec`].
    pub struct VecStream<T> {
        vec: Vec<T>,
        idx: isize,
    }

    impl<T> VecStream<T> {
        pub fn new(vec: Vec<T>) -> Self {
            // initialize at -1 because `advance()` is always called before
            // `get()`.
            Self { vec, idx: -1 }
        }
    }

    impl<T> StreamingIterator for VecStream<T> {
        type Item = T;

        fn advance(&mut self) {
            self.idx += 1;
        }

        fn get(&self) -> Option<&Self::Item> {
            self.vec.get(self.idx as usize)
        }
    }
}
