# Command line argument

## Use how many threads to run test
cargo test -- --test-threads=${number}

if some test shuld be consucutively excute, set the number to 1

## Show output of printn!
cargo test -- --show-output

## Run single test
cargo test ${test name}

## Run multiple tests
cargo test ${substring of test name}

## Run ignored tests
cargo test -- --ignored