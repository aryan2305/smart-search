extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
               .add(b'<').add(b'>').add(b'`');

pub fn construct_codeforces_url(query: &str) -> String {
    if query == "cd" {
        let codeforces_dotcom = "https://codeforces.com";
        
        codeforces_dotcom.to_string()
        
        // Check if it looks like a codeforces profile
        } else if &query[..4] == "cd @" {
            construct_codeforces_profile_url(&query[4..])
        } else {
            // Assume the other match is "cd sometext"
            // and search on codeforces
            construct_codeforces_search_url(&query[3..])
    }
}

pub fn construct_codeforces_profile_url(profile: &str) -> String {
    format!("https://codeforces.com/profile/{}", profile)
}

pub fn construct_codeforces_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT)
               .to_string();
    let codeforces_search_url = format!(
    "https://codeforces.com/problemset?tags={}", encoded_query);
    codeforces_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_codeforces_url() {
        let fake_query = "cd";
        assert_eq!(construct_codeforces_url(fake_query), 
        "https://codeforces.com");
    }

    #[test]
    fn test_construct_codeforces_url_query() {
        let fake_query = "cd divide and conquer";
        assert_eq!(construct_codeforces_url(fake_query),     
        "https://codeforces.com/problemset?tags=divide%20and%20conquer");
    }

    #[test]
    fn test_construct_codeforces_url_profile() {
        let fake_query = "cd @aryan2305";
        assert_eq!(construct_codeforces_url(fake_query), 
        "https://codeforces.com/profile/aryan2305");
    }

    #[test]
    fn test_construct_codeforces_profile_url() {
        let fake_profile = "aryan2305";
        assert_eq!(
            construct_codeforces_profile_url(fake_profile),
            "https://codeforces.com/profile/aryan2305"
        );
    }

    #[test]
    fn test_construct_codeforces_search_url() {
        let fake_query = "dp";
        assert_eq!(
            construct_codeforces_search_url(fake_query),
            "https://codeforces.com/problemset?tags=dp"
        );
     }
}