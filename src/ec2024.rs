//! Solutions for season 2024

pub mod ec2024q01;
pub mod ec2024q02;
// pub mod ec2024q03;
// pub mod ec2024q04;
// pub mod ec2024q05;
// pub mod ec2024q06;
// pub mod ec2024q07;
// pub mod ec2024q08;
// pub mod ec2024q09;
// pub mod ec2024q10;
// pub mod ec2024q11;
// pub mod ec2024q12;
// pub mod ec2024q13;
// pub mod ec2024q14;
// pub mod ec2024q15;
// pub mod ec2024q16;
// pub mod ec2024q17;
// pub mod ec2024q18;
// pub mod ec2024q19;
// pub mod ec2024q20;

/// array of implemented solutions for the season
pub const PUZZLES: crate::ec::Season = [
    Some((ec2024q01::metadata, ec2024q01::solve)),
    Some((ec2024q02::metadata, ec2024q02::solve)),
    None, // Some((ec2024q03::metadata, ec2024q03::solve)),
    None, // Some((ec2024q04::metadata, ec2024q04::solve)),
    None, // Some((ec2024q05::metadata, ec2024q05::solve)),
    None, // Some((ec2024q06::metadata, ec2024q06::solve)),
    None, // Some((ec2024q07::metadata, ec2024q07::solve)),
    None, // Some((ec2024q08::metadata, ec2024q08::solve)),
    None, // Some((ec2024q09::metadata, ec2024q09::solve)),
    None, // Some((ec2024q10::metadata, ec2024q10::solve)),
    None, // Some((ec2024q11::metadata, ec2024q11::solve)),
    None, // Some((ec2024q12::metadata, ec2024q12::solve)),
    None, // Some((ec2024q13::metadata, ec2024q13::solve)),
    None, // Some((ec2024q14::metadata, ec2024q14::solve)),
    None, // Some((ec2024q15::metadata, ec2024q15::solve)),
    None, // Some((ec2024q16::metadata, ec2024q16::solve)),
    None, // Some((ec2024q17::metadata, ec2024q17::solve)),
    None, // Some((ec2024q18::metadata, ec2024q18::solve)),
    None, // Some((ec2024q19::metadata, ec2024q19::solve)),
    None, // Some((ec2024q20::metadata, ec2024q20::solve)),
];
