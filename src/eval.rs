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
mod eval_tests {
    use super::*;
    use std::convert::TryInto;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_eval_string_equal() {
        let mut message = Vec::new();
        let mut f = File::open("tests/parking-citations-10K.csv").unwrap();
        f.read_to_end(&mut message).unwrap(); // read the whole file

        let indices_vec: Vec<u32> = vec![
            94, 256, 382, 504, 618, 731, 859, 988, 1115, 1241, 1363, 1486, 1609, 1732, 1854, 1974,
            2094, 2211, 2330, 2447, 2573, 2686, 2786, 2916, 3042, 3166, 3285, 3401, 3520, 3630,
            3753, 3871, 3990, 4117, 4245, 4368, 4499, 4627, 4759, 4878, 5002, 5125, 5248, 5367,
            5497, 5619, 5741, 5867, 5989, 6118, 6243, 6369, 6487, 6612, 6736, 6860, 6985, 7105,
            7234, 7351, 7476, 7605, 7735, 7864, 7991, 8112, 8221, 8341, 8450, 8567, 8687, 8817,
            8948, 9079, 9203, 9328, 9457, 9574, 9700, 9818, 9948, 10085, 10205, 10332, 10455,
            10588, 10721, 10849, 10986, 11122, 11259, 11391, 11512, 11639, 11776, 11894, 12022,
            12146, 12265, 12390, 12515, 12630, 12749, 12872, 12994, 13114, 13235, 13362, 13474,
            13598, 13729, 13860, 13988, 14116, 14244, 14384, 14512, 14640, 14766, 14894, 15022,
            15141, 15272, 15403, 15525, 15646, 15773, 15893,
        ];

        let active_vec: Vec<u32> = vec![0; 16];

        const HONDA: u32 = 0x444e4f48;
        static HONDA_EXPECTED: &'static [u32] = &[
            256, 2573, 2916, 4245, 5367, 7991, 9574, 10455, 10588, 14384, 15403, 15646, 0, 0, 0, 0,
        ];

        let entries = eval_string_equal(
            message.as_ptr(),
            indices_vec.as_ptr(),
            indices_vec.len().try_into().unwrap(),
            HONDA,
            active_vec.as_ptr(),
            active_vec.len().try_into().unwrap(),
        );
        assert_eq!(entries, 12);
        assert_eq!(active_vec, HONDA_EXPECTED);

        const NISSAN: u32 = 0x5353494e;
        static NISSAN_EXPECTED: &'static [u32] = &[
            504, 1363, 2211, 2330, 2786, 5248, 5989, 6369, 7735, 9457, 10986, 14766, 15773, 0, 0, 0,
        ];

        let entries = eval_string_equal(
            message.as_ptr(),
            indices_vec.as_ptr(),
            indices_vec.len().try_into().unwrap(),
            NISSAN,
            active_vec.as_ptr(),
            active_vec.len().try_into().unwrap(),
        );
        assert_eq!(entries, 13);
        assert_eq!(active_vec, NISSAN_EXPECTED);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn eval_string_equal(
    message_ptr: *const u8,
    indices_ptr: *const u32,
    indices_len: u64,
    pattern: u32,
    active_ptr: *const u32,
    active_len: u64,
) -> u64 {
    let entries: u64;
    unsafe {
        asm!("
    jmp eval_main

eval_string_equal:
	knotq         k4, k0
	knotq         k5, k0
	knotq         k6, k0
	knotq         k7, k0
	vpgatherdd    zmm28 {k4}, [rsi+zmm4*1]
	vpgatherdd    zmm29 {k5}, [rsi+zmm5*1]
	vpgatherdd    zmm30 {k6}, [rsi+zmm6*1]
	vpgatherdd    zmm31 {k7}, [rsi+zmm7*1]
	vpcmpeqd      k4, zmm28, zmm2
	vpcmpeqd      k5, zmm29, zmm2
	vpcmpeqd      k6, zmm30, zmm2
	vpcmpeqd      k7, zmm31, zmm2
	kshiftlq      k5, k5, 0x10
	kshiftlq      k6, k6, 0x20
	kshiftlq      k7, k7, 0x30
	korq          k4, k4, k5
	korq          k4, k4, k6
	korq          k1, k4, k7
	ret

eval_main:
    shl           r9, 0x02
    add           r9, rdi
	xor           r13, r13

eval_main_loop:
	vmovdqu32     zmm4, zmmword ptr [rdi]
	vmovdqu32     zmm5, zmmword ptr [rdi+0x40]
	vmovdqu32     zmm6, zmmword ptr [rdi+0x80]
	vmovdqu32     zmm7, zmmword ptr [rdi+0xc0]
	vpbroadcastd  zmm2, ecx
    call          eval_string_equal

    mov           rbx, r13
	vpcompressd   zmmword ptr [r14+rbx*4] {k1}, zmm4
	kmovq         rax, k1
	and           rax, 0xFFFF
	popcnt        rax, rax
	add           rbx, rax
	kshiftrq      k2, k1, 0x10
	vpcompressd   zmmword ptr [r14+rbx*4] {k2}, zmm5
	kmovq         rax, k2
	and           rax, 0xFFFF
	popcnt        rax, rax
	add           rbx, rax
	kshiftrq      k2, k1, 0x20
	vpcompressd   zmmword ptr [r14+rbx*4] {k2}, zmm6
	kmovq         rax, k2
	and           rax, 0xFFFF
	popcnt        rax, rax
	add           rbx, rax
	kshiftrq      k2, k1, 0x30
	vpcompressd   zmmword ptr [r14+rbx*4] {k2}, zmm7
	kmovq         rax, k2
	and           rax, 0xFFFF
	popcnt        rax, rax
	add           rbx, rax

    mov           r13, rbx
	cmp           r13, r15
	jnl           eval_done
    add           rdi, 0x100
	cmp           rdi, r9
	jl            eval_main_loop

eval_done:
	vzeroupper"
             : "={r13}"(entries)
             : "{rsi}"(message_ptr),
               "{rdi}"(indices_ptr),
               "{r9}"(indices_len),
               "{rcx}"(pattern),
               "{r14}"(active_ptr),
               "{r15}"(active_len)
             :: "intel" );
    }
    return entries;
}
