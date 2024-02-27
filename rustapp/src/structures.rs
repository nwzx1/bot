pub mod strcture {
    use std::collections::btree_map::Range;

    pub struct Pettern;

    pub struct Perem {
        pub to: i32,
        pub end: i32,
    }

    impl Pettern {
        pub fn PETTERN_OF_RightAngleTringle(per: Perem) {
            for i in per.to..per.end {
                for j in per.to..i {
                    print!("*");
                }
                println!()
            }
        }

        pub fn PETTERN_OF_kiteTringle(per: Perem) {
            for i in per.to..per.end {
                for j in per.to..i {
                    let am = [per.to..i];
                    let (first, last) = (am.first(), am.last());
                    if (first != None && last != None) {
                        print!("*");
                    }
                }
                println!()
            }
        }
    }
}
