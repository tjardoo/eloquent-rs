use eloquent::Eloquent;

fn main() {
    println!("Hello, world!");

    test_example_query_1();

    test_example_query_2();
}

fn test_example_query_1() {
    let result = Eloquent::query()
        .table("flights")
        .r#where("origin", "AMS")
        .where_not("destination", "BKK")
        .where_gt("flight_duration", 8)
        .where_gte("number_of_passengers", 250)
        .where_lte("number_of_stops", 1)
        .where_not_null("gate_number")
        .where_closure(|q| {
            q.where_in("status", vec!["scheduled", "delayed"])
                .where_like("aircraft_code", "KLM%")
        });

    assert_eq!(
        result.build(),
        "SELECT * FROM flights WHERE origin = 'AMS' AND destination != 'BKK' AND flight_duration > 8 AND number_of_passengers >= 250 AND number_of_stops <= 1 AND gate_number IS NOT NULL AND (status IN ('scheduled', 'delayed') AND aircraft_code LIKE 'KLM%')"
    );
}

fn test_example_query_2() {
    let result = Eloquent::query()
        .table("flights")
        .r#where("origin", "AMS")
        .where_closure(|q| {
            q.where_closure(|q| q.where_in("destination", vec!["BKK", "DMK"]))
                .or_where_closure(|q| q.where_like("aircraft_code", "THA%"))
        });

    assert_eq!(
        result.build(),
        "SELECT * FROM flights WHERE origin = 'AMS' AND (destination IN ('BKK', 'DMK') OR aircraft_code LIKE 'THA%')"
    );
}
