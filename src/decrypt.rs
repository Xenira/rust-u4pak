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

use aes::cipher::{BlockDecrypt, NewBlockCipher};
use aes::{Aes256, Block};

pub const BLOCK_SIZE: usize = 16;

pub fn decrypt(data: &mut Vec<u8>, key: Vec<u8>) {
    let cipher = Aes256::new_from_slice(&key).expect("Unable to convert key to Aes256 cipher");
    assert_eq!(data.len() % 16, 0, "Data length must be a multiple of 16");

    for block in data.chunks_mut(BLOCK_SIZE) {
        cipher.decrypt_block(Block::from_mut_slice(block));
    }
}

