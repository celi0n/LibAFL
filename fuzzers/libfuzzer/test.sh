#!/bin/sh

cargo build --release || exit 1
cp ./target/release/libfuzzer ./.libfuzzer_test.elf

RUST_BACKTRACE=1 ./.libfuzzer_test.elf &

test "$!" -gt 0 && {

  usleep 250
  RUST_BACKTRACE=1 ./.libfuzzer_test.elf -x a -x b -T5 in1 in2 &

}

sleep 10
killall .libfuzzer_test.elf
rm -rf ./.libfuzzer_test.elf