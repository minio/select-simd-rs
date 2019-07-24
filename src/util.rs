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

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::prelude::*;


pub fn write_file(filename: &str, data_vec: &Vec<u32>) {
    let mut fileout = File::create(filename).unwrap();

    let mut result: Vec<u8> = Vec::new();
    for n in data_vec {
        let _ = result.write_u32::<LittleEndian>(*n);
    }

    fileout.write_all(&result).unwrap();
}

pub fn index_and(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity((a.len() + b.len()) / 2);
    let mut ai = 0;
    let mut bi = 0;

    while ai < a.len() && bi < b.len() {
        if a[ai] == b[bi] {
            vec.push(a[ai]);
            ai += 1;
            bi += 1;
        } else if a[ai] < b[bi] {
            ai += 1;
        } else {
            bi += 1;
        }
    }

    return vec;
}

pub fn index_or(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::with_capacity((a.len() + b.len()) / 2);
    let mut ai = 0;
    let mut bi = 0;

    while ai < a.len() && bi < b.len() {
        if a[ai] == b[bi] {
            vec.push(a[ai]);
            ai += 1;
            bi += 1;
        } else if a[ai] < b[bi] {
            vec.push(a[ai]);
            ai += 1;
        } else {
            vec.push(b[bi]);
            bi += 1;
        }
    }

    // OR in remaining items
    while ai < a.len() {
        vec.push(a[ai]);
        ai += 1;
    }
    while bi < b.len() {
        vec.push(b[bi]);
        bi += 1;
    }

    return vec;
}

#[cfg(test)]
mod util {
    use super::*;

    #[test]
    fn test_index_and() {
        let left_vec: Vec<i32> = vec![100, 300, 400, 600, 800, 850];
        let right_vec: Vec<i32> = vec![100, 200, 300, 400, 500, 700, 800, 900, 1000];
        static EXPECTED: &'static [i32] = &[100, 300, 400, 800];

        let anded = index_and(&left_vec, &right_vec);
        assert_eq!(anded, EXPECTED);
    }

    #[test]
    fn test_index_or() {
        let left_vec: Vec<i32> = vec![100, 300, 400, 600, 800, 850];
        let right_vec: Vec<i32> = vec![100, 200, 300, 400, 500, 700, 800, 900, 1000];
        static EXPECTED: &'static [i32] = &[100, 200, 300, 400, 500, 600, 700, 800, 850, 900, 1000];

        let orred = index_or(&left_vec, &right_vec);
        assert_eq!(orred, EXPECTED);

        let orred_rev = index_or(&right_vec, &left_vec);
        assert_eq!(orred_rev, EXPECTED);
    }

}
