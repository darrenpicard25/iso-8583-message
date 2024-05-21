use std::{collections::VecDeque, mem::size_of_val};

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

        let secondary_bitmap = if primary_bitmap & 0b100 != 0 {
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
            if secondary_bitmap & 0b100 != 0 {
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

pub struct BitmapIntoIterator(VecDeque<usize>);

impl IntoIterator for Bitmap {
    type Item = usize;

    type IntoIter = BitmapIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let Bitmap {
            primary_bitmap,
            secondary_bitmap,
            tertiary_bitmap,
        } = self;
        let mut data_elements = VecDeque::new();

        let mut add_bits_to_data_elements = |bitmap: u64, bitmap_num: usize| {
            for (index, byte) in bitmap.to_be_bytes().into_iter().enumerate() {
                for bit_index in 0..8 {
                    if byte & (1 << bit_index) != 0 {
                        data_elements.push_back((64 * bitmap_num) + (8 * index) + bit_index)
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

        BitmapIntoIterator(data_elements)
    }
}

impl Iterator for BitmapIntoIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
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
}
