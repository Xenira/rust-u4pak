// This file is part of rust-u4pak.
//
// rust-u4pak is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rust-u4pak is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rust-u4pak.  If not, see <https://www.gnu.org/licenses/>.

use crate::{pak::{Pak, COMPR_NONE, COMPR_ZLIB, COMPR_BIAS_MEMORY, COMPR_BIAS_SPEED}, util::print_table};
use crate::result::Result;
use crate::util::{format_size, Align};

pub fn info(pak: &Pak, human_readable: bool) -> Result<()> {
    let fmt_size = if human_readable {
        |size: u64| format_size(size)
    } else {
        |size: u64| format!("{}", size)
    };

    let mut sum_size              = 0;
    let mut sum_uncompressed_size = 0;
    let mut uncompr_count     = 0usize;
    let mut zlib_count        = 0usize;
    let mut bias_speed_count  = 0usize;
    let mut bias_memory_count = 0usize;
    let mut other_count       = 0usize;
    let mut encrypted_count   = 0usize;
    let mut sum_uncompr_size     = 0;
    let mut sum_zlib_size        = 0;
    let mut sum_bias_speed_size  = 0;
    let mut sum_bias_memory_size = 0;
    let mut sum_unknown_size     = 0;
    let mut sum_encrypted_size   = 0;

    let mut sum_uncompr_zlib_size        = 0;
    let mut sum_uncompr_bias_speed_size  = 0;
    let mut sum_uncompr_bias_memory_size = 0;
    let mut sum_uncompr_unknown_size     = 0;
    let mut sum_uncompr_encrypted_size   = 0;

    for record in pak.records() {
        sum_size += record.size();
        sum_uncompressed_size += record.uncompressed_size();
        if record.encrypted() {
            encrypted_count += 1;
            sum_encrypted_size += record.size();
            sum_uncompr_encrypted_size += record.uncompressed_size();
        }
        match record.compression_method() {
            self::COMPR_NONE => {
                uncompr_count += 1;
                sum_uncompr_size += record.size();
            }
            self::COMPR_ZLIB => {
                zlib_count += 1;
                sum_zlib_size += record.size();
                sum_uncompr_zlib_size += record.uncompressed_size();
            }
            self::COMPR_BIAS_SPEED => {
                bias_speed_count += 1;
                sum_bias_speed_size += record.size();
                sum_uncompr_bias_speed_size += record.uncompressed_size();
            }
            self::COMPR_BIAS_MEMORY => {
                bias_memory_count += 1;
                sum_bias_memory_size += record.size();
                sum_uncompr_bias_memory_size += record.uncompressed_size();
            }
            _ => {
                other_count += 1;
                sum_unknown_size += record.size();
                sum_uncompr_unknown_size += record.uncompressed_size();
            }
        }
    }

    println!("Pak Version: {}", pak.version());
    println!("Mount Point: {}", pak.mount_point().unwrap_or(""));
    println!();

    print_table(
        &["", "Count", "Size", "Uncompr."],
        &[Align::Left, Align::Right, Align::Right, Align::Right],
        &[
            vec!["Files:",              &format!("{}", pak.records().len()), &fmt_size(sum_size),             &fmt_size(sum_uncompressed_size)],
            vec!["Uncompr.:",           &format!("{}", uncompr_count),       &fmt_size(sum_uncompr_size),     ""],
            vec!["ZLIB Compr.:",        &format!("{}", zlib_count),          &fmt_size(sum_zlib_size),        &fmt_size(sum_uncompr_zlib_size)],
            vec!["Bias Speed Compr.:",  &format!("{}", bias_speed_count),    &fmt_size(sum_bias_speed_size),  &fmt_size(sum_uncompr_bias_speed_size)],
            vec!["Bias Memory Compr.:", &format!("{}", bias_memory_count),   &fmt_size(sum_bias_memory_size), &fmt_size(sum_uncompr_bias_memory_size)],
            vec!["Unknown Compr.:",     &format!("{}", other_count),         &fmt_size(sum_unknown_size),     &fmt_size(sum_uncompr_unknown_size)],
            vec!["Encrypted:",          &format!("{}", encrypted_count),     &fmt_size(sum_encrypted_size),   &fmt_size(sum_uncompr_encrypted_size)],
        ],
    );

    Ok(())
}
