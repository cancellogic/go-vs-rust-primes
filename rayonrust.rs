extern crate rayon;  //dustan doud coded this multithread RAYON prime finding toy. :) RUST+RAYON ROCKS!   
                      
use std::time::Instant;
use rayon::prelude::*;

fn isqrt(num: u32) -> u32 { // thanks leonardo at rust.lang.org for this function
    let r= (num as f64).sqrt() as u32; //float point precision vs integer discrete
    if r<4096 {return r} // compute root of num and return, large root may a[[-1]]have  error delta
    (num/r + r)/2   // so divide num by delta root for anti-delta root, average result & return
}

fn prime_b(testprime:u32, primelist:&Vec<u32>) -> bool {
    let limit = isqrt(testprime);   //run ordered low to high, no reason to check above square root
    let mut prime:bool = false;  //default assumption is not a prime
    'calculation: for i in primelist{
        if *i > limit { prime=true; //if i is larger than test limit num must be prime
            break 'calculation;}   //example 7:  7%2 !=0, 7%3
            else {
                if testprime % *i == 0 {  //if remainder is 0, testprime is not a prime,
                    break 'calculation; }  // break calculation loop
            }
    }
    return prime
}

fn main()  {
    let mut known_primes:Vec<u32> = Vec::new();
    known_primes.extend([2,3,5,7,11].iter());
    
    let start_timer= Instant::now();
    for _growing_knowledge in 1..4 {  //knowledge loop splits finding  primes and adding them to list of known primes
        let mut start_at :u32=  1+ &known_primes[&known_primes.len() - 1] ;  //start after last prime found
        let mut block_end :u32=  &start_at * &start_at - 1;
                                 //for odd prime n, n^2 not prime, n^2 -1 %2 is even ∴ not prime
        let found_primes:Vec<u32> = (start_at..block_end)
            .into_par_iter()
            .filter_map(|x| { if prime_b(x,&known_primes) {Some(x)} else {None } } )
            .collect();

        for num in found_primes { //
            known_primes.push(num);
        }
    }
    println!("Multithread {:?}",start_timer.elapsed() );
    println!("found {} primes up to a value of {}",known_primes.len(),known_primes[&known_primes.len()-1]);
    print!("{{");for value in known_primes { print!("{}, ",value) }; print!("0 }}")
}
