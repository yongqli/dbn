mod csv;
mod dbz;
mod json;

use std::{fmt, io};

use anyhow::anyhow;

use self::csv::{serialize::CsvSerialize, write_csv};
use crate::Dbz;
use databento_defs::{
    enums::Schema,
    tick::{Mbp10Msg, Mbp1Msg, OhlcvMsg, StatusMsg, SymDefMsg, TbboMsg, Tick, TickMsg, TradeMsg},
};
use json::write_json;

/// An encoding that DBZs can be translated to.
#[derive(Clone, Copy, Debug)]
pub enum OutputEncoding {
    /// Comma-separate values.
    Csv,
    /// JavaScript object notation.
    Json,
}

impl<R: io::BufRead> Dbz<R> {
    /// Streams the contents of the [Dbz] to `writer` encoding it using `encoding`. Consumes the
    /// [Dbz] object.
    ///
    /// # Errors
    /// This function returns an error if [Dbz::schema()] is [Schema::Statistics]. It will also
    /// return an error if there's an issue writing the output to `writer`.
    pub fn write_to(self, writer: impl io::Write, encoding: OutputEncoding) -> anyhow::Result<()> {
        match self.schema() {
            Schema::Mbo => self.write_with_tick_to::<TickMsg, _>(writer, encoding),
            Schema::Mbp1 => self.write_with_tick_to::<Mbp1Msg, _>(writer, encoding),
            Schema::Mbp10 => self.write_with_tick_to::<Mbp10Msg, _>(writer, encoding),
            Schema::Tbbo => self.write_with_tick_to::<TbboMsg, _>(writer, encoding),
            Schema::Trades => self.write_with_tick_to::<TradeMsg, _>(writer, encoding),
            Schema::Ohlcv1s | Schema::Ohlcv1m | Schema::Ohlcv1h | Schema::Ohlcv1d => {
                self.write_with_tick_to::<OhlcvMsg, _>(writer, encoding)
            }
            Schema::Definition => self.write_with_tick_to::<SymDefMsg, _>(writer, encoding),
            Schema::Statistics => Err(anyhow!("Not implemented for schema Statistics")),
            Schema::Status => self.write_with_tick_to::<StatusMsg, _>(writer, encoding),
        }
    }

    fn write_with_tick_to<T, W>(self, writer: W, encoding: OutputEncoding) -> anyhow::Result<()>
    where
        T: TryFrom<Tick> + CsvSerialize + fmt::Debug,
        W: io::Write,
    {
        let iter = self.try_into_iter::<T>()?;
        match encoding {
            OutputEncoding::Csv => write_csv(iter, writer),
            OutputEncoding::Json => write_json(iter, writer),
        }
    }
}

#[cfg(test)]
mod test_data {
    use databento_defs::tick::{BidAskPair, CommonHeader};

    pub const COMMON_HEADER: CommonHeader = CommonHeader {
        nwords: 30,
        type_: 4,
        publisher_id: 1,
        product_id: 323,
        ts_event: 1658441851000000000,
    };

    pub const BID_ASK: BidAskPair = BidAskPair {
        bid_price: 372000000000000,
        ask_price: 372500000000000,
        bid_size: 10,
        ask_size: 5,
        bid_orders: 5,
        ask_orders: 2,
    };
}
