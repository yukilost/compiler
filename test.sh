#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/compiler "$input" > result.s
    cc -o result result.s
    ./result
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual" 
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 0 0 
assert 42 42

echo OK