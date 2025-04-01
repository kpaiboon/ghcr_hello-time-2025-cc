#!/usr/bin/env perl

use Mojolicious::Lite;
use Time::Piece;
use Mojo::Util qw(trim);

my $greeting = trim( $ENV{GREETING_TEXT} || "Hello World" );
my $port = $ENV{PORT} || 6000; # Default to port 6000

get '/' => sub {
  my $self = shift;
  my $now = gmtime;
  $self->render_text("$greeting and Time GMT: " . $now->strftime("%Y-%m-%d %H:%M:%S"));
};

app->start;
