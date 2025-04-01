FROM perl:alpine

RUN apk update && apk add --no-cache \
    build-base \
    && rm -rf /var/cache/apk/*

COPY . /app

WORKDIR /app

RUN cpanm --notest --installdeps .

EXPOSE 3000

CMD ["hypnotoad", "app.pl"]

