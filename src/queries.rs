pub trait ToQuerys {
    fn to_querys(&self) -> String;
}

pub struct QeuryValue {
    pub name: String,
    pub value: String,
}

pub trait ToQueryValue {
    fn to_query_value(&self) -> QeuryValue;
}

pub fn parameters_to_qeury(parameters: &[Option<String>]) -> String {
    let strings = parameters
        .iter()
        .filter_map(|f| f.as_ref().map(|s| s.to_owned()))
        .collect::<Vec<String>>();
    if !strings.is_empty() {
        strings.join("&")
    } else {
        "".to_string()
    }
}

pub fn to_qeury_string_single<T>(item: Option<&T>) -> Option<String>
where
    T: ToQueryValue,
{
    item.map(|item| {
        let query_value = item.to_query_value();
        format!("{}={}", query_value.name, query_value.value)
    })
}

pub fn to_qeury_string<T>(items: Option<&[T]>) -> Option<String>
where
    T: ToQueryValue,
{
    match items {
        Some(items) if !items.is_empty() => {
            let name = items.first().unwrap().to_query_value().name;
            let value = items
                .iter()
                .map(ToQueryValue::to_query_value)
                .map(|x| x.value)
                .collect::<Vec<String>>()
                .join(",");
            Some(format!("{}={}", name, value))
        }
        _ => None,
    }
}
