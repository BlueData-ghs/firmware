run:
	poetry run -- python3 bluedata/main.py

upload:
	rsync -rvp . pi@bluedata.local:/home/pi/bluedata/