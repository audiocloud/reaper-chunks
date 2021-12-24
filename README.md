# reaper-chunks

Parses RPP project files and their chunks (elements) made by [REAPER](https://reaper.fm),
with [nom](https://github.com/Geal/nom) parser combinator library.

## project-level API example

```rust
fn test() {
    let input = "<PROJECT ..."; // input from RPP file
    let (_, parsed) = reaper_chunks::parse_element(input)?;
    let project = reaper_chunks::Project(element);
    let mut total_length = 0.0;
    for track in project.tracks() {
        for item in track.items() {
            total_length += item.len();
        }
    }
}
```

## chunk-level API example

```rust
fn test() {
    let input = "<TRACK ..."; // input from REAPER API GetTrackStateChunk
    let (_, parsed) = reaper_chunks::parse_element(input)?;
    let mut track = Track(parsed);
    
    // do things with Track
    
    let serialized = track.to_string(); // for going back to SetTrackStateChunk
}
```

## raw element example

```rust
fn test() {
    let input = "<PROJECT ..."; // input
    let (_, parsed) = reaper_chunks::parse_element(input)?; // parsed: RElement

    let mut total_length = 0.0;
    for track in &parsed.children {
        if track.tag != "TRACK" {
            continue;
        }
        for media_item in &track.children {
            if media_item.tag != "ITEM" {
                continue;
            }
            total_length += media_item.get_num_attr("LENGTH");
        }
    }
}
```