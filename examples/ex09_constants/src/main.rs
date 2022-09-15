use tetr_ch::model::league::Rank;

fn main() {
    // This library is providing the color code for each rank as a constant.
    dbg!(Rank::D_COL);
    dbg!(Rank::D_PLUS_COL);
    dbg!(Rank::C_MINUS_COL);
    dbg!(Rank::C_COL);
    dbg!(Rank::C_PLUS_COL);
    dbg!(Rank::B_MINUS_COL);
    dbg!(Rank::B_COL);
    dbg!(Rank::B_PLUS_COL);
    dbg!(Rank::A_MINUS_COL);
    dbg!(Rank::A_COL);
    dbg!(Rank::A_PLUS_COL);
    dbg!(Rank::S_MINUS_COL);
    dbg!(Rank::S_COL);
    dbg!(Rank::S_PLUS_COL);
    dbg!(Rank::SS_COL);
    dbg!(Rank::U_COL);
    dbg!(Rank::X_COL);
    dbg!(Rank::XX_COL);
    dbg!(Rank::Z_COL);

    // You can also get the rank color from each rank model.
    // Example:
    assert_eq!(Rank::D_COL, Rank::D.color());

    // You can see each rank color visually in the doc below:
    // https://docs.rs/tetr_ch/latest/tetr_ch/model/league/enum.Rank.html#associatedconstant.D_COL



    // ! The following constants are deprecated:
    // https://docs.rs/tetr_ch/latest/tetr_ch/constants/rank_col/
    /* 
    use tetr_ch::constants::rank_col;
    dbg!(rank_col::D);
    dbg!(rank_col::D_PLUS);
    dbg!(rank_col::C_MINUS);
    dbg!(rank_col::C);
    dbg!(rank_col::C_PLUS);
    dbg!(rank_col::B_MINUS);
    dbg!(rank_col::B);
    dbg!(rank_col::B_PLUS);
    dbg!(rank_col::A_MINUS);
    dbg!(rank_col::A);
    dbg!(rank_col::A_PLUS);
    dbg!(rank_col::S_MINUS);
    dbg!(rank_col::S);
    dbg!(rank_col::S_PLUS);
    dbg!(rank_col::SS);
    dbg!(rank_col::U);
    dbg!(rank_col::X);
    dbg!(rank_col::XX);
    dbg!(rank_col::Z);
    */
}
