BINARY = ./target/release/star-catalog
STARS = hipparcos.json -m 7.5

# 15mm lens on Rebelt T2i is 107 degree horizontal FOV
VIEW = -d 70 -f 107 -W 2000 -H 2000 -a 340

STARS = 1775 1776  2026 1486  1677 1979  4069 1314  2064 1781 \
3982 1006  \
3776 2159 \
3038 2709 \
2302 2979 \
3278 2101 \
2744 741 \
2623 663 \
2797 675 \
2660 796 \
2756 993 \
2907 731 \
2157 1195 \
3788 1668 \
2969 2479 \
2869 2329 \
2410 2548 \
2438 2496 \
1751 2448 \
2045 2477 \
2182 2513 \
3863 1836 \
2870 1452 \
3059 1513 \
2988 1519 \
2846 1542 \
2577 2093 \
2501 2067 \
2391 1978 \
2334 2023 \
3175 2041 \
1686 2068 \
1802 2017 \
1741 1557 \
1962 1815


.PHONY: help
help:
	@echo "Use 'test_all' to run tests (on release) with a suitable set of feature sets"
	@echo "Use 'release' to build a release image with all the features snabled"
	@echo "Use 'cubemap' to generate a cubemap of the Hipp bright stars of magnitude 8 and above focuses on Polaris with Dubhe 'up', with a 1024-by-1024 square for each face"
	@echo "Use 'stars_of_image' to test a set of stars on an image taken with a focal length of 23.48mm on an 18MP camera

.PHONY: test_all
test_all:
	cargo test --release --features image,postcard,csv,hipp_bright
	cargo test --release
	cargo test --release --features image
	cargo test --release --features postcard
	cargo test --release --features csv
	cargo test --release --features hipp_bright

.PHONY: test_hipp_bright
test_hipp_bright:
	cargo test --release --features hipp_bright

.PHONY: install
	cargo install --path . --features image,postcard,csv,hipp_bright

.PHONY: release
release:
	cargo build --release --features image,postcard,csv,hipp_bright

.PHONY: stars_of_image
stars_of_image:
	cargo build --release --features image,postcard,csv,hipp_bright
	./target/release/star-catalog hipp_bright -m 6 stars_of_image -f 23.48 -W 5184 -H 3456 -a 0.3 ${STARS}

.PHONY: dtrace
dtrace:
	cargo build --release --features image,postcard,csv,hipp_bright
	rm -rf a.trace
	xctrace record --output a.trace --template "Time Profiler" --target-stdout - --launch -- ./target/release/star-catalog hipp_bright -m 6 stars_of_image -f 23.48 -W 5184 -H 3456 -a 0.3 ${STARS}

.PHONY: clippy
clippy:
	cargo clippy --features image,postcard,csv,hipp_bright

.PHONY: docs
docs:
	cargo doc --all-features

PHONY: cubemap
cubemap: release
	# ${BINARY} hipp_bright -m 7. --names collated cubemap -W 1024 -H 1024 --output ~/test.png
	${BINARY} hipp_bright -m 8. --names collated cubemap --star Polaris --up Dubhe --angle 90 -W 1024 -H 1024 --output ~/test.png

images: release
	${BINARY} ${STARS} image -r 0 $(VIEW) -o winter_hexagon_0.png
	${BINARY} ${STARS} image -r 30 $(VIEW) -o winter_hexagon_30.png
	${BINARY} ${STARS} image -r 60 $(VIEW) -o winter_hexagon_60.png
	${BINARY} ${STARS} image -r 90 $(VIEW) -o winter_hexagon_90.png
	${BINARY} ${STARS} image -r 120 $(VIEW) -o winter_hexagon_120.png
	${BINARY} ${STARS} image -r 150 $(VIEW) -o winter_hexagon_150.png
	${BINARY} ${STARS} image -r 180 $(VIEW) -o winter_hexagon_180.png
	${BINARY} ${STARS} image -r 210 $(VIEW) -o winter_hexagon_210.png
	${BINARY} ${STARS} image -r 240 $(VIEW) -o winter_hexagon_240.png
	${BINARY} ${STARS} image -r 270 $(VIEW) -o winter_hexagon_270.png
	${BINARY} ${STARS} image -r 300 $(VIEW) -o winter_hexagon_300.png
	${BINARY} ${STARS} image -r 330 $(VIEW) -o winter_hexagon_330.png
