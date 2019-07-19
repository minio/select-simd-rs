// This file is part of select-simd-rs
// Copyright (c) 2019 MinIO, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(asm)]

mod conv;
mod eval;
mod parse;
mod scan;
mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_query() {
        let mut message = Vec::new();
        let mut f = File::open("tests/parking-citations-10K.csv").unwrap();
        f.read_to_end(&mut message).unwrap(); // read the whole file

        let size: usize = 10000 / 64;
        let indices_vec: Vec<u32> = vec![0; size * 64];

        let _result = scan::scan_delimiter(
            message.as_ptr(),
            message.len().try_into().unwrap(),
            indices_vec.as_ptr(),
            indices_vec.len().try_into().unwrap(),
            0,
            0x0a,
        );

        let make_indices_vec: Vec<u32> = vec![0; indices_vec.len()];

        parse::detect_separator(
            message.as_ptr(),
            indices_vec.as_ptr(),
            indices_vec.len().try_into().unwrap(),
            make_indices_vec.as_ptr(),
            8,
            0x2c,
        );

        let active_vec: Vec<u32> = vec![0; 2048];

        const HONDA: u32 = 0x444e4f48;

        let _count = eval::eval_string_equal(
            message.as_ptr(),
            make_indices_vec.as_ptr(),
            make_indices_vec.len().try_into().unwrap(),
            HONDA,
            active_vec.as_ptr(),
            active_vec.len().try_into().unwrap(),
        );

        let fine_size: usize = make_indices_vec.len() / 64;
        let fine_indices_vec: Vec<u32> = vec![0; fine_size * 64];

        parse::detect_separator(
            message.as_ptr(),
            active_vec.as_ptr(),
            active_vec.len().try_into().unwrap(),
            fine_indices_vec.as_ptr(),
            8,
            0x2c,
        );

        let mut fine_indices_vec64: Vec<u64> = vec![0; fine_size * 64];
        for v in fine_indices_vec.iter().enumerate() {
            fine_indices_vec64[v.0] = *v.1 as u64;
        }

        let i64s_vec: Vec<i64> = vec![0; 8];
        conv::conv_atoi64(
            message.as_ptr(),
            fine_indices_vec64.as_ptr(),
            i64s_vec.as_ptr(),
        );

        static EXPECTED: &'static [i64] = &[50, 73, 50, 93, 363, 50, 50, 50];

        assert_eq!(i64s_vec, EXPECTED);
    }
}
