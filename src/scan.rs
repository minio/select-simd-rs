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
mod scan_tests {
    use super::*;
    use std::convert::TryInto;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_scan_delimiter() {
        let mut message = Vec::new();
        let mut f = File::open("tests/parking-citations-10K.csv").unwrap();
        f.read_to_end(&mut message).unwrap(); // read the whole file

        let indices_vec: Vec<u32> = vec![0; 64];

        let result = scan_delimiter(
            message.as_ptr(),
            message.len().try_into().unwrap(),
            indices_vec.as_ptr(),
            indices_vec.len().try_into().unwrap(),
            0,
            0x0a,
        );
        assert_eq!(result.0, 7943);

        static EXPECTED: &'static [u32] = &[
            0, 207, 333, 455, 575, 683, 812, 939, 1066, 1193, 1314, 1437, 1560, 1683, 1806, 1925,
            2045, 2162, 2281, 2404, 2524, 2637, 2737, 2867, 2993, 3118, 3237, 3353, 3472, 3582,
            3705, 3823, 3942, 4069, 4196, 4319, 4450, 4578, 4706, 4831, 4947, 5072, 5194, 5318,
            5454, 5570, 5692, 5818, 5941, 6070, 6195, 6320, 6438, 6564, 6688, 6812, 6936, 7056,
            7185, 7302, 7428, 7556, 7686, 7815,
        ];

        assert_eq!(indices_vec, EXPECTED);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn scan_delimiter(
    message_ptr: *const u8,
    message_len: u64,
    indices_ptr: *const u32,
    indices_len: u64,
    start_pos: u64,
    delimiter: u64,
) -> (u64, u64) {
    let end_pos: u64;
    let entries: u64;

    unsafe {
        asm!("
    jmp scan_main${:uid}

scan_delimiters${:uid}:
	kmovq         k7, rax
	kmovq         k6, r10
	vpbroadcastb  zmm0, eax
	mov           rbx, rcx
	xor           rdx, rdx
	vpbroadcastd  zmm28, ebx
	and           rcx, 0x3F
	cmp           rcx, 0x00
	jnz           scan_initialAlignedLoad${:uid}

scan_loop${:uid}:
	vpcmpeqb      k4, zmm0, zmmword ptr [rsi+rbx*1]

scan_afterAlignedLoad${:uid}:
	kmovq         rax, k4
	cmp           rax, 0x00
	jz            scan_skipCtz${:uid}

scan_loopCtz${:uid}:
	tzcnt         r10, rax
	add           rdx, 0x01
	add           r10, rbx
	blsr          rax, rax
	vpbroadcastd  zmm29, r10d
	valignd       zmm30, zmm29, zmm30, 0x01
	jnz           scan_loopCtz${:uid}

scan_skipCtz${:uid}:
	add           rbx, 0x40
	cmp           rbx, r9
	jnl           scan_done${:uid}
	cmp           rdx, 0x10
	jl            scan_loop${:uid}

scan_done${:uid}:
	vpextrd       ecx, xmm29, 0x00
	add           rcx, 0x01
	cmp           rdx, 0x10
	jl            scan_shiftOutput${:uid}

scan_afterShiftOutput${:uid}:
	mov           rax, 0x01
	vpbroadcastd  zmm31, eax
	vpaddd        zmm30, zmm30, zmm31
	valignd       zmm0, zmm30, zmm28, 0x0F
	kmovq         r10, k6
	kmovq         rax, k7
	ret

scan_initialAlignedLoad${:uid}:
	mov           rax, 0x01
	shl           rax, cl
	kmovq         k4, rax
	sub           rbx, rcx
	vpcmpeqb      k4 {k4}, zmm0, zmmword ptr [rsi+rbx*1]
	jmp           scan_afterAlignedLoad${:uid}

scan_shiftOutput${:uid}:
	mov           rax, rdx

scan_shiftOutputLoop${:uid}:
	valignd       zmm30, zmm29, zmm30, 0x01
	inc           rax
	cmp           rax, 0x10
	jl            scan_shiftOutputLoop${:uid}
	jmp           scan_afterShiftOutput${:uid}

scan_main${:uid}:
    shl           r10, 0x02
	xor           r12, r12
	xor           r13, r13

scan_main_loop${:uid}:
	call		  scan_delimiters${:uid}
	vmovdqu32     zmmword ptr [r11+r12*1], zmm0
	add           r13, rdx
	cmp           rdx, 0x10
	jnz           scan_main_done${:uid}
	mov           r8, r9
	sub           r8, 0x40
	cmp           rbx, r8
	jnl           scan_main_done${:uid}
	add           r12, 0x40
	cmp           r12, r10
	jl            scan_main_loop${:uid}

scan_main_done${:uid}:
	mov           rax, rcx
	shr           r12, 0x02
	vzeroupper"
             : "={rax}"(end_pos),
               "={r12}"(entries)
             : "{rsi}"(message_ptr),
               "{r9}"(message_len),
               "{r11}"(indices_ptr),
               "{r10}"(indices_len),
               "{rcx}"(start_pos),
               "{rax}"(delimiter)
             : "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
             : "intel" );
    }
    (end_pos, entries)
}
