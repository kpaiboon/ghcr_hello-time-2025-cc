FROM perl:5.40.1-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY . /app

WORKDIR /app

RUN cpanm --notest --installdeps .

EXPOSE 3000

CMD ["hypnotoad", "app.pl"]
