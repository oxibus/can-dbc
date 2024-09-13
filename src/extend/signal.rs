use crate::{ByteOrder, Signal, ValueType};

impl Signal {
    /// Decodes/Extracts a signal from the message data
    ///
    /// # Params
    ///
    ///   * data: source data
    ///
    /// # Return
    ///
    ///   Raw signal value
    pub fn decode(&self, data: &[u8]) -> u64 {
        if self.signal_size == 0 {
            return 0;
        }

        let mut result = 0;
        match self.byte_order {
            ByteOrder::LittleEndian => {
                let mut src_bit = self.start_bit as usize;
                let mut dst_bit = 0;
                for _ in 0..self.signal_size {
                    /* copy bit */
                    if (data[src_bit / 8] & (1 << (src_bit % 8))) > 0 {
                        result |= 1 << dst_bit;
                    }

                    /* calculate next position */
                    src_bit += 1;
                    dst_bit += 1;
                }
            },
            ByteOrder::BigEndian => {
                let mut src_bit = self.start_bit as usize;
                let mut dst_bit = self.signal_size - 1;
                for _ in 0..self.signal_size {
                    /* copy bit */
                    if (data[src_bit / 8] & (1 << (src_bit % 8))) > 0 {
                        result |= 1 << dst_bit;
                    }

                    /* calculate next position */
                    if (src_bit % 8) == 0 {
                        src_bit += 15;
                    }
                    else {
                        src_bit -= 1;
                    }
                    dst_bit -= 1;
                }
            },
        }

        match self.value_type() {
            ValueType::Signed => {
                if (result & (1 << (self.signal_size - 1))) > 0 {
                    for i in self.signal_size..64 {
                        result |= 1 << i;
                    }
                }
            },
            ValueType::Unsigned => {}
        }

        result
    }

    /// Encodes a signal into the message data
    ///
    /// # Params
    ///
    ///   * data: source data
    ///
    ///   * value: Raw signal value
    pub fn encode(&self, data: &mut Vec<u8>, value: u64) {
        if self.signal_size == 0 {
            return;
        }

        match self.byte_order {
            ByteOrder::LittleEndian => {
                let mut src_bit = self.start_bit as usize;
                let mut dst_bit = 0;
                for _ in 0..self.signal_size {
                    /* copy bit */
                    if (value & 1 << dst_bit) > 0 {
                        data[src_bit / 8] |= 1 << (src_bit % 8);
                    }
                    else {
                        data[src_bit / 8] &= !(1 << (src_bit % 8));
                    }

                    /* calculate next position */
                    src_bit += 1;
                    dst_bit += 1;
                }
            },
            ByteOrder::BigEndian => {
                let mut src_bit = self.start_bit as usize;
                let mut dst_bit = self.signal_size - 1;
                for _ in 0..self.signal_size {
                    /* copy bit */
                    if (value & 1 << dst_bit) > 0 {
                        data[src_bit / 8] |= 1 << (dst_bit % 8);
                    }
                    else {
                        data[src_bit / 8] &= !(1 << (src_bit % 8));
                    }

                    /* calculate next position */
                    if (src_bit % 8) == 0 {
                        src_bit += 15;
                    }
                    else {
                        src_bit -= 1;
                    }
                    dst_bit -= 1;
                }
            },
        }
    }
}

