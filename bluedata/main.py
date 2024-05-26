from loguru import logger
import db


def main() -> None:
    logger.info("Hello world!")
    db.write_data(temp=50.3, light=0.80, salinity=0.30)


if __name__ == "__main__":
    main()
