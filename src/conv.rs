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
mod conv_tests {
    use super::*;
    use std::convert::TryInto;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_conv_atoi64() {
        let numbers = String::from(
            r#"
1,22,333,4444,55555,666666,7777777,88888888,999999999,1111111111,22222222222,333333333333,
2,3,4,5,6,7,8,9,10,100,1000,10000,100000,1000000,10000000,100000000,0,00,000,0000,00000,
01,02,03,04,05,06,07,08,09,010,0100,01000,0009,00099,000999,0009999,00099999,000999999,
0009999999,00099999999,000999999999,0009999999999,
9876543210,876543210,76543210,6543210,543210,43210,3210,210,10
"#,
        );

        let position_vec: Vec<Vec<u64>> = vec![
            vec![1, 3, 6, 10, 15, 21, 28, 36],
            vec![45, 55, 66, 78, 92, 94, 96, 98],
            vec![100, 102, 104, 106, 108, 111, 115, 120],
            vec![126, 133, 141, 150, 160, 162, 165, 169],
            vec![174, 181, 184, 187, 190, 193, 196, 199],
            vec![202, 205, 208, 212, 217, 223, 228, 234],
            vec![241, 249, 258, 269, 280, 292, 305, 320],
            vec![331, 341, 350, 358, 365, 371, 376, 380],
        ];

        static EXPECTED: &'static [i64] = &[
            1,
            22,
            333,
            4444,
            55555,
            666666,
            7777777,
            88888888,
            999999999,
            1111111111,
            22222222222,
            333333333333,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            0,
            0,
            0,
            0,
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            100,
            1000,
            9,
            99,
            999,
            9999,
            99999,
            999999,
            9999999,
            99999999,
            999999999,
            9999999999,
            9876543210,
            876543210,
            76543210,
            6543210,
            543210,
            43210,
            3210,
            210,
            10,
        ];

        for (i, v) in position_vec.iter().enumerate() {
            let i64s_vec: Vec<i64> = vec![11223344; 8];
            conv_atoi64(numbers.as_ptr(), v.as_ptr(), i64s_vec.as_ptr());

            assert_eq!(i64s_vec, &EXPECTED[i * 8..(i + 1) * 8]);
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn conv_atoi64(message_ptr: *const u8, indices_ptr: *const u64, i64_ptr: *const i64) {
    unsafe {
        asm!("
    jmp main_atoi64

conv_atoi64:
	mov           rcx, 0x30
	vpbroadcastb  zmm30, ecx
	mov           rcx, 0x0A
	vpbroadcastb  zmm29, ecx
	vpbroadcastq  zmm24, rcx
	mov           rcx, 0xFF
	kmovq         k7, rcx
	vpbroadcastq  zmm26, rcx
	vpxorq        zmm1, zmm1, zmm1
	mov           rax, rsi

loop:
	knotq         k6, k0
	vpgatherqq    zmm31 {k6}, [rax+zmm0*1]
	add           rax, 0x08
	vpsubb        zmm31, zmm31, zmm30
	vpcmpub       k6, zmm31, zmm29, 0x01
	vpmovm2b      zmm28, k6
	vpsllq        zmm27, zmm28, 0x38
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x30
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x28
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x20
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	kmovq         rcx, k7
	cmp           rcx, 0x00
	jz            done
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x18
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x10
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x08
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	vpsrlq        zmm31, zmm31, 0x08
	vpsllq        zmm27, zmm28, 0x00
	vpmovq2m      k6, zmm27
	kandq         k7, k6, k7
	vpmullq       zmm1 {k7}, zmm1, zmm24
	vpandq        zmm25 {k7}, zmm26, zmm31
	vpaddq        zmm1 {k7}, zmm1, zmm25
	kmovq         rcx, k7
	cmp           rcx, 0x00
	jnz           loop
done:
	ret

main_atoi64:
	vmovdqu32     zmm0, zmmword ptr [rdi]
	call          conv_atoi64
	vmovdqu32     zmmword ptr [r11], zmm1
	vzeroupper"
             :
             : "{rsi}"(message_ptr),
               "{rdi}"(indices_ptr),
               "{r11}"(i64_ptr)
             :: "intel" );
    }
}
