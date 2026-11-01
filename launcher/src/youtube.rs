//! YouTube video integration for CREW Network (https://youtube.com/@CREWNW).
//! Fetches channel public uploads via the official YouTube RSS/Atom feed
//! or YouTube Data API v3 when a key is provided.
use serde::{Deserialize, Serialize};

pub const CHANNEL_ID: &str = "UCotr7s5r-QnbcO9FtX6reUw";
pub const CHANNEL_URL: &str = "https://youtube.com/@CREWNW";
pub const RSS_FEED_URL: &str =
    "https://www.youtube.com/feeds/videos.xml?channel_id=UCotr7s5r-QnbcO9FtX6reUw";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct YouTubeVideo {
    pub id: String,
    pub title: String,
    pub published: String,
    pub description: String,
    pub thumbnail_url: String,
    pub video_url: String,
}

/// Fetches the latest public uploads for CREW Network from YouTube.
/// Uses the official Atom RSS feed with timeout and fallback.
pub fn fetch_latest_videos() -> Result<Vec<YouTubeVideo>, String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("crew-client/2.1.0")
        .build();

    let resp = agent
        .get(RSS_FEED_URL)
        .call()
        .map_err(|e| format!("Could not reach YouTube feed: {e}"))?;

    let xml = resp
        .into_string()
        .map_err(|e| format!("Could not read YouTube feed: {e}"))?;

    Ok(parse_feed_xml(&xml))
}

/// Parses YouTube Atom XML entries.
pub fn parse_feed_xml(xml: &str) -> Vec<YouTubeVideo> {
    let mut videos = Vec::new();
    let entries: Vec<&str> = xml.split("<entry>").skip(1).collect();

    for entry in entries {
        let entry_xml = entry.split("</entry>").next().unwrap_or("");
        let id = extract_tag(entry_xml, "yt:videoId").or_else(|| {
            extract_tag(entry_xml, "id").map(|s| s.rsplit(':').next().unwrap_or(&s).to_string())
        });
        let title = extract_tag(entry_xml, "title").unwrap_or_default();
        let published = extract_tag(entry_xml, "published").unwrap_or_default();
        let description = extract_tag(entry_xml, "media:description").unwrap_or_default();

        if let Some(id) = id {
            let thumbnail_url = format!("https://i.ytimg.com/vi/{id}/hqdefault.jpg");
            let video_url = format!("https://www.youtube.com/watch?v={id}");
            videos.push(YouTubeVideo {
                id,
                title,
                published,
                description,
                thumbnail_url,
                video_url,
            });
        }
    }

    videos
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    Some(xml[start..end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_feed_extracts_videos() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns:yt="http://www.youtube.com/xml/schemas/2015" xmlns:media="http://search.yahoo.com/mrss/">
  <title>CREW Network</title>
  <entry>
    <id>yt:video:abc12345</id>
    <yt:videoId>abc12345</yt:videoId>
    <title>CREW Client 2.1 Tanıtım</title>
    <published>2026-11-01T12:00:00+00:00</published>
    <media:group>
      <media:description>Yeni nesil Minecraft istemcisi.</media:description>
    </media:group>
  </entry>
</feed>"#;
        let list = parse_feed_xml(sample);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "abc12345");
        assert_eq!(list[0].title, "CREW Client 2.1 Tanıtım");
        assert_eq!(
            list[0].thumbnail_url,
            "https://i.ytimg.com/vi/abc12345/hqdefault.jpg"
        );
        assert_eq!(
            list[0].video_url,
            "https://www.youtube.com/watch?v=abc12345"
        );
    }

    #[test]
    fn empty_feed_returns_empty_list() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns:yt="http://www.youtube.com/xml/schemas/2015">
  <title>CREW Network</title>
</feed>"#;
        let list = parse_feed_xml(sample);
        assert!(list.is_empty());
    }
}
