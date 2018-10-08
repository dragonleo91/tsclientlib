#![feature(test)]

extern crate gmp;
extern crate num;
extern crate num_traits;
extern crate test;

use gmp::mpz::Mpz;
use num::bigint::BigUint;
use num_traits::One;
use test::Bencher;

#[bench]
fn modpow(b: &mut Bencher) {
	let n = "9387019355706217197639129234358945126657617361248696932841794255538327365072557602175160199263073329488914880215590036563068284078359088114486271428098753";
	let x = "2148617454765635492758175407769288127281667975788420713054995716016550287184632946544163990319181591625774561067011999700977775946073267145316355582522577";
	let level = 10_000;
	let n = n.parse().unwrap();
	let x: BigUint = x.parse().unwrap();
	let mut e = BigUint::one();
	e <<= level as usize;

	b.iter(|| {
		x.modpow(&e, &n);
	});
}

#[cfg(non_existant)]
#[bench]
fn modpow2(b: &mut Bencher) {
	// Without allocating the exponent - uses a patched num-bigint
	let n = "9387019355706217197639129234358945126657617361248696932841794255538327365072557602175160199263073329488914880215590036563068284078359088114486271428098753";
	let x = "2148617454765635492758175407769288127281667975788420713054995716016550287184632946544163990319181591625774561067011999700977775946073267145316355582522577";
	let level = 10_000;
	let n = n.parse().unwrap();
	let x: BigUint = x.parse().unwrap();

	b.iter(|| {
		x.modpow2(level, &n);
	});
}

#[bench]
fn gmp_modpow(b: &mut Bencher) {
	let n = "9387019355706217197639129234358945126657617361248696932841794255538327365072557602175160199263073329488914880215590036563068284078359088114486271428098753";
	let x = "2148617454765635492758175407769288127281667975788420713054995716016550287184632946544163990319181591625774561067011999700977775946073267145316355582522577";
	let level = 10_000;
	let n = n.parse().unwrap();
	let x: Mpz = x.parse().unwrap();
	let mut e = Mpz::new();
	e.setbit(level);

	b.iter(|| {
		x.powm(&e, &n);
	});
}
