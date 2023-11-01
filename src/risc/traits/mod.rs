pub mod reg;
pub mod signedunsigned;



#[cfg(test)]
pub mod test {
    use super::reg::Reg;

    #[test]
    fn shift_right_arithmetic() {
        macro_rules! test_all_types_shift_right {
            ($($u_t:ty, $i_t:ty),*) => {
                $(
                    let mut j = <$i_t>::MIN as $u_t;
                    for _ in 0.. 256 {
                        j.sra(1);
                        assert!((j as $i_t) < 0);
                    }
                    let mut sra = <$i_t>::MAX as $u_t;
                    let mut srl = <$i_t>::MAX as $u_t;
                    for _ in 0..256 {
                        sra = sra.sra(1);
                        srl = srl.srl(1);
                        assert_eq!(sra, srl);
                    }
                )*
            };
        }

        test_all_types_shift_right!{
            // u16, i16, // unsupported
            u32, i32,
            u64, i64,
            u128, i128
        }
    }

    #[test]
    fn set_less_than() {
        macro_rules! set_less_than_all_types {
            ($($u_t:ty, $i_t:ty),*) => {
                $(
                    // -32 to 32 and MIN and MAX of signed type
                    for i in (-32..=32).chain([<$i_t>::MIN, <$i_t>::MAX].into_iter()) {
                        for j in (-32..=32).chain([<$i_t>::MIN, <$i_t>::MAX].into_iter()) {
                            let ui = i as $u_t;
                            let slt = ui.slt(j as $u_t);
                            let sltu = ui.sltu(j as $u_t);

                            assert_eq!(slt, if i < j {1} else {0});
                            assert_eq!(sltu, if (i as $u_t) < (j as $u_t) {1} else {0});
                        }
                    }
                )*
            };
        }

        set_less_than_all_types!{
            // u16, i16, // unsupported
            u32, i32,
            u64, i64,
            u128, i128
        }
    }
}