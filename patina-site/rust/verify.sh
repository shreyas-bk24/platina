#!/usr/bin/env bash
# Compiles every example with warnings as errors and runs the tests.
set -euo pipefail
cd "$(dirname "$0")"
out=$(mktemp -d)
echo "DSA patterns (unit tests):"
for f in patterns/*.rs; do
  rustc --edition 2021 --test -D warnings "$f" -o "$out/t" && "$out/t" -q >/dev/null && echo "  ok  $f"
done
echo "Concept programs (std only):"
for f in concepts/*.rs; do
  rustc --edition 2021 -D warnings "$f" -o "$out/c" && echo "  ok  $f -> $("$out/c" | tr '\n' ' ')"
done
echo "Async and Rayon programs:"
(cd cargo && RUSTFLAGS="-D warnings" cargo build -q --bins && for b in async_tokio async_select scoped; do echo "  ok  $b -> $(./target/debug/$b | tr '\n' ' ')"; done)
echo "Failing examples are in concepts/failing/ and are expected NOT to compile (or to panic, leak, or hang)."
