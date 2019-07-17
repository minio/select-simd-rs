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

mod scan;
mod parse;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn test_args(a: i32, b: i32, message_ptr: *const u8, buf_ptr: *const u8) -> i32 {
    let c: i32;
    unsafe {
        asm!("
    add $0, $2
	mov           rax, 0x40
	vpbroadcastb  zmm2, eax
    vmovdqu32     [rcx], zmm2 
	mov           rax, 0x17
	vpbroadcastb  zmm3, eax
    vmovdqu32     [rdx], zmm3 
"
             : "=r"(c)
             : "0"(a), "r"(b), "{rdx}"(message_ptr), "{rcx}"(buf_ptr)
             :: "intel" );
    }
    c
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;
    
    #[test]
    fn test_arguments() {
        let mut f = File::open("/home/ec2-user/go/src/github.com/minio/select-simd/parking-citations-10K.csv").unwrap();
        let mut buffer = Vec::new();
    
        // read the whole file
        f.read_to_end(&mut buffer).unwrap();

        let zero_vec = vec![0; 128];

        let buffer_ptr: *const u8 = buffer.as_ptr();
        let zero_ptr: *const u8 = zero_vec.as_ptr();
        assert_eq!(test_args(7, 13, buffer_ptr, zero_ptr), 20);

        println!("{:?}", zero_vec);
        println!("{:?}", &buffer[..128]);
        
        // scan_delimiters
        // detect_separators
        // eval_string_compare
        // 
    }
}
