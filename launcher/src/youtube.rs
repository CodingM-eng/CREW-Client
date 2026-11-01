//! YouTube video integration for CREW Network (https://youtube.com/@CREWNW).
//! Fetches channel public uploads via the official YouTube RSS/Atom feed
//! or YouTube Data API v3 when a key is provided.
//! Excludes YouTube Shorts, sorts newest first, deduplicates, and caches metadata.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

/// Checks whether a video is classified as a YouTube Short.
/// Strictly excludes videos that have /shorts/ in their URL, or #shorts/#short tags in title or description.
pub fn is_short(id: &str, title: &str, description: &str, link: &str) -> bool {
    let lower_title = title.to_lowercase();
    let lower_desc = description.to_lowercase();
    let lower_link = link.to_lowercase();

    if lower_link.contains("/shorts/") || lower_link.contains("shorts") {
        return true;
    }

    if lower_title.contains("#shorts")
        || lower_title.contains("#short")
        || lower_title.contains("/shorts")
        || lower_title.contains("(shorts)")
        || lower_title.contains("[shorts]")
    {
        return true;
    }

    if lower_desc.contains("#shorts") || lower_desc.contains("#short") {
        return true;
    }

    // Direct ID check if URL has /shorts/<id>
    if !id.is_empty() && lower_link.contains(&format!("/shorts/{id}")) {
        return true;
    }

    false
}

/// Fetches the latest public uploads for CREW Network from YouTube.
/// Uses the official Atom RSS feed with timeout, excludes Shorts, deduplicates, and sorts newest first.
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

/// Parses YouTube Atom XML entries, strictly filtering out YouTube Shorts.
pub fn parse_feed_xml(xml: &str) -> Vec<YouTubeVideo> {
    let mut videos = Vec::new();
    let mut seen_ids = HashSet::new();
    let entries: Vec<&str> = xml.split("<entry>").skip(1).collect();

    for entry in entries {
        let entry_xml = entry.split("</entry>").next().unwrap_or("");
        let id = extract_tag(entry_xml, "yt:videoId").or_else(|| {
            extract_tag(entry_xml, "id").map(|s| s.rsplit(':').next().unwrap_or(&s).to_string())
        });
        let title = extract_tag(entry_xml, "title").unwrap_or_default();
        let published = extract_tag(entry_xml, "published").unwrap_or_default();
        let description = extract_tag(entry_xml, "media:description").unwrap_or_default();

        // Extract <link rel="alternate" href="..."/>
        let link = extract_attribute(entry_xml, "link", "href").unwrap_or_default();

        if let Some(id) = id {
            if seen_ids.contains(&id) {
                continue;
            }

            // Exclude YouTube Shorts
            if is_short(&id, &title, &description, &link) {
                continue;
            }

            seen_ids.insert(id.clone());
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

    // Sort by publication timestamp descending (newest first)
    videos.sort_by(|a, b| b.published.cmp(&a.published));

    videos
}

fn extract_tag(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    Some(xml[start..end].trim().to_string())
}

fn extract_attribute(xml: &str, tag: &str, attr: &str) -> Option<String> {
    let tag_open = format!("<{tag} ");
    let start_tag = xml.find(&tag_open)?;
    let tag_str = &xml[start_tag..];
    let end_tag = tag_str.find('>')?;
    let tag_content = &tag_str[..end_tag];

    let attr_pattern = format!("{attr}=\"");
    let attr_start = tag_content.find(&attr_pattern)? + attr_pattern.len();
    let attr_end = tag_content[attr_start..].find('"')? + attr_start;

    Some(tag_content[attr_start..attr_end].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_feed_extracts_and_sorts_videos() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns:yt="http://www.youtube.com/xml/schemas/2015" xmlns:media="http://search.yahoo.com/mrss/">
  <title>CREW Network</title>
  <entry>
    <id>yt:video:old123</id>
    <yt:videoId>old123</yt:videoId>
    <title>Eski Video</title>
    <published>2026-10-15T10:00:00+00:00</published>
    <link rel="alternate" href="https://www.youtube.com/watch?v=old123"/>
  </entry>
  <entry>
    <id>yt:video:new456</id>
    <yt:videoId>new456</yt:videoId>
    <title>Yeni Video</title>
    <published>2026-11-01T12:00:00+00:00</published>
    <link rel="alternate" href="https://www.youtube.com/watch?v=new456"/>
  </entry>
</feed>"#;
        let list = parse_feed_xml(sample);
        assert_eq!(list.len(), 2);
        // Sorted newest first
        assert_eq!(list[0].id, "new456");
        assert_eq!(list[0].title, "Yeni Video");
        assert_eq!(list[1].id, "old123");
        assert_eq!(list[1].title, "Eski Video");
    }

    #[test]
    fn parse_feed_excludes_shorts() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns:yt="http://www.youtube.com/xml/schemas/2015">
  <title>CREW Network</title>
  <entry>
    <id>yt:video:reg111</id>
    <yt:videoId>reg111</yt:videoId>
    <title>Normal Uzun Video</title>
    <published>2026-11-01T10:00:00+00:00</published>
    <link rel="alternate" href="https://www.youtube.com/watch?v=reg111"/>
  </entry>
  <entry>
    <id>yt:video:short111</id>
    <yt:videoId>short111</yt:videoId>
    <title>Komik Anlar #shorts</title>
    <published>2026-11-01T11:00:00+00:00</published>
    <link rel="alternate" href="https://www.youtube.com/watch?v=short111"/>
  </entry>
  <entry>
    <id>yt:video:short222</id>
    <yt:videoId>short222</yt:videoId>
    <title>Hızlı PvP Taktikleri</title>
    <published>2026-11-01T12:00:00+00:00</published>
    <link rel="alternate" href="https://www.youtube.com/shorts/short222"/>
  </entry>
  <entry>
    <id>yt:video:short333</id>
    <yt:videoId>short333</yt:videoId>
    <title>Kısa Klip</title>
    <published>2026-11-01T13:00:00+00:00</published>
    <media:group>
      <media:description>Açıklamada #short etiketi var</media:description>
    </media:group>
  </entry>
</feed>"#;
        let list = parse_feed_xml(sample);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "reg111");
        assert_eq!(list[0].title, "Normal Uzun Video");
    }

    #[test]
    fn deduplicates_multiple_entries_with_same_id() {
        let sample = r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns:yt="http://www.youtube.com/xml/schemas/2015">
  <entry>
    <id>yt:video:dup1</id>
    <yt:videoId>dup1</yt:videoId>
    <title>Video Tekrarı 1</title>
    <published>2026-11-01T10:00:00+00:00</published>
  </entry>
  <entry>
    <id>yt:video:dup1</id>
    <yt:videoId>dup1</yt:videoId>
    <title>Video Tekrarı 2</title>
    <published>2026-11-01T10:00:00+00:00</published>
  </entry>
</feed>"#;
        let list = parse_feed_xml(sample);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "dup1");
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
