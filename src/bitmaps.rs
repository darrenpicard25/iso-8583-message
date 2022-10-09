use crate::IsoMessageMap;

pub struct Bitmaps {
    maps: Vec<u64>,
}

impl Bitmaps {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let mut bitmaps = Vec::new();

        let primary_bitmap = u64::from_be_bytes(buffer[0..8].try_into().unwrap());
        bitmaps.push(primary_bitmap);

        if Self::has_next_bitmap(primary_bitmap) {
            let secondary_bitmap = u64::from_be_bytes(buffer[8..16].try_into().unwrap());
            bitmaps.push(secondary_bitmap);

            if Self::has_next_bitmap(secondary_bitmap) {
                let tertiary_bitmap = u64::from_be_bytes(buffer[16..24].try_into().unwrap());
                bitmaps.push(tertiary_bitmap);
            }
        }

        Bitmaps { maps: bitmaps }
    }

    pub fn from_iso_message_map(map: &IsoMessageMap) -> Self {
        const STARTING_BIT: u64 = 1u64.reverse_bits();

        let mut bitmaps = map
            .keys()
            .fold(Vec::from([0_u64, 0_u64, 0_u64]), |mut acc, key| {
                if key == &0 {
                    return acc;
                }

                let index = (key - 1) / 64;
                let position = (key - 1) % 64;

                if let Some(bitmap) = acc.get_mut(index as usize) {
                    *bitmap = *bitmap | (STARTING_BIT >> position);
                };

                acc
            });

        {
            let mut bitmap_iterator = bitmaps.iter_mut().peekable();

            while let Some(bitmap) = bitmap_iterator.next() {
                if let Some(next_bitmap) = bitmap_iterator.peek() {
                    if next_bitmap != &&0 {
                        *bitmap = *bitmap | STARTING_BIT;
                    }
                }
            }
        }

        Self {
            maps: bitmaps.into_iter().filter(|bitmap| bitmap != &0).collect(),
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        self.maps
            .iter()
            .flat_map(|bitmap| bitmap.to_be_bytes().to_vec())
            .collect()
    }

    fn has_next_bitmap(bitmap: u64) -> bool {
        bitmap & (1 << 63) != 0
    }

    pub fn byte_length(&self) -> usize {
        self.maps.len() * 8
    }

    pub fn items(&self) -> Vec<u8> {
        const STARTING_BIT: u64 = 1u64.reverse_bits();
        self.maps
            .iter()
            .enumerate()
            .flat_map(|(index, map)| {
                (0..64u8).filter_map(move |position| {
                    // Dont want to return bitmap positions
                    if position == 0 || position == 64 || position == 128 {
                        return None;
                    }

                    if (STARTING_BIT >> position & map) != 0 {
                        return Some(1 + position + 64u8 * index as u8);
                    }

                    None
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Bitmaps;

    mod items {
        use super::Bitmaps;

        #[test]
        fn should_return_array_with_elements_in_1_bitmap() {
            let bitmaps = Bitmaps {
                maps: [4611686018427387905].to_vec(),
            };

            assert_eq!(bitmaps.items(), [2, 64].to_vec())
        }

        #[test]
        fn should_return_array_with_elements_in_2_bitmap() {
            let bitmaps = Bitmaps {
                maps: [13835058055282163713, 4611686018427387905].to_vec(),
            };

            assert_eq!(bitmaps.items(), [1, 2, 64, 66, 128].to_vec())
        }

        #[test]
        fn should_return_array_with_elements_in_3_bitmap() {
            let bitmaps = Bitmaps {
                maps: [
                    13835058055282163713,
                    13835058055282163713,
                    4611686018427387905,
                ]
                .to_vec(),
            };

            assert_eq!(bitmaps.items(), [1, 2, 64, 65, 66, 128, 130, 192].to_vec())
        }
    }

    mod from_buffer {
        use super::Bitmaps;

        #[test]
        fn should_grab_only_1_bit_map() {
            let buffer: Vec<u8> = [2u64.reverse_bits(), u64::MAX >> 1, u64::MAX >> 1]
                .iter()
                .flat_map(|bitmap| bitmap.to_be_bytes().to_vec())
                .collect();

            let bitmaps = Bitmaps::from_buffer(&buffer);

            assert_eq!(bitmaps.maps.len(), 1);
            assert_eq!(bitmaps.byte_length(), 8);
        }

        #[test]
        fn should_grab_only_2_bit_map() {
            let buffer: Vec<u8> = [3u64.reverse_bits(), u64::MAX >> 1, u64::MAX >> 1]
                .iter()
                .flat_map(|bitmap| bitmap.to_be_bytes().to_vec())
                .collect();

            let bitmaps = Bitmaps::from_buffer(&buffer);

            assert_eq!(bitmaps.maps.len(), 2);
            assert_eq!(bitmaps.byte_length(), 16);
        }

        #[test]
        fn should_grab_3_bit_map() {
            let buffer: Vec<u8> = [3u64.reverse_bits(), u64::MAX, u64::MAX >> 1]
                .iter()
                .flat_map(|bitmap| bitmap.to_be_bytes().to_vec())
                .collect();

            let bitmaps = Bitmaps::from_buffer(&buffer);

            assert_eq!(bitmaps.maps.len(), 3);
            assert_eq!(bitmaps.byte_length(), 24);
        }
    }

    mod from_iso_message_map {
        use crate::IsoMessageMap;

        use super::Bitmaps;

        #[test]
        fn should_construct_1_bitmap_from_map() {
            let map = IsoMessageMap::from([(2, "".to_owned()), (64, "".to_owned())]);

            let bitmaps = Bitmaps::from_iso_message_map(&map);

            assert_eq!(bitmaps.maps, [4611686018427387905])
        }

        #[test]
        fn should_construct_2_bitmap_from_map() {
            let map =
                IsoMessageMap::from([(2, "".to_owned()), (64, "".to_owned()), (66, "".to_owned())]);

            let bitmaps = Bitmaps::from_iso_message_map(&map);

            assert_eq!(bitmaps.maps, [13835058055282163713, 4611686018427387904])
        }

        #[test]
        fn should_construct_2_bitmap_from_map_at_both_ends() {
            let map = IsoMessageMap::from([
                (2, "".to_owned()),
                (64, "".to_owned()),
                (66, "".to_owned()),
                (128, "".to_owned()),
            ]);

            let bitmaps = Bitmaps::from_iso_message_map(&map);

            assert_eq!(bitmaps.maps, [13835058055282163713, 4611686018427387905])
        }

        #[test]
        fn should_construct_3_bitmap_from_map() {
            let map = IsoMessageMap::from([
                (2, "".to_owned()),
                (64, "".to_owned()),
                (66, "".to_owned()),
                (128, "".to_owned()),
                (130, "".to_owned()),
            ]);

            let bitmaps = Bitmaps::from_iso_message_map(&map);

            assert_eq!(
                bitmaps.maps,
                [
                    13835058055282163713,
                    13835058055282163713,
                    4611686018427387904
                ]
            )
        }

        #[test]
        fn should_construct_2_bitmap_completely_full() {
            let mut map: IsoMessageMap = (2..=128).map(|key| (key, "".to_owned())).collect();
            map.remove(&65);

            let bitmaps = Bitmaps::from_iso_message_map(&map);

            assert_eq!(bitmaps.maps, [u64::MAX, u64::MAX >> 1])
        }
    }

    mod to_buffer {
        use super::Bitmaps;

        #[test]
        fn should_return_buffer_of_3_bitmap() {
            let bitmaps = Bitmaps {
                maps: vec![256u64, 8u64, 1u64],
            };

            assert_eq!(
                bitmaps.to_buffer(),
                vec![
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8,
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8
                ]
            );
        }

        #[test]
        fn should_return_buffer_of_2_bitmap() {
            let bitmaps = Bitmaps {
                maps: vec![2u64, 8u64],
            };

            assert_eq!(
                bitmaps.to_buffer(),
                vec![
                    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 8u8
                ]
            );
        }

        #[test]
        fn should_return_buffer_of_1_bitmap() {
            let bitmaps = Bitmaps { maps: vec![1u64] };

            assert_eq!(
                bitmaps.to_buffer(),
                vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8]
            );
        }
    }

    mod byte_length {
        use super::Bitmaps;

        #[test]
        fn should_return_8_bytes_if_only_contains_1_bitmap() {
            let bitmaps = Bitmaps { maps: vec![1u64] };
            assert_eq!(bitmaps.byte_length(), 8);
        }

        #[test]
        fn should_return_16_bytes_if_contains_2_bitmap() {
            let bitmaps = Bitmaps {
                maps: vec![1u64, 1u64],
            };
            assert_eq!(bitmaps.byte_length(), 16);
        }

        #[test]
        fn should_return_24_bytes_if_contains_3_bitmap() {
            let bitmaps = Bitmaps {
                maps: vec![1u64, 1u64, 1u64],
            };
            assert_eq!(bitmaps.byte_length(), 24);
        }
    }
}
