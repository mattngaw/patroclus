#[cfg(test)]
mod rank_tests {
    use crate::bits::Rank;
    
    #[test]
    fn u32_to_rank() {
        let r = 5.try_into();
        assert_eq!(r, Ok(Rank::Sixth));
        let r = Rank::try_from(69);
        assert_eq!(r, Err(69));
    }

    #[test]
    fn rank_to_u32() {
        let i = Rank::Sixth as u32;
        assert_eq!(i, 5);
    }

    #[test]
    fn char_to_rank() {
        let r = '3'.try_into();
        assert_eq!(r, Ok(Rank::Third));
        let r = Rank::try_from('a');
        assert_eq!(r, Err('a'));
    }

    #[test]
    fn rank_to_char() {
        let c = char::from(Rank::First);
        assert_eq!(c, '1');
    }

    #[test]
    fn rank_iter() {
        let v1 = Rank::iter().collect::<Vec<Rank>>();
        let v2 = vec![
            Rank::First, 
            Rank::Second, 
            Rank::Third, 
            Rank::Fourth, 
            Rank::Fifth, 
            Rank::Sixth, 
            Rank::Seventh, 
            Rank::Eighth
        ];
        assert_eq!(v1, v2);
    }
}

#[cfg(test)]
mod file_tests {
    use crate::bits::File;
    
    #[test]
    fn u32_to_file() {
        let f = 5.try_into();
        assert_eq!(f, Ok(File::F));
        let f = File::try_from(69);
        assert_eq!(f, Err(69));
    }

    #[test]
    fn file_to_u32() {
        let i = File::H as u32;
        assert_eq!(i, 7);
    }

    #[test]
    fn char_to_file() {
        let f = 'a'.try_into();
        assert_eq!(f, Ok(File::A));
        let f = File::try_from('9');
        assert_eq!(f, Err('9'));
    }

    #[test]
    fn file_to_char() {
        let c = char::from(File::E);
        assert_eq!(c, 'e');
    }

    #[test]
    fn file_iter() {
        let v1 = File::iter().collect::<Vec<File>>();
        let v2 = vec![
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H
        ];
        assert_eq!(v1, v2);
    }
}

#[cfg(test)]
mod square_tests {
    
}

#[cfg(test)]
mod bitboard_tests{

}