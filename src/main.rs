use eloquent::Eloquent;

fn main() {
    println!("Hello, world!");

    let result = Eloquent::query()
        .table("flights")
        .select("origin_airport")
        .select_avg("startup_time_in_minutes")
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
        .having_gt("AVG(startup_time_in_minutes)", 120)
        .order_by_asc("AVG(startup_time_in_minutes)")
        .limit(20);

    println!("{}", result.pretty_sql().unwrap()); // or .sql() for unformatted SQL

    // SELECT
    //     origin_airport,
    //     AVG(startup_time_in_minutes),
    //     airports.city AS destination_city
    // FROM
    //     flights
    //     JOIN airports ON flights.destination_airport = airports.iata_code
    // WHERE
    //     origin_airport = 'AMS'
    //     AND flight_number NOT IN ('KL123', 'KL456')
    //     AND gate_number IS NOT NULL
    //     AND (
    //         flight_duration >= 120
    //         OR airports.city LIKE '%NY%'
    //     )
    // GROUP BY
    //     origin_airport,
    //     airports.city
    // HAVING
    //     AVG(startup_time_in_minutes) > 120
    // ORDER BY
    //     AVG(startup_time_in_minutes) ASC
    // LIMIT
    //     20
}
