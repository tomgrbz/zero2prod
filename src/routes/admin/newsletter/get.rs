use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn submit_newsletter_form() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Create Newsletter</title>
</head>
<body>
    <form action="/admin/newsletter" method="post">
        <label>Title
            <input
                type="text"
                placeholder="Enter Newsletter Title"
                name="title"
            >
        </label>
        <br>
        <label for="text_content">Text Content
        </label>
        <br>
        <textarea
        placeholder="Enter the text content"
        name="text_content"
        id="text_content"
        rows="20"
        cols="30">
        </textarea>

        <label for="html_content">HTML Content
        </label>
        <br>
        <textarea
        placeholder="Enter the HTML content"
        name="html_content"
        id="html_content"
        rows="20"
        cols="30">
        </textarea>
        <br>
    <button type="submit">Publish</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#
            .to_string(),
    ))
}
