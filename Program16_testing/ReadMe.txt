to run test 
    cargo test

two sets of command line options
1. one set goes to cargo test command
2. resulting test binary
they are seperated by 2 dashes like cargo test -- --show-output

->cargo test --help

if want to find out which command can be passed to resulting test binary\
->cargo test -- --help
cargo test -- --test-thread=1
    this will slow the processing since each thread is executed

showing output:
    cargo test -- --show-output
    print statement is omitted coz test pass. but if we want to see all the process we use this.

run specific test
    cargo test test_fail 
    here mention test name

run test that contain failing keyword such as failing_add and failing_subtract. here multiple can run that contain with failing
    cargo test failing

run tests that is under cetain module
    cargo test test::
    this will run all test under test module

ignore test to run
    for example, only run occasionally or that take alot of resources or time executed
    #[ignore]
    mention this below #[test]

only run ignored test i.e. that takes time etc
    cargo test -- --ignored
    this will run only ignored tests

Rust community believe in 2 tests
1. Unit - small ,focused,test one module in isolation and could test private interfaces
2. Integration - they are completely external to your library


