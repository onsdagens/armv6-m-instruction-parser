/// Normal register type.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    ZERO = 0,
    RA = 1,
    SP = 2,
    GP = 3,
    TP = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    S0 = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

impl TryFrom<u8> for Register {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::ZERO),
            1 => Ok(Register::RA),
            2 => Ok(Register::SP),
            3 => Ok(Register::GP),
            4 => Ok(Register::TP),
            5 => Ok(Register::T0),
            6 => Ok(Register::T1),
            7 => Ok(Register::T2),
            8 => Ok(Register::S0),
            9 => Ok(Register::S1),
            10 => Ok(Register::A0),
            11 => Ok(Register::A1),
            12 => Ok(Register::A2),
            13 => Ok(Register::A3),
            14 => Ok(Register::A4),
            15 => Ok(Register::A5),
            16 => Ok(Register::A6),
            17 => Ok(Register::A7),
            18 => Ok(Register::S2),
            19 => Ok(Register::S3),
            20 => Ok(Register::S4),
            21 => Ok(Register::S5),
            22 => Ok(Register::S6),
            23 => Ok(Register::S7),
            24 => Ok(Register::S8),
            25 => Ok(Register::S9),
            26 => Ok(Register::S10),
            27 => Ok(Register::S11),
            28 => Ok(Register::T3),
            29 => Ok(Register::T4),
            30 => Ok(Register::T5),
            31 => Ok(Register::T6),
            _ => Err("Invalid register"),
        }
    }
}

/// Creates a register list from a bit array.
pub fn register_list_from_bit_array(bit_array: u32) -> Vec<Register> {
    let mut ret = vec![];
    for i in 0u8..32u8 {
        if (bit_array >> i) & 0b1 == 0b1 {
            ret.push(i.try_into().unwrap())
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_to_register() {
        assert_eq!(0.try_into(), Ok(Register::ZERO));
        assert_eq!(1.try_into(), Ok(Register::RA));
        assert_eq!(2.try_into(), Ok(Register::SP));
        assert_eq!(3.try_into(), Ok(Register::GP));
        assert_eq!(4.try_into(), Ok(Register::TP));
        assert_eq!(5.try_into(), Ok(Register::T0));
        assert_eq!(6.try_into(), Ok(Register::T1));
        assert_eq!(7.try_into(), Ok(Register::T2));
        assert_eq!(8.try_into(), Ok(Register::S0));
        assert_eq!(9.try_into(), Ok(Register::S1));
        assert_eq!(10.try_into(), Ok(Register::A0));
        assert_eq!(11.try_into(), Ok(Register::A1));
        assert_eq!(12.try_into(), Ok(Register::A2));
        assert_eq!(13.try_into(), Ok(Register::A3));
        assert_eq!(14.try_into(), Ok(Register::A4));
        assert_eq!(15.try_into(), Ok(Register::A5));
        assert_eq!(16.try_into(), Ok(Register::A6));
    }

    #[test]
    fn register_list() {
        assert_eq!(register_list_from_bit_array(0), vec![]);
        assert_eq!(register_list_from_bit_array(0b1), vec![Register::ZERO]);
        assert_eq!(
            register_list_from_bit_array(0b111),
            vec![Register::ZERO, Register::RA, Register::SP]
        );
        assert_eq!(
            register_list_from_bit_array(0b1000000000000000),
            vec![Register::A5]
        );
        assert_eq!(
            register_list_from_bit_array(0b1110000000000000),
            vec![Register::A3, Register::A4, Register::A5]
        );
        assert_eq!(
            register_list_from_bit_array(0xffff),
            vec![
                Register::ZERO,
                Register::RA,
                Register::SP,
                Register::GP,
                Register::TP,
                Register::T0,
                Register::T1,
                Register::T2,
                Register::S0,
                Register::S1,
                Register::A0,
                Register::A1,
                Register::A2,
                Register::A3,
                Register::A4,
                Register::A5
            ]
        );
    }
}
