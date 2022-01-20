// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_arrow::arrow::bitmap::Bitmap;
use common_datavalues2::chrono::DateTime;
use common_datavalues2::chrono::TimeZone;
use common_datavalues2::chrono::Utc;
use common_datavalues2::prelude::*;
use common_exception::Result;

use super::cast_with_type::arrow_cast_compute;
use super::cast_with_type::CastOptions;

const DATE_FMT: &str = "%Y-%m-%d";
const TIME_FMT: &str = "%Y-%m-%d %H:%M:%S";
const TIME64_FMT: &str = "%Y-%m-%d %H:%M:%S%.f";

pub fn cast_from_date16(
    column: &ColumnRef,
    data_type: &DataTypePtr,
    cast_options: &CastOptions,
) -> Result<(ColumnRef, Option<Bitmap>)> {
    let c = Series::remove_nullable(column);
    let c: &UInt16Column = Series::check_get(&c)?;
    let size = c.len();

    match data_type.data_type_id() {
        TypeID::String => {
            let mut builder = MutableStringColumn::with_capacity(size);

            for v in c.iter() {
                let s = datetime_to_string(Utc.timestamp(*v as i64 * 24 * 3600, 0_u32), DATE_FMT);
                builder.append(s.as_bytes());
            }
            Ok((builder.as_column(), None))
        }

        _ => arrow_cast_compute(column, data_type, cast_options),
    }
}

pub fn cast_from_date32(
    column: &ColumnRef,
    data_type: &DataTypePtr,
    cast_options: &CastOptions,
) -> Result<(ColumnRef, Option<Bitmap>)> {
    let c = Series::remove_nullable(column);
    let c: &Int32Column = Series::check_get(&c)?;
    let size = c.len();

    match data_type.data_type_id() {
        TypeID::String => {
            let mut builder = MutableStringColumn::with_capacity(size);

            for v in c.iter() {
                let s = datetime_to_string(Utc.timestamp(*v as i64 * 24 * 3600, 0_u32), DATE_FMT);
                builder.append(s.as_bytes());
            }
            Ok((builder.as_column(), None))
        }

        _ => arrow_cast_compute(column, data_type, cast_options),
    }
}

pub fn cast_from_datetime32(
    column: &ColumnRef,
    data_type: &DataTypePtr,
    cast_options: &CastOptions,
) -> Result<(ColumnRef, Option<Bitmap>)> {
    let c = Series::remove_nullable(column);
    let c: &UInt32Column = Series::check_get(&c)?;
    let size = c.len();

    match data_type.data_type_id() {
        TypeID::String => {
            let mut builder = MutableStringColumn::with_capacity(size);

            for v in c.iter() {
                let s = datetime_to_string(Utc.timestamp(*v as i64, 0_u32), TIME_FMT);
                builder.append(s.as_bytes());
            }
            Ok((builder.as_column(), None))
        }

        _ => arrow_cast_compute(column, data_type, cast_options),
    }
}

pub fn cast_from_datetime64(
    column: &ColumnRef,
    data_type: &DataTypePtr,
    cast_options: &CastOptions,
) -> Result<(ColumnRef, Option<Bitmap>)> {
    let c = Series::remove_nullable(column);
    let c: &UInt64Column = Series::check_get(&c)?;
    let size = c.len();

    match data_type.data_type_id() {
        TypeID::String => {
            let mut builder = MutableStringColumn::with_capacity(size);
            let date_time64 = data_type.as_any().downcast_ref::<DateTime64Type>().unwrap();
            for v in c.iter() {
                let s = datetime_to_string(date_time64.utc_timestamp(*v), TIME64_FMT);
                builder.append(s.as_bytes());
            }
            Ok((builder.as_column(), None))
        }

        _ => arrow_cast_compute(column, data_type, cast_options),
    }
}

#[inline]
fn datetime_to_string(date: DateTime<Utc>, fmt: &str) -> String {
    date.format(fmt).to_string()
}
