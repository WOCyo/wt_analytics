pub fn is_allowed_origin(origin: &str) -> bool {
    (origin == "https://wt.tepis.me")
        || (origin == "https://wt.bgme.me")
        || (origin == "https://rbq.desi")
        || (origin == "https://wt.makai.city")
        || (origin == "https://wt.0w0.bid")
        // || (origin == "http://localhost:2333")
        // || (origin == "http://127.0.0.1:2333")
}
