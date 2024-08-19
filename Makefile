BINARY = ./target/release/star-catalog
STARS = hipparcos.json -m 7.5

# 15mm lens on Rebelt T2i is 107 degree horizontal FOV
VIEW = -d 70 -f 107 -W 2000 -H 2000 -a 340

.PHONY:
help:
	@echo "Help goes here"

.PHONY:
release:
	cargo build --release --features image

all: release
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
