use rocket::*;

#[get("/docs")]
pub fn docs_route() -> &'static str {
    "
    USAGE
      POST /
          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          Example: curl --data-binary @file.txt http://localhost:8000

      GET /<id>
          retrieves the content for the paste with id `<id>`

          Example: curl http://localhost:8000/abc

      GET /all
          retrieves all the paste ids from the upload directory

          Example: curl http://localhost:8000/all

      GET /health
          returns 'Ok' if the service is running

          Example: curl http://localhost:8000/health
    "
}
