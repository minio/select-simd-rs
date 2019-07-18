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

#[cfg(test)]
mod parse_tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;
    use std::convert::TryInto;
    
    #[test]
    fn test_detect_separator() {
        let mut message = Vec::new();
        let mut f = File::open("tests/parking-citations-10K.csv").unwrap();
        f.read_to_end(&mut message).unwrap(); // read the whole file

        let indices_vec: Vec<u32> = vec![0, 207, 333, 455, 575, 683, 812, 939, 1066, 1193, 1314, 1437, 1560, 1683, 1806, 1925,
											2045, 2162, 2281, 2404, 2524, 2637, 2737, 2867, 2993, 3118, 3237, 3353, 3472, 3582, 3705, 3823,
											3942, 4069, 4196, 4319, 4450, 4578, 4706, 4831, 4947, 5072, 5194, 5318, 5454, 5570, 5692, 5818,
											5941, 6070, 6195, 6320, 6438, 6564, 6688, 6812, 6936, 7056, 7185, 7302, 7428, 7556, 7686, 7815];
        let separator_indices_vec: Vec<u32> = vec![0; 64];

        detect_separator(message.as_ptr(), indices_vec.as_ptr(), indices_vec.len().try_into().unwrap(), separator_indices_vec.as_ptr(), 8, 0x2c);

		static EXPECTED: &'static [u32] = &[94, 256, 382, 504, 618, 731, 859, 988, 1115, 1241, 1363, 1486, 1609, 1732, 1854, 1974,
											2094, 2211, 2330, 2447, 2573, 2686, 2786, 2916, 3042, 3166, 3285, 3401, 3520, 3630, 3753, 3871,
											3990, 4117, 4245, 4368, 4499, 4627, 4759, 4878, 5002, 5125, 5248, 5367, 5497, 5619, 5741, 5867,
											0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(separator_indices_vec, EXPECTED);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn detect_separator(message_ptr: *const u8, indices_ptr: *const u32, indices_len: u64, separator_indices_ptr: *const u32, num_separator: u64, separator_char: u64) {
    unsafe {
        asm!("
    jmp main

detect_separator:
	vpbroadcastd  zmm28, ebx
	vpbroadcastb  zmm31, ecx
	kmovq         k7, rbx
	kmovq         k6, rcx
	mov           rcx, 0x01
	vpbroadcastd  zmm30, ecx
	vpxord        zmm29, zmm29, zmm29
	vpxord        zmm2, zmm2, zmm2
	vpcmpgtd      k5, zmm1, zmm0

loop:
	vpgatherdd    zmm25 {k5}, [rsi+zmm0*1]
	vpcmpeqb      k5, zmm25, zmm31
	vpmovm2b      zmm26, k5
	vpslld        zmm27, zmm26, 0x18
	vpmovd2m      k4, zmm27
	vpsubd        zmm28 {k4}, zmm28, zmm30
	vpaddd        zmm0, zmm0, zmm30
	vpcmpeqd      k3, zmm29, zmm28
	kandq         k3, k3, k4
	vmovdqa32     zmm2 {k3}, zmm0
	vpslld        zmm27, zmm26, 0x10
	vpmovd2m      k4, zmm27
	vpsubd        zmm28 {k4}, zmm28, zmm30
	vpaddd        zmm0, zmm0, zmm30
	vpcmpeqd      k3, zmm29, zmm28
	kandq         k3, k3, k4
	vmovdqa32     zmm2 {k3}, zmm0
	vpslld        zmm27, zmm26, 0x08
	vpmovd2m      k4, zmm27
	vpsubd        zmm28 {k4}, zmm28, zmm30
	vpaddd        zmm0, zmm0, zmm30
	vpcmpeqd      k3, zmm29, zmm28
	kandq         k3, k3, k4
	vmovdqa32     zmm2 {k3}, zmm0
	vpmovd2m      k4, zmm26
	vpsubd        zmm28 {k4}, zmm28, zmm30
	vpaddd        zmm0, zmm0, zmm30
	vpcmpeqd      k3, zmm29, zmm28
	kandq         k3, k3, k4
	vmovdqa32     zmm2 {k3}, zmm0
	vpcmpgtd      k5, zmm1, zmm0
	ktestq        k5, k5
	jz            done
	vpcmpgtd      k3, zmm28, zmm29
	ktestq        k3, k3
	jnz           loop

done:
	kmovq         rcx, k6
	kmovq         rbx, k7
	ret  

main:
	shl           r11, 0x02
	sub           r11, 0x40					// fix overrun
	mov           r10, 0x01
	vpbroadcastd  zmm8, r10d
	mov           r10, 0x00
	vmovdqu32     zmm0, zmmword ptr [rdi+r10*1]

mainloop:
	vmovdqu32     zmm4, zmmword ptr [rdi+r10*1+0x40]
	valignd       zmm1, zmm4, zmm0, 0x01
	vpsubd        zmm1, zmm1, zmm8
	call          detect_separator
	vmovdqu32     zmmword ptr [r8+r10*1], zmm2
	vmovdqa64     zmm0, zmm4
	add           r10, 0x40
	cmp           r10, r11
	jl            mainloop
	mov           qword ptr [rsp+0x60], r10
	vzeroupper"
             :
             : "{rsi}"(message_ptr),
               "{rdi}"(indices_ptr),
               "{r11}"(indices_len),
               "{r8}"(separator_indices_ptr),
               "{rbx}"(num_separator),
               "{rcx}"(separator_char)
             :: "intel" );
    }
}
