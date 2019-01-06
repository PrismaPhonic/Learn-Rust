# Chapter 20

# Table of Contents
1. [Single Threaded Web Server](#single-threaded-web-server)

# Web Server Project

For the last chapter of the book we build a multiple-threaded web server, but we
have to walk before we can crawl!  We'll start out by building a simple single
threaded web server from the ground up. Like the other project heavy sections of
this book I'll mostly be posting large blocks of code and explaining what went
on - the book already does a great job of stepping you through every single
piece of the build out.

## Single Threaded Web Server

If we type `127.0.0.1:7878` into our web browser we will see the http request in
our terminal that the browser is making:

```
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0
Accept:
text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
```

If we curl our single threaded web server with a command line we will get a much
shorter web request:

```
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/7.63.0
Accept: */*
```

We can instead query our server with a POST request and some json to actually
see what's in the message body:

```
Request: POST / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/7.63.0
Accept: */*
Content-Type: application/json
Content-Length: 17

{"key1":"value1"}
```


