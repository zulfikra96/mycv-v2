FROM rust:1.75-slim-bookworm 

WORKDIR /home/app

COPY back-end .
COPY .env .

RUN apt-get update && apt-get install -y
RUN apt-get upgrade -y
RUN apt-get install libpq-dev -y