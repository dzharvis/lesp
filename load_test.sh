# ./target/release/lesp  0.61s user 0.00s system 99% cpu 0.613 total !!!! RELEASE
echo "(do (map (genlist 100) (fn _ (_) (map (genlist 100) square))) 1)" | ./target/debug/lesp