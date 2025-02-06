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

