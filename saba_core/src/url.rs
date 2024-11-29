use alloc::string::{String, ToString};

/// URL
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    /// URL全体
    url: String,
    /// 完全修飾ドメイン名またはIPアドレス
    host: String,
    /// ポート番号
    port: String,
    /// 階層構造
    path: String,
    /// クエリパラメータ
    searchpart: String,
}

impl Url {
    pub const fn new(url: String) -> Self {
        Self {
            url,
            host: String::new(),
            port: String::new(),
            path: String::new(),
            searchpart: String::new(),
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn searchpart(&self) -> &str {
        &self.searchpart
    }

    /// URLをパースする
    pub fn parse(&mut self) -> Result<&Self, String> {
        if !self.is_http() {
            return Err("Only HTTP is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self)
    }

    /// スキームがHTTPかどうか
    fn is_http(&self) -> bool {
        self.url.contains("http://")
    }

    /// ホスト名を抽出する
    fn extract_host(&self) -> String {
        let first_part = url_parts(&self.url).nth(0).unwrap();

        // ポート番号を取り除く
        if let Some(index) = first_part.find(':') {
            first_part[..index].to_string()
        } else {
            first_part.to_string()
        }
    }

    /// ポート番号を抽出する
    fn extract_port(&self) -> String {
        let first_part = url_parts(&self.url).nth(0).unwrap();

        if let Some(index) = first_part.find(':') {
            first_part[index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    /// パスを抽出する
    fn extract_path(&self) -> String {
        let mut url_parts = url_parts(&self.url);

        // パスが存在しない
        if url_parts.clone().count() < 2 {
            return String::new();
        }

        // クエリパラメータを取り除く
        url_parts
            .nth(1)
            .unwrap()
            .split('?')
            .nth(0)
            .unwrap()
            .to_string()
    }

    fn extract_searchpart(&self) -> String {
        let mut url_parts = url_parts(&self.url);

        // パスが存在しない
        if url_parts.clone().count() < 2 {
            return String::new();
        }

        // [パス, クエリパラメータ]
        let mut path_and_searchpart = url_parts.nth(1).unwrap().splitn(2, '?');

        if path_and_searchpart.clone().count() < 2 {
            String::new()
        } else {
            path_and_searchpart.nth(1).unwrap().to_string()
        }
    }
}

/// スキームを取り除き，URLをホスト名とパス&クエリに分割する
fn url_parts(full_url: &str) -> impl Iterator<Item = &str> + Clone {
    full_url.trim_start_matches("http://").splitn(2, '/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse().cloned());
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse().cloned());
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse().cloned());
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse().cloned());
    }

    #[test]
    fn test_url_host_port_path_query() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });

        assert_eq!(expected, Url::new(url).parse().cloned());
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP is supported.".to_string());

        assert_eq!(expected, Url::new(url).parse().cloned());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com".to_string();
        let expected = Err("Only HTTP is supported.".to_string());

        assert_eq!(expected, Url::new(url).parse().cloned());
    }
}
