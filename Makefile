upload:
	rsync -rvp --exclude target . pi@bluedata.local:/home/pi/bluedata --delete