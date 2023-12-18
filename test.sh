#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  ./target/debug/rcr "$input" > tmp.s
  riscv32-unknown-linux-gnu-gcc -o tmp tmp.s -static
  spike $(which pk) ./tmp >/dev/null 2>&1
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

cargo build

assert 0 0
assert 42 42

echo OK
