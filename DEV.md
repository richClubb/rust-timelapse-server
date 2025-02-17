# Developer Notes

## use of the `DefaultBodyLimit::max(2000000)`

This seems to throw an error if you set the value too low when using multipart data, so you might need to handle this more gracefully and report an error to the user

```
richard@UKCARL-7LNZC44:~ $ curl -F "image=@test_image.jpg" localhost:3000/foo/test # Image is 2857744 bytes, max set to 2000000
curl: (56) Recv failure: Connection reset by peer
```

Error seen in console
```
Entire form: Multipart { inner: Multipart { state: Mutex { data: MultipartState { buffer: StreamBuffer, boundary: "------------------------7Hv5jpw0rQI0EhyGaHYQKy", stage: FindingFirstBoundary, next_field_idx: 0, curr_field_name: None, curr_field_size_limit: 18446744073709551615, curr_field_size_counter: 0, constraints: Constraints { size_limit: SizeLimit { whole_stream: 18446744073709551615, per_field: 18446744073709551615, field_map: {} }, allowed_fields: None } }} } }
thread 'tokio-runtime-worker' panicked at src/main.rs:25:40:
called `Result::unwrap()` on an `Err` value: MultipartError { source: failed to read stream }
```

## Handling `post` requests

I need to do a writeup on how to handle post requests as I think this is badly documented.

## CORS

What is the cors middleware? 

https://fastapi.tiangolo.com/tutorial/cors/ ? Not sure what purpose it solves really.

## influxdb2

(https://docs.influxdata.com/influxdb/v2/install/use-docker-compose/)

influxdb3 is still in alpha but worth keeping an eye on.

getting the db running you have to run `docker compose up` from `integration/influxdb`

## Jaeger

This works as a collector, bit heavy at the moment but is good for demo purposes.

`docker run -d --name jaeger   -e COLLECTOR_ZIPKIN_HOST_PORT=:9411   -e COLLECTOR_OTLP_ENABLED=true   -p 6831:6831/udp   -p 6832:6832/udp   -p 5778:5778   -p 16686:16686   -p 4317:4317   -p 4318:4318   -p 14250:14250   -p 14268:14268   -p 14269:14269   -p 9411:9411   jaegertracing/all-in-one:1.47`



