#!/bin/bash

if [ $# -le 1 ]; then
	echo "Needs more args"
	echo "Usage: $0 {num1} {num2}"
	return 1
fi

if [ $2 -lt $1 ]; then
	echo "arg 1 sould be less then or equal to arg 2"
	echo "Usage: $0 {num1} {num2}"
	return 1
fi

# use std::time::{Duration, Instant};
BUILD_STRING="use std::time::{Duration, Instant};"

# mod pe_1;
# mod pe_2;
COUNT=$1
while [ $COUNT -le $2 ]; do
	BUILD_STRING="${BUILD_STRING} mod pe_$COUNT;"
	(( COUNT += 1 ))
done


# fn main() {
#     println!("Starting Euler!!!");
#     println!("--------------------");
#     let mut results = Vec::new();
#     let start = Instant::now();

BUILD_STRING="
${BUILD_STRING} fn main(){
	println!(\"Starting Euler!!!\");
	let mut results = Vec::new();	
	let start = Instant::now();
"


#     results.push(time(pe_1::main));
#     results.push(time(pe_2::main));
COUNT=$1
while [ $COUNT -le $2 ]; do
	BUILD_STRING="${BUILD_STRING} results.push(time(pe_$COUNT::main));"
	(( COUNT += 1 ))
done


#     let end = Instant::now();

#     for (i, (result, dur)) in results.iter().enumerate() {
#         println!("{}: {result:<15} | Time: {dur:?}", i + 1);
#     }

#     println!("--------------------");
#     println!("Total Time: {:?}", end.duration_since(start));
# }

# fn time<T, F: Fn() -> T>(f: F) -> (i128, Duration)
# where
#     T: Into<i128>,
# {
#     let start = Instant::now();
#     let result = f();
#     let end = Instant::now();

#     let result = result;

#     return (result.into(), end.duration_since(start));
# }

BUILD_STRING="${BUILD_STRING}
    let end = Instant::now();

	println!(\"----------------------\");

    for (i, (result, dur)) in results.iter().enumerate() {
        println!(\"{:>3}: {result:<15} | Time: {dur:?}\", i + 1);
    }

    println!(\"----------------------\");
    println!(\"Total Time: {:?}\", end.duration_since(start));
}

fn time<T, F: Fn() -> T>(f: F) -> (i128, Duration)
where
    T: Into<i128>,
{
    let start = Instant::now();
    let result = f();
    let end = Instant::now();

    let result = result;

    return (result.into(), end.duration_since(start));
}
"

echo $BUILD_STRING



