use core::convert::TryFrom;

mod iter;
mod model;
mod query;
mod response;

const HOST: &str = "https://wabi-us-gov-virginia-api.analysis.usgovcloudapi.net/public/reports/querydata?synchronous=true";

const USER_AGENT: &str = "User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0";

const BI_RESOURCE_KEY_KEY: &str = "X-PowerBI-ResourceKey";
const BI_RESOURCE_KEY_VALUE: &str = "b2c8d2f2-3ad1-48dc-883c-d4163a6e2d8f";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(BI_RESOURCE_KEY_KEY, reqwest::header::HeaderValue::from_static(BI_RESOURCE_KEY_VALUE));
    headers.insert("Accept", reqwest::header::HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert("ActivityId", reqwest::header::HeaderValue::from_static("a366f021-d490-ed01-6681-0fe32cf1255a"));
    headers.insert("RequestId", reqwest::header::HeaderValue::from_static("141bd242-9744-e182-52a8-ed8a1633b878"));
    headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/json;charset=UTF-8"));
    headers.insert("Origin", reqwest::header::HeaderValue::from_static("https://app.powerbigov.us"));
    headers.insert("Connection", reqwest::header::HeaderValue::from_static("keep-alive"));
    headers.insert("Referer", reqwest::header::HeaderValue::from_static("https://app.powerbigov.us/view?r=eyJrIjoiYjJjOGQyZjItM2FkMS00OGRjLTg4M2MtZDQxNjNhNmUyZDhmIiwidCI6IjczZDYxNzk5LWMyODQtNDAyMi04ZDQxLTU0Y2M0ZjE5MjllZiJ9"));
    headers.insert("Sec-GPC", reqwest::header::HeaderValue::from_static("1"));
    headers.insert("Pragma", reqwest::header::HeaderValue::from_static("no-cache"));
    headers.insert("Cache-Control", reqwest::header::HeaderValue::from_static("no-cache"));

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(headers)
        .build()?;

    let mut w = csv::Writer::from_writer(
        std::io::BufWriter::new(
            std::fs::File::create(
                "./output.csv",
            )?,
        ),
    );

    let mut records = iter::Index::new(client).await?;

    let mut count = 0;

    while let Some(row) = records.next().await? {
        let officer = model::Officer::try_from(row)?;

        w.serialize(&officer)?;

        if let Some(tokens) = records.progress() {
            println!("querying from {:?}", tokens);

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            count += 1;
            if count % 10 == 0 {
                w.flush()?;
            }
        }
    }

    w.flush()?;

    Ok(())
}
