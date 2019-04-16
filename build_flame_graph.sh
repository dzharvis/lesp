sudo perf record --call-graph dwarf  ./load_test.sh
sudo perf script | ~/tools/FlameGraph/stackcollapse-perf.pl | ~/tools/FlameGraph/flamegraph.pl > perf.svg