 export HASH_PREFIX="0000" && cargo run --release
    Finished release [optimized] target(s) in 0.0 secs
     Running `target/release/main`
test_mine_single_threaded_mutably:
10 ms
8 ms
8 ms
8 ms
8 ms
7 ms
11 ms
8 ms
8 ms
10 ms
mean:	9 ms/iter
median:	8 ms/iter
min:	7 ms/iter
max:	11 ms/iter
stddev:	2

test_mine_with_iterator:
8 ms
9 ms
8 ms
10 ms
13 ms
15 ms
10 ms
10 ms
8 ms
9 ms
mean:	10 ms/iter
median:	10 ms/iter
min:	8 ms/iter
max:	15 ms/iter
stddev:	3

test_mine_with_parallel_iterator_find_first:
18 ms
23 ms
23 ms
29 ms
28 ms
16 ms
16 ms
16 ms
17 ms
19 ms
mean:	21 ms/iter
median:	19 ms/iter
min:	16 ms/iter
max:	29 ms/iter
stddev:	5

test_mine_with_parallel_iterator_find_any:
17 ms
21 ms
28 ms
18 ms
16 ms
15 ms
29 ms
16 ms
17 ms
20 ms
mean:	20 ms/iter
median:	18 ms/iter
min:	15 ms/iter
max:	29 ms/iter
stddev:	5

test_mine_with_channels:
9 ms
3 ms
3 ms
10 ms
10 ms
10 ms
14 ms
10 ms
6 ms
4 ms
mean:	8 ms/iter
median:	10 ms/iter
min:	3 ms/iter
max:	14 ms/iter
stddev:	4


export HASH_PREFIX="00000" && cargo run --release
    Finished release [optimized] target(s) in 0.0 secs
     Running `target/release/main`
test_mine_single_threaded_mutably:
610 ms
849 ms
666 ms
742 ms
626 ms
782 ms
665 ms
612 ms
614 ms
594 ms
mean:	677 ms/iter
median:	665 ms/iter
min:	594 ms/iter
max:	849 ms/iter
stddev:	83

test_mine_with_iterator:
649 ms
629 ms
657 ms
690 ms
682 ms
667 ms
620 ms
615 ms
669 ms
631 ms
mean:	651 ms/iter
median:	657 ms/iter
min:	615 ms/iter
max:	690 ms/iter
stddev:	25

test_mine_with_parallel_iterator_find_first:
1226 ms
1273 ms
1296 ms
1271 ms
1178 ms
1195 ms
1281 ms
1330 ms
1284 ms
1273 ms
mean:	1262 ms/iter
median:	1274 ms/iter
min:	1179 ms/iter
max:	1331 ms/iter
stddev:	45

test_mine_with_parallel_iterator_find_any:
1273 ms
1279 ms
1285 ms
1391 ms
1320 ms
1274 ms
1393 ms
1368 ms
1351 ms
1436 ms
mean:	1338 ms/iter
median:	1352 ms/iter
min:	1274 ms/iter
max:	1437 ms/iter
stddev:	57

test_mine_with_channels:
280 ms
279 ms
285 ms
284 ms
275 ms
273 ms
278 ms
279 ms
279 ms
270 ms
mean:	279 ms/iter
median:	279 ms/iter
min:	270 ms/iter
max:	285 ms/iter
stddev:	5


export HASH_PREFIX="000000" && cargo run --release
    Finished release [optimized] target(s) in 0.0 secs
     Running `target/release/main`
test_mine_single_threaded_mutably:
10221 ms
10169 ms
10579 ms
9691 ms
9849 ms
9947 ms
10410 ms
9503 ms
10637 ms
9490 ms
mean:	10055 ms/iter
median:	10175 ms/iter
min:	9495 ms/iter
max:	10642 ms/iter
stddev:	399

test_mine_with_iterator:
10048 ms
10079 ms
10187 ms
10224 ms
9547 ms
9584 ms
9639 ms
10159 ms
9725 ms
9561 ms
mean:	9881 ms/iter
median:	10052 ms/iter
min:	9552 ms/iter
max:	10232 ms/iter
stddev:	273

test_mine_with_parallel_iterator_find_first:
19304 ms
19616 ms
18655 ms
18535 ms
18708 ms
18605 ms
19087 ms
18667 ms
18750 ms
18834 ms
mean:	18885 ms/iter
median:	18760 ms/iter
min:	18547 ms/iter
max:	19629 ms/iter
stddev:	334

test_mine_with_parallel_iterator_find_any:
5622 ms
5644 ms
5678 ms
5802 ms
5890 ms
5867 ms
5636 ms
5837 ms
5580 ms
5606 ms
mean:	5719 ms/iter
median:	5682 ms/iter
min:	5583 ms/iter
max:	5891 ms/iter
stddev:	114

test_mine_with_channels:
4532 ms
4962 ms
4453 ms
4637 ms
4404 ms
4474 ms
4460 ms
4712 ms
4500 ms
4769 ms
mean:	4594 ms/iter
median:	4535 ms/iter
min:	4408 ms/iter
max:	4965 ms/iter
stddev:	169

top for "000000":

test_mine_single_threaded_mutably:
2622  main         99.4
test_mine_with_iterator:
2681  main         98.8
test_mine_with_parallel_iterator_find_first:
2732  main         371.9
test_mine_with_parallel_iterator_find_any:
2783  main         362.3
test_mine_with_channels:
2833  main         383.5