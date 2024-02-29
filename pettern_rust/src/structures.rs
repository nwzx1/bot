pub mod strcture {

    pub struct Pettern;

    pub struct Perem {
        pub to: i32,
        pub end: i32,
    }

    impl Pettern {
        pub fn pettern_of_right_angle_tringle(per: Perem) {
            for i in per.to..per.end {
                for _j in per.to..i {
                    print!("*");
                }
                println!()
            }
        }

        pub fn pettern_of_informal_kite_tringle(per: Perem) {
            for i in per.to..per.end {
                for _j in i..per.end {
                    print!("  ");
                }
                for _j in per.to..i {
                    print!("* ");
                }
                for _j in i..per.end {
                    print!("* ");
                }
                println!()
            }
        }

        pub fn pettern_of_formal_kite_tringle(per: Perem) {
            for i in per.to..per.end {
                for _j in i..per.end {
                    print!("  ");
                }
                for _j in per.to..i {
                    print!("* ");
                }
                for _j in per.to..i - 1 {
                    print!("* ");
                }
                println!()
            }
            for i in per.to..per.end {
                for _j in per.to..i {
                    print!("  ");
                }
                for _j in i..per.end {
                    print!("* ");
                }
                for _j in i..per.end - 1 {
                    print!("* ");
                }
                println!()
            }
        }

        pub fn pettern_of_arrowup_tringle(per: Perem) {
            for i in per.to..per.end {
                for _j in i..per.end {
                    print!("  ");
                }
                for _j in per.to..i {
                    print!("* ");
                }
                for _j in per.to..i - 1 {
                    print!("* ");
                }
                println!()
            }
        }

        pub fn pettern_of_number_tringle(per: Perem) {
            for i in per.to..per.end {
                for _j in i..per.end {
                    print!("  ");
                }
                for _j in per.to..i + 1 {
                    print!("{} ", _j + 1);
                }
                for _j in per.to..i {
                    print!("{} ", _j +1 );
                }
                println!()
            }
        }

        pub fn pettern_of_number_formal_kite_tringle(per: Perem) {
            for i in per.to..per.end {
                for _j in i..per.end {
                    print!("  ");
                }
                for _j in per.to..i {
                    print!("{} ",_j);
                }
                for _j in per.to..i - 1 {
                    print!("{} ",_j);
                }
                println!()
            }
            for i in per.to..per.end {
                for _j in per.to..i {
                    print!("  ");
                }
                for _j in i..per.end {
                    print!("{} ",_j);
                }
                for _j in i..per.end - 1 {
                    print!("{} ",_j);
                }
                println!()
            }
        }


        
    }
}
