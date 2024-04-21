pub fn to_list(list: &[String]) -> String {
    let mut out = String::new();
    for e in list {
        out.push_str(e);
        out.push(',');
    }
    out.pop();

    out
}