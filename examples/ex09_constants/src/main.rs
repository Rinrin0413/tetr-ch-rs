use tetr_ch::constants::rank_col;

fn main() {
    // This library is providing the color code for each rank as a constant
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

    // You can see each rank color visually in the doc below:
    // https://docs.rs/tetr_ch/latest/tetr_ch/constants/rank_col/
}
