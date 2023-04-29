use crate::models::TrainDeparture;
use html_parser::Dom;

fn parse_dom_from_provider_response(provider_response: &str) -> html_parser::Dom {
    Dom::parse(&provider_response).unwrap()
}

fn get_plan_table(dom: &html_parser::Dom) -> &html_parser::Element {
    let content_table = dom
        .children
        .iter()
        .filter(|x| match &x.element().unwrap().id {
            Some(x) => x == "content-table",
            None => false,
        })
        .collect::<Vec<&html_parser::Node>>()[0]
        .element()
        .unwrap();

    // Get table mobile landscape
    let table_mobile_landscape = content_table
        .children
        .iter()
        .filter(|child| match &child.element() {
            Some(element) => match &element.id {
                Some(id) => id == "table-mobile-landscape",
                None => false,
            },
            None => false,
        })
        .collect::<Vec<&html_parser::Node>>()[0]
        .element()
        .unwrap();

    // Get Plan table
    let plan_table = table_mobile_landscape
        .children
        .iter()
        .filter(|child| match &child.element() {
            Some(element) => match &element.id {
                Some(id) => id == "plan-table",
                None => false,
            },
            None => false,
        })
        .collect::<Vec<&html_parser::Node>>()[0]
        .element()
        .unwrap();

    return plan_table;
}

fn parse_departures(
    plan_table: &html_parser::Element,
    departure_station: &str,
) -> Vec<TrainDeparture> {
    let mut entries = Vec::new();

    for child in plan_table.children[1..].iter() {
        let entry = child.element().unwrap();
        let platform_node = entry.children[4].element().unwrap();

        let platform = if platform_node.children.len() > 0 {
            let child = &platform_node.children[0];

            let platform_id_node = match child.element() {
                Some(element) => element.children[0].text().unwrap(),
                None => child.text().unwrap(),
            };

            Some(
                platform_id_node
                    .replace("\n", "")
                    .replace("\r", "")
                    .replace("\t", "")
                    .to_string(),
            )
        } else {
            None
        };

        let time = if entry.children[0].element().unwrap().children.len() > 0 {
            entry.children[0].element().unwrap().children[0]
                .element()
                .unwrap()
                .children[0]
                .text()
                .unwrap()
                .to_string()
        } else {
            "0 min".to_string()
        };

        let td = TrainDeparture {
            time,
            destination: entry.children[1].element().unwrap().children[0]
                .element()
                .unwrap()
                .children[0]
                .text()
                .unwrap()
                .to_string(),
            service: entry.children[2].element().unwrap().children[0]
                .element()
                .unwrap()
                .classes[0]
                .to_string(),
            station: departure_station.to_string(),
            platform,
            date: entry.children[7].element().unwrap().children[0]
                .text()
                .unwrap()
                .to_string(),
        };
        entries.push(td);
    }
    return entries;
}

pub fn parse_provider_response(
    provider_response: &str,
    departure_station: &str,
) -> Vec<TrainDeparture> {
    let dom = parse_dom_from_provider_response(&provider_response);
    let plan_table = get_plan_table(&dom);
    let tds = parse_departures(plan_table, departure_station);
    return tds;
}
