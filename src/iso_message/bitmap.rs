use std::mem::size_of_val;

#[derive(Default)]
pub struct Bitmap {
    primary_bitmap: u64,
    secondary_bitmap: Option<u64>,
    tertiary_bitmap: Option<u64>,
}

impl TryFrom<&[u8]> for Bitmap {
    type Error = std::io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; 8];

        let primary_bitmap_bytes = value.get(0..8).ok_or(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer not long enough to extract primary bitmap",
        ))?;

        buffer.copy_from_slice(primary_bitmap_bytes);

        let primary_bitmap = u64::from_be_bytes(buffer);

        let secondary_bitmap = if primary_bitmap & 1 << 63 != 0 {
            let secondary_bitmap_bytes = value.get(8..16).ok_or(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Buffer not long enough to extract secondary bitmap",
            ))?;
            buffer.copy_from_slice(secondary_bitmap_bytes);
            Some(u64::from_be_bytes(buffer))
        } else {
            None
        };

        let tertiary_bitmap = if let Some(secondary_bitmap) = secondary_bitmap {
            if secondary_bitmap & 1 << 63 != 0 {
                let tertiary_bitmap_bytes = value.get(16..24).ok_or(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "Buffer not long enough to extract tertiary bitmap",
                ))?;

                buffer.copy_from_slice(tertiary_bitmap_bytes);
                Some(u64::from_be_bytes(buffer))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            primary_bitmap,
            secondary_bitmap,
            tertiary_bitmap,
        })
    }
}

impl Bitmap {
    pub fn bytes_consumed(&self) -> usize {
        size_of_val(&self.primary_bitmap) * 8
            + self
                .secondary_bitmap
                .map(|secondary_bitmap| size_of_val(&secondary_bitmap) * 8)
                .unwrap_or(0)
            + self
                .tertiary_bitmap
                .map(|tertiary_bitmap| size_of_val(&tertiary_bitmap) * 8)
                .unwrap_or(0)
    }
}

impl IntoIterator for Bitmap {
    type Item = usize;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let Bitmap {
            primary_bitmap,
            secondary_bitmap,
            tertiary_bitmap,
        } = self;
        let mut data_elements = Vec::new();

        let mut add_bits_to_data_elements = |bitmap: u64, bitmap_num: usize| {
            for (index, byte) in bitmap.to_be_bytes().into_iter().enumerate() {
                for bit_index in 0..8 {
                    if byte & (0b10000000 >> bit_index) != 0 {
                        data_elements.push((64 * bitmap_num) + (8 * index) + bit_index + 1)
                    }
                }
            }
        };

        add_bits_to_data_elements(primary_bitmap, 0);

        if let Some(secondary_bitmap) = secondary_bitmap {
            add_bits_to_data_elements(secondary_bitmap, 1);

            if let Some(tertiary_bitmap) = tertiary_bitmap {
                add_bits_to_data_elements(tertiary_bitmap, 2);
            }
        }

        data_elements.into_iter()
    }
}

#[cfg(test)]
mod unit_test {
    mod bytes_consumed {
        use crate::iso_message::bitmap::Bitmap;

        #[test]
        fn should_return_64_if_only_primary_bitmap_set() {
            let bitmaps = Bitmap {
                primary_bitmap: 1,
                ..Default::default()
            };

            assert_eq!(bitmaps.bytes_consumed(), 64);
        }

        #[test]
        fn should_return_128_if_primary_and_secondary_bitmap_set() {
            let bitmaps = Bitmap {
                primary_bitmap: 1,
                secondary_bitmap: Some(1),
                ..Default::default()
            };

            assert_eq!(bitmaps.bytes_consumed(), 128);
        }

        #[test]
        fn should_return_192_if_all_bitmaps_set() {
            let bitmaps = Bitmap {
                primary_bitmap: 1,
                secondary_bitmap: Some(1),
                tertiary_bitmap: Some(1),
            };

            assert_eq!(bitmaps.bytes_consumed(), 192);
        }
    }

    mod try_from_buffer {
        use crate::iso_message::bitmap::Bitmap;

        #[test]
        fn should_parse_primary_bitmap_out_of_buffer() {
            let input = &[0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8];
            let bitmap = Bitmap::try_from(&input[..]);

            assert!(&bitmap.is_ok());
            let Bitmap {
                primary_bitmap,
                secondary_bitmap,
                tertiary_bitmap,
            } = bitmap.unwrap();
            assert_eq!(primary_bitmap, 1u64);
            assert_eq!(secondary_bitmap, None);
            assert_eq!(tertiary_bitmap, None);
        }

        #[test]
        fn should_parse_primary_and_secondary_bitmap_out_of_buffer() {
            let input = &[
                0b10000000, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                1u8,
            ];
            let bitmap = Bitmap::try_from(&input[..]);

            assert!(&bitmap.is_ok());
            let Bitmap {
                primary_bitmap,
                secondary_bitmap,
                tertiary_bitmap,
            } = bitmap.unwrap();

            assert_eq!(
                primary_bitmap,
                0b1000000000000000000000000000000000000000000000000000000000000001
            );
            assert_eq!(
                secondary_bitmap,
                Some(0b0000000000000000000000000000000000000000000000000000000000000001)
            );
            assert_eq!(tertiary_bitmap, None);
        }

        #[test]
        fn should_parse_primary_secondary_and_tertiary_bitmap_out_of_buffer() {
            let input = &[
                0b10000000, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0b10000000, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
            ];
            let bitmap = Bitmap::try_from(&input[..]);

            println!("{:#10b}", u8::from_be(0b100));
            assert!(&bitmap.is_ok());
            let Bitmap {
                primary_bitmap,
                secondary_bitmap,
                tertiary_bitmap,
            } = bitmap.unwrap();

            assert_eq!(
                primary_bitmap,
                0b1000000000000000000000000000000000000000000000000000000000000001
            );
            assert_eq!(
                secondary_bitmap,
                Some(0b1000000000000000000000000000000000000000000000000000000000000001)
            );
            assert_eq!(
                tertiary_bitmap,
                Some(0b0000000000000000000000000000000000000000000000000000000000000001)
            );
        }
    }

    mod into_iter {
        use crate::iso_message::bitmap::Bitmap;

        #[test]
        fn returns_iterator_of_which_bits_are_set_to_two_bitmap() {
            let primary_bitmap = u64::from_str_radix("9222208100001081", 16).unwrap();
            let secondary_bitmap = u64::from_str_radix("8411010000040040", 16).unwrap();
            let bitmap = Bitmap {
                primary_bitmap,
                secondary_bitmap: Some(secondary_bitmap),
                tertiary_bitmap: None,
            };

            let fields_outlined_by_bitmap: Vec<usize> = bitmap.into_iter().collect();

            assert_eq!(
                fields_outlined_by_bitmap,
                vec![1, 4, 7, 11, 15, 19, 25, 32, 52, 57, 64, 65, 70, 76, 80, 88, 110, 122]
            )
        }
        #[test]
        fn returns_iterator_of_which_bits_are_set_to_one() {
            let primary_bitmap = u64::from_str_radix("7010001102C04804", 16).unwrap();
            let bitmap = Bitmap {
                primary_bitmap,
                secondary_bitmap: None,
                tertiary_bitmap: None,
            };

            let fields_outlined_by_bitmap: Vec<usize> = bitmap.into_iter().collect();

            assert_eq!(
                fields_outlined_by_bitmap,
                vec![2, 3, 4, 12, 28, 32, 39, 41, 42, 50, 53, 62]
            )
        }
    }
}
