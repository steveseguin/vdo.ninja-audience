use url::Url;

/// Encode each value separately so URL delimiters stay part of the user's value.
pub fn build_vdo_ninja_url(push_id: &str, audience: &str) -> String {
    let mut url = Url::parse("https://vdo.ninja/").expect("static VDO.Ninja URL is valid");
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("push", push_id);
        if !audience.is_empty() {
            query.append_pair("audience", audience);
        }
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::build_vdo_ninja_url;
    use url::Url;

    fn assert_round_trip(push: &str, audience: &str) {
        let url = Url::parse(&build_vdo_ninja_url(push, audience)).unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("vdo.ninja"));
        assert_eq!(url.fragment(), None);
        let pairs: Vec<_> = url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect();
        let mut expected = vec![("push".to_string(), push.to_string())];
        if !audience.is_empty() {
            expected.push(("audience".to_string(), audience.to_string()));
        }
        assert_eq!(pairs, expected);
    }

    #[test]
    fn preserves_all_generated_special_characters() {
        for special in "!@#$%^&*".chars() {
            assert_round_trip("Demo1234", &format!("{}Abcdefg12345678", special));
            assert_round_trip("Demo1234", &format!("Abcdefg1{}Tail", special));
        }
    }

    #[test]
    fn preserves_manual_values_and_prevents_extra_parameters() {
        for value in ["A+bc123!", "A%23bc123!", "A bc123!", "A&password=other1!", "A#fragment123!", "É密碼 Abc123!"] {
            assert_round_trip("Room &name=#one+two%23", value);
        }
    }

    #[test]
    fn empty_audience_is_omitted() {
        assert_round_trip("Room &name=#one+two%23", "");
    }

    #[test]
    fn ordinary_values_keep_existing_url_shape() {
        assert_eq!(build_vdo_ninja_url("Demo1234", "Abcdefg1234"), "https://vdo.ninja/?push=Demo1234&audience=Abcdefg1234");
    }
}
