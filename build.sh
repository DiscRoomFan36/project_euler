
# set -xe

TOTAL=`ls -1q src/* | wc -l` 

T1=$(($TOTAL-1))

./derive_main.sh 1 ${T1} > src/main.rs
cargo run $1
