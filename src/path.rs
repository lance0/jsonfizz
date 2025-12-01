#[derive(Debug, Clone)]
pub enum PathSegment {
    Key(String),
    Index(usize),
}

pub type JsonPath = Vec<PathSegment>;

pub fn parse_path(path: &str) -> Result<JsonPath, crate::error::JsonfizzError> {
    let mut segments = Vec::new();
    for part in path.split('.') {
        if let Some(pos) = part.rfind('[') {
            let key = part[..pos].to_string();
            let index_str = &part[pos + 1..part.len() - 1];
            let index: usize = index_str.parse().map_err(|_| crate::error::JsonfizzError::Path(format!("Invalid index '{}' in path '{}'", index_str, path)))?;
            segments.push(PathSegment::Key(key));
            segments.push(PathSegment::Index(index));
        } else {
            segments.push(PathSegment::Key(part.to_string()));
        }
    }
    Ok(segments)
}

pub fn resolve(value: &serde_json::Value, path: &JsonPath) -> Result<serde_json::Value, crate::error::JsonfizzError> {
    let mut current = value.clone();
    for segment in path {
        match segment {
            PathSegment::Key(key) => {
                if let Some(obj) = current.as_object() {
                    if let Some(v) = obj.get(key) {
                        current = v.clone();
                    } else {
                        return Err(crate::error::JsonfizzError::Path(format!("Key '{}' not found", key)));
                    }
                } else {
                    return Err(crate::error::JsonfizzError::Path(format!("Expected object for key '{}', found {:?}", key, current)));
                }
            }
            PathSegment::Index(index) => {
                if let Some(arr) = current.as_array() {
                    if *index < arr.len() {
                        current = arr[*index].clone();
                    } else {
                        return Err(crate::error::JsonfizzError::Path(format!("Index {} out of bounds (len {})", index, arr.len())));
                    }
                } else {
                    return Err(crate::error::JsonfizzError::Path(format!("Expected array for index {}, found {:?}", index, current)));
                }
            }
        }
    }
    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_path() {
        assert_eq!(parse_path("data.items[0].id").unwrap(), vec![
            PathSegment::Key("data".to_string()),
            PathSegment::Key("items".to_string()),
            PathSegment::Index(0),
            PathSegment::Key("id".to_string()),
        ]);
    }

    #[test]
    fn test_resolve() {
        let value = json!({
            "data": {
                "items": [{"id": "foo"}],
            }
        });
        let path = parse_path("data.items[0].id").unwrap();
        assert_eq!(resolve(&value, &path).unwrap(), json!("foo"));
    }
}