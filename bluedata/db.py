import csv
from loguru import logger
from datetime import datetime


def write_data(temp: float, light: float, salinity: float) -> None:
    with open("test.csv", mode="a") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow([temp, light, salinity, datetime.now()])
        logger.info(
            f"Wrote data to CSV file (temp: {temp}, light: {light}, salinity: {salinity}, and current time)"
        )
