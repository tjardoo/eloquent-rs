use eloquent::{Eloquent, ToSql};
use ftail::Ftail;

fn main() {
    // Initialize logger to see the output (optional)
    Ftail::new()
        .formatted_console(log::LevelFilter::Off)
        .init()
        .unwrap();

    // Query example 1
    let result = Eloquent::query()
        .table("flights")
        .select("origin_airport")
        .select_avg("startup_time_in_minutes", "startup_time_in_minutes_avg")
        .select_as("airports.city", "destination_city")
        .join(
            "airports",
            "flights.destination_airport",
            "airports.iata_code",
        )
        .r#where("origin_airport", "AMS")
        .where_not_in("flight_number", vec!["KL123", "KL456"])
        .where_not_null("gate_number")
        .where_closure(|q| {
            q.where_gte("flight_duration", 120)
                .or_where_like("airports.city", "%NY%")
        })
        .group_by(vec!["origin_airport", "airports.city"])
        .having_gt("startup_time_in_minutes_avg", 120)
        .order_by_asc("startup_time_in_minutes_avg")
        .limit(20);

    assert_eq!(
        result.sql().unwrap(),
        "SELECT origin_airport, AVG(startup_time_in_minutes) AS startup_time_in_minutes_avg, airports.city AS destination_city FROM flights JOIN airports ON flights.destination_airport = airports.iata_code WHERE origin_airport = 'AMS' AND flight_number NOT IN ('KL123', 'KL456') AND gate_number IS NOT NULL AND (flight_duration >= 120 OR airports.city LIKE '%NY%') GROUP BY origin_airport, airports.city HAVING startup_time_in_minutes_avg > 120 ORDER BY startup_time_in_minutes_avg ASC LIMIT 20"
    );

    // Query example 2
    let result = Eloquent::query()
        .table("arcticles")
        .select(vec![
            "articles.title",
            "articles.slug",
            "articles.published_at",
        ])
        .select_as("authors.name", "author_name")
        .select_count("comments.id", "comments_count")
        .select_avg("comments.rating", "average_rating")
        .join("authors", "articles.author_id", "authors.id")
        .left_join("comments", "articles.id", "comments.article_id")
        .r#where("articles.category", "Technology")
        .where_between("articles.published_at", "2024-01-01", "2024-12-31")
        .r#where("articles.is_published", true)
        .group_by("authors.name")
        .having_gt("comments_count", 10)
        .order_by_desc("articles.published_at")
        .limit(10);

    assert_eq!(
        result.sql().unwrap(),
        "SELECT articles.title, articles.slug, articles.published_at, authors.name AS author_name, COUNT(comments.id) AS comments_count, AVG(comments.rating) AS average_rating FROM arcticles JOIN authors ON articles.author_id = authors.id LEFT JOIN comments ON articles.id = comments.article_id WHERE articles.category = 'Technology' AND articles.published_at BETWEEN '2024-01-01' AND '2024-12-31' AND articles.is_published = true GROUP BY authors.name HAVING comments_count > 10 ORDER BY articles.published_at DESC LIMIT 10"
    );

    // Subquery example
    let subquery = Eloquent::subquery()
        .table("tickets")
        .select("event_id")
        .select_avg("price", "price_avg")
        .group_by("event_id")
        .order_by_desc("price_avg")
        .limit(1);

    let result = Eloquent::query()
        .table("events")
        .select(vec!["event_name", "event_date"])
        .r#where("event_id", subquery);

    assert_eq!(
        result.sql().unwrap(),
        "SELECT event_name, event_date FROM events WHERE event_id = (SELECT event_id, AVG(price) AS price_avg FROM tickets GROUP BY event_id ORDER BY price_avg DESC LIMIT 1)"
    );

    // Pagination example
    let mut last_id = None;

    let query = Eloquent::query()
        .table("departures")
        .select(vec!["flight_number", "departure_date"])
        .paginate::<u64>("id", last_id, 25);

    assert_eq!(
        query.sql().unwrap(),
        "SELECT flight_number, departure_date FROM departures LIMIT 25"
    );

    last_id = Some(1000);

    let query = Eloquent::query()
        .table("departures")
        .select(vec!["flight_number", "departure_date"])
        .paginate("id", last_id, 25);

    assert_eq!(
        query.sql().unwrap(),
        "SELECT flight_number, departure_date FROM departures WHERE id > 1000 LIMIT 25"
    );

    // Insert Example
    let rows = vec![
        vec![
            ("name", Box::new("Alice") as Box<dyn ToSql>),
            ("email", Box::new("alice@example.com") as Box<dyn ToSql>),
            ("age", Box::new(21) as Box<dyn ToSql>),
        ],
        vec![
            ("name", Box::new("Bob") as Box<dyn ToSql>),
            ("email", Box::new("bob@example.com") as Box<dyn ToSql>),
            ("age", Box::new(22) as Box<dyn ToSql>),
        ],
    ];

    let query = Eloquent::query().table("users").insert_many(rows);

    assert_eq!(
    query.sql().unwrap(),
    "INSERT INTO users (name, email, age) VALUES ('Alice', 'alice@example.com', 21), ('Bob', 'bob@example.com', 22)"
    );
}
