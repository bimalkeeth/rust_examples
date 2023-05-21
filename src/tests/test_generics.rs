use crate::and;
use crate::xor;
use crate::generics;
use crate::generics::complex::Complex;
use crate::generics::games::{Enemy, Games, Hero};
use crate::generics::media_player::Playable;
use crate::generics::trait_bounds::{Bob, Programmer};
use crate::generics::vehicle::{Car, TeslaRoadStar, Vehicle};

#[test]
fn test_and() {
    assert_eq!(1, and(1, 1));
    assert_eq!(0, and(0, 1));
    assert_eq!(0, and(1, 0));
    assert_eq!(0, and(0, 0));
}

#[test]
fn test_xor() {
    assert_eq!(1, xor(1, 0));
    assert_eq!(0, xor(0, 0));
    assert_eq!(0, xor(1, 1));
    assert_eq!(1, xor(0, 1));
}

#[test]
fn test_generic() {
    generics::functions::test_generic()
}

#[test]
fn test_playable() {
    let audio = generics::media_player::Audio("ambient_music.mp3".to_string());
    let video = generics::media_player::Video("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();
}

#[test]
fn test_vehicle() {
    let road_start = TeslaRoadStar::new("Tesla RoadStar II", 2020);
    println!("{} is priced at ${}", road_start.model(), road_start.get_price())
}

#[test]
fn test_games() {
    let games = Games;
    games.load(Hero);
    games.load(Enemy);
}

#[test]
fn eater_repeat() {
    let bob = Bob;
    bob.animate()
}

#[test]
fn lazy_addr() {
    let add = generics::trait_bounds::lazy_addr(100, 34);
    println!("{:?}", add())
}

#[test]
fn surround_with_braces() {
    println!("{}", generics::trait_bounds::surrounds_with_braces("Hello"))
}

#[test]
fn complex_basic() {
    let first = Complex::new(3, 5);
    let second:Complex<i32> = Complex::default();

    assert_eq!(first.re,3);
    assert_eq!(first.im,5);
}

#[test]
fn complex_add(){
    let a =Complex::new(1,-2);
    let b:Complex<i32> = Complex::default();

    let res = a +b;
    assert_eq!(res,a);

    generics::complex::mutable_borrow_check()
}


#[test]
fn borrow_checker() {

        generics::unsafe_blocks::unsafe_trait_test();


}
