pub mod reit_data {
	use scraper::selector::{Selector};
	use selectors::parser::SelectorParseErrorKind;
	use scylla::transport::errors;

    #[derive(Debug)]
	 pub enum ParsError<'a> {
            ReqError(reqwest::Error),
            SerialError(serde_json::Error),
            ParseError(cssparser::ParseError<'a, SelectorParseErrorKind<'a>>),
            NewSessError(errors::NewSessionError),
            SQueryError(errors::QueryError)
    }

    impl From<reqwest::Error> for ParsError<'_> {
        #[inline]
        fn from(err: reqwest::Error) -> ParsError<'static> {
            return ParsError::ReqError(err);
        }
    }
    impl From<errors::QueryError> for ParsError<'_> {
        #[inline]
        fn from(err: errors::QueryError) -> ParsError<'static> {
            return ParsError::SQueryError(err);
        }
    }

    impl From<errors::NewSessionError> for ParsError<'_> {
        #[inline]
        fn from(err: errors::NewSessionError) -> ParsError<'static> {
            return ParsError::NewSessError(err);
        }
    }
    pub async fn reitfetch<'i>(url: String) -> Result<(String), ParsError<'i>> {
    	let reit_data: String = reqwest::Client::new().get(url).send().await?.text().await?;
    	Ok(reit_data)
    }

    pub fn selector_parse<'i>(parse_val: &'i str) -> Result<Selector, ParsError<'i>>{
        let parser = Selector::parse(&parse_val).unwrap();
        Ok(parser)
    }
}

